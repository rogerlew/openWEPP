#![allow(
    clippy::cast_possible_wrap,
    clippy::float_cmp,
    clippy::many_single_char_names,
    clippy::manual_let_else,
    clippy::needless_range_loop,
    clippy::similar_names,
    clippy::single_match_else,
    clippy::too_many_arguments
)]
//! Digest-bound deterministic scalar and vector nonlinear solvers.

use crate::VegetationError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NewtonSystem {
    Energy,
    Hydraulic,
}
impl NewtonSystem {
    fn error(self, message: &'static str) -> VegetationError {
        match self {
            Self::Energy => VegetationError::Energy(message),
            Self::Hydraulic => VegetationError::Hydraulic(message),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SolverDiagnostics {
    pub iterations: u32,
    pub evaluations: u32,
    pub backtracks: u32,
    pub residual_norm: f64,
    pub step_norm: f64,
    pub pivot_failure: bool,
}

pub fn brent_dekker<F>(
    mut lower: f64,
    mut upper: f64,
    atol: f64,
    rtol: f64,
    residual_tolerance: f64,
    max_evaluations: u32,
    mut function: F,
) -> Result<(f64, SolverDiagnostics), VegetationError>
where
    F: FnMut(f64) -> Result<f64, VegetationError>,
{
    if ![lower, upper, atol, rtol, residual_tolerance]
        .iter()
        .all(|v| v.is_finite())
        || lower > upper
        || atol < 0.0
        || rtol < 0.0
        || residual_tolerance < 0.0
    {
        return Err(VegetationError::Domain("Brent configuration"));
    }
    let mut fa = function(lower)?;
    let mut fb = function(upper)?;
    let mut evaluations = 2;
    if !fa.is_finite() || !fb.is_finite() {
        return Err(VegetationError::Domain("Brent residual"));
    }
    if fa.abs() <= residual_tolerance {
        return Ok((lower, diag(0, evaluations, fa.abs(), 0.0)));
    }
    if fb.abs() <= residual_tolerance {
        return Ok((upper, diag(0, evaluations, fb.abs(), 0.0)));
    }
    if fa.signum() == fb.signum() {
        return Err(VegetationError::CiNonConvergence);
    }
    let mut c = lower;
    let mut fc = fa;
    let mut d = upper - lower;
    let mut e = d;
    for iteration in 1..=max_evaluations.saturating_sub(2) {
        if fb.signum() == fc.signum() {
            c = lower;
            fc = fa;
            d = upper - lower;
            e = d;
        }
        if fc.abs() < fb.abs() {
            let old_lower = lower;
            let old_upper = upper;
            let old_c = c;
            let old_fa = fa;
            let old_fb = fb;
            let old_fc = fc;
            lower = old_upper;
            upper = old_c;
            c = old_lower;
            fa = old_fb;
            fb = old_fc;
            fc = old_fa;
        }
        let tolerance = atol + rtol * upper.abs();
        let midpoint = 0.5 * (c - upper);
        if midpoint.abs() <= tolerance || fb.abs() <= residual_tolerance {
            return Ok((upper, diag(iteration, evaluations, fb.abs(), d.abs())));
        }
        if e.abs() >= tolerance && fa.abs() > fb.abs() {
            let s = fb / fa;
            let (mut p, mut q) = if lower == c {
                (2.0 * midpoint * s, 1.0 - s)
            } else {
                let q0 = fa / fc;
                let r = fb / fc;
                (
                    s * (2.0 * midpoint * q0 * (q0 - r) - (upper - lower) * (r - 1.0)),
                    (q0 - 1.0) * (r - 1.0) * (s - 1.0),
                )
            };
            if p > 0.0 {
                q = -q;
            } else {
                p = -p;
            }
            let old_e = e;
            e = d;
            if 2.0 * p < (3.0 * midpoint * q - (tolerance * q).abs()).min((old_e * q).abs()) {
                d = p / q;
            } else {
                d = midpoint;
                e = d;
            }
        } else {
            d = midpoint;
            e = d;
        }
        lower = upper;
        fa = fb;
        upper += if d.abs() > tolerance {
            d
        } else {
            tolerance.copysign(midpoint)
        };
        fb = function(upper)?;
        evaluations += 1;
        if !fb.is_finite() {
            return Err(VegetationError::Domain("Brent residual"));
        }
    }
    Err(VegetationError::CiNonConvergence)
}

fn diag(
    iterations: u32,
    evaluations: u32,
    residual_norm: f64,
    step_norm: f64,
) -> SolverDiagnostics {
    SolverDiagnostics {
        iterations,
        evaluations,
        backtracks: 0,
        residual_norm,
        step_norm,
        pivot_failure: false,
    }
}

#[allow(clippy::too_many_lines)]
pub fn damped_newton<F>(
    system: NewtonSystem,
    initial: &[f64],
    unit_scales: &[f64],
    residual_atol: &[f64],
    residual_physical_scales: &[f64],
    residual_rtol: f64,
    step_tolerance: f64,
    max_iterations: u32,
    max_halvings: u32,
    mut residual: F,
) -> Result<(Vec<f64>, SolverDiagnostics), VegetationError>
where
    F: FnMut(&[f64]) -> Result<Vec<f64>, VegetationError>,
{
    let n = initial.len();
    if n == 0
        || unit_scales.len() != n
        || residual_atol.len() != n
        || residual_physical_scales.len() != n
    {
        return Err(VegetationError::Domain("Newton shape"));
    }
    let mut x = initial.to_vec();
    let mut r = residual(&x)?;
    validate_vector(&x, n)?;
    validate_vector(&r, n)?;
    let mut evaluations = 1;
    let mut backtracks = 0;
    let mut last_step = 0.0;
    for iteration in 1..=max_iterations {
        let norm = normalized_norm(&r, residual_atol, residual_physical_scales, residual_rtol)?;
        if norm <= 1.0 && last_step <= step_tolerance {
            return Ok((
                x,
                SolverDiagnostics {
                    iterations: iteration - 1,
                    evaluations,
                    backtracks,
                    residual_norm: norm,
                    step_norm: last_step,
                    pivot_failure: false,
                },
            ));
        }
        let mut jac = vec![vec![0.0; n]; n];
        for column in 0..n {
            let step = f64::EPSILON.sqrt() * x[column].abs().max(unit_scales[column]);
            let mut plus = x.clone();
            let mut minus = x.clone();
            plus[column] += step;
            minus[column] -= step;
            let rp = residual(&plus)?;
            let rm = residual(&minus)?;
            evaluations += 2;
            validate_vector(&rp, n)?;
            validate_vector(&rm, n)?;
            for row in 0..n {
                jac[row][column] = (rp[row] - rm[row]) / (2.0 * step);
            }
        }
        let rhs = r.iter().map(|v| -v).collect::<Vec<_>>();
        let delta = solve_pivoted(system, jac, rhs)?;
        let full_step = delta.iter().copied().map(f64::abs).fold(0.0, f64::max);
        if norm <= 1.0 && full_step <= step_tolerance {
            return Ok((
                x,
                SolverDiagnostics {
                    iterations: iteration - 1,
                    evaluations,
                    backtracks,
                    residual_norm: norm,
                    step_norm: full_step,
                    pivot_failure: false,
                },
            ));
        }
        let mut accepted = None;
        for half in 0..=max_halvings {
            let factor = 2.0_f64.powi(-(half as i32));
            let trial = x
                .iter()
                .zip(&delta)
                .map(|(v, d)| v + factor * d)
                .collect::<Vec<_>>();
            let rt = match residual(&trial) {
                Ok(value) => value,
                Err(_) => {
                    backtracks += 1;
                    continue;
                }
            };
            evaluations += 1;
            if validate_vector(&rt, n).is_err() {
                backtracks += 1;
                continue;
            }
            let trial_norm =
                normalized_norm(&rt, residual_atol, residual_physical_scales, residual_rtol)?;
            if trial_norm < norm {
                accepted = Some((trial, rt, factor));
                break;
            }
            backtracks += 1;
        }
        let Some((next, next_r, factor)) = accepted else {
            return Err(system.error("Newton backtracking limit"));
        };
        x = next;
        r = next_r;
        last_step = full_step * factor;
        if full_step * factor <= step_tolerance {
            let norm = normalized_norm(&r, residual_atol, residual_physical_scales, residual_rtol)?;
            if norm <= 1.0 {
                return Ok((
                    x,
                    SolverDiagnostics {
                        iterations: iteration,
                        evaluations,
                        backtracks,
                        residual_norm: norm,
                        step_norm: full_step * factor,
                        pivot_failure: false,
                    },
                ));
            }
        }
    }
    Err(system.error("Newton iteration limit"))
}

fn validate_vector(values: &[f64], n: usize) -> Result<(), VegetationError> {
    if values.len() != n || values.iter().any(|v| !v.is_finite()) {
        Err(VegetationError::Domain("nonfinite solver vector"))
    } else {
        Ok(())
    }
}
fn normalized_norm(
    r: &[f64],
    atol: &[f64],
    physical_scales: &[f64],
    rtol: f64,
) -> Result<f64, VegetationError> {
    let mut norm = 0.0_f64;
    for ((&value, &absolute), &physical) in r.iter().zip(atol).zip(physical_scales) {
        let scale = absolute + rtol * physical.abs();
        if !scale.is_finite() || scale <= 0.0 {
            return Err(VegetationError::Domain("solver scale"));
        }
        norm = norm.max(value.abs() / scale);
    }
    Ok(norm)
}

fn solve_pivoted(
    system: NewtonSystem,
    mut a: Vec<Vec<f64>>,
    mut b: Vec<f64>,
) -> Result<Vec<f64>, VegetationError> {
    let n = b.len();
    let matrix_norm = a
        .iter()
        .flat_map(|row| row.iter())
        .copied()
        .map(f64::abs)
        .fold(0.0, f64::max);
    let threshold = 64.0 * f64::EPSILON * matrix_norm;
    for col in 0..n {
        let pivot = (col..n)
            .max_by(|&left, &right| a[left][col].abs().total_cmp(&a[right][col].abs()))
            .ok_or_else(|| system.error("empty Jacobian"))?;
        if a[pivot][col].abs() <= threshold {
            return Err(system.error("singular Jacobian pivot"));
        }
        a.swap(col, pivot);
        b.swap(col, pivot);
        for row in col + 1..n {
            let factor = a[row][col] / a[col][col];
            for k in col..n {
                a[row][k] -= factor * a[col][k];
            }
            b[row] -= factor * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let tail = (row + 1..n).map(|k| a[row][k] * x[k]).sum::<f64>();
        x[row] = (b[row] - tail) / a[row][row];
    }
    validate_vector(&x, n)?;
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brent_rotation_and_newton_failure_taxonomy_are_stable() {
        let (root, diagnostics) =
            brent_dekker(0.0, 2.0, 1e-12, 1e-12, 1e-12, 64, |x| Ok(x * x - 2.0))
                .expect("bracketed square root");
        assert!((root - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(diagnostics.evaluations <= 64);

        let singular = damped_newton(
            NewtonSystem::Energy,
            &[1.0],
            &[1.0],
            &[1e-6],
            &[1.0],
            1e-10,
            1e-8,
            2,
            2,
            |_| Ok(vec![1.0]),
        );
        assert_eq!(
            singular,
            Err(VegetationError::Energy("singular Jacobian pivot"))
        );
    }
}
