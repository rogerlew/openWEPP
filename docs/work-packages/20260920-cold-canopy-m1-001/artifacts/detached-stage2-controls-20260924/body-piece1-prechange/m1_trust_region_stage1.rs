//! Private frozen-weight arithmetic shared by detached Stage 1 controls.

use std::cmp::Ordering;

use super::{
    M1TrustRegionBallEndpoint, M1TrustRegionBallObservation, M1TrustRegionJacobiInput,
    M1TrustRegionJacobiObservation, M1TrustRegionJacobiPair, M1TrustRegionLambdaEvaluation,
    M1TrustRegionRefusalKind,
};
use super::{M1TrustRegionMeritInput, M1TrustRegionMeritObservation};

pub(crate) fn checked_sum_squares(values: &[f64]) -> Result<f64, &'static str> {
    let mut sum = 0.0;
    for &value in values {
        if !value.is_finite() {
            return Err("nonfinite operand");
        }
        let square = value * value;
        if !square.is_finite() {
            return Err("nonfinite square");
        }
        sum += square;
        if !sum.is_finite() {
            return Err("nonfinite sum");
        }
    }
    Ok(sum)
}

pub(crate) fn weighted_system(
    raw: &[f64],
    jacobian: &[&[f64]],
    normalizers: &[f64],
    scales: &[f64],
) -> Result<(Vec<f64>, Vec<Vec<f64>>), &'static str> {
    if raw.len() != jacobian.len() || raw.len() != normalizers.len() {
        return Err("dimension mismatch");
    }
    let mut f = Vec::with_capacity(raw.len());
    let mut a = Vec::with_capacity(raw.len());
    for row in 0..raw.len() {
        if jacobian[row].len() != scales.len()
            || !raw[row].is_finite()
            || !normalizers[row].is_finite()
            || normalizers[row] <= 0.0
        {
            return Err("invalid weighted system input");
        }
        let weight = 1.0 / normalizers[row];
        let weighted_residual = weight * raw[row];
        if !weight.is_finite() || !weighted_residual.is_finite() {
            return Err("nonfinite weighting");
        }
        let mut a_row = Vec::with_capacity(scales.len());
        for col in 0..scales.len() {
            if !jacobian[row][col].is_finite() || !scales[col].is_finite() {
                return Err("nonfinite weighted system operand");
            }
            let weighted_jacobian = weight * jacobian[row][col];
            let coefficient = weighted_jacobian * scales[col];
            if !weighted_jacobian.is_finite() || !coefficient.is_finite() {
                return Err("nonfinite weighted system product");
            }
            a_row.push(coefficient);
        }
        f.push(weighted_residual);
        a.push(a_row);
    }
    Ok((f, a))
}

fn frozen_weighted_merit(
    raw: &[f64],
    jacobian: &[&[f64]],
    normalizers: &[f64],
    scales: &[f64],
    step: &[f64],
    candidate_raw: &[f64],
    moving_normalizers: &[f64],
) -> Result<(f64, f64, f64, f64), &'static str> {
    if raw.len() != jacobian.len()
        || raw.len() != normalizers.len()
        || raw.len() != candidate_raw.len()
        || raw.len() != moving_normalizers.len()
    {
        return Err("dimension mismatch");
    }
    let (f, a) = weighted_system(raw, jacobian, normalizers, scales)?;
    let vector_input = MeritVectorInput {
        raw,
        normalizers,
        scales,
        step,
        candidate_raw,
        moving_normalizers,
        weighted_residual: &f,
        weighted_matrix: &a,
    };
    let vectors = merit_vectors(&vector_input)?;
    merit_reductions(&f, vectors)
}

struct MeritVectorInput<'a> {
    raw: &'a [f64],
    normalizers: &'a [f64],
    scales: &'a [f64],
    step: &'a [f64],
    candidate_raw: &'a [f64],
    moving_normalizers: &'a [f64],
    weighted_residual: &'a [f64],
    weighted_matrix: &'a [Vec<f64>],
}

struct MeritVectors {
    linear: Vec<f64>,
    candidate: Vec<f64>,
    moving: Vec<f64>,
}

fn merit_vectors(input: &MeritVectorInput<'_>) -> Result<MeritVectors, &'static str> {
    let MeritVectorInput {
        raw,
        normalizers,
        scales,
        step,
        candidate_raw,
        moving_normalizers,
        weighted_residual,
        weighted_matrix,
    } = input;
    let mut linear = Vec::with_capacity(raw.len());
    let mut candidate = Vec::with_capacity(raw.len());
    let mut moving = Vec::with_capacity(raw.len());
    for row in 0..raw.len() {
        if !moving_normalizers[row].is_finite() || moving_normalizers[row] <= 0.0 {
            return Err("invalid normalizer");
        }
        if scales.len() != step.len() {
            return Err("dimension mismatch");
        }
        let weight = 1.0 / normalizers[row];
        let moving_weight = 1.0 / moving_normalizers[row];
        if !weight.is_finite() || !moving_weight.is_finite() {
            return Err("nonfinite weight");
        }
        let weighted = weighted_residual[row];
        let frozen_candidate = weight * candidate_raw[row];
        let moving_candidate = moving_weight * candidate_raw[row];
        if !weighted.is_finite() || !frozen_candidate.is_finite() || !moving_candidate.is_finite() {
            return Err("nonfinite weighting");
        }
        let mut ap = 0.0;
        for col in 0..scales.len() {
            if !step[col].is_finite() {
                return Err("nonfinite operand");
            }
            let term = weighted_matrix[row][col] * step[col];
            if !term.is_finite() {
                return Err("nonfinite intermediate");
            }
            ap += term;
            if !ap.is_finite() {
                return Err("nonfinite accumulation");
            }
        }
        let linear_value = weighted + ap;
        if !linear_value.is_finite() {
            return Err("nonfinite residual");
        }
        linear.push(linear_value);
        candidate.push(frozen_candidate);
        moving.push(moving_candidate);
    }
    Ok(MeritVectors {
        linear,
        candidate,
        moving,
    })
}

fn merit_reductions(
    f: &[f64],
    vectors: MeritVectors,
) -> Result<(f64, f64, f64, f64), &'static str> {
    let MeritVectors {
        linear,
        candidate,
        moving,
    } = vectors;
    let f2 = checked_sum_squares(f)?;
    let linear2 = checked_sum_squares(linear.as_slice())?;
    let candidate2 = checked_sum_squares(candidate.as_slice())?;
    let predicted = 0.5 * (f2 - linear2);
    let actual = (0.5 * f2) - (0.5 * candidate2);
    let moving_merit = 0.5 * checked_sum_squares(moving.as_slice())?;
    let rho = actual / predicted;
    if !f2.is_finite()
        || !linear2.is_finite()
        || !candidate2.is_finite()
        || !predicted.is_finite()
        || !actual.is_finite()
        || !moving_merit.is_finite()
        || !rho.is_finite()
    {
        return Err("nonfinite merit");
    }
    Ok((predicted, actual, rho, moving_merit))
}

pub(crate) fn m1_trust_region_frozen_merit_for_test(
    _input: &super::M1CoupledColumnInput,
    _trial: &super::M1CoupledColumnTrial,
    value: M1TrustRegionMeritInput,
) -> Result<M1TrustRegionMeritObservation, &'static str> {
    let rows: [&[f64]; 2] = [&value.raw_jacobian[0], &value.raw_jacobian[1]];
    let (predicted_reduction, actual_reduction, rho, moving_weight_trial_merit) =
        frozen_weighted_merit(
            &value.raw_residual,
            &rows,
            &value.frozen_normalizers,
            &value.coordinate_scales,
            &value.scaled_step,
            &value.candidate_raw_residual,
            &value.moving_candidate_normalizers,
        )?;
    Ok(M1TrustRegionMeritObservation {
        predicted_reduction,
        actual_reduction,
        rho,
        moving_weight_trial_merit,
    })
}

struct JacobiFactor {
    // Keep the input matrix as well as its rotated columns.  The frozen method
    // defines U as A_free * V / sigma, with this multiplication accumulated in
    // ascending free-coordinate order; the rotated B is not a substitute in
    // binary64 arithmetic.
    a: Vec<Vec<f64>>,
    b: Vec<Vec<f64>>,
    v: Vec<Vec<f64>>,
    sigma: Vec<f64>,
    order: Vec<usize>,
    visits: Vec<M1TrustRegionJacobiPair>,
    rotations: usize,
    sweeps: u8,
}

pub(crate) struct LambdaTrace {
    pub(crate) initial_bracket: [f64; 2],
    pub(crate) bracket: [f64; 2],
    pub(crate) bracket_evaluations: Vec<(f64, f64)>,
    pub(crate) bisection_evaluations: Vec<(f64, f64)>,
    pub(crate) lambda: f64,
    pub(crate) step: Vec<f64>,
    pub(crate) natural_step: Vec<f64>,
    pub(crate) pre_hook_step: Vec<f64>,
    pub(crate) post_hook_step: Vec<f64>,
    pub(crate) norm: f64,
    pub(crate) gap: f64,
    pub(crate) tolerance: f64,
}

impl Default for LambdaTrace {
    fn default() -> Self {
        Self {
            initial_bracket: [0.0, 0.0],
            bracket: [0.0, 0.0],
            bracket_evaluations: Vec::new(),
            bisection_evaluations: Vec::new(),
            lambda: 0.0,
            step: Vec::new(),
            natural_step: Vec::new(),
            pre_hook_step: Vec::new(),
            post_hook_step: Vec::new(),
            norm: f64::NAN,
            gap: f64::NAN,
            tolerance: f64::NAN,
        }
    }
}

fn lambda_step(factor: &JacobiFactor, y: &[f64], lambda: f64) -> Result<Vec<f64>, &'static str> {
    if y.len() != factor.b.len() || !lambda.is_finite() || lambda < 0.0 {
        return Err("invalid lambda input");
    }
    let cols = factor.sigma.len();
    let mut coefficients = vec![0.0; cols];
    for sorted in 0..cols {
        let j = factor.order[sorted];
        let sigma = factor.sigma[j];
        let mut uty = 0.0;
        for (row, &y_value) in y.iter().enumerate() {
            let mut av = 0.0;
            for free in 0..cols {
                let term = factor.a[row][free] * factor.v[free][j];
                if !term.is_finite() {
                    return Err("nonfinite A_free V product");
                }
                av += term;
                if !av.is_finite() {
                    return Err("nonfinite A_free V sum");
                }
            }
            let u = av / sigma;
            if !u.is_finite() || !y_value.is_finite() {
                return Err("nonfinite U");
            }
            let term = u * y_value;
            if !term.is_finite() {
                return Err("nonfinite Uty product");
            }
            uty += term;
            if !uty.is_finite() {
                return Err("nonfinite Uty sum");
            }
        }
        let denom = sigma * sigma + lambda;
        let gain = sigma / denom;
        let coefficient = -(gain * uty);
        if !denom.is_finite() || !gain.is_finite() || !coefficient.is_finite() {
            return Err("nonfinite lambda coefficient");
        }
        coefficients[j] = coefficient;
    }
    let mut step = vec![0.0; cols];
    for (out, output) in step.iter_mut().enumerate() {
        let mut sum = 0.0;
        for sorted in 0..cols {
            let j = factor.order[sorted];
            let term = factor.v[out][j] * coefficients[j];
            if !term.is_finite() {
                return Err("nonfinite V product");
            }
            sum += term;
            if !sum.is_finite() {
                return Err("nonfinite V sum");
            }
        }
        *output = sum;
    }
    Ok(step)
}

fn lambda_norm(step: &[f64]) -> Result<f64, &'static str> {
    let mut sum = 0.0;
    for &x in step {
        let sq = x * x;
        if !sq.is_finite() {
            return Err("nonfinite norm");
        }
        sum += sq;
        if !sum.is_finite() {
            return Err("nonfinite norm");
        }
    }
    let norm = sum.sqrt();
    if norm.is_finite() {
        Ok(norm)
    } else {
        Err("nonfinite norm")
    }
}

fn svd_lambda_ball(
    factor: &JacobiFactor,
    y: &[f64],
    radius: f64,
    override_endpoint: Option<&[f64]>,
    trace: &mut LambdaTrace,
) -> Result<(), M1TrustRegionRefusalKind> {
    if !radius.is_finite() || radius <= 0.0 {
        return Err(M1TrustRegionRefusalKind::DegenerateFace);
    }
    let zero = lambda_step(factor, y, 0.0).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    let n0 = lambda_norm(&zero).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    trace.initial_bracket = [0.0, 0.0];
    trace.bracket = [0.0, 0.0];
    trace.bracket_evaluations.push((0.0, n0));
    if n0 <= radius {
        trace.lambda = 0.0;
        trace.step = zero.clone();
        trace.natural_step = zero.clone();
        trace.pre_hook_step = zero.clone();
        trace.post_hook_step = zero;
        trace.norm = n0;
        trace.gap = radius - n0;
        trace.tolerance = (2_f64.powi(-40) * radius).max(64.0 * f64::EPSILON * radius.max(1.0));
        return Ok(());
    }
    let mut lo = 0.0;
    let mut hi = 1.0;
    let mut high =
        lambda_step(factor, y, hi).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    let mut nh = lambda_norm(&high).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    trace.initial_bracket = [0.0, hi];
    trace.bracket = [0.0, hi];
    trace.bracket_evaluations.push((hi, nh));
    for _ in 0..48 {
        if nh <= radius {
            break;
        }
        lo = hi;
        hi *= 4.0;
        high = lambda_step(factor, y, hi).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        nh = lambda_norm(&high).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        trace.bracket_evaluations.push((hi, nh));
    }
    if nh > radius {
        return Err(M1TrustRegionRefusalKind::LambdaBracket);
    }
    for _ in 0..48 {
        let mid = (lo + hi) * 0.5;
        let step =
            lambda_step(factor, y, mid).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        let norm = lambda_norm(&step).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        trace.bisection_evaluations.push((mid, norm));
        if norm <= radius {
            hi = mid;
            high = step;
        } else {
            lo = mid;
        }
    }
    trace.bracket = [lo, hi];
    trace.lambda = hi;
    trace.natural_step = high.clone();
    trace.pre_hook_step = high;
    trace.post_hook_step =
        override_endpoint.map_or_else(|| trace.pre_hook_step.clone(), <[f64]>::to_vec);
    trace.step = trace.post_hook_step.clone();
    let norm = lambda_norm(&trace.step).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    let gap = radius - norm;
    let tolerance = (2_f64.powi(-40) * radius).max(64.0 * f64::EPSILON * radius.max(1.0));
    trace.norm = norm;
    trace.gap = gap;
    trace.tolerance = tolerance;
    if norm > radius || gap > tolerance {
        return Err(M1TrustRegionRefusalKind::BallAccuracy);
    }
    Ok(())
}

pub(crate) fn m1_trust_region_validate_ball_endpoint_for_test(
    endpoint: M1TrustRegionBallEndpoint,
) -> M1TrustRegionBallObservation {
    // This is deliberately only a data adapter around the shared factorization
    // and ball solve.  The test seam may replace the final coordinate after the
    // canonical bisections; it does not select a verdict or alter their count.
    let mut observed_sweeps = 0;
    let factor = jacobi_factor(vec![vec![endpoint.a]], &[0], 0, &mut observed_sweeps);
    let mut trace = LambdaTrace::default();
    let (factorization_entered, kind) = match factor {
        Ok(factor) => {
            let y = [endpoint.f];
            let result = svd_lambda_ball(
                &factor,
                &y,
                endpoint.radius,
                endpoint.endpoint_override.as_ref().map(|x| &x[..]),
                &mut trace,
            );
            (true, result.err())
        }
        Err("rank deficient") => (true, Some(M1TrustRegionRefusalKind::RankDeficient)),
        Err("no convergence") => (true, Some(M1TrustRegionRefusalKind::SvdNoConvergence)),
        Err(_) => (true, Some(M1TrustRegionRefusalKind::SvdNonFinite)),
    };
    let evaluation = |(lambda, norm): &(f64, f64)| M1TrustRegionLambdaEvaluation {
        lambda: *lambda,
        norm: *norm,
        feasible: *norm <= endpoint.radius,
    };
    let scalar = |values: &[f64]| values.first().copied().unwrap_or(f64::NAN);
    M1TrustRegionBallObservation {
        factorization_entered,
        bracket_endpoint_evaluations: trace.bracket_evaluations.iter().map(evaluation).collect(),
        bisection_evaluations: trace.bisection_evaluations.iter().map(evaluation).collect(),
        bracket_initial: trace.initial_bracket,
        bracket_final: trace.bracket,
        final_upper_lambda: trace.lambda,
        natural_upper_endpoint: scalar(&trace.natural_step),
        pre_hook_endpoint: scalar(&trace.pre_hook_step),
        endpoint_hook_observed: endpoint.endpoint_override,
        post_hook_endpoint: [scalar(&trace.post_hook_step)],
        computed_norm: trace.norm,
        computed_gap: trace.gap,
        computed_tolerance: trace.tolerance,
        accepted: kind.is_none(),
        kind,
    }
}

fn jacobi_column_products(
    matrix: &[Vec<f64>],
    left: usize,
    right: usize,
) -> Result<(f64, f64, f64), &'static str> {
    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut gamma = 0.0;
    for row in matrix {
        let left_value = row[left];
        let right_value = row[right];
        let left_square = left_value * left_value;
        let right_square = right_value * right_value;
        let product = left_value * right_value;
        if !left_square.is_finite() || !right_square.is_finite() || !product.is_finite() {
            return Err("nonfinite SVD product");
        }
        alpha += left_square;
        beta += right_square;
        gamma += product;
        if !alpha.is_finite() || !beta.is_finite() || !gamma.is_finite() {
            return Err("nonfinite SVD accumulation");
        }
    }
    Ok((alpha, beta, gamma))
}

fn rotate_jacobi_columns(
    matrix: &mut [Vec<f64>],
    left: usize,
    right: usize,
    cosine: f64,
    sine: f64,
) -> Result<(), &'static str> {
    for row in matrix {
        let left_value = row[left];
        let right_value = row[right];
        let new_left = cosine * left_value - sine * right_value;
        let new_right = sine * left_value + cosine * right_value;
        if !new_left.is_finite() || !new_right.is_finite() {
            return Err("nonfinite rotation product");
        }
        row[left] = new_left;
        row[right] = new_right;
    }
    Ok(())
}

fn sorted_singular_values(
    rotated_b: &[Vec<f64>],
    free_ids: &[usize],
) -> Result<(Vec<f64>, Vec<usize>), &'static str> {
    let cols = free_ids.len();
    let mut sigma = vec![0.0; cols];
    for col in 0..cols {
        for row in rotated_b {
            let square = row[col] * row[col];
            if !square.is_finite() {
                return Err("nonfinite singular value");
            }
            sigma[col] += square;
            if !sigma[col].is_finite() {
                return Err("nonfinite singular value");
            }
        }
        sigma[col] = sigma[col].sqrt();
        if !sigma[col].is_finite() {
            return Err("nonfinite singular value");
        }
    }
    let max = sigma.iter().copied().fold(0.0, f64::max);
    if max == 0.0 || sigma.iter().any(|s| *s <= 2_f64.powi(-40) * max) {
        return Err("rank deficient");
    }
    let mut order: Vec<usize> = (0..cols).collect();
    order.sort_by(|a, b| {
        sigma[*b]
            .total_cmp(&sigma[*a])
            .then_with(|| free_ids[*a].cmp(&free_ids[*b]))
    });
    Ok((sigma, order))
}

fn jacobi_factor(
    mut b: Vec<Vec<f64>>,
    free_ids: &[usize],
    completed_sweeps: u8,
    work_counter: &mut u8,
) -> Result<JacobiFactor, &'static str> {
    let a = b.clone();
    let cols = free_ids.len();
    if cols == 0
        || b.iter()
            .any(|row| row.len() != cols || row.iter().any(|x| !x.is_finite()))
    {
        return Err("invalid SVD input");
    }
    let mut v = vec![vec![0.0; cols]; cols];
    for (index, row) in v.iter_mut().enumerate() {
        row[index] = 1.0;
    }
    let mut visits = Vec::new();
    let mut rotations = 0usize;
    let mut sweeps = completed_sweeps;
    *work_counter = completed_sweeps;
    if completed_sweeps >= 64 {
        return Err("no convergence");
    }
    for sweep in (completed_sweeps + 1)..=64u8 {
        let mut any = false;
        for left in 0..cols {
            for right in left + 1..cols {
                let (alpha, beta, gamma) = jacobi_column_products(&b, left, right)?;
                if !alpha.is_finite() || !beta.is_finite() || !gamma.is_finite() {
                    return Err("nonfinite SVD intermediate");
                }
                let product = alpha * beta;
                let root = product.sqrt();
                let threshold = 2_f64.powi(-48) * root;
                if !product.is_finite() || !root.is_finite() || !threshold.is_finite() {
                    return Err("nonfinite SVD threshold");
                }
                let rotate = gamma.abs() > threshold;
                visits.push(M1TrustRegionJacobiPair {
                    left,
                    right,
                    rotated: rotate,
                    sweep,
                });
                if rotate {
                    let numerator = beta - alpha;
                    let denominator = 2.0 * gamma;
                    let tau = numerator / denominator;
                    let hypot = tau.hypot(1.0);
                    let divisor = tau.abs() + hypot;
                    let tangent = if tau == 0.0 {
                        1.0
                    } else {
                        tau.signum() / divisor
                    };
                    let cosine = 1.0 / tangent.hypot(1.0);
                    let sine = cosine * tangent;
                    if !numerator.is_finite()
                        || !denominator.is_finite()
                        || !tau.is_finite()
                        || !hypot.is_finite()
                        || !divisor.is_finite()
                        || !tangent.is_finite()
                        || !cosine.is_finite()
                        || !sine.is_finite()
                    {
                        return Err("nonfinite rotation");
                    }
                    rotate_jacobi_columns(&mut b, left, right, cosine, sine)?;
                    rotate_jacobi_columns(&mut v, left, right, cosine, sine)?;
                    rotations += 1;
                    any = true;
                }
            }
        }
        sweeps = sweep;
        *work_counter = sweeps;
        if !any {
            break;
        }
    }
    if sweeps == 64
        && visits
            .iter()
            .rev()
            .take(cols * (cols - 1) / 2)
            .any(|p| p.rotated)
    {
        return Err("no convergence");
    }
    let (sigma, order) = sorted_singular_values(&b, free_ids)?;
    Ok(JacobiFactor {
        a,
        b,
        v,
        sigma,
        order,
        visits,
        rotations,
        sweeps,
    })
}

pub(crate) fn m1_trust_region_jacobi_observation_for_test(
    input: M1TrustRegionJacobiInput,
) -> M1TrustRegionJacobiObservation {
    let mut observed_sweeps = 0;
    let factor = jacobi_factor(
        input.matrix.iter().map(|row| row.to_vec()).collect(),
        &input.free_ids,
        0,
        &mut observed_sweeps,
    )
    .expect("Jacobi fixture");
    let column = |m: &Vec<Vec<f64>>, c| [m[0][c], m[1][c], m[2][c]];
    let mut sorted_b = [[0.0; 3]; 3];
    let mut sorted_v = [[0.0; 3]; 3];
    let mut sorted_sigma = [0.0; 3];
    let mut sorted_ids = [0; 3];
    let mut order = [0; 3];
    for k in 0..3 {
        let c = factor.order[k];
        order[k] = c;
        sorted_sigma[k] = factor.sigma[c];
        sorted_ids[k] = input.free_ids[c];
        sorted_b[k] = column(&factor.b, c);
        sorted_v[k] = column(&factor.v, c);
    }
    M1TrustRegionJacobiObservation {
        completed_sweeps: factor.sweeps,
        pair_visits: factor.visits,
        rotations: factor.rotations,
        b_columns: [
            column(&factor.b, 0),
            column(&factor.b, 1),
            column(&factor.b, 2),
        ],
        v_columns: [
            column(&factor.v, 0),
            column(&factor.v, 1),
            column(&factor.v, 2),
        ],
        unsorted_singular_values: [factor.sigma[0], factor.sigma[1], factor.sigma[2]],
        sorted_singular_values: sorted_sigma,
        sorted_free_ids: sorted_ids,
        sorted_from_unsorted: order,
        sorted_b_columns: sorted_b,
        sorted_v_columns: sorted_v,
    }
}

fn subproblem_refusal(
    kind: M1TrustRegionRefusalKind,
    attempted_subproblems: u8,
    completed_jacobi_sweeps: u8,
    completed_face_pivots: u8,
    entered_svd_factorizations: u8,
    raw_residual_max: f64,
) -> super::M1TrustRegionSubproblemRefusal {
    super::M1TrustRegionSubproblemRefusal {
        kind,
        attempted_subproblems,
        completed_jacobi_sweeps,
        completed_face_pivots,
        entered_svd_factorizations,
        raw_residual_max,
    }
}

struct PreparedSubproblem {
    raw_max: f64,
    weighted_residual: Vec<f64>,
    weighted_matrix: Vec<Vec<f64>>,
    scaled_lower: [f64; 21],
    scaled_upper: [f64; 21],
}

struct KktValues {
    residual: [f64; 21],
    gradient: [f64; 21],
    lambda_times_step: [f64; 21],
    h: [f64; 21],
}

struct SubproblemState {
    p: [f64; 21],
    lower_active: [bool; 21],
    upper_active: [bool; 21],
    free: [bool; 21],
    initial_free: [bool; 21],
    events: Vec<super::M1TrustRegionKktEvent>,
    entered: u8,
    completed_pivots: u8,
    completed_sweeps: u8,
}

impl SubproblemState {
    fn new(input: &super::M1TrustRegionSubproblemInput) -> Self {
        let mut free = [false; 21];
        for (col, is_free) in free.iter_mut().enumerate() {
            *is_free = input.lower[col].partial_cmp(&input.upper[col]) != Some(Ordering::Equal);
        }
        Self {
            p: [0.0; 21],
            lower_active: [false; 21],
            upper_active: [false; 21],
            initial_free: free,
            free,
            events: Vec::new(),
            entered: 0,
            completed_pivots: input.test_work.completed_face_pivots,
            completed_sweeps: input.test_work.completed_jacobi_sweeps,
        }
    }
}

fn state_refusal(
    kind: M1TrustRegionRefusalKind,
    state: &SubproblemState,
    raw_max: f64,
) -> super::M1TrustRegionSubproblemRefusal {
    subproblem_refusal(
        kind,
        1,
        state.completed_sweeps,
        state.completed_pivots,
        state.entered,
        raw_max,
    )
}

fn box_feasible_kkt_values(
    weighted_matrix: &[Vec<f64>],
    weighted_residual: &[f64],
    step: &[f64; 21],
    lambda: f64,
) -> Result<KktValues, M1TrustRegionRefusalKind> {
    const N: usize = 21;
    let mut residual = [0.0; N];
    let mut gradient = [0.0; N];
    let mut lambda_times_step = [0.0; N];
    let mut h = [0.0; N];
    for row in 0..N {
        let mut ap_sum = 0.0;
        for col in 0..N {
            let term = weighted_matrix[row][col] * step[col];
            if !term.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
            ap_sum += term;
            if !ap_sum.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
        }
        residual[row] = weighted_residual[row] + ap_sum;
        if !residual[row].is_finite() {
            return Err(M1TrustRegionRefusalKind::SvdNonFinite);
        }
    }
    for col in 0..N {
        for row in 0..N {
            let term = weighted_matrix[row][col] * residual[row];
            if !term.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
            gradient[col] += term;
            if !gradient[col].is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
        }
        lambda_times_step[col] = lambda * step[col];
        h[col] = gradient[col] + lambda_times_step[col];
        if !lambda_times_step[col].is_finite() || !h[col].is_finite() {
            return Err(M1TrustRegionRefusalKind::SvdNonFinite);
        }
    }
    Ok(KktValues {
        residual,
        gradient,
        lambda_times_step,
        h,
    })
}

struct FacePreparation {
    radius: f64,
    free_ids: Vec<usize>,
    residual: [f64; 21],
    matrix: Vec<Vec<f64>>,
}

fn prepare_face(
    initial_radius: f64,
    step: &[f64; 21],
    lower_active: &[bool; 21],
    upper_active: &[bool; 21],
    free: &[bool; 21],
    weighted_residual: &[f64],
    weighted_matrix: &[Vec<f64>],
) -> Result<FacePreparation, M1TrustRegionRefusalKind> {
    const N: usize = 21;
    let mut active_square_sum = 0.0;
    for col in 0..N {
        if lower_active[col] || upper_active[col] {
            let square = step[col] * step[col];
            if !square.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
            active_square_sum += square;
            if !active_square_sum.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
        }
    }
    let r2 = initial_radius * initial_radius - active_square_sum;
    if !r2.is_finite() || r2 < 0.0 || r2 == 0.0 || !free.iter().any(|value| *value) {
        return Err(M1TrustRegionRefusalKind::DegenerateFace);
    }
    let free_ids: Vec<usize> = (0..N).filter(|&index| free[index]).collect();
    let mut residual = [0.0; N];
    for row in 0..N {
        let mut active_product_sum = 0.0;
        for col in 0..N {
            if lower_active[col] || upper_active[col] {
                let term = weighted_matrix[row][col] * step[col];
                if !term.is_finite() {
                    return Err(M1TrustRegionRefusalKind::SvdNonFinite);
                }
                active_product_sum += term;
                if !active_product_sum.is_finite() {
                    return Err(M1TrustRegionRefusalKind::SvdNonFinite);
                }
            }
        }
        residual[row] = weighted_residual[row] + active_product_sum;
        if !residual[row].is_finite() {
            return Err(M1TrustRegionRefusalKind::SvdNonFinite);
        }
    }
    let matrix = (0..N)
        .map(|row| {
            free_ids
                .iter()
                .map(|&col| weighted_matrix[row][col])
                .collect()
        })
        .collect();
    Ok(FacePreparation {
        radius: r2.sqrt(),
        free_ids,
        residual,
        matrix,
    })
}

fn first_crossing(
    step: &[f64; 21],
    previous_step: &[f64; 21],
    free_ids: &[usize],
    lower: &[f64; 21],
    upper: &[f64; 21],
) -> Result<Option<(f64, usize, bool)>, M1TrustRegionRefusalKind> {
    let mut crossing = None;
    for &col in free_ids {
        let bound = if step[col] < lower[col] {
            Some((lower[col], true))
        } else if step[col] > upper[col] {
            Some((upper[col], false))
        } else {
            None
        };
        if let Some((bound, is_lower)) = bound {
            let theta = (bound - previous_step[col]) / (step[col] - previous_step[col]);
            if !theta.is_finite() || !(0.0..=1.0).contains(&theta) {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
            if crossing.is_none_or(|(best, index, _)| {
                theta < best || (theta.partial_cmp(&best) == Some(Ordering::Equal) && col < index)
            }) {
                crossing = Some((theta, col, is_lower));
            }
        }
    }
    Ok(crossing)
}

fn largest_kkt_release(
    lower_active: &[bool; 21],
    upper_active: &[bool; 21],
    h: &[f64; 21],
) -> Option<(usize, bool)> {
    let mut release = None;
    for col in 0..21 {
        let violation = if lower_active[col] && h[col] < 0.0 {
            Some((-h[col], true))
        } else if upper_active[col] && h[col] > 0.0 {
            Some((h[col], false))
        } else {
            None
        };
        if let Some((size, is_lower)) = violation {
            if release.is_none_or(|(best, index, _)| {
                size > best || (size.partial_cmp(&best) == Some(Ordering::Equal) && col < index)
            }) {
                release = Some((size, col, is_lower));
            }
        }
    }
    release.map(|(_, col, is_lower)| (col, is_lower))
}

fn prepare_subproblem(
    input: &super::M1TrustRegionSubproblemInput,
) -> Result<PreparedSubproblem, super::M1TrustRegionSubproblemRefusal> {
    const N: usize = 21;
    let mut raw_max = 0.0_f64;
    for row in 0..N {
        raw_max = raw_max.max(input.raw_residual[row].abs());
        if !input.raw_residual[row].is_finite()
            || !input.normalizers[row].is_finite()
            || input.normalizers[row] <= 0.0
            || !input.base[row].is_finite()
            || !input.scales[row].is_finite()
            || input.scales[row] <= 0.0
            || input.lower[row].is_nan()
            || input.upper[row].is_nan()
            || input.lower[row] > input.upper[row]
            || input.base[row] < input.lower[row]
            || input.base[row] > input.upper[row]
        {
            return Err(subproblem_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                1,
                input.test_work.completed_jacobi_sweeps,
                input.test_work.completed_face_pivots,
                0,
                raw_max,
            ));
        }
    }
    if !input.initial_radius.is_finite() || input.initial_radius <= 0.0 {
        return Err(subproblem_refusal(
            M1TrustRegionRefusalKind::DegenerateFace,
            1,
            input.test_work.completed_jacobi_sweeps,
            input.test_work.completed_face_pivots,
            0,
            raw_max,
        ));
    }
    let rows: Vec<&[f64]> = input.raw_jacobian.iter().map(|row| &row[..]).collect();
    let (weighted_residual, weighted_matrix) = weighted_system(
        &input.raw_residual,
        &rows,
        &input.normalizers,
        &input.scales,
    )
    .map_err(|_| {
        subproblem_refusal(
            M1TrustRegionRefusalKind::SvdNonFinite,
            1,
            input.test_work.completed_jacobi_sweeps,
            input.test_work.completed_face_pivots,
            0,
            raw_max,
        )
    })?;
    let mut scaled_lower = [0.0; N];
    let mut scaled_upper = [0.0; N];
    for col in 0..N {
        scaled_lower[col] = (input.lower[col] - input.base[col]) / input.scales[col];
        scaled_upper[col] = (input.upper[col] - input.base[col]) / input.scales[col];
        if scaled_lower[col].is_nan()
            || scaled_upper[col].is_nan()
            || scaled_lower[col] > scaled_upper[col]
            || (input.lower[col].is_finite() && !scaled_lower[col].is_finite())
            || (input.upper[col].is_finite() && !scaled_upper[col].is_finite())
        {
            return Err(subproblem_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                1,
                input.test_work.completed_jacobi_sweeps,
                input.test_work.completed_face_pivots,
                0,
                raw_max,
            ));
        }
    }
    Ok(PreparedSubproblem {
        raw_max,
        weighted_residual,
        weighted_matrix,
        scaled_lower,
        scaled_upper,
    })
}

fn candidate_coordinates(
    input: &super::M1TrustRegionSubproblemInput,
    step: &[f64; 21],
) -> Option<[f64; 21]> {
    let mut candidate = [0.0; 21];
    for col in 0..21 {
        candidate[col] = input.base[col] + input.scales[col] * step[col];
        if !candidate[col].is_finite() {
            return None;
        }
    }
    Some(candidate)
}

fn raw_jacobian_columns(input: &super::M1TrustRegionSubproblemInput) -> [[f64; 21]; 21] {
    let mut columns = [[0.0; 21]; 21];
    for (col, column) in columns.iter_mut().enumerate() {
        for (row, entry) in column.iter_mut().enumerate() {
            *entry = input.raw_jacobian[row][col];
        }
    }
    columns
}

struct FaceSolution {
    free_ids: Vec<usize>,
    trace: LambdaTrace,
}

fn solve_face(
    input: &super::M1TrustRegionSubproblemInput,
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
) -> Result<FaceSolution, super::M1TrustRegionSubproblemRefusal> {
    let face = prepare_face(
        input.initial_radius,
        &state.p,
        &state.lower_active,
        &state.upper_active,
        &state.free,
        &prepared.weighted_residual,
        &prepared.weighted_matrix,
    )
    .map_err(|kind| state_refusal(kind, state, prepared.raw_max))?;
    state.entered += 1;
    let factor = match jacobi_factor(
        face.matrix,
        &face.free_ids,
        input.test_work.completed_jacobi_sweeps,
        &mut state.completed_sweeps,
    ) {
        Ok(value) => value,
        Err("rank deficient") => {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::RankDeficient,
                state,
                prepared.raw_max,
            ));
        }
        Err("no convergence") => {
            return Err(subproblem_refusal(
                M1TrustRegionRefusalKind::SvdNoConvergence,
                1,
                64,
                state.completed_pivots,
                state.entered,
                prepared.raw_max,
            ));
        }
        Err(_) => {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                state,
                prepared.raw_max,
            ));
        }
    };
    state.completed_sweeps = state.completed_sweeps.max(factor.sweeps);
    let mut trace = LambdaTrace::default();
    svd_lambda_ball(&factor, &face.residual, face.radius, None, &mut trace)
        .map_err(|kind| state_refusal(kind, state, prepared.raw_max))?;
    Ok(FaceSolution {
        free_ids: face.free_ids,
        trace,
    })
}

fn activate_first_crossing(
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
    solution: &FaceSolution,
) -> Result<bool, super::M1TrustRegionSubproblemRefusal> {
    let previous_p = state.p;
    for (slot, &col) in solution.free_ids.iter().enumerate() {
        state.p[col] = solution.trace.step[slot];
    }
    let crossing = first_crossing(
        &state.p,
        &previous_p,
        &solution.free_ids,
        &prepared.scaled_lower,
        &prepared.scaled_upper,
    )
    .map_err(|kind| state_refusal(kind, state, prepared.raw_max))?;
    let Some((theta, col, is_lower)) = crossing else {
        return Ok(false);
    };
    if state.completed_pivots >= 42 {
        return Err(state_refusal(
            M1TrustRegionRefusalKind::FacePivotLimit,
            state,
            prepared.raw_max,
        ));
    }
    for &id in &solution.free_ids {
        state.p[id] = previous_p[id] + theta * (state.p[id] - previous_p[id]);
        if !state.p[id].is_finite() {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                state,
                prepared.raw_max,
            ));
        }
    }
    state.p[col] = if is_lower {
        prepared.scaled_lower[col]
    } else {
        prepared.scaled_upper[col]
    };
    state.free[col] = false;
    state.lower_active[col] = is_lower;
    state.upper_active[col] = !is_lower;
    state.completed_pivots += 1;
    state.events.push(super::M1TrustRegionKktEvent {
        action: if is_lower {
            super::M1TrustRegionKktAction::ActivateLower
        } else {
            super::M1TrustRegionKktAction::ActivateUpper
        },
        selected_coordinate: col,
        lower_mask: state.lower_active,
        upper_mask: state.upper_active,
        free_mask: state.free,
        g: None,
        lambda_times_p: None,
        h: None,
    });
    Ok(true)
}

fn kkt_transition(
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
    lambda: f64,
) -> Result<Option<KktValues>, super::M1TrustRegionSubproblemRefusal> {
    let values = box_feasible_kkt_values(
        &prepared.weighted_matrix,
        &prepared.weighted_residual,
        &state.p,
        lambda,
    )
    .map_err(|kind| state_refusal(kind, state, prepared.raw_max))?;
    if let Some((col, is_lower)) =
        largest_kkt_release(&state.lower_active, &state.upper_active, &values.h)
    {
        if state.completed_pivots >= 42 {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::FacePivotLimit,
                state,
                prepared.raw_max,
            ));
        }
        state.events.push(super::M1TrustRegionKktEvent {
            action: if is_lower {
                super::M1TrustRegionKktAction::ReleaseLower
            } else {
                super::M1TrustRegionKktAction::ReleaseUpper
            },
            selected_coordinate: col,
            lower_mask: state.lower_active,
            upper_mask: state.upper_active,
            free_mask: state.free,
            g: Some(values.gradient),
            lambda_times_p: Some(values.lambda_times_step),
            h: Some(values.h),
        });
        state.lower_active[col] = false;
        state.upper_active[col] = false;
        state.free[col] = true;
        state.completed_pivots += 1;
        return Ok(None);
    }
    state.events.push(super::M1TrustRegionKktEvent {
        action: super::M1TrustRegionKktAction::Return,
        selected_coordinate: usize::MAX,
        lower_mask: state.lower_active,
        upper_mask: state.upper_active,
        free_mask: state.free,
        g: Some(values.gradient),
        lambda_times_p: Some(values.lambda_times_step),
        h: Some(values.h),
    });
    Ok(Some(values))
}

pub(crate) fn solve_subproblem(
    input: &super::M1TrustRegionSubproblemInput,
) -> Result<super::M1TrustRegionSubproblemObservation, super::M1TrustRegionSubproblemRefusal> {
    const N: usize = 21;
    let prepared = prepare_subproblem(input)?;
    let mut state = SubproblemState::new(input);
    loop {
        let solution = solve_face(input, &prepared, &mut state)?;
        if activate_first_crossing(&prepared, &mut state, &solution)? {
            continue;
        }
        let Some(values) = kkt_transition(&prepared, &mut state, solution.trace.lambda)? else {
            continue;
        };
        let candidate = candidate_coordinates(input, &state.p).ok_or_else(|| {
            state_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                &state,
                prepared.raw_max,
            )
        })?;
        return Ok(super::M1TrustRegionSubproblemObservation {
            initial_face_step: [0.0; N],
            base_coordinates: input.base,
            candidate_coordinates: candidate,
            initial_lower_mask: [false; N],
            initial_upper_mask: [false; N],
            initial_free_mask: state.initial_free,
            scaled_step: state.p,
            final_residual: values.residual,
            final_g: values.gradient,
            final_lambda_times_p: values.lambda_times_step,
            final_h: values.h,
            lambda: solution.trace.lambda,
            event_trace: state.events,
            entered_svd_factorizations: state.entered,
            completed_face_pivots: state.completed_pivots,
            raw_jacobian_columns: raw_jacobian_columns(input),
            active_lower: state.lower_active,
            active_upper: state.upper_active,
        });
    }
}

pub(crate) fn m1_trust_region_solve_subproblem_for_test(
    _input: &super::M1CoupledColumnInput,
    _trial: &super::M1CoupledColumnTrial,
    input: &super::M1TrustRegionSubproblemInput,
) -> Result<super::M1TrustRegionSubproblemObservation, super::M1TrustRegionSubproblemRefusal> {
    solve_subproblem(input)
}
