use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Catalog {
    pub schema_version: u32,
    pub contract_version: u32,
    pub generated_root: PathBuf,
    pub export_output: PathBuf,
    pub templates: Templates,
    pub dossiers: Vec<CatalogDossier>,
    pub shared_outputs: SharedOutputs,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Templates {
    pub index: PathBuf,
    pub method: PathBuf,
    pub dossier: PathBuf,
    pub worksheet: PathBuf,
}

impl Templates {
    pub(crate) fn ordered_paths(&self) -> [&PathBuf; 4] {
        [&self.index, &self.method, &self.dossier, &self.worksheet]
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SharedOutputs {
    pub index: PathBuf,
    pub worksheet: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CatalogDossier {
    pub dossier_id: String,
    pub dossier_version: String,
    pub lifecycle: Lifecycle,
    pub source: PathBuf,
    pub method: PathBuf,
    pub evidence: PathBuf,
    pub interpretation: PathBuf,
    pub limitations: PathBuf,
    pub authoring: PathBuf,
    pub review: PathBuf,
    pub outputs: DossierOutputs,
}

impl CatalogDossier {
    pub(crate) fn source_paths(&self) -> [&PathBuf; 7] {
        [
            &self.source,
            &self.method,
            &self.evidence,
            &self.interpretation,
            &self.limitations,
            &self.authoring,
            &self.review,
        ]
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DossierOutputs {
    pub dossier: PathBuf,
    pub method: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Lifecycle {
    Draft,
    Candidate,
    Published,
    Superseded,
    Withdrawn,
}

impl Lifecycle {
    #[must_use]
    pub const fn requires_review_lock(self) -> bool {
        matches!(self, Self::Published | Self::Superseded | Self::Withdrawn)
    }

    #[must_use]
    pub const fn snapshot_eligible(self) -> bool {
        !matches!(self, Self::Draft)
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Draft => "DRAFT",
            Self::Candidate => "CANDIDATE",
            Self::Published => "PUBLISHED",
            Self::Superseded => "SUPERSEDED",
            Self::Withdrawn => "WITHDRAWN",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
// Schema identity fields intentionally retain their unambiguous external names.
#[allow(clippy::struct_field_names)]
pub(crate) struct Method {
    pub schema_version: u32,
    pub method_id: String,
    pub version: String,
    pub title: String,
    pub owner: String,
    pub design: Design,
    pub question: String,
    pub quantities: Vec<String>,
    pub domain: Vec<String>,
    pub datasets: Vec<String>,
    pub metrics: Vec<String>,
    pub criteria: Vec<String>,
    pub uncertainty: Vec<String>,
    pub reproduction: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Design {
    Prospective,
    Retrospective,
}

impl Design {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Prospective => "prospective",
            Self::Retrospective => "retrospective",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
// Schema identity fields intentionally retain their unambiguous external names.
#[allow(clippy::struct_field_names)]
pub(crate) struct Dossier {
    pub schema_version: u32,
    pub dossier_id: String,
    pub version: String,
    pub title: String,
    pub lifecycle: Lifecycle,
    pub evidence_as_of: String,
    pub assessment_owner: String,
    pub method_id: String,
    pub narrative: NarrativeRef,
    pub question: String,
    pub quantities: Vec<String>,
    pub tested_domain: Vec<String>,
    pub verification: VerificationStatus,
    pub verification_obligations: Vec<VerificationObligation>,
    pub empirical: EmpiricalStatus,
    pub summary: String,
    pub applies_to: Vec<String>,
    pub unknowns: Vec<String>,
    pub application_boundary: String,
    pub source_identity: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NarrativeRef {
    pub path: PathBuf,
    pub doc_id: String,
    pub title: String,
    pub nav_key: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerificationObligation {
    pub obligation_id: String,
    pub title: String,
    pub mandatory: bool,
    pub status: VerificationStatus,
    pub realization: String,
    pub executed_on: Option<String>,
    pub requirement: String,
    pub tolerance: String,
    pub result: String,
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pass,
    Fail,
    Blocked,
    NotRun,
}

impl VerificationStatus {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Blocked => "BLOCKED",
            Self::NotRun => "NOT_RUN",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmpiricalStatus {
    CorroboratedWithinTestedDomain,
    MixedEvidence,
    ContradictedWithinTestedDomain,
    InsufficientEvidence,
    NotEvaluated,
}

impl EmpiricalStatus {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::CorroboratedWithinTestedDomain => "CORROBORATED_WITHIN_TESTED_DOMAIN",
            Self::MixedEvidence => "MIXED_EVIDENCE",
            Self::ContradictedWithinTestedDomain => "CONTRADICTED_WITHIN_TESTED_DOMAIN",
            Self::InsufficientEvidence => "INSUFFICIENT_EVIDENCE",
            Self::NotEvaluated => "NOT_EVALUATED",
        }
    }

    #[must_use]
    pub const fn is_favorable(self) -> bool {
        matches!(self, Self::CorroboratedWithinTestedDomain)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceManifest {
    pub schema_version: u32,
    pub dossier_id: String,
    pub entries: Vec<EvidenceEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceEntry {
    pub evidence_id: String,
    pub kind: EvidenceKind,
    pub role: String,
    pub path: Option<PathBuf>,
    pub location: Option<String>,
    pub sha256: Option<String>,
    pub availability: Availability,
    pub forcing: ForcingClass,
    pub claim_bearing: bool,
    pub note: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EvidenceKind {
    Verification,
    Empirical,
    Comparative,
    Provenance,
    Review,
}

impl EvidenceKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Verification => "verification",
            Self::Empirical => "empirical",
            Self::Comparative => "comparative",
            Self::Provenance => "provenance",
            Self::Review => "review",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Availability {
    Tracked,
    External,
    Restricted,
    Unavailable,
}

impl Availability {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Tracked => "tracked",
            Self::External => "external",
            Self::Restricted => "restricted",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ForcingClass {
    Robust,
    Limited,
    Mixed,
    NotApplicable,
}

impl ForcingClass {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Robust => "forcing-robust",
            Self::Limited => "forcing-limited",
            Self::Mixed => "mixed",
            Self::NotApplicable => "not applicable",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Review {
    pub schema_version: u32,
    pub dossier_id: String,
    pub dossier_version: String,
    pub conclusion_authors: Vec<ReviewAuthor>,
    pub approvals: Vec<ReviewApproval>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReviewAuthor {
    pub name: String,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReviewApproval {
    pub review_id: String,
    pub scope: ReviewScope,
    pub state: ReviewState,
    pub reviewers: Vec<ReviewParticipant>,
    pub review_date: Option<String>,
    pub reviewed_root_sha256: Option<String>,
    pub payload_sha256: Option<String>,
    pub disposition_summary: String,
    pub findings: Vec<ReviewFinding>,
    pub residual_disagreements: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReviewParticipant {
    pub name: String,
    pub role: String,
    pub expertise: String,
    pub independent_of_authors: bool,
    pub independence_basis: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReviewFinding {
    pub finding_id: String,
    pub severity: ReviewSeverity,
    pub summary: String,
    pub disposition: FindingDisposition,
    pub rationale: String,
    pub closure_blocking: bool,
    pub resolved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReviewState {
    Pending,
    Approved,
    Rejected,
}

impl ReviewState {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Pending => "PENDING",
            Self::Approved => "APPROVED",
            Self::Rejected => "REJECTED",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReviewScope {
    Scientific,
    Publication,
}

impl ReviewScope {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Scientific => "scientific",
            Self::Publication => "publication",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReviewSeverity {
    Low,
    Medium,
    High,
}

impl ReviewSeverity {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FindingDisposition {
    Accepted,
    Rejected,
    Deferred,
    FollowUp,
}

impl FindingDisposition {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
            Self::FollowUp => "follow-up",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AuthoringAnalysis {
    pub schema_version: u32,
    pub dossier_id: String,
    pub dossier_version: String,
    pub procedure_version: String,
    pub task: String,
    pub agent_identity: String,
    pub tool_version: String,
    pub execution_date: String,
    pub nondeterministic_settings: String,
    pub input_revision: String,
    pub inputs: Vec<AnalysisInput>,
    pub accepted_outputs: Vec<AnalysisOutput>,
    pub accepted_decisions: Vec<String>,
    pub accepted_output_root_sha256: String,
    pub review: AuthoringReview,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AnalysisInput {
    pub repository: AnalysisRepository,
    pub revision: String,
    pub path: PathBuf,
    pub sha256: String,
    pub availability: AnalysisAvailability,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AnalysisOutput {
    pub path: PathBuf,
    pub sha256: String,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AuthoringReview {
    pub state: ReviewState,
    pub reviewer: Option<ReviewParticipant>,
    pub review_date: Option<String>,
    pub findings: Vec<String>,
    pub disposition: Option<String>,
    pub approved_output_root_sha256: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AnalysisRepository {
    Openwepp,
    Wepppy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AnalysisAvailability {
    Tracked,
    Historical,
    External,
}

#[cfg(test)]
mod tests {
    use super::Lifecycle;

    #[test]
    fn only_drafts_are_snapshot_ineligible() {
        assert!(!Lifecycle::Draft.snapshot_eligible());
        for lifecycle in [
            Lifecycle::Candidate,
            Lifecycle::Published,
            Lifecycle::Superseded,
            Lifecycle::Withdrawn,
        ] {
            assert!(lifecycle.snapshot_eligible());
        }
    }
}
