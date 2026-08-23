//! Typed, default-off Stage-3/V11 parent attachment.
//!
//! This boundary owns the constitutive Stage-3 support cadence and terminal
//! event projection.  It deliberately accepts a prepared forcing capability
//! rather than an event request or live carrier receipt.  The legacy
//! caller-built handoff remains test-only in `direct_runtime::snow_stage3_shadow`.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_coupled_time::{
    ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, Digest32, LedgerEntryV1,
    ModelTimeNs, OwnerState, ParentAuthorityV1, ParentIntervalId, StepConstraintV1, TimeSupport,
    accept_slab, complete_owner_set_digest, digest_bytes, quantize_seconds_to_tick,
    reduce_constraints,
};
use openwepp_kernel_contract::TileId;
use openwepp_land_surface_energy::OfeId;
use openwepp_vegetation::v11::{
    V11OwnerEnvelope, V11ParentCandidate, V11ParentTransaction, VegetationConfigurationV11,
    execute_v11_segment,
};
use serde::Serialize;
use thiserror::Error;

use crate::hydrology::{
    DirectActiveSnowPartitionInputs, DirectSnowStage3PersistentDayResult,
    DirectSnowStage3PersistentState, DirectSnowStage3SupportInput, DirectSnowTerminalEventRequest,
    DirectSnowTerminalEventResult, Wb11HydrologyKernel,
};
use crate::runtime_inputs::{
    PreparedSnowFreeGsiDayV1, SnowFreeHalfHourForcingError, SnowFreeHalfHourProviderCursor,
    SnowFreePrecipitationParcelReceipt, direct_gsi_state,
};
use crate::snow_stage3_open_boundary::{
    SealedOpenSnowTileForcingV1, SealedStage3TileBoundaryForcingV1,
};
use crate::snow_stage3_terminal_handoff::SealedCoveredCarrierForcing;
use crate::v9_real_consumer_shadow::DirectV10RealConsumerShadow;
use crate::v9_real_consumer_shadow::{
    CoveredParentOwnerJoinReceiptV1, DirectV9ShadowIntervalInput, DirectV11RealConsumerError,
    DirectV11RealConsumerStack, DirectV11SnowCoveredRealConsumerStack,
    DirectV11SnowCoveredSegmentInput, DirectV11SnowCoveredStackInputs,
};
use crate::{DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidConfigurationRecord};

pub const STAGE3_V11_PARENT_SUPPORT_NS: u128 = 1_800_000_000_000;
pub const STAGE3_V11_PARENT_SUPPORT_COUNT: usize = 48;
pub const STAGE3_V11_DAY_NS: u128 = 86_400_000_000_000;

#[derive(Debug, Error)]
pub enum DirectSnowStage3V11AttachmentError {
    #[error("Stage-3/V11 attachment identity failure: {0}")]
    Identity(&'static str),
    #[error("Stage-3/V11 attachment support failure: {0}")]
    Support(&'static str),
    #[error("Stage-3/V11 attachment terminal candidate failure: {0}")]
    Terminal(&'static str),
    #[error(transparent)]
    Stage3(#[from] crate::hydrology::DirectSnowStage3EvaluationError),
    #[error(transparent)]
    CoupledTime(#[from] openwepp_coupled_time::CoupledTimeError),
    #[error(transparent)]
    Owner(#[from] DirectV11RealConsumerError),
    #[error(transparent)]
    V11(#[from] openwepp_vegetation::v11::V11ExecutionError<DirectV11RealConsumerError>),
    #[error(transparent)]
    V11Authority(#[from] openwepp_vegetation::v11::V11Error),
    #[error(transparent)]
    ForcingProvider(#[from] SnowFreeHalfHourForcingError),
}

/// Static configuration and topology identity.  There is intentionally no
/// event day, lane, elapsed time, live surface receipt, or ending owner here.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11StaticContext {
    pub run_identity: Digest32,
    pub topology_identity: Digest32,
    pub parent_duration_ns: u128,
    pub minimum_support_ns: u128,
    pub calendar_receipt: Digest32,
    pub controller_policy: Digest32,
    pub parent_sequence: u128,
    pub lane_ids: Vec<u32>,
    pub vegetation_configuration: VegetationConfigurationV11,
    pub surface_liquid_configuration: DirectSurfaceLiquidConfiguration,
    pub wb14_parameters: Vec<crate::DirectOfeWb14Parameters>,
}

impl DirectSnowStage3V11StaticContext {
    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.parent_duration_ns != STAGE3_V11_PARENT_SUPPORT_NS
            || self.minimum_support_ns == 0
            || self.minimum_support_ns > self.parent_duration_ns
            || self.lane_ids.is_empty()
            || self.lane_ids.windows(2).any(|pair| pair[0] >= pair[1])
            || self.surface_liquid_configuration.records.is_empty()
            || self.wb14_parameters.is_empty()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "static parent, lane, receiver, or WB14 configuration",
            ));
        }
        self.vegetation_configuration
            .validate()
            .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("V11 configuration"))?;
        validate_receiver_topology(&self.surface_liquid_configuration.records)
    }
}

/// One sealed 1,800-second support for every Stage-3 lane.  The snow inputs
/// are the actual Stage-3 owner operands for this support; no daily result is
/// accepted as a substitute.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11PreparedSupport {
    support: TimeSupport,
    snow_inputs_by_lane: BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    support_forcing_by_lane: BTreeMap<u32, DirectSnowStage3SupportInput>,
    /// Sealed lower-boundary/atmospheric input for the actual V11 owner.
    /// It contains no event request, carrier operand, or ending owner.
    v11_interval: DirectV9ShadowIntervalInput,
    /// Sealed Child-2C carrier inputs, keyed by production lane. A missing
    /// entry means this support is snow-free and must remain on the existing
    /// snow-free adopter.
    covered_carrier_forcing_by_lane: BTreeMap<u32, SealedCoveredCarrierForcing>,
    /// Complete active snow-surface forcing topology, keyed by physical
    /// destination. Empty means the support is snow-free.
    snow_surface_forcing_by_destination:
        BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
    /// Covered V11 projection. It is a separate type from the snow-free
    /// interval so regime selection is explicit at the sealed-support seam.
    covered_v11_interval: Option<DirectV11SnowCoveredSegmentInput>,
    /// Provider-owned destination and receipt identity. The physical
    /// precipitation parcel remains sealed input; it is not a terminal parcel
    /// and cannot contain an ending owner or event time.
    support_identity_by_lane: BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage3SnowSurfaceRegime {
    SnowFree,
    OpenSnowOnly,
    CanopyCoveredOrMixed,
}

impl DirectSnowStage3V11PreparedSupport {
    /// Construct an unsealed support draft. Provider/GSI identity is admitted
    /// only when `PreparedStage3V11DayV1::bind_provider_day` consumes this
    /// draft and returns the opaque validated day capability.
    pub fn try_new(
        support: TimeSupport,
        snow_inputs_by_lane: BTreeMap<u32, DirectActiveSnowPartitionInputs>,
        support_forcing_by_lane: BTreeMap<u32, DirectSnowStage3SupportInput>,
        v11_interval: DirectV9ShadowIntervalInput,
        support_identity_by_lane: BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        validate_parent_support_duration(support.duration_ns())?;
        let lane_ids = snow_inputs_by_lane.keys().copied().collect::<BTreeSet<_>>();
        if lane_ids.is_empty()
            || lane_ids != support_forcing_by_lane.keys().copied().collect()
            || lane_ids != support_identity_by_lane.keys().copied().collect()
            || support_identity_by_lane.values().any(Vec::is_empty)
            || support_identity_by_lane.values().any(|identities| {
                identities.windows(2).any(|pair| {
                    (
                        pair[0].destination_ofe_id.as_str(),
                        pair[0].destination_tile_id.as_str(),
                    ) >= (
                        pair[1].destination_ofe_id.as_str(),
                        pair[1].destination_tile_id.as_str(),
                    )
                })
            })
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "support draft lane and destination identity",
            ));
        }
        Ok(Self {
            support,
            snow_inputs_by_lane,
            support_forcing_by_lane,
            v11_interval,
            covered_carrier_forcing_by_lane: BTreeMap::new(),
            snow_surface_forcing_by_destination: BTreeMap::new(),
            covered_v11_interval: None,
            support_identity_by_lane,
        })
    }

    /// Attach the sealed covered lower-boundary carrier to a support draft.
    /// The provider bind consumes the resulting draft and rechecks all joins.
    #[must_use]
    pub fn with_covered_carrier_forcing(
        mut self,
        lane_id: u32,
        forcing: SealedCoveredCarrierForcing,
    ) -> Self {
        self.covered_carrier_forcing_by_lane
            .insert(lane_id, forcing);
        self
    }

    /// Attach a covered forcing to one typed physical destination.
    #[must_use]
    pub fn with_covered_tile_forcing(
        mut self,
        destination: (OfeId, TileId),
        forcing: SealedCoveredCarrierForcing,
    ) -> Self {
        self.snow_surface_forcing_by_destination.insert(
            destination,
            SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing),
        );
        self
    }

    /// Attach one independently sealed open-snow forcing to its destination.
    #[must_use]
    pub fn with_open_snow_tile_forcing(mut self, forcing: SealedOpenSnowTileForcingV1) -> Self {
        self.snow_surface_forcing_by_destination.insert(
            forcing.destination.clone(),
            SealedStage3TileBoundaryForcingV1::OpenSnow(forcing),
        );
        self
    }

    /// Attach the distinct covered V11 atmospheric projection to this support.
    #[must_use]
    pub fn with_covered_v11_interval(mut self, interval: DirectV11SnowCoveredSegmentInput) -> Self {
        self.covered_v11_interval = Some(interval);
        self
    }

    #[must_use]
    pub const fn support(&self) -> TimeSupport {
        self.support
    }

    #[must_use]
    pub fn covered_carrier_forcing_by_lane(&self) -> &BTreeMap<u32, SealedCoveredCarrierForcing> {
        &self.covered_carrier_forcing_by_lane
    }

    #[must_use]
    pub fn snow_surface_forcing_by_destination(
        &self,
    ) -> &BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1> {
        &self.snow_surface_forcing_by_destination
    }

    fn has_snow_surface_forcing(&self) -> bool {
        !self.snow_surface_forcing_by_destination.is_empty()
            || !self.covered_carrier_forcing_by_lane.is_empty()
    }

    #[must_use]
    pub fn snow_surface_regime(&self) -> Stage3SnowSurfaceRegime {
        if self.snow_surface_forcing_by_destination.is_empty() {
            Stage3SnowSurfaceRegime::SnowFree
        } else if self
            .snow_surface_forcing_by_destination
            .values()
            .all(|forcing| matches!(forcing, SealedStage3TileBoundaryForcingV1::OpenSnow(_)))
        {
            Stage3SnowSurfaceRegime::OpenSnowOnly
        } else {
            Stage3SnowSurfaceRegime::CanopyCoveredOrMixed
        }
    }

    fn validate_explicit_snow_surface_set(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.snow_surface_forcing_by_destination.is_empty() {
            return Ok(());
        }
        let expected = self
            .support_identity_by_lane
            .values()
            .flatten()
            .map(|identity| {
                Ok((
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface OFE identity")
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface tile identity")
                    })?,
                ))
            })
            .collect::<Result<BTreeSet<_>, DirectSnowStage3V11AttachmentError>>()?;
        if expected
            != self
                .snow_surface_forcing_by_destination
                .keys()
                .cloned()
                .collect()
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "complete destination-keyed snow-surface set",
            ));
        }
        let mut covered_lanes = BTreeSet::new();
        for (lane_id, identities) in &self.support_identity_by_lane {
            for identity in identities {
                let destination = (
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface OFE identity")
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface tile identity")
                    })?,
                );
                let physical = self
                    .snow_surface_forcing_by_destination
                    .get(&destination)
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "snow-surface destination forcing",
                    ))?;
                let exposure_identity = match physical {
                    SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing) => {
                        covered_lanes.insert(*lane_id);
                        if self.covered_carrier_forcing_by_lane.get(lane_id) != Some(forcing) {
                            return Err(DirectSnowStage3V11AttachmentError::Support(
                                "legacy lane carrier differs from destination authority",
                            ));
                        }
                        forcing.exposure_identity()
                    }
                    SealedStage3TileBoundaryForcingV1::OpenSnow(forcing) => {
                        forcing.validate().map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Support("sealed open-snow forcing")
                        })?;
                        if forcing.forcing_receipt_sha256 != identity.forcing_receipt_digest {
                            return Err(DirectSnowStage3V11AttachmentError::Support(
                                "open-snow/provider forcing receipt join",
                            ));
                        }
                        forcing.exposure.receipt_sha256
                    }
                };
                if identity.exposure_identity != exposure_identity {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "support exposure identity/physical receipt join",
                    ));
                }
            }
        }
        if self
            .covered_carrier_forcing_by_lane
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != covered_lanes
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "legacy lane carrier set differs from covered destination subset",
            ));
        }
        self.validate_zero_precipitation_custody()?;
        Ok(())
    }

    fn validate_zero_precipitation_custody(
        &self,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.snow_surface_forcing_by_destination.is_empty() {
            return Ok(());
        }
        let interval = self
            .covered_v11_interval
            .as_ref()
            .map_or(&self.v11_interval, |covered| {
                // Both interval types expose the same forcing fields, but not
                // a common Rust type; the checks below are split accordingly.
                let _ = covered;
                &self.v11_interval
            });
        let stage3_is_dry = self.support_forcing_by_lane.values().all(|support| {
            support.forcing.active_precipitation_m.to_bits() == 0.0_f64.to_bits()
                && support.forcing.rain_m.to_bits() == 0.0_f64.to_bits()
                && support.forcing.snowfall_m.to_bits() == 0.0_f64.to_bits()
        });
        let identities_are_dry = self
            .support_identity_by_lane
            .values()
            .flatten()
            .all(|identity| identity.precipitation_parcels.is_empty());
        let base_lse_is_dry = interval.lse_forcing.precipitation_parcels.is_empty()
            && interval.lse_forcing.runon_parcels.is_empty()
            && interval.vegetation_forcing.rain_kg_m2.to_bits() == 0.0_f64.to_bits();
        let covered_lse_is_dry = self.covered_v11_interval.as_ref().is_none_or(|covered| {
            covered.lse_forcing.precipitation_parcels.is_empty()
                && covered.lse_forcing.runon_parcels.is_empty()
                && covered.vegetation_forcing.rain_kg_m2.to_bits() == 0.0_f64.to_bits()
        });
        if !stage3_is_dry || !identities_are_dry || !base_lse_is_dry || !covered_lse_is_dry {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "snow-surface precipitation custody is unavailable",
            ));
        }
        Ok(())
    }

    fn forcing_projections(&self) -> (Digest32, Digest32, Digest32, Digest32) {
        let stage3_support_forcing_sha256 =
            canonical_stage3_support_forcing_digest(&self.support_forcing_by_lane);
        let stage3_configuration_sha256 =
            canonical_stage3_configuration_digest(&self.snow_inputs_by_lane);
        let covered_v11_forcing_sha256 =
            if let Some(covered_interval) = self.covered_v11_interval.as_ref() {
                canonical_v11_forcing_digest(
                    &covered_interval.lse_forcing,
                    &covered_interval.vegetation_forcing,
                )
            } else {
                canonical_v11_forcing_digest(
                    &self.v11_interval.lse_forcing,
                    &self.v11_interval.vegetation_forcing,
                )
            };
        let carrier_configuration_sha256 = canonical_snow_surface_forcing_digest(
            &self.covered_carrier_forcing_by_lane,
            &self.snow_surface_forcing_by_destination,
        );
        (
            stage3_support_forcing_sha256,
            stage3_configuration_sha256,
            covered_v11_forcing_sha256,
            carrier_configuration_sha256,
        )
    }
}

fn append_canonical_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

fn append_canonical_str(bytes: &mut Vec<u8>, value: &str) {
    append_canonical_bytes(bytes, value.as_bytes());
}

fn append_canonical_f64(bytes: &mut Vec<u8>, value: f64) {
    bytes.extend_from_slice(&value.to_bits().to_be_bytes());
}

fn append_canonical_option_f64(bytes: &mut Vec<u8>, value: Option<f64>) {
    match value {
        Some(value) => {
            bytes.push(1);
            append_canonical_f64(bytes, value);
        }
        None => bytes.push(0),
    }
}

fn canonical_stage3_support_forcing_digest(
    forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_STAGE3_SUPPORT_FORCING_V2");
    for (lane, support) in forcing_by_lane {
        bytes.extend_from_slice(&lane.to_be_bytes());
        append_canonical_f64(&mut bytes, support.duration_seconds);
        let forcing = support.forcing;
        for value in [
            forcing.active_precipitation_m,
            forcing.rain_m,
            forcing.snowfall_m,
            forcing.radiation_mj_m2,
            forcing.air_temperature_c,
            forcing.cloud_fraction,
            forcing.rain_fraction,
            forcing.snow_fraction,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        append_canonical_str(&mut bytes, forcing.phase_model.id());
        append_canonical_option_f64(&mut bytes, forcing.hydrometeor_temperature_c);
    }
    digest_bytes(&bytes)
}

fn canonical_stage3_configuration_digest(
    inputs_by_lane: &BTreeMap<u32, DirectActiveSnowPartitionInputs>,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_STAGE3_CONFIGURATION_V2");
    for (lane, input) in inputs_by_lane {
        bytes.extend_from_slice(&lane.to_be_bytes());
        for value in [
            input.hyetograph_rainfall_m,
            input.rst_c,
            input.newsnw_kg_m3,
            input.ssd_kg_m3,
            input.tmax_c,
            input.tmin_c,
            input.canopy_cover_fraction,
            input.wind_m_s,
            input.dewpoint_c,
            input.coe_boundary_depth_m,
            input.coe_boundary_density_kg_m3,
            input.coe_boundary_settle_day_count,
            input.underlying_surface_albedo,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        for id in [
            input.snow_melt_model.id(),
            input.snow_density_model.id(),
            input.stage3_liquid_routing_model.id(),
            input.surface_energy_options.longwave_model.id(),
            input.surface_energy_options.sublimation_model.id(),
        ] {
            append_canonical_str(&mut bytes, id);
        }
        if let Some(model) = input.snow_albedo_model {
            bytes.push(1);
            append_canonical_str(&mut bytes, model.id());
        } else {
            bytes.push(0);
        }
        if let Some(class) = input.sturm_climate_class {
            bytes.push(1);
            append_canonical_str(&mut bytes, class.id());
        } else {
            bytes.push(0);
        }
        append_canonical_option_f64(&mut bytes, input.sturm_day_of_year);
        let options = input.surface_energy_options;
        for value in [
            options.daily_solar_radiation_mj_m2,
            options.daily_extraterrestrial_radiation_mj_m2,
            options.atmospheric_pressure_pa,
            options.turbulent_geometry.air_temperature_height_m,
            options.turbulent_geometry.vapor_pressure_height_m,
            options.turbulent_geometry.wind_speed_height_m,
            options.turbulent_geometry.aerodynamic_roughness_length_m,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        bytes.push(u8::from(options.daylight));
        bytes.push(u8::from(options.complete_carrier_shadow));
    }
    digest_bytes(&bytes)
}

fn canonical_v11_forcing_digest(
    lse_forcing: &openwepp_land_surface_energy::LandSurfaceForcing,
    vegetation_forcing: &openwepp_vegetation::SnowFreeForcing,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_COVERED_V11_FORCING_V2");
    append_canonical_str(&mut bytes, lse_forcing.forcing_sha256.as_str());
    bytes.extend_from_slice(&lse_forcing.transaction_id.0.to_be_bytes());
    append_canonical_f64(&mut bytes, lse_forcing.interval_s);
    for value in [
        vegetation_forcing.air_temperature_k,
        vegetation_forcing.pressure_pa,
        vegetation_forcing.co2_pa,
        vegetation_forcing.vapor_pressure_deficit_kpa,
        vegetation_forcing.wind_m_s,
        vegetation_forcing.rain_kg_m2,
        vegetation_forcing.direct_par_w_m2,
        vegetation_forcing.diffuse_par_w_m2,
        vegetation_forcing.direct_nir_w_m2,
        vegetation_forcing.diffuse_nir_w_m2,
        vegetation_forcing.solar_zenith_cosine,
        vegetation_forcing.ground_albedo_vis,
        vegetation_forcing.ground_albedo_nir,
        vegetation_forcing.longwave_down_w_m2,
        vegetation_forcing.longwave_up_w_m2,
        vegetation_forcing.specific_humidity,
        vegetation_forcing.reference_height_m,
        vegetation_forcing.gsi,
    ] {
        append_canonical_f64(&mut bytes, value);
    }
    for layer in &vegetation_forcing.soil_layers {
        append_canonical_str(&mut bytes, layer.layer_id.as_str());
        for value in [
            layer.water_beginning_kg_m2,
            layer.matric_potential_mm,
            layer.hydraulic_conductivity_mm_s,
            layer.root_path_length_mm,
            layer.gravity_root_mm,
            layer.temperature_k,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        bytes.push(u8::from(layer.accessible));
        bytes.push(u8::from(layer.frozen));
    }
    digest_bytes(&bytes)
}

fn canonical_carrier_forcing_digest(
    forcing_by_lane: &BTreeMap<u32, SealedCoveredCarrierForcing>,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_COVERED_CARRIER_CONFIGURATION_V2");
    for (lane, forcing) in forcing_by_lane {
        bytes.extend_from_slice(&lane.to_be_bytes());
        for value in [
            forcing.rho_air_kg_m3,
            forcing.cp_air_j_kg_k,
            forcing.reference_temperature_k,
            forcing.reference_specific_humidity,
            forcing.atmospheric_longwave_w_m2,
            forcing.effective_canopy_cover,
            forcing.exposure.wind_m_s,
            forcing.exposure.transfer_height_m,
            forcing.exposure.roughness_m,
        ] {
            append_canonical_f64(&mut bytes, value);
        }
        for value in [
            forcing.exposure.receipt_id.as_str(),
            forcing.exposure.provider.as_str(),
            forcing.exposure.provider_digest.as_str(),
            forcing.exposure.source.as_str(),
        ] {
            append_canonical_str(&mut bytes, value);
        }
        for participant in &forcing.active_participants {
            append_canonical_str(&mut bytes, participant);
        }
        for receipt in &forcing.support_receipts {
            append_canonical_str(&mut bytes, &receipt.participant_id);
            append_canonical_str(&mut bytes, &receipt.support_receipt_id);
            bytes.extend_from_slice(&receipt.minimum_support_ns.get().to_be_bytes());
        }
    }
    digest_bytes(&bytes)
}

fn canonical_snow_surface_forcing_digest(
    covered_by_lane: &BTreeMap<u32, SealedCoveredCarrierForcing>,
    by_destination: &BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_STAGE3_SNOW_SURFACE_SET_V1");
    bytes.extend_from_slice(canonical_carrier_forcing_digest(covered_by_lane).as_bytes());
    for (destination, forcing) in by_destination {
        append_canonical_str(&mut bytes, destination.0.as_str());
        append_canonical_str(&mut bytes, destination.1.as_str());
        match forcing {
            SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing) => {
                bytes.push(0);
                bytes.extend_from_slice(forcing.exposure_identity().as_bytes());
            }
            SealedStage3TileBoundaryForcingV1::OpenSnow(forcing) => {
                bytes.push(1);
                bytes.extend_from_slice(forcing.receipt_sha256.as_bytes());
            }
        }
    }
    digest_bytes(&bytes)
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedStage3V11SupportIdentityV1 {
    destination_ofe_id: String,
    destination_tile_id: String,
    wb14_configuration_sha256: String,
    exposure_identity: Digest32,
    precipitation_parcels: Vec<SnowFreePrecipitationParcelReceipt>,
    forcing_receipt_digest: Digest32,
}

impl PreparedStage3V11SupportIdentityV1 {
    #[must_use]
    pub fn new(
        destination_ofe_id: String,
        destination_tile_id: String,
        wb14_configuration_sha256: String,
        exposure_identity: Digest32,
        precipitation_parcels: Vec<SnowFreePrecipitationParcelReceipt>,
        forcing_receipt_digest: Digest32,
    ) -> Self {
        Self {
            destination_ofe_id,
            destination_tile_id,
            wb14_configuration_sha256,
            exposure_identity,
            precipitation_parcels,
            forcing_receipt_digest,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedStage3V11DayV1 {
    day_index: usize,
    accepted_gsi_receipt: Digest32,
    beginning_provider_cursor: SnowFreeHalfHourProviderCursor,
    ending_provider_cursor: SnowFreeHalfHourProviderCursor,
    supports: Vec<DirectSnowStage3V11PreparedSupport>,
}

/// Opaque provider/GSI-joined capability accepted by the closure path.
#[derive(Clone, Debug)]
pub struct ValidatedPreparedStage3V11DayV1 {
    inner: PreparedStage3V11DayV1,
    provider_day: PreparedSnowFreeGsiDayV1,
}

pub type DirectSnowStage3V11PreparedDay = ValidatedPreparedStage3V11DayV1;
pub type PreparedStage3V11SupportV1 = DirectSnowStage3V11PreparedSupport;

impl PreparedStage3V11DayV1 {
    /// Bind runner-built support operands to the already validated repository
    /// provider day. This is the only constructor that admits provider/GSI
    /// identity into the sealed 48-support capability.
    #[allow(clippy::too_many_lines)]
    pub fn bind_provider_day(
        provider: &PreparedSnowFreeGsiDayV1,
        day_index: usize,
        supports: Vec<DirectSnowStage3V11PreparedSupport>,
    ) -> Result<ValidatedPreparedStage3V11DayV1, DirectSnowStage3V11AttachmentError> {
        if supports.len() != STAGE3_V11_PARENT_SUPPORT_COUNT {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "provider binding requires exactly 48 supports",
            ));
        }
        provider.gsi_receipt().validate()?;
        if provider.gsi_receipt().day_index
            != u64::try_from(day_index).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Support("provider day index width")
            })?
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "provider day index",
            ));
        }
        let accepted_gsi_receipt = provider.gsi_receipt_digest()?;
        let day_start_ns = day_start_ns(day_index)?;
        for (support_index, support) in supports.iter().enumerate() {
            support.validate_explicit_snow_surface_set()?;
            let provider_destinations = provider
                .forcing_receipts()
                .receipts()
                .iter()
                .filter(|day| day.day_index == day_index)
                .map(|day| {
                    let interval = day.intervals.get(support_index).ok_or(
                        DirectSnowStage3V11AttachmentError::Support(
                            "provider interval cardinality",
                        ),
                    )?;
                    Ok((
                        (interval.ofe_id.clone(), interval.tile_id.clone()),
                        interval,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>, DirectSnowStage3V11AttachmentError>>()?;
            if provider_destinations.is_empty() {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "provider day destination set",
                ));
            }
            let mut support_destinations = BTreeSet::new();
            for identity in support
                .support_identity_by_lane
                .values()
                .flat_map(|identities| identities.iter())
            {
                if identity.exposure_identity == Digest32::zero() {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "missing sealed exposure identity",
                    ));
                }
                if !support_destinations.insert((
                    identity.destination_ofe_id.clone(),
                    identity.destination_tile_id.clone(),
                )) {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "duplicate support destination identity",
                    ));
                }
            }
            if support_destinations != provider_destinations.keys().cloned().collect() {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "support/provider destination set",
                ));
            }
            for identity in support
                .support_identity_by_lane
                .values()
                .flat_map(|identities| identities.iter())
            {
                let interval = provider_destinations
                    .get(&(
                        identity.destination_ofe_id.clone(),
                        identity.destination_tile_id.clone(),
                    ))
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "support/provider destination interval join",
                    ))?;
                let receipt_digest = parse_lower_hex_digest(&interval.interval_receipt_sha256)?;
                let interval_start_ns = day_start_ns
                    .checked_add(
                        u128::try_from(interval.start_s)
                            .map_err(|_| {
                                DirectSnowStage3V11AttachmentError::Support(
                                    "provider interval start width",
                                )
                            })?
                            .checked_mul(1_000_000_000)
                            .ok_or(DirectSnowStage3V11AttachmentError::Support(
                                "provider interval start overflow",
                            ))?,
                    )
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "provider interval start day overflow",
                    ))?;
                let interval_end_ns = day_start_ns
                    .checked_add(
                        u128::try_from(interval.end_s)
                            .map_err(|_| {
                                DirectSnowStage3V11AttachmentError::Support(
                                    "provider interval end width",
                                )
                            })?
                            .checked_mul(1_000_000_000)
                            .ok_or(DirectSnowStage3V11AttachmentError::Support(
                                "provider interval end overflow",
                            ))?,
                    )
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "provider interval end day overflow",
                    ))?;
                if identity.forcing_receipt_digest != receipt_digest
                    || interval.gsi_receipt_sha256 != provider.gsi_receipt().receipt_sha256
                    || interval.wb14_configuration_sha256 != identity.wb14_configuration_sha256
                    || interval.precipitation_parcels != identity.precipitation_parcels
                    || support.support.start_ns().get() != interval_start_ns
                    || support.support.end_ns().get() != interval_end_ns
                {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "sealed provider support operands",
                    ));
                }
            }
        }
        Ok(ValidatedPreparedStage3V11DayV1 {
            inner: Self {
                day_index,
                accepted_gsi_receipt,
                beginning_provider_cursor: provider.forcing_receipts().beginning_cursor().clone(),
                ending_provider_cursor: provider.forcing_receipts().ending_cursor().clone(),
                supports,
            },
            provider_day: provider.clone(),
        })
    }

    fn validate(
        &self,
        context: &DirectSnowStage3V11StaticContext,
        expected_start_ns: u128,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.supports.len() != STAGE3_V11_PARENT_SUPPORT_COUNT {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "a prepared day requires exactly 48 parent supports",
            ));
        }
        let expected_lanes = context.lane_ids.iter().copied().collect::<BTreeSet<_>>();
        let mut cursor = expected_start_ns;
        for support in &self.supports {
            if support.support.start_ns().get() != cursor
                || support.support.duration_ns() != context.parent_duration_ns
                || support
                    .snow_inputs_by_lane
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    != expected_lanes
                || support
                    .support_forcing_by_lane
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    != expected_lanes
                || support
                    .support_identity_by_lane
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    != expected_lanes
                || (!support.covered_carrier_forcing_by_lane.is_empty()
                    && support
                        .covered_carrier_forcing_by_lane
                        .keys()
                        .copied()
                        .collect::<BTreeSet<_>>()
                        != expected_lanes)
                || support.support_identity_by_lane.values().any(Vec::is_empty)
            {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "support chronology or lane forcing identity",
                ));
            }
            support.validate_explicit_snow_surface_set()?;
            if (support.snow_surface_regime() != Stage3SnowSurfaceRegime::SnowFree)
                != support.covered_v11_interval.is_some()
            {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "snow-surface support requires persistent-snow V11 projection",
                ));
            }
            cursor = support.support.end_ns().get();
        }
        Ok(())
    }

    fn validate_provider_join(
        &self,
        expected_beginning_cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if &self.beginning_provider_cursor != expected_beginning_cursor
            || self.beginning_provider_cursor == self.ending_provider_cursor
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "prepared day provider cursor join",
            ));
        }
        Ok(())
    }
}

impl ValidatedPreparedStage3V11DayV1 {
    #[must_use]
    pub const fn day_index(&self) -> usize {
        self.inner.day_index
    }

    #[must_use]
    pub const fn accepted_gsi_receipt(&self) -> Digest32 {
        self.inner.accepted_gsi_receipt
    }

    #[must_use]
    pub fn supports(&self) -> &[DirectSnowStage3V11PreparedSupport] {
        &self.inner.supports
    }

    #[must_use]
    pub const fn beginning_provider_cursor(&self) -> &SnowFreeHalfHourProviderCursor {
        &self.inner.beginning_provider_cursor
    }

    #[must_use]
    pub const fn ending_provider_cursor(&self) -> &SnowFreeHalfHourProviderCursor {
        &self.inner.ending_provider_cursor
    }

    fn validate(
        &self,
        context: &DirectSnowStage3V11StaticContext,
        expected_start_ns: u128,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.inner.validate(context, expected_start_ns)?;
        self.validate_lane_destination_bindings(context)
    }

    fn validate_lane_destination_bindings(
        &self,
        context: &DirectSnowStage3V11StaticContext,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let provider_destinations_by_ofe = self
            .provider_day
            .forcing_receipts()
            .receipts()
            .iter()
            .flat_map(|day| day.intervals.iter())
            .fold(
                BTreeMap::<String, BTreeSet<(String, String)>>::new(),
                |mut destinations, interval| {
                    destinations
                        .entry(interval.ofe_id.clone())
                        .or_default()
                        .insert((interval.ofe_id.clone(), interval.tile_id.clone()));
                    destinations
                },
            );
        for support in &self.inner.supports {
            for (lane_id, identities) in &support.support_identity_by_lane {
                let binding = context
                    .surface_liquid_configuration
                    .ofe_bindings
                    .iter()
                    .find(|binding| binding.production_lane_id == *lane_id)
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "support lane surface-liquid binding",
                    ))?;
                let expected = provider_destinations_by_ofe
                    .get(binding.ofe_id.as_str())
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "support lane provider OFE destinations",
                    ))?;
                validate_lane_destination_set(binding.ofe_id.as_str(), identities, expected)?;
            }
        }
        Ok(())
    }

    fn validate_provider_join(
        &self,
        expected_beginning_cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.inner.validate_provider_join(expected_beginning_cursor)
    }

    fn into_provider_day(self) -> PreparedSnowFreeGsiDayV1 {
        self.provider_day
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11TerminalReceipt {
    pub lane_id: u32,
    pub support: TimeSupport,
    pub result: DirectSnowTerminalEventResult,
    pub candidate_ticks: Vec<ModelTimeNs>,
    pub accepted_event_tick: ModelTimeNs,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11TerminalParcel {
    pub source_lane_id: u32,
    pub destination_ofe_id: String,
    pub destination_tile_id: String,
    pub mass_kg_m2_tile_ground: f64,
    pub temperature_k: f64,
    pub specific_liquid_enthalpy_j_kg: f64,
    pub parcel_digest: Digest32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11ParentReceipt {
    pub day_index: usize,
    pub support_count: usize,
    pub terminal_events: Vec<DirectSnowStage3V11TerminalReceipt>,
    pub ending_stage3_state_digests: BTreeMap<u32, Digest32>,
    pub complete_owner_bytes: BTreeMap<String, Vec<u8>>,
    pub covered_owner_joins: Vec<CoveredParentOwnerJoinReceiptV1>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11CommittedState {
    pub stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub real_consumer: DirectV10RealConsumerShadow,
    pub v11_parent_state: V11ParentTransaction,
    pub coupled_clock: CoupledClockStateV1,
    pub next_parent_sequence: u128,
    pub last_v11_parent_candidate: Option<V11ParentCandidate>,
    pub accepted_event_ordinal: u64,
    pub receipt_chain: Vec<DirectSnowStage3V11ParentReceipt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11ParentCandidate {
    pub ending_state: DirectSnowStage3V11CommittedState,
    pub parent_receipt: DirectSnowStage3V11ParentReceipt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11ShadowAttachment {
    pub static_context: DirectSnowStage3V11StaticContext,
    pub committed: DirectSnowStage3V11CommittedState,
    pending_candidate: Option<DirectSnowStage3V11ParentCandidate>,
}

impl DirectSnowStage3V11ShadowAttachment {
    pub fn new(
        static_context: DirectSnowStage3V11StaticContext,
        committed: DirectSnowStage3V11CommittedState,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        static_context.validate()?;
        if committed.stage3_by_lane.len() != static_context.lane_ids.len()
            || committed
                .stage3_by_lane
                .keys()
                .copied()
                .collect::<BTreeSet<_>>()
                != static_context
                    .lane_ids
                    .iter()
                    .copied()
                    .collect::<BTreeSet<_>>()
            || committed.v11_parent_state.parent_transaction_id().digest() == Digest32::zero()
            || committed.coupled_clock.parent_transaction_id()
                != committed.v11_parent_state.parent_transaction_id()
            || committed.coupled_clock.parent_support().duration_ns()
                != static_context.parent_duration_ns
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "complete committed lane and V11 parent state",
            ));
        }
        let canonical_snow = canonical_stage3_snow_owner_bytes(&committed.stage3_by_lane)?;
        let parent_snow = committed
            .v11_parent_state
            .staged_resource_owners()
            .get("snow")
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing canonical Stage-3 snow owner",
            ))?;
        let clock_snow = committed
            .coupled_clock
            .owners()
            .iter()
            .find(|owner| owner.owner_id() == "snow")
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing coupled-time Stage-3 snow owner",
            ))?;
        if parent_snow.state_bytes != canonical_snow
            || clock_snow.state_bytes() != canonical_snow.as_slice()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "hydrology and Stage-3 snow owners are not exact-one custody",
            ));
        }
        Ok(Self {
            static_context,
            committed,
            pending_candidate: None,
        })
    }

    pub fn stage_prepared_day(
        &mut self,
        prepared: &ValidatedPreparedStage3V11DayV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.pending_candidate.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "duplicate uncommitted Stage-3/V11 parent",
            ));
        }
        let candidate = self.execute_prepared_day(prepared)?;
        self.pending_candidate = Some(candidate);
        Ok(())
    }

    pub fn commit_staged_day(&mut self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let candidate =
            self.pending_candidate
                .take()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "missing staged Stage-3/V11 parent",
                ))?;
        self.install_candidate(candidate)
    }

    /// Execute all 48 actual Stage-3 transitions atomically.  Terminal
    /// candidates are rerun against the actual Stage-3 support evaluator; no
    /// rate projection or completed production day frame is consulted.
    #[allow(clippy::too_many_lines)]
    pub fn execute_prepared_day(
        &self,
        prepared: &ValidatedPreparedStage3V11DayV1,
    ) -> Result<DirectSnowStage3V11ParentCandidate, DirectSnowStage3V11AttachmentError> {
        prepared.validate(&self.static_context, day_start_ns(prepared.day_index())?)?;
        validate_prepared_day_against_committed_provider(&self.committed, prepared)?;
        let mut candidate = self.committed.clone();
        let mut terminal_events = Vec::new();
        let mut covered_owner_joins = Vec::new();
        for (support_index, support) in prepared.supports().iter().enumerate() {
            let covered_support =
                support.snow_surface_regime() != Stage3SnowSurfaceRegime::SnowFree;
            let beginning_stage3 = candidate.stage3_by_lane.clone();
            if !covered_support {
                for lane_id in &self.static_context.lane_ids {
                    let inputs = support.snow_inputs_by_lane.get(lane_id).ok_or(
                        DirectSnowStage3V11AttachmentError::Support("missing lane support input"),
                    )?;
                    let support_forcing = support
                        .support_forcing_by_lane
                        .get(lane_id)
                        .copied()
                        .ok_or(DirectSnowStage3V11AttachmentError::Support(
                            "missing sealed support forcing",
                        ))?;
                    let state = candidate.stage3_by_lane.get(lane_id).ok_or(
                        DirectSnowStage3V11AttachmentError::Identity(
                            "missing committed Stage-3 lane",
                        ),
                    )?;
                    let result = Wb11HydrologyKernel::evaluate_stage3_persistent_support(
                        inputs,
                        state,
                        *lane_id,
                        state.next_interval_index,
                        support_forcing,
                        DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
                    )?;
                    let (ending, event) = if let Some(event) = result.terminal_event {
                        let selected = select_actual_terminal_candidate(
                            inputs,
                            state,
                            *lane_id,
                            state.next_interval_index,
                            support,
                            support_forcing,
                            event,
                            self.static_context.minimum_support_ns,
                        )?;
                        let ending = selected.1.state.clone();
                        (ending, Some(selected.0))
                    } else {
                        (result.state, None)
                    };
                    candidate.stage3_by_lane.insert(*lane_id, ending.clone());
                    if let Some(event) = event {
                        candidate.accepted_event_ordinal = candidate
                            .accepted_event_ordinal
                            .checked_add(1)
                            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                                "terminal event ordinal overflow",
                            ))?;
                        terminal_events.push(event);
                    }
                }
            }

            let forcing_receipt = canonical_parent_forcing_digest(
                prepared.day_index(),
                support_index,
                prepared.accepted_gsi_receipt(),
                support,
            )?;
            let (beginning_parent, beginning_clock) = begin_v11_parent_for_support(
                &self.static_context,
                &candidate,
                support,
                forcing_receipt,
                candidate.next_parent_sequence,
            )?;
            let (parent, consumer, clock, finalized, covered_stage3) = if covered_support {
                let (parent, consumer, clock, finalized, ending_stage3, owner_join) =
                    execute_covered_real_v11_parent(
                        &self.static_context,
                        &beginning_parent,
                        &candidate.real_consumer,
                        &beginning_clock,
                        support,
                        prepared.day_index(),
                        support_index,
                        forcing_receipt,
                        beginning_stage3,
                    )?;
                covered_owner_joins.push(owner_join);
                (parent, consumer, clock, finalized, Some(ending_stage3))
            } else {
                let (parent, consumer, clock, finalized) = execute_real_v11_parent(
                    &self.static_context,
                    &beginning_parent,
                    &candidate.real_consumer,
                    &beginning_clock,
                    support,
                    prepared.day_index(),
                    support_index,
                    forcing_receipt,
                    canonical_stage3_snow_owner_bytes(&candidate.stage3_by_lane)?,
                )?;
                (parent, consumer, clock, finalized, None)
            };
            candidate.v11_parent_state = parent;
            candidate.real_consumer = consumer;
            candidate.coupled_clock = clock;
            candidate.last_v11_parent_candidate = Some(finalized);
            if let Some(ending_stage3) = covered_stage3 {
                candidate.stage3_by_lane = ending_stage3;
            }
            candidate.next_parent_sequence = candidate.next_parent_sequence.checked_add(1).ok_or(
                DirectSnowStage3V11AttachmentError::Identity("V11 parent sequence overflow"),
            )?;
        }
        candidate
            .real_consumer
            .commit_prepared_provider_day(prepared.clone().into_provider_day())?;
        let stage3_digests = candidate
            .stage3_by_lane
            .iter()
            .map(|(lane, state)| {
                let bytes = Wb11HydrologyKernel::serialize_stage3_persistent_state(state).map_err(
                    |_| DirectSnowStage3V11AttachmentError::Identity("Stage-3 restart bytes"),
                )?;
                Ok((*lane, openwepp_coupled_time::digest_bytes(&bytes)))
            })
            .collect::<Result<BTreeMap<_, _>, DirectSnowStage3V11AttachmentError>>()?;
        let complete_owner_bytes = candidate
            .real_consumer
            .canonical_owner_state_bytes()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("canonical V11 owner bytes")
            })?;
        let mut complete_owner_bytes = complete_owner_bytes;
        complete_owner_bytes.insert(
            "snow".to_owned(),
            canonical_stage3_snow_owner_bytes(&candidate.stage3_by_lane)?,
        );
        let receipt = DirectSnowStage3V11ParentReceipt {
            day_index: prepared.day_index(),
            support_count: prepared.supports().len(),
            terminal_events,
            ending_stage3_state_digests: stage3_digests,
            complete_owner_bytes,
            covered_owner_joins,
        };
        candidate.receipt_chain.push(receipt.clone());
        Ok(DirectSnowStage3V11ParentCandidate {
            ending_state: candidate,
            parent_receipt: receipt,
        })
    }

    /// The only installation point.  Every owner and receipt check happens
    /// before this non-fallible replacement, preserving rollback on failure.
    pub fn install_candidate(
        &mut self,
        candidate: DirectSnowStage3V11ParentCandidate,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if candidate.ending_state.receipt_chain.len() != self.committed.receipt_chain.len() + 1 {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt chain installation",
            ));
        }
        self.committed = candidate.ending_state;
        Ok(())
    }

    /// Split one solver-owned terminal-liquid operand over the declared
    /// surface topology exactly once. The parcel remains in the candidate
    /// until the real surface-liquid owner consumes it.
    pub fn terminal_parcels(
        &self,
        lane_id: u32,
        terminal_liquid_kg_m2: f64,
    ) -> Result<Vec<DirectSnowStage3V11TerminalParcel>, DirectSnowStage3V11AttachmentError> {
        if !self.static_context.lane_ids.contains(&lane_id)
            || !terminal_liquid_kg_m2.is_finite()
            || terminal_liquid_kg_m2 < 0.0
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal parcel source or mass",
            ));
        }
        let destination_ofe = self
            .static_context
            .surface_liquid_configuration
            .ofe_bindings
            .iter()
            .find(|binding| binding.production_lane_id == lane_id)
            .map(|binding| binding.ofe_id.clone())
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal source lane receiver binding",
            ))?;
        let records = self
            .static_context
            .surface_liquid_configuration
            .records
            .iter()
            .filter(|record| record.key.ofe_id == destination_ofe)
            .collect::<Vec<_>>();
        if records.is_empty() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal source lane receiver records",
            ));
        }
        let fraction_sum = records
            .iter()
            .map(|record| record.tile_fraction)
            .sum::<f64>();
        if (fraction_sum - 1.0).abs() > 1.0e-12 {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal source lane receiver split",
            ));
        }
        // The solver result is OFE-ground basis. Use the declared uniform-depth
        // basis for every receiving tile, then reconstruct the OFE amount with
        // the tile fractions. Dividing by each fraction would duplicate the
        // same OFE mass once per tile.
        let reconstructed_mass = records
            .iter()
            .map(|record| record.tile_fraction * terminal_liquid_kg_m2)
            .sum::<f64>();
        let mass_tolerance = 1.0e-12_f64.max(1.0e-12 * terminal_liquid_kg_m2.abs());
        if !reconstructed_mass.is_finite()
            || (reconstructed_mass - terminal_liquid_kg_m2).abs() > mass_tolerance
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal parcel mass basis closure",
            ));
        }
        records
            .into_iter()
            .map(|record| {
                let mass = terminal_liquid_kg_m2;
                let digest = openwepp_coupled_time::digest_bytes(
                    format!(
                        "OPENWEPP_STAGE3_TERMINAL_PARCEL_V1|{lane_id}|{}|{}|{:016x}",
                        record.key.ofe_id,
                        record.key.tile_id.as_str(),
                        mass.to_bits()
                    )
                    .as_bytes(),
                );
                Ok(DirectSnowStage3V11TerminalParcel {
                    source_lane_id: lane_id,
                    destination_ofe_id: record.key.ofe_id.to_string(),
                    destination_tile_id: record.key.tile_id.as_str().to_owned(),
                    mass_kg_m2_tile_ground: mass,
                    temperature_k: 273.15,
                    specific_liquid_enthalpy_j_kg: 0.0,
                    parcel_digest: digest,
                })
            })
            .collect()
    }

    /// Validate the complete liquid identity carried by the actual terminal
    /// event before constructing any receiver parcel.
    pub fn terminal_parcels_from_event(
        &self,
        lane_id: u32,
        event: &DirectSnowTerminalEventResult,
    ) -> Result<Vec<DirectSnowStage3V11TerminalParcel>, DirectSnowStage3V11AttachmentError> {
        if !event.event_occurred
            || !event.terminal_liquid_kg_m2.is_finite()
            || event.terminal_liquid_kg_m2 < 0.0
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal parcel requires an actual event",
            ));
        }
        let liquid_reconstruction =
            event.start_liquid_kg_m2 + event.external_liquid_kg_m2 + event.melt_kg_m2
                - event.refrozen_kg_m2
                - event.terminal_liquid_kg_m2;
        let tolerance = 1.0e-12_f64.max(
            1.0e-12
                * (event.start_liquid_kg_m2.abs()
                    + event.external_liquid_kg_m2.abs()
                    + event.melt_kg_m2.abs()
                    + event.refrozen_kg_m2.abs()
                    + event.terminal_liquid_kg_m2.abs()),
        );
        if !liquid_reconstruction.is_finite()
            || liquid_reconstruction.abs() > tolerance
            || event.liquid_mass_closure_residual_kg_m2.abs() > tolerance
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal liquid independent reconstruction",
            ));
        }
        self.terminal_parcels(lane_id, event.terminal_liquid_kg_m2)
    }
}

fn validate_prepared_day_against_committed_provider(
    committed: &DirectSnowStage3V11CommittedState,
    prepared: &ValidatedPreparedStage3V11DayV1,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let owner_destinations = committed
        .real_consumer
        .provider_static_configuration()
        .destinations
        .iter()
        .map(|destination| (destination.ofe_id.clone(), destination.tile_id.clone()))
        .collect::<BTreeSet<_>>();
    let prepared_destinations = prepared
        .supports()
        .first()
        .into_iter()
        .flat_map(|support| {
            support
                .support_identity_by_lane
                .values()
                .flat_map(|identities| identities.iter())
                .map(|identity| {
                    (
                        identity.destination_ofe_id.clone(),
                        identity.destination_tile_id.clone(),
                    )
                })
        })
        .collect::<BTreeSet<_>>();
    if prepared_destinations != owner_destinations {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "prepared/committed provider destination topology",
        ));
    }
    committed
        .real_consumer
        .provider_cursor()
        .validate_for_configuration(
            committed.real_consumer.provider_static_configuration(),
            prepared.day_index(),
        )?;
    let beginning_gsi_state = direct_gsi_state(committed.real_consumer.gsi_state())?;
    let prepared_gsi_receipt = prepared.provider_day.gsi_receipt();
    if prepared_gsi_receipt.configuration_sha256
        != committed
            .real_consumer
            .gsi_owner_configuration()
            .configuration_sha256
        || prepared_gsi_receipt.run_id
            != committed
                .real_consumer
                .provider_static_configuration()
                .run_id
        || prepared_gsi_receipt.beginning_state != beginning_gsi_state
    {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "prepared beginning GSI owner state",
        ));
    }
    prepared.validate_provider_join(committed.real_consumer.provider_cursor())
}

fn begin_v11_parent_for_support(
    context: &DirectSnowStage3V11StaticContext,
    committed: &DirectSnowStage3V11CommittedState,
    prepared: &DirectSnowStage3V11PreparedSupport,
    forcing_receipt: Digest32,
    parent_sequence: u128,
) -> Result<(V11ParentTransaction, CoupledClockStateV1), DirectSnowStage3V11AttachmentError> {
    let beginning_state = committed.last_v11_parent_candidate.as_ref().map_or_else(
        || committed.v11_parent_state.beginning_state(),
        |candidate| &candidate.ending_state,
    );
    let beginning_owners = committed.coupled_clock.owners().to_vec();
    let beginning_owner_digest = complete_owner_set_digest(&beginning_owners)?;
    let authority = ParentAuthorityV1::new(
        context.run_identity,
        context.calendar_receipt,
        forcing_receipt,
        parent_sequence,
        prepared.support,
        beginning_owner_digest,
    )?;
    let participants = committed.coupled_clock.active_participants().to_vec();
    let clock = CoupledClockStateV1::new(
        authority,
        beginning_owners.clone(),
        "snow-stage3-v11".to_owned(),
        participants,
        context.controller_policy,
        Vec::new(),
    )?;
    let parent = V11ParentTransaction::new_with_complete_owners(
        &context.vegetation_configuration,
        beginning_state,
        clock.parent_transaction_id(),
        prepared.support.start_ns(),
        owner_envelopes_from_states(&beginning_owners)?,
    )?;
    Ok((parent, clock))
}

fn canonical_parent_forcing_digest(
    day_index: usize,
    interval_index: usize,
    accepted_gsi_receipt: Digest32,
    support: &DirectSnowStage3V11PreparedSupport,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let v11_forcing_receipt = support.covered_v11_interval.as_ref().map_or(
        support.v11_interval.lse_forcing.forcing_sha256.as_str(),
        |interval| interval.lse_forcing.forcing_sha256.as_str(),
    );
    let base = canonical_parent_forcing_digest_from_parts(
        day_index,
        interval_index,
        accepted_gsi_receipt,
        support.support,
        v11_forcing_receipt,
        &support.support_identity_by_lane,
    )?;
    let (
        stage3_support_forcing_sha256,
        stage3_configuration_sha256,
        covered_v11_forcing_sha256,
        carrier_configuration_sha256,
    ) = support.forcing_projections();
    let mut bytes = Vec::with_capacity(32 + 4 * 32);
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_V11_PARENT_FORCING_COVERED_V1\0");
    bytes.extend_from_slice(base.as_bytes());
    for projection in [
        stage3_support_forcing_sha256,
        stage3_configuration_sha256,
        covered_v11_forcing_sha256,
        carrier_configuration_sha256,
    ] {
        bytes.extend_from_slice(projection.as_bytes());
    }
    Ok(digest_bytes(&bytes))
}

fn canonical_parent_forcing_digest_from_parts(
    day_index: usize,
    interval_index: usize,
    accepted_gsi_receipt: Digest32,
    support: TimeSupport,
    v11_forcing_receipt: &str,
    support_identity_by_lane: &BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_V11_PARENT_FORCING_V1\0");
    bytes.extend_from_slice(
        &u128::try_from(day_index)
            .map_err(|_| DirectSnowStage3V11AttachmentError::Support("day index width"))?
            .to_be_bytes(),
    );
    bytes.extend_from_slice(
        &u128::try_from(interval_index)
            .map_err(|_| DirectSnowStage3V11AttachmentError::Support("interval index width"))?
            .to_be_bytes(),
    );
    bytes.extend_from_slice(&support.start_ns().get().to_be_bytes());
    bytes.extend_from_slice(&support.end_ns().get().to_be_bytes());
    bytes.extend_from_slice(accepted_gsi_receipt.as_bytes());
    append_framed_bytes(&mut bytes, v11_forcing_receipt.as_bytes());
    for (lane_id, identities) in support_identity_by_lane {
        bytes.extend_from_slice(&u32::to_be_bytes(*lane_id));
        bytes.extend_from_slice(
            &u64::try_from(identities.len())
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Support("support destination count width")
                })?
                .to_be_bytes(),
        );
        for identity in identities {
            append_framed_bytes(&mut bytes, identity.destination_ofe_id.as_bytes());
            append_framed_bytes(&mut bytes, identity.destination_tile_id.as_bytes());
            append_framed_bytes(&mut bytes, identity.wb14_configuration_sha256.as_bytes());
            bytes.extend_from_slice(identity.exposure_identity.as_bytes());
            bytes.extend_from_slice(identity.forcing_receipt_digest.as_bytes());
            bytes.extend_from_slice(
                &u64::try_from(identity.precipitation_parcels.len())
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support(
                            "precipitation parcel count width",
                        )
                    })?
                    .to_be_bytes(),
            );
            for parcel in &identity.precipitation_parcels {
                append_framed_bytes(&mut bytes, parcel.parcel_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.source_owner_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.destination_ofe_id.as_bytes());
                append_framed_bytes(&mut bytes, parcel.destination_tile_id.as_bytes());
                bytes.extend_from_slice(&parcel.start_s.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.end_s.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.mass_kg_m2.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.temperature_k.to_bits().to_be_bytes());
                bytes.extend_from_slice(&parcel.enthalpy_j_m2.to_bits().to_be_bytes());
            }
        }
    }
    Ok(digest_bytes(&bytes))
}

fn append_framed_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

#[allow(
    clippy::large_types_passed_by_value,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]
fn execute_real_v11_parent(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    ending_snow_owner_bytes: Vec<u8>,
) -> Result<
    (
        V11ParentTransaction,
        DirectV10RealConsumerShadow,
        CoupledClockStateV1,
        V11ParentCandidate,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    if beginning_parent.parent_transaction_id() != beginning_clock.parent_transaction_id()
        || beginning_clock.accepted_until() != prepared.support.start_ns()
        || beginning_clock.parent_support() != prepared.support
        || beginning_clock.owners().len()
            != openwepp_vegetation::v11::V11_COMPLETE_OWNER_MANIFEST.len()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "V11/coupled-time parent beginning",
        ));
    }

    // The current released DirectV11RealConsumerStack is a snow-free lower
    // boundary.  A snow-covered interval is rejected here until the released
    // covered-boundary executor is available; it is never silently routed
    // through the snow-free branch.
    if prepared.v11_interval.lse_forcing.snow_present_at_beginning
        || prepared.v11_interval.lse_forcing.snow_present_at_end
        || prepared
            .v11_interval
            .lse_forcing
            .snow_terminal_payload_present
    {
        return Err(DirectSnowStage3V11AttachmentError::Owner(
            DirectV11RealConsumerError::Identity(
                "snow-covered V11 lower-boundary executor is not released",
            ),
        ));
    }

    let parent_id = beginning_parent.parent_transaction_id();
    let support = prepared.support;
    let start = support.start_ns();
    let end = support.end_ns();
    let constraint = StepConstraintV1::new(
        parent_id,
        start,
        end,
        "v11-real-consumer".to_owned(),
        ConstraintClass::HardBoundary,
        context.controller_policy,
        context.calendar_receipt,
        forcing_receipt,
    )?;
    let reduction = reduce_constraints(&[constraint], parent_id, start, end, None)?;
    let ledger_digest = complete_owner_set_digest(beginning_clock.owners())?;
    let mut ledger_preimage = Vec::new();
    ledger_preimage.extend_from_slice(parent_id.digest().as_bytes());
    ledger_preimage.extend_from_slice(&support.start_ns().get().to_be_bytes());
    ledger_preimage.extend_from_slice(&support.end_ns().get().to_be_bytes());
    let ledger = LedgerEntryV1::new(
        "complete-owner-custody".to_owned(),
        "canonical-owner-state".to_owned(),
        ledger_digest,
        ledger_digest,
        digest_bytes(&ledger_preimage),
    )?;
    let segment = beginning_clock.active_segment_id();

    // The coupled-time receipt includes the ending owner digest.  Obtain the
    // actual V11 ending owners with a provisional identity receipt, then rerun
    // the real V11 stack against the final receipt before accepting anything.
    let provisional_slab = CoupledSlabCandidateV1::new(
        beginning_clock,
        segment,
        support,
        &reduction,
        beginning_clock.owners().to_vec(),
        vec![ledger.clone()],
    )?;
    let mut provisional_clock = beginning_clock.clone();
    let provisional_receipt = accept_slab(&mut provisional_clock, provisional_slab)?;
    let provisional_parent = beginning_parent.clone();
    let provisional_stack = DirectV11RealConsumerStack::new_with_ending_snow_owner(
        beginning_consumer,
        &prepared.v11_interval,
        day_index,
        interval_index,
        ending_snow_owner_bytes.clone(),
    );
    let mut provisional_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
        stack: provisional_stack,
    };
    let provisional_segment = execute_v11_segment(
        &context.vegetation_configuration,
        &provisional_parent,
        &provisional_receipt,
        &mut provisional_executor,
    )?;
    let ending_owners = owner_states_from_envelopes(&provisional_segment.ending_resource_owners)?;
    let final_slab = CoupledSlabCandidateV1::new(
        beginning_clock,
        segment,
        support,
        &reduction,
        ending_owners,
        vec![ledger],
    )?;
    let mut final_clock = beginning_clock.clone();
    let final_receipt = accept_slab(&mut final_clock, final_slab)?;
    let final_stack = DirectV11RealConsumerStack::new_with_ending_snow_owner(
        beginning_consumer,
        &prepared.v11_interval,
        day_index,
        interval_index,
        ending_snow_owner_bytes,
    );
    let mut final_executor =
        crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack: final_stack };
    let final_segment = execute_v11_segment(
        &context.vegetation_configuration,
        beginning_parent,
        &final_receipt,
        &mut final_executor,
    )?;
    if final_segment.ending_resource_owners != provisional_segment.ending_resource_owners {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "V11 ending owner fixed point",
        ));
    }
    let mut parent = beginning_parent.clone();
    parent.accept_segment(&context.vegetation_configuration, final_segment)?;
    let consumer = final_executor.stack.take_staged_ending().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged real-consumer ending"),
    )?;
    let parent_after_segment = parent.clone();
    let finalized = parent.finalize(&context.vegetation_configuration)?;
    Ok((parent_after_segment, consumer, final_clock, finalized))
}

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::type_complexity
)]
fn execute_covered_real_v11_parent(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<
    (
        V11ParentTransaction,
        DirectV10RealConsumerShadow,
        CoupledClockStateV1,
        V11ParentCandidate,
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        CoveredParentOwnerJoinReceiptV1,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    if beginning_parent.parent_transaction_id() != beginning_clock.parent_transaction_id()
        || beginning_clock.accepted_until() != prepared.support.start_ns()
        || beginning_clock.parent_support() != prepared.support
        || beginning_clock.owners().len()
            != openwepp_vegetation::v11::V11_COMPLETE_OWNER_MANIFEST.len()
        || !prepared.has_snow_surface_forcing()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time parent beginning",
        ));
    }
    let parent_id = beginning_parent.parent_transaction_id();
    let support = prepared.support;
    let start = support.start_ns();
    let end = support.end_ns();
    let constraint = StepConstraintV1::new(
        parent_id,
        start,
        end,
        "v11-snow-covered-real-consumer".to_owned(),
        ConstraintClass::HardBoundary,
        context.controller_policy,
        context.calendar_receipt,
        forcing_receipt,
    )?;
    let reduction = reduce_constraints(&[constraint], parent_id, start, end, None)?;
    let ledger_digest = complete_owner_set_digest(beginning_clock.owners())?;
    let mut ledger_preimage = Vec::new();
    ledger_preimage.extend_from_slice(parent_id.digest().as_bytes());
    ledger_preimage.extend_from_slice(&start.get().to_be_bytes());
    ledger_preimage.extend_from_slice(&end.get().to_be_bytes());
    let ledger = LedgerEntryV1::new(
        "complete-owner-custody".to_owned(),
        "canonical-owner-state".to_owned(),
        ledger_digest,
        ledger_digest,
        digest_bytes(&ledger_preimage),
    )?;
    let segment = beginning_clock.active_segment_id();
    let covered_interval = prepared.covered_v11_interval.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Support(
            "covered support missing covered V11 projection",
        ),
    )?;

    let provisional_slab = CoupledSlabCandidateV1::new(
        beginning_clock,
        segment,
        support,
        &reduction,
        beginning_clock.owners().to_vec(),
        vec![ledger.clone()],
    )?;
    let mut provisional_clock = beginning_clock.clone();
    let provisional_receipt = accept_slab(&mut provisional_clock, provisional_slab)?;
    let provisional_stack = DirectV11SnowCoveredRealConsumerStack::new(
        beginning_consumer,
        DirectV11SnowCoveredStackInputs {
            interval: covered_interval,
            stage3_inputs_by_lane: &prepared.snow_inputs_by_lane,
            stage3_forcing_by_lane: &prepared.support_forcing_by_lane,
            snow_surface_forcing_by_destination: &prepared.snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: beginning_stage3.clone(),
            day_index,
            interval_index,
        },
    );
    let mut provisional_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
        stack: provisional_stack,
    };
    let provisional_segment = execute_v11_segment(
        &context.vegetation_configuration,
        beginning_parent,
        &provisional_receipt,
        &mut provisional_executor,
    )?;
    let ending_owners = owner_states_from_envelopes(&provisional_segment.ending_resource_owners)?;
    let final_slab = CoupledSlabCandidateV1::new(
        beginning_clock,
        segment,
        support,
        &reduction,
        ending_owners,
        vec![ledger],
    )?;
    let mut final_clock = beginning_clock.clone();
    let final_receipt = accept_slab(&mut final_clock, final_slab)?;
    let final_stack = DirectV11SnowCoveredRealConsumerStack::new(
        beginning_consumer,
        DirectV11SnowCoveredStackInputs {
            interval: covered_interval,
            stage3_inputs_by_lane: &prepared.snow_inputs_by_lane,
            stage3_forcing_by_lane: &prepared.support_forcing_by_lane,
            snow_surface_forcing_by_destination: &prepared.snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: beginning_stage3,
            day_index,
            interval_index,
        },
    );
    let mut final_executor =
        crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack: final_stack };
    let final_segment = execute_v11_segment(
        &context.vegetation_configuration,
        beginning_parent,
        &final_receipt,
        &mut final_executor,
    )?;
    if final_segment.ending_resource_owners != provisional_segment.ending_resource_owners {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11 ending owner fixed point",
        ));
    }
    let ending_stage3 = final_executor.stack.take_staged_stage3().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged covered Stage-3 ending"),
    )?;
    let owner_join = CoveredParentOwnerJoinReceiptV1::try_new(
        context.run_identity,
        ParentIntervalId::derive(
            context.run_identity,
            context.calendar_receipt,
            forcing_receipt,
            support,
        )?
        .digest(),
        parent_id.digest(),
        final_receipt.segment_id().digest(),
        final_receipt.slab_id().digest(),
        forcing_receipt,
        ledger_digest,
        support,
        final_executor.stack.last_final_boundary_receipts().ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "missing final covered boundary receipt set",
            ),
        )?,
        final_executor.stack.last_lane_boundary_receipts().ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "missing final covered lane-boundary receipt set",
            ),
        )?,
        final_executor
            .stack
            .last_component_carrier_receipts()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing component-resolved carrier receipt set",
            ))?,
        &ending_stage3,
        &final_segment.ending_resource_owners,
    )?;
    owner_join.validate(
        final_executor.stack.last_final_boundary_receipts().ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "missing final covered boundary receipt set",
            ),
        )?,
        final_executor.stack.last_lane_boundary_receipts().ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "missing final covered lane-boundary receipt set",
            ),
        )?,
        final_executor
            .stack
            .last_component_carrier_receipts()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing component-resolved carrier receipt set",
            ))?,
        &ending_stage3,
        &final_segment.ending_resource_owners,
    )?;
    let mut parent = beginning_parent.clone();
    parent.accept_segment(&context.vegetation_configuration, final_segment)?;
    let consumer = final_executor.stack.take_staged_ending().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged covered ending"),
    )?;
    let parent_after_segment = parent.clone();
    let finalized = parent.finalize(&context.vegetation_configuration)?;
    Ok((
        parent_after_segment,
        consumer,
        final_clock,
        finalized,
        ending_stage3,
        owner_join,
    ))
}

fn owner_states_from_envelopes(
    owners: &BTreeMap<String, V11OwnerEnvelope>,
) -> Result<Vec<OwnerState>, DirectSnowStage3V11AttachmentError> {
    let values = owners
        .values()
        .map(V11OwnerEnvelope::to_owner_state)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(values)
}

/// The Stage-3 persistent state is the sole authoritative snow owner. The
/// hydrology winter-column fields remain a checked compatibility projection;
/// they are intentionally absent from this canonical owner envelope.
fn canonical_stage3_snow_owner_bytes(
    states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    #[derive(Serialize)]
    struct CanonicalSnowOwner<'a> {
        schema: &'static str,
        lanes: Vec<(&'a u32, &'a DirectSnowStage3PersistentState)>,
    }
    serde_json::to_vec(&CanonicalSnowOwner {
        schema: "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V1",
        lanes: states.iter().collect(),
    })
    .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("canonical Stage-3 snow bytes"))
}

fn parse_lower_hex_digest(value: &str) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    if value.len() != 64
        || value
            .bytes()
            .any(|byte| !(byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
    {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "provider receipt digest encoding",
        ));
    }
    let mut bytes = [0_u8; 32];
    for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
        let text = std::str::from_utf8(chunk).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Support("provider receipt digest encoding")
        })?;
        bytes[index] = u8::from_str_radix(text, 16).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Support("provider receipt digest digits")
        })?;
    }
    Ok(Digest32::from_bytes(bytes))
}

fn validate_lane_destination_set(
    bound_ofe_id: &str,
    identities: &[PreparedStage3V11SupportIdentityV1],
    expected: &BTreeSet<(String, String)>,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let actual = identities
        .iter()
        .map(|identity| {
            (
                identity.destination_ofe_id.clone(),
                identity.destination_tile_id.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    if identities
        .iter()
        .any(|identity| identity.destination_ofe_id != bound_ofe_id)
        || &actual != expected
    {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "support lane/OFE destination binding",
        ));
    }
    Ok(())
}

fn validate_parent_support_duration(
    duration_ns: u128,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if duration_ns != STAGE3_V11_PARENT_SUPPORT_NS {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "support duration is not 1,800 seconds",
        ));
    }
    Ok(())
}

fn day_start_ns(day_index: usize) -> Result<u128, DirectSnowStage3V11AttachmentError> {
    u128::try_from(day_index)
        .map_err(|_| DirectSnowStage3V11AttachmentError::Support("day index width"))?
        .checked_mul(STAGE3_V11_DAY_NS)
        .ok_or(DirectSnowStage3V11AttachmentError::Support(
            "run-relative day start overflow",
        ))
}

fn owner_envelopes_from_states(
    owners: &[OwnerState],
) -> Result<BTreeMap<String, V11OwnerEnvelope>, DirectSnowStage3V11AttachmentError> {
    owners
        .iter()
        .map(|owner| {
            Ok((
                owner.owner_id().to_owned(),
                V11OwnerEnvelope::try_new(
                    owner.owner_id().to_owned(),
                    owner.state_bytes().to_vec(),
                )?,
            ))
        })
        .collect()
}

#[allow(
    clippy::cast_precision_loss,
    clippy::large_types_passed_by_value,
    clippy::too_many_arguments
)]
fn select_actual_terminal_candidate(
    inputs: &DirectActiveSnowPartitionInputs,
    state: &DirectSnowStage3PersistentState,
    lane_id: u32,
    interval_index: u64,
    support: &DirectSnowStage3V11PreparedSupport,
    support_forcing: DirectSnowStage3SupportInput,
    full_result: DirectSnowTerminalEventResult,
    minimum_support_ns: u128,
) -> Result<
    (
        DirectSnowStage3V11TerminalReceipt,
        DirectSnowStage3PersistentDayResult,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let start = support.support.start_ns();
    let end = support.support.end_ns();
    let duration_s = support.support.duration_ns() as f64 / 1.0e9;
    let mut relative_seconds = vec![
        0.0,
        duration_s,
        full_result.hour_offset_seconds,
        full_result.event_bracket_lower_seconds,
        full_result.event_bracket_upper_seconds,
    ];
    relative_seconds
        .retain(|seconds| seconds.is_finite() && *seconds >= 0.0 && *seconds <= duration_s);
    let mut candidate_ticks = relative_seconds
        .into_iter()
        .map(|seconds| {
            quantize_seconds_to_tick(
                ModelTimeNs::new(0),
                ModelTimeNs::new(support.support.duration_ns()),
                seconds,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    candidate_ticks.sort_unstable();
    candidate_ticks.dedup();
    let candidate_ticks = candidate_ticks
        .into_iter()
        .map(|tick| ModelTimeNs::new(start.get() + tick.get()))
        .collect::<Vec<_>>();
    let mut accepted = Vec::new();
    for tick in &candidate_ticks {
        let pre = tick.get() - start.get();
        let post = end.get() - tick.get();
        if pre != 0 && pre < minimum_support_ns || post != 0 && post < minimum_support_ns {
            continue;
        }
        let duration_seconds = (tick.get() - start.get()) as f64 / 1.0e9;
        if duration_seconds <= 0.0 {
            continue;
        }
        let trial = Wb11HydrologyKernel::evaluate_stage3_persistent_support(
            inputs,
            state,
            lane_id,
            interval_index,
            DirectSnowStage3SupportInput {
                forcing: support_forcing.forcing,
                duration_seconds,
            },
            DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
        )?;
        let Some(result) = trial.terminal_event else {
            continue;
        };
        let actual_offset = quantize_seconds_to_tick(
            ModelTimeNs::new(0),
            ModelTimeNs::new(support.support.duration_ns()),
            result.hour_offset_seconds,
        )?;
        let actual_tick = ModelTimeNs::new(start.get() + actual_offset.get());
        if result.event_occurred && actual_tick == *tick {
            accepted.push((*tick, trial, result));
        }
    }
    let (accepted_event_tick, result, terminal) = accepted
        .into_iter()
        .min_by_key(|(tick, _, _)| {
            tick.get().abs_diff(
                start.get()
                    + quantize_seconds_to_tick(
                        ModelTimeNs::new(0),
                        ModelTimeNs::new(support.support.duration_ns()),
                        full_result.hour_offset_seconds,
                    )
                    .map_or(0, ModelTimeNs::get),
            )
        })
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "no actual terminal candidate satisfied coupled support",
        ))?;
    Ok((
        DirectSnowStage3V11TerminalReceipt {
            lane_id,
            support: support.support,
            result: terminal,
            candidate_ticks,
            accepted_event_tick,
        },
        result,
    ))
}

fn validate_receiver_topology(
    records: &[DirectSurfaceLiquidConfigurationRecord],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let mut fractions = BTreeMap::<String, f64>::new();
    for record in records {
        if !record.tile_fraction.is_finite() || record.tile_fraction <= 0.0 {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "receiver tile fraction",
            ));
        }
        let entry = fractions
            .entry(record.key.ofe_id.to_string())
            .or_insert(0.0);
        *entry += record.tile_fraction;
    }
    if fractions.values().any(|sum| (sum - 1.0).abs() > 1.0e-12) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "receiver area split",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn support_identity(ofe_id: &str, tile_id: &str) -> PreparedStage3V11SupportIdentityV1 {
        PreparedStage3V11SupportIdentityV1::new(
            ofe_id.to_owned(),
            tile_id.to_owned(),
            "a".repeat(64),
            Digest32::zero(),
            Vec::new(),
            Digest32::zero(),
        )
    }

    #[test]
    fn parent_support_cadence_is_exactly_1_800_seconds() {
        assert_eq!(STAGE3_V11_PARENT_SUPPORT_NS, 1_800_000_000_000);
        let support = TimeSupport::new(
            ModelTimeNs::new(0),
            ModelTimeNs::new(STAGE3_V11_PARENT_SUPPORT_NS),
        )
        .expect("valid parent support");
        assert_eq!(support.duration_ns(), 1_800_000_000_000);
        assert_eq!(support.duration_s_bits(), 1_800.0_f64.to_bits());
        assert_eq!(
            STAGE3_V11_PARENT_SUPPORT_NS * STAGE3_V11_PARENT_SUPPORT_COUNT as u128,
            86_400_000_000_000
        );
        assert!(validate_parent_support_duration(1_800_000_000).is_err());
        assert!(validate_parent_support_duration(STAGE3_V11_PARENT_SUPPORT_NS + 1).is_err());
        validate_parent_support_duration(STAGE3_V11_PARENT_SUPPORT_NS)
            .expect("1,800-second support accepted");
    }

    #[test]
    fn run_relative_day_supports_are_contiguous_across_midnight() {
        assert_eq!(day_start_ns(0).expect("day zero start"), 0);
        assert_eq!(day_start_ns(1).expect("day one start"), STAGE3_V11_DAY_NS);
        let day_zero_last = TimeSupport::new(
            ModelTimeNs::new(
                STAGE3_V11_DAY_NS
                    .checked_sub(STAGE3_V11_PARENT_SUPPORT_NS)
                    .expect("day zero last support start"),
            ),
            ModelTimeNs::new(STAGE3_V11_DAY_NS),
        )
        .expect("day zero last support");
        let day_one_first = TimeSupport::new(
            ModelTimeNs::new(STAGE3_V11_DAY_NS),
            ModelTimeNs::new(
                STAGE3_V11_DAY_NS
                    .checked_add(STAGE3_V11_PARENT_SUPPORT_NS)
                    .expect("day one first support end"),
            ),
        )
        .expect("day one first support");
        assert_eq!(day_zero_last.end_ns(), day_one_first.start_ns());
        assert_eq!(day_one_first.start_ns().get(), 86_400_000_000_000);
    }

    #[test]
    fn parent_forcing_digest_binds_interval_receipt_identity() {
        let support = TimeSupport::new(
            ModelTimeNs::new(0),
            ModelTimeNs::new(STAGE3_V11_PARENT_SUPPORT_NS),
        )
        .expect("support");
        let mut identities = BTreeMap::from([(7, vec![support_identity("ofe-1", "tile-1")])]);
        let first = canonical_parent_forcing_digest_from_parts(
            0,
            0,
            Digest32::from_bytes([1; 32]),
            support,
            "b".repeat(64).as_str(),
            &identities,
        )
        .expect("first forcing digest");
        identities.get_mut(&7).expect("lane")[0].forcing_receipt_digest =
            Digest32::from_bytes([2; 32]);
        let second = canonical_parent_forcing_digest_from_parts(
            0,
            0,
            Digest32::from_bytes([1; 32]),
            support,
            "b".repeat(64).as_str(),
            &identities,
        )
        .expect("second forcing digest");
        assert_ne!(first, second);
    }

    #[test]
    fn parent_interval_identity_binds_run_relative_support() {
        let first_support = TimeSupport::new(
            ModelTimeNs::new(0),
            ModelTimeNs::new(STAGE3_V11_PARENT_SUPPORT_NS),
        )
        .expect("first support");
        let second_support = TimeSupport::new(
            ModelTimeNs::new(STAGE3_V11_PARENT_SUPPORT_NS),
            ModelTimeNs::new(STAGE3_V11_DAY_NS),
        )
        .expect("second support");
        let first = ParentIntervalId::derive(
            Digest32::from_bytes([1; 32]),
            Digest32::from_bytes([2; 32]),
            Digest32::from_bytes([3; 32]),
            first_support,
        )
        .expect("first parent interval");
        let second = ParentIntervalId::derive(
            Digest32::from_bytes([1; 32]),
            Digest32::from_bytes([2; 32]),
            Digest32::from_bytes([3; 32]),
            second_support,
        )
        .expect("second parent interval");
        assert_ne!(first, second);
    }

    #[test]
    fn lane_destination_permutation_fails_exact_lane_ofe_join() {
        let mut provider_destinations_by_ofe = BTreeMap::new();
        provider_destinations_by_ofe.insert(
            "ofe-1".to_owned(),
            BTreeSet::from([("ofe-1".to_owned(), "tile-1".to_owned())]),
        );
        provider_destinations_by_ofe.insert(
            "ofe-2".to_owned(),
            BTreeSet::from([("ofe-2".to_owned(), "tile-2".to_owned())]),
        );
        let lane_one_identities = vec![support_identity("ofe-2", "tile-2")];
        let lane_two_identities = vec![support_identity("ofe-1", "tile-1")];

        assert!(
            validate_lane_destination_set(
                "ofe-1",
                &lane_one_identities,
                provider_destinations_by_ofe
                    .get("ofe-1")
                    .expect("lane one OFE destinations"),
            )
            .is_err()
        );
        assert!(
            validate_lane_destination_set(
                "ofe-2",
                &lane_two_identities,
                provider_destinations_by_ofe
                    .get("ofe-2")
                    .expect("lane two OFE destinations"),
            )
            .is_err()
        );
    }
}
