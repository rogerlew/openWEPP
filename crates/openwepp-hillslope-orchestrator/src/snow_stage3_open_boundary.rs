//! Destination-keyed exposed-snow boundary production for Stage 3.
//!
//! This module is deliberately separate from the canopy carrier. An open-snow
//! tile terminates turbulent and longwave exchange at the sealed reference
//! atmosphere and never acquires a V11 canopy-air or reciprocal-canopy receipt.

use openwepp_coupled_time::{Digest32, FramedField, TimeSupport, framed_sha256};
use openwepp_kernel_contract::TileId;
use openwepp_land_surface_energy::OfeId;
use openwepp_land_surface_energy::physics::{OpenNeutralGeometry, open_neutral_resistances};
use openwepp_meteorology::surface_energy::{
    latent_heat_for_surface_temperature, saturation_vapor_pressure_snobal_pa,
};
use openwepp_unit_boundary::TemperatureCelsius;

use crate::hydrology::{
    DirectActiveSnowPartitionInputs, DirectSnowStage3PersistentState, STAGE3_DEFAULT_SNOW_ALBEDO,
};
use crate::snow_stage3_terminal_handoff::FinalStage3CanopyBoundaryReceiptV1;
use crate::snow_stage3_terminal_handoff::SealedCoveredCarrierForcing;
use crate::snow_stage3_terminal_handoff::SnowStage3HandoffError;

const OPEN_SNOW_TRANSFER_HEIGHT_M: f64 = 5.0;
const OPEN_SNOW_ROUGHNESS_M: f64 = 0.005;
const SIGMA_W_M2_K4: f64 = 5.670_374_419e-8;

/// Closed physical forcing class for every active Stage 3 snow-surface tile.
#[derive(Clone, Debug, PartialEq)]
pub enum SealedStage3TileBoundaryForcingV1 {
    V11CanopyCovered(SealedCoveredCarrierForcing),
    OpenSnow(SealedOpenSnowTileForcingV1),
}

impl SealedStage3TileBoundaryForcingV1 {
    #[must_use]
    pub const fn is_covered(&self) -> bool {
        matches!(self, Self::V11CanopyCovered(_))
    }

    #[must_use]
    pub const fn is_open_snow(&self) -> bool {
        matches!(self, Self::OpenSnow(_))
    }
}

/// Closed final destination receipt set consumed by Stage 3 lane aggregation.
#[derive(Clone, Debug, PartialEq)]
pub enum FinalStage3TileBoundaryReceiptV1 {
    V11Canopy(FinalStage3CanopyBoundaryReceiptV1),
    OpenSnow(FinalStage3OpenSnowBoundaryReceiptV1),
}

impl FinalStage3TileBoundaryReceiptV1 {
    pub fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        match self {
            Self::V11Canopy(value) => value.validate(),
            Self::OpenSnow(value) => value.validate(),
        }
    }

    #[must_use]
    pub fn destination(&self) -> &(OfeId, TileId) {
        match self {
            Self::V11Canopy(value) => &value.destination,
            Self::OpenSnow(value) => &value.candidate.destination,
        }
    }

    #[must_use]
    pub fn receipt_sha256(&self) -> &Digest32 {
        match self {
            Self::V11Canopy(value) => &value.receipt_sha256,
            Self::OpenSnow(value) => &value.receipt_sha256,
        }
    }

    #[must_use]
    pub fn beginning_stage3_state_sha256(&self) -> Digest32 {
        match self {
            Self::V11Canopy(value) => value.beginning_stage3_state_sha256,
            Self::OpenSnow(value) => value.candidate.beginning_stage3_state_sha256,
        }
    }

    #[must_use]
    pub fn ending_stage3_state_sha256(&self) -> Digest32 {
        match self {
            Self::V11Canopy(value) => value.ending_stage3_state_sha256,
            Self::OpenSnow(value) => value.ending_stage3_state_sha256,
        }
    }

    #[must_use]
    pub fn source_digests(&self) -> (Digest32, Digest32, Digest32, Digest32) {
        match self {
            Self::V11Canopy(value) => (
                value.provisional_carrier_receipt_sha256,
                value.optical_receipt_sha256,
                value.reciprocal_longwave_receipt_sha256,
                value.receipt_sha256,
            ),
            Self::OpenSnow(value) => (
                value.candidate.exposure_receipt_sha256,
                value.candidate.optical_receipt_sha256,
                value.candidate.longwave_receipt_sha256,
                value.receipt_sha256,
            ),
        }
    }

    #[must_use]
    /// Returns tile-boundary exchange positive outward from the snow for the
    /// first three operands, followed by snow-owned radiative/state operands.
    pub fn physical_operands(&self) -> [f64; 7] {
        match self {
            Self::V11Canopy(value) => [
                value.sensible_to_canopy_air_w_m2,
                value.vapor_to_canopy_air_kg_m2_s,
                value.latent_energy_to_canopy_air_j_m2,
                value.snow_absorbed_shortwave_w_m2,
                value.snow_net_longwave_w_m2,
                value.snow_temperature_k,
                value.latent_heat_j_kg,
            ],
            Self::OpenSnow(value) => [
                value.candidate.sensible_outward_w_m2,
                value.candidate.vapor_outward_kg_m2_s,
                value.candidate.latent_energy_outward_j_m2,
                value.candidate.snow_absorbed_shortwave_w_m2,
                value.candidate.snow_net_longwave_w_m2,
                value.candidate.snow_temperature_k,
                value.candidate.latent_heat_j_kg,
            ],
        }
    }
}

fn append_destination(bytes: &mut Vec<u8>, destination: &(OfeId, TileId)) {
    for value in [destination.0.as_str(), destination.1.as_str()] {
        bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }
}

/// Destination-specific exposed-snow aerodynamic authority.
#[derive(Clone, Debug, PartialEq)]
pub struct SealedOpenSnowExposureReceiptV1 {
    pub support: TimeSupport,
    pub destination: (OfeId, TileId),
    pub source_forcing_receipt_sha256: Digest32,
    pub source_wind_provider_sha256: Digest32,
    pub raw_or_projected_wind_m_s: f64,
    pub transfer_height_m: f64,
    pub roughness_m: f64,
    pub projection_model_definition_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl SealedOpenSnowExposureReceiptV1 {
    pub fn try_new(
        support: TimeSupport,
        destination: (OfeId, TileId),
        source_forcing_receipt_sha256: Digest32,
        source_wind_provider_sha256: Digest32,
        raw_or_projected_wind_m_s: f64,
        projection_model_definition_sha256: Digest32,
    ) -> Result<Self, SnowStage3HandoffError> {
        let mut value = Self {
            support,
            destination,
            source_forcing_receipt_sha256,
            source_wind_provider_sha256,
            raw_or_projected_wind_m_s,
            transfer_height_m: OPEN_SNOW_TRANSFER_HEIGHT_M,
            roughness_m: OPEN_SNOW_ROUGHNESS_M,
            projection_model_definition_sha256,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        if self.destination.0.as_str().is_empty()
            || self.destination.1.as_str().is_empty()
            || self.source_forcing_receipt_sha256 == Digest32::zero()
            || self.source_wind_provider_sha256 == Digest32::zero()
            || self.projection_model_definition_sha256 == Digest32::zero()
            || !self.raw_or_projected_wind_m_s.is_finite()
            || self.raw_or_projected_wind_m_s <= 0.0
            || self.transfer_height_m.to_bits() != OPEN_SNOW_TRANSFER_HEIGHT_M.to_bits()
            || self.roughness_m.to_bits() != OPEN_SNOW_ROUGHNESS_M.to_bits()
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(SnowStage3HandoffError::InvalidExposure(
                "destination-specific open-snow exposure seal",
            ));
        }
        Ok(())
    }

    fn reconstructed_digest(&self) -> Result<Digest32, SnowStage3HandoffError> {
        let mut identity = Vec::new();
        identity.extend_from_slice(&self.support.start_ns().get().to_be_bytes());
        identity.extend_from_slice(&self.support.end_ns().get().to_be_bytes());
        append_destination(&mut identity, &self.destination);
        let scalars = [
            self.raw_or_projected_wind_m_s,
            self.transfer_height_m,
            self.roughness_m,
        ]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
        framed_sha256(
            "openwepp-stage3-open-snow-exposure-v1",
            &[
                FramedField {
                    tag: "identity",
                    value: &identity,
                },
                FramedField {
                    tag: "source_forcing",
                    value: self.source_forcing_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "wind_provider",
                    value: self.source_wind_provider_sha256.as_bytes(),
                },
                FramedField {
                    tag: "projection_model",
                    value: self.projection_model_definition_sha256.as_bytes(),
                },
                FramedField {
                    tag: "geometry",
                    value: &scalars,
                },
            ],
        )
        .map_err(|_| SnowStage3HandoffError::InvalidExposure("open-snow exposure framing"))
    }
}

/// Sealed external operands for one open-snow tile.
#[derive(Clone, Debug, PartialEq)]
pub struct SealedOpenSnowTileForcingV1 {
    pub support: TimeSupport,
    pub destination: (OfeId, TileId),
    pub forcing_receipt_sha256: Digest32,
    pub exposure: SealedOpenSnowExposureReceiptV1,
    pub rho_air_kg_m3: f64,
    pub cp_air_j_kg_k: f64,
    pub reference_temperature_k: f64,
    pub reference_specific_humidity_kg_kg: f64,
    pub air_pressure_pa: f64,
    pub atmospheric_downward_longwave_w_m2: f64,
    pub direct_vis_w_m2: f64,
    pub diffuse_vis_w_m2: f64,
    pub direct_nir_w_m2: f64,
    pub diffuse_nir_w_m2: f64,
    pub rain_m: f64,
    pub snowfall_m: f64,
    pub precipitation_parcel_count: usize,
    pub receipt_sha256: Digest32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SealedOpenSnowTileForcingInputsV1 {
    pub support: TimeSupport,
    pub destination: (OfeId, TileId),
    pub forcing_receipt_sha256: Digest32,
    pub exposure: SealedOpenSnowExposureReceiptV1,
    pub rho_air_kg_m3: f64,
    pub cp_air_j_kg_k: f64,
    pub reference_temperature_k: f64,
    pub reference_specific_humidity_kg_kg: f64,
    pub air_pressure_pa: f64,
    pub atmospheric_downward_longwave_w_m2: f64,
    pub direct_vis_w_m2: f64,
    pub diffuse_vis_w_m2: f64,
    pub direct_nir_w_m2: f64,
    pub diffuse_nir_w_m2: f64,
    pub rain_m: f64,
    pub snowfall_m: f64,
    pub precipitation_parcel_count: usize,
}

impl SealedOpenSnowTileForcingV1 {
    pub fn try_new(
        inputs: SealedOpenSnowTileForcingInputsV1,
    ) -> Result<Self, SnowStage3HandoffError> {
        if inputs.rain_m.to_bits() != 0.0_f64.to_bits()
            || inputs.snowfall_m.to_bits() != 0.0_f64.to_bits()
            || inputs.precipitation_parcel_count != 0
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "open-snow precipitation custody is unavailable",
            ));
        }
        let mut value = Self {
            support: inputs.support,
            destination: inputs.destination,
            forcing_receipt_sha256: inputs.forcing_receipt_sha256,
            exposure: inputs.exposure,
            rho_air_kg_m3: inputs.rho_air_kg_m3,
            cp_air_j_kg_k: inputs.cp_air_j_kg_k,
            reference_temperature_k: inputs.reference_temperature_k,
            reference_specific_humidity_kg_kg: inputs.reference_specific_humidity_kg_kg,
            air_pressure_pa: inputs.air_pressure_pa,
            atmospheric_downward_longwave_w_m2: inputs.atmospheric_downward_longwave_w_m2,
            direct_vis_w_m2: inputs.direct_vis_w_m2,
            diffuse_vis_w_m2: inputs.diffuse_vis_w_m2,
            direct_nir_w_m2: inputs.direct_nir_w_m2,
            diffuse_nir_w_m2: inputs.diffuse_nir_w_m2,
            rain_m: inputs.rain_m,
            snowfall_m: inputs.snowfall_m,
            precipitation_parcel_count: inputs.precipitation_parcel_count,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        self.exposure.validate()?;
        let finite = [
            self.rho_air_kg_m3,
            self.cp_air_j_kg_k,
            self.reference_temperature_k,
            self.reference_specific_humidity_kg_kg,
            self.air_pressure_pa,
            self.atmospheric_downward_longwave_w_m2,
            self.direct_vis_w_m2,
            self.diffuse_vis_w_m2,
            self.direct_nir_w_m2,
            self.diffuse_nir_w_m2,
        ]
        .iter()
        .all(|value| value.is_finite());
        if self.support != self.exposure.support
            || self.destination != self.exposure.destination
            || self.forcing_receipt_sha256 == Digest32::zero()
            || self.forcing_receipt_sha256 != self.exposure.source_forcing_receipt_sha256
            || !finite
            || self.rho_air_kg_m3 <= 0.0
            || self.cp_air_j_kg_k <= 0.0
            || self.reference_temperature_k <= 0.0
            || !(0.0..=1.0).contains(&self.reference_specific_humidity_kg_kg)
            || self.air_pressure_pa <= 0.0
            || self.rain_m.to_bits() != 0.0_f64.to_bits()
            || self.snowfall_m.to_bits() != 0.0_f64.to_bits()
            || self.precipitation_parcel_count != 0
            || [
                self.atmospheric_downward_longwave_w_m2,
                self.direct_vis_w_m2,
                self.diffuse_vis_w_m2,
                self.direct_nir_w_m2,
                self.diffuse_nir_w_m2,
            ]
            .iter()
            .any(|value| *value < 0.0)
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "sealed destination open-snow forcing",
            ));
        }
        Ok(())
    }

    fn reconstructed_digest(&self) -> Result<Digest32, SnowStage3HandoffError> {
        let mut identity = Vec::new();
        identity.extend_from_slice(&self.support.start_ns().get().to_be_bytes());
        identity.extend_from_slice(&self.support.end_ns().get().to_be_bytes());
        append_destination(&mut identity, &self.destination);
        let scalars = [
            self.rho_air_kg_m3,
            self.cp_air_j_kg_k,
            self.reference_temperature_k,
            self.reference_specific_humidity_kg_kg,
            self.air_pressure_pa,
            self.atmospheric_downward_longwave_w_m2,
            self.direct_vis_w_m2,
            self.diffuse_vis_w_m2,
            self.direct_nir_w_m2,
            self.diffuse_nir_w_m2,
            self.rain_m,
            self.snowfall_m,
        ]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
        let precipitation_count = (self.precipitation_parcel_count as u64).to_be_bytes();
        framed_sha256(
            "openwepp-stage3-open-snow-tile-forcing-v1",
            &[
                FramedField {
                    tag: "identity",
                    value: &identity,
                },
                FramedField {
                    tag: "forcing",
                    value: self.forcing_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "exposure",
                    value: self.exposure.receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "atmosphere",
                    value: &scalars,
                },
                FramedField {
                    tag: "precipitation_count",
                    value: &precipitation_count,
                },
            ],
        )
        .map_err(|_| SnowStage3HandoffError::InvalidCarrier("open-snow forcing framing"))
    }
}

/// Physical open-snow boundary evaluated from current Stage 3 state.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenSnowTileBoundaryCandidateV1 {
    pub support: TimeSupport,
    pub destination: (OfeId, TileId),
    pub beginning_stage3_state_sha256: Digest32,
    pub forcing_receipt_sha256: Digest32,
    pub exposure_receipt_sha256: Digest32,
    pub optical_receipt_sha256: Digest32,
    pub longwave_receipt_sha256: Digest32,
    pub snow_temperature_k: f64,
    pub latent_heat_j_kg: f64,
    pub sensible_outward_w_m2: f64,
    pub vapor_outward_kg_m2_s: f64,
    pub latent_energy_outward_j_m2: f64,
    pub snow_absorbed_shortwave_w_m2: f64,
    pub snow_net_longwave_w_m2: f64,
}

/// Final accepted open-snow boundary. It is physically disjoint from the
/// canopy receipt and seals the exact Stage 3 ending selected by replay.
#[derive(Clone, Debug, PartialEq)]
pub struct FinalStage3OpenSnowBoundaryReceiptV1 {
    pub candidate: OpenSnowTileBoundaryCandidateV1,
    pub ending_stage3_state_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl FinalStage3OpenSnowBoundaryReceiptV1 {
    pub fn try_new(
        candidate: OpenSnowTileBoundaryCandidateV1,
        ending_stage3_state_sha256: Digest32,
    ) -> Result<Self, SnowStage3HandoffError> {
        if ending_stage3_state_sha256 == Digest32::zero() {
            return Err(SnowStage3HandoffError::InvalidState(
                "open-snow ending Stage 3 state",
            ));
        }
        let mut value = Self {
            candidate,
            ending_stage3_state_sha256,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), SnowStage3HandoffError> {
        let values = [
            self.candidate.snow_temperature_k,
            self.candidate.latent_heat_j_kg,
            self.candidate.sensible_outward_w_m2,
            self.candidate.vapor_outward_kg_m2_s,
            self.candidate.latent_energy_outward_j_m2,
            self.candidate.snow_absorbed_shortwave_w_m2,
            self.candidate.snow_net_longwave_w_m2,
        ];
        if self.candidate.beginning_stage3_state_sha256 == Digest32::zero()
            || self.ending_stage3_state_sha256 == Digest32::zero()
            || self.candidate.forcing_receipt_sha256 == Digest32::zero()
            || self.candidate.exposure_receipt_sha256 == Digest32::zero()
            || self.candidate.optical_receipt_sha256 == Digest32::zero()
            || self.candidate.longwave_receipt_sha256 == Digest32::zero()
            || values.iter().any(|value| !value.is_finite())
            || self.candidate.latent_heat_j_kg <= 0.0
            || !(200.0..=350.0).contains(&self.candidate.snow_temperature_k)
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(SnowStage3HandoffError::InvalidCarrier(
                "final open-snow boundary seal",
            ));
        }
        Ok(())
    }

    fn reconstructed_digest(&self) -> Result<Digest32, SnowStage3HandoffError> {
        let mut identity = Vec::new();
        identity.extend_from_slice(&self.candidate.support.start_ns().get().to_be_bytes());
        identity.extend_from_slice(&self.candidate.support.end_ns().get().to_be_bytes());
        append_destination(&mut identity, &self.candidate.destination);
        let scalars = [
            self.candidate.snow_temperature_k,
            self.candidate.latent_heat_j_kg,
            self.candidate.sensible_outward_w_m2,
            self.candidate.vapor_outward_kg_m2_s,
            self.candidate.latent_energy_outward_j_m2,
            self.candidate.snow_absorbed_shortwave_w_m2,
            self.candidate.snow_net_longwave_w_m2,
        ]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
        framed_sha256(
            "openwepp-final-stage3-open-snow-boundary-v1",
            &[
                FramedField {
                    tag: "identity",
                    value: &identity,
                },
                FramedField {
                    tag: "beginning_stage3",
                    value: self.candidate.beginning_stage3_state_sha256.as_bytes(),
                },
                FramedField {
                    tag: "ending_stage3",
                    value: self.ending_stage3_state_sha256.as_bytes(),
                },
                FramedField {
                    tag: "forcing",
                    value: self.candidate.forcing_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "exposure",
                    value: self.candidate.exposure_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "optical",
                    value: self.candidate.optical_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "longwave",
                    value: self.candidate.longwave_receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "physics",
                    value: &scalars,
                },
            ],
        )
        .map_err(|_| SnowStage3HandoffError::InvalidCarrier("open-snow boundary framing"))
    }
}

pub fn evaluate_open_snow_tile_boundary(
    beginning_stage3: &DirectSnowStage3PersistentState,
    beginning_stage3_state_sha256: Digest32,
    stage3_inputs: &DirectActiveSnowPartitionInputs,
    forcing: &SealedOpenSnowTileForcingV1,
) -> Result<OpenSnowTileBoundaryCandidateV1, SnowStage3HandoffError> {
    forcing.validate()?;
    if beginning_stage3_state_sha256 == Digest32::zero() || beginning_stage3.layers.is_empty() {
        return Err(SnowStage3HandoffError::InvalidState(
            "open-snow beginning Stage 3 state",
        ));
    }
    let snow_temperature_k = beginning_stage3.layers[0].temperature_c + 273.15;
    let temperature = TemperatureCelsius::try_new(beginning_stage3.layers[0].temperature_c)
        .map_err(|_| SnowStage3HandoffError::InvalidState("open-snow temperature"))?;
    let saturation_pressure_pa = saturation_vapor_pressure_snobal_pa(temperature)
        .map_err(|_| SnowStage3HandoffError::InvalidState("open-snow saturation pressure"))?
        .as_pascals();
    if forcing.air_pressure_pa <= 0.378 * saturation_pressure_pa {
        return Err(SnowStage3HandoffError::InvalidCarrier(
            "open-snow saturation specific humidity pressure",
        ));
    }
    let surface_humidity =
        0.622 * saturation_pressure_pa / (forcing.air_pressure_pa - 0.378 * saturation_pressure_pa);
    let resistance = open_neutral_resistances(
        OpenNeutralGeometry {
            reference_height_m: forcing.exposure.transfer_height_m,
            roughness_momentum_m: forcing.exposure.roughness_m,
            roughness_heat_m: forcing.exposure.roughness_m,
            roughness_vapor_m: forcing.exposure.roughness_m,
        },
        forcing.exposure.raw_or_projected_wind_m_s,
    )
    .map_err(|_| SnowStage3HandoffError::InvalidExposure("open-snow neutral resistance"))?;
    let sensible_outward_w_m2 = forcing.rho_air_kg_m3
        * forcing.cp_air_j_kg_k
        * (snow_temperature_k - forcing.reference_temperature_k)
        / resistance.heat_s_m;
    let vapor_outward_kg_m2_s = forcing.rho_air_kg_m3
        * (surface_humidity - forcing.reference_specific_humidity_kg_kg)
        / resistance.vapor_s_m;
    let latent_heat_j_kg = latent_heat_for_surface_temperature(temperature)
        .map_err(|_| SnowStage3HandoffError::InvalidState("open-snow latent heat"))?
        .as_joules_per_kilogram();
    let latent_energy_outward_j_m2 = vapor_outward_kg_m2_s
        * latent_heat_j_kg
        * f64::from_bits(forcing.support.duration_s_bits());
    let albedo = stage3_inputs
        .snow_albedo_state
        .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
    if !albedo.is_finite() || !(0.0..=1.0).contains(&albedo) {
        return Err(SnowStage3HandoffError::InvalidState("open-snow albedo"));
    }
    let vis_in = forcing.direct_vis_w_m2 + forcing.diffuse_vis_w_m2;
    let nir_in = forcing.direct_nir_w_m2 + forcing.diffuse_nir_w_m2;
    let snow_absorbed_shortwave_w_m2 = (vis_in + nir_in) * (1.0 - albedo);
    let snow_net_longwave_w_m2 =
        forcing.atmospheric_downward_longwave_w_m2 - SIGMA_W_M2_K4 * snow_temperature_k.powi(4);
    let optical_bytes = [albedo, vis_in, nir_in, snow_absorbed_shortwave_w_m2]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
    let longwave_bytes = [
        forcing.atmospheric_downward_longwave_w_m2,
        snow_temperature_k,
        snow_net_longwave_w_m2,
    ]
    .into_iter()
    .flat_map(|value| value.to_bits().to_be_bytes())
    .collect::<Vec<_>>();
    let optical_receipt_sha256 = framed_sha256(
        "openwepp-stage3-open-snow-optical-v1",
        &[FramedField {
            tag: "operands",
            value: &optical_bytes,
        }],
    )
    .map_err(|_| SnowStage3HandoffError::InvalidCarrier("open-snow optical framing"))?;
    let longwave_receipt_sha256 = framed_sha256(
        "openwepp-stage3-open-snow-longwave-v1",
        &[FramedField {
            tag: "operands",
            value: &longwave_bytes,
        }],
    )
    .map_err(|_| SnowStage3HandoffError::InvalidLongwave("open-snow longwave framing"))?;
    Ok(OpenSnowTileBoundaryCandidateV1 {
        support: forcing.support,
        destination: forcing.destination.clone(),
        beginning_stage3_state_sha256,
        forcing_receipt_sha256: forcing.forcing_receipt_sha256,
        exposure_receipt_sha256: forcing.exposure.receipt_sha256,
        optical_receipt_sha256,
        longwave_receipt_sha256,
        snow_temperature_k,
        latent_heat_j_kg,
        sensible_outward_w_m2,
        vapor_outward_kg_m2_s,
        latent_energy_outward_j_m2,
        snow_absorbed_shortwave_w_m2,
        snow_net_longwave_w_m2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_coupled_time::ModelTimeNs;

    fn support() -> TimeSupport {
        TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000)).expect("support")
    }

    fn destination(tile: &str) -> (OfeId, TileId) {
        (
            OfeId::try_new("ofe-1").expect("OFE"),
            TileId::try_new(tile).expect("tile"),
        )
    }

    fn exposure(tile: &str) -> SealedOpenSnowExposureReceiptV1 {
        SealedOpenSnowExposureReceiptV1::try_new(
            support(),
            destination(tile),
            Digest32::from_bytes([1; 32]),
            Digest32::from_bytes([2; 32]),
            3.0,
            Digest32::from_bytes([3; 32]),
        )
        .expect("open exposure")
    }

    fn forcing_inputs(tile: &str) -> SealedOpenSnowTileForcingInputsV1 {
        SealedOpenSnowTileForcingInputsV1 {
            support: support(),
            destination: destination(tile),
            forcing_receipt_sha256: Digest32::from_bytes([1; 32]),
            exposure: exposure(tile),
            rho_air_kg_m3: 1.2,
            cp_air_j_kg_k: 1005.0,
            reference_temperature_k: 270.0,
            reference_specific_humidity_kg_kg: 0.002,
            air_pressure_pa: 90_000.0,
            atmospheric_downward_longwave_w_m2: 250.0,
            direct_vis_w_m2: 100.0,
            diffuse_vis_w_m2: 20.0,
            direct_nir_w_m2: 80.0,
            diffuse_nir_w_m2: 10.0,
            rain_m: 0.0,
            snowfall_m: 0.0,
            precipitation_parcel_count: 0,
        }
    }

    #[test]
    fn open_exposure_identity_is_destination_specific() {
        let left = exposure("open-a");
        let right = exposure("open-b");
        assert_ne!(left.receipt_sha256, right.receipt_sha256);
        assert_ne!(left.destination, right.destination);
    }

    #[test]
    fn swapped_open_exposure_destination_rejects() {
        let mut inputs = forcing_inputs("open-a");
        inputs.exposure = exposure("open-b");
        assert!(matches!(
            SealedOpenSnowTileForcingV1::try_new(inputs),
            Err(SnowStage3HandoffError::InvalidCarrier(
                "sealed destination open-snow forcing"
            ))
        ));
    }

    #[test]
    fn open_precipitation_custody_is_fail_closed() {
        let mut rain = forcing_inputs("open-a");
        rain.rain_m = 1.0e-3;
        assert!(matches!(
            SealedOpenSnowTileForcingV1::try_new(rain),
            Err(SnowStage3HandoffError::InvalidCarrier(
                "open-snow precipitation custody is unavailable"
            ))
        ));
        let mut snow = forcing_inputs("open-a");
        snow.snowfall_m = 1.0e-3;
        assert!(SealedOpenSnowTileForcingV1::try_new(snow).is_err());
        let mut parcel = forcing_inputs("open-a");
        parcel.precipitation_parcel_count = 1;
        assert!(SealedOpenSnowTileForcingV1::try_new(parcel).is_err());

        let mut resealed_surface = SealedOpenSnowTileForcingV1::try_new(forcing_inputs("open-a"))
            .expect("dry sealed forcing");
        resealed_surface.rain_m = f64::from_bits(1);
        assert!(resealed_surface.validate().is_err());
    }
}
