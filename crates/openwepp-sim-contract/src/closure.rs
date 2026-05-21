/// Closure violation severity used by orchestration promotion gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClosureSeverity {
    HardFail,
    GovernanceHold,
}

/// Typed closure/invariant violation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClosureViolationKind {
    NonFinite,
    DomainLowerBound,
    DomainUpperBound,
    DomainRange,
    ResidualExceeded,
    CardinalityMismatch,
}

/// Structured payload for closure violation diagnostics.
#[derive(Debug, Clone, PartialEq)]
pub enum ClosureViolationDetails {
    Scalar {
        value: f64,
    },
    LowerBound {
        value: f64,
        minimum: f64,
    },
    UpperBound {
        value: f64,
        maximum: f64,
    },
    Range {
        value: f64,
        minimum: f64,
        maximum: f64,
    },
    Residual {
        inputs_total: f64,
        outputs_total: f64,
        storage_delta: f64,
        residual: f64,
        tolerance: f64,
    },
    Cardinality {
        expected: usize,
        observed: usize,
    },
}

/// Typed closure/invariant violation.
#[derive(Debug, Clone, PartialEq)]
pub struct ClosureViolation {
    pub check_id: String,
    pub message_id: String,
    pub subject: String,
    pub kind: ClosureViolationKind,
    pub severity: ClosureSeverity,
    pub details: ClosureViolationDetails,
}

impl ClosureViolation {
    #[must_use]
    pub fn new(
        check_id: impl Into<String>,
        message_id: impl Into<String>,
        subject: impl Into<String>,
        kind: ClosureViolationKind,
        severity: ClosureSeverity,
        details: ClosureViolationDetails,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            message_id: message_id.into(),
            subject: subject.into(),
            kind,
            severity,
            details,
        }
    }
}

/// Standard result type for closure checks.
pub type ClosureCheckResult = Result<(), Box<ClosureViolation>>;

/// Assert a scalar is finite.
///
/// # Errors
///
/// Returns a typed `NonFinite` violation when `value` is `NaN` or infinite.
pub fn check_finite(
    check_id: &str,
    message_id: &str,
    subject: &str,
    value: f64,
) -> ClosureCheckResult {
    if value.is_finite() {
        Ok(())
    } else {
        Err(Box::new(ClosureViolation::new(
            check_id,
            message_id,
            subject,
            ClosureViolationKind::NonFinite,
            ClosureSeverity::HardFail,
            ClosureViolationDetails::Scalar { value },
        )))
    }
}

/// Assert a scalar is greater than or equal to `minimum`.
///
/// # Errors
///
/// Returns a typed `DomainLowerBound` violation when `value < minimum`.
pub fn check_min(
    check_id: &str,
    message_id: &str,
    subject: &str,
    value: f64,
    minimum: f64,
) -> ClosureCheckResult {
    if value >= minimum {
        Ok(())
    } else {
        Err(Box::new(ClosureViolation::new(
            check_id,
            message_id,
            subject,
            ClosureViolationKind::DomainLowerBound,
            ClosureSeverity::HardFail,
            ClosureViolationDetails::LowerBound { value, minimum },
        )))
    }
}

/// Assert a scalar is less than or equal to `maximum`.
///
/// # Errors
///
/// Returns a typed `DomainUpperBound` violation when `value > maximum`.
pub fn check_max(
    check_id: &str,
    message_id: &str,
    subject: &str,
    value: f64,
    maximum: f64,
) -> ClosureCheckResult {
    if value <= maximum {
        Ok(())
    } else {
        Err(Box::new(ClosureViolation::new(
            check_id,
            message_id,
            subject,
            ClosureViolationKind::DomainUpperBound,
            ClosureSeverity::HardFail,
            ClosureViolationDetails::UpperBound { value, maximum },
        )))
    }
}

/// Assert a scalar is in the closed interval `[minimum, maximum]`.
///
/// # Errors
///
/// Returns a typed `DomainRange` violation when bounds are invalid
/// (`minimum > maximum`) or when `value` is outside `[minimum, maximum]`.
pub fn check_range(
    check_id: &str,
    message_id: &str,
    subject: &str,
    value: f64,
    minimum: f64,
    maximum: f64,
) -> ClosureCheckResult {
    if minimum > maximum {
        return Err(Box::new(ClosureViolation::new(
            check_id,
            "CLOSURE-PRIMITIVE-INVALID-BOUNDS",
            subject,
            ClosureViolationKind::DomainRange,
            ClosureSeverity::HardFail,
            ClosureViolationDetails::Range {
                value,
                minimum,
                maximum,
            },
        )));
    }

    if (minimum..=maximum).contains(&value) {
        Ok(())
    } else {
        Err(Box::new(ClosureViolation::new(
            check_id,
            message_id,
            subject,
            ClosureViolationKind::DomainRange,
            ClosureSeverity::HardFail,
            ClosureViolationDetails::Range {
                value,
                minimum,
                maximum,
            },
        )))
    }
}

/// Assert a scalar is in the closed unit interval `[0, 1]`.
///
/// # Errors
///
/// Returns a typed `DomainRange` violation when `value` is outside `[0, 1]`.
pub fn check_unit_interval(
    check_id: &str,
    message_id: &str,
    subject: &str,
    value: f64,
) -> ClosureCheckResult {
    check_range(check_id, message_id, subject, value, 0.0, 1.0)
}

/// Assert conservation closure within tolerance.
///
/// Residual is computed as:
/// `residual = inputs_total - outputs_total - storage_delta`.
///
/// # Errors
///
/// Returns a typed `ResidualExceeded` violation when `tolerance < 0` or when
/// `abs(residual) > tolerance`.
pub fn check_balance_residual(
    check_id: &str,
    message_id: &str,
    subject: &str,
    inputs_total: f64,
    outputs_total: f64,
    storage_delta: f64,
    tolerance: f64,
) -> ClosureCheckResult {
    if tolerance < 0.0 {
        return Err(Box::new(ClosureViolation::new(
            check_id,
            "CLOSURE-PRIMITIVE-NEGATIVE-TOLERANCE",
            subject,
            ClosureViolationKind::ResidualExceeded,
            ClosureSeverity::HardFail,
            ClosureViolationDetails::Residual {
                inputs_total,
                outputs_total,
                storage_delta,
                residual: f64::NAN,
                tolerance,
            },
        )));
    }

    let residual = inputs_total - outputs_total - storage_delta;
    if residual.abs() <= tolerance {
        Ok(())
    } else {
        Err(Box::new(ClosureViolation::new(
            check_id,
            message_id,
            subject,
            ClosureViolationKind::ResidualExceeded,
            ClosureSeverity::HardFail,
            ClosureViolationDetails::Residual {
                inputs_total,
                outputs_total,
                storage_delta,
                residual,
                tolerance,
            },
        )))
    }
}

/// Assert expected and observed counts match.
///
/// # Errors
///
/// Returns a typed `CardinalityMismatch` violation when `expected != observed`.
pub fn check_equal_count(
    check_id: &str,
    message_id: &str,
    subject: &str,
    expected: usize,
    observed: usize,
) -> ClosureCheckResult {
    if expected == observed {
        Ok(())
    } else {
        Err(Box::new(ClosureViolation::new(
            check_id,
            message_id,
            subject,
            ClosureViolationKind::CardinalityMismatch,
            ClosureSeverity::HardFail,
            ClosureViolationDetails::Cardinality { expected, observed },
        )))
    }
}
