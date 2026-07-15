//! Fail-closed zero-report transition tool for openWEPP scientific assurance.
//!
//! ASSURE-03 deliberately supports only the neutral state between retirement of
//! the v1 status-first publisher and implementation of the v2 manuscript-first
//! builder. Any nonempty legacy dossier catalog is rejected.

mod engine;
mod error;
mod hash;

pub mod cli;

pub use engine::{Assurance, BuildOptions, BuildResult, Plan};
pub use error::{AssuranceError, Result};
pub use hash::{sha256_bytes, sha256_file};
