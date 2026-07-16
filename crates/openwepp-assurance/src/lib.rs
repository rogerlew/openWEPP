//! Fail-closed scientific-assurance source and transition tooling.
//!
//! The tracked public builder remains fixed at zero reports. ASSURE-04D adds
//! fail-closed review-root, external-publication, immutable-snapshot, receipt,
//! and release-verification mechanics for manuscript-first v2 sources. The
//! mechanics validate declared authority; they do not create scientific
//! approval or authorize tracked publication, export, or vendoring.

mod engine;
mod error;
mod hash;
mod v2;

pub mod cli;

pub use engine::{Assurance, BuildOptions, BuildResult, Plan};
pub use error::{AssuranceError, Result};
pub use hash::{sha256_bytes, sha256_file};
pub use v2::{
    V2AssemblyResult, V2AssemblySummary, V2NormalizationChange, V2NormalizationMode,
    V2NormalizationOptions, V2NormalizationReceipt, V2Plan, V2PlanNode, V2PlanState,
    V2PublicationFault, V2PublicationOptions, V2PublicationResult, V2ReleaseIdentity,
    V2ReleaseVerification, V2ReportPlan, V2ReportSummary, V2Repository, V2ReviewRoots,
    V2TrustDomain, V2ValidationSummary, verify_v2_release_snapshot,
};
