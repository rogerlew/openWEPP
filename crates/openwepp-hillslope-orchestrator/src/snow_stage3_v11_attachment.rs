//! Typed, default-off Stage-3/V11 parent attachment.
//!
//! This boundary owns the constitutive Stage-3 support cadence and terminal
//! event projection.  It deliberately accepts a prepared forcing capability
//! rather than an event request or live carrier receipt.  The legacy
//! caller-built handoff remains test-only in `direct_runtime::snow_stage3_shadow`.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_coupled_time::{
    ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, Digest32, EventClass,
    EventProposalV1, EventQueueV1, FramedField, LedgerEntryV1, ModelTimeNs, OwnerState,
    ParentAuthorityV1, ParentIntervalId, StepConstraintV1, TimeSupport, accept_slab,
    complete_owner_set_digest, digest_bytes, framed_sha256, quantize_seconds_to_tick,
    reduce_constraints,
};
use openwepp_kernel_contract::{SoilLayerId, TileId};
use openwepp_land_surface_energy::OfeId;
use openwepp_meteorology::psychrometrics::saturation_vapor_pressure_water_kpa;
use openwepp_meteorology::snow_free_forcing::{celsius_to_kelvin, kilopascals_to_pascals};
use openwepp_unit_boundary::TemperatureCelsius;
use openwepp_vegetation::v11::{
    V11OwnerEnvelope, V11ParentCandidate, V11ParentTransaction, VegetationConfigurationV11,
};
use serde::Serialize;
use thiserror::Error;

use crate::hydrology::{
    DirectActiveSnowPartitionInputs, DirectSnowStage3PersistentDayResult,
    DirectSnowStage3PersistentState, DirectSnowStage3SupportInput, DirectSnowTerminalEventRequest,
    DirectSnowTerminalEventResult, Wb11HydrologyKernel, stage3_has_represented_ice,
    stage3_is_resolved_thermal_domain,
};
use crate::runtime_inputs::{
    PreparedSnowFreeGsiDayV1, SnowFreeHalfHourForcingError, SnowFreeHalfHourIntervalReceipt,
    SnowFreeHalfHourProviderCursor, SnowFreePrecipitationParcelReceipt, direct_gsi_state,
};
use crate::snow_stage3_open_boundary::{
    FinalStage3TileBoundaryReceiptV1, SealedOpenSnowExposureReceiptV1,
    SealedOpenSnowTileForcingInputsV1, SealedOpenSnowTileForcingV1,
    SealedStage3TileBoundaryForcingV1,
};
use crate::snow_stage3_terminal_handoff::{
    LaneStage3BoundaryReceiptV1, SealedCoveredCarrierForcing, SnowStage3HandoffError,
};
use crate::v9_real_consumer_shadow::DirectV10RealConsumerShadow;
use crate::v9_real_consumer_shadow::{
    CoveredParentOwnerJoinReceiptV1, CoveredPhysicalCustodyJoinInputs, DirectV9ShadowIntervalInput,
    DirectV11RealConsumerError, DirectV11RealConsumerStack, DirectV11SnowCoveredRealConsumerStack,
    DirectV11SnowCoveredSegmentInput, DirectV11SnowCoveredStackInputs,
};
use crate::v11_vegetation_consumer::{accept_direct_v11_segment, execute_direct_v11_segment};
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
    #[error("SNOWENERGY-E-PRECIP-001: {0}")]
    Precipitation(&'static str),
    #[error("SNOWENERGY-E-SOIL-HEAT-001: {0}")]
    SnowSoilHeat(&'static str),
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
    #[error(transparent)]
    SnowBoundary(#[from] SnowStage3HandoffError),
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
    /// Complete active snow-surface forcing topology, keyed by physical
    /// destination. Empty means the support is snow-free.
    snow_surface_forcing_by_destination:
        BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
    open_snow_destination_requests: BTreeSet<(OfeId, TileId)>,
    atmospheric_receipt_by_destination: BTreeMap<(OfeId, TileId), Stage3ParentAtmosphericReceiptV1>,
    /// Covered V11 projection. It is a separate type from the snow-free
    /// interval so regime selection is explicit at the sealed-support seam.
    covered_v11_interval: Option<DirectV11SnowCoveredSegmentInput>,
    /// Provider-owned destination and receipt identity. The physical
    /// precipitation parcel remains sealed input; it is not a terminal parcel
    /// and cannot contain an ending owner or event time.
    support_identity_by_lane: BTreeMap<u32, Vec<PreparedStage3V11SupportIdentityV1>>,
    hard_boundaries: Vec<ModelTimeNs>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage3SnowSurfaceRegime {
    SnowFree,
    OpenSnowOnly,
    CanopyCoveredOrMixed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage3LaneLifecycleV1 {
    SnowFree,
    ResolvedSnow,
    TerminalPending,
    SolidPrecipitationPending,
}

include!("snow_stage3_v11_precipitation.rs");
include!("snow_stage3_v11_snow_soil_heat.rs");
include!("snow_stage3_v11_terminal_chronology.rs");
include!("stage3_parent_atmosphere.rs");

pub(crate) fn stage3_lane_lifecycle(
    state: &DirectSnowStage3PersistentState,
    snowfall_m: f64,
) -> Stage3LaneLifecycleV1 {
    if stage3_is_resolved_thermal_domain(state) {
        return Stage3LaneLifecycleV1::ResolvedSnow;
    }
    let has_terminal_liquid = state.detached_retained_liquid_kg_m2 > 0.0
        || state
            .layers
            .iter()
            .any(|layer| layer.liquid_water_m > 0.0 || layer.refrozen_liquid_m > 0.0);
    if stage3_has_represented_ice(state) || has_terminal_liquid {
        return Stage3LaneLifecycleV1::TerminalPending;
    }
    if snowfall_m > 0.0 {
        return Stage3LaneLifecycleV1::SolidPrecipitationPending;
    }
    Stage3LaneLifecycleV1::SnowFree
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
            snow_surface_forcing_by_destination: BTreeMap::new(),
            open_snow_destination_requests: BTreeSet::new(),
            atmospheric_receipt_by_destination: BTreeMap::new(),
            covered_v11_interval: None,
            support_identity_by_lane,
            hard_boundaries: Vec::new(),
        })
    }

    /// Add exact coupled-time event/restart/output boundaries that may
    /// truncate a Stage-3 cadence proposal without creating a zero-duration
    /// physics child.
    pub fn with_hard_boundaries(
        mut self,
        mut boundaries: Vec<ModelTimeNs>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        boundaries.sort_unstable();
        boundaries.dedup();
        if boundaries.iter().any(|boundary| {
            *boundary <= self.support.start_ns() || *boundary >= self.support.end_ns()
        }) {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "coupled hard boundary outside parent interior",
            ));
        }
        self.hard_boundaries = boundaries;
        Ok(self)
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

    #[cfg(test)]
    #[must_use]
    pub(crate) fn with_sealed_open_tile_forcing(
        mut self,
        destination: (OfeId, TileId),
        forcing: SealedOpenSnowTileForcingV1,
    ) -> Self {
        self.snow_surface_forcing_by_destination.insert(
            destination,
            SealedStage3TileBoundaryForcingV1::OpenSnow(forcing),
        );
        self
    }

    /// Derive and seal one open-snow destination from the prepared provider
    /// projection. Callers identify the retained raw-wind provider and the
    /// admitted identity projection; all meteorological scalars come from the
    /// interval that is subsequently joined to the provider day.
    pub fn with_provider_open_snow_destination(
        mut self,
        destination: (OfeId, TileId),
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        self.support_identity_by_lane
            .values()
            .flatten()
            .find(|identity| {
                identity.destination_ofe_id == destination.0.as_str()
                    && identity.destination_tile_id == destination.1.as_str()
            })
            .ok_or(DirectSnowStage3V11AttachmentError::Support(
                "open-snow destination provider identity",
            ))?;
        self.open_snow_destination_requests.insert(destination);
        Ok(self)
    }

    fn bind_provider_atmosphere(
        &mut self,
        provider_destinations: &BTreeMap<(String, String), &SnowFreeHalfHourIntervalReceipt>,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.atmospheric_receipt_by_destination.clear();
        for (destination, provider) in provider_destinations {
            let typed_destination = (
                OfeId::try_new(destination.0.clone()).map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity("provider atmosphere OFE")
                })?,
                TileId::try_new(destination.1.clone()).map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity("provider atmosphere tile")
                })?,
            );
            let atmosphere =
                Stage3ParentAtmosphericReceiptV1::from_provider(self.support, provider)?;
            self.validate_atmospheric_projections(provider, &atmosphere)?;
            self.atmospheric_receipt_by_destination
                .insert(typed_destination, atmosphere);
        }
        let requests = self.open_snow_destination_requests.clone();
        for destination in requests {
            let atmosphere = self
                .atmospheric_receipt_by_destination
                .get(&destination)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "open-snow provider atmosphere destination",
                ))?;
            let provider = provider_destinations
                .get(&(
                    destination.0.as_str().to_owned(),
                    destination.1.as_str().to_owned(),
                ))
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "open-snow provider destination",
                ))?;
            let source_wind_provider_sha256 =
                parse_lower_hex_digest(&provider.provider_definition_sha256)?;
            let projection_model_definition_sha256 =
                digest_bytes(b"OPENWEPP_STAGE3_RAW_WIND_IDENTITY_PROJECTION_V1");
            let exposure = SealedOpenSnowExposureReceiptV1::try_new(
                self.support,
                destination.clone(),
                atmosphere.provider_interval_receipt_sha256,
                source_wind_provider_sha256,
                atmosphere.raw_wind_m_s,
                projection_model_definition_sha256,
            )?;
            let open = SealedOpenSnowTileForcingV1::try_new(SealedOpenSnowTileForcingInputsV1 {
                support: self.support,
                destination: destination.clone(),
                forcing_receipt_sha256: atmosphere.provider_interval_receipt_sha256,
                exposure: exposure.clone(),
                reference_temperature_k: atmosphere.air_temperature_k,
                reference_specific_humidity_kg_kg: atmosphere.specific_humidity_kg_kg,
                air_pressure_pa: atmosphere.air_pressure_pa,
                atmospheric_downward_longwave_w_m2: atmosphere.downward_longwave_w_m2,
                direct_vis_w_m2: atmosphere.direct_vis_w_m2,
                diffuse_vis_w_m2: atmosphere.diffuse_vis_w_m2,
                direct_nir_w_m2: atmosphere.direct_nir_w_m2,
                diffuse_nir_w_m2: atmosphere.diffuse_nir_w_m2,
                rain_m: 0.0,
                snowfall_m: 0.0,
                precipitation_parcel_count: provider.precipitation_parcels.len(),
            })?;
            let identity = self
                .support_identity_by_lane
                .values_mut()
                .flatten()
                .find(|identity| {
                    identity.destination_ofe_id == destination.0.as_str()
                        && identity.destination_tile_id == destination.1.as_str()
                })
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "open-snow provider identity update",
                ))?;
            identity.exposure_identity = exposure.receipt_sha256;
            self.snow_surface_forcing_by_destination.insert(
                destination,
                SealedStage3TileBoundaryForcingV1::OpenSnow(open),
            );
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn validate_atmospheric_projections(
        &self,
        provider: &SnowFreeHalfHourIntervalReceipt,
        atmosphere: &Stage3ParentAtmosphericReceiptV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        fn same(lhs: f64, rhs: f64) -> bool {
            lhs.to_bits() == rhs.to_bits()
        }
        let validate_lse = |forcing: &openwepp_land_surface_energy::LandSurfaceForcing| {
            same(forcing.air_temperature_k, atmosphere.air_temperature_k)
                && same(
                    forcing.air_specific_humidity_kg_kg,
                    atmosphere.specific_humidity_kg_kg,
                )
                && same(forcing.air_pressure_pa, atmosphere.air_pressure_pa)
                && same(forcing.reference_wind_m_s, atmosphere.raw_wind_m_s)
                && same(forcing.direct_vis_w_m2, atmosphere.direct_vis_w_m2)
                && same(forcing.diffuse_vis_w_m2, atmosphere.diffuse_vis_w_m2)
                && same(forcing.direct_nir_w_m2, atmosphere.direct_nir_w_m2)
                && same(forcing.diffuse_nir_w_m2, atmosphere.diffuse_nir_w_m2)
                && same(
                    forcing.atmospheric_downward_longwave_w_m2,
                    atmosphere.downward_longwave_w_m2,
                )
        };
        if !validate_lse(&self.v11_interval.lse_forcing)
            || self
                .covered_v11_interval
                .as_ref()
                .is_some_and(|covered| !validate_lse(&covered.lse_forcing))
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "base/covered V11 provider atmosphere projection",
            ));
        }
        let base_vegetation = &self.v11_interval.vegetation_forcing;
        if !same(
            base_vegetation.air_temperature_k,
            atmosphere.air_temperature_k,
        ) || !same(base_vegetation.pressure_pa, atmosphere.air_pressure_pa)
            || !same(base_vegetation.wind_m_s, atmosphere.raw_wind_m_s)
            || !same(
                base_vegetation.specific_humidity,
                atmosphere.specific_humidity_kg_kg,
            )
            || !same(base_vegetation.direct_par_w_m2, atmosphere.direct_vis_w_m2)
            || !same(
                base_vegetation.diffuse_par_w_m2,
                atmosphere.diffuse_vis_w_m2,
            )
            || !same(base_vegetation.direct_nir_w_m2, atmosphere.direct_nir_w_m2)
            || !same(
                base_vegetation.diffuse_nir_w_m2,
                atmosphere.diffuse_nir_w_m2,
            )
            || !same(
                base_vegetation.longwave_down_w_m2,
                atmosphere.downward_longwave_w_m2,
            )
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "base V11 vegetation provider atmosphere projection",
            ));
        }
        if let Some(covered) = &self.covered_v11_interval {
            let vegetation = &covered.vegetation_forcing;
            if !same(vegetation.air_temperature_k, atmosphere.air_temperature_k)
                || !same(vegetation.pressure_pa, atmosphere.air_pressure_pa)
                || !same(vegetation.wind_m_s, atmosphere.raw_wind_m_s)
                || !same(
                    vegetation.specific_humidity,
                    atmosphere.specific_humidity_kg_kg,
                )
                || !same(vegetation.direct_par_w_m2, atmosphere.direct_vis_w_m2)
                || !same(vegetation.diffuse_par_w_m2, atmosphere.diffuse_vis_w_m2)
                || !same(vegetation.direct_nir_w_m2, atmosphere.direct_nir_w_m2)
                || !same(vegetation.diffuse_nir_w_m2, atmosphere.diffuse_nir_w_m2)
                || !same(
                    vegetation.longwave_down_w_m2,
                    atmosphere.downward_longwave_w_m2,
                )
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "covered V11 vegetation provider atmosphere projection",
                ));
            }
        }
        let dewpoint = TemperatureCelsius::try_new(provider.dew_point_c).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("provider dewpoint domain")
        })?;
        let dewpoint_vapor_pa = kilopascals_to_pascals(
            saturation_vapor_pressure_water_kpa(dewpoint)
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "provider dewpoint vapor projection",
                    )
                })?
                .as_kilopascals(),
        );
        if !same(dewpoint_vapor_pa, atmosphere.actual_vapor_pressure_pa) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "specific-humidity/dewpoint provider contradiction",
            ));
        }
        for (lane_id, inputs) in &self.snow_inputs_by_lane {
            let support_forcing = self.support_forcing_by_lane.get(lane_id).ok_or(
                DirectSnowStage3V11AttachmentError::Identity("Stage-3 atmosphere lane"),
            )?;
            if !same(inputs.wind_m_s, atmosphere.raw_wind_m_s)
                || !same(inputs.dewpoint_c, provider.dew_point_c)
                || !same(
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                    atmosphere.air_pressure_pa,
                )
                || !same(
                    support_forcing.forcing.air_temperature_c,
                    provider.air_temperature_c,
                )
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "Stage-3/open-forcing atmosphere projection",
                ));
            }
        }
        for (destination, forcing) in &self.snow_surface_forcing_by_destination {
            let matches_destination = destination.0.as_str() == provider.ofe_id
                && destination.1.as_str() == provider.tile_id;
            if !matches_destination {
                continue;
            }
            match forcing {
                SealedStage3TileBoundaryForcingV1::V11CanopyCovered(covered) => {
                    if !same(
                        covered.reference_temperature_k,
                        atmosphere.air_temperature_k,
                    ) || !same(
                        covered.reference_specific_humidity,
                        atmosphere.specific_humidity_kg_kg,
                    ) || !same(
                        covered.atmospheric_longwave_w_m2,
                        atmosphere.downward_longwave_w_m2,
                    ) || !same(covered.exposure.wind_m_s, atmosphere.raw_wind_m_s)
                        || covered.exposure.provider_digest != provider.provider_definition_sha256
                    {
                        return Err(DirectSnowStage3V11AttachmentError::Identity(
                            "covered carrier provider atmosphere projection",
                        ));
                    }
                }
                SealedStage3TileBoundaryForcingV1::OpenSnow(_) => {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "open-snow forcing must be sealed during provider binding",
                    ));
                }
            }
        }
        Ok(())
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
    pub fn snow_surface_forcing_by_destination(
        &self,
    ) -> &BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1> {
        &self.snow_surface_forcing_by_destination
    }

    #[must_use]
    pub fn atmospheric_receipt_by_destination(
        &self,
    ) -> &BTreeMap<(OfeId, TileId), Stage3ParentAtmosphericReceiptV1> {
        &self.atmospheric_receipt_by_destination
    }

    fn coupled_subslab(
        &self,
        support: TimeSupport,
        _child_ordinal: u32,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        if support.start_ns() < self.support.start_ns() || support.end_ns() > self.support.end_ns()
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "coupled subslab outside prepared parent support",
            ));
        }
        let duration_seconds = f64::from_bits(support.duration_s_bits());
        let parent_duration_seconds = f64::from_bits(self.support.duration_s_bits());
        let child_offset_seconds =
            (support.start_ns().get() - self.support.start_ns().get()) as f64 / 1_000_000_000.0;
        let partition_parcels = |parcels: &[openwepp_land_surface_energy::LiquidParcel]| {
            parcels
                .iter()
                .filter_map(|parcel| {
                    let overlap_start = parcel.start_s.max(child_offset_seconds);
                    let overlap_end = parcel.end_s.min(child_offset_seconds + duration_seconds);
                    (overlap_end > overlap_start).then(|| {
                        let mut child = parcel.clone();
                        let fraction =
                            (overlap_end - overlap_start) / (parcel.end_s - parcel.start_s);
                        child.start_s = overlap_start - child_offset_seconds;
                        child.end_s = overlap_end - child_offset_seconds;
                        child.amount_kg_m2_destination_tile_ground *= fraction;
                        child
                    })
                })
                .collect::<Vec<_>>()
        };
        let segment_interval = |input: &DirectV9ShadowIntervalInput| {
            let mut value = input.clone();
            value.lse_forcing.interval_s = duration_seconds;
            value.lse_forcing.precipitation_parcels =
                partition_parcels(&input.lse_forcing.precipitation_parcels);
            value.lse_forcing.runon_parcels = partition_parcels(&input.lse_forcing.runon_parcels);
            value.lse_forcing.forcing_sha256 =
                value.lse_forcing.canonical_sha256().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "coupled subslab LSE forcing digest",
                    )
                })?;
            Ok::<_, DirectSnowStage3V11AttachmentError>(value)
        };
        let v11_interval = segment_interval(&self.v11_interval)?;
        let covered_v11_interval = self
            .covered_v11_interval
            .as_ref()
            .map(|input| {
                let mut lse_forcing = input.lse_forcing.clone();
                lse_forcing.interval_s = duration_seconds;
                lse_forcing.precipitation_parcels =
                    partition_parcels(&input.lse_forcing.precipitation_parcels);
                lse_forcing.runon_parcels = partition_parcels(&input.lse_forcing.runon_parcels);
                lse_forcing.forcing_sha256 = lse_forcing.canonical_sha256().map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "coupled subslab covered LSE forcing digest",
                    )
                })?;
                DirectV11SnowCoveredSegmentInput::try_new(
                    lse_forcing,
                    input.vegetation_forcing.clone(),
                    input.wb14_parameters.clone(),
                )
            })
            .transpose()?;
        let support_forcing_by_lane = self
            .support_forcing_by_lane
            .iter()
            .map(|(lane_id, forcing)| {
                let mut child_forcing = forcing.forcing;
                let support_fraction = duration_seconds / parent_duration_seconds;
                child_forcing.active_precipitation_m *= support_fraction;
                child_forcing.rain_m *= support_fraction;
                child_forcing.snowfall_m *= support_fraction;
                child_forcing.radiation_mj_m2 *= support_fraction;
                (
                    *lane_id,
                    DirectSnowStage3SupportInput {
                        forcing: child_forcing,
                        duration_seconds,
                    },
                )
            })
            .collect();
        let snow_surface_forcing_by_destination = self
            .snow_surface_forcing_by_destination
            .iter()
            .map(|(destination, forcing)| {
                let projected = match forcing {
                    SealedStage3TileBoundaryForcingV1::V11CanopyCovered(value) => {
                        SealedStage3TileBoundaryForcingV1::V11CanopyCovered(value.clone())
                    }
                    SealedStage3TileBoundaryForcingV1::OpenSnow(value) => {
                        let exposure = SealedOpenSnowExposureReceiptV1::try_new(
                            support,
                            destination.clone(),
                            value.exposure.source_forcing_receipt_sha256,
                            value.exposure.source_wind_provider_sha256,
                            value.exposure.raw_or_projected_wind_m_s,
                            value.exposure.projection_model_definition_sha256,
                        )?;
                        SealedStage3TileBoundaryForcingV1::OpenSnow(
                            SealedOpenSnowTileForcingV1::try_new(
                                SealedOpenSnowTileForcingInputsV1 {
                                    support,
                                    destination: destination.clone(),
                                    forcing_receipt_sha256: value.forcing_receipt_sha256,
                                    exposure,
                                    reference_temperature_k: value.reference_temperature_k,
                                    reference_specific_humidity_kg_kg: value
                                        .reference_specific_humidity_kg_kg,
                                    air_pressure_pa: value.air_pressure_pa,
                                    atmospheric_downward_longwave_w_m2: value
                                        .atmospheric_downward_longwave_w_m2,
                                    direct_vis_w_m2: value.direct_vis_w_m2,
                                    diffuse_vis_w_m2: value.diffuse_vis_w_m2,
                                    direct_nir_w_m2: value.direct_nir_w_m2,
                                    diffuse_nir_w_m2: value.diffuse_nir_w_m2,
                                    rain_m: value.rain_m,
                                    snowfall_m: value.snowfall_m,
                                    precipitation_parcel_count: value.precipitation_parcel_count,
                                },
                            )?,
                        )
                    }
                };
                Ok((destination.clone(), projected))
            })
            .collect::<Result<BTreeMap<_, _>, DirectSnowStage3V11AttachmentError>>()?;
        Ok(Self {
            support,
            snow_inputs_by_lane: self.snow_inputs_by_lane.clone(),
            support_forcing_by_lane,
            v11_interval,
            snow_surface_forcing_by_destination,
            open_snow_destination_requests: self.open_snow_destination_requests.clone(),
            atmospheric_receipt_by_destination: self.atmospheric_receipt_by_destination.clone(),
            covered_v11_interval,
            support_identity_by_lane: self.support_identity_by_lane.clone(),
            hard_boundaries: self.hard_boundaries.clone(),
        })
    }

    fn retain_active_snow_lanes(
        mut self,
        active_lanes: &BTreeSet<u32>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let active_destinations = self
            .support_identity_by_lane
            .iter()
            .filter(|(lane, _)| active_lanes.contains(lane))
            .flat_map(|(_, identities)| identities)
            .map(|identity| {
                Ok((
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "terminal successor OFE identity",
                        )
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "terminal successor tile identity",
                        )
                    })?,
                ))
            })
            .collect::<Result<BTreeSet<_>, DirectSnowStage3V11AttachmentError>>()?;
        self.snow_surface_forcing_by_destination
            .retain(|destination, _| active_destinations.contains(destination));
        self.open_snow_destination_requests
            .retain(|destination| active_destinations.contains(destination));
        Ok(self)
    }

    fn snow_free_successor(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        self.snow_surface_forcing_by_destination.clear();
        self.open_snow_destination_requests.clear();
        self.atmospheric_receipt_by_destination.clear();
        self.covered_v11_interval = None;
        self.v11_interval.lse_forcing.snow_present_at_beginning = false;
        self.v11_interval.lse_forcing.snow_present_at_end = false;
        self.v11_interval.lse_forcing.snow_terminal_payload_present = false;
        self.v11_interval.lse_forcing.forcing_sha256 = self
            .v11_interval
            .lse_forcing
            .canonical_sha256()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal successor LSE forcing digest",
                )
            })?;
        Ok(self)
    }

    #[cfg(test)]
    pub(crate) fn poison_base_air_temperature(&mut self) {
        self.v11_interval.lse_forcing.air_temperature_k = f64::from_bits(
            self.v11_interval
                .lse_forcing
                .air_temperature_k
                .to_bits()
                .wrapping_add(1),
        );
    }

    #[cfg(test)]
    pub(crate) fn poison_base_wind(&mut self) {
        self.v11_interval.lse_forcing.reference_wind_m_s = f64::from_bits(
            self.v11_interval
                .lse_forcing
                .reference_wind_m_s
                .to_bits()
                .wrapping_add(1),
        );
    }

    #[cfg(test)]
    pub(crate) fn poison_covered_atmosphere(&mut self, wind: bool) {
        let mut covered = DirectV11SnowCoveredSegmentInput::from_snow_free(&self.v11_interval);
        if wind {
            covered.lse_forcing.reference_wind_m_s = f64::from_bits(
                covered
                    .lse_forcing
                    .reference_wind_m_s
                    .to_bits()
                    .wrapping_add(1),
            );
        } else {
            covered.lse_forcing.air_temperature_k = f64::from_bits(
                covered
                    .lse_forcing
                    .air_temperature_k
                    .to_bits()
                    .wrapping_add(1),
            );
        }
        self.covered_v11_interval = Some(covered);
    }

    #[cfg(test)]
    pub(crate) fn poison_stage3_pressure(&mut self) {
        if let Some(inputs) = self.snow_inputs_by_lane.values_mut().next() {
            inputs.surface_energy_options.atmospheric_pressure_pa = f64::from_bits(
                inputs
                    .surface_energy_options
                    .atmospheric_pressure_pa
                    .to_bits()
                    .wrapping_add(1),
            );
        }
    }

    #[cfg(test)]
    pub(crate) fn poison_stage3_dewpoint(&mut self) {
        if let Some(inputs) = self.snow_inputs_by_lane.values_mut().next() {
            inputs.dewpoint_c = f64::from_bits(inputs.dewpoint_c.to_bits().wrapping_add(1));
        }
    }

    fn has_snow_surface_forcing(&self) -> bool {
        !self.snow_surface_forcing_by_destination.is_empty()
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
        if !self
            .snow_surface_forcing_by_destination
            .keys()
            .all(|destination| expected.contains(destination))
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "snow-surface destination outside configured topology",
            ));
        }
        for identities in self.support_identity_by_lane.values() {
            for identity in identities {
                let destination = (
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface OFE identity")
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow-surface tile identity")
                    })?,
                );
                let Some(physical) = self.snow_surface_forcing_by_destination.get(&destination)
                else {
                    continue;
                };
                let exposure_identity = match physical {
                    SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing) => {
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

    fn state_derived_active_snow_lanes(
        &self,
        beginning: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeSet<u32>, DirectSnowStage3V11AttachmentError> {
        let mut represented = BTreeMap::<u32, BTreeSet<(OfeId, TileId)>>::new();
        for (lane_id, identities) in &self.support_identity_by_lane {
            for identity in identities {
                let destination = (
                    OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow regime OFE")
                    })?,
                    TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Support("snow regime tile")
                    })?,
                );
                if self
                    .snow_surface_forcing_by_destination
                    .contains_key(&destination)
                {
                    represented.entry(*lane_id).or_default().insert(destination);
                }
            }
        }
        let mut active = BTreeSet::new();
        for (lane_id, state) in beginning {
            let forcing = self.support_forcing_by_lane.get(lane_id).ok_or(
                DirectSnowStage3V11AttachmentError::Support("snow regime lane forcing"),
            )?;
            let lifecycle = stage3_lane_lifecycle(state, forcing.forcing.snowfall_m);
            let expected = self
                .support_identity_by_lane
                .get(lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Support(
                    "snow regime lane topology",
                ))?
                .iter()
                .map(|identity| {
                    Ok((
                        OfeId::try_new(identity.destination_ofe_id.clone()).map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Support("snow regime OFE")
                        })?,
                        TileId::try_new(identity.destination_tile_id.clone()).map_err(|_| {
                            DirectSnowStage3V11AttachmentError::Support("snow regime tile")
                        })?,
                    ))
                })
                .collect::<Result<BTreeSet<_>, DirectSnowStage3V11AttachmentError>>()?;
            let actual = represented.get(lane_id).cloned().unwrap_or_default();
            match lifecycle {
                Stage3LaneLifecycleV1::ResolvedSnow => {
                    active.insert(*lane_id);
                    if actual != expected {
                        return Err(DirectSnowStage3V11AttachmentError::Support(
                            "active snow lane requires complete destination boundary set",
                        ));
                    }
                }
                Stage3LaneLifecycleV1::SnowFree => {
                    if !actual.is_empty() {
                        return Err(DirectSnowStage3V11AttachmentError::Support(
                            "snow-free lane cannot claim Stage-3 surface ownership",
                        ));
                    }
                }
                Stage3LaneLifecycleV1::TerminalPending => {
                    return Err(DirectSnowStage3V11AttachmentError::Terminal(
                        "Stage-3 lane requires terminal disposition",
                    ));
                }
                Stage3LaneLifecycleV1::SolidPrecipitationPending => {
                    return Err(DirectSnowStage3V11AttachmentError::Support(
                        "solid precipitation custody is unavailable",
                    ));
                }
            }
        }
        Ok(active)
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
        let carrier_configuration_sha256 =
            canonical_snow_surface_forcing_digest(&self.snow_surface_forcing_by_destination);
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

fn canonical_snow_surface_forcing_digest(
    by_destination: &BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
) -> Digest32 {
    let mut bytes = Vec::new();
    append_canonical_bytes(&mut bytes, b"OPENWEPP_STAGE3_SNOW_SURFACE_SET_V1");
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
        mut supports: Vec<DirectSnowStage3V11PreparedSupport>,
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
        for (support_index, support) in supports.iter_mut().enumerate() {
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
            support.bind_provider_atmosphere(&provider_destinations)?;
            support.validate_explicit_snow_surface_set()?;
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

include!("snow_stage3_v11_attachment_receipts.rs");
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11CommittedState {
    pub stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub real_consumer: DirectV10RealConsumerShadow,
    pub v11_parent_state: V11ParentTransaction,
    pub coupled_clock: CoupledClockStateV1,
    pub next_parent_sequence: u128,
    pub last_v11_parent_candidate: Option<V11ParentCandidate>,
    pub accepted_event_ordinal: u64,
    pub terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
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
    failure_injection: Option<Stage3V11FailureInjection>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stage3V11FailureInjection {
    SubslabAccepted(usize),
    OutcomeLedgerBuilt(usize),
    PrecipitationReceiptRejected(usize),
    SnowSoilHeatReceiptRejected(usize),
    FinalOwnerJoinCompleted,
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
            failure_injection: None,
        })
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn inject_failure_after_subslab(&mut self, ordinal: usize) {
        self.failure_injection = Some(Stage3V11FailureInjection::SubslabAccepted(ordinal));
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn inject_failure_after_outcome_ledger(&mut self, ordinal: usize) {
        self.failure_injection = Some(Stage3V11FailureInjection::OutcomeLedgerBuilt(ordinal));
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn inject_failure_after_final_owner_join(&mut self) {
        self.failure_injection = Some(Stage3V11FailureInjection::FinalOwnerJoinCompleted);
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) const fn pending_candidate_is_none(&self) -> bool {
        self.pending_candidate.is_none()
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
        let mut terminal_event_groups = Vec::new();
        let mut covered_owner_joins = Vec::new();
        let mut coupled_subslabs = Vec::new();
        for (support_index, support) in prepared.supports().iter().enumerate() {
            let beginning_stage3 = candidate.stage3_by_lane.clone();
            let active_snow_lanes = support.state_derived_active_snow_lanes(&beginning_stage3)?;
            let covered_support = !active_snow_lanes.is_empty();
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
                let (
                    parent,
                    consumer,
                    clock,
                    finalized,
                    ending_stage3,
                    owner_joins,
                    support_event_groups,
                    support_terminal_parcels,
                ) = execute_covered_real_v11_parent(
                    &self.static_context,
                    &beginning_parent,
                    &candidate.real_consumer,
                    &beginning_clock,
                    support,
                    prepared.day_index(),
                    support_index,
                    forcing_receipt,
                    beginning_stage3,
                    self.failure_injection,
                )?;
                for parcel in support_terminal_parcels {
                    if candidate
                        .terminal_parcels
                        .insert(parcel.parcel_digest, parcel)
                        .is_some()
                    {
                        return Err(DirectSnowStage3V11AttachmentError::Terminal(
                            "duplicate terminal parcel identity",
                        ));
                    }
                }
                terminal_event_groups.extend(support_event_groups);
                covered_owner_joins
                    .extend(owner_joins.iter().map(|receipt| receipt.owner_join.clone()));
                coupled_subslabs.extend(owner_joins);
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
        let integrated_boundary_ledger = reconstruct_integrated_boundary_ledger(&coupled_subslabs);
        let receipt = DirectSnowStage3V11ParentReceipt {
            day_index: prepared.day_index(),
            support_count: prepared.supports().len(),
            terminal_events,
            terminal_event_groups,
            ending_stage3_state_digests: stage3_digests,
            complete_owner_bytes,
            covered_owner_joins,
            coupled_subslabs,
            integrated_boundary_ledger,
            ending_coupled_owner_set_sha256: complete_owner_set_digest(
                candidate.coupled_clock.owners(),
            )?,
            ending_coupled_accepted_until_ns: candidate.coupled_clock.accepted_until(),
            ending_next_parent_sequence: candidate.next_parent_sequence,
            ending_event_ordinal: candidate.accepted_event_ordinal,
            ending_terminal_parcels: candidate.terminal_parcels.clone(),
            ending_v11_parent_state: candidate.v11_parent_state.clone(),
            ending_last_v11_parent_candidate: candidate.last_v11_parent_candidate.clone(),
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
        if candidate.ending_state.receipt_chain.len() != self.committed.receipt_chain.len() + 1
            || candidate.ending_state.receipt_chain[..self.committed.receipt_chain.len()]
                != self.committed.receipt_chain
            || candidate.ending_state.receipt_chain.last() != Some(&candidate.parent_receipt)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt chain installation",
            ));
        }
        candidate
            .parent_receipt
            .validate_against_ending(&candidate.ending_state)?;
        let expected_beginning_owner =
            complete_owner_set_digest(self.committed.coupled_clock.owners())?;
        if candidate.parent_receipt.day_index != self.committed.real_consumer.v11_next_day_index()
            || candidate.parent_receipt.support_count as u128
                != candidate
                    .ending_state
                    .next_parent_sequence
                    .checked_sub(self.committed.next_parent_sequence)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "parent sequence installation",
                    ))?
            || candidate.parent_receipt.terminal_events.len() as u64
                != candidate
                    .ending_state
                    .accepted_event_ordinal
                    .checked_sub(self.committed.accepted_event_ordinal)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "event ordinal installation",
                    ))?
            || candidate
                .parent_receipt
                .coupled_subslabs
                .first()
                .is_some_and(|first| {
                    first.owner_join.beginning_complete_owner_set_sha256 != expected_beginning_owner
                        || first.support.start_ns() != self.committed.coupled_clock.accepted_until()
                })
            || candidate
                .parent_receipt
                .coupled_subslabs
                .last()
                .is_some_and(|last| {
                    last.owner_join.ending_complete_owner_set_sha256
                        != candidate.parent_receipt.ending_coupled_owner_set_sha256
                })
            || candidate
                .parent_receipt
                .terminal_events
                .iter()
                .any(|event| {
                    !event.candidate_ticks.contains(&event.accepted_event_tick)
                        || !self.static_context.lane_ids.contains(&event.lane_id)
                })
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt committed-state installation join",
            ));
        }
        self.committed = candidate.ending_state;
        Ok(())
    }

    /// Split one solver-owned terminal-liquid operand over the declared
    /// surface topology exactly once. The parcel remains in the candidate
    /// until the real surface-liquid owner consumes it.
    fn terminal_parcels(
        &self,
        support: TimeSupport,
        lane_id: u32,
        event_ordinal: u64,
        event_receipt_sha256: Digest32,
        terminal_snow_state_sha256: Digest32,
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
        let receiver_topology_sha256 = openwepp_coupled_time::framed_sha256(
            "stage3-terminal-receiver-topology-v1",
            &records
                .iter()
                .map(|record| openwepp_coupled_time::FramedField {
                    tag: "receiver",
                    value: record.key.tile_id.as_str().as_bytes(),
                })
                .collect::<Vec<_>>(),
        )?;
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
                let lane_bytes = lane_id.to_be_bytes();
                let ordinal_bytes = event_ordinal.to_be_bytes();
                let mass_bytes = mass.to_bits().to_be_bytes();
                let digest = openwepp_coupled_time::framed_sha256(
                    "stage3-terminal-parcel-v1",
                    &[
                        openwepp_coupled_time::FramedField {
                            tag: "event_receipt",
                            value: event_receipt_sha256.as_bytes(),
                        },
                        openwepp_coupled_time::FramedField {
                            tag: "terminal_snow_state",
                            value: terminal_snow_state_sha256.as_bytes(),
                        },
                        openwepp_coupled_time::FramedField {
                            tag: "receiver_topology",
                            value: receiver_topology_sha256.as_bytes(),
                        },
                        openwepp_coupled_time::FramedField {
                            tag: "lane",
                            value: &lane_bytes,
                        },
                        openwepp_coupled_time::FramedField {
                            tag: "event_ordinal",
                            value: &ordinal_bytes,
                        },
                        openwepp_coupled_time::FramedField {
                            tag: "ofe",
                            value: record.key.ofe_id.as_str().as_bytes(),
                        },
                        openwepp_coupled_time::FramedField {
                            tag: "tile",
                            value: record.key.tile_id.as_str().as_bytes(),
                        },
                        openwepp_coupled_time::FramedField {
                            tag: "mass_bits",
                            value: &mass_bytes,
                        },
                        openwepp_coupled_time::FramedField {
                            tag: "posture",
                            value: b"ProducedUnconsumed",
                        },
                    ],
                )?;
                Ok(DirectSnowStage3V11TerminalParcel {
                    support,
                    source_lane_id: lane_id,
                    event_ordinal,
                    event_receipt_sha256,
                    terminal_snow_state_sha256,
                    receiver_topology_sha256,
                    destination_ofe_id: record.key.ofe_id.to_string(),
                    destination_tile_id: record.key.tile_id.as_str().to_owned(),
                    mass_kg_m2_tile_ground: mass,
                    temperature_k: 273.15,
                    specific_liquid_enthalpy_j_kg: 0.0,
                    posture: DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed,
                    parcel_digest: digest,
                })
            })
            .collect()
    }

    /// Validate the complete liquid identity carried by the actual terminal
    /// event before constructing any receiver parcel.
    pub fn terminal_parcels_from_event(
        &self,
        support: TimeSupport,
        lane_id: u32,
        event_ordinal: u64,
        terminal_state: &DirectSnowStage3PersistentState,
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
        let event_receipt_sha256 =
            openwepp_coupled_time::digest_bytes(&serde_json::to_vec(event).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("terminal event serialization")
            })?);
        let terminal_snow_state_sha256 = openwepp_coupled_time::digest_bytes(
            &Wb11HydrologyKernel::serialize_stage3_persistent_state(terminal_state).map_err(
                |_| DirectSnowStage3V11AttachmentError::Identity("terminal snow state bytes"),
            )?,
        );
        self.terminal_parcels(
            support,
            lane_id,
            event_ordinal,
            event_receipt_sha256,
            terminal_snow_state_sha256,
            event.terminal_liquid_kg_m2,
        )
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
        || prepared.support.start_ns() < beginning_clock.parent_support().start_ns()
        || prepared.support.end_ns() > beginning_clock.parent_support().end_ns()
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
    let provisional_segment = execute_direct_v11_segment(
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
    let final_segment = execute_direct_v11_segment(
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
    accept_direct_v11_segment(
        &mut parent,
        &context.vegetation_configuration,
        final_segment,
        beginning_consumer,
    )?;
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
pub(crate) fn execute_covered_real_v11_parent(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    failure_injection: Option<Stage3V11FailureInjection>,
) -> Result<
    (
        V11ParentTransaction,
        DirectV10RealConsumerShadow,
        CoupledClockStateV1,
        V11ParentCandidate,
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        Vec<Stage3CoupledSubslabReceiptV1>,
        Vec<Stage3V11TerminalEventGroupV1>,
        Vec<DirectSnowStage3V11TerminalParcel>,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let mut parent = beginning_parent.clone();
    let mut consumer = beginning_consumer.clone();
    let mut clock = beginning_clock.clone();
    let mut stage3 = beginning_stage3;
    let mut owner_joins = Vec::new();
    let mut event_groups = Vec::new();
    let mut terminal_parcels = Vec::new();
    let mut expected_child_beginning = complete_owner_set_digest(beginning_clock.owners())?;
    while clock.accepted_until() < prepared.support.end_ns() {
        let active_lanes = stage3
            .iter()
            .filter_map(|(lane, state)| {
                (stage3_is_resolved_thermal_domain(state)
                    || crate::hydrology::stage3_is_terminal_event_domain(state))
                .then_some(*lane)
            })
            .collect::<BTreeSet<_>>();
        if active_lanes.is_empty() {
            let remainder_support =
                TimeSupport::new(clock.accepted_until(), prepared.support.end_ns())?;
            let successor = prepared
                .coupled_subslab(
                    remainder_support,
                    u32::try_from(owner_joins.len()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity("successor subslab ordinal")
                    })?,
                )?
                .snow_free_successor()?;
            let (next_parent, next_consumer, next_clock, _) = execute_real_v11_parent(
                context,
                &parent,
                &consumer,
                &clock,
                &successor,
                day_index,
                interval_index,
                forcing_receipt,
                canonical_stage3_snow_owner_bytes(&stage3)?,
            )?;
            parent = next_parent;
            consumer = next_consumer;
            clock = next_clock;
            break;
        }
        let selected_seconds = stage3
            .values()
            .filter(|state| active_lanes.contains(&state.lane_id))
            .map(|state| {
                if crate::hydrology::stage3_is_terminal_event_domain(state) {
                    Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(state)
                } else {
                    Wb11HydrologyKernel::project_stage3_surface_state_v1(state)
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|surface| surface.selected_substep_seconds)
            .reduce(f64::min)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "covered subslab requires an active Stage-3 lane",
            ))?;
        let selected_ns = match selected_seconds.to_bits() {
            bits if bits == 1_800.0_f64.to_bits() => 1_800_000_000_000,
            bits if bits == 900.0_f64.to_bits() => 900_000_000_000,
            bits if bits == 60.0_f64.to_bits() => 60_000_000_000,
            _ => {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "unreleased Stage-3 coupled cadence",
                ));
            }
        };
        let proposed_end_ns = ModelTimeNs::new(
            clock
                .accepted_until()
                .get()
                .checked_add(selected_ns)
                .ok_or(DirectSnowStage3V11AttachmentError::Support(
                    "coupled subslab end overflow",
                ))?
                .min(prepared.support.end_ns().get()),
        );
        let end_ns = prepared
            .hard_boundaries
            .iter()
            .copied()
            .find(|boundary| *boundary > clock.accepted_until() && *boundary < proposed_end_ns)
            .unwrap_or(proposed_end_ns);
        let support = TimeSupport::new(clock.accepted_until(), end_ns)?;
        let child_ordinal = u32::try_from(owner_joins.len()).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("coupled subslab ordinal overflow")
        })?;
        let subslab = prepared
            .coupled_subslab(support, child_ordinal)?
            .retain_active_snow_lanes(&active_lanes)?;
        if let Some(actual) = try_actual_terminal_subslab(
            context,
            &parent,
            &consumer,
            &clock,
            &subslab,
            day_index,
            interval_index,
            forcing_receipt,
            &stage3,
            selected_seconds,
            child_ordinal,
            u64::try_from(clock.event_ordinal()).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("terminal event ordinal width")
            })?,
        )? {
            if actual
                .receipt
                .owner_join
                .beginning_complete_owner_set_sha256
                != expected_child_beginning
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal child complete-owner predecessor join",
                ));
            }
            parent = actual.parent;
            consumer = actual.consumer;
            clock = actual.clock;
            stage3 = actual.stage3;
            owner_joins.push(actual.receipt);
            event_groups.push(actual.group);
            terminal_parcels.extend(actual.parcels);
            expected_child_beginning = complete_owner_set_digest(clock.owners())?;
            continue;
        }
        let (next_parent, next_consumer, next_clock, next_stage3, owner_join) =
            execute_covered_real_v11_subslab(
                context,
                &parent,
                &consumer,
                &clock,
                &subslab,
                day_index,
                interval_index,
                forcing_receipt,
                stage3,
                selected_seconds,
                false,
            )?;
        if failure_injection
            == Some(Stage3V11FailureInjection::OutcomeLedgerBuilt(
                owner_joins.len() + 1,
            ))
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "injected post-outcome-ledger rollback",
            ));
        }
        if failure_injection
            == Some(Stage3V11FailureInjection::PrecipitationReceiptRejected(
                owner_joins.len() + 1,
            ))
        {
            return Err(DirectSnowStage3V11AttachmentError::Precipitation(
                "injected live precipitation-receipt rejection",
            ));
        }
        if failure_injection
            == Some(Stage3V11FailureInjection::SnowSoilHeatReceiptRejected(
                owner_joins.len() + 1,
            ))
        {
            return Err(DirectSnowStage3V11AttachmentError::SnowSoilHeat(
                "injected live snow-soil-receipt rejection",
            ));
        }
        if owner_join.owner_join.beginning_complete_owner_set_sha256 != expected_child_beginning {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "covered child complete-owner predecessor join",
            ));
        }
        expected_child_beginning = owner_join.owner_join.ending_complete_owner_set_sha256;
        if complete_owner_set_digest(next_clock.owners())? != expected_child_beginning {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "covered child ending complete-owner clock join",
            ));
        }
        parent = next_parent;
        consumer = next_consumer;
        clock = next_clock;
        stage3 = next_stage3;
        owner_joins.push(owner_join);
        if failure_injection
            == Some(Stage3V11FailureInjection::SubslabAccepted(
                owner_joins.len(),
            ))
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "injected coupled subslab rollback",
            ));
        }
    }
    if failure_injection == Some(Stage3V11FailureInjection::FinalOwnerJoinCompleted) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "injected post-owner-join rollback",
        ));
    }
    let finalized = parent.clone().finalize(&context.vegetation_configuration)?;
    Ok((
        parent,
        consumer,
        clock,
        finalized,
        stage3,
        owner_joins,
        event_groups,
        terminal_parcels,
    ))
}

struct ActualTerminalSubslabV1 {
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    receipt: Stage3CoupledSubslabReceiptV1,
    group: Stage3V11TerminalEventGroupV1,
    parcels: Vec<DirectSnowStage3V11TerminalParcel>,
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn try_actual_terminal_subslab(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    event_ordinal: u64,
) -> Result<Option<ActualTerminalSubslabV1>, DirectSnowStage3V11AttachmentError> {
    let active_lanes = beginning_stage3
        .iter()
        .filter_map(|(lane, state)| {
            (stage3_is_resolved_thermal_domain(state)
                || crate::hydrology::stage3_is_terminal_event_domain(state))
            .then_some(*lane)
        })
        .collect::<BTreeSet<_>>();
    let mut candidate_ticks = BTreeSet::new();
    for lane_id in &active_lanes {
        let state =
            beginning_stage3
                .get(lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal beginning lane",
                ))?;
        let inputs = prepared.snow_inputs_by_lane.get(lane_id).ok_or(
            DirectSnowStage3V11AttachmentError::Identity("terminal input lane"),
        )?;
        let forcing = prepared
            .support_forcing_by_lane
            .get(lane_id)
            .copied()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal forcing lane",
            ))?;
        let result = Wb11HydrologyKernel::evaluate_stage3_persistent_support(
            inputs,
            state,
            *lane_id,
            state.next_interval_index,
            forcing,
            DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
        )?;
        let Some(event) = result.terminal_event else {
            continue;
        };
        for seconds in [
            event.hour_offset_seconds,
            event.event_bracket_lower_seconds,
            event.event_bracket_upper_seconds,
        ] {
            if seconds.is_finite()
                && seconds > 0.0
                && seconds <= f64::from_bits(prepared.support.duration_s_bits())
            {
                let relative = quantize_seconds_to_tick(
                    ModelTimeNs::new(0),
                    ModelTimeNs::new(prepared.support.duration_ns()),
                    seconds,
                )?;
                candidate_ticks.insert(ModelTimeNs::new(
                    prepared.support.start_ns().get() + relative.get(),
                ));
            }
        }
    }
    for tick in candidate_ticks {
        let pre = tick.get() - prepared.support.start_ns().get();
        let post = beginning_clock.parent_support().end_ns().get() - tick.get();
        if (pre != 0 && pre < context.minimum_support_ns)
            || (post != 0 && post < context.minimum_support_ns)
        {
            continue;
        }
        let support = TimeSupport::new(prepared.support.start_ns(), tick)?;
        let projected = prepared.coupled_subslab(support, current_child_ordinal)?;
        let trial = execute_covered_real_v11_subslab(
            context,
            beginning_parent,
            beginning_consumer,
            beginning_clock,
            &projected,
            day_index,
            interval_index,
            forcing_receipt,
            beginning_stage3.clone(),
            selected_upper_bound_s,
            true,
        );
        let Ok((parent, consumer, clock, stage3, receipt)) = trial else {
            continue;
        };
        if receipt.terminal_events.is_empty() {
            continue;
        }
        let candidates = receipt
            .terminal_events
            .iter()
            .map(|(lane_id, event)| {
                let state =
                    stage3
                        .get(lane_id)
                        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                            "terminal ending lane",
                        ))?;
                Ok(Stage3V11ActualTerminalCandidateV1 {
                    lane_id: *lane_id,
                    tick,
                    support,
                    event: *event,
                    terminal_state_sha256: digest_bytes(
                        &Wb11HydrologyKernel::serialize_stage3_persistent_state(state)?,
                    ),
                    shortened_forcing_sha256: canonical_stage3_support_forcing_digest(
                        &projected.support_forcing_by_lane,
                    ),
                    shortened_owner_set_sha256: complete_owner_set_digest(clock.owners())?,
                })
            })
            .collect::<Result<Vec<_>, DirectSnowStage3V11AttachmentError>>()?;
        let Some(group) = select_common_earliest_actual_terminal_group_v1(
            beginning_clock.parent_support(),
            event_ordinal,
            &active_lanes,
            candidates,
        )?
        else {
            continue;
        };
        if group.tick != tick
            || group.terminating_lanes
                != receipt
                    .terminal_events
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
        {
            continue;
        }
        let (parent, clock, stage3, parcels) =
            apply_actual_terminal_group(context, parent, clock, stage3, &group)?;
        return Ok(Some(ActualTerminalSubslabV1 {
            parent,
            consumer,
            clock,
            stage3,
            receipt,
            group,
            parcels,
        }));
    }
    Ok(None)
}

fn apply_actual_terminal_group(
    context: &DirectSnowStage3V11StaticContext,
    mut parent: V11ParentTransaction,
    mut clock: CoupledClockStateV1,
    mut stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    group: &Stage3V11TerminalEventGroupV1,
) -> Result<
    (
        V11ParentTransaction,
        CoupledClockStateV1,
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        Vec<DirectSnowStage3V11TerminalParcel>,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    if clock.accepted_until() != group.tick || u64::from(clock.event_ordinal()) != group.ordinal {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal event cursor or ordinal",
        ));
    }
    let mut parcels = Vec::new();
    for candidate in &group.candidates {
        let terminal =
            stage3
                .get(&candidate.lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal event lane owner",
                ))?;
        parcels.extend(terminal_parcels_for_event_group(
            &context.surface_liquid_configuration,
            candidate,
            group,
        )?);
        let dormant = Wb11HydrologyKernel::consume_stage3_terminal_liquid_v1(
            terminal,
            candidate.event.terminal_liquid_kg_m2,
        )?;
        stage3.insert(candidate.lane_id, dormant);
    }
    let ending_snow_bytes = canonical_stage3_snow_owner_bytes(&stage3)?;
    let ending_owners = clock
        .owners()
        .iter()
        .map(|owner| {
            if owner.owner_id() == "snow" {
                OwnerState::new("snow".to_owned(), ending_snow_bytes.clone())
            } else {
                Ok(owner.clone())
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let beginning_snow = clock
        .owners()
        .iter()
        .find(|owner| owner.owner_id() == "snow")
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal beginning snow owner",
        ))?;
    let ending_snow = ending_owners
        .iter()
        .find(|owner| owner.owner_id() == "snow")
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal ending snow owner",
        ))?;
    let ledger = LedgerEntryV1::new(
        "terminal-snow-liquid-custody".to_owned(),
        "kg-m-2-ofe-ground".to_owned(),
        beginning_snow.state_digest(),
        ending_snow.state_digest(),
        group.receipt_sha256,
    )?;
    let mut participants = clock
        .active_participants()
        .iter()
        .filter(|value| !value.starts_with("stage3-lane-"))
        .cloned()
        .collect::<Vec<_>>();
    participants.extend(
        group
            .post_active_lanes
            .iter()
            .map(|lane| format!("stage3-lane-{lane}")),
    );
    participants.sort();
    participants.dedup();
    let event = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "snow".to_owned(),
        group.receipt_sha256,
        ending_owners.clone(),
        vec!["snow".to_owned()],
        if group.post_active_lanes.is_empty() {
            "snow-free".to_owned()
        } else {
            "snow-stage3-v11-mixed".to_owned()
        },
        participants,
        vec![ledger],
    )?;
    let mut queue = EventQueueV1::new(group.tick, vec![event])?;
    queue
        .apply_next(&mut clock)?
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal event application",
        ))?;
    if queue.apply_next(&mut clock)?.is_some() {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal event queue cardinality",
        ));
    }
    parent.accept_zero_duration_owner_transition(
        &context.vegetation_configuration,
        group.tick,
        owner_envelopes_from_states(&ending_owners)?,
        &["snow".to_owned()],
    )?;
    Ok((parent, clock, stage3, parcels))
}

fn terminal_parcels_for_event_group(
    configuration: &DirectSurfaceLiquidConfiguration,
    candidate: &Stage3V11ActualTerminalCandidateV1,
    group: &Stage3V11TerminalEventGroupV1,
) -> Result<Vec<DirectSnowStage3V11TerminalParcel>, DirectSnowStage3V11AttachmentError> {
    let destination_ofe = configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.production_lane_id == candidate.lane_id)
        .map(|binding| binding.ofe_id.clone())
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal receiver lane binding",
        ))?;
    let records = configuration
        .records
        .iter()
        .filter(|record| record.key.ofe_id == destination_ofe)
        .collect::<Vec<_>>();
    let fraction_sum = records
        .iter()
        .map(|record| record.tile_fraction)
        .sum::<f64>();
    if records.is_empty() || (fraction_sum - 1.0).abs() > 1.0e-12 {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal receiver topology closure",
        ));
    }
    let mut topology_bytes = Vec::new();
    for record in &records {
        topology_bytes.extend_from_slice(record.key.ofe_id.as_str().as_bytes());
        topology_bytes.push(0);
        topology_bytes.extend_from_slice(record.key.tile_id.as_str().as_bytes());
        topology_bytes.extend_from_slice(&record.tile_fraction.to_bits().to_be_bytes());
    }
    let topology = digest_bytes(&topology_bytes);
    records
        .into_iter()
        .map(|record| {
            let lane = candidate.lane_id.to_be_bytes();
            let ordinal = group.ordinal.to_be_bytes();
            let mass = candidate.event.terminal_liquid_kg_m2;
            let mass_bits = mass.to_bits().to_be_bytes();
            let parcel_digest = framed_sha256(
                "stage3-terminal-parcel-v1",
                &[
                    FramedField {
                        tag: "event_receipt",
                        value: group.receipt_sha256.as_bytes(),
                    },
                    FramedField {
                        tag: "terminal_snow_state",
                        value: candidate.terminal_state_sha256.as_bytes(),
                    },
                    FramedField {
                        tag: "receiver_topology",
                        value: topology.as_bytes(),
                    },
                    FramedField {
                        tag: "lane",
                        value: &lane,
                    },
                    FramedField {
                        tag: "event_ordinal",
                        value: &ordinal,
                    },
                    FramedField {
                        tag: "ofe",
                        value: record.key.ofe_id.as_str().as_bytes(),
                    },
                    FramedField {
                        tag: "tile",
                        value: record.key.tile_id.as_str().as_bytes(),
                    },
                    FramedField {
                        tag: "mass_bits",
                        value: &mass_bits,
                    },
                    FramedField {
                        tag: "posture",
                        value: b"ProducedUnconsumed",
                    },
                ],
            )?;
            Ok(DirectSnowStage3V11TerminalParcel {
                support: candidate.support,
                source_lane_id: candidate.lane_id,
                event_ordinal: group.ordinal,
                event_receipt_sha256: group.receipt_sha256,
                terminal_snow_state_sha256: candidate.terminal_state_sha256,
                receiver_topology_sha256: topology,
                destination_ofe_id: record.key.ofe_id.to_string(),
                destination_tile_id: record.key.tile_id.as_str().to_owned(),
                mass_kg_m2_tile_ground: mass,
                temperature_k: 273.15,
                specific_liquid_enthalpy_j_kg: 0.0,
                posture: DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed,
                parcel_digest,
            })
        })
        .collect()
}

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::type_complexity
)]
fn execute_covered_real_v11_subslab(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    selected_upper_bound_s: f64,
    terminal_endpoint_mode: bool,
) -> Result<
    (
        V11ParentTransaction,
        DirectV10RealConsumerShadow,
        CoupledClockStateV1,
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        Stage3CoupledSubslabReceiptV1,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    if beginning_parent.parent_transaction_id() != beginning_clock.parent_transaction_id() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time parent identity",
        ));
    }
    if beginning_clock.accepted_until() != prepared.support.start_ns()
        || prepared.support.start_ns() < beginning_clock.parent_support().start_ns()
        || prepared.support.end_ns() > beginning_clock.parent_support().end_ns()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time subslab support",
        ));
    }
    if beginning_clock.owners().len() != openwepp_vegetation::v11::V11_COMPLETE_OWNER_MANIFEST.len()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time complete owner set",
        ));
    }
    if !prepared.has_snow_surface_forcing() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time snow surface forcing",
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
    let mut provisional_stack = DirectV11SnowCoveredRealConsumerStack::new(
        beginning_consumer,
        DirectV11SnowCoveredStackInputs {
            interval: covered_interval,
            stage3_inputs_by_lane: &prepared.snow_inputs_by_lane,
            stage3_forcing_by_lane: &prepared.support_forcing_by_lane,
            snow_surface_forcing_by_destination: &prepared.snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: beginning_stage3.clone(),
            day_index,
            interval_index,
            finalize_wb14_parent_interval: support.end_ns()
                == beginning_clock.parent_support().end_ns(),
            wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
                coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
                accepted_slab_sha256: *provisional_receipt.slab_id().digest().as_bytes(),
                parent_beginning_complete_owner_set_sha256: *ledger_digest.as_bytes(),
                parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
                parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
                child_support_start_ns: support.start_ns().get() as u128,
                child_support_end_ns: support.end_ns().get() as u128,
            },
        },
    );
    if terminal_endpoint_mode {
        provisional_stack = provisional_stack.with_terminal_endpoint_mode();
    }
    let mut provisional_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
        stack: provisional_stack,
    };
    let provisional_segment = execute_direct_v11_segment(
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
    let mut final_stack = DirectV11SnowCoveredRealConsumerStack::new(
        beginning_consumer,
        DirectV11SnowCoveredStackInputs {
            interval: covered_interval,
            stage3_inputs_by_lane: &prepared.snow_inputs_by_lane,
            stage3_forcing_by_lane: &prepared.support_forcing_by_lane,
            snow_surface_forcing_by_destination: &prepared.snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: beginning_stage3,
            day_index,
            interval_index,
            finalize_wb14_parent_interval: support.end_ns()
                == beginning_clock.parent_support().end_ns(),
            wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
                coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
                accepted_slab_sha256: *final_receipt.slab_id().digest().as_bytes(),
                parent_beginning_complete_owner_set_sha256: *ledger_digest.as_bytes(),
                parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
                parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
                child_support_start_ns: support.start_ns().get() as u128,
                child_support_end_ns: support.end_ns().get() as u128,
            },
        },
    );
    if terminal_endpoint_mode {
        final_stack = final_stack.with_terminal_endpoint_mode();
    }
    let mut final_executor =
        crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack: final_stack };
    let final_segment = execute_direct_v11_segment(
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
    let final_boundary_receipts = final_executor
        .stack
        .last_final_boundary_receipts()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing final covered boundary receipt set",
        ))?
        .clone();
    let final_lane_receipts = final_executor
        .stack
        .last_lane_boundary_receipts()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing final covered lane-boundary receipt set",
        ))?
        .clone();
    let (wb14_child_receipt_set, wb14_parent_receipt_set) = final_executor
        .stack
        .last_wb14_receipt_sets()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing live WB14 receipt-set attachment",
        ))?;
    let wb14_child_receipt_set = parse_lower_hex_digest(wb14_child_receipt_set)?;
    let wb14_parent_receipt_set = wb14_parent_receipt_set
        .map(parse_lower_hex_digest)
        .transpose()?;
    let snow_soil_heat_receipts = final_executor.stack.last_snow_soil_heat_receipts().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing snow-soil heat receipt set"),
    )?;
    let installed_soil: openwepp_land_surface_energy::SoilThermalSnapshot = serde_json::from_slice(
        &final_segment
            .ending_resource_owners
            .get("soil_thermal")
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing installed soil owner",
            ))?
            .state_bytes,
    )
    .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("installed soil owner bytes"))?;
    for (lane_id, receipt) in snow_soil_heat_receipts {
        let state =
            ending_stage3
                .get(lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "snow-soil installed snow lane",
                ))?;
        let inputs = final_executor
            .stack
            .stage3_inputs_by_lane
            .get(lane_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil installed lane inputs",
            ))?;
        let installed_snow_bottom = Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
            state,
            inputs.surface_energy_options.atmospheric_pressure_pa,
        )?;
        let soil_ofe = installed_soil
            .ofes
            .iter()
            .find(|value| value.ofe_id == receipt.ofe_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil installed soil OFE",
            ))?;
        let installed_soil_top =
            soil_ofe
                .ordered_layers
                .first()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "snow-soil installed top soil node",
                ))?;
        let installed_snow_identity = digest_bytes(&serde_json::to_vec(state).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("installed Stage-3 lane identity")
        })?);
        let installed_soil_identity =
            digest_bytes(&serde_json::to_vec(soil_ofe).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("installed soil OFE identity")
            })?);
        let close_temperature = |left: f64, right: f64| {
            left.is_finite() && right.is_finite() && (left - right).abs() <= 1.0e-8
        };
        validate_snow_soil_heat_receipt_installed_join(
            receipt,
            &installed_soil_top.layer_id,
            installed_snow_identity,
            installed_soil_identity,
        )?;
        if !close_temperature(
            installed_snow_bottom.temperature_k,
            receipt.ending_bottom_snow_temperature_k,
        ) || !close_temperature(
            installed_soil_top.temperature_k,
            receipt.ending_top_soil_temperature_k,
        ) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil receipt/installed ending join",
            ));
        }
    }
    let physical_custody_join = CoveredPhysicalCustodyJoinInputs {
        snow_soil_heat_receipts,
        beginning_stage3_states: &final_executor.stack.stage3_beginning_by_lane,
        ending_stage3_states: &ending_stage3,
    };
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
        wb14_child_receipt_set,
        wb14_parent_receipt_set,
        support,
        &final_boundary_receipts,
        &final_lane_receipts,
        final_executor
            .stack
            .last_component_carrier_receipts()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing component-resolved carrier receipt set",
            ))?,
        &physical_custody_join,
        &final_segment.ending_resource_owners,
    )?;
    owner_join.validate(
        &final_boundary_receipts,
        &final_lane_receipts,
        final_executor
            .stack
            .last_component_carrier_receipts()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing component-resolved carrier receipt set",
            ))?,
        &physical_custody_join,
        &final_segment.ending_resource_owners,
    )?;
    let mut parent = beginning_parent.clone();
    accept_direct_v11_segment(
        &mut parent,
        &context.vegetation_configuration,
        final_segment,
        beginning_consumer,
    )?;
    let consumer = final_executor.stack.take_staged_ending().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged covered ending"),
    )?;
    let (wb14_child_replay_bytes, wb14_parent_replay_bytes) =
        final_executor.stack.last_wb14_replay_bytes().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("missing WB14 replay receipt payload"),
        )?;
    let wb14_child_replay_bytes = wb14_child_replay_bytes.to_vec();
    let wb14_parent_replay_bytes = wb14_parent_replay_bytes.map(ToOwned::to_owned);
    let parent_after_segment = parent;
    let mut subslab_receipt = Stage3CoupledSubslabReceiptV1 {
        parent_support: beginning_clock.parent_support(),
        support,
        selected_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
        accepted_slab_sha256: final_receipt.slab_id().digest(),
        wb14_child_receipt_set_sha256: owner_join.wb14_child_receipt_set_sha256,
        wb14_parent_receipt_set_sha256: owner_join.wb14_parent_receipt_set_sha256,
        wb14_child_replay_bytes,
        wb14_parent_replay_bytes,
        destination_receipts: final_boundary_receipts,
        lane_receipts: final_lane_receipts,
        physical_outcome_ledger_set_sha256:
            crate::v9_real_consumer_shadow::stage3_physical_outcome_ledger_set_digest(
                final_executor.stack.last_physical_outcome_ledgers().ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "missing physical outcome ledger set",
                    ),
                )?,
            ),
        terminal_events: final_executor.stack.last_terminal_events().cloned().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("missing terminal event receipt set"),
        )?,
        owner_join,
        receipt_sha256: Digest32::zero(),
    };
    subslab_receipt.receipt_sha256 = subslab_receipt.reconstructed_digest()?;
    subslab_receipt.validate()?;
    Ok((
        parent_after_segment,
        consumer,
        final_clock,
        ending_stage3,
        subslab_receipt,
    ))
}

include!("snow_stage3_v11_attachment_helpers.rs");
#[cfg(test)]
include!("snow_stage3_v11_attachment_tests.rs");
