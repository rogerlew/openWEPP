//! Test-only same-support implicit-method machinery for terminal exploration.
//!
//! This module is deliberately absent from production builds. It operates on
//! typed storage-coordinate vectors supplied by a fixture adapter; it neither
//! publishes candidates nor changes the production temporal operator.

#[derive(Clone, Debug)]
pub(crate) struct NonlinearSolve {
    pub state: Vec<f64>,
    pub residual_max: f64,
    pub iterations: u8,
}

fn infinity_norm(values: &[f64]) -> f64 {
    values
        .iter()
        .fold(0.0_f64, |maximum, value| maximum.max(value.abs()))
}

fn solve_dense(mut matrix: Vec<Vec<f64>>, mut rhs: Vec<f64>) -> Result<Vec<f64>, &'static str> {
    let n = rhs.len();
    if matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
        return Err("dense solve shape");
    }
    for column in 0..n {
        let pivot = (column..n)
            .max_by(|left, right| {
                matrix[*left][column]
                    .abs()
                    .total_cmp(&matrix[*right][column].abs())
            })
            .ok_or("dense solve pivot")?;
        if !matrix[pivot][column].is_finite() || matrix[pivot][column].abs() <= 1.0e-14 {
            return Err("singular dense solve");
        }
        matrix.swap(column, pivot);
        rhs.swap(column, pivot);
        let diagonal = matrix[column][column];
        for entry in column..n {
            matrix[column][entry] /= diagonal;
        }
        rhs[column] /= diagonal;
        for row in 0..n {
            if row == column {
                continue;
            }
            let factor = matrix[row][column];
            for entry in column..n {
                matrix[row][entry] -= factor * matrix[column][entry];
            }
            rhs[row] -= factor * rhs[column];
        }
    }
    rhs.iter()
        .all(|value| value.is_finite())
        .then_some(rhs)
        .ok_or("nonfinite dense solution")
}

fn finite_difference_jacobian<R>(state: &[f64], residual: &R) -> Result<Vec<Vec<f64>>, &'static str>
where
    R: Fn(&[f64]) -> Result<Vec<f64>, &'static str>,
{
    let base = residual(state)?;
    let mut jacobian = vec![vec![0.0; state.len()]; base.len()];
    for column in 0..state.len() {
        let step = f64::EPSILON.sqrt() * state[column].abs().max(1.0);
        let mut displaced = state.to_vec();
        displaced[column] += step;
        let value = residual(&displaced)?;
        if value.len() != base.len() {
            return Err("residual cardinality changed");
        }
        for row in 0..base.len() {
            jacobian[row][column] = (value[row] - base[row]) / step;
        }
    }
    Ok(jacobian)
}

fn damped_newton_with_jacobian<R, J>(
    seed: &[f64],
    residual: R,
    jacobian: J,
) -> Result<NonlinearSolve, &'static str>
where
    R: Fn(&[f64]) -> Result<Vec<f64>, &'static str>,
    J: Fn(&[f64]) -> Result<Vec<Vec<f64>>, &'static str>,
{
    let mut state = seed.to_vec();
    for iteration in 0..32_u8 {
        let value = residual(&state)?;
        let residual_max = infinity_norm(&value);
        if residual_max <= 1.0e-12 {
            return Ok(NonlinearSolve {
                state,
                residual_max,
                iterations: iteration,
            });
        }
        let matrix = jacobian(&state)?;
        let direction = solve_dense(matrix, value.iter().map(|value| -value).collect())?;
        let mut factor = 1.0;
        let mut accepted = None;
        for _ in 0..16 {
            let trial = state
                .iter()
                .zip(&direction)
                .map(|(value, step)| value + factor * step)
                .collect::<Vec<_>>();
            let trial_norm = infinity_norm(&residual(&trial)?);
            if trial_norm <= (1.0 - 1.0e-4 * factor) * residual_max {
                accepted = Some(trial);
                break;
            }
            factor *= 0.5;
        }
        state = accepted.ok_or("Newton globalization exhausted")?;
    }
    Err("Newton iteration exhausted")
}

pub(crate) fn damped_newton<R>(seed: &[f64], residual: R) -> Result<NonlinearSolve, &'static str>
where
    R: Fn(&[f64]) -> Result<Vec<f64>, &'static str>,
{
    damped_newton_with_jacobian(seed, &residual, |state| {
        finite_difference_jacobian(state, &residual)
    })
}

pub(crate) fn cn_solve<F, J>(
    beginning: &[f64],
    support_s: f64,
    rate: F,
    jacobian: J,
) -> Result<NonlinearSolve, &'static str>
where
    F: Fn(&[f64]) -> Result<Vec<f64>, &'static str>,
    J: Fn(&[f64]) -> Result<Vec<Vec<f64>>, &'static str>,
{
    if support_s < 0.6 || !support_s.is_finite() || beginning.is_empty() {
        return Err("inadmissible CN support/state");
    }
    let beginning_rate = rate(beginning)?;
    let seed = beginning
        .iter()
        .zip(&beginning_rate)
        .map(|(state, rate)| state + support_s * rate)
        .collect::<Vec<_>>();
    let residual = |ending: &[f64]| {
        let ending_rate = rate(ending)?;
        Ok(ending
            .iter()
            .zip(beginning)
            .zip(beginning_rate.iter().zip(ending_rate))
            .map(|((ending, beginning), (rate0, rate1))| {
                ending - beginning - 0.5 * support_s * (rate0 + rate1)
            })
            .collect())
    };
    let residual_jacobian = |ending: &[f64]| {
        let rate_jacobian = jacobian(ending)?;
        if rate_jacobian.len() != beginning.len()
            || rate_jacobian.iter().any(|row| row.len() != beginning.len())
        {
            return Err("rate Jacobian cardinality");
        }
        Ok(rate_jacobian
            .iter()
            .enumerate()
            .map(|(row, entries)| {
                entries
                    .iter()
                    .enumerate()
                    .map(|(column, value)| f64::from(row == column) - 0.5 * support_s * value)
                    .collect()
            })
            .collect())
    };
    damped_newton_with_jacobian(&seed, residual, residual_jacobian)
}

#[derive(Clone, Debug)]
pub(crate) struct HermiteGaussEstimate {
    pub signed_endpoint_error: Vec<f64>,
    pub collocation_states: [Vec<f64>; 2],
    pub defects: [Vec<f64>; 2],
}

pub(crate) fn hermite_gauss_error_transport<F, J>(
    beginning: &[f64],
    ending: &[f64],
    support_s: f64,
    rate: F,
    jacobian: J,
) -> Result<HermiteGaussEstimate, &'static str>
where
    F: Fn(f64, &[f64]) -> Result<Vec<f64>, &'static str>,
    J: Fn(f64, &[f64]) -> Result<Vec<Vec<f64>>, &'static str>,
{
    if support_s < 0.6 || beginning.len() != ending.len() || beginning.is_empty() {
        return Err("inadmissible Hermite support/state");
    }
    let width = beginning.len();
    let endpoint_rates = [rate(0.0, beginning)?, rate(support_s, ending)?];
    if endpoint_rates.iter().any(|values| values.len() != width) {
        return Err("Hermite endpoint rate cardinality");
    }
    let ticks = [0.5 - 3.0_f64.sqrt() / 6.0, 0.5 + 3.0_f64.sqrt() / 6.0];
    let mut collocation_states = [vec![0.0; width], vec![0.0; width]];
    let mut extension_rates = [vec![0.0; width], vec![0.0; width]];
    for (stage, theta) in ticks.iter().copied().enumerate() {
        for component in 0..width {
            let y0 = beginning[component];
            let y1 = ending[component];
            let f0 = endpoint_rates[0][component];
            let f1 = endpoint_rates[1][component];
            collocation_states[stage][component] =
                (2.0 * theta.powi(3) - 3.0 * theta.powi(2) + 1.0) * y0
                    + (theta.powi(3) - 2.0 * theta.powi(2) + theta) * support_s * f0
                    + (-2.0 * theta.powi(3) + 3.0 * theta.powi(2)) * y1
                    + (theta.powi(3) - theta.powi(2)) * support_s * f1;
            extension_rates[stage][component] = ((6.0 * theta.powi(2) - 6.0 * theta) * y0
                + (3.0 * theta.powi(2) - 4.0 * theta + 1.0) * support_s * f0
                + (-6.0 * theta.powi(2) + 6.0 * theta) * y1
                + (3.0 * theta.powi(2) - 2.0 * theta) * support_s * f1)
                / support_s;
        }
    }
    let rates = [
        rate(ticks[0] * support_s, &collocation_states[0])?,
        rate(ticks[1] * support_s, &collocation_states[1])?,
    ];
    let jacobians = [
        jacobian(ticks[0] * support_s, &collocation_states[0])?,
        jacobian(ticks[1] * support_s, &collocation_states[1])?,
    ];
    if rates.iter().any(|values| values.len() != width)
        || jacobians
            .iter()
            .any(|matrix| matrix.len() != width || matrix.iter().any(|row| row.len() != width))
    {
        return Err("Hermite collocation cardinality");
    }
    let defects = [
        extension_rates[0]
            .iter()
            .zip(&rates[0])
            .map(|(extension, actual)| extension - actual)
            .collect::<Vec<_>>(),
        extension_rates[1]
            .iter()
            .zip(&rates[1])
            .map(|(extension, actual)| extension - actual)
            .collect::<Vec<_>>(),
    ];
    let root_three = 3.0_f64.sqrt();
    let a = [
        [0.25, 0.25 - root_three / 6.0],
        [0.25 + root_three / 6.0, 0.25],
    ];
    let mut matrix = vec![vec![0.0; 2 * width]; 2 * width];
    let mut rhs = vec![0.0; 2 * width];
    for stage in 0..2 {
        for row in 0..width {
            rhs[stage * width + row] = -support_s
                * (0..2)
                    .map(|other| a[stage][other] * defects[other][row])
                    .sum::<f64>();
            for other in 0..2 {
                for column in 0..width {
                    matrix[stage * width + row][other * width + column] =
                        f64::from(stage == other && row == column)
                            - support_s * a[stage][other] * jacobians[other][row][column];
                }
            }
        }
    }
    let transported = solve_dense(matrix, rhs)?;
    let signed_endpoint_error = (0..width)
        .map(|row| {
            0.5 * support_s
                * (0..2)
                    .map(|stage| {
                        (0..width)
                            .map(|column| {
                                jacobians[stage][row][column] * transported[stage * width + column]
                            })
                            .sum::<f64>()
                            - defects[stage][row]
                    })
                    .sum::<f64>()
        })
        .collect();
    Ok(HermiteGaussEstimate {
        signed_endpoint_error,
        collocation_states,
        defects,
    })
}

const SQRT_15: f64 = 3.872_983_346_207_417;
const SQRT_6: f64 = 2.449_489_742_783_178;
const GAUSS3_C: [f64; 3] = [0.5 - SQRT_15 / 10.0, 0.5, 0.5 + SQRT_15 / 10.0];
const GAUSS3_B: [f64; 3] = [5.0 / 18.0, 4.0 / 9.0, 5.0 / 18.0];
const GAUSS3_A: [[f64; 3]; 3] = [
    [
        5.0 / 36.0,
        2.0 / 9.0 - SQRT_15 / 15.0,
        5.0 / 36.0 - SQRT_15 / 30.0,
    ],
    [
        5.0 / 36.0 + SQRT_15 / 24.0,
        2.0 / 9.0,
        5.0 / 36.0 - SQRT_15 / 24.0,
    ],
    [
        5.0 / 36.0 + SQRT_15 / 30.0,
        2.0 / 9.0 + SQRT_15 / 15.0,
        5.0 / 36.0,
    ],
];

const RADAU3_C: [f64; 3] = [(4.0 - SQRT_6) / 10.0, (4.0 + SQRT_6) / 10.0, 1.0];
const RADAU3_B: [f64; 3] = [(16.0 - SQRT_6) / 36.0, (16.0 + SQRT_6) / 36.0, 1.0 / 9.0];
const RADAU3_A: [[f64; 3]; 3] = [
    [
        (88.0 - 7.0 * SQRT_6) / 360.0,
        (296.0 - 169.0 * SQRT_6) / 1800.0,
        (-2.0 + 3.0 * SQRT_6) / 225.0,
    ],
    [
        (296.0 + 169.0 * SQRT_6) / 1800.0,
        (88.0 + 7.0 * SQRT_6) / 360.0,
        (-2.0 - 3.0 * SQRT_6) / 225.0,
    ],
    [(16.0 - SQRT_6) / 36.0, (16.0 + SQRT_6) / 36.0, 1.0 / 9.0],
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum ReferenceMethod {
    GaussLegendre3,
    RadauIia3,
}

pub(crate) fn implicit_reference<F>(
    beginning: &[f64],
    support_s: f64,
    method: ReferenceMethod,
    rate: F,
) -> Result<NonlinearSolve, &'static str>
where
    F: Fn(f64, &[f64]) -> Result<Vec<f64>, &'static str>,
{
    if support_s < 0.6 || beginning.is_empty() {
        return Err("inadmissible reference support/state");
    }
    let (a, b, c) = match method {
        ReferenceMethod::GaussLegendre3 => (&GAUSS3_A, &GAUSS3_B, &GAUSS3_C),
        ReferenceMethod::RadauIia3 => (&RADAU3_A, &RADAU3_B, &RADAU3_C),
    };
    let width = beginning.len();
    let initial_rate = rate(0.0, beginning)?;
    let seed = c
        .iter()
        .flat_map(|tick| {
            beginning
                .iter()
                .zip(&initial_rate)
                .map(move |(y, f)| y + support_s * tick * f)
        })
        .collect::<Vec<_>>();
    let solved = damped_newton(&seed, |flat| {
        let stages = flat.chunks_exact(width).collect::<Vec<_>>();
        let rates = stages
            .iter()
            .enumerate()
            .map(|(stage, state)| rate(c[stage] * support_s, state))
            .collect::<Result<Vec<_>, _>>()?;
        let mut residual = Vec::with_capacity(3 * width);
        for (stage, state) in stages.iter().enumerate() {
            for component in 0..width {
                residual.push(
                    state[component]
                        - beginning[component]
                        - support_s
                            * (0..3)
                                .map(|other| a[stage][other] * rates[other][component])
                                .sum::<f64>(),
                );
            }
        }
        Ok(residual)
    })?;
    let stages = solved.state.chunks_exact(width).collect::<Vec<_>>();
    let rates = stages
        .iter()
        .enumerate()
        .map(|(stage, state)| rate(c[stage] * support_s, state))
        .collect::<Result<Vec<_>, _>>()?;
    let ending = (0..width)
        .map(|component| {
            beginning[component]
                + support_s
                    * (0..3)
                        .map(|stage| b[stage] * rates[stage][component])
                        .sum::<f64>()
        })
        .collect();
    Ok(NonlinearSolve {
        state: ending,
        residual_max: solved.residual_max,
        iterations: solved.iterations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_support_cn_and_reference_methods_have_expected_affine_accuracy() {
        let rate = |_tick: f64, state: &[f64]| Ok(vec![-0.1 * state[0] + 0.375]);
        let autonomous = |state: &[f64]| rate(0.0, state);
        let jacobian = |_state: &[f64]| Ok(vec![vec![-0.1]]);
        let cn = cn_solve(&[1.25], 0.9375, autonomous, jacobian).expect("CN");
        let gauss = implicit_reference(&[1.25], 0.9375, ReferenceMethod::GaussLegendre3, rate)
            .expect("Gauss reference");
        let radau = implicit_reference(&[1.25], 0.9375, ReferenceMethod::RadauIia3, rate)
            .expect("Radau reference");
        let exact = 3.75 + (1.25 - 3.75) * (-0.1_f64 * 0.9375).exp();
        let estimate =
            hermite_gauss_error_transport(&[1.25], &cn.state, 0.9375, rate, |_tick, _state| {
                Ok(vec![vec![-0.1]])
            })
            .expect("Hermite--Gauss transport");
        assert!((gauss.state[0] - exact).abs() < 1.0e-11);
        assert!((radau.state[0] - exact).abs() < 1.0e-9);
        assert!((cn.state[0] - exact).abs() > (gauss.state[0] - exact).abs());
        let reference_error = gauss.state[0] - cn.state[0];
        assert!(estimate.signed_endpoint_error[0] * reference_error > 0.0);
        assert!(estimate.signed_endpoint_error[0].abs() >= reference_error.abs());
        assert_eq!(estimate.collocation_states.len(), 2);
        assert_eq!(estimate.defects.len(), 2);
    }
}
