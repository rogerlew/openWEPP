const CANONICAL_COVERED_MAX_AUTHENTIC_MAPS: u8 = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)] // Tightened/loosened scales are exact-boundary test probes.
enum CanonicalCoveredToleranceScaleV1 {
    Tightened10x,
    Nominal,
    Loosened2x,
}

impl CanonicalCoveredToleranceScaleV1 {
    const fn multiplier(self) -> f64 {
        match self {
            Self::Tightened10x => 0.1,
            Self::Nominal => 1.0,
            Self::Loosened2x => 2.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CanonicalCoveredToleranceClassV1 {
    SnowTemperature,
    TopSoilTemperature,
    HeatFlux,
    VaporFlux,
    SnowWater,
    Energy,
    Density,
    Thickness,
    SpecificHumidity,
}

impl CanonicalCoveredToleranceClassV1 {
    const ALL: [Self; 9] = [
        Self::SnowTemperature,
        Self::TopSoilTemperature,
        Self::HeatFlux,
        Self::VaporFlux,
        Self::SnowWater,
        Self::Energy,
        Self::Density,
        Self::Thickness,
        Self::SpecificHumidity,
    ];

    const fn tolerance(self) -> (f64, f64) {
        match self {
            Self::SnowTemperature => (1.0e-5, 1.0e-9),
            Self::TopSoilTemperature => (1.0e-8, 0.0),
            Self::HeatFlux => (1.0e-5, 1.0e-8),
            Self::VaporFlux => (1.0e-10, 1.0e-6),
            Self::SnowWater => (1.0e-6, 1.0e-9),
            Self::Energy => (1.0e-6, 1.0e-10),
            Self::Density => (1.0e-6, 0.0),
            Self::Thickness => (1.0e-9, 1.0e-9),
            Self::SpecificHumidity => (1.0e-12, 1.0e-8),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CanonicalCoveredMapRoleV1 {
    Initial,
    FixedPointAdjudication,
    MultisecantAdjudication(u8),
}

#[cfg(test)]
include!("canonical_covered_solver_test_audit.rs");

#[cfg(not(test))]
fn canonical_covered_audit_charge_v1(_: CanonicalCoveredMapRoleV1, _: u32) {
    crate::snow_stage3_v11_attachment::record_release_qualification_canonical_covered_map_v1();
}

fn canonical_covered_role_ordinal_valid_v1(role: CanonicalCoveredMapRoleV1, ordinal: u32) -> bool {
    match role {
        CanonicalCoveredMapRoleV1::Initial => ordinal == 0,
        CanonicalCoveredMapRoleV1::FixedPointAdjudication => ordinal == 1,
        CanonicalCoveredMapRoleV1::MultisecantAdjudication(trial) => {
            (1..=5).contains(&trial) && ordinal == u32::from(trial) + 1
        }
    }
}

std::thread_local! {
    static CANONICAL_TOLERANCE_SCALE: std::cell::Cell<CanonicalCoveredToleranceScaleV1> =
        const { std::cell::Cell::new(CanonicalCoveredToleranceScaleV1::Nominal) };
}

fn take_canonical_covered_tolerance_scale_v1() -> CanonicalCoveredToleranceScaleV1 {
    CANONICAL_TOLERANCE_SCALE.with(|slot| {
        let value = slot.get();
        slot.set(CanonicalCoveredToleranceScaleV1::Nominal);
        value
    })
}

fn canonical_total_snow_water_kg_m2(owner: &DirectSnowStage3PersistentState) -> f64 {
    owner
        .layers
        .iter()
        .map(|layer| 1_000.0 * (layer.mass_swe_m + layer.liquid_water_m))
        .sum::<f64>()
        + owner.detached_retained_liquid_kg_m2
}

trait CanonicalStage3OpenSnowExecutionV1<I, C, V> {
    type Output;
    type Error;

    fn execute_canonical_stage3_open_snow(
        &mut self,
        input: I,
        evaluate_covered: C,
        execute_thin_pack_v22: V,
    ) -> Result<Self::Output, Self::Error>;
}

struct CanonicalCoveredPhysicalEndpointV1 {
    role: CanonicalCoveredMapRoleV1,
    ending_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    density_model_by_lane: BTreeMap<u32, crate::SnowDensityModel>,
    lane_coordinates: BTreeMap<u32, [f64; 9]>,
    diagnostics: BTreeMap<u32, (f64, f64, f64, f64)>,
}

struct CanonicalCoveredIterationMapV1 {
    endpoint: CanonicalCoveredPhysicalEndpointV1,
    _physical: CoveredCarrierPhysicalPhaseResultV1,
}

struct CanonicalCoveredPendingAdjudicationMapV1 {
    endpoint: CanonicalCoveredPhysicalEndpointV1,
    physical: CoveredCarrierPhysicalPhaseResultV1,
    completion_child: CoveredProbeChildIdentityV1,
    request: CoveredTerminalBatchTrialRequestV2,
}

impl std::ops::Deref for CanonicalCoveredPendingAdjudicationMapV1 {
    type Target = CanonicalCoveredPhysicalEndpointV1;

    fn deref(&self) -> &Self::Target {
        &self.endpoint
    }
}

impl std::ops::Deref for CanonicalCoveredIterationMapV1 {
    type Target = CanonicalCoveredPhysicalEndpointV1;

    fn deref(&self) -> &Self::Target {
        &self.endpoint
    }
}

struct CanonicalCoveredFinalMapV1 {
    endpoint: CanonicalCoveredPhysicalEndpointV1,
    phase: CoveredCarrierPhaseResultV1,
}

impl std::ops::Deref for CanonicalCoveredFinalMapV1 {
    type Target = CanonicalCoveredPhysicalEndpointV1;

    fn deref(&self) -> &Self::Target {
        &self.endpoint
    }
}

trait CanonicalCoveredCarrierReadViewV1 {
    fn precipitation_sets(&self) -> &BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>;
    fn batch_boundaries(&self) -> &BTreeMap<u32, Stage3SnowSurfaceBoundaryReceiptV1>;
    fn covered_lse_states(&self) -> &BTreeMap<(OfeId, TileId), CoveredLseIterationState>;
    fn soil_candidate(&self) -> &DirectSoilThermalCandidate;
    fn complete_lower_boundaries(
        &self,
    ) -> &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>;
    #[cfg(test)]
    fn is_stage3_covered_native(&self) -> bool;
    #[cfg(test)]
    fn native_inactive_projection_v1(
        &self,
    ) -> Result<
        Option<
            crate::v9_real_consumer_shadow::frozen_litter_v4_adoption::
                CoveredNativeInactiveProjectionSnapshotV1,
        >,
        DirectV11RealConsumerError,
    >;
}

impl CanonicalCoveredCarrierReadViewV1 for CoveredCarrierPhysicalPhaseResultV1 {
    fn precipitation_sets(&self) -> &BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1> {
        &self.precipitation_sets
    }

    fn batch_boundaries(&self) -> &BTreeMap<u32, Stage3SnowSurfaceBoundaryReceiptV1> {
        &self.batch_boundaries_by_lane
    }

    fn covered_lse_states(&self) -> &BTreeMap<(OfeId, TileId), CoveredLseIterationState> {
        &self.covered_lse_states
    }

    fn soil_candidate(&self) -> &DirectSoilThermalCandidate {
        self.validated_soil_ending.candidate()
    }

    fn complete_lower_boundaries(
        &self,
    ) -> &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary> {
        &self.complete_lower_boundaries
    }

    #[cfg(test)]
    fn is_stage3_covered_native(&self) -> bool {
        matches!(
            self.native_finalization_posture,
            CoveredNativeFinalizationPostureV1::Stage3CoveredNative
        )
    }

    #[cfg(test)]
    fn native_inactive_projection_v1(
        &self,
    ) -> Result<
        Option<
            crate::v9_real_consumer_shadow::frozen_litter_v4_adoption::
                CoveredNativeInactiveProjectionSnapshotV1,
        >,
        DirectV11RealConsumerError,
    >{
        Ok(self
            .physical
            .native_inactive_projection_for_test()
            .map(|(v3_sha256, v4_sha256)| {
                crate::v9_real_consumer_shadow::frozen_litter_v4_adoption::
                    CoveredNativeInactiveProjectionSnapshotV1 {
                        v3_sha256,
                        v4_sha256,
                    }
            }))
    }
}

impl CanonicalCoveredCarrierReadViewV1 for CoveredCarrierPhaseResultV1 {
    fn precipitation_sets(&self) -> &BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1> {
        &self.precipitation_sets
    }

    fn batch_boundaries(&self) -> &BTreeMap<u32, Stage3SnowSurfaceBoundaryReceiptV1> {
        &self.batch_boundaries_by_lane
    }

    fn covered_lse_states(&self) -> &BTreeMap<(OfeId, TileId), CoveredLseIterationState> {
        &self.covered_lse_states
    }

    fn soil_candidate(&self) -> &DirectSoilThermalCandidate {
        &self.soil_candidate
    }

    fn complete_lower_boundaries(
        &self,
    ) -> &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary> {
        &self.complete_lower_boundaries
    }

    #[cfg(test)]
    fn is_stage3_covered_native(&self) -> bool {
        self.carrier_envelope
            .hydrology()
            .surface_ingress()
            .is_stage3_covered_native_inactive()
    }

    #[cfg(test)]
    fn native_inactive_projection_v1(
        &self,
    ) -> Result<
        Option<
            crate::v9_real_consumer_shadow::frozen_litter_v4_adoption::
                CoveredNativeInactiveProjectionSnapshotV1,
        >,
        DirectV11RealConsumerError,
    >{
        Ok(crate::v9_real_consumer_shadow::frozen_litter_v4_adoption::
            capture_represented_snow_inactive_projection_v1(self.ending_candidates.shadow())?)
    }
}

fn canonical_covered_lane_trial_v1(
    lane_id: u32,
    state: &DirectSnowStage3PersistentState,
    snow_density_model: crate::SnowDensityModel,
) -> Result<CoveredTerminalLaneTrialStateV2, DirectV11RealConsumerError> {
    Wb11HydrologyKernel::validate_stage3_persistent_state(state)
        .map_err(|_| DirectV11RealConsumerError::Identity("canonical covered Stage-3 proposal"))?;
    let surface = if crate::hydrology::stage3_is_terminal_event_domain(state) {
        Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(state)
    } else {
        Wb11HydrologyKernel::project_stage3_surface_state_v1(state)
    }
    .map_err(DirectV11RealConsumerError::Stage3)?;
    let ice_kg_m2 = state
        .layers
        .iter()
        .map(|layer| layer.mass_swe_m * 1_000.0)
        .sum::<f64>();
    let liquid_kg_m2 = state
        .layers
        .iter()
        .map(|layer| layer.liquid_water_m * 1_000.0)
        .sum::<f64>()
        + state.detached_retained_liquid_kg_m2;
    let cold_content_j_m2 = state
        .layers
        .iter()
        .map(|layer| layer.cold_content_j_m2)
        .sum::<f64>();
    let snow_depth_m = state
        .layers
        .iter()
        .map(|layer| layer.thickness_m)
        .sum::<f64>();
    let snow_density_kg_m3 = if snow_depth_m > 0.0 {
        ice_kg_m2 / snow_depth_m
    } else {
        100.0
    };
    let trial = CoveredTerminalLaneTrialStateV2 {
        lane_id,
        schema_version: state.schema_version,
        terminal_event_model: state.terminal_event_model,
        next_interval_index: state.next_interval_index,
        snow_density_model,
        ice_kg_m2,
        liquid_kg_m2,
        cold_content_j_m2,
        surface_temperature_c: surface.surface_temperature_k - 273.15,
        snow_depth_m,
        snow_density_kg_m3,
        layer_density_kg_m3: state
            .layers
            .iter()
            .map(|layer| layer.density_kg_m3)
            .collect(),
        layer_settle_day_count: state
            .layers
            .iter()
            .map(|layer| layer.settle_day_count)
            .collect(),
        represented_layers: state.layers.clone(),
        resolved_beginning: crate::hydrology::stage3_is_resolved_thermal_domain(state),
        candidate_event_tick: None,
    };
    if [
        trial.ice_kg_m2,
        trial.liquid_kg_m2,
        trial.cold_content_j_m2,
        trial.surface_temperature_c,
        trial.snow_depth_m,
        trial.snow_density_kg_m3,
    ]
    .into_iter()
    .any(|value| !value.is_finite())
    {
        return Err(DirectV11RealConsumerError::Identity(
            "canonical covered nonfinite proposal",
        ));
    }
    Ok(trial)
}

fn canonical_covered_outer_coordinate_converged_v1(
    candidate: f64,
    mapped: f64,
    class: CanonicalCoveredToleranceClassV1,
    scale: CanonicalCoveredToleranceScaleV1,
) -> bool {
    if !candidate.is_finite() || !mapped.is_finite() {
        return false;
    }
    let (absolute, relative) = class.tolerance();
    (candidate - mapped).abs()
        <= scale.multiplier() * (absolute + relative * candidate.abs().max(mapped.abs()))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CanonicalCoveredProductionConvergenceV1 {
    outer_coordinates: bool,
    dependent_carriers: bool,
}

fn canonical_covered_layer_branch_v1(layer: &crate::DirectSnowLayerState) -> (bool, bool, bool) {
    (
        layer.mass_swe_m > 0.0,
        layer.liquid_water_m > 0.0,
        layer.cold_content_j_m2 > 0.0 && layer.temperature_c < 0.0,
    )
}

fn canonical_covered_exact_proposal_envelope_v1(
    lane_id: u32,
    proposal: &CoveredTerminalLaneTrialStateV2,
    mapped: &DirectSnowStage3PersistentState,
    mapped_density_model: crate::SnowDensityModel,
) -> bool {
    proposal.lane_id == lane_id
        && proposal.schema_version == mapped.schema_version
        && proposal.terminal_event_model == mapped.terminal_event_model
        && proposal.next_interval_index == mapped.next_interval_index
        && proposal.snow_density_model == mapped_density_model
        && proposal.resolved_beginning
            == crate::hydrology::stage3_is_resolved_thermal_domain(mapped)
        && proposal.candidate_event_tick.is_none()
        && proposal.represented_layers.len() == mapped.layers.len()
        && proposal.layer_density_kg_m3.len() == mapped.layers.len()
        && proposal.layer_settle_day_count.len() == mapped.layers.len()
        && proposal
            .represented_layers
            .iter()
            .zip(&mapped.layers)
            .zip(&proposal.layer_density_kg_m3)
            .zip(&proposal.layer_settle_day_count)
            .all(|(((template, mapped), density), settle)| {
                canonical_covered_layer_branch_v1(template)
                    == canonical_covered_layer_branch_v1(mapped)
                    && template.density_kg_m3.to_bits() == density.to_bits()
                    && template.settle_day_count.to_bits() == settle.to_bits()
                    && settle.to_bits() == mapped.settle_day_count.to_bits()
            })
}

fn canonical_covered_production_converged_v1<P, C>(
    previous: &P,
    current: &C,
    current_proposal: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    scale: CanonicalCoveredToleranceScaleV1,
) -> Result<CanonicalCoveredProductionConvergenceV1, DirectV11RealConsumerError>
where
    P: std::ops::Deref<Target = CanonicalCoveredPhysicalEndpointV1>,
    C: std::ops::Deref<Target = CanonicalCoveredPhysicalEndpointV1>,
{
    if previous
        .lane_coordinates
        .keys()
        .ne(current.lane_coordinates.keys())
        || previous
            .ending_stage3
            .keys()
            .ne(current.ending_stage3.keys())
        || previous
            .lane_coordinates
            .keys()
            .ne(previous.ending_stage3.keys())
        || current.lane_coordinates.keys().ne(current_proposal.keys())
    {
        return Err(DirectV11RealConsumerError::Identity(
            "canonical covered coordinate topology",
        ));
    }
    let mut outer_coordinates = true;
    let mut dependent_carriers = true;
    for (lane_id, current_values) in &current.lane_coordinates {
        let previous_values =
            previous
                .lane_coordinates
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical covered coordinate lane",
                ))?;
        let current_state =
            current
                .ending_stage3
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical covered current state lane",
                ))?;
        let proposal =
            current_proposal
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical covered proposal lane",
                ))?;
        let mapped_density_model = current.density_model_by_lane.get(lane_id).copied().ok_or(
            DirectV11RealConsumerError::Identity("canonical covered mapped density-model lane"),
        )?;
        if !canonical_covered_exact_proposal_envelope_v1(
            *lane_id,
            proposal,
            current_state,
            mapped_density_model,
        ) {
            outer_coordinates = false;
        } else {
            for (layer_index, current_layer) in current_state.layers.iter().enumerate() {
                // Settling chronology is a non-iterated exact envelope
                // coordinate.  Compare charged x_k against fresh F(x_k), not
                // two successive mapped endpoints.
                let settling_chronology_exact = proposal.layer_settle_day_count[layer_index]
                    .to_bits()
                    == current_layer.settle_day_count.to_bits();
                outer_coordinates &= settling_chronology_exact;
                let layer_density_converged = canonical_covered_outer_coordinate_converged_v1(
                    proposal.layer_density_kg_m3[layer_index],
                    current_layer.density_kg_m3,
                    CanonicalCoveredToleranceClassV1::Density,
                    scale,
                );
                outer_coordinates &= layer_density_converged;
            }
        }

        let outer_candidates = [
            proposal.surface_temperature_c + 273.15,
            proposal.ice_kg_m2 + proposal.liquid_kg_m2,
            proposal.cold_content_j_m2,
            proposal.snow_density_kg_m3,
            proposal.snow_depth_m,
        ];
        for (candidate, index, class) in [
            (
                outer_candidates[0],
                0,
                CanonicalCoveredToleranceClassV1::SnowTemperature,
            ),
            (
                outer_candidates[1],
                4,
                CanonicalCoveredToleranceClassV1::SnowWater,
            ),
            (
                outer_candidates[2],
                5,
                CanonicalCoveredToleranceClassV1::Energy,
            ),
            (
                outer_candidates[3],
                6,
                CanonicalCoveredToleranceClassV1::Density,
            ),
            (
                outer_candidates[4],
                7,
                CanonicalCoveredToleranceClassV1::Thickness,
            ),
        ] {
            outer_coordinates &= canonical_covered_outer_coordinate_converged_v1(
                candidate,
                current_values[index],
                class,
                scale,
            );
        }
        for index in [1_usize, 2, 3, 8] {
            dependent_carriers &= canonical_covered_outer_coordinate_converged_v1(
                previous_values[index],
                current_values[index],
                CanonicalCoveredToleranceClassV1::ALL[index],
                scale,
            );
        }
    }
    Ok(CanonicalCoveredProductionConvergenceV1 {
        outer_coordinates,
        dependent_carriers,
    })
}

fn canonical_covered_multisecant_alpha_v1(
    dot: f64,
    norm: f64,
) -> Result<f64, DirectV11RealConsumerError> {
    if !dot.is_finite() || !norm.is_finite() || norm <= 0.0 {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "canonical covered degenerate multisecant history",
        ));
    }
    let raw_alpha = -dot / norm;
    if !raw_alpha.is_finite() || raw_alpha == 0.0 {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "canonical covered nonadvancing multisecant history",
        ));
    }
    // A negative coefficient is the canonical convex contraction between the
    // two already charged authentic map outputs. Saturating at -0.75 keeps a
    // strict new point even when the unconstrained minimum is at or beyond the
    // previous endpoint. Positive extrapolation is bounded independently.
    Ok(raw_alpha.clamp(-0.75, 1.0))
}

fn canonical_covered_lane_trial_exact_eq_v1(
    left: &CoveredTerminalLaneTrialStateV2,
    right: &CoveredTerminalLaneTrialStateV2,
) -> bool {
    left.lane_id == right.lane_id
        && left.schema_version == right.schema_version
        && left.terminal_event_model == right.terminal_event_model
        && left.next_interval_index == right.next_interval_index
        && left.snow_density_model == right.snow_density_model
        && left.ice_kg_m2.to_bits() == right.ice_kg_m2.to_bits()
        && left.liquid_kg_m2.to_bits() == right.liquid_kg_m2.to_bits()
        && left.cold_content_j_m2.to_bits() == right.cold_content_j_m2.to_bits()
        && left.surface_temperature_c.to_bits() == right.surface_temperature_c.to_bits()
        && left.snow_depth_m.to_bits() == right.snow_depth_m.to_bits()
        && left.snow_density_kg_m3.to_bits() == right.snow_density_kg_m3.to_bits()
        && left
            .layer_density_kg_m3
            .iter()
            .map(|value| value.to_bits())
            .eq(right
                .layer_density_kg_m3
                .iter()
                .map(|value| value.to_bits()))
        && left
            .layer_settle_day_count
            .iter()
            .map(|value| value.to_bits())
            .eq(right
                .layer_settle_day_count
                .iter()
                .map(|value| value.to_bits()))
        && left.represented_layers == right.represented_layers
        && left.resolved_beginning == right.resolved_beginning
        && left.candidate_event_tick == right.candidate_event_tick
}

fn canonical_covered_lane_trial_maps_exact_eq_v1(
    left: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    right: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
) -> bool {
    left.len() == right.len()
        && left.iter().all(|(lane_id, lane)| {
            right
                .get(lane_id)
                .is_some_and(|other| canonical_covered_lane_trial_exact_eq_v1(lane, other))
        })
}

fn canonical_covered_multisecant_coordinate_v1(previous: f64, current: f64, alpha: f64) -> f64 {
    if alpha.to_bits() == (-0.75_f64).to_bits() {
        0.25 * current + 0.75 * previous
    } else {
        current + alpha * (current - previous)
    }
}

fn canonical_covered_phase_split_v1(
    total_water_kg_m2: f64,
    cold_content_j_m2: f64,
    current_authentic_ice_kg_m2: f64,
) -> Result<(f64, f64), DirectV11RealConsumerError> {
    if !total_water_kg_m2.is_finite()
        || !cold_content_j_m2.is_finite()
        || !current_authentic_ice_kg_m2.is_finite()
        || total_water_kg_m2 < 0.0
        || cold_content_j_m2 < 0.0
        || current_authentic_ice_kg_m2 < 0.0
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "canonical covered multisecant proposal domain",
        ));
    }
    if cold_content_j_m2 > 0.0 {
        // Version-22 Pi(W,H), cold branch: H=-C<0, so every represented
        // water unit is ice and liquid is exact positive zero. Carrying ice
        // from another authentic endpoint would combine two phase images and
        // can construct the impossible W<I posture at snow reappearance.
        return Ok((total_water_kg_m2, 0.0));
    }
    if total_water_kg_m2 < current_authentic_ice_kg_m2 {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "canonical covered multisecant proposal domain",
        ));
    }
    Ok((
        current_authentic_ice_kg_m2,
        total_water_kg_m2 - current_authentic_ice_kg_m2,
    ))
}

fn canonical_covered_multisecant_lane_proposal_valid_v1(
    lane_id: u32,
    lane: &CoveredTerminalLaneTrialStateV2,
) -> bool {
    let scalar_domain = [
        lane.ice_kg_m2,
        lane.liquid_kg_m2,
        lane.cold_content_j_m2,
        lane.surface_temperature_c,
        lane.snow_depth_m,
        lane.snow_density_kg_m3,
    ]
    .into_iter()
    .all(f64::is_finite)
        && lane.ice_kg_m2 >= 0.0
        && lane.liquid_kg_m2 >= 0.0
        && lane.cold_content_j_m2 >= 0.0
        && lane.snow_depth_m >= 0.0
        && lane.snow_density_kg_m3 > 0.0
        && (lane.ice_kg_m2 <= 0.0 || lane.snow_depth_m > 0.0)
        && (!lane.resolved_beginning || lane.snow_depth_m > 0.0);
    let derived_density = if lane.snow_depth_m > 0.0 {
        lane.ice_kg_m2 / lane.snow_depth_m
    } else {
        100.0
    };
    let layer_count = lane.represented_layers.len();
    let layer_coordinates_valid = lane.layer_density_kg_m3.len() == layer_count
        && lane.layer_settle_day_count.len() == layer_count
        && lane
            .layer_density_kg_m3
            .iter()
            .all(|value| value.is_finite() && *value > 0.0)
        && lane
            .layer_settle_day_count
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0);
    let phase = openwepp_kernel_contract::HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
    let scalar_domain_probe = crate::DirectSnowLayerState {
        mass_swe_m: 0.0,
        thickness_m: 0.0,
        density_kg_m3: lane.snow_density_kg_m3,
        settle_day_count: 0.0,
        temperature_c: lane.surface_temperature_c,
        liquid_water_m: 0.0,
        cold_content_j_m2: 0.0,
        refrozen_liquid_m: 0.0,
    };
    let authoritative_scalar_domain =
        Wb11HydrologyKernel::validate_stage3_layer(phase, &scalar_domain_probe).is_ok();
    let represented_layers_valid = lane
        .represented_layers
        .iter()
        .zip(&lane.layer_density_kg_m3)
        .zip(&lane.layer_settle_day_count)
        .all(|((represented, density), settle_day_count)| {
            let mut candidate = *represented;
            candidate.density_kg_m3 = *density;
            candidate.settle_day_count = *settle_day_count;
            Wb11HydrologyKernel::validate_stage3_layer(phase, &candidate).is_ok()
        });
    lane.lane_id == lane_id
        && lane.schema_version > 0
        && lane.candidate_event_tick.is_none()
        && scalar_domain
        && derived_density.is_finite()
        && derived_density > 0.0
        && derived_density.to_bits() == lane.snow_density_kg_m3.to_bits()
        && layer_coordinates_valid
        && authoritative_scalar_domain
        && represented_layers_valid
}

fn canonical_covered_validate_complete_proposal_before_charge_v1(
    proposal: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
) -> Result<(), DirectV11RealConsumerError> {
    if proposal.is_empty()
        || proposal.iter().any(|(lane_id, lane)| {
            !canonical_covered_multisecant_lane_proposal_valid_v1(*lane_id, lane)
        })
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "canonical covered complete proposal domain before charge",
        ));
    }
    Ok(())
}

fn canonical_covered_multisecant_trial_v1(
    previous_proposal: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    previous_map: &CanonicalCoveredIterationMapV1,
    current_proposal: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    current_map: &CanonicalCoveredIterationMapV1,
    tolerance_scale: CanonicalCoveredToleranceScaleV1,
) -> Result<BTreeMap<u32, CoveredTerminalLaneTrialStateV2>, DirectV11RealConsumerError> {
    let proposal_values = |lane: &CoveredTerminalLaneTrialStateV2| {
        [
            lane.surface_temperature_c + 273.15,
            lane.ice_kg_m2 + lane.liquid_kg_m2,
            lane.cold_content_j_m2,
            lane.snow_density_kg_m3,
            lane.snow_depth_m,
        ]
    };
    let mapped_values = |values: &[f64; 9]| [values[0], values[4], values[5], values[6], values[7]];
    let classes = [
        CanonicalCoveredToleranceClassV1::SnowTemperature,
        CanonicalCoveredToleranceClassV1::SnowWater,
        CanonicalCoveredToleranceClassV1::Energy,
        CanonicalCoveredToleranceClassV1::Density,
        CanonicalCoveredToleranceClassV1::Thickness,
    ];
    let mut dot = 0.0;
    let mut norm = 0.0;
    for (lane_id, current) in current_proposal {
        let previous =
            previous_proposal
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical multisecant proposal topology",
                ))?;
        let previous_mapped = mapped_values(previous_map.lane_coordinates.get(lane_id).ok_or(
            DirectV11RealConsumerError::Identity("canonical multisecant previous map"),
        )?);
        let current_mapped = mapped_values(current_map.lane_coordinates.get(lane_id).ok_or(
            DirectV11RealConsumerError::Identity("canonical multisecant current map"),
        )?);
        let previous_values = proposal_values(previous);
        let current_values = proposal_values(current);
        for index in 0..5 {
            let (absolute, relative) = classes[index].tolerance();
            let magnitude = previous_values[index]
                .abs()
                .max(previous_mapped[index].abs())
                .max(current_values[index].abs())
                .max(current_mapped[index].abs());
            let scale = tolerance_scale.multiplier() * (absolute + relative * magnitude);
            if !scale.is_finite() || scale <= 0.0 {
                return Err(DirectV11RealConsumerError::Identity(
                    "canonical multisecant coordinate scale",
                ));
            }
            let previous_residual = (previous_mapped[index] - previous_values[index]) / scale;
            let current_residual = (current_mapped[index] - current_values[index]) / scale;
            let delta = current_residual - previous_residual;
            dot += current_residual * delta;
            norm += delta * delta;
        }
        let previous_layers =
            previous_map
                .ending_stage3
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical multisecant previous layer map",
                ))?;
        let current_layers =
            current_map
                .ending_stage3
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical multisecant current layer map",
                ))?;
        if current.layer_density_kg_m3.len() != current_layers.layers.len()
            || previous_layers.layers.len() != current_layers.layers.len()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "canonical multisecant layer-density topology",
            ));
        }
        // A map-authorized layer lifecycle can change topology on the first
        // image. Do not invent a density coordinate for the missing charged
        // layer. Once both charged endpoints share topology, every layer is
        // included in the multisecant residual.
        if previous.layer_density_kg_m3.len() == previous_layers.layers.len() {
            let (absolute, relative) = CanonicalCoveredToleranceClassV1::Density.tolerance();
            for index in 0..current_layers.layers.len() {
                let previous_mapped = previous_layers.layers[index].density_kg_m3;
                let current_mapped = current_layers.layers[index].density_kg_m3;
                let previous_candidate = previous.layer_density_kg_m3[index];
                let current_candidate = current.layer_density_kg_m3[index];
                let magnitude = previous_candidate
                    .abs()
                    .max(previous_mapped.abs())
                    .max(current_candidate.abs())
                    .max(current_mapped.abs());
                let coordinate_scale =
                    tolerance_scale.multiplier() * (absolute + relative * magnitude);
                if !coordinate_scale.is_finite() || coordinate_scale <= 0.0 {
                    return Err(DirectV11RealConsumerError::Identity(
                        "canonical multisecant layer-density scale",
                    ));
                }
                let previous_residual = (previous_mapped - previous_candidate) / coordinate_scale;
                let current_residual = (current_mapped - current_candidate) / coordinate_scale;
                let delta = current_residual - previous_residual;
                dot += current_residual * delta;
                norm += delta * delta;
            }
        }
    }
    let alpha = canonical_covered_multisecant_alpha_v1(dot, norm)?;
    let proposal = current_proposal
        .iter()
        .map(|(lane_id, current)| {
            let previous_values = previous_map.lane_coordinates.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical multisecant previous lane"),
            )?;
            let current_values = current_map.lane_coordinates.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical multisecant current lane"),
            )?;
            let interpolate = |index: usize| {
                canonical_covered_multisecant_coordinate_v1(
                    previous_values[index],
                    current_values[index],
                    alpha,
                )
            };
            let current_authentic_ice_kg_m2 = current_map
                .ending_stage3
                .get(lane_id)
                .map(|state| {
                    state
                        .layers
                        .iter()
                        .map(|layer| layer.mass_swe_m * 1_000.0)
                        .sum::<f64>()
                })
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical multisecant ending lane",
                ))?;
            let previous_layers = previous_map.ending_stage3.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical multisecant previous layers"),
            )?;
            let current_layers = current_map.ending_stage3.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical multisecant current layers"),
            )?;
            if previous_layers.layers.len() != current_layers.layers.len() {
                return Err(DirectV11RealConsumerError::Identity(
                    "canonical multisecant layer topology",
                ));
            }
            let layer_density_kg_m3 = previous_layers
                .layers
                .iter()
                .zip(&current_layers.layers)
                .map(|(previous, current)| {
                    canonical_covered_multisecant_coordinate_v1(
                        previous.density_kg_m3,
                        current.density_kg_m3,
                        alpha,
                    )
                })
                .collect::<Vec<_>>();
            let layer_settle_day_count = current_layers
                .layers
                .iter()
                .map(|layer| layer.settle_day_count)
                .collect::<Vec<_>>();
            if layer_density_kg_m3
                .iter()
                .any(|density| !density.is_finite() || *density <= 0.0)
            {
                return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                    "canonical covered multisecant layer-density domain",
                ));
            }
            let total_water = interpolate(4);
            let cold_content_j_m2 = interpolate(5);
            let surface_temperature_k = interpolate(0);
            let (ice_kg_m2, liquid_kg_m2) = canonical_covered_phase_split_v1(
                total_water,
                cold_content_j_m2,
                current_authentic_ice_kg_m2,
            )?;
            if !total_water.is_finite()
                || !cold_content_j_m2.is_finite()
                || !surface_temperature_k.is_finite()
                || cold_content_j_m2 < 0.0
            {
                return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                    "canonical covered multisecant proposal domain",
                ));
            }
            let current_ice_m = current_authentic_ice_kg_m2 / 1_000.0;
            let projected_depth_m = if ice_kg_m2 > 0.0 {
                if current_ice_m <= 0.0 || current_layers.layers.is_empty() {
                    return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                        "canonical covered multisecant layer-mass domain",
                    ));
                }
                current_layers
                    .layers
                    .iter()
                    .zip(&layer_density_kg_m3)
                    .map(|(layer, density)| {
                        (ice_kg_m2 / 1_000.0) * (layer.mass_swe_m / current_ice_m) * 1_000.0
                            / density
                    })
                    .sum::<f64>()
            } else {
                0.0
            };
            #[cfg(test)]
            let mut projected_depth_m = projected_depth_m;
            #[cfg(test)]
            let proposal_poison = take_canonical_covered_multisecant_proposal_poison_v1();
            #[cfg(test)]
            let proposal_poisoned = proposal_poison.is_some();
            #[cfg(test)]
            if proposal_poison
                == Some(CanonicalCoveredMultisecantProposalPoisonV1::ExtremeFiniteDepth)
            {
                projected_depth_m = f64::MIN_POSITIVE;
            }
            if !projected_depth_m.is_finite()
                || projected_depth_m < 0.0
                || (ice_kg_m2 > 0.0 && projected_depth_m <= 0.0)
            {
                let error = DirectV11RealConsumerError::AdaptiveRefinement(
                    "canonical covered multisecant projected-depth domain",
                );
                #[cfg(test)]
                if proposal_poisoned {
                    return Err(canonical_covered_parity_rejection_v1(
                        error,
                        CanonicalCoveredRejectionStageV1::MultisecantProposal,
                    ));
                }
                return Err(error);
            }
            let density = if projected_depth_m > 0.0 {
                ice_kg_m2 / projected_depth_m
            } else {
                100.0
            };
            if !density.is_finite() || density <= 0.0 {
                let error = DirectV11RealConsumerError::AdaptiveRefinement(
                    "canonical covered multisecant bulk-density domain",
                );
                #[cfg(test)]
                if proposal_poisoned {
                    return Err(canonical_covered_parity_rejection_v1(
                        error,
                        CanonicalCoveredRejectionStageV1::MultisecantProposal,
                    ));
                }
                return Err(error);
            }
            let next = CoveredTerminalLaneTrialStateV2 {
                lane_id: *lane_id,
                schema_version: current.schema_version,
                terminal_event_model: current.terminal_event_model,
                next_interval_index: current.next_interval_index,
                snow_density_model: current.snow_density_model,
                ice_kg_m2,
                liquid_kg_m2,
                cold_content_j_m2,
                surface_temperature_c: surface_temperature_k - 273.15,
                snow_depth_m: projected_depth_m,
                snow_density_kg_m3: density,
                layer_density_kg_m3,
                layer_settle_day_count,
                represented_layers: current_layers.layers.clone(),
                resolved_beginning: current.resolved_beginning,
                candidate_event_tick: None,
            };
            #[cfg(test)]
            let mut next = next;
            #[cfg(test)]
            match proposal_poison {
                Some(CanonicalCoveredMultisecantProposalPoisonV1::FiniteDensityAboveCap) => {
                    let density = next.layer_density_kg_m3.first_mut().ok_or(
                        DirectV11RealConsumerError::Identity(
                            "canonical covered density-cap proposal poison layer",
                        ),
                    )?;
                    *density = f64::from_bits(522.0_f64.to_bits().saturating_add(1));
                }
                Some(
                    CanonicalCoveredMultisecantProposalPoisonV1::FiniteAboveFreezingTemperature,
                ) => {
                    next.surface_temperature_c = f64::MIN_POSITIVE;
                }
                Some(CanonicalCoveredMultisecantProposalPoisonV1::ExtremeFiniteDepth) | None => {}
            }
            if !canonical_covered_multisecant_lane_proposal_valid_v1(*lane_id, &next) {
                let error = DirectV11RealConsumerError::AdaptiveRefinement(
                    "canonical covered multisecant complete proposal domain",
                );
                #[cfg(test)]
                if proposal_poisoned {
                    return Err(canonical_covered_parity_rejection_v1(
                        error,
                        CanonicalCoveredRejectionStageV1::MultisecantProposal,
                    ));
                }
                return Err(error);
            }
            Ok((*lane_id, next))
        })
        .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
    if proposal.len() != current_proposal.len()
        || proposal.iter().any(|(lane_id, lane)| {
            !canonical_covered_multisecant_lane_proposal_valid_v1(*lane_id, lane)
        })
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "canonical covered multisecant complete proposal map domain",
        ));
    }
    let endpoint = |map: &CanonicalCoveredIterationMapV1,
                    template: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>| {
        map.ending_stage3
            .iter()
            .map(|(lane_id, state)| {
                let density_model = template
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "canonical multisecant exact-envelope lane",
                    ))?
                    .snow_density_model;
                Ok((
                    *lane_id,
                    canonical_covered_lane_trial_v1(*lane_id, state, density_model)?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()
    };
    let previous_endpoint = endpoint(previous_map, previous_proposal)?;
    let current_endpoint = endpoint(current_map, current_proposal)?;
    if canonical_covered_lane_trial_maps_exact_eq_v1(&proposal, &previous_endpoint)
        || canonical_covered_lane_trial_maps_exact_eq_v1(&proposal, &current_endpoint)
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "canonical covered repeated multisecant proposal",
        ));
    }
    Ok(proposal)
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    fn canonical_covered_initial_candidates_v1(
        &self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<CoveredCarrierEphemeralCandidatesV1, DirectV11RealConsumerError> {
        let mut owner_bytes = crate::v9_real_consumer_shadow::
            covered_carrier_initial_owner_bytes_with_deferred_native_v2_soil_custody_v1(
                &self.beginning,
                self.deferred_native_v2_soil_custody.as_ref(),
            )?;
        let snow = input.staged_resource_owners.get("snow").ok_or(
            DirectV11RealConsumerError::Identity("canonical covered staged snow owner"),
        )?;
        owner_bytes.insert("snow".to_owned(), snow.state_bytes.clone());
        let source_owner_set_sha256 = Digest32::from_bytes(
            self.wb14_coupled_child_binding
                .parent_beginning_complete_owner_set_sha256,
        );
        let source_snow_owner_sha256 = digest_bytes(owner_bytes.get("snow").ok_or(
            DirectV11RealConsumerError::Identity("canonical covered source snow owner"),
        )?);
        let (&leader_id, _) = self.stage3_beginning_by_lane.first_key_value().ok_or(
            DirectV11RealConsumerError::Identity("canonical covered empty lane topology"),
        )?;
        #[cfg(test)]
        let poison = canonical_covered_parity_poison_v1();
        #[cfg(test)]
        let poisoned_support = matches!(
            poison,
            Some(
                CanonicalCoveredPhysicalParityPoisonV1::Support
                    | CanonicalCoveredPhysicalParityPoisonV1::SupportAndPhysicalOneUlp
            )
        );
        #[cfg(test)]
        let state_support = if poisoned_support {
            TimeSupport::new(
                input.support.start_ns(),
                openwepp_coupled_time::ModelTimeNs::new(
                    input.support.end_ns().get().saturating_sub(1),
                ),
            )
            .map_err(|_| DirectV11RealConsumerError::Identity("poisoned support construction"))?
        } else {
            input.support
        };
        #[cfg(not(test))]
        let state_support = input.support;
        #[cfg(test)]
        let source_owner_set_sha256 =
            if poison == Some(CanonicalCoveredPhysicalParityPoisonV1::Transaction) {
                Digest32::zero()
            } else {
                source_owner_set_sha256
            };
        #[cfg(test)]
        let leader_id = if poison == Some(CanonicalCoveredPhysicalParityPoisonV1::Topology) {
            leader_id.saturating_add(10_000)
        } else {
            leader_id
        };
        #[cfg(test)]
        if poison == Some(CanonicalCoveredPhysicalParityPoisonV1::BeginningOwner) {
            owner_bytes
                .get_mut("snow")
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical covered poisoned beginning snow owner",
                ))?
                .push(0);
        }
        let identity =
            (|| {
                if state_support != input.support {
                    return Err(DirectV11RealConsumerError::Identity(
                        "canonical covered beginning support identity",
                    ));
                }
                if source_owner_set_sha256 == Digest32::zero() {
                    return Err(DirectV11RealConsumerError::Identity(
                        "canonical covered beginning owner-set identity",
                    ));
                }
                if digest_bytes(owner_bytes.get("snow").ok_or(
                    DirectV11RealConsumerError::Identity("canonical covered snow owner bytes"),
                )?) != source_snow_owner_sha256
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "canonical covered beginning snow identity",
                    ));
                }
                if !self.stage3_beginning_by_lane.contains_key(&leader_id) {
                    return Err(DirectV11RealConsumerError::Identity(
                        "canonical covered beginning topology identity",
                    ));
                }
                if self
                    .beginning
                    .validate_frozen_litter_v4_resident_pair_v1()
                    .is_err()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "canonical covered beginning native resident identity",
                    ));
                }
                Ok(())
            })();
        #[cfg(test)]
        identity.map_err(|error| {
            if poison.is_some() {
                canonical_covered_parity_rejection_v1(
                    error,
                    CanonicalCoveredRejectionStageV1::PreflightIdentity,
                )
            } else {
                error
            }
        })?;
        #[cfg(not(test))]
        identity?;
        let joint = CoveredTerminalJointTrialStateV1::try_new(
            crate::hydrology::JointTrialAuthorityV1 {
                source_owner_set_sha256,
                lane_id: leader_id,
                source_snow_owner_sha256,
                interval_index: u64::try_from(self.interval_index).map_err(|_| {
                    DirectV11RealConsumerError::Identity("canonical covered interval width")
                })?,
                state_support,
                accepted_predecessors: Vec::new(),
            },
            owner_bytes,
        )
        .map_err(DirectV11RealConsumerError::Stage3)?;
        match self.deferred_native_v2_soil_custody.as_ref() {
            Some(custody) => {
                CoveredCarrierEphemeralCandidatesV1::try_new_with_deferred_native_v2_soil_custody(
                    joint,
                    self.beginning.clone(),
                    self.stage3_beginning_by_lane.clone(),
                    custody,
                )
            }
            None => CoveredCarrierEphemeralCandidatesV1::try_new(
                joint,
                self.beginning.clone(),
                self.stage3_beginning_by_lane.clone(),
            ),
        }
    }

    fn canonical_covered_map_endpoint_v1<P>(
        &self,
        input: &V11ImportedV10SegmentInput,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        role: CanonicalCoveredMapRoleV1,
        attempt_ordinal: u32,
        proposal: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
        execute: impl FnOnce(
            &Self,
            &CoveredCarrierEphemeralCandidatesV1,
            &CoveredTerminalBatchTrialRequestV2,
            CoveredProbeChildIdentityV1,
        ) -> Result<P, DirectV11RealConsumerError>,
    ) -> Result<(CanonicalCoveredPhysicalEndpointV1, P), DirectV11RealConsumerError>
    where
        P: CanonicalCoveredCarrierReadViewV1,
    {
        if attempt_ordinal >= u32::from(CANONICAL_COVERED_MAX_AUTHENTIC_MAPS) {
            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                "canonical covered evaluation budget",
            ));
        }
        let request = CoveredTerminalBatchTrialRequestV2 {
            support: input.support,
            role: crate::hydrology::CoveredTerminalTrialRoleV1::Root,
            attempt_ordinal,
            lanes: proposal.clone(),
            beginning_joint: beginning.joint().clone(),
        };
        let owner_states = beginning
            .joint()
            .owner_bytes()
            .iter()
            .map(|(owner_id, bytes)| {
                openwepp_coupled_time::OwnerState::new(owner_id.clone(), bytes.clone())
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("canonical covered beginning owner frame")
            })?;
        let beginning_owner_set_sha256 =
            complete_owner_set_digest(&owner_states).map_err(|_| {
                DirectV11RealConsumerError::Identity("canonical covered beginning owner set")
            })?;
        let parent_support = TimeSupport::new(
            openwepp_coupled_time::ModelTimeNs::new(
                self.wb14_coupled_child_binding.parent_support_start_ns,
            ),
            openwepp_coupled_time::ModelTimeNs::new(
                self.wb14_coupled_child_binding.parent_support_end_ns,
            ),
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("canonical covered parent support"))?;
        let complete_forcing_sha256 = digest_bytes(
            self.interval
                .lse_forcing
                .canonical_sha256()
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(
                        error,
                    ))
                })?
                .as_str()
                .as_bytes(),
        );
        let topology_sha256 = digest_bytes(
            self.beginning
                .inner
                .surface_configuration
                .configuration_sha256
                .as_bytes(),
        );
        let child = CoveredProbeChildIdentityV1::try_new(crate::hydrology::ProbeChildAuthorityV1 {
            parent_transaction_sha256: input.parent_transaction_id.digest(),
            enclosing_parent_support: parent_support,
            trial_support: input.support,
            physical_child_ordinal: 0,
            attempt_ordinal,
            role: request.role,
            beginning_joint_sha256: beginning.joint().receipt_sha256(),
            beginning_owner_set_sha256,
            complete_forcing_sha256,
            topology_sha256,
        })
        .map_err(DirectV11RealConsumerError::Stage3)?;
        #[cfg(test)]
        let inactive_beginning =
            crate::v9_real_consumer_shadow::frozen_litter_v4_adoption::
                capture_represented_snow_inactive_projection_v1(beginning.shadow())?;
        let phase = execute(self, beginning, &request, child)?;
        #[cfg(test)]
        {
            let inactive_ending = phase.native_inactive_projection_v1()?;
            crate::v9_real_consumer_shadow::frozen_litter_v4_adoption::
                record_represented_snow_map_retention_v1(
                    inactive_beginning,
                    inactive_ending,
                    phase.is_stage3_covered_native(),
                )?;
        }
        if phase.complete_lower_boundaries().is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "canonical covered empty physical boundary custody",
            ));
        }
        let mut ending_stage3 = BTreeMap::new();
        let mut diagnostics = BTreeMap::new();
        #[cfg(test)]
        let mut stage3_refreeze_by_lane = BTreeMap::new();
        for (lane_id, boundary) in phase.batch_boundaries() {
            let immutable = self.stage3_beginning_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical covered immutable lane"),
            )?;
            let stage3_inputs = self.stage3_inputs_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical covered Stage-3 input lane"),
            )?;
            let mut forcing = *self.stage3_forcing_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical covered Stage-3 forcing lane"),
            )?;
            let precipitation = phase.precipitation_sets().get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical covered precipitation lane"),
            )?;
            let mut liquid = 0.0;
            let mut solid = 0.0;
            for parcel in &precipitation.parcels {
                let fraction = precipitation
                    .destinations
                    .get(parcel.destination_topology_index as usize)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "canonical covered precipitation destination",
                    ))?
                    .fraction_of_ofe;
                match parcel.phase {
                    Stage3PrecipitationPhaseV1::Solid => {
                        solid += fraction * parcel.mass_kg_m2_tile_ground;
                    }
                    Stage3PrecipitationPhaseV1::Liquid => {
                        liquid += fraction * parcel.mass_kg_m2_tile_ground;
                    }
                }
            }
            let (rain_m, snowfall_m, active_precipitation_m) =
                reconstruct_stage3_phase_forcing_v1(liquid, solid)?;
            forcing.forcing.rain_m = rain_m;
            forcing.forcing.snowfall_m = snowfall_m;
            forcing.forcing.active_precipitation_m = active_precipitation_m;
            let mut result = if crate::hydrology::stage3_is_terminal_event_domain(immutable) {
                Wb11HydrologyKernel::evaluate_stage3_terminal_batch_support_with_boundary_v2(
                    stage3_inputs,
                    immutable,
                    *lane_id,
                    immutable.next_interval_index,
                    forcing,
                    *boundary,
                )
            } else {
                Wb11HydrologyKernel::evaluate_stage3_persistent_support_with_boundary(
                    stage3_inputs,
                    immutable,
                    *lane_id,
                    immutable.next_interval_index,
                    forcing,
                    *boundary,
                )
            }
            .map_err(DirectV11RealConsumerError::Stage3)?;
            if result
                .terminal_event
                .as_ref()
                .is_some_and(|event| event.event_occurred)
            {
                return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                    "canonical covered terminal event requires adaptive chronology",
                ));
            }
            Wb11HydrologyKernel::project_stage3_parent_cadence_result(
                immutable,
                &mut result,
                self.finalize_wb14_parent_interval,
            )
            .map_err(DirectV11RealConsumerError::Stage3)?;
            if result.start_state.as_ref() != immutable
                || !result.ice_mass_closure_residual_kg_m2.is_finite()
                || !result.total_water_closure_residual_kg_m2.is_finite()
                || result.ice_mass_closure_residual_kg_m2.abs() > 1.0e-6
                || result.total_water_closure_residual_kg_m2.abs() > 1.0e-6
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "canonical covered physical mass/discrete custody",
                ));
            }
            let mut interlayer_active = 0.0;
            let mut interlayer_lower = 0.0;
            for tuple in &result.reconciliation.tuples {
                if tuple.applicable {
                    let (active, lower) = reconstruct_interlayer_from_owner_states(
                        tuple.lower_cold_before_conduction_j_m2.ok_or(
                            DirectV11RealConsumerError::Identity(
                                "canonical covered lower beginning",
                            ),
                        )?,
                        tuple.lower_cold_after_conduction_j_m2.ok_or(
                            DirectV11RealConsumerError::Identity("canonical covered lower ending"),
                        )?,
                        tuple.internal_active_lower_conduction_j_m2.ok_or(
                            DirectV11RealConsumerError::Identity(
                                "canonical covered active interlayer",
                            ),
                        )?,
                        tuple.lower_cold_energy_change_j_m2.ok_or(
                            DirectV11RealConsumerError::Identity(
                                "canonical covered lower interlayer",
                            ),
                        )?,
                    )?;
                    interlayer_active += active;
                    interlayer_lower += lower;
                }
            }
            diagnostics.insert(
                *lane_id,
                (
                    result.evaluation.complete_arm_cold_content_export_j_m2,
                    interlayer_active,
                    interlayer_lower,
                    result.evaluation.complete_arm_snow_soil_heat_j_m2,
                ),
            );
            #[cfg(test)]
            stage3_refreeze_by_lane.insert(*lane_id, result.refrozen_kg_m2);
            ending_stage3.insert(*lane_id, result.state);
        }
        let mut lane_coordinates = BTreeMap::new();
        for (lane_id, state) in &ending_stage3 {
            Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(state)
                .map_err(DirectV11RealConsumerError::Stage3)?;
            let surface = if crate::hydrology::stage3_is_terminal_event_domain(state) {
                Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(state)
            } else {
                Wb11HydrologyKernel::project_stage3_surface_state_v1(state)
            }
            .map_err(DirectV11RealConsumerError::Stage3)?;
            let binding = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.production_lane_id == *lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical covered lane/OFE binding",
                ))?;
            let soil_temperature_k = phase
                .soil_candidate()
                .read_view()
                .ordered_ofes()
                .into_iter()
                .find(|ofe| ofe.ofe_id() == &binding.ofe_id)
                .and_then(|ofe| ofe.ordered_layers().into_iter().next())
                .map(crate::v9_real_consumer_shadow::DirectSoilThermalLayerReadView::temperature_k)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical covered top-soil coordinate",
                ))?;
            let boundary = phase.batch_boundaries().get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("canonical covered boundary coordinate"),
            )?;
            let humidity = phase
                .covered_lse_states()
                .iter()
                .find(|(destination, _)| destination.0 == binding.ofe_id)
                .map(|(_, state)| state.canopy_air_specific_humidity_kg_kg)
                .unwrap_or(self.interval.lse_forcing.air_specific_humidity_kg_kg);
            let water = canonical_total_snow_water_kg_m2(state);
            let energy = state
                .layers
                .iter()
                .map(|layer| layer.cold_content_j_m2)
                .sum::<f64>();
            let depth = state
                .layers
                .iter()
                .map(|layer| layer.thickness_m)
                .sum::<f64>();
            let density = if depth > 0.0 {
                state
                    .layers
                    .iter()
                    .map(|layer| layer.mass_swe_m * 1_000.0)
                    .sum::<f64>()
                    / depth
            } else {
                100.0
            };
            let duration_s = f64::from_bits(input.duration_s_bits);
            let coordinates = [
                surface.surface_temperature_k,
                soil_temperature_k,
                boundary.snow_soil_heat_j_m2 / duration_s,
                boundary.vapor_mass_kg_m2 / duration_s,
                water,
                energy,
                density,
                depth,
                humidity,
            ];
            if coordinates.into_iter().any(|value| !value.is_finite()) {
                return Err(DirectV11RealConsumerError::Identity(
                    "canonical covered nonfinite accepted coordinate",
                ));
            }
            lane_coordinates.insert(*lane_id, coordinates);
        }
        #[cfg(test)]
        record_covered_provisional_stage3_endpoint_audit_v1(
            &ending_stage3,
            &stage3_refreeze_by_lane,
        );
        Ok((
            CanonicalCoveredPhysicalEndpointV1 {
                role,
                ending_stage3,
                density_model_by_lane: self
                    .stage3_inputs_by_lane
                    .iter()
                    .map(|(lane_id, inputs)| (*lane_id, inputs.snow_density_model))
                    .collect(),
                lane_coordinates,
                diagnostics,
            },
            phase,
        ))
    }

    fn execute_canonical_covered_iteration_map_v1(
        &self,
        input: &V11ImportedV10SegmentInput,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        role: CanonicalCoveredMapRoleV1,
        attempt_ordinal: u32,
        proposal: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    ) -> Result<CanonicalCoveredIterationMapV1, DirectV11RealConsumerError> {
        #[cfg(test)]
        let validated_ordinal = if attempt_ordinal == 0
            && matches!(
                canonical_covered_parity_poison_v1(),
                Some(
                    CanonicalCoveredPhysicalParityPoisonV1::RoleOrdinal
                        | CanonicalCoveredPhysicalParityPoisonV1::RoleOrdinalAndPhysicalOneUlp
                )
            ) {
            attempt_ordinal.saturating_add(1)
        } else {
            attempt_ordinal
        };
        #[cfg(not(test))]
        let validated_ordinal = attempt_ordinal;
        if role != CanonicalCoveredMapRoleV1::Initial
            || !canonical_covered_role_ordinal_valid_v1(role, validated_ordinal)
        {
            let error = DirectV11RealConsumerError::AdaptiveRefinement(
                "canonical covered iteration role/ordinal",
            );
            #[cfg(test)]
            if validated_ordinal != attempt_ordinal {
                return Err(canonical_covered_parity_rejection_v1(
                    error,
                    CanonicalCoveredRejectionStageV1::RoleOrdinal,
                ));
            }
            return Err(error);
        }
        canonical_covered_validate_complete_proposal_before_charge_v1(proposal)?;
        canonical_covered_audit_charge_v1(role, attempt_ordinal);
        #[cfg(test)]
        let _current_map = enter_canonical_covered_current_map_v1(role, attempt_ordinal);
        let endpoint_and_physical = self.canonical_covered_map_endpoint_v1(
            input,
            beginning,
            role,
            attempt_ordinal,
            proposal,
            |stack, beginning, request, child| {
                stack.execute_covered_carrier_physical_phase_v1(beginning, request, child)
            },
        );
        #[cfg(test)]
        let endpoint_and_physical = endpoint_and_physical.map_err(|error| {
            if matches!(
                canonical_covered_parity_poison_v1(),
                Some(
                    CanonicalCoveredPhysicalParityPoisonV1::LowerBoundary
                        | CanonicalCoveredPhysicalParityPoisonV1::Precipitation
                        | CanonicalCoveredPhysicalParityPoisonV1::SoilCandidate
                        | CanonicalCoveredPhysicalParityPoisonV1::LowerBoundaryAndV8Persistent
                )
            ) && canonical_covered_parity_poison_v1()
                .is_some_and(canonical_covered_parity_poison_selected_for_current_map_v1)
            {
                canonical_covered_parity_rejection_v1(
                    error,
                    CanonicalCoveredRejectionStageV1::Physical,
                )
            } else {
                error
            }
        });
        let (endpoint, physical) = endpoint_and_physical?;
        #[cfg(test)]
        let mut endpoint = endpoint;
        #[cfg(test)]
        if attempt_ordinal == 0
            && matches!(
                canonical_covered_parity_poison_v1(),
                Some(
                    CanonicalCoveredPhysicalParityPoisonV1::PhysicalOneUlp
                        | CanonicalCoveredPhysicalParityPoisonV1::RoleOrdinalAndPhysicalOneUlp
                        | CanonicalCoveredPhysicalParityPoisonV1::SupportAndPhysicalOneUlp
                )
            )
        {
            let state = endpoint
                .ending_stage3
                .iter_mut()
                .next()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical covered one-ULP physical endpoint lane",
                ))?
                .1;
            if let Some(layer) = state.layers.first_mut() {
                layer.density_kg_m3 = f64::from_bits(layer.density_kg_m3.to_bits() + 1);
            } else {
                state.detached_retained_liquid_kg_m2 =
                    f64::from_bits(state.detached_retained_liquid_kg_m2.to_bits() + 1);
            }
            Wb11HydrologyKernel::validate_stage3_persistent_state(state).map_err(|error| {
                canonical_covered_parity_rejection_v1(
                    DirectV11RealConsumerError::Stage3(error),
                    CanonicalCoveredRejectionStageV1::PhysicalValidation,
                )
            })?;
        }
        #[cfg(test)]
        canonical_covered_audit_update_v1(|solve| {
            solve.validated_iteration_endpoint_count =
                solve.validated_iteration_endpoint_count.saturating_add(1);
            solve.validated_physical_endpoint_count =
                solve.validated_physical_endpoint_count.saturating_add(1);
        });
        Ok(CanonicalCoveredIterationMapV1 {
            endpoint,
            _physical: physical,
        })
    }

    fn execute_canonical_covered_pending_adjudication_map_v1(
        &self,
        input: &V11ImportedV10SegmentInput,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        role: CanonicalCoveredMapRoleV1,
        attempt_ordinal: u32,
        proposal: &BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    ) -> Result<CanonicalCoveredPendingAdjudicationMapV1, DirectV11RealConsumerError> {
        if role == CanonicalCoveredMapRoleV1::Initial
            || !canonical_covered_role_ordinal_valid_v1(role, attempt_ordinal)
        {
            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                "canonical covered adjudication role/ordinal",
            ));
        }
        canonical_covered_validate_complete_proposal_before_charge_v1(proposal)?;
        canonical_covered_audit_charge_v1(role, attempt_ordinal);
        #[cfg(test)]
        let _current_map = enter_canonical_covered_current_map_v1(role, attempt_ordinal);
        let request = CoveredTerminalBatchTrialRequestV2 {
            support: input.support,
            role: crate::hydrology::CoveredTerminalTrialRoleV1::Root,
            attempt_ordinal,
            lanes: proposal.clone(),
            beginning_joint: beginning.joint().clone(),
        };
        let mut completion_child = None;
        let endpoint_and_physical = self.canonical_covered_map_endpoint_v1(
            input,
            beginning,
            role,
            attempt_ordinal,
            proposal,
            |stack, beginning, request, child| {
                completion_child = Some(child.clone());
                stack.execute_covered_carrier_physical_phase_v1(beginning, request, child)
            },
        );
        #[cfg(test)]
        let endpoint_and_physical = endpoint_and_physical.map_err(|error| {
            if matches!(
                canonical_covered_parity_poison_v1(),
                Some(
                    CanonicalCoveredPhysicalParityPoisonV1::LowerBoundary
                        | CanonicalCoveredPhysicalParityPoisonV1::Precipitation
                        | CanonicalCoveredPhysicalParityPoisonV1::SoilCandidate
                        | CanonicalCoveredPhysicalParityPoisonV1::LowerBoundaryAndV8Persistent
                )
            ) && canonical_covered_parity_poison_v1()
                .is_some_and(canonical_covered_parity_poison_selected_for_current_map_v1)
            {
                canonical_covered_parity_rejection_v1(
                    error,
                    CanonicalCoveredRejectionStageV1::Physical,
                )
            } else {
                error
            }
        });
        let (endpoint, physical) = endpoint_and_physical?;
        #[cfg(test)]
        let mut endpoint = endpoint;
        #[cfg(test)]
        match take_canonical_covered_final_convergence_poison_v1() {
            Some(CanonicalCoveredFinalConvergencePoisonV1::OuterNonclosure) => {
                let coordinates = endpoint.lane_coordinates.values_mut().next().ok_or(
                    DirectV11RealConsumerError::Identity(
                        "canonical covered adjudication outer-convergence poison lane",
                    ),
                )?;
                coordinates[5] += 1.0;
            }
            Some(CanonicalCoveredFinalConvergencePoisonV1::DependentNonclosure) => {
                let coordinates = endpoint.lane_coordinates.values_mut().next().ok_or(
                    DirectV11RealConsumerError::Identity(
                        "canonical covered adjudication dependent-convergence poison lane",
                    ),
                )?;
                coordinates[1] += 1.0;
            }
            None => {}
        }
        #[cfg(test)]
        canonical_covered_audit_update_v1(|solve| {
            solve.validated_physical_endpoint_count =
                solve.validated_physical_endpoint_count.saturating_add(1);
            solve.validated_pending_adjudication_count =
                solve.validated_pending_adjudication_count.saturating_add(1);
        });
        let completion_child = completion_child.ok_or(DirectV11RealConsumerError::Identity(
            "canonical covered adjudication physical child retention",
        ))?;
        Ok(CanonicalCoveredPendingAdjudicationMapV1 {
            endpoint,
            physical,
            completion_child,
            request,
        })
    }

    fn consume_canonical_covered_pending_as_history_v1(
        pending: CanonicalCoveredPendingAdjudicationMapV1,
    ) -> CanonicalCoveredIterationMapV1 {
        #[cfg(test)]
        canonical_covered_audit_update_v1(|solve| {
            solve.history_disposition_count = solve.history_disposition_count.saturating_add(1);
            solve.validated_iteration_endpoint_count =
                solve.validated_iteration_endpoint_count.saturating_add(1);
        });
        CanonicalCoveredIterationMapV1 {
            endpoint: pending.endpoint,
            _physical: pending.physical,
        }
    }

    fn consume_canonical_covered_pending_as_adaptive_rejection_v1(
        _pending: CanonicalCoveredPendingAdjudicationMapV1,
    ) -> DirectV11RealConsumerError {
        #[cfg(test)]
        canonical_covered_audit_update_v1(|solve| {
            solve.dependent_rejection_disposition_count = solve
                .dependent_rejection_disposition_count
                .saturating_add(1);
        });
        let error = DirectV11RealConsumerError::AdaptiveRefinement(
            "canonical covered dependent-output instability",
        );
        #[cfg(test)]
        let error = canonical_covered_parity_rejection_v1(
            error,
            CanonicalCoveredRejectionStageV1::AdjudicationConvergence,
        );
        error
    }

    fn consume_canonical_covered_pending_as_final_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        pending: CanonicalCoveredPendingAdjudicationMapV1,
    ) -> Result<CanonicalCoveredFinalMapV1, DirectV11RealConsumerError> {
        #[cfg(test)]
        let _current_map = enter_canonical_covered_current_map_v1(
            pending.endpoint.role,
            pending.request.attempt_ordinal,
        );
        #[cfg(test)]
        canonical_covered_audit_update_v1(|solve| {
            solve.final_disposition_count = solve.final_disposition_count.saturating_add(1);
            solve.validated_final_physical_endpoint_count = solve
                .validated_final_physical_endpoint_count
                .saturating_add(1);
        });
        let phase = self.complete_covered_carrier_physical_phase_v1(
            beginning,
            &pending.request,
            pending.completion_child,
            pending.physical,
        )?;
        Ok(CanonicalCoveredFinalMapV1 {
            endpoint: pending.endpoint,
            phase,
        })
    }

    fn execute_canonical_covered_production_v1(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, DirectV11RealConsumerError> {
        #[cfg(test)]
        let _solve_termination = begin_canonical_covered_solve_termination_guard_v1();
        let tolerance_scale = take_canonical_covered_tolerance_scale_v1();
        let immutable_stack = self.clone();
        let initial_candidates = immutable_stack.canonical_covered_initial_candidates_v1(input)?;
        let mut proposal = self
            .stage3_beginning_by_lane
            .iter()
            .filter(|(_, state)| {
                crate::hydrology::stage3_is_resolved_thermal_domain(state)
                    || crate::hydrology::stage3_is_terminal_event_domain(state)
            })
            .map(|(lane_id, state)| {
                let density_model = immutable_stack
                    .stage3_inputs_by_lane
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "canonical covered density-model lane",
                    ))?
                    .snow_density_model;
                Ok((
                    *lane_id,
                    canonical_covered_lane_trial_v1(*lane_id, state, density_model)?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
        if proposal.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "canonical covered empty active topology",
            ));
        }
        let mut roles = Vec::with_capacity(usize::from(CANONICAL_COVERED_MAX_AUTHENTIC_MAPS));
        let initial = immutable_stack.execute_canonical_covered_iteration_map_v1(
            input,
            &initial_candidates,
            CanonicalCoveredMapRoleV1::Initial,
            0,
            &proposal,
        )?;
        roles.push(initial.role);
        let initial_proposal = proposal;
        proposal = initial
            .ending_stage3
            .iter()
            .map(|(lane_id, state)| {
                let density_model = immutable_stack
                    .stage3_inputs_by_lane
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "canonical covered density-model lane",
                    ))?
                    .snow_density_model;
                Ok((
                    *lane_id,
                    canonical_covered_lane_trial_v1(*lane_id, state, density_model)?,
                ))
            })
            .collect::<Result<_, DirectV11RealConsumerError>>()?;
        let mut previous_proposal = initial_proposal;
        let mut previous = initial;
        let mut role = CanonicalCoveredMapRoleV1::FixedPointAdjudication;
        let mut ordinal = 1_u32;
        let final_map = loop {
            let pending = immutable_stack.execute_canonical_covered_pending_adjudication_map_v1(
                input,
                &initial_candidates,
                role,
                ordinal,
                &proposal,
            )?;
            roles.push(pending.role);
            let convergence = canonical_covered_production_converged_v1(
                &previous,
                &pending,
                &proposal,
                tolerance_scale,
            )?;
            if convergence.outer_coordinates {
                if !convergence.dependent_carriers {
                    return Err(
                        Self::consume_canonical_covered_pending_as_adaptive_rejection_v1(pending),
                    );
                }
                break immutable_stack
                    .consume_canonical_covered_pending_as_final_v1(&initial_candidates, pending)?;
            }

            let history = Self::consume_canonical_covered_pending_as_history_v1(pending);
            if ordinal >= 6 {
                return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                    "canonical covered evaluation budget",
                ));
            }
            let next_proposal = canonical_covered_multisecant_trial_v1(
                &previous_proposal,
                &previous,
                &proposal,
                &history,
                tolerance_scale,
            )?;
            previous_proposal = proposal;
            proposal = next_proposal;
            previous = history;
            let trial = u8::try_from(ordinal).map_err(|_| {
                DirectV11RealConsumerError::AdaptiveRefinement(
                    "canonical covered evaluation budget",
                )
            })?;
            role = CanonicalCoveredMapRoleV1::MultisecantAdjudication(trial);
            ordinal = ordinal.saturating_add(1);
        };
        if !(2..=7).contains(&roles.len())
            || roles.first() != Some(&CanonicalCoveredMapRoleV1::Initial)
            || roles.get(1) != Some(&CanonicalCoveredMapRoleV1::FixedPointAdjudication)
            || !roles[2..].iter().enumerate().all(|(offset, role)| {
                *role
                    == CanonicalCoveredMapRoleV1::MultisecantAdjudication(
                        u8::try_from(offset + 1).unwrap_or(u8::MAX),
                    )
            })
        {
            return Err(DirectV11RealConsumerError::Identity(
                "canonical covered charged-map chronology",
            ));
        }
        let evidence = self.seal_accepted_carrier_evidence_v1(
            &final_map.phase,
            input,
            &final_map.ending_stage3,
        )?;
        let persistent_receipts = match final_map.phase.soil_candidate.v1() {
            Ok(v1) => self.snow_soil_heat_receipts(input.support, &final_map.ending_stage3, v1)?,
            Err(_) if final_map.phase.soil_candidate.v2().is_ok() => self
                .snow_soil_heat_receipts_v2(
                    input.support,
                    &final_map.ending_stage3,
                    &final_map.phase.soil_candidate,
                )?,
            Err(error) => {
                return Err(DirectV11RealConsumerError::Runtime(
                    DirectV10RealConsumerError::Runtime(error),
                ));
            }
        };
        self.validate_snow_soil_heat_receipt_iterate_joins(
            &persistent_receipts,
            &final_map.ending_stage3,
            final_map.phase.soil_candidate.read_view(),
        )?;
        let terminal_soil = self.terminal_snow_soil_heat_receipts(
            input.support,
            &final_map.ending_stage3,
            &final_map.phase.soil_candidate,
            &persistent_receipts,
            &final_map.diagnostics,
        )?;
        let terminal_events = BTreeMap::new();
        let accepted_trial_soil = final_map
            .phase
            .batch_terminal_snow_soil_trial_receipts_by_lane
            .clone();
        let physical_ledgers = self.physical_outcome_ledgers(&PhysicalOutcomeLedgerInputs {
            support: input.support,
            ending: &final_map.ending_stage3,
            lanes: &evidence.final_lanes,
            destinations: &evidence.final_boundaries,
            precipitation: &final_map.phase.precipitation_sets,
            soil: &persistent_receipts,
            terminal_soil: &terminal_soil,
            adaptive_trial_soil: &accepted_trial_soil,
            terminal_events: &terminal_events,
            diagnostics: &final_map.diagnostics,
        })?;
        let ending_snow_owner_bytes =
            canonical_stage3_snow_owner_bytes_v11_with_pending_and_receipts(
                &final_map.ending_stage3,
                &self.pending_terminal_parcels,
                &evidence.final_lanes,
                &evidence.final_boundaries,
            )?;
        let soil_credits = final_map
            .phase
            .batch_soil_top_boundary_credits_by_lane
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let ordinary_physical_ending = final_map.phase.ending_candidates.shadow().clone();
        #[cfg(test)]
        let complete_owner_scope = begin_canonical_covered_complete_owner_scope_v1();
        let (output, candidate, support_receipt) =
            finalize_v11_imported_segment_with_soil_continuation(
                &self.beginning,
                input,
                &final_map.phase.carrier_envelope,
                None,
                Some(final_map.phase.ending_candidates.shadow()),
                Some(&final_map.phase.soil_candidate),
                final_map.phase.ending_candidates.soil_continuation(),
                ending_snow_owner_bytes,
                self.day_index,
                self.interval_index,
                self.interval,
                &soil_credits,
                &physical_ledgers,
                AcceptedPublicationFinalizationPostureV1::PrivateNoHistory,
            )?;
        #[cfg(test)]
        drop(complete_owner_scope);
        self.last_support_receipt = Some(support_receipt);
        self.last_final_boundary_receipts = Some(evidence.final_boundaries);
        self.last_lane_boundary_receipts = Some(evidence.final_lanes);
        self.last_component_carrier_receipts = Some(evidence.component_receipts);
        self.last_snow_soil_heat_receipts = Some(persistent_receipts);
        self.last_terminal_snow_soil_heat_receipts = Some(terminal_soil);
        self.last_adaptive_terminal_snow_soil_trial_receipts = Some(accepted_trial_soil);
        self.last_precipitation_parcel_sets = Some(final_map.phase.precipitation_sets.clone());
        self.last_physical_outcome_ledgers = Some(physical_ledgers);
        self.last_terminal_events = Some(terminal_events);
        self.last_wb14_child_receipt_set_sha256 = Some(evidence.wb14_child_receipt_set_sha256);
        self.last_wb14_parent_receipt_set_sha256 = evidence.wb14_parent_receipt_set_sha256;
        self.last_wb14_child_replay_bytes = Some(evidence.wb14_child_replay_bytes);
        self.last_wb14_parent_replay_bytes = evidence.wb14_parent_replay_bytes;
        self.ending_stage3_by_lane = Some(final_map.endpoint.ending_stage3);
        self.ending = Some(candidate);
        self.last_publication_retained = Some(true);
        self.ordinary_physical_reuse_seed = Some(CoveredOrdinaryPhysicalReuseSeedV1 {
            physical_authority: covered_ordinary_physical_authority_v1(input)?,
            envelope: final_map.phase.carrier_envelope,
            physical_ending: ordinary_physical_ending,
            soil_candidate: final_map.phase.soil_candidate,
            soil_continuation: final_map
                .phase
                .ending_candidates
                .soil_continuation()
                .cloned(),
            soil_top_boundary_credits: soil_credits,
        });
        #[cfg(test)]
        {
            canonical_covered_audit_update_v1(|solve| {
                solve.completed = true;
                solve.completed_final_envelope_count =
                    solve.completed_final_envelope_count.saturating_add(1);
            });
        }
        Ok(output)
    }
}

fn canonical_stage3_open_snow_execute_v1<S, I, C, V>(
    installed: &mut S,
    input: I,
    evaluate_covered: C,
    execute_thin_pack_v22: V,
) -> Result<S::Output, S::Error>
where
    S: CanonicalStage3OpenSnowExecutionV1<I, C, V>,
{
    installed.execute_canonical_stage3_open_snow(input, evaluate_covered, execute_thin_pack_v22)
}
