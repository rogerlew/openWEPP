//! Deterministic, fail-closed ADR-0039 gate planning and evidence verification.
//!
//! This crate is shadow-only. It never executes planned gate commands or
//! mutates Git, campaign, assurance, CI, or evidence state.

#![forbid(unsafe_code)]

pub mod canonical;
pub mod error;
pub mod ledger;
pub mod planner;
pub mod policy;
pub mod repository;
pub mod verifier;

pub use error::{ErrorClass, GatePolicyError, Result};
