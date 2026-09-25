//! Test-only arithmetic oracle for COLD-CANOPY-M1-TR-SVD-BVLS-02.
//!
//! This is deliberately separate from the Stage 1 body.  It evaluates the
//! contract's ordered binary64 KKT expression on supplied operands; it does
//! not factor a matrix, choose a face, or supply an outcome to the solver.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BoundClass {
    Lower,
    Upper,
    Free,
    Fixed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Failure {
    Nonfinite,
    NegativeLambda,
    NonNormalOperand,
    SubnormalIntermediate,
    UnderflowToZero,
    ZeroAddition,
    UpwardEnclosure,
    ReleaseRatio,
    FreeStationarity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Coordinate {
    pub(crate) h: f64,
    pub(crate) cbar: f64,
    pub(crate) tau: f64,
}

fn normal_or_zero(value: f64) -> Result<(), Failure> {
    if !value.is_finite() {
        Err(Failure::Nonfinite)
    } else if value != 0.0 && !value.is_normal() {
        Err(Failure::NonNormalOperand)
    } else {
        Ok(())
    }
}

fn checked(value: f64) -> Result<f64, Failure> {
    if !value.is_finite() {
        Err(Failure::Nonfinite)
    } else if value != 0.0 && !value.is_normal() {
        Err(Failure::SubnormalIntermediate)
    } else {
        Ok(value)
    }
}

fn next_up(value: f64) -> Result<f64, Failure> {
    if !value.is_finite() || value < 0.0 {
        return Err(Failure::UpwardEnclosure);
    }
    if value == 0.0 {
        return Ok(f64::from_bits(1));
    }
    checked(f64::from_bits(value.to_bits().checked_add(1).ok_or(Failure::UpwardEnclosure)?))
        .map_err(|_| Failure::UpwardEnclosure)
}

// These are the literal amendment definitions.  The zero right-addend rule
// matters because a nonzero term entering a zero accumulator still steps up.
fn up_mul(left: f64, right: f64) -> Result<f64, Failure> {
    normal_or_zero(left)?;
    normal_or_zero(right)?;
    if left == 0.0 || right == 0.0 {
        return Ok(0.0);
    }
    let product = left * right;
    if product == 0.0 {
        return Err(Failure::UnderflowToZero);
    }
    checked(product)?;
    next_up(product)
}

fn up_add(left: f64, right: f64) -> Result<f64, Failure> {
    normal_or_zero(left)?;
    normal_or_zero(right)?;
    if right == 0.0 {
        return Ok(left);
    }
    let sum = left + right;
    if sum == 0.0 {
        return Err(Failure::ZeroAddition);
    }
    checked(sum)?;
    next_up(sum)
}

fn abs_checked(value: f64) -> Result<f64, Failure> {
    normal_or_zero(value)?;
    Ok(value.abs())
}

fn kkt_add(left: f64, right: f64) -> Result<f64, Failure> {
    normal_or_zero(left)?;
    normal_or_zero(right)?;
    let sum = left + right;
    if sum == 0.0 && !(left == 0.0 && right == 0.0) && left.to_bits() != (-right).to_bits() {
        return Err(Failure::ZeroAddition);
    }
    checked(sum)
}

/// Evaluates the fixed 21-coordinate expression in ascending row/column order.
pub(crate) fn coordinate(
    matrix: &[[f64; 21]; 21],
    residual: &[f64; 21],
    step: &[f64; 21],
    lambda: f64,
    index: usize,
) -> Result<Coordinate, Failure> {
    if lambda < 0.0 {
        return Err(Failure::NegativeLambda);
    }
    normal_or_zero(lambda)?;
    let mut residual_hat = [0.0; 21];
    for row in 0..21 {
        normal_or_zero(residual[row])?;
        let mut total = residual[row];
        for column in 0..21 {
            normal_or_zero(matrix[row][column])?;
            normal_or_zero(step[column])?;
            let product = matrix[row][column] * step[column];
            if product == 0.0 && matrix[row][column] != 0.0 && step[column] != 0.0 {
                return Err(Failure::UnderflowToZero);
            }
            checked(product)?;
            total = kkt_add(total, product)?;
        }
        residual_hat[row] = total;
    }
    let mut gradient = 0.0;
    for row in 0..21 {
        let product = matrix[row][index] * residual_hat[row];
        if product == 0.0 && matrix[row][index] != 0.0 && residual_hat[row] != 0.0 {
            return Err(Failure::UnderflowToZero);
        }
        checked(product)?;
        gradient = kkt_add(gradient, product)?;
    }
    let lambda_step = lambda * step[index];
    if lambda_step == 0.0 && lambda != 0.0 && step[index] != 0.0 {
        return Err(Failure::UnderflowToZero);
    }
    checked(lambda_step)?;
    let h = kkt_add(gradient, lambda_step)?;

    let mut cbar = 0.0;
    for row in 0..21 {
        let mut inner = 0.0;
        for column in 0..21 {
            inner = up_add(inner, up_mul(abs_checked(matrix[row][column])?, abs_checked(step[column])?)?)?;
        }
        let row_scale = up_add(abs_checked(residual[row])?, inner)?;
        cbar = up_add(cbar, up_mul(abs_checked(matrix[row][index])?, row_scale)?)?;
    }
    cbar = up_add(cbar, up_mul(abs_checked(lambda)?, abs_checked(step[index])?)?)?;
    if cbar == 0.0 {
        if h != 0.0 {
            return Err(Failure::UpwardEnclosure);
        }
        return Ok(Coordinate { h, cbar, tau: 0.0 });
    }
    checked(cbar)?;
    // upward(87 / (2^53 - 87)); the denominator is exact in binary64.
    let gamma = next_up(87.0 / ((1_u64 << 53) as f64 - 87.0))?;
    let tau = up_mul(gamma, cbar)?;
    Ok(Coordinate { h, cbar, tau })
}

pub(crate) fn decision(value: Coordinate, class: BoundClass) -> Result<bool, Failure> {
    match class {
        BoundClass::Fixed => Ok(false),
        BoundClass::Lower => Ok(value.h < -value.tau),
        BoundClass::Upper => Ok(value.h > value.tau),
        BoundClass::Free => {
            if value.h.abs() <= value.tau {
                Ok(false)
            } else {
                Err(Failure::FreeStationarity)
            }
        }
    }
}

pub(crate) fn release_ratio(value: Coordinate) -> Result<f64, Failure> {
    if value.tau == 0.0 {
        return Err(Failure::ReleaseRatio);
    }
    let ratio = value.h.abs() / value.tau;
    if !ratio.is_finite() || !ratio.is_normal() {
        return Err(Failure::ReleaseRatio);
    }
    Ok(ratio)
}

pub(crate) enum Assessment {
    Release { coordinate: usize, lower: bool },
    Return,
    Indeterminate(Failure),
}

/// Applies the all-coordinate policy after a face solution.  Fixed coordinates
/// are deliberately absent from every release and free-stationarity test.
pub(crate) fn assess(
    matrix: &[[f64; 21]; 21],
    residual: &[f64; 21],
    step: &[f64; 21],
    lambda: f64,
    classes: &[BoundClass; 21],
) -> Assessment {
    let mut selected: Option<(f64, usize, bool)> = None;
    let mut free_failure = false;
    for index in 0..21 {
        let value = match coordinate(matrix, residual, step, lambda, index) {
            Ok(value) => value,
            Err(reason) => return Assessment::Indeterminate(reason),
        };
        match classes[index] {
            BoundClass::Lower if value.h < -value.tau => {
                let ratio = if value.tau == 0.0 { f64::INFINITY } else {
                    match release_ratio(value) { Ok(ratio) => ratio, Err(reason) => return Assessment::Indeterminate(reason) }
                };
                if selected.is_none_or(|(best, _, _)| ratio > best) {
                    selected = Some((ratio, index, true));
                }
            }
            BoundClass::Upper if value.h > value.tau => {
                let ratio = if value.tau == 0.0 { f64::INFINITY } else {
                    match release_ratio(value) { Ok(ratio) => ratio, Err(reason) => return Assessment::Indeterminate(reason) }
                };
                if selected.is_none_or(|(best, _, _)| ratio > best) {
                    selected = Some((ratio, index, false));
                }
            }
            BoundClass::Free if value.h.abs() > value.tau => free_failure = true,
            _ => {}
        }
    }
    if let Some((_, coordinate, lower)) = selected {
        Assessment::Release { coordinate, lower }
    } else if free_failure {
        Assessment::Indeterminate(Failure::FreeStationarity)
    } else {
        Assessment::Return
    }
}

#[cfg(all(test, feature = "m1-trust-region-analytic-stage"))]
mod tests {
    use super::{coordinate, decision, release_ratio, BoundClass, Failure};
    use serde_json::Value;

    const CAPTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/face-pivot-observed-capture.json"
    ));

    fn bits(value: &Value) -> f64 {
        f64::from_bits(u64::from_str_radix(value.as_str().expect("hex binary64"), 16).expect("bits"))
    }

    fn vector(value: &Value) -> [f64; 21] {
        std::array::from_fn(|i| bits(&value[i]))
    }

    fn matrix(value: &Value) -> [[f64; 21]; 21] {
        std::array::from_fn(|row| vector(&value[row]))
    }

    #[test]
    fn retained_capture_exercises_revised_all_coordinate_decision() {
        let capture: Value = serde_json::from_str(CAPTURE).expect("authenticated retained capture");
        let release = capture["operations"]
            .as_array()
            .expect("operations")
            .iter()
            .find(|operation| operation["action"] == "release")
            .expect("first recorded release");
        let a = matrix(&capture["weighted_matrix"]);
        let f = vector(&capture["weighted_residual"]);
        let p = vector(&release["p_before"]);
        let lambda = bits(&release["lambda"]);

        // These classifications come from the retained masks and the frozen
        // arithmetic, not from a fixture-selected tolerance.  The recorded
        // old body must be replaced because it reaches its pivot cap instead
        // of carrying this all-coordinate decision to its typed outcome.
        let lower = coordinate(&a, &f, &p, lambda, 5).expect("normal retained lower KKT");
        assert!(decision(lower, BoundClass::Lower).expect("resolvable lower sign"));
        assert!(release_ratio(lower).expect("checked lower ranking") > 1.0);
        let free = coordinate(&a, &f, &p, lambda, 11).expect("normal retained free KKT");
        assert_eq!(decision(free, BoundClass::Free), Err(Failure::FreeStationarity));
    }

    #[test]
    fn zero_subnormal_nonfinite_and_ratio_operands_fail_closed() {
        let mut a = [[0.0; 21]; 21];
        let mut f = [0.0; 21];
        let mut p = [0.0; 21];
        a[0][0] = 1.0;
        f[0] = 1.0;
        p[0] = 1.0;
        assert!(coordinate(&a, &f, &p, 0.0, 0).is_ok());
        a[0][0] = f64::from_bits(1);
        assert_eq!(coordinate(&a, &f, &p, 0.0, 0), Err(Failure::NonNormalOperand));
        a[0][0] = 1.0;
        f[0] = f64::NAN;
        assert_eq!(coordinate(&a, &f, &p, 0.0, 0), Err(Failure::Nonfinite));
        f[0] = 1.0;
        p[0] = f64::MIN_POSITIVE;
        a[0][0] = f64::MIN_POSITIVE;
        assert_eq!(coordinate(&a, &f, &p, 0.0, 0), Err(Failure::UnderflowToZero));
    }

    #[test]
    fn exact_bounds_signed_zero_and_fixed_classification_do_not_create_a_release() {
        let mut a = [[0.0; 21]; 21];
        let mut f = [0.0; 21];
        let p = [0.0; 21];
        a[0][0] = 1.0;
        f[0] = -0.0;
        let at_signed_zero = coordinate(&a, &f, &p, 0.0, 0).expect("signed zero KKT");
        assert!(!decision(at_signed_zero, BoundClass::Lower).expect("no signed-zero descent release"));
        assert!(!decision(at_signed_zero, BoundClass::Upper).expect("no signed-zero ascent release"));
        assert!(!decision(at_signed_zero, BoundClass::Fixed).expect("fixed is outside release tests"));
    }

    #[test]
    fn cancellation_band_blocks_old_sign_only_release_under_scale_and_permutation() {
        let mut a = [[0.0; 21]; 21];
        let mut f = [0.0; 21];
        let mut p = [0.0; 21];
        a[0][0] = 1.0;
        a[0][1] = 1.0;
        f[0] = -1.0 - f64::EPSILON;
        p[1] = 1.0;
        let lower = coordinate(&a, &f, &p, 0.0, 0).expect("normal cancellation band");
        assert!(lower.h < 0.0, "old sign-only rule would release this lower bound");
        assert!(!decision(lower, BoundClass::Lower).expect("enclosure makes the sign unresolved"));

        // A positive power-of-two row scaling and a coordinate permutation
        // preserve this dimensionless decision when the corresponding p and
        // matrix columns are moved together.
        a[0][0] *= 8.0;
        a[0][1] *= 8.0;
        f[0] *= 8.0;
        let scaled = coordinate(&a, &f, &p, 0.0, 0).expect("scaled normal band");
        assert!(!decision(scaled, BoundClass::Lower).expect("scaled unresolved sign"));
        let mut permuted = [[0.0; 21]; 21];
        permuted[0][0] = a[0][1];
        permuted[0][1] = a[0][0];
        let mut permuted_p = [0.0; 21];
        permuted_p[0] = p[1];
        let upper = coordinate(&permuted, &f, &permuted_p, 0.0, 1)
            .expect("permuted normal band");
        assert!(!decision(upper, BoundClass::Lower).expect("permuted unresolved sign"));
    }
}
