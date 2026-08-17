//! Deterministic nonlinear solve infrastructure shared by open and covered
//! land-surface-energy systems.

use crate::{LandSurfaceEnergyError, StepNorms};

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

#[derive(Clone, Debug, PartialEq)]
pub struct NumericalFailure {
    pub kind: NumericalFailureKind,
    pub iterations: u32,
    pub normalized_residuals: Vec<f64>,
    pub backtracking_count: u32,
    pub step_norms: StepNorms,
    pub pivot_magnitude: Option<f64>,
    pub matrix_norm: Option<f64>,
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
            for inner in column + 1..n {
                a[row][inner] -= factor * a[column][inner];
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

pub(crate) fn is_strict_residual_decrease(
    current_norm: f64,
    trial_residuals: &[f64],
) -> bool {
    normalized_infinity_norm(trial_residuals) < current_norm
}

/// Frozen centered-difference Newton algorithm shared by open and joint columns.
pub fn solve_normalized_system<D, B, E, V, F>(
    mut evaluator: E,
    initial: Vec<f64>,
    unit_scales: &[f64],
    mut valid_trial: V,
    mut freeze_branches: F,
) -> Result<NormalizedSolveOutcome<D>, LandSurfaceEnergyError>
where
    D: Clone,
    E: FnMut(&[f64], Option<&B>) -> Result<(Vec<f64>, D), LandSurfaceEnergyError>,
    V: FnMut(&[f64]) -> bool,
    F: FnMut(&D) -> B,
    B: Clone,
{
    if initial.len() != unit_scales.len() || initial.is_empty() {
        return Err(LandSurfaceEnergyError::topology_domain(
            "normalized_solver_shape",
        ));
    }
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
            return Ok(NormalizedSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::IterationLimit,
                iterations: iteration,
                normalized_residuals: normalized,
                backtracking_count,
                step_norms: StepNorms {
                    temperature_k: last_step,
                    humidity_kg_kg: None,
                    ci_pa: None,
                    hydraulic_mm: None,
                    beta: None,
                },
                pivot_magnitude: pivot,
                matrix_norm,
            }));
        }
        let frozen = freeze_branches(&detail);
        let perturbations: Vec<f64> = x
            .iter()
            .zip(unit_scales.iter())
            .map(|(value, scale)| f64::EPSILON.sqrt() * value.abs().max(*scale))
            .collect();
        let mut jacobian = vec![vec![0.0; x.len()]; x.len()];
        for column in 0..x.len() {
            let mut minus = x.clone();
            let mut plus = x.clone();
            minus[column] -= perturbations[column];
            plus[column] += perturbations[column];
            let (minus_residual, _) = evaluator(&minus, Some(&frozen))?;
            let (plus_residual, _) = evaluator(&plus, Some(&frozen))?;
            for row in 0..x.len() {
                jacobian[row][column] =
                    (plus_residual[row] - minus_residual[row]) / (2.0 * perturbations[column]);
            }
        }
        let right_hand_side: Vec<f64> = normalized.iter().map(|value| -value).collect();
        let (delta, current_pivot, current_matrix_norm) =
            match solve_linear(&jacobian, &right_hand_side) {
                Ok(value) => value,
                Err(evidence) => {
                    return Ok(NormalizedSolveOutcome::Rejected(NumericalFailure {
                        kind: NumericalFailureKind::SingularPivot,
                        iterations: iteration,
                        normalized_residuals: normalized,
                        backtracking_count,
                        step_norms: StepNorms {
                            temperature_k: last_step,
                            humidity_kg_kg: None,
                            ci_pa: None,
                            hydraulic_mm: None,
                            beta: None,
                        },
                        pivot_magnitude: Some(evidence.pivot),
                        matrix_norm: Some(evidence.matrix_norm),
                    }));
                }
            };
        pivot = Some(current_pivot);
        matrix_norm = Some(current_matrix_norm);
        let prospective_step = delta.iter().map(|value| value.abs()).fold(0.0, f64::max);
        let mut accepted = None;
        for exponent in 0..=MAX_BACKTRACKING_HALVINGS {
            let factor = 0.5_f64.powf(f64::from(exponent));
            let trial: Vec<f64> = x
                .iter()
                .zip(delta.iter())
                .map(|(value, change)| value + factor * change)
                .collect();
            if !valid_trial(&trial) {
                continue;
            }
            let trial_result = evaluator(&trial, None);
            let Ok((trial_residual, _)) = trial_result else {
                continue;
            };
            if is_strict_residual_decrease(norm, &trial_residual) {
                accepted = Some((trial, factor * prospective_step, exponent));
                break;
            }
        }
        if let Some((trial, step, exponent)) = accepted {
            x = trial;
            last_step = Some(step);
            backtracking_count += exponent;
        } else {
            return Ok(NormalizedSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::BacktrackingLimit,
                iterations: iteration,
                normalized_residuals: normalized,
                backtracking_count: backtracking_count + MAX_BACKTRACKING_HALVINGS,
                step_norms: StepNorms {
                    temperature_k: Some(prospective_step),
                    humidity_kg_kg: None,
                    ci_pa: None,
                    hydraulic_mm: None,
                    beta: None,
                },
                pivot_magnitude: pivot,
                matrix_norm,
            }));
        }
    }
    Err(LandSurfaceEnergyError::ConstitutiveDomain(
        "unreachable_normalized_solver_state",
    ))
}
