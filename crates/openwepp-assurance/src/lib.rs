//! Deterministic compiler for openWEPP scientific assurance dossiers.

mod authoring;
mod engine;
mod error;
mod graph;
mod hash;
mod model;
mod path;
mod publication;
mod render;
mod review;
mod snapshot;

pub mod cli;

pub use engine::{Assurance, BuildOptions, BuildResult, Plan, Selection};
pub use error::{AssuranceError, Result};
pub use graph::{DependencyGraph, Node, NodeKind};
pub use hash::{sha256_bytes, sha256_file};
pub use model::{EmpiricalStatus, Lifecycle, VerificationStatus};
