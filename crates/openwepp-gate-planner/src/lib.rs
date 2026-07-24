//! Deterministic, fail-closed ADR-0039 gate planning and evidence verification.
//!
//! The planner and verifier remain shadow-only policy components. The executor
//! consumes only independently reconstructable plans and emits local,
//! untrusted receipts; it never promotes trust, certifies a campaign, or
//! mutates Git, assurance, or repository policy.

#![forbid(unsafe_code)]

mod artifact_contract;

mod assurance;
pub mod canonical;
mod checkpoint_mirror;
mod documentation;
pub mod error;
mod execution_context;
mod execution_nextest;
mod execution_temp;
pub mod executor;
mod executor_source;
pub mod ledger;
mod nextest_inventory;
pub mod package_validation;
pub mod planner;
pub mod policy;
pub mod pre_heavy;
pub mod repository;
pub mod resume;
pub mod verifier;

pub use error::{ErrorClass, GatePolicyError, Result};
