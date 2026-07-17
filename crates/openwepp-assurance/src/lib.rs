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
    V2AmendMode, V2AmendmentReceipt, V2AssemblyResult, V2AssemblySummary, V2Inspection,
    V2NormalizationChange, V2NormalizationMode, V2NormalizationOptions, V2NormalizationReceipt,
    V2Plan, V2PlanNode, V2PlanState, V2PublicationFault, V2PublicationOptions, V2PublicationResult,
    V2RecoveryAction, V2ReleaseIdentity, V2ReleaseVerification, V2ReportPlan, V2ReportSummary,
    V2Repository, V2ReviewRoots, V2TrustDomain, V2ValidationSummary, amend_attribution,
    amend_attribution_at_generation, amend_lifecycle, amend_lifecycle_at_generation,
    amend_normalize, amend_normalize_at_generation, amend_principal, amend_principal_at_generation,
    amend_role, amend_role_at_generation, copy_v2_test_fixture, inspect_report,
    rebind_implementation, rebind_invalid_v2_test_fixture, rebind_v2_test_fixture,
    recover_amendment, retain_v2_test_report, verify_generation, verify_v2_release_snapshot,
};
