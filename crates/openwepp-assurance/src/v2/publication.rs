use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Write as _};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::assembly::{ASSEMBLY_TOOL_ID, has_canonical_markdown_link};
use super::confined::ConfinedDirectory;
use super::{
    Approval, Finding, Principal, PrincipalKind, Publication, Report, ReportSource, Review,
    V2Repository, parse_yaml, read_identified, read_regular_confined, validate_catalog_binding,
    validate_report,
};
use crate::{AssuranceError, Result, sha256_bytes};

const SUBJECT_DOMAIN: &str = "openwepp-assurance-subject-v1";
const FINDING_DOMAIN: &str = "openwepp-assurance-findings-v1";
const APPROVAL_DOMAIN: &str = "openwepp-assurance-approvals-v1";
const TRANSFER_DOMAIN: &str = "openwepp-assurance-transfer-v1";
const SNAPSHOT_DOMAIN: &str = "openwepp-assurance-snapshot-v1";
const RECEIPT_DOMAIN: &str = "openwepp-assurance-receipt-v1";
const PLANNER_TOOL_ID: &str = "openwepp-assurance-planner:1";
const PUBLIC_FORMAT: &str = "openwepp-assurance-public:1";
const SNAPSHOT_FORMAT: &str = "openwepp-assurance-snapshot:1";
const RECEIPT_FORMAT: &str = "openwepp-assurance-receipt:1";
const TEST_BANNER: &str = "TEST ONLY — NOT SCIENTIFICALLY APPROVED";
const PUBLICATION_BUILDER_ID: &str =
    "openwepp-assurance-planner:1+openwepp-assurance-assembly:1+publication:1";
const ZERO_REPORT_README_SHA256: &str =
    "65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70";
const REVIEW_TRANSITION_FIELDS: &[&str] = &[
    "state",
    "decision",
    "subject_root",
    "review_charge",
    "build_maintainer_id",
    "material_producer_ids",
    "findings",
    "finding_ledger_root",
    "approvals",
    "approval_lock_root",
    "independence_assessment",
];
const PUBLICATION_TRANSITION_FIELDS: &[&str] = &[
    "state",
    "approval_lock_root",
    "target_release_commit",
    "target_release_configuration",
    "prior_realization",
    "candidate_realization",
    "impact_assessment",
    "reproduction_disposition",
    "semantic_differences",
    "release_owner_id",
    "assurance_steward_id",
    "publication_date",
    "public_path",
    "release_transfer_root",
    "export_authorized",
    "vendoring_authorized",
    "supersedes",
    "withdrawn",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V2TrustDomain {
    Production,
    TestOnly,
}

impl fmt::Display for V2TrustDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Production => "production",
            Self::TestOnly => "test_only",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct V2ReleaseIdentity {
    commit: String,
    configuration: String,
}

impl V2ReleaseIdentity {
    /// Creates an independently supplied release identity.
    ///
    /// # Errors
    ///
    /// Returns an error unless the commit and configuration are stable lexical
    /// identities.
    pub fn new(commit: impl Into<String>, configuration: impl Into<String>) -> Result<Self> {
        let identity = Self {
            commit: commit.into(),
            configuration: configuration.into(),
        };
        identity.validate()?;
        Ok(identity)
    }

    #[must_use]
    pub fn commit(&self) -> &str {
        &self.commit
    }

    #[must_use]
    pub fn configuration(&self) -> &str {
        &self.configuration
    }

    fn validate(&self) -> Result<()> {
        if self.commit.len() != 40
            || !self
                .commit
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            return Err(AssuranceError::Invalid(
                "release commit must be a full 40-character lowercase Git object ID".to_owned(),
            ));
        }
        if self.configuration.is_empty()
            || !self.configuration.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-')
            })
        {
            return Err(AssuranceError::Invalid(
                "release configuration must be a nonempty stable identity".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2PublicationOptions {
    staging_root: PathBuf,
    usersum_root: PathBuf,
    snapshot_root: PathBuf,
    release: V2ReleaseIdentity,
    fault_injection: Option<V2PublicationFault>,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V2PublicationFault {
    AfterSnapshotInstall,
    AfterReceiptInstall,
    BeforePublicCommit,
}

impl V2PublicationOptions {
    #[must_use]
    pub fn new(
        staging_root: PathBuf,
        usersum_root: PathBuf,
        snapshot_root: PathBuf,
        release: V2ReleaseIdentity,
    ) -> Self {
        Self {
            staging_root,
            usersum_root,
            snapshot_root,
            release,
            fault_injection: None,
        }
    }

    #[must_use]
    pub fn release(&self) -> &V2ReleaseIdentity {
        &self.release
    }

    #[doc(hidden)]
    #[must_use]
    pub fn with_fault_injection_for_test(mut self, fault: V2PublicationFault) -> Self {
        self.fault_injection = Some(fault);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2PublicationResult {
    pub report_ids: Vec<String>,
    pub snapshot_id: String,
    pub snapshot_path: PathBuf,
    pub receipt_id: String,
    pub receipt_path: PathBuf,
    pub public_tree_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2ReleaseVerification {
    pub report_ids: Vec<String>,
    pub snapshot_id: String,
    pub receipt_id: String,
    pub public_tree_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2ReviewRoots {
    pub report_id: String,
    pub subject_root: String,
    pub finding_ledger_root: Option<String>,
    pub approval_lock_root: Option<String>,
    pub release_transfer_root: Option<String>,
}

struct ReportContext {
    report: Report,
    report_value: Value,
    input_bytes: BTreeMap<PathBuf, Vec<u8>>,
    staged_bytes: BTreeMap<PathBuf, Vec<u8>>,
    roots: V2ReviewRoots,
    capabilities: Option<ContextCapabilities>,
}

struct ContextCapabilities {
    repository: ConfinedDirectory,
    staging: ConfinedDirectory,
}

struct HeldRootGuard<'a> {
    source: &'a ContextCapabilities,
    repository_path: &'a Path,
    staging_path: &'a Path,
    public: &'a ConfinedDirectory,
    public_path: &'a Path,
    snapshot: &'a ConfinedDirectory,
    snapshot_path: &'a Path,
}

#[derive(Default)]
struct ReceiptBindings {
    subjects: BTreeMap<String, String>,
    findings: BTreeMap<String, String>,
    approvals: BTreeMap<String, String>,
    transfers: BTreeMap<String, String>,
}

impl ReceiptBindings {
    fn from_receipt(receipt: &PublicationReceipt) -> Self {
        Self {
            subjects: receipt.subject_roots.clone(),
            findings: receipt.finding_ledger_roots.clone(),
            approvals: receipt.approval_lock_roots.clone(),
            transfers: receipt.release_transfer_roots.clone(),
        }
    }

    fn remove(&mut self, report_id: &str) {
        self.subjects.remove(report_id);
        self.findings.remove(report_id);
        self.approvals.remove(report_id);
        self.transfers.remove(report_id);
    }

    fn insert(&mut self, context: &ReportContext) -> Result<()> {
        let report_id = context.report.id.clone();
        self.subjects
            .insert(report_id.clone(), context.roots.subject_root.clone());
        self.findings.insert(
            report_id.clone(),
            context
                .roots
                .finding_ledger_root
                .clone()
                .ok_or_else(|| AssuranceError::Invalid("finding root is missing".to_owned()))?,
        );
        self.approvals.insert(
            report_id.clone(),
            context
                .roots
                .approval_lock_root
                .clone()
                .ok_or_else(|| AssuranceError::Invalid("approval root is missing".to_owned()))?,
        );
        self.transfers.insert(
            report_id,
            context
                .roots
                .release_transfer_root
                .clone()
                .ok_or_else(|| AssuranceError::Invalid("transfer root is missing".to_owned()))?,
        );
        Ok(())
    }

    fn clear(&mut self) {
        self.subjects.clear();
        self.findings.clear();
        self.approvals.clear();
        self.transfers.clear();
    }
}

struct PriorPublic {
    catalog: PublicCatalog,
    files: BTreeMap<PathBuf, Vec<u8>>,
    bindings: ReceiptBindings,
    source_payload: SnapshotPayload,
}

type SnapshotPayload = BTreeMap<PathBuf, (Vec<u8>, SnapshotFileKind)>;

struct PriorVerification {
    bindings: ReceiptBindings,
    source_payload: SnapshotPayload,
}

struct PreparedPublication {
    public_files: BTreeMap<PathBuf, Vec<u8>>,
    snapshot_payload: SnapshotPayload,
    bindings: ReceiptBindings,
    report_ids: Vec<String>,
    public_tree_sha256: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PublicCatalog {
    format: String,
    trust_domain: V2TrustDomain,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_marker: Option<String>,
    reports: Vec<PublicCatalogEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PublicCatalogEntry {
    report_id: String,
    version: String,
    title: String,
    scientific_question: String,
    assessed_process: String,
    assessed_quantity: String,
    realization: String,
    publication_date: String,
    report_path: String,
    supplement_path: String,
    related_model_narrative: String,
    subject_root: String,
    approval_lock_root: String,
    release_transfer_root: String,
    publication_state: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotManifest {
    format: String,
    domain: String,
    trust_domain: V2TrustDomain,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_marker: Option<String>,
    release: V2ReleaseIdentity,
    report_ids: Vec<String>,
    public_tree_sha256: String,
    files: Vec<SnapshotFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotFile {
    path: String,
    sha256: String,
    bytes: usize,
    kind: SnapshotFileKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SnapshotFileKind {
    Source,
    Public,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PublicationReceipt {
    format: String,
    domain: String,
    trust_domain: V2TrustDomain,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_marker: Option<String>,
    release: V2ReleaseIdentity,
    report_ids: Vec<String>,
    subject_roots: BTreeMap<String, String>,
    finding_ledger_roots: BTreeMap<String, String>,
    approval_lock_roots: BTreeMap<String, String>,
    release_transfer_roots: BTreeMap<String, String>,
    snapshot_id: String,
    public_tree_sha256: String,
    builder_identity: String,
}

pub(super) fn review_roots(
    repository: &V2Repository,
    report_id: &str,
    staging_root: &Path,
) -> Result<V2ReviewRoots> {
    let source = repository
        .sources
        .get(report_id)
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown v2 report ID '{report_id}'")))?;
    load_context(repository, source, staging_root).map(|context| context.roots)
}

fn load_context(
    repository: &V2Repository,
    source: &ReportSource,
    staging_root: &Path,
) -> Result<ReportContext> {
    let repository_capability = ConfinedDirectory::open_ambient(&repository.root, false)?;
    let assembly = repository.check_report(&source.id, staging_root)?;
    let staging = ConfinedDirectory::open_ambient(staging_root, false)?;
    let mut staged_bytes = BTreeMap::new();
    for (path, expected) in assembly.outputs {
        let bytes = staging.read_regular(&path)?;
        if sha256_bytes(&bytes) != expected {
            return Err(AssuranceError::Drift(format!(
                "checked staging byte changed during capture: {}",
                path.display()
            )));
        }
        staged_bytes.insert(path, bytes);
    }
    staging.verify_ambient_identity(staging_root)?;
    context_from_staged(
        repository,
        source,
        staged_bytes,
        Some(ContextCapabilities {
            repository: repository_capability,
            staging,
        }),
    )
}

fn context_from_staged(
    repository: &V2Repository,
    source: &ReportSource,
    staged_bytes: BTreeMap<PathBuf, Vec<u8>>,
    capabilities: Option<ContextCapabilities>,
) -> Result<ReportContext> {
    repository.verify_inputs()?;
    let mut inputs = repository.inputs.clone();
    let manifest_bytes = read_identified(
        &repository.root,
        &source.manifest_path,
        Some(&source.manifest_sha256),
        &mut inputs,
    )?;
    let report: Report = parse_yaml(&source.manifest_path, &manifest_bytes)?;
    validate_catalog_binding(source, &report)?;
    validate_report(&repository.root, &report, &mut inputs)?;
    let report_value = yaml_json(&source.manifest_path, &manifest_bytes)?;
    let mut input_bytes = BTreeMap::new();
    for path in inputs.keys() {
        input_bytes.insert(path.clone(), read_regular_confined(&repository.root, path)?);
    }
    let roots = calculate_roots(
        repository,
        source,
        &report,
        &report_value,
        &inputs,
        &staged_bytes,
    )?;
    Ok(ReportContext {
        report,
        report_value,
        input_bytes,
        staged_bytes,
        roots,
        capabilities,
    })
}

fn calculate_roots(
    repository: &V2Repository,
    source: &ReportSource,
    report: &Report,
    report_value: &Value,
    inputs: &BTreeMap<PathBuf, String>,
    staged_bytes: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<V2ReviewRoots> {
    let normalized_report = normalized_subject_report(report_value)?;
    let catalog_bytes = read_regular_confined(&repository.root, Path::new(super::V2_CATALOG_PATH))?;
    let normalized_catalog = normalized_catalog(&catalog_bytes, &source.id)?;
    let catalog_source_bytes_sha256 = catalog_subject_bytes_digest(&catalog_bytes, repository)?;
    let stable_inputs = inputs
        .iter()
        .filter(|(path, _)| {
            path.as_path() != source.manifest_path.as_path()
                && path.as_path() != Path::new(super::V2_CATALOG_PATH)
        })
        .map(|(path, digest)| path_string(path).map(|path| (path, Value::String(digest.clone()))))
        .collect::<Result<serde_json::Map<_, _>>>()?;
    let staged = staged_bytes
        .iter()
        .map(|(path, bytes)| {
            staged_subject_digest(path, bytes)
                .and_then(|digest| path_string(path).map(|path| (path, Value::String(digest))))
        })
        .collect::<Result<serde_json::Map<_, _>>>()?;
    let subject = serde_json::json!({
        "algorithm": "sha256-canonical-json-v1",
        "domain": SUBJECT_DOMAIN,
        "report": normalized_report,
        "catalog": normalized_catalog,
        "catalog_source_bytes_sha256": catalog_source_bytes_sha256,
        "inputs": stable_inputs,
        "staged_outputs": staged,
        "planner_tool": PLANNER_TOOL_ID,
        "assembly_tool": ASSEMBLY_TOOL_ID,
    });
    let subject_root = digest_value(&subject)?;
    let finding_ledger_root = if report.review.state == "DRAFT" {
        None
    } else {
        Some(finding_root(&subject_root, &report.review)?)
    };
    let approval_lock_root = if report.review.state == "APPROVED" {
        Some(approval_root(
            finding_ledger_root.as_deref().ok_or_else(|| {
                AssuranceError::Invalid("approved review has no finding ledger".to_owned())
            })?,
            &report.review,
        )?)
    } else {
        None
    };
    let release_transfer_root = if report.publication.state == "APPROVED" {
        Some(transfer_root(
            approval_lock_root.as_deref().ok_or_else(|| {
                AssuranceError::Invalid("approved publication has no approval lock".to_owned())
            })?,
            &report.lifecycle,
            &report.publication,
        )?)
    } else {
        None
    };
    Ok(V2ReviewRoots {
        report_id: report.id.clone(),
        subject_root,
        finding_ledger_root,
        approval_lock_root,
        release_transfer_root,
    })
}

fn catalog_subject_bytes_digest(bytes: &[u8], repository: &V2Repository) -> Result<String> {
    let mut normalized = bytes.to_vec();
    for source in repository.sources.values() {
        let needle = source.manifest_sha256.as_bytes();
        let matches = normalized
            .windows(needle.len())
            .enumerate()
            .filter_map(|(index, candidate)| (candidate == needle).then_some(index))
            .collect::<Vec<_>>();
        let [start] = matches.as_slice() else {
            return Err(AssuranceError::Invalid(format!(
                "source catalog must contain report manifest identity exactly once: {}",
                source.id
            )));
        };
        normalized[*start..*start + needle.len()].fill(b'0');
    }
    Ok(sha256_bytes(&normalized))
}

fn staged_subject_digest(path: &Path, bytes: &[u8]) -> Result<String> {
    if path.file_name().and_then(|name| name.to_str()) != Some("build-manifest.json") {
        return Ok(sha256_bytes(bytes));
    }
    let mut manifest: Value =
        serde_json::from_slice(bytes).map_err(|error| AssuranceError::Parse {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;
    let object = manifest.as_object_mut().ok_or_else(|| {
        AssuranceError::Invalid("assembly build manifest must be an object".to_owned())
    })?;
    if object
        .insert("source_root_sha256".to_owned(), Value::Null)
        .is_none()
    {
        return Err(AssuranceError::Invalid(
            "assembly build manifest lacks source_root_sha256".to_owned(),
        ));
    }
    canonical_bytes(&manifest).map(|bytes| sha256_bytes(&bytes))
}

fn finding_root(subject_root: &str, review: &Review) -> Result<String> {
    let mut findings = review
        .findings
        .iter()
        .map(finding_value)
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
    digest_value(&serde_json::json!({
        "algorithm": "sha256-canonical-json-v1",
        "domain": FINDING_DOMAIN,
        "subject_root": subject_root,
        "review_charge": review.charge.as_deref(),
        "build_maintainer_id": review.build_maintainer_id.as_deref(),
        "material_producer_ids": review.material_producer_ids,
        "findings": findings,
    }))
}

fn approval_root(finding_root: &str, review: &Review) -> Result<String> {
    let mut approvals = review
        .approvals
        .iter()
        .map(approval_value)
        .collect::<Vec<_>>();
    approvals.sort_by(|left, right| left["role"].as_str().cmp(&right["role"].as_str()));
    digest_value(&serde_json::json!({
        "algorithm": "sha256-canonical-json-v1",
        "domain": APPROVAL_DOMAIN,
        "finding_ledger_root": finding_root,
        "review_state": review.state,
        "review_decision": review.decision,
        "approvals": approvals,
        "independence_assessment": review.independence_assessment,
    }))
}

fn transfer_root(
    approval_root: &str,
    lifecycle: &str,
    publication: &Publication,
) -> Result<String> {
    digest_value(&serde_json::json!({
        "algorithm": "sha256-canonical-json-v1",
        "domain": TRANSFER_DOMAIN,
        "lifecycle": lifecycle,
        "publication_state": publication.state,
        "approval_lock_root": approval_root,
        "target_release_commit": publication.target_release_commit.as_deref(),
        "target_release_configuration": publication.target_release_configuration.as_deref(),
        "prior_realization": publication.prior_realization.as_deref(),
        "candidate_realization": publication.candidate_realization.as_deref(),
        "impact_assessment": publication.impact_assessment.as_deref(),
        "reproduction_disposition": publication.reproduction_disposition.as_deref(),
        "semantic_differences": publication.semantic_differences,
        "release_owner_id": publication.release_owner_id.as_deref(),
        "assurance_steward_id": publication.assurance_steward_id.as_deref(),
        "publication_date": publication.date.as_deref(),
        "public_path": publication.public_path.as_deref().map(path_string).transpose()?,
        "supersedes": publication.supersedes.as_deref(),
        "withdrawn": publication.withdrawn,
        "export_authorized": publication.export_authorized,
        "vendoring_authorized": publication.vendoring_authorized,
    }))
}

fn finding_value(finding: &Finding) -> Value {
    serde_json::json!({
        "id": finding.id,
        "summary": finding.summary,
        "severity": finding.severity,
        "disposition": finding.disposition,
        "rationale": finding.rationale.as_deref(),
        "resolution": finding.resolution.as_deref(),
        "verification": finding.verification.as_deref(),
        "verifier_id": finding.verifier_id.as_deref(),
    })
}

fn approval_value(approval: &Approval) -> Value {
    serde_json::json!({
        "role": approval.role,
        "principal_id": approval.principal_id,
        "finding_ledger_root": approval.finding_ledger_root,
        "decision": approval.decision,
        "competence_basis": approval.competence_basis,
        "independence_attestation": approval.independence_attestation,
        "approved_on": approval.approved_on,
    })
}

pub(super) fn publish(
    repository: &V2Repository,
    report_id: Option<&str>,
    options: &V2PublicationOptions,
    expected_domain: V2TrustDomain,
) -> Result<V2PublicationResult> {
    options.release.validate()?;
    let sources = match report_id {
        Some(report_id) => vec![repository.sources.get(report_id).ok_or_else(|| {
            AssuranceError::Invalid(format!("unknown v2 report ID '{report_id}'"))
        })?],
        None => repository.sources.values().collect(),
    };
    if repository.trust_domain != expected_domain {
        return Err(AssuranceError::Invalid(format!(
            "{} publication cannot consume {} trust-domain sources",
            expected_domain, repository.trust_domain
        )));
    }
    validate_roots(repository, options)?;
    let mut contexts = Vec::new();
    for source in sources {
        let context = load_context(repository, source, &options.staging_root)?;
        validate_publishable(repository, &context, expected_domain, &options.release)?;
        contexts.push(context);
    }
    contexts.sort_by(|left, right| left.report.id.cmp(&right.report.id));
    if report_id.is_none() {
        require_all_staging_exact(&options.staging_root, &contexts)?;
    }
    finalize_publication(
        repository,
        &contexts,
        options,
        expected_domain,
        report_id.is_none(),
    )
}

fn finalize_publication(
    repository: &V2Repository,
    contexts: &[ReportContext],
    options: &V2PublicationOptions,
    expected_domain: V2TrustDomain,
    replace_all: bool,
) -> Result<V2PublicationResult> {
    let public_root = ConfinedDirectory::open_ambient(&options.usersum_root, false)?;
    let snapshot_root = ConfinedDirectory::open_ambient(&options.snapshot_root, false)?;
    lock_roots(
        &public_root,
        &options.usersum_root,
        &snapshot_root,
        &options.snapshot_root,
    )?;
    let source_capabilities = contexts
        .first()
        .and_then(|context| context.capabilities.as_ref())
        .ok_or_else(|| {
            AssuranceError::Invalid("publication lacks held source capabilities".to_owned())
        })?;
    let root_guard = HeldRootGuard {
        source: source_capabilities,
        repository_path: &repository.root,
        staging_path: &options.staging_root,
        public: &public_root,
        public_path: &options.usersum_root,
        snapshot: &snapshot_root,
        snapshot_path: &options.snapshot_root,
    };
    root_guard.validate()?;
    let prepared = prepare_publication(
        &public_root,
        &snapshot_root,
        contexts,
        options,
        expected_domain,
        replace_all,
    )?;
    let PreparedPublication {
        public_files,
        snapshot_payload,
        bindings,
        report_ids,
        public_tree_sha256,
    } = prepared;
    let manifest = snapshot_manifest(
        expected_domain,
        &options.release,
        &report_ids,
        &snapshot_payload,
        &public_tree_sha256,
    )?;
    let manifest_bytes = canonical_bytes(&manifest)?;
    let snapshot_id = sha256_bytes(&manifest_bytes);
    install_snapshot(
        &snapshot_root,
        &snapshot_id,
        &manifest_bytes,
        &snapshot_payload,
    )?;
    inject_fault(options, V2PublicationFault::AfterSnapshotInstall)?;

    let receipt = publication_receipt(
        expected_domain,
        &options.release,
        &report_ids,
        &bindings,
        &snapshot_id,
        &public_tree_sha256,
    )?;
    let receipt_bytes = canonical_bytes(&receipt)?;
    let receipt_id = sha256_bytes(&receipt_bytes);
    install_receipt(&snapshot_root, &receipt_id, &receipt_bytes)?;
    inject_fault(options, V2PublicationFault::AfterReceiptInstall)?;

    repository.verify_inputs()?;
    for context in contexts {
        revalidate_context(repository, context, &options.staging_root)?;
    }
    root_guard.validate()?;
    inject_fault(options, V2PublicationFault::BeforePublicCommit)?;
    commit_public_generation(
        &public_root,
        &public_files,
        &receipt_id,
        &options.usersum_root,
        &root_guard,
    )?;

    Ok(V2PublicationResult {
        report_ids,
        snapshot_id: snapshot_id.clone(),
        snapshot_path: options.snapshot_root.join(&snapshot_id),
        receipt_id: receipt_id.clone(),
        receipt_path: options
            .snapshot_root
            .join("receipts")
            .join(format!("{receipt_id}.json")),
        public_tree_sha256,
    })
}

fn inject_fault(options: &V2PublicationOptions, point: V2PublicationFault) -> Result<()> {
    if options.fault_injection == Some(point) {
        Err(AssuranceError::Invalid(format!(
            "injected ASSURE-04D publication failure at {point:?}"
        )))
    } else {
        Ok(())
    }
}

fn prepare_publication(
    public_root: &ConfinedDirectory,
    snapshot_root: &ConfinedDirectory,
    contexts: &[ReportContext],
    options: &V2PublicationOptions,
    expected_domain: V2TrustDomain,
    replace_all: bool,
) -> Result<PreparedPublication> {
    let PriorPublic {
        mut catalog,
        files: mut public_files,
        mut bindings,
        mut source_payload,
    } = read_prior_public(
        public_root,
        snapshot_root,
        &options.snapshot_root,
        expected_domain,
        &options.release,
    )?;
    if replace_all {
        catalog.reports.clear();
        public_files.clear();
        bindings.clear();
        source_payload.clear();
    }
    for context in contexts {
        remove_catalog_report(&mut catalog, &mut public_files, &context.report.id);
        bindings.remove(&context.report.id);
        let source_prefix = Path::new("source").join(&context.report.id);
        source_payload.retain(|path, _| !path.starts_with(&source_prefix));
        install_context_files(&mut public_files, context, expected_domain)?;
        catalog.reports.push(catalog_entry(context)?);
        bindings.insert(context)?;
    }
    catalog
        .reports
        .sort_by(|left, right| left.report_id.cmp(&right.report_id));
    reject_duplicate_catalog_reports(&catalog)?;
    public_files.insert(
        PathBuf::from("assurance/catalog.json"),
        canonical_bytes(&catalog)?,
    );
    public_files.insert(
        PathBuf::from("assurance/README.md"),
        render_public_readme(&catalog),
    );
    validate_public_links(public_root, &public_files, &catalog, contexts)?;

    let public_tree_sha256 = digest_files("openwepp-assurance-public-tree-v1", &public_files)?;
    let snapshot_payload = snapshot_payload(contexts, &public_files, source_payload)?;
    let report_ids = catalog
        .reports
        .iter()
        .map(|entry| entry.report_id.clone())
        .collect::<Vec<_>>();
    Ok(PreparedPublication {
        public_files,
        snapshot_payload,
        bindings,
        report_ids,
        public_tree_sha256,
    })
}

fn validate_publishable(
    repository: &V2Repository,
    context: &ReportContext,
    expected_domain: V2TrustDomain,
    release: &V2ReleaseIdentity,
) -> Result<()> {
    let report = &context.report;
    if report.lifecycle != "APPROVED" {
        return Err(AssuranceError::Invalid(format!(
            "report '{}' is {}; publication requires APPROVED",
            report.id, report.lifecycle
        )));
    }
    if report.trust_domain != expected_domain {
        return Err(AssuranceError::Invalid(format!(
            "report '{}' trust domain does not match the publication entry point",
            report.id
        )));
    }
    require_declared_root(
        report.review.subject_root.as_deref(),
        &context.roots.subject_root,
        "review subject",
    )?;
    require_declared_root(
        report.review.finding_ledger_root.as_deref(),
        context
            .roots
            .finding_ledger_root
            .as_deref()
            .ok_or_else(|| {
                AssuranceError::Invalid("approved report has no calculated finding root".to_owned())
            })?,
        "finding ledger",
    )?;
    require_declared_root(
        report.review.approval_lock_root.as_deref(),
        context.roots.approval_lock_root.as_deref().ok_or_else(|| {
            AssuranceError::Invalid("approved report has no calculated approval root".to_owned())
        })?,
        "approval lock",
    )?;
    require_declared_root(
        report.publication.release_transfer_root.as_deref(),
        context
            .roots
            .release_transfer_root
            .as_deref()
            .ok_or_else(|| {
                AssuranceError::Invalid(
                    "approved report has no calculated transfer root".to_owned(),
                )
            })?,
        "release transfer",
    )?;
    require_declared_root(
        report.publication.approval_lock_root.as_deref(),
        context.roots.approval_lock_root.as_deref().ok_or_else(|| {
            AssuranceError::Invalid("approved publication has no approval root".to_owned())
        })?,
        "publication approval lock",
    )?;
    if report.publication.target_release_commit.as_deref() != Some(release.commit())
        || report.publication.target_release_configuration.as_deref()
            != Some(release.configuration())
    {
        return Err(AssuranceError::Invalid(format!(
            "report '{}' release transfer disagrees with independently supplied release identity",
            report.id
        )));
    }
    validate_principal_roles(repository, report)
}

fn require_declared_root(declared: Option<&str>, calculated: &str, label: &str) -> Result<()> {
    if declared == Some(calculated) {
        Ok(())
    } else {
        Err(AssuranceError::Drift(format!(
            "declared {label} root does not match calculated root"
        )))
    }
}

fn validate_principal_roles(repository: &V2Repository, report: &Report) -> Result<()> {
    let principals = repository
        .principals
        .principals
        .iter()
        .map(|principal| (principal.id.as_str(), principal))
        .collect::<BTreeMap<_, _>>();
    let maintainer = require_principal(
        &principals,
        report.review.build_maintainer_id.as_deref(),
        "build_maintainer",
        None,
    )?;
    let mut producers = BTreeSet::new();
    for producer in &report.review.material_producer_ids {
        require_principal(&principals, Some(producer), "material_producer", None)?;
        producers.insert(producer.as_str());
    }
    for finding in &report.review.findings {
        if let Some(verifier) = finding.verifier_id.as_deref() {
            require_principal(&principals, Some(verifier), "finding_verifier", None)?;
            if finding.disposition == "resolved_and_verified" && producers.contains(verifier) {
                return Err(AssuranceError::Invalid(format!(
                    "finding '{}' verifier conflicts with a material producer",
                    finding.id
                )));
            }
        }
    }
    validate_approval_principals(&principals, report, &maintainer.id, &producers)
}

fn validate_approval_principals(
    principals: &BTreeMap<&str, &Principal>,
    report: &Report,
    maintainer_id: &str,
    producers: &BTreeSet<&str>,
) -> Result<()> {
    let lead = report
        .authorship
        .human_report_lead
        .as_deref()
        .ok_or_else(|| {
            AssuranceError::Invalid("approved report requires a human report lead".to_owned())
        })?;
    require_principal(
        principals,
        Some(lead),
        "report_lead",
        Some(PrincipalKind::Human),
    )?;
    let mut approval_ids = BTreeSet::new();
    for approval in &report.review.approvals {
        let role = match approval.role.as_str() {
            "scientific" => "scientific_reviewer",
            "reproduction_publication" => "reproduction_publication_reviewer",
            "assurance_steward" => "assurance_steward",
            _ => {
                return Err(AssuranceError::Invalid(format!(
                    "unknown approval role '{}'",
                    approval.role
                )));
            }
        };
        require_principal(
            principals,
            Some(&approval.principal_id),
            role,
            Some(PrincipalKind::Human),
        )?;
        approval_ids.insert(approval.principal_id.as_str());
        if matches!(
            approval.role.as_str(),
            "scientific" | "reproduction_publication"
        ) && (approval.principal_id == lead
            || producers.contains(approval.principal_id.as_str()))
        {
            return Err(AssuranceError::Invalid(format!(
                "{} approver conflicts with report lead or material producer",
                approval.role
            )));
        }
        if approval.role == "reproduction_publication" && approval.principal_id == maintainer_id {
            return Err(AssuranceError::Invalid(
                "reproduction/publication approver conflicts with build maintainer".to_owned(),
            ));
        }
    }
    if approval_ids.len() != 3 {
        return Err(AssuranceError::Invalid(
            "approval principals must be distinct".to_owned(),
        ));
    }
    let scientific = report
        .review
        .approvals
        .iter()
        .find(|approval| approval.role == "scientific")
        .map(|approval| approval.principal_id.as_str());
    if report.authorship.scientific_approver.as_deref() != scientific {
        return Err(AssuranceError::Invalid(
            "authorship scientific approver does not match the scientific approval".to_owned(),
        ));
    }
    require_principal(
        principals,
        report.publication.release_owner_id.as_deref(),
        "release_owner",
        Some(PrincipalKind::Human),
    )?;
    require_principal(
        principals,
        report.publication.assurance_steward_id.as_deref(),
        "assurance_steward",
        Some(PrincipalKind::Human),
    )?;
    Ok(())
}

fn require_principal<'a>(
    principals: &BTreeMap<&str, &'a Principal>,
    id: Option<&str>,
    role: &str,
    kind: Option<PrincipalKind>,
) -> Result<&'a Principal> {
    let id = id.ok_or_else(|| AssuranceError::Invalid(format!("{role} principal is required")))?;
    let principal = principals
        .get(id)
        .copied()
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown principal ID '{id}'")))?;
    if !principal.roles.iter().any(|candidate| candidate == role) {
        return Err(AssuranceError::Invalid(format!(
            "principal '{id}' does not declare role '{role}'"
        )));
    }
    if kind.is_some_and(|expected| principal.kind != expected) {
        return Err(AssuranceError::Invalid(format!(
            "principal '{id}' has the wrong principal kind for '{role}'"
        )));
    }
    Ok(principal)
}

fn validate_roots(repository: &V2Repository, options: &V2PublicationOptions) -> Result<()> {
    let paths = [
        ("staging", &options.staging_root),
        ("usersum", &options.usersum_root),
        ("snapshot", &options.snapshot_root),
    ];
    let mut canonical = Vec::new();
    for (label, path) in paths {
        if !path.is_absolute()
            || path
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return Err(AssuranceError::Invalid(format!(
                "{label} root must be an absolute normalized path"
            )));
        }
        let resolved = path
            .canonicalize()
            .map_err(|error| AssuranceError::io(path, error))?;
        if resolved != *path {
            return Err(AssuranceError::Invalid(format!(
                "{label} root must not contain symlink aliases or lexical indirection"
            )));
        }
        canonical.push((label, resolved));
    }
    canonical.push(("repository", repository.root.clone()));
    for left in 0..canonical.len() {
        for right in left + 1..canonical.len() {
            let (left_label, left_path) = &canonical[left];
            let (right_label, right_path) = &canonical[right];
            if left_path.starts_with(right_path)
                || right_path.starts_with(left_path)
                || share_directory_identity(left_path, right_path)?
            {
                return Err(AssuranceError::Invalid(format!(
                    "{left_label} and {right_label} roots must be unrelated"
                )));
            }
        }
    }
    ConfinedDirectory::open_ambient(&options.staging_root, false)?;
    ConfinedDirectory::open_ambient(&options.usersum_root, false)?;
    ConfinedDirectory::open_ambient(&options.snapshot_root, false)?;
    Ok(())
}

#[cfg(unix)]
fn share_directory_identity(left: &Path, right: &Path) -> Result<bool> {
    use std::os::unix::fs::MetadataExt as _;

    let identities = |path: &Path| -> Result<BTreeSet<(u64, u64)>> {
        let mut values = BTreeSet::new();
        let mut cursor = Some(path);
        while let Some(value) = cursor {
            let metadata = fs::metadata(value).map_err(|error| AssuranceError::io(value, error))?;
            values.insert((metadata.dev(), metadata.ino()));
            cursor = value.parent();
        }
        Ok(values)
    };
    let left_ids = identities(left)?;
    let right_ids = identities(right)?;
    let left_metadata = fs::metadata(left).map_err(|error| AssuranceError::io(left, error))?;
    let right_metadata = fs::metadata(right).map_err(|error| AssuranceError::io(right, error))?;
    Ok(
        left_ids.contains(&(right_metadata.dev(), right_metadata.ino()))
            || right_ids.contains(&(left_metadata.dev(), left_metadata.ino())),
    )
}

#[cfg(not(unix))]
fn share_directory_identity(_left: &Path, _right: &Path) -> Result<bool> {
    Err(AssuranceError::Invalid(
        "publication confinement requires Unix directory identities".to_owned(),
    ))
}

impl HeldRootGuard<'_> {
    fn validate(&self) -> Result<()> {
        self.source
            .repository
            .verify_ambient_identity(self.repository_path)?;
        self.source
            .staging
            .verify_ambient_identity(self.staging_path)?;
        self.public.verify_ambient_identity(self.public_path)?;
        self.snapshot.verify_ambient_identity(self.snapshot_path)?;
        let roots = [
            ("repository", &self.source.repository),
            ("staging", &self.source.staging),
            ("usersum", self.public),
            ("snapshot", self.snapshot),
        ];
        for left in 0..roots.len() {
            for right in left + 1..roots.len() {
                let (left_label, left_root) = roots[left];
                let (right_label, right_root) = roots[right];
                if left_root.contains_directory(right_root)?
                    || right_root.contains_directory(left_root)?
                {
                    return Err(AssuranceError::Invalid(format!(
                        "{left_label} and {right_label} held roots must be unrelated"
                    )));
                }
            }
        }
        Ok(())
    }
}

fn lock_roots(
    public: &ConfinedDirectory,
    public_path: &Path,
    snapshot: &ConfinedDirectory,
    snapshot_path: &Path,
) -> Result<()> {
    if public_path < snapshot_path {
        public.lock_exclusive(public_path)?;
        snapshot.lock_exclusive(snapshot_path)
    } else {
        snapshot.lock_exclusive(snapshot_path)?;
        public.lock_exclusive(public_path)
    }
}

fn require_all_staging_exact(staging_root: &Path, contexts: &[ReportContext]) -> Result<()> {
    let staging = ConfinedDirectory::open_ambient(staging_root, false)?;
    let base = Path::new("usersum/assurance/reports");
    let observed = staging.collect_regular_files(base)?;
    let observed_directories = staging.collect_directories(base)?;
    let expected = contexts
        .iter()
        .flat_map(|context| context.staged_bytes.keys())
        .map(|path| {
            path.strip_prefix(base).map(Path::to_path_buf).map_err(|_| {
                AssuranceError::Invalid(format!(
                    "staged publication output escaped report root: {}",
                    path.display()
                ))
            })
        })
        .collect::<Result<BTreeSet<_>>>()?;
    if observed != expected {
        return Err(AssuranceError::Drift(
            "all publication staging contains a missing, stale, hidden, or unknown report byte"
                .to_owned(),
        ));
    }
    if observed_directories != parent_directories(&expected) {
        return Err(AssuranceError::Drift(
            "all publication staging contains an unknown or empty report directory".to_owned(),
        ));
    }
    Ok(())
}

fn read_prior_public(
    root: &ConfinedDirectory,
    snapshot_root: &ConfinedDirectory,
    snapshot_ambient: &Path,
    expected_domain: V2TrustDomain,
    release: &V2ReleaseIdentity,
) -> Result<PriorPublic> {
    if !root.directory_exists(Path::new("assurance"))? {
        return Ok(empty_catalog(expected_domain));
    }
    let observed = root.collect_regular_files(Path::new("assurance"))?;
    let observed_directories = root.collect_directories(Path::new("assurance"))?;
    if observed_directories != parent_directories(&observed) {
        return Err(AssuranceError::Invalid(
            "existing assurance generation contains an unknown or empty directory".to_owned(),
        ));
    }
    if observed == BTreeSet::from([PathBuf::from("README.md")]) {
        let readme = root.read_regular(Path::new("assurance/README.md"))?;
        if sha256_bytes(&readme) == ZERO_REPORT_README_SHA256 {
            return Ok(empty_catalog(expected_domain));
        }
        return Err(AssuranceError::Invalid(
            "README-only assurance bootstrap does not match the accepted zero-report bytes"
                .to_owned(),
        ));
    }
    if !observed.contains(Path::new("catalog.json")) || !observed.contains(Path::new("README.md")) {
        return Err(AssuranceError::Invalid(
            "existing assurance generation is not machine-catalog-owned".to_owned(),
        ));
    }
    let catalog_bytes = root.read_regular(Path::new("assurance/catalog.json"))?;
    let catalog: PublicCatalog =
        serde_json::from_slice(&catalog_bytes).map_err(|error| AssuranceError::Parse {
            path: PathBuf::from("assurance/catalog.json"),
            message: error.to_string(),
        })?;
    if catalog.format != PUBLIC_FORMAT
        || catalog.trust_domain != expected_domain
        || !marker_matches(expected_domain, catalog.test_marker.as_deref())
    {
        return Err(AssuranceError::Invalid(
            "existing public catalog format or trust domain is incompatible".to_owned(),
        ));
    }
    reject_duplicate_catalog_reports(&catalog)?;
    let owned = catalog
        .reports
        .iter()
        .map(|entry| PathBuf::from(format!("reports/{}/{}", entry.report_id, entry.version)))
        .collect::<Vec<_>>();
    for path in &observed {
        if path == Path::new("README.md") || path == Path::new("catalog.json") {
            continue;
        }
        if !owned.iter().any(|prefix| path.starts_with(prefix)) {
            return Err(AssuranceError::Invalid(format!(
                "existing assurance content is not owned by its machine catalog: {}",
                path.display()
            )));
        }
    }
    let mut files = BTreeMap::new();
    for path in observed {
        files.insert(
            Path::new("assurance").join(&path),
            root.read_regular(&Path::new("assurance").join(path))?,
        );
    }
    let PriorVerification {
        bindings,
        source_payload,
    } = verify_prior_publication(
        snapshot_root,
        snapshot_ambient,
        expected_domain,
        release,
        &catalog,
        &files,
    )?;
    files.remove(Path::new("assurance/README.md"));
    files.remove(Path::new("assurance/catalog.json"));
    Ok(PriorPublic {
        catalog,
        files,
        bindings,
        source_payload,
    })
}

fn parent_directories(paths: &BTreeSet<PathBuf>) -> BTreeSet<PathBuf> {
    let mut directories = BTreeSet::new();
    for path in paths {
        let mut parent = path.parent();
        while let Some(path) = parent {
            if path.as_os_str().is_empty() {
                break;
            }
            directories.insert(path.to_path_buf());
            parent = path.parent();
        }
    }
    directories
}

fn empty_catalog(domain: V2TrustDomain) -> PriorPublic {
    PriorPublic {
        catalog: PublicCatalog {
            format: PUBLIC_FORMAT.to_owned(),
            trust_domain: domain,
            test_marker: test_marker(domain),
            reports: Vec::new(),
        },
        files: BTreeMap::new(),
        bindings: ReceiptBindings::default(),
        source_payload: BTreeMap::new(),
    }
}

fn verify_prior_publication(
    snapshot_root: &ConfinedDirectory,
    snapshot_ambient: &Path,
    expected_domain: V2TrustDomain,
    release: &V2ReleaseIdentity,
    catalog: &PublicCatalog,
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<PriorVerification> {
    let public_tree_sha256 = digest_files("openwepp-assurance-public-tree-v1", files)?;
    let receipt_names = snapshot_root
        .collect_regular_files(Path::new("receipts"))
        .map_err(|_| {
            AssuranceError::Invalid(
                "existing public assurance generation has no verifiable publication receipt"
                    .to_owned(),
            )
        })?;
    let mut candidate_error = None;
    for name in receipt_names {
        if name
            .parent()
            .is_some_and(|parent| !parent.as_os_str().is_empty())
        {
            continue;
        }
        let receipt_relative = Path::new("receipts").join(&name);
        let receipt_bytes = snapshot_root.read_regular(&receipt_relative)?;
        let Some(receipt_id) = name
            .file_name()
            .and_then(|value| value.to_str())
            .and_then(|value| value.strip_suffix(".json"))
        else {
            continue;
        };
        if validate_sha256(receipt_id, "prior receipt ID").is_err()
            || sha256_bytes(&receipt_bytes) != receipt_id
        {
            continue;
        }
        let Ok(receipt) = serde_json::from_slice::<PublicationReceipt>(&receipt_bytes) else {
            continue;
        };
        if !receipt_matches_catalog(
            &receipt,
            catalog,
            expected_domain,
            release,
            &public_tree_sha256,
        ) {
            continue;
        }
        let snapshot_dir = snapshot_ambient.join(&receipt.snapshot_id);
        let receipt_path = snapshot_ambient.join(&receipt_relative);
        let (snapshot_id, manifest, observed_tree) =
            match verify_snapshot_content(&snapshot_dir, release, expected_domain) {
                Ok(verified) => verified,
                Err(error) => {
                    candidate_error = Some(error);
                    continue;
                }
            };
        if let Err(error) = verify_receipt_content(
            &receipt_path,
            release,
            expected_domain,
            &snapshot_id,
            &manifest,
            observed_tree,
        ) {
            candidate_error = Some(error);
            continue;
        }
        let snapshot = ConfinedDirectory::open_ambient(&snapshot_dir, false)?;
        let mut source_payload = BTreeMap::new();
        for file in &manifest.files {
            if file.kind == SnapshotFileKind::Source {
                let path = PathBuf::from(&file.path);
                source_payload.insert(
                    path.clone(),
                    (snapshot.read_regular(&path)?, SnapshotFileKind::Source),
                );
            }
        }
        return Ok(PriorVerification {
            bindings: ReceiptBindings::from_receipt(&receipt),
            source_payload,
        });
    }
    Err(candidate_error.unwrap_or_else(|| {
        AssuranceError::Invalid(
            "existing public assurance generation is not backed by a verified receipt for its exact bytes and release"
                .to_owned(),
        )
    }))
}

fn receipt_matches_catalog(
    receipt: &PublicationReceipt,
    catalog: &PublicCatalog,
    expected_domain: V2TrustDomain,
    release: &V2ReleaseIdentity,
    public_tree_sha256: &str,
) -> bool {
    let report_ids = catalog
        .reports
        .iter()
        .map(|entry| entry.report_id.clone())
        .collect::<Vec<_>>();
    if receipt.format != RECEIPT_FORMAT
        || receipt.domain != RECEIPT_DOMAIN
        || receipt.trust_domain != expected_domain
        || !marker_matches(expected_domain, receipt.test_marker.as_deref())
        || receipt.release != *release
        || receipt.public_tree_sha256 != public_tree_sha256
        || receipt.report_ids != report_ids
    {
        return false;
    }
    catalog.reports.iter().all(|entry| {
        receipt.subject_roots.get(&entry.report_id) == Some(&entry.subject_root)
            && receipt.approval_lock_roots.get(&entry.report_id) == Some(&entry.approval_lock_root)
            && receipt.release_transfer_roots.get(&entry.report_id)
                == Some(&entry.release_transfer_root)
    })
}

fn remove_catalog_report(
    catalog: &mut PublicCatalog,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
    report_id: &str,
) {
    catalog.reports.retain(|entry| entry.report_id != report_id);
    let prefix = PathBuf::from(format!("assurance/reports/{report_id}"));
    files.retain(|path, _| !path.starts_with(&prefix));
}

fn install_context_files(
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
    context: &ReportContext,
    domain: V2TrustDomain,
) -> Result<()> {
    for (staged, bytes) in &context.staged_bytes {
        let public = staged.strip_prefix("usersum").map_err(|_| {
            AssuranceError::Invalid(format!(
                "staged report output is not usersum-shaped: {}",
                staged.display()
            ))
        })?;
        if files.insert(public.to_path_buf(), bytes.clone()).is_some() {
            return Err(AssuranceError::Invalid(format!(
                "duplicate public output path: {}",
                public.display()
            )));
        }
    }
    if domain == V2TrustDomain::TestOnly {
        for name in ["index.md", "supplement.md"] {
            let path = PathBuf::from(format!(
                "assurance/reports/{}/{}/{name}",
                context.report.id, context.report.version
            ));
            let bytes = files.get(&path).ok_or_else(|| {
                AssuranceError::Invalid(format!("synthetic output is missing {}", path.display()))
            })?;
            let text = std::str::from_utf8(bytes).map_err(|error| AssuranceError::Parse {
                path: path.clone(),
                message: error.to_string(),
            })?;
            if !text.contains(TEST_BANNER) {
                return Err(AssuranceError::Invalid(format!(
                    "synthetic output must visibly contain '{TEST_BANNER}' in {}",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

fn catalog_entry(context: &ReportContext) -> Result<PublicCatalogEntry> {
    let publication = &context.report.publication;
    let expected_public_path = format!(
        "assurance/reports/{}/{}/index.md",
        context.report.id, context.report.version
    );
    let declared_public_path =
        path_string(publication.public_path.as_deref().ok_or_else(|| {
            AssuranceError::Invalid("publication public path is missing".to_owned())
        })?)?;
    if declared_public_path != expected_public_path {
        return Err(AssuranceError::Invalid(format!(
            "report '{}' publication path must be '{expected_public_path}'",
            context.report.id
        )));
    }
    Ok(PublicCatalogEntry {
        report_id: context.report.id.clone(),
        version: context.report.version.clone(),
        title: context.report.title.clone(),
        scientific_question: context.report.reader_metadata.scientific_question.clone(),
        assessed_process: context.report.reader_metadata.assessed_process.clone(),
        assessed_quantity: context.report.reader_metadata.assessed_quantity.clone(),
        realization: context.report.reader_metadata.realization.clone(),
        publication_date: publication
            .date
            .as_deref()
            .ok_or_else(|| AssuranceError::Invalid("publication date is missing".to_owned()))?
            .to_owned(),
        report_path: declared_public_path,
        supplement_path: format!(
            "assurance/reports/{}/{}/supplement.md",
            context.report.id, context.report.version
        ),
        related_model_narrative: path_string(
            &context.report.reader_metadata.related_model_narrative,
        )?,
        subject_root: context.roots.subject_root.clone(),
        approval_lock_root: context
            .roots
            .approval_lock_root
            .clone()
            .ok_or_else(|| AssuranceError::Invalid("approval root is missing".to_owned()))?,
        release_transfer_root: context
            .roots
            .release_transfer_root
            .clone()
            .ok_or_else(|| AssuranceError::Invalid("transfer root is missing".to_owned()))?,
        publication_state: "PUBLISHED".to_owned(),
    })
}

fn reject_duplicate_catalog_reports(catalog: &PublicCatalog) -> Result<()> {
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for entry in &catalog.reports {
        if !ids.insert(&entry.report_id) || !paths.insert(&entry.report_path) {
            return Err(AssuranceError::Invalid(
                "public catalog contains duplicate report identities or paths".to_owned(),
            ));
        }
        if entry.publication_state != "PUBLISHED" {
            return Err(AssuranceError::Invalid(
                "public catalog may contain only PUBLISHED reports".to_owned(),
            ));
        }
    }
    Ok(())
}

fn render_public_readme(catalog: &PublicCatalog) -> Vec<u8> {
    let mut output = String::from("# Scientific Assurance Reports\n\n");
    if catalog.trust_domain == V2TrustDomain::TestOnly {
        let _ = writeln!(output, "> **{TEST_BANNER}**\n");
    }
    output.push_str(
        "These reports present the methods, evidence, results, and limits used to assess specific openWEPP processes and software realizations. They support informed scientific judgment; they do not declare a watershed application fit for use.\n\n",
    );
    if catalog.reports.is_empty() {
        output.push_str("No scientific assurance reports are published.\n");
        return output.into_bytes();
    }
    for entry in &catalog.reports {
        let _ = writeln!(
            output,
            "## [{}]({})\n\n**Question:** {}\n\n**Assessed process and quantity:** {}; {}\n\n**Realization:** {}\n\n**Published:** {}\n\n[Technical supplement]({}) · [Related model description](../{})\n",
            entry.title,
            strip_assurance_prefix(&entry.report_path),
            entry.scientific_question,
            entry.assessed_process,
            entry.assessed_quantity,
            entry.realization,
            entry.publication_date,
            strip_assurance_prefix(&entry.supplement_path),
            entry.related_model_narrative,
        );
    }
    output.into_bytes()
}

fn test_marker(domain: V2TrustDomain) -> Option<String> {
    (domain == V2TrustDomain::TestOnly).then(|| TEST_BANNER.to_owned())
}

fn marker_matches(domain: V2TrustDomain, marker: Option<&str>) -> bool {
    match domain {
        V2TrustDomain::Production => marker.is_none(),
        V2TrustDomain::TestOnly => marker == Some(TEST_BANNER),
    }
}

fn strip_assurance_prefix(path: &str) -> &str {
    path.strip_prefix("assurance/").unwrap_or(path)
}

fn validate_public_links(
    public_root: &ConfinedDirectory,
    files: &BTreeMap<PathBuf, Vec<u8>>,
    catalog: &PublicCatalog,
    contexts: &[ReportContext],
) -> Result<()> {
    for entry in &catalog.reports {
        for path in [&entry.report_path, &entry.supplement_path] {
            if !files.contains_key(Path::new(path)) {
                return Err(AssuranceError::Invalid(format!(
                    "public catalog link does not resolve: {path}"
                )));
            }
        }
        let narrative = Path::new(&entry.related_model_narrative);
        let narrative_bytes = public_root.read_regular(narrative)?;
        if let Some(context) = contexts
            .iter()
            .find(|context| context.report.id == entry.report_id)
        {
            let source_path = Path::new("usersum").join(narrative);
            if context.input_bytes.get(&source_path) != Some(&narrative_bytes) {
                return Err(AssuranceError::Drift(format!(
                    "related model narrative differs from the approved source for '{}'",
                    entry.report_id
                )));
            }
        }
        let report = files
            .get(Path::new(&entry.report_path))
            .ok_or_else(|| AssuranceError::Invalid("public report is absent".to_owned()))?;
        let report = std::str::from_utf8(report).map_err(|error| AssuranceError::Parse {
            path: PathBuf::from(&entry.report_path),
            message: error.to_string(),
        })?;
        let expected_target = format!("../../../../{}", path_string(narrative)?);
        if !has_canonical_markdown_link(report, &expected_target) {
            return Err(AssuranceError::Invalid(format!(
                "public report '{}' does not contain the canonical Markdown link to its related model narrative",
                entry.report_id
            )));
        }
    }
    Ok(())
}

fn snapshot_payload(
    contexts: &[ReportContext],
    public_files: &BTreeMap<PathBuf, Vec<u8>>,
    mut payload: SnapshotPayload,
) -> Result<SnapshotPayload> {
    for context in contexts {
        for (path, bytes) in &context.input_bytes {
            insert_snapshot_file(
                &mut payload,
                Path::new("source").join(&context.report.id).join(path),
                bytes.clone(),
                SnapshotFileKind::Source,
            )?;
        }
    }
    for (path, bytes) in public_files {
        insert_snapshot_file(
            &mut payload,
            Path::new("public").join(path),
            bytes.clone(),
            SnapshotFileKind::Public,
        )?;
    }
    Ok(payload)
}

fn insert_snapshot_file(
    payload: &mut SnapshotPayload,
    path: PathBuf,
    bytes: Vec<u8>,
    kind: SnapshotFileKind,
) -> Result<()> {
    if let Some((observed, observed_kind)) = payload.get(&path) {
        if observed != &bytes || observed_kind != &kind {
            return Err(AssuranceError::Invalid(format!(
                "snapshot payload path has conflicting bytes: {}",
                path.display()
            )));
        }
    } else {
        payload.insert(path, (bytes, kind));
    }
    Ok(())
}

fn snapshot_manifest(
    domain: V2TrustDomain,
    release: &V2ReleaseIdentity,
    report_ids: &[String],
    payload: &SnapshotPayload,
    public_tree_sha256: &str,
) -> Result<SnapshotManifest> {
    let files = payload
        .iter()
        .map(|(path, (bytes, kind))| {
            Ok(SnapshotFile {
                path: path_string(path)?,
                sha256: sha256_bytes(bytes),
                bytes: bytes.len(),
                kind: *kind,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(SnapshotManifest {
        format: SNAPSHOT_FORMAT.to_owned(),
        domain: SNAPSHOT_DOMAIN.to_owned(),
        trust_domain: domain,
        test_marker: test_marker(domain),
        release: release.clone(),
        report_ids: report_ids.to_vec(),
        public_tree_sha256: public_tree_sha256.to_owned(),
        files,
    })
}

fn publication_receipt(
    domain: V2TrustDomain,
    release: &V2ReleaseIdentity,
    report_ids: &[String],
    bindings: &ReceiptBindings,
    snapshot_id: &str,
    public_tree_sha256: &str,
) -> Result<PublicationReceipt> {
    let report_set = report_ids.iter().collect::<BTreeSet<_>>();
    if report_set.len() != report_ids.len()
        || !root_map_matches(&bindings.subjects, &report_set)
        || !root_map_matches(&bindings.findings, &report_set)
        || !root_map_matches(&bindings.approvals, &report_set)
        || !root_map_matches(&bindings.transfers, &report_set)
    {
        return Err(AssuranceError::Invalid(
            "publication bindings do not exactly cover the public report set".to_owned(),
        ));
    }
    Ok(PublicationReceipt {
        format: RECEIPT_FORMAT.to_owned(),
        domain: RECEIPT_DOMAIN.to_owned(),
        trust_domain: domain,
        test_marker: test_marker(domain),
        release: release.clone(),
        report_ids: report_ids.to_vec(),
        subject_roots: bindings.subjects.clone(),
        finding_ledger_roots: bindings.findings.clone(),
        approval_lock_roots: bindings.approvals.clone(),
        release_transfer_roots: bindings.transfers.clone(),
        snapshot_id: snapshot_id.to_owned(),
        public_tree_sha256: public_tree_sha256.to_owned(),
        builder_identity: PUBLICATION_BUILDER_ID.to_owned(),
    })
}

fn install_snapshot(
    root: &ConfinedDirectory,
    snapshot_id: &str,
    manifest: &[u8],
    payload: &SnapshotPayload,
) -> Result<()> {
    let destination = PathBuf::from(snapshot_id);
    let mut expected = payload
        .iter()
        .map(|(path, (bytes, _))| (path.clone(), bytes.clone()))
        .collect::<BTreeMap<_, _>>();
    expected.insert(PathBuf::from("manifest.json"), manifest.to_vec());
    if root.directory_exists(&destination)? {
        return verify_tree(root, &destination, &expected, "snapshot");
    }
    let preparation = PathBuf::from(format!("snapshot.prepare-{}", &snapshot_id[..16]));
    root.remove_directory_if_exists(&preparation)?;
    root.create_dir_all(&preparation)?;
    write_tree(root, &preparation, &expected)?;
    root.sync_tree(&preparation)?;
    root.directory
        .sync_all()
        .map_err(|error| AssuranceError::io(snapshot_id, error))?;
    match root.rename_noreplace(&preparation, &destination) {
        Ok(()) => {
            let _ = root.directory.sync_all();
            Ok(())
        }
        Err(AssuranceError::Io { source, .. })
            if source.kind() == std::io::ErrorKind::AlreadyExists =>
        {
            root.remove_directory_if_exists(&preparation)?;
            verify_tree(root, &destination, &expected, "snapshot")
        }
        Err(error) => {
            let _ = root.remove_directory_if_exists(&preparation);
            Err(error)
        }
    }
}

fn install_receipt(root: &ConfinedDirectory, receipt_id: &str, bytes: &[u8]) -> Result<()> {
    root.create_dir_all(Path::new("receipts"))?;
    let destination = PathBuf::from(format!("receipts/{receipt_id}.json"));
    if verify_existing_receipt(root, &destination, receipt_id, bytes)? {
        return Ok(());
    }
    let preparation = PathBuf::from(format!(
        "receipts/receipt.prepare-{}.json",
        &receipt_id[..16]
    ));
    prepare_receipt(root, &preparation, receipt_id, bytes)?;
    root.sync_tree(Path::new("receipts"))?;
    root.directory
        .sync_all()
        .map_err(|error| AssuranceError::io(receipt_id, error))?;
    commit_receipt(root, &preparation, &destination, receipt_id, bytes)
}

fn verify_existing_receipt(
    root: &ConfinedDirectory,
    destination: &Path,
    receipt_id: &str,
    bytes: &[u8],
) -> Result<bool> {
    match root.read_regular(destination) {
        Ok(observed) => {
            if observed == bytes {
                Ok(true)
            } else {
                Err(AssuranceError::SnapshotConflict(format!(
                    "receipt '{receipt_id}' exists with different bytes"
                )))
            }
        }
        Err(AssuranceError::Io { source, .. }) if source.kind() == std::io::ErrorKind::NotFound => {
            Ok(false)
        }
        Err(error) => Err(error),
    }
}

fn prepare_receipt(
    root: &ConfinedDirectory,
    preparation: &Path,
    receipt_id: &str,
    bytes: &[u8],
) -> Result<()> {
    match root.read_regular(preparation) {
        Ok(observed) if observed != bytes => Err(AssuranceError::SnapshotConflict(format!(
            "receipt preparation for '{receipt_id}' has different bytes"
        ))),
        Ok(_) => Ok(()),
        Err(AssuranceError::Io { source, .. }) if source.kind() == std::io::ErrorKind::NotFound => {
            root.write_new(preparation, bytes)
        }
        Err(error) => Err(error),
    }
}

fn commit_receipt(
    root: &ConfinedDirectory,
    preparation: &Path,
    destination: &Path,
    receipt_id: &str,
    bytes: &[u8],
) -> Result<()> {
    match root.rename_noreplace(preparation, destination) {
        Ok(()) => {
            let _ = root.sync_tree(Path::new("receipts"));
            let _ = root.directory.sync_all();
            Ok(())
        }
        Err(AssuranceError::Io { source, .. })
            if source.kind() == std::io::ErrorKind::AlreadyExists =>
        {
            let observed = root.read_regular(destination)?;
            let result = if observed == bytes {
                Ok(())
            } else {
                Err(AssuranceError::SnapshotConflict(format!(
                    "receipt '{receipt_id}' raced with different bytes"
                )))
            };
            root.remove_regular_if_exists(preparation)?;
            result
        }
        Err(error) => {
            let _ = root.remove_regular_if_exists(preparation);
            Err(error)
        }
    }
}

fn write_tree(
    root: &ConfinedDirectory,
    base: &Path,
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<()> {
    for (path, bytes) in files {
        let destination = base.join(path);
        if let Some(parent) = destination.parent() {
            root.create_dir_all(parent)?;
        }
        root.write_new(&destination, bytes)?;
    }
    Ok(())
}

fn verify_tree(
    root: &ConfinedDirectory,
    base: &Path,
    expected: &BTreeMap<PathBuf, Vec<u8>>,
    kind: &str,
) -> Result<()> {
    let observed = root.collect_regular_files(base)?;
    let expected_paths = expected.keys().cloned().collect::<BTreeSet<_>>();
    if observed != expected_paths {
        return Err(AssuranceError::SnapshotConflict(format!(
            "existing {kind} has a different path set"
        )));
    }
    for (path, bytes) in expected {
        if root.read_regular(&base.join(path))? != *bytes {
            return Err(AssuranceError::SnapshotConflict(format!(
                "existing {kind} differs at {}",
                path.display()
            )));
        }
    }
    Ok(())
}

fn commit_public_generation(
    root: &ConfinedDirectory,
    files: &BTreeMap<PathBuf, Vec<u8>>,
    receipt_id: &str,
    ambient: &Path,
    root_guard: &HeldRootGuard<'_>,
) -> Result<()> {
    let preparation = PathBuf::from(format!("assurance.prepare-{}", &receipt_id[..16]));
    root.remove_directory_if_exists(&preparation)?;
    root.create_dir_all(&preparation)?;
    let generation = files
        .iter()
        .map(|(path, bytes)| {
            path.strip_prefix("assurance")
                .map(|relative| (relative.to_path_buf(), bytes.clone()))
                .map_err(|_| {
                    AssuranceError::Invalid(format!(
                        "public output escaped assurance generation: {}",
                        path.display()
                    ))
                })
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    write_tree(root, &preparation, &generation)?;
    root.sync_tree(&preparation)?;
    root.directory
        .sync_all()
        .map_err(|error| AssuranceError::io(ambient, error))?;
    root_guard.validate()?;
    if root.directory_exists(Path::new("assurance"))? {
        root.exchange(&preparation, Path::new("assurance"))?;
        let _ = root.directory.sync_all();
        let _ = root.remove_directory_if_exists(&preparation);
    } else {
        root.rename_noreplace(&preparation, Path::new("assurance"))?;
        let _ = root.directory.sync_all();
    }
    Ok(())
}

fn revalidate_context(
    repository: &V2Repository,
    prior: &ReportContext,
    staging_root: &Path,
) -> Result<()> {
    let capabilities = prior.capabilities.as_ref().ok_or_else(|| {
        AssuranceError::Invalid("publication context lacks held root capabilities".to_owned())
    })?;
    capabilities
        .repository
        .verify_ambient_identity(&repository.root)?;
    capabilities.staging.verify_ambient_identity(staging_root)?;
    let source = repository.sources.get(&prior.report.id).ok_or_else(|| {
        AssuranceError::Drift(format!(
            "report '{}' disappeared during publication",
            prior.report.id
        ))
    })?;
    let current = load_context(repository, source, staging_root)?;
    if current.report_value != prior.report_value
        || current.input_bytes != prior.input_bytes
        || current.staged_bytes != prior.staged_bytes
        || current.roots != prior.roots
    {
        return Err(AssuranceError::Drift(format!(
            "report '{}' changed before publication commit",
            prior.report.id
        )));
    }
    capabilities
        .repository
        .verify_ambient_identity(&repository.root)?;
    capabilities.staging.verify_ambient_identity(staging_root)?;
    Ok(())
}

fn normalized_subject_report(value: &Value) -> Result<Value> {
    let mut report = value.as_object().cloned().ok_or_else(|| {
        AssuranceError::Invalid("report source must normalize from an object".to_owned())
    })?;
    report.remove("lifecycle");
    for (field, transitions) in [
        ("review", REVIEW_TRANSITION_FIELDS),
        ("publication", PUBLICATION_TRANSITION_FIELDS),
    ] {
        let mut object = report
            .get(field)
            .and_then(Value::as_object)
            .cloned()
            .ok_or_else(|| AssuranceError::Invalid(format!("report {field} must be an object")))?;
        for transition in transitions {
            if object.remove(*transition).is_none() {
                return Err(AssuranceError::Invalid(format!(
                    "report {field} is missing classified transition leaf '{transition}'"
                )));
            }
        }
        report.insert(field.to_owned(), Value::Object(object));
    }
    Ok(Value::Object(report))
}

fn normalized_catalog(bytes: &[u8], selected_report: &str) -> Result<Value> {
    let mut value = yaml_json(Path::new(super::V2_CATALOG_PATH), bytes)?;
    let reports = value
        .get_mut("reports")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| AssuranceError::Invalid("catalog reports must be an array".to_owned()))?;
    let mut selected = false;
    for report in reports {
        let object = report.as_object_mut().ok_or_else(|| {
            AssuranceError::Invalid("catalog report must be an object".to_owned())
        })?;
        selected |= object.get("id").and_then(Value::as_str) == Some(selected_report);
        object.remove("manifest_sha256");
    }
    if !selected {
        return Err(AssuranceError::Invalid(format!(
            "normalized catalog omitted selected report '{selected_report}'"
        )));
    }
    Ok(value)
}

fn yaml_json(path: &Path, bytes: &[u8]) -> Result<Value> {
    let yaml: serde_yaml::Value =
        serde_yaml::from_slice(bytes).map_err(|error| AssuranceError::Parse {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;
    serde_json::to_value(yaml).map_err(|error| AssuranceError::Parse {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let value = serde_json::to_value(value).map_err(|error| {
        AssuranceError::Invalid(format!("cannot serialize assurance identity: {error}"))
    })?;
    let normalized = canonical_value(value)?;
    let mut bytes = serde_json::to_vec(&normalized).map_err(|error| {
        AssuranceError::Invalid(format!("cannot encode assurance identity: {error}"))
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn canonical_value(value: Value) -> Result<Value> {
    match value {
        Value::Object(values) => {
            let mut ordered = values.into_iter().collect::<Vec<_>>();
            ordered.sort_by(|left, right| left.0.cmp(&right.0));
            let values = ordered
                .into_iter()
                .map(|(key, value)| canonical_value(value).map(|value| (key, value)))
                .collect::<Result<serde_json::Map<_, _>>>()?;
            Ok(Value::Object(values))
        }
        Value::Array(values) => values
            .into_iter()
            .map(canonical_value)
            .collect::<Result<Vec<_>>>()
            .map(Value::Array),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => Ok(value),
    }
}

fn digest_value(value: &Value) -> Result<String> {
    canonical_bytes(value).map(|bytes| sha256_bytes(&bytes))
}

fn digest_files(domain: &str, files: &BTreeMap<PathBuf, Vec<u8>>) -> Result<String> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(domain.as_bytes());
    bytes.push(0);
    for (path, value) in files {
        bytes.extend_from_slice(path_string(path)?.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(sha256_bytes(value).as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(value.len().to_string().as_bytes());
        bytes.push(b'\n');
    }
    Ok(sha256_bytes(&bytes))
}

fn path_string(path: &Path) -> Result<String> {
    path.to_str()
        .map(|value| value.replace('\\', "/"))
        .ok_or_else(|| AssuranceError::Invalid("assurance paths must be UTF-8".to_owned()))
}

/// Verifies a production release snapshot and independently supplied identity.
///
/// # Errors
///
/// Returns an error for malformed, test-only, drifted, incomplete, or
/// release-mismatched snapshot and receipt artifacts.
pub fn verify_v2_release_snapshot(
    snapshot_dir: &Path,
    receipt_path: &Path,
    expected_release: &V2ReleaseIdentity,
) -> Result<V2ReleaseVerification> {
    expected_release.validate()?;
    if !snapshot_dir.is_absolute() || !receipt_path.is_absolute() {
        return Err(AssuranceError::Invalid(
            "release snapshot and receipt paths must be absolute".to_owned(),
        ));
    }
    let (snapshot_id, manifest, public_tree_sha256) =
        verify_snapshot_content(snapshot_dir, expected_release, V2TrustDomain::Production)?;
    let verification = verify_receipt_content(
        receipt_path,
        expected_release,
        V2TrustDomain::Production,
        &snapshot_id,
        &manifest,
        public_tree_sha256,
    )?;
    let receipt = read_receipt(receipt_path)?;
    verify_release_authority(snapshot_dir, &manifest, &receipt, expected_release)?;
    Ok(verification)
}

fn verify_snapshot_content(
    snapshot_dir: &Path,
    expected_release: &V2ReleaseIdentity,
    expected_domain: V2TrustDomain,
) -> Result<(String, SnapshotManifest, String)> {
    let (snapshot_id, snapshot, manifest) =
        open_verified_snapshot(snapshot_dir, expected_release, expected_domain)?;
    let (expected_paths, public_files) =
        verify_manifest_files(&snapshot, &manifest, expected_domain)?;
    if snapshot.collect_all_regular_files()? != expected_paths {
        return Err(AssuranceError::SnapshotConflict(
            "snapshot tree contains an unmanifested or missing file".to_owned(),
        ));
    }
    let public_tree_sha256 = digest_files("openwepp-assurance-public-tree-v1", &public_files)?;
    if public_tree_sha256 != manifest.public_tree_sha256 {
        return Err(AssuranceError::SnapshotConflict(
            "snapshot public-tree digest does not reconstruct".to_owned(),
        ));
    }

    Ok((snapshot_id, manifest, public_tree_sha256))
}

fn open_verified_snapshot(
    snapshot_dir: &Path,
    expected_release: &V2ReleaseIdentity,
    expected_domain: V2TrustDomain,
) -> Result<(String, ConfinedDirectory, SnapshotManifest)> {
    let snapshot_id = snapshot_dir
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| AssuranceError::Invalid("snapshot directory has no UTF-8 ID".to_owned()))?;
    validate_sha256(snapshot_id, "snapshot ID")?;
    let snapshot = ConfinedDirectory::open_ambient(snapshot_dir, false)?;
    let manifest_bytes = snapshot.read_regular(Path::new("manifest.json"))?;
    if sha256_bytes(&manifest_bytes) != snapshot_id {
        return Err(AssuranceError::SnapshotConflict(
            "snapshot directory ID does not match manifest content".to_owned(),
        ));
    }
    let manifest: SnapshotManifest =
        serde_json::from_slice(&manifest_bytes).map_err(|error| AssuranceError::Parse {
            path: snapshot_dir.join("manifest.json"),
            message: error.to_string(),
        })?;
    if manifest.format != SNAPSHOT_FORMAT
        || manifest.domain != SNAPSHOT_DOMAIN
        || manifest.trust_domain != expected_domain
        || !marker_matches(expected_domain, manifest.test_marker.as_deref())
        || manifest.release != *expected_release
    {
        return Err(AssuranceError::Invalid(
            "release snapshot format, trust domain, or release identity is invalid".to_owned(),
        ));
    }
    Ok((snapshot_id.to_owned(), snapshot, manifest))
}

type VerifiedManifestFiles = (BTreeSet<PathBuf>, BTreeMap<PathBuf, Vec<u8>>);
type VerifiedManifestFile = (PathBuf, Option<(PathBuf, Vec<u8>)>);

fn verify_manifest_files(
    snapshot: &ConfinedDirectory,
    manifest: &SnapshotManifest,
    expected_domain: V2TrustDomain,
) -> Result<VerifiedManifestFiles> {
    let mut expected_paths = BTreeSet::from([PathBuf::from("manifest.json")]);
    let mut public_files = BTreeMap::new();
    for file in &manifest.files {
        let (path, public) = verify_manifest_file(snapshot, file, expected_domain)?;
        if !expected_paths.insert(path.clone()) {
            return Err(AssuranceError::Invalid(format!(
                "snapshot manifest repeats path '{}'",
                file.path
            )));
        }
        if let Some((relative, bytes)) = public {
            public_files.insert(relative, bytes);
        }
    }
    Ok((expected_paths, public_files))
}

fn verify_manifest_file(
    snapshot: &ConfinedDirectory,
    file: &SnapshotFile,
    expected_domain: V2TrustDomain,
) -> Result<VerifiedManifestFile> {
    let path = PathBuf::from(&file.path);
    super::confined::validate_relative(&path)?;
    validate_sha256(&file.sha256, "snapshot file digest")?;
    let bytes = snapshot.read_regular(&path)?;
    if bytes.len() != file.bytes || sha256_bytes(&bytes) != file.sha256 {
        return Err(AssuranceError::SnapshotConflict(format!(
            "snapshot payload differs at '{}'",
            file.path
        )));
    }
    let public = if file.kind == SnapshotFileKind::Public {
        Some(verify_public_snapshot_file(
            &path,
            &file.path,
            bytes,
            expected_domain,
        )?)
    } else {
        None
    };
    Ok((path, public))
}

fn verify_public_snapshot_file(
    path: &Path,
    manifest_path: &str,
    bytes: Vec<u8>,
    expected_domain: V2TrustDomain,
) -> Result<(PathBuf, Vec<u8>)> {
    let relative = path.strip_prefix("public").map_err(|_| {
        AssuranceError::Invalid(format!(
            "public snapshot entry is outside public/: {manifest_path}"
        ))
    })?;
    if expected_domain == V2TrustDomain::Production
        && std::str::from_utf8(&bytes).is_ok_and(|text| text.contains(TEST_BANNER))
    {
        return Err(AssuranceError::Invalid(
            "production release rejects TEST ONLY publication bytes".to_owned(),
        ));
    }
    Ok((relative.to_path_buf(), bytes))
}

fn verify_release_authority(
    snapshot_dir: &Path,
    manifest: &SnapshotManifest,
    receipt: &PublicationReceipt,
    expected_release: &V2ReleaseIdentity,
) -> Result<()> {
    if manifest.report_ids.is_empty() {
        return Err(AssuranceError::Invalid(
            "production release snapshot must contain at least one approved report".to_owned(),
        ));
    }
    let (observed_payload, public_files) = read_snapshot_payload(snapshot_dir, manifest)?;
    let catalog = parse_release_catalog(snapshot_dir, &public_files)?;
    let report_ids = catalog
        .reports
        .iter()
        .map(|entry| entry.report_id.clone())
        .collect::<Vec<_>>();
    if report_ids != manifest.report_ids || report_ids != receipt.report_ids {
        return Err(AssuranceError::Invalid(
            "production catalog, snapshot, and receipt report sets differ".to_owned(),
        ));
    }

    let (contexts, bindings, expected_catalog) =
        reconstruct_release_contexts(snapshot_dir, &public_files, &catalog, expected_release)?;
    if receipt.subject_roots != bindings.subjects
        || receipt.finding_ledger_roots != bindings.findings
        || receipt.approval_lock_roots != bindings.approvals
        || receipt.release_transfer_roots != bindings.transfers
    {
        return Err(AssuranceError::Drift(
            "publication receipt roots do not reconstruct from approved source".to_owned(),
        ));
    }
    let mut expected_public_files = BTreeMap::new();
    for context in &contexts {
        install_context_files(
            &mut expected_public_files,
            context,
            V2TrustDomain::Production,
        )?;
    }
    expected_public_files.insert(
        PathBuf::from("assurance/catalog.json"),
        canonical_bytes(&expected_catalog)?,
    );
    expected_public_files.insert(
        PathBuf::from("assurance/README.md"),
        render_public_readme(&expected_catalog),
    );
    if expected_public_files != public_files {
        return Err(AssuranceError::Drift(
            "production public tree does not exactly reconstruct from approved source".to_owned(),
        ));
    }
    let expected_payload = snapshot_payload(&contexts, &expected_public_files, BTreeMap::new())?;
    if expected_payload != observed_payload {
        return Err(AssuranceError::Drift(
            "production snapshot source/public payload does not exactly reconstruct".to_owned(),
        ));
    }
    Ok(())
}

fn read_snapshot_payload(
    snapshot_dir: &Path,
    manifest: &SnapshotManifest,
) -> Result<(SnapshotPayload, BTreeMap<PathBuf, Vec<u8>>)> {
    let snapshot = ConfinedDirectory::open_ambient(snapshot_dir, false)?;
    let payload = manifest
        .files
        .iter()
        .map(|file| {
            let path = PathBuf::from(&file.path);
            snapshot
                .read_regular(&path)
                .map(|bytes| (path, (bytes, file.kind)))
        })
        .collect::<Result<SnapshotPayload>>()?;
    let public_files = payload
        .iter()
        .filter(|(_, (_, kind))| *kind == SnapshotFileKind::Public)
        .map(|(path, (bytes, _))| {
            path.strip_prefix("public")
                .map(|relative| (relative.to_path_buf(), bytes.clone()))
                .map_err(|_| {
                    AssuranceError::Invalid(format!(
                        "public snapshot path escaped public/: {}",
                        path.display()
                    ))
                })
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    Ok((payload, public_files))
}

fn parse_release_catalog(
    snapshot_dir: &Path,
    public_files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<PublicCatalog> {
    let catalog_bytes = public_files
        .get(Path::new("assurance/catalog.json"))
        .ok_or_else(|| {
            AssuranceError::Invalid("production snapshot lacks public catalog".to_owned())
        })?;
    let catalog: PublicCatalog =
        serde_json::from_slice(catalog_bytes).map_err(|error| AssuranceError::Parse {
            path: snapshot_dir.join("public/assurance/catalog.json"),
            message: error.to_string(),
        })?;
    if catalog.format != PUBLIC_FORMAT
        || catalog.trust_domain != V2TrustDomain::Production
        || catalog.test_marker.is_some()
        || catalog.reports.is_empty()
    {
        return Err(AssuranceError::Invalid(
            "production public catalog format, trust domain, or report set is invalid".to_owned(),
        ));
    }
    reject_duplicate_catalog_reports(&catalog)?;
    Ok(catalog)
}

fn reconstruct_release_contexts(
    snapshot_dir: &Path,
    public_files: &BTreeMap<PathBuf, Vec<u8>>,
    catalog: &PublicCatalog,
    expected_release: &V2ReleaseIdentity,
) -> Result<(Vec<ReportContext>, ReceiptBindings, PublicCatalog)> {
    let mut contexts = Vec::new();
    let mut bindings = ReceiptBindings::default();
    let mut expected_catalog = PublicCatalog {
        format: PUBLIC_FORMAT.to_owned(),
        trust_domain: V2TrustDomain::Production,
        test_marker: None,
        reports: Vec::new(),
    };
    for entry in &catalog.reports {
        let source_root = snapshot_dir.join("source").join(&entry.report_id);
        let repository = V2Repository::open(&source_root)?;
        if repository.trust_domain != V2TrustDomain::Production {
            return Err(AssuranceError::Invalid(format!(
                "snapshotted source for '{}' is not production-domain",
                entry.report_id
            )));
        }
        let source = repository.sources.get(&entry.report_id).ok_or_else(|| {
            AssuranceError::Invalid(format!(
                "snapshotted source catalog omits public report '{}'",
                entry.report_id
            ))
        })?;
        let report_prefix = PathBuf::from(format!(
            "assurance/reports/{}/{}",
            entry.report_id, entry.version
        ));
        let staged_bytes = public_files
            .iter()
            .filter(|(path, _)| path.starts_with(&report_prefix))
            .map(|(path, bytes)| (Path::new("usersum").join(path), bytes.clone()))
            .collect::<BTreeMap<_, _>>();
        let context = context_from_staged(&repository, source, staged_bytes, None)?;
        validate_publishable(
            &repository,
            &context,
            V2TrustDomain::Production,
            expected_release,
        )?;
        let reconstructed_entry = catalog_entry(&context)?;
        if reconstructed_entry != *entry {
            return Err(AssuranceError::Drift(format!(
                "public catalog entry for '{}' does not reconstruct from approved source",
                entry.report_id
            )));
        }
        validate_context_narrative_link(&context, entry)?;
        bindings.insert(&context)?;
        expected_catalog.reports.push(reconstructed_entry);
        contexts.push(context);
    }
    Ok((contexts, bindings, expected_catalog))
}

fn validate_context_narrative_link(
    context: &ReportContext,
    entry: &PublicCatalogEntry,
) -> Result<()> {
    let narrative = Path::new(&entry.related_model_narrative);
    let source_path = Path::new("usersum").join(narrative);
    if !context.input_bytes.contains_key(&source_path) {
        return Err(AssuranceError::Invalid(format!(
            "approved source lacks related model narrative for '{}'",
            entry.report_id
        )));
    }
    let report = context
        .staged_bytes
        .get(&Path::new("usersum").join(&entry.report_path))
        .ok_or_else(|| AssuranceError::Invalid("approved staged report is missing".to_owned()))?;
    let report = std::str::from_utf8(report).map_err(|error| AssuranceError::Parse {
        path: PathBuf::from(&entry.report_path),
        message: error.to_string(),
    })?;
    let expected_target = format!("../../../../{}", path_string(narrative)?);
    if !has_canonical_markdown_link(report, &expected_target) {
        return Err(AssuranceError::Invalid(format!(
            "approved report '{}' lacks canonical related-model Markdown link",
            entry.report_id
        )));
    }
    Ok(())
}

fn read_receipt(receipt_path: &Path) -> Result<PublicationReceipt> {
    let parent = receipt_path.parent().ok_or_else(|| {
        AssuranceError::Invalid("receipt path has no parent directory".to_owned())
    })?;
    let name = receipt_path
        .file_name()
        .ok_or_else(|| AssuranceError::Invalid("receipt path has no file name".to_owned()))?;
    let root = ConfinedDirectory::open_ambient(parent, false)?;
    let bytes = root.read_regular(Path::new(name))?;
    serde_json::from_slice(&bytes).map_err(|error| AssuranceError::Parse {
        path: receipt_path.to_path_buf(),
        message: error.to_string(),
    })
}

fn verify_receipt_content(
    receipt_path: &Path,
    expected_release: &V2ReleaseIdentity,
    expected_domain: V2TrustDomain,
    snapshot_id: &str,
    manifest: &SnapshotManifest,
    public_tree_sha256: String,
) -> Result<V2ReleaseVerification> {
    let receipt_parent = receipt_path.parent().ok_or_else(|| {
        AssuranceError::Invalid("receipt path has no parent directory".to_owned())
    })?;
    let receipt_name = receipt_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| AssuranceError::Invalid("receipt path has no UTF-8 file name".to_owned()))?;
    let receipt_id = receipt_name
        .strip_suffix(".json")
        .ok_or_else(|| AssuranceError::Invalid("receipt file must use <sha256>.json".to_owned()))?;
    validate_sha256(receipt_id, "receipt ID")?;
    let receipts = ConfinedDirectory::open_ambient(receipt_parent, false)?;
    let receipt_bytes = receipts.read_regular(Path::new(receipt_name))?;
    if sha256_bytes(&receipt_bytes) != receipt_id {
        return Err(AssuranceError::SnapshotConflict(
            "receipt file ID does not match its content".to_owned(),
        ));
    }
    let receipt: PublicationReceipt =
        serde_json::from_slice(&receipt_bytes).map_err(|error| AssuranceError::Parse {
            path: receipt_path.to_path_buf(),
            message: error.to_string(),
        })?;
    if receipt.format != RECEIPT_FORMAT
        || receipt.domain != RECEIPT_DOMAIN
        || receipt.trust_domain != expected_domain
        || !marker_matches(expected_domain, receipt.test_marker.as_deref())
        || receipt.release != *expected_release
        || receipt.snapshot_id != snapshot_id
        || receipt.public_tree_sha256 != public_tree_sha256
        || receipt.report_ids != manifest.report_ids
        || receipt.builder_identity != PUBLICATION_BUILDER_ID
    {
        return Err(AssuranceError::Invalid(
            "publication receipt does not bind the verified production snapshot".to_owned(),
        ));
    }
    let report_set = receipt.report_ids.iter().collect::<BTreeSet<_>>();
    if report_set.len() != receipt.report_ids.len()
        || !root_map_matches(&receipt.subject_roots, &report_set)
        || !root_map_matches(&receipt.finding_ledger_roots, &report_set)
        || !root_map_matches(&receipt.approval_lock_roots, &report_set)
        || !root_map_matches(&receipt.release_transfer_roots, &report_set)
    {
        return Err(AssuranceError::Invalid(
            "publication receipt root maps do not exactly cover report IDs".to_owned(),
        ));
    }
    Ok(V2ReleaseVerification {
        report_ids: receipt.report_ids,
        snapshot_id: snapshot_id.to_owned(),
        receipt_id: receipt_id.to_owned(),
        public_tree_sha256,
    })
}

fn root_map_matches(roots: &BTreeMap<String, String>, reports: &BTreeSet<&String>) -> bool {
    roots.len() == reports.len()
        && roots
            .iter()
            .all(|(report, root)| reports.contains(report) && is_sha256(root))
}

fn validate_sha256(value: &str, label: &str) -> Result<()> {
    if is_sha256(value) {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "{label} must be 64 lowercase hexadecimal characters"
        )))
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_classification_exactly_covers_current_nonidentity_leaves() {
        let identities = BTreeSet::from(["id", "title", "owner"]);
        let review_transitions = REVIEW_TRANSITION_FIELDS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let publication_transitions = PUBLICATION_TRANSITION_FIELDS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert!(identities.is_disjoint(&review_transitions));
        assert!(identities.is_disjoint(&publication_transitions));
        assert_eq!(
            super::super::REVIEW_FIELDS
                .iter()
                .copied()
                .collect::<BTreeSet<_>>(),
            identities.union(&review_transitions).copied().collect()
        );
        assert_eq!(
            super::super::PUBLICATION_FIELDS
                .iter()
                .copied()
                .collect::<BTreeSet<_>>(),
            identities
                .union(&publication_transitions)
                .copied()
                .collect()
        );
    }

    #[test]
    fn unclassified_future_transition_subtree_leaves_default_to_subject_bound() {
        let transition_object = |fields: &[&str]| {
            fields
                .iter()
                .map(|field| ((*field).to_owned(), Value::Null))
                .chain([
                    ("id".to_owned(), Value::String("id".to_owned())),
                    ("title".to_owned(), Value::String("title".to_owned())),
                    ("owner".to_owned(), Value::String("owner".to_owned())),
                    (
                        "future_subject_leaf".to_owned(),
                        Value::String("must remain".to_owned()),
                    ),
                ])
                .collect::<serde_json::Map<_, _>>()
        };
        let report = serde_json::json!({
            "lifecycle": "APPROVED",
            "review": transition_object(REVIEW_TRANSITION_FIELDS),
            "publication": transition_object(PUBLICATION_TRANSITION_FIELDS),
        });
        let normalized = normalized_subject_report(&report).unwrap();
        assert_eq!(
            normalized["review"]["future_subject_leaf"],
            Value::String("must remain".to_owned())
        );
        assert_eq!(
            normalized["publication"]["future_subject_leaf"],
            Value::String("must remain".to_owned())
        );
    }

    #[test]
    fn every_finding_leaf_changes_the_finding_layer() {
        let review_value = review_value();
        let review: Review = serde_json::from_value(review_value.clone()).unwrap();
        let finding = finding_root("a", &review).unwrap();
        for (pointer, replacement) in [
            ("/review_charge", serde_json::json!("changed charge")),
            ("/build_maintainer_id", serde_json::json!("changed-builder")),
            (
                "/material_producer_ids",
                serde_json::json!(["changed-producer"]),
            ),
            ("/findings/0/id", serde_json::json!("F-2")),
            ("/findings/0/summary", serde_json::json!("changed summary")),
            ("/findings/0/severity", serde_json::json!("advisory")),
            ("/findings/0/disposition", serde_json::json!("accepted")),
            ("/findings/0/rationale", serde_json::json!("rationale")),
            (
                "/findings/0/resolution",
                serde_json::json!("changed resolution"),
            ),
            (
                "/findings/0/verification",
                serde_json::json!("changed verification"),
            ),
            (
                "/findings/0/verifier_id",
                serde_json::json!("changed-verifier"),
            ),
        ] {
            let changed: Review =
                serde_json::from_value(mutate(review_value.clone(), pointer, replacement)).unwrap();
            assert_ne!(finding, finding_root("a", &changed).unwrap(), "{pointer}");
        }
    }

    #[test]
    fn every_approval_leaf_changes_the_approval_layer() {
        let review_value = review_value();
        let review: Review = serde_json::from_value(review_value.clone()).unwrap();
        let finding = finding_root("a", &review).unwrap();
        let approval = approval_root(&finding, &review).unwrap();
        for (pointer, replacement) in [
            ("/state", serde_json::json!("changed-state")),
            ("/decision", serde_json::json!("changed-decision")),
            ("/approvals/0/role", serde_json::json!("assurance_steward")),
            (
                "/approvals/0/principal_id",
                serde_json::json!("changed-reviewer"),
            ),
            (
                "/approvals/0/finding_ledger_root",
                serde_json::json!("d".repeat(64)),
            ),
            (
                "/approvals/0/decision",
                serde_json::json!("changed-decision"),
            ),
            (
                "/approvals/0/competence_basis",
                serde_json::json!("changed competence"),
            ),
            (
                "/approvals/0/independence_attestation",
                serde_json::json!("changed independence"),
            ),
            ("/approvals/0/approved_on", serde_json::json!("2026-07-17")),
            (
                "/independence_assessment",
                serde_json::json!("changed assessment"),
            ),
        ] {
            let changed: Review =
                serde_json::from_value(mutate(review_value.clone(), pointer, replacement)).unwrap();
            assert_ne!(
                approval,
                approval_root(&finding, &changed).unwrap(),
                "{pointer}"
            );
        }
    }

    #[test]
    fn every_transfer_leaf_changes_the_transfer_layer() {
        let review: Review = serde_json::from_value(review_value()).unwrap();
        let finding = finding_root("a", &review).unwrap();
        let approval = approval_root(&finding, &review).unwrap();
        let publication_value = serde_json::json!({
            "id": "publication", "title": "Publication", "owner": "owner",
            "state": "APPROVED", "approval_lock_root": "c".repeat(64),
            "target_release_commit": "d".repeat(40),
            "target_release_configuration": "configuration",
            "prior_realization": "prior", "candidate_realization": "candidate",
            "impact_assessment": "impact", "reproduction_disposition": "reproduced",
            "semantic_differences": ["difference"], "release_owner_id": "release-owner",
            "assurance_steward_id": "steward", "publication_date": "2026-07-16",
            "public_path": "assurance/report.md", "release_transfer_root": "e".repeat(64),
            "export_authorized": false, "vendoring_authorized": false,
            "supersedes": null, "withdrawn": false
        });
        let publication: Publication = serde_json::from_value(publication_value.clone()).unwrap();
        let transfer = transfer_root(&approval, "APPROVED", &publication).unwrap();
        for (pointer, replacement) in [
            ("/state", serde_json::json!("changed-state")),
            ("/target_release_commit", serde_json::json!("e".repeat(40))),
            (
                "/target_release_configuration",
                serde_json::json!("changed-configuration"),
            ),
            ("/prior_realization", serde_json::json!("changed prior")),
            (
                "/candidate_realization",
                serde_json::json!("changed candidate"),
            ),
            ("/impact_assessment", serde_json::json!("changed impact")),
            (
                "/reproduction_disposition",
                serde_json::json!("changed reproduction"),
            ),
            (
                "/semantic_differences",
                serde_json::json!(["changed difference"]),
            ),
            ("/release_owner_id", serde_json::json!("changed-owner")),
            (
                "/assurance_steward_id",
                serde_json::json!("changed-steward"),
            ),
            ("/publication_date", serde_json::json!("2026-07-17")),
            ("/public_path", serde_json::json!("assurance/changed.md")),
            ("/export_authorized", serde_json::json!(true)),
            ("/vendoring_authorized", serde_json::json!(true)),
            ("/supersedes", serde_json::json!("prior-report")),
            ("/withdrawn", serde_json::json!(true)),
        ] {
            let changed: Publication =
                serde_json::from_value(mutate(publication_value.clone(), pointer, replacement))
                    .unwrap();
            assert_ne!(
                transfer,
                transfer_root(&approval, "APPROVED", &changed).unwrap(),
                "{pointer}"
            );
        }
        assert_ne!(
            transfer,
            transfer_root(&approval, "changed-lifecycle", &publication).unwrap()
        );
    }

    #[test]
    fn canonical_link_recognizer_rejects_nonrendered_examples() {
        let target = "../../../../model.md";
        assert!(has_canonical_markdown_link(
            "See [the model](../../../../model.md).",
            target
        ));
        for fake in [
            "```text\n[example](../../../../model.md)\n```",
            "````\n```not-a-close\n[example](../../../../model.md)\n````",
            "<div>\n[example](../../../../model.md)\n</div>",
            "<!-- [comment](../../../../model.md) -->",
            "`[code](../../../../model.md)`",
            "\\[escaped](../../../../model.md)",
            "![image](../../../../model.md)",
        ] {
            assert!(!has_canonical_markdown_link(fake, target), "{fake}");
        }
    }

    fn review_value() -> Value {
        serde_json::json!({
            "id": "review", "title": "Review", "owner": "owner",
            "state": "APPROVED", "decision": "approved",
            "subject_root": "a".repeat(64),
            "review_charge": "charge", "build_maintainer_id": "builder",
            "material_producer_ids": ["producer"],
            "findings": [{
                "id": "F-1", "summary": "summary", "severity": "blocking",
                "disposition": "resolved_and_verified", "rationale": null,
                "resolution": "resolution", "verification": "verification",
                "verifier_id": "verifier"
            }],
            "finding_ledger_root": "b".repeat(64),
            "approvals": [{
                "role": "scientific", "principal_id": "reviewer",
                "finding_ledger_root": "b".repeat(64), "decision": "approved",
                "competence_basis": "competence",
                "independence_attestation": "independence",
                "approved_on": "2026-07-16"
            }],
            "approval_lock_root": "c".repeat(64),
            "independence_assessment": "assessment"
        })
    }

    fn mutate(mut value: Value, pointer: &str, replacement: Value) -> Value {
        *value.pointer_mut(pointer).expect("valid mutation pointer") = replacement;
        value
    }
}
