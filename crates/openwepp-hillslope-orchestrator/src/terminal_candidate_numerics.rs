//! Test-only same-support implicit-method machinery for terminal exploration.
//!
//! This module is deliberately absent from production builds. It operates on
//! typed storage-coordinate vectors supplied by a fixture adapter; it neither
//! publishes candidates nor changes the production temporal operator.

use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

const MINIMUM_CARRIER_NS: u128 = 60_000_000_000;

#[derive(Clone, Copy, Debug)]
pub(crate) struct EvaluationTime {
    pub outer_support: TimeSupport,
    pub normalized_abscissa: f64,
}

fn evaluation_time(
    outer_support: TimeSupport,
    normalized_abscissa: f64,
) -> Result<EvaluationTime, &'static str> {
    if !normalized_abscissa.is_finite() || !(0.0..=1.0).contains(&normalized_abscissa) {
        return Err("invalid normalized evaluation time");
    }
    Ok(EvaluationTime {
        outer_support,
        normalized_abscissa,
    })
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ConditioningDiagnostics {
    pub minimum_pivot: f64,
    pub maximum_pivot: f64,
    pub pivot_ratio: f64,
}

#[derive(Clone, Debug)]
pub(crate) struct NonlinearSolve {
    pub state: Vec<f64>,
    pub residual_max: f64,
    pub step_max: f64,
    pub iterations: u8,
    pub backtracking_count: u16,
    pub conditioning: ConditioningDiagnostics,
}

fn require_finite(values: &[f64], error: &'static str) -> Result<(), &'static str> {
    values
        .iter()
        .all(|value| value.is_finite())
        .then_some(())
        .ok_or(error)
}

fn scaled_infinity_norm(values: &[f64], scales: &[f64]) -> Result<f64, &'static str> {
    if values.len() != scales.len()
        || scales
            .iter()
            .any(|scale| !scale.is_finite() || *scale <= 0.0)
    {
        return Err("invalid numerical scales");
    }
    require_finite(values, "nonfinite scaled norm operand")?;
    Ok(values
        .iter()
        .zip(scales)
        .fold(0.0_f64, |maximum, (value, scale)| {
            maximum.max(value.abs() / scale)
        }))
}

fn solve_dense_with_diagnostics(
    mut matrix: Vec<Vec<f64>>,
    mut rhs: Vec<f64>,
) -> Result<(Vec<f64>, ConditioningDiagnostics), &'static str> {
    let n = rhs.len();
    if n == 0 || matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
        return Err("dense solve shape");
    }
    require_finite(&rhs, "nonfinite dense right-hand side")?;
    if matrix
        .iter()
        .any(|row| row.iter().any(|value| !value.is_finite()))
    {
        return Err("nonfinite dense matrix");
    }
    let mut minimum_pivot = f64::INFINITY;
    let mut maximum_pivot = 0.0_f64;
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
        minimum_pivot = minimum_pivot.min(diagonal.abs());
        maximum_pivot = maximum_pivot.max(diagonal.abs());
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
    require_finite(&rhs, "nonfinite dense solution")?;
    let pivot_ratio = maximum_pivot / minimum_pivot;
    if !pivot_ratio.is_finite() {
        return Err("nonfinite conditioning diagnostic");
    }
    Ok((
        rhs,
        ConditioningDiagnostics {
            minimum_pivot,
            maximum_pivot,
            pivot_ratio,
        },
    ))
}

fn solve_dense(matrix: Vec<Vec<f64>>, rhs: Vec<f64>) -> Result<Vec<f64>, &'static str> {
    solve_dense_with_diagnostics(matrix, rhs).map(|value| value.0)
}

fn finite_difference_jacobian<R>(state: &[f64], residual: &R) -> Result<Vec<Vec<f64>>, &'static str>
where
    R: Fn(&[f64]) -> Result<Vec<f64>, &'static str>,
{
    let base = residual(state)?;
    if base.len() != state.len() || base.is_empty() {
        return Err("finite-difference residual shape");
    }
    require_finite(&base, "nonfinite finite-difference residual")?;
    let mut jacobian = vec![vec![0.0; state.len()]; base.len()];
    for column in 0..state.len() {
        let step = f64::EPSILON.sqrt() * state[column].abs().max(1.0);
        let mut displaced = state.to_vec();
        displaced[column] += step;
        let value = residual(&displaced)?;
        if value.len() != base.len() {
            return Err("residual cardinality changed");
        }
        require_finite(&value, "nonfinite displaced residual")?;
        for row in 0..base.len() {
            jacobian[row][column] = (value[row] - base[row]) / step;
        }
    }
    Ok(jacobian)
}

fn damped_newton_with_jacobian_scaled<R, J>(
    seed: &[f64],
    unknown_scales: &[f64],
    residual_scales: &[f64],
    residual: R,
    jacobian: J,
) -> Result<NonlinearSolve, &'static str>
where
    R: Fn(&[f64]) -> Result<Vec<f64>, &'static str>,
    J: Fn(&[f64]) -> Result<Vec<Vec<f64>>, &'static str>,
{
    if seed.is_empty() || seed.len() != unknown_scales.len() || seed.len() != residual_scales.len()
    {
        return Err("Newton scale cardinality");
    }
    require_finite(seed, "nonfinite Newton seed")?;
    let mut state = seed.to_vec();
    let mut backtracking_count = 0_u16;
    for iteration in 0..32_u8 {
        let value = residual(&state)?;
        if value.len() != state.len() {
            return Err("Newton residual cardinality");
        }
        let residual_max = scaled_infinity_norm(&value, residual_scales)?;
        let matrix = jacobian(&state)?;
        if matrix.len() != state.len() || matrix.iter().any(|row| row.len() != state.len()) {
            return Err("Newton Jacobian cardinality");
        }
        let (direction, current_conditioning) =
            solve_dense_with_diagnostics(matrix, value.iter().map(|value| -value).collect())?;
        require_finite(&direction, "nonfinite Newton direction")?;
        let direction_max = scaled_infinity_norm(&direction, unknown_scales)?;
        if residual_max <= 1.0e-12 && direction_max <= 1.0e-12 {
            return Ok(NonlinearSolve {
                state,
                residual_max,
                step_max: direction_max,
                iterations: iteration,
                backtracking_count,
                conditioning: current_conditioning,
            });
        }
        let mut factor = 1.0;
        let mut accepted = None;
        for _ in 0..16 {
            let trial = state
                .iter()
                .zip(&direction)
                .map(|(value, step)| value + factor * step)
                .collect::<Vec<_>>();
            require_finite(&trial, "nonfinite Newton trial")?;
            let trial_value = residual(&trial)?;
            if trial_value.len() != residual_scales.len() {
                return Err("Newton trial residual cardinality");
            }
            let trial_norm = scaled_infinity_norm(&trial_value, residual_scales)?;
            if trial_norm <= (1.0 - 1.0e-4 * factor) * residual_max {
                accepted = Some(trial);
                break;
            }
            factor *= 0.5;
            backtracking_count = backtracking_count
                .checked_add(1)
                .ok_or("Newton backtracking count overflow")?;
        }
        state = accepted.ok_or("Newton globalization exhausted")?;
        let _accepted_step_max = scaled_infinity_norm(
            &direction
                .iter()
                .map(|step| factor * step)
                .collect::<Vec<_>>(),
            unknown_scales,
        )?;
    }
    Err("Newton iteration exhausted")
}

fn support_seconds(support: TimeSupport) -> Result<f64, &'static str> {
    if support.duration_ns() < MINIMUM_CARRIER_NS {
        return Err("support below carrier floor");
    }
    let seconds = f64::from_bits(support.duration_s_bits());
    seconds
        .is_finite()
        .then_some(seconds)
        .ok_or("nonfinite support")
}

pub(crate) fn cn_solve<F, J>(
    beginning: &[f64],
    support: TimeSupport,
    prescribed_increment: &[f64],
    unknown_scales: &[f64],
    residual_scales: &[f64],
    rate: F,
    jacobian: J,
) -> Result<NonlinearSolve, &'static str>
where
    F: Fn(EvaluationTime, &[f64]) -> Result<Vec<f64>, &'static str>,
    J: Fn(EvaluationTime, &[f64]) -> Result<Vec<Vec<f64>>, &'static str>,
{
    let support_s = support_seconds(support)?;
    if beginning.is_empty()
        || beginning.len() != prescribed_increment.len()
        || beginning.len() != unknown_scales.len()
        || beginning.len() != residual_scales.len()
    {
        return Err("inadmissible CN support/state");
    }
    require_finite(beginning, "nonfinite CN beginning")?;
    require_finite(prescribed_increment, "nonfinite prescribed increment")?;
    let start_time = evaluation_time(support, 0.0)?;
    let end_time = evaluation_time(support, 1.0)?;
    let beginning_rate = rate(start_time, beginning)?;
    if beginning_rate.len() != beginning.len() {
        return Err("beginning rate cardinality");
    }
    require_finite(&beginning_rate, "nonfinite beginning rate")?;
    let seed = beginning
        .iter()
        .zip(prescribed_increment)
        .zip(&beginning_rate)
        .map(|((state, amount), rate)| state + amount + support_s * rate)
        .collect::<Vec<_>>();
    let residual = |ending: &[f64]| {
        if ending.len() != beginning.len() {
            return Err("ending state cardinality");
        }
        let ending_rate = rate(end_time, ending)?;
        if ending_rate.len() != beginning.len() {
            return Err("ending rate cardinality");
        }
        require_finite(&ending_rate, "nonfinite ending rate")?;
        Ok(ending
            .iter()
            .zip(beginning)
            .zip(prescribed_increment)
            .zip(beginning_rate.iter().zip(ending_rate))
            .map(|(((ending, beginning), amount), (rate0, rate1))| {
                ending - beginning - amount - 0.5 * support_s * (rate0 + rate1)
            })
            .collect())
    };
    let residual_jacobian = |ending: &[f64]| {
        let rate_jacobian = jacobian(end_time, ending)?;
        if rate_jacobian.len() != beginning.len()
            || rate_jacobian.iter().any(|row| row.len() != beginning.len())
        {
            return Err("rate Jacobian cardinality");
        }
        if rate_jacobian
            .iter()
            .any(|row| row.iter().any(|value| !value.is_finite()))
        {
            return Err("nonfinite rate Jacobian");
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
    damped_newton_with_jacobian_scaled(
        &seed,
        unknown_scales,
        residual_scales,
        residual,
        residual_jacobian,
    )
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
    support: TimeSupport,
    rate: F,
    jacobian: J,
) -> Result<HermiteGaussEstimate, &'static str>
where
    F: Fn(EvaluationTime, &[f64]) -> Result<Vec<f64>, &'static str>,
    J: Fn(EvaluationTime, &[f64]) -> Result<Vec<Vec<f64>>, &'static str>,
{
    let support_s = support_seconds(support)?;
    if beginning.len() != ending.len() || beginning.is_empty() {
        return Err("inadmissible Hermite support/state");
    }
    require_finite(beginning, "nonfinite Hermite beginning")?;
    require_finite(ending, "nonfinite Hermite ending")?;
    let width = beginning.len();
    let endpoint_rates = [
        rate(evaluation_time(support, 0.0)?, beginning)?,
        rate(evaluation_time(support, 1.0)?, ending)?,
    ];
    if endpoint_rates.iter().any(|values| values.len() != width) {
        return Err("Hermite endpoint rate cardinality");
    }
    if endpoint_rates
        .iter()
        .any(|values| values.iter().any(|value| !value.is_finite()))
    {
        return Err("nonfinite Hermite endpoint rate");
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
        rate(evaluation_time(support, ticks[0])?, &collocation_states[0])?,
        rate(evaluation_time(support, ticks[1])?, &collocation_states[1])?,
    ];
    let jacobians = [
        jacobian(evaluation_time(support, ticks[0])?, &collocation_states[0])?,
        jacobian(evaluation_time(support, ticks[1])?, &collocation_states[1])?,
    ];
    if rates.iter().any(|values| values.len() != width)
        || jacobians
            .iter()
            .any(|matrix| matrix.len() != width || matrix.iter().any(|row| row.len() != width))
    {
        return Err("Hermite collocation cardinality");
    }
    if rates
        .iter()
        .any(|values| values.iter().any(|value| !value.is_finite()))
        || jacobians.iter().any(|matrix| {
            matrix
                .iter()
                .any(|row| row.iter().any(|value| !value.is_finite()))
        })
    {
        return Err("nonfinite Hermite collocation evaluation");
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
    support: TimeSupport,
    method: ReferenceMethod,
    unknown_scales: &[f64],
    residual_scales: &[f64],
    rate: F,
) -> Result<NonlinearSolve, &'static str>
where
    F: Fn(EvaluationTime, &[f64]) -> Result<Vec<f64>, &'static str>,
{
    let support_s = support_seconds(support)?;
    if beginning.is_empty()
        || beginning.len() != unknown_scales.len()
        || beginning.len() != residual_scales.len()
    {
        return Err("inadmissible reference support/state");
    }
    require_finite(beginning, "nonfinite reference beginning")?;
    let (a, b, c) = match method {
        ReferenceMethod::GaussLegendre3 => (&GAUSS3_A, &GAUSS3_B, &GAUSS3_C),
        ReferenceMethod::RadauIia3 => (&RADAU3_A, &RADAU3_B, &RADAU3_C),
    };
    let width = beginning.len();
    let initial_rate = rate(evaluation_time(support, 0.0)?, beginning)?;
    if initial_rate.len() != beginning.len() {
        return Err("reference initial rate cardinality");
    }
    require_finite(&initial_rate, "nonfinite reference initial rate")?;
    let seed = c
        .iter()
        .flat_map(|tick| {
            beginning
                .iter()
                .zip(&initial_rate)
                .map(move |(y, f)| y + support_s * tick * f)
        })
        .collect::<Vec<_>>();
    let stage_unknown_scales = (0..3)
        .flat_map(|_| unknown_scales.iter().copied())
        .collect::<Vec<_>>();
    let stage_residual_scales = (0..3)
        .flat_map(|_| residual_scales.iter().copied())
        .collect::<Vec<_>>();
    let residual = |flat: &[f64]| {
        let stages = flat.chunks_exact(width).collect::<Vec<_>>();
        let rates = stages
            .iter()
            .enumerate()
            .map(|(stage, state)| rate(evaluation_time(support, c[stage])?, state))
            .collect::<Result<Vec<_>, _>>()?;
        if rates
            .iter()
            .any(|values| values.len() != width || values.iter().any(|value| !value.is_finite()))
        {
            return Err("reference stage rate cardinality/nonfinite");
        }
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
    };
    let solved = damped_newton_with_jacobian_scaled(
        &seed,
        &stage_unknown_scales,
        &stage_residual_scales,
        &residual,
        |state| finite_difference_jacobian(state, &residual),
    )?;
    let stages = solved.state.chunks_exact(width).collect::<Vec<_>>();
    let rates = stages
        .iter()
        .enumerate()
        .map(|(stage, state)| rate(evaluation_time(support, c[stage])?, state))
        .collect::<Result<Vec<_>, _>>()?;
    if rates
        .iter()
        .any(|values| values.len() != width || values.iter().any(|value| !value.is_finite()))
    {
        return Err("reference endpoint rate cardinality/nonfinite");
    }
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
        step_max: solved.step_max,
        iterations: solved.iterations,
        backtracking_count: solved.backtracking_count,
        conditioning: solved.conditioning,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PYTHON_MATRIX: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/",
        "artifacts/candidate-v22-hermite-gauss-matrix.json"
    ));

    fn exact_support(seconds: f64) -> TimeSupport {
        let duration_ns = match seconds.to_bits() {
            bits if bits == 0.6_f64.to_bits() => 60_000_000_000,
            bits if bits == 0.600_000_001_f64.to_bits() => 60_000_000_100,
            bits if bits == 0.9_f64.to_bits() => 90_000_000_000,
            bits if bits == 0.9375_f64.to_bits() => 93_750_000_000,
            bits if bits == 1.199_999_999_f64.to_bits() => 119_999_999_900,
            _ => panic!("unsupported analytical nanosecond duration {seconds}"),
        };
        let start = 17_000_000_000;
        TimeSupport::new(
            ModelTimeNs::new(start),
            ModelTimeNs::new(start + duration_ns),
        )
        .expect("exact analytical support")
    }

    fn assert_python_row_parity<F, J>(row: &serde_json::Value, rate: F, jacobian: J)
    where
        F: Fn(EvaluationTime, &[f64]) -> Result<Vec<f64>, &'static str>,
        J: Fn(EvaluationTime, &[f64]) -> Result<Vec<Vec<f64>>, &'static str>,
    {
        let support_s = row["support_s"].as_f64().expect("Python support");
        let support = exact_support(support_s);
        // The frozen matrix predates the 60-second constitutive floor. Scale
        // its rates by the same factor used to lift support so the autonomous
        // dimensionless system and every expected endpoint remain unchanged.
        let scaled_rate = |time, state: &[f64]| {
            rate(time, state).map(|values| values.into_iter().map(|value| value / 100.0).collect())
        };
        let scaled_jacobian = |time, state: &[f64]| {
            jacobian(time, state).map(|rows| {
                rows.into_iter()
                    .map(|row| row.into_iter().map(|value| value / 100.0).collect())
                    .collect()
            })
        };
        let beginning = match row["family"].as_str().expect("row family") {
            "nonlinear_manufactured" => row["parameter"].as_f64().expect("Python initial"),
            "conservative_two_node" => 5.0,
            "affine" | "stiff_affine" | "index1_linear_dae" => 1.25,
            family => panic!("unexpected Python beginning-state family {family}"),
        };
        let cn = cn_solve(
            &[beginning],
            support,
            &[0.0],
            &[1.0],
            &[1.0],
            &scaled_rate,
            &scaled_jacobian,
        )
        .expect("Rust analytical CN");
        let estimate = hermite_gauss_error_transport(
            &[beginning],
            &cn.state,
            support,
            scaled_rate,
            scaled_jacobian,
        )
        .expect("Rust analytical Hermite--Gauss");
        let python_installed = row["installed"].as_f64().expect("Python installed");
        let python_estimate = row["signed_estimate"]
            .as_f64()
            .expect("Python signed estimate");
        let parity_tolerance = |value: f64| 5.0e-12 * value.abs().max(1.0);
        assert!(
            (cn.state[0] - python_installed).abs() <= parity_tolerance(python_installed),
            "installed parity: Rust={} Python={python_installed} row={row}",
            cn.state[0]
        );
        assert!(
            (estimate.signed_endpoint_error[0] - python_estimate).abs()
                <= parity_tolerance(python_estimate),
            "estimate parity: Rust={} Python={python_estimate} row={row}",
            estimate.signed_endpoint_error[0]
        );
        let python_defects = row["gauss_defects"]
            .as_array()
            .expect("Python Gauss defects");
        for (rust, python) in estimate.defects.iter().zip(python_defects) {
            let scaled_python = python.as_f64().expect("Python defect") / 100.0;
            assert!(
                (rust[0] - scaled_python).abs() <= parity_tolerance(scaled_python),
                "scaled defect parity: Rust={} Python={} row={row}",
                rust[0],
                python
            );
        }
    }

    #[test]
    fn analytical_hermite_gauss_rows_match_package_python() {
        let matrix: serde_json::Value = serde_json::from_str(PYTHON_MATRIX).expect("Python matrix");
        assert_eq!(
            matrix["schema"],
            "openwepp-child1-hermite-gauss-candidate-matrix-v1"
        );
        let rows = matrix["numeric_rows"].as_array().expect("numeric rows");
        let mut compared = 0_usize;
        for row in rows {
            let Some(support_s) = row["support_s"].as_f64() else {
                continue;
            };
            if support_s >= 1.2 || row.get("signed_reference_error").is_none() {
                continue;
            }
            let parameter = row["parameter"].as_f64().expect("row parameter");
            match row["family"].as_str().expect("row family") {
                "affine" | "stiff_affine" => assert_python_row_parity(
                    row,
                    |_tick, state| Ok(vec![parameter * state[0] + 0.375]),
                    |_tick, _state| Ok(vec![vec![parameter]]),
                ),
                "index1_linear_dae" => {
                    let reduced = match parameter as i32 {
                        10 => -0.1,
                        100 => -1.0,
                        1000 => -10.0,
                        _ => panic!("unexpected Python DAE parameter {parameter}"),
                    };
                    assert_python_row_parity(
                        row,
                        |_tick, state| Ok(vec![reduced * state[0] + 0.375]),
                        |_tick, _state| Ok(vec![vec![reduced]]),
                    );
                }
                "conservative_two_node" => assert_python_row_parity(
                    row,
                    |_tick, state| Ok(vec![-2.0 * parameter * state[0]]),
                    |_tick, _state| Ok(vec![vec![-2.0 * parameter]]),
                ),
                "nonlinear_manufactured" => assert_python_row_parity(
                    row,
                    |_tick, state| Ok(vec![-state[0] * state[0]]),
                    |_tick, state| Ok(vec![vec![-2.0 * state[0]]]),
                ),
                family => panic!("unexpected Python analytical family {family}"),
            }
            compared += 1;
        }
        assert_eq!(compared, 48, "frozen analytical floor-row count");
    }

    #[test]
    fn numerical_guards_preserve_exact_support_amounts_and_absolute_time() {
        let start = ModelTimeNs::new(17_000_000_000);
        let below_floor = TimeSupport::new(start, ModelTimeNs::new(76_999_999_999))
            .expect("positive below-floor support");
        assert_eq!(
            cn_solve(
                &[1.0],
                below_floor,
                &[0.0],
                &[1.0],
                &[1.0],
                |_tick, _state| Ok(vec![0.0]),
                |_tick, _state| Ok(vec![vec![0.0]]),
            )
            .expect_err("subfloor support must fail"),
            "support below carrier floor"
        );

        let large_start = ModelTimeNs::new(9_007_199_254_740_993);
        let support = TimeSupport::new(large_start, ModelTimeNs::new(9_007_259_254_740_993))
            .expect("exact carrier-floor support");
        let solved = cn_solve(
            &[1.0],
            support,
            &[0.2],
            &[1.0],
            &[1.0],
            |time, _state| {
                assert_eq!(time.outer_support, support);
                Ok(vec![17.0 + 0.6 * time.normalized_abscissa])
            },
            |time, _state| {
                assert_eq!(time.outer_support, support);
                Ok(vec![vec![0.0]])
            },
        )
        .expect("time-dependent CN with exact prescribed amount");
        let expected = 1.0 + 0.2 + 0.5 * 60.0 * (17.0 + 17.6);
        assert!((solved.state[0] - expected).abs() <= 1.0e-14);
        assert!(solved.residual_max <= 1.0e-12);
        assert!(solved.step_max <= 1.0e-12);

        assert_eq!(
            cn_solve(
                &[1.0],
                support,
                &[],
                &[1.0],
                &[1.0],
                |_tick, _state| Ok(vec![0.0]),
                |_tick, _state| Ok(vec![vec![0.0]]),
            )
            .expect_err("increment cardinality must fail"),
            "inadmissible CN support/state"
        );
        assert_eq!(
            cn_solve(
                &[f64::NAN],
                support,
                &[0.0],
                &[1.0],
                &[1.0],
                |_tick, _state| Ok(vec![0.0]),
                |_tick, _state| Ok(vec![vec![0.0]]),
            )
            .expect_err("nonfinite beginning must fail"),
            "nonfinite CN beginning"
        );
    }

    #[test]
    fn scaled_newton_checks_residual_step_and_conditioning() {
        let solved = damped_newton_with_jacobian_scaled(
            &[1_001.0, 1.0e-4],
            &[1_000.0, 1.0e-3],
            &[1.0e-3, 1.0e-9],
            |state| Ok(vec![state[0] - 1_000.0, state[1] - 2.0e-4]),
            |_state| Ok(vec![vec![1.0, 0.0], vec![0.0, 1.0]]),
        )
        .expect("scaled Newton");
        assert_eq!(solved.state, vec![1_000.0, 2.0e-4]);
        assert!(solved.residual_max <= 1.0e-12);
        assert!(solved.step_max <= 1.0e-12);
        assert!(solved.conditioning.minimum_pivot > 0.0);
        assert!(solved.conditioning.pivot_ratio.is_finite());

        assert_eq!(
            damped_newton_with_jacobian_scaled(
                &[0.0],
                &[0.0],
                &[1.0],
                |_state| Ok(vec![0.0]),
                |_state| Ok(vec![vec![1.0]]),
            )
            .expect_err("zero scale must fail"),
            "invalid numerical scales"
        );
        assert_eq!(
            damped_newton_with_jacobian_scaled(
                &[0.0],
                &[1.0],
                &[1.0],
                |_state| Ok(vec![f64::INFINITY]),
                |_state| Ok(vec![vec![1.0]]),
            )
            .expect_err("nonfinite residual must fail"),
            "nonfinite scaled norm operand"
        );
    }

    #[test]
    fn same_support_cn_and_reference_methods_have_expected_affine_accuracy() {
        let rate = |_tick: EvaluationTime, state: &[f64]| Ok(vec![-0.001 * state[0] + 0.00375]);
        let jacobian = |_tick: EvaluationTime, _state: &[f64]| Ok(vec![vec![-0.001]]);
        let support = exact_support(0.9375);
        let cn = cn_solve(&[1.25], support, &[0.0], &[2.0], &[0.5], rate, jacobian).expect("CN");
        let gauss = implicit_reference(
            &[1.25],
            support,
            ReferenceMethod::GaussLegendre3,
            &[2.0],
            &[0.5],
            rate,
        )
        .expect("Gauss reference");
        let radau = implicit_reference(
            &[1.25],
            support,
            ReferenceMethod::RadauIia3,
            &[2.0],
            &[0.5],
            rate,
        )
        .expect("Radau reference");
        let exact = 3.75 + (1.25 - 3.75) * (-0.001_f64 * 93.75).exp();
        let estimate =
            hermite_gauss_error_transport(&[1.25], &cn.state, support, rate, |_tick, _state| {
                Ok(vec![vec![-0.001]])
            })
            .expect("Hermite--Gauss transport");
        assert!((gauss.state[0] - exact).abs() < 1.0e-11);
        assert!((radau.state[0] - exact).abs() < 1.0e-9);
        assert!((cn.state[0] - exact).abs() > (gauss.state[0] - exact).abs());
        let reference_error = gauss.state[0] - cn.state[0];
        assert!(estimate.signed_endpoint_error[0] * reference_error > 0.0);
        assert!(estimate.signed_endpoint_error[0].abs() >= reference_error.abs());
        assert!(cn.conditioning.minimum_pivot > 0.0);
        assert!(cn.conditioning.maximum_pivot >= cn.conditioning.minimum_pivot);
        assert!(cn.conditioning.pivot_ratio >= 1.0);
        assert_eq!(estimate.collocation_states.len(), 2);
        assert_eq!(estimate.defects.len(), 2);
    }
}
