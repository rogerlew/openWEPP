use std::collections::{BTreeMap, BTreeSet};

use openwepp_coupled_time::{
    AcceptedEventReceiptV1, Digest32, FramedField, digest_bytes, framed_sha256,
};
use openwepp_land_surface_energy::{OfeId, RequestingComponent, SurfaceClass, WaterSourceType};
use openwepp_unit_boundary::TemperatureCelsius;
use openwepp_vegetation::v11::V11ResourceKey;

use crate::hydrology::{
    DirectSnowLiquidDispositionLedger, DirectSnowMassTransitionLedgers,
    DirectSnowSolidToLiquidLedger, DirectSnowStage3Outcome, DirectSnowStage3PersistentState,
    Wb11HydrologyKernel,
};
use crate::v9_real_consumer_shadow::{
    Stage3AcceptedBeginningLaneCarryV1, Stage3AcceptedPublicationSupportV1,
};
use crate::winter_column::DirectSnowLaneState;

use super::*;

const KG_M2_PER_M_WATER: f64 = 1_000.0;
const DAY_NS: u128 = 86_400_000_000_000;
const ACCEPTED_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;

include!("stage3_committed_support_liquid.rs");
include!("stage3_committed_publication_wat5.rs");

/// Runner-owned, parsed-soil publication authority retained across the
/// adaptive day commit. The four WB13 operands are static soil symbols and the
/// aggregate-storage tolerance is an immutable projection operand; none is
/// accepted-support physics. The attachment binds them to the exact lane soil
/// configuration and carries them into its committed day frame.
#[derive(Clone, Debug, PartialEq)]
struct Stage3PublicationProfileAuthorityV1 {
    day_index: usize,
    lane_index: usize,
    lane_id: u32,
    profile_depth_m: f64,
    profile_porosity_cap_m: f64,
    profile_field_capacity_m: f64,
    profile_wilting_point_m: f64,
    aggregate_storage_tolerance_m: f64,
    soil_configuration_sha256: Digest32,
    receipt_sha256: Digest32,
}

impl Stage3PublicationProfileAuthorityV1 {
    fn try_from_day_input(
        day_index: usize,
        lane_index: usize,
        lane: &DirectLaneFrame,
        input: &DirectPublicationDayInput,
    ) -> Result<Self, DirectRuntimeError> {
        let projection = input
            .hydrology_projection_inputs
            .ok_or(stage3_publication_guard(
                "missing parsed-soil WB13 profile authority",
            ))?;
        let profile_depth_m = projection.profile_depth_m.ok_or(stage3_publication_guard(
            "incomplete parsed-soil WB13 profile authority",
        ))?;
        let profile_porosity_cap_m =
            projection
                .profile_porosity_cap_m
                .ok_or(stage3_publication_guard(
                    "incomplete parsed-soil WB13 profile authority",
                ))?;
        let profile_field_capacity_m =
            projection
                .profile_field_capacity_m
                .ok_or(stage3_publication_guard(
                    "incomplete parsed-soil WB13 profile authority",
                ))?;
        let profile_wilting_point_m =
            projection
                .profile_wilting_point_m
                .ok_or(stage3_publication_guard(
                    "incomplete parsed-soil WB13 profile authority",
                ))?;
        let aggregate_storage_tolerance_m = projection.aggregate_storage_tolerance_m;
        if !profile_depth_m.is_finite()
            || profile_depth_m <= 0.0
            || !profile_porosity_cap_m.is_finite()
            || !profile_field_capacity_m.is_finite()
            || !profile_wilting_point_m.is_finite()
            || profile_wilting_point_m < 0.0
            || profile_field_capacity_m < profile_wilting_point_m
            || profile_porosity_cap_m < profile_field_capacity_m
            || !aggregate_storage_tolerance_m.is_finite()
            || aggregate_storage_tolerance_m < 0.0
        {
            return Err(stage3_publication_guard(
                "parsed-soil WB13 profile authority domain/order",
            ));
        }
        let subsurface =
            input
                .subsurface_compute_inputs
                .as_ref()
                .ok_or(stage3_publication_guard(
                    "missing parsed-soil WB13 lane configuration",
                ))?;
        let soil_configuration_sha256 = static_soil_configuration_sha256_from_inputs(subsurface)?;
        if soil_configuration_sha256 != static_soil_configuration_sha256_from_lane(lane)? {
            return Err(stage3_publication_guard(
                "parsed-soil WB13 profile/lane configuration cross-join",
            ));
        }
        let mut value = Self {
            day_index,
            lane_index,
            lane_id: lane.lane_id,
            profile_depth_m,
            profile_porosity_cap_m,
            profile_field_capacity_m,
            profile_wilting_point_m,
            aggregate_storage_tolerance_m,
            soil_configuration_sha256,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.compute_receipt()?;
        value.validate(lane)?;
        Ok(value)
    }

    fn validate(&self, lane: &DirectLaneFrame) -> Result<(), DirectRuntimeError> {
        if self.lane_id != lane.lane_id
            || self.receipt_sha256 == Digest32::zero()
            || !self.aggregate_storage_tolerance_m.is_finite()
            || self.aggregate_storage_tolerance_m < 0.0
            || self.soil_configuration_sha256 != static_soil_configuration_sha256_from_lane(lane)?
            || self.compute_receipt()? != self.receipt_sha256
        {
            return Err(stage3_publication_guard(
                "parsed-soil WB13 profile authority seal/identity",
            ));
        }
        Ok(())
    }

    fn install(
        &self,
        day: &mut DirectDayFrame,
        lane: &DirectLaneFrame,
    ) -> Result<(), DirectRuntimeError> {
        self.validate(lane)?;
        if day.day_index != self.day_index
            || day.lane_index != self.lane_index
            || day.hydrology_projection_inputs.profile_depth_m.is_some()
            || day
                .hydrology_projection_inputs
                .profile_porosity_cap_m
                .is_some()
            || day
                .hydrology_projection_inputs
                .profile_field_capacity_m
                .is_some()
            || day
                .hydrology_projection_inputs
                .profile_wilting_point_m
                .is_some()
            || day
                .hydrology_projection_inputs
                .aggregate_storage_tolerance_m
                .to_bits()
                != 0.0_f64.to_bits()
        {
            return Err(stage3_publication_guard(
                "parsed-soil WB13 profile installation identity",
            ));
        }
        day.hydrology_projection_inputs.profile_depth_m = Some(self.profile_depth_m);
        day.hydrology_projection_inputs.profile_porosity_cap_m = Some(self.profile_porosity_cap_m);
        day.hydrology_projection_inputs.profile_field_capacity_m =
            Some(self.profile_field_capacity_m);
        day.hydrology_projection_inputs.profile_wilting_point_m =
            Some(self.profile_wilting_point_m);
        day.hydrology_projection_inputs
            .aggregate_storage_tolerance_m = self.aggregate_storage_tolerance_m;
        Ok(())
    }

    fn compute_receipt(&self) -> Result<Digest32, DirectRuntimeError> {
        framed_sha256(
            "stage3-v11-publication-profile-authority-v1",
            &[
                FramedField {
                    tag: "day_index",
                    value: &u128::try_from(self.day_index)
                        .map_err(|_| stage3_publication_guard("profile day index width"))?
                        .to_be_bytes(),
                },
                FramedField {
                    tag: "lane_index",
                    value: &u128::try_from(self.lane_index)
                        .map_err(|_| stage3_publication_guard("profile lane index width"))?
                        .to_be_bytes(),
                },
                FramedField {
                    tag: "lane_id",
                    value: &self.lane_id.to_be_bytes(),
                },
                FramedField {
                    tag: "profile_depth_m",
                    value: &self.profile_depth_m.to_bits().to_be_bytes(),
                },
                FramedField {
                    tag: "profile_porosity_cap_m",
                    value: &self.profile_porosity_cap_m.to_bits().to_be_bytes(),
                },
                FramedField {
                    tag: "profile_field_capacity_m",
                    value: &self.profile_field_capacity_m.to_bits().to_be_bytes(),
                },
                FramedField {
                    tag: "profile_wilting_point_m",
                    value: &self.profile_wilting_point_m.to_bits().to_be_bytes(),
                },
                FramedField {
                    tag: "aggregate_storage_tolerance_m",
                    value: &self.aggregate_storage_tolerance_m.to_bits().to_be_bytes(),
                },
                FramedField {
                    tag: "soil_configuration",
                    value: self.soil_configuration_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| stage3_publication_guard("parsed-soil WB13 profile authority seal"))
    }
}

fn static_soil_configuration_sha256_from_inputs(
    inputs: &DirectSubsurfaceComputeInputs,
) -> Result<Digest32, DirectRuntimeError> {
    static_soil_configuration_sha256(inputs.layers.iter().map(|layer| {
        (
            layer.depth_m,
            layer.conductivity_m_s,
            layer.residual_theta,
            layer.porosity,
            layer.field_capacity_theta,
            layer.coca,
            layer.lateral_conductivity_m_s,
        )
    }))
}

fn static_soil_configuration_sha256_from_lane(
    lane: &DirectLaneFrame,
) -> Result<Digest32, DirectRuntimeError> {
    static_soil_configuration_sha256(lane.subsurface_layers.iter().map(|layer| {
        (
            layer.depth_m,
            layer.conductivity_m_s,
            layer.residual_theta,
            layer.porosity,
            layer.field_capacity_theta,
            layer.coca,
            layer.lateral_conductivity_m_s,
        )
    }))
}

fn static_soil_configuration_sha256(
    layers: impl Iterator<Item = (f64, f64, f64, f64, f64, f64, f64)>,
) -> Result<Digest32, DirectRuntimeError> {
    let mut bytes = Vec::new();
    let mut count = 0_u64;
    for layer in layers {
        for value in [
            layer.0, layer.1, layer.2, layer.3, layer.4, layer.5, layer.6,
        ] {
            if !value.is_finite() {
                return Err(stage3_publication_guard(
                    "parsed-soil WB13 lane configuration domain",
                ));
            }
            bytes.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        count = count.checked_add(1).ok_or(stage3_publication_guard(
            "parsed-soil WB13 lane configuration cardinality",
        ))?;
    }
    if count == 0 {
        return Err(stage3_publication_guard(
            "empty parsed-soil WB13 lane configuration",
        ));
    }
    bytes.extend_from_slice(&count.to_be_bytes());
    Ok(digest_bytes(&bytes))
}

/// Sealed, accepted-only V11 publication capability for one complete day.
///
/// The contained frames have already consumed every retained accepted support
/// and executed the downstream-only daily owners.  They are immutable after
/// installation and are the sole Stage-3 publication source.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stage3AcceptedPublicationDayV1 {
    day_index: usize,
    beginning_complete_owner_set_sha256: Digest32,
    ending_complete_owner_set_sha256: Digest32,
    ordered_support_receipt_set_sha256: Digest32,
    lane_frames: Vec<DirectDayFrame>,
    stage3_surface_temperature_c_by_lane: Vec<Option<f64>>,
    receipt_sha256: Digest32,
}

impl Stage3AcceptedPublicationDayV1 {
    #[must_use]
    pub(crate) const fn day_index(&self) -> usize {
        self.day_index
    }

    #[must_use]
    pub(crate) fn lane_frames(&self) -> &[DirectDayFrame] {
        &self.lane_frames
    }

    #[must_use]
    pub(crate) const fn beginning_complete_owner_set_sha256(&self) -> Digest32 {
        self.beginning_complete_owner_set_sha256
    }

    #[must_use]
    pub(crate) const fn ending_complete_owner_set_sha256(&self) -> Digest32 {
        self.ending_complete_owner_set_sha256
    }

    #[must_use]
    pub(crate) const fn ordered_support_receipt_set_sha256(&self) -> Digest32 {
        self.ordered_support_receipt_set_sha256
    }

    #[must_use]
    pub(crate) const fn receipt_sha256(&self) -> Digest32 {
        self.receipt_sha256
    }

    pub(crate) fn validate_publication_exogenous_input(
        &self,
        lane_index: usize,
        day_input: &DirectPublicationDayInput,
    ) -> Result<(), DirectRuntimeError> {
        let frame =
            self.lane_frames
                .get(lane_index)
                .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                    lane_index,
                    lane_count: self.lane_frames.len(),
                })?;
        validate_nonnegative_direct_m(
            "stage3_publication.day_input_precipitation_m",
            day_input.precipitation_m,
        )?;
        validate_finite(
            "stage3_publication.day_input_effective_temperature_c",
            day_input.effective_temperature_c,
        )?;
        validate_publication_exogenous_climate_identity(
            lane_index,
            frame.normalization.precipitation_m,
            day_input,
        )
    }

    pub(crate) fn validate_for_install(
        &self,
        expected_day_index: usize,
        expected_lane_count: usize,
        expected_ending_owner_set_sha256: Digest32,
    ) -> Result<(), DirectRuntimeError> {
        if self.day_index != expected_day_index
            || self.lane_frames.len() != expected_lane_count
            || self.stage3_surface_temperature_c_by_lane.len() != expected_lane_count
            || self.beginning_complete_owner_set_sha256 == Digest32::zero()
            || self.ending_complete_owner_set_sha256 != expected_ending_owner_set_sha256
            || self.ordered_support_receipt_set_sha256 == Digest32::zero()
            || self.receipt_sha256 == Digest32::zero()
            || self
                .lane_frames
                .iter()
                .enumerate()
                .any(|(lane_index, frame)| {
                    frame.day_index != expected_day_index || frame.lane_index != lane_index
                })
            || self
                .stage3_surface_temperature_c_by_lane
                .iter()
                .flatten()
                .any(|temperature| !temperature.is_finite() || *temperature > 0.0)
        {
            return Err(stage3_publication_guard(
                "installed committed-day capability identity",
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(crate) fn try_complete(
        frame: &mut DirectRunFrame,
        day_index: usize,
        publication_inputs: &[DirectPublicationDayInput],
        supports: &[Stage3AcceptedPublicationSupportV1],
        event_handoffs: &[AcceptedEventReceiptV1],
        terminal_event_groups: &[crate::snow_stage3_v11_attachment::Stage3V11TerminalEventGroupV1],
        coupled_subslabs: &[crate::snow_stage3_v11_attachment::Stage3CoupledSubslabReceiptV1],
        beginning_stage3_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        ending_stage3_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        surface_configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<Self, DirectRuntimeError> {
        let (beginning_owner, ending_owner, receipt_set) =
            validate_complete_support_day(frame, day_index, supports, event_handoffs)?;
        if publication_inputs.len() != frame.identity.lane_count
            || beginning_stage3_by_lane.len() != frame.identity.lane_count
            || ending_stage3_by_lane.len() != frame.identity.lane_count
            || surface_configuration.ofe_bindings.len() != frame.identity.lane_count
        {
            return Err(stage3_publication_guard(
                "committed-day Stage-3/surface lane cardinality",
            ));
        }

        let first_support = supports
            .first()
            .ok_or(stage3_publication_guard("missing accepted support"))?;
        let last_support = supports
            .last()
            .ok_or(stage3_publication_guard("missing accepted support"))?;
        if first_support.run_identity() != frame.identity {
            return Err(stage3_publication_guard(
                "committed-day beginning frame identity",
            ));
        }

        let mut lane_frames = Vec::with_capacity(frame.identity.lane_count);
        let mut surface_temperatures = Vec::with_capacity(frame.identity.lane_count);
        for lane_index in 0..frame.identity.lane_count {
            let lane =
                frame
                    .lanes
                    .get(lane_index)
                    .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                        lane_index,
                        lane_count: frame.lanes.len(),
                    })?;
            let publication_input = publication_inputs.get(lane_index).ok_or(
                DirectRuntimeError::LaneIndexOutOfRange {
                    lane_index,
                    lane_count: publication_inputs.len(),
                },
            )?;
            let profile_authority = Stage3PublicationProfileAuthorityV1::try_from_day_input(
                day_index,
                lane_index,
                lane,
                publication_input,
            )?;
            let binding = surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| {
                    binding.production_lane_index == lane_index
                        && binding.production_lane_id == lane.lane_id
                })
                .ok_or(stage3_publication_guard("committed-day OFE/lane binding"))?;
            let beginning_layers = first_support
                .beginning_subsurface_layers(lane_index)
                .ok_or(stage3_publication_guard(
                    "missing accepted beginning subsurface lane",
                ))?;
            let beginning_carry = first_support
                .beginning_lane_carries()
                .iter()
                .find(|carry| carry.lane_id == lane.lane_id)
                .ok_or(stage3_publication_guard(
                    "missing accepted beginning lane carry",
                ))?;
            let accepted_ending_layers = last_support.ending_subsurface_layers(lane_index).ok_or(
                stage3_publication_guard("missing accepted ending subsurface lane"),
            )?;
            validate_accepted_ending_layer_identity(
                accepted_ending_layers,
                &lane.subsurface_layers,
            )?;
            let accepted = aggregate_accepted_lane_day(
                supports,
                event_handoffs,
                terminal_event_groups,
                coupled_subslabs,
                binding,
                surface_configuration,
            )?;
            let beginning_soil_m = aggregate_direct_soil_water(
                beginning_layers,
                "stage3_publication.beginning_soil_water_m",
            )?;
            let accepted_ending_soil_m = accepted_ending_soil_water_m(accepted_ending_layers)?;

            let mut day = frame.seed_day_frame(lane_index, day_index)?;
            profile_authority.install(&mut day, lane)?;
            seed_accepted_upstream_day(
                &mut day,
                beginning_soil_m,
                accepted_ending_soil_m,
                accepted_ending_layers,
                &accepted,
                beginning_carry,
                lane,
            )?;
            run_downstream_only_hydrology(&mut day, &accepted, lane.plant_water_stress)?;

            let ending_stage3 = ending_stage3_by_lane
                .get(&lane.lane_id)
                .ok_or(stage3_publication_guard("missing ending Stage-3 lane"))?;
            let beginning_stage3 = beginning_stage3_by_lane
                .get(&lane.lane_id)
                .ok_or(stage3_publication_guard("missing beginning Stage-3 lane"))?;
            let surface_temperature =
                install_stage3_projection(&mut day, beginning_stage3, ending_stage3, &accepted)?;
            day.run_r4l_saturation_addback_span()?;
            day.run_r4a_runoff_partition_span()?;
            day.run_r7d6_peak_runoff_span()?;
            install_requested_accepted_wat5_source(
                &mut day,
                publication_input,
                supports,
                &accepted,
                binding,
                surface_configuration,
            )?;
            day.run_wat5_subhourly_generation()?;
            finish_storage_and_projection(&mut day, beginning_soil_m, &accepted)?;
            lane_frames.push(day);
            surface_temperatures.push(surface_temperature);
        }

        frame.run_groundwater_day_from_lane_frames(day_index, &mut lane_frames)?;
        for lane_index in 0..lane_frames.len() {
            if lane_index > 0 {
                lane_frames[lane_index].erosion_inflow_intake =
                    frame.lanes[lane_index].erosion_inflow_intake.clone();
            }
            DirectFrameExecutor::run_day_spans_erosion_and_ledger(
                &mut lane_frames[lane_index],
                &mut DirectExecutionCounters::default(),
            )?;
            DirectFrameExecutor::publish_dynamic_transfer_to_downstream_with_ownership(
                frame,
                &lane_frames[lane_index],
                true,
            )?;
            DirectFrameExecutor::publish_erosion_inflow_to_downstream(
                frame,
                &lane_frames[lane_index],
            )?;
        }
        for day in &lane_frames {
            commit_stage3_downstream_day(frame, day)?;
        }

        let receipt_sha256 = committed_day_receipt(
            day_index,
            beginning_owner,
            ending_owner,
            receipt_set,
            &lane_frames,
            &surface_temperatures,
        )?;
        let value = Self {
            day_index,
            beginning_complete_owner_set_sha256: beginning_owner,
            ending_complete_owner_set_sha256: ending_owner,
            ordered_support_receipt_set_sha256: receipt_set,
            lane_frames,
            stage3_surface_temperature_c_by_lane: surface_temperatures,
            receipt_sha256,
        };
        value.validate_for_install(day_index, frame.identity.lane_count, ending_owner)?;
        Ok(value)
    }
}

fn validate_publication_exogenous_climate_identity(
    lane_index: usize,
    accepted_precipitation_m: f64,
    day_input: &DirectPublicationDayInput,
) -> Result<(), DirectRuntimeError> {
    // Accepted support air temperature is integrated over the exact adaptive
    // microsteps. `day_input.effective_temperature_c` is the distinct CLIGEN
    // daily midpoint `(tmax + tmin) / 2`; the SIMIMPL28 hourly curve does not
    // in general have that exact discrete mean. Publication never consumes
    // the runner scalar. Its sole temperature source remains the sealed
    // accepted-support frame, while the independently reconstructible daily
    // precipitation must still match the exogenous climate input.
    let residual_m = accepted_precipitation_m - day_input.precipitation_m;
    if residual_m.abs() > ACCEPTED_CLOSURE_TOLERANCE_M {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "stage3_committed_publication",
            detail: format!(
                "accepted support/day-input precipitation identity: lane={lane_index} accepted_m={accepted_precipitation_m:?} day_input_m={:?} residual_m={residual_m:?} tolerance_m={ACCEPTED_CLOSURE_TOLERANCE_M:?}",
                day_input.precipitation_m,
            ),
        });
    }
    Ok(())
}

#[derive(Clone, Debug, Default)]
struct AcceptedLaneDay {
    ingress_m: f64,
    precipitation_m: f64,
    local_liquid_m: f64,
    runon_m: f64,
    infiltration_m: f64,
    retained_surface_liquid_m: f64,
    runoff_m: f64,
    support_liquid_runon_m: f64,
    soil_evaporation_m: f64,
    residue_evaporation_m: f64,
    plant_transpiration_m: f64,
    soil_storage_et_m: f64,
    effective_temperature_c: f64,
    root_request_by_layer_m: Vec<f64>,
    root_use_by_layer_m: Vec<f64>,
    snow_terminal_liquid_m: f64,
    snow_refreeze_m: f64,
    /// Mass-weighted temperature of the exact terminal-liquid custody stream.
    ///
    /// This is a bulk parcel property, including externally supplied liquid
    /// sensible enthalpy. It is not the legacy WB11 meltwater phase-reference
    /// temperature published by `DirectSnowStage3Outcome`.
    snow_terminal_custody_temperature_k_mass: f64,
    hourly_snow_terminal_liquid_m: [f64; 24],
    hourly_runoff_m: [f64; 24],
}

impl AcceptedLaneDay {
    fn evapotranspiration_m(&self) -> f64 {
        self.soil_evaporation_m + self.residue_evaporation_m + self.plant_transpiration_m
    }
}

fn validate_complete_support_day(
    frame: &DirectRunFrame,
    day_index: usize,
    supports: &[Stage3AcceptedPublicationSupportV1],
    event_handoffs: &[AcceptedEventReceiptV1],
) -> Result<(Digest32, Digest32, Digest32), DirectRuntimeError> {
    let day_start = u128::try_from(day_index)
        .map_err(|_| stage3_publication_guard("day index width"))?
        .checked_mul(DAY_NS)
        .ok_or(stage3_publication_guard("day support start overflow"))?;
    let day_end = day_start
        .checked_add(DAY_NS)
        .ok_or(stage3_publication_guard("day support end overflow"))?;
    let headers = supports
        .iter()
        .map(|support| AcceptedSupportHeader {
            day_index: support.day_index(),
            interval_index: support.interval_index(),
            parent_transaction_sha256: support.parent_transaction_id().digest(),
            support_start_ns: support.support().start_ns().get(),
            support_end_ns: support.support().end_ns().get(),
            accepted_slab_sha256: support.accepted_slab_sha256(),
            beginning_complete_owner_set_sha256: support.beginning_complete_owner_set_sha256(),
            ending_complete_owner_set_sha256: support.ending_complete_owner_set_sha256(),
            receipt_sha256: support.receipt_sha256(),
            run_identity: support.run_identity(),
            accepted_infiltration_is_installed: support.accepted_infiltration_is_installed(),
        })
        .collect::<Vec<_>>();
    let event_headers = event_handoffs
        .iter()
        .map(|event| AcceptedEventHandoffHeader {
            receipt_id_sha256: event.id().digest(),
            parent_transaction_sha256: event.parent_transaction_id().digest(),
            tick_ns: event.tick().get(),
            ordinal: event.ordinal(),
            beginning_complete_owner_set_sha256: event.beginning_owner_set_digest(),
            ending_complete_owner_set_sha256: event.ending_owner_set_digest(),
            seal_is_valid: event.validate().is_ok(),
        })
        .collect::<Vec<_>>();
    let (beginning_owner, ending_owner) = validate_complete_support_headers(
        day_index,
        day_start,
        day_end,
        frame.identity,
        &headers,
        &event_headers,
    )?;
    let mut bytes = Vec::with_capacity((supports.len() + event_handoffs.len()) * 33 + 64);
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_ACCEPTED_PUBLICATION_SUPPORT_SET_V1\0");
    for support in supports {
        bytes.push(b'S');
        bytes.extend_from_slice(support.receipt_sha256().as_bytes());
    }
    for event in event_handoffs {
        bytes.push(b'E');
        bytes.extend_from_slice(event.id().digest().as_bytes());
    }
    Ok((beginning_owner, ending_owner, digest_bytes(&bytes)))
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct AcceptedSupportHeader {
    day_index: usize,
    interval_index: usize,
    parent_transaction_sha256: Digest32,
    support_start_ns: u128,
    support_end_ns: u128,
    accepted_slab_sha256: Digest32,
    beginning_complete_owner_set_sha256: Digest32,
    ending_complete_owner_set_sha256: Digest32,
    receipt_sha256: Digest32,
    run_identity: DirectRunIdentity,
    accepted_infiltration_is_installed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct AcceptedEventHandoffHeader {
    receipt_id_sha256: Digest32,
    parent_transaction_sha256: Digest32,
    tick_ns: u128,
    ordinal: u32,
    beginning_complete_owner_set_sha256: Digest32,
    ending_complete_owner_set_sha256: Digest32,
    seal_is_valid: bool,
}

fn validate_complete_support_headers(
    day_index: usize,
    day_start_ns: u128,
    day_end_ns: u128,
    run_identity: DirectRunIdentity,
    headers: &[AcceptedSupportHeader],
    event_handoffs: &[AcceptedEventHandoffHeader],
) -> Result<(Digest32, Digest32), DirectRuntimeError> {
    let first = headers
        .first()
        .ok_or(stage3_publication_guard("missing accepted day supports"))?;
    let last = headers
        .last()
        .ok_or(stage3_publication_guard("missing accepted day supports"))?;
    let mut receipts = BTreeSet::new();
    let mut slabs = BTreeSet::new();
    let mut parent_transactions = BTreeSet::new();
    let parent_count = crate::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_COUNT;
    let parent_ns = crate::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_NS;
    let minimum_ns = crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
    if first.support_start_ns != day_start_ns || last.support_end_ns != day_end_ns {
        return Err(stage3_publication_guard(
            "accepted day support completeness/chronology",
        ));
    }
    let mut cursor = 0usize;
    for expected_interval in 0..parent_count {
        let group_start = cursor;
        while cursor < headers.len() && headers[cursor].interval_index == expected_interval {
            cursor += 1;
        }
        if group_start == cursor {
            return Err(stage3_publication_guard(
                "accepted day support completeness/chronology",
            ));
        }
        let group = &headers[group_start..cursor];
        let expected_parent_offset =
            (expected_interval as u128)
                .checked_mul(parent_ns)
                .ok_or(stage3_publication_guard(
                    "accepted parent support offset overflow",
                ))?;
        let expected_start =
            day_start_ns
                .checked_add(expected_parent_offset)
                .ok_or(stage3_publication_guard(
                    "accepted parent support start overflow",
                ))?;
        let expected_end =
            expected_start
                .checked_add(parent_ns)
                .ok_or(stage3_publication_guard(
                    "accepted parent support end overflow",
                ))?;
        let parent_transaction = group[0].parent_transaction_sha256;
        if group[0].support_start_ns != expected_start
            || group[group.len() - 1].support_end_ns != expected_end
            || parent_transaction == Digest32::zero()
            || !parent_transactions.insert(parent_transaction)
            || group.iter().any(|header| {
                let duration = header.support_end_ns.checked_sub(header.support_start_ns);
                header.day_index != day_index
                    || header.interval_index != expected_interval
                    || header.parent_transaction_sha256 != parent_transaction
                    || header.run_identity != run_identity
                    || !header.accepted_infiltration_is_installed
                    || header.accepted_slab_sha256 == Digest32::zero()
                    || header.receipt_sha256 == Digest32::zero()
                    || !slabs.insert(header.accepted_slab_sha256)
                    || !receipts.insert(header.receipt_sha256)
                    || header.beginning_complete_owner_set_sha256 == Digest32::zero()
                    || header.ending_complete_owner_set_sha256 == Digest32::zero()
                    || duration
                        .is_none_or(|duration| duration < minimum_ns || duration % minimum_ns != 0)
                    || header
                        .support_start_ns
                        .checked_sub(day_start_ns)
                        .is_none_or(|offset| offset % minimum_ns != 0)
                    || header
                        .support_end_ns
                        .checked_sub(day_start_ns)
                        .is_none_or(|offset| offset % minimum_ns != 0)
            })
        {
            return Err(stage3_publication_guard(
                "accepted day support completeness/chronology",
            ));
        }
    }
    if cursor != headers.len()
        || headers
            .windows(2)
            .any(|pair| pair[0].support_end_ns != pair[1].support_start_ns)
    {
        return Err(stage3_publication_guard(
            "accepted day support completeness/chronology",
        ));
    }

    let mut event_index = 0usize;
    let mut event_ids = BTreeSet::new();
    let mut last_event_ordinal_by_parent = BTreeMap::new();
    let mut beginning_owner = first.beginning_complete_owner_set_sha256;
    let mut traversed_ending_owner = None;
    while let Some(event) = event_handoffs
        .get(event_index)
        .filter(|event| event.tick_ns == first.support_start_ns)
    {
        let expected_beginning =
            traversed_ending_owner.unwrap_or(event.beginning_complete_owner_set_sha256);
        validate_event_handoff_header(
            event,
            first.parent_transaction_sha256,
            expected_beginning,
            &mut event_ids,
            &mut last_event_ordinal_by_parent,
        )?;
        if event_index == 0 {
            beginning_owner = event.beginning_complete_owner_set_sha256;
        }
        traversed_ending_owner = Some(event.ending_complete_owner_set_sha256);
        event_index += 1;
    }
    if traversed_ending_owner
        .is_some_and(|ending| ending != first.beginning_complete_owner_set_sha256)
    {
        return Err(stage3_publication_guard(
            "accepted publication genesis event/support handoff",
        ));
    }

    for (support_index, support) in headers.iter().enumerate() {
        if traversed_ending_owner
            .is_some_and(|ending| ending != support.beginning_complete_owner_set_sha256)
        {
            return Err(stage3_publication_guard(
                "accepted publication support/event owner adjacency",
            ));
        }
        let mut ending_owner = support.ending_complete_owner_set_sha256;
        let mut crossed_into_next_parent = false;
        while let Some(event) = event_handoffs
            .get(event_index)
            .filter(|event| event.tick_ns == support.support_end_ns)
        {
            let expected_parent =
                if event.parent_transaction_sha256 == support.parent_transaction_sha256 {
                    if crossed_into_next_parent {
                        return Err(stage3_publication_guard(
                            "accepted publication event parent ordering",
                        ));
                    }
                    support.parent_transaction_sha256
                } else {
                    let next = headers
                        .get(support_index + 1)
                        .ok_or(stage3_publication_guard(
                            "orphan accepted publication pre-support event",
                        ))?;
                    if event.parent_transaction_sha256 != next.parent_transaction_sha256
                        || event.tick_ns != next.support_start_ns
                    {
                        return Err(stage3_publication_guard(
                            "accepted publication cross-parent event handoff",
                        ));
                    }
                    crossed_into_next_parent = true;
                    next.parent_transaction_sha256
                };
            validate_event_handoff_header(
                event,
                expected_parent,
                ending_owner,
                &mut event_ids,
                &mut last_event_ordinal_by_parent,
            )?;
            ending_owner = event.ending_complete_owner_set_sha256;
            event_index += 1;
        }
        traversed_ending_owner = Some(ending_owner);
    }
    if event_index != event_handoffs.len() {
        return Err(stage3_publication_guard(
            "orphan or out-of-order accepted publication event handoff",
        ));
    }
    Ok((
        beginning_owner,
        traversed_ending_owner.unwrap_or(last.ending_complete_owner_set_sha256),
    ))
}

include!("stage3_committed_publication_event_helpers.rs");

fn aggregate_accepted_lane_day(
    supports: &[Stage3AcceptedPublicationSupportV1],
    event_handoffs: &[AcceptedEventReceiptV1],
    terminal_event_groups: &[crate::snow_stage3_v11_attachment::Stage3V11TerminalEventGroupV1],
    coupled_subslabs: &[crate::snow_stage3_v11_attachment::Stage3CoupledSubslabReceiptV1],
    binding: &DirectSurfaceLiquidOfeBinding,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<AcceptedLaneDay, DirectRuntimeError> {
    let ofe_id = &binding.ofe_id;
    let mut out = AcceptedLaneDay {
        root_request_by_layer_m: vec![0.0; binding.ordered_soil_layer_ids.len()],
        root_use_by_layer_m: vec![0.0; binding.ordered_soil_layer_ids.len()],
        ..AcceptedLaneDay::default()
    };
    let mut temperature_k_seconds = 0.0;
    let mut total_seconds = 0.0;
    let mut accepted_snow_liquid_receipts = BTreeSet::new();
    let day_start_ns = supports
        .first()
        .ok_or(stage3_publication_guard("missing accepted support"))?
        .support()
        .start_ns()
        .get();
    for support in supports {
        let ledger = support
            .ingress_ledgers()
            .iter()
            .find(|ledger| &ledger.ofe_id == ofe_id)
            .ok_or(stage3_publication_guard(
                "accepted support OFE ingress ledger",
            ))?;
        add_nonnegative(
            &mut out.ingress_m,
            ledger.ingress_mass_kg_m2_ofe_ground / KG_M2_PER_M_WATER,
        )?;
        add_nonnegative(
            &mut out.infiltration_m,
            ledger.infiltration_mass_kg_m2_ofe_ground / KG_M2_PER_M_WATER,
        )?;
        add_nonnegative(
            &mut out.retained_surface_liquid_m,
            ledger.retained_mass_kg_m2_ofe_ground / KG_M2_PER_M_WATER,
        )?;
        let support_runoff_m = ledger.runoff_mass_kg_m2_ofe_ground / KG_M2_PER_M_WATER;
        add_nonnegative(&mut out.runoff_m, support_runoff_m)?;

        let support_seconds = support.support().duration_ns() as f64 / 1.0e9;
        temperature_k_seconds += support.lse_forcing().air_temperature_k * support_seconds;
        total_seconds += support_seconds;
        validate_finite(
            "stage3_publication.temperature_support_integral",
            temperature_k_seconds,
        )?;
        for parcel in support
            .lse_forcing()
            .precipitation_parcels
            .iter()
            .filter(|parcel| &parcel.destination_ofe_id == ofe_id)
        {
            let record = surface_configuration
                .records
                .iter()
                .find(|record| {
                    record.key.ofe_id == parcel.destination_ofe_id
                        && record.key.tile_id == parcel.destination_tile_id
                })
                .ok_or(stage3_publication_guard(
                    "accepted forcing destination configuration",
                ))?;
            let amount_m = parcel.amount_kg_m2_destination_tile_ground * record.tile_fraction
                / KG_M2_PER_M_WATER;
            match parcel.parcel_kind {
                openwepp_land_surface_energy::LiquidParcelKind::Precipitation => {
                    add_nonnegative(&mut out.precipitation_m, amount_m)?;
                }
                openwepp_land_surface_energy::LiquidParcelKind::RoutedRunon => {
                    return Err(stage3_publication_guard(
                        "runon collapsed into precipitation forcing",
                    ));
                }
                openwepp_land_surface_energy::LiquidParcelKind::SnowTerminalReceiver => {
                    add_nonnegative(&mut out.snow_terminal_liquid_m, amount_m)?;
                    distribute_receipt_to_hours(
                        &mut out.hourly_snow_terminal_liquid_m,
                        day_start_ns,
                        support.support(),
                        parcel.start_s,
                        parcel.end_s,
                        amount_m,
                    )?;
                    if amount_m > 0.0 {
                        let temperature_k =
                            parcel.temperature_k.ok_or(stage3_publication_guard(
                                "positive accepted snow-terminal parcel temperature",
                            ))?;
                        out.snow_terminal_custody_temperature_k_mass += temperature_k * amount_m;
                        validate_finite(
                            "stage3_publication.snow_terminal_custody_temperature_mass",
                            out.snow_terminal_custody_temperature_k_mass,
                        )?;
                    }
                }
            }
        }
        let receipt_components = accepted_runon_receipt_components(
            support.ingress_receipts(),
            ofe_id,
            surface_configuration,
        )?;
        let receipt_ingress_m =
            receipt_components.local_liquid_m + receipt_components.upstream_runon_m;
        if (receipt_ingress_m - ledger.ingress_mass_kg_m2_ofe_ground / KG_M2_PER_M_WATER).abs()
            > ACCEPTED_CLOSURE_TOLERANCE_M
        {
            return Err(stage3_publication_guard(
                "accepted ingress ledger/receipt source closure",
            ));
        }
        add_nonnegative(&mut out.runon_m, receipt_components.upstream_runon_m)?;
        add_nonnegative(
            &mut out.support_liquid_runon_m,
            receipt_components.upstream_runon_m,
        )?;
        let material_outputs = support
            .accepted_snow_liquid_outputs()
            .iter()
            .filter(|output| output.mass_kg_m2_ofe_ground > 0.0)
            .filter(|output| {
                !terminal_event_groups.iter().any(|group| {
                    group.candidates.iter().any(|candidate| {
                        terminal_lane_support_matches(
                            candidate.lane_id,
                            candidate.support,
                            output.lane_id,
                            output.support,
                        )
                    })
                })
            })
            .collect::<Vec<_>>();
        let ordinary_receiver_event = if material_outputs.is_empty() {
            None
        } else {
            let output_receipts = material_outputs
                .iter()
                .map(|output| output.receipt_sha256)
                .collect::<Vec<_>>();
            let fields = output_receipts
                .iter()
                .map(|receipt| FramedField {
                    tag: "snow_liquid_output",
                    value: receipt.as_bytes(),
                })
                .collect::<Vec<_>>();
            let output_set =
                framed_sha256("stage3-v11-positive-support-liquid-output-set", &fields)
                    .map_err(|_| stage3_publication_guard("accepted snow-liquid output set"))?;
            let mass_kg_m2 = material_outputs
                .iter()
                .map(|output| output.mass_kg_m2_ofe_ground)
                .sum::<f64>();
            let enthalpy_j_m2 = material_outputs
                .iter()
                .map(|output| output.sensible_enthalpy_j_m2_ofe_ground)
                .sum::<f64>();
            let context = framed_sha256(
                "stage3-v11-positive-support-liquid-receiver",
                &[
                    FramedField {
                        tag: "parent_transaction",
                        value: support.parent_transaction_id().digest().as_bytes(),
                    },
                    FramedField {
                        tag: "support_start",
                        value: &support.support().start_ns().get().to_be_bytes(),
                    },
                    FramedField {
                        tag: "support_end",
                        value: &support.support().end_ns().get().to_be_bytes(),
                    },
                    FramedField {
                        tag: "support_ending_owner",
                        value: support.ending_complete_owner_set_sha256().as_bytes(),
                    },
                    FramedField {
                        tag: "output_set",
                        value: output_set.as_bytes(),
                    },
                    FramedField {
                        tag: "mass_kg_m2",
                        value: &mass_kg_m2.to_bits().to_be_bytes(),
                    },
                    FramedField {
                        tag: "enthalpy_j_m2",
                        value: &enthalpy_j_m2.to_bits().to_be_bytes(),
                    },
                ],
            )
            .map_err(|_| stage3_publication_guard("accepted snow-liquid receiver context"))?;
            let matches = event_handoffs
                .iter()
                .filter(|event| {
                    event.tick() == support.support().end_ns()
                        && event.parent_transaction_id() == support.parent_transaction_id()
                        && event.beginning_owner_set_digest()
                            == support.ending_complete_owner_set_sha256()
                        && event.event_context_digest() == context
                })
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err(stage3_publication_guard(
                    "accepted snow-liquid real receiver event",
                ));
            }
            Some(matches[0])
        };
        if let Some(event) = ordinary_receiver_event {
            let matching_receipts = coupled_subslabs
                .iter()
                .filter(|subslab| {
                    subslab.support == support.support()
                        && subslab
                            .post_support_liquid_receiver_event
                            .as_ref()
                            .is_some_and(|candidate| candidate.id() == event.id())
                })
                .collect::<Vec<_>>();
            if matching_receipts.len() != 1 {
                return Err(stage3_publication_guard(
                    "accepted snow-liquid receiver surface receipt cardinality",
                ));
            }
            let receiver = matching_receipts[0];
            receiver.validate().map_err(|_| {
                stage3_publication_guard("accepted snow-liquid receiver subslab seal")
            })?;
            apply_support_liquid_custody_v2_to_lane_day(
                &mut out,
                receiver,
                binding,
                surface_configuration,
                day_start_ns,
            )?;
        }
        for output in support
            .accepted_snow_liquid_outputs()
            .iter()
            .filter(|output| output.lane_id == binding.production_lane_id)
        {
            output
                .validate()
                .map_err(|_| stage3_publication_guard("accepted snow-liquid output seal"))?;
            if output.support != support.support()
                || output.ofe_id != binding.ofe_id
                || !accepted_snow_liquid_receipts.insert(output.receipt_sha256)
            {
                return Err(stage3_publication_guard(
                    "accepted snow-liquid output support/lane/order",
                ));
            }
            add_nonnegative(
                &mut out.snow_refreeze_m,
                output.refreeze_kg_m2_ofe_ground / KG_M2_PER_M_WATER,
            )?;
            if output.mass_kg_m2_ofe_ground.to_bits() == 0.0_f64.to_bits() {
                if output.sensible_enthalpy_j_m2_ofe_ground.to_bits() != 0.0_f64.to_bits() {
                    return Err(stage3_publication_guard(
                        "zero accepted snow-liquid output enthalpy",
                    ));
                }
                continue;
            }
            let terminal_event_mode = terminal_event_groups.iter().any(|group| {
                group.candidates.iter().any(|candidate| {
                    terminal_lane_support_matches(
                        candidate.lane_id,
                        candidate.support,
                        output.lane_id,
                        output.support,
                    )
                })
            });
            if terminal_event_mode {
                continue;
            }
            let event = ordinary_receiver_event.ok_or(stage3_publication_guard(
                "accepted snow-liquid receiver event omission",
            ))?;
            event
                .validate()
                .map_err(|_| stage3_publication_guard("accepted snow-liquid receiver seal"))?;
            let amount_m = output.mass_kg_m2_ofe_ground / KG_M2_PER_M_WATER;
            let (temperature_k, specific_enthalpy) =
                crate::snow_stage3_v11_attachment::terminal_liquid_thermodynamics_v1(
                    output.mass_kg_m2_ofe_ground,
                    output.sensible_enthalpy_j_m2_ofe_ground,
                )
                .map_err(|_| stage3_publication_guard("accepted snow-liquid thermodynamics"))?;
            if output.mass_kg_m2_ofe_ground > 0.0
                && (output.sensible_enthalpy_j_m2_ofe_ground
                    - output.mass_kg_m2_ofe_ground * specific_enthalpy)
                    .abs()
                    > 1.0e-9
            {
                return Err(stage3_publication_guard(
                    "accepted snow-liquid enthalpy reconstruction",
                ));
            }
            add_nonnegative(&mut out.snow_terminal_liquid_m, amount_m)?;
            if amount_m > 0.0 {
                out.snow_terminal_custody_temperature_k_mass += temperature_k * amount_m;
                validate_finite(
                    "stage3_publication.snow_terminal_custody_temperature_mass",
                    out.snow_terminal_custody_temperature_k_mass,
                )?;
                distribute_receipt_to_hours(
                    &mut out.hourly_snow_terminal_liquid_m,
                    day_start_ns,
                    support.support(),
                    0.0,
                    support_seconds,
                    amount_m,
                )?;
            }
        }
        // Both endpoints are sealed operands.  Requiring their topology here
        // poisons publication if compact retention drops either owner even
        // though the WB14 ledger is the authoritative interval disposition.
        let _ = ofe_surface_storage_m(
            support.surface_beginning_state(),
            surface_configuration,
            ofe_id,
        )?;
        let _ = ofe_surface_storage_m(
            support.surface_ending_state(),
            surface_configuration,
            ofe_id,
        )?;

        let mut receipt_runoff_m = 0.0;
        for receipt in support.ingress_receipts().iter().filter(|receipt| {
            &receipt.basis_ofe_id == ofe_id
                && matches!(
                    receipt.disposition,
                    DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
                        | DirectSurfaceLiquidReceiptDisposition::OutletRunoff
                )
        }) {
            let amount_m = receipt.mass_kg_m2_basis_ofe_ground / KG_M2_PER_M_WATER;
            add_nonnegative(&mut receipt_runoff_m, amount_m)?;
            distribute_receipt_to_hours(
                &mut out.hourly_runoff_m,
                day_start_ns,
                support.support(),
                receipt.start_s,
                receipt.end_s,
                amount_m,
            )?;
        }
        if (receipt_runoff_m - support_runoff_m).abs() > ACCEPTED_CLOSURE_TOLERANCE_M {
            return Err(stage3_publication_guard(
                "accepted runoff ledger/receipt closure",
            ));
        }

        for amount in support
            .finalized_water_uses()
            .iter()
            .filter(|amount| &amount.key.ofe_id == ofe_id)
        {
            let amount_m = amount.amount_kg_m2_stand_ground / KG_M2_PER_M_WATER;
            if amount_m < 0.0 || !amount_m.is_finite() {
                return Err(stage3_publication_guard("accepted ET amount domain"));
            }
            match amount.key.requesting_component {
                RequestingComponent::VegetationRoot => {
                    add_nonnegative(&mut out.plant_transpiration_m, amount_m)?;
                }
                RequestingComponent::GroundSurface => match amount.key.surface_class {
                    Some(SurfaceClass::BareMineralSoil) => {
                        add_nonnegative(&mut out.soil_evaporation_m, amount_m)?;
                    }
                    Some(SurfaceClass::ForestLitter) => {
                        add_nonnegative(&mut out.residue_evaporation_m, amount_m)?;
                    }
                    None => return Err(stage3_publication_guard("accepted ET surface class")),
                },
            }
            if amount.key.source_type == WaterSourceType::SoilLayerLiquid {
                add_nonnegative(&mut out.soil_storage_et_m, amount_m)?;
            }
        }
        for debit in support.resource_debits() {
            let V11ResourceKey::Water(key) = &debit.resource_key else {
                continue;
            };
            if debit.ofe_id != ofe_id.as_str() {
                continue;
            }
            let layer_index = binding
                .ordered_soil_layer_ids
                .iter()
                .position(|layer| layer == &key.layer_id)
                .ok_or(stage3_publication_guard(
                    "accepted root debit layer binding",
                ))?;
            add_nonnegative(
                &mut out.root_request_by_layer_m[layer_index],
                debit.request / KG_M2_PER_M_WATER,
            )?;
            add_nonnegative(
                &mut out.root_use_by_layer_m[layer_index],
                debit.final_use / KG_M2_PER_M_WATER,
            )?;
        }
    }
    aggregate_parent_end_terminal_receiver_custody(
        &mut out,
        supports,
        event_handoffs,
        terminal_event_groups,
        binding,
        surface_configuration,
        day_start_ns,
    )?;
    if total_seconds <= 0.0 {
        return Err(stage3_publication_guard("accepted temperature support"));
    }
    out.effective_temperature_c = temperature_k_seconds / total_seconds - 273.15;
    validate_finite(
        "stage3_publication.effective_temperature_c",
        out.effective_temperature_c,
    )?;
    out.local_liquid_m = out.ingress_m - out.runon_m;
    validate_nonnegative_direct_m("stage3_publication.local_liquid_m", out.local_liquid_m)?;
    let hourly_total = out.hourly_runoff_m.iter().sum::<f64>();
    if (hourly_total - out.runoff_m).abs() > ACCEPTED_CLOSURE_TOLERANCE_M {
        return Err(stage3_publication_guard(
            "accepted daily runoff timing closure",
        ));
    }
    if (out.infiltration_m + out.retained_surface_liquid_m + out.runoff_m - out.ingress_m).abs()
        > ACCEPTED_CLOSURE_TOLERANCE_M
    {
        return Err(stage3_publication_guard(
            "accepted ingress disposition closure",
        ));
    }
    Ok(out)
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn aggregate_parent_end_terminal_receiver_custody(
    out: &mut AcceptedLaneDay,
    supports: &[Stage3AcceptedPublicationSupportV1],
    event_handoffs: &[AcceptedEventReceiptV1],
    groups: &[crate::snow_stage3_v11_attachment::Stage3V11TerminalEventGroupV1],
    binding: &DirectSurfaceLiquidOfeBinding,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    day_start_ns: u128,
) -> Result<(), DirectRuntimeError> {
    use crate::snow_stage3_v11_attachment::{
        DirectSnowStage3V11TerminalParcel, DirectSnowStage3V11TerminalParcelPosture,
        DirectSnowStage3V11TerminalReceiverDestinationV1,
    };

    let mut parent_ends = BTreeMap::new();
    for support in supports {
        parent_ends
            .entry(support.parent_transaction_id().digest())
            .and_modify(|ending: &mut u128| {
                *ending = (*ending).max(support.support().end_ns().get())
            })
            .or_insert(support.support().end_ns().get());
    }
    let mut accepted_parcels = BTreeSet::new();
    let mut previous_tick = None;
    for group in groups {
        let accepted = group
            .accepted_event_receipt
            .as_ref()
            .ok_or(stage3_publication_guard(
                "terminal receiver custody accepted event",
            ))?;
        accepted
            .validate()
            .map_err(|_| stage3_publication_guard("terminal receiver custody event seal"))?;
        let parent = accepted.parent_transaction_id().digest();
        let parent_end = parent_ends
            .get(&parent)
            .ok_or(stage3_publication_guard("terminal receiver custody parent"))?;
        let support_retains = |digest: Digest32| {
            let mut hex = String::with_capacity(64);
            const HEX: &[u8; 16] = b"0123456789abcdef";
            for byte in digest.as_bytes() {
                hex.push(char::from(HEX[usize::from(byte >> 4)]));
                hex.push(char::from(HEX[usize::from(byte & 0x0f)]));
            }
            let prefix = format!("snow-terminal-{hex}-");
            supports.iter().any(|support| {
                support
                    .lse_forcing()
                    .precipitation_parcels
                    .iter()
                    .chain(&support.lse_forcing().runon_parcels)
                    .any(|parcel| {
                        parcel.parcel_kind
                            == openwepp_land_surface_energy::LiquidParcelKind::SnowTerminalReceiver
                            && parcel.parcel_id.as_str().starts_with(&prefix)
                    })
            })
        };
        if group.tick.get() != *parent_end {
            if group
                .produced_unconsumed_parcels
                .iter()
                .any(|custody| !support_retains(custody.parcel_digest))
            {
                return Err(stage3_publication_guard(
                    "terminal receiver positive-support custody omission",
                ));
            }
            continue;
        }
        if previous_tick.is_some_and(|tick| tick > group.tick) {
            return Err(stage3_publication_guard("terminal receiver custody order"));
        }
        previous_tick = Some(group.tick);
        group
            .validate_terminal_receiver_custody_v2()
            .map_err(|_| stage3_publication_guard("terminal receiver typed custody seal"))?;
        let typed_custody =
            group
                .terminal_receiver_custody_v2()
                .ok_or(stage3_publication_guard(
                    "terminal receiver typed custody omission",
                ))?;
        let receiver_ordinal =
            accepted
                .ordinal()
                .checked_add(1)
                .ok_or(stage3_publication_guard(
                    "terminal receiver custody ordinal",
                ))?;
        let receivers = event_handoffs
            .iter()
            .filter(|event| {
                event.parent_transaction_id() == accepted.parent_transaction_id()
                    && event.tick() == group.tick
                    && event.ordinal() == receiver_ordinal
                    && event.beginning_owner_set_digest() == accepted.ending_owner_set_digest()
            })
            .collect::<Vec<_>>();
        if receivers.len() != 1
            || receivers[0].validate().is_err()
            || receivers[0] != &typed_custody.receiver_event
            || group.produced_unconsumed_parcels.len() != group.candidates.len()
        {
            return Err(stage3_publication_guard(
                "terminal receiver custody event handoff",
            ));
        }
        let mut ordered_digests = Vec::with_capacity(group.produced_unconsumed_parcels.len());
        for custody in &group.produced_unconsumed_parcels {
            if support_retains(custody.parcel_digest) {
                return Err(stage3_publication_guard(
                    "terminal receiver duplicate consumption mode",
                ));
            }
            let candidate = group
                .candidates
                .iter()
                .find(|candidate| candidate.lane_id == custody.source_lane_id)
                .ok_or(stage3_publication_guard("terminal receiver custody lane"))?;
            let parcel = DirectSnowStage3V11TerminalParcel {
                support: custody.support,
                source_lane_id: custody.source_lane_id,
                parent_transaction_id: custody.parent_transaction_id,
                event_ordinal: custody.event_ordinal,
                terminal_event_proposal_core_id: custody.terminal_event_proposal_core_id,
                event_result_digest: custody.event_result_digest,
                receiver_topology_sha256: custody.receiver_topology_sha256,
                destination_ofe_id: custody.destination_ofe_id.clone(),
                receiver_destinations: custody
                    .receiver_destinations
                    .iter()
                    .map(
                        |destination| DirectSnowStage3V11TerminalReceiverDestinationV1 {
                            destination_ofe_id: destination.destination_ofe_id.clone(),
                            destination_tile_id: destination.destination_tile_id.clone(),
                            destination_fraction: destination.destination_fraction,
                        },
                    )
                    .collect(),
                mass_kg_m2_tile_ground: custody.mass_kg_m2_tile_ground,
                temperature_k: custody.temperature_k,
                specific_liquid_enthalpy_j_kg: custody.specific_liquid_enthalpy_j_kg,
                posture: DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed,
                parcel_digest: custody.parcel_digest,
            };
            let canonical_digest = crate::snow_owner_v4::canonical_terminal_parcel_digest(&parcel)
                .map_err(|_| stage3_publication_guard("terminal receiver custody parcel seal"))?;
            let (temperature_k, specific_enthalpy) =
                crate::snow_stage3_v11_attachment::terminal_liquid_thermodynamics_v1(
                    candidate.event.terminal_liquid_kg_m2,
                    candidate.event.terminal_unallocated_energy_j_m2,
                )
                .map_err(|_| {
                    stage3_publication_guard("terminal receiver custody thermodynamics")
                })?;
            let records = surface_configuration
                .records
                .iter()
                .filter(|record| record.key.ofe_id.as_str() == custody.destination_ofe_id)
                .collect::<Vec<_>>();
            let mut topology_bytes = Vec::new();
            for record in &records {
                topology_bytes.extend_from_slice(record.key.ofe_id.as_str().as_bytes());
                topology_bytes.push(0);
                topology_bytes.extend_from_slice(record.key.tile_id.as_str().as_bytes());
                topology_bytes.extend_from_slice(&record.tile_fraction.to_bits().to_be_bytes());
            }
            let topology = digest_bytes(&topology_bytes);
            if canonical_digest != custody.parcel_digest
                || !accepted_parcels.insert(custody.parcel_digest)
                || custody.parent_transaction_id != parent
                || custody.event_ordinal != accepted.ordinal()
                || custody.terminal_event_proposal_core_id
                    != group.proposal_core_sha256.ok_or(stage3_publication_guard(
                        "terminal receiver custody proposal core",
                    ))?
                || custody.event_result_digest != candidate.event_result_digest
                || custody.support != candidate.support
                || custody.mass_kg_m2_tile_ground.to_bits()
                    != candidate.event.terminal_liquid_kg_m2.to_bits()
                || custody.temperature_k.to_bits() != temperature_k.to_bits()
                || custody.specific_liquid_enthalpy_j_kg.to_bits() != specific_enthalpy.to_bits()
                || records.len() != custody.receiver_destinations.len()
                || topology != custody.receiver_topology_sha256
                || records.iter().zip(&custody.receiver_destinations).any(
                    |(record, destination)| {
                        record.key.ofe_id.as_str() != destination.destination_ofe_id
                            || record.key.tile_id.as_str() != destination.destination_tile_id
                            || record.tile_fraction.to_bits()
                                != destination.destination_fraction.to_bits()
                    },
                )
            {
                return Err(stage3_publication_guard(
                    "terminal receiver custody identity/topology",
                ));
            }
            ordered_digests.push(custody.parcel_digest);
            if custody.destination_ofe_id != binding.ofe_id.as_str() {
                continue;
            }
            let amount_m = custody.mass_kg_m2_tile_ground / KG_M2_PER_M_WATER;
            add_nonnegative(&mut out.snow_terminal_liquid_m, amount_m)?;
            if amount_m > 0.0 {
                out.snow_terminal_custody_temperature_k_mass += custody.temperature_k * amount_m;
                validate_finite(
                    "stage3_publication.snow_terminal_custody_temperature_mass",
                    out.snow_terminal_custody_temperature_k_mass,
                )?;
            }
            let tick_offset =
                group
                    .tick
                    .get()
                    .checked_sub(day_start_ns)
                    .ok_or(stage3_publication_guard(
                        "terminal receiver custody day tick",
                    ))?;
            let hour = usize::try_from(tick_offset.saturating_sub(1) / 3_600_000_000_000)
                .map_err(|_| stage3_publication_guard("terminal receiver custody hour"))?
                .min(23);
            add_nonnegative(&mut out.hourly_snow_terminal_liquid_m[hour], amount_m)?;
        }
        ordered_digests.sort_unstable();
        if ordered_digests != group.produced_unconsumed_parcel_digests {
            return Err(stage3_publication_guard(
                "terminal receiver custody parcel order/set",
            ));
        }
        project_zero_duration_snow_liquid_receipts_to_lane_day(
            out,
            &typed_custody.receiver_receipts,
            typed_custody.receiver_receipt_set_sha256,
            binding,
            surface_configuration,
            &typed_custody.surface_beginning_state,
            &typed_custody.surface_ending_state,
            day_start_ns,
            typed_custody.support,
            false,
            true,
        )?;
    }
    Ok(())
}

fn ofe_surface_storage_m(
    state: &DirectSurfaceLiquidOwnedState,
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: &OfeId,
) -> Result<f64, DirectRuntimeError> {
    let mut storage_m = 0.0;
    for state_record in state
        .records
        .iter()
        .filter(|record| &record.key.ofe_id == ofe_id)
    {
        let configuration_record = configuration
            .records
            .iter()
            .find(|record| record.key == state_record.key)
            .ok_or(stage3_publication_guard(
                "accepted surface storage configuration join",
            ))?;
        storage_m +=
            state_record.liquid_kg_m2_tile * configuration_record.tile_fraction / KG_M2_PER_M_WATER;
        validate_nonnegative_direct_m("stage3_publication.surface_storage_m", storage_m)?;
    }
    Ok(storage_m)
}

fn accepted_ending_soil_water_m(
    accepted_ending_layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, DirectRuntimeError> {
    aggregate_direct_soil_water(
        accepted_ending_layers,
        "stage3_publication.accepted_ending_soil_water_m",
    )
}

fn validate_accepted_ending_layer_identity(
    accepted_ending_layers: &[DirectSubsurfaceLayerState],
    staged_frame_layers: &[DirectSubsurfaceLayerState],
) -> Result<(), DirectRuntimeError> {
    if accepted_ending_layers != staged_frame_layers {
        return Err(stage3_publication_guard(
            "accepted ending subsurface/frame substitution",
        ));
    }
    Ok(())
}

fn seed_accepted_upstream_day(
    day: &mut DirectDayFrame,
    beginning_soil_m: f64,
    accepted_ending_soil_m: f64,
    accepted_ending_layers: &[DirectSubsurfaceLayerState],
    accepted: &AcceptedLaneDay,
    beginning_carry: &Stage3AcceptedBeginningLaneCarryV1,
    retained_lane: &DirectLaneFrame,
) -> Result<(), DirectRuntimeError> {
    if beginning_carry.lane_id != retained_lane.lane_id
        || beginning_carry.upstream_lane_id != retained_lane.upstream_lane_id
        || beginning_carry.downstream_lane_id != retained_lane.downstream_lane_id
        || beginning_carry.upstream_area_ratio.to_bits() != day.upstream_area_ratio.to_bits()
        || beginning_carry.surface_carry_m != day.transfer.surface_carry_m
        || beginning_carry.surface_hourly_weights != day.transfer.surface_hourly_weights
        || beginning_carry.lateral_carry_m != day.transfer.lateral_carry_m
        || beginning_carry.upstream_flow_m.to_bits() != day.transfer.upstream_flow_m.to_bits()
        || beginning_carry.subsurface_input_m.to_bits() != day.transfer.subsurface_input_m.to_bits()
    {
        return Err(stage3_publication_guard(
            "accepted beginning lane carry/frame identity",
        ));
    }
    let raw_lateral_carry_m = sum_nonnegative_direct_m(
        "stage3_publication.accepted_lateral_carry_m",
        &beginning_carry.lateral_carry_m,
    )?;
    let lateral_transfer_m = scaled_direct_transfer_total_m(
        "stage3_publication.accepted_lateral_transfer_m",
        raw_lateral_carry_m,
        beginning_carry.upstream_area_ratio,
    )?;
    validate_nonnegative_direct_m(
        "stage3_publication.accepted_lateral_transfer_m",
        lateral_transfer_m,
    )?;
    let raw_surface_carry_m = sum_nonnegative_direct_m(
        "stage3_publication.accepted_surface_carry_m",
        &beginning_carry.surface_carry_m,
    )?;
    let surface_transfer_m = scaled_direct_transfer_total_m(
        "stage3_publication.accepted_surface_transfer_m",
        raw_surface_carry_m,
        beginning_carry.upstream_area_ratio,
    )?;
    let retained_runon_m = surface_transfer_m + beginning_carry.upstream_flow_m;
    validate_nonnegative_direct_m(
        "stage3_publication.accepted_surface_transfer_m",
        retained_runon_m,
    )?;
    if (retained_runon_m + accepted.support_liquid_runon_m - accepted.runon_m).abs()
        > ACCEPTED_CLOSURE_TOLERANCE_M
    {
        return Err(stage3_publication_guard(
            "accepted retained-carry/routed-runon closure",
        ));
    }
    let subsurface_carry_m = lateral_transfer_m + beginning_carry.subsurface_input_m;
    validate_nonnegative_direct_m(
        "stage3_publication.accepted_subsurface_carry_m",
        subsurface_carry_m,
    )?;
    let transfer_input_m = surface_transfer_m
        + lateral_transfer_m
        + beginning_carry.upstream_flow_m
        + beginning_carry.subsurface_input_m;
    validate_nonnegative_direct_m(
        "stage3_publication.accepted_transfer_input_m",
        transfer_input_m,
    )?;
    let total_accounted_input_m = accepted.precipitation_m + transfer_input_m;
    validate_nonnegative_direct_m(
        "stage3_publication.accepted_total_accounted_input_m",
        total_accounted_input_m,
    )?;

    day.forcing.precipitation_m = accepted.precipitation_m;
    day.water.soil_water_m = beginning_soil_m;
    day.input_accounting = DirectInputAccountingState {
        precipitation_m: accepted.precipitation_m,
        surface_transfer_m,
        lateral_transfer_m,
        upstream_flow_m: beginning_carry.upstream_flow_m,
        subsurface_input_m: beginning_carry.subsurface_input_m,
        transfer_input_m,
        total_accounted_input_m,
    };
    day.downstream_operands = DirectDownstreamOperands::from(day.input_accounting);
    day.shadow_projection = Some(DirectShadowProjection {
        lane_index: day.lane_index,
        day_index: day.day_index,
        precipitation_m: accepted.precipitation_m,
        transfer_input_m,
        total_accounted_input_m,
    });
    day.normalization = DirectNormalizationState {
        precipitation_m: accepted.precipitation_m,
        effective_temperature_c: accepted.effective_temperature_c,
        storage_initial_m: beginning_soil_m,
        surface_transfer_m,
        lateral_transfer_m,
        upstream_flow_m: beginning_carry.upstream_flow_m,
        subsurface_input_m: beginning_carry.subsurface_input_m,
        transfer_input_m,
        total_accounted_input_m,
    };
    day.normalization_downstream_operands =
        DirectNormalizationDownstreamOperands::from(day.normalization);
    day.normalization_shadow_projection = Some(DirectNormalizationShadowProjection {
        lane_index: day.lane_index,
        day_index: day.day_index,
        precipitation_m: accepted.precipitation_m,
        storage_initial_m: beginning_soil_m,
        transfer_input_m,
        total_accounted_input_m,
    });

    day.storage_input = DirectStorageInputState {
        storage_initial_m: beginning_soil_m,
        precip_input_m: accepted.local_liquid_m,
    };
    day.storage_input_downstream_operands =
        DirectStorageInputDownstreamOperands::from(day.storage_input);
    day.storage_input_shadow_projection = Some(DirectStorageInputShadowProjection {
        lane_index: day.lane_index,
        day_index: day.day_index,
        storage_initial_m: beginning_soil_m,
        precip_input_m: accepted.local_liquid_m,
    });

    day.liquid_input = DirectLiquidInputState {
        liquid_input_m: accepted.local_liquid_m,
    };
    day.liquid_input_downstream_operands =
        DirectLiquidInputDownstreamOperands::from(day.liquid_input);
    day.liquid_input_shadow_projection = Some(DirectLiquidInputShadowProjection {
        lane_index: day.lane_index,
        day_index: day.day_index,
        liquid_input_m: accepted.local_liquid_m,
    });
    day.runon_carry = DirectRunonCarryState {
        runon_input_m: accepted.runon_m,
        subsurface_carry_m,
    };
    day.runon_carry_downstream_operands = DirectRunonCarryDownstreamOperands::from(day.runon_carry);
    day.runon_carry_shadow_projection = Some(DirectRunonCarryShadowProjection {
        lane_index: day.lane_index,
        day_index: day.day_index,
        runon_input_m: accepted.runon_m,
        subsurface_carry_m,
    });
    day.infiltration_depression = DirectInfiltrationDepressionState {
        cumulative_infiltration_m: accepted.infiltration_m,
        depression_storage_delta_m: accepted.retained_surface_liquid_m,
    };
    day.infiltration_depression_downstream_operands =
        DirectInfiltrationDepressionDownstreamOperands::from(day.infiltration_depression);
    day.infiltration_depression_shadow_projection =
        Some(DirectInfiltrationDepressionShadowProjection {
            lane_index: day.lane_index,
            day_index: day.day_index,
            cumulative_infiltration_m: accepted.infiltration_m,
            depression_storage_delta_m: accepted.retained_surface_liquid_m,
        });
    day.runoff_partition_inputs.liquid_input_m = accepted.local_liquid_m;
    day.runoff_partition_inputs.runon_input_m = accepted.runon_m + subsurface_carry_m;
    day.runoff_partition_inputs.cumulative_infiltration_m = accepted.infiltration_m;
    day.runoff_partition_inputs.depression_storage_delta_m = accepted.retained_surface_liquid_m;
    day.percolation_inputs.layers = accepted_ending_layers.to_vec();
    day.percolation_inputs.soil_water_initial_m = accepted_ending_soil_m;
    day.percolation_inputs.same_pass_infiltration_m = accepted_installed_infiltration_m();
    day.percolation_inputs.same_pass_infiltration_lineage = true;
    day.evapotranspiration_compute_inputs
        .same_pass_infiltration_m = accepted_installed_infiltration_m();
    day.water.soil_water_m = accepted_ending_soil_m;
    day.evapotranspiration_surface_shadow_projection = None;
    day.wb14_hourly_excess_m = accepted.hourly_runoff_m;
    Ok(())
}

/// The accepted WB14 owner has already installed its infiltration in the
/// retained ending layers. This typed consumed value prevents a second
/// application when the downstream-only day owners execute.
const fn accepted_installed_infiltration_m() -> f64 {
    0.0
}

fn run_downstream_only_hydrology(
    day: &mut DirectDayFrame,
    accepted: &AcceptedLaneDay,
    accepted_water_stress: f64,
) -> Result<(), DirectRuntimeError> {
    day.run_r4m_percolation_span()?;
    day.evapotranspiration_surface_shadow_projection = None;
    day.run_r4o_subsurface_compute_span()?;
    let layers = day.subsurface_compute.layer_state_after.clone();
    let soil_water_m =
        aggregate_direct_soil_water(&layers, "stage3_publication.post_subsurface_soil_water_m")?;
    let total_et = accepted.evapotranspiration_m();
    if accepted.root_request_by_layer_m.len() != layers.len()
        || accepted.root_use_by_layer_m.len() != layers.len()
    {
        return Err(stage3_publication_guard(
            "accepted root-uptake layer cardinality",
        ));
    }
    let uptake_potential_m = accepted.root_request_by_layer_m.iter().sum::<f64>();
    let uptake_actual_m = accepted.root_use_by_layer_m.iter().sum::<f64>();
    validate_nonnegative_direct_m(
        "stage3_publication.accepted_root_uptake_potential_m",
        uptake_potential_m,
    )?;
    validate_nonnegative_direct_m(
        "stage3_publication.accepted_root_uptake_actual_m",
        uptake_actual_m,
    )?;
    if (uptake_actual_m - accepted.plant_transpiration_m).abs() > ACCEPTED_CLOSURE_TOLERANCE_M {
        return Err(stage3_publication_guard(
            "accepted root-use/transpiration closure",
        ));
    }
    let soil_water_before_root_uptake_m = soil_water_m + uptake_actual_m;
    validate_nonnegative_direct_m(
        "stage3_publication.soil_water_before_root_uptake_m",
        soil_water_before_root_uptake_m,
    )?;
    let state = DirectEvapotranspirationComputeState {
        soil_water_before_root_uptake_m,
        soil_water_after_m: soil_water_m,
        evapotranspiration_m: total_et,
        soil_evaporation_m: accepted.soil_evaporation_m,
        residue_evaporation_m: accepted.residue_evaporation_m,
        soil_evaporation_storage_return_m: total_et - accepted.soil_storage_et_m,
        plant_transpiration_m: accepted.plant_transpiration_m,
        transpiration_demand_m: uptake_potential_m,
        water_stress: accepted_water_stress,
        uptake_potential_m,
        uptake_actual_m,
        effective_plant_tolerance: day.evapotranspiration_compute_inputs.plant_tolerance,
        layer_uptake_potential_m: accepted.root_request_by_layer_m.clone(),
        layer_uptake_actual_m: accepted.root_use_by_layer_m.clone(),
        layer_state_after_root_uptake: layers.clone(),
    };
    day.evapotranspiration_compute = state.clone();
    day.evapotranspiration_compute_downstream_operands = state.clone().into();
    day.evapotranspiration_compute_shadow_projection =
        Some(DirectEvapotranspirationComputeShadowProjection {
            lane_index: day.lane_index,
            day_index: day.day_index,
            soil_water_before_root_uptake_m: state.soil_water_before_root_uptake_m,
            soil_water_after_m: state.soil_water_after_m,
            evapotranspiration_m: state.evapotranspiration_m,
            soil_evaporation_m: state.soil_evaporation_m,
            residue_evaporation_m: state.residue_evaporation_m,
            soil_evaporation_storage_return_m: state.soil_evaporation_storage_return_m,
            plant_transpiration_m: state.plant_transpiration_m,
            transpiration_demand_m: state.transpiration_demand_m,
            water_stress: state.water_stress,
            uptake_potential_m: state.uptake_potential_m,
            uptake_actual_m: state.uptake_actual_m,
            effective_plant_tolerance: state.effective_plant_tolerance,
            layer_uptake_potential_m: state.layer_uptake_potential_m,
            layer_uptake_actual_m: state.layer_uptake_actual_m,
            layer_state_after_root_uptake: layers,
        });
    day.storage_reconciliation_inputs.evapotranspiration_m = total_et;
    day.storage_reconciliation_inputs
        .evapotranspiration_storage_return_m = total_et - accepted.soil_storage_et_m;
    Ok(())
}

fn install_stage3_projection(
    day: &mut DirectDayFrame,
    beginning: &DirectSnowStage3PersistentState,
    ending: &DirectSnowStage3PersistentState,
    accepted: &AcceptedLaneDay,
) -> Result<Option<f64>, DirectRuntimeError> {
    let accepted_albedo = day.winter_column.snow.snow_albedo_state;
    let snow = direct_snow_lane_from_stage3(ending, accepted_albedo)?;
    let sublimation_m = checked_cumulative_delta(
        beginning.cumulative_sublimation_kg_m2,
        ending.cumulative_sublimation_kg_m2,
        "stage3_publication.sublimation_m",
    )? / KG_M2_PER_M_WATER;
    let melt_m = checked_cumulative_delta(
        beginning.cumulative_melt_kg_m2,
        ending.cumulative_melt_kg_m2,
        "stage3_publication.melt_m",
    )? / KG_M2_PER_M_WATER;
    let external_liquid_m = checked_cumulative_delta(
        beginning.cumulative_external_liquid_kg_m2,
        ending.cumulative_external_liquid_kg_m2,
        "stage3_publication.external_liquid_m",
    )? / KG_M2_PER_M_WATER;
    let snowfall_m = checked_cumulative_delta(
        beginning.cumulative_snowfall_kg_m2,
        ending.cumulative_snowfall_kg_m2,
        "stage3_publication.snowfall_m",
    )? / KG_M2_PER_M_WATER;
    let climate_precipitation_m = accepted.precipitation_m + snowfall_m;
    validate_nonnegative_direct_m(
        "stage3_publication.climate_precipitation_m",
        climate_precipitation_m,
    )?;
    day.forcing.precipitation_m = climate_precipitation_m;
    let climate_total_accounted_input_m =
        climate_precipitation_m + day.input_accounting.transfer_input_m;
    validate_nonnegative_direct_m(
        "stage3_publication.climate_total_accounted_input_m",
        climate_total_accounted_input_m,
    )?;
    day.input_accounting.precipitation_m = climate_precipitation_m;
    day.input_accounting.total_accounted_input_m = climate_total_accounted_input_m;
    day.downstream_operands.precipitation_m = climate_precipitation_m;
    day.downstream_operands.total_accounted_input_m = climate_total_accounted_input_m;
    day.normalization.precipitation_m = climate_precipitation_m;
    day.normalization.total_accounted_input_m = climate_total_accounted_input_m;
    day.normalization_downstream_operands.precipitation_m = climate_precipitation_m;
    day.normalization_downstream_operands
        .total_accounted_input_m = climate_total_accounted_input_m;
    if let Some(projection) = &mut day.normalization_shadow_projection {
        projection.precipitation_m = climate_precipitation_m;
        projection.total_accounted_input_m = climate_total_accounted_input_m;
    } else {
        return Err(stage3_publication_guard(
            "accepted normalization projection missing",
        ));
    }
    let deposition_m = checked_cumulative_delta(
        beginning.cumulative_deposition_kg_m2,
        ending.cumulative_deposition_kg_m2,
        "stage3_publication.deposition_m",
    )? / KG_M2_PER_M_WATER;
    let unresolved_liquid_m = checked_cumulative_delta(
        beginning.cumulative_unresolved_liquid_kg_m2,
        ending.cumulative_unresolved_liquid_kg_m2,
        "stage3_publication.unresolved_liquid_m",
    )? / KG_M2_PER_M_WATER;
    validate_accepted_terminal_liquid_receiver_custody(
        unresolved_liquid_m,
        accepted.snow_terminal_liquid_m,
    )?;
    let beginning_ice_m = stage3_ice_m(beginning)?;
    let ending_ice_m = stage3_ice_m(ending)?;
    let reconstructed_refrozen_liquid_m =
        ending_ice_m - beginning_ice_m - snowfall_m - deposition_m + sublimation_m + melt_m;
    let refrozen_liquid_m = accepted.snow_refreeze_m;
    if !reconstructed_refrozen_liquid_m.is_finite()
        || !refrozen_liquid_m.is_finite()
        || refrozen_liquid_m < 0.0
        || (reconstructed_refrozen_liquid_m - refrozen_liquid_m).abs()
            > ACCEPTED_CLOSURE_TOLERANCE_M
    {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "stage3_committed_publication",
            detail: format!(
                "accepted refrozen-liquid physical ledger closure; refrozen_liquid_m={refrozen_liquid_m:.17e} bits={:#018x}; reconstructed_refrozen_liquid_m={reconstructed_refrozen_liquid_m:.17e} bits={:#018x}; residual_m={:.17e}; tolerance_m={ACCEPTED_CLOSURE_TOLERANCE_M:.17e}; beginning_ice_m={beginning_ice_m:.17e}; ending_ice_m={ending_ice_m:.17e}; snowfall_m={snowfall_m:.17e}; deposition_m={deposition_m:.17e}; sublimation_m={sublimation_m:.17e}; melt_m={melt_m:.17e}; external_liquid_m={external_liquid_m:.17e}; accepted_terminal_liquid_m={:.17e}",
                refrozen_liquid_m.to_bits(),
                reconstructed_refrozen_liquid_m.to_bits(),
                reconstructed_refrozen_liquid_m - refrozen_liquid_m,
                accepted.snow_terminal_liquid_m,
            ),
        });
    }
    let retained_liquid_delta_m =
        stage3_retained_liquid_m(ending)? - stage3_retained_liquid_m(beginning)?;
    validate_finite(
        "stage3_publication.retained_liquid_delta_m",
        retained_liquid_delta_m,
    )?;
    let incoming_liquid_m = melt_m + external_liquid_m;
    validate_nonnegative_direct_m(
        "stage3_publication.incoming_snow_liquid_m",
        incoming_liquid_m,
    )?;
    // Receiver custody retains every strictly-positive parcel and its exact
    // sensible enthalpy. That bulk parcel can include externally supplied
    // liquid above the freezing reference. WB11's nullable
    // `MeltwaterTemperature` is instead the Stage-3 phase reference and must
    // never be inferred from the mixed parcel temperature.
    let meltwater_temperature_c = accepted_meltwater_phase_reference_c(
        accepted.snow_terminal_liquid_m,
        accepted.snow_terminal_custody_temperature_k_mass,
    )?;
    let ledgers = DirectSnowMassTransitionLedgers::try_from_parts(
        DirectSnowSolidToLiquidLedger {
            raw_signed_melt_m: melt_m,
            redistributed_positive_melt_m: melt_m,
            snowpack_swe_loss_m: melt_m,
            rain_released_m: external_liquid_m,
            liquid_handoff_m: incoming_liquid_m,
        },
        DirectSnowLiquidDispositionLedger {
            incoming_liquid_m,
            routed_liquid_m: accepted.snow_terminal_liquid_m,
            retained_liquid_delta_m,
            refrozen_liquid_m,
            liquid_closure_residual_m: accepted_stage3_closed_residual_m(),
        },
        DirectSnowStage3Outcome {
            enabled: true,
            meltwater_temperature_c,
            sublimation_m,
        },
    )
    .map_err(|error| DirectRuntimeError::DirectKernelGuardFailure {
        phase: "stage3_committed_publication",
        detail: format!(
            "accepted snow ledger: {error}; incoming_liquid_m={incoming_liquid_m:.17e} bits={:#018x}; routed_liquid_m={:.17e} bits={:#018x}; retained_liquid_delta_m={retained_liquid_delta_m:.17e} bits={:#018x}; refrozen_liquid_m={refrozen_liquid_m:.17e} bits={:#018x}; melt_m={melt_m:.17e}; external_liquid_m={external_liquid_m:.17e}; sublimation_m={sublimation_m:.17e}; meltwater_temperature_c={:?}",
            incoming_liquid_m.to_bits(),
            accepted.snow_terminal_liquid_m,
            accepted.snow_terminal_liquid_m.to_bits(),
            retained_liquid_delta_m.to_bits(),
            refrozen_liquid_m.to_bits(),
            meltwater_temperature_c.map(TemperatureCelsius::as_celsius),
        ),
    })?;
    let state = DirectSnowCouplingState {
        snow_coupling_m: accepted_terminal_liquid_already_in_ingress_m(),
        snow_state_projected: true,
        active_snow_coupling: true,
        mass_transition_ledgers: ledgers,
        sublimation_m,
        post_winter_rain_m: external_liquid_m,
        runtime_swe_after_m: snow.runtime_swe_m,
        runtime_depth_after_m: snow.runtime_depth_m,
        runtime_density_after_kg_m3: snow.runtime_density_kg_m3,
        runtime_settle_day_count_after: snow.runtime_settle_day_count,
        coe_boundary_depth_after_m: snow.runtime_depth_m,
        coe_boundary_density_after_kg_m3: snow.runtime_density_kg_m3,
        coe_boundary_settle_day_count_after: snow.runtime_settle_day_count,
        liquid_holding_capacity_after_m: snow.liquid_water_retained_m,
        liquid_water_retained_after_m: snow.liquid_water_retained_m,
        liquid_water_released_m: accepted.snow_terminal_liquid_m,
        snow_albedo_state_after: snow.snow_albedo_state,
        snow_layers_after: snow.layers.clone(),
    };
    day.snow_coupling = state.clone();
    day.snow_coupling_downstream_operands =
        DirectSnowCouplingDownstreamOperands::from_state_and_hourly_routed_melt(
            state.clone(),
            accepted.hourly_snow_terminal_liquid_m,
        );
    day.snow_coupling_shadow_projection = Some(Box::new(DirectSnowCouplingShadowProjection {
        lane_index: day.lane_index,
        day_index: day.day_index,
        snow_coupling_m: accepted_terminal_liquid_already_in_ingress_m(),
        active_snow_coupling: true,
        mass_transition_ledgers: ledgers,
        sublimation_m,
        post_winter_rain_m: external_liquid_m,
        runtime_swe_after_m: snow.runtime_swe_m,
        runtime_depth_after_m: snow.runtime_depth_m,
        runtime_density_after_kg_m3: snow.runtime_density_kg_m3,
        runtime_settle_day_count_after: snow.runtime_settle_day_count,
        coe_boundary_depth_after_m: snow.runtime_depth_m,
        coe_boundary_density_after_kg_m3: snow.runtime_density_kg_m3,
        coe_boundary_settle_day_count_after: snow.runtime_settle_day_count,
        snow_albedo_state_after: snow.snow_albedo_state,
    }));
    day.winter_column.snow = snow;
    day.snow_runtime_carry = direct_snow_runtime_carry_from_winter_state(&day.winter_column.snow);
    day.storage_reconciliation_inputs.snow_coupling_m =
        accepted_terminal_liquid_already_in_ingress_m();
    let surface_temperature_c = if ending.layers.is_empty() {
        None
    } else {
        Some(
            Wb11HydrologyKernel::project_stage3_surface_state_v1(ending)
                .map_err(|error| DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "stage3_committed_publication",
                    detail: format!("Stage-3 surface projection: {error}"),
                })?
                .surface_temperature_k
                - 273.15,
        )
    };
    Ok(surface_temperature_c)
}

fn validate_accepted_terminal_liquid_receiver_custody(
    cumulative_terminal_liquid_m: f64,
    sealed_receiver_parcel_liquid_m: f64,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "stage3_publication.cumulative_terminal_liquid_m",
        cumulative_terminal_liquid_m,
    )?;
    validate_nonnegative_direct_m(
        "stage3_publication.sealed_receiver_parcel_liquid_m",
        sealed_receiver_parcel_liquid_m,
    )?;
    let residual_m = cumulative_terminal_liquid_m - sealed_receiver_parcel_liquid_m;
    validate_finite(
        "stage3_publication.terminal_liquid_receiver_residual_m",
        residual_m,
    )?;
    if residual_m.abs() > ACCEPTED_CLOSURE_TOLERANCE_M {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "stage3_committed_publication",
            detail: format!(
                "accepted Stage-3 terminal-liquid receiver custody: cumulative_terminal_liquid_m={cumulative_terminal_liquid_m:?}, sealed_receiver_parcel_liquid_m={sealed_receiver_parcel_liquid_m:?}, residual_m={residual_m:?}, tolerance_m={ACCEPTED_CLOSURE_TOLERANCE_M:?}"
            ),
        });
    }
    Ok(())
}

fn finish_storage_and_projection(
    day: &mut DirectDayFrame,
    beginning_soil_m: f64,
    accepted: &AcceptedLaneDay,
) -> Result<(), DirectRuntimeError> {
    day.storage_reconciliation_inputs.storage_initial_m = beginning_soil_m;
    day.storage_reconciliation_inputs.precip_input_m = accepted.local_liquid_m;
    day.storage_reconciliation_inputs.runon_input_m =
        day.runon_carry.runon_input_m + day.runon_carry.subsurface_carry_m;
    day.storage_reconciliation_inputs.interception_m = accepted.retained_surface_liquid_m;
    day.storage_reconciliation_inputs.deep_seepage_m = day.percolation.deep_seepage_m;
    day.storage_reconciliation_inputs.subsurface_loss_m = day.subsurface_compute.subsurface_loss_m;
    day.run_r4b_storage_reconciliation_span()?;
    day.hydrology_projection_inputs.snow_water_m =
        day.winter_column.snow.runtime_swe_m + day.winter_column.snow.liquid_water_retained_m;
    day.run_r4pqz_hydrology_projection_span()?;
    day.water.infiltration_m = accepted.infiltration_m;
    day.water.runoff_m = day.hydrology_projection.q_runoff_m;
    day.water.evapotranspiration_m = accepted.evapotranspiration_m();
    day.publication.infiltration_m = accepted.infiltration_m;
    day.publication.runoff_m = day.hydrology_projection.q_runoff_m;
    day.publication.evapotranspiration_m = accepted.evapotranspiration_m();
    day.publication.drainage_m = day.hydrology_projection.deep_percolation_m;
    day.publication.lateral_flow_m = day.hydrology_projection.lateral_flow_m;
    Ok(())
}

fn direct_snow_lane_from_stage3(
    state: &DirectSnowStage3PersistentState,
    accepted_albedo: Option<SnowAlbedoState>,
) -> Result<DirectSnowLaneState, DirectRuntimeError> {
    let runtime_swe_m = state
        .layers
        .iter()
        .map(|layer| layer.mass_swe_m)
        .sum::<f64>();
    let runtime_depth_m = state
        .layers
        .iter()
        .map(|layer| layer.thickness_m)
        .sum::<f64>();
    let layer_liquid_m = state
        .layers
        .iter()
        .map(|layer| layer.liquid_water_m)
        .sum::<f64>();
    let liquid_water_retained_m =
        layer_liquid_m + state.detached_retained_liquid_kg_m2 / KG_M2_PER_M_WATER;
    for (field, value) in [
        ("stage3_publication.runtime_swe_m", runtime_swe_m),
        ("stage3_publication.runtime_depth_m", runtime_depth_m),
        (
            "stage3_publication.liquid_water_retained_m",
            liquid_water_retained_m,
        ),
    ] {
        validate_nonnegative_direct_m(field, value)?;
    }
    let runtime_density_kg_m3 = if runtime_depth_m > 0.0 {
        runtime_swe_m * KG_M2_PER_M_WATER / runtime_depth_m
    } else if runtime_swe_m == 0.0 {
        0.0
    } else {
        return Err(stage3_publication_guard("Stage-3 zero-depth snow mass"));
    };
    let runtime_settle_day_count = state
        .layers
        .first()
        .map_or_else(no_active_snow_settle_day_count, |layer| {
            layer.settle_day_count
        });
    validate_nonnegative_direct_m(
        "stage3_publication.runtime_settle_day_count",
        runtime_settle_day_count,
    )?;
    Ok(
        DirectSnowLaneState::from_runtime_values_boundary_liquid_albedo_and_layers(
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            liquid_water_retained_m,
            accepted_albedo,
            state.layers.clone(),
        ),
    )
}

fn stage3_ice_m(state: &DirectSnowStage3PersistentState) -> Result<f64, DirectRuntimeError> {
    let ice_m = state.layers.iter().map(|layer| layer.mass_swe_m).sum();
    validate_nonnegative_direct_m("stage3_publication.stage3_ice_m", ice_m)?;
    Ok(ice_m)
}

fn stage3_retained_liquid_m(
    state: &DirectSnowStage3PersistentState,
) -> Result<f64, DirectRuntimeError> {
    let retained_m = state
        .layers
        .iter()
        .map(|layer| layer.liquid_water_m)
        .sum::<f64>()
        + state.detached_retained_liquid_kg_m2 / KG_M2_PER_M_WATER;
    validate_nonnegative_direct_m("stage3_publication.stage3_retained_liquid_m", retained_m)?;
    Ok(retained_m)
}

const fn no_active_snow_settle_day_count() -> f64 {
    0.0
}

const fn accepted_stage3_closed_residual_m() -> f64 {
    0.0
}

fn canonical_stage3_terminal_liquid_temperature_k(temperature_k: f64) -> f64 {
    let reference_temperature_k = 273.15_f64;
    let reference_next_up_k = f64::from_bits(reference_temperature_k.to_bits().saturating_add(1));
    if temperature_k.to_bits() == reference_next_up_k.to_bits() {
        reference_temperature_k
    } else {
        temperature_k
    }
}

fn accepted_meltwater_phase_reference_c(
    routed_liquid_m: f64,
    custody_temperature_k_mass: f64,
) -> Result<Option<TemperatureCelsius>, DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "stage3_publication.snow_terminal_custody_mass",
        routed_liquid_m,
    )?;
    validate_finite(
        "stage3_publication.snow_terminal_custody_temperature_mass",
        custody_temperature_k_mass,
    )?;

    if routed_liquid_m.to_bits() == 0.0_f64.to_bits() {
        if custody_temperature_k_mass.to_bits() != 0.0_f64.to_bits() {
            return Err(stage3_publication_guard(
                "zero snow-terminal custody with temperature mass",
            ));
        }
        return Ok(None);
    }

    // Validate the independently retained bulk custody property before
    // projecting the distinct phase-reference diagnostic. This rejects an
    // omitted, substituted, or nonphysical temperature-mass operand without
    // changing parcel mass or enthalpy custody.
    let custody_temperature_k = custody_temperature_k_mass / routed_liquid_m;
    if !custody_temperature_k.is_finite() || !(200.0..=350.0).contains(&custody_temperature_k) {
        return Err(stage3_publication_guard(
            "accepted snow-terminal custody temperature domain",
        ));
    }

    if routed_liquid_m <= crate::constants::WB11_ZERO_THRESHOLD {
        return Ok(None);
    }

    let phase_reference_k = canonical_stage3_terminal_liquid_temperature_k(273.15_f64);
    let phase_reference_c = TemperatureCelsius::try_new(phase_reference_k - 273.15_f64)
        .map_err(|_| stage3_publication_guard("accepted snow-terminal phase reference"))?;
    Ok(Some(phase_reference_c))
}

const fn accepted_terminal_liquid_already_in_ingress_m() -> f64 {
    0.0
}

fn distribute_receipt_to_hours(
    hourly: &mut [f64; 24],
    day_start_ns: u128,
    support: openwepp_coupled_time::TimeSupport,
    receipt_start_s: f64,
    receipt_end_s: f64,
    amount_m: f64,
) -> Result<(), DirectRuntimeError> {
    let support_offset_s = (support.start_ns().get() - day_start_ns) as f64 / 1.0e9;
    let start_s = support_offset_s + receipt_start_s;
    let end_s = support_offset_s + receipt_end_s;
    let duration_s = end_s - start_s;
    if !start_s.is_finite()
        || !end_s.is_finite()
        || start_s < 0.0
        || end_s > 86_400.0
        || duration_s <= 0.0
    {
        return Err(stage3_publication_guard("accepted runoff receipt timing"));
    }
    for (hour, slot) in hourly.iter_mut().enumerate() {
        let hour_start = hour as f64 * 3_600.0;
        let hour_end = hour_start + 3_600.0;
        let overlap_s = end_s.min(hour_end) - start_s.max(hour_start);
        if overlap_s > 0.0 {
            add_nonnegative(slot, amount_m * overlap_s / duration_s)?;
        }
    }
    Ok(())
}

fn commit_stage3_downstream_day(
    frame: &mut DirectRunFrame,
    day: &DirectDayFrame,
) -> Result<(), DirectRuntimeError> {
    let lane_count = frame.lanes.len();
    let lane =
        frame
            .lanes
            .get_mut(day.lane_index)
            .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                lane_index: day.lane_index,
                lane_count,
            })?;
    lane.water = day.water.clone();
    lane.water.soil_water_m = day.storage_reconciliation.storage_reconciled_m;
    lane.subsurface_layers = day
        .evapotranspiration_compute
        .layer_state_after_root_uptake
        .clone();
    lane.publication = day.publication.clone();
    lane.erosion_downstream_operands = day.erosion_downstream_operands.clone();
    lane.erosion_runtime_carry = day.erosion_runtime_carry;
    lane.winter_column.clone_from(&day.winter_column);
    lane.snow_runtime_carry =
        direct_snow_runtime_carry_from_winter_state(&lane.winter_column.snow).map(Box::new);
    Ok(())
}

fn committed_day_receipt(
    day_index: usize,
    beginning_owner: Digest32,
    ending_owner: Digest32,
    support_set: Digest32,
    frames: &[DirectDayFrame],
    temperatures: &[Option<f64>],
) -> Result<Digest32, DirectRuntimeError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_ACCEPTED_PUBLICATION_DAY_V1\0");
    bytes.extend_from_slice(
        &u64::try_from(day_index)
            .map_err(|_| stage3_publication_guard("receipt day index width"))?
            .to_be_bytes(),
    );
    bytes.extend_from_slice(beginning_owner.as_bytes());
    bytes.extend_from_slice(ending_owner.as_bytes());
    bytes.extend_from_slice(support_set.as_bytes());
    for (frame, temperature) in frames.iter().zip(temperatures) {
        bytes.extend_from_slice(
            &u64::try_from(frame.lane_index)
                .map_err(|_| stage3_publication_guard("receipt lane index width"))?
                .to_be_bytes(),
        );
        for value in [
            frame.hydrology_projection.q_runoff_m,
            frame.hydrology_projection.evapotranspiration_m,
            frame.hydrology_projection.deep_percolation_m,
            frame.hydrology_projection.lateral_flow_m,
            frame.hydrology_projection.tile_drainage_m,
            frame.hydrology_projection.total_soil_m,
            frame.hydrology_projection.snow_water_m,
            temperature.unwrap_or(f64::NAN),
        ] {
            bytes.extend_from_slice(&value.to_bits().to_be_bytes());
        }
    }
    Ok(digest_bytes(&bytes))
}

include!("stage3_committed_publication_numeric_helpers.rs");

#[cfg(test)]
#[path = "stage3_committed_publication_tests.rs"]
mod tests;
