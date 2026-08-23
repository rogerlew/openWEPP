//! Default-off Child 2C shared-carrier and terminal-owner handoff.
//!
//! This module is deliberately independent of the existing Stage 3 evaluation
//! shadow. It owns the released shared-air equations, integer-tick event
//! admissibility, complete-owner staging, and the commit boundary used by the
//! opt-in direct scheduler method. The normal `CoE` path never calls this module.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_coupled_time::{CoupledTimeError, Digest32, ModelTimeNs, TimeSupport, digest_bytes};
use openwepp_kernel_contract::TileId;
use openwepp_land_surface_energy::OfeId;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub const SHARED_CARRIER_ID: &str = "shared-carrier";
pub const STAGE3_SNOW_ID: &str = "stage3-snow";
pub const V11_CANOPY_ID: &str = "v11-canopy";
pub const LSE_MINIMUM_SUPPORT_NS: u128 = 600_000_000;
pub const COMPLETE_OWNER_MANIFEST: [&str; 7] = [
    "vegetation",
    "snow",
    "land_surface_energy",
    "surface_liquid",
    "hydrology",
    "bgc",
    "soil_thermal",
];
const SIGMA_W_M2_K4: f64 = 5.670_374_419e-8;
const CLOSURE_TOLERANCE: f64 = 1.0e-9;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum SnowStage3HandoffError {
    #[error(transparent)]
    CoupledTime(#[from] CoupledTimeError),
    #[error("SNOWENERGY-E-WIND-001: {0}")]
    InvalidExposure(&'static str),
    #[error("SNOWENERGY-E-CARRIER-001: {0}")]
    InvalidCarrier(&'static str),
    #[error("SNOWENERGY-E-SCOPE-001: canopy-intercepted snow is outside Child 2C")]
    CanopyInterceptedSnow,
    #[error("SNOWENERGY-E-REGIME-001: {0}")]
    InvalidRegime(&'static str),
    #[error("SNOWENERGY-E-LW-001: {0}")]
    InvalidLongwave(&'static str),
    #[error("SNOWENERGY-E-LEDGER-001: {0}")]
    InvalidLedger(&'static str),
    #[error("SC-VEGETATIONTRANSACTION-E-OWNER-001: {0}")]
    InvalidOwnerSet(&'static str),
    #[error("SC-LANDSURFACEENERGY-E-SUPPORT-001: {0}")]
    InvalidSnowFreeSupport(&'static str),
    #[error("SC-LANDSURFACEENERGY-E-SNOW-OPERAND-001")]
    SnowOperandInSnowFreeContinuation,
    #[error("SC-VEGETATIONTRANSACTION-E-STATE-001: {0}")]
    InvalidState(&'static str),
    #[error(
        "SC-SNOWENERGY-E-FIXED-POINT-001: bounded covered fixed-point iteration did not converge"
    )]
    FixedPointIterationLimit,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum SegmentPhase {
    SnowCovered,
    SnowFree,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SealedExposureReceipt {
    pub receipt_id: String,
    pub provider: String,
    pub provider_digest: String,
    pub source: String,
    pub wind_m_s: f64,
    pub transfer_height_m: f64,
    pub roughness_m: f64,
}

impl SealedExposureReceipt {
    fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        if self.receipt_id.is_empty()
            || self.provider != "sealed-stage3-exposure"
            || self.provider_digest.is_empty()
            || self.source != "sealed-exposure-v1"
            || !self.wind_m_s.is_finite()
            || self.wind_m_s <= 0.0
            || !self.transfer_height_m.is_finite()
            || self.transfer_height_m <= 0.0
            || !self.roughness_m.is_finite()
            || self.roughness_m <= 0.0
        {
            return Err(SnowStage3HandoffError::InvalidExposure(
                "sealed provider, geometry, and projected wind are required",
            ));
        }
        if self.transfer_height_m.to_bits() != 5.0_f64.to_bits()
            || self.roughness_m.to_bits() != 0.005_f64.to_bits()
        {
            return Err(SnowStage3HandoffError::InvalidExposure(
                "Child 2C requires the sealed 5 m transfer height and 0.005 m roughness",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ParticipantSupportReceipt {
    pub participant_id: String,
    pub support_receipt_id: String,
    pub minimum_support_ns: ModelTimeNs,
}

impl ParticipantSupportReceipt {
    fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        if self.participant_id.is_empty() || self.support_receipt_id.is_empty() {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "support identities must be nonempty",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct CarrierSurface {
    pub temperature_k: f64,
    pub specific_humidity: f64,
    pub heat_conductance_m_s: f64,
    pub vapor_conductance_m_s: f64,
}

impl CarrierSurface {
    fn validate(&self, name: &'static str) -> Result<(), SnowStage3HandoffError> {
        if !self.temperature_k.is_finite()
            || self.temperature_k <= 0.0
            || !self.specific_humidity.is_finite()
            || !(0.0..=1.0).contains(&self.specific_humidity)
            || !self.heat_conductance_m_s.is_finite()
            || self.heat_conductance_m_s <= 0.0
            || !self.vapor_conductance_m_s.is_finite()
            || self.vapor_conductance_m_s <= 0.0
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(name));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct CanopyLongwaveComponent {
    pub temperature_k: f64,
    pub emissive_area_weight: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct SnowCarrierLedgerInput {
    pub duration_s: f64,
    pub snow_ice_start_kg_m2: f64,
    pub solid_precipitation_kg_m2: f64,
    pub melt_kg_m2: f64,
    pub sublimation_kg_m2: f64,
    pub deposition_kg_m2: f64,
    pub liquid_start_kg_m2: f64,
    pub rain_kg_m2: f64,
    pub refreeze_kg_m2: f64,
    pub liquid_runoff_kg_m2: f64,
    pub energy_start_j_m2: f64,
    pub external_energy_j_m2: f64,
    pub canopy_energy_j_m2: f64,
    pub snow_energy_j_m2: f64,
    pub energy_end_j_m2: f64,
    pub canopy_snow_longwave_exchange_j_m2: f64,
    pub snow_canopy_longwave_exchange_j_m2: f64,
}

impl SnowCarrierLedgerInput {
    fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        let nonnegative = [
            self.snow_ice_start_kg_m2,
            self.solid_precipitation_kg_m2,
            self.melt_kg_m2,
            self.sublimation_kg_m2,
            self.deposition_kg_m2,
            self.liquid_start_kg_m2,
            self.rain_kg_m2,
            self.refreeze_kg_m2,
            self.liquid_runoff_kg_m2,
        ];
        if !self.duration_s.is_finite()
            || self.duration_s <= 0.0
            || nonnegative
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
            || [
                self.energy_start_j_m2,
                self.external_energy_j_m2,
                self.canopy_energy_j_m2,
                self.snow_energy_j_m2,
                self.energy_end_j_m2,
                self.canopy_snow_longwave_exchange_j_m2,
                self.snow_canopy_longwave_exchange_j_m2,
            ]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err(SnowStage3HandoffError::InvalidLedger(
                "ledger operands must be finite and mass operands nonnegative",
            ));
        }
        let snow_end = self.snow_ice_start_kg_m2 + self.solid_precipitation_kg_m2
            - self.melt_kg_m2
            - self.sublimation_kg_m2
            + self.deposition_kg_m2;
        let liquid_end = self.liquid_start_kg_m2 + self.rain_kg_m2 + self.melt_kg_m2
            - self.refreeze_kg_m2
            - self.liquid_runoff_kg_m2;
        if snow_end < -CLOSURE_TOLERANCE || liquid_end < -CLOSURE_TOLERANCE {
            return Err(SnowStage3HandoffError::InvalidLedger(
                "derived snow or liquid end is negative",
            ));
        }
        let energy_closure =
            self.external_energy_j_m2 + self.canopy_energy_j_m2 + self.snow_energy_j_m2
                - (self.energy_end_j_m2 - self.energy_start_j_m2);
        let longwave_closure =
            self.canopy_snow_longwave_exchange_j_m2 + self.snow_canopy_longwave_exchange_j_m2;
        if energy_closure.abs() > CLOSURE_TOLERANCE || longwave_closure.abs() > CLOSURE_TOLERANCE {
            return Err(SnowStage3HandoffError::InvalidLedger(
                "energy and reciprocal longwave ledgers do not close",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SharedCarrierInput {
    pub phase: SegmentPhase,
    pub rho_air_kg_m3: f64,
    pub cp_air_j_kg_k: f64,
    pub reference: CarrierSurface,
    pub canopy: CarrierSurface,
    pub snow: CarrierSurface,
    pub canopy_longwave_components: Vec<CanopyLongwaveComponent>,
    pub exposure: SealedExposureReceipt,
    pub active_participants: Vec<String>,
    pub support_receipts: Vec<ParticipantSupportReceipt>,
    pub atmospheric_longwave_w_m2: f64,
    pub effective_canopy_cover: f64,
    pub canopy_intercepted_snow: bool,
    pub ledger: SnowCarrierLedgerInput,
}

/// Sealed external operands admitted at the covered-consumer seam.
///
/// This type intentionally contains atmospheric, exposure, cadence, and
/// topology receipts only. Live canopy/snow temperatures, conductances, and
/// carrier ledgers are derived by the covered V11 adopter from committed
/// owners. Keeping those fields crate-private prevents a runner or prepared
/// day caller from substituting result physics at the capability boundary.
#[derive(Clone, Debug, PartialEq)]
pub struct SealedCoveredCarrierForcing {
    pub(crate) rho_air_kg_m3: f64,
    pub(crate) cp_air_j_kg_k: f64,
    pub(crate) reference_temperature_k: f64,
    pub(crate) reference_specific_humidity: f64,
    pub(crate) atmospheric_longwave_w_m2: f64,
    pub(crate) effective_canopy_cover: f64,
    pub(crate) exposure: SealedExposureReceipt,
    pub(crate) active_participants: Vec<String>,
    pub(crate) support_receipts: Vec<ParticipantSupportReceipt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SealedCoveredCarrierForcingInputs {
    pub rho_air_kg_m3: f64,
    pub cp_air_j_kg_k: f64,
    pub reference_temperature_k: f64,
    pub reference_specific_humidity: f64,
    pub atmospheric_longwave_w_m2: f64,
    pub effective_canopy_cover: f64,
    pub exposure: SealedExposureReceipt,
    pub active_participants: Vec<String>,
    pub support_receipts: Vec<ParticipantSupportReceipt>,
}

impl SealedCoveredCarrierForcing {
    pub fn try_new(
        inputs: SealedCoveredCarrierForcingInputs,
    ) -> Result<Self, SnowStage3HandoffError> {
        if !inputs.rho_air_kg_m3.is_finite()
            || inputs.rho_air_kg_m3 <= 0.0
            || !inputs.cp_air_j_kg_k.is_finite()
            || inputs.cp_air_j_kg_k <= 0.0
            || !inputs.reference_temperature_k.is_finite()
            || inputs.reference_temperature_k <= 0.0
            || !inputs.reference_specific_humidity.is_finite()
            || !(0.0..=1.0).contains(&inputs.reference_specific_humidity)
            || !inputs.atmospheric_longwave_w_m2.is_finite()
            || !inputs.effective_canopy_cover.is_finite()
            || inputs.effective_canopy_cover < 0.0
            || inputs.effective_canopy_cover >= 1.0
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "sealed covered atmosphere and geometry domain",
            ));
        }
        inputs.exposure.validate()?;
        if inputs.active_participants.is_empty()
            || inputs
                .active_participants
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
            || inputs.support_receipts.len() != inputs.active_participants.len()
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "sealed covered participant topology",
            ));
        }
        validate_participants(&inputs.active_participants, &inputs.support_receipts)?;
        Ok(Self {
            rho_air_kg_m3: inputs.rho_air_kg_m3,
            cp_air_j_kg_k: inputs.cp_air_j_kg_k,
            reference_temperature_k: inputs.reference_temperature_k,
            reference_specific_humidity: inputs.reference_specific_humidity,
            atmospheric_longwave_w_m2: inputs.atmospheric_longwave_w_m2,
            effective_canopy_cover: inputs.effective_canopy_cover,
            exposure: inputs.exposure,
            active_participants: inputs.active_participants,
            support_receipts: inputs.support_receipts,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stage3BoundaryIdentity {
    Provisional {
        carrier_receipt_sha256: Digest32,
    },
    Final {
        provisional_carrier_receipt_sha256: Digest32,
        optical_receipt_sha256: Digest32,
        reciprocal_longwave_receipt_sha256: Digest32,
        final_destination_receipt_sha256: Digest32,
        final_lane_receipt_sha256: Digest32,
    },
}

impl Stage3BoundaryIdentity {
    fn validate(self) -> Result<(), SnowStage3HandoffError> {
        let digests = match self {
            Self::Provisional {
                carrier_receipt_sha256,
            } => [
                carrier_receipt_sha256,
                Digest32::zero(),
                Digest32::zero(),
                Digest32::zero(),
                Digest32::zero(),
            ],
            Self::Final {
                provisional_carrier_receipt_sha256,
                optical_receipt_sha256,
                reciprocal_longwave_receipt_sha256,
                final_destination_receipt_sha256,
                final_lane_receipt_sha256,
            } => [
                provisional_carrier_receipt_sha256,
                optical_receipt_sha256,
                reciprocal_longwave_receipt_sha256,
                final_destination_receipt_sha256,
                final_lane_receipt_sha256,
            ],
        };
        if digests[0] == Digest32::zero()
            || (matches!(self, Self::Final { .. }) && digests[1..].contains(&Digest32::zero()))
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "Stage-3 boundary identity is incomplete",
            ));
        }
        Ok(())
    }
}

/// Exact Stage-3 snow boundary supplied by the shared canopy-air carrier.
/// All energy and vapor operands are support totals; Stage-3 scales them only
/// when its admitted thermal substep is shorter than the parent support.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stage3SnowSurfaceBoundaryReceiptV1 {
    pub support: TimeSupport,
    pub sensible_energy_j_m2: f64,
    pub vapor_mass_kg_m2: f64,
    pub latent_energy_j_m2: f64,
    pub shortwave_energy_j_m2: f64,
    pub net_longwave_energy_j_m2: f64,
    pub precipitation_advection_j_m2: f64,
    pub latent_heat_j_kg: f64,
    pub beginning_stage3_state_sha256: Digest32,
    pub identity: Stage3BoundaryIdentity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stage3SnowSurfaceBoundaryReceiptInputs {
    pub support: TimeSupport,
    pub sensible_energy_j_m2: f64,
    pub vapor_mass_kg_m2: f64,
    pub latent_energy_j_m2: f64,
    pub shortwave_energy_j_m2: f64,
    pub net_longwave_energy_j_m2: f64,
    pub precipitation_advection_j_m2: f64,
    pub latent_heat_j_kg: f64,
    pub beginning_stage3_state_sha256: Digest32,
    pub identity: Stage3BoundaryIdentity,
}

impl Stage3SnowSurfaceBoundaryReceiptV1 {
    pub fn try_new(
        inputs: Stage3SnowSurfaceBoundaryReceiptInputs,
    ) -> Result<Self, SnowStage3HandoffError> {
        if [
            inputs.sensible_energy_j_m2,
            inputs.vapor_mass_kg_m2,
            inputs.latent_energy_j_m2,
            inputs.shortwave_energy_j_m2,
            inputs.net_longwave_energy_j_m2,
            inputs.precipitation_advection_j_m2,
            inputs.latent_heat_j_kg,
        ]
        .iter()
        .any(|value| !value.is_finite())
            || inputs.latent_heat_j_kg <= 0.0
            || inputs.beginning_stage3_state_sha256 == Digest32::zero()
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "Stage-3 covered boundary receipt domain",
            ));
        }
        let expected_latent_energy_j_m2 = inputs.vapor_mass_kg_m2 * inputs.latent_heat_j_kg;
        if expected_latent_energy_j_m2.to_bits() != inputs.latent_energy_j_m2.to_bits() {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "Stage-3 covered latent mass-energy identity",
            ));
        }
        inputs.identity.validate()?;
        Ok(Self {
            support: inputs.support,
            sensible_energy_j_m2: inputs.sensible_energy_j_m2,
            vapor_mass_kg_m2: inputs.vapor_mass_kg_m2,
            latent_energy_j_m2: inputs.latent_energy_j_m2,
            shortwave_energy_j_m2: inputs.shortwave_energy_j_m2,
            net_longwave_energy_j_m2: inputs.net_longwave_energy_j_m2,
            precipitation_advection_j_m2: inputs.precipitation_advection_j_m2,
            latent_heat_j_kg: inputs.latent_heat_j_kg,
            beginning_stage3_state_sha256: inputs.beginning_stage3_state_sha256,
            identity: inputs.identity,
        })
    }
}

/// Final accepted covered canopy/snow boundary for one keyed LSE destination.
///
/// The digest is sealed only after the optical and reciprocal-longwave
/// corrections are known. Stage 3 and the V11 parent retain this digest as a
/// join rather than continuing to identify corrected values by a provisional
/// carrier receipt.
#[derive(Clone, Debug, PartialEq)]
pub struct FinalStage3CanopyBoundaryReceiptV1 {
    pub support: TimeSupport,
    pub destination: (OfeId, TileId),
    pub beginning_v11_state_sha256: Digest32,
    pub beginning_stage3_state_sha256: Digest32,
    pub ending_v11_state_sha256: Digest32,
    pub ending_stage3_state_sha256: Digest32,
    pub provisional_carrier_receipt_sha256: Digest32,
    pub optical_receipt_sha256: Digest32,
    pub reciprocal_longwave_receipt_sha256: Digest32,
    pub sensible_to_canopy_air_w_m2: f64,
    pub vapor_to_canopy_air_kg_m2_s: f64,
    pub latent_energy_to_canopy_air_j_m2: f64,
    pub snow_temperature_k: f64,
    pub latent_heat_j_kg: f64,
    pub snow_absorbed_shortwave_w_m2: f64,
    pub snow_net_longwave_w_m2: f64,
    pub receipt_sha256: Digest32,
}

impl FinalStage3CanopyBoundaryReceiptV1 {
    pub fn try_new(
        inputs: FinalStage3CanopyBoundaryReceiptInputs,
    ) -> Result<Self, SnowStage3HandoffError> {
        if [
            inputs.sensible_to_canopy_air_w_m2,
            inputs.vapor_to_canopy_air_kg_m2_s,
            inputs.latent_energy_to_canopy_air_j_m2,
            inputs.snow_temperature_k,
            inputs.latent_heat_j_kg,
            inputs.snow_absorbed_shortwave_w_m2,
            inputs.snow_net_longwave_w_m2,
        ]
        .iter()
        .any(|value| !value.is_finite())
            || inputs.beginning_v11_state_sha256 == Digest32::zero()
            || inputs.beginning_stage3_state_sha256 == Digest32::zero()
            || inputs.ending_v11_state_sha256 == Digest32::zero()
            || inputs.ending_stage3_state_sha256 == Digest32::zero()
            || inputs.provisional_carrier_receipt_sha256 == Digest32::zero()
            || inputs.optical_receipt_sha256 == Digest32::zero()
            || inputs.reciprocal_longwave_receipt_sha256 == Digest32::zero()
            || inputs.latent_heat_j_kg <= 0.0
            || !(200.0..=350.0).contains(&inputs.snow_temperature_k)
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "final covered canopy boundary receipt domain",
            ));
        }
        let duration_s = f64::from_bits(inputs.support.duration_s_bits());
        let expected_latent_energy =
            inputs.vapor_to_canopy_air_kg_m2_s * inputs.latent_heat_j_kg * duration_s;
        if expected_latent_energy.to_bits() != inputs.latent_energy_to_canopy_air_j_m2.to_bits() {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "final covered latent mass-energy identity",
            ));
        }
        let receipt_sha256 = final_canopy_boundary_receipt_digest(&inputs);
        Ok(Self {
            support: inputs.support,
            destination: inputs.destination,
            beginning_v11_state_sha256: inputs.beginning_v11_state_sha256,
            beginning_stage3_state_sha256: inputs.beginning_stage3_state_sha256,
            ending_v11_state_sha256: inputs.ending_v11_state_sha256,
            ending_stage3_state_sha256: inputs.ending_stage3_state_sha256,
            provisional_carrier_receipt_sha256: inputs.provisional_carrier_receipt_sha256,
            optical_receipt_sha256: inputs.optical_receipt_sha256,
            reciprocal_longwave_receipt_sha256: inputs.reciprocal_longwave_receipt_sha256,
            sensible_to_canopy_air_w_m2: inputs.sensible_to_canopy_air_w_m2,
            vapor_to_canopy_air_kg_m2_s: inputs.vapor_to_canopy_air_kg_m2_s,
            latent_energy_to_canopy_air_j_m2: inputs.latent_energy_to_canopy_air_j_m2,
            snow_temperature_k: inputs.snow_temperature_k,
            latent_heat_j_kg: inputs.latent_heat_j_kg,
            snow_absorbed_shortwave_w_m2: inputs.snow_absorbed_shortwave_w_m2,
            snow_net_longwave_w_m2: inputs.snow_net_longwave_w_m2,
            receipt_sha256,
        })
    }

    pub fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        let inputs = FinalStage3CanopyBoundaryReceiptInputs {
            support: self.support,
            destination: self.destination.clone(),
            beginning_v11_state_sha256: self.beginning_v11_state_sha256,
            beginning_stage3_state_sha256: self.beginning_stage3_state_sha256,
            ending_v11_state_sha256: self.ending_v11_state_sha256,
            ending_stage3_state_sha256: self.ending_stage3_state_sha256,
            provisional_carrier_receipt_sha256: self.provisional_carrier_receipt_sha256,
            optical_receipt_sha256: self.optical_receipt_sha256,
            reciprocal_longwave_receipt_sha256: self.reciprocal_longwave_receipt_sha256,
            sensible_to_canopy_air_w_m2: self.sensible_to_canopy_air_w_m2,
            vapor_to_canopy_air_kg_m2_s: self.vapor_to_canopy_air_kg_m2_s,
            latent_energy_to_canopy_air_j_m2: self.latent_energy_to_canopy_air_j_m2,
            snow_temperature_k: self.snow_temperature_k,
            latent_heat_j_kg: self.latent_heat_j_kg,
            snow_absorbed_shortwave_w_m2: self.snow_absorbed_shortwave_w_m2,
            snow_net_longwave_w_m2: self.snow_net_longwave_w_m2,
        };
        if final_canopy_boundary_receipt_digest(&inputs) != self.receipt_sha256 {
            return Err(SnowStage3HandoffError::InvalidState(
                "final covered canopy boundary receipt digest",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalStage3CanopyBoundaryReceiptInputs {
    pub support: TimeSupport,
    pub destination: (OfeId, TileId),
    pub beginning_v11_state_sha256: Digest32,
    pub beginning_stage3_state_sha256: Digest32,
    pub ending_v11_state_sha256: Digest32,
    pub ending_stage3_state_sha256: Digest32,
    pub provisional_carrier_receipt_sha256: Digest32,
    pub optical_receipt_sha256: Digest32,
    pub reciprocal_longwave_receipt_sha256: Digest32,
    pub sensible_to_canopy_air_w_m2: f64,
    pub vapor_to_canopy_air_kg_m2_s: f64,
    pub latent_energy_to_canopy_air_j_m2: f64,
    pub snow_temperature_k: f64,
    pub latent_heat_j_kg: f64,
    pub snow_absorbed_shortwave_w_m2: f64,
    pub snow_net_longwave_w_m2: f64,
}

fn final_canopy_boundary_receipt_digest(
    inputs: &FinalStage3CanopyBoundaryReceiptInputs,
) -> Digest32 {
    let mut bytes = Vec::with_capacity(512);
    bytes.extend_from_slice(b"OPENWEPP_FINAL_STAGE3_CANOPY_BOUNDARY_V1\0");
    bytes.extend_from_slice(&inputs.support.start_ns().get().to_le_bytes());
    bytes.extend_from_slice(&inputs.support.end_ns().get().to_le_bytes());
    append_framed_str(&mut bytes, inputs.destination.0.as_str());
    append_framed_str(&mut bytes, inputs.destination.1.as_str());
    for digest in [
        inputs.beginning_v11_state_sha256,
        inputs.beginning_stage3_state_sha256,
        inputs.ending_v11_state_sha256,
        inputs.ending_stage3_state_sha256,
        inputs.provisional_carrier_receipt_sha256,
        inputs.optical_receipt_sha256,
        inputs.reciprocal_longwave_receipt_sha256,
    ] {
        bytes.extend_from_slice(digest.as_bytes());
    }
    for value in [
        inputs.sensible_to_canopy_air_w_m2,
        inputs.vapor_to_canopy_air_kg_m2_s,
        inputs.latent_energy_to_canopy_air_j_m2,
        inputs.snow_temperature_k,
        inputs.latent_heat_j_kg,
        inputs.snow_absorbed_shortwave_w_m2,
        inputs.snow_net_longwave_w_m2,
    ] {
        bytes.extend_from_slice(&value.to_bits().to_le_bytes());
    }
    digest_bytes(&bytes)
}

fn append_framed_str(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Stage3LaneAreaBasisV1 {
    OfeGround,
}

pub const STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE: f64 = 1.0e-12;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Stage3TileBoundaryClassV1 {
    V11CanopyCovered,
    OpenSnow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneBoundaryTopologyExpectationV1 {
    pub tile_id: TileId,
    pub tile_fraction_bits: u64,
    pub boundary_class: Stage3TileBoundaryClassV1,
    pub boundary_model_definition_sha256: Digest32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LaneBoundaryContributionV1 {
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub boundary_class: Stage3TileBoundaryClassV1,
    pub boundary_model_definition_sha256: Digest32,
    pub beginning_stage3_state_sha256: Digest32,
    pub provisional_carrier_receipt_sha256: Digest32,
    pub optical_receipt_sha256: Digest32,
    pub reciprocal_longwave_receipt_sha256: Digest32,
    pub final_boundary_receipt_sha256: Digest32,
    pub sensible_to_canopy_air_w_m2: f64,
    pub vapor_to_canopy_air_kg_m2_s: f64,
    pub latent_energy_to_canopy_air_j_m2: f64,
    pub snow_absorbed_shortwave_w_m2: f64,
    pub snow_net_longwave_w_m2: f64,
    pub snow_temperature_k: f64,
    pub latent_heat_j_kg: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LaneStage3BoundaryReceiptV1 {
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub support: TimeSupport,
    pub area_basis: Stage3LaneAreaBasisV1,
    pub topology_configuration_sha256: Digest32,
    pub provisional_carrier_receipt_sha256: Digest32,
    pub optical_receipt_sha256: Digest32,
    pub reciprocal_longwave_receipt_sha256: Digest32,
    pub final_destination_receipt_sha256: Digest32,
    pub ordered_destinations: Vec<LaneBoundaryContributionV1>,
    pub aggregate_sensible_to_canopy_air_w_m2: f64,
    pub aggregate_vapor_to_canopy_air_kg_m2_s: f64,
    pub aggregate_latent_energy_to_canopy_air_j_m2: f64,
    pub aggregate_snow_absorbed_shortwave_w_m2: f64,
    pub aggregate_snow_net_longwave_w_m2: f64,
    pub aggregate_snow_temperature_k: f64,
    pub aggregate_latent_heat_j_kg: f64,
    pub receipt_sha256: Digest32,
}

impl LaneStage3BoundaryReceiptV1 {
    pub fn try_new(
        mut value: Self,
        expected_topology: &[LaneBoundaryTopologyExpectationV1],
    ) -> Result<Self, SnowStage3HandoffError> {
        value.receipt_sha256 = Digest32::zero();
        value.seal_source_set_digests();
        value.validate_body()?;
        value.validate_topology(expected_topology)?;
        value.receipt_sha256 = lane_stage3_boundary_receipt_digest(&value);
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        self.validate_body()?;
        let mut body = self.clone();
        body.receipt_sha256 = Digest32::zero();
        if lane_stage3_boundary_receipt_digest(&body) != self.receipt_sha256 {
            return Err(SnowStage3HandoffError::InvalidState(
                "lane Stage-3 boundary receipt digest",
            ));
        }
        Ok(())
    }

    fn validate_body(&self) -> Result<(), SnowStage3HandoffError> {
        if self.ofe_id.as_str().is_empty()
            || self.topology_configuration_sha256 == Digest32::zero()
            || self.provisional_carrier_receipt_sha256 == Digest32::zero()
            || self.optical_receipt_sha256 == Digest32::zero()
            || self.reciprocal_longwave_receipt_sha256 == Digest32::zero()
            || self.final_destination_receipt_sha256 == Digest32::zero()
            || self.ordered_destinations.is_empty()
            || self
                .ordered_destinations
                .windows(2)
                .any(|pair| pair[0].tile_id >= pair[1].tile_id)
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "lane Stage-3 boundary topology",
            ));
        }
        let mut sum = 0.0;
        let mut aggregates = [0.0; 7];
        let common =
            self.ordered_destinations
                .first()
                .ok_or(SnowStage3HandoffError::InvalidCarrier(
                    "lane Stage-3 boundary topology",
                ))?;
        let duration_s = f64::from_bits(self.support.duration_s_bits());
        for contribution in &self.ordered_destinations {
            if contribution.final_boundary_receipt_sha256 == Digest32::zero()
                || contribution.boundary_model_definition_sha256 == Digest32::zero()
                || contribution.beginning_stage3_state_sha256 == Digest32::zero()
                || contribution.provisional_carrier_receipt_sha256 == Digest32::zero()
                || contribution.optical_receipt_sha256 == Digest32::zero()
                || contribution.reciprocal_longwave_receipt_sha256 == Digest32::zero()
                || !contribution.tile_fraction.is_finite()
                || contribution.tile_fraction <= 0.0
                || contribution.tile_fraction > 1.0
                || ![
                    contribution.sensible_to_canopy_air_w_m2,
                    contribution.vapor_to_canopy_air_kg_m2_s,
                    contribution.latent_energy_to_canopy_air_j_m2,
                    contribution.snow_absorbed_shortwave_w_m2,
                    contribution.snow_net_longwave_w_m2,
                    contribution.snow_temperature_k,
                    contribution.latent_heat_j_kg,
                ]
                .iter()
                .all(|value| value.is_finite())
                || contribution.latent_heat_j_kg <= 0.0
                || !(200.0..=350.0).contains(&contribution.snow_temperature_k)
            {
                return Err(SnowStage3HandoffError::InvalidCarrier(
                    "lane Stage-3 boundary contribution domain",
                ));
            }
            if contribution.beginning_stage3_state_sha256 != common.beginning_stage3_state_sha256
                || contribution.snow_temperature_k.to_bits() != common.snow_temperature_k.to_bits()
                || contribution.latent_heat_j_kg.to_bits() != common.latent_heat_j_kg.to_bits()
            {
                return Err(SnowStage3HandoffError::InvalidCarrier(
                    "lane Stage-3 boundary common snow state",
                ));
            }
            let expected_latent_energy = contribution.vapor_to_canopy_air_kg_m2_s
                * contribution.latent_heat_j_kg
                * duration_s;
            if expected_latent_energy.to_bits()
                != contribution.latent_energy_to_canopy_air_j_m2.to_bits()
            {
                return Err(SnowStage3HandoffError::InvalidCarrier(
                    "lane Stage-3 contribution latent mass-energy identity",
                ));
            }
            sum += contribution.tile_fraction;
            for (index, value) in [
                contribution.sensible_to_canopy_air_w_m2,
                contribution.vapor_to_canopy_air_kg_m2_s,
                contribution.latent_energy_to_canopy_air_j_m2,
                contribution.snow_absorbed_shortwave_w_m2,
                contribution.snow_net_longwave_w_m2,
                contribution.snow_temperature_k,
                contribution.latent_heat_j_kg,
            ]
            .into_iter()
            .enumerate()
            {
                aggregates[index] += contribution.tile_fraction * value;
            }
        }
        if !sum.is_finite() || (sum - 1.0).abs() > STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "lane Stage-3 snow-surface fractions must close on OFE ground",
            ));
        }
        aggregates[5] = common.snow_temperature_k;
        aggregates[6] = common.latent_heat_j_kg;
        let expected = [
            self.aggregate_sensible_to_canopy_air_w_m2,
            self.aggregate_vapor_to_canopy_air_kg_m2_s,
            self.aggregate_latent_energy_to_canopy_air_j_m2,
            self.aggregate_snow_absorbed_shortwave_w_m2,
            self.aggregate_snow_net_longwave_w_m2,
            self.aggregate_snow_temperature_k,
        ];
        if expected
            .iter()
            .zip(aggregates.into_iter().take(6))
            .any(|(actual, reconstructed)| actual.to_bits() != reconstructed.to_bits())
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "lane Stage-3 boundary aggregate reconstruction",
            ));
        }
        if self.aggregate_latent_heat_j_kg <= 0.0 {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "lane Stage-3 aggregate latent heat",
            ));
        }
        if self.aggregate_latent_heat_j_kg.to_bits() != common.latent_heat_j_kg.to_bits() {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "lane Stage-3 aggregate latent mass-energy identity",
            ));
        }
        let expected_latent_energy =
            self.aggregate_vapor_to_canopy_air_kg_m2_s * common.latent_heat_j_kg * duration_s;
        if expected_latent_energy.to_bits()
            != self.aggregate_latent_energy_to_canopy_air_j_m2.to_bits()
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "lane Stage-3 aggregate latent mass-energy identity",
            ));
        }
        let expected_source_sets = lane_source_set_digests(&self.ordered_destinations);
        if self.provisional_carrier_receipt_sha256 != expected_source_sets[0]
            || self.optical_receipt_sha256 != expected_source_sets[1]
            || self.reciprocal_longwave_receipt_sha256 != expected_source_sets[2]
            || self.final_destination_receipt_sha256 != expected_source_sets[3]
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "lane Stage-3 source receipt-set reconstruction",
            ));
        }
        Ok(())
    }

    pub fn validate_topology(
        &self,
        expected: &[LaneBoundaryTopologyExpectationV1],
    ) -> Result<(), SnowStage3HandoffError> {
        if expected.len() != self.ordered_destinations.len()
            || expected
                .iter()
                .zip(&self.ordered_destinations)
                .any(|(authority, contribution)| {
                    authority.tile_id != contribution.tile_id
                        || authority.tile_fraction_bits != contribution.tile_fraction.to_bits()
                        || authority.boundary_class != contribution.boundary_class
                        || authority.boundary_model_definition_sha256
                            != contribution.boundary_model_definition_sha256
                })
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "lane Stage-3 boundary topology authority join",
            ));
        }
        Ok(())
    }

    fn seal_source_set_digests(&mut self) {
        let digests = lane_source_set_digests(&self.ordered_destinations);
        self.provisional_carrier_receipt_sha256 = digests[0];
        self.optical_receipt_sha256 = digests[1];
        self.reciprocal_longwave_receipt_sha256 = digests[2];
        self.final_destination_receipt_sha256 = digests[3];
    }
}

fn lane_source_set_digests(contributions: &[LaneBoundaryContributionV1]) -> [Digest32; 4] {
    std::array::from_fn(|source_index| {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"OPENWEPP_LANE_STAGE3_SOURCE_SET_V1\0");
        bytes.push(source_index as u8);
        bytes.extend_from_slice(&(contributions.len() as u64).to_le_bytes());
        for contribution in contributions {
            append_framed_str(&mut bytes, contribution.tile_id.as_str());
            bytes.extend_from_slice(&contribution.tile_fraction.to_bits().to_le_bytes());
            bytes.push(match contribution.boundary_class {
                Stage3TileBoundaryClassV1::V11CanopyCovered => 0,
                Stage3TileBoundaryClassV1::OpenSnow => 1,
            });
            bytes.extend_from_slice(contribution.boundary_model_definition_sha256.as_bytes());
            let source = match source_index {
                0 => contribution.provisional_carrier_receipt_sha256,
                1 => contribution.optical_receipt_sha256,
                2 => contribution.reciprocal_longwave_receipt_sha256,
                _ => contribution.final_boundary_receipt_sha256,
            };
            bytes.extend_from_slice(source.as_bytes());
        }
        digest_bytes(&bytes)
    })
}

fn lane_stage3_boundary_receipt_digest(value: &LaneStage3BoundaryReceiptV1) -> Digest32 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OPENWEPP_LANE_STAGE3_BOUNDARY_RECEIPT_V1\0");
    bytes.extend_from_slice(&value.lane_id.to_le_bytes());
    append_framed_str(&mut bytes, value.ofe_id.as_str());
    bytes.extend_from_slice(&value.support.start_ns().get().to_le_bytes());
    bytes.extend_from_slice(&value.support.end_ns().get().to_le_bytes());
    bytes.push(0); // Stage3LaneAreaBasisV1::OfeGround
    for digest in [
        value.topology_configuration_sha256,
        value.provisional_carrier_receipt_sha256,
        value.optical_receipt_sha256,
        value.reciprocal_longwave_receipt_sha256,
        value.final_destination_receipt_sha256,
    ] {
        bytes.extend_from_slice(digest.as_bytes());
    }
    bytes.extend_from_slice(&(value.ordered_destinations.len() as u64).to_le_bytes());
    for contribution in &value.ordered_destinations {
        append_framed_str(&mut bytes, contribution.tile_id.as_str());
        bytes.extend_from_slice(&contribution.tile_fraction.to_bits().to_le_bytes());
        bytes.push(match contribution.boundary_class {
            Stage3TileBoundaryClassV1::V11CanopyCovered => 0,
            Stage3TileBoundaryClassV1::OpenSnow => 1,
        });
        bytes.extend_from_slice(contribution.boundary_model_definition_sha256.as_bytes());
        bytes.extend_from_slice(contribution.beginning_stage3_state_sha256.as_bytes());
        bytes.extend_from_slice(contribution.provisional_carrier_receipt_sha256.as_bytes());
        bytes.extend_from_slice(contribution.optical_receipt_sha256.as_bytes());
        bytes.extend_from_slice(contribution.reciprocal_longwave_receipt_sha256.as_bytes());
        bytes.extend_from_slice(contribution.final_boundary_receipt_sha256.as_bytes());
        for scalar in [
            contribution.sensible_to_canopy_air_w_m2,
            contribution.vapor_to_canopy_air_kg_m2_s,
            contribution.latent_energy_to_canopy_air_j_m2,
            contribution.snow_absorbed_shortwave_w_m2,
            contribution.snow_net_longwave_w_m2,
            contribution.snow_temperature_k,
            contribution.latent_heat_j_kg,
        ] {
            bytes.extend_from_slice(&scalar.to_bits().to_le_bytes());
        }
    }
    for scalar in [
        value.aggregate_sensible_to_canopy_air_w_m2,
        value.aggregate_vapor_to_canopy_air_kg_m2_s,
        value.aggregate_latent_energy_to_canopy_air_j_m2,
        value.aggregate_snow_absorbed_shortwave_w_m2,
        value.aggregate_snow_net_longwave_w_m2,
        value.aggregate_snow_temperature_k,
        value.aggregate_latent_heat_j_kg,
    ] {
        bytes.extend_from_slice(&scalar.to_bits().to_le_bytes());
    }
    digest_bytes(&bytes)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SharedCarrierReceipt {
    pub active_participants: Vec<String>,
    pub common_minimum_support_ns: ModelTimeNs,
    pub exposure_receipt_id: String,
    pub shared_air_temperature_k: f64,
    pub shared_air_specific_humidity: f64,
    pub snow_temperature_k: f64,
    pub reference_sensible_into_node_w_m2: f64,
    pub canopy_sensible_into_surface_w_m2: f64,
    pub snow_sensible_into_surface_w_m2: f64,
    pub reference_vapor_into_node_kg_m2_s: f64,
    pub canopy_vapor_into_surface_kg_m2_s: f64,
    pub snow_vapor_into_surface_kg_m2_s: f64,
    pub sky_view_fraction: f64,
    pub snow_longwave_net_w_m2: f64,
    pub snow_canopy_longwave_exchange_w_m2: f64,
    pub snow_ice_end_kg_m2: f64,
    pub liquid_end_kg_m2: f64,
    pub vapor_net_kg_m2: f64,
    pub energy_closure_j_m2: f64,
    pub longwave_reciprocal_closure_j_m2: f64,
    pub receipt_id: Digest32,
}

fn serialized_digest<T: Serialize>(value: &T) -> Result<Digest32, SnowStage3HandoffError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| SnowStage3HandoffError::InvalidState("canonical receipt serialization"))?;
    Ok(digest_bytes(&bytes))
}

fn validate_participants(
    participants: &[String],
    receipts: &[ParticipantSupportReceipt],
) -> Result<ModelTimeNs, SnowStage3HandoffError> {
    if participants.windows(2).any(|pair| pair[0] >= pair[1])
        || participants.iter().any(String::is_empty)
        || receipts.len() != participants.len()
    {
        return Err(SnowStage3HandoffError::InvalidCarrier(
            "active participants must be nonempty, sorted, and unique",
        ));
    }
    let mut seen_receipt_ids = BTreeSet::new();
    let mut maximum = 0_u128;
    for (participant, receipt) in participants.iter().zip(receipts) {
        receipt.validate()?;
        if participant != &receipt.participant_id
            || !seen_receipt_ids.insert(&receipt.support_receipt_id)
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "support receipts must join one-to-one with participants",
            ));
        }
        maximum = maximum.max(receipt.minimum_support_ns.get());
    }
    Ok(ModelTimeNs::new(maximum))
}

#[allow(clippy::too_many_lines)]
pub fn evaluate_shared_carrier(
    input: &SharedCarrierInput,
) -> Result<SharedCarrierReceipt, SnowStage3HandoffError> {
    if input.phase != SegmentPhase::SnowCovered {
        return Err(SnowStage3HandoffError::InvalidRegime(
            "shared carrier is only admitted in the snow-covered segment",
        ));
    }
    if input.canopy_intercepted_snow {
        return Err(SnowStage3HandoffError::CanopyInterceptedSnow);
    }
    input.exposure.validate()?;
    input.reference.validate("reference carrier surface")?;
    input.canopy.validate("canopy carrier surface")?;
    input.snow.validate("snow carrier surface")?;
    input.ledger.validate()?;
    if input.active_participants.is_empty() {
        return Err(SnowStage3HandoffError::InvalidCarrier(
            "shared carrier requires active participants",
        ));
    }
    let common_support =
        validate_participants(&input.active_participants, &input.support_receipts)?;
    if !input.rho_air_kg_m3.is_finite()
        || input.rho_air_kg_m3 <= 0.0
        || !input.cp_air_j_kg_k.is_finite()
        || input.cp_air_j_kg_k <= 0.0
        || !input.atmospheric_longwave_w_m2.is_finite()
        || !input.effective_canopy_cover.is_finite()
        || input.effective_canopy_cover < 0.0
        || input.effective_canopy_cover >= 1.0
    {
        return Err(SnowStage3HandoffError::InvalidCarrier(
            "carrier atmosphere and cover domain",
        ));
    }
    if input.canopy_longwave_components.len() < 2 {
        return Err(SnowStage3HandoffError::InvalidLongwave(
            "at least leaf and stem components are required",
        ));
    }
    let weight_sum = input
        .canopy_longwave_components
        .iter()
        .try_fold(0.0, |sum, component| {
            if !component.temperature_k.is_finite()
                || component.temperature_k <= 0.0
                || !component.emissive_area_weight.is_finite()
                || component.emissive_area_weight <= 0.0
            {
                return Err(SnowStage3HandoffError::InvalidLongwave(
                    "longwave component domain",
                ));
            }
            Ok(sum + component.emissive_area_weight)
        })?;
    if (weight_sum - 1.0).abs() > 1.0e-12 {
        return Err(SnowStage3HandoffError::InvalidLongwave(
            "longwave component weights must sum to one",
        ));
    }

    let heat_total = input.reference.heat_conductance_m_s
        + input.canopy.heat_conductance_m_s
        + input.snow.heat_conductance_m_s;
    let vapor_total = input.reference.vapor_conductance_m_s
        + input.canopy.vapor_conductance_m_s
        + input.snow.vapor_conductance_m_s;
    let shared_temperature = (input.reference.heat_conductance_m_s * input.reference.temperature_k
        + input.canopy.heat_conductance_m_s * input.canopy.temperature_k
        + input.snow.heat_conductance_m_s * input.snow.temperature_k)
        / heat_total;
    let shared_humidity = (input.reference.vapor_conductance_m_s
        * input.reference.specific_humidity
        + input.canopy.vapor_conductance_m_s * input.canopy.specific_humidity
        + input.snow.vapor_conductance_m_s * input.snow.specific_humidity)
        / vapor_total;
    let reference_sensible = input.rho_air_kg_m3
        * input.cp_air_j_kg_k
        * input.reference.heat_conductance_m_s
        * (input.reference.temperature_k - shared_temperature);
    let canopy_sensible = -input.rho_air_kg_m3
        * input.cp_air_j_kg_k
        * input.canopy.heat_conductance_m_s
        * (input.canopy.temperature_k - shared_temperature);
    let snow_sensible = -input.rho_air_kg_m3
        * input.cp_air_j_kg_k
        * input.snow.heat_conductance_m_s
        * (input.snow.temperature_k - shared_temperature);
    let reference_vapor = input.rho_air_kg_m3
        * input.reference.vapor_conductance_m_s
        * (input.reference.specific_humidity - shared_humidity);
    let canopy_vapor = -input.rho_air_kg_m3
        * input.canopy.vapor_conductance_m_s
        * (input.canopy.specific_humidity - shared_humidity);
    let snow_vapor = -input.rho_air_kg_m3
        * input.snow.vapor_conductance_m_s
        * (input.snow.specific_humidity - shared_humidity);
    let sky_view_fraction = (1.0 - input.effective_canopy_cover).powf(1.6);
    let canopy_longwave = input
        .canopy_longwave_components
        .iter()
        .map(|component| {
            component.emissive_area_weight * SIGMA_W_M2_K4 * component.temperature_k.powi(4)
        })
        .sum::<f64>();
    let snow_emission = SIGMA_W_M2_K4 * input.snow.temperature_k.powi(4);
    let snow_down = sky_view_fraction * input.atmospheric_longwave_w_m2
        + (1.0 - sky_view_fraction) * canopy_longwave;
    let snow_longwave_net = snow_down - snow_emission;
    let snow_canopy_exchange = (1.0 - sky_view_fraction) * (snow_emission - canopy_longwave);
    let expected_snow_canopy_exchange_j_m2 = -snow_canopy_exchange * input.ledger.duration_s;
    if (input.ledger.snow_canopy_longwave_exchange_j_m2 - expected_snow_canopy_exchange_j_m2).abs()
        > 1.0e-6
        || (input.ledger.canopy_snow_longwave_exchange_j_m2 + expected_snow_canopy_exchange_j_m2)
            .abs()
            > 1.0e-6
    {
        return Err(SnowStage3HandoffError::InvalidLedger(
            "reciprocal longwave ledger does not match the shared-carrier exchange",
        ));
    }
    let temperature_residual = reference_sensible - snow_sensible - canopy_sensible;
    let vapor_residual = reference_vapor - snow_vapor - canopy_vapor;
    if temperature_residual.abs() > CLOSURE_TOLERANCE || vapor_residual.abs() > CLOSURE_TOLERANCE {
        return Err(SnowStage3HandoffError::InvalidCarrier(
            "shared carrier residual does not close",
        ));
    }
    let snow_ice_end = input.ledger.snow_ice_start_kg_m2 + input.ledger.solid_precipitation_kg_m2
        - input.ledger.melt_kg_m2
        - input.ledger.sublimation_kg_m2
        + input.ledger.deposition_kg_m2;
    let liquid_end =
        input.ledger.liquid_start_kg_m2 + input.ledger.rain_kg_m2 + input.ledger.melt_kg_m2
            - input.ledger.refreeze_kg_m2
            - input.ledger.liquid_runoff_kg_m2;
    let mut receipt = SharedCarrierReceipt {
        active_participants: input.active_participants.clone(),
        common_minimum_support_ns: common_support,
        exposure_receipt_id: input.exposure.receipt_id.clone(),
        shared_air_temperature_k: shared_temperature,
        shared_air_specific_humidity: shared_humidity,
        snow_temperature_k: input.snow.temperature_k,
        reference_sensible_into_node_w_m2: reference_sensible,
        canopy_sensible_into_surface_w_m2: canopy_sensible,
        snow_sensible_into_surface_w_m2: snow_sensible,
        reference_vapor_into_node_kg_m2_s: reference_vapor,
        canopy_vapor_into_surface_kg_m2_s: canopy_vapor,
        snow_vapor_into_surface_kg_m2_s: snow_vapor,
        sky_view_fraction,
        snow_longwave_net_w_m2: snow_longwave_net,
        snow_canopy_longwave_exchange_w_m2: snow_canopy_exchange,
        snow_ice_end_kg_m2: snow_ice_end,
        liquid_end_kg_m2: liquid_end,
        vapor_net_kg_m2: input.ledger.deposition_kg_m2 - input.ledger.sublimation_kg_m2,
        energy_closure_j_m2: input.ledger.external_energy_j_m2
            + input.ledger.canopy_energy_j_m2
            + input.ledger.snow_energy_j_m2
            - (input.ledger.energy_end_j_m2 - input.ledger.energy_start_j_m2),
        longwave_reciprocal_closure_j_m2: input.ledger.canopy_snow_longwave_exchange_j_m2
            + input.ledger.snow_canopy_longwave_exchange_j_m2,
        receipt_id: Digest32::zero(),
    };
    receipt.receipt_id = serialized_digest(&receipt)?;
    Ok(receipt)
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct TerminalStateRates {
    pub snow_start_kg_m2: f64,
    pub snow_rate_kg_m2_s: f64,
    pub snow_target_kg_m2: f64,
    pub liquid_start_kg_m2: f64,
    pub liquid_rate_kg_m2_s: f64,
    pub liquid_target_kg_m2: f64,
    pub energy_start_j_m2: f64,
    pub energy_rate_j_m2_s: f64,
    pub energy_target_j_m2: f64,
}

impl TerminalStateRates {
    fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        let values = [
            self.snow_start_kg_m2,
            self.snow_rate_kg_m2_s,
            self.snow_target_kg_m2,
            self.liquid_start_kg_m2,
            self.liquid_rate_kg_m2_s,
            self.liquid_target_kg_m2,
            self.energy_start_j_m2,
            self.energy_rate_j_m2_s,
            self.energy_target_j_m2,
        ];
        if values.iter().any(|value| !value.is_finite()) {
            return Err(SnowStage3HandoffError::InvalidState(
                "terminal state rates must be finite",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TerminalEventInput {
    pub parent_identity: String,
    pub segment_identity: String,
    pub event_ordinal: u64,
    pub parent_start_tick: ModelTimeNs,
    pub parent_end_tick: ModelTimeNs,
    pub proposed_event_tick: ModelTimeNs,
    pub candidate_ticks: Vec<ModelTimeNs>,
    pub pre_active_participants: Vec<ParticipantSupportReceipt>,
    pub post_active_participants: Vec<ParticipantSupportReceipt>,
    pub event_time_tolerance_ns: ModelTimeNs,
    pub snow_mass_tolerance_kg_m2: f64,
    pub liquid_mass_tolerance_kg_m2: f64,
    pub energy_tolerance_j_m2: f64,
    pub terminal_state: TerminalStateRates,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TerminalCandidateEvaluation {
    pub tick: ModelTimeNs,
    pub support_admissible: bool,
    pub event_time_error_ns: ModelTimeNs,
    pub snow_mass_error_kg_m2: f64,
    pub liquid_mass_error_kg_m2: f64,
    pub energy_error_j_m2: f64,
    pub combined_normalized_error: Option<f64>,
    pub accepted: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TerminalEventReceipt {
    pub parent_identity: String,
    pub segment_identity: String,
    pub event_ordinal: u64,
    pub candidate_set_digest: Digest32,
    pub accepted_tie_rank: Option<u32>,
    pub accepted_event_tick: Option<ModelTimeNs>,
    pub proposed_event_tick: ModelTimeNs,
    pub pre_active_participants: Vec<ParticipantSupportReceipt>,
    pub post_active_participants: Vec<ParticipantSupportReceipt>,
    pub pre_common_minimum_support_ns: ModelTimeNs,
    pub post_common_minimum_support_ns: ModelTimeNs,
    pub candidate_evaluations: Vec<TerminalCandidateEvaluation>,
    pub event_time_error_ns: Option<ModelTimeNs>,
    pub snow_mass_error_kg_m2: Option<f64>,
    pub liquid_mass_error_kg_m2: Option<f64>,
    pub energy_error_j_m2: Option<f64>,
    pub combined_normalized_error: Option<f64>,
    pub receipt_id: Digest32,
}

fn normalized_error(error: f64, tolerance: f64) -> Option<f64> {
    if tolerance == 0.0 {
        (error == 0.0).then_some(0.0)
    } else {
        Some(error / tolerance)
    }
}

#[derive(Serialize)]
struct TerminalCandidateSetDigestInput<'a> {
    parent_identity: &'a str,
    segment_identity: &'a str,
    event_ordinal: u64,
    candidate_ticks: &'a [ModelTimeNs],
    pre_active_participants: &'a [ParticipantSupportReceipt],
    post_active_participants: &'a [ParticipantSupportReceipt],
    event_time_tolerance_ns: ModelTimeNs,
    snow_mass_tolerance_kg_m2: f64,
    liquid_mass_tolerance_kg_m2: f64,
    energy_tolerance_j_m2: f64,
    terminal_state: TerminalStateRates,
}

#[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
pub fn locate_terminal_event(
    input: &TerminalEventInput,
) -> Result<TerminalEventReceipt, SnowStage3HandoffError> {
    if input.parent_start_tick > input.parent_end_tick
        || input.parent_identity.is_empty()
        || input.segment_identity.is_empty()
        || input.proposed_event_tick < input.parent_start_tick
        || input.proposed_event_tick > input.parent_end_tick
        || input.event_time_tolerance_ns.get()
            > input.parent_end_tick.get() - input.parent_start_tick.get()
        || !input.snow_mass_tolerance_kg_m2.is_finite()
        || input.snow_mass_tolerance_kg_m2 < 0.0
        || !input.liquid_mass_tolerance_kg_m2.is_finite()
        || input.liquid_mass_tolerance_kg_m2 < 0.0
        || !input.energy_tolerance_j_m2.is_finite()
        || input.energy_tolerance_j_m2 < 0.0
    {
        return Err(SnowStage3HandoffError::InvalidState(
            "terminal event parent and tolerance domain",
        ));
    }
    input.terminal_state.validate()?;
    if input
        .candidate_ticks
        .windows(2)
        .any(|pair| pair[0] >= pair[1])
    {
        return Err(SnowStage3HandoffError::InvalidState(
            "candidate ticks must be strictly increasing canonical decimals",
        ));
    }
    let pre_support = validate_participants(
        &input
            .pre_active_participants
            .iter()
            .map(|receipt| receipt.participant_id.clone())
            .collect::<Vec<_>>(),
        &input.pre_active_participants,
    )?;
    let post_support = validate_participants(
        &input
            .post_active_participants
            .iter()
            .map(|receipt| receipt.participant_id.clone())
            .collect::<Vec<_>>(),
        &input.post_active_participants,
    )?;
    let mut evaluations = Vec::with_capacity(input.candidate_ticks.len());
    let mut selected: Option<(ModelTimeNs, f64, ModelTimeNs, f64, f64, f64, u32)> = None;
    for (candidate_index, &tick) in input.candidate_ticks.iter().enumerate() {
        if tick < input.parent_start_tick || tick > input.parent_end_tick {
            continue;
        }
        // Model ticks are bounded by the admitted parent interval. The event
        // equations are f64 by contract, so this is the single explicit
        // integer-to-real projection at their boundary.
        let elapsed_s = (tick.get() - input.parent_start_tick.get()) as f64 / 1.0e9;
        let snow = input.terminal_state.snow_start_kg_m2
            + input.terminal_state.snow_rate_kg_m2_s * elapsed_s;
        let liquid = input.terminal_state.liquid_start_kg_m2
            + input.terminal_state.liquid_rate_kg_m2_s * elapsed_s;
        let energy = input.terminal_state.energy_start_j_m2
            + input.terminal_state.energy_rate_j_m2_s * elapsed_s;
        let snow_error = (snow - input.terminal_state.snow_target_kg_m2).abs();
        let liquid_error = (liquid - input.terminal_state.liquid_target_kg_m2).abs();
        let energy_error = (energy - input.terminal_state.energy_target_j_m2).abs();
        let pre_duration = tick.get() - input.parent_start_tick.get();
        let post_duration = input.parent_end_tick.get() - tick.get();
        let support_admissible = (pre_duration == 0 || pre_duration >= pre_support.get())
            && (post_duration == 0 || post_duration >= post_support.get());
        let event_time_error =
            ModelTimeNs::new(tick.get().abs_diff(input.proposed_event_tick.get()));
        let mass_score = normalized_error(snow_error, input.snow_mass_tolerance_kg_m2);
        let liquid_score = normalized_error(liquid_error, input.liquid_mass_tolerance_kg_m2);
        let energy_score = normalized_error(energy_error, input.energy_tolerance_j_m2);
        let tolerance_admissible = mass_score.is_some()
            && liquid_score.is_some()
            && energy_score.is_some()
            && snow_error <= input.snow_mass_tolerance_kg_m2
            && liquid_error <= input.liquid_mass_tolerance_kg_m2
            && energy_error <= input.energy_tolerance_j_m2
            && event_time_error.get() <= input.event_time_tolerance_ns.get();
        let accepted = support_admissible && tolerance_admissible;
        let combined = accepted.then(|| {
            mass_score.unwrap_or(0.0) + liquid_score.unwrap_or(0.0) + energy_score.unwrap_or(0.0)
        });
        evaluations.push(TerminalCandidateEvaluation {
            tick,
            support_admissible,
            event_time_error_ns: event_time_error,
            snow_mass_error_kg_m2: snow_error,
            liquid_mass_error_kg_m2: liquid_error,
            energy_error_j_m2: energy_error,
            combined_normalized_error: combined,
            accepted,
        });
        if let Some(score) = combined {
            let candidate = (
                event_time_error,
                score,
                tick,
                snow_error,
                liquid_error,
                energy_error,
                u32::try_from(candidate_index + 1).map_err(|_| {
                    SnowStage3HandoffError::InvalidState("terminal candidate ordinal overflow")
                })?,
            );
            let replace = selected.as_ref().is_none_or(|current| {
                (candidate.0.get(), candidate.1, candidate.2)
                    < (current.0.get(), current.1, current.2)
            });
            if replace {
                selected = Some(candidate);
            }
        }
    }
    let Some((time_error, combined, tick, snow_error, liquid_error, energy_error, tie_rank)) =
        selected
    else {
        return Err(CoupledTimeError::EventBoundaryNoCandidate.into());
    };
    let mut receipt = TerminalEventReceipt {
        parent_identity: input.parent_identity.clone(),
        segment_identity: input.segment_identity.clone(),
        event_ordinal: input.event_ordinal,
        candidate_set_digest: serialized_digest(&TerminalCandidateSetDigestInput {
            parent_identity: &input.parent_identity,
            segment_identity: &input.segment_identity,
            event_ordinal: input.event_ordinal,
            candidate_ticks: &input.candidate_ticks,
            pre_active_participants: &input.pre_active_participants,
            post_active_participants: &input.post_active_participants,
            event_time_tolerance_ns: input.event_time_tolerance_ns,
            snow_mass_tolerance_kg_m2: input.snow_mass_tolerance_kg_m2,
            liquid_mass_tolerance_kg_m2: input.liquid_mass_tolerance_kg_m2,
            energy_tolerance_j_m2: input.energy_tolerance_j_m2,
            terminal_state: input.terminal_state,
        })?,
        accepted_tie_rank: Some(tie_rank),
        accepted_event_tick: Some(tick),
        proposed_event_tick: input.proposed_event_tick,
        pre_active_participants: input.pre_active_participants.clone(),
        post_active_participants: input.post_active_participants.clone(),
        pre_common_minimum_support_ns: pre_support,
        post_common_minimum_support_ns: post_support,
        candidate_evaluations: evaluations,
        event_time_error_ns: Some(time_error),
        snow_mass_error_kg_m2: Some(snow_error),
        liquid_mass_error_kg_m2: Some(liquid_error),
        energy_error_j_m2: Some(energy_error),
        combined_normalized_error: Some(combined),
        receipt_id: Digest32::zero(),
    };
    receipt.receipt_id = serialized_digest(&receipt)?;
    Ok(receipt)
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CompleteOwnerSet {
    pub owners: BTreeMap<String, Vec<u8>>,
}

impl CompleteOwnerSet {
    pub fn new(owners: BTreeMap<String, Vec<u8>>) -> Result<Self, SnowStage3HandoffError> {
        let expected = COMPLETE_OWNER_MANIFEST
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let found = owners.keys().map(String::as_str).collect::<BTreeSet<_>>();
        if found != expected || owners.values().any(Vec::is_empty) {
            return Err(SnowStage3HandoffError::InvalidOwnerSet(
                "complete owner manifest or state payload does not match V11",
            ));
        }
        Ok(Self { owners })
    }

    pub fn digest(&self) -> Result<Digest32, SnowStage3HandoffError> {
        serialized_digest(&self.owners)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SnowStage3OwnerExecutionReceipt {
    pub executor_id: String,
    pub ending_owners: CompleteOwnerSet,
    pub owner_state_digests: BTreeMap<String, Digest32>,
    pub v11_state_digest: Digest32,
    pub lse_state_digest: Digest32,
    pub bgc_state_digest: Digest32,
    pub soil_thermal_state_digest: Digest32,
}

impl SnowStage3OwnerExecutionReceipt {
    pub fn from_owner_set(
        executor_id: impl Into<String>,
        ending_owners: CompleteOwnerSet,
    ) -> Result<Self, SnowStage3HandoffError> {
        let owner_state_digests = ending_owners
            .owners
            .iter()
            .map(|(owner_id, state_bytes)| (owner_id.clone(), digest_bytes(state_bytes)))
            .collect::<BTreeMap<_, _>>();
        let v11_state_digest = *owner_state_digests.get("vegetation").ok_or(
            SnowStage3HandoffError::InvalidOwnerSet(
                "owner execution receipt is missing a required owner",
            ),
        )?;
        let lse_state_digest = *owner_state_digests.get("land_surface_energy").ok_or(
            SnowStage3HandoffError::InvalidOwnerSet(
                "owner execution receipt is missing a required owner",
            ),
        )?;
        let bgc_state_digest =
            *owner_state_digests
                .get("bgc")
                .ok_or(SnowStage3HandoffError::InvalidOwnerSet(
                    "owner execution receipt is missing a required owner",
                ))?;
        let soil_thermal_state_digest = *owner_state_digests.get("soil_thermal").ok_or(
            SnowStage3HandoffError::InvalidOwnerSet(
                "owner execution receipt is missing a required owner",
            ),
        )?;
        Ok(Self {
            executor_id: executor_id.into(),
            ending_owners,
            owner_state_digests,
            v11_state_digest,
            lse_state_digest,
            bgc_state_digest,
            soil_thermal_state_digest,
        })
    }

    pub fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        if self.executor_id.is_empty() {
            return Err(SnowStage3HandoffError::InvalidOwnerSet(
                "owner execution receipt has no executor identity",
            ));
        }
        CompleteOwnerSet::new(self.ending_owners.owners.clone())?;
        let expected = self
            .ending_owners
            .owners
            .iter()
            .map(|(owner_id, bytes)| (owner_id.clone(), digest_bytes(bytes)))
            .collect::<BTreeMap<_, _>>();
        if expected != self.owner_state_digests {
            return Err(SnowStage3HandoffError::InvalidOwnerSet(
                "owner execution receipt digest join",
            ));
        }
        if self.v11_state_digest != expected["vegetation"]
            || self.lse_state_digest != expected["land_surface_energy"]
            || self.bgc_state_digest != expected["bgc"]
            || self.soil_thermal_state_digest != expected["soil_thermal"]
        {
            return Err(SnowStage3HandoffError::InvalidOwnerSet(
                "owner execution receipt scientific-owner digest join",
            ));
        }
        Ok(())
    }
}

pub trait SnowStage3OwnerExecutor: Clone {
    type Error: std::fmt::Debug + std::fmt::Display;

    fn stage_owner_execution(
        &mut self,
        request: &SnowStage3TerminalHandoffRequest,
    ) -> Result<SnowStage3OwnerExecutionReceipt, Self::Error>;

    fn commit_owner_execution(&mut self) -> Result<(), Self::Error>;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SnowFreeContinuationInput {
    pub duration_ns: ModelTimeNs,
    pub terminal_liquid_kg_m2: f64,
    pub post_event_contains_snow_operands: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SnowStage3TerminalHandoffRequest {
    pub carrier: SharedCarrierInput,
    pub event: TerminalEventInput,
    pub beginning_owners: CompleteOwnerSet,
    pub ending_owners: CompleteOwnerSet,
    pub owner_execution: SnowStage3OwnerExecutionReceipt,
    pub retained_liquid_kg_m2: f64,
    pub snow_support_rain_kg_m2: f64,
    pub terminal_melt_kg_m2: f64,
    pub terminal_refreeze_kg_m2: f64,
    pub continuation: SnowFreeContinuationInput,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SnowStage3TerminalHandoffReceipt {
    pub parent_identity: String,
    pub segment_identity: String,
    pub event_ordinal: u64,
    pub candidate_set_digest: Digest32,
    pub accepted_tie_rank: u32,
    pub predecessor_receipt_id: Option<Digest32>,
    pub carrier_receipt_id: Digest32,
    pub event_receipt_id: Digest32,
    pub accepted_event_tick: ModelTimeNs,
    pub continuation_duration_ns: ModelTimeNs,
    pub terminal_liquid_kg_m2: f64,
    pub beginning_owner_digest: Digest32,
    pub ending_owner_digest: Digest32,
    pub receipt_id: Digest32,
}

fn validate_receipt_identity(
    receipt: &SnowStage3TerminalHandoffReceipt,
) -> Result<(), SnowStage3HandoffError> {
    if receipt.parent_identity.is_empty() || receipt.segment_identity.is_empty() {
        return Err(SnowStage3HandoffError::InvalidState(
            "handoff receipt identity is incomplete",
        ));
    }
    let mut unsealed = receipt.clone();
    unsealed.receipt_id = Digest32::zero();
    if serialized_digest(&unsealed)? != receipt.receipt_id {
        return Err(SnowStage3HandoffError::InvalidState(
            "handoff receipt digest does not match its body",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct StagedHandoff {
    receipt: SnowStage3TerminalHandoffReceipt,
    ending_owners: CompleteOwnerSet,
    accepted_cursor_ns: ModelTimeNs,
    accepted_event_ordinal: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SnowStage3HandoffRuntime {
    committed_owners: CompleteOwnerSet,
    accepted_cursor_ns: ModelTimeNs,
    accepted_event_ordinal: u64,
    receipt_chain: Vec<Digest32>,
    #[serde(default)]
    receipt_history: Vec<SnowStage3TerminalHandoffReceipt>,
    #[serde(skip)]
    pending: Option<StagedHandoff>,
}

impl SnowStage3HandoffRuntime {
    pub fn new(
        accepted_cursor_ns: ModelTimeNs,
        committed_owners: CompleteOwnerSet,
    ) -> Result<Self, SnowStage3HandoffError> {
        let committed_owners = CompleteOwnerSet::new(committed_owners.owners)?;
        committed_owners.digest()?;
        Ok(Self {
            committed_owners,
            accepted_cursor_ns,
            accepted_event_ordinal: 0,
            receipt_chain: Vec::new(),
            receipt_history: Vec::new(),
            pending: None,
        })
    }

    #[must_use]
    pub const fn accepted_cursor_ns(&self) -> ModelTimeNs {
        self.accepted_cursor_ns
    }

    #[must_use]
    pub const fn accepted_event_ordinal(&self) -> u64 {
        self.accepted_event_ordinal
    }

    #[must_use]
    pub fn committed_owners(&self) -> &CompleteOwnerSet {
        &self.committed_owners
    }

    #[must_use]
    pub fn receipt_chain(&self) -> &[Digest32] {
        &self.receipt_chain
    }

    #[must_use]
    pub fn receipt_history(&self) -> &[SnowStage3TerminalHandoffReceipt] {
        &self.receipt_history
    }

    pub fn committed_owner_digest(&self) -> Result<Digest32, SnowStage3HandoffError> {
        self.committed_owners.digest()
    }

    pub fn validate_restored(&self) -> Result<(), SnowStage3HandoffError> {
        let _ = CompleteOwnerSet::new(self.committed_owners.owners.clone())?;
        if self.pending.is_some() {
            return Err(SnowStage3HandoffError::InvalidState(
                "a restart cannot contain an uncommitted handoff",
            ));
        }
        if self.receipt_chain.len() != self.receipt_history.len()
            || self
                .receipt_chain
                .iter()
                .zip(&self.receipt_history)
                .any(|(digest, receipt)| digest != &receipt.receipt_id)
        {
            return Err(SnowStage3HandoffError::InvalidState(
                "restart receipt history does not match the receipt chain",
            ));
        }
        for receipt in &self.receipt_history {
            validate_receipt_identity(receipt)?;
        }
        for (index, receipt) in self.receipt_history.iter().enumerate() {
            let expected_predecessor = index
                .checked_sub(1)
                .and_then(|previous| self.receipt_history.get(previous))
                .map(|previous| previous.receipt_id);
            if receipt.predecessor_receipt_id != expected_predecessor {
                return Err(SnowStage3HandoffError::InvalidState(
                    "restart receipt predecessor chain is not contiguous",
                ));
            }
        }
        if let Some(last) = self.receipt_history.last() {
            let restored_end = last
                .accepted_event_tick
                .get()
                .checked_add(last.continuation_duration_ns.get())
                .ok_or(SnowStage3HandoffError::InvalidState(
                    "restart accepted cursor overflow",
                ))?;
            if restored_end != self.accepted_cursor_ns.get()
                || self.committed_owners.digest()? != last.ending_owner_digest
            {
                return Err(SnowStage3HandoffError::InvalidState(
                    "restart cursor or ending-owner digest does not match the final receipt",
                ));
            }
        }
        for (index, receipt) in self.receipt_history.iter().enumerate() {
            let expected_ordinal = u64::try_from(index)
                .map_err(|_| SnowStage3HandoffError::InvalidState("restart ordinal overflow"))?
                .checked_add(1)
                .ok_or(SnowStage3HandoffError::InvalidState(
                    "restart ordinal overflow",
                ))?;
            if receipt.event_ordinal != expected_ordinal {
                return Err(SnowStage3HandoffError::InvalidState(
                    "restart event ordinal history is not contiguous",
                ));
            }
        }
        if self
            .receipt_history
            .last()
            .is_some_and(|receipt| receipt.event_ordinal != self.accepted_event_ordinal)
            || (self.receipt_history.is_empty() && self.accepted_event_ordinal != 0)
        {
            return Err(SnowStage3HandoffError::InvalidState(
                "restart event ordinal does not match receipt history",
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub fn stage(
        &mut self,
        request: SnowStage3TerminalHandoffRequest,
    ) -> Result<(), SnowStage3HandoffError> {
        if self.pending.is_some() {
            return Err(SnowStage3HandoffError::InvalidState(
                "a terminal handoff is already staged",
            ));
        }
        if request.beginning_owners != self.committed_owners {
            return Err(SnowStage3HandoffError::InvalidOwnerSet(
                "beginning owner bytes do not match committed owner bytes",
            ));
        }
        request.owner_execution.validate()?;
        if request.owner_execution.ending_owners != request.ending_owners {
            return Err(SnowStage3HandoffError::InvalidOwnerSet(
                "owner execution receipt does not match ending owners",
            ));
        }
        if request.event.parent_start_tick != self.accepted_cursor_ns {
            return Err(SnowStage3HandoffError::InvalidState(
                "event parent does not start at accepted cursor",
            ));
        }
        let expected_event_ordinal = self.accepted_event_ordinal.checked_add(1).ok_or(
            SnowStage3HandoffError::InvalidState("terminal event ordinal overflow"),
        )?;
        if request.event.event_ordinal != expected_event_ordinal {
            return Err(SnowStage3HandoffError::InvalidState(
                "terminal event ordinal is not exactly once",
            ));
        }
        if self.receipt_history.iter().any(|receipt| {
            receipt.parent_identity == request.event.parent_identity
                && receipt.segment_identity == request.event.segment_identity
        }) {
            return Err(SnowStage3HandoffError::InvalidState(
                "terminal segment identity was already accepted",
            ));
        }
        if request.continuation.post_event_contains_snow_operands {
            return Err(SnowStage3HandoffError::SnowOperandInSnowFreeContinuation);
        }
        let carrier = evaluate_shared_carrier(&request.carrier)?;
        let event = locate_terminal_event(&request.event)?;
        if request.carrier.support_receipts != request.event.pre_active_participants {
            return Err(SnowStage3HandoffError::InvalidState(
                "carrier and terminal event pre-participant support receipts do not join",
            ));
        }
        let accepted_tick =
            event
                .accepted_event_tick
                .ok_or(SnowStage3HandoffError::InvalidState(
                    "accepted event tick missing",
                ))?;
        let continuation_duration = request.event.parent_end_tick.get() - accepted_tick.get();
        let post_support = event.post_common_minimum_support_ns.get();
        if continuation_duration != 0 && request.event.post_active_participants.is_empty() {
            return Err(SnowStage3HandoffError::InvalidSnowFreeSupport(
                "nonzero continuation requires successor participants",
            ));
        }
        if continuation_duration != 0 && continuation_duration < post_support {
            return Err(SnowStage3HandoffError::InvalidSnowFreeSupport(
                "post-event continuation is below the active participant support",
            ));
        }
        if continuation_duration != 0 && post_support < LSE_MINIMUM_SUPPORT_NS {
            return Err(SnowStage3HandoffError::InvalidSnowFreeSupport(
                "nonzero continuation is below the LSE minimum support",
            ));
        }
        if request.continuation.duration_ns.get() != continuation_duration {
            return Err(SnowStage3HandoffError::InvalidSnowFreeSupport(
                "continuation duration does not equal the half-open remainder",
            ));
        }
        let mass = [
            request.retained_liquid_kg_m2,
            request.snow_support_rain_kg_m2,
            request.terminal_melt_kg_m2,
            request.terminal_refreeze_kg_m2,
        ];
        if mass.iter().any(|value| !value.is_finite() || *value < 0.0) {
            return Err(SnowStage3HandoffError::InvalidLedger(
                "terminal liquid operands must be finite and nonnegative",
            ));
        }
        let terminal_liquid = request.retained_liquid_kg_m2
            + request.snow_support_rain_kg_m2
            + request.terminal_melt_kg_m2
            - request.terminal_refreeze_kg_m2;
        if terminal_liquid < -CLOSURE_TOLERANCE {
            return Err(SnowStage3HandoffError::InvalidLedger(
                "terminal liquid debit-credit join is negative",
            ));
        }
        if (request.continuation.terminal_liquid_kg_m2 - terminal_liquid).abs() > CLOSURE_TOLERANCE
        {
            return Err(SnowStage3HandoffError::InvalidLedger(
                "terminal liquid was not transferred exactly once",
            ));
        }
        let beginning_digest = request.beginning_owners.digest()?;
        let ending_digest = request.ending_owners.digest()?;
        let mut receipt = SnowStage3TerminalHandoffReceipt {
            parent_identity: event.parent_identity.clone(),
            segment_identity: event.segment_identity.clone(),
            event_ordinal: event.event_ordinal,
            candidate_set_digest: event.candidate_set_digest,
            accepted_tie_rank: event.accepted_tie_rank.ok_or(
                SnowStage3HandoffError::InvalidState("accepted terminal event has no tie rank"),
            )?,
            predecessor_receipt_id: self
                .receipt_history
                .last()
                .map(|receipt| receipt.receipt_id),
            carrier_receipt_id: carrier.receipt_id,
            event_receipt_id: event.receipt_id,
            accepted_event_tick: accepted_tick,
            continuation_duration_ns: ModelTimeNs::new(continuation_duration),
            terminal_liquid_kg_m2: terminal_liquid,
            beginning_owner_digest: beginning_digest,
            ending_owner_digest: ending_digest,
            receipt_id: Digest32::zero(),
        };
        receipt.receipt_id = serialized_digest(&receipt)?;
        self.pending = Some(StagedHandoff {
            receipt,
            ending_owners: request.ending_owners,
            accepted_cursor_ns: request.event.parent_end_tick,
            accepted_event_ordinal: event.event_ordinal,
        });
        Ok(())
    }

    pub fn commit_pending(
        &mut self,
    ) -> Result<SnowStage3TerminalHandoffReceipt, SnowStage3HandoffError> {
        let staged = self
            .pending
            .take()
            .ok_or(SnowStage3HandoffError::InvalidState(
                "no staged terminal handoff",
            ))?;
        self.committed_owners = staged.ending_owners;
        self.committed_owners = CompleteOwnerSet::new(self.committed_owners.owners.clone())?;
        self.accepted_cursor_ns = staged.accepted_cursor_ns;
        self.accepted_event_ordinal = staged.accepted_event_ordinal;
        self.receipt_chain.push(staged.receipt.receipt_id);
        self.receipt_history.push(staged.receipt.clone());
        Ok(staged.receipt)
    }

    pub fn checkpoint_digest(&self) -> Result<String, SnowStage3HandoffError> {
        let bytes = serde_json::to_vec(self)
            .map_err(|_| SnowStage3HandoffError::InvalidState("checkpoint serialization"))?;
        let mut digest = Sha256::new();
        digest.update(bytes);
        Ok(format!("{:x}", digest.finalize()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boundary_inputs(latent_energy_j_m2: f64) -> Stage3SnowSurfaceBoundaryReceiptInputs {
        Stage3SnowSurfaceBoundaryReceiptInputs {
            support: TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
                .expect("valid support"),
            sensible_energy_j_m2: 0.0,
            vapor_mass_kg_m2: 0.001,
            latent_energy_j_m2,
            shortwave_energy_j_m2: 0.0,
            net_longwave_energy_j_m2: 0.0,
            precipitation_advection_j_m2: 0.0,
            latent_heat_j_kg: 2_500_000.0,
            beginning_stage3_state_sha256: Digest32::from_bytes([1; 32]),
            identity: Stage3BoundaryIdentity::Provisional {
                carrier_receipt_sha256: Digest32::from_bytes([2; 32]),
            },
        }
    }

    #[test]
    fn covered_boundary_receipt_binds_latent_mass_and_energy() {
        let receipt = Stage3SnowSurfaceBoundaryReceiptV1::try_new(boundary_inputs(2_500.0));
        assert!(receipt.is_ok());

        let poisoned = Stage3SnowSurfaceBoundaryReceiptV1::try_new(boundary_inputs(2_500.0 + 1.0));
        assert!(matches!(
            poisoned,
            Err(SnowStage3HandoffError::InvalidCarrier(
                "Stage-3 covered latent mass-energy identity"
            ))
        ));
    }

    #[test]
    fn final_covered_boundary_receipt_is_sealed_and_one_bit_poisoned() {
        let receipt =
            FinalStage3CanopyBoundaryReceiptV1::try_new(FinalStage3CanopyBoundaryReceiptInputs {
                support: boundary_inputs(2_500.0).support,
                destination: (
                    OfeId::try_new("ofe-1").expect("OFE"),
                    TileId::try_new("tile-1").expect("tile"),
                ),
                beginning_v11_state_sha256: Digest32::from_bytes([1; 32]),
                beginning_stage3_state_sha256: Digest32::from_bytes([2; 32]),
                ending_v11_state_sha256: Digest32::from_bytes([6; 32]),
                ending_stage3_state_sha256: Digest32::from_bytes([7; 32]),
                provisional_carrier_receipt_sha256: Digest32::from_bytes([3; 32]),
                optical_receipt_sha256: Digest32::from_bytes([4; 32]),
                reciprocal_longwave_receipt_sha256: Digest32::from_bytes([5; 32]),
                sensible_to_canopy_air_w_m2: 1.0,
                vapor_to_canopy_air_kg_m2_s: 2.0e-6,
                latent_energy_to_canopy_air_j_m2: 9_000.0,
                snow_temperature_k: 268.0,
                latent_heat_j_kg: 2_500_000.0,
                snow_absorbed_shortwave_w_m2: 4.0,
                snow_net_longwave_w_m2: -20.0,
            })
            .expect("final receipt");
        receipt.validate().expect("sealed receipt");

        let mut poisoned = receipt.clone();
        poisoned.snow_net_longwave_w_m2 =
            f64::from_bits(poisoned.snow_net_longwave_w_m2.to_bits() + 1);
        assert!(poisoned.validate().is_err());
    }

    #[test]
    fn boundary_identity_rejects_partial_final_sealing() {
        let identity = Stage3BoundaryIdentity::Final {
            provisional_carrier_receipt_sha256: Digest32::from_bytes([1; 32]),
            optical_receipt_sha256: Digest32::from_bytes([2; 32]),
            reciprocal_longwave_receipt_sha256: Digest32::zero(),
            final_destination_receipt_sha256: Digest32::from_bytes([4; 32]),
            final_lane_receipt_sha256: Digest32::from_bytes([5; 32]),
        };
        let mut inputs = boundary_inputs(2_500.0);
        inputs.identity = identity;
        assert!(matches!(
            Stage3SnowSurfaceBoundaryReceiptV1::try_new(inputs),
            Err(SnowStage3HandoffError::InvalidCarrier(
                "Stage-3 boundary identity is incomplete"
            ))
        ));
    }

    #[test]
    fn ofe_ground_lane_receipt_requires_complete_fraction_and_binds_area_basis() {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("valid support");
        let contribution = LaneBoundaryContributionV1 {
            tile_id: TileId::try_new("tile-1").expect("tile"),
            tile_fraction: 1.0,
            boundary_class: Stage3TileBoundaryClassV1::V11CanopyCovered,
            boundary_model_definition_sha256: Digest32::from_bytes([9; 32]),
            beginning_stage3_state_sha256: Digest32::from_bytes([10; 32]),
            provisional_carrier_receipt_sha256: Digest32::from_bytes([11; 32]),
            optical_receipt_sha256: Digest32::from_bytes([12; 32]),
            reciprocal_longwave_receipt_sha256: Digest32::from_bytes([13; 32]),
            final_boundary_receipt_sha256: Digest32::from_bytes([1; 32]),
            sensible_to_canopy_air_w_m2: 1.0,
            vapor_to_canopy_air_kg_m2_s: 2.0e-6,
            latent_energy_to_canopy_air_j_m2: 9_000.0,
            snow_absorbed_shortwave_w_m2: 4.0,
            snow_net_longwave_w_m2: -20.0,
            snow_temperature_k: 268.0,
            latent_heat_j_kg: 2_500_000.0,
        };
        let expected_topology = [LaneBoundaryTopologyExpectationV1 {
            tile_id: contribution.tile_id.clone(),
            tile_fraction_bits: contribution.tile_fraction.to_bits(),
            boundary_class: contribution.boundary_class,
            boundary_model_definition_sha256: contribution.boundary_model_definition_sha256,
        }];
        let receipt = LaneStage3BoundaryReceiptV1::try_new(
            LaneStage3BoundaryReceiptV1 {
                lane_id: 1,
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                support,
                area_basis: Stage3LaneAreaBasisV1::OfeGround,
                topology_configuration_sha256: Digest32::from_bytes([2; 32]),
                provisional_carrier_receipt_sha256: Digest32::from_bytes([3; 32]),
                optical_receipt_sha256: Digest32::from_bytes([4; 32]),
                reciprocal_longwave_receipt_sha256: Digest32::from_bytes([5; 32]),
                final_destination_receipt_sha256: Digest32::from_bytes([6; 32]),
                ordered_destinations: vec![contribution.clone()],
                aggregate_sensible_to_canopy_air_w_m2: contribution.sensible_to_canopy_air_w_m2,
                aggregate_vapor_to_canopy_air_kg_m2_s: contribution.vapor_to_canopy_air_kg_m2_s,
                aggregate_latent_energy_to_canopy_air_j_m2: contribution
                    .latent_energy_to_canopy_air_j_m2,
                aggregate_snow_absorbed_shortwave_w_m2: contribution.snow_absorbed_shortwave_w_m2,
                aggregate_snow_net_longwave_w_m2: contribution.snow_net_longwave_w_m2,
                aggregate_snow_temperature_k: contribution.snow_temperature_k,
                aggregate_latent_heat_j_kg: contribution.latent_heat_j_kg,
                receipt_sha256: Digest32::zero(),
            },
            &expected_topology,
        )
        .expect("OFE-ground lane receipt");
        receipt.validate().expect("lane receipt validates");

        let mut poisoned_source_set = receipt.clone();
        poisoned_source_set.final_destination_receipt_sha256 = Digest32::from_bytes([99; 32]);
        assert!(poisoned_source_set.validate().is_err());

        let mut poisoned_fraction = receipt;
        poisoned_fraction.ordered_destinations[0].tile_fraction = 0.38;
        assert!(poisoned_fraction.validate().is_err());
    }

    #[test]
    fn mixed_open_covered_lane_uses_ofe_ground_sum_without_renormalization() {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("valid support");
        let contribution =
            |tile: &str,
             fraction: f64,
             sensible: f64,
             vapor: f64,
             digest: u8,
             boundary_class: Stage3TileBoundaryClassV1| LaneBoundaryContributionV1 {
                tile_id: TileId::try_new(tile).expect("tile"),
                tile_fraction: fraction,
                boundary_class,
                boundary_model_definition_sha256: Digest32::from_bytes([9 + digest; 32]),
                beginning_stage3_state_sha256: Digest32::from_bytes([10; 32]),
                provisional_carrier_receipt_sha256: Digest32::from_bytes([11 + digest; 32]),
                optical_receipt_sha256: Digest32::from_bytes([12 + digest; 32]),
                reciprocal_longwave_receipt_sha256: Digest32::from_bytes([13 + digest; 32]),
                final_boundary_receipt_sha256: Digest32::from_bytes([digest; 32]),
                sensible_to_canopy_air_w_m2: sensible,
                vapor_to_canopy_air_kg_m2_s: vapor,
                latent_energy_to_canopy_air_j_m2: vapor * 2_500_000.0 * 1_800.0,
                snow_absorbed_shortwave_w_m2: 0.0,
                snow_net_longwave_w_m2: 0.0,
                snow_temperature_k: 268.0,
                latent_heat_j_kg: 2_500_000.0,
            };
        let contributions = vec![
            contribution(
                "covered",
                0.6,
                100.0,
                1.0e-6,
                7,
                Stage3TileBoundaryClassV1::V11CanopyCovered,
            ),
            contribution(
                "open",
                0.4,
                0.0,
                -1.0e-6,
                8,
                Stage3TileBoundaryClassV1::OpenSnow,
            ),
        ];
        let expected_topology = contributions
            .iter()
            .map(|item| LaneBoundaryTopologyExpectationV1 {
                tile_id: item.tile_id.clone(),
                tile_fraction_bits: item.tile_fraction.to_bits(),
                boundary_class: item.boundary_class,
                boundary_model_definition_sha256: item.boundary_model_definition_sha256,
            })
            .collect::<Vec<_>>();
        let receipt = LaneStage3BoundaryReceiptV1::try_new(
            LaneStage3BoundaryReceiptV1 {
                lane_id: 1,
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                support,
                area_basis: Stage3LaneAreaBasisV1::OfeGround,
                topology_configuration_sha256: Digest32::from_bytes([2; 32]),
                provisional_carrier_receipt_sha256: Digest32::from_bytes([3; 32]),
                optical_receipt_sha256: Digest32::from_bytes([4; 32]),
                reciprocal_longwave_receipt_sha256: Digest32::from_bytes([5; 32]),
                final_destination_receipt_sha256: Digest32::from_bytes([6; 32]),
                ordered_destinations: contributions,
                aggregate_sensible_to_canopy_air_w_m2: 60.0,
                aggregate_vapor_to_canopy_air_kg_m2_s: 0.6 * 1.0e-6 + 0.4 * -1.0e-6,
                aggregate_latent_energy_to_canopy_air_j_m2: 0.6 * (1.0e-6 * 2_500_000.0 * 1_800.0)
                    + 0.4 * (-1.0e-6 * 2_500_000.0 * 1_800.0),
                aggregate_snow_absorbed_shortwave_w_m2: 0.0,
                aggregate_snow_net_longwave_w_m2: 0.0,
                aggregate_snow_temperature_k: 268.0,
                aggregate_latent_heat_j_kg: 2_500_000.0,
                receipt_sha256: Digest32::zero(),
            },
            &expected_topology,
        )
        .expect("mixed OFE-ground receipt");

        assert_eq!(receipt.aggregate_sensible_to_canopy_air_w_m2, 60.0);
        receipt.validate().expect("mixed lane receipt validates");

        let mut poisoned_common_state = receipt.clone();
        poisoned_common_state.ordered_destinations[1].snow_temperature_k =
            f64::from_bits(268.0_f64.to_bits() + 1);
        assert!(poisoned_common_state.validate().is_err());

        let mut poisoned_class = receipt.clone();
        poisoned_class.ordered_destinations[1].boundary_class =
            Stage3TileBoundaryClassV1::V11CanopyCovered;
        assert!(LaneStage3BoundaryReceiptV1::try_new(poisoned_class, &expected_topology).is_err());

        let mut poisoned_model = receipt;
        poisoned_model.ordered_destinations[1].boundary_model_definition_sha256 =
            Digest32::from_bytes([99; 32]);
        assert!(LaneStage3BoundaryReceiptV1::try_new(poisoned_model, &expected_topology).is_err());
    }
}
