#![allow(
    clippy::float_cmp,
    clippy::manual_let_else,
    clippy::needless_range_loop,
    clippy::single_match_else,
    clippy::struct_field_names,
    clippy::too_many_lines
)]
//! Exact V3 six-unknown Stage-A coupling driver.
//!
//! The caller supplies a complete gas/energy/hydraulic evaluation for every
//! trial state. This module owns the admitted residual identities, scaling,
//! beta bounds, centered finite differences, pivot rule, damping, and typed
//! failure payload. It cannot be used as a sequential hydraulic clamp.

use std::collections::BTreeSet;

use openwepp_kernel_contract::{OccupancyId, SoilLayerId, TransactionId};

use crate::diagnostics::{
    BoundIdentity, CoupledSolvePass, NormalizedResidual, NumericalFailureDiagnostics, SolveIdentity,
};
use crate::{MODEL_SHA256, VegetationError};

const MAX_ITERATIONS: u32 = 50;
const MAX_HALVINGS: u32 = 20;
const POTENTIAL_STEP_TOLERANCE_MM: f64 = 1.0e-7;
const WATER_ATOL: f64 = 1.0e-12;
const WATER_RTOL: f64 = 1.0e-9;
const UNIT_SCALES: [f64; 6] = [1_000.0, 1_000.0, 1_000.0, 1_000.0, 1.0, 1.0];
const RESIDUAL_IDENTITIES: [&str; 6] = [
    "sun_gas_minus_q1",
    "shade_gas_minus_q1",
    "sun_gas_minus_vulnerability_demand",
    "shade_gas_minus_vulnerability_demand",
    "q1_sum_minus_q2",
    "q3_sum_minus_q2",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct StageAState {
    pub psi_sunleaf_mm: f64,
    pub psi_shadeleaf_mm: f64,
    pub psi_stem_mm: f64,
    pub psi_root_mm: f64,
    pub beta_sun: f64,
    pub beta_shade: f64,
}

impl StageAState {
    fn array(self) -> [f64; 6] {
        [
            self.psi_sunleaf_mm,
            self.psi_shadeleaf_mm,
            self.psi_stem_mm,
            self.psi_root_mm,
            self.beta_sun,
            self.beta_shade,
        ]
    }

    fn from_array(value: [f64; 6]) -> Self {
        Self {
            psi_sunleaf_mm: value[0],
            psi_shadeleaf_mm: value[1],
            psi_stem_mm: value[2],
            psi_root_mm: value[3],
            beta_sun: value[4],
            beta_shade: value[5],
        }
    }
}

/// Complete flux operands produced by one nested gas/energy/hydraulic trial.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StageAEvaluation {
    pub emax_sun_kg_m2_s: f64,
    pub emax_shade_kg_m2_s: f64,
    pub gas_sun_kg_m2_s: f64,
    pub gas_shade_kg_m2_s: f64,
    pub vulnerability_demand_sun_kg_m2_s: f64,
    pub vulnerability_demand_shade_kg_m2_s: f64,
    pub q1_sun_kg_m2_s: f64,
    pub q1_shade_kg_m2_s: f64,
    pub q2_kg_m2_s: f64,
    pub q3_kg_m2_s: Vec<(SoilLayerId, f64)>,
}

pub(crate) trait StageAEvaluator {
    fn evaluate(&self, state: StageAState) -> Result<StageAEvaluation, VegetationError>;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StageASolution {
    pub state: StageAState,
    pub persisted_beta_hyd: f64,
    pub evaluation: StageAEvaluation,
    pub iterations: u32,
    pub backtracking_count: u32,
    pub normalized_residuals: Vec<NormalizedResidual>,
    pub potential_step_mm: f64,
    pub pivot_magnitude: f64,
    pub matrix_norm: f64,
}

#[derive(Clone, Debug)]
pub(crate) struct StageASolveIdentity {
    pub transaction_id: TransactionId,
    pub occupancy_id: OccupancyId,
}

pub(crate) fn solve_uncapped_stage_a(
    identity: &StageASolveIdentity,
    initial: StageAState,
    evaluator: &dyn StageAEvaluator,
) -> Result<StageASolution, VegetationError> {
    validate_state(initial)?;
    let initial_evaluation = evaluator
        .evaluate(initial)
        .map_err(|error| wrap_evaluator_error(identity, error, 0))?;
    validate_evaluation(&initial_evaluation)
        .map_err(|error| wrap_evaluator_error(identity, error, 0))?;
    if initial_evaluation.emax_sun_kg_m2_s == 0.0 && initial_evaluation.emax_shade_kg_m2_s == 0.0 {
        let mut state = initial;
        state.beta_sun = 1.0;
        state.beta_shade = 1.0;
        let evaluation = evaluator
            .evaluate(state)
            .map_err(|error| wrap_evaluator_error(identity, error, 0))?;
        validate_evaluation(&evaluation)
            .map_err(|error| wrap_evaluator_error(identity, error, 0))?;
        validate_accepted_fluxes(&evaluation)?;
        if residuals(&evaluation).iter().any(|value| *value != 0.0) {
            return Err(failure(
                identity,
                0,
                &evaluation,
                None,
                0,
                None,
                None,
                &state,
                SolveIdentity::OuterGasEnergyHydraulicCoupling,
            ));
        }
        return Ok(StageASolution {
            state,
            persisted_beta_hyd: 1.0,
            evaluation,
            iterations: 0,
            backtracking_count: 0,
            normalized_residuals: zero_residuals(),
            potential_step_mm: 0.0,
            pivot_magnitude: 0.0,
            matrix_norm: 0.0,
        });
    }

    let mut x = initial.array();
    let mut evaluation = initial_evaluation;
    let mut backtracks = 0;
    let mut last_step = None;
    let mut last_pivot = None;
    let mut last_matrix_norm = None;
    for iteration in 0..=MAX_ITERATIONS {
        let raw = residuals(&evaluation);
        let scale = water_scale(&evaluation);
        let normalized = normalize(&raw, scale);
        if raw
            .iter()
            .chain(normalized.iter())
            .any(|value| !value.is_finite())
        {
            return Err(failure(
                identity,
                iteration,
                &evaluation,
                last_step,
                backtracks,
                last_pivot,
                last_matrix_norm,
                &StageAState::from_array(x),
                SolveIdentity::OuterGasEnergyHydraulicCoupling,
            ));
        }
        let norm = infinity_norm(&normalized);
        if norm <= 1.0 && last_step.is_none_or(|step| step <= POTENTIAL_STEP_TOLERANCE_MM) {
            let state = StageAState::from_array(x);
            validate_accepted_fluxes(&evaluation)?;
            return Ok(StageASolution {
                state,
                persisted_beta_hyd: persisted_beta(&evaluation, state)?,
                evaluation,
                iterations: iteration,
                backtracking_count: backtracks,
                normalized_residuals: labeled(&normalized),
                potential_step_mm: last_step.unwrap_or(0.0),
                pivot_magnitude: last_pivot.unwrap_or(0.0),
                matrix_norm: last_matrix_norm.unwrap_or(0.0),
            });
        }
        if iteration == MAX_ITERATIONS {
            return Err(failure(
                identity,
                iteration,
                &evaluation,
                last_step,
                backtracks,
                last_pivot,
                last_matrix_norm,
                &StageAState::from_array(x),
                SolveIdentity::OuterGasEnergyHydraulicCoupling,
            ));
        }
        let jacobian = centered_jacobian(&x, evaluator).map_err(|error| match error {
            VegetationError::NumericalFailure(_) => error,
            _ => failure(
                identity,
                iteration,
                &evaluation,
                last_step,
                backtracks,
                None,
                None,
                &StageAState::from_array(x),
                SolveIdentity::OuterGasEnergyHydraulicCoupling,
            ),
        })?;
        let matrix_norm = matrix_infinity_norm(&jacobian);
        if !matrix_norm.is_finite() {
            return Err(failure(
                identity,
                iteration,
                &evaluation,
                last_step,
                backtracks,
                None,
                None,
                &StageAState::from_array(x),
                SolveIdentity::HydraulicSystem,
            ));
        }
        let (delta, pivot) = solve_pivoted(jacobian, raw.map(|value| -value), matrix_norm)
            .map_err(|pivot| {
                failure(
                    identity,
                    iteration,
                    &evaluation,
                    None,
                    backtracks,
                    Some(pivot),
                    Some(matrix_norm),
                    &StageAState::from_array(x),
                    SolveIdentity::HydraulicSystem,
                )
            })?;
        last_pivot = Some(pivot);
        last_matrix_norm = Some(matrix_norm);
        let full_potential_step = delta[..4].iter().copied().map(f64::abs).fold(0.0, f64::max);
        if norm <= 1.0 && full_potential_step <= POTENTIAL_STEP_TOLERANCE_MM {
            last_step = Some(full_potential_step);
            continue;
        }
        let mut accepted = None;
        let mut last_trial_error = None;
        for half in 0..=MAX_HALVINGS {
            let factor = 2.0_f64.powi(
                -i32::try_from(half)
                    .map_err(|_| VegetationError::Domain("V3 Stage-A backtracking count"))?,
            );
            let mut trial = x;
            for index in 0..6 {
                trial[index] += factor * delta[index];
            }
            if !(0.0..=1.0).contains(&trial[4]) || !(0.0..=1.0).contains(&trial[5]) {
                backtracks += 1;
                continue;
            }
            let trial_state = StageAState::from_array(trial);
            let trial_evaluation = match evaluator.evaluate(trial_state) {
                Ok(value) => value,
                Err(error) => {
                    last_trial_error = Some(error);
                    backtracks += 1;
                    continue;
                }
            };
            validate_evaluation(&trial_evaluation).map_err(|_| {
                failure(
                    identity,
                    iteration,
                    &evaluation,
                    last_step,
                    backtracks,
                    last_pivot,
                    last_matrix_norm,
                    &StageAState::from_array(x),
                    SolveIdentity::OuterGasEnergyHydraulicCoupling,
                )
            })?;
            let trial_norm = infinity_norm(&normalize(
                &residuals(&trial_evaluation),
                water_scale(&trial_evaluation),
            ));
            if trial_norm < norm {
                accepted = Some((trial, trial_evaluation, factor));
                break;
            }
            backtracks += 1;
        }
        let Some((next, next_evaluation, factor)) = accepted else {
            if let Some(error) = last_trial_error {
                return Err(match error {
                    VegetationError::NumericalFailure(_) => error,
                    _ => failure(
                        identity,
                        iteration,
                        &evaluation,
                        last_step,
                        backtracks,
                        last_pivot,
                        last_matrix_norm,
                        &StageAState::from_array(x),
                        SolveIdentity::HydraulicSystem,
                    ),
                });
            }
            return Err(failure(
                identity,
                iteration,
                &evaluation,
                last_step,
                backtracks,
                last_pivot,
                last_matrix_norm,
                &StageAState::from_array(x),
                SolveIdentity::HydraulicSystem,
            ));
        };
        last_step = Some(
            delta[..4]
                .iter()
                .map(|value| (factor * value).abs())
                .fold(0.0, f64::max),
        );
        x = next;
        evaluation = next_evaluation;
    }
    unreachable!("bounded Stage-A loop")
}

fn validate_state(state: StageAState) -> Result<(), VegetationError> {
    let values = state.array();
    if values.iter().any(|value| !value.is_finite())
        || !(0.0..=1.0).contains(&state.beta_sun)
        || !(0.0..=1.0).contains(&state.beta_shade)
    {
        return Err(VegetationError::Domain("V3 Stage-A state"));
    }
    Ok(())
}

fn validate_evaluation(value: &StageAEvaluation) -> Result<(), VegetationError> {
    let scalars = [
        value.emax_sun_kg_m2_s,
        value.emax_shade_kg_m2_s,
        value.gas_sun_kg_m2_s,
        value.gas_shade_kg_m2_s,
        value.vulnerability_demand_sun_kg_m2_s,
        value.vulnerability_demand_shade_kg_m2_s,
        value.q1_sun_kg_m2_s,
        value.q1_shade_kg_m2_s,
        value.q2_kg_m2_s,
    ];
    let layer_ids = value
        .q3_kg_m2_s
        .iter()
        .map(|(layer_id, _)| layer_id)
        .collect::<BTreeSet<_>>();
    if value.q3_kg_m2_s.is_empty()
        || layer_ids.len() != value.q3_kg_m2_s.len()
        || scalars.iter().any(|item| !item.is_finite())
        || value.q3_kg_m2_s.iter().any(|(_, item)| !item.is_finite())
    {
        return Err(VegetationError::Domain("V3 Stage-A flux operands"));
    }
    Ok(())
}

fn validate_accepted_fluxes(value: &StageAEvaluation) -> Result<(), VegetationError> {
    if [
        value.emax_sun_kg_m2_s,
        value.emax_shade_kg_m2_s,
        value.gas_sun_kg_m2_s,
        value.gas_shade_kg_m2_s,
        value.vulnerability_demand_sun_kg_m2_s,
        value.vulnerability_demand_shade_kg_m2_s,
        value.q1_sun_kg_m2_s,
        value.q1_shade_kg_m2_s,
        value.q2_kg_m2_s,
    ]
    .iter()
    .any(|flux| *flux < 0.0)
        || value.q3_kg_m2_s.iter().any(|(_, flux)| *flux < 0.0)
    {
        return Err(VegetationError::Hydraulic(
            "hydraulic redistribution unsupported",
        ));
    }
    Ok(())
}

fn residuals(value: &StageAEvaluation) -> [f64; 6] {
    let q3_sum = value
        .q3_kg_m2_s
        .iter()
        .try_fold(0.0, |sum, (_, flux)| {
            let next = sum + flux;
            next.is_finite().then_some(next)
        })
        .unwrap_or(f64::NAN);
    [
        value.q1_sun_kg_m2_s - value.gas_sun_kg_m2_s,
        value.q1_shade_kg_m2_s - value.gas_shade_kg_m2_s,
        value.gas_sun_kg_m2_s - value.vulnerability_demand_sun_kg_m2_s,
        value.gas_shade_kg_m2_s - value.vulnerability_demand_shade_kg_m2_s,
        value.q2_kg_m2_s - value.q1_sun_kg_m2_s - value.q1_shade_kg_m2_s,
        q3_sum - value.q2_kg_m2_s,
    ]
}

fn water_scale(value: &StageAEvaluation) -> f64 {
    [
        WATER_ATOL,
        value.emax_sun_kg_m2_s.abs(),
        value.emax_shade_kg_m2_s.abs(),
        value.q1_sun_kg_m2_s.abs(),
        value.q1_shade_kg_m2_s.abs(),
        value.q2_kg_m2_s.abs(),
        value
            .q3_kg_m2_s
            .iter()
            .map(|(_, flux)| *flux)
            .map(f64::abs)
            .fold(0.0, f64::max),
    ]
    .into_iter()
    .fold(WATER_ATOL, f64::max)
}

fn normalize(raw: &[f64; 6], scale: f64) -> [f64; 6] {
    let tolerance = WATER_ATOL + WATER_RTOL * scale;
    raw.map(|value| value / tolerance)
}

fn infinity_norm(values: &[f64; 6]) -> f64 {
    values.iter().copied().map(f64::abs).fold(0.0, f64::max)
}

fn labeled(values: &[f64; 6]) -> Vec<NormalizedResidual> {
    RESIDUAL_IDENTITIES
        .iter()
        .zip(values)
        .map(|(identity, value)| NormalizedResidual {
            identity: (*identity).into(),
            value: *value,
        })
        .collect()
}

fn zero_residuals() -> Vec<NormalizedResidual> {
    labeled(&[0.0; 6])
}

fn persisted_beta(value: &StageAEvaluation, state: StageAState) -> Result<f64, VegetationError> {
    let scale = value.emax_sun_kg_m2_s.max(value.emax_shade_kg_m2_s);
    let beta = if scale == 0.0 {
        1.0
    } else {
        let sun_weight = value.emax_sun_kg_m2_s / scale;
        let shade_weight = value.emax_shade_kg_m2_s / scale;
        (state.beta_sun * sun_weight + state.beta_shade * shade_weight)
            / (sun_weight + shade_weight)
    };
    if beta.is_finite() && (0.0..=1.0).contains(&beta) {
        Ok(beta)
    } else {
        Err(VegetationError::Domain("V3 persisted beta_hyd"))
    }
}

fn centered_jacobian(
    x: &[f64; 6],
    evaluator: &dyn StageAEvaluator,
) -> Result<[[f64; 6]; 6], VegetationError> {
    let mut jacobian = [[0.0; 6]; 6];
    for column in 0..6 {
        let step = f64::EPSILON.sqrt() * x[column].abs().max(UNIT_SCALES[column]);
        let mut plus = *x;
        let mut minus = *x;
        plus[column] += step;
        minus[column] -= step;
        let plus_evaluation = evaluator.evaluate(StageAState::from_array(plus))?;
        let minus_evaluation = evaluator.evaluate(StageAState::from_array(minus))?;
        validate_evaluation(&plus_evaluation)?;
        validate_evaluation(&minus_evaluation)?;
        let rp = residuals(&plus_evaluation);
        let rm = residuals(&minus_evaluation);
        for row in 0..6 {
            jacobian[row][column] = (rp[row] - rm[row]) / (2.0 * step);
        }
    }
    Ok(jacobian)
}

fn matrix_infinity_norm(matrix: &[[f64; 6]; 6]) -> f64 {
    matrix
        .iter()
        .map(|row| row.iter().copied().map(f64::abs).sum::<f64>())
        .fold(0.0, f64::max)
}

fn solve_pivoted(
    mut matrix: [[f64; 6]; 6],
    mut rhs: [f64; 6],
    matrix_norm: f64,
) -> Result<([f64; 6], f64), f64> {
    let threshold = 64.0 * f64::EPSILON * matrix_norm;
    let mut minimum_pivot = f64::INFINITY;
    for column in 0..6 {
        let pivot_row = (column..6)
            .max_by(|left, right| {
                matrix[*left][column]
                    .abs()
                    .total_cmp(&matrix[*right][column].abs())
            })
            .unwrap_or(column);
        let pivot = matrix[pivot_row][column].abs();
        minimum_pivot = minimum_pivot.min(pivot);
        if !pivot.is_finite() || pivot < threshold {
            return Err(pivot);
        }
        matrix.swap(column, pivot_row);
        rhs.swap(column, pivot_row);
        for row in column + 1..6 {
            let factor = matrix[row][column] / matrix[column][column];
            for index in column..6 {
                matrix[row][index] -= factor * matrix[column][index];
            }
            rhs[row] -= factor * rhs[column];
            if matrix[row][column..].iter().any(|value| !value.is_finite()) || !rhs[row].is_finite()
            {
                return Err(f64::NAN);
            }
        }
    }
    let mut solution = [0.0; 6];
    for row in (0..6).rev() {
        let known = (row + 1..6)
            .map(|column| matrix[row][column] * solution[column])
            .sum::<f64>();
        solution[row] = (rhs[row] - known) / matrix[row][row];
    }
    Ok((solution, minimum_pivot))
}

#[allow(clippy::too_many_arguments)]
fn failure(
    identity: &StageASolveIdentity,
    iterations: u32,
    evaluation: &StageAEvaluation,
    step_norm: Option<f64>,
    backtracking_count: u32,
    pivot_magnitude: Option<f64>,
    matrix_norm: Option<f64>,
    state: &StageAState,
    solve: SolveIdentity,
) -> VegetationError {
    let normalized = normalize(&residuals(evaluation), water_scale(evaluation));
    let mut active_bounds = Vec::new();
    if state.beta_sun == 0.0 || state.beta_sun == 1.0 {
        active_bounds.push(BoundIdentity("beta_sun".into()));
    }
    if state.beta_shade == 0.0 || state.beta_shade == 1.0 {
        active_bounds.push(BoundIdentity("beta_shade".into()));
    }
    let residual_norms = if normalized.iter().all(|value| value.is_finite()) {
        labeled(&normalized)
    } else {
        Vec::new()
    };
    let diagnostics = NumericalFailureDiagnostics {
        model_definition_sha256: MODEL_SHA256.into(),
        transaction_id: identity.transaction_id,
        occupancy_id: identity.occupancy_id.clone(),
        pass: CoupledSolvePass::Potential,
        solve,
        iterations,
        residual_norms,
        step_norm: step_norm.filter(|value| value.is_finite()),
        backtracking_count,
        active_bounds,
        active_water_caps: Vec::new(),
        bracket: None,
        pivot_magnitude: pivot_magnitude.filter(|value| value.is_finite()),
        matrix_norm: matrix_norm.filter(|value| value.is_finite()),
    };
    debug_assert!(diagnostics.validate().is_ok());
    VegetationError::NumericalFailure(Box::new(diagnostics))
}

fn wrap_evaluator_error(
    identity: &StageASolveIdentity,
    error: VegetationError,
    iterations: u32,
) -> VegetationError {
    if matches!(error, VegetationError::NumericalFailure(_)) {
        return error;
    }
    VegetationError::NumericalFailure(Box::new(NumericalFailureDiagnostics {
        model_definition_sha256: MODEL_SHA256.into(),
        transaction_id: identity.transaction_id,
        occupancy_id: identity.occupancy_id.clone(),
        pass: CoupledSolvePass::Potential,
        solve: SolveIdentity::OuterGasEnergyHydraulicCoupling,
        iterations,
        residual_norms: Vec::new(),
        step_norm: None,
        backtracking_count: 0,
        active_bounds: Vec::new(),
        active_water_caps: Vec::new(),
        bracket: None,
        pivot_magnitude: None,
        matrix_norm: None,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_kernel_contract::{StratumId, TileId};

    fn layer(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("layer")
    }

    struct LinearOracle;

    impl StageAEvaluator for LinearOracle {
        fn evaluate(&self, state: StageAState) -> Result<StageAEvaluation, VegetationError> {
            let emax_sun = 4.0e-5;
            let emax_shade = 2.0e-5;
            let gas_sun = state.beta_sun * emax_sun;
            let gas_shade = state.beta_shade * emax_shade;
            let q1_sun = 1.0e-9 * (state.psi_stem_mm - state.psi_sunleaf_mm);
            let q1_shade = 1.0e-9 * (state.psi_stem_mm - state.psi_shadeleaf_mm);
            let q2 = 1.0e-9 * (state.psi_root_mm - state.psi_stem_mm - 1_000.0);
            let q3_total = 1.0e-9 * (10_000.0 - state.psi_root_mm);
            Ok(StageAEvaluation {
                emax_sun_kg_m2_s: emax_sun,
                emax_shade_kg_m2_s: emax_shade,
                gas_sun_kg_m2_s: gas_sun,
                gas_shade_kg_m2_s: gas_shade,
                vulnerability_demand_sun_kg_m2_s: emax_sun * 0.25,
                vulnerability_demand_shade_kg_m2_s: emax_shade * 0.5,
                q1_sun_kg_m2_s: q1_sun,
                q1_shade_kg_m2_s: q1_shade,
                q2_kg_m2_s: q2,
                q3_kg_m2_s: vec![
                    (layer("soil-1"), q3_total * 0.6),
                    (layer("soil-2"), q3_total * 0.4),
                ],
            })
        }
    }

    fn identity() -> StageASolveIdentity {
        StageASolveIdentity {
            transaction_id: TransactionId(17),
            occupancy_id: OccupancyId {
                stratum_id: StratumId::try_new("upper").expect("stratum"),
                tile_id: TileId::try_new("tile-a").expect("tile"),
            },
        }
    }

    #[test]
    fn exact_six_residual_driver_solves_distinct_class_betas() {
        let solved = solve_uncapped_stage_a(
            &identity(),
            StageAState {
                psi_sunleaf_mm: -16_000.0,
                psi_shadeleaf_mm: -12_000.0,
                psi_stem_mm: -6_000.0,
                psi_root_mm: 6_000.0,
                beta_sun: 0.4,
                beta_shade: 0.4,
            },
            &LinearOracle,
        )
        .expect("coupled solution");
        assert!((solved.state.beta_sun - 0.25).abs() < 1.0e-10);
        assert!((solved.state.beta_shade - 0.5).abs() < 1.0e-10);
        assert!((solved.persisted_beta_hyd - 1.0 / 3.0).abs() < 1.0e-10);
        assert!(
            solved
                .normalized_residuals
                .iter()
                .all(|value| value.value.abs() <= 1.0)
        );
    }

    #[test]
    fn initially_converged_state_returns_without_jacobian() {
        let solved = solve_uncapped_stage_a(
            &identity(),
            StageAState {
                psi_sunleaf_mm: -41_000.0,
                psi_shadeleaf_mm: -41_000.0,
                psi_stem_mm: -31_000.0,
                psi_root_mm: -10_000.0,
                beta_sun: 0.25,
                beta_shade: 0.5,
            },
            &LinearOracle,
        )
        .expect("initial exact solution");
        assert_eq!(solved.iterations, 0);
        assert_eq!(solved.potential_step_mm.to_bits(), 0.0_f64.to_bits());
    }

    struct ZeroOracle;
    impl StageAEvaluator for ZeroOracle {
        fn evaluate(&self, _: StageAState) -> Result<StageAEvaluation, VegetationError> {
            Ok(StageAEvaluation {
                emax_sun_kg_m2_s: 0.0,
                emax_shade_kg_m2_s: 0.0,
                gas_sun_kg_m2_s: 0.0,
                gas_shade_kg_m2_s: 0.0,
                vulnerability_demand_sun_kg_m2_s: 0.0,
                vulnerability_demand_shade_kg_m2_s: 0.0,
                q1_sun_kg_m2_s: 0.0,
                q1_shade_kg_m2_s: 0.0,
                q2_kg_m2_s: 0.0,
                q3_kg_m2_s: vec![(layer("soil-1"), 0.0), (layer("soil-2"), 0.0)],
            })
        }
    }

    #[test]
    fn exact_zero_maximum_branch_sets_betas_without_division() {
        let solution = solve_uncapped_stage_a(
            &identity(),
            StageAState {
                psi_sunleaf_mm: -1.0,
                psi_shadeleaf_mm: -1.0,
                psi_stem_mm: -1.0,
                psi_root_mm: -1.0,
                beta_sun: 0.2,
                beta_shade: 0.3,
            },
            &ZeroOracle,
        )
        .expect("zero branch");
        assert_eq!(solution.state.beta_sun, 1.0);
        assert_eq!(solution.state.beta_shade, 1.0);
        assert_eq!(solution.persisted_beta_hyd, 1.0);
    }
}
