use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::diagnostics::NumericalFailureDiagnostics;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NumericalFailureCategory {
    BacktrackingLimit,
    IterationLimit,
    SingularPivot,
    BracketFailure,
    Domain,
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum VegetationError {
    #[error("VEG-E-INJECT-001: injected failure at {0}")]
    InjectedFailure(&'static str),
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
    #[error("VEG-E-NUM-003: canopy energy solve failed: {0}")]
    Energy(&'static str),
    #[error("VEG-E-NUM-004: hydraulic solve failed: {0}")]
    Hydraulic(&'static str),
    #[error("VEG-E-NUM-005: coupled gas/energy/hydraulic solve failed: {0}")]
    Coupled(&'static str),
    #[error("VEG-E-NUM-006: radiation quadrature failed: {0}")]
    Radiation(&'static str),
    #[error("VEG-E-NUM-007: coupled numerical failure ({category:?}): {diagnostics:?}")]
    NumericalFailure {
        category: NumericalFailureCategory,
        diagnostics: Box<NumericalFailureDiagnostics>,
    },
    #[error("VEG-E-TRANSACTION-001: resource receipt is invalid: {0}")]
    Receipt(String),
    #[error("VEG-E-093: capped candidate rejected without owner mutation: {0}")]
    CappedCandidateRollback(&'static str),
    #[error("VEG-E-097: V7 allocation or owner ledger rejected without a candidate: {0}")]
    V7Candidate(&'static str),
    #[error("VEG-E-097: V7 {ledger} closure rejected with residual {residual}")]
    V7Closure { ledger: &'static str, residual: f64 },
    #[error("VEG-E-100: V7 candidate transaction rejected without owner mutation: {0}")]
    V7CandidateRollback(&'static str),
    #[error("VEG-E-CLOSURE-001: {ledger} residual {residual} exceeds tolerance")]
    Closure { ledger: &'static str, residual: f64 },
}
