//! Deterministic, fail-closed ADR-0039 gate planning and evidence verification.
//!
//! The planner and verifier remain shadow-only policy components. The executor
//! consumes only independently reconstructable plans and emits local,
//! untrusted receipts; it never promotes trust, certifies a campaign, or
//! mutates Git, assurance, or repository policy.

#![forbid(unsafe_code)]

mod assurance;
pub mod canonical;
mod documentation;
pub mod error;
mod execution_context;
pub mod executor;
pub mod ledger;
pub mod planner;
pub mod policy;
pub mod repository;
pub mod verifier;

pub use error::{ErrorClass, GatePolicyError, Result};
