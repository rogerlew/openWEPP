//! M1's bounded, diagnostic 21-coordinate covered-column evaluator.

use super::*;
use openwepp_vegetation::cold_canopy_m1::{
    M1CapacityTrial, M1DrySurface, M1EmptySurfaceInput, M1Error, M1ExternalPhase, M1PhaseReservoir,
    evaluate_m1_capacity_residual, m1_qsat_external, validate_m1_empty_surface,
};
#[cfg(any(test, feature = "test-support"))]
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::BTreeSet;

thread_local! {
    static M1_RUNTIME_ACTIVITY: RefCell<M1LoopActivityObservation> = const { RefCell::new(M1LoopActivityObservation::new()) };
}

#[cfg(test)]
thread_local! {
    static M1_REFINEMENT_PHASE_JOIN_SELECTIONS: Cell<u32> = const { Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn m1_refinement_phase_join_selections_for_test() -> u32 {
    M1_REFINEMENT_PHASE_JOIN_SELECTIONS.with(Cell::get)
}

// Private diagnostic opt-in.  The observer is off by default, including for
// test-support dependents, and consumes exactly one subsequent solve.
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static M1_DIAGNOSTIC_GENERATION: Cell<u64> = const { Cell::new(0) };
    static M1_DIAGNOSTIC_PENDING_INPUT: RefCell<Option<M1DiagnosticPending>> = const { RefCell::new(None) };
    static M1_DIAGNOSTIC_OBSERVATION: RefCell<Option<M1DiagnosticObservation>> = const { RefCell::new(None) };
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone)]
struct M1DiagnosticPending {
    input: M1CoupledColumnInput,
    generation: u64,
}

/// Enables a bounded, private observation of this input's next M1 solve.
///
/// This is available only to the `test-support` seam.  It neither changes the
/// supplied input nor changes the numerical path; the completed record is
/// retrieved with [`Self::take_m1_diagnostic_observation`].
#[cfg(any(test, feature = "test-support"))]
impl M1CoupledColumnInput {
    #[must_use]
    pub fn enable_one_m1_diagnostic_observation(&self) -> M1DiagnosticObservationGuard {
        let generation = M1_DIAGNOSTIC_GENERATION.with(|current| {
            let next = current.get().wrapping_add(1);
            current.set(next);
            next
        });
        M1_DIAGNOSTIC_PENDING_INPUT.with(|pending| {
            if let Ok(mut pending) = pending.try_borrow_mut() {
                *pending = Some(M1DiagnosticPending {
                    input: self.clone(),
                    generation,
                });
            }
        });
        M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
            if let Ok(mut observation) = observation.try_borrow_mut() {
                *observation = None;
            }
        });
        M1DiagnosticObservationGuard { generation }
    }

    #[must_use]
    pub fn take_m1_diagnostic_observation(&self) -> Option<M1DiagnosticObservation> {
        M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
            let matches_input = match observation.try_borrow() {
                Ok(record) => record.as_ref().is_some_and(|record| record.input == *self),
                Err(_) => false,
            };
            if matches_input {
                observation.try_borrow_mut().ok()?.take()
            } else {
                None
            }
        })
    }
}

/// Bounded record of the real Newton path for one opt-in solve.
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Debug, PartialEq)]
pub struct M1DiagnosticObservation {
    generation: u64,
    pub input: M1CoupledColumnInput,
    pub seed_coordinates: Option<[f64; 21]>,
    pub bases: Vec<M1DiagnosticNewtonBase>,
    pub linear_solves: Vec<M1DiagnosticLinearSolve>,
    pub trials: Vec<M1DiagnosticTrial>,
    pub terminal_error_code: Option<String>,
    pub status: M1DiagnosticCaptureStatus,
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum M1DiagnosticCaptureStatus {
    Active,
    Complete,
    Incomplete(M1DiagnosticCaptureReason),
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum M1DiagnosticCaptureReason {
    BorrowConflict,
    Capacity,
    InjectedFault,
}

#[cfg(any(test, feature = "test-support"))]
const M1_DIAGNOSTIC_BASE_CAP: usize = crate::numerics::MAX_NEWTON_ITERATIONS as usize + 1;
#[cfg(any(test, feature = "test-support"))]
const M1_DIAGNOSTIC_TRIAL_CAP: usize =
    M1_DIAGNOSTIC_BASE_CAP * (crate::numerics::MAX_BACKTRACKING_HALVINGS as usize + 1);
#[cfg(any(test, feature = "test-support"))]
const M1_DIAGNOSTIC_LINEAR_CAP: usize = M1_DIAGNOSTIC_BASE_CAP * 2;

#[cfg(test)]
fn m1_diagnostic_forced_fault() -> bool {
    M1_DIAGNOSTIC_FORCE_FAULT.with(|value| value.get())
}
#[cfg(all(not(test), feature = "test-support"))]
const fn m1_diagnostic_forced_fault() -> bool {
    false
}
#[cfg(test)]
fn m1_diagnostic_forced_overflow() -> bool {
    M1_DIAGNOSTIC_FORCE_OVERFLOW.with(|value| value.get())
}
#[cfg(all(not(test), feature = "test-support"))]
const fn m1_diagnostic_forced_overflow() -> bool {
    false
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Debug, PartialEq)]
pub struct M1DiagnosticNewtonBase {
    pub iteration: u32,
    pub coordinates: [f64; 21],
    pub raw_residuals: Vec<f64>,
    pub normalized_residuals: Vec<f64>,
    pub normalizers: Vec<f64>,
    pub row_identities: Vec<String>,
    pub row_units: Vec<ResidualUnit>,
    pub normalized_jacobian: Vec<Vec<f64>>,
    pub rhs: Vec<f64>,
    pub direction: [f64; 21],
    pub natural_selections: [M1PhaseJoinSelection; 2],
    pub predictor_selections: [M1PhaseJoinSelection; 2],
    pub final_selections: [M1PhaseJoinSelection; 2],
    pub corrected_assembly: bool,
    pub full_linear_dimension: usize,
    pub reduced_linear_dimension: usize,
}

/// Exact operands and result of one controller LU invocation.
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Debug, PartialEq)]
pub struct M1DiagnosticLinearSolve {
    pub iteration: u32,
    pub corrected_assembly: bool,
    pub full_matrix: Vec<Vec<f64>>,
    pub full_rhs: Vec<f64>,
    pub reduced_matrix: Vec<Vec<f64>>,
    pub reduced_rhs: Vec<f64>,
    pub retained_indices: Vec<usize>,
    pub eliminated_indices: Vec<usize>,
    pub reduced_direction: Vec<f64>,
    pub full_direction: Vec<f64>,
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Debug, PartialEq)]
pub struct M1DiagnosticTrial {
    pub iteration: u32,
    pub exponent: u32,
    pub factor: f64,
    pub coordinates: [f64; 21],
    pub raw_residuals: Option<Vec<f64>>,
    pub normalized_residuals: Option<Vec<f64>>,
    pub reason: String,
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_begin(input: &M1CoupledColumnInput, generation: u64, seed_coordinates: [f64; 21]) {
    m1_diagnostic_begin_record(input, generation, Some(seed_coordinates));
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_begin_empty(input: &M1CoupledColumnInput, generation: u64) {
    m1_diagnostic_begin_record(input, generation, None);
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_begin_record(
    input: &M1CoupledColumnInput,
    generation: u64,
    seed_coordinates: Option<[f64; 21]>,
) {
    let mut bases = Vec::new();
    let mut linear_solves = Vec::new();
    let mut trials = Vec::new();
    let capacity_ok = bases.try_reserve_exact(M1_DIAGNOSTIC_BASE_CAP).is_ok()
        && linear_solves
            .try_reserve_exact(M1_DIAGNOSTIC_LINEAR_CAP)
            .is_ok()
        && trials.try_reserve_exact(M1_DIAGNOSTIC_TRIAL_CAP).is_ok();
    let forced_fault = m1_diagnostic_forced_fault();
    let forced_overflow = m1_diagnostic_forced_overflow();
    M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
        if let Ok(mut observation) = observation.try_borrow_mut() {
            *observation = Some(M1DiagnosticObservation {
                generation,
                input: input.clone(),
                seed_coordinates,
                bases,
                linear_solves,
                trials,
                terminal_error_code: None,
                status: if forced_fault {
                    M1DiagnosticCaptureStatus::Incomplete(M1DiagnosticCaptureReason::InjectedFault)
                } else if capacity_ok && !forced_overflow {
                    M1DiagnosticCaptureStatus::Active
                } else {
                    M1DiagnosticCaptureStatus::Incomplete(M1DiagnosticCaptureReason::Capacity)
                },
            });
        }
    });
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_terminal(result: &Result<M1CoupledEvaluation, M1CoupledError>) {
    M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
        if let Ok(mut observation) = observation.try_borrow_mut()
            && let Some(observation) = observation.as_mut()
        {
            if observation.status == M1DiagnosticCaptureStatus::Active {
                observation.status = M1DiagnosticCaptureStatus::Complete;
            }
            observation.terminal_error_code =
                result.as_ref().err().map(|error| error.code().to_owned());
        }
    });
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_mutate(mutator: impl FnOnce(&mut M1DiagnosticObservation)) {
    M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
        let Ok(mut observation) = observation.try_borrow_mut() else {
            return;
        };
        let Some(observation) = observation.as_mut() else {
            return;
        };
        if m1_diagnostic_forced_fault() {
            m1_diagnostic_incomplete(observation, M1DiagnosticCaptureReason::InjectedFault);
            return;
        }
        if observation.status == M1DiagnosticCaptureStatus::Active {
            mutator(observation);
        }
    });
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_capture_active() -> bool {
    M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
        observation.try_borrow().is_ok_and(|observation| {
            observation
                .as_ref()
                .is_some_and(|record| record.status == M1DiagnosticCaptureStatus::Active)
        })
    })
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_incomplete(
    observation: &mut M1DiagnosticObservation,
    reason: M1DiagnosticCaptureReason,
) {
    observation.status = M1DiagnosticCaptureStatus::Incomplete(reason);
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_base(
    iteration: u32,
    trial: &M1CoupledColumnTrial,
    base: &Core,
    report: &M1PhaseJoinControllerReport,
    residuals: &[f64],
    input: &M1CoupledColumnInput,
) {
    m1_diagnostic_mutate(|observation| {
        if observation.bases.len() == M1_DIAGNOSTIC_BASE_CAP {
            m1_diagnostic_incomplete(observation, M1DiagnosticCaptureReason::Capacity);
        } else {
            observation.bases.push(M1DiagnosticNewtonBase {
                iteration,
                coordinates: trial.coordinates,
                raw_residuals: base.residuals.clone(),
                normalized_residuals: residuals.to_vec(),
                normalizers: m1_tolerances(input, base).to_vec(),
                row_identities: (0..base.residuals.len())
                    .map(|index| format!("m1_row_{index}"))
                    .collect(),
                row_units: (0..base.residuals.len())
                    .map(m1_diagnostic_row_unit)
                    .collect(),
                normalized_jacobian: report.normalized_jacobian.clone(),
                rhs: m1_normalized_linear_rhs(base, input),
                direction: report.direction,
                natural_selections: report.natural_selections,
                predictor_selections: report.predictor_selections,
                final_selections: report.final_selections,
                corrected_assembly: report.corrected_assembly,
                full_linear_dimension: 21,
                reduced_linear_dimension: report.linear_dimension,
            });
        }
    });
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_linear_solve(
    iteration: u32,
    corrected_assembly: bool,
    full_matrix: &[Vec<f64>],
    full_rhs: &[f64],
    reduced_matrix: &[Vec<f64>],
    reduced_rhs: &[f64],
    retained_indices: &[usize],
    eliminated_indices: &[usize],
    reduced_direction: &[f64],
    full_direction: &[f64],
) {
    m1_diagnostic_mutate(|observation| {
        if observation.linear_solves.len() == M1_DIAGNOSTIC_LINEAR_CAP {
            m1_diagnostic_incomplete(observation, M1DiagnosticCaptureReason::Capacity);
        } else {
            observation.linear_solves.push(M1DiagnosticLinearSolve {
                iteration,
                corrected_assembly,
                full_matrix: full_matrix.to_vec(),
                full_rhs: full_rhs.to_vec(),
                reduced_matrix: reduced_matrix.to_vec(),
                reduced_rhs: reduced_rhs.to_vec(),
                retained_indices: retained_indices.to_vec(),
                eliminated_indices: eliminated_indices.to_vec(),
                reduced_direction: reduced_direction.to_vec(),
                full_direction: full_direction.to_vec(),
            });
        }
    });
}

#[cfg(any(test, feature = "test-support"))]
const fn m1_diagnostic_row_unit(index: usize) -> ResidualUnit {
    match index {
        3 | 5 | 9 | 11 => ResidualUnit::KilogramsPerSquareMeter,
        4 | 10 => ResidualUnit::JoulesPerSquareMeter,
        13 => ResidualUnit::KilogramsPerSquareMeterSecond,
        14..=20 => ResidualUnit::Kelvin,
        _ => ResidualUnit::WattsPerSquareMeter,
    }
}

#[cfg(any(test, feature = "test-support"))]
fn m1_diagnostic_trial(
    iteration: u32,
    exponent: u32,
    factor: f64,
    candidate: &M1CoupledColumnTrial,
    core: Option<&Core>,
    input: &M1CoupledColumnInput,
    reason: impl Into<String>,
) {
    m1_diagnostic_mutate(|observation| {
        if observation.trials.len() == M1_DIAGNOSTIC_TRIAL_CAP {
            m1_diagnostic_incomplete(observation, M1DiagnosticCaptureReason::Capacity);
        } else {
            observation.trials.push(M1DiagnosticTrial {
                iteration,
                exponent,
                factor,
                coordinates: candidate.coordinates,
                raw_residuals: core.map(|value| value.residuals.clone()),
                normalized_residuals: core.map(|value| normalized(value, input)),
                reason: reason.into(),
            });
        }
    });
}

#[cfg(any(test, feature = "test-support"))]
pub struct M1DiagnosticObservationGuard {
    generation: u64,
}
#[cfg(any(test, feature = "test-support"))]
impl M1DiagnosticObservationGuard {
    pub fn cancel(self) {}
}
#[cfg(any(test, feature = "test-support"))]
impl Drop for M1DiagnosticObservationGuard {
    fn drop(&mut self) {
        M1_DIAGNOSTIC_PENDING_INPUT.with(|pending| {
            if let Ok(mut pending) = pending.try_borrow_mut()
                && pending
                    .as_ref()
                    .is_some_and(|pending| pending.generation == self.generation)
            {
                *pending = None;
            }
        });
        M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
            if let Ok(mut observation) = observation.try_borrow_mut()
                && observation
                    .as_ref()
                    .is_some_and(|observation| observation.generation == self.generation)
            {
                *observation = None;
            }
        });
    }
}

#[cfg(test)]
thread_local! {
    static M1_JACOBIAN_INVOCATIONS: Cell<u32> = const { Cell::new(0) };
    static M1_CONTROLLER_ASSEMBLIES: Cell<u32> = const { Cell::new(0) };
    static M1_ACCEPTED_CAPTURE: RefCell<Option<(M1CoupledColumnInput, M1CoupledColumnTrial, Core)>> = const { RefCell::new(None) };
    static M1_MATERIALIZATION_OUTPUT_POISON: Cell<u8> = const { Cell::new(0) };
    static M1_HALVED_TRIAL_CORE_INCOMPLETE_AT: Cell<Option<u32>> = const { Cell::new(None) };
    static M1_HALVED_TRIAL_HYDRAULIC_INCOMPLETE_AT: Cell<Option<u32>> = const { Cell::new(None) };
    static M1_HALVED_TRIAL_TARGET_ITERATION: Cell<Option<u32>> = const { Cell::new(None) };
    static M1_HALVED_TRIAL_DOMAIN_INVALID_MASK: Cell<u32> = const { Cell::new(0) };
    static M1_HALVED_TRIAL_FULL_DOMAIN_INVALID: Cell<bool> = const { Cell::new(false) };
    static M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS: Cell<bool> = const { Cell::new(false) };
    static M1_HALVED_TRIAL_STEP_POISON: Cell<Option<(u32, u8)>> = const { Cell::new(None) };
    static M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE: Cell<bool> = const { Cell::new(false) };
    static M1_INSTALLED_STEP_POISON: Cell<Option<u8>> = const { Cell::new(None) };
    static M1_DISABLE_NO_UPDATE: Cell<bool> = const { Cell::new(false) };
    static M1_HALVED_TRIAL_WITNESS_TRACE: RefCell<Vec<(u32, bool)>> = const { RefCell::new(Vec::new()) };
    static M1_HALVED_TRIAL_OUTCOME: RefCell<Vec<(u32, bool)>> = const { RefCell::new(Vec::new()) };
    static M1_LAST_CONTROLLER_FAILURE_ACTIVITY: RefCell<Option<M1PhaseJoinActivity>> = const { RefCell::new(None) };
    static M1_DIAGNOSTIC_FORCE_FAULT: Cell<bool> = const { Cell::new(false) };
    static M1_DIAGNOSTIC_FORCE_OVERFLOW: Cell<bool> = const { Cell::new(false) };
}

#[cfg(test)]
pub(crate) fn m1_set_diagnostic_capture_fault_for_test(fault: bool, overflow: bool) {
    M1_DIAGNOSTIC_FORCE_FAULT.with(|value| value.set(fault));
    M1_DIAGNOSTIC_FORCE_OVERFLOW.with(|value| value.set(overflow));
}

const TF: f64 = 273.15;
const CW: f64 = 4218.0;
const LF: f64 = 333_700.0;
const LV: f64 = 2_501_000.0;
const CPV: f64 = 1849.0;
const M1_STEP_TEMPERATURE_MAX_K: f64 = 1.0e-8;
const M1_STEP_HUMIDITY_MAX_KG_KG: f64 = 1.0e-12;
const M1_STEP_HYDRAULIC_MAX_MM: f64 = 1.0e-7;
const M1_STEP_BETA_MAX: f64 = 1.0e-10;

fn m1_governed_step_components_pass(
    temperature_k: f64,
    humidity_kg_kg: f64,
    hydraulic_mm: f64,
    beta: f64,
) -> bool {
    [temperature_k, humidity_kg_kg, hydraulic_mm, beta]
        .iter()
        .all(|value| value.is_finite())
        && temperature_k <= M1_STEP_TEMPERATURE_MAX_K
        && humidity_kg_kg <= M1_STEP_HUMIDITY_MAX_KG_KG
        && hydraulic_mm <= M1_STEP_HYDRAULIC_MAX_MM
        && beta <= M1_STEP_BETA_MAX
}

#[cfg(all(
    test,
    any(
        feature = "m1-trust-region-controller-stage",
        feature = "m1-trust-region-analytic-stage",
        feature = "m1-trust-region-physical-stage"
    )
))]
pub(crate) fn m1_governed_step_components_pass_for_trust_region(
    temperature_k: f64,
    humidity_kg_kg: f64,
    hydraulic_mm: f64,
    beta: f64,
    ci_pa: &[f64],
) -> bool {
    ci_pa.iter().all(|value| value.is_finite())
        && m1_governed_step_components_pass(temperature_k, humidity_kg_kg, hydraulic_mm, beta)
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_governed_step_thresholds_for_trust_region() -> [f64; 4] {
    [
        M1_STEP_TEMPERATURE_MAX_K,
        M1_STEP_HUMIDITY_MAX_KG_KG,
        M1_STEP_HYDRAULIC_MAX_MM,
        M1_STEP_BETA_MAX,
    ]
}

#[derive(Clone, Debug, PartialEq)]
pub struct M1ColumnTopologyBinding {
    pub occupancy_ids: [String; 2],
    pub soil_layer_ids: [String; 6],
}

#[derive(Clone, Debug, PartialEq)]
pub struct M1OccupancySupportCondition {
    pub occupancy_id: String,
    pub liquid_conducting: bool,
    pub full_supply: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct M1SupportConditions {
    pub model_definition_id: String,
    pub occupancy_conditions: Vec<M1OccupancySupportCondition>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct M1PhaseReservoirInput {
    pub occupancy_id: String,
    pub mass_kg_m2: f64,
    pub enthalpy_j_m2: f64,
}
impl M1PhaseReservoirInput {
    #[must_use]
    pub fn new(occupancy_id: impl Into<String>, mass_kg_m2: f64, enthalpy_j_m2: f64) -> Self {
        Self {
            occupancy_id: occupancy_id.into(),
            mass_kg_m2,
            enthalpy_j_m2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct M1CoupledColumnInput {
    pub column: CoveredColumnInputs,
    pub reservoirs: [M1PhaseReservoirInput; 2],
    pub support: M1SupportConditions,
    pub topology: M1ColumnTopologyBinding,
    pub top_liquid_specific_enthalpy_j_kg: Option<f64>,
}
impl M1CoupledColumnInput {
    #[must_use]
    pub fn from_covered_column(
        column: CoveredColumnInputs,
        reservoirs: [M1PhaseReservoirInput; 2],
        support: M1SupportConditions,
        topology: M1ColumnTopologyBinding,
    ) -> Self {
        Self {
            column,
            reservoirs,
            support,
            topology,
            top_liquid_specific_enthalpy_j_kg: None,
        }
    }
    #[must_use]
    pub fn with_top_liquid_specific_enthalpy(mut self, value: f64) -> Self {
        self.top_liquid_specific_enthalpy_j_kg = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct M1CoupledColumnTrial {
    coordinates: [f64; 21],
}
impl M1CoupledColumnTrial {
    #[must_use]
    pub const fn from_coordinates(coordinates: [f64; 21]) -> Self {
        Self { coordinates }
    }
    pub fn with_coordinate_offset(
        &self,
        index: usize,
        offset: f64,
    ) -> Result<Self, M1CoupledError> {
        if index >= 21 || !offset.is_finite() {
            return Err(saturation_error());
        }
        let mut trial = self.clone();
        trial.coordinates[index] += offset;
        Ok(trial)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct M1LeafEvaluation {
    pub gas_branch: String,
    pub beta: f64,
    pub internal_specific_humidity_kg_kg: f64,
    pub dark_respiration: f64,
    pub net_assimilation: f64,
    pub gross_assimilation: f64,
    pub conductance_m_s: f64,
    pub ci_pa: f64,
    pub vapor_kg_m2_tile_s: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct M1HydraulicEvaluation {
    pub potentials_mm: Vec<f64>,
    pub root_layer_ids: Vec<String>,
    pub root_fluxes_kg_m2_tile_s: Vec<f64>,
    pub stem_flux_kg_m2_tile_s: f64,
    pub total_leaf_demand_kg_m2_tile_s: f64,
    pub continuity_residuals_kg_m2_tile_s: Vec<f64>,
    pub all_root_layer_ids: Vec<String>,
    pub all_root_fluxes_kg_m2_tile_s: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct M1OccupancyEvaluation {
    pub occupancy_id: String,
    pub sun: M1LeafEvaluation,
    pub shade: M1LeafEvaluation,
    pub hydraulics: M1HydraulicEvaluation,
}
#[derive(Clone, Debug, PartialEq)]
pub struct M1ReservoirOperands {
    pub beginning_mass_kg_m2: f64,
    pub beginning_enthalpy_j_m2: f64,
    pub ending_mass_kg_m2: f64,
    pub ending_enthalpy_j_m2: f64,
    pub dt_s: f64,
    pub incident_liquid_kg_m2_s: f64,
    pub incident_enthalpy_j_kg: f64,
    pub vapor_kg_m2_s: f64,
    pub drainage_kg_m2_s: f64,
    pub non_vapor_heat_w_m2: f64,
    pub wet_temperature_k: Option<f64>,
    pub diagnosed_liquid_mass_kg_m2: f64,
    pub liquid_capacity_kg_m2: f64,
    pub wet_fraction: f64,
    pub emission_w_m2: f64,
    pub wet_longwave_w_m2: f64,
    pub wet_sensible_w_m2: f64,
    pub mass_residual_kg_m2: f64,
    pub enthalpy_residual_j_m2: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct M1CoupledEvaluation {
    // Minted only by accepted materialization.  This keeps the complete
    // input used by the solver with its result, so a caller can authenticate
    // model/configuration, topology, M/H and physical operands without
    // replaying the accepted solve.
    accepted_input: M1CoupledColumnInput,
    residuals: Vec<f64>,
    jacobian: Option<Vec<Vec<f64>>>,
    occupancies: Vec<M1OccupancyEvaluation>,
    reservoirs: Vec<M1ReservoirOperands>,
    duration_s: f64,
    // Exact accepted nonlinear trial retained for diagnostic continuation; this
    // is the materialized accepted solve, never an evaluation retry.
    accepted_coordinates: [f64; 21],
}
impl M1CoupledEvaluation {
    #[must_use]
    pub fn is_bound_to(&self, input: &M1CoupledColumnInput) -> bool {
        &self.accepted_input == input
    }

    #[must_use]
    pub fn residuals(&self) -> &[f64] {
        &self.residuals
    }
    #[must_use]
    pub fn jacobian(&self) -> Result<&[Vec<f64>], M1CoupledError> {
        self.jacobian.as_deref().ok_or_else(|| {
            LandSurfaceEnergyError::UnsupportedDomain("m1_diagnostic_jacobian").into()
        })
    }
    #[must_use]
    pub fn jacobian_column(&self, index: usize) -> Result<Vec<f64>, M1CoupledError> {
        let jacobian = self.jacobian()?;
        if index >= jacobian.len() {
            return Err(domain_error());
        }
        Ok(jacobian.iter().map(|row| row[index]).collect())
    }
    #[must_use]
    pub fn reservoir_operands(&self) -> &[M1ReservoirOperands] {
        &self.reservoirs
    }
    #[must_use]
    pub fn occupancies(&self) -> &[M1OccupancyEvaluation] {
        &self.occupancies
    }
    #[must_use]
    pub fn support_duration_s(&self) -> f64 {
        self.duration_s
    }
    /// Exact coordinate vector of the accepted materialized M1 solve.
    #[must_use]
    pub const fn accepted_coordinates(&self) -> &[f64; 21] {
        &self.accepted_coordinates
    }
    pub fn occupancy(&self, id: &str) -> Result<&M1OccupancyEvaluation, M1CoupledError> {
        self.occupancies
            .iter()
            .find(|value| value.occupancy_id == id)
            .ok_or_else(condition_error)
    }
}

fn condition_error() -> M1CoupledError {
    M1Error::condition_error().into()
}
fn saturation_error() -> M1CoupledError {
    M1Error::saturation_error().into()
}
fn domain_error() -> M1CoupledError {
    M1Error::domain_error().into()
}

fn validate_accepted_materialization(
    input: &M1CoupledColumnInput,
    result: &Core,
    trial: &M1CoupledColumnTrial,
    evaluation: &M1CoupledEvaluation,
) -> Result<(), M1CoupledError> {
    let finite = result
        .residuals
        .iter()
        .chain(&result.scales)
        .all(|x| x.is_finite())
        && evaluation.residuals.iter().all(|x| x.is_finite())
        && evaluation.duration_s.is_finite()
        && evaluation
            .jacobian
            .as_ref()
            .is_none_or(|matrix| matrix.iter().flatten().all(|x| x.is_finite()));
    if !finite {
        return Err(saturation_error());
    }
    for occupancy in &evaluation.occupancies {
        let leaf = [&occupancy.sun, &occupancy.shade];
        for value in leaf.into_iter().flat_map(|leaf| {
            [
                leaf.beta,
                leaf.internal_specific_humidity_kg_kg,
                leaf.dark_respiration,
                leaf.net_assimilation,
                leaf.gross_assimilation,
                leaf.conductance_m_s,
                leaf.ci_pa,
                leaf.vapor_kg_m2_tile_s,
            ]
        }) {
            if !value.is_finite() {
                return Err(saturation_error());
            }
        }
        if !occupancy
            .hydraulics
            .potentials_mm
            .iter()
            .chain(&occupancy.hydraulics.root_fluxes_kg_m2_tile_s)
            .chain(&occupancy.hydraulics.all_root_fluxes_kg_m2_tile_s)
            .chain(&occupancy.hydraulics.continuity_residuals_kg_m2_tile_s)
            .all(|x| x.is_finite())
            || !occupancy.hydraulics.stem_flux_kg_m2_tile_s.is_finite()
            || !occupancy
                .hydraulics
                .total_leaf_demand_kg_m2_tile_s
                .is_finite()
        {
            return Err(saturation_error());
        }
    }
    for reservoir in &evaluation.reservoirs {
        let values = [
            reservoir.beginning_mass_kg_m2,
            reservoir.beginning_enthalpy_j_m2,
            reservoir.ending_mass_kg_m2,
            reservoir.ending_enthalpy_j_m2,
            reservoir.dt_s,
            reservoir.incident_liquid_kg_m2_s,
            reservoir.incident_enthalpy_j_kg,
            reservoir.vapor_kg_m2_s,
            reservoir.drainage_kg_m2_s,
            reservoir.non_vapor_heat_w_m2,
            reservoir.diagnosed_liquid_mass_kg_m2,
            reservoir.liquid_capacity_kg_m2,
            reservoir.wet_fraction,
            reservoir.emission_w_m2,
            reservoir.wet_longwave_w_m2,
            reservoir.wet_sensible_w_m2,
            reservoir.mass_residual_kg_m2,
            reservoir.enthalpy_residual_j_m2,
        ];
        if values.iter().any(|x| !x.is_finite())
            || reservoir.wet_temperature_k.is_some_and(|x| !x.is_finite())
        {
            return Err(saturation_error());
        }
    }
    for (index, reservoir) in evaluation.reservoirs.iter().enumerate() {
        let base = index * 6;
        if !(trial.coordinates[base + 3] > 0.0)
            || trial.coordinates[base + 5] < 0.0
            || !(0.0..=reservoir.liquid_capacity_kg_m2)
                .contains(&reservoir.diagnosed_liquid_mass_kg_m2)
        {
            return Err(domain_error());
        }
        let wet_temperature = reservoir.wet_temperature_k.ok_or_else(domain_error)?;
        let mass_delta = reservoir.dt_s
            * (reservoir.incident_liquid_kg_m2_s
                - reservoir.vapor_kg_m2_s
                - reservoir.drainage_kg_m2_s);
        let heat_delta = reservoir.dt_s
            * (reservoir.non_vapor_heat_w_m2
                + reservoir.incident_liquid_kg_m2_s * reservoir.incident_enthalpy_j_kg
                - reservoir.vapor_kg_m2_s * (LV + CPV * (wet_temperature - TF))
                - reservoir.drainage_kg_m2_s * (CW * (wet_temperature - TF)));
        let mass_scale = reservoir
            .beginning_mass_kg_m2
            .abs()
            .max(mass_delta.abs())
            .max(1.0e-9);
        let heat_scale = reservoir
            .beginning_enthalpy_j_m2
            .abs()
            .max(heat_delta.abs())
            .max(1.0);
        let reconstructed_mass =
            reservoir.ending_mass_kg_m2 - reservoir.beginning_mass_kg_m2 - mass_delta;
        let reconstructed_enthalpy =
            reservoir.ending_enthalpy_j_m2 - reservoir.beginning_enthalpy_j_m2 - heat_delta;
        let rcap = (reservoir.dt_s * reservoir.drainage_kg_m2_s)
            .min(reservoir.liquid_capacity_kg_m2 - reservoir.diagnosed_liquid_mass_kg_m2);
        let valid_mass = |value: f64| {
            value.is_finite() && value.abs() <= 1.0e-9 && value.abs() / mass_scale <= 1.0e-8
        };
        let valid_heat = |value: f64| {
            value.is_finite() && value.abs() <= 1.0e-6 && value.abs() / heat_scale <= 1.0e-8
        };
        if !mass_delta.is_finite()
            || !heat_delta.is_finite()
            || !mass_scale.is_finite()
            || !heat_scale.is_finite()
            || !reconstructed_mass.is_finite()
            || !reconstructed_enthalpy.is_finite()
            || !rcap.is_finite()
        {
            return Err(saturation_error());
        }
        if !valid_mass(reconstructed_mass)
            || !valid_mass(reservoir.mass_residual_kg_m2)
            || !valid_heat(reconstructed_enthalpy)
            || !valid_heat(reservoir.enthalpy_residual_j_m2)
            || !valid_mass(result.residuals[base + 3])
            || !valid_heat(result.residuals[base + 4])
            || result.residuals[base + 5].abs() > 1.0e-9
            || result.residuals[base + 5].abs() / reservoir.liquid_capacity_kg_m2.max(1.0e-9)
                > 1.0e-8
            || rcap.abs() > 1.0e-9
            || rcap.abs() / reservoir.liquid_capacity_kg_m2.max(1.0e-9) > 1.0e-8
            || (reconstructed_mass - reservoir.mass_residual_kg_m2).abs() > 1.0e-9
            || (reconstructed_enthalpy - reservoir.enthalpy_residual_j_m2).abs() > 1.0e-6
        {
            return Err(domain_error());
        }
    }
    let normalized_residuals = normalized(result, input);
    if normalized_residuals.iter().any(|r| !r.is_finite()) {
        return Err(saturation_error());
    }
    if normalized_residuals.iter().any(|r| r.abs() > 1.0) {
        return Err(domain_error());
    }
    Ok(())
}

fn validate(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
) -> Result<(), M1CoupledError> {
    let occupancy_ids = input.topology.occupancy_ids.iter().collect::<BTreeSet<_>>();
    let soil_ids = input
        .topology
        .soil_layer_ids
        .iter()
        .collect::<BTreeSet<_>>();
    let structural = occupancy_ids.len() == 2
        && soil_ids.len() == 6
        && input.column.authority == CoveredColumnAuthority::V11SnowCovered
        && input.column.occupancies.len() == 2
        && input.column.ground.soil_nodes.len() == 6
        && input
            .column
            .occupancies
            .iter()
            .map(|x| &x.occupancy_id)
            .eq(input.topology.occupancy_ids.iter())
        && input
            .column
            .ground
            .soil_nodes
            .iter()
            .map(|x| &x.layer_id)
            .eq(input.topology.soil_layer_ids.iter())
        && input
            .column
            .stage3_lower_boundary
            .as_ref()
            .is_some_and(|x| x.native_snow_exchange.is_none())
        && input.column.stage3_optical.is_some();
    if !structural {
        return Err(LandSurfaceEnergyError::UnsupportedDomain("m1_diagnostic_envelope").into());
    }
    input.column.validate_transparent_liquid(None)?;
    validate_covered_shortwave_inputs(&input.column)?;
    let boundary = input
        .column
        .stage3_lower_boundary
        .as_ref()
        .ok_or_else(|| LandSurfaceEnergyError::UnsupportedDomain("m1_diagnostic_envelope"))?;
    boundary.validate()?;
    let optical = input
        .column
        .stage3_optical
        .as_ref()
        .ok_or_else(|| LandSurfaceEnergyError::UnsupportedDomain("m1_diagnostic_envelope"))?;
    if optical.snow_vis_albedo.to_bits() != boundary.snow_vis_albedo.to_bits()
        || optical.snow_nir_albedo.to_bits() != boundary.snow_nir_albedo.to_bits()
        || optical.stage3_albedo_state_sha256 != boundary.stage3_albedo_state_sha256
        || optical.forcing_receipt_sha256 != boundary.forcing_receipt_sha256
    {
        return Err(LandSurfaceEnergyError::StateLineage(
            "Stage-3 snow optical/lower-boundary identity",
        )
        .into());
    }
    if input.support.model_definition_id != "OPENWEPP_C3_WOODY_COLD_M1_V1"
        || input.support.occupancy_conditions.len() != 2
        || input
            .support
            .occupancy_conditions
            .iter()
            .map(|x| &x.occupancy_id)
            .ne(input.topology.occupancy_ids.iter())
        || input
            .reservoirs
            .iter()
            .map(|x| &x.occupancy_id)
            .ne(input.topology.occupancy_ids.iter())
        || input
            .support
            .occupancy_conditions
            .iter()
            .any(|x| !x.liquid_conducting || !x.full_supply)
    {
        return Err(condition_error());
    }
    if !input.column.interval_s.is_finite() {
        return Err(saturation_error());
    }
    if input.column.interval_s <= 0.0 {
        return Err(condition_error());
    }
    if input.column.interval_s < 60.0 {
        return Err(LandSurfaceEnergyError::SupportBelowMinimum {
            requested_ns: (input.column.interval_s * 1_000_000_000.0) as u128,
            minimum_ns: 60_000_000_000,
        }
        .into());
    }
    let column_physical = [
        input.column.pressure_pa,
        input.column.air_temperature_k,
        input.column.air_specific_humidity_kg_kg,
        input.column.ca_pa,
        input.column.canopy_to_atmosphere_heat_resistance_s_m,
        input.column.canopy_to_atmosphere_vapor_resistance_s_m,
        input.column.top_rain_kg_m2_tile,
        input.column.atmospheric_downward_longwave_w_m2,
    ];
    if column_physical.iter().any(|value| !value.is_finite())
        || trial.coordinates.iter().any(|x| !x.is_finite())
    {
        return Err(saturation_error());
    }
    if !(80_000.0..=110_000.0).contains(&input.column.pressure_pa) {
        return Err(saturation_error());
    }
    if !input.column.latent_heat_j_kg.is_finite() {
        return Err(saturation_error());
    }
    if input.column.latent_heat_j_kg <= 0.0 {
        return Err(condition_error());
    }
    if [0, 1, 2, 6, 7, 8]
        .iter()
        .any(|&index| !(263.15..=373.15).contains(&trial.coordinates[index]))
        || !(258.15..=373.15).contains(&trial.coordinates[12])
        || !(0.0..=0.1).contains(&trial.coordinates[13])
        || input.column.ca_pa >= input.column.pressure_pa
    {
        return Err(condition_error());
    }
    if input.column.pressure_pa <= 0.0
        || !(258.15..=373.15).contains(&input.column.air_temperature_k)
        || !(0.0..=0.1).contains(&input.column.air_specific_humidity_kg_kg)
        || input.column.ca_pa <= 0.0
        || input.column.canopy_to_atmosphere_heat_resistance_s_m <= 0.0
        || input.column.canopy_to_atmosphere_vapor_resistance_s_m <= 0.0
        || input.column.top_rain_kg_m2_tile < 0.0
    {
        return Err(condition_error());
    }
    for occupancy in &input.column.occupancies {
        let values = [
            occupancy.g0_umol_m2_s,
            occupancy.gb_leaf_m_s,
            occupancy.gb_wet_m_s,
            occupancy.gb_stem_m_s,
            occupancy.liquid_interception_fraction,
            occupancy.stemflow_fraction,
        ];
        if values.iter().any(|value| !value.is_finite()) {
            return Err(saturation_error());
        }
        if ((occupancy.sun.leaf_area_m2_m2_tile > 0.0
            || occupancy.shade.leaf_area_m2_m2_tile > 0.0)
            && (occupancy.g0_umol_m2_s <= 0.0 || input.column.ca_pa <= 0.0))
            || occupancy.gb_leaf_m_s <= 0.0
            || occupancy.gb_wet_m_s <= 0.0
            || occupancy.gb_stem_m_s <= 0.0
            || !(0.0..=1.0).contains(&occupancy.liquid_interception_fraction)
            || !(0.0..=1.0).contains(&occupancy.stemflow_fraction)
        {
            return Err(condition_error());
        }
        let capacity = occupancy.liquid_capacity_kg_m2_plant * (occupancy.lai + occupancy.sai);
        if !capacity.is_finite() || capacity <= 0.0 {
            return Err(domain_error());
        }
    }
    if input.column.top_rain_kg_m2_tile > 0.0
        && input
            .top_liquid_specific_enthalpy_j_kg
            .is_none_or(|x| !x.is_finite() || x < 0.0 || TF + x / CW > 373.15)
    {
        return Err(domain_error());
    }
    for value in &input.reservoirs {
        M1PhaseReservoir::from_mass_enthalpy(value.mass_kg_m2, value.enthalpy_j_m2)?;
    }
    Ok(())
}

#[derive(Clone)]
struct CoreOccupancy {
    sun: M1ZeroParLeafState,
    shade: M1ZeroParLeafState,
    wet_temperature_k: Option<f64>,
    wet_fraction: f64,
    wet_vapor: f64,
    wet_sensible: f64,
    wet_longwave: f64,
    emission: f64,
    non_vapor: f64,
    capacity: f64,
    mass_residual: f64,
    enthalpy_residual: f64,
    incident_liquid: f64,
    incident_enthalpy: f64,
}
#[derive(Clone)]
struct Core {
    residuals: Vec<f64>,
    scales: Vec<f64>,
    occupancies: Vec<CoreOccupancy>,
}

fn capacity_and_wet(
    reservoir: M1PhaseReservoir,
    capacity: f64,
) -> Result<(Option<f64>, f64, f64), M1CoupledError> {
    let temperature = reservoir.wet_temperature_k();
    let mass = reservoir.mass_kg_m2();
    let liquid = reservoir.liquid_mass_kg_m2();
    if !capacity.is_finite()
        || capacity <= 0.0
        || !mass.is_finite()
        || mass < 0.0
        || !liquid.is_finite()
        || liquid < 0.0
    {
        return Err(domain_error());
    }
    let fraction = (mass / capacity).powf(2.0 / 3.0).min(1.0);
    if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        return Err(domain_error());
    }
    Ok((temperature, liquid, fraction))
}

fn core(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
) -> Result<Core, M1CoupledError> {
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    crate::m1_physical_budget_enter(crate::M1PhysicalBudgetOperation::Core)
        .map_err(M1CoupledError::PhysicalBudget)?;
    M1_RUNTIME_ACTIVITY.with(|activity| activity.borrow_mut().complete_evaluator_entries += 1);
    let column = &input.column;
    let tcan = trial.coordinates[12];
    let qcan = trial.coordinates[13];
    let rho = column.pressure_pa / (crate::physics::DRY_AIR_GAS_CONSTANT_J_KG_K * tcan);
    if !rho.is_finite() || rho <= 0.0 {
        return Err(saturation_error());
    }
    let boundary = column
        .stage3_lower_boundary
        .as_ref()
        .ok_or_else(|| LandSurfaceEnergyError::UnsupportedDomain("m1_diagnostic_envelope"))?;
    let mut layers = Vec::with_capacity(2);
    let mut wet_info = Vec::with_capacity(2);
    for index in 0..2 {
        let occupancy = &column.occupancies[index];
        let base = 6 * index;
        let reservoir = M1PhaseReservoir::from_mass_enthalpy(
            trial.coordinates[base + 3],
            trial.coordinates[base + 4],
        )?;
        let capacity = occupancy.liquid_capacity_kg_m2_plant * (occupancy.lai + occupancy.sai);
        let (wet_t, _, wet_fraction) = capacity_and_wet(reservoir, capacity)?;
        let wet_t_for_lw = wet_t.unwrap_or(tcan);
        layers.push(crate::physics::CanopyLongwaveLayer {
            clumping_index: occupancy.clumping_index,
            leaf_area_index: occupancy.lai,
            stem_area_index: occupancy.sai,
            component_areas: [
                occupancy.sun.leaf_area_m2_m2_tile * (1.0 - wet_fraction),
                occupancy.shade.leaf_area_m2_m2_tile * (1.0 - wet_fraction),
                wet_fraction * (occupancy.lai + occupancy.sai),
                occupancy.stem_area_m2_m2_tile * (1.0 - wet_fraction),
            ],
            component_temperatures_k: [
                trial.coordinates[base],
                trial.coordinates[base + 1],
                wet_t_for_lw,
                trial.coordinates[base + 2],
            ],
        });
        wet_info.push((reservoir, capacity, wet_t, wet_fraction));
    }
    let longwave = crate::physics::reciprocal_longwave_column(
        column.atmospheric_downward_longwave_w_m2,
        trial.coordinates[14],
        &layers,
    )?;
    let mut residuals = vec![0.0; 21];
    let mut scales = vec![0.0; 21];
    let mut occupancies = Vec::with_capacity(2);
    let mut canopy_h = 0.0;
    let mut canopy_e = 0.0;
    let rain = column.top_rain_kg_m2_tile / column.interval_s;
    let top_rain_enthalpy = if rain > 0.0 {
        input
            .top_liquid_specific_enthalpy_j_kg
            .ok_or_else(domain_error)?
    } else {
        0.0
    };
    let upper = &column.occupancies[0];
    let upper_capture = crate::m1_liquid_interception_fraction(
        upper.lai + upper.sai,
        upper.liquid_interception_fraction,
    )? * rain;
    let upper_remainder = rain - upper_capture;
    let upper_through = (1.0 - upper.stemflow_fraction) * upper_remainder;
    let lower = &column.occupancies[1];
    let lower_capture_fraction = crate::m1_liquid_interception_fraction(
        lower.lai + lower.sai,
        lower.liquid_interception_fraction,
    )?;
    for index in 0..2 {
        let occupancy = &column.occupancies[index];
        let base = 6 * index;
        let (reservoir, capacity, wet_t, wet_fraction) = wet_info[index];
        let incident = if index == 0 {
            upper_capture
        } else {
            lower_capture_fraction * (upper_through + trial.coordinates[5])
        };
        if reservoir.mass_kg_m2() == 0.0 && reservoir.enthalpy_j_m2() == 0.0 && incident == 0.0 {
            let liquid_conducting_tissue = input
                .support
                .occupancy_conditions
                .iter()
                .find(|condition| condition.occupancy_id == occupancy.occupancy_id)
                .is_some_and(|condition| condition.liquid_conducting);
            for (surface, area, temperature) in [
                (
                    M1DrySurface::Sun,
                    occupancy.sun.leaf_area_m2_m2_tile,
                    trial.coordinates[base],
                ),
                (
                    M1DrySurface::Shade,
                    occupancy.shade.leaf_area_m2_m2_tile,
                    trial.coordinates[base + 1],
                ),
                (
                    M1DrySurface::Stem,
                    occupancy.stem_area_m2_m2_tile,
                    trial.coordinates[base + 2],
                ),
            ] {
                validate_m1_empty_surface(M1EmptySurfaceInput {
                    surface,
                    area_m2_m2: area,
                    temperature_k: temperature,
                    pressure_pa: column.pressure_pa,
                    qcan_kg_kg: qcan,
                    liquid_conducting_tissue,
                })?;
            }
        }
        let sun = m1_zero_par_leaf_state(
            occupancy.sun,
            occupancy.biochemical,
            trial.coordinates[base],
            tcan,
            qcan,
            column.pressure_pa,
            column.ca_pa,
            occupancy.gb_leaf_m_s,
            occupancy.g0_umol_m2_s,
            wet_fraction,
        )?;
        let shade = m1_zero_par_leaf_state(
            occupancy.shade,
            occupancy.biochemical,
            trial.coordinates[base + 1],
            tcan,
            qcan,
            column.pressure_pa,
            column.ca_pa,
            occupancy.gb_leaf_m_s,
            occupancy.g0_umol_m2_s,
            wet_fraction,
        )?;
        let wet_t = wet_t.unwrap_or(tcan);
        let phase = if reservoir.phase() == openwepp_vegetation::cold_canopy_m1::M1Phase::Ice {
            M1ExternalPhase::Ice
        } else {
            M1ExternalPhase::Liquid
        };
        let wet_q = m1_qsat_external(wet_t, column.pressure_pa, phase)?.specific_humidity_kg_kg;
        let wet_area = wet_fraction * (occupancy.lai + occupancy.sai);
        let wet_vapor = rho * occupancy.gb_wet_m_s * wet_area * (wet_q - qcan);
        let wet_sensible = rho * 1004.64 * occupancy.gb_wet_m_s * wet_area * (wet_t - tcan);
        let wet_longwave = longwave.component_net_w_m2[index][2];
        let plant_area = occupancy.lai + occupancy.sai;
        let dry_emission = occupancy.sun.leaf_area_m2_m2_tile * trial.coordinates[base].powi(4)
            + occupancy.shade.leaf_area_m2_m2_tile * trial.coordinates[base + 1].powi(4)
            + occupancy.stem_area_m2_m2_tile * trial.coordinates[base + 2].powi(4);
        let emission = crate::physics::STEFAN_BOLTZMANN_W_M2_K4
            * ((1.0 - wet_fraction) * dry_emission / plant_area + wet_fraction * wet_t.powi(4));
        let q_nonvapor = wet_fraction
            * (occupancy.sun.absorbed_shortwave_w_m2_tile
                + occupancy.shade.absorbed_shortwave_w_m2_tile
                + occupancy.stem_absorbed_shortwave_w_m2_tile)
            + wet_longwave
            - wet_sensible;
        let incident_energy = if index == 0 {
            upper_capture * top_rain_enthalpy
        } else {
            let upper_wet_t = wet_info[0].2.unwrap_or(tcan);
            lower_capture_fraction
                * (upper_through * top_rain_enthalpy
                    + trial.coordinates[5] * CW * (upper_wet_t - TF))
        };
        let incident_h = if incident > 0.0 {
            incident_energy / incident
        } else {
            0.0
        };
        let drainage = trial.coordinates[base + 5];
        let hv = LV + CPV * (wet_t - TF);
        let hl = CW * (wet_t - TF);
        residuals[base] = if occupancy.sun.leaf_area_m2_m2_tile == 0.0 {
            trial.coordinates[base] - inactive_component_temperature_anchor_k(0, tcan)
        } else {
            occupancy.sun.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction)
                + longwave.component_net_w_m2[index][0]
                - rho
                    * 1004.64
                    * occupancy.gb_leaf_m_s
                    * occupancy.sun.leaf_area_m2_m2_tile
                    * (1.0 - wet_fraction)
                    * (trial.coordinates[base] - tcan)
                - column.latent_heat_j_kg * sun.vapor_kg_m2_tile_s
        };
        residuals[base + 1] = if occupancy.shade.leaf_area_m2_m2_tile == 0.0 {
            trial.coordinates[base + 1] - inactive_component_temperature_anchor_k(1, tcan)
        } else {
            occupancy.shade.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction)
                + longwave.component_net_w_m2[index][1]
                - rho
                    * 1004.64
                    * occupancy.gb_leaf_m_s
                    * occupancy.shade.leaf_area_m2_m2_tile
                    * (1.0 - wet_fraction)
                    * (trial.coordinates[base + 1] - tcan)
                - column.latent_heat_j_kg * shade.vapor_kg_m2_tile_s
        };
        residuals[base + 2] = if occupancy.stem_area_m2_m2_tile == 0.0 {
            trial.coordinates[base + 2] - inactive_component_temperature_anchor_k(3, tcan)
        } else {
            (1.0 - wet_fraction) * occupancy.stem_absorbed_shortwave_w_m2_tile
                + longwave.component_net_w_m2[index][3]
                - rho
                    * 1004.64
                    * occupancy.gb_stem_m_s
                    * occupancy.stem_area_m2_m2_tile
                    * (1.0 - wet_fraction)
                    * (trial.coordinates[base + 2] - tcan)
        };
        residuals[base + 3] = trial.coordinates[base + 3]
            - input.reservoirs[index].mass_kg_m2
            - column.interval_s * (incident - wet_vapor - drainage);
        residuals[base + 4] = trial.coordinates[base + 4]
            - input.reservoirs[index].enthalpy_j_m2
            - column.interval_s
                * (q_nonvapor + incident * incident_h - wet_vapor * hv - drainage * hl);
        residuals[base + 5] = evaluate_m1_capacity_residual(M1CapacityTrial::new(
            reservoir.liquid_mass_kg_m2(),
            capacity,
            drainage,
            column.interval_s,
        ))?
        .residual_kg_m2;
        let dry_area = 1.0 - wet_fraction;
        scales[base] = (occupancy.sun.absorbed_shortwave_w_m2_tile * dry_area).abs()
            + longwave.component_net_w_m2[index][0].abs()
            + (rho
                * 1004.64
                * occupancy.gb_leaf_m_s
                * occupancy.sun.leaf_area_m2_m2_tile
                * dry_area
                * (trial.coordinates[base] - tcan))
                .abs()
            + (column.latent_heat_j_kg * sun.vapor_kg_m2_tile_s).abs();
        scales[base + 1] = (occupancy.shade.absorbed_shortwave_w_m2_tile * dry_area).abs()
            + longwave.component_net_w_m2[index][1].abs()
            + (rho
                * 1004.64
                * occupancy.gb_leaf_m_s
                * occupancy.shade.leaf_area_m2_m2_tile
                * dry_area
                * (trial.coordinates[base + 1] - tcan))
                .abs()
            + (column.latent_heat_j_kg * shade.vapor_kg_m2_tile_s).abs();
        scales[base + 2] = (occupancy.stem_absorbed_shortwave_w_m2_tile * dry_area).abs()
            + longwave.component_net_w_m2[index][3].abs()
            + (rho
                * 1004.64
                * occupancy.gb_stem_m_s
                * occupancy.stem_area_m2_m2_tile
                * dry_area
                * (trial.coordinates[base + 2] - tcan))
                .abs();
        scales[base + 3] = (incident - wet_vapor - drainage).abs() * column.interval_s;
        scales[base + 4] = (q_nonvapor + incident * incident_h - wet_vapor * hv - drainage * hl)
            .abs()
            * column.interval_s;
        canopy_h += wet_sensible
            + rho
                * 1004.64
                * occupancy.gb_leaf_m_s
                * (1.0 - wet_fraction)
                * (occupancy.sun.leaf_area_m2_m2_tile * (trial.coordinates[base] - tcan)
                    + occupancy.shade.leaf_area_m2_m2_tile * (trial.coordinates[base + 1] - tcan))
            + rho
                * 1004.64
                * occupancy.gb_stem_m_s
                * occupancy.stem_area_m2_m2_tile
                * (1.0 - wet_fraction)
                * (trial.coordinates[base + 2] - tcan);
        canopy_e += sun.vapor_kg_m2_tile_s + shade.vapor_kg_m2_tile_s + wet_vapor;
        occupancies.push(CoreOccupancy {
            sun,
            shade,
            wet_temperature_k: wet_info[index].2,
            wet_fraction,
            wet_vapor,
            wet_sensible,
            wet_longwave,
            emission,
            non_vapor: q_nonvapor,
            capacity,
            mass_residual: residuals[base + 3],
            enthalpy_residual: residuals[base + 4],
            incident_liquid: incident,
            incident_enthalpy: incident_h,
        });
    }
    residuals[12] = canopy_h + boundary.sensible_to_canopy_air_w_m2
        - rho * 1004.64 * (tcan - column.air_temperature_k)
            / column.canopy_to_atmosphere_heat_resistance_s_m;
    residuals[13] = canopy_e + boundary.vapor_to_canopy_air_kg_m2_s
        - rho * (qcan - column.air_specific_humidity_kg_kg)
            / column.canopy_to_atmosphere_vapor_resistance_s_m;
    let reference_heat = rho * 1004.64 * (tcan - column.air_temperature_k)
        / column.canopy_to_atmosphere_heat_resistance_s_m;
    let reference_vapor = rho * (qcan - column.air_specific_humidity_kg_kg)
        / column.canopy_to_atmosphere_vapor_resistance_s_m;
    scales[12] = canopy_h.abs() + boundary.sensible_to_canopy_air_w_m2.abs() + reference_heat.abs();
    scales[13] = canopy_e
        .abs()
        .max(boundary.vapor_to_canopy_air_kg_m2_s.abs())
        .max(reference_vapor.abs());
    residuals[14] = trial.coordinates[14] - boundary.snow_temperature_k;
    for soil in 0..6 {
        residuals[15 + soil] =
            trial.coordinates[15 + soil] - column.ground.soil_nodes[soil].beginning_temperature_k;
    }
    Ok(Core {
        residuals,
        scales,
        occupancies,
    })
}

fn fd_trial_is_admissible(trial: &M1CoupledColumnTrial) -> bool {
    let temperature_coordinates = [0, 1, 2, 6, 7, 8];
    if temperature_coordinates
        .iter()
        .any(|&index| !(263.15..=373.15).contains(&trial.coordinates[index]))
        || !(258.15..=373.15).contains(&trial.coordinates[12])
        || !(0.0..=0.1).contains(&trial.coordinates[13])
        || trial.coordinates[5] < 0.0
        || trial.coordinates[11] < 0.0
    {
        return false;
    }
    [0, 6].into_iter().all(|base| {
        M1PhaseReservoir::from_mass_enthalpy(
            trial.coordinates[base + 3],
            trial.coordinates[base + 4],
        )
        .is_ok()
    })
}

#[cfg(any(test, feature = "test-support"))]
fn fd_trial_rejection_reason(trial: &M1CoupledColumnTrial) -> Option<String> {
    for index in [0, 1, 2, 6, 7, 8] {
        if !(263.15..=373.15).contains(&trial.coordinates[index]) {
            return Some(format!(
                "temperature[{index}]={:.17e}",
                trial.coordinates[index]
            ));
        }
    }
    if !(258.15..=373.15).contains(&trial.coordinates[12]) {
        return Some(format!("tcan={:.17e}", trial.coordinates[12]));
    }
    if !(0.0..=0.1).contains(&trial.coordinates[13]) {
        return Some(format!("qcan={:.17e}", trial.coordinates[13]));
    }
    for index in [5, 11] {
        if trial.coordinates[index] < 0.0 {
            return Some(format!(
                "drainage[{index}]={:.17e}",
                trial.coordinates[index]
            ));
        }
    }
    for base in [0, 6] {
        if let Err(error) = M1PhaseReservoir::from_mass_enthalpy(
            trial.coordinates[base + 3],
            trial.coordinates[base + 4],
        ) {
            return Some(format!(
                "reservoir[{base}] M={:.17e} H={:.17e}: {error:?}",
                trial.coordinates[base + 3],
                trial.coordinates[base + 4]
            ));
        }
    }
    None
}

fn capacity_selected_derivative(
    reservoir: M1PhaseReservoir,
    capacity: f64,
    drainage: f64,
    duration_s: f64,
    coordinate: usize,
    base: usize,
) -> f64 {
    let drainage_limited = duration_s * drainage < capacity - reservoir.liquid_mass_kg_m2();
    if drainage_limited {
        return if coordinate == base + 5 {
            duration_s
        } else {
            0.0
        };
    }
    let (mass_derivative, enthalpy_derivative) = match reservoir.phase() {
        openwepp_vegetation::cold_canopy_m1::M1Phase::Liquid => (1.0, 0.0),
        openwepp_vegetation::cold_canopy_m1::M1Phase::Mixed => (1.0, 1.0 / LF),
        openwepp_vegetation::cold_canopy_m1::M1Phase::Empty
        | openwepp_vegetation::cold_canopy_m1::M1Phase::Ice => (0.0, 0.0),
    };
    if coordinate == base + 3 {
        -mass_derivative
    } else if coordinate == base + 4 {
        -enthalpy_derivative
    } else {
        0.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum M1PhaseJoinFailure {
    InconsistentFinalSet {
        activity: M1PhaseJoinActivity,
    },
    UnrepresentableProbe {
        activity: M1PhaseJoinActivity,
    },
    Source {
        error: Box<M1CoupledError>,
        activity: M1PhaseJoinActivity,
    },
}
impl M1PhaseJoinFailure {
    #[must_use]
    pub fn code(&self) -> &str {
        match self {
            Self::Source { error, .. } => error.code(),
            _ => "LSEB-E-034",
        }
    }
    #[must_use]
    pub const fn controller_activity(&self) -> M1PhaseJoinActivity {
        match self {
            Self::InconsistentFinalSet { activity }
            | Self::UnrepresentableProbe { activity }
            | Self::Source { activity, .. } => *activity,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct M1PhaseJoinActivity {
    pub assemblies: u32,
    pub linear_solves: u32,
    pub candidate_attempts: u32,
    pub backtracking_attempts: u32,
    pub materialization_attempts: u32,
    pub receipt_attempts: u32,
    pub state_install_attempts: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct M1LoopActivityObservation {
    pub complete_evaluator_entries: u32,
    pub hydraulic_entries: u32,
    pub actual_hydraulic_entries: u32,
    pub trial_entries: u32,
    pub materialization_entries: u32,
    pub predictor_assemblies: u32,
    pub final_assemblies: u32,
    pub full_linear_dimension_counts: [u32; 22],
    pub reduced_linear_dimension_counts: [u32; 22],
    pub controller_failure_activity: Option<M1PhaseJoinActivity>,
    pub no_update_cumulative_backtracking: Option<u32>,
    pub last_candidate_coordinates: Option<[f64; 21]>,
    pub last_current_coordinates: Option<[f64; 21]>,
    pub last_step_norms: Option<StepNorms>,
    pub last_current_norm: Option<f64>,
    pub last_candidate_norm: Option<f64>,
    pub installed_current_norm: Option<f64>,
    pub installed_candidate_norm: Option<f64>,
    pub installed_candidate_coordinates: Option<[f64; 21]>,
    pub installed_current_coordinates: Option<[f64; 21]>,
    pub last_direction: Option<Vec<f64>>,
    pub last_factor: Option<f64>,
    pub last_current_dry_temperatures: Option<[f64; 14]>,
    pub last_candidate_dry_temperatures: Option<[f64; 14]>,
    pub last_current_wet_temperatures: Option<[Option<f64>; 2]>,
    pub last_candidate_wet_temperatures: Option<[Option<f64>; 2]>,
    pub last_current_beta: Option<[f64; 4]>,
    pub last_candidate_beta: Option<[f64; 4]>,
    pub last_current_ci_pa: Option<[f64; 4]>,
    pub last_candidate_ci_pa: Option<[f64; 4]>,
}

impl M1LoopActivityObservation {
    const fn new() -> Self {
        Self {
            complete_evaluator_entries: 0,
            hydraulic_entries: 0,
            actual_hydraulic_entries: 0,
            trial_entries: 0,
            materialization_entries: 0,
            predictor_assemblies: 0,
            final_assemblies: 0,
            full_linear_dimension_counts: [0; 22],
            reduced_linear_dimension_counts: [0; 22],
            controller_failure_activity: None,
            no_update_cumulative_backtracking: None,
            last_candidate_coordinates: None,
            last_current_coordinates: None,
            last_step_norms: None,
            last_current_norm: None,
            last_candidate_norm: None,
            installed_current_norm: None,
            installed_candidate_norm: None,
            installed_candidate_coordinates: None,
            installed_current_coordinates: None,
            last_direction: None,
            last_factor: None,
            last_current_dry_temperatures: None,
            last_candidate_dry_temperatures: None,
            last_current_wet_temperatures: None,
            last_candidate_wet_temperatures: None,
            last_current_beta: None,
            last_candidate_beta: None,
            last_current_ci_pa: None,
            last_candidate_ci_pa: None,
        }
    }
}

fn m1_record_linear_dimension(counts: &mut [u32; 22], dimension: usize) {
    if let Some(count) = counts.get_mut(dimension) {
        *count = count.saturating_add(1);
    }
}

#[cfg(test)]
pub(crate) fn m1_reset_loop_activity_observation_for_test() {
    M1_RUNTIME_ACTIVITY.with(|observation| {
        *observation.borrow_mut() = M1LoopActivityObservation::new();
    });
}

#[cfg(test)]
pub(crate) fn m1_loop_activity_observation_for_test() -> M1LoopActivityObservation {
    M1_RUNTIME_ACTIVITY.with(|observation| observation.borrow().clone())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct M1PhaseJoinSelection {
    phase: openwepp_vegetation::cold_canopy_m1::M1Phase,
    capacity: openwepp_vegetation::cold_canopy_m1::M1CapacityBranch,
    corrected: bool,
    mass: f64,
    enthalpy: f64,
    capacity_value: f64,
    drainage: f64,
    duration_s: f64,
}
impl M1PhaseJoinSelection {
    #[must_use]
    pub const fn phase(self) -> openwepp_vegetation::cold_canopy_m1::M1Phase {
        self.phase
    }
    #[must_use]
    pub const fn capacity_branch(self) -> openwepp_vegetation::cold_canopy_m1::M1CapacityBranch {
        self.capacity
    }
    #[must_use]
    pub const fn requires_corrected_solve(self) -> bool {
        self.corrected
    }
}

fn classify_m1_phase_join_active_set(
    reservoir: M1PhaseReservoir,
    capacity: f64,
    drainage: f64,
    duration_s: f64,
    d_mass: f64,
    d_enthalpy: f64,
    d_drainage: f64,
) -> Result<M1PhaseJoinSelection, M1PhaseJoinFailure> {
    if ![
        capacity, drainage, duration_s, d_mass, d_enthalpy, d_drainage,
    ]
    .iter()
    .all(|x| x.is_finite())
    {
        return Err(M1PhaseJoinFailure::UnrepresentableProbe {
            activity: M1PhaseJoinActivity::default(),
        });
    }
    let current = reservoir.phase();
    let phase = if reservoir.enthalpy_j_m2() == 0.0 && reservoir.mass_kg_m2() > 0.0 {
        if d_enthalpy < 0.0 {
            openwepp_vegetation::cold_canopy_m1::M1Phase::Mixed
        } else {
            openwepp_vegetation::cold_canopy_m1::M1Phase::Liquid
        }
    } else if reservoir.enthalpy_j_m2() + LF * reservoir.mass_kg_m2() == 0.0
        && reservoir.mass_kg_m2() > 0.0
    {
        if d_enthalpy + LF * d_mass < 0.0 {
            openwepp_vegetation::cold_canopy_m1::M1Phase::Ice
        } else {
            openwepp_vegetation::cold_canopy_m1::M1Phase::Mixed
        }
    } else {
        current
    };
    let d_liquid = match phase {
        openwepp_vegetation::cold_canopy_m1::M1Phase::Liquid => d_mass,
        openwepp_vegetation::cold_canopy_m1::M1Phase::Mixed => d_mass + d_enthalpy / LF,
        openwepp_vegetation::cold_canopy_m1::M1Phase::Empty
        | openwepp_vegetation::cold_canopy_m1::M1Phase::Ice => 0.0,
    };
    let lower_normal = d_enthalpy + LF * d_mass;
    let capacity_normal = duration_s * d_drainage + d_liquid;
    if !d_liquid.is_finite() || !lower_normal.is_finite() || !capacity_normal.is_finite() {
        return Err(M1PhaseJoinFailure::UnrepresentableProbe {
            activity: M1PhaseJoinActivity::default(),
        });
    }
    let cap_tie = duration_s * drainage == capacity - reservoir.liquid_mass_kg_m2();
    let capacity_branch = if cap_tie {
        if capacity_normal >= 0.0 {
            openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Capacity
        } else {
            openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Drainage
        }
    } else if duration_s * drainage < capacity - reservoir.liquid_mass_kg_m2() {
        openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Drainage
    } else {
        openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Capacity
    };
    let current_capacity = if duration_s * drainage < capacity - reservoir.liquid_mass_kg_m2() {
        openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Drainage
    } else {
        openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Capacity
    };
    Ok(M1PhaseJoinSelection {
        phase,
        capacity: capacity_branch,
        corrected: phase != current || capacity_branch != current_capacity,
        mass: reservoir.mass_kg_m2(),
        enthalpy: reservoir.enthalpy_j_m2(),
        capacity_value: capacity,
        drainage,
        duration_s,
    })
}

// The selector's public diagnostic boundary also checks its local M/H/D ray.
// The production controller classifies first and performs the same probe in
// the selected assembly, so failed work belongs to that actual assembly.
pub fn select_m1_phase_join_active_set(
    reservoir: M1PhaseReservoir,
    capacity: f64,
    drainage: f64,
    duration_s: f64,
    d_mass: f64,
    d_enthalpy: f64,
    d_drainage: f64,
) -> Result<M1PhaseJoinSelection, M1PhaseJoinFailure> {
    let selected = classify_m1_phase_join_active_set(
        reservoir, capacity, drainage, duration_s, d_mass, d_enthalpy, d_drainage,
    )?;
    let mut direction = [0.0; 21];
    direction[3] = d_mass;
    direction[4] = d_enthalpy;
    direction[5] = d_drainage;
    let factor = m1_probe_factor(&direction);
    check_m1_probe_side(
        &selected,
        d_mass,
        d_enthalpy,
        d_drainage,
        selected.mass + factor * d_mass,
        selected.enthalpy + factor * d_enthalpy,
        drainage + factor * d_drainage,
    )?;
    Ok(selected)
}

#[cfg(test)]
pub fn m1_phase_join_set_matches_direction(
    selection: &M1PhaseJoinSelection,
    d_mass: f64,
    d_enthalpy: f64,
    d_drainage: f64,
) -> bool {
    let Ok(reservoir) = M1PhaseReservoir::from_mass_enthalpy(selection.mass, selection.enthalpy)
    else {
        return false;
    };
    classify_m1_phase_join_active_set(
        reservoir,
        selection.capacity_value,
        selection.drainage,
        selection.duration_s,
        d_mass,
        d_enthalpy,
        d_drainage,
    )
    .is_ok_and(|next| next.phase == selection.phase && next.capacity == selection.capacity)
}

fn m1_probe_factor(direction: &[f64; 21]) -> f64 {
    let mut factor = 1.0_f64;
    for (column, value) in direction.iter().enumerate() {
        let step = match column {
            3 | 9 | 13 => 1.0e-8,
            4 | 10 => 1.0e-3,
            5 | 11 => 1.0e-10,
            _ => 1.0e-4,
        };
        if *value != 0.0 {
            factor = factor.min(step / value.abs());
        }
    }
    factor
}

fn check_m1_probe_side(
    selected: &M1PhaseJoinSelection,
    dm: f64,
    dh: f64,
    dd: f64,
    mass: f64,
    enthalpy: f64,
    drainage: f64,
) -> Result<(), M1PhaseJoinFailure> {
    let unrepresentable = || M1PhaseJoinFailure::UnrepresentableProbe {
        activity: M1PhaseJoinActivity::default(),
    };
    let next = M1PhaseReservoir::from_mass_enthalpy(mass, enthalpy).map_err(|error| {
        M1PhaseJoinFailure::Source {
            error: Box::new(error.into()),
            activity: M1PhaseJoinActivity::default(),
        }
    })?;
    if selected.mass > 0.0 {
        let normal = if selected.enthalpy == 0.0 {
            dh
        } else if selected.enthalpy + LF * selected.mass == 0.0 {
            dh + LF * dm
        } else {
            0.0
        };
        if normal != 0.0 {
            let observed = if selected.enthalpy == 0.0 {
                enthalpy
            } else {
                enthalpy + LF * mass
            };
            if observed == 0.0
                || !observed.is_finite()
                || observed.is_sign_negative() != normal.is_sign_negative()
                || next.phase() != selected.phase
            {
                return Err(unrepresentable());
            }
        }
    }
    let current = M1PhaseReservoir::from_mass_enthalpy(selected.mass, selected.enthalpy).map_err(
        |error| M1PhaseJoinFailure::Source {
            error: Box::new(error.into()),
            activity: M1PhaseJoinActivity::default(),
        },
    )?;
    if selected.duration_s * selected.drainage
        == selected.capacity_value - current.liquid_mass_kg_m2()
    {
        use openwepp_vegetation::cold_canopy_m1::M1Phase;
        let dml = match selected.phase {
            M1Phase::Liquid => dm,
            M1Phase::Mixed => dm + dh / LF,
            M1Phase::Ice | M1Phase::Empty => 0.0,
        };
        let normal = selected.duration_s * dd + dml;
        if normal != 0.0 {
            let observed = selected.duration_s * drainage
                - (selected.capacity_value - next.liquid_mass_kg_m2());
            if observed == 0.0
                || !observed.is_finite()
                || observed.is_sign_negative() != normal.is_sign_negative()
            {
                return Err(unrepresentable());
            }
        }
    }
    Ok(())
}

fn m1_selected_probe(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    selected: &[M1PhaseJoinSelection; 2],
    direction: &[f64; 21],
    base: &Core,
    iteration: u32,
    backtracking_count: u32,
) -> Result<(), M1CoupledError> {
    let has_tie = selected.iter().any(|value| {
        value.mass > 0.0 && (value.enthalpy == 0.0 || value.enthalpy + LF * value.mass == 0.0)
            || M1PhaseReservoir::from_mass_enthalpy(value.mass, value.enthalpy).is_ok_and(
                |reservoir| {
                    value.duration_s * value.drainage
                        == value.capacity_value - reservoir.liquid_mass_kg_m2()
                },
            )
    });
    if !has_tie {
        return Ok(());
    }
    let factor = m1_probe_factor(direction);
    let mut probe = trial.clone();
    for (value, delta) in probe.coordinates.iter_mut().zip(direction) {
        *value += factor * delta;
    }
    validate(input, &probe)?;
    for (occupancy, selection) in selected.iter().enumerate() {
        let offset = occupancy * 6;
        check_m1_probe_side(
            selection,
            direction[offset + 3],
            direction[offset + 4],
            direction[offset + 5],
            probe.coordinates[offset + 3],
            probe.coordinates[offset + 4],
            probe.coordinates[offset + 5],
        )
        .map_err(|failure| match failure {
            M1PhaseJoinFailure::Source { error, .. } => *error,
            _ => m1_failure(
                crate::numerics::NumericalFailureKind::PhaseActiveSetUnrepresentable,
                iteration,
                input,
                base,
                trial,
                backtracking_count,
                StepNorms {
                    temperature_k: None,
                    humidity_kg_kg: None,
                    ci_pa: None,
                    hydraulic_mm: None,
                    beta: Some(0.0),
                },
                None,
                None,
            ),
        })?;
    }
    // This is the canonical primal evaluator, not a selected-phase surrogate.
    core(input, &probe)?;
    Ok(())
}

fn evaluate_core_with_jacobian(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    selected_probe: Option<(&[M1PhaseJoinSelection; 2], &[f64; 21], u32, u32)>,
) -> Result<(Core, Vec<Vec<f64>>, Vec<Vec<f64>>), M1CoupledError> {
    #[cfg(test)]
    M1_JACOBIAN_INVOCATIONS.with(|count| count.set(count.get() + 1));
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    crate::m1_physical_budget_enter(crate::M1PhysicalBudgetOperation::RawJacobian)
        .map_err(M1CoupledError::PhysicalBudget)?;
    validate(input, trial)?;
    let result = core(input, trial)?;
    if let Some((selected, direction, iteration, backtracking_count)) = selected_probe {
        m1_selected_probe(
            input,
            trial,
            selected,
            direction,
            &result,
            iteration,
            backtracking_count,
        )?;
    }
    let selected_set = selected_probe.map(|value| value.0);
    let base_normalized = normalized(&result, input);
    let tolerances = m1_tolerances(input, &result);
    let mut jacobian = vec![vec![0.0; 21]; 21];
    let mut normalized_jacobian = vec![vec![0.0; 21]; 21];
    for column in 0..21 {
        let step = match column {
            3 | 9 => 1.0e-8,
            4 | 10 => 1.0e-3,
            5 | 11 => 1.0e-10,
            13 => 1.0e-8,
            _ => 1.0e-4,
        };
        let leaf_phase_join = matches!(column, 0 | 1 | 6 | 7)
            && trial.coordinates[column] == TF
            && if column % 6 == 0 {
                input.column.occupancies[column / 6]
                    .sun
                    .leaf_area_m2_m2_tile
                    > 0.0
            } else {
                input.column.occupancies[column / 6]
                    .shade
                    .leaf_area_m2_m2_tile
                    > 0.0
            };
        let phase_join = match column {
            3 => {
                let mass = trial.coordinates[3];
                let enthalpy = trial.coordinates[4];
                enthalpy + LF * mass == 0.0
            }
            4 => {
                let mass = trial.coordinates[3];
                let enthalpy = trial.coordinates[4];
                enthalpy == 0.0 || enthalpy + LF * mass == 0.0
            }
            9 => {
                let mass = trial.coordinates[9];
                let enthalpy = trial.coordinates[10];
                enthalpy + LF * mass == 0.0
            }
            10 => {
                let mass = trial.coordinates[9];
                let enthalpy = trial.coordinates[10];
                enthalpy == 0.0 || enthalpy + LF * mass == 0.0
            }
            _ => false,
        };
        let force_forward = leaf_phase_join || phase_join;
        let force_backward = selected_set.is_some_and(|selected| {
            let base = if column < 6 { 0 } else { 6 };
            let occupancy = base / 6;
            let h = trial.coordinates[base + 4];
            let lower = h + LF * trial.coordinates[base + 3] == 0.0;
            (selected[occupancy].phase == openwepp_vegetation::cold_canopy_m1::M1Phase::Mixed
                && h == 0.0
                && column == base + 4)
                || (selected[occupancy].phase == openwepp_vegetation::cold_canopy_m1::M1Phase::Ice
                    && lower
                    && matches!(column, c if c == base + 3 || c == base + 4))
        });
        let high = if force_backward {
            None
        } else {
            let high_trial = trial.clone().with_coordinate_offset(column, step)?;
            fd_trial_is_admissible(&high_trial)
                .then(|| core(input, &high_trial))
                .transpose()?
        };
        let high_normalized = high.as_ref().map(|value| normalized(value, input));
        let low = if force_forward && !force_backward {
            None
        } else {
            let low_trial = trial.clone().with_coordinate_offset(column, -step)?;
            fd_trial_is_admissible(&low_trial)
                .then(|| core(input, &low_trial))
                .transpose()?
        };
        if high.is_none() && low.is_none() {
            return Err(domain_error());
        }
        let low_normalized = low.as_ref().map(|value| normalized(value, input));
        for row in 0..21 {
            jacobian[row][column] = match (&high, &low) {
                (Some(high), Some(low)) => {
                    (high.residuals[row] - low.residuals[row]) / (2.0 * step)
                }
                (Some(high), None) => (high.residuals[row] - result.residuals[row]) / step,
                (None, Some(low)) => (result.residuals[row] - low.residuals[row]) / step,
                (None, None) => return Err(domain_error()),
            };
            normalized_jacobian[row][column] = match (&high_normalized, &low_normalized) {
                (Some(high), Some(low)) => (high[row] - low[row]) / (2.0 * step),
                (Some(high), None) => (high[row] - base_normalized[row]) / step,
                (None, Some(low)) => (base_normalized[row] - low[row]) / step,
                (None, None) => return Err(domain_error()),
            };
        }
        for base in [0, 6] {
            let reservoir = M1PhaseReservoir::from_mass_enthalpy(
                trial.coordinates[base + 3],
                trial.coordinates[base + 4],
            )?;
            let capacity = input.column.occupancies[base / 6].liquid_capacity_kg_m2_plant
                * (input.column.occupancies[base / 6].lai + input.column.occupancies[base / 6].sai);
            jacobian[base + 5][column] = selected_set.map_or_else(
                || {
                    capacity_selected_derivative(
                        reservoir,
                        capacity,
                        trial.coordinates[base + 5],
                        input.column.interval_s,
                        column,
                        base,
                    )
                },
                |selected| match selected[base / 6].capacity {
                    openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Drainage => {
                        if column == base + 5 {
                            input.column.interval_s
                        } else {
                            0.0
                        }
                    }
                    openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Capacity => {
                        let (mass, enthalpy) = match selected[base / 6].phase {
                            openwepp_vegetation::cold_canopy_m1::M1Phase::Liquid => (1.0, 0.0),
                            openwepp_vegetation::cold_canopy_m1::M1Phase::Mixed => (1.0, 1.0 / LF),
                            openwepp_vegetation::cold_canopy_m1::M1Phase::Empty
                            | openwepp_vegetation::cold_canopy_m1::M1Phase::Ice => (0.0, 0.0),
                        };
                        if column == base + 3 {
                            -mass
                        } else if column == base + 4 {
                            -enthalpy
                        } else {
                            0.0
                        }
                    }
                },
            );
            normalized_jacobian[base + 5][column] =
                jacobian[base + 5][column] / tolerances[base + 5];
        }
    }
    // Inactive dry components are exact anchor identities.  Their Jacobian is
    // assembled directly so finite differences never cross the anchor max.
    for index in 0..2 {
        let occupancy = &input.column.occupancies[index];
        let base = index * 6;
        for (offset, area) in [
            occupancy.sun.leaf_area_m2_m2_tile,
            occupancy.shade.leaf_area_m2_m2_tile,
            occupancy.stem_area_m2_m2_tile,
        ]
        .into_iter()
        .enumerate()
        {
            if area == 0.0 {
                let row = base + offset;
                jacobian[row].fill(0.0);
                normalized_jacobian[row].fill(0.0);
                jacobian[row][row] = 1.0;
                // Retained direct anchor row uses the 1 K coordinate unit.
                normalized_jacobian[row][row] = 1.0;
            }
        }
    }
    Ok((result, jacobian, normalized_jacobian))
}

#[cfg(test)]
pub fn m1_reset_work_observation_for_test() {
    M1_JACOBIAN_INVOCATIONS.with(|count| count.set(0));
    M1_CONTROLLER_ASSEMBLIES.with(|count| count.set(0));
}

#[cfg(test)]
pub fn m1_work_observation_for_test() -> (u32, u32) {
    (
        M1_JACOBIAN_INVOCATIONS.with(Cell::get),
        M1_CONTROLLER_ASSEMBLIES.with(Cell::get),
    )
}

fn materialize_m1_coupled_column(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    result: Core,
    jacobian: Option<Vec<Vec<f64>>>,
) -> Result<M1CoupledEvaluation, M1CoupledError> {
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    crate::m1_physical_budget_enter(crate::M1PhysicalBudgetOperation::Materialization)
        .map_err(M1CoupledError::PhysicalBudget)?;
    M1_RUNTIME_ACTIVITY.with(|activity| activity.borrow_mut().materialization_entries += 1);
    let mut outputs = Vec::with_capacity(2);
    let mut reservoirs = Vec::with_capacity(2);
    for index in 0..2 {
        let occupancy = &input.column.occupancies[index];
        let detail = &result.occupancies[index];
        let hydraulics = solve_m1_full_supply_hydraulics(
            &input.column,
            occupancy,
            [
                detail.sun.vapor_kg_m2_tile_s,
                detail.shade.vapor_kg_m2_tile_s,
            ],
            [detail.sun.gas_branch, detail.shade.gas_branch],
        )?;
        let all_root_layer_ids = hydraulics.root_layer_ids;
        let all_root_fluxes_kg_m2_tile_s = hydraulics.root_fluxes_kg_m2_tile_s;
        let active_indices: Vec<_> = occupancy
            .root_layers
            .iter()
            .enumerate()
            .filter_map(|(root, layer)| {
                (layer.accessible && !layer.frozen && layer.root_fraction > 0.0).then_some(root)
            })
            .collect();
        let root_layer_ids = active_indices
            .iter()
            .map(|root| all_root_layer_ids[*root].clone())
            .collect();
        let root_fluxes_kg_m2_tile_s = active_indices
            .iter()
            .map(|root| all_root_fluxes_kg_m2_tile_s[*root])
            .collect();
        outputs.push(M1OccupancyEvaluation {
            occupancy_id: occupancy.occupancy_id.clone(),
            sun: M1LeafEvaluation {
                gas_branch: format!("{:?}", detail.sun.gas_branch),
                beta: detail.sun.beta,
                internal_specific_humidity_kg_kg: detail.sun.internal_specific_humidity_kg_kg,
                dark_respiration: detail.sun.dark_respiration_umol_co2_m2_leaf_s,
                net_assimilation: detail.sun.net_assimilation_umol_co2_m2_leaf_s,
                gross_assimilation: detail.sun.gross_assimilation_umol_co2_m2_leaf_s,
                conductance_m_s: detail.sun.conductance_m_s,
                ci_pa: detail.sun.ci_pa,
                vapor_kg_m2_tile_s: detail.sun.vapor_kg_m2_tile_s,
            },
            shade: M1LeafEvaluation {
                gas_branch: format!("{:?}", detail.shade.gas_branch),
                beta: detail.shade.beta,
                internal_specific_humidity_kg_kg: detail.shade.internal_specific_humidity_kg_kg,
                dark_respiration: detail.shade.dark_respiration_umol_co2_m2_leaf_s,
                net_assimilation: detail.shade.net_assimilation_umol_co2_m2_leaf_s,
                gross_assimilation: detail.shade.gross_assimilation_umol_co2_m2_leaf_s,
                conductance_m_s: detail.shade.conductance_m_s,
                ci_pa: detail.shade.ci_pa,
                vapor_kg_m2_tile_s: detail.shade.vapor_kg_m2_tile_s,
            },
            hydraulics: M1HydraulicEvaluation {
                potentials_mm: hydraulics.potentials_mm.to_vec(),
                root_layer_ids,
                root_fluxes_kg_m2_tile_s,
                stem_flux_kg_m2_tile_s: hydraulics.stem_flux_kg_m2_tile_s,
                total_leaf_demand_kg_m2_tile_s: detail.sun.vapor_kg_m2_tile_s
                    + detail.shade.vapor_kg_m2_tile_s,
                continuity_residuals_kg_m2_tile_s: hydraulics
                    .continuity_residuals_kg_m2_tile_s
                    .to_vec(),
                all_root_layer_ids,
                all_root_fluxes_kg_m2_tile_s,
            },
        });
        let base = 6 * index;
        let reservoir = M1PhaseReservoir::from_mass_enthalpy(
            trial.coordinates[base + 3],
            trial.coordinates[base + 4],
        )?;
        reservoirs.push(M1ReservoirOperands {
            beginning_mass_kg_m2: input.reservoirs[index].mass_kg_m2,
            beginning_enthalpy_j_m2: input.reservoirs[index].enthalpy_j_m2,
            ending_mass_kg_m2: trial.coordinates[base + 3],
            ending_enthalpy_j_m2: trial.coordinates[base + 4],
            dt_s: input.column.interval_s,
            incident_liquid_kg_m2_s: detail.incident_liquid,
            incident_enthalpy_j_kg: detail.incident_enthalpy,
            vapor_kg_m2_s: detail.wet_vapor,
            drainage_kg_m2_s: trial.coordinates[base + 5],
            non_vapor_heat_w_m2: detail.non_vapor,
            wet_temperature_k: detail.wet_temperature_k,
            diagnosed_liquid_mass_kg_m2: reservoir.liquid_mass_kg_m2(),
            liquid_capacity_kg_m2: detail.capacity,
            wet_fraction: detail.wet_fraction,
            emission_w_m2: detail.emission,
            wet_longwave_w_m2: detail.wet_longwave,
            wet_sensible_w_m2: detail.wet_sensible,
            mass_residual_kg_m2: detail.mass_residual,
            enthalpy_residual_j_m2: detail.enthalpy_residual,
        });
    }
    let accepted_result = result.clone();
    let evaluation = M1CoupledEvaluation {
        accepted_input: input.clone(),
        residuals: result.residuals,
        jacobian,
        occupancies: outputs,
        reservoirs,
        duration_s: input.column.interval_s,
        accepted_coordinates: trial.coordinates,
    };
    #[cfg(test)]
    let mut evaluation = evaluation;
    #[cfg(test)]
    match M1_MATERIALIZATION_OUTPUT_POISON.with(|poison| poison.replace(0)) {
        1 => evaluation.occupancies[0].hydraulics.potentials_mm[0] = f64::NAN,
        2 => {
            evaluation.occupancies[0]
                .hydraulics
                .root_fluxes_kg_m2_tile_s[0] = f64::NAN
        }
        3 => {
            evaluation.occupancies[0]
                .hydraulics
                .continuity_residuals_kg_m2_tile_s[0] = f64::NAN
        }
        4 => evaluation.occupancies[0].sun.vapor_kg_m2_tile_s = f64::NAN,
        5 => evaluation.reservoirs[0].wet_fraction = f64::NAN,
        6 => evaluation.reservoirs[0].wet_temperature_k = Some(f64::NAN),
        7 => evaluation.reservoirs[0].liquid_capacity_kg_m2 = f64::NAN,
        _ => {}
    }
    if evaluation.jacobian.is_none() {
        validate_accepted_materialization(input, &accepted_result, trial, &evaluation)?;
        #[cfg(test)]
        M1_ACCEPTED_CAPTURE.with(|capture| {
            if capture.borrow().is_none() {
                *capture.borrow_mut() = Some((input.clone(), trial.clone(), accepted_result));
            }
        });
    }
    Ok(evaluation)
}

pub fn evaluate_m1_coupled_column(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
) -> Result<M1CoupledEvaluation, M1CoupledError> {
    let (result, jacobian, _) = evaluate_core_with_jacobian(input, trial, None)?;
    materialize_m1_coupled_column(input, trial, result, Some(jacobian))
}

// Confined physical-adapter boundary for COLD-CANOPY-M1-TR-SVD-BVLS-01.  Its
// private feature is distinct from the analytic controller and full skeleton.
#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Debug)]
pub(crate) enum M1PhysicalTrustRegionAdapterOutcome {
    Accepted(M1CoupledEvaluation),
    Refused(crate::M1TrustRegionRefusalKind),
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) struct M1PhysicalTrustRegionAdapter {
    pub(crate) input: M1CoupledColumnInput,
    pub(crate) initial_trial: M1CoupledColumnTrial,
    observation: M1PhysicalTrustRegionAdapterObservation,
    natural_phase_cache: Option<([f64; 21], [M1PhaseJoinSelection; 2])>,
    materialized: Option<M1CoupledEvaluation>,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone)]
pub(crate) struct M1PhysicalTrustRegionPayload {
    core: Core,
    trial: M1CoupledColumnTrial,
    raw_jacobian: Option<Vec<Vec<f64>>>,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug, Default)]
pub(crate) struct M1PhysicalTrustRegionAdapterObservation {
    pub(crate) current_evaluations: u16,
    pub(crate) raw_jacobian_assemblies: u16,
    pub(crate) natural_phase_selections: u16,
    pub(crate) trial_domain_checks: u16,
    pub(crate) selected_side_probes: u16,
    pub(crate) materialization_entries: u16,
    pub(crate) current_hydraulics:
        crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics,
    pub(crate) current_coordinates: Option<[f64; 21]>,
    pub(crate) raw_residual: Option<[f64; 21]>,
    pub(crate) dynamic_normalizers: Option<[f64; 21]>,
    pub(crate) raw_jacobian: Option<[[f64; 21]; 21]>,
    /// Solve-owned capture is intentionally populated by the next lifecycle body.
    pub(crate) budget_snapshot: Option<crate::M1PhysicalBudgetSnapshot>,
    pub(crate) controller_run: Option<M1PhysicalTrustRegionControllerRunObservation>,
    pub(crate) phase_events: Vec<M1PhysicalTrustRegionPhaseObservation>,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1PhysicalTrustRegionPhaseObservation {
    pub(crate) coordinates: [f64; 21],
    pub(crate) direction: [f64; 21],
    pub(crate) selections: [M1PhaseJoinSelection; 2],
    pub(crate) natural_cache: bool,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
impl M1PhysicalTrustRegionAdapterObservation {
    pub(crate) fn face_pivot_phase_json_for_test(&self) -> String {
        use std::fmt::Write;

        let mut output = String::from("[");
        for (event_index, event) in self.phase_events.iter().enumerate() {
            if event_index != 0 {
                output.push(',');
            }
            let _ = write!(output, "{{\"coordinates\":[");
            for (index, value) in event.coordinates.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                let _ = write!(output, "\"{:016x}\"", value.to_bits());
            }
            output.push_str("],\"direction\":[");
            for (index, value) in event.direction.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                let _ = write!(output, "\"{:016x}\"", value.to_bits());
            }
            output.push_str("],\"selections\":[");
            for (index, selection) in event.selections.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                let _ = write!(
                    output,
                    "{{\"phase\":\"{:?}\",\"capacity\":\"{:?}\",\"corrected\":{},\"mass\":\"{:016x}\",\"enthalpy\":\"{:016x}\",\"capacity_value\":\"{:016x}\",\"drainage\":\"{:016x}\",\"duration_s\":\"{:016x}\"}}",
                    selection.phase,
                    selection.capacity,
                    selection.corrected,
                    selection.mass.to_bits(),
                    selection.enthalpy.to_bits(),
                    selection.capacity_value.to_bits(),
                    selection.drainage.to_bits(),
                    selection.duration_s.to_bits()
                );
            }
            let _ = write!(output, "],\"natural_cache\":{}}}", event.natural_cache);
        }
        output.push(']');
        output
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[test]
fn m1_face_pivot_populated_phase_json_parses() {
    let selection = M1PhaseJoinSelection {
        phase: openwepp_vegetation::cold_canopy_m1::M1Phase::Ice,
        capacity: openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Capacity,
        corrected: true,
        mass: -0.0,
        enthalpy: f64::INFINITY,
        capacity_value: f64::NAN,
        drainage: 1.0,
        duration_s: 2.0,
    };
    let observation = M1PhysicalTrustRegionAdapterObservation {
        phase_events: vec![M1PhysicalTrustRegionPhaseObservation {
            coordinates: [0.0; 21],
            direction: [-0.0; 21],
            selections: [selection; 2],
            natural_cache: false,
        }],
        ..Default::default()
    };
    serde_json::from_str::<serde_json::Value>(&observation.face_pivot_phase_json_for_test())
        .expect("bit-preserving populated phase JSON");
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct M1PhysicalTrustRegionControllerRunObservation {
    pub(crate) proposals: u16,
    pub(crate) accepted_updates: u8,
    pub(crate) installed_updates: u8,
    pub(crate) entered_materializations: u8,
    pub(crate) entered_svd_factorizations: u16,
    pub(crate) final_coordinates: [f64; 21],
    pub(crate) materialized_coordinates: Option<[f64; 21]>,
    pub(crate) terminal_refusal: Option<crate::M1TrustRegionRefusalKind>,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
impl M1PhysicalTrustRegionAdapter {
    pub(crate) fn new(input: M1CoupledColumnInput, initial_trial: M1CoupledColumnTrial) -> Self {
        Self {
            input,
            initial_trial,
            observation: M1PhysicalTrustRegionAdapterObservation::default(),
            natural_phase_cache: None,
            materialized: None,
        }
    }

    pub(crate) fn observation(&self) -> &M1PhysicalTrustRegionAdapterObservation {
        &self.observation
    }

    pub(crate) fn has_materialized_result_for_test(&self) -> bool {
        self.materialized.is_some()
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_trust_region_physical_adapter_initialize_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
) -> Result<
    (),
    crate::m1_trust_region_controller::M1TrustRegionControllerError<
        M1PhysicalTrustRegionAdapterOwnError,
    >,
> {
    match crate::m1_trust_region_controller::FoundationState::initialize(adapter) {
        Err(crate::m1_trust_region_controller::M1TrustRegionControllerError::Own(
            M1PhysicalTrustRegionAdapterOwnError::IncompleteScope,
        )) => Err(crate::m1_trust_region_controller::M1TrustRegionControllerError::IncompleteScope),
        Err(error) => Err(error),
        Ok(_) => Ok(()),
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn m1_trust_region_physical_adapter_finalize_run_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
    run: &crate::m1_trust_region_controller_interface::M1TrustRegionControllerRun,
) -> Result<
    M1PhysicalTrustRegionAdapterOutcome,
    crate::m1_trust_region_controller::M1TrustRegionControllerError<
        M1PhysicalTrustRegionAdapterOwnError,
    >,
> {
    if let Some(kind) = run.terminal_refusal {
        return Ok(M1PhysicalTrustRegionAdapterOutcome::Refused(kind));
    }
    adapter
        .materialized
        .take()
        .map(M1PhysicalTrustRegionAdapterOutcome::Accepted)
        .ok_or(crate::m1_trust_region_controller::M1TrustRegionControllerError::IncompleteScope)
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_trust_region_physical_adapter_terminal_outcome_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
    run: &crate::m1_trust_region_controller_interface::M1TrustRegionControllerRun,
) -> Result<
    M1PhysicalTrustRegionAdapterOutcome,
    crate::m1_trust_region_controller::M1TrustRegionControllerError<
        M1PhysicalTrustRegionAdapterOwnError,
    >,
> {
    m1_trust_region_physical_adapter_finalize_run_for_test(adapter, run)
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_trust_region_physical_adapter_run_outcome_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
) -> Result<
    M1PhysicalTrustRegionAdapterOutcome,
    crate::m1_trust_region_controller::M1TrustRegionControllerError<
        M1PhysicalTrustRegionAdapterOwnError,
    >,
> {
    let scope = crate::m1_physical_budget_scope_for_test();
    let result = (|| {
        let run =
            crate::m1_trust_region_controller::m1_trust_region_controller_run_for_test(adapter)?;
        adapter.observation.controller_run = Some(M1PhysicalTrustRegionControllerRunObservation {
            proposals: run.proposals,
            accepted_updates: run.accepted_updates,
            installed_updates: run
                .proposal_trace
                .iter()
                .filter(|proposal| proposal.installed)
                .count()
                .try_into()
                .map_err(|_| {
                    crate::m1_trust_region_controller::M1TrustRegionControllerError::IncompleteScope
                })?,
            entered_materializations: run.entered_materializations,
            entered_svd_factorizations: run.entered_svd_factorizations,
            final_coordinates: run.final_coordinates,
            materialized_coordinates: run.materialized_coordinates,
            terminal_refusal: run.terminal_refusal,
        });
        m1_trust_region_physical_adapter_finalize_run_for_test(adapter, &run)
    })();
    adapter.observation.budget_snapshot = crate::m1_physical_budget_observation_for_test();
    drop(scope);
    result
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_trust_region_physical_adapter_proposal_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
) -> Result<
    M1CoupledEvaluation,
    crate::m1_trust_region_controller::M1TrustRegionControllerError<
        M1PhysicalTrustRegionAdapterOwnError,
    >,
> {
    match m1_trust_region_physical_adapter_run_outcome_for_test(adapter)? {
        M1PhysicalTrustRegionAdapterOutcome::Accepted(result) => Ok(result),
        M1PhysicalTrustRegionAdapterOutcome::Refused(_) => {
            Err(crate::m1_trust_region_controller::M1TrustRegionControllerError::IncompleteScope)
        }
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_trust_region_physical_adapter_hydraulic_height_poison_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
) -> Result<(), M1PhysicalTrustRegionAdapterOwnError> {
    use crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluator;
    let mut current = adapter.evaluate(adapter.initial_coordinates())?;
    let height = adapter.input.column.occupancies[0].height_m;
    adapter.input.column.occupancies[0].height_m = f64::NAN;
    let result = adapter.complete_current_hydraulics(&mut current);
    adapter.input.column.occupancies[0].height_m = height;
    result
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_trust_region_physical_adapter_phase_disagreement_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
) -> Result<(), M1PhysicalTrustRegionAdapterOwnError> {
    use crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluator;
    let mut coordinates = adapter.initial_coordinates();
    for base in [0, 6] {
        coordinates[base + 3] = 0.018;
        coordinates[base + 4] = 0.0;
        coordinates[base + 5] = 0.018 / 60.0;
    }
    let current = adapter.evaluate(coordinates)?;
    let trial = &current.payload.trial;
    let mut cooling = [0.0; 21];
    let mut warming = [0.0; 21];
    for base in [0, 6] {
        cooling[base + 3] = -0.000_276_901_700_229_317_9;
        cooling[base + 4] = -783.510_921_253_580_5;
        warming[base + 4] = 1.0;
    }
    let selected = phase_join_selections(&adapter.input, trial, &cooling)
        .map_err(m1_physical_adapter_phase_failure)?;
    let final_selection = phase_join_selections(&adapter.input, trial, &warming)
        .map_err(m1_physical_adapter_phase_failure)?;
    if selected == final_selection {
        return Err(M1PhysicalTrustRegionAdapterOwnError::IncompleteScope);
    }
    Err(adapter.phase_active_set_inconsistent(&current, &selected, &final_selection))
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_trust_region_physical_adapter_candidate_raw_jacobian_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
    candidate_coordinates: [f64; 21],
) -> Result<[[f64; 21]; 21], M1PhysicalTrustRegionAdapterOwnError> {
    use crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluator;
    let _base = adapter.evaluate(adapter.initial_coordinates())?;
    let candidate = adapter.evaluate(candidate_coordinates)?;
    if !matches!(
        candidate.current_hydraulics,
        crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(_)
    ) {
        return Err(M1PhysicalTrustRegionAdapterOwnError::IncompleteScope);
    }
    adapter.raw_jacobian(&candidate, None, [0.0; 21])
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_trust_region_physical_adapter_over_capacity_materialize_for_test(
    adapter: &mut M1PhysicalTrustRegionAdapter,
) -> Result<(), M1PhysicalTrustRegionAdapterOwnError> {
    use crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluator;
    let mut current = adapter.evaluate(adapter.initial_coordinates())?;
    m1_apply_over_capacity_one_ulp_poison(&mut current.payload.core, &current.payload.trial)
        .map_err(M1CoupledError::from)?;
    adapter.materialize(&current).map(|_| ())
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) enum M1PhysicalTrustRegionAdapterOwnError {
    Coupled(Box<M1CoupledError>),
    IncompleteScope,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
impl std::fmt::Debug for M1PhysicalTrustRegionAdapterOwnError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coupled(error) => formatter.debug_tuple("Coupled").field(error).finish(),
            Self::IncompleteScope => formatter.write_str("IncompleteScope"),
        }
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
impl From<M1CoupledError> for M1PhysicalTrustRegionAdapterOwnError {
    fn from(error: M1CoupledError) -> Self {
        Self::Coupled(Box::new(error))
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn m1_physical_adapter_phase_failure(
    failure: M1PhaseJoinFailure,
) -> M1PhysicalTrustRegionAdapterOwnError {
    match failure {
        M1PhaseJoinFailure::Source { error, .. } => {
            M1PhysicalTrustRegionAdapterOwnError::Coupled(error)
        }
        M1PhaseJoinFailure::InconsistentFinalSet { .. }
        | M1PhaseJoinFailure::UnrepresentableProbe { .. } => {
            M1PhysicalTrustRegionAdapterOwnError::Coupled(Box::new(domain_error()))
        }
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
impl crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluator
    for M1PhysicalTrustRegionAdapter
{
    type Error = M1PhysicalTrustRegionAdapterOwnError;
    type Payload = M1PhysicalTrustRegionPayload;
    type Phase = [M1PhaseJoinSelection; 2];

    fn initial_coordinates(&self) -> [f64; 21] {
        self.initial_trial.coordinates
    }

    fn coordinate_scales(&self) -> [f64; 21] {
        let mut scales = [1.0; 21];
        for coordinate in [3, 9] {
            scales[coordinate] = 1.0e-2;
        }
        for coordinate in [4, 10] {
            scales[coordinate] = 1.0e3;
        }
        for coordinate in [5, 11] {
            scales[coordinate] = 1.0e-5;
        }
        scales[13] = 1.0e-3;
        scales
    }

    fn coordinate_bounds(&self) -> [[f64; 21]; 2] {
        let mut lower = [f64::NEG_INFINITY; 21];
        let mut upper = [f64::INFINITY; 21];
        for coordinate in [0, 1, 2, 6, 7, 8] {
            lower[coordinate] = 263.15;
            upper[coordinate] = 373.15;
        }
        for coordinate in [5, 11] {
            lower[coordinate] = 0.0;
        }
        lower[12] = 258.15;
        upper[12] = 373.15;
        lower[13] = 0.0;
        upper[13] = 0.1;
        [lower, upper]
    }

    fn evaluate(
        &mut self,
        coordinates: [f64; 21],
    ) -> Result<
        crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation<
            Self::Payload,
        >,
        Self::Error,
    > {
        let trial = M1CoupledColumnTrial::from_coordinates(coordinates);
        let is_base = self.observation.current_evaluations == 0;
        if is_base {
            let natural_phase = phase_join_selections(&self.input, &trial, &[0.0; 21])
                .map_err(m1_physical_adapter_phase_failure)?;
            self.observation.natural_phase_selections += 1;
            self.natural_phase_cache = Some((coordinates, natural_phase));
        }
        let (core, raw_jacobian) = if is_base {
            let (core, raw_jacobian, _) = evaluate_core_with_jacobian(&self.input, &trial, None)
                .map_err(Self::Error::from)?;
            (core, Some(raw_jacobian))
        } else {
            (core(&self.input, &trial).map_err(Self::Error::from)?, None)
        };
        let raw = raw_jacobian.as_ref().map(|matrix| {
            let mut raw = [[0.0; 21]; 21];
            for (row, source) in raw.iter_mut().zip(matrix) {
                row.copy_from_slice(source);
            }
            raw
        });
        let mut raw_residual = [0.0; 21];
        raw_residual.copy_from_slice(&core.residuals);
        let normalizers = m1_tolerances(&self.input, &core);
        self.observation.current_evaluations += 1;
        if is_base {
            self.observation.raw_jacobian_assemblies += 1;
        }
        let current_hydraulics = if is_base {
            crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Pending
        } else {
            let values = hydraulic_potentials(&self.input, &core).map_err(Self::Error::from)?;
            crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(
                values
                    .try_into()
                    .map_err(|_| M1PhysicalTrustRegionAdapterOwnError::IncompleteScope)?,
            )
        };
        self.observation.current_hydraulics = current_hydraulics;
        self.observation.current_coordinates = Some(coordinates);
        self.observation.raw_residual = Some(raw_residual);
        self.observation.dynamic_normalizers = Some(normalizers);
        if let Some(raw) = raw {
            self.observation.raw_jacobian = Some(raw);
        }
        Ok(
            crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation {
                coordinates,
                raw_residual,
                dynamic_normalizers: normalizers,
                wet_temperature_k: [
                    core.occupancies[0].wet_temperature_k,
                    core.occupancies[1].wet_temperature_k,
                ],
                current_hydraulics,
                beta: [
                    core.occupancies[0].sun.beta,
                    core.occupancies[0].shade.beta,
                    core.occupancies[1].sun.beta,
                    core.occupancies[1].shade.beta,
                ],
                ci_pa: [
                    core.occupancies[0].sun.ci_pa,
                    core.occupancies[0].shade.ci_pa,
                    core.occupancies[1].sun.ci_pa,
                    core.occupancies[1].shade.ci_pa,
                ],
                payload: M1PhysicalTrustRegionPayload {
                    core,
                    trial,
                    raw_jacobian,
                },
            },
        )
    }

    fn pre_evaluator_trial_domain(&mut self, coordinates: [f64; 21]) -> Result<bool, Self::Error> {
        let trial = M1CoupledColumnTrial::from_coordinates(coordinates);
        self.observation.trial_domain_checks += 1;
        Ok(fd_trial_is_admissible(&trial))
    }

    fn raw_jacobian(
        &mut self,
        current: &crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation<
            Self::Payload,
        >,
        selected_phase: Option<&Self::Phase>,
        predictor_direction: [f64; 21],
    ) -> Result<[[f64; 21]; 21], Self::Error> {
        if selected_phase.is_none() && predictor_direction == [0.0; 21] {
            if let Some(cached) = &current.payload.raw_jacobian {
                let mut raw = [[0.0; 21]; 21];
                for (row, source) in raw.iter_mut().zip(cached) {
                    row.copy_from_slice(source);
                }
                return Ok(raw);
            }
        }
        let (_, raw_jacobian, _) = evaluate_core_with_jacobian(
            &self.input,
            &current.payload.trial,
            selected_phase.map(|phase| (phase, &predictor_direction, 0, 0)),
        )
        .map_err(Self::Error::from)?;
        let mut raw = [[0.0; 21]; 21];
        for (row, source) in raw.iter_mut().zip(&raw_jacobian) {
            row.copy_from_slice(source);
        }
        self.observation.raw_jacobian_assemblies += 1;
        self.observation.raw_jacobian = Some(raw);
        Ok(raw)
    }

    fn complete_current_hydraulics(
        &mut self,
        current: &mut crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation<
            Self::Payload,
        >,
    ) -> Result<(), Self::Error> {
        if matches!(
            current.current_hydraulics,
            crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Pending
        ) {
            let values = hydraulic_potentials(&self.input, &current.payload.core)
                .map_err(Self::Error::from)?;
            let hydraulics: [f64; 8] = values
                .try_into()
                .map_err(|_| M1PhysicalTrustRegionAdapterOwnError::IncompleteScope)?;
            current.current_hydraulics =
                crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(hydraulics);
            self.observation.current_hydraulics = current.current_hydraulics;
        }
        Ok(())
    }

    fn reset_current_hydraulics_after_nonroot_post_update(
        &mut self,
        current: &mut crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation<
            Self::Payload,
        >,
    ) {
        current.current_hydraulics =
            crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Pending;
        self.observation.current_hydraulics = current.current_hydraulics;
    }

    fn select_phase(
        &mut self,
        current: &crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation<
            Self::Payload,
        >,
        physical_direction: [f64; 21],
    ) -> Result<Self::Phase, Self::Error> {
        if physical_direction == [0.0; 21]
            && let Some((coordinates, phase)) = self.natural_phase_cache
            && coordinates == current.coordinates
        {
            if crate::m1_face_pivot_capture_enabled_for_test() {
                self.observation
                    .phase_events
                    .push(M1PhysicalTrustRegionPhaseObservation {
                        coordinates: current.coordinates,
                        direction: physical_direction,
                        selections: phase,
                        natural_cache: true,
                    });
            }
            return Ok(phase);
        }
        let phase = phase_join_selections(&self.input, &current.payload.trial, &physical_direction)
            .map_err(m1_physical_adapter_phase_failure)?;
        if crate::m1_face_pivot_capture_enabled_for_test() {
            self.observation
                .phase_events
                .push(M1PhysicalTrustRegionPhaseObservation {
                    coordinates: current.coordinates,
                    direction: physical_direction,
                    selections: phase,
                    natural_cache: false,
                });
        }
        Ok(phase)
    }

    fn phase_active_set_inconsistent(
        &mut self,
        current: &crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation<
            Self::Payload,
        >,
        _selected_phase: &Self::Phase,
        _final_phase: &Self::Phase,
    ) -> Self::Error {
        m1_failure(
            crate::numerics::NumericalFailureKind::PhaseActiveSetInconsistent,
            0,
            &self.input,
            &current.payload.core,
            &current.payload.trial,
            0,
            StepNorms {
                temperature_k: None,
                humidity_kg_kg: None,
                ci_pa: None,
                hydraulic_mm: None,
                beta: None,
            },
            None,
            None,
        )
        .into()
    }

    fn selected_side_probe(
        &mut self,
        current: &crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation<
            Self::Payload,
        >,
        phase: &Self::Phase,
        direction: [f64; 21],
    ) -> Result<(), Self::Error> {
        m1_selected_probe(
            &self.input,
            &current.payload.trial,
            phase,
            &direction,
            &current.payload.core,
            0,
            0,
        )
        .map_err(Self::Error::from)?;
        self.observation.selected_side_probes += 1;
        Ok(())
    }

    fn materialize(
        &mut self,
        current: &crate::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluation<
            Self::Payload,
        >,
    ) -> Result<Self::Payload, Self::Error> {
        let materialized = materialize_m1_coupled_column(
            &self.input,
            &current.payload.trial,
            current.payload.core.clone(),
            None,
        )
        .map_err(Self::Error::from)?;
        self.observation.materialization_entries += 1;
        self.materialized = Some(materialized);
        Ok(current.payload.clone())
    }

}

pub fn m1_phase_join_primal_normalizers(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
) -> Result<Vec<f64>, M1CoupledError> {
    let base = core(input, trial)?;
    Ok(m1_tolerances(input, &base).to_vec())
}

#[derive(Clone)]
pub struct M1PhaseJoinControllerReport {
    base: Core,
    raw_jacobian: Vec<Vec<f64>>,
    normalized_jacobian: Vec<Vec<f64>>,
    #[cfg(test)]
    rhs: Vec<f64>,
    phases: [openwepp_vegetation::cold_canopy_m1::M1Phase; 2],
    #[cfg(test)]
    capacity: [openwepp_vegetation::cold_canopy_m1::M1CapacityBranch; 2],
    #[cfg(test)]
    current_drainage: [f64; 2],
    #[cfg(any(test, feature = "test-support"))]
    natural_selections: [M1PhaseJoinSelection; 2],
    #[cfg(any(test, feature = "test-support"))]
    predictor_selections: [M1PhaseJoinSelection; 2],
    #[cfg(any(test, feature = "test-support"))]
    final_selections: [M1PhaseJoinSelection; 2],
    #[cfg(any(test, feature = "test-support"))]
    corrected_assembly: bool,
    eliminated_coordinates: [bool; 2],
    linear_dimension: usize,
    direction: [f64; 21],
    pivot: f64,
    matrix_norm: f64,
    activity: M1PhaseJoinActivity,
}
impl std::fmt::Debug for M1PhaseJoinControllerReport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M1PhaseJoinControllerReport")
            .field("phases", &self.phases)
            .field("direction", &self.direction)
            .field("activity", &self.activity)
            .finish_non_exhaustive()
    }
}
impl M1PhaseJoinControllerReport {
    #[must_use]
    pub fn selected_raw_jacobian_times(&self, direction: &[f64; 21]) -> Vec<f64> {
        self.raw_jacobian
            .iter()
            .map(|row| row.iter().zip(direction).map(|(a, b)| a * b).sum())
            .collect()
    }
    #[must_use]
    pub fn selected_normalized_jacobian_times(&self, direction: &[f64; 21]) -> Vec<f64> {
        self.normalized_jacobian
            .iter()
            .map(|row| row.iter().zip(direction).map(|(a, b)| a * b).sum())
            .collect()
    }
    /// Residual of the exact normalized system handed to the controller's LU.
    #[cfg(test)]
    #[must_use]
    pub fn assembled_system_residual(&self) -> Vec<f64> {
        self.normalized_jacobian
            .iter()
            .zip(&self.rhs)
            .map(|(row, rhs)| {
                row.iter()
                    .zip(&self.direction)
                    .map(|(a, d)| a * d)
                    .sum::<f64>()
                    - rhs
            })
            .collect()
    }
    /// Residual of the source (unnormalized) physical row system.
    #[cfg(test)]
    #[must_use]
    pub fn original_system_residual(&self) -> Vec<f64> {
        self.raw_jacobian
            .iter()
            .zip(&self.base.residuals)
            .map(|(row, residual)| {
                row.iter()
                    .zip(&self.direction)
                    .map(|(a, d)| a * d)
                    .sum::<f64>()
                    + residual
            })
            .collect()
    }
    /// Source-defined selected-drainage affine residuals, one per occupancy.
    #[cfg(test)]
    #[must_use]
    pub fn source_affine_residuals(&self) -> [Option<f64>; 2] {
        std::array::from_fn(|occupancy| {
            (self.capacity[occupancy]
                == openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Drainage)
                .then(|| self.direction[occupancy * 6 + 5] + self.current_drainage[occupancy])
        })
    }
    #[must_use]
    pub const fn selected_phase(
        &self,
        occupancy: usize,
    ) -> openwepp_vegetation::cold_canopy_m1::M1Phase {
        self.phases[occupancy]
    }
    #[cfg(test)]
    #[must_use]
    pub const fn selected_capacity(
        &self,
        occupancy: usize,
    ) -> openwepp_vegetation::cold_canopy_m1::M1CapacityBranch {
        self.capacity[occupancy]
    }
    #[cfg(test)]
    #[must_use]
    pub const fn eliminated_coordinates(&self) -> [bool; 2] {
        self.eliminated_coordinates
    }
    #[cfg(test)]
    #[must_use]
    pub const fn linear_dimension(&self) -> usize {
        self.linear_dimension
    }
    #[must_use]
    pub const fn work_counts(&self) -> M1PhaseJoinActivity {
        self.activity
    }
    #[must_use]
    pub const fn direction(&self) -> [f64; 21] {
        self.direction
    }
}

fn m1_candidate_coordinate(
    current: f64,
    change: f64,
    factor: f64,
    eliminated_drainage: bool,
) -> Result<f64, ()> {
    if eliminated_drainage {
        let candidate = (1.0 - factor) * current;
        if !current.is_finite() || current < 0.0 || !candidate.is_finite() || candidate < 0.0 {
            return Err(());
        }
        Ok(candidate)
    } else {
        Ok(current + factor * change)
    }
}

fn phase_join_selections(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    direction: &[f64; 21],
) -> Result<[M1PhaseJoinSelection; 2], M1PhaseJoinFailure> {
    #[cfg(test)]
    M1_REFINEMENT_PHASE_JOIN_SELECTIONS.with(|count| count.set(count.get().saturating_add(1)));
    let reservoir = |base| {
        M1PhaseReservoir::from_mass_enthalpy(
            trial.coordinates[base + 3],
            trial.coordinates[base + 4],
        )
        .map_err(|error| M1PhaseJoinFailure::Source {
            error: Box::new(error.into()),
            activity: M1PhaseJoinActivity::default(),
        })
    };
    let selection = |base: usize| {
        let occupancy = &input.column.occupancies[base / 6];
        classify_m1_phase_join_active_set(
            reservoir(base)?,
            occupancy.liquid_capacity_kg_m2_plant * (occupancy.lai + occupancy.sai),
            trial.coordinates[base + 5],
            input.column.interval_s,
            direction[base + 3],
            direction[base + 4],
            direction[base + 5],
        )
    };
    Ok([selection(0)?, selection(6)?])
}

#[derive(Clone, Copy)]
struct M1DrainageEliminationRow {
    row: usize,
    identity: usize,
    support: f64,
    duration: f64,
    scale: f64,
    current: f64,
    raw_residual: f64,
    raw_diagonal: f64,
}

enum M1DrainageLinearFailure {
    Integrity,
    Singular { pivot: f64, norm: f64 },
    NonfiniteResult,
}

/// Shared selected-affine reduction used by the controller and its test
/// adapter. It contains only assembled linear-system data, so test calls do
/// not recreate physical state or a second reduction implementation.
fn solve_selected_drainage_linear(
    matrix: &[Vec<f64>],
    rhs: &[f64],
    raw: &[Vec<f64>],
    rows: &[M1DrainageEliminationRow],
    iteration: u32,
    corrected_assembly: bool,
    activity: &mut M1PhaseJoinActivity,
) -> Result<(Vec<f64>, f64, f64, [bool; 2], usize), M1DrainageLinearFailure> {
    if matrix.len() != rhs.len()
        || raw.len() != rhs.len()
        || matrix.iter().any(|row| row.len() != rhs.len())
        || raw.iter().any(|row| row.len() != rhs.len())
    {
        return Err(M1DrainageLinearFailure::Integrity);
    }
    if rows.len() > 2
        || rows.iter().any(|row| !matches!(row.row, 5 | 11))
        || rows.windows(2).any(|pair| pair[0].row >= pair[1].row)
    {
        return Err(M1DrainageLinearFailure::Integrity);
    }
    if rows.is_empty() {
        M1_RUNTIME_ACTIVITY.with(|activity| {
            m1_record_linear_dimension(
                &mut activity.borrow_mut().full_linear_dimension_counts,
                matrix.len(),
            );
        });
        activity.linear_solves += 1;
        let (direction, pivot, norm) =
            crate::numerics::solve_linear(matrix, rhs).map_err(|error| {
                M1DrainageLinearFailure::Singular {
                    pivot: error.pivot,
                    norm: error.matrix_norm,
                }
            })?;
        if direction.iter().any(|value| !value.is_finite()) {
            return Err(M1DrainageLinearFailure::NonfiniteResult);
        }
        #[cfg(any(test, feature = "test-support"))]
        if m1_diagnostic_capture_active() {
            let retained_indices = (0..rhs.len()).collect::<Vec<_>>();
            m1_diagnostic_linear_solve(
                iteration,
                corrected_assembly,
                matrix,
                rhs,
                matrix,
                rhs,
                &retained_indices,
                &[],
                &direction,
                &direction,
            );
        }
        #[cfg(test)]
        eprintln!(
            "m1-drainage-assembly {}",
            serde_json::json!({
                "normalized_matrix": matrix, "rhs": rhs, "raw_matrix": raw,
                "direction": direction,
                "normalized_a_d_minus_b": matrix.iter().zip(rhs).map(|(row, value)| row.iter().zip(&direction).map(|(a, d)| a * d).sum::<f64>() - value).collect::<Vec<_>>(),
                "source_affine_d_plus_current": Vec::<(usize, f64)>::new(),
                "current_drainage": Vec::<(usize, f64)>::new(),
                "eliminated": [false, false], "retained_dimension": rhs.len(),
                "pivot": pivot, "reduced_norm": norm,
            })
        );
        return Ok((direction, pivot, norm, [false; 2], rhs.len()));
    }
    let mut eliminated = Vec::with_capacity(rows.len());
    for metadata in rows {
        let row = metadata.row;
        if row >= rhs.len()
            || metadata.identity != row
            || !metadata.support.is_finite()
            || metadata.support <= 0.0
            || !metadata.duration.is_finite()
            || metadata.duration <= 0.0
            || !metadata.scale.is_finite()
            || metadata.scale <= 0.0
            || !metadata.current.is_finite()
            || metadata.current < 0.0
            || !metadata.raw_residual.is_finite()
            || !metadata.raw_diagonal.is_finite()
            || !rhs[row].is_finite()
            || metadata.raw_diagonal != metadata.duration
            || metadata.raw_residual != metadata.duration * metadata.current
            || raw[row].iter().any(|value| !value.is_finite())
            || raw[row][row] != metadata.raw_diagonal
            || raw[row]
                .iter()
                .enumerate()
                .any(|(column, value)| column != row && *value != 0.0)
            || matrix[row][row] != metadata.duration / metadata.scale
            || matrix[row].iter().any(|value| !value.is_finite())
            || rhs[row] != -(metadata.raw_residual / metadata.scale)
            || matrix[row]
                .iter()
                .enumerate()
                .any(|(column, value)| column != row && *value != 0.0)
        {
            return Err(M1DrainageLinearFailure::Integrity);
        }
        eliminated.push(row);
    }
    let retained: Vec<_> = (0..rhs.len())
        .filter(|index| !eliminated.contains(index))
        .collect();
    if !rows.is_empty()
        && retained.iter().any(|&row| {
            !rhs[row].is_finite()
                || retained
                    .iter()
                    .any(|&column| !matrix[row][column].is_finite())
                || eliminated
                    .iter()
                    .any(|&column| !matrix[row][column].is_finite())
        })
    {
        return Err(M1DrainageLinearFailure::NonfiniteResult);
    }
    let full_norm = matrix
        .iter()
        .map(|row| row.iter().map(|value| value.abs()).sum::<f64>())
        .fold(0.0, f64::max);
    if !rows.is_empty() && !full_norm.is_finite() {
        return Err(M1DrainageLinearFailure::NonfiniteResult);
    }
    if !rows.is_empty()
        && rows.iter().any(|metadata| {
            matrix[metadata.row][metadata.row].abs() < 64.0 * f64::EPSILON * full_norm
        })
    {
        return Err(M1DrainageLinearFailure::Integrity);
    }
    let mut reduced = vec![vec![0.0; retained.len()]; retained.len()];
    let mut reduced_rhs = vec![0.0; retained.len()];
    for (reduced_row, &row) in retained.iter().enumerate() {
        reduced_rhs[reduced_row] = rhs[row];
        for metadata in rows {
            reduced_rhs[reduced_row] -= matrix[row][metadata.row] * -metadata.current;
        }
        for (reduced_column, &column) in retained.iter().enumerate() {
            reduced[reduced_row][reduced_column] = matrix[row][column];
        }
    }
    if reduced_rhs.iter().any(|value| !value.is_finite()) {
        return Err(M1DrainageLinearFailure::NonfiniteResult);
    }
    M1_RUNTIME_ACTIVITY.with(|activity| {
        m1_record_linear_dimension(
            &mut activity.borrow_mut().reduced_linear_dimension_counts,
            reduced.len(),
        );
    });
    activity.linear_solves += 1;
    let (solution, pivot, norm) =
        crate::numerics::solve_linear(&reduced, &reduced_rhs).map_err(|error| {
            M1DrainageLinearFailure::Singular {
                pivot: error.pivot,
                norm: error.matrix_norm,
            }
        })?;
    if solution.iter().any(|value| !value.is_finite()) {
        return Err(M1DrainageLinearFailure::NonfiniteResult);
    }
    let mut direction = vec![0.0; rhs.len()];
    let retained_dimension = retained.len();
    for (&index, &value) in retained.iter().zip(&solution) {
        direction[index] = value;
    }
    for metadata in rows {
        direction[metadata.row] = -metadata.current;
    }
    if direction.iter().any(|value| !value.is_finite()) {
        return Err(M1DrainageLinearFailure::NonfiniteResult);
    }
    #[cfg(any(test, feature = "test-support"))]
    if m1_diagnostic_capture_active() {
        m1_diagnostic_linear_solve(
            iteration,
            corrected_assembly,
            matrix,
            rhs,
            &reduced,
            &reduced_rhs,
            &retained,
            &eliminated,
            &solution,
            &direction,
        );
    }
    let eliminated_coordinates = [eliminated.contains(&5), eliminated.contains(&11)];
    #[cfg(test)]
    eprintln!(
        "m1-drainage-assembly {}",
        serde_json::json!({
            "normalized_matrix": matrix,
            "rhs": rhs,
            "raw_matrix": raw,
            "direction": direction,
            "normalized_a_d_minus_b": matrix.iter().zip(rhs).map(|(row, value)| row.iter().zip(&direction).map(|(a, d)| a * d).sum::<f64>() - value).collect::<Vec<_>>(),
            "source_affine_d_plus_current": rows.iter().map(|row| (row.row, direction[row.row] + row.current)).collect::<Vec<_>>(),
            "current_drainage": rows.iter().map(|row| (row.row, row.current)).collect::<Vec<_>>(),
            "eliminated": eliminated_coordinates,
            "retained_dimension": retained_dimension,
            "pivot": pivot,
            "reduced_norm": norm,
        })
    );
    Ok((
        direction,
        pivot,
        norm,
        eliminated_coordinates,
        retained_dimension,
    ))
}

fn solve_selected_drainage_system(
    matrix: &[Vec<f64>],
    rhs: &[f64],
    raw: &[Vec<f64>],
    base: &Core,
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    selections: &[M1PhaseJoinSelection; 2],
    iteration: u32,
    backtracking_count: u32,
    corrected_assembly: bool,
    activity: &mut M1PhaseJoinActivity,
) -> Result<([f64; 21], f64, f64, [bool; 2], usize), M1CoupledError> {
    use openwepp_vegetation::cold_canopy_m1::M1CapacityBranch;
    let selected = [
        selections[0].capacity == M1CapacityBranch::Drainage,
        selections[1].capacity == M1CapacityBranch::Drainage,
    ];
    let map_failure = |failure| match failure {
        M1DrainageLinearFailure::Integrity => m1_failure(
            crate::numerics::NumericalFailureKind::DrainageEliminationIntegrity,
            iteration,
            input,
            base,
            trial,
            backtracking_count,
            StepNorms {
                temperature_k: None,
                humidity_kg_kg: None,
                ci_pa: None,
                hydraulic_mm: None,
                beta: Some(0.0),
            },
            None,
            None,
        ),
        M1DrainageLinearFailure::Singular { pivot, norm } => m1_failure(
            crate::numerics::NumericalFailureKind::SingularPivot,
            iteration,
            input,
            base,
            trial,
            backtracking_count,
            StepNorms {
                temperature_k: None,
                humidity_kg_kg: None,
                ci_pa: None,
                hydraulic_mm: None,
                beta: Some(0.0),
            },
            Some(pivot),
            Some(norm),
        ),
        M1DrainageLinearFailure::NonfiniteResult => {
            LandSurfaceEnergyError::NonFinite("m1_reduced_linear_result").into()
        }
    };
    if raw.len() != 21 || raw.iter().any(|row| row.len() != 21) {
        return Err(map_failure(M1DrainageLinearFailure::Integrity));
    }
    let tolerances = m1_tolerances(input, base);
    let mut rows = Vec::with_capacity(2);
    for (slot, active) in selected.iter().enumerate() {
        if !active {
            continue;
        }
        let row = slot * 6 + 5;
        if raw[row]
            .iter()
            .enumerate()
            .any(|(column, value)| column != row && *value != 0.0)
        {
            return Err(map_failure(M1DrainageLinearFailure::Integrity));
        }
        rows.push(M1DrainageEliminationRow {
            row,
            identity: row,
            support: input.column.interval_s,
            duration: input.column.interval_s,
            scale: tolerances[row],
            current: trial.coordinates[row],
            raw_residual: base.residuals[row],
            raw_diagonal: raw[row][row],
        });
    }
    let (direction, pivot, norm, eliminated, dimension) = solve_selected_drainage_linear(
        matrix,
        rhs,
        raw,
        &rows,
        iteration,
        corrected_assembly,
        activity,
    )
    .map_err(map_failure)?;
    let mut direction_array = [0.0; 21];
    direction_array.copy_from_slice(&direction);
    Ok((direction_array, pivot, norm, eliminated, dimension))
}

struct M1PhaseJoinControllerError {
    error: M1CoupledError,
    activity: M1PhaseJoinActivity,
}
impl M1PhaseJoinControllerError {
    fn into_source(self) -> M1CoupledError {
        M1_RUNTIME_ACTIVITY.with(|observation| {
            observation.borrow_mut().controller_failure_activity = Some(self.activity);
        });
        #[cfg(test)]
        M1_LAST_CONTROLLER_FAILURE_ACTIVITY.with(|observation| {
            *observation.borrow_mut() = Some(self.activity);
        });
        self.error
    }
}

fn m1_consume_first_domain_valid_halved_trial(
    exponent: u32,
    first_domain_valid_seen: &mut bool,
) -> bool {
    if exponent == 0 || *first_domain_valid_seen {
        return false;
    }
    *first_domain_valid_seen = true;
    true
}

#[cfg(test)]
pub fn m1_controller_failure_activity_for_test(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    predictor: [f64; 21],
    poison: M1SelectedAffineAssemblyPoison,
) -> M1CoupledError {
    M1_LAST_CONTROLLER_FAILURE_ACTIVITY.with(|observation| *observation.borrow_mut() = None);
    m1_phase_join_controller(input, trial, 0, 0, Some(predictor), None, Some(poison))
        .expect_err("test-only selected assembly poison must fail")
        .into_source()
}

#[cfg(test)]
pub fn m1_last_controller_failure_activity_for_test() -> Option<M1PhaseJoinActivity> {
    M1_LAST_CONTROLLER_FAILURE_ACTIVITY.with(|observation| *observation.borrow())
}

fn m1_phase_join_controller(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    iteration: u32,
    backtracking_count: u32,
    forced_predictor: Option<[f64; 21]>,
    forced_final: Option<[f64; 21]>,
    #[cfg(test)] selected_assembly_poison: Option<M1SelectedAffineAssemblyPoison>,
) -> Result<M1PhaseJoinControllerReport, M1PhaseJoinControllerError> {
    let mut activity = M1PhaseJoinActivity::default();
    let natural_direction = [0.0; 21];
    let natural_selections =
        phase_join_selections(input, trial, &natural_direction).map_err(|failure| {
            M1PhaseJoinControllerError {
                error: match failure {
                    M1PhaseJoinFailure::Source { error, .. } => *error,
                    _ => domain_error(),
                },
                activity,
            }
        })?;
    activity.assemblies += 1;
    M1_RUNTIME_ACTIVITY.with(|activity| activity.borrow_mut().predictor_assemblies += 1);
    #[cfg(test)]
    M1_CONTROLLER_ASSEMBLIES.with(|count| count.set(count.get() + 1));
    let (mut base, mut raw_jacobian, mut normalized_jacobian) =
        evaluate_core_with_jacobian(input, trial, None)
            .map_err(|error| M1PhaseJoinControllerError { error, activity })?;
    let numerical = |kind, pivot, matrix_norm, failure_base: &Core| {
        m1_failure(
            kind,
            iteration,
            input,
            failure_base,
            trial,
            backtracking_count,
            StepNorms {
                temperature_k: None,
                humidity_kg_kg: None,
                ci_pa: None,
                hydraulic_mm: None,
                beta: Some(0.0),
            },
            pivot,
            matrix_norm,
        )
    };
    let selection_base = base.clone();
    let selection_error = |failure| match failure {
        M1PhaseJoinFailure::Source { error, .. } => *error,
        _ => numerical(
            crate::numerics::NumericalFailureKind::PhaseActiveSetUnrepresentable,
            None,
            None,
            &selection_base,
        ),
    };
    let mut rhs = m1_normalized_linear_rhs(&base, input);
    let (initial_direction, mut pivot, mut matrix_norm, initial_eliminated, initial_dimension) =
        solve_selected_drainage_system(
            &normalized_jacobian,
            &rhs,
            &raw_jacobian,
            &base,
            input,
            trial,
            &natural_selections,
            iteration,
            backtracking_count,
            false,
            &mut activity,
        )
        .map_err(|error| M1PhaseJoinControllerError { error, activity })?;
    let mut eliminated_coordinates = initial_eliminated;
    let mut linear_dimension = initial_dimension;
    let predictor = forced_predictor.unwrap_or(initial_direction);
    let selections = phase_join_selections(input, trial, &predictor).map_err(|failure| {
        M1PhaseJoinControllerError {
            error: selection_error(failure),
            activity,
        }
    })?;
    let changed = selections != natural_selections;
    let direction = if changed {
        // Count the invocation even when a probe inside this assembly refuses.
        activity.assemblies += 1;
        M1_RUNTIME_ACTIVITY.with(|activity| activity.borrow_mut().final_assemblies += 1);
        #[cfg(test)]
        M1_CONTROLLER_ASSEMBLIES.with(|count| count.set(count.get() + 1));
        let (selected_base, raw, normalized_matrix) = evaluate_core_with_jacobian(
            input,
            trial,
            Some((&selections, &predictor, iteration, backtracking_count)),
        )
        .map_err(|error| M1PhaseJoinControllerError { error, activity })?;
        base = selected_base;
        raw_jacobian = raw;
        normalized_jacobian = normalized_matrix;
        rhs = m1_normalized_linear_rhs(&base, input);
        #[cfg(test)]
        if let Some(poison) = selected_assembly_poison {
            raw_jacobian[poison.row][poison.column] = poison.value;
        }
        let (
            selected_direction,
            selected_pivot,
            selected_norm,
            selected_eliminated,
            selected_dimension,
        ) = solve_selected_drainage_system(
            &normalized_jacobian,
            &rhs,
            &raw_jacobian,
            &base,
            input,
            trial,
            &selections,
            iteration,
            backtracking_count,
            true,
            &mut activity,
        )
        .map_err(|error| M1PhaseJoinControllerError { error, activity })?;
        pivot = selected_pivot;
        matrix_norm = selected_norm;
        eliminated_coordinates = selected_eliminated;
        linear_dimension = selected_dimension;
        selected_direction
    } else {
        predictor
    };
    let final_direction = forced_final.unwrap_or(direction);
    let final_selections =
        phase_join_selections(input, trial, &final_direction).map_err(|failure| {
            M1PhaseJoinControllerError {
                error: selection_error(failure),
                activity,
            }
        })?;
    if final_selections != selections {
        return Err(M1PhaseJoinControllerError {
            error: numerical(
                crate::numerics::NumericalFailureKind::PhaseActiveSetInconsistent,
                Some(pivot),
                Some(matrix_norm),
                &base,
            ),
            activity,
        });
    }
    m1_selected_probe(
        input,
        trial,
        &selections,
        &final_direction,
        &base,
        iteration,
        backtracking_count,
    )
    .map_err(|error| M1PhaseJoinControllerError { error, activity })?;
    Ok(M1PhaseJoinControllerReport {
        base,
        raw_jacobian,
        normalized_jacobian,
        #[cfg(test)]
        rhs,
        phases: [selections[0].phase, selections[1].phase],
        #[cfg(test)]
        capacity: [selections[0].capacity, selections[1].capacity],
        #[cfg(test)]
        current_drainage: [trial.coordinates[5], trial.coordinates[11]],
        #[cfg(any(test, feature = "test-support"))]
        natural_selections,
        #[cfg(any(test, feature = "test-support"))]
        predictor_selections: selections,
        #[cfg(any(test, feature = "test-support"))]
        final_selections,
        #[cfg(any(test, feature = "test-support"))]
        corrected_assembly: changed,
        eliminated_coordinates,
        linear_dimension,
        direction,
        pivot,
        matrix_norm,
        activity,
    })
}

#[cfg(test)]
fn phase_test_result(
    result: Result<M1PhaseJoinControllerReport, M1PhaseJoinControllerError>,
) -> Result<M1PhaseJoinControllerReport, M1PhaseJoinFailure> {
    result.map_err(|failure| match &failure.error {
        M1CoupledError::Numerical(numerical)
            if numerical.kind
                == crate::numerics::NumericalFailureKind::PhaseActiveSetInconsistent =>
        {
            M1PhaseJoinFailure::InconsistentFinalSet {
                activity: failure.activity,
            }
        }
        M1CoupledError::Numerical(numerical)
            if numerical.kind
                == crate::numerics::NumericalFailureKind::PhaseActiveSetUnrepresentable =>
        {
            M1PhaseJoinFailure::UnrepresentableProbe {
                activity: failure.activity,
            }
        }
        _ => M1PhaseJoinFailure::Source {
            error: Box::new(failure.error),
            activity: failure.activity,
        },
    })
}

#[cfg(test)]
pub fn m1_phase_join_controller_iteration(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
) -> Result<M1PhaseJoinControllerReport, M1PhaseJoinFailure> {
    phase_test_result(m1_phase_join_controller(
        input, trial, 0, 0, None, None, None,
    ))
}
#[cfg(test)]
pub fn m1_phase_join_controller_with_forced_predictor_direction(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    predictor: [f64; 21],
) -> Result<M1PhaseJoinControllerReport, M1PhaseJoinFailure> {
    phase_test_result(m1_phase_join_controller(
        input,
        trial,
        0,
        0,
        Some(predictor),
        Some(predictor),
        None,
    ))
}
#[cfg(test)]
pub fn m1_phase_join_controller_with_forced_directions(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    predictor: [f64; 21],
    final_direction: [f64; 21],
) -> Result<M1PhaseJoinControllerReport, M1PhaseJoinFailure> {
    phase_test_result(m1_phase_join_controller(
        input,
        trial,
        0,
        0,
        Some(predictor),
        Some(final_direction),
        None,
    ))
}

/// A test-only mutation of an already-selected assembly.  It models a bad row
/// reaching the old LU; it neither validates the row nor manufactures a result.
#[cfg(test)]
#[derive(Clone, Copy)]
pub struct M1SelectedAffineAssemblyPoison {
    pub row: usize,
    pub column: usize,
    pub value: f64,
}

#[cfg(test)]
pub fn m1_phase_join_controller_with_selected_assembly_poison(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    predictor: [f64; 21],
    poison: M1SelectedAffineAssemblyPoison,
) -> Result<M1PhaseJoinControllerReport, M1PhaseJoinFailure> {
    phase_test_result(m1_phase_join_controller(
        input,
        trial,
        0,
        0,
        Some(predictor),
        Some(predictor),
        Some(poison),
    ))
}

/// Test-only observation of the shared selected-drainage controller. This is
/// not a second solver: contract tests observe the production reduction and
/// candidate arithmetic through its public test seam.
#[cfg(test)]
pub struct M1CurrentLinearSystemTestView {
    pub raw_residuals: Vec<f64>,
    pub raw_jacobian: Vec<Vec<f64>>,
    pub normalized_jacobian: Vec<Vec<f64>>,
    pub rhs: Vec<f64>,
    pub direction: [f64; 21],
    pub current_drainage: [f64; 2],
    pub capacity: [openwepp_vegetation::cold_canopy_m1::M1CapacityBranch; 2],
    pub drainage_selected: [bool; 2],
    pub activity: M1PhaseJoinActivity,
}

#[cfg(test)]
pub fn m1_current_unreduced_linear_system_for_test(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
) -> Result<M1CurrentLinearSystemTestView, M1PhaseJoinFailure> {
    let report = phase_test_result(m1_phase_join_controller(
        input, trial, 0, 0, None, None, None,
    ))?;
    let selections = phase_join_selections(input, trial, &report.direction)?;
    Ok(M1CurrentLinearSystemTestView {
        raw_residuals: report.base.residuals,
        raw_jacobian: report.raw_jacobian,
        normalized_jacobian: report.normalized_jacobian,
        rhs: report.rhs,
        direction: report.direction,
        current_drainage: report.current_drainage,
        capacity: [selections[0].capacity, selections[1].capacity],
        drainage_selected: [
            selections[0].capacity
                == openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Drainage,
            selections[1].capacity
                == openwepp_vegetation::cold_canopy_m1::M1CapacityBranch::Drainage,
        ],
        activity: report.activity,
    })
}

#[cfg(test)]
pub fn m1_current_unreduced_candidate_for_test(
    coordinates: [f64; 21],
    direction: [f64; 21],
    alpha: f64,
    drainage_selected: [bool; 2],
) -> [f64; 21] {
    std::array::from_fn(|index| {
        m1_candidate_coordinate(
            coordinates[index],
            direction[index],
            alpha,
            matches!(index, 5) && drainage_selected[0]
                || matches!(index, 11) && drainage_selected[1],
        )
        .expect("test candidate inputs valid")
    })
}

#[cfg(test)]
pub fn m1_trial_coordinates_for_test(trial: &M1CoupledColumnTrial) -> [f64; 21] {
    trial.coordinates
}

#[cfg(test)]
pub fn m1_core_for_test(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
) -> Result<(), M1CoupledError> {
    core(input, trial).map(|_| ())
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub enum M1AcceptedMaterializationPoison {
    NonfiniteResidual,
    NonfiniteScale,
    NonfiniteLeaf,
    NonfiniteHydraulic,
    NonfiniteHydraulicFlux,
    NonfiniteHydraulicContinuity,
    NonfiniteReservoirWetFraction,
    NonfiniteReservoirWetTemperature,
    NonfiniteReservoirCapacity,
    NonfiniteReservoirOperand,
    NonfiniteDuration,
    EmptyMass,
    NegativeDrainage,
    OverCapacityOneUlp,
    RawMassResidual,
    RawEnthalpyResidual,
    RawCapacity,
    ActualCapacityResidual,
    NonfiniteCapacity,
    ScaledMass,
    EnthalpyLedger,
}

#[cfg(test)]
pub fn m1_reset_accepted_capture_for_test() {
    M1_ACCEPTED_CAPTURE.with(|capture| *capture.borrow_mut() = None);
}

#[cfg(test)]
#[must_use]
pub fn m1_captured_accepted_adapter_binding_for_test() -> Option<(M1CoupledColumnInput, [f64; 21])>
{
    M1_ACCEPTED_CAPTURE.with(|capture| {
        capture
            .borrow()
            .as_ref()
            .map(|(input, trial, _)| (input.clone(), trial.coordinates))
    })
}

#[cfg(test)]
pub fn m1_materialize_accepted_poison_for_test(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    poison: M1AcceptedMaterializationPoison,
) -> Result<M1CoupledEvaluation, M1CoupledError> {
    m1_materialize_accepted_from_parts_for_test(
        input.clone(),
        trial.clone(),
        core(input, trial)?,
        Some(poison),
    )
}

#[cfg(test)]
fn m1_apply_over_capacity_one_ulp_poison(
    core: &mut Core,
    trial: &M1CoupledColumnTrial,
) -> Result<(), openwepp_vegetation::cold_canopy_m1::M1Error> {
    let liquid = M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[3], trial.coordinates[4])?
        .liquid_mass_kg_m2();
    core.occupancies[0].capacity = liquid.next_down();
    Ok(())
}

#[cfg(test)]
fn m1_materialize_accepted_from_parts_for_test(
    mut input: M1CoupledColumnInput,
    mut trial: M1CoupledColumnTrial,
    mut core: Core,
    poison: Option<M1AcceptedMaterializationPoison>,
) -> Result<M1CoupledEvaluation, M1CoupledError> {
    match poison {
        None => {}
        Some(poison) => match poison {
            M1AcceptedMaterializationPoison::NonfiniteResidual => core.residuals[0] = f64::NAN,
            M1AcceptedMaterializationPoison::NonfiniteScale => core.scales[0] = f64::NAN,
            M1AcceptedMaterializationPoison::NonfiniteLeaf => {
                M1_MATERIALIZATION_OUTPUT_POISON.with(|flag| flag.set(4))
            }
            M1AcceptedMaterializationPoison::NonfiniteHydraulic => {
                M1_MATERIALIZATION_OUTPUT_POISON.with(|flag| flag.set(1))
            }
            M1AcceptedMaterializationPoison::NonfiniteHydraulicFlux => {
                M1_MATERIALIZATION_OUTPUT_POISON.with(|flag| flag.set(2))
            }
            M1AcceptedMaterializationPoison::NonfiniteHydraulicContinuity => {
                M1_MATERIALIZATION_OUTPUT_POISON.with(|flag| flag.set(3))
            }
            M1AcceptedMaterializationPoison::NonfiniteReservoirWetFraction => {
                M1_MATERIALIZATION_OUTPUT_POISON.with(|flag| flag.set(5))
            }
            M1AcceptedMaterializationPoison::NonfiniteReservoirWetTemperature => {
                M1_MATERIALIZATION_OUTPUT_POISON.with(|flag| flag.set(6))
            }
            M1AcceptedMaterializationPoison::NonfiniteReservoirCapacity => {
                M1_MATERIALIZATION_OUTPUT_POISON.with(|flag| flag.set(7))
            }
            M1AcceptedMaterializationPoison::NonfiniteReservoirOperand => {
                core.occupancies[0].non_vapor = f64::NAN
            }
            M1AcceptedMaterializationPoison::NonfiniteDuration => {
                input.column.interval_s = f64::NAN
            }
            M1AcceptedMaterializationPoison::EmptyMass => {
                trial.coordinates[3] = 0.0;
                trial.coordinates[4] = 0.0;
            }
            M1AcceptedMaterializationPoison::NegativeDrainage => {
                trial.coordinates[5] = -f64::MIN_POSITIVE
            }
            M1AcceptedMaterializationPoison::OverCapacityOneUlp => {
                m1_apply_over_capacity_one_ulp_poison(&mut core, &trial)?;
            }
            M1AcceptedMaterializationPoison::RawMassResidual => {
                core.occupancies[0].mass_residual = 1.0
            }
            M1AcceptedMaterializationPoison::RawEnthalpyResidual => {
                core.occupancies[0].enthalpy_residual = 1.0
            }
            M1AcceptedMaterializationPoison::RawCapacity => core.residuals[5] = 2.0e-9,
            M1AcceptedMaterializationPoison::ActualCapacityResidual => {
                trial.coordinates[3] -= 2.0e-9;
                trial.coordinates[5] = 2.0e-9 / input.column.interval_s;
            }
            M1AcceptedMaterializationPoison::NonfiniteCapacity => {
                core.occupancies[0].capacity = f64::NAN
            }
            M1AcceptedMaterializationPoison::ScaledMass => {
                core.residuals[3] = 5.0e-10;
                core.occupancies[0].mass_residual = 5.0e-10;
            }
            M1AcceptedMaterializationPoison::EnthalpyLedger => core.occupancies[0].non_vapor += 1.0,
        },
    }
    let materialized = materialize_m1_coupled_column(&input, &trial, core, None);
    M1_MATERIALIZATION_OUTPUT_POISON.with(|flag| flag.set(0));
    materialized
}

#[cfg(test)]
pub fn m1_materialize_captured_accepted_poison_for_test(
    poison: M1AcceptedMaterializationPoison,
) -> Result<M1CoupledEvaluation, M1CoupledError> {
    let (input, trial, core) = M1_ACCEPTED_CAPTURE
        .with(|capture| capture.borrow().clone())
        .expect("accepted materialization capture");
    m1_materialize_accepted_from_parts_for_test(input, trial, core, Some(poison))
}

#[cfg(test)]
pub fn m1_materialize_captured_accepted_for_test() -> Result<M1CoupledEvaluation, M1CoupledError> {
    let (input, trial, core) = M1_ACCEPTED_CAPTURE
        .with(|capture| capture.borrow().clone())
        .expect("accepted materialization capture");
    m1_materialize_accepted_from_parts_for_test(input, trial, core, None)
}

#[cfg(test)]
pub fn m1_captured_finite_overcapacity_core_for_test() -> Result<(f64, f64, f64), M1CoupledError> {
    let (input, mut trial, core_result) = M1_ACCEPTED_CAPTURE
        .with(|capture| capture.borrow().clone())
        .expect("accepted materialization capture");
    trial.coordinates[3] = core_result.occupancies[0].capacity.next_up();
    trial.coordinates[4] = 0.0;
    let liquid = M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[3], trial.coordinates[4])?
        .liquid_mass_kg_m2();
    let capacity = core(&input, &trial)?.occupancies[0].capacity;
    Ok((liquid, capacity, trial.coordinates[3]))
}

#[cfg(test)]
pub fn m1_captured_accepted_guard_binding_for_test() -> (f64, f64, f64, f64, f64, f64) {
    let (input, trial, core) = M1_ACCEPTED_CAPTURE
        .with(|capture| capture.borrow().clone())
        .expect("accepted materialization capture");
    (
        input.reservoirs[0].mass_kg_m2,
        core.occupancies[0].mass_residual,
        core.occupancies[0].enthalpy_residual,
        core.occupancies[0].non_vapor + 1.0,
        M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[3], trial.coordinates[4])
            .expect("captured accepted phase")
            .liquid_mass_kg_m2(),
        core.occupancies[0].capacity,
    )
}

/// Test metadata for the shared selected-drainage validation boundary. The
/// adapter forwards it to the same production reduction helper used by the
/// controller; it does not install a second solver in test code.
#[cfg(test)]
pub struct M1DrainageMetadataTestView {
    pub rows: [usize; 2],
    pub raw_row_identity: [usize; 2],
    pub drainage_selected: [bool; 2],
    pub duration_s: [f64; 2],
    pub support_s: [f64; 2],
    pub row_scales: [f64; 2],
    pub current_drainage: [f64; 2],
    pub raw_drainage_residual: [f64; 2],
    pub raw_drainage_diagonal: [f64; 2],
}

#[cfg(test)]
pub fn m1_current_full_lu_ignoring_drainage_metadata_for_test(
    matrix: &[Vec<f64>],
    rhs: &[f64],
    metadata: M1DrainageMetadataTestView,
) -> Result<(Vec<f64>, f64, f64), &'static str> {
    m1_current_full_lu_activity_for_test(matrix, rhs, metadata).0
}

#[cfg(test)]
pub fn m1_current_full_lu_activity_for_test(
    matrix: &[Vec<f64>],
    rhs: &[f64],
    metadata: M1DrainageMetadataTestView,
) -> (Result<(Vec<f64>, f64, f64), &'static str>, u32) {
    let mut rows = Vec::with_capacity(2);
    for slot in 0..2 {
        if !metadata.drainage_selected[slot] {
            continue;
        }
        rows.push(M1DrainageEliminationRow {
            row: metadata.rows[slot],
            identity: metadata.raw_row_identity[slot],
            support: metadata.support_s[slot],
            duration: metadata.duration_s[slot],
            scale: metadata.row_scales[slot],
            current: metadata.current_drainage[slot],
            raw_residual: metadata.raw_drainage_residual[slot],
            raw_diagonal: metadata.raw_drainage_diagonal[slot],
        });
    }
    let mut raw = vec![vec![0.0; rhs.len()]; rhs.len()];
    for row in &rows {
        if row.row < rhs.len() {
            raw[row.row][row.row] = row.raw_diagonal;
        }
    }
    let mut activity = M1PhaseJoinActivity::default();
    let result = solve_selected_drainage_linear(matrix, rhs, &raw, &rows, 0, false, &mut activity)
        .map(|(direction, pivot, norm, _, _)| (direction, pivot, norm))
        .map_err(|failure| match failure {
            M1DrainageLinearFailure::Integrity => "drainage_elimination_integrity",
            M1DrainageLinearFailure::Singular { .. } => "singular_pivot",
            M1DrainageLinearFailure::NonfiniteResult => "nonfinite_reduced_result",
        });
    (result, activity.linear_solves)
}

#[cfg(test)]
pub fn m1_phase_join_controller_unrepresentable_probe(
    input: &M1CoupledColumnInput,
) -> Result<M1PhaseJoinControllerReport, M1PhaseJoinFailure> {
    let mut input = input.clone();
    let mass = (2_f64).powi(44) / LF;
    input.reservoirs[0].mass_kg_m2 = mass;
    input.reservoirs[0].enthalpy_j_m2 = -(2_f64).powi(44);
    let area = input.column.occupancies[0].lai + input.column.occupancies[0].sai;
    input.column.occupancies[0].liquid_capacity_kg_m2_plant = 2.0 * mass / area;
    let mut coordinates = [input.column.air_temperature_k; 21];
    coordinates[0..3].copy_from_slice(&[267.0, 266.0, 268.0]);
    coordinates[5] = 0.0;
    coordinates[11] = 0.0;
    coordinates[3] = mass;
    coordinates[4] = -(2_f64).powi(44);
    coordinates[6..9].copy_from_slice(&[269.0, 267.0, 270.0]);
    coordinates[9] = input.reservoirs[1].mass_kg_m2;
    coordinates[10] = input.reservoirs[1].enthalpy_j_m2;
    coordinates[12] = 268.0;
    coordinates[13] = 0.0015;
    coordinates[14] = 265.15;
    coordinates[15..].copy_from_slice(&[291.5, 289.8, 293.0, 293.0, 293.0, 270.0]);
    let mut predictor = [0.0; 21];
    predictor[4] = -1.0;
    phase_test_result(m1_phase_join_controller(
        &input,
        &M1CoupledColumnTrial::from_coordinates(coordinates),
        0,
        0,
        Some(predictor),
        None,
        None,
    ))
}

fn m1_tolerances(input: &M1CoupledColumnInput, core: &Core) -> [f64; 21] {
    let mut values = [1.0e-9; 21];
    for row in [0, 1, 2, 6, 7, 8, 12] {
        values[row] = crate::physics::energy_tolerance(core.scales[row]);
    }
    values[13] = crate::physics::water_tolerance(core.scales[13]);
    for index in 0..2 {
        let base = index * 6;
        let detail = &core.occupancies[index];
        let mass_scale = input.reservoirs[index]
            .mass_kg_m2
            .abs()
            .max(core.scales[base + 3])
            .max(1.0e-9);
        let heat_scale = input.reservoirs[index]
            .enthalpy_j_m2
            .abs()
            .max(core.scales[base + 4])
            .max(1.0);
        values[base + 3] = 1.0e-9_f64.min(1.0e-8 * mass_scale);
        values[base + 4] = 1.0e-6_f64.min(1.0e-8 * heat_scale);
        values[base + 5] = 1.0e-9_f64.min(1.0e-8 * detail.capacity.max(1.0e-9));
    }
    // represented-snow Stage-3 ground and soil identity rows
    values[14] = 1.0e-9;
    values[15..].fill(1.0e-9);
    values
}

fn normalized(core: &Core, input: &M1CoupledColumnInput) -> Vec<f64> {
    let tolerances = m1_tolerances(input, core);
    core.residuals
        .iter()
        .zip(tolerances)
        .map(|(r, t)| r / t)
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum M1FullTrialNoUpdateRefusal {
    DomainInvalid,
    GovernedStepThresholdExceeded,
}

#[derive(Clone, Copy, Debug)]
struct M1CandidateStepNorms {
    temperature_k: f64,
    humidity_kg_kg: f64,
    ci_pa: f64,
    hydraulic_mm: f64,
    beta: f64,
}

impl M1CandidateStepNorms {
    fn diagnostics(self) -> StepNorms {
        StepNorms {
            temperature_k: Some(self.temperature_k),
            humidity_kg_kg: Some(self.humidity_kg_kg),
            ci_pa: Some(self.ci_pa),
            hydraulic_mm: Some(self.hydraulic_mm),
            beta: Some(self.beta),
        }
    }

    fn governed_passes(self) -> bool {
        self.ci_pa.is_finite()
            && m1_governed_step_components_pass(
                self.temperature_k,
                self.humidity_kg_kg,
                self.hydraulic_mm,
                self.beta,
            )
    }

    fn governed_threshold_exceeded(self) -> bool {
        [
            self.temperature_k,
            self.humidity_kg_kg,
            self.hydraulic_mm,
            self.beta,
        ]
        .iter()
        .all(|value| value.is_finite())
            && !m1_governed_step_components_pass(
                self.temperature_k,
                self.humidity_kg_kg,
                self.hydraulic_mm,
                self.beta,
            )
    }
}

fn m1_complete_step_norms_pass(steps: &StepNorms) -> bool {
    steps.ci_pa.is_some_and(f64::is_finite)
        && steps
            .temperature_k
            .zip(steps.humidity_kg_kg)
            .zip(steps.hydraulic_mm.zip(steps.beta))
            .is_some_and(|((temperature_k, humidity_kg_kg), (hydraulic_mm, beta))| {
                m1_governed_step_components_pass(temperature_k, humidity_kg_kg, hydraulic_mm, beta)
            })
}

fn m1_complete_residuals_pass(residuals: &[f64]) -> bool {
    !residuals.is_empty()
        && residuals
            .iter()
            .all(|residual| residual.is_finite() && residual.abs() <= 1.0)
}

fn m1_finite_max_abs(values: impl IntoIterator<Item = f64>) -> Option<f64> {
    values.into_iter().try_fold(0.0_f64, |maximum, value| {
        value.is_finite().then_some(maximum.max(value.abs()))
    })
}

fn m1_candidate_step_norms(
    current: &M1CoupledColumnTrial,
    candidate: &M1CoupledColumnTrial,
    base: &Core,
    candidate_core: &Core,
    base_hydraulics: &[f64],
    candidate_hydraulics: &[f64],
) -> Option<M1CandidateStepNorms> {
    const TEMPERATURE_COORDINATES: [usize; 14] = [0, 1, 2, 6, 7, 8, 12, 14, 15, 16, 17, 18, 19, 20];
    let dry_temperature = m1_finite_max_abs(
        TEMPERATURE_COORDINATES
            .into_iter()
            .map(|index| candidate.coordinates[index] - current.coordinates[index]),
    )?;
    let wet_temperature = m1_finite_max_abs(
        candidate_core
            .occupancies
            .iter()
            .zip(&base.occupancies)
            .map(
                |(next, previous)| match (next.wet_temperature_k, previous.wet_temperature_k) {
                    (Some(next), Some(previous)) => next - previous,
                    (None, None) => 0.0,
                    _ => f64::NAN,
                },
            ),
    )?;
    let humidity_kg_kg = (candidate.coordinates[13] - current.coordinates[13]).abs();
    let hydraulic_mm = m1_finite_max_abs(
        candidate_hydraulics
            .iter()
            .zip(base_hydraulics)
            .map(|(next, previous)| next - previous),
    )?;
    if candidate_hydraulics.len() != base_hydraulics.len() {
        return None;
    }
    let beta = m1_finite_max_abs(
        candidate_core
            .occupancies
            .iter()
            .zip(&base.occupancies)
            .flat_map(|(next, previous)| {
                [
                    next.sun.beta - previous.sun.beta,
                    next.shade.beta - previous.shade.beta,
                ]
            }),
    )?;
    let ci_pa = m1_finite_max_abs(
        candidate_core
            .occupancies
            .iter()
            .zip(&base.occupancies)
            .flat_map(|(next, previous)| {
                [
                    next.sun.ci_pa - previous.sun.ci_pa,
                    next.shade.ci_pa - previous.shade.ci_pa,
                ]
            }),
    )?;
    Some(M1CandidateStepNorms {
        temperature_k: dry_temperature.max(wet_temperature),
        humidity_kg_kg,
        ci_pa,
        hydraulic_mm,
        beta,
    })
}

#[cfg(test)]
fn m1_test_current_residuals(iteration: u32, mut residuals: Vec<f64>) -> Vec<f64> {
    if iteration == 3 && M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE.with(Cell::get) {
        // The real terminal decision seam must reject a nonfinite current
        // residual before either full or halved witness handling.
        residuals[0] = f64::NAN;
    }
    residuals
}

#[cfg(test)]
fn m1_test_targets_iteration(iteration: u32, norm: f64) -> bool {
    M1_HALVED_TRIAL_TARGET_ITERATION
        .with(Cell::get)
        .is_some_and(|target| target == iteration)
        || norm <= 1.0
}

#[cfg(test)]
fn m1_test_full_governed_excess(
    current_norm: f64,
    exponent: u32,
    mut steps: M1CandidateStepNorms,
) -> M1CandidateStepNorms {
    if current_norm <= 1.0 && exponent == 0 && M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(Cell::get)
    {
        // Candidate evaluation, hydraulics, and candidate-current
        // reconstruction have already run. This test-only seam changes only
        // the full-witness decision outcome.
        steps.beta = M1_STEP_BETA_MAX + f64::EPSILON;
    }
    steps
}

#[cfg(test)]
fn m1_test_step_poison(exponent: u32, mut steps: M1CandidateStepNorms) -> M1CandidateStepNorms {
    if let Some((target, component)) = M1_HALVED_TRIAL_STEP_POISON.with(Cell::get) {
        if target == exponent {
            let value = if component & 0x80 != 0 {
                f64::NAN
            } else {
                f64::INFINITY
            };
            match component & 0x3f {
                0 => {
                    steps.temperature_k = if component & 0x40 != 0 {
                        M1_STEP_TEMPERATURE_MAX_K + f64::EPSILON
                    } else {
                        value
                    }
                }
                1 => {
                    steps.humidity_kg_kg = if component & 0x40 != 0 {
                        M1_STEP_HUMIDITY_MAX_KG_KG + f64::EPSILON
                    } else {
                        value
                    }
                }
                2 => {
                    steps.hydraulic_mm = if component & 0x40 != 0 {
                        M1_STEP_HYDRAULIC_MAX_MM + f64::EPSILON
                    } else {
                        value
                    }
                }
                3 => {
                    steps.beta = if component & 0x40 != 0 {
                        M1_STEP_BETA_MAX + f64::EPSILON
                    } else {
                        value
                    }
                }
                4 => steps.ci_pa = value,
                5 => steps.ci_pa = 1.0e300,
                _ => {}
            }
        }
    }
    steps
}

#[cfg(test)]
fn m1_test_no_update_enabled() -> bool {
    !M1_DISABLE_NO_UPDATE.with(Cell::get)
}

#[cfg(not(test))]
const fn m1_test_no_update_enabled() -> bool {
    true
}

#[cfg(test)]
fn m1_test_poison_installed_steps(steps: StepNorms) -> StepNorms {
    if M1_INSTALLED_STEP_POISON.with(Cell::get).is_none() {
        return steps;
    }
    // Decision-seam-only test record: coordinates, residuals, Core, and the
    // installed candidate remain real; this isolates the stored diagnostic
    // acceptance predicate from unrelated physical step magnitudes.
    let mut steps = StepNorms {
        temperature_k: Some(0.0),
        humidity_kg_kg: Some(0.0),
        ci_pa: Some(0.0),
        hydraulic_mm: Some(0.0),
        beta: Some(0.0),
    };
    match M1_INSTALLED_STEP_POISON.with(Cell::get) {
        Some(0) => steps.beta = Some(M1_STEP_BETA_MAX + f64::EPSILON),
        Some(1) => steps.ci_pa = Some(f64::NAN),
        _ => {}
    }
    steps
}

/// Exact stored RHS arithmetic for both controller assemblies and diagnostics.
fn m1_normalized_linear_rhs(core: &Core, input: &M1CoupledColumnInput) -> Vec<f64> {
    let mut rhs: Vec<f64> = normalized(core, input).iter().map(|value| -value).collect();
    for (index, occupancy) in input.column.occupancies.iter().enumerate() {
        for (offset, area) in [
            occupancy.sun.leaf_area_m2_m2_tile,
            occupancy.shade.leaf_area_m2_m2_tile,
            occupancy.stem_area_m2_m2_tile,
        ]
        .into_iter()
        .enumerate()
        {
            if area == 0.0 {
                rhs[index * 6 + offset] = -core.residuals[index * 6 + offset];
            }
        }
    }
    rhs
}

#[cfg(test)]
fn m1_trace_before_materialization(
    input: &M1CoupledColumnInput,
    trial: &M1CoupledColumnTrial,
    core: &Core,
    steps: Option<&StepNorms>,
) {
    eprintln!(
        "m1-before-materialization {}",
        serde_json::json!({
            "coordinates": trial.coordinates,
            "raw_residuals": core.residuals,
            "normalized_residuals": normalized(core, input),
            "step_norms": steps.map(|value| format!("{value:?}")),
            "phases": [
                M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[3], trial.coordinates[4]).ok().map(|value| format!("{:?}", value.phase())),
                M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[9], trial.coordinates[10]).ok().map(|value| format!("{:?}", value.phase())),
            ],
            "liquid_mass": [
                M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[3], trial.coordinates[4]).ok().map(|value| value.liquid_mass_kg_m2()),
                M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[9], trial.coordinates[10]).ok().map(|value| value.liquid_mass_kg_m2()),
            ],
            "capacity": core.occupancies.iter().map(|value| value.capacity).collect::<Vec<_>>(),
            "drainage": [trial.coordinates[5], trial.coordinates[11]],
        })
    );
}

fn m1_failure(
    kind: crate::numerics::NumericalFailureKind,
    iterations: u32,
    input: &M1CoupledColumnInput,
    core: &Core,
    trial: &M1CoupledColumnTrial,
    backtracking_count: u32,
    step_norms: StepNorms,
    pivot: Option<f64>,
    matrix_norm: Option<f64>,
) -> M1CoupledError {
    let tolerances = m1_tolerances(input, core);
    let normalized_residuals = normalized(core, input);
    #[cfg(test)]
    eprintln!(
        "m1-failure-endpoint {}",
        serde_json::json!({
            "kind": format!("{kind:?}"), "iteration": iterations, "backtracking": backtracking_count,
            "coordinates": trial.coordinates, "raw_residuals": core.residuals,
            "normalized_residuals": normalized_residuals, "step_norms": format!("{step_norms:?}"),
            "phases": [
                M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[3], trial.coordinates[4]).ok().map(|value| format!("{:?}", value.phase())),
                M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[9], trial.coordinates[10]).ok().map(|value| format!("{:?}", value.phase())),
            ],
            "liquid_mass": [
                M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[3], trial.coordinates[4]).ok().map(|value| value.liquid_mass_kg_m2()),
                M1PhaseReservoir::from_mass_enthalpy(trial.coordinates[9], trial.coordinates[10]).ok().map(|value| value.liquid_mass_kg_m2()),
            ],
            "capacity": core.occupancies.iter().map(|value| value.capacity).collect::<Vec<_>>(),
            "drainage": [trial.coordinates[5], trial.coordinates[11]],
            "pivot": pivot, "matrix_norm": matrix_norm,
        })
    );
    M1CoupledError::Numerical(crate::numerics::NumericalFailure {
        kind,
        iterations,
        normalized_residuals,
        ordered_residuals: core
            .residuals
            .iter()
            .enumerate()
            .map(|(index, raw)| NormalizedResidual {
                identity: format!("m1_row_{index}"),
                raw: *raw,
                scale: core.scales[index],
                tolerance: tolerances[index],
                normalized: raw / tolerances[index],
                unit: match index {
                    3 | 5 | 9 | 11 => ResidualUnit::KilogramsPerSquareMeter,
                    4 | 10 => ResidualUnit::JoulesPerSquareMeter,
                    13 => ResidualUnit::KilogramsPerSquareMeterSecond,
                    14..=20 => ResidualUnit::Kelvin,
                    _ => ResidualUnit::WattsPerSquareMeter,
                },
            })
            .collect(),
        failed_solution: trial.coordinates.to_vec(),
        occupancy_id: None,
        active_bounds: Vec::new(),
        backtracking_count,
        step_norms,
        pivot_magnitude: pivot,
        matrix_norm,
    })
}

fn hydraulic_potentials(
    input: &M1CoupledColumnInput,
    core: &Core,
) -> Result<Vec<f64>, M1CoupledError> {
    M1_RUNTIME_ACTIVITY.with(|activity| activity.borrow_mut().hydraulic_entries += 1);
    let mut values = Vec::with_capacity(8);
    for index in 0..2 {
        let occupancy = &input.column.occupancies[index];
        let detail = &core.occupancies[index];
        values.extend(
            solve_m1_full_supply_hydraulics(
                &input.column,
                occupancy,
                [
                    detail.sun.vapor_kg_m2_tile_s,
                    detail.shade.vapor_kg_m2_tile_s,
                ],
                [detail.sun.gas_branch, detail.shade.gas_branch],
            )?
            .potentials_mm,
        );
    }
    Ok(values)
}

pub(crate) fn record_m1_actual_hydraulic_entry() {
    M1_RUNTIME_ACTIVITY.with(|activity| activity.borrow_mut().actual_hydraulic_entries += 1);
}

/// Solves the bounded M1 diagnostic column from the canonical cold-column seed.
pub fn solve_m1_coupled_column(
    input: &M1CoupledColumnInput,
) -> Result<M1CoupledEvaluation, M1CoupledError> {
    #[cfg(any(test, feature = "test-support"))]
    let diagnostic_generation = M1_DIAGNOSTIC_PENDING_INPUT.with(|pending| {
        let Ok(mut pending) = pending.try_borrow_mut() else {
            return None;
        };
        match pending.take() {
            Some(expected) if expected.input == *input => Some(expected.generation),
            Some(_) => {
                M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
                    if let Ok(mut observation) = observation.try_borrow_mut() {
                        *observation = None;
                    }
                });
                None
            }
            None => None,
        }
    });
    #[cfg(any(test, feature = "test-support"))]
    if let Some(generation) = diagnostic_generation {
        m1_diagnostic_begin_empty(input, generation);
    }
    let result = solve_m1_coupled_column_inner(input);
    #[cfg(any(test, feature = "test-support"))]
    if diagnostic_generation.is_some() {
        m1_diagnostic_terminal(&result);
    }
    result
}

fn solve_m1_coupled_column_inner(
    input: &M1CoupledColumnInput,
) -> Result<M1CoupledEvaluation, M1CoupledError> {
    #[cfg(any(test, feature = "test-support"))]
    let diagnostic_generation = M1_DIAGNOSTIC_OBSERVATION.with(|observation| {
        observation.try_borrow().ok().and_then(|observation| {
            observation
                .as_ref()
                .map(|observation| observation.generation)
        })
    });
    #[cfg(any(test, feature = "test-support"))]
    let diagnostic_active = diagnostic_generation.is_some();
    M1_RUNTIME_ACTIVITY.with(|activity| {
        *activity.borrow_mut() = M1LoopActivityObservation::new();
    });
    // Structural preflight precedes all topology-indexed seed reads.
    if input.column.occupancies.len() != 2
        || input.column.ground.soil_nodes.len() != 6
        || input
            .topology
            .occupancy_ids
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            != 2
        || input
            .topology
            .soil_layer_ids
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            != 6
    {
        return Err(LandSurfaceEnergyError::UnsupportedDomain("m1_diagnostic_envelope").into());
    }
    let boundary = input
        .column
        .stage3_lower_boundary
        .as_ref()
        .ok_or_else(|| LandSurfaceEnergyError::UnsupportedDomain("m1_diagnostic_envelope"))?;
    let mut coordinates = [input.column.air_temperature_k; 21];
    coordinates[3] = input.reservoirs[0].mass_kg_m2;
    coordinates[4] = input.reservoirs[0].enthalpy_j_m2;
    coordinates[5] = 0.0;
    coordinates[9] = input.reservoirs[1].mass_kg_m2;
    coordinates[10] = input.reservoirs[1].enthalpy_j_m2;
    coordinates[11] = 0.0;
    coordinates[12] = input.column.air_temperature_k;
    coordinates[13] = input.column.air_specific_humidity_kg_kg;
    coordinates[14] = boundary.snow_temperature_k;
    for soil in 0..6 {
        coordinates[15 + soil] = input.column.ground.soil_nodes[soil].beginning_temperature_k;
    }
    let mut trial = M1CoupledColumnTrial::from_coordinates(coordinates);
    #[cfg(any(test, feature = "test-support"))]
    if let Some(generation) = diagnostic_generation {
        // This is the exact constructed canonical seed, after preflight and
        // including the canonical zero drainage coordinates.
        m1_diagnostic_begin(input, generation, trial.coordinates);
    }
    validate(input, &trial)?;
    let mut backtracking_count = 0;
    let mut last_installed_steps: Option<StepNorms> = None;
    for iteration in 0..=crate::numerics::MAX_NEWTON_ITERATIONS {
        if let Some(steps) = &last_installed_steps {
            let current = core(input, &trial)?;
            let current_residuals = normalized(&current, input);
            if m1_complete_residuals_pass(&current_residuals) && m1_complete_step_norms_pass(steps)
            {
                #[cfg(test)]
                m1_trace_before_materialization(input, &trial, &current, Some(steps));
                return materialize_m1_coupled_column(input, &trial, current, None);
            }
        }
        if iteration == crate::numerics::MAX_NEWTON_ITERATIONS {
            let terminal = core(input, &trial)?;
            if normalized(&terminal, input)
                .iter()
                .all(|value| value.is_finite())
                && normalized(&terminal, input)
                    .iter()
                    .map(|value| value.abs())
                    .fold(0.0_f64, f64::max)
                    <= 1.0
                && last_installed_steps
                    .as_ref()
                    .is_some_and(m1_complete_step_norms_pass)
            {
                #[cfg(test)]
                m1_trace_before_materialization(
                    input,
                    &trial,
                    &terminal,
                    last_installed_steps.as_ref(),
                );
                return materialize_m1_coupled_column(input, &trial, terminal, None);
            }
            return Err(m1_failure(
                crate::numerics::NumericalFailureKind::IterationLimit,
                iteration,
                input,
                &terminal,
                &trial,
                backtracking_count,
                StepNorms {
                    temperature_k: None,
                    humidity_kg_kg: None,
                    ci_pa: None,
                    hydraulic_mm: None,
                    beta: Some(0.0),
                },
                None,
                None,
            ));
        }
        // The controller owns the only predictor and optional corrected solve
        // for this Newton base.  It is deliberately called before candidate
        // construction so a frozen-set failure cannot enter backtracking.
        let phase_report = m1_phase_join_controller(
            input,
            &trial,
            iteration,
            backtracking_count,
            None,
            None,
            #[cfg(test)]
            None,
        )
        .map_err(M1PhaseJoinControllerError::into_source)?;
        let base = phase_report.base.clone();
        let residuals = normalized(&base, input);
        #[cfg(test)]
        let residuals = m1_test_current_residuals(iteration, residuals);
        if residuals.iter().any(|x| !x.is_finite())
            || phase_report
                .raw_jacobian
                .iter()
                .flatten()
                .any(|x| !x.is_finite())
        {
            return Err(saturation_error());
        }
        let Some(norm) = m1_finite_max_abs(residuals.iter().copied()) else {
            return Err(saturation_error());
        };
        let delta = phase_report.direction().to_vec();
        #[cfg(any(test, feature = "test-support"))]
        if diagnostic_active {
            m1_diagnostic_base(iteration, &trial, &base, &phase_report, &residuals, input);
        }
        let base_hydraulics = hydraulic_potentials(input, &base)?;
        let mut accepted = None;
        let mut first_domain_valid_halved_seen = false;
        let current_complete_residuals_pass = m1_complete_residuals_pass(&residuals);
        let mut full_trial_refusal = None;
        for exponent in 0..=crate::numerics::MAX_BACKTRACKING_HALVINGS {
            let factor = 0.5_f64.powi(exponent as i32);
            let mut candidate = trial.clone();
            for (index, change) in delta.iter().enumerate() {
                let drainage_slot = match index {
                    5 => Some(0),
                    11 => Some(1),
                    _ => None,
                };
                candidate.coordinates[index] = m1_candidate_coordinate(
                    trial.coordinates[index],
                    *change,
                    factor,
                    drainage_slot.is_some_and(|slot| phase_report.eliminated_coordinates[slot]),
                )
                .map_err(|()| {
                    m1_failure(
                        crate::numerics::NumericalFailureKind::DrainageEliminationIntegrity,
                        iteration,
                        input,
                        &base,
                        &trial,
                        exponent,
                        StepNorms {
                            temperature_k: None,
                            humidity_kg_kg: None,
                            ci_pa: None,
                            hydraulic_mm: None,
                            beta: Some(0.0),
                        },
                        None,
                        None,
                    )
                })?;
                #[cfg(test)]
                if drainage_slot.is_some_and(|slot| phase_report.eliminated_coordinates[slot])
                    && trial.coordinates[index] > 0.0
                    && factor < 1.0
                    && candidate.coordinates[index] == 0.0
                {
                    eprintln!(
                        "m1-drainage-underflow iteration={iteration} b={exponent} index={index} current={:.17e} factor={factor:.17e} candidate_sign={}",
                        trial.coordinates[index],
                        if candidate.coordinates[index].is_sign_negative() {
                            "negative"
                        } else {
                            "positive"
                        }
                    );
                }
            }
            M1_RUNTIME_ACTIVITY.with(|activity| activity.borrow_mut().trial_entries += 1);
            #[cfg(test)]
            if (m1_test_targets_iteration(iteration, norm)
                && exponent == 0
                && M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(Cell::get))
                || M1_HALVED_TRIAL_DOMAIN_INVALID_MASK
                    .with(|mask| mask.get() & (1 << exponent) != 0)
            {
                // Test-only: force the actual finite-difference predicate to
                // classify the full candidate as outside its domain.
                candidate.coordinates[0] = f64::NAN;
            }
            if !fd_trial_is_admissible(&candidate) {
                #[cfg(any(test, feature = "test-support"))]
                if diagnostic_active {
                    m1_diagnostic_trial(
                        iteration,
                        exponent,
                        factor,
                        &candidate,
                        None,
                        input,
                        format!(
                            "pre_evaluator_domain:{:?}",
                            fd_trial_rejection_reason(&candidate)
                        ),
                    );
                }
                if exponent == 0 && current_complete_residuals_pass {
                    full_trial_refusal = Some(M1FullTrialNoUpdateRefusal::DomainInvalid);
                }
                continue;
            }
            let is_first_halved_witness = m1_consume_first_domain_valid_halved_trial(
                exponent,
                &mut first_domain_valid_halved_seen,
            );
            #[cfg(test)]
            if m1_test_targets_iteration(iteration, norm)
                && M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.get() == Some(exponent))
            {
                M1_HALVED_TRIAL_WITNESS_TRACE
                    .with(|trace| trace.borrow_mut().push((exponent, false)));
                continue;
            }
            let candidate_core = match core(input, &candidate) {
                Ok(core) => core,
                Err(error) => {
                    #[cfg(any(test, feature = "test-support"))]
                    if diagnostic_active {
                        m1_diagnostic_trial(
                            iteration,
                            exponent,
                            factor,
                            &candidate,
                            None,
                            input,
                            format!("core_incomplete:{}", error.code()),
                        );
                    }
                    continue;
                }
            };
            let candidate_residuals = normalized(&candidate_core, input);
            if candidate_residuals.iter().any(|x| !x.is_finite()) {
                #[cfg(any(test, feature = "test-support"))]
                if diagnostic_active {
                    m1_diagnostic_trial(
                        iteration,
                        exponent,
                        factor,
                        &candidate,
                        Some(&candidate_core),
                        input,
                        "nonfinite_residual",
                    );
                }
                continue;
            }
            let Some(candidate_norm) = m1_finite_max_abs(candidate_residuals.iter().copied())
            else {
                continue;
            };
            #[cfg(test)]
            {
                let controlling = candidate_residuals
                    .iter()
                    .enumerate()
                    .max_by(|(_, left), (_, right)| left.abs().total_cmp(&right.abs()))
                    .map(|(row, value)| format!("row={row} value={value:.17e}"));
                eprintln!(
                    "m1-newton iteration={iteration} b={exponent} norm={candidate_norm:.17e} current={norm:.17e} controlling={controlling:?}"
                );
            }
            let potentials = match hydraulic_potentials(input, &candidate_core) {
                Ok(potentials) => potentials,
                Err(error) => {
                    #[cfg(any(test, feature = "test-support"))]
                    if diagnostic_active {
                        m1_diagnostic_trial(
                            iteration,
                            exponent,
                            factor,
                            &candidate,
                            Some(&candidate_core),
                            input,
                            format!("hydraulic_incomplete:{}", error.code()),
                        );
                    }
                    continue;
                }
            };
            #[cfg(test)]
            if m1_test_targets_iteration(iteration, norm)
                && M1_HALVED_TRIAL_HYDRAULIC_INCOMPLETE_AT
                    .with(|forced| forced.get() == Some(exponent))
            {
                M1_HALVED_TRIAL_WITNESS_TRACE
                    .with(|trace| trace.borrow_mut().push((exponent, false)));
                continue;
            }
            let Some(step_norms) = m1_candidate_step_norms(
                &trial,
                &candidate,
                &base,
                &candidate_core,
                &base_hydraulics,
                &potentials,
            ) else {
                #[cfg(any(test, feature = "test-support"))]
                if diagnostic_active {
                    m1_diagnostic_trial(
                        iteration,
                        exponent,
                        factor,
                        &candidate,
                        Some(&candidate_core),
                        input,
                        "step_ineligible",
                    );
                }
                continue;
            };
            M1_RUNTIME_ACTIVITY.with(|activity| {
                let mut activity = activity.borrow_mut();
                activity.last_current_coordinates = Some(trial.coordinates);
                activity.last_candidate_coordinates = Some(candidate.coordinates);
                activity.last_step_norms = Some(step_norms.diagnostics());
                activity.last_current_norm = Some(norm);
                activity.last_candidate_norm = Some(candidate_norm);
                activity.last_direction = Some(delta.clone());
                activity.last_factor = Some(factor);
                activity.last_current_dry_temperatures = Some([
                    trial.coordinates[0],
                    trial.coordinates[1],
                    trial.coordinates[2],
                    trial.coordinates[6],
                    trial.coordinates[7],
                    trial.coordinates[8],
                    trial.coordinates[12],
                    trial.coordinates[14],
                    trial.coordinates[15],
                    trial.coordinates[16],
                    trial.coordinates[17],
                    trial.coordinates[18],
                    trial.coordinates[19],
                    trial.coordinates[20],
                ]);
                activity.last_candidate_dry_temperatures = Some([
                    candidate.coordinates[0],
                    candidate.coordinates[1],
                    candidate.coordinates[2],
                    candidate.coordinates[6],
                    candidate.coordinates[7],
                    candidate.coordinates[8],
                    candidate.coordinates[12],
                    candidate.coordinates[14],
                    candidate.coordinates[15],
                    candidate.coordinates[16],
                    candidate.coordinates[17],
                    candidate.coordinates[18],
                    candidate.coordinates[19],
                    candidate.coordinates[20],
                ]);
                activity.last_current_wet_temperatures = Some([
                    base.occupancies[0].wet_temperature_k,
                    base.occupancies[1].wet_temperature_k,
                ]);
                activity.last_candidate_wet_temperatures = Some([
                    candidate_core.occupancies[0].wet_temperature_k,
                    candidate_core.occupancies[1].wet_temperature_k,
                ]);
                activity.last_current_beta = Some([
                    base.occupancies[0].sun.beta,
                    base.occupancies[0].shade.beta,
                    base.occupancies[1].sun.beta,
                    base.occupancies[1].shade.beta,
                ]);
                activity.last_candidate_beta = Some([
                    candidate_core.occupancies[0].sun.beta,
                    candidate_core.occupancies[0].shade.beta,
                    candidate_core.occupancies[1].sun.beta,
                    candidate_core.occupancies[1].shade.beta,
                ]);
                activity.last_current_ci_pa = Some([
                    base.occupancies[0].sun.ci_pa,
                    base.occupancies[0].shade.ci_pa,
                    base.occupancies[1].sun.ci_pa,
                    base.occupancies[1].shade.ci_pa,
                ]);
                activity.last_candidate_ci_pa = Some([
                    candidate_core.occupancies[0].sun.ci_pa,
                    candidate_core.occupancies[0].shade.ci_pa,
                    candidate_core.occupancies[1].sun.ci_pa,
                    candidate_core.occupancies[1].shade.ci_pa,
                ]);
            });
            #[cfg(test)]
            let step_norms = m1_test_full_governed_excess(norm, exponent, step_norms);
            #[cfg(test)]
            let step_norms = m1_test_step_poison(exponent, step_norms);
            if exponent == 0
                && current_complete_residuals_pass
                && step_norms.governed_threshold_exceeded()
            {
                full_trial_refusal =
                    Some(M1FullTrialNoUpdateRefusal::GovernedStepThresholdExceeded);
            }
            #[cfg(test)]
            if exponent > 0 {
                M1_HALVED_TRIAL_WITNESS_TRACE.with(|trace| {
                    trace.borrow_mut().push((exponent, is_first_halved_witness));
                });
            }
            #[cfg(test)]
            if exponent > 0 && current_complete_residuals_pass {
                eprintln!(
                    "m1-halved-decision iteration={iteration} b={exponent} full_refusal={full_trial_refusal:?} first_valid={is_first_halved_witness} steps={step_norms:?} no_update_enabled={}",
                    m1_test_no_update_enabled(),
                );
            }
            let no_update_witness = current_complete_residuals_pass
                && step_norms.governed_passes()
                && m1_test_no_update_enabled()
                && (exponent == 0 || (full_trial_refusal.is_some() && is_first_halved_witness));
            if no_update_witness {
                #[cfg(any(test, feature = "test-support"))]
                if diagnostic_active {
                    m1_diagnostic_trial(
                        iteration,
                        exponent,
                        factor,
                        &candidate,
                        Some(&candidate_core),
                        input,
                        "no_update_witness",
                    );
                }
                if exponent > 0 {
                    M1_RUNTIME_ACTIVITY.with(|activity| {
                        activity.borrow_mut().no_update_cumulative_backtracking =
                            Some(backtracking_count + exponent);
                    });
                }
                #[cfg(test)]
                M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow_mut().push((exponent, true)));
                #[cfg(test)]
                {
                    let witness_steps = step_norms.diagnostics();
                    m1_trace_before_materialization(input, &trial, &base, Some(&witness_steps));
                }
                return materialize_m1_coupled_column(input, &trial, base, None);
            }
            if candidate_norm < norm {
                #[cfg(any(test, feature = "test-support"))]
                if diagnostic_active {
                    m1_diagnostic_trial(
                        iteration,
                        exponent,
                        factor,
                        &candidate,
                        Some(&candidate_core),
                        input,
                        "accepted_strict_decrease",
                    );
                }
                M1_RUNTIME_ACTIVITY.with(|activity| {
                    let mut activity = activity.borrow_mut();
                    activity.installed_current_norm = Some(norm);
                    activity.installed_candidate_norm = Some(candidate_norm);
                    activity.installed_candidate_coordinates = Some(candidate.coordinates);
                    activity.installed_current_coordinates = Some(trial.coordinates);
                });
                #[cfg(test)]
                M1_HALVED_TRIAL_OUTCOME
                    .with(|outcome| outcome.borrow_mut().push((exponent, false)));
                #[cfg(test)]
                eprintln!(
                    "m1-accepted-newton-step {}",
                    serde_json::json!({
                        "iteration": iteration, "backtracking": exponent,
                        "coordinates": candidate.coordinates,
                        "raw_residuals": candidate_core.residuals,
                        "normalized_residuals": candidate_residuals,
                        "step_norms": { "temperature_k": step_norms.temperature_k, "humidity_kg_kg": step_norms.humidity_kg_kg, "ci_pa": step_norms.ci_pa, "hydraulic_mm": step_norms.hydraulic_mm, "beta": step_norms.beta },
                        "drainage": [candidate.coordinates[5], candidate.coordinates[11]],
                        "capacity": candidate_core.occupancies.iter().map(|value| value.capacity).collect::<Vec<_>>(),
                        "phases": [
                            M1PhaseReservoir::from_mass_enthalpy(candidate.coordinates[3], candidate.coordinates[4]).ok().map(|value| format!("{:?}", value.phase())),
                            M1PhaseReservoir::from_mass_enthalpy(candidate.coordinates[9], candidate.coordinates[10]).ok().map(|value| format!("{:?}", value.phase())),
                        ],
                    })
                );
                accepted = Some((candidate, exponent, step_norms.diagnostics()));
                break;
            }
            #[cfg(any(test, feature = "test-support"))]
            if diagnostic_active {
                m1_diagnostic_trial(
                    iteration,
                    exponent,
                    factor,
                    &candidate,
                    Some(&candidate_core),
                    input,
                    "evaluated_non_decrease",
                );
            }
        }
        let Some((candidate, exponent, installed_steps)) = accepted else {
            return Err(m1_failure(
                crate::numerics::NumericalFailureKind::BacktrackingLimit,
                iteration,
                input,
                &base,
                &trial,
                backtracking_count,
                StepNorms {
                    temperature_k: None,
                    humidity_kg_kg: None,
                    ci_pa: None,
                    hydraulic_mm: None,
                    beta: Some(0.0),
                },
                Some(phase_report.pivot),
                Some(phase_report.matrix_norm),
            ));
        };
        backtracking_count += exponent;
        #[cfg(test)]
        let installed_steps = m1_test_poison_installed_steps(installed_steps);
        last_installed_steps = Some(installed_steps);
        trial = candidate;
    }
    Err(domain_error())
}

#[cfg(test)]
pub fn m1_solve_with_halved_core_incomplete_for_test(
    input: &M1CoupledColumnInput,
    exponent: u32,
) -> (
    Result<M1CoupledEvaluation, M1CoupledError>,
    Vec<(u32, bool)>,
    Vec<(u32, bool)>,
) {
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(Some(exponent)));
    M1_HALVED_TRIAL_HYDRAULIC_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(true));
    M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_WITNESS_TRACE.with(|trace| trace.borrow_mut().clear());
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow_mut().clear());
    let result = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    let trace = M1_HALVED_TRIAL_WITNESS_TRACE.with(|trace| trace.borrow().clone());
    let outcome = M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow().clone());
    (result, trace, outcome)
}

#[cfg(test)]
pub fn m1_solve_with_halved_hydraulic_incomplete_for_test(
    input: &M1CoupledColumnInput,
    exponent: u32,
) -> (Vec<(u32, bool)>, Vec<(u32, bool)>) {
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_HYDRAULIC_INCOMPLETE_AT.with(|forced| forced.set(Some(exponent)));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(true));
    M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_WITNESS_TRACE.with(|trace| trace.borrow_mut().clear());
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow_mut().clear());
    let _ = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_HYDRAULIC_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    (
        M1_HALVED_TRIAL_WITNESS_TRACE.with(|trace| trace.borrow().clone()),
        M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow().clone()),
    )
}

#[cfg(test)]
pub(crate) fn m1_solve_with_early_first_witness_incomplete_for_test(
    input: &M1CoupledColumnInput,
) -> (Vec<(u32, bool)>, M1LoopActivityObservation) {
    m1_reset_loop_activity_observation_for_test();
    M1_HALVED_TRIAL_TARGET_ITERATION.with(|target| target.set(Some(0)));
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(Some(1)));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(true));
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow_mut().clear());
    let _ = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_TARGET_ITERATION.with(|target| target.set(None));
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    (
        M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow().clone()),
        m1_loop_activity_observation_for_test(),
    )
}

#[cfg(test)]
pub(crate) fn m1_solve_with_no_update_exponent_for_test(
    input: &M1CoupledColumnInput,
    exponent: u32,
) -> M1LoopActivityObservation {
    m1_reset_loop_activity_observation_for_test();
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(true));
    M1_HALVED_TRIAL_DOMAIN_INVALID_MASK.with(|mask| {
        mask.set(if exponent == 2 { 1 << 1 } else { 0 });
    });
    let _ = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_DOMAIN_INVALID_MASK.with(|mask| mask.set(0));
    m1_loop_activity_observation_for_test()
}

#[cfg(test)]
pub(crate) fn m1_solve_with_poisoned_installed_step_for_test(
    input: &M1CoupledColumnInput,
    component: u8,
) -> (
    Result<M1CoupledEvaluation, M1CoupledError>,
    M1LoopActivityObservation,
) {
    M1_INSTALLED_STEP_POISON.with(|poison| poison.set(Some(component)));
    M1_DISABLE_NO_UPDATE.with(|disabled| disabled.set(true));
    let result = solve_m1_coupled_column(input);
    M1_INSTALLED_STEP_POISON.with(|poison| poison.set(None));
    M1_DISABLE_NO_UPDATE.with(|disabled| disabled.set(false));
    (result, m1_loop_activity_observation_for_test())
}

#[cfg(test)]
pub fn m1_solve_with_full_witness_for_test(input: &M1CoupledColumnInput) -> Vec<(u32, bool)> {
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_HYDRAULIC_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_TARGET_ITERATION.with(|target| target.set(None));
    M1_HALVED_TRIAL_DOMAIN_INVALID_MASK.with(|mask| mask.set(0));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_STEP_POISON.with(|poison| poison.set(None));
    M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow_mut().clear());
    let _ = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow().clone())
}

#[cfg(test)]
pub fn m1_solve_with_full_governed_excess_for_test(
    input: &M1CoupledColumnInput,
) -> (
    Result<M1CoupledEvaluation, M1CoupledError>,
    Vec<(u32, bool)>,
) {
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(|forced| forced.set(true));
    M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_WITNESS_TRACE.with(|trace| trace.borrow_mut().clear());
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow_mut().clear());
    let result = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(|forced| forced.set(false));
    let outcome = M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow().clone());
    (result, outcome)
}

#[cfg(test)]
pub fn m1_solve_with_full_core_incomplete_for_test(
    input: &M1CoupledColumnInput,
) -> (
    Result<M1CoupledEvaluation, M1CoupledError>,
    Vec<(u32, bool)>,
) {
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(Some(0)));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_WITNESS_TRACE.with(|trace| trace.borrow_mut().clear());
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow_mut().clear());
    let result = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(None));
    let outcome = M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow().clone());
    (result, outcome)
}

#[cfg(test)]
pub fn m1_solve_with_nonfinite_current_residual_for_test(
    input: &M1CoupledColumnInput,
) -> Result<M1CoupledEvaluation, M1CoupledError> {
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE.with(|forced| forced.set(true));
    let result = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE.with(|forced| forced.set(false));
    result
}

#[cfg(test)]
pub fn m1_solve_with_first_halved_step_poison_for_test(
    input: &M1CoupledColumnInput,
    component: u8,
) -> Vec<(u32, bool)> {
    M1_HALVED_TRIAL_CORE_INCOMPLETE_AT.with(|forced| forced.set(None));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(true));
    M1_HALVED_TRIAL_FULL_GOVERNED_EXCESS.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_CURRENT_RESIDUAL_NONFINITE.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_STEP_POISON.with(|poison| poison.set(Some((1, component))));
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow_mut().clear());
    let _ = solve_m1_coupled_column(input);
    M1_HALVED_TRIAL_STEP_POISON.with(|poison| poison.set(None));
    M1_HALVED_TRIAL_FULL_DOMAIN_INVALID.with(|forced| forced.set(false));
    M1_HALVED_TRIAL_OUTCOME.with(|outcome| outcome.borrow().clone())
}
