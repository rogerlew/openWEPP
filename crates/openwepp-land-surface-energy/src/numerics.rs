//! Deterministic nonlinear solve infrastructure shared by open and covered
//! land-surface-energy systems.

use crate::{LandSurfaceEnergyError, NormalizedResidual, StepNorms};

pub(crate) const MAX_NEWTON_ITERATIONS: u32 = 50;
pub(crate) const MAX_BACKTRACKING_HALVINGS: u32 = 20;
const TEMPERATURE_STEP_TOLERANCE_K: f64 = 1.0e-8;
const PIVOT_MULTIPLIER: f64 = 64.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumericalFailureKind {
    SingularPivot,
    BacktrackingLimit,
    IterationLimit,
}

#[derive(Clone, PartialEq)]
pub struct NumericalFailure {
    pub kind: NumericalFailureKind,
    pub iterations: u32,
    pub normalized_residuals: Vec<f64>,
    pub ordered_residuals: Vec<NormalizedResidual>,
    pub(crate) failed_solution: Vec<f64>,
    pub occupancy_id: Option<String>,
    pub active_bounds: Vec<String>,
    pub backtracking_count: u32,
    pub step_norms: StepNorms,
    pub pivot_magnitude: Option<f64>,
    pub matrix_norm: Option<f64>,
}

impl std::fmt::Debug for NumericalFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("NumericalFailure")
            .field("kind", &self.kind)
            .field("iterations", &self.iterations)
            .field("normalized_residuals", &self.normalized_residuals)
            .field("ordered_residuals", &self.ordered_residuals)
            .field("occupancy_id", &self.occupancy_id)
            .field("active_bounds", &self.active_bounds)
            .field("backtracking_count", &self.backtracking_count)
            .field("step_norms", &self.step_norms)
            .field("pivot_magnitude", &self.pivot_magnitude)
            .field("matrix_norm", &self.matrix_norm)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NormalizedSolveOutcome<D> {
    Accepted {
        solution: Vec<f64>,
        detail: D,
        iterations: u32,
        residual_norm_history: Vec<f64>,
        backtracking_count: u32,
        step_norm: f64,
        pivot_magnitude: Option<f64>,
        matrix_norm: Option<f64>,
    },
    Rejected(NumericalFailure),
}

#[derive(Debug)]
pub(crate) struct SingularEvidence {
    pub(crate) pivot: f64,
    pub(crate) matrix_norm: f64,
}

pub(crate) fn solve_linear(
    matrix: &[Vec<f64>],
    rhs: &[f64],
) -> Result<(Vec<f64>, f64, f64), SingularEvidence> {
    let n = rhs.len();
    let matrix_norm = matrix
        .iter()
        .map(|row| row.iter().map(|value| value.abs()).sum::<f64>())
        .fold(0.0, f64::max);
    let threshold = PIVOT_MULTIPLIER * f64::EPSILON * matrix_norm;
    let mut a = matrix.to_vec();
    let mut b = rhs.to_vec();
    let mut minimum_pivot = f64::INFINITY;
    for column in 0..n {
        let mut pivot_row = column;
        for row in column + 1..n {
            if a[row][column].abs() > a[pivot_row][column].abs() {
                pivot_row = row;
            }
        }
        let pivot = a[pivot_row][column].abs();
        minimum_pivot = minimum_pivot.min(pivot);
        if !pivot.is_finite() || pivot < threshold {
            return Err(SingularEvidence { pivot, matrix_norm });
        }
        if pivot_row != column {
            a.swap(column, pivot_row);
            b.swap(column, pivot_row);
        }
        for row in column + 1..n {
            let factor = a[row][column] / a[column][column];
            a[row][column] = 0.0;
            let pivot_tail = a[column][column + 1..].to_vec();
            for (entry, pivot_entry) in a[row][column + 1..].iter_mut().zip(pivot_tail) {
                *entry -= factor * pivot_entry;
            }
            b[row] -= factor * b[column];
        }
    }
    let mut solution = vec![0.0; n];
    for row in (0..n).rev() {
        let tail: f64 = a[row][row + 1..]
            .iter()
            .zip(solution[row + 1..].iter())
            .map(|(coefficient, value)| coefficient * value)
            .sum();
        solution[row] = (b[row] - tail) / a[row][row];
    }
    Ok((solution, minimum_pivot, matrix_norm))
}

pub(crate) fn normalized_infinity_norm(residuals: &[f64]) -> f64 {
    residuals
        .iter()
        .map(|value| value.abs())
        .fold(0.0, f64::max)
}

pub(crate) fn is_strict_residual_decrease(current_norm: f64, trial_residuals: &[f64]) -> bool {
    normalized_infinity_norm(trial_residuals) < current_norm
}

fn backtracked_trial<D, B, E, V>(
    evaluator: &mut E,
    valid_trial: &mut V,
    x: &[f64],
    delta: &[f64],
    current_norm: f64,
    prospective_step: f64,
) -> Option<(Vec<f64>, f64, u32)>
where
    E: FnMut(&[f64], Option<&B>) -> Result<(Vec<f64>, D), LandSurfaceEnergyError>,
    V: FnMut(&[f64]) -> bool,
{
    for exponent in 0..=MAX_BACKTRACKING_HALVINGS {
        let factor = 0.5_f64.powf(f64::from(exponent));
        let trial: Vec<f64> = x
            .iter()
            .zip(delta)
            .map(|(value, change)| value + factor * change)
            .collect();
        if !valid_trial(&trial) {
            continue;
        }
        let Ok((trial_residual, _)) = evaluator(&trial, None) else {
            continue;
        };
        if is_strict_residual_decrease(current_norm, &trial_residual) {
            return Some((trial, factor * prospective_step, exponent));
        }
    }
    None
}

fn bounded_jacobian<D, B, E, V>(
    evaluator: &mut E,
    valid_trial: &mut V,
    x: &[f64],
    current_residual: &[f64],
    unit_scales: &[f64],
    frozen: &B,
) -> Result<Vec<Vec<f64>>, LandSurfaceEnergyError>
where
    E: FnMut(&[f64], Option<&B>) -> Result<(Vec<f64>, D), LandSurfaceEnergyError>,
    V: FnMut(&[f64]) -> bool,
{
    if !valid_trial(x) || current_residual.len() != x.len() {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "normalized_jacobian_current_domain",
        ));
    }
    let perturbations: Vec<f64> = x
        .iter()
        .zip(unit_scales)
        .map(|(value, scale)| f64::EPSILON.sqrt() * value.abs().max(*scale))
        .collect();
    let mut jacobian = vec![vec![0.0; x.len()]; x.len()];
    for column in 0..x.len() {
        let mut minus = x.to_vec();
        let mut plus = x.to_vec();
        minus[column] -= perturbations[column];
        plus[column] += perturbations[column];
        let minus_valid = valid_trial(&minus);
        let plus_valid = valid_trial(&plus);
        let minus_residual = minus_valid
            .then(|| evaluator(&minus, Some(frozen)))
            .transpose()?
            .map(|value| value.0);
        let plus_residual = plus_valid
            .then(|| evaluator(&plus, Some(frozen)))
            .transpose()?
            .map(|value| value.0);
        if !minus_valid && !plus_valid {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "normalized_jacobian_bound",
            ));
        }
        for row in 0..x.len() {
            jacobian[row][column] = match (&minus_residual, &plus_residual) {
                (Some(minus), Some(plus)) => {
                    (plus[row] - minus[row]) / (2.0 * perturbations[column])
                }
                (Some(minus), None) => (current_residual[row] - minus[row]) / perturbations[column],
                (None, Some(plus)) => (plus[row] - current_residual[row]) / perturbations[column],
                (None, None) => {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "normalized_jacobian_bound",
                    ));
                }
            };
        }
    }
    Ok(jacobian)
}

fn rejected<D>(
    kind: NumericalFailureKind,
    iterations: u32,
    normalized_residuals: Vec<f64>,
    backtracking_count: u32,
    step: Option<f64>,
    evidence: (Option<f64>, Option<f64>),
    failed_solution: Vec<f64>,
) -> NormalizedSolveOutcome<D> {
    NormalizedSolveOutcome::Rejected(NumericalFailure {
        kind,
        iterations,
        normalized_residuals,
        ordered_residuals: Vec::new(),
        failed_solution,
        occupancy_id: None,
        active_bounds: Vec::new(),
        backtracking_count,
        step_norms: StepNorms {
            temperature_k: step,
            humidity_kg_kg: None,
            ci_pa: None,
            hydraulic_mm: None,
            beta: None,
        },
        pivot_magnitude: evidence.0,
        matrix_norm: evidence.1,
    })
}

fn validate_solver_shape(
    initial: &[f64],
    unit_scales: &[f64],
) -> Result<(), LandSurfaceEnergyError> {
    if initial.len() == unit_scales.len() && !initial.is_empty() {
        Ok(())
    } else {
        Err(LandSurfaceEnergyError::topology_domain(
            "normalized_solver_shape",
        ))
    }
}

fn unreachable_solver_state<D>() -> Result<NormalizedSolveOutcome<D>, LandSurfaceEnergyError> {
    Err(LandSurfaceEnergyError::ConstitutiveDomain(
        "unreachable_normalized_solver_state",
    ))
}

/// Frozen centered-difference Newton algorithm shared by open and joint columns.
///
/// # Errors
///
/// Returns a typed domain error when shapes or residual evaluations are invalid.
pub fn solve_normalized_system<D, B, E, V, F>(
    evaluator: E,
    initial: Vec<f64>,
    unit_scales: &[f64],
    valid_trial: V,
    freeze_branches: F,
) -> Result<NormalizedSolveOutcome<D>, LandSurfaceEnergyError>
where
    D: Clone,
    E: FnMut(&[f64], Option<&B>) -> Result<(Vec<f64>, D), LandSurfaceEnergyError>,
    V: FnMut(&[f64]) -> bool,
    F: FnMut(&D) -> B,
    B: Clone,
{
    solve_normalized_system_with_adjustment(
        evaluator,
        initial,
        unit_scales,
        valid_trial,
        freeze_branches,
        |_: &[f64],
         _: &D,
         _: &[f64],
         _: &mut [Vec<f64>],
         _: &mut [f64]|
         -> Result<(), LandSurfaceEnergyError> { Ok(()) },
    )
}

pub(crate) fn solve_normalized_system_with_adjustment<D, B, E, V, F, A>(
    mut evaluator: E,
    initial: Vec<f64>,
    unit_scales: &[f64],
    mut valid_trial: V,
    mut freeze_branches: F,
    mut adjust_linear_system: A,
) -> Result<NormalizedSolveOutcome<D>, LandSurfaceEnergyError>
where
    D: Clone,
    E: FnMut(&[f64], Option<&B>) -> Result<(Vec<f64>, D), LandSurfaceEnergyError>,
    V: FnMut(&[f64]) -> bool,
    F: FnMut(&D) -> B,
    A: FnMut(&[f64], &D, &[f64], &mut [Vec<f64>], &mut [f64]) -> Result<(), LandSurfaceEnergyError>,
    B: Clone,
{
    validate_solver_shape(&initial, unit_scales)?;
    let mut x = initial;
    let mut last_step = None;
    let mut backtracking_count = 0;
    let mut pivot = None;
    let mut matrix_norm = None;
    let mut history = Vec::new();
    for iteration in 0..=MAX_NEWTON_ITERATIONS {
        let (normalized, detail) = evaluator(&x, None)?;
        if normalized.len() != x.len() || normalized.iter().any(|value| !value.is_finite()) {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "normalized_residual_shape_or_domain",
            ));
        }
        let norm = normalized_infinity_norm(&normalized);
        history.push(norm);
        if norm <= 1.0 && last_step.is_some_and(|step| step <= TEMPERATURE_STEP_TOLERANCE_K) {
            return Ok(NormalizedSolveOutcome::Accepted {
                solution: x,
                detail,
                iterations: iteration,
                residual_norm_history: history,
                backtracking_count,
                step_norm: last_step.unwrap_or(0.0),
                pivot_magnitude: pivot,
                matrix_norm,
            });
        }
        if iteration == MAX_NEWTON_ITERATIONS {
            return Ok(rejected(
                NumericalFailureKind::IterationLimit,
                iteration,
                normalized,
                backtracking_count,
                last_step,
                (pivot, matrix_norm),
                x,
            ));
        }
        let frozen = freeze_branches(&detail);
        let mut jacobian = bounded_jacobian(
            &mut evaluator,
            &mut valid_trial,
            &x,
            &normalized,
            unit_scales,
            &frozen,
        )?;
        let mut right_hand_side: Vec<f64> = normalized.iter().map(|value| -value).collect();
        adjust_linear_system(
            &x,
            &detail,
            unit_scales,
            &mut jacobian,
            &mut right_hand_side,
        )?;
        let (delta, current_pivot, current_matrix_norm) =
            match solve_linear(&jacobian, &right_hand_side) {
                Ok(value) => value,
                Err(evidence) => {
                    return Ok(rejected(
                        NumericalFailureKind::SingularPivot,
                        iteration,
                        normalized,
                        backtracking_count,
                        last_step,
                        (Some(evidence.pivot), Some(evidence.matrix_norm)),
                        x,
                    ));
                }
            };
        pivot = Some(current_pivot);
        matrix_norm = Some(current_matrix_norm);
        let prospective_step = delta.iter().map(|value| value.abs()).fold(0.0, f64::max);
        if norm <= 1.0 && prospective_step <= TEMPERATURE_STEP_TOLERANCE_K {
            return Ok(NormalizedSolveOutcome::Accepted {
                solution: x,
                detail,
                iterations: iteration,
                residual_norm_history: history,
                backtracking_count,
                step_norm: prospective_step,
                pivot_magnitude: pivot,
                matrix_norm,
            });
        }
        let accepted = backtracked_trial(
            &mut evaluator,
            &mut valid_trial,
            &x,
            &delta,
            norm,
            prospective_step,
        );
        if let Some((trial, step, exponent)) = accepted {
            x = trial;
            last_step = Some(step);
            backtracking_count += exponent;
        } else {
            return Ok(rejected(
                NumericalFailureKind::BacktrackingLimit,
                iteration,
                normalized,
                backtracking_count + MAX_BACKTRACKING_HALVINGS,
                Some(prospective_step),
                (pivot, matrix_norm),
                x,
            ));
        }
    }
    unreachable_solver_state()
}
