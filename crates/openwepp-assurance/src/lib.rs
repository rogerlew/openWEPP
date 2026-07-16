//! Fail-closed scientific-assurance source and transition tooling.
//!
//! The public builder remains fixed at zero reports. ASSURE-04C adds
//! deterministic assembly and checking for internal manuscript-first v2
//! sources, but only under an explicit disposable staging root. It does not
//! grant approval, export, or publication authority.

mod engine;
mod error;
mod hash;
mod v2;

pub mod cli;

pub use engine::{Assurance, BuildOptions, BuildResult, Plan};
pub use error::{AssuranceError, Result};
pub use hash::{sha256_bytes, sha256_file};
pub use v2::{
    V2AssemblyResult, V2AssemblySummary, V2Plan, V2PlanNode, V2PlanState, V2ReportPlan,
    V2ReportSummary, V2Repository, V2ValidationSummary,
};
