const COVERED_PHYSICAL_EVALUATION_LIMIT_V1: usize = 96;
const COVERED_STABLE_MONOTONE_RAW_MAPS_REQUIRED: usize = 8;
const COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1: usize = 3;
const COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1: usize = 2;
const COVERED_RECEIPT_REPLAY_RESERVE_V1: usize = 1;
const COVERED_AUTHENTIC_RECEIPT_EXACT_CYCLE_MAXIMUM_V1: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredPhaseConsistentPhysicalMapPostureV1 {
    FinalizationEquivalent,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredFinalizationEquivalentPhysicalMapV1<T> {
    posture: CoveredPhaseConsistentPhysicalMapPostureV1,
    stage3_physical_map_count: usize,
    physical_evaluation_ordinal: usize,
    value: T,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredFinalizationEquivalentReplayInputsV1 {
    proposed_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    proposed_soil: DirectSoilThermalCandidate,
    input_covered_boundaries: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    input_open_boundaries: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    destination_receipts: BTreeMap<(OfeId, TileId), Digest32>,
}

impl<T> CoveredFinalizationEquivalentPhysicalMapV1<T> {
    fn validate(&self) -> Result<(), PhaseConsistentCoupledSolveErrorV1> {
        if self.posture != CoveredPhaseConsistentPhysicalMapPostureV1::FinalizationEquivalent
            || self.stage3_physical_map_count != 1
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        Ok(())
    }
}

fn covered_phase_consistent_finalization_equivalent_map_v1<T>(
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    evaluate_once: impl FnOnce() -> Result<T, PhaseConsistentCoupledSolveErrorV1>,
) -> Result<CoveredFinalizationEquivalentPhysicalMapV1<T>, PhaseConsistentCoupledSolveErrorV1> {
    covered_physical_evaluation_budget_charge_v1(budget)?;
    let value = evaluate_once()?;
    Ok(CoveredFinalizationEquivalentPhysicalMapV1 {
        posture: CoveredPhaseConsistentPhysicalMapPostureV1::FinalizationEquivalent,
        stage3_physical_map_count: 1,
        physical_evaluation_ordinal: budget.used,
        value,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CoveredPhaseConsistentPhaseV1 {
    water_kg_m2: f64,
    enthalpy_j_m2: f64,
    ice_kg_m2: f64,
    liquid_kg_m2: f64,
    cold_content_j_m2: f64,
    snow_temperature_k: f64,
    depth_m: f64,
    density_kg_m3: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CoveredTerminalDensityGeometryCoordinateV1 {
    rho_1_kg_m3: f64,
    ice_1_kg_m2: f64,
    z_1_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CoveredDerivedThicknessClosureV1 {
    proposed_z_m: f64,
    physical_z_m: f64,
    r_z_m: f64,
    scaled_merit: f64,
}

fn covered_derived_thickness_closure_evaluate_v1(
    coordinate: CoveredTerminalDensityGeometryCoordinateV1,
    physical_ice_kg_m2: f64,
    physical_density_kg_m3: f64,
    physical_thickness_m: f64,
) -> Result<CoveredDerivedThicknessClosureV1, PhaseConsistentCoupledSolveErrorV1> {
    if !coordinate.ice_1_kg_m2.is_finite()
        || coordinate.ice_1_kg_m2 <= 0.0
        || !coordinate.rho_1_kg_m3.is_finite()
        || coordinate.rho_1_kg_m3 <= 0.0
        || !coordinate.z_1_m.is_finite()
        || coordinate.z_1_m <= 0.0
        || coordinate.z_1_m.to_bits() != (coordinate.ice_1_kg_m2 / coordinate.rho_1_kg_m3).to_bits()
        || !physical_ice_kg_m2.is_finite()
        || physical_ice_kg_m2 <= 0.0
        || !physical_density_kg_m3.is_finite()
        || physical_density_kg_m3 <= 0.0
        || !physical_thickness_m.is_finite()
        || physical_thickness_m <= 0.0
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let physical_z_m = physical_ice_kg_m2 / physical_density_kg_m3;
    if !physical_z_m.is_finite()
        || physical_z_m <= 0.0
        || physical_z_m.to_bits() != physical_thickness_m.to_bits()
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let r_z_m = coordinate.z_1_m - physical_z_m;
    let scaled_merit = r_z_m.abs() / COVERED_FIXED_POINT_POLICY.depth_abs_m;
    if !r_z_m.is_finite() || !scaled_merit.is_finite() {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    Ok(CoveredDerivedThicknessClosureV1 {
        proposed_z_m: coordinate.z_1_m,
        physical_z_m,
        r_z_m,
        scaled_merit,
    })
}

impl CoveredTerminalDensityGeometryCoordinateV1 {
    fn from_canonical_phase(
        phase: &CoveredPhaseConsistentPhaseV1,
    ) -> Result<Self, PhaseConsistentCoupledSolveErrorV1> {
        let rho_1_kg_m3 = phase.density_kg_m3;
        let ice_1_kg_m2 = phase.ice_kg_m2;
        let z_1_m = ice_1_kg_m2 / rho_1_kg_m3;
        if !rho_1_kg_m3.is_finite()
            || rho_1_kg_m3 <= 0.0
            || !ice_1_kg_m2.is_finite()
            || ice_1_kg_m2 <= 0.0
            || !z_1_m.is_finite()
            || z_1_m <= 0.0
            || z_1_m.to_bits() != phase.depth_m.to_bits()
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        Ok(Self {
            rho_1_kg_m3,
            ice_1_kg_m2,
            z_1_m,
        })
    }

    fn density_absolute_tolerance_kg_m3(self) -> f64 {
        // This is the existing depth-closure tolerance expressed through the
        // exact z=I/rho basis. It introduces no density tolerance or repair.
        self.rho_1_kg_m3 * COVERED_FIXED_POINT_POLICY.depth_abs_m / self.z_1_m
    }
}

fn covered_terminal_density_geometry_residual_evaluate_v1(
    coordinate: CoveredTerminalDensityGeometryCoordinateV1,
    physical_stage3_density_kg_m3: f64,
    exact_density_settling_branch_satisfied: bool,
) -> Result<f64, PhaseConsistentCoupledSolveErrorV1> {
    if !exact_density_settling_branch_satisfied
        || !physical_stage3_density_kg_m3.is_finite()
        || physical_stage3_density_kg_m3 <= 0.0
        || !coordinate.rho_1_kg_m3.is_finite()
        || coordinate.rho_1_kg_m3 <= 0.0
        || !coordinate.z_1_m.is_finite()
        || coordinate.z_1_m <= 0.0
        || coordinate.z_1_m.to_bits() != (coordinate.ice_1_kg_m2 / coordinate.rho_1_kg_m3).to_bits()
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    // The right operand is the unchanged Stage-3 density/settling
    // constitutive result. This is the physical R_rho, not a generic map
    // difference synthesized from the coupled coordinate vector.
    let r_rho_kg_m3 = coordinate.rho_1_kg_m3 - physical_stage3_density_kg_m3;
    if !r_rho_kg_m3.is_finite() {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    Ok(r_rho_kg_m3)
}

fn covered_terminal_density_physical_layer_v1(
    state: &DirectSnowStage3PersistentState,
    expected_settle_day_count: f64,
) -> Result<&crate::DirectSnowLayerState, PhaseConsistentCoupledSolveErrorV1> {
    if state.layers.len() != 1
        || state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
        || !expected_settle_day_count.is_finite()
        || state.layers[0].settle_day_count.to_bits() != expected_settle_day_count.to_bits()
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    Ok(&state.layers[0])
}

fn phase_consistent_canonical_phase_projection_v1(
    water_kg_m2: f64,
    enthalpy_j_m2: f64,
    density_kg_m3: f64,
) -> Result<CoveredPhaseConsistentPhaseV1, PhaseConsistentCoupledSolveErrorV1> {
    let fusion = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG;
    if !water_kg_m2.is_finite()
        || water_kg_m2 <= 0.0
        || !enthalpy_j_m2.is_finite()
        || !density_kg_m3.is_finite()
        || density_kg_m3 <= 0.0
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    let fusion_capacity = fusion * water_kg_m2;
    if enthalpy_j_m2 >= fusion_capacity {
        // The all-liquid side belongs to the existing terminal-event
        // partition and may not be crossed by an unpublished coupled trial.
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let (ice_kg_m2, liquid_kg_m2, cold_content_j_m2) = if enthalpy_j_m2 <= 0.0 {
        (water_kg_m2, 0.0, -enthalpy_j_m2)
    } else {
        let liquid = enthalpy_j_m2 / fusion;
        (water_kg_m2 - liquid, liquid, 0.0)
    };
    let mass_swe_m = ice_kg_m2 / 1_000.0;
    let snow_temperature_k = Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
        mass_swe_m,
        cold_content_j_m2,
    ) + 273.15;
    let depth_m = ice_kg_m2 / density_kg_m3;
    if !snow_temperature_k.is_finite()
        || !(200.0..=273.15).contains(&snow_temperature_k)
        || !depth_m.is_finite()
        || depth_m <= 0.0
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    Ok(CoveredPhaseConsistentPhaseV1 {
        water_kg_m2,
        enthalpy_j_m2,
        ice_kg_m2,
        liquid_kg_m2,
        cold_content_j_m2,
        snow_temperature_k,
        depth_m,
        density_kg_m3,
    })
}

fn covered_phase_consistent_project_stage3_coordinates_v1(
    baseline: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    lane_ids: &[u32],
    coordinates: &[f64],
) -> Result<
    (
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        BTreeMap<u32, CoveredPhaseConsistentPhaseV1>,
    ),
    PhaseConsistentCoupledSolveErrorV1,
> {
    if baseline.len() != lane_ids.len() || coordinates.len() < 3 * lane_ids.len() {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let mut projected = baseline.clone();
    let mut phases = BTreeMap::new();
    for (lane_index, lane_id) in lane_ids.iter().enumerate() {
        let phase = phase_consistent_canonical_phase_projection_v1(
            coordinates[3 * lane_index],
            coordinates[3 * lane_index + 1],
            coordinates[3 * lane_index + 2],
        )?;
        let state = projected
            .get_mut(lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        let prior = covered_terminal_density_physical_layer_v1(
            state,
            state
                .layers
                .first()
                .ok_or(PhaseConsistentCoupledSolveErrorV1::SideConstraint)?
                .settle_day_count,
        )?
        .clone();
        state.layers = vec![crate::DirectSnowLayerState {
            mass_swe_m: phase.ice_kg_m2 / 1_000.0,
            thickness_m: phase.depth_m,
            density_kg_m3: phase.density_kg_m3,
            settle_day_count: prior.settle_day_count,
            temperature_c: phase.snow_temperature_k - 273.15,
            liquid_water_m: phase.liquid_kg_m2 / 1_000.0,
            cold_content_j_m2: phase.cold_content_j_m2,
            refrozen_liquid_m: prior.refrozen_liquid_m,
        }];
        state.detached_retained_liquid_kg_m2 = 0.0;
        state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
        Wb11HydrologyKernel::validate_stage3_persistent_state(state)
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        phases.insert(*lane_id, phase);
    }
    if projected.keys().ne(lane_ids.iter()) {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    Ok((projected, phases))
}

fn phase_consistent_support_coordinates_v1(
    beginning: &DirectSnowStage3PersistentState,
    support: &CoveredExactFloorTerminalPhaseSupportImageV1,
) -> Result<(f64, f64), PhaseConsistentCoupledSolveErrorV1> {
    support
        .validate()
        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
    if beginning.layers.len() > 1
        || beginning.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let beginning_ice = beginning
        .layers
        .iter()
        .map(|layer| 1_000.0 * layer.mass_swe_m)
        .sum::<f64>();
    let beginning_liquid = beginning
        .layers
        .iter()
        .map(|layer| 1_000.0 * layer.liquid_water_m)
        .sum::<f64>();
    let beginning_cold = beginning
        .layers
        .iter()
        .map(|layer| layer.cold_content_j_m2)
        .sum::<f64>();
    let water =
        beginning_ice + beginning_liquid + support.snowfall_kg_m2 + support.deposition_kg_m2
            - support.sublimation_kg_m2
            + support.external_liquid_kg_m2;
    let enthalpy = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG
        * (beginning_liquid + support.external_liquid_kg_m2)
        - beginning_cold
        + support.complete_energy_j_m2
        + support.cold_content_export_j_m2;
    if !water.is_finite() || !enthalpy.is_finite() {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    Ok((water, enthalpy))
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredPhysicalEvaluationBudgetV1 {
    used: usize,
    maximum: usize,
}

/// Exact, non-physical coordinates which must remain unchanged while a raw
/// authentic covered map contracts.  The byte carriers are assembled from
/// the typed runtime owners; they deliberately exclude the evolving receipt
/// digest, candidate-ending identities, applied energy, and V2 high/carry
/// coordinate values.
#[derive(Clone, Debug, Eq, PartialEq)]
struct CoveredStableMonotoneStaticJoinsV1 {
    support_start_ns: u128,
    support_end_ns: u128,
    source_event_topology_custody: Vec<u8>,
    static_receipt_joins: Vec<Vec<u8>>,
    phase_branch: Vec<u8>,
    density_model_branch: Vec<u8>,
    carry_authority_and_representation: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredStableMonotoneRawAuthenticMapV1 {
    static_joins: CoveredStableMonotoneStaticJoinsV1,
    physical_receipt_digests: Vec<Digest32>,
    evolving_carry_coordinate_bits: Vec<(u64, i8, String, i32)>,
    residual: CoveredPhaseConsistentResidualEvaluationV1,
    authentic_seed_coordinates: Vec<f64>,
    physical_evaluation_ordinal: usize,
    event_free_terminal_one_volume: bool,
    exact_carry_reconstruction_satisfied: bool,
    active_set_transition: bool,
    finalization_restart: bool,
    publication_eligible: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredStableMonotoneSolveEligibilityV1 {
    seed_coordinates: Vec<f64>,
    density_model_branch: Vec<u8>,
    raw_maps_charged: usize,
    publication_eligible: bool,
}

fn covered_stable_monotone_solve_eligibility_v1(
    maps: &[CoveredStableMonotoneRawAuthenticMapV1],
    budget: &CoveredPhysicalEvaluationBudgetV1,
) -> Result<CoveredStableMonotoneSolveEligibilityV1, PhaseConsistentCoupledSolveErrorV1> {
    if maps.len() != COVERED_STABLE_MONOTONE_RAW_MAPS_REQUIRED
        || budget.maximum != COVERED_PHYSICAL_EVALUATION_LIMIT_V1
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let first = maps
        .first()
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let minimum_support_ns =
        crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
    if first.static_joins.support_end_ns <= first.static_joins.support_start_ns
        || first.static_joins.support_end_ns - first.static_joins.support_start_ns
            < minimum_support_ns
        || first.physical_evaluation_ordinal == 0
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    for (index, map) in maps.iter().enumerate() {
        if map.static_joins != first.static_joins
            || !map.event_free_terminal_one_volume
            || !map.exact_carry_reconstruction_satisfied
            || map.active_set_transition
            || map.finalization_restart
            || map.publication_eligible
            || !map.residual.algebraic_side_constraints_satisfied
            || !map.residual.scaled_merit.is_finite()
            || map
                .residual
                .residuals
                .iter()
                .any(|value| !value.is_finite())
            || map.physical_evaluation_ordinal != first.physical_evaluation_ordinal + index
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        if index > 0 && map.residual.scaled_merit >= maps[index - 1].residual.scaled_merit {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonDescent);
        }
    }
    let last_ordinal = maps
        .last()
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?
        .physical_evaluation_ordinal;
    if last_ordinal > budget.used {
        return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
    }
    Ok(CoveredStableMonotoneSolveEligibilityV1 {
        seed_coordinates: maps
            .last()
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?
            .authentic_seed_coordinates
            .clone(),
        density_model_branch: first.static_joins.density_model_branch.clone(),
        raw_maps_charged: maps.len(),
        publication_eligible: false,
    })
}

fn covered_stable_monotone_observe_raw_authentic_map_v1(
    trace: &mut Vec<CoveredStableMonotoneRawAuthenticMapV1>,
    map: CoveredStableMonotoneRawAuthenticMapV1,
    budget: &CoveredPhysicalEvaluationBudgetV1,
) -> Option<CoveredStableMonotoneSolveEligibilityV1> {
    let locally_admissible = map.event_free_terminal_one_volume
        && map.exact_carry_reconstruction_satisfied
        && !map.active_set_transition
        && !map.finalization_restart
        && !map.publication_eligible
        && map.residual.algebraic_side_constraints_satisfied
        && map.residual.scaled_merit.is_finite()
        && map.residual.residuals.iter().all(|value| value.is_finite());
    let extends = trace.last().is_none_or(|previous| {
        map.static_joins == previous.static_joins
            && map.physical_evaluation_ordinal == previous.physical_evaluation_ordinal + 1
            && map.residual.scaled_merit < previous.residual.scaled_merit
    });
    if !locally_admissible || !extends {
        trace.clear();
    }
    if locally_admissible {
        trace.push(map);
    }
    if trace.len() > COVERED_STABLE_MONOTONE_RAW_MAPS_REQUIRED {
        trace.remove(0);
    }
    covered_stable_monotone_solve_eligibility_v1(trace, budget).ok()
}

fn covered_stable_monotone_clear_on_finalization_restart_v1(
    trace: &mut Vec<CoveredStableMonotoneRawAuthenticMapV1>,
    pre_root_refusal_disabled: &mut bool,
) {
    trace.clear();
    *pre_root_refusal_disabled = true;
}

fn covered_stable_monotone_disable_after_pre_root_refusal_v1(
    trace: &mut Vec<CoveredStableMonotoneRawAuthenticMapV1>,
    pre_root_refusal_disabled: &mut bool,
) {
    trace.clear();
    *pre_root_refusal_disabled = true;
}

impl CoveredPhysicalEvaluationBudgetV1 {
    fn new(already_used: usize) -> Result<Self, PhaseConsistentCoupledSolveErrorV1> {
        if already_used > COVERED_PHYSICAL_EVALUATION_LIMIT_V1 {
            return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
        }
        Ok(Self {
            used: already_used,
            maximum: COVERED_PHYSICAL_EVALUATION_LIMIT_V1,
        })
    }
}

fn covered_physical_evaluation_budget_charge_v1(
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
) -> Result<(), PhaseConsistentCoupledSolveErrorV1> {
    if budget.used >= budget.maximum {
        return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
    }
    budget.used += 1;
    Ok(())
}

fn covered_physical_evaluation_budget_preserve_v1(
    budget: &CoveredPhysicalEvaluationBudgetV1,
    required_after_charge: usize,
) -> Result<(), PhaseConsistentCoupledSolveErrorV1> {
    if budget.maximum != COVERED_PHYSICAL_EVALUATION_LIMIT_V1
        || required_after_charge >= budget.maximum
        || budget.maximum.saturating_sub(budget.used) <= required_after_charge
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredPhaseConsistentTransitionResetV1 {
    root_join_fingerprints: Vec<u64>,
    reset_join_fingerprints: Vec<u64>,
    root_coordinates_bits: Vec<u64>,
    reset_coordinates_bits: Vec<u64>,
    root_branch_predicates: Vec<u8>,
    reset_branch_predicates: Vec<u8>,
    branch_entry_vapor_sides: Vec<i8>,
    opposite_raw_vapor_sides: Vec<i8>,
    raw_authentic_continuous_owner_bits: Vec<u64>,
}

fn phase_consistent_coupled_active_set_transition_reset_v1(
    trace: &CoveredPhaseConsistentTransitionResetV1,
) -> bool {
    !trace.root_join_fingerprints.is_empty()
        && trace.root_join_fingerprints == trace.reset_join_fingerprints
        && !trace.root_coordinates_bits.is_empty()
        && trace.root_coordinates_bits == trace.reset_coordinates_bits
        && trace.root_branch_predicates == trace.reset_branch_predicates
        && !trace.root_branch_predicates.is_empty()
        && trace.branch_entry_vapor_sides.len() == trace.opposite_raw_vapor_sides.len()
        && !trace.branch_entry_vapor_sides.is_empty()
        && trace
            .branch_entry_vapor_sides
            .iter()
            .zip(&trace.opposite_raw_vapor_sides)
            .all(|(entry, opposite)| matches!((*entry, *opposite), (1, -1) | (-1, 1)))
}

/// Observe one complete exact active-set reset window.
///
/// The current interface has already passed the active-set identity, support,
/// closure, and projection checks. A nonexact reset is therefore the next
/// valid root anchor, not a reason to retain the stale first root forever.
/// Solver dispatch remains restricted to the unchanged exact reset predicate.
fn phase_consistent_coupled_active_set_transition_window_v1<T: Clone>(
    root_anchor: &mut Option<T>,
    branch_entry_seen: &mut bool,
    branch_entry_vapor_sides: &mut Option<Vec<i8>>,
    current_validated_interface: &T,
    trace: &CoveredPhaseConsistentTransitionResetV1,
) -> bool {
    if phase_consistent_coupled_active_set_transition_reset_v1(trace) {
        return true;
    }
    *root_anchor = Some(current_validated_interface.clone());
    *branch_entry_seen = false;
    *branch_entry_vapor_sides = None;
    false
}

const COVERED_PARITY_MONOTONE_ACTIVE_SET_WINDOWS_REQUIRED: usize = 4;

#[derive(Clone, Debug, PartialEq)]
struct CoveredParityMonotoneActiveSetResetV1 {
    support_start_ns: u128,
    support_end_ns: u128,
    reset: CoveredPhaseConsistentTransitionResetV1,
    physical_evaluation_ordinal: usize,
    publication_eligible: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredParityMonotoneActiveSetEligibilityV1 {
    seed_coordinates: Vec<f64>,
    reset_windows_observed: usize,
    minimum_solver_reserve: usize,
    publication_eligible: bool,
}

fn covered_parity_monotone_active_set_root_drift_v1(
    reset: &CoveredPhaseConsistentTransitionResetV1,
) -> Result<f64, PhaseConsistentCoupledSolveErrorV1> {
    if reset.root_coordinates_bits.is_empty()
        || reset.root_coordinates_bits.len() != reset.reset_coordinates_bits.len()
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let mut squared = 0.0;
    for (root_bits, reset_bits) in reset
        .root_coordinates_bits
        .iter()
        .zip(&reset.reset_coordinates_bits)
    {
        let root = f64::from_bits(*root_bits);
        let current = f64::from_bits(*reset_bits);
        if !root.is_finite() || !current.is_finite() {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
        }
        let scale = root.abs().max(current.abs()).max(1.0);
        squared += ((current - root) / scale).powi(2);
    }
    let drift = squared.sqrt();
    if !drift.is_finite() {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    if drift == 0.0 {
        return Err(PhaseConsistentCoupledSolveErrorV1::Stagnation);
    }
    Ok(drift)
}

fn covered_parity_monotone_active_set_eligibility_v1(
    windows: &[CoveredParityMonotoneActiveSetResetV1],
    budget: &CoveredPhysicalEvaluationBudgetV1,
    minimum_solver_reserve: usize,
) -> Result<CoveredParityMonotoneActiveSetEligibilityV1, PhaseConsistentCoupledSolveErrorV1> {
    if windows.len() != COVERED_PARITY_MONOTONE_ACTIVE_SET_WINDOWS_REQUIRED
        || budget.maximum != COVERED_PHYSICAL_EVALUATION_LIMIT_V1
        || minimum_solver_reserve == 0
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let first = windows
        .first()
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let minimum_support = crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
    if first.support_start_ns >= first.support_end_ns
        || first.support_end_ns - first.support_start_ns < minimum_support
        || first.reset.root_join_fingerprints.is_empty()
        || first.reset.root_branch_predicates.is_empty()
        || first.reset.branch_entry_vapor_sides.is_empty()
        || first.physical_evaluation_ordinal == 0
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let mut previous_drift = f64::INFINITY;
    for (index, window) in windows.iter().enumerate() {
        let reset = &window.reset;
        let finite_raw_owner = !reset.raw_authentic_continuous_owner_bits.is_empty()
            && reset
                .raw_authentic_continuous_owner_bits
                .iter()
                .all(|bits| f64::from_bits(*bits).is_finite());
        let exact_static = window.support_start_ns == first.support_start_ns
            && window.support_end_ns == first.support_end_ns
            && reset.root_join_fingerprints == reset.reset_join_fingerprints
            && reset.root_join_fingerprints == first.reset.root_join_fingerprints
            && reset.root_branch_predicates == reset.reset_branch_predicates
            && reset.root_branch_predicates == first.reset.root_branch_predicates
            && reset.branch_entry_vapor_sides == first.reset.branch_entry_vapor_sides
            && reset.opposite_raw_vapor_sides == first.reset.opposite_raw_vapor_sides
            && reset.branch_entry_vapor_sides.len() == reset.opposite_raw_vapor_sides.len()
            && reset
                .branch_entry_vapor_sides
                .iter()
                .zip(&reset.opposite_raw_vapor_sides)
                .all(|(entry, opposite)| matches!((*entry, *opposite), (1, -1) | (-1, 1)));
        let exact_cadence = first.physical_evaluation_ordinal.checked_add(2 * index)
            == Some(window.physical_evaluation_ordinal);
        let rolling_chain = index == 0
            || reset.root_coordinates_bits == windows[index - 1].reset.reset_coordinates_bits;
        let parity_nonstagnant = index == 0
            || windows[index - 1].reset.raw_authentic_continuous_owner_bits
                != reset.raw_authentic_continuous_owner_bits;
        let parity_has_no_aba = index < 2
            || windows[index - 2].reset.raw_authentic_continuous_owner_bits
                != reset.raw_authentic_continuous_owner_bits;
        if !exact_static
            || !exact_cadence
            || !rolling_chain
            || !parity_nonstagnant
            || !parity_has_no_aba
            || !finite_raw_owner
            || window.publication_eligible
            || phase_consistent_coupled_active_set_transition_reset_v1(reset)
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        let drift = covered_parity_monotone_active_set_root_drift_v1(reset)?;
        if drift >= previous_drift {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonDescent);
        }
        previous_drift = drift;
    }
    let last = windows
        .last()
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    if last.physical_evaluation_ordinal != budget.used
        || budget.maximum.saturating_sub(budget.used) < minimum_solver_reserve
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
    }
    Ok(CoveredParityMonotoneActiveSetEligibilityV1 {
        seed_coordinates: last
            .reset
            .reset_coordinates_bits
            .iter()
            .map(|bits| f64::from_bits(*bits))
            .collect(),
        reset_windows_observed: windows.len(),
        minimum_solver_reserve,
        publication_eligible: false,
    })
}

fn covered_parity_monotone_active_set_observe_v1(
    trace: &mut Vec<CoveredParityMonotoneActiveSetResetV1>,
    window: CoveredParityMonotoneActiveSetResetV1,
    budget: &CoveredPhysicalEvaluationBudgetV1,
    minimum_solver_reserve: usize,
) -> Result<Option<CoveredParityMonotoneActiveSetEligibilityV1>, PhaseConsistentCoupledSolveErrorV1>
{
    trace.push(window);
    if trace.len() > COVERED_PARITY_MONOTONE_ACTIVE_SET_WINDOWS_REQUIRED {
        trace.remove(0);
    }
    if trace.len() < COVERED_PARITY_MONOTONE_ACTIVE_SET_WINDOWS_REQUIRED {
        return Ok(None);
    }
    covered_parity_monotone_active_set_eligibility_v1(trace, budget, minimum_solver_reserve)
        .map(Some)
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredOneWayPhaseBoundaryEligibilityV1 {
    seed_coordinates: Vec<f64>,
    reset_windows_observed: usize,
    canonical_boundary_crossings: usize,
    minimum_solver_reserve: usize,
    publication_eligible: bool,
}

fn covered_canonical_phase_predicate_v1(
    water_kg_m2: f64,
    enthalpy_j_m2: f64,
) -> Result<u8, PhaseConsistentCoupledSolveErrorV1> {
    if !water_kg_m2.is_finite() || water_kg_m2 < 0.0 || !enthalpy_j_m2.is_finite() {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    let fusion_capacity = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG * water_kg_m2;
    if !fusion_capacity.is_finite() {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    Ok(if enthalpy_j_m2 <= 0.0 {
        0
    } else if enthalpy_j_m2 < fusion_capacity {
        1
    } else {
        2
    })
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredOneWayPhaseBoundaryValidatedTraceV1 {
    enthalpy_points_by_lane: Vec<Vec<f64>>,
    predicate_points_by_lane: Vec<Vec<u8>>,
    seed_coordinates: Vec<f64>,
    reset_windows_observed: usize,
    minimum_solver_reserve: usize,
}

fn covered_one_way_phase_boundary_validated_trace_v1(
    windows: &[CoveredParityMonotoneActiveSetResetV1],
    budget: &CoveredPhysicalEvaluationBudgetV1,
    minimum_solver_reserve: usize,
) -> Result<CoveredOneWayPhaseBoundaryValidatedTraceV1, PhaseConsistentCoupledSolveErrorV1> {
    if windows.len() != COVERED_PARITY_MONOTONE_ACTIVE_SET_WINDOWS_REQUIRED
        || budget.maximum != COVERED_PHYSICAL_EVALUATION_LIMIT_V1
        || minimum_solver_reserve == 0
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let first = windows
        .first()
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let coordinate_count = first.reset.root_coordinates_bits.len();
    if coordinate_count == 0
        || !coordinate_count.is_multiple_of(2)
        || first.reset.root_branch_predicates.len() * 2 != coordinate_count
        || first.support_start_ns >= first.support_end_ns
        || first.support_end_ns - first.support_start_ns
            < crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS
        || first.reset.root_join_fingerprints.is_empty()
        || first.reset.branch_entry_vapor_sides.is_empty()
        || first.physical_evaluation_ordinal == 0
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }

    for (index, window) in windows.iter().enumerate() {
        let reset = &window.reset;
        let exact_static = window.support_start_ns == first.support_start_ns
            && window.support_end_ns == first.support_end_ns
            && reset.root_join_fingerprints == reset.reset_join_fingerprints
            && reset.root_join_fingerprints == first.reset.root_join_fingerprints
            && reset.branch_entry_vapor_sides == first.reset.branch_entry_vapor_sides
            && reset.opposite_raw_vapor_sides == first.reset.opposite_raw_vapor_sides
            && reset.branch_entry_vapor_sides.len() == reset.opposite_raw_vapor_sides.len()
            && reset
                .branch_entry_vapor_sides
                .iter()
                .zip(&reset.opposite_raw_vapor_sides)
                .all(|(entry, opposite)| matches!((*entry, *opposite), (1, -1) | (-1, 1)));
        let exact_cadence = first.physical_evaluation_ordinal.checked_add(2 * index)
            == Some(window.physical_evaluation_ordinal);
        let rolling_chain = index == 0
            || reset.root_coordinates_bits == windows[index - 1].reset.reset_coordinates_bits;
        let finite_raw_owner = !reset.raw_authentic_continuous_owner_bits.is_empty()
            && reset
                .raw_authentic_continuous_owner_bits
                .iter()
                .all(|bits| f64::from_bits(*bits).is_finite());
        let parity_nonstagnant = index == 0
            || windows[index - 1].reset.raw_authentic_continuous_owner_bits
                != reset.raw_authentic_continuous_owner_bits;
        let parity_has_no_aba = index < 2
            || windows[index - 2].reset.raw_authentic_continuous_owner_bits
                != reset.raw_authentic_continuous_owner_bits;
        if !exact_static
            || !exact_cadence
            || !rolling_chain
            || !finite_raw_owner
            || !parity_nonstagnant
            || !parity_has_no_aba
            || window.publication_eligible
            || reset.root_coordinates_bits.len() != coordinate_count
            || reset.reset_coordinates_bits.len() != coordinate_count
            || reset.root_branch_predicates.len() * 2 != coordinate_count
            || reset.reset_branch_predicates.len() * 2 != coordinate_count
            || phase_consistent_coupled_active_set_transition_reset_v1(reset)
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
    }

    let lane_count = coordinate_count / 2;
    let mut enthalpy_points_by_lane = Vec::with_capacity(lane_count);
    let mut predicate_points_by_lane = Vec::with_capacity(lane_count);
    for lane_index in 0..lane_count {
        let coordinate_index = 2 * lane_index;
        let expected_water_bits = first.reset.root_coordinates_bits[coordinate_index];
        let mut enthalpy_points = Vec::with_capacity(windows.len() + 1);
        let mut predicate_points = Vec::with_capacity(windows.len() + 1);
        for (index, window) in windows.iter().enumerate() {
            let reset = &window.reset;
            let root_water_bits = reset.root_coordinates_bits[coordinate_index];
            let reset_water_bits = reset.reset_coordinates_bits[coordinate_index];
            if root_water_bits != expected_water_bits || reset_water_bits != expected_water_bits {
                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
            }
            let root_water = f64::from_bits(root_water_bits);
            let reset_water = f64::from_bits(reset_water_bits);
            let root_enthalpy = f64::from_bits(reset.root_coordinates_bits[coordinate_index + 1]);
            let reset_enthalpy = f64::from_bits(reset.reset_coordinates_bits[coordinate_index + 1]);
            let root_predicate = covered_canonical_phase_predicate_v1(root_water, root_enthalpy)?;
            let reset_predicate =
                covered_canonical_phase_predicate_v1(reset_water, reset_enthalpy)?;
            if root_predicate != reset.root_branch_predicates[lane_index]
                || reset_predicate != reset.reset_branch_predicates[lane_index]
            {
                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
            }
            if index == 0 {
                enthalpy_points.push(root_enthalpy);
                predicate_points.push(root_predicate);
            }
            enthalpy_points.push(reset_enthalpy);
            predicate_points.push(reset_predicate);
        }
        enthalpy_points_by_lane.push(enthalpy_points);
        predicate_points_by_lane.push(predicate_points);
    }
    let last = windows
        .last()
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    if last.physical_evaluation_ordinal != budget.used
        || budget.maximum.saturating_sub(budget.used) < minimum_solver_reserve
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
    }
    Ok(CoveredOneWayPhaseBoundaryValidatedTraceV1 {
        enthalpy_points_by_lane,
        predicate_points_by_lane,
        seed_coordinates: last
            .reset
            .reset_coordinates_bits
            .iter()
            .map(|bits| f64::from_bits(*bits))
            .collect(),
        reset_windows_observed: windows.len(),
        minimum_solver_reserve,
    })
}

fn covered_one_way_phase_boundary_monotone_crossings_v1(
    trace: &CoveredOneWayPhaseBoundaryValidatedTraceV1,
) -> Result<usize, PhaseConsistentCoupledSolveErrorV1> {
    let mut canonical_boundary_crossings = 0_usize;
    let mut shared_direction = None::<std::cmp::Ordering>;
    for (enthalpy_points, predicate_points) in trace
        .enthalpy_points_by_lane
        .iter()
        .zip(&trace.predicate_points_by_lane)
    {
        for index in 0..trace.reset_windows_observed {
            let direction = enthalpy_points[index]
                .partial_cmp(&enthalpy_points[index + 1])
                .ok_or(PhaseConsistentCoupledSolveErrorV1::NonFinite)?;
            if direction == std::cmp::Ordering::Equal {
                return Err(PhaseConsistentCoupledSolveErrorV1::Stagnation);
            }
            if shared_direction.is_some_and(|expected| expected != direction) {
                return Err(PhaseConsistentCoupledSolveErrorV1::NonDescent);
            }
            shared_direction = Some(direction);
            let left_predicate = predicate_points[index];
            let right_predicate = predicate_points[index + 1];
            if left_predicate != right_predicate {
                let adjacent = left_predicate.abs_diff(right_predicate) == 1;
                let follows_direction = matches!(
                    (direction, left_predicate.cmp(&right_predicate)),
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Less)
                        | (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater)
                );
                if !adjacent || !follows_direction {
                    return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
                }
                canonical_boundary_crossings += 1;
            }
        }
    }
    Ok(canonical_boundary_crossings)
}

fn covered_one_way_phase_boundary_eligibility_v1(
    windows: &[CoveredParityMonotoneActiveSetResetV1],
    budget: &CoveredPhysicalEvaluationBudgetV1,
    minimum_solver_reserve: usize,
) -> Result<CoveredOneWayPhaseBoundaryEligibilityV1, PhaseConsistentCoupledSolveErrorV1> {
    let trace =
        covered_one_way_phase_boundary_validated_trace_v1(windows, budget, minimum_solver_reserve)?;
    let canonical_boundary_crossings =
        covered_one_way_phase_boundary_monotone_crossings_v1(&trace)?;
    if canonical_boundary_crossings != 1 {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    Ok(CoveredOneWayPhaseBoundaryEligibilityV1 {
        seed_coordinates: trace.seed_coordinates,
        reset_windows_observed: trace.reset_windows_observed,
        canonical_boundary_crossings,
        minimum_solver_reserve: trace.minimum_solver_reserve,
        publication_eligible: false,
    })
}

fn covered_one_way_post_crossing_contraction_eligibility_v1(
    windows: &[CoveredParityMonotoneActiveSetResetV1],
    budget: &CoveredPhysicalEvaluationBudgetV1,
    minimum_solver_reserve: usize,
) -> Result<CoveredOneWayPhaseBoundaryEligibilityV1, PhaseConsistentCoupledSolveErrorV1> {
    let trace =
        covered_one_way_phase_boundary_validated_trace_v1(windows, budget, minimum_solver_reserve)?;
    if covered_one_way_phase_boundary_monotone_crossings_v1(&trace)
        != Err(PhaseConsistentCoupledSolveErrorV1::NonDescent)
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let mut canonical_boundary_crossings = 0_usize;
    let mut shared_pre_crossing_direction = None::<std::cmp::Ordering>;
    for (enthalpy_points, predicate_points) in trace
        .enthalpy_points_by_lane
        .iter()
        .zip(&trace.predicate_points_by_lane)
    {
        let mut entered_phase = None::<u8>;
        let mut previous_step_magnitude = None::<f64>;
        let mut post_crossing_previous_direction = None::<std::cmp::Ordering>;
        let mut post_crossing_corrections = 0_usize;
        for index in 0..trace.reset_windows_observed {
            let left = enthalpy_points[index];
            let right = enthalpy_points[index + 1];
            let direction = left
                .partial_cmp(&right)
                .ok_or(PhaseConsistentCoupledSolveErrorV1::NonFinite)?;
            if direction == std::cmp::Ordering::Equal {
                return Err(PhaseConsistentCoupledSolveErrorV1::Stagnation);
            }
            let step_magnitude = (right - left).abs();
            if !step_magnitude.is_finite() || step_magnitude == 0.0 {
                return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
            }
            let left_predicate = predicate_points[index];
            let right_predicate = predicate_points[index + 1];
            if left_predicate != right_predicate {
                if entered_phase.is_some()
                    || left_predicate.abs_diff(right_predicate) != 1
                    || !matches!(
                        (direction, left_predicate.cmp(&right_predicate)),
                        (std::cmp::Ordering::Less, std::cmp::Ordering::Less)
                            | (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater)
                    )
                    || shared_pre_crossing_direction.is_some_and(|expected| expected != direction)
                {
                    return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
                }
                shared_pre_crossing_direction = Some(direction);
                entered_phase = Some(right_predicate);
                previous_step_magnitude = Some(step_magnitude);
                canonical_boundary_crossings += 1;
                continue;
            }
            if let Some(phase) = entered_phase {
                if right_predicate != phase
                    || previous_step_magnitude.is_none_or(|previous| step_magnitude >= previous)
                    || post_crossing_previous_direction
                        .is_some_and(|previous| previous == direction)
                {
                    return Err(PhaseConsistentCoupledSolveErrorV1::NonDescent);
                }
                previous_step_magnitude = Some(step_magnitude);
                post_crossing_previous_direction = Some(direction);
                post_crossing_corrections += 1;
            } else {
                if shared_pre_crossing_direction.is_some_and(|expected| expected != direction) {
                    return Err(PhaseConsistentCoupledSolveErrorV1::NonDescent);
                }
                shared_pre_crossing_direction = Some(direction);
            }
        }
        if entered_phase.is_some() && post_crossing_corrections < 2 {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
    }
    if canonical_boundary_crossings != 1 {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    Ok(CoveredOneWayPhaseBoundaryEligibilityV1 {
        seed_coordinates: trace.seed_coordinates,
        reset_windows_observed: trace.reset_windows_observed,
        canonical_boundary_crossings,
        minimum_solver_reserve: trace.minimum_solver_reserve,
        publication_eligible: false,
    })
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredPhaseConsistentResidualInputsV1 {
    coordinates: Vec<f64>,
    beginning_snow_water_kg_m2: Vec<f64>,
    beginning_snow_enthalpy_j_m2: Vec<f64>,
    physical_delta_water_kg_m2: Vec<f64>,
    physical_complete_energy_j_m2: Vec<f64>,
    physical_ice_kg_m2: Vec<f64>,
    physical_density_kg_m3: Vec<f64>,
    physical_thickness_m: Vec<f64>,
    exact_density_settling_branch_satisfied: Vec<bool>,
    beginning_soil_enthalpy_j_m2: Vec<f64>,
    physical_soil_delta_energy_j_m2: Vec<f64>,
    owner_soil_temperature_k: Vec<f64>,
    absolute_tolerances: Vec<f64>,
    algebraic_side_constraints_satisfied: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredPhaseConsistentResidualEvaluationV1 {
    coordinates: Vec<f64>,
    residuals: Vec<f64>,
    absolute_tolerances: Vec<f64>,
    r_w_kg_m2: Vec<f64>,
    r_h_j_m2: Vec<f64>,
    r_rho_kg_m3: Vec<f64>,
    r_q_cn_j_m2: Vec<f64>,
    physical_q_cn_j_m2: Vec<f64>,
    derived_thickness_closures: Vec<CoveredDerivedThicknessClosureV1>,
    r_e_j_m2: Vec<f64>,
    r_t_k: Vec<f64>,
    scaled_merit: f64,
    derived_constraints_scaled_merit: f64,
    algebraic_side_constraints_satisfied: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredConvergenceAdmissionV1 {
    Picard,
    CoupledAuthentic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredPhaseConsistentPhysicalEvaluationKindV1 {
    PrivateTrial,
    ReceiptStabilizationProbe,
    ReceiptStabilizationReplay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredPhaseConsistentCarrierClosurePostureV1 {
    UncommittedPrivateLseExchange,
    StrictAuthenticWeightedOfe,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredPhaseConsistentProjectedSoilConsumptionV1 {
    SnowSoilCnOnly,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CoveredPhaseConsistentCnConsumptionV1 {
    snow_candidate_heat_j_m2_ofe_ground: f64,
    soil_candidate_heat_j_m2_ofe_ground: f64,
}

fn covered_phase_consistent_cn_consumption_v1(
    kind: CoveredPhaseConsistentPhysicalEvaluationKindV1,
    coordinate_trial: Option<&CoveredPhaseConsistentCnTrialOperandV1>,
    sealed_receipt: Option<&SnowSoilHeatReceiptV1>,
) -> Result<CoveredPhaseConsistentCnConsumptionV1, PhaseConsistentCoupledSolveErrorV1> {
    let receipt = sealed_receipt.ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt)
        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::ReplayMismatch)?;
    let trial = coordinate_trial.ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    trial
        .validate_against(receipt)
        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
    match kind {
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial => {
            let snow = trial.snow_candidate_heat_j_m2_ofe_ground;
            let soil = -snow;
            if !soil.is_finite() {
                return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
            }
            Ok(CoveredPhaseConsistentCnConsumptionV1 {
                snow_candidate_heat_j_m2_ofe_ground: snow,
                soil_candidate_heat_j_m2_ofe_ground: soil,
            })
        }
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe
        | CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay => {
            Ok(CoveredPhaseConsistentCnConsumptionV1 {
                snow_candidate_heat_j_m2_ofe_ground: receipt.snow_candidate_heat_j_m2_ofe_ground,
                soil_candidate_heat_j_m2_ofe_ground: receipt.soil_candidate_heat_j_m2_ofe_ground,
            })
        }
    }
}

fn covered_phase_consistent_same_map_cn_heat_seed_v1(
    legacy_coordinates: &[f64],
    lane_ids: &[u32],
    soil_ofe_count: usize,
    endpoint_receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
) -> Result<Vec<f64>, PhaseConsistentCoupledSolveErrorV1> {
    let legacy_lane_coordinate_count = lane_ids
        .len()
        .checked_mul(3)
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let soil_coordinate_count = soil_ofe_count
        .checked_mul(2)
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let legacy_expected = legacy_lane_coordinate_count
        .checked_add(soil_coordinate_count)
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    if legacy_coordinates.len() != legacy_expected
        || endpoint_receipts.len() != lane_ids.len()
        || endpoint_receipts.keys().copied().ne(lane_ids.iter().copied())
        || legacy_coordinates.iter().any(|coordinate| !coordinate.is_finite())
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }

    let complete_expected = lane_ids
        .len()
        .checked_mul(4)
        .and_then(|lane_coordinates| lane_coordinates.checked_add(soil_coordinate_count))
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let mut complete = Vec::with_capacity(complete_expected);
    for (lane_index, lane_id) in lane_ids.iter().enumerate() {
        let receipt = endpoint_receipts
            .get(lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt)
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        let q = receipt.snow_candidate_heat_j_m2_ofe_ground;
        if receipt.lane_id != *lane_id || !q.is_finite() {
            return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
        }
        let lane_offset = lane_index
            .checked_mul(3)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        complete.extend_from_slice(&legacy_coordinates[lane_offset..lane_offset + 3]);
        complete.push(q);
    }
    complete.extend_from_slice(&legacy_coordinates[legacy_lane_coordinate_count..]);
    if complete.len() != complete_expected {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    Ok(complete)
}

fn covered_phase_consistent_carrier_closure_posture_v1(
    kind: CoveredPhaseConsistentPhysicalEvaluationKindV1,
) -> CoveredPhaseConsistentCarrierClosurePostureV1 {
    match kind {
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial => {
            CoveredPhaseConsistentCarrierClosurePostureV1::UncommittedPrivateLseExchange
        }
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe
        | CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay => {
            CoveredPhaseConsistentCarrierClosurePostureV1::StrictAuthenticWeightedOfe
        }
    }
}

fn covered_phase_consistent_carrier_input_exchange_v1(
    kind: CoveredPhaseConsistentPhysicalEvaluationKindV1,
    private_input: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    corrected_authentic_input: Option<&BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>>,
) -> Result<
    BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    PhaseConsistentCoupledSolveErrorV1,
> {
    match kind {
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial => Ok(private_input.clone()),
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe
        | CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay => {
            corrected_authentic_input
                .cloned()
                .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)
        }
    }
}

impl CoveredPhaseConsistentCarrierClosurePostureV1 {
    const fn carrier_is_provisional(self) -> bool {
        matches!(self, Self::UncommittedPrivateLseExchange)
    }

    const fn requires_strict_weighted_ofe_closure(self) -> bool {
        matches!(self, Self::StrictAuthenticWeightedOfe)
    }
}

impl CoveredPhaseConsistentProjectedSoilConsumptionV1 {
    fn cn_heat_coordinate(
        self,
        coordinates: &[f64],
        lane_index: usize,
    ) -> Result<f64, PhaseConsistentCoupledSolveErrorV1> {
        let coordinate = 4_usize
            .checked_mul(lane_index)
            .and_then(|offset| offset.checked_add(3))
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        let heat = coordinates
            .get(coordinate)
            .copied()
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        if !heat.is_finite() {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
        }
        Ok(heat)
    }

    fn cn_heat_coordinate_trial_operand(
        self,
        coordinates: &[f64],
        lane_index: usize,
        receipt: &SnowSoilHeatReceiptV1,
    ) -> Result<CoveredPhaseConsistentCnTrialOperandV1, PhaseConsistentCoupledSolveErrorV1> {
        CoveredPhaseConsistentCnTrialOperandV1::from_sealed_receipt(
            receipt,
            self.cn_heat_coordinate(coordinates, lane_index)?,
        )
        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    }

    fn cn_temperature_coordinate(
        self,
        coordinates: &[f64],
        snow_lane_count: usize,
        soil_index: usize,
    ) -> Result<f64, PhaseConsistentCoupledSolveErrorV1> {
        let coordinate = 3_usize
            .checked_mul(snow_lane_count)
            .and_then(|offset| {
                2_usize
                    .checked_mul(soil_index)
                    .and_then(|soil_offset| offset.checked_add(soil_offset))
            })
            .and_then(|offset| offset.checked_add(1))
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        let temperature = coordinates
            .get(coordinate)
            .copied()
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        if !temperature.is_finite() {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
        }
        Ok(temperature)
    }

    fn cn_trial_operand(
        self,
        coordinates: &[f64],
        snow_lane_count: usize,
        soil_index: usize,
        receipt: &SnowSoilHeatReceiptV1,
        ending_snow_temperature_k: f64,
        interval_s: f64,
    ) -> Result<CoveredPhaseConsistentCnTrialOperandV1, PhaseConsistentCoupledSolveErrorV1> {
        let ending_soil_temperature_k =
            self.cn_temperature_coordinate(coordinates, snow_lane_count, soil_index)?;
        if !ending_snow_temperature_k.is_finite() || !interval_s.is_finite() || interval_s <= 0.0 {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
        }
        let (_, _, heat_w_m2) = crate::snow_stage3_v11_attachment::snow_soil_heat_w_m2_ofe_ground(
            receipt.bottom_snow_half_thickness_m,
            receipt.bottom_snow_conductivity_w_m_k,
            receipt.top_soil_half_thickness_m,
            receipt.top_soil_conductivity_w_m_k,
            receipt.beginning_bottom_snow_temperature_k,
            receipt.beginning_top_soil_temperature_k,
            ending_snow_temperature_k,
            ending_soil_temperature_k,
        )
        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
        CoveredPhaseConsistentCnTrialOperandV1::from_sealed_receipt(
            receipt,
            -heat_w_m2 * interval_s,
        )
        .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    }
}

fn covered_phase_consistent_projected_soil_exact_once_v1(
    expected_cn_lanes: &[u32],
    consumed_cn_lanes: &[u32],
    stage3_v8_soil_ground_heat_or_storage_lanes: &[u32],
) -> Result<(), PhaseConsistentCoupledSolveErrorV1> {
    if expected_cn_lanes != consumed_cn_lanes
        || !stage3_v8_soil_ground_heat_or_storage_lanes.is_empty()
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    Ok(())
}

impl CoveredPhaseConsistentPhysicalEvaluationKindV1 {
    const fn requires_authentic_receipts(self) -> bool {
        matches!(
            self,
            Self::ReceiptStabilizationProbe | Self::ReceiptStabilizationReplay
        )
    }
}

impl CoveredConvergenceAdmissionV1 {
    fn admits(
        self,
        picard_equal_and_converged: bool,
        residuals_and_side_constraints_closed: bool,
        fresh_authentic_replay_resealed: bool,
    ) -> bool {
        match self {
            CoveredConvergenceAdmissionV1::Picard => picard_equal_and_converged,
            CoveredConvergenceAdmissionV1::CoupledAuthentic => {
                residuals_and_side_constraints_closed && fresh_authentic_replay_resealed
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg(test)]
struct PhaseConsistentCoupledSolveV1 {
    root: CoveredPhaseConsistentResidualEvaluationV1,
    evaluations_used: usize,
    trust_radius: f64,
    publication_eligible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PhaseConsistentCoupledSolveErrorV1 {
    Structure,
    NonFinite,
    SingularGeneralizedSystem,
    NonDescent,
    Stagnation,
    SideConstraint,
    EvaluationBudget,
    ReplayMismatch,
    ReceiptOscillation,
    PrivateQLatticeInterval,
    PrivateQLatticeNoWitness,
}

#[cfg(test)]
fn covered_phase_consistent_residual_evaluate_v1(
    inputs: CoveredPhaseConsistentResidualInputsV1,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
) -> Result<CoveredPhaseConsistentResidualEvaluationV1, PhaseConsistentCoupledSolveErrorV1> {
    covered_physical_evaluation_budget_charge_v1(budget)?;
    covered_phase_consistent_residual_assemble_v1(inputs)
}

fn covered_phase_consistent_residual_assemble_v1(
    inputs: CoveredPhaseConsistentResidualInputsV1,
) -> Result<CoveredPhaseConsistentResidualEvaluationV1, PhaseConsistentCoupledSolveErrorV1> {
    if !inputs.algebraic_side_constraints_satisfied {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let lanes = inputs.beginning_snow_water_kg_m2.len();
    let soils = inputs.beginning_soil_enthalpy_j_m2.len();
    let expected = 3 * lanes + 2 * soils;
    if lanes == 0
        || inputs.coordinates.len() != expected
        || inputs.beginning_snow_enthalpy_j_m2.len() != lanes
        || inputs.physical_delta_water_kg_m2.len() != lanes
        || inputs.physical_complete_energy_j_m2.len() != lanes
        || inputs.physical_ice_kg_m2.len() != lanes
        || inputs.physical_density_kg_m3.len() != lanes
        || inputs.physical_thickness_m.len() != lanes
        || inputs.exact_density_settling_branch_satisfied.len() != lanes
        || inputs.physical_soil_delta_energy_j_m2.len() != soils
        || inputs.owner_soil_temperature_k.len() != soils
        || inputs.absolute_tolerances.len() != expected
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    if inputs
        .coordinates
        .iter()
        .chain(&inputs.beginning_snow_water_kg_m2)
        .chain(&inputs.beginning_snow_enthalpy_j_m2)
        .chain(&inputs.physical_delta_water_kg_m2)
        .chain(&inputs.physical_complete_energy_j_m2)
        .chain(&inputs.physical_ice_kg_m2)
        .chain(&inputs.physical_density_kg_m3)
        .chain(&inputs.physical_thickness_m)
        .chain(&inputs.beginning_soil_enthalpy_j_m2)
        .chain(&inputs.physical_soil_delta_energy_j_m2)
        .chain(&inputs.owner_soil_temperature_k)
        .chain(&inputs.absolute_tolerances)
        .any(|value| !value.is_finite())
        || inputs.absolute_tolerances.iter().any(|value| *value <= 0.0)
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    let mut r_w_kg_m2 = Vec::with_capacity(lanes);
    let mut r_h_j_m2 = Vec::with_capacity(lanes);
    let mut r_rho_kg_m3 = Vec::with_capacity(lanes);
    let mut derived_thickness_closures = Vec::with_capacity(lanes);
    for lane in 0..lanes {
        let phase = phase_consistent_canonical_phase_projection_v1(
            inputs.coordinates[3 * lane],
            inputs.coordinates[3 * lane + 1],
            inputs.coordinates[3 * lane + 2],
        )?;
        let geometry = CoveredTerminalDensityGeometryCoordinateV1::from_canonical_phase(&phase)?;
        r_w_kg_m2.push(
            inputs.coordinates[3 * lane]
                - inputs.beginning_snow_water_kg_m2[lane]
                - inputs.physical_delta_water_kg_m2[lane],
        );
        r_h_j_m2.push(
            inputs.coordinates[3 * lane + 1]
                - inputs.beginning_snow_enthalpy_j_m2[lane]
                - inputs.physical_complete_energy_j_m2[lane],
        );
        r_rho_kg_m3.push(covered_terminal_density_geometry_residual_evaluate_v1(
            geometry,
            inputs.physical_density_kg_m3[lane],
            inputs.exact_density_settling_branch_satisfied[lane],
        )?);
        derived_thickness_closures.push(covered_derived_thickness_closure_evaluate_v1(
            geometry,
            inputs.physical_ice_kg_m2[lane],
            inputs.physical_density_kg_m3[lane],
            inputs.physical_thickness_m[lane],
        )?);
    }
    let mut r_e_j_m2 = Vec::with_capacity(soils);
    let mut r_t_k = Vec::with_capacity(soils);
    for soil in 0..soils {
        let coordinate = 3 * lanes + 2 * soil;
        r_e_j_m2.push(
            inputs.coordinates[coordinate]
                - inputs.beginning_soil_enthalpy_j_m2[soil]
                - inputs.physical_soil_delta_energy_j_m2[soil],
        );
        r_t_k.push(inputs.coordinates[coordinate + 1] - inputs.owner_soil_temperature_k[soil]);
    }
    let mut residuals = Vec::with_capacity(expected);
    for lane in 0..lanes {
        residuals.extend([r_w_kg_m2[lane], r_h_j_m2[lane], r_rho_kg_m3[lane]]);
    }
    for soil in 0..soils {
        residuals.extend([r_e_j_m2[soil], r_t_k[soil]]);
    }
    let residual_scaled_merit = residuals
        .iter()
        .zip(&inputs.absolute_tolerances)
        .map(|(residual, tolerance)| residual.abs() / tolerance)
        .fold(0.0_f64, f64::max);
    let derived_constraints_scaled_merit = derived_thickness_closures
        .iter()
        .map(|closure| closure.scaled_merit)
        .fold(0.0_f64, f64::max);
    let scaled_merit = residual_scaled_merit.max(derived_constraints_scaled_merit);
    if !scaled_merit.is_finite() || residuals.iter().any(|value| !value.is_finite()) {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    Ok(CoveredPhaseConsistentResidualEvaluationV1 {
        coordinates: inputs.coordinates,
        residuals,
        absolute_tolerances: inputs.absolute_tolerances,
        r_w_kg_m2,
        r_h_j_m2,
        r_rho_kg_m3,
        r_q_cn_j_m2: Vec::new(),
        physical_q_cn_j_m2: Vec::new(),
        derived_thickness_closures,
        r_e_j_m2,
        r_t_k,
        scaled_merit,
        derived_constraints_scaled_merit,
        algebraic_side_constraints_satisfied: true,
    })
}

fn covered_cn_heat_coordinate_residual_evaluate_v1(
    base: CoveredPhaseConsistentResidualEvaluationV1,
    coordinates: Vec<f64>,
    snow_candidate_cn_heat_j_m2: &[f64],
    physical_snow_candidate_cn_heat_j_m2: &[f64],
    absolute_tolerances: Vec<f64>,
) -> Result<CoveredPhaseConsistentResidualEvaluationV1, PhaseConsistentCoupledSolveErrorV1> {
    let lanes = base.r_w_kg_m2.len();
    let soils = base.r_e_j_m2.len();
    let expected = 4 * lanes + 2 * soils;
    if lanes == 0
        || base.r_h_j_m2.len() != lanes
        || base.r_rho_kg_m3.len() != lanes
        || base.derived_thickness_closures.len() != lanes
        || base.r_t_k.len() != soils
        || base.coordinates.len() != 3 * lanes + 2 * soils
        || base.residuals.len() != base.coordinates.len()
        || coordinates.len() != expected
        || absolute_tolerances.len() != expected
        || snow_candidate_cn_heat_j_m2.len() != lanes
        || physical_snow_candidate_cn_heat_j_m2.len() != lanes
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    if coordinates
        .iter()
        .chain(&absolute_tolerances)
        .chain(snow_candidate_cn_heat_j_m2)
        .chain(physical_snow_candidate_cn_heat_j_m2)
        .any(|value| !value.is_finite())
        || absolute_tolerances.iter().any(|value| *value <= 0.0)
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    for lane in 0..lanes {
        let base_coordinate = 3 * lane;
        let coordinate = 4 * lane;
        if coordinates[coordinate].to_bits() != base.coordinates[base_coordinate].to_bits()
            || coordinates[coordinate + 1].to_bits()
                != base.coordinates[base_coordinate + 1].to_bits()
            || coordinates[coordinate + 2].to_bits()
                != base.coordinates[base_coordinate + 2].to_bits()
            || coordinates[coordinate + 3].to_bits() != snow_candidate_cn_heat_j_m2[lane].to_bits()
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
        }
    }
    let base_soil_offset = 3 * lanes;
    let soil_offset = 4 * lanes;
    if coordinates[soil_offset..]
        .iter()
        .zip(&base.coordinates[base_soil_offset..])
        .any(|(coordinate, base_coordinate)| coordinate.to_bits() != base_coordinate.to_bits())
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }

    let r_q_cn_j_m2 = snow_candidate_cn_heat_j_m2
        .iter()
        .zip(physical_snow_candidate_cn_heat_j_m2)
        .map(|(coordinate, physical)| {
            if coordinate == physical && coordinate.to_bits() != physical.to_bits() {
                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
            }
            Ok(coordinate - physical)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut residuals = Vec::with_capacity(expected);
    for lane in 0..lanes {
        residuals.extend([
            base.r_w_kg_m2[lane],
            base.r_h_j_m2[lane],
            base.r_rho_kg_m3[lane],
            r_q_cn_j_m2[lane],
        ]);
    }
    for soil in 0..soils {
        residuals.extend([base.r_e_j_m2[soil], base.r_t_k[soil]]);
    }
    let residual_scaled_merit = residuals
        .iter()
        .zip(&absolute_tolerances)
        .map(|(residual, tolerance)| residual.abs() / tolerance)
        .fold(0.0_f64, f64::max);
    let scaled_merit = residual_scaled_merit.max(base.derived_constraints_scaled_merit);
    if !scaled_merit.is_finite() || residuals.iter().any(|value| !value.is_finite()) {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    Ok(CoveredPhaseConsistentResidualEvaluationV1 {
        coordinates,
        residuals,
        absolute_tolerances,
        r_q_cn_j_m2,
        physical_q_cn_j_m2: physical_snow_candidate_cn_heat_j_m2.to_vec(),
        scaled_merit,
        ..base
    })
}

fn solve_dense_linear_system_v1(
    mut matrix: Vec<Vec<f64>>,
    mut rhs: Vec<f64>,
) -> Result<Vec<f64>, PhaseConsistentCoupledSolveErrorV1> {
    let n = rhs.len();
    if n == 0 || matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    for pivot in 0..n {
        let row = (pivot..n)
            .max_by(|left, right| {
                matrix[*left][pivot]
                    .abs()
                    .total_cmp(&matrix[*right][pivot].abs())
            })
            .ok_or(PhaseConsistentCoupledSolveErrorV1::SingularGeneralizedSystem)?;
        if matrix[row][pivot].abs() <= f64::EPSILON {
            return Err(PhaseConsistentCoupledSolveErrorV1::SingularGeneralizedSystem);
        }
        matrix.swap(pivot, row);
        rhs.swap(pivot, row);
        for row in (pivot + 1)..n {
            let factor = matrix[row][pivot] / matrix[pivot][pivot];
            for column in pivot..n {
                matrix[row][column] -= factor * matrix[pivot][column];
            }
            rhs[row] -= factor * rhs[pivot];
        }
    }
    let mut solution = vec![0.0; n];
    for row in (0..n).rev() {
        let trailing = ((row + 1)..n)
            .map(|column| matrix[row][column] * solution[column])
            .sum::<f64>();
        solution[row] = (rhs[row] - trailing) / matrix[row][row];
        if !solution[row].is_finite() {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
        }
    }
    Ok(solution)
}

trait CoveredSafeguardedEvaluationV1: Clone {
    fn residual(&self) -> &CoveredPhaseConsistentResidualEvaluationV1;

    fn validate(
        &self,
        expected_coordinates: &[f64],
        expected_ordinal: Option<usize>,
        expected_branch: Option<&CoveredPhaseConsistentPhysicalBranchIdentityV1>,
    ) -> Result<(), PhaseConsistentCoupledSolveErrorV1>;
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredSafeguardedSolveStateV1<T> {
    root: T,
    evaluations_used: usize,
    trust_radius: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum CoveredSafeguardedStepV1<T> {
    Admitted { evaluation: T, trust_radius: f64 },
    Stagnation,
    NonDescent,
    ReceiptEntryReserve,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CoveredSafeguardedCompleteStepCapacityV1 {
    dimension: usize,
    minimum_physical_evaluations: usize,
    reserve_after_step: usize,
}

fn covered_safeguarded_complete_step_capacity_v1(
    dimension: usize,
    budget: &CoveredPhysicalEvaluationBudgetV1,
    reserve_after_step: usize,
) -> Result<CoveredSafeguardedCompleteStepCapacityV1, PhaseConsistentCoupledSolveErrorV1> {
    if dimension == 0 {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let minimum_physical_evaluations = dimension
        .checked_add(1)
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let required_capacity = minimum_physical_evaluations
        .checked_add(reserve_after_step)
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    if budget.maximum != COVERED_PHYSICAL_EVALUATION_LIMIT_V1
        || budget.used > budget.maximum
        || budget.maximum - budget.used < required_capacity
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
    }
    Ok(CoveredSafeguardedCompleteStepCapacityV1 {
        dimension,
        minimum_physical_evaluations,
        reserve_after_step,
    })
}

impl CoveredSafeguardedEvaluationV1 for CoveredPhaseConsistentResidualEvaluationV1 {
    fn residual(&self) -> &CoveredPhaseConsistentResidualEvaluationV1 {
        self
    }

    fn validate(
        &self,
        expected_coordinates: &[f64],
        _expected_ordinal: Option<usize>,
        _expected_branch: Option<&CoveredPhaseConsistentPhysicalBranchIdentityV1>,
    ) -> Result<(), PhaseConsistentCoupledSolveErrorV1> {
        if expected_coordinates.len() != self.coordinates.len()
            || self.residuals.len() != self.coordinates.len()
            || self.absolute_tolerances.len() != self.coordinates.len()
            || expected_coordinates
                .iter()
                .zip(&self.coordinates)
                .any(|(expected, actual)| expected.to_bits() != actual.to_bits())
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
        }
        if !covered_phase_consistent_residual_finite_v1(self) {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
        }
        if !self.algebraic_side_constraints_satisfied {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        Ok(())
    }
}

fn covered_safeguarded_evaluate_v1<T, F>(
    coordinates: &[f64],
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    reserve_after_charge: Option<usize>,
    reserve_is_private_stop: bool,
    expected_branch: Option<&CoveredPhaseConsistentPhysicalBranchIdentityV1>,
    evaluate: &mut F,
) -> Result<Option<T>, PhaseConsistentCoupledSolveErrorV1>
where
    T: CoveredSafeguardedEvaluationV1,
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<T, PhaseConsistentCoupledSolveErrorV1>,
{
    if let Some(required_after_charge) = reserve_after_charge {
        if let Err(error) =
            covered_physical_evaluation_budget_preserve_v1(budget, required_after_charge)
        {
            return if reserve_is_private_stop {
                Ok(None)
            } else {
                Err(error)
            };
        }
    }
    let before = budget.used;
    let result = evaluate(coordinates, budget);
    if budget.used != before.saturating_add(1) {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let evaluation = result?;
    evaluation.validate(coordinates, Some(budget.used), expected_branch)?;
    Ok(Some(evaluation))
}

fn phase_consistent_coupled_safeguarded_step_v1<T, F>(
    current: &T,
    mut trust_radius: f64,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    reserve_after_charge: Option<usize>,
    reserve_is_private_stop: bool,
    expected_branch: Option<&CoveredPhaseConsistentPhysicalBranchIdentityV1>,
    evaluate: &mut F,
) -> Result<CoveredSafeguardedStepV1<T>, PhaseConsistentCoupledSolveErrorV1>
where
    T: CoveredSafeguardedEvaluationV1,
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<T, PhaseConsistentCoupledSolveErrorV1>,
{
    let residual = current.residual();
    let dimension = residual.coordinates.len();
    if dimension == 0
        || residual.residuals.len() != dimension
        || residual.absolute_tolerances.len() != dimension
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let reserve_after_step = reserve_after_charge.unwrap_or(0);
    if let Err(error) =
        covered_safeguarded_complete_step_capacity_v1(dimension, budget, reserve_after_step)
    {
        return if reserve_is_private_stop
            && error == PhaseConsistentCoupledSolveErrorV1::EvaluationBudget
        {
            Ok(CoveredSafeguardedStepV1::ReceiptEntryReserve)
        } else {
            Err(error)
        };
    }
    let mut jacobian = vec![vec![0.0; dimension]; dimension];
    for column in 0..dimension {
        let perturbation = (residual.coordinates[column].abs() * f64::EPSILON.sqrt())
            .max(residual.absolute_tolerances[column]);
        let mut coordinates = residual.coordinates.clone();
        coordinates[column] += perturbation;
        let (perturbed, signed_perturbation) = match covered_safeguarded_evaluate_v1(
            &coordinates,
            budget,
            reserve_after_charge,
            reserve_is_private_stop,
            expected_branch,
            evaluate,
        ) {
            Ok(Some(perturbed)) => (perturbed, perturbation),
            Ok(None) => return Ok(CoveredSafeguardedStepV1::ReceiptEntryReserve),
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint) => {
                coordinates[column] = residual.coordinates[column] - perturbation;
                match covered_safeguarded_evaluate_v1(
                    &coordinates,
                    budget,
                    reserve_after_charge,
                    reserve_is_private_stop,
                    expected_branch,
                    evaluate,
                )? {
                    Some(perturbed) => (perturbed, -perturbation),
                    None => return Ok(CoveredSafeguardedStepV1::ReceiptEntryReserve),
                }
            }
            Err(error) => return Err(error),
        };
        for row in 0..dimension {
            jacobian[row][column] = (perturbed.residual().residuals[row] - residual.residuals[row])
                / signed_perturbation;
        }
    }
    let rhs = residual.residuals.iter().map(|value| -*value).collect();
    let direction = solve_dense_linear_system_v1(jacobian, rhs)?;
    let direction_norm = direction
        .iter()
        .zip(&residual.coordinates)
        .map(|(value, coordinate)| (value / coordinate.abs().max(1.0)).powi(2))
        .sum::<f64>()
        .sqrt();
    if !direction_norm.is_finite() {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    if direction_norm == 0.0 {
        return Ok(CoveredSafeguardedStepV1::Stagnation);
    }
    for _ in 0..12 {
        let scale = (trust_radius / direction_norm).min(1.0);
        let coordinates = residual
            .coordinates
            .iter()
            .zip(&direction)
            .map(|(coordinate, direction)| coordinate + scale * direction)
            .collect::<Vec<_>>();
        let trial = match covered_safeguarded_evaluate_v1(
            &coordinates,
            budget,
            reserve_after_charge,
            reserve_is_private_stop,
            expected_branch,
            evaluate,
        ) {
            Ok(Some(trial)) => trial,
            Ok(None) => return Ok(CoveredSafeguardedStepV1::ReceiptEntryReserve),
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint) => {
                trust_radius *= 0.5;
                continue;
            }
            Err(error) => return Err(error),
        };
        if trial.residual().scaled_merit < residual.scaled_merit {
            return Ok(CoveredSafeguardedStepV1::Admitted {
                evaluation: trial,
                trust_radius: (2.0 * trust_radius).min(4.0),
            });
        }
        trust_radius *= 0.5;
    }
    Ok(CoveredSafeguardedStepV1::NonDescent)
}

fn phase_consistent_coupled_solve_engine_v1<T, F>(
    initial_coordinates: Vec<f64>,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    reserve_after_charge: Option<usize>,
    mut evaluate: F,
) -> Result<CoveredSafeguardedSolveStateV1<T>, PhaseConsistentCoupledSolveErrorV1>
where
    T: CoveredSafeguardedEvaluationV1,
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<T, PhaseConsistentCoupledSolveErrorV1>,
{
    let starting_used = budget.used;
    let mut current = covered_safeguarded_evaluate_v1(
        &initial_coordinates,
        budget,
        reserve_after_charge,
        false,
        None,
        &mut evaluate,
    )?
    .ok_or(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)?;
    let mut trust_radius = 1.0_f64;
    while current.residual().scaled_merit > 1.0 {
        match phase_consistent_coupled_safeguarded_step_v1(
            &current,
            trust_radius,
            budget,
            reserve_after_charge,
            false,
            None,
            &mut evaluate,
        )? {
            CoveredSafeguardedStepV1::Admitted {
                evaluation,
                trust_radius: next_trust_radius,
            } => {
                current = evaluation;
                trust_radius = next_trust_radius;
            }
            CoveredSafeguardedStepV1::Stagnation => {
                return Err(PhaseConsistentCoupledSolveErrorV1::Stagnation);
            }
            CoveredSafeguardedStepV1::NonDescent => {
                return Err(PhaseConsistentCoupledSolveErrorV1::NonDescent);
            }
            CoveredSafeguardedStepV1::ReceiptEntryReserve => {
                return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
            }
        }
    }
    Ok(CoveredSafeguardedSolveStateV1 {
        root: current,
        evaluations_used: budget.used - starting_used,
        trust_radius,
    })
}

#[cfg(test)]
fn phase_consistent_coupled_solve_v1<F>(
    initial_coordinates: Vec<f64>,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    evaluate: F,
) -> Result<PhaseConsistentCoupledSolveV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        CoveredPhaseConsistentResidualEvaluationV1,
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    let state =
        phase_consistent_coupled_solve_engine_v1(initial_coordinates, budget, None, evaluate)?;
    Ok(PhaseConsistentCoupledSolveV1 {
        root: state.root,
        evaluations_used: state.evaluations_used,
        trust_radius: state.trust_radius,
        publication_eligible: false,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CoveredPhaseConsistentPhysicalBranchIdentityV1 {
    phase_branch: Vec<u8>,
    density_model_branch: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredPhaseConsistentPhysicalEvaluationV1 {
    residual: CoveredPhaseConsistentResidualEvaluationV1,
    artifacts: CoveredPhaseConsistentPhysicalArtifactsV1,
    finalization_inputs: CoveredFinalizationEquivalentReplayInputsV1,
    branch_identity: CoveredPhaseConsistentPhysicalBranchIdentityV1,
    coordinate_posture: CoveredPhaseConsistentCoordinatePostureV1,
    physical_evaluation_ordinal: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredCoupledPolishStopV1 {
    ExactResidualVector,
    PrivateQLatticeExactWitness,
    SubToleranceNonDescent,
    SubToleranceStagnation,
    ReceiptEntryReserve,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredCoupledPolishedRootV1 {
    evaluation: CoveredPhaseConsistentPhysicalEvaluationV1,
    stop: CoveredCoupledPolishStopV1,
    evaluations_used: usize,
    publication_eligible: bool,
}

fn covered_phase_consistent_physical_evaluation_validate_v1(
    expected_coordinates: &[f64],
    evaluation: &CoveredPhaseConsistentPhysicalEvaluationV1,
    expected_ordinal: Option<usize>,
    expected_branch: Option<&CoveredPhaseConsistentPhysicalBranchIdentityV1>,
) -> Result<(), PhaseConsistentCoupledSolveErrorV1> {
    evaluation
        .residual
        .validate(expected_coordinates, None, None)?;
    let lane_count = evaluation.residual.r_w_kg_m2.len();
    if lane_count == 0
        || evaluation.residual.r_h_j_m2.len() != lane_count
        || evaluation.residual.r_rho_kg_m3.len() != lane_count
        || (!evaluation.residual.r_q_cn_j_m2.is_empty()
            && evaluation.residual.r_q_cn_j_m2.len() != lane_count)
        || evaluation.residual.physical_q_cn_j_m2.len()
            != evaluation.residual.r_q_cn_j_m2.len()
        || evaluation.branch_identity.phase_branch.len() != lane_count
        || evaluation.branch_identity.density_model_branch.is_empty()
        || expected_ordinal.is_some_and(|ordinal| evaluation.physical_evaluation_ordinal != ordinal)
        || evaluation.finalization_inputs.proposed_stage3.len()
            != evaluation.artifacts.stage3_candidate.len()
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    if expected_branch.is_some_and(|branch| branch != &evaluation.branch_identity) {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let lane_stride = match evaluation.coordinate_posture {
        CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat => {
            if evaluation.residual.r_q_cn_j_m2.len() != lane_count {
                return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
            }
            4
        }
        CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary => {
            if !evaluation.residual.r_q_cn_j_m2.is_empty()
                || !evaluation.residual.physical_q_cn_j_m2.is_empty()
            {
                return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
            }
            3
        }
    };
    for lane in 0..lane_count {
        let coordinate = lane_stride * lane;
        let predicate = match evaluation.coordinate_posture {
            CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat => {
                covered_canonical_phase_predicate_v1(
                    evaluation.residual.coordinates[coordinate],
                    evaluation.residual.coordinates[coordinate + 1],
                )?
            }
            CoveredPhaseConsistentCoordinatePostureV1::FrozenTemperaturePrimary => {
                let water = evaluation.residual.coordinates[coordinate];
                let temperature = evaluation.residual.coordinates[coordinate + 1];
                let density = evaluation.residual.coordinates[coordinate + 2];
                if !water.is_finite()
                    || water <= 0.0
                    || !temperature.is_finite()
                    || !(0.0..273.15).contains(&temperature)
                    || !density.is_finite()
                    || density <= 0.0
                {
                    return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
                }
                0
            }
        };
        if predicate != evaluation.branch_identity.phase_branch[lane] {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
    }
    Ok(())
}

impl CoveredSafeguardedEvaluationV1 for CoveredPhaseConsistentPhysicalEvaluationV1 {
    fn residual(&self) -> &CoveredPhaseConsistentResidualEvaluationV1 {
        &self.residual
    }

    fn validate(
        &self,
        expected_coordinates: &[f64],
        expected_ordinal: Option<usize>,
        expected_branch: Option<&CoveredPhaseConsistentPhysicalBranchIdentityV1>,
    ) -> Result<(), PhaseConsistentCoupledSolveErrorV1> {
        covered_phase_consistent_physical_evaluation_validate_v1(
            expected_coordinates,
            self,
            expected_ordinal,
            expected_branch,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredPhaseConsistentPhysicalSolveV1 {
    root: CoveredPhaseConsistentPhysicalEvaluationV1,
    evaluations_used: usize,
    trust_radius: f64,
    publication_eligible: bool,
}

fn phase_consistent_coupled_physical_solve_v1<F>(
    initial_coordinates: Vec<f64>,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    evaluate: F,
) -> Result<CoveredPhaseConsistentPhysicalSolveV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        CoveredPhaseConsistentPhysicalEvaluationV1,
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    let state = phase_consistent_coupled_solve_engine_v1(
        initial_coordinates,
        budget,
        Some(COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1),
        evaluate,
    )?;
    Ok(CoveredPhaseConsistentPhysicalSolveV1 {
        root: state.root,
        evaluations_used: state.evaluations_used,
        trust_radius: state.trust_radius,
        publication_eligible: false,
    })
}

fn covered_phase_consistent_residual_is_exact_zero_v1(
    residual: &CoveredPhaseConsistentResidualEvaluationV1,
) -> bool {
    residual
        .residuals
        .iter()
        .all(|value| value.to_bits() == 0.0_f64.to_bits())
        && residual
            .derived_thickness_closures
            .iter()
            .all(|closure| closure.r_z_m.to_bits() == 0.0_f64.to_bits())
}

fn covered_coupled_polished_root_v1(
    evaluation: CoveredPhaseConsistentPhysicalEvaluationV1,
    stop: CoveredCoupledPolishStopV1,
    starting_used: usize,
    budget: &CoveredPhysicalEvaluationBudgetV1,
) -> CoveredCoupledPolishedRootV1 {
    CoveredCoupledPolishedRootV1 {
        evaluation,
        stop,
        evaluations_used: budget.used.saturating_sub(starting_used),
        publication_eligible: false,
    }
}

include!("phase_consistent_temperature_primary.rs");
include!("phase_consistent_private_q_lattice.rs");

fn phase_consistent_coupled_root_polish_v1<F>(
    initial_evaluation: CoveredPhaseConsistentPhysicalEvaluationV1,
    initial_trust_radius: f64,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    mut evaluate: F,
) -> Result<CoveredCoupledPolishedRootV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        CoveredPhaseConsistentPhysicalEvaluationV1,
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    let starting_used = budget.used;
    let initial_coordinates = initial_evaluation.residual.coordinates.clone();
    covered_phase_consistent_physical_evaluation_validate_v1(
        &initial_coordinates,
        &initial_evaluation,
        Some(budget.used),
        None,
    )?;
    if !initial_trust_radius.is_finite() || initial_trust_radius <= 0.0 {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let root_branch = initial_evaluation.branch_identity.clone();
    let mut current = initial_evaluation;
    if current.residual.scaled_merit > 1.0 {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let dimension = current.residual.coordinates.len();
    if dimension == 0 || current.residual.residuals.len() != dimension {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    if current.coordinate_posture
        == CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat
    {
        if let Some(witness) = covered_private_q_lattice_witness_v1(
            &current,
            budget,
            &root_branch,
            &mut evaluate,
        )? {
            return Ok(covered_coupled_polished_root_v1(
                witness,
                CoveredCoupledPolishStopV1::PrivateQLatticeExactWitness,
                starting_used,
                budget,
            ));
        }
    }
    let mut trust_radius = initial_trust_radius;
    loop {
        if covered_phase_consistent_residual_is_exact_zero_v1(&current.residual) {
            return Ok(covered_coupled_polished_root_v1(
                current,
                CoveredCoupledPolishStopV1::ExactResidualVector,
                starting_used,
                budget,
            ));
        }
        match phase_consistent_coupled_safeguarded_step_v1(
            &current,
            trust_radius,
            budget,
            Some(COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1),
            true,
            Some(&root_branch),
            &mut evaluate,
        )? {
            CoveredSafeguardedStepV1::Admitted {
                evaluation,
                trust_radius: next_trust_radius,
            } => {
                current = evaluation;
                trust_radius = next_trust_radius;
            }
            CoveredSafeguardedStepV1::Stagnation => {
                return Ok(covered_coupled_polished_root_v1(
                    current,
                    CoveredCoupledPolishStopV1::SubToleranceStagnation,
                    starting_used,
                    budget,
                ));
            }
            CoveredSafeguardedStepV1::NonDescent => {
                return Ok(covered_coupled_polished_root_v1(
                    current,
                    CoveredCoupledPolishStopV1::SubToleranceNonDescent,
                    starting_used,
                    budget,
                ));
            }
            CoveredSafeguardedStepV1::ReceiptEntryReserve => {
                return Ok(covered_coupled_polished_root_v1(
                    current,
                    CoveredCoupledPolishStopV1::ReceiptEntryReserve,
                    starting_used,
                    budget,
                ));
            }
        }
    }
}

#[cfg(test)]
fn phase_consistent_coupled_authentic_final_evaluation_v1<F>(
    root: &CoveredPhaseConsistentResidualEvaluationV1,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    mut evaluate: F,
) -> Result<CoveredPhaseConsistentResidualEvaluationV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        CoveredPhaseConsistentResidualEvaluationV1,
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    let fresh = evaluate(&root.coordinates, budget)?;
    if fresh.scaled_merit > 1.0 || !fresh.algebraic_side_constraints_satisfied {
        return Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch);
    }
    Ok(fresh)
}

#[cfg(test)]
fn phase_consistent_coupled_authentic_final_replay_reseal_v1<F>(
    fresh: &CoveredPhaseConsistentResidualEvaluationV1,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    mut evaluate: F,
) -> Result<CoveredPhaseConsistentResidualEvaluationV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        &[f64],
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        CoveredPhaseConsistentResidualEvaluationV1,
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    let replay = evaluate(&fresh.coordinates, budget)?;
    if replay != *fresh {
        return Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch);
    }
    Ok(replay)
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredAuthenticReceiptStabilizationProbeV1 {
    input_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    residual: CoveredPhaseConsistentResidualEvaluationV1,
    artifacts: CoveredPhaseConsistentPhysicalArtifactsV1,
    finalization_inputs: CoveredFinalizationEquivalentReplayInputsV1,
    reconstructed_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredAuthenticReceiptStabilizationV1 {
    residual: CoveredPhaseConsistentResidualEvaluationV1,
    artifacts: CoveredPhaseConsistentPhysicalArtifactsV1,
    finalization_inputs: CoveredFinalizationEquivalentReplayInputsV1,
    stabilized_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    stabilization_probe_count: usize,
    independent_replay_count: usize,
    publication_eligible: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredAuthenticReceiptExactCycleV1 {
    members: Vec<CoveredAuthenticReceiptStabilizationProbeV1>,
    discovery_probe_count: usize,
    publication_eligible: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum CoveredAuthenticReceiptStabilizationOutcomeV1 {
    Stabilized(Box<CoveredAuthenticReceiptStabilizationV1>),
    ExactCycle(Box<CoveredAuthenticReceiptExactCycleV1>),
}

fn covered_phase_consistent_residual_exact_v1(
    left: &CoveredPhaseConsistentResidualEvaluationV1,
    right: &CoveredPhaseConsistentResidualEvaluationV1,
) -> bool {
    fn exact_slice(left: &[f64], right: &[f64]) -> bool {
        left.len() == right.len()
            && left
                .iter()
                .zip(right)
                .all(|(left, right)| left.to_bits() == right.to_bits())
    }
    exact_slice(&left.coordinates, &right.coordinates)
        && exact_slice(&left.residuals, &right.residuals)
        && exact_slice(&left.absolute_tolerances, &right.absolute_tolerances)
        && exact_slice(&left.r_w_kg_m2, &right.r_w_kg_m2)
        && exact_slice(&left.r_h_j_m2, &right.r_h_j_m2)
        && exact_slice(&left.r_rho_kg_m3, &right.r_rho_kg_m3)
        && exact_slice(&left.r_q_cn_j_m2, &right.r_q_cn_j_m2)
        && exact_slice(&left.physical_q_cn_j_m2, &right.physical_q_cn_j_m2)
        && left.derived_thickness_closures.len() == right.derived_thickness_closures.len()
        && left
            .derived_thickness_closures
            .iter()
            .zip(&right.derived_thickness_closures)
            .all(|(left, right)| {
                left.proposed_z_m.to_bits() == right.proposed_z_m.to_bits()
                    && left.physical_z_m.to_bits() == right.physical_z_m.to_bits()
                    && left.r_z_m.to_bits() == right.r_z_m.to_bits()
                    && left.scaled_merit.to_bits() == right.scaled_merit.to_bits()
            })
        && exact_slice(&left.r_e_j_m2, &right.r_e_j_m2)
        && exact_slice(&left.r_t_k, &right.r_t_k)
        && left.scaled_merit.to_bits() == right.scaled_merit.to_bits()
        && left.derived_constraints_scaled_merit.to_bits()
            == right.derived_constraints_scaled_merit.to_bits()
        && left.algebraic_side_constraints_satisfied == right.algebraic_side_constraints_satisfied
}

fn covered_phase_consistent_residual_finite_v1(
    residual: &CoveredPhaseConsistentResidualEvaluationV1,
) -> bool {
    residual
        .coordinates
        .iter()
        .chain(&residual.residuals)
        .chain(&residual.absolute_tolerances)
        .chain(&residual.r_w_kg_m2)
        .chain(&residual.r_h_j_m2)
        .chain(&residual.r_rho_kg_m3)
        .chain(&residual.r_q_cn_j_m2)
        .chain(&residual.physical_q_cn_j_m2)
        .chain(&residual.r_e_j_m2)
        .chain(&residual.r_t_k)
        .chain(std::iter::once(&residual.scaled_merit))
        .chain(std::iter::once(&residual.derived_constraints_scaled_merit))
        .all(|value| value.is_finite())
        && residual.derived_thickness_closures.iter().all(|closure| {
            closure.proposed_z_m.is_finite()
                && closure.physical_z_m.is_finite()
                && closure.r_z_m.is_finite()
                && closure.scaled_merit.is_finite()
        })
        && residual
            .absolute_tolerances
            .iter()
            .all(|value| *value > 0.0)
}

fn covered_phase_consistent_artifacts_exact_v1(
    left: &CoveredPhaseConsistentPhysicalArtifactsV1,
    right: &CoveredPhaseConsistentPhysicalArtifactsV1,
) -> bool {
    left == right
}

fn covered_authentic_receipt_stabilization_probe_v1<F>(
    input_receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    evaluate: &mut F,
) -> Result<CoveredAuthenticReceiptStabilizationProbeV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        CoveredPhaseConsistentPhysicalEvaluationKindV1,
        &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        (
            CoveredPhaseConsistentResidualEvaluationV1,
            CoveredPhaseConsistentPhysicalArtifactsV1,
            CoveredFinalizationEquivalentReplayInputsV1,
            BTreeMap<u32, SnowSoilHeatReceiptV1>,
        ),
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    if !covered_snow_soil_receipt_sets_exact_v1(input_receipts, input_receipts) {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    let (residual, artifacts, finalization_inputs, reconstructed_receipts) = evaluate(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe,
        input_receipts,
        budget,
    )?;
    if !covered_phase_consistent_residual_finite_v1(&residual) {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    if !residual.algebraic_side_constraints_satisfied
        || !covered_snow_soil_receipt_sets_exact_v1(
            &reconstructed_receipts,
            &reconstructed_receipts,
        )
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    Ok(CoveredAuthenticReceiptStabilizationProbeV1 {
        input_receipts: input_receipts.clone(),
        residual,
        artifacts,
        finalization_inputs,
        reconstructed_receipts,
    })
}

fn covered_authentic_receipt_stabilization_replay_v1<F>(
    stabilized: &CoveredAuthenticReceiptStabilizationProbeV1,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    evaluate: &mut F,
) -> Result<CoveredAuthenticReceiptStabilizationV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        CoveredPhaseConsistentPhysicalEvaluationKindV1,
        &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        (
            CoveredPhaseConsistentResidualEvaluationV1,
            CoveredPhaseConsistentPhysicalArtifactsV1,
            CoveredFinalizationEquivalentReplayInputsV1,
            BTreeMap<u32, SnowSoilHeatReceiptV1>,
        ),
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    if !covered_snow_soil_receipt_sets_exact_v1(
        &stabilized.input_receipts,
        &stabilized.reconstructed_receipts,
    ) {
        return Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch);
    }
    let (residual, artifacts, finalization_inputs, reconstructed_receipts) = evaluate(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
        &stabilized.input_receipts,
        budget,
    )?;
    if !covered_phase_consistent_residual_finite_v1(&residual) {
        return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
    }
    if !residual.algebraic_side_constraints_satisfied
        || !covered_snow_soil_receipt_sets_exact_v1(
            &reconstructed_receipts,
            &reconstructed_receipts,
        )
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
    }
    if !covered_phase_consistent_residual_exact_v1(&stabilized.residual, &residual)
        || !covered_phase_consistent_artifacts_exact_v1(&stabilized.artifacts, &artifacts)
        || stabilized.finalization_inputs != finalization_inputs
        || !covered_snow_soil_receipt_sets_exact_v1(
            &stabilized.reconstructed_receipts,
            &reconstructed_receipts,
        )
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch);
    }
    Ok(CoveredAuthenticReceiptStabilizationV1 {
        residual,
        artifacts,
        finalization_inputs,
        stabilized_receipts: reconstructed_receipts,
        stabilization_probe_count: 0,
        independent_replay_count: 1,
        publication_eligible: false,
    })
}

fn covered_authentic_receipt_stabilize_or_cycle_v1<F>(
    initial_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    mut evaluate: F,
) -> Result<CoveredAuthenticReceiptStabilizationOutcomeV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        CoveredPhaseConsistentPhysicalEvaluationKindV1,
        &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        (
            CoveredPhaseConsistentResidualEvaluationV1,
            CoveredPhaseConsistentPhysicalArtifactsV1,
            CoveredFinalizationEquivalentReplayInputsV1,
            BTreeMap<u32, SnowSoilHeatReceiptV1>,
        ),
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    if initial_receipts.is_empty() {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let mut seen = vec![initial_receipts.clone()];
    let mut produced = Vec::new();
    let mut input = initial_receipts;
    let mut probe_count = 0usize;
    loop {
        covered_physical_evaluation_budget_preserve_v1(budget, COVERED_RECEIPT_REPLAY_RESERVE_V1)?;
        let probe =
            covered_authentic_receipt_stabilization_probe_v1(&input, budget, &mut evaluate)?;
        probe_count = probe_count.saturating_add(1);
        if covered_snow_soil_receipt_sets_exact_v1(
            &probe.input_receipts,
            &probe.reconstructed_receipts,
        ) {
            covered_physical_evaluation_budget_preserve_v1(budget, 0)?;
            let mut stabilized =
                covered_authentic_receipt_stabilization_replay_v1(&probe, budget, &mut evaluate)?;
            stabilized.stabilization_probe_count = probe_count;
            return Ok(CoveredAuthenticReceiptStabilizationOutcomeV1::Stabilized(
                Box::new(stabilized),
            ));
        }
        let next = probe.reconstructed_receipts.clone();
        produced.push(probe);
        if let Some(cycle_start) = seen
            .iter()
            .position(|prior| covered_snow_soil_receipt_sets_exact_v1(prior, &next))
        {
            let cycle_receipts = &seen[cycle_start..];
            if cycle_receipts.is_empty()
                || cycle_receipts.len() > COVERED_AUTHENTIC_RECEIPT_EXACT_CYCLE_MAXIMUM_V1
            {
                return Err(PhaseConsistentCoupledSolveErrorV1::ReceiptOscillation);
            }
            let members = cycle_receipts
                .iter()
                .map(|receipt| {
                    produced
                        .iter()
                        .find(|probe| {
                            covered_snow_soil_receipt_sets_exact_v1(
                                &probe.reconstructed_receipts,
                                receipt,
                            )
                        })
                        .cloned()
                        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)
                })
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(CoveredAuthenticReceiptStabilizationOutcomeV1::ExactCycle(
                Box::new(CoveredAuthenticReceiptExactCycleV1 {
                    members,
                    discovery_probe_count: probe_count,
                    publication_eligible: false,
                }),
            ));
        }
        seen.push(next.clone());
        input = next;
    }
}

#[cfg(test)]
fn covered_authentic_receipt_stabilize_v1<F>(
    initial_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    evaluate: F,
) -> Result<CoveredAuthenticReceiptStabilizationV1, PhaseConsistentCoupledSolveErrorV1>
where
    F: FnMut(
        CoveredPhaseConsistentPhysicalEvaluationKindV1,
        &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        (
            CoveredPhaseConsistentResidualEvaluationV1,
            CoveredPhaseConsistentPhysicalArtifactsV1,
            CoveredFinalizationEquivalentReplayInputsV1,
            BTreeMap<u32, SnowSoilHeatReceiptV1>,
        ),
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    match covered_authentic_receipt_stabilize_or_cycle_v1(initial_receipts, budget, evaluate)? {
        CoveredAuthenticReceiptStabilizationOutcomeV1::Stabilized(stabilized) => Ok(*stabilized),
        CoveredAuthenticReceiptStabilizationOutcomeV1::ExactCycle(_) => {
            Err(PhaseConsistentCoupledSolveErrorV1::ReceiptOscillation)
        }
    }
}

fn covered_receipt_cycle_endpoint_coordinates_assemble_v1(
    lane_ids: &[u32],
    stage_coordinates: &BTreeMap<u32, (f64, f64, f64)>,
    receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
    soil_coordinates: &[(f64, f64)],
) -> Result<Vec<f64>, PhaseConsistentCoupledSolveErrorV1> {
    if lane_ids.is_empty()
        || stage_coordinates.keys().copied().ne(lane_ids.iter().copied())
        || receipts.keys().copied().ne(lane_ids.iter().copied())
        || soil_coordinates.is_empty()
        || !covered_snow_soil_receipt_sets_exact_v1(receipts, receipts)
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let expected = lane_ids
        .len()
        .checked_mul(4)
        .and_then(|lane_coordinates| {
            soil_coordinates
                .len()
                .checked_mul(2)
                .and_then(|soil_coordinates| lane_coordinates.checked_add(soil_coordinates))
        })
        .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
    let mut coordinates = Vec::with_capacity(expected);
    for lane_id in lane_ids {
        let (water, enthalpy, density) = stage_coordinates
            .get(lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        let receipt = receipts
            .get(lane_id)
            .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
        coordinates.extend([
            *water,
            *enthalpy,
            *density,
            receipt.snow_candidate_heat_j_m2_ofe_ground,
        ]);
    }
    for (enthalpy, temperature) in soil_coordinates {
        if !enthalpy.is_finite() || !temperature.is_finite() {
            return Err(PhaseConsistentCoupledSolveErrorV1::NonFinite);
        }
        coordinates.extend([*enthalpy, *temperature]);
    }
    if coordinates.len() != expected || coordinates.iter().any(|value| !value.is_finite()) {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    Ok(coordinates)
}

fn covered_receipt_cycle_endpoint_coordinates_v1(
    member: &CoveredAuthenticReceiptStabilizationProbeV1,
    lane_ids: &[u32],
) -> Result<Vec<f64>, PhaseConsistentCoupledSolveErrorV1> {
    if member
        .artifacts
        .stage3_candidate
        .keys()
        .copied()
        .ne(lane_ids.iter().copied())
    {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let stage_coordinates = member
        .artifacts
        .stage3_candidate
        .iter()
        .map(|(lane_id, state)| {
            stable_monotone_stage_coordinates_v1(state)
                .map(|coordinates| (*lane_id, coordinates))
                .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let (_, _, soil_enthalpy) =
        stable_monotone_v2_carry_coordinates_v1(&member.artifacts.soil_candidate)
            .map_err(|_| PhaseConsistentCoupledSolveErrorV1::SideConstraint)?;
    let soil_ofes = member.artifacts.soil_candidate.read_view().ordered_ofes();
    if soil_enthalpy.is_empty() || soil_enthalpy.len() != soil_ofes.len() {
        return Err(PhaseConsistentCoupledSolveErrorV1::Structure);
    }
    let soil_coordinates = soil_ofes
        .iter()
        .enumerate()
        .map(|(index, ofe)| {
            let top = ofe
                .ordered_layers()
                .into_iter()
                .next()
                .ok_or(PhaseConsistentCoupledSolveErrorV1::Structure)?;
            Ok((soil_enthalpy[index], top.temperature_k()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    covered_receipt_cycle_endpoint_coordinates_assemble_v1(
        lane_ids,
        &stage_coordinates,
        &member.reconstructed_receipts,
        &soil_coordinates,
    )
}

fn covered_authentic_receipt_cycle_endpoint_witness_v1<P, F>(
    cycle: &CoveredAuthenticReceiptExactCycleV1,
    budget: &mut CoveredPhysicalEvaluationBudgetV1,
    expected_branch: &CoveredPhaseConsistentPhysicalBranchIdentityV1,
    mut project: P,
    mut evaluate: F,
) -> Result<CoveredAuthenticReceiptStabilizationV1, PhaseConsistentCoupledSolveErrorV1>
where
    P: FnMut(
        &CoveredAuthenticReceiptStabilizationProbeV1,
    ) -> Result<Vec<f64>, PhaseConsistentCoupledSolveErrorV1>,
    F: FnMut(
        CoveredPhaseConsistentPhysicalEvaluationKindV1,
        &[f64],
        &BTreeMap<u32, SnowSoilHeatReceiptV1>,
        &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<
        (
            CoveredPhaseConsistentPhysicalEvaluationV1,
            BTreeMap<u32, SnowSoilHeatReceiptV1>,
        ),
        PhaseConsistentCoupledSolveErrorV1,
    >,
{
    let members = cycle.members.len();
    let required = members
        .checked_add(COVERED_RECEIPT_REPLAY_RESERVE_V1)
        .ok_or(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)?;
    if cycle.publication_eligible
        || members == 0
        || members > COVERED_AUTHENTIC_RECEIPT_EXACT_CYCLE_MAXIMUM_V1
        || budget.maximum != COVERED_PHYSICAL_EVALUATION_LIMIT_V1
        || budget.maximum.saturating_sub(budget.used) < required
    {
        return Err(if budget.maximum.saturating_sub(budget.used) < required {
            PhaseConsistentCoupledSolveErrorV1::EvaluationBudget
        } else {
            PhaseConsistentCoupledSolveErrorV1::SideConstraint
        });
    }

    let mut attempted = 0usize;
    for member in &cycle.members {
        let coordinates = project(member)?;
        let receipts = &member.reconstructed_receipts;
        if !covered_snow_soil_receipt_sets_exact_v1(receipts, receipts) {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        let used_before = budget.used;
        let (probe, reconstructed) = evaluate(
            CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe,
            &coordinates,
            receipts,
            budget,
        )?;
        attempted = attempted.saturating_add(1);
        if budget.used != used_before.saturating_add(1) {
            return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
        }
        covered_phase_consistent_physical_evaluation_validate_v1(
            &coordinates,
            &probe,
            Some(budget.used),
            Some(expected_branch),
        )?;
        if probe.residual.scaled_merit > 1.0
            || !probe.residual.algebraic_side_constraints_satisfied
            || !covered_snow_soil_receipt_sets_exact_v1(&reconstructed, &reconstructed)
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
        }
        if !covered_snow_soil_receipt_sets_exact_v1(receipts, &reconstructed) {
            continue;
        }

        let replay_used_before = budget.used;
        let (replay, replay_receipts) = evaluate(
            CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
            &coordinates,
            receipts,
            budget,
        )?;
        if budget.used != replay_used_before.saturating_add(1) {
            return Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget);
        }
        covered_phase_consistent_physical_evaluation_validate_v1(
            &coordinates,
            &replay,
            Some(budget.used),
            Some(expected_branch),
        )?;
        if !covered_phase_consistent_residual_exact_v1(&probe.residual, &replay.residual)
            || !covered_phase_consistent_artifacts_exact_v1(&probe.artifacts, &replay.artifacts)
            || probe.finalization_inputs != replay.finalization_inputs
            || probe.branch_identity != replay.branch_identity
            || !covered_snow_soil_receipt_sets_exact_v1(&reconstructed, &replay_receipts)
        {
            return Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch);
        }
        return Ok(CoveredAuthenticReceiptStabilizationV1 {
            residual: replay.residual,
            artifacts: replay.artifacts,
            finalization_inputs: replay.finalization_inputs,
            stabilized_receipts: replay_receipts,
            stabilization_probe_count: cycle
                .discovery_probe_count
                .saturating_add(attempted),
            independent_replay_count: 1,
            publication_eligible: false,
        });
    }
    Err(PhaseConsistentCoupledSolveErrorV1::ReceiptOscillation)
}
