//! Independent post-candidate Stage-3 snow mass and energy accounting.
//!
//! No API in this module converts ledger fields into solver operands.

use super::{OfeId, Stage3LaneAreaBasisV1};
use openwepp_coupled_time::{Digest32, TimeSupport, digest_bytes};
use openwepp_kernel_contract::TileId;
use serde::{Deserialize, Serialize};

/// Opt-in, process-local closure evidence for qualification tests.
///
/// This audit is deliberately absent from every receipt, owner, restart, and
/// publication schema. Production execution does not enable it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stage3PhysicalOutcomeClosureAuditV1 {
    pub validated_ledger_count: u64,
    pub maximum_abs_mass_residual_kg_m2: f64,
    pub maximum_abs_energy_residual_j_m2: f64,
}

std::thread_local! {
    static STAGE3_PHYSICAL_OUTCOME_CLOSURE_AUDIT: std::cell::RefCell<
        Option<Stage3PhysicalOutcomeClosureAuditV1>
    > = const { std::cell::RefCell::new(None) };
}

pub fn begin_stage3_physical_outcome_closure_audit_v1() {
    STAGE3_PHYSICAL_OUTCOME_CLOSURE_AUDIT.with(|audit| {
        *audit.borrow_mut() = Some(Stage3PhysicalOutcomeClosureAuditV1::default());
    });
}

#[must_use]
pub fn take_stage3_physical_outcome_closure_audit_v1() -> Stage3PhysicalOutcomeClosureAuditV1 {
    STAGE3_PHYSICAL_OUTCOME_CLOSURE_AUDIT
        .with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

fn record_stage3_physical_outcome_closure_audit_v1(value: &Stage3LanePhysicalOutcomeLedgerV1) {
    STAGE3_PHYSICAL_OUTCOME_CLOSURE_AUDIT.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(audit) = audit.as_mut() else {
            return;
        };
        audit.validated_ledger_count = audit.validated_ledger_count.saturating_add(1);
        audit.maximum_abs_mass_residual_kg_m2 = audit.maximum_abs_mass_residual_kg_m2.max(
            [
                value.mass_residual_kg_m2,
                value.ice_residual_kg_m2,
                value.liquid_residual_kg_m2,
                value.vapor_residual_kg_m2,
                value.ending_liquid_residual_kg_m2,
            ]
            .into_iter()
            .map(f64::abs)
            .fold(0.0, f64::max),
        );
        audit.maximum_abs_energy_residual_j_m2 = audit
            .maximum_abs_energy_residual_j_m2
            .max(value.energy_residual_j_m2.abs());
    });
}

#[derive(Clone, Debug)]
pub(crate) struct TerminalSnowBottomSoilTrialInputsV1<'a> {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: &'a OfeId,
    pub canonical_source_sha256: Digest32,
    pub ice_kg_m2: f64,
    pub liquid_kg_m2: f64,
    pub cold_content_j_m2: f64,
    pub depth_m: f64,
    pub density_kg_m3: f64,
    pub temperature_k: f64,
    pub atmospheric_pressure_pa: f64,
    pub first_soil_configuration: &'a openwepp_land_surface_energy::SoilInterfaceLayer,
    pub beginning_first_soil: &'a openwepp_land_surface_energy::SoilThermalLayerSnapshot,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TerminalSnowBottomSoilTrialResultV1 {
    pub ending_first_soil: openwepp_land_surface_energy::SoilThermalLayerSnapshot,
    pub ending_snow_temperature_k: f64,
    pub snow_heat_j_m2: f64,
    pub soil_heat_j_m2: f64,
    pub receipt: TerminalSnowSoilTrialReceiptV1,
}

/// Unpublished receipt for one adaptive/root trial. This is deliberately not
/// the final dormant-endpoint receipt: rejected/full alternatives remain
/// evidence and cannot claim an accepted terminal owner transition.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TerminalSnowSoilTrialReceiptV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub canonical_source_sha256: Digest32,
    pub beginning_snow_temperature_k: f64,
    pub ending_snow_temperature_k: f64,
    pub beginning_soil_temperature_k: f64,
    pub ending_soil_temperature_k: f64,
    pub snow_heat_j_m2: f64,
    pub soil_heat_j_m2: f64,
    pub ending_soil_candidate_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl TerminalSnowSoilTrialReceiptV1 {
    fn seal(mut self) -> Result<Self, Stage3PhysicalOutcomeLedgerError> {
        self.receipt_sha256 = Digest32::zero();
        if self.canonical_source_sha256 == Digest32::zero()
            || self.ending_soil_candidate_sha256 == Digest32::zero()
            || self.snow_heat_j_m2.to_bits() != (-self.soil_heat_j_m2).to_bits()
            || [
                self.beginning_snow_temperature_k,
                self.ending_snow_temperature_k,
                self.beginning_soil_temperature_k,
                self.ending_soil_temperature_k,
                self.snow_heat_j_m2,
                self.soil_heat_j_m2,
            ]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "terminal snow-soil trial receipt",
            ));
        }
        let mut bytes = b"OPENWEPP_TERMINAL_SNOW_SOIL_TRIAL_RECEIPT_V1".to_vec();
        bytes.extend_from_slice(&self.support.start_ns().get().to_be_bytes());
        bytes.extend_from_slice(&self.support.end_ns().get().to_be_bytes());
        bytes.extend_from_slice(&self.lane_id.to_be_bytes());
        append_str(&mut bytes, self.ofe_id.as_str());
        bytes.extend_from_slice(self.canonical_source_sha256.as_bytes());
        for value in [
            self.beginning_snow_temperature_k,
            self.ending_snow_temperature_k,
            self.beginning_soil_temperature_k,
            self.ending_soil_temperature_k,
            self.snow_heat_j_m2,
            self.soil_heat_j_m2,
        ] {
            bytes.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        bytes.extend_from_slice(self.ending_soil_candidate_sha256.as_bytes());
        self.receipt_sha256 = digest_bytes(&bytes);
        Ok(self)
    }

    pub(crate) fn validate(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        let mut candidate = self.clone();
        candidate.receipt_sha256 = Digest32::zero();
        if candidate.seal()? != *self {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "terminal snow-soil trial receipt seal",
            ));
        }
        Ok(())
    }
}

/// Advance the first soil node across one exact positive terminal-snow trial.
///
/// The snow volume is supplied directly by the terminal solver. It is never
/// projected through a persistent Stage-3 state, and no ending snow
/// temperature is requested. The first-order trial map evaluates the
/// snow--soil Crank--Nicolson flux from the coupled ending snow/soil
/// candidates and applies identical-bit opposite custody to snow and soil.
pub(crate) fn evaluate_terminal_snow_bottom_soil_trial_v1(
    inputs: &TerminalSnowBottomSoilTrialInputsV1<'_>,
) -> Result<TerminalSnowBottomSoilTrialResultV1, Stage3PhysicalOutcomeLedgerError> {
    let duration_s = f64::from_bits(inputs.support.duration_s_bits());
    let scalars = [
        duration_s,
        inputs.ice_kg_m2,
        inputs.liquid_kg_m2,
        inputs.cold_content_j_m2,
        inputs.depth_m,
        inputs.density_kg_m3,
        inputs.temperature_k,
        inputs.atmospheric_pressure_pa,
        inputs.first_soil_configuration.thickness_m,
        inputs.first_soil_configuration.thermal_conductivity_w_m_k,
        inputs.first_soil_configuration.areal_heat_capacity_j_m2_k,
        inputs.beginning_first_soil.temperature_k,
        inputs.beginning_first_soil.enthalpy_j_m2_ofe_ground,
    ];
    if inputs.canonical_source_sha256 == Digest32::zero()
        || scalars.iter().any(|value| !value.is_finite())
        || duration_s <= 0.0
        || inputs.ice_kg_m2 <= 0.0
        || inputs.liquid_kg_m2 < 0.0
        || inputs.cold_content_j_m2 < 0.0
        || inputs.depth_m <= 0.0
        || inputs.density_kg_m3 <= 0.0
        || inputs.temperature_k <= 0.0
        || inputs.atmospheric_pressure_pa <= 0.0
        || inputs.first_soil_configuration.thickness_m <= 0.0
        || inputs.first_soil_configuration.thermal_conductivity_w_m_k <= 0.0
        || inputs.first_soil_configuration.areal_heat_capacity_j_m2_k <= 0.0
        || inputs.beginning_first_soil.temperature_k <= 0.0
        || inputs.beginning_first_soil.layer_id != inputs.first_soil_configuration.layer_id
        || (inputs.ice_kg_m2 - inputs.density_kg_m3 * inputs.depth_m).abs() > 1.0e-9
    {
        return Err(Stage3PhysicalOutcomeLedgerError::Numeric(
            "terminal snow-bottom soil trial operands",
        ));
    }

    let temperature_c =
        openwepp_unit_boundary::TemperatureCelsius::try_new(inputs.temperature_k - 273.15)
            .map_err(|_| {
                Stage3PhysicalOutcomeLedgerError::Numeric("terminal snow-bottom temperature")
            })?;
    let pressure = openwepp_meteorology::surface_energy::PressurePascals::try_new(
        inputs.atmospheric_pressure_pa,
    )
    .map_err(|_| Stage3PhysicalOutcomeLedgerError::Numeric("terminal snow-bottom pressure"))?;
    let snow_conductivity =
        openwepp_meteorology::surface_energy::snow_effective_thermal_conductivity_snobal(
            inputs.density_kg_m3,
            temperature_c,
            pressure,
        )
        .map_err(|_| {
            Stage3PhysicalOutcomeLedgerError::Numeric("terminal snow-bottom conductivity")
        })?
        .as_watts_per_meter_kelvin();
    let resistance = 0.5 * inputs.depth_m / snow_conductivity
        + 0.5 * inputs.first_soil_configuration.thickness_m
            / inputs.first_soil_configuration.thermal_conductivity_w_m_k;
    if !resistance.is_finite() || resistance <= 0.0 {
        return Err(Stage3PhysicalOutcomeLedgerError::Numeric(
            "terminal snow-soil resistance",
        ));
    }
    let conductance_w_m2_k = 1.0 / resistance;
    let snow_capacity_j_m2_k = inputs.ice_kg_m2 * 2_100.0;
    let soil_capacity_j_m2_k = inputs.first_soil_configuration.areal_heat_capacity_j_m2_k;
    let beginning_delta_k = inputs.temperature_k - inputs.beginning_first_soil.temperature_k;
    let cn_denominator = 1.0
        + 0.5
            * duration_s
            * conductance_w_m2_k
            * (1.0 / snow_capacity_j_m2_k + 1.0 / soil_capacity_j_m2_k);
    if !cn_denominator.is_finite() || cn_denominator <= 0.0 {
        return Err(Stage3PhysicalOutcomeLedgerError::Numeric(
            "terminal snow-soil Crank-Nicolson denominator",
        ));
    }
    let soil_heat_j_m2 = duration_s * conductance_w_m2_k * beginning_delta_k / cn_denominator;
    let snow_heat_j_m2 = -soil_heat_j_m2;
    if !soil_heat_j_m2.is_finite() || snow_heat_j_m2.to_bits() != (-soil_heat_j_m2).to_bits() {
        return Err(Stage3PhysicalOutcomeLedgerError::Closure(
            "terminal snow-soil equal and opposite heat",
        ));
    }
    let ending_snow_temperature = inputs.temperature_k - soil_heat_j_m2 / snow_capacity_j_m2_k;
    let ending_enthalpy = inputs.beginning_first_soil.enthalpy_j_m2_ofe_ground + soil_heat_j_m2;
    let ending_temperature = inputs.beginning_first_soil.temperature_k
        + soil_heat_j_m2 / inputs.first_soil_configuration.areal_heat_capacity_j_m2_k;
    if !ending_snow_temperature.is_finite()
        || ending_snow_temperature <= 0.0
        || !ending_enthalpy.is_finite()
        || !ending_temperature.is_finite()
        || ending_temperature <= 0.0
    {
        return Err(Stage3PhysicalOutcomeLedgerError::Numeric(
            "terminal first-soil ending candidate",
        ));
    }
    let ending_first_soil = openwepp_land_surface_energy::SoilThermalLayerSnapshot {
        layer_id: inputs.beginning_first_soil.layer_id.clone(),
        temperature_k: ending_temperature,
        enthalpy_j_m2_ofe_ground: ending_enthalpy,
    };
    let ending_soil_owner_sha256 =
        digest_bytes(&serde_json::to_vec(&ending_first_soil).map_err(|_| {
            Stage3PhysicalOutcomeLedgerError::Identity("terminal first-soil candidate seal")
        })?);
    let receipt = TerminalSnowSoilTrialReceiptV1 {
        support: inputs.support,
        lane_id: inputs.lane_id,
        ofe_id: inputs.ofe_id.clone(),
        canonical_source_sha256: inputs.canonical_source_sha256,
        beginning_snow_temperature_k: inputs.temperature_k,
        ending_snow_temperature_k: ending_snow_temperature,
        beginning_soil_temperature_k: inputs.beginning_first_soil.temperature_k,
        ending_soil_temperature_k: ending_temperature,
        snow_heat_j_m2,
        soil_heat_j_m2,
        ending_soil_candidate_sha256: ending_soil_owner_sha256,
        receipt_sha256: Digest32::zero(),
    }
    .seal()?;
    Ok(TerminalSnowBottomSoilTrialResultV1 {
        ending_first_soil,
        ending_snow_temperature_k: ending_snow_temperature,
        snow_heat_j_m2,
        soil_heat_j_m2,
        receipt,
    })
}

/// Event-integrated lower-boundary custody for a lane whose accepted ending
/// snow owner is dormant.  Unlike the persistent receipt, this wire has no
/// ending snow temperature: dormancy is proved by the ending owner identity.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TerminalSnowSoilHeatReceiptV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub beginning_snow_owner_sha256: Digest32,
    pub ending_dormant_snow_owner_sha256: Digest32,
    pub ending_soil_owner_sha256: Digest32,
    pub limiting_boundary_receipt_sha256: Digest32,
    pub snow_heat_j_m2: f64,
    pub soil_heat_j_m2: f64,
    pub receipt_sha256: Digest32,
}

impl TerminalSnowSoilHeatReceiptV1 {
    pub(crate) fn seal(mut self) -> Result<Self, Stage3PhysicalOutcomeLedgerError> {
        self.receipt_sha256 = Digest32::zero();
        if self.beginning_snow_owner_sha256 == Digest32::zero()
            || self.ending_dormant_snow_owner_sha256 == Digest32::zero()
            || self.ending_soil_owner_sha256 == Digest32::zero()
            || self.limiting_boundary_receipt_sha256 == Digest32::zero()
            || !self.snow_heat_j_m2.is_finite()
            || self.soil_heat_j_m2.to_bits() != (-self.snow_heat_j_m2).to_bits()
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "terminal snow-soil custody",
            ));
        }
        self.receipt_sha256 = self.digest();
        Ok(self)
    }

    pub(crate) fn validate(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        let mut unsealed = self.clone();
        unsealed.receipt_sha256 = Digest32::zero();
        let expected = unsealed.seal()?;
        if &expected != self {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "terminal snow-soil receipt seal",
            ));
        }
        Ok(())
    }

    fn digest(&self) -> Digest32 {
        let mut bytes = b"OPENWEPP_TERMINAL_SNOW_SOIL_HEAT_RECEIPT_V1".to_vec();
        bytes.extend_from_slice(&self.support.start_ns().get().to_le_bytes());
        bytes.extend_from_slice(&self.support.end_ns().get().to_le_bytes());
        bytes.extend_from_slice(&self.lane_id.to_le_bytes());
        append_str(&mut bytes, self.ofe_id.as_str());
        bytes.extend_from_slice(self.beginning_snow_owner_sha256.as_bytes());
        bytes.extend_from_slice(self.ending_dormant_snow_owner_sha256.as_bytes());
        bytes.extend_from_slice(self.ending_soil_owner_sha256.as_bytes());
        bytes.extend_from_slice(self.limiting_boundary_receipt_sha256.as_bytes());
        bytes.extend_from_slice(&self.snow_heat_j_m2.to_bits().to_le_bytes());
        bytes.extend_from_slice(&self.soil_heat_j_m2.to_bits().to_le_bytes());
        digest_bytes(&bytes)
    }
}

pub(crate) fn ledger_set_digest(
    ledgers: &std::collections::BTreeMap<u32, Stage3LanePhysicalOutcomeLedgerV1>,
) -> Digest32 {
    let mut bytes = b"OPENWEPP_STAGE3_PHYSICAL_OUTCOME_LEDGER_SET_V1".to_vec();
    for (lane, ledger) in ledgers {
        bytes.extend_from_slice(&lane.to_be_bytes());
        bytes.extend_from_slice(ledger.receipt_sha256.as_bytes());
    }
    digest_bytes(&bytes)
}

const MASS_TOL: f64 = 1.0e-9;
const ENERGY_TOL: f64 = 1.0e-6;
const LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;

/// Destination-resolved authority for liquid exported by one OFE/lane snow
/// owner. Stage 3 owns one common snow depth/enthalpy density per lane. The
/// ordered tile vector is sealed here, at physical-outcome creation, so a
/// downstream receiver never invents an allocation from the OFE aggregate.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Stage3DestinationLiquidOutcomeV1 {
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub mass_kg_m2_tile_ground: f64,
    pub sensible_enthalpy_j_m2_tile_ground: f64,
}

pub(crate) fn seal_destination_liquid_outcomes_v1<'a>(
    ofe_id: &OfeId,
    destinations: impl IntoIterator<Item = (&'a TileId, f64)>,
    common_mass_kg_m2_tile_ground: f64,
    common_enthalpy_j_m2_tile_ground: f64,
) -> Result<(Vec<Stage3DestinationLiquidOutcomeV1>, f64, f64), Stage3PhysicalOutcomeLedgerError> {
    if !common_mass_kg_m2_tile_ground.is_finite()
        || common_mass_kg_m2_tile_ground < 0.0
        || !common_enthalpy_j_m2_tile_ground.is_finite()
        || common_enthalpy_j_m2_tile_ground < 0.0
    {
        return Err(Stage3PhysicalOutcomeLedgerError::Numeric(
            "destination liquid outcome",
        ));
    }
    let mut values = destinations
        .into_iter()
        .map(
            |(tile_id, tile_fraction)| Stage3DestinationLiquidOutcomeV1 {
                ofe_id: ofe_id.clone(),
                tile_id: tile_id.clone(),
                tile_fraction,
                mass_kg_m2_tile_ground: common_mass_kg_m2_tile_ground,
                sensible_enthalpy_j_m2_tile_ground: common_enthalpy_j_m2_tile_ground,
            },
        )
        .collect::<Vec<_>>();
    values.sort_by(|left, right| left.tile_id.cmp(&right.tile_id));
    if values.is_empty()
        || values
            .windows(2)
            .any(|pair| pair[0].tile_id >= pair[1].tile_id)
        || values
            .iter()
            .any(|value| !value.tile_fraction.is_finite() || value.tile_fraction <= 0.0)
    {
        return Err(Stage3PhysicalOutcomeLedgerError::Identity(
            "destination liquid topology",
        ));
    }
    let fraction_sum = values
        .iter()
        .try_fold(0.0_f64, |sum, value| {
            let next = sum + value.tile_fraction;
            next.is_finite().then_some(next)
        })
        .ok_or(Stage3PhysicalOutcomeLedgerError::Numeric(
            "destination liquid fraction sum",
        ))?;
    if (fraction_sum - 1.0).abs() > super::STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE {
        return Err(Stage3PhysicalOutcomeLedgerError::Identity(
            "destination liquid fraction closure",
        ));
    }
    let mass = values.iter().try_fold(0.0_f64, |sum, value| {
        let next = sum + value.tile_fraction * value.mass_kg_m2_tile_ground;
        next.is_finite().then_some(next)
    });
    let enthalpy = values.iter().try_fold(0.0_f64, |sum, value| {
        let next = sum + value.tile_fraction * value.sensible_enthalpy_j_m2_tile_ground;
        next.is_finite().then_some(next)
    });
    match (mass, enthalpy) {
        (Some(mass), Some(enthalpy)) => Ok((values, mass, enthalpy)),
        _ => Err(Stage3PhysicalOutcomeLedgerError::Numeric(
            "destination liquid reconstruction",
        )),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stage3LanePhysicalOutcomeExpectationV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub topology_sha256: Digest32,
    pub beginning_snow_owner_sha256: Digest32,
    pub ending_snow_owner_sha256: Digest32,
    pub precipitation_set_sha256: Digest32,
    /// SW, LW, sensible, latent/vapor, snow-soil, interlayer, in that order.
    pub source_receipts_sha256: [Digest32; 6],
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stage3LanePhysicalOutcomeLedgerV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub area_basis: Stage3LaneAreaBasisV1,
    pub topology_sha256: Digest32,
    pub beginning_snow_owner_sha256: Digest32,
    pub ending_snow_owner_sha256: Digest32,
    pub precipitation_set_sha256: Digest32,
    pub source_receipts_sha256: [Digest32; 6],
    pub beginning_ice_kg_m2: f64,
    pub beginning_liquid_kg_m2: f64,
    pub beginning_cold_content_j_m2: f64,
    pub beginning_enthalpy_j_m2: f64,
    pub ending_ice_kg_m2: f64,
    pub ending_liquid_kg_m2: f64,
    pub ending_cold_content_j_m2: f64,
    pub ending_enthalpy_j_m2: f64,
    pub solid_precipitation_kg_m2: f64,
    pub liquid_precipitation_kg_m2: f64,
    pub precipitation_advection_j_m2: f64,
    pub deposition_kg_m2: f64,
    pub sublimation_kg_m2: f64,
    /// Positive into snow, reconstructed independently from deposition/sublimation.
    pub vapor_transfer_kg_m2: f64,
    pub latent_heat_j_kg: f64,
    pub snow_surface_temperature_k: f64,
    pub vapor_material_enthalpy_j_m2: f64,
    pub melt_kg_m2: f64,
    pub refreeze_kg_m2: f64,
    pub terminal_liquid_kg_m2: f64,
    /// Sensible enthalpy above the 0 C liquid reference exported with the
    /// terminal-liquid boundary parcel.
    pub terminal_liquid_sensible_enthalpy_j_m2: f64,
    pub destination_liquid_outcomes: Vec<Stage3DestinationLiquidOutcomeV1>,
    pub retained_liquid_kg_m2: f64,
    /// All energy terms use positive-into-snow sign convention.
    pub shortwave_j_m2: f64,
    pub longwave_j_m2: f64,
    pub sensible_j_m2: f64,
    pub latent_j_m2: f64,
    pub soil_heat_j_m2: f64,
    pub interlayer_active_conduction_j_m2: f64,
    pub interlayer_lower_conduction_j_m2: f64,
    pub interlayer_conduction_j_m2: f64,
    pub refreeze_fusion_j_m2: f64,
    pub mass_residual_kg_m2: f64,
    pub ice_residual_kg_m2: f64,
    pub liquid_residual_kg_m2: f64,
    pub vapor_residual_kg_m2: f64,
    pub energy_residual_j_m2: f64,
    pub ending_liquid_residual_kg_m2: f64,
    pub receipt_sha256: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub(crate) enum Stage3PhysicalOutcomeLedgerError {
    #[error("physical outcome identity mismatch: {0}")]
    Identity(&'static str),
    #[error("physical outcome numeric domain failure: {0}")]
    Numeric(&'static str),
    #[error("physical outcome closure failure: {0}")]
    Closure(&'static str),
}

impl Stage3LanePhysicalOutcomeLedgerV1 {
    pub(crate) fn try_new(
        mut value: Self,
        expected: &Stage3LanePhysicalOutcomeExpectationV1,
    ) -> Result<Self, Stage3PhysicalOutcomeLedgerError> {
        value.receipt_sha256 = Digest32::zero();
        let r = value.reconstruct();
        value.mass_residual_kg_m2 = r[0];
        value.ice_residual_kg_m2 = r[1];
        value.liquid_residual_kg_m2 = r[2];
        value.vapor_residual_kg_m2 = r[3];
        value.energy_residual_j_m2 = r[4];
        value.ending_liquid_residual_kg_m2 = r[5];
        value.receipt_sha256 = value.digest();
        value.validate(expected)?;
        record_stage3_physical_outcome_closure_audit_v1(&value);
        Ok(value)
    }

    pub(crate) fn validate(
        &self,
        expected: &Stage3LanePhysicalOutcomeExpectationV1,
    ) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        self.validate_bindings(expected)?;
        self.validate_closure()
    }

    fn validate_bindings(
        &self,
        expected: &Stage3LanePhysicalOutcomeExpectationV1,
    ) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        if self.support != expected.support
            || self.lane_id != expected.lane_id
            || self.ofe_id != expected.ofe_id
            || self.area_basis != Stage3LaneAreaBasisV1::OfeGround
            || self.topology_sha256 != expected.topology_sha256
            || self.beginning_snow_owner_sha256 != expected.beginning_snow_owner_sha256
            || self.ending_snow_owner_sha256 != expected.ending_snow_owner_sha256
            || self.precipitation_set_sha256 != expected.precipitation_set_sha256
            || self.source_receipts_sha256 != expected.source_receipts_sha256
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "expected binding",
            ));
        }
        if self.receipt_sha256 == Digest32::zero() || self.receipt_sha256 != self.digest() {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity("seal"));
        }
        let (expected_destinations, mass, enthalpy) = seal_destination_liquid_outcomes_v1(
            &self.ofe_id,
            self.destination_liquid_outcomes
                .iter()
                .map(|value| (&value.tile_id, value.tile_fraction)),
            self.destination_liquid_outcomes
                .first()
                .map_or(0.0, |value| value.mass_kg_m2_tile_ground),
            self.destination_liquid_outcomes
                .first()
                .map_or(0.0, |value| value.sensible_enthalpy_j_m2_tile_ground),
        )?;
        if expected_destinations != self.destination_liquid_outcomes
            || mass.to_bits() != self.terminal_liquid_kg_m2.to_bits()
            || enthalpy.to_bits() != self.terminal_liquid_sensible_enthalpy_j_m2.to_bits()
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "destination liquid reconstruction",
            ));
        }
        let mut receipts = self.source_receipts_sha256;
        receipts.sort_unstable();
        if receipts.iter().any(|v| *v == Digest32::zero())
            || receipts.windows(2).any(|v| v[0] == v[1])
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "complete unique receipt set",
            ));
        }
        Ok(())
    }

    fn validate_closure(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        let nonnegative = [
            self.beginning_ice_kg_m2,
            self.beginning_liquid_kg_m2,
            self.beginning_cold_content_j_m2,
            self.ending_ice_kg_m2,
            self.ending_liquid_kg_m2,
            self.ending_cold_content_j_m2,
            self.solid_precipitation_kg_m2,
            self.liquid_precipitation_kg_m2,
            self.deposition_kg_m2,
            self.sublimation_kg_m2,
            self.latent_heat_j_kg,
            self.snow_surface_temperature_k,
            self.melt_kg_m2,
            self.refreeze_kg_m2,
            self.terminal_liquid_kg_m2,
            self.terminal_liquid_sensible_enthalpy_j_m2,
            self.retained_liquid_kg_m2,
        ];
        if nonnegative.iter().any(|v| !v.is_finite() || *v < 0.0) {
            return Err(Stage3PhysicalOutcomeLedgerError::Numeric(
                "nonnegative operand",
            ));
        }
        if self.signed().iter().any(|v| !v.is_finite()) {
            return Err(Stage3PhysicalOutcomeLedgerError::Numeric("signed operand"));
        }
        let vapor = self.deposition_kg_m2 - self.sublimation_kg_m2;
        if !close(vapor, self.vapor_transfer_kg_m2, MASS_TOL)
            || !close(self.latent_j_m2, vapor * self.latent_heat_j_kg, ENERGY_TOL)
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Closure(
                "vapor/latent join",
            ));
        }
        // Refreeze/fusion is an internal exchange between represented ice,
        // liquid, and cold content. The owner-derived material enthalpy
        // already includes it, so its whole-column external contribution must
        // cancel exactly rather than being counted a second time.
        if self.refreeze_fusion_j_m2.to_bits() != 0.0_f64.to_bits() {
            return Err(Stage3PhysicalOutcomeLedgerError::Closure(
                "refreeze/fusion internal cancellation",
            ));
        }
        if !close(
            self.interlayer_active_conduction_j_m2 + self.interlayer_lower_conduction_j_m2,
            self.interlayer_conduction_j_m2,
            ENERGY_TOL,
        ) || self.interlayer_conduction_j_m2.abs() > ENERGY_TOL
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Closure(
                "interlayer equal-and-opposite cancellation",
            ));
        }
        let reconstructed = self.reconstruct();
        let supplied = [
            self.mass_residual_kg_m2,
            self.ice_residual_kg_m2,
            self.liquid_residual_kg_m2,
            self.vapor_residual_kg_m2,
            self.energy_residual_j_m2,
            self.ending_liquid_residual_kg_m2,
        ];
        if reconstructed
            .iter()
            .zip(supplied)
            .any(|(a, b)| a.to_bits() != b.to_bits())
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "reconstructed residual",
            ));
        }
        if supplied[..4].iter().any(|v| v.abs() > MASS_TOL)
            || supplied[4].abs() > ENERGY_TOL
            || supplied[5].abs() > MASS_TOL
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Closure(
                "mass/energy/ending",
            ));
        }
        Ok(())
    }

    fn reconstruct(&self) -> [f64; 6] {
        let vapor = self.deposition_kg_m2 - self.sublimation_kg_m2;
        let ice = self.ending_ice_kg_m2
            - (self.beginning_ice_kg_m2 + self.solid_precipitation_kg_m2 + vapor - self.melt_kg_m2
                + self.refreeze_kg_m2);
        let liquid = self.ending_liquid_kg_m2
            - (self.beginning_liquid_kg_m2 + self.liquid_precipitation_kg_m2 + self.melt_kg_m2
                - self.refreeze_kg_m2
                - self.terminal_liquid_kg_m2);
        let mass = self.ending_ice_kg_m2 + self.ending_liquid_kg_m2
            - (self.beginning_ice_kg_m2
                + self.beginning_liquid_kg_m2
                + self.solid_precipitation_kg_m2
                + self.liquid_precipitation_kg_m2
                + vapor
                - self.terminal_liquid_kg_m2);
        let energy = self.ending_enthalpy_j_m2
            - (self.beginning_enthalpy_j_m2
                + self.shortwave_j_m2
                + self.longwave_j_m2
                + self.sensible_j_m2
                + self.latent_j_m2
                + self.vapor_material_enthalpy_j_m2
                + self.precipitation_advection_j_m2
                + self.soil_heat_j_m2
                + self.interlayer_conduction_j_m2
                + self.refreeze_fusion_j_m2
                // Material enthalpy uses ice at the fusion reference and
                // liquid at +L_f. These boundary phase transports are
                // independent of precipitation sensible advection.
                + LATENT_HEAT_FUSION_J_KG
                    * (self.liquid_precipitation_kg_m2 - self.terminal_liquid_kg_m2)
                - self.terminal_liquid_sensible_enthalpy_j_m2);
        [
            mass,
            ice,
            liquid,
            vapor - self.vapor_transfer_kg_m2,
            energy,
            self.ending_liquid_kg_m2 - self.retained_liquid_kg_m2,
        ]
    }

    fn signed(&self) -> [f64; 20] {
        [
            self.beginning_enthalpy_j_m2,
            self.ending_enthalpy_j_m2,
            self.precipitation_advection_j_m2,
            self.vapor_transfer_kg_m2,
            self.shortwave_j_m2,
            self.longwave_j_m2,
            self.sensible_j_m2,
            self.latent_j_m2,
            self.soil_heat_j_m2,
            self.interlayer_active_conduction_j_m2,
            self.interlayer_lower_conduction_j_m2,
            self.interlayer_conduction_j_m2,
            self.refreeze_fusion_j_m2,
            self.vapor_material_enthalpy_j_m2,
            self.mass_residual_kg_m2,
            self.ice_residual_kg_m2,
            self.liquid_residual_kg_m2,
            self.vapor_residual_kg_m2,
            self.energy_residual_j_m2,
            self.ending_liquid_residual_kg_m2,
        ]
    }

    fn digest(&self) -> Digest32 {
        let mut b = b"OPENWEPP_STAGE3_PHYSICAL_OUTCOME_LEDGER_V1".to_vec();
        b.extend_from_slice(&self.support.start_ns().get().to_le_bytes());
        b.extend_from_slice(&self.support.end_ns().get().to_le_bytes());
        b.extend_from_slice(&self.lane_id.to_le_bytes());
        append_str(&mut b, self.ofe_id.as_str());
        b.push(0);
        for d in [
            self.topology_sha256,
            self.beginning_snow_owner_sha256,
            self.ending_snow_owner_sha256,
            self.precipitation_set_sha256,
        ]
        .iter()
        .chain(self.source_receipts_sha256.iter())
        {
            b.extend_from_slice(d.as_bytes());
        }
        for v in self.values() {
            b.extend_from_slice(&v.to_bits().to_le_bytes());
        }
        for value in &self.destination_liquid_outcomes {
            append_str(&mut b, value.ofe_id.as_str());
            append_str(&mut b, value.tile_id.as_str());
            b.extend_from_slice(&value.tile_fraction.to_bits().to_le_bytes());
            b.extend_from_slice(&value.mass_kg_m2_tile_ground.to_bits().to_le_bytes());
            b.extend_from_slice(
                &value
                    .sensible_enthalpy_j_m2_tile_ground
                    .to_bits()
                    .to_le_bytes(),
            );
        }
        digest_bytes(&b)
    }

    fn values(&self) -> [f64; 37] {
        [
            self.beginning_ice_kg_m2,
            self.beginning_liquid_kg_m2,
            self.beginning_cold_content_j_m2,
            self.beginning_enthalpy_j_m2,
            self.ending_ice_kg_m2,
            self.ending_liquid_kg_m2,
            self.ending_cold_content_j_m2,
            self.ending_enthalpy_j_m2,
            self.solid_precipitation_kg_m2,
            self.liquid_precipitation_kg_m2,
            self.precipitation_advection_j_m2,
            self.deposition_kg_m2,
            self.sublimation_kg_m2,
            self.vapor_transfer_kg_m2,
            self.latent_heat_j_kg,
            self.snow_surface_temperature_k,
            self.vapor_material_enthalpy_j_m2,
            self.melt_kg_m2,
            self.refreeze_kg_m2,
            self.terminal_liquid_kg_m2,
            self.terminal_liquid_sensible_enthalpy_j_m2,
            self.retained_liquid_kg_m2,
            self.shortwave_j_m2,
            self.longwave_j_m2,
            self.sensible_j_m2,
            self.latent_j_m2,
            self.soil_heat_j_m2,
            self.interlayer_active_conduction_j_m2,
            self.interlayer_lower_conduction_j_m2,
            self.interlayer_conduction_j_m2,
            self.refreeze_fusion_j_m2,
            self.mass_residual_kg_m2,
            self.ice_residual_kg_m2,
            self.liquid_residual_kg_m2,
            self.vapor_residual_kg_m2,
            self.energy_residual_j_m2,
            self.ending_liquid_residual_kg_m2,
        ]
    }
}

fn close(a: f64, b: f64, tolerance: f64) -> bool {
    a.is_finite() && b.is_finite() && (a - b).abs() <= tolerance
}
fn append_str(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_coupled_time::ModelTimeNs;
    use openwepp_kernel_contract::SoilLayerId;

    fn d(v: u8) -> Digest32 {
        Digest32::from_bytes([v; 32])
    }
    fn fixture() -> (
        Stage3LanePhysicalOutcomeLedgerV1,
        Stage3LanePhysicalOutcomeExpectationV1,
    ) {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("support");
        let expected = Stage3LanePhysicalOutcomeExpectationV1 {
            support,
            lane_id: 2,
            ofe_id: OfeId::try_new("ofe-2").expect("OFE"),
            topology_sha256: d(1),
            beginning_snow_owner_sha256: d(2),
            ending_snow_owner_sha256: d(3),
            precipitation_set_sha256: d(4),
            source_receipts_sha256: [d(5), d(6), d(7), d(8), d(9), d(10)],
        };
        let destination_ofe = OfeId::try_new("ofe-2").expect("OFE");
        let destination_tile_a = TileId::try_new("tile-a").expect("tile");
        let destination_tile_b = TileId::try_new("tile-b").expect("tile");
        let (
            destination_liquid_outcomes,
            terminal_liquid_kg_m2,
            terminal_liquid_sensible_enthalpy_j_m2,
        ) = seal_destination_liquid_outcomes_v1(
            &destination_ofe,
            [(&destination_tile_a, 0.38), (&destination_tile_b, 0.62)],
            0.15,
            0.0,
        )
        .expect("destination outcomes");
        let value = Stage3LanePhysicalOutcomeLedgerV1 {
            support,
            lane_id: 2,
            ofe_id: destination_ofe,
            area_basis: Stage3LaneAreaBasisV1::OfeGround,
            topology_sha256: d(1),
            beginning_snow_owner_sha256: d(2),
            ending_snow_owner_sha256: d(3),
            precipitation_set_sha256: d(4),
            source_receipts_sha256: expected.source_receipts_sha256,
            beginning_ice_kg_m2: 10.0,
            beginning_liquid_kg_m2: 1.0,
            beginning_cold_content_j_m2: 100.0,
            beginning_enthalpy_j_m2: -100.0,
            ending_ice_kg_m2: 10.6,
            ending_liquid_kg_m2: 1.05,
            ending_cold_content_j_m2: 75.0,
            ending_enthalpy_j_m2: 16_606.0,
            solid_precipitation_kg_m2: 0.5,
            liquid_precipitation_kg_m2: 0.2,
            precipitation_advection_j_m2: 2.0,
            deposition_kg_m2: 0.2,
            sublimation_kg_m2: 0.1,
            vapor_transfer_kg_m2: 0.1,
            latent_heat_j_kg: 100.0,
            snow_surface_temperature_k: 273.15,
            vapor_material_enthalpy_j_m2: 0.0,
            melt_kg_m2: 0.1,
            refreeze_kg_m2: 0.1,
            terminal_liquid_kg_m2,
            terminal_liquid_sensible_enthalpy_j_m2,
            destination_liquid_outcomes,
            retained_liquid_kg_m2: 1.05,
            shortwave_j_m2: 20.0,
            longwave_j_m2: -5.0,
            sensible_j_m2: -4.0,
            latent_j_m2: 10.0,
            soil_heat_j_m2: 3.0,
            interlayer_active_conduction_j_m2: 2.0,
            interlayer_lower_conduction_j_m2: -2.0,
            interlayer_conduction_j_m2: 0.0,
            refreeze_fusion_j_m2: 0.0,
            mass_residual_kg_m2: 0.0,
            ice_residual_kg_m2: 0.0,
            liquid_residual_kg_m2: 0.0,
            vapor_residual_kg_m2: 0.0,
            energy_residual_j_m2: 0.0,
            ending_liquid_residual_kg_m2: 0.0,
            receipt_sha256: Digest32::zero(),
        };
        (value, expected)
    }

    #[test]
    fn independent_ledger_closes() {
        let (value, expected) = fixture();
        Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &expected).expect("closed ledger");
    }

    #[test]
    fn destination_vector_order_cardinality_and_redistribution_poisons_fail_closed() {
        let (value, expected) = fixture();
        let sealed = Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &expected)
            .expect("closed destination vector");
        let reseal = |mut value: Stage3LanePhysicalOutcomeLedgerV1| {
            value.receipt_sha256 = Digest32::zero();
            value.receipt_sha256 = value.digest();
            value
        };

        let mut permuted = sealed.clone();
        permuted.destination_liquid_outcomes.swap(0, 1);
        assert!(reseal(permuted).validate(&expected).is_err());

        let mut omitted = sealed.clone();
        omitted.destination_liquid_outcomes.pop();
        assert!(reseal(omitted).validate(&expected).is_err());

        let mut duplicated = sealed.clone();
        duplicated
            .destination_liquid_outcomes
            .push(duplicated.destination_liquid_outcomes[0].clone());
        assert!(reseal(duplicated).validate(&expected).is_err());

        let mut redistributed = sealed;
        redistributed.destination_liquid_outcomes[0].mass_kg_m2_tile_ground = 0.16;
        redistributed.destination_liquid_outcomes[1].mass_kg_m2_tile_ground =
            (0.15 - 0.38 * 0.16) / 0.62;
        assert!(reseal(redistributed).validate(&expected).is_err());
    }

    #[test]
    fn terminal_snow_soil_receipt_has_no_endpoint_temperature_and_closes_custody() {
        let support =
            TimeSupport::new(ModelTimeNs::new(10), ModelTimeNs::new(20)).expect("support");
        let receipt = TerminalSnowSoilHeatReceiptV1 {
            support,
            lane_id: 7,
            ofe_id: OfeId::try_new("ofe-7").expect("ofe"),
            beginning_snow_owner_sha256: d(1),
            ending_dormant_snow_owner_sha256: d(2),
            ending_soil_owner_sha256: d(3),
            limiting_boundary_receipt_sha256: d(4),
            snow_heat_j_m2: -12.5,
            soil_heat_j_m2: 12.5,
            receipt_sha256: Digest32::zero(),
        }
        .seal()
        .expect("terminal receipt");
        assert_ne!(receipt.receipt_sha256, Digest32::zero());
    }

    #[test]
    fn terminal_bottom_soil_trial_closes_and_matches_one_volume_persistent_conductivity() {
        let support = TimeSupport::new(
            ModelTimeNs::new(1_000_000_000),
            ModelTimeNs::new(61_000_000_000),
        )
        .expect("support");
        let state = crate::hydrology::Wb11HydrologyKernel::initialize_stage3_persistent_state(
            41,
            vec![crate::DirectSnowLayerState {
                mass_swe_m: 0.08,
                thickness_m: 0.20,
                density_kg_m3: 400.0,
                settle_day_count: 1.0,
                temperature_c: -4.0,
                liquid_water_m: 0.0,
                cold_content_j_m2: 672_000.0,
                refrozen_liquid_m: 0.0,
            }],
        )
        .expect("persistent state");
        let projection = crate::hydrology::Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
            &state, 101_324.6,
        )
        .expect("persistent bottom projection");
        let layer_id = SoilLayerId::try_new("thermal-top").expect("layer");
        let config = openwepp_land_surface_energy::SoilInterfaceLayer {
            layer_id: layer_id.clone(),
            thickness_m: 0.10,
            thermal_conductivity_w_m_k: 1.5,
            areal_heat_capacity_j_m2_k: 120_000.0,
        };
        let beginning = openwepp_land_surface_energy::SoilThermalLayerSnapshot {
            layer_id,
            temperature_k: 271.15,
            enthalpy_j_m2_ofe_ground: -240_000.0,
        };
        let result =
            evaluate_terminal_snow_bottom_soil_trial_v1(&TerminalSnowBottomSoilTrialInputsV1 {
                support,
                lane_id: 41,
                ofe_id: &OfeId::try_new("ofe-41").expect("ofe"),
                canonical_source_sha256: projection.beginning_stage3_state_sha256,
                ice_kg_m2: 80.0,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 672_000.0,
                depth_m: projection.thickness_m,
                density_kg_m3: 400.0,
                temperature_k: projection.temperature_k,
                atmospheric_pressure_pa: 101_324.6,
                first_soil_configuration: &config,
                beginning_first_soil: &beginning,
            })
            .expect("terminal trial");
        let resistance = 0.5 * projection.thickness_m / projection.thermal_conductivity_w_m_k
            + 0.5 * config.thickness_m / config.thermal_conductivity_w_m_k;
        let conductance = 1.0 / resistance;
        let snow_capacity = 80.0 * 2_100.0;
        let expected_soil_heat =
            60.0 * conductance * (projection.temperature_k - beginning.temperature_k)
                / (1.0
                    + 0.5
                        * 60.0
                        * conductance
                        * (1.0 / snow_capacity + 1.0 / config.areal_heat_capacity_j_m2_k));
        assert_eq!(
            result.soil_heat_j_m2.to_bits(),
            expected_soil_heat.to_bits()
        );
        assert_eq!(
            result.snow_heat_j_m2.to_bits(),
            (-result.soil_heat_j_m2).to_bits()
        );
        assert!(
            ((result.ending_first_soil.enthalpy_j_m2_ofe_ground
                - beginning.enthalpy_j_m2_ofe_ground)
                - result.soil_heat_j_m2)
                .abs()
                <= 1.0e-9
        );
        let beginning_flux = conductance * (projection.temperature_k - beginning.temperature_k);
        let ending_flux = conductance
            * (result.ending_snow_temperature_k - result.ending_first_soil.temperature_k);
        assert!(
            (result.soil_heat_j_m2 - 0.5 * (beginning_flux + ending_flux) * 60.0).abs() <= 1.0e-9
        );
        result.receipt.validate().expect("sealed receipt");
    }

    #[test]
    fn terminal_bottom_soil_trial_rejects_inconsistent_scalar_volume() {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_000_000_000))
            .expect("support");
        let layer_id = SoilLayerId::try_new("thermal-top").expect("layer");
        let config = openwepp_land_surface_energy::SoilInterfaceLayer {
            layer_id: layer_id.clone(),
            thickness_m: 0.1,
            thermal_conductivity_w_m_k: 1.5,
            areal_heat_capacity_j_m2_k: 120_000.0,
        };
        let beginning = openwepp_land_surface_energy::SoilThermalLayerSnapshot {
            layer_id,
            temperature_k: 271.15,
            enthalpy_j_m2_ofe_ground: 0.0,
        };
        let result =
            evaluate_terminal_snow_bottom_soil_trial_v1(&TerminalSnowBottomSoilTrialInputsV1 {
                support,
                lane_id: 1,
                ofe_id: &OfeId::try_new("ofe-1").expect("ofe"),
                canonical_source_sha256: d(1),
                ice_kg_m2: 79.0,
                liquid_kg_m2: 0.0,
                cold_content_j_m2: 1.0,
                depth_m: 0.2,
                density_kg_m3: 400.0,
                temperature_k: 269.15,
                atmospheric_pressure_pa: 101_324.6,
                first_soil_configuration: &config,
                beginning_first_soil: &beginning,
            });
        assert!(matches!(
            result,
            Err(Stage3PhysicalOutcomeLedgerError::Numeric(
                "terminal snow-bottom soil trial operands"
            ))
        ));
    }

    #[test]
    fn identity_and_physics_poisons_fail_closed() {
        let (value, expected) = fixture();
        let sealed = Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &expected).expect("ledger");
        let poisons: [fn(&mut Stage3LanePhysicalOutcomeLedgerV1); 14] = [
            |v| v.receipt_sha256 = d(90),
            |v| v.precipitation_set_sha256 = d(91),
            |v| v.beginning_snow_owner_sha256 = d(92),
            |v| v.ending_snow_owner_sha256 = d(93),
            |v| v.source_receipts_sha256[0] = Digest32::zero(),
            |v| v.source_receipts_sha256[5] = v.source_receipts_sha256[4],
            |v| v.shortwave_j_m2 = -v.shortwave_j_m2,
            |v| v.latent_j_m2 = -v.latent_j_m2,
            |v| v.vapor_transfer_kg_m2 = -v.vapor_transfer_kg_m2,
            |v| v.interlayer_conduction_j_m2 = 1.0,
            |v| v.interlayer_lower_conduction_j_m2 += 1.0,
            |v| v.refreeze_fusion_j_m2 = 1.0,
            |v| v.vapor_material_enthalpy_j_m2 += 1.0,
            |v| v.ending_liquid_kg_m2 += 0.01,
        ];
        for poison in poisons {
            let mut candidate = sealed.clone();
            poison(&mut candidate);
            assert!(candidate.validate(&expected).is_err());
        }
    }

    #[test]
    fn resealed_substitution_does_not_replace_external_authority() {
        let (mut value, expected) = fixture();
        let mut substituted_expected = expected.clone();
        substituted_expected.ending_snow_owner_sha256 = d(77);
        value.ending_snow_owner_sha256 = d(77);
        let substituted = Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &substituted_expected)
            .expect("internally valid substituted receipt");
        assert!(substituted.validate(&expected).is_err());
    }

    #[test]
    fn resealed_ending_material_owner_mutation_fails_physical_closure() {
        let (mut value, expected) = fixture();
        value.ending_snow_owner_sha256 = d(77);
        value.ending_liquid_kg_m2 += 0.01;
        value.retained_liquid_kg_m2 += 0.01;
        value.ending_cold_content_j_m2 += 3_336.0;
        value.ending_enthalpy_j_m2 =
            -value.ending_cold_content_j_m2 + 333_600.0 * value.ending_liquid_kg_m2;
        let mut substituted_expected = expected.clone();
        substituted_expected.ending_snow_owner_sha256 = d(77);
        assert!(Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &substituted_expected).is_err());
    }
}
