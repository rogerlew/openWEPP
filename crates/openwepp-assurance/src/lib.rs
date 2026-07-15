//! Fail-closed scientific-assurance source and transition tooling.
//!
//! The ASSURE-03 public builder remains fixed at zero reports. ASSURE-04A adds
//! offline admission of internal manuscript-first v2 sources without rendering,
//! approval, export, or publication authority.

mod engine;
mod error;
mod hash;
mod v2;

pub mod cli;

pub use engine::{Assurance, BuildOptions, BuildResult, Plan};
pub use error::{AssuranceError, Result};
pub use hash::{sha256_bytes, sha256_file};
pub use v2::{V2ReportSummary, V2Repository, V2ValidationSummary};
