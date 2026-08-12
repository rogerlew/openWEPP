use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum VegetationError {
    #[error("VEG-E-ID-001: model definition digest mismatch: expected {expected}, found {found}")]
    ModelDigestMismatch { expected: String, found: String },
    #[error("VEG-E-SCHEMA-001: invalid canonical input: {0}")]
    Schema(String),
    #[error("VEG-E-DOM-001: nonfinite or out-of-domain operand {0}")]
    Domain(&'static str),
    #[error("VEG-E-UNSUPPORTED-001: unsupported branch {0}")]
    Unsupported(&'static str),
    #[error("VEG-E-NUM-001: quadratic has a materially negative discriminant")]
    QuadraticDomain,
    #[error("VEG-E-NUM-002: leaf ci solve did not converge")]
    CiNonConvergence,
    #[error("VEG-E-NUM-004: hydraulic solve failed: {0}")]
    Hydraulic(&'static str),
    #[error("VEG-E-TRANSACTION-001: resource receipt is invalid: {0}")]
    Receipt(String),
    #[error("VEG-E-CLOSURE-001: {ledger} residual {residual} exceeds tolerance")]
    Closure { ledger: &'static str, residual: f64 },
}
