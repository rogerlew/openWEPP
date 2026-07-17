use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::amendment_support::{
    gate_argv, gate_id, parse_yaml, read_regular, receipt_bytes, receipt_id, render_yaml,
    require_optional_text, require_text, set_yaml_string, yaml_key,
};
use super::confined::ConfinedDirectory;
use super::identity::{
    IDENTITY_LOCK_PATH, IdentityLock, ReviewEvent, ReviewLock, calculate_review_lock,
};
use crate::{AssuranceError, Result, sha256_bytes};

pub(super) const V2_ROOT: &str = "assurance/v2";
pub(super) const NEXT_ROOT: &str = "assurance/.v2.amend.next";
const CATALOG_PATH: &str = "assurance/v2/catalog.yaml";
const IMPLEMENTATION_CONTRACT_PATHS: &[&str] = &[
    "assurance/v2/README.md",
    "assurance/v2/schemas/catalog.schema.json",
    "assurance/v2/schemas/identity-lock.schema.json",
    "assurance/v2/schemas/principals.schema.json",
    "assurance/v2/schemas/report.schema.json",
    "assurance/v2/schemas/review-event.schema.json",
    "assurance/v2/schemas/review-lock.schema.json",
    "assurance/v2/schemas/transaction-receipt.schema.json",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V2AmendMode {
    Check,
    Apply,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V2RecoveryAction {
    Inspect,
    FinishCleanup,
    RestoreOld,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct V2AmendmentReceipt {
    pub schema_version: u32,
    pub operation: String,
    pub impact_class: String,
    pub changed: bool,
    pub old_generation_id: Option<String>,
    pub new_generation_id: String,
    pub affected_reports: Vec<String>,
    pub affected_paths: Vec<String>,
    pub invalidated_authority: Vec<String>,
    pub gate_ids: Vec<String>,
    pub gate_argv: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct V2RoleRequest {
    pub schema_version: u32,
    pub operation: String,
    pub principal_id: String,
    pub assignments: V2RoleAssignments,
    pub attestation: V2Attestation,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct V2RoleAssignments {
    #[serde(default)]
    pub report_lead: bool,
    #[serde(default)]
    pub material_producer: bool,
    #[serde(default)]
    pub build_maintainer: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct V2Attestation {
    pub authority: String,
    pub statement: String,
    pub recorded_on: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct V2LifecycleRequest {
    schema_version: u32,
    event_type: String,
    principal_id: String,
    decision: String,
    rationale: String,
    recorded_on: String,
    authority_source: String,
    #[serde(default)]
    predecessor_event_ids: Vec<String>,
    #[serde(default)]
    review_charge: Option<String>,
    #[serde(default)]
    build_maintainer_id: Option<String>,
    #[serde(default)]
    material_producer_ids: Vec<String>,
    #[serde(default)]
    independence_assessment: Option<String>,
    #[serde(default)]
    scientific_approver_id: Option<String>,
    #[serde(default)]
    competence_basis: Option<String>,
    #[serde(default)]
    independence_attestation: Option<String>,
    #[serde(default)]
    target_release_commit: Option<String>,
    #[serde(default)]
    target_release_configuration: Option<String>,
    #[serde(default)]
    prior_realization: Option<String>,
    #[serde(default)]
    candidate_realization: Option<String>,
    #[serde(default)]
    impact_assessment: Option<String>,
    #[serde(default)]
    reproduction_disposition: Option<String>,
    #[serde(default)]
    semantic_differences: Vec<String>,
    #[serde(default)]
    assurance_steward_id: Option<String>,
    #[serde(default)]
    publication_date: Option<String>,
    #[serde(default)]
    public_path: Option<String>,
    #[serde(default)]
    superseding_report_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct V2PrincipalRequest {
    schema_version: u32,
    principal_id: String,
    display_name: Option<String>,
    affiliations: Option<Vec<String>>,
    roles: Option<Vec<String>>,
    identity_authority: Option<String>,
    identity_reference: Option<String>,
    attestation: V2Attestation,
}

#[derive(Debug, Clone, Serialize)]
pub struct V2Inspection {
    pub schema_version: u32,
    pub report_id: String,
    pub generation_id: String,
    pub lifecycle: String,
    pub science_root: String,
    pub communication_root: String,
    pub attribution_root: String,
    pub review_governance_root: String,
    pub content_review_subject_root: String,
    pub finding_ledger_root: Option<String>,
    pub approval_lock_root: Option<String>,
    pub active_event_ids: Vec<String>,
    pub invalidated_event_ids: Vec<String>,
}

impl V2AmendmentReceipt {
    /// Renders the deterministic receipt as pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn render_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map(|mut text| {
                text.push('\n');
                text
            })
            .map_err(|error| {
                AssuranceError::Invalid(format!("amendment receipt serialization failed: {error}"))
            })
    }
}

impl V2Inspection {
    /// Renders inspection data as pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn render_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map(|mut text| {
                text.push('\n');
                text
            })
            .map_err(|error| {
                AssuranceError::Invalid(format!("inspection serialization failed: {error}"))
            })
    }

    #[must_use]
    pub fn render_human(&self) -> String {
        format!(
            "assurance inspection: PASS\nreport: {}\nlifecycle: {}\ngeneration: {}\nscience_root: {}\ncommunication_root: {}\nattribution_root: {}\nreview_governance_root: {}\ncontent_review_subject_root: {}\nfinding_ledger_root: {}\napproval_lock_root: {}\nactive_events: {}\ninvalidated_events: {}\n",
            self.report_id,
            self.lifecycle,
            self.generation_id,
            self.science_root,
            self.communication_root,
            self.attribution_root,
            self.review_governance_root,
            self.content_review_subject_root,
            self.finding_ledger_root
                .as_deref()
                .unwrap_or("not_applicable"),
            self.approval_lock_root
                .as_deref()
                .unwrap_or("not_applicable"),
            self.active_event_ids.len(),
            self.invalidated_event_ids.len(),
        )
    }
}

/// Inspects one report's current generated identity without mutation.
///
/// # Errors
///
/// Returns an error for an unknown report or invalid generated identity.
pub fn inspect_report(root: &Path, report_id: &str) -> Result<V2Inspection> {
    let identity = IdentityLock::load(root)?;
    identity.verify_files(root)?;
    let lock = load_review_lock(root, report_id)?;
    Ok(V2Inspection {
        schema_version: 1,
        report_id: report_id.to_owned(),
        generation_id: identity.generation_id,
        lifecycle: lock.lifecycle,
        science_root: lock.science_root,
        communication_root: lock.communication_root,
        attribution_root: lock.attribution_root,
        review_governance_root: lock.review_governance_root,
        content_review_subject_root: lock.content_review_subject_root,
        finding_ledger_root: lock.finding_ledger_root,
        approval_lock_root: lock.approval_lock_root,
        active_event_ids: lock.event_ids,
        invalidated_event_ids: lock.invalidated_event_ids,
    })
}

/// Rebinds generated review locks after an assurance implementation change.
///
/// The operation cannot alter authored sources or review events. Existing
/// approval bindings must validate against the recalculated implementation
/// identity or the transaction fails closed.
///
/// # Errors
///
/// Returns an error for stale source identity, invalid approval authority, or
/// transaction failure.
pub fn rebind_implementation(root: &Path, mode: V2AmendMode) -> Result<V2AmendmentReceipt> {
    let catalog_path = PathBuf::from(CATALOG_PATH);
    let catalog: serde_yaml::Value =
        parse_yaml(&catalog_path, &read_regular(root, &catalog_path)?)?;
    let mut reports = report_paths(&catalog)?
        .into_iter()
        .map(|(report_id, _)| report_id)
        .collect::<Vec<_>>();
    reports.sort();
    let previous = IdentityLock::load(root)?;
    let exceptions = IMPLEMENTATION_CONTRACT_PATHS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    previous.verify_files_except(root, &exceptions)?;
    let mut projected = implementation_contract_replacements(root)?;
    regenerate_review_locks(root, &mut projected, &BTreeMap::new(), &reports)?;
    let review_locks = replacement_review_lock_digests(&projected);
    let mut sources = previous.sources.clone();
    for (path, bytes) in &projected {
        let text = path.to_string_lossy();
        if IMPLEMENTATION_CONTRACT_PATHS.contains(&text.as_ref()) {
            sources.insert(text.into_owned(), sha256_bytes(bytes));
        }
    }
    if review_locks == previous.review_locks && sources == previous.sources {
        return no_op_receipt(
            root,
            "rebind-implementation",
            "scientific-full",
            reports,
            None,
        );
    }
    prepare_or_apply_successor_with_drift(
        root,
        "rebind-implementation",
        "scientific-full",
        reports,
        projected,
        BTreeMap::new(),
        mode,
        None,
        IMPLEMENTATION_CONTRACT_PATHS
            .iter()
            .map(|path| (*path).to_owned())
            .collect(),
    )
}

fn implementation_contract_replacements(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    IMPLEMENTATION_CONTRACT_PATHS
        .iter()
        .map(|path| {
            let path = PathBuf::from(path);
            read_regular(root, &path).map(|bytes| (path, bytes))
        })
        .collect()
}

/// Plans or applies a typed bibliographic attribution correction.
///
/// # Errors
///
/// Returns an error for invalid input, identity drift, or transaction failure.
pub fn amend_attribution(
    root: &Path,
    principal_id: &str,
    display_name: Option<&str>,
    affiliations: Option<Vec<String>>,
    mode: V2AmendMode,
) -> Result<V2AmendmentReceipt> {
    amend_attribution_at_generation(root, principal_id, display_name, affiliations, mode, None)
}

/// Applies attribution only when the supplied generation remains current.
///
/// # Errors
///
/// Returns an error for a stale generation or invalid amendment.
pub fn amend_attribution_at_generation(
    root: &Path,
    principal_id: &str,
    display_name: Option<&str>,
    affiliations: Option<Vec<String>>,
    mode: V2AmendMode,
    if_generation: Option<&str>,
) -> Result<V2AmendmentReceipt> {
    let path = PathBuf::from("assurance/v2/principals.yaml");
    let bytes = read_regular(root, &path)?;
    let mut registry: serde_yaml::Value = parse_yaml(&path, &bytes)?;
    let changed =
        append_principal_version(&mut registry, principal_id, display_name, affiliations)?;
    if !changed {
        return no_op_receipt(
            root,
            "attribution",
            "metadata-fast",
            Vec::new(),
            if_generation,
        );
    }
    let consumers = principal_consumers(root, principal_id)?;
    require_consumer_lifecycles(root, &consumers, "attribution")?;
    let replacements = BTreeMap::from([(path, render_yaml(&registry)?)]);
    prepare_or_apply_successor(
        root,
        "attribution",
        "metadata-fast",
        consumers,
        replacements,
        BTreeMap::new(),
        mode,
        if_generation,
    )
}

/// Plans or applies a typed principal-record version.
///
/// # Errors
///
/// Returns an error for invalid authority input or transaction failure.
pub fn amend_principal(
    root: &Path,
    request_bytes: &[u8],
    mode: V2AmendMode,
) -> Result<V2AmendmentReceipt> {
    amend_principal_at_generation(root, request_bytes, mode, None)
}

/// Applies a principal version only at the supplied generation.
///
/// # Errors
///
/// Returns an error for a stale generation or invalid request.
pub fn amend_principal_at_generation(
    root: &Path,
    request_bytes: &[u8],
    mode: V2AmendMode,
    if_generation: Option<&str>,
) -> Result<V2AmendmentReceipt> {
    let request = parse_principal_request(request_bytes)?;
    let path = PathBuf::from("assurance/v2/principals.yaml");
    let bytes = read_regular(root, &path)?;
    let mut registry: serde_yaml::Value = parse_yaml(&path, &bytes)?;
    let governance_changed = request.roles.is_some()
        || request.identity_authority.is_some()
        || request.identity_reference.is_some();
    let changed = append_principal_request_version(&mut registry, &request)?;
    let consumers = principal_consumers(root, &request.principal_id)?;
    require_consumer_lifecycles(root, &consumers, "principal")?;
    if !changed {
        return no_op_receipt(
            root,
            "principal",
            principal_impact_class(governance_changed),
            consumers,
            if_generation,
        );
    }
    let mut replacements = BTreeMap::from([(path, render_yaml(&registry)?)]);
    let event_updates = prepare_principal_governance_events(
        root,
        &request,
        &consumers,
        &mut replacements,
        governance_changed,
    )?;
    prepare_or_apply_successor(
        root,
        "principal",
        principal_impact_class(governance_changed),
        consumers,
        replacements,
        event_updates,
        mode,
        if_generation,
    )
}

fn parse_principal_request(request_bytes: &[u8]) -> Result<V2PrincipalRequest> {
    let request: V2PrincipalRequest = parse_yaml("principal-request.yaml", request_bytes)?;
    if request.schema_version != 1 {
        return Err(AssuranceError::Usage(
            "principal request schema_version must be 1".to_owned(),
        ));
    }
    validate_role_request_attestation(&request.attestation)?;
    Ok(request)
}

fn principal_impact_class(governance_changed: bool) -> &'static str {
    if governance_changed {
        "governance-focused"
    } else {
        "metadata-fast"
    }
}

fn prepare_principal_governance_events(
    root: &Path,
    request: &V2PrincipalRequest,
    consumers: &[String],
    replacements: &mut BTreeMap<PathBuf, Vec<u8>>,
    governance_changed: bool,
) -> Result<BTreeMap<String, EventUpdate>> {
    let mut updates = BTreeMap::new();
    if !governance_changed {
        return Ok(updates);
    }
    for report_id in consumers {
        let report_path = report_path(root, report_id)?;
        let report: serde_yaml::Value =
            parse_yaml(&report_path, &read_regular(root, &report_path)?)?;
        let provisional =
            candidate_review_lock(root, report_id, &report, replacements, Vec::new())?;
        let event = principal_version_event(request, report_id, &provisional)?;
        let event_path = PathBuf::from(format!(
            "assurance/v2/reports/{report_id}/review-events/{}.json",
            event.event_id
        ));
        replacements.insert(event_path, event.render()?);
        updates.insert(
            report_id.clone(),
            EventUpdate {
                event_ids: vec![event.event_id],
                invalidate_existing: true,
            },
        );
    }
    Ok(updates)
}

fn principal_version_event(
    request: &V2PrincipalRequest,
    report_id: &str,
    provisional: &ReviewLock,
) -> Result<ReviewEvent> {
    ReviewEvent::new(
        "principal_version".to_owned(),
        report_id.to_owned(),
        request.principal_id.clone(),
        "authority_or_eligibility_updated".to_owned(),
        request.attestation.statement.clone(),
        request.attestation.recorded_on.clone(),
        BTreeMap::from([
            (
                "review_governance_root".to_owned(),
                provisional.review_governance_root.clone(),
            ),
            (
                "content_review_subject_root".to_owned(),
                provisional.content_review_subject_root.clone(),
            ),
        ]),
        Vec::new(),
        request.attestation.authority.clone(),
        BTreeMap::new(),
    )
}

/// Plans or applies a typed report-local role assignment.
///
/// # Errors
///
/// Returns an error for ineligible principals or invalid role authority.
pub fn amend_role(
    root: &Path,
    report_id: &str,
    request_bytes: &[u8],
    mode: V2AmendMode,
) -> Result<V2AmendmentReceipt> {
    amend_role_at_generation(root, report_id, request_bytes, mode, None)
}

/// Applies a role assignment only at the supplied generation.
///
/// # Errors
///
/// Returns an error for a stale generation or invalid role request.
pub fn amend_role_at_generation(
    root: &Path,
    report_id: &str,
    request_bytes: &[u8],
    mode: V2AmendMode,
    if_generation: Option<&str>,
) -> Result<V2AmendmentReceipt> {
    let request: V2RoleRequest = parse_yaml("role-request.yaml", request_bytes)?;
    validate_role_request(&request)?;
    let report_path = report_path(root, report_id)?;
    let report_bytes = read_regular(root, &report_path)?;
    let mut report: serde_yaml::Value = parse_yaml(&report_path, &report_bytes)?;
    require_report_lifecycle(&report, report_id, "role assignment")?;
    require_role_eligible_principal(root, &request.principal_id, &request.assignments)?;
    let changed = apply_role_assignments(&mut report, &request)?;
    if !changed {
        return no_op_receipt(
            root,
            "role_assignment",
            "governance-focused",
            vec![report_id.to_owned()],
            if_generation,
        );
    }
    let provisional =
        candidate_review_lock(root, report_id, &report, &BTreeMap::new(), Vec::new())?;
    let event = ReviewEvent::new(
        "role_assignment".to_owned(),
        report_id.to_owned(),
        request.principal_id.clone(),
        "roles_assigned".to_owned(),
        request.attestation.statement.clone(),
        request.attestation.recorded_on.clone(),
        BTreeMap::from([
            ("science_root".to_owned(), provisional.science_root),
            (
                "communication_root".to_owned(),
                provisional.communication_root,
            ),
            (
                "content_review_subject_root".to_owned(),
                provisional.content_review_subject_root,
            ),
        ]),
        Vec::new(),
        request.attestation.authority.clone(),
        BTreeMap::new(),
    )?;
    let event_path = PathBuf::from(format!(
        "assurance/v2/reports/{report_id}/review-events/{}.json",
        event.event_id
    ));
    let replacements = BTreeMap::from([
        (report_path, render_yaml(&report)?),
        (event_path, event.render()?),
    ]);
    prepare_or_apply_successor(
        root,
        "role_assignment",
        "governance-focused",
        vec![report_id.to_owned()],
        replacements,
        BTreeMap::from([(
            report_id.to_owned(),
            EventUpdate {
                event_ids: vec![event.event_id],
                invalidate_existing: true,
            },
        )]),
        mode,
        if_generation,
    )
}

/// Plans or applies the enumerated American-English normalization.
///
/// # Errors
///
/// Returns an error for protected-region changes or transaction failure.
pub fn amend_normalize(
    root: &Path,
    report_id: &str,
    language: &str,
    mode: V2AmendMode,
) -> Result<V2AmendmentReceipt> {
    amend_normalize_at_generation(root, report_id, language, mode, None)
}

/// Normalizes only when the supplied generation remains current.
///
/// # Errors
///
/// Returns an error for a stale generation or invalid normalization.
pub fn amend_normalize_at_generation(
    root: &Path,
    report_id: &str,
    language: &str,
    mode: V2AmendMode,
    if_generation: Option<&str>,
) -> Result<V2AmendmentReceipt> {
    if language != "en-US" {
        return Err(AssuranceError::Usage(
            "assurance normalization supports only --language en-US".to_owned(),
        ));
    }
    let path = report_path(root, report_id)?;
    let report_bytes = read_regular(root, &path)?;
    let report: serde_yaml::Value = parse_yaml(&path, &report_bytes)?;
    let lifecycle = report
        .get("lifecycle")
        .and_then(serde_yaml::Value::as_str)
        .ok_or_else(|| AssuranceError::Invalid("report lifecycle is missing".to_owned()))?;
    if !matches!(lifecycle, "DRAFT" | "IN_REVIEW") {
        return Err(AssuranceError::Invalid(format!(
            "normalization is not allowed for lifecycle {lifecycle}"
        )));
    }
    if lifecycle == "DRAFT"
        && report
            .get("agent_assistance")
            .and_then(|value| value.get("review_entry_authorized"))
            .and_then(serde_yaml::Value::as_bool)
            == Some(true)
    {
        return Err(AssuranceError::Invalid(
            "DRAFT normalization refuses an already authorized review entry".to_owned(),
        ));
    }
    let converter = std::ffi::OsString::from("uk2us");
    let mut replacements = BTreeMap::new();
    for field in ["manuscript", "supplement"] {
        let content_path = report
            .get(field)
            .and_then(|value| value.get("path"))
            .and_then(serde_yaml::Value::as_str)
            .ok_or_else(|| AssuranceError::Invalid(format!("report {field} path is missing")))?;
        let content_path = PathBuf::from(content_path);
        let current = read_regular(root, &content_path)?;
        let normalized = super::normalization::run_converter(&converter, &current)?;
        if super::normalization::run_converter(&converter, &normalized)? != normalized {
            return Err(AssuranceError::Invalid(
                "uk2us normalization is not idempotent".to_owned(),
            ));
        }
        verify_normalization_boundaries(&current, &normalized, &content_path)?;
        if normalized != current {
            replacements.insert(content_path, normalized);
        }
    }
    if replacements.is_empty() {
        return no_op_receipt(
            root,
            "normalization",
            "editorial-fast",
            vec![report_id.to_owned()],
            if_generation,
        );
    }
    let reset = BTreeMap::from([(
        report_id.to_owned(),
        EventUpdate {
            event_ids: Vec::new(),
            invalidate_existing: true,
        },
    )]);
    prepare_or_apply_successor(
        root,
        "normalization",
        "editorial-fast",
        vec![report_id.to_owned()],
        replacements,
        reset,
        mode,
        if_generation,
    )
}

fn verify_normalization_boundaries(current: &[u8], normalized: &[u8], path: &Path) -> Result<()> {
    let before = std::str::from_utf8(current).map_err(|error| AssuranceError::Parse {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;
    let after = std::str::from_utf8(normalized).map_err(|error| AssuranceError::Parse {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;
    let before_skeleton = before
        .chars()
        .filter(|character| !character.is_alphabetic())
        .collect::<String>();
    let after_skeleton = after
        .chars()
        .filter(|character| !character.is_alphabetic())
        .collect::<String>();
    if before_skeleton != after_skeleton {
        return Err(AssuranceError::Invalid(format!(
            "normalization changed protected punctuation, identifiers, paths, numbers, equations, or structure in {}",
            path.display()
        )));
    }
    let mut fenced = false;
    for (before_line, after_line) in before.lines().zip(after.lines()) {
        let trimmed = before_line.trim_start();
        let fence_boundary = trimmed.starts_with("```") || trimmed.starts_with("~~~");
        let whole_line_protected = fenced
            || fence_boundary
            || trimmed.starts_with('[') && trimmed.contains("]:")
            || before_line.contains('|')
            || before_line.contains("{{");
        if before_line != after_line
            && (whole_line_protected
                || protected_inline_spans(before_line) != protected_inline_spans(after_line))
        {
            return Err(AssuranceError::Invalid(format!(
                "normalization changed a protected Markdown region in {}",
                path.display()
            )));
        }
        if fence_boundary {
            fenced = !fenced;
        }
    }
    Ok(())
}

fn protected_inline_spans(line: &str) -> Vec<&str> {
    let mut spans = delimited_spans(line, "`", "`");
    spans.extend(delimited_spans(line, "$", "$"));
    spans.extend(delimited_spans(line, "](", ")"));
    spans.extend(delimited_spans(line, "<", ">"));
    spans
}

fn delimited_spans<'a>(line: &'a str, open: &str, close: &str) -> Vec<&'a str> {
    let mut spans = Vec::new();
    let mut offset = 0;
    while let Some(start) = line[offset..].find(open).map(|index| offset + index) {
        let content_start = start + open.len();
        let Some(end) = line[content_start..]
            .find(close)
            .map(|index| content_start + index + close.len())
        else {
            break;
        };
        spans.push(&line[start..end]);
        offset = end;
    }
    spans
}

/// Plans or records one typed immutable lifecycle event.
///
/// # Errors
///
/// Returns an error for invalid authority, ordering, or lifecycle state.
pub fn amend_lifecycle(
    root: &Path,
    report_id: &str,
    request_bytes: &[u8],
    mode: V2AmendMode,
) -> Result<V2AmendmentReceipt> {
    amend_lifecycle_at_generation(root, report_id, request_bytes, mode, None)
}

/// Records a lifecycle event only at the supplied generation.
///
/// # Errors
///
/// Returns an error for a stale generation or invalid event.
pub fn amend_lifecycle_at_generation(
    root: &Path,
    report_id: &str,
    request_bytes: &[u8],
    mode: V2AmendMode,
    if_generation: Option<&str>,
) -> Result<V2AmendmentReceipt> {
    let request: V2LifecycleRequest = parse_yaml("lifecycle-request.yaml", request_bytes)?;
    validate_lifecycle_request(&request)?;
    let path = report_path(root, report_id)?;
    let bytes = read_regular(root, &path)?;
    let mut report: serde_yaml::Value = parse_yaml(&path, &bytes)?;
    let current = load_review_lock(root, report_id)?;
    validate_lifecycle_transition(&request.event_type, &current.lifecycle)?;
    require_lifecycle_principal(root, &request.principal_id, &request.event_type)?;
    let mut replacements = BTreeMap::new();
    if request.event_type == "review_entry" {
        apply_review_entry(&mut report, &request)?;
        replacements.insert(path.clone(), render_yaml(&report)?);
    } else if matches!(request.event_type.as_str(), "withdrawal" | "supersession") {
        apply_terminal_lifecycle(&mut report, &request)?;
        replacements.insert(path.clone(), render_yaml(&report)?);
    }
    let provisional = candidate_review_lock(
        root,
        report_id,
        &report,
        &replacements,
        current.event_ids.clone(),
    )?;
    let bound_roots = lifecycle_bound_roots(&request.event_type, &provisional)?;
    let event = ReviewEvent::new(
        request.event_type.clone(),
        report_id.to_owned(),
        request.principal_id.clone(),
        request.decision.clone(),
        request.rationale.clone(),
        request.recorded_on.clone(),
        bound_roots,
        request.predecessor_event_ids.clone(),
        request.authority_source.clone(),
        lifecycle_event_inputs(&request),
    )?;
    if request.event_type == "steward_approval" {
        apply_steward_approval(&mut report)?;
        replacements.insert(path.clone(), render_yaml(&report)?);
    } else if request.event_type == "release_transfer" {
        apply_release_transfer(&mut report, &request)?;
        replacements.insert(path.clone(), render_yaml(&report)?);
    }
    let event_path = PathBuf::from(format!(
        "assurance/v2/reports/{report_id}/review-events/{}.json",
        event.event_id
    ));
    if root.join(&event_path).exists() {
        return no_op_receipt(
            root,
            "lifecycle",
            "governance-focused",
            vec![report_id.to_owned()],
            if_generation,
        );
    }
    replacements.insert(event_path, event.render()?);
    prepare_or_apply_successor(
        root,
        "lifecycle",
        "governance-focused",
        vec![report_id.to_owned()],
        replacements,
        BTreeMap::from([(
            report_id.to_owned(),
            EventUpdate {
                event_ids: vec![event.event_id],
                invalidate_existing: matches!(
                    request.event_type.as_str(),
                    "withdrawal" | "supersession"
                ),
            },
        )]),
        mode,
        if_generation,
    )
}

fn validate_lifecycle_request(request: &V2LifecycleRequest) -> Result<()> {
    if request.schema_version != 1 {
        return Err(AssuranceError::Usage(
            "lifecycle request schema_version must be 1".to_owned(),
        ));
    }
    validate_lifecycle_event_type(&request.event_type)?;
    for (value, label) in [
        (&request.principal_id, "principal ID"),
        (&request.decision, "decision"),
        (&request.rationale, "rationale"),
        (&request.authority_source, "authority source"),
    ] {
        require_text(value, label)?;
    }
    super::validate_date(&request.recorded_on, "lifecycle event date")?;
    validate_lifecycle_decision(&request.event_type, &request.decision)?;
    validate_lifecycle_payload(request)
}

fn validate_lifecycle_event_type(event_type: &str) -> Result<()> {
    if matches!(
        event_type,
        "review_entry"
            | "finding"
            | "disposition"
            | "scientific_approval"
            | "reproduction_approval"
            | "steward_approval"
            | "withdrawal"
            | "supersession"
            | "release_transfer"
    ) {
        Ok(())
    } else {
        Err(AssuranceError::Usage(format!(
            "unsupported lifecycle event type '{event_type}'"
        )))
    }
}

fn validate_lifecycle_decision(event_type: &str, decision: &str) -> Result<()> {
    let expected_decision = match event_type {
        "review_entry" => "entered_pending_review",
        "finding" => "open",
        "disposition" => {
            if !matches!(decision, "resolved_and_verified" | "rejected") {
                return Err(AssuranceError::Usage(
                    "disposition decision must be resolved_and_verified or rejected".to_owned(),
                ));
            }
            decision
        }
        "scientific_approval"
        | "reproduction_approval"
        | "steward_approval"
        | "release_transfer" => "approved",
        "withdrawal" => "withdrawn",
        "supersession" => "superseded",
        _ => "",
    };
    if decision != expected_decision {
        return Err(AssuranceError::Usage(format!(
            "{event_type} decision must be '{expected_decision}'",
        )));
    }
    Ok(())
}

fn validate_lifecycle_payload(request: &V2LifecycleRequest) -> Result<()> {
    match request.event_type.as_str() {
        "review_entry" => {
            require_optional_text(request.review_charge.as_ref(), "review charge")?;
            require_optional_text(request.build_maintainer_id.as_ref(), "build maintainer ID")?;
            require_optional_text(
                request.independence_assessment.as_ref(),
                "independence assessment",
            )?;
            require_optional_text(
                request.scientific_approver_id.as_ref(),
                "scientific approver ID",
            )?;
            if request.material_producer_ids.is_empty() {
                return Err(AssuranceError::Usage(
                    "review entry requires material_producer_ids".to_owned(),
                ));
            }
        }
        "scientific_approval" | "reproduction_approval" | "steward_approval" => {
            require_optional_text(request.competence_basis.as_ref(), "competence basis")?;
            require_optional_text(
                request.independence_attestation.as_ref(),
                "independence attestation",
            )?;
        }
        "release_transfer" => validate_release_transfer_request(request)?,
        "supersession" => {
            require_optional_text(
                request.superseding_report_id.as_ref(),
                "superseding report ID",
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn validate_lifecycle_transition(event_type: &str, lifecycle: &str) -> Result<()> {
    let allowed = match event_type {
        "review_entry" => lifecycle == "DRAFT",
        "finding"
        | "disposition"
        | "scientific_approval"
        | "reproduction_approval"
        | "steward_approval" => lifecycle == "IN_REVIEW",
        "withdrawal" | "supersession" => matches!(lifecycle, "IN_REVIEW" | "APPROVED"),
        "release_transfer" => lifecycle == "APPROVED",
        _ => false,
    };
    if allowed {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "{event_type} is not allowed for lifecycle {lifecycle}"
        )))
    }
}

fn require_consumer_lifecycles(root: &Path, reports: &[String], operation: &str) -> Result<()> {
    for report_id in reports {
        let path = report_path(root, report_id)?;
        let report: serde_yaml::Value = parse_yaml(&path, &read_regular(root, &path)?)?;
        require_report_lifecycle(&report, report_id, operation)?;
    }
    Ok(())
}

fn require_report_lifecycle(
    report: &serde_yaml::Value,
    report_id: &str,
    operation: &str,
) -> Result<()> {
    let lifecycle = report
        .get("lifecycle")
        .and_then(serde_yaml::Value::as_str)
        .ok_or_else(|| AssuranceError::Invalid("report lifecycle is missing".to_owned()))?;
    if matches!(lifecycle, "DRAFT" | "IN_REVIEW") {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "{operation} refuses report '{report_id}' in lifecycle {lifecycle}"
        )))
    }
}

fn validate_release_transfer_request(request: &V2LifecycleRequest) -> Result<()> {
    for (value, label) in [
        (&request.target_release_commit, "target release commit"),
        (
            &request.target_release_configuration,
            "target release configuration",
        ),
        (&request.prior_realization, "prior realization"),
        (&request.candidate_realization, "candidate realization"),
        (&request.impact_assessment, "impact assessment"),
        (
            &request.reproduction_disposition,
            "reproduction disposition",
        ),
        (&request.assurance_steward_id, "assurance steward ID"),
        (&request.publication_date, "publication date"),
        (&request.public_path, "public path"),
    ] {
        require_optional_text(value.as_ref(), label)?;
    }
    super::validate_date(
        request.publication_date.as_deref().ok_or_else(|| {
            AssuranceError::Usage("release transfer requires publication date".to_owned())
        })?,
        "publication date",
    )?;
    super::validate_relative(Path::new(request.public_path.as_deref().ok_or_else(
        || AssuranceError::Usage("release transfer requires public path".to_owned()),
    )?))?;
    if request.semantic_differences.is_empty()
        || request
            .semantic_differences
            .iter()
            .any(|value| value.trim().is_empty())
    {
        return Err(AssuranceError::Usage(
            "release transfer requires nonempty semantic_differences".to_owned(),
        ));
    }
    Ok(())
}

fn lifecycle_event_inputs(request: &V2LifecycleRequest) -> BTreeMap<String, String> {
    let mut inputs = BTreeMap::new();
    for (field, value) in [
        ("review_charge", request.review_charge.as_ref()),
        ("build_maintainer_id", request.build_maintainer_id.as_ref()),
        (
            "independence_assessment",
            request.independence_assessment.as_ref(),
        ),
        (
            "scientific_approver_id",
            request.scientific_approver_id.as_ref(),
        ),
        ("competence_basis", request.competence_basis.as_ref()),
        (
            "independence_attestation",
            request.independence_attestation.as_ref(),
        ),
        (
            "target_release_commit",
            request.target_release_commit.as_ref(),
        ),
        (
            "target_release_configuration",
            request.target_release_configuration.as_ref(),
        ),
        ("prior_realization", request.prior_realization.as_ref()),
        (
            "candidate_realization",
            request.candidate_realization.as_ref(),
        ),
        ("impact_assessment", request.impact_assessment.as_ref()),
        (
            "reproduction_disposition",
            request.reproduction_disposition.as_ref(),
        ),
        (
            "assurance_steward_id",
            request.assurance_steward_id.as_ref(),
        ),
        ("publication_date", request.publication_date.as_ref()),
        ("public_path", request.public_path.as_ref()),
        (
            "superseding_report_id",
            request.superseding_report_id.as_ref(),
        ),
    ] {
        if let Some(value) = value {
            inputs.insert(field.to_owned(), value.clone());
        }
    }
    if !request.material_producer_ids.is_empty() {
        inputs.insert(
            "material_producer_ids".to_owned(),
            request.material_producer_ids.join(","),
        );
    }
    if !request.semantic_differences.is_empty() {
        inputs.insert(
            "semantic_differences".to_owned(),
            request.semantic_differences.join("\n"),
        );
    }
    inputs
}

fn apply_review_entry(report: &mut serde_yaml::Value, request: &V2LifecycleRequest) -> Result<()> {
    let mapping = report
        .as_mapping_mut()
        .ok_or_else(|| AssuranceError::Invalid("report is not an object".to_owned()))?;
    mapping.insert(
        yaml_key("lifecycle"),
        serde_yaml::Value::String("IN_REVIEW".to_owned()),
    );
    let review = mapping
        .get_mut(yaml_key("review"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report review is missing".to_owned()))?;
    set_yaml_string(review, "state", "IN_REVIEW");
    set_yaml_string(review, "decision", "pending");
    set_yaml_string(
        review,
        "review_charge",
        request.review_charge.as_deref().ok_or_else(|| {
            AssuranceError::Usage("review entry requires review_charge".to_owned())
        })?,
    );
    set_yaml_string(
        review,
        "build_maintainer_id",
        request.build_maintainer_id.as_deref().ok_or_else(|| {
            AssuranceError::Usage("review entry requires build_maintainer_id".to_owned())
        })?,
    );
    review.insert(
        yaml_key("material_producer_ids"),
        serde_yaml::Value::Sequence(
            request
                .material_producer_ids
                .iter()
                .cloned()
                .map(serde_yaml::Value::String)
                .collect(),
        ),
    );
    set_yaml_string(
        review,
        "independence_assessment",
        request.independence_assessment.as_deref().ok_or_else(|| {
            AssuranceError::Usage("review entry requires independence_assessment".to_owned())
        })?,
    );
    let authorship = mapping
        .get_mut(yaml_key("authorship"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report authorship is missing".to_owned()))?;
    set_yaml_string(authorship, "human_report_lead", &request.principal_id);
    set_yaml_string(
        authorship,
        "scientific_approver",
        request.scientific_approver_id.as_deref().ok_or_else(|| {
            AssuranceError::Usage("review entry requires scientific_approver_id".to_owned())
        })?,
    );
    set_yaml_string(authorship, "accountability_state", "assigned");
    let assistance = mapping
        .get_mut(yaml_key("agent_assistance"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("agent assistance is missing".to_owned()))?;
    assistance.insert(
        yaml_key("provenance_complete"),
        serde_yaml::Value::Bool(true),
    );
    assistance.insert(
        yaml_key("review_entry_authorized"),
        serde_yaml::Value::Bool(true),
    );
    set_yaml_string(
        assistance,
        "human_disposition",
        &format!(
            "{} — {} ({}, {}).",
            request.decision, request.rationale, request.principal_id, request.recorded_on
        ),
    );
    Ok(())
}

fn apply_steward_approval(report: &mut serde_yaml::Value) -> Result<()> {
    let mapping = report
        .as_mapping_mut()
        .ok_or_else(|| AssuranceError::Invalid("report is not an object".to_owned()))?;
    mapping.insert(
        yaml_key("lifecycle"),
        serde_yaml::Value::String("APPROVED".to_owned()),
    );
    let review = mapping
        .get_mut(yaml_key("review"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report review is missing".to_owned()))?;
    set_yaml_string(review, "state", "APPROVED");
    set_yaml_string(review, "decision", "approved");
    Ok(())
}

fn apply_terminal_lifecycle(
    report: &mut serde_yaml::Value,
    request: &V2LifecycleRequest,
) -> Result<()> {
    let terminal = match request.event_type.as_str() {
        "withdrawal" => "WITHDRAWN",
        "supersession" => "SUPERSEDED",
        _ => {
            return Err(AssuranceError::Invalid(
                "terminal lifecycle writer requires withdrawal or supersession".to_owned(),
            ));
        }
    };
    let mapping = report
        .as_mapping_mut()
        .ok_or_else(|| AssuranceError::Invalid("report is not an object".to_owned()))?;
    mapping.insert(
        yaml_key("lifecycle"),
        serde_yaml::Value::String(terminal.to_owned()),
    );
    let review = mapping
        .get_mut(yaml_key("review"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report review is missing".to_owned()))?;
    set_yaml_string(review, "state", terminal);
    set_yaml_string(review, "decision", &request.decision);

    let publication = mapping
        .get_mut(yaml_key("publication"))
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report publication is missing".to_owned()))?;
    set_yaml_string(publication, "state", "DRAFT");
    for field in [
        "target_release_commit",
        "target_release_configuration",
        "prior_realization",
        "candidate_realization",
        "impact_assessment",
        "reproduction_disposition",
        "release_owner_id",
        "assurance_steward_id",
        "publication_date",
        "public_path",
    ] {
        publication.insert(yaml_key(field), serde_yaml::Value::Null);
    }
    publication.insert(
        yaml_key("semantic_differences"),
        serde_yaml::Value::Sequence(Vec::new()),
    );
    publication.insert(
        yaml_key("withdrawn"),
        serde_yaml::Value::Bool(request.event_type == "withdrawal"),
    );
    publication.insert(
        yaml_key("supersedes"),
        request
            .superseding_report_id
            .as_ref()
            .map_or(serde_yaml::Value::Null, |value| {
                serde_yaml::Value::String(value.clone())
            }),
    );
    Ok(())
}

fn apply_release_transfer(
    report: &mut serde_yaml::Value,
    request: &V2LifecycleRequest,
) -> Result<()> {
    let publication = report
        .get_mut("publication")
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report publication is missing".to_owned()))?;
    set_yaml_string(publication, "state", "APPROVED");
    for (field, value) in [
        ("target_release_commit", &request.target_release_commit),
        (
            "target_release_configuration",
            &request.target_release_configuration,
        ),
        ("prior_realization", &request.prior_realization),
        ("candidate_realization", &request.candidate_realization),
        ("impact_assessment", &request.impact_assessment),
        (
            "reproduction_disposition",
            &request.reproduction_disposition,
        ),
        ("release_owner_id", &Some(request.principal_id.clone())),
        ("assurance_steward_id", &request.assurance_steward_id),
        ("publication_date", &request.publication_date),
        ("public_path", &request.public_path),
    ] {
        set_yaml_string(
            publication,
            field,
            value.as_deref().ok_or_else(|| {
                AssuranceError::Usage(format!("release transfer requires {field}"))
            })?,
        );
    }
    publication.insert(
        yaml_key("semantic_differences"),
        serde_yaml::Value::Sequence(
            request
                .semantic_differences
                .iter()
                .cloned()
                .map(serde_yaml::Value::String)
                .collect(),
        ),
    );
    Ok(())
}

fn lifecycle_bound_roots(event_type: &str, lock: &ReviewLock) -> Result<BTreeMap<String, String>> {
    let mut roots = BTreeMap::new();
    match event_type {
        "review_entry" | "finding" | "disposition" | "withdrawal" | "supersession" => {
            roots.insert(
                "content_review_subject_root".to_owned(),
                lock.content_review_subject_root.clone(),
            );
            if let Some(ledger) = &lock.finding_ledger_root {
                roots.insert("finding_ledger_root".to_owned(), ledger.clone());
            }
        }
        "scientific_approval" | "reproduction_approval" => {
            roots.insert("science_root".to_owned(), lock.science_root.clone());
            roots.insert(
                "communication_root".to_owned(),
                lock.communication_root.clone(),
            );
            roots.insert(
                "review_governance_root".to_owned(),
                lock.review_governance_root.clone(),
            );
            roots.insert(
                "content_review_subject_root".to_owned(),
                lock.content_review_subject_root.clone(),
            );
            roots.insert(
                "finding_ledger_root".to_owned(),
                lock.finding_ledger_root.clone().ok_or_else(|| {
                    AssuranceError::Invalid("approval requires a finding ledger".to_owned())
                })?,
            );
            if event_type == "reproduction_approval" {
                roots.insert(
                    "preapproval_realization_root".to_owned(),
                    lock.preapproval_realization_root.clone(),
                );
            }
        }
        "steward_approval" => {
            roots.insert("science_root".to_owned(), lock.science_root.clone());
            roots.insert(
                "communication_root".to_owned(),
                lock.communication_root.clone(),
            );
            roots.insert(
                "review_governance_root".to_owned(),
                lock.review_governance_root.clone(),
            );
            roots.insert(
                "content_review_subject_root".to_owned(),
                lock.content_review_subject_root.clone(),
            );
            roots.insert(
                "finding_ledger_root".to_owned(),
                lock.finding_ledger_root.clone().ok_or_else(|| {
                    AssuranceError::Invalid("steward approval requires a finding ledger".to_owned())
                })?,
            );
            roots.insert(
                "preapproval_realization_root".to_owned(),
                lock.preapproval_realization_root.clone(),
            );
            roots.insert("attribution_root".to_owned(), lock.attribution_root.clone());
            roots.insert(
                "pre_steward_approval_root".to_owned(),
                lock.pre_steward_approval_root.clone().ok_or_else(|| {
                    AssuranceError::Invalid(
                        "steward approval requires predecessor approvals".to_owned(),
                    )
                })?,
            );
        }
        "release_transfer" => {
            roots.insert(
                "approval_lock_root".to_owned(),
                lock.approval_lock_root.clone().ok_or_else(|| {
                    AssuranceError::Invalid("release transfer requires approval lock".to_owned())
                })?,
            );
            roots.insert(
                "realization_root".to_owned(),
                lock.realization_root.clone().ok_or_else(|| {
                    AssuranceError::Invalid("release transfer requires realization".to_owned())
                })?,
            );
        }
        _ => {}
    }
    Ok(roots)
}

fn require_lifecycle_principal(root: &Path, principal_id: &str, event_type: &str) -> Result<()> {
    let path = PathBuf::from("assurance/v2/principals.yaml");
    let bytes = read_regular(root, &path)?;
    let registry: serde_yaml::Value = parse_yaml(&path, &bytes)?;
    let principal = current_principal(&registry, principal_id)?;
    let kind = principal
        .get("kind")
        .and_then(serde_yaml::Value::as_str)
        .ok_or_else(|| AssuranceError::Invalid("principal kind is missing".to_owned()))?;
    if matches!(
        event_type,
        "scientific_approval" | "reproduction_approval" | "steward_approval" | "release_transfer"
    ) && kind != "human"
    {
        return Err(AssuranceError::Invalid(
            "approval and release-transfer authority requires a human principal".to_owned(),
        ));
    }
    let required = match event_type {
        "review_entry" | "withdrawal" | "supersession" => "report_lead",
        "finding" | "disposition" => "reviewer",
        "scientific_approval" => "scientific_approver",
        "reproduction_approval" => "reproduction_approver",
        "steward_approval" => "assurance_steward",
        "release_transfer" => "release_owner",
        _ => return Ok(()),
    };
    let eligible = principal
        .get("roles")
        .and_then(serde_yaml::Value::as_sequence)
        .is_some_and(|roles| roles.iter().any(|role| role.as_str() == Some(required)));
    if eligible {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "principal '{principal_id}' is not eligible for lifecycle role '{required}'"
        )))
    }
}

fn current_principal<'a>(
    registry: &'a serde_yaml::Value,
    principal_id: &str,
) -> Result<&'a serde_yaml::Value> {
    registry
        .get("principals")
        .and_then(serde_yaml::Value::as_sequence)
        .and_then(|principals| {
            principals
                .iter()
                .filter(|principal| {
                    principal.get("id").and_then(serde_yaml::Value::as_str) == Some(principal_id)
                })
                .max_by_key(|principal| {
                    principal
                        .get("record_version")
                        .and_then(serde_yaml::Value::as_u64)
                        .unwrap_or(0)
                })
        })
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown principal '{principal_id}'")))
}

pub(super) struct MigrationCandidate {
    pub(super) replacements: BTreeMap<PathBuf, Vec<u8>>,
    pub(super) receipt: V2AmendmentReceipt,
    pub(super) receipt_path: PathBuf,
    pub(super) allowed_preexisting_drift: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct EventUpdate {
    event_ids: Vec<String>,
    invalidate_existing: bool,
}

/// Verifies the complete anchored generation transition chain.
///
/// # Errors
///
/// Returns an error for a broken receipt chain or mismatched base anchor.
pub fn verify_generation(root: &Path, base_ref: &str) -> Result<String> {
    require_text(base_ref, "generation base ref")?;
    let lock = IdentityLock::load(root)?;
    lock.verify_files(root)?;
    verify_generation_anchor(root, &lock, base_ref)?;
    let transitions = load_generation_transitions(root)?;
    verify_current_predecessor(&lock, &transitions)?;
    let count = follow_generation_chain(&lock.generation_id, &transitions)?;
    Ok(format!(
        "generation verification: PASS\nbase_ref: {base_ref}\ncurrent_generation: {}\ntransitions: {count}\n",
        lock.generation_id,
    ))
}

fn verify_generation_anchor(root: &Path, lock: &IdentityLock, base_ref: &str) -> Result<()> {
    let genesis = lock.genesis.as_ref().ok_or_else(|| {
        AssuranceError::Invalid("generated identity lacks migration genesis".to_owned())
    })?;
    if genesis.base_ref != base_ref {
        return Err(AssuranceError::Drift(
            "generation base ref does not match retained migration genesis".to_owned(),
        ));
    }
    let catalog = git_object_bytes(root, base_ref, Path::new(CATALOG_PATH))?;
    if sha256_bytes(&catalog) != genesis.legacy_catalog_sha256 {
        return Err(AssuranceError::Drift(
            "retained migration genesis does not match the frozen Git catalog".to_owned(),
        ));
    }
    Ok(())
}

fn load_generation_transitions(root: &Path) -> Result<BTreeMap<String, Option<String>>> {
    let transactions = root.join("assurance/v2/transactions");
    let mut transitions = BTreeMap::new();
    for entry in std::fs::read_dir(&transactions)
        .map_err(|error| AssuranceError::io(&transactions, error))?
    {
        let entry = entry.map_err(|error| AssuranceError::io(&transactions, error))?;
        if !entry
            .file_type()
            .map_err(|error| AssuranceError::io(entry.path(), error))?
            .is_file()
        {
            continue;
        }
        let receipt = load_generation_receipt(&entry)?;
        if transitions
            .insert(receipt.new_generation_id, receipt.old_generation_id)
            .is_some()
        {
            return Err(AssuranceError::Invalid(
                "transaction receipts contain duplicate successor generations".to_owned(),
            ));
        }
    }
    Ok(transitions)
}

fn load_generation_receipt(entry: &std::fs::DirEntry) -> Result<V2AmendmentReceipt> {
    let path = entry.path();
    let bytes = std::fs::read(&path).map_err(|error| AssuranceError::io(&path, error))?;
    let receipt: V2AmendmentReceipt =
        serde_json::from_slice(&bytes).map_err(|error| AssuranceError::Parse {
            path: path.clone(),
            message: error.to_string(),
        })?;
    let canonical = receipt_bytes(&receipt)?;
    if canonical != bytes {
        return Err(AssuranceError::Invalid(format!(
            "transaction receipt is not canonical JSON: {}",
            path.display()
        )));
    }
    let expected_name = format!("{}.json", receipt_id(&canonical));
    if entry.file_name() != std::ffi::OsStr::new(&expected_name) {
        return Err(AssuranceError::Invalid(format!(
            "transaction receipt filename does not match its content: {}",
            path.display()
        )));
    }
    if !receipt.changed {
        return Err(AssuranceError::Invalid(
            "transaction archive cannot contain no-op receipts".to_owned(),
        ));
    }
    Ok(receipt)
}

fn verify_current_predecessor(
    lock: &IdentityLock,
    transitions: &BTreeMap<String, Option<String>>,
) -> Result<()> {
    if transitions.get(&lock.generation_id) != Some(&lock.previous_generation_id) {
        return Err(AssuranceError::Invalid(
            "current transition does not match identity-lock predecessor".to_owned(),
        ));
    }
    Ok(())
}

fn follow_generation_chain(
    current: &str,
    transitions: &BTreeMap<String, Option<String>>,
) -> Result<usize> {
    let mut cursor = Some(current.to_owned());
    let mut visited = BTreeSet::new();
    while let Some(generation) = cursor {
        if !visited.insert(generation.clone()) {
            return Err(AssuranceError::Invalid(
                "transaction generation chain contains a cycle".to_owned(),
            ));
        }
        cursor = transitions.get(&generation).cloned().ok_or_else(|| {
            AssuranceError::Invalid(format!(
                "transaction receipts do not explain generation '{generation}'"
            ))
        })?;
    }
    Ok(visited.len())
}

/// Inspects or resolves a typed amendment recovery state.
///
/// # Errors
///
/// Returns an error when a requested generation cannot be verified safely.
pub fn recover_amendment(root: &Path, action: V2RecoveryAction) -> Result<String> {
    let root = root
        .canonicalize()
        .map_err(|error| AssuranceError::io(root, error))?;
    let transaction = ConfinedDirectory::open_ambient(&root, false)?;
    transaction.lock_exclusive(&root)?;
    let pending = transaction.directory_exists(Path::new(NEXT_ROOT))?;
    if action == V2RecoveryAction::Inspect {
        return inspect_recovery(&root, pending);
    }
    if !pending {
        return Err(AssuranceError::Invalid(
            "no amendment recovery state exists".to_owned(),
        ));
    }
    match action {
        V2RecoveryAction::FinishCleanup => finish_recovery_cleanup(&root, &transaction),
        V2RecoveryAction::RestoreOld => restore_recovery_generation(&root, &transaction),
        V2RecoveryAction::Inspect => Err(AssuranceError::Invalid(
            "inspect cannot reach mutation dispatch".to_owned(),
        )),
    }
}

fn inspect_recovery(root: &Path, pending: bool) -> Result<String> {
    let active = IdentityLock::load(root)?;
    active.verify_files(root)?;
    let held = if pending {
        super::transaction::verify_generation_tree(root, Path::new(NEXT_ROOT))?
    } else {
        "not_applicable".to_owned()
    };
    Ok(format!(
        "amendment recovery: PASS\npending_cleanup: {pending}\nactive_generation: {}\nheld_generation: {held}\n",
        active.generation_id,
    ))
}

fn finish_recovery_cleanup(root: &Path, transaction: &ConfinedDirectory) -> Result<String> {
    let generation = validated_active_generation(root)?;
    remove_recovery_tree(transaction)?;
    Ok(format!(
        "amendment recovery: PASS\naction: finish_cleanup\nactive_generation: {generation}\n"
    ))
}

fn validated_active_generation(root: &Path) -> Result<String> {
    let active = IdentityLock::load(root)?;
    active.verify_files(root)?;
    super::V2Repository::open(root)?.validate_all()?;
    Ok(active.generation_id)
}

fn remove_recovery_tree(transaction: &ConfinedDirectory) -> Result<()> {
    transaction.remove_directory_if_exists(Path::new(NEXT_ROOT))?;
    transaction.sync_parent()
}

fn restore_recovery_generation(root: &Path, transaction: &ConfinedDirectory) -> Result<String> {
    let held_generation = super::transaction::verify_generation_tree(root, Path::new(NEXT_ROOT))?;
    exchange_and_remove_recovery_tree(transaction)?;
    Ok(format!(
        "amendment recovery: PASS\naction: restore_old\nactive_generation: {held_generation}\n"
    ))
}

fn exchange_and_remove_recovery_tree(transaction: &ConfinedDirectory) -> Result<()> {
    transaction.exchange(Path::new(V2_ROOT), Path::new(NEXT_ROOT))?;
    transaction.sync_parent()?;
    transaction.remove_directory_if_exists(Path::new(NEXT_ROOT))?;
    transaction.sync_parent()
}

#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
fn prepare_or_apply_successor(
    root: &Path,
    operation: &str,
    impact_class: &str,
    affected_reports: Vec<String>,
    replacements: BTreeMap<PathBuf, Vec<u8>>,
    event_updates: BTreeMap<String, EventUpdate>,
    mode: V2AmendMode,
    if_generation: Option<&str>,
) -> Result<V2AmendmentReceipt> {
    prepare_or_apply_successor_with_drift(
        root,
        operation,
        impact_class,
        affected_reports,
        replacements,
        event_updates,
        mode,
        if_generation,
        BTreeSet::new(),
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
fn prepare_or_apply_successor_with_drift(
    root: &Path,
    operation: &str,
    impact_class: &str,
    affected_reports: Vec<String>,
    mut replacements: BTreeMap<PathBuf, Vec<u8>>,
    event_updates: BTreeMap<String, EventUpdate>,
    mode: V2AmendMode,
    if_generation: Option<&str>,
    allowed_preexisting_drift: BTreeSet<String>,
) -> Result<V2AmendmentReceipt> {
    let previous = IdentityLock::load(root)?;
    let exceptions = allowed_preexisting_drift
        .iter()
        .map(String::as_str)
        .collect();
    previous.verify_files_except(root, &exceptions)?;
    require_expected_generation(&previous, if_generation)?;
    regenerate_review_locks(root, &mut replacements, &event_updates, &affected_reports)?;
    let mut sources = previous.sources.clone();
    for (path, bytes) in &replacements {
        let text = path.to_str().ok_or_else(|| {
            AssuranceError::Invalid(format!("amendment path is not UTF-8: {}", path.display()))
        })?;
        if !text.ends_with("/review.lock.json")
            && text != IDENTITY_LOCK_PATH
            && !text.starts_with("assurance/v2/transactions/")
        {
            sources.insert(text.to_owned(), sha256_bytes(bytes));
        }
    }
    let mut review_locks = previous.review_locks.clone();
    review_locks.extend(replacement_review_lock_digests(&replacements));
    let next = IdentityLock::successor(&previous, sources, review_locks)?;
    replacements.insert(PathBuf::from(IDENTITY_LOCK_PATH), next.render()?);
    let mut paths = replacements
        .keys()
        .map(|path| path.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    paths.sort();
    let invalidated = event_updates
        .iter()
        .filter(|(_, update)| update.invalidate_existing)
        .map(|(report, _)| report)
        .map(|report| format!("prior_review_events:{report}"))
        .collect::<Vec<_>>();
    let receipt = V2AmendmentReceipt {
        schema_version: 1,
        operation: operation.to_owned(),
        impact_class: impact_class.to_owned(),
        changed: true,
        old_generation_id: Some(previous.generation_id),
        new_generation_id: next.generation_id,
        affected_reports,
        affected_paths: paths,
        invalidated_authority: invalidated,
        gate_ids: vec![gate_id(impact_class).to_owned()],
        gate_argv: gate_argv(impact_class),
    };
    let mut candidate = with_receipt(replacements, receipt)?;
    candidate.allowed_preexisting_drift = allowed_preexisting_drift;
    if mode == V2AmendMode::Check {
        Ok(candidate.receipt)
    } else {
        apply_candidate(root, candidate)
    }
}

fn regenerate_review_locks(
    root: &Path,
    replacements: &mut BTreeMap<PathBuf, Vec<u8>>,
    event_updates: &BTreeMap<String, EventUpdate>,
    affected_reports: &[String],
) -> Result<()> {
    let catalog_path = PathBuf::from(CATALOG_PATH);
    let catalog_bytes = candidate_bytes(root, replacements, &catalog_path)?;
    let catalog: serde_yaml::Value = parse_yaml(&catalog_path, &catalog_bytes)?;
    let principal_path = PathBuf::from("assurance/v2/principals.yaml");
    let principal_bytes = candidate_bytes(root, replacements, &principal_path)?;
    let principals: serde_yaml::Value = parse_yaml(&principal_path, &principal_bytes)?;
    for (report_id, path) in report_paths(&catalog)? {
        if !affected_reports.contains(&report_id) {
            continue;
        }
        let report_bytes = candidate_bytes(root, replacements, &path)?;
        let report: serde_yaml::Value = parse_yaml(&path, &report_bytes)?;
        let existing = load_review_lock(root, &report_id)?;
        let (events, invalidated) = match event_updates.get(&report_id) {
            Some(update) if update.invalidate_existing => {
                let mut invalidated = existing.invalidated_event_ids.clone();
                invalidated.extend(existing.event_ids);
                invalidated.sort();
                invalidated.dedup();
                (update.event_ids.clone(), invalidated)
            }
            Some(update) => {
                let mut events = existing.event_ids;
                events.extend(update.event_ids.clone());
                events.sort();
                events.dedup();
                (events, existing.invalidated_event_ids)
            }
            None => (existing.event_ids, existing.invalidated_event_ids),
        };
        let mut lock = calculate_review_lock(
            root,
            &report_id,
            &report,
            &principals,
            replacements,
            existing.legacy_subject_root,
            events,
        )?;
        lock.invalidated_event_ids = invalidated;
        replacements.insert(
            PathBuf::from(format!("assurance/v2/reports/{report_id}/review.lock.json")),
            lock.render()?,
        );
    }
    Ok(())
}

fn candidate_review_lock(
    root: &Path,
    report_id: &str,
    report: &serde_yaml::Value,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
    event_ids: Vec<String>,
) -> Result<ReviewLock> {
    let principal_path = PathBuf::from("assurance/v2/principals.yaml");
    let principal_bytes = candidate_bytes(root, replacements, &principal_path)?;
    let principals: serde_yaml::Value = parse_yaml(&principal_path, &principal_bytes)?;
    let existing = load_review_lock(root, report_id)?;
    calculate_review_lock(
        root,
        report_id,
        report,
        &principals,
        replacements,
        existing.legacy_subject_root,
        event_ids,
    )
}

fn load_review_lock(root: &Path, report_id: &str) -> Result<ReviewLock> {
    let path = PathBuf::from(format!("assurance/v2/reports/{report_id}/review.lock.json"));
    let bytes = read_regular(root, &path)?;
    serde_json::from_slice(&bytes).map_err(|error| AssuranceError::Parse {
        path,
        message: error.to_string(),
    })
}

fn candidate_bytes(
    root: &Path,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
    path: &Path,
) -> Result<Vec<u8>> {
    match replacements.get(path) {
        Some(bytes) => Ok(bytes.clone()),
        None => read_regular(root, path),
    }
}

fn apply_candidate(root: &Path, candidate: MigrationCandidate) -> Result<V2AmendmentReceipt> {
    super::transaction::apply_candidate(root, candidate)
}

fn report_paths(catalog: &serde_yaml::Value) -> Result<Vec<(String, PathBuf)>> {
    let reports = catalog
        .get("reports")
        .and_then(serde_yaml::Value::as_sequence)
        .ok_or_else(|| AssuranceError::Invalid("catalog reports are missing".to_owned()))?;
    reports
        .iter()
        .map(|report| {
            let id = report
                .get("id")
                .and_then(serde_yaml::Value::as_str)
                .ok_or_else(|| {
                    AssuranceError::Invalid("catalog report ID is missing".to_owned())
                })?;
            let path = report
                .get("manifest_path")
                .and_then(serde_yaml::Value::as_str)
                .ok_or_else(|| {
                    AssuranceError::Invalid("catalog report manifest path is missing".to_owned())
                })?;
            Ok((id.to_owned(), PathBuf::from(path)))
        })
        .collect()
}

fn replacement_review_lock_digests(
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
) -> BTreeMap<String, String> {
    replacements
        .iter()
        .filter_map(|(path, bytes)| {
            let text = path.to_str()?;
            text.ends_with("/review.lock.json")
                .then(|| (text.to_owned(), sha256_bytes(bytes)))
        })
        .collect()
}

fn with_receipt(
    mut replacements: BTreeMap<PathBuf, Vec<u8>>,
    receipt: V2AmendmentReceipt,
) -> Result<MigrationCandidate> {
    let receipt_bytes = receipt_bytes(&receipt)?;
    let receipt_path = PathBuf::from(format!(
        "assurance/v2/transactions/{}.json",
        receipt_id(&receipt_bytes)
    ));
    replacements.insert(receipt_path.clone(), receipt_bytes);
    Ok(MigrationCandidate {
        replacements,
        receipt,
        receipt_path,
        allowed_preexisting_drift: BTreeSet::new(),
    })
}

fn git_object_bytes(root: &Path, base_ref: &str, path: &Path) -> Result<Vec<u8>> {
    let object = format!("{base_ref}:{}", path.display());
    let output = std::process::Command::new("git")
        .arg("show")
        .arg(&object)
        .current_dir(root)
        .output()
        .map_err(|error| AssuranceError::io("git", error))?;
    if !output.status.success() {
        return Err(AssuranceError::Invalid(format!(
            "cannot read frozen migration source '{object}'"
        )));
    }
    Ok(output.stdout)
}

fn append_principal_version(
    registry: &mut serde_yaml::Value,
    principal_id: &str,
    display_name: Option<&str>,
    affiliations: Option<Vec<String>>,
) -> Result<bool> {
    require_text(principal_id, "principal ID")?;
    let principals = registry
        .get_mut("principals")
        .and_then(serde_yaml::Value::as_sequence_mut)
        .ok_or_else(|| AssuranceError::Invalid("principal records are missing".to_owned()))?;
    let current = principals
        .iter()
        .filter(|principal| {
            principal.get("id").and_then(serde_yaml::Value::as_str) == Some(principal_id)
        })
        .max_by_key(|principal| {
            principal
                .get("record_version")
                .and_then(serde_yaml::Value::as_u64)
                .unwrap_or(0)
        })
        .cloned()
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown principal '{principal_id}'")))?;
    let current_display = current
        .get("display_name")
        .and_then(serde_yaml::Value::as_str)
        .ok_or_else(|| AssuranceError::Invalid("principal display name is missing".to_owned()))?
        .to_owned();
    let current_affiliations = current
        .get("affiliations")
        .and_then(serde_yaml::Value::as_sequence)
        .ok_or_else(|| AssuranceError::Invalid("principal affiliations are missing".to_owned()))?;
    let desired_display = display_name.map_or_else(|| current_display.clone(), ToOwned::to_owned);
    require_text(&desired_display, "principal display name")?;
    let desired_affiliations = affiliations.unwrap_or_else(|| {
        current_affiliations
            .iter()
            .filter_map(serde_yaml::Value::as_str)
            .map(ToOwned::to_owned)
            .collect()
    });
    if desired_affiliations
        .iter()
        .any(|affiliation| affiliation.trim().is_empty())
    {
        return Err(AssuranceError::Usage(
            "principal affiliations cannot be empty".to_owned(),
        ));
    }
    let current_affiliations = current_affiliations
        .iter()
        .filter_map(serde_yaml::Value::as_str)
        .collect::<Vec<_>>();
    if desired_display == current_display
        && desired_affiliations
            .iter()
            .map(String::as_str)
            .eq(current_affiliations)
    {
        return Ok(false);
    }
    let version = current
        .get("record_version")
        .and_then(serde_yaml::Value::as_u64)
        .ok_or_else(|| AssuranceError::Invalid("principal record_version is missing".to_owned()))?;
    let mut next = current;
    let mapping = next
        .as_mapping_mut()
        .ok_or_else(|| AssuranceError::Invalid("principal record is not an object".to_owned()))?;
    mapping.insert(
        yaml_key("record_version"),
        serde_yaml::Value::Number((version + 1).into()),
    );
    mapping.insert(
        yaml_key("supersedes"),
        serde_yaml::Value::String(format!("{principal_id}@{version}")),
    );
    mapping.insert(
        yaml_key("display_name"),
        serde_yaml::Value::String(desired_display),
    );
    mapping.insert(
        yaml_key("affiliations"),
        serde_yaml::Value::Sequence(
            desired_affiliations
                .into_iter()
                .map(serde_yaml::Value::String)
                .collect(),
        ),
    );
    principals.push(next);
    Ok(true)
}

fn append_principal_request_version(
    registry: &mut serde_yaml::Value,
    request: &V2PrincipalRequest,
) -> Result<bool> {
    require_text(&request.principal_id, "principal ID")?;
    let current = current_principal(registry, &request.principal_id)?.clone();
    let mut next = current.clone();
    let mapping = next
        .as_mapping_mut()
        .ok_or_else(|| AssuranceError::Invalid("principal record is not an object".to_owned()))?;
    if let Some(display_name) = &request.display_name {
        require_text(display_name, "principal display name")?;
        mapping.insert(
            yaml_key("display_name"),
            serde_yaml::Value::String(display_name.clone()),
        );
    }
    if let Some(affiliations) = &request.affiliations {
        if affiliations.iter().any(|value| value.trim().is_empty()) {
            return Err(AssuranceError::Usage(
                "principal affiliations cannot be empty".to_owned(),
            ));
        }
        mapping.insert(
            yaml_key("affiliations"),
            serde_yaml::Value::Sequence(
                affiliations
                    .iter()
                    .cloned()
                    .map(serde_yaml::Value::String)
                    .collect(),
            ),
        );
    }
    if let Some(roles) = &request.roles {
        if roles.is_empty() || roles.iter().any(|value| value.trim().is_empty()) {
            return Err(AssuranceError::Usage(
                "principal roles require nonempty values".to_owned(),
            ));
        }
        let mut roles = roles.clone();
        roles.sort();
        roles.dedup();
        mapping.insert(
            yaml_key("roles"),
            serde_yaml::Value::Sequence(roles.into_iter().map(serde_yaml::Value::String).collect()),
        );
    }
    for (field, value) in [
        ("identity_authority", request.identity_authority.as_ref()),
        ("identity_reference", request.identity_reference.as_ref()),
    ] {
        if let Some(value) = value {
            require_text(value, field)?;
            mapping.insert(yaml_key(field), serde_yaml::Value::String(value.clone()));
        }
    }
    for field in ["record_version", "supersedes"] {
        mapping.remove(yaml_key(field));
    }
    let mut comparison = current.clone();
    let comparison_mapping = comparison
        .as_mapping_mut()
        .ok_or_else(|| AssuranceError::Invalid("principal record is not an object".to_owned()))?;
    for field in ["record_version", "supersedes"] {
        comparison_mapping.remove(yaml_key(field));
    }
    if next == comparison {
        return Ok(false);
    }
    let version = current
        .get("record_version")
        .and_then(serde_yaml::Value::as_u64)
        .ok_or_else(|| AssuranceError::Invalid("principal record_version is missing".to_owned()))?;
    let mapping = next
        .as_mapping_mut()
        .ok_or_else(|| AssuranceError::Invalid("principal record is not an object".to_owned()))?;
    mapping.insert(
        yaml_key("record_version"),
        serde_yaml::Value::Number((version + 1).into()),
    );
    mapping.insert(
        yaml_key("supersedes"),
        serde_yaml::Value::String(format!("{}@{version}", request.principal_id)),
    );
    registry
        .get_mut("principals")
        .and_then(serde_yaml::Value::as_sequence_mut)
        .ok_or_else(|| AssuranceError::Invalid("principal records are missing".to_owned()))?
        .push(next);
    Ok(true)
}

fn validate_role_request_attestation(attestation: &V2Attestation) -> Result<()> {
    require_text(&attestation.authority, "attestation authority")?;
    require_text(&attestation.statement, "attestation statement")?;
    super::validate_date(&attestation.recorded_on, "attestation date")
}

fn principal_consumers(root: &Path, principal_id: &str) -> Result<Vec<String>> {
    let catalog_bytes = read_regular(root, Path::new(CATALOG_PATH))?;
    let catalog: serde_yaml::Value = parse_yaml(CATALOG_PATH, &catalog_bytes)?;
    let mut consumers = Vec::new();
    for (report_id, path) in report_paths(&catalog)? {
        let bytes = read_regular(root, &path)?;
        let report: serde_yaml::Value = parse_yaml(&path, &bytes)?;
        if report_uses_principal(&report, principal_id) {
            consumers.push(report_id);
        }
    }
    if consumers.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "principal '{principal_id}' has no report consumers"
        )));
    }
    Ok(consumers)
}

fn report_uses_principal(report: &serde_yaml::Value, principal_id: &str) -> bool {
    let scalar = [
        ("authorship", "human_report_lead"),
        ("authorship", "scientific_approver"),
        ("review", "build_maintainer_id"),
    ]
    .into_iter()
    .any(|(section, field)| {
        report
            .get(section)
            .and_then(|value| value.get(field))
            .and_then(serde_yaml::Value::as_str)
            == Some(principal_id)
    });
    scalar
        || report
            .get("review")
            .and_then(|value| value.get("material_producer_ids"))
            .and_then(serde_yaml::Value::as_sequence)
            .is_some_and(|values| {
                values
                    .iter()
                    .any(|value| value.as_str() == Some(principal_id))
            })
}

fn report_path(root: &Path, report_id: &str) -> Result<PathBuf> {
    let bytes = read_regular(root, Path::new(CATALOG_PATH))?;
    let catalog: serde_yaml::Value = parse_yaml(CATALOG_PATH, &bytes)?;
    report_paths(&catalog)?
        .into_iter()
        .find(|(id, _)| id == report_id)
        .map(|(_, path)| path)
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown v2 report ID '{report_id}'")))
}

fn validate_role_request(request: &V2RoleRequest) -> Result<()> {
    if request.schema_version != 1 {
        return Err(AssuranceError::Usage(
            "role request schema_version must be 1".to_owned(),
        ));
    }
    if request.operation != "role_assignment" {
        return Err(AssuranceError::Usage(
            "role request operation must be role_assignment".to_owned(),
        ));
    }
    if !request.assignments.report_lead
        && !request.assignments.material_producer
        && !request.assignments.build_maintainer
    {
        return Err(AssuranceError::Usage(
            "role request must assign at least one role".to_owned(),
        ));
    }
    validate_role_request_attestation(&request.attestation)
}

fn apply_role_assignments(report: &mut serde_yaml::Value, request: &V2RoleRequest) -> Result<bool> {
    let mut changed = false;
    if request.assignments.report_lead {
        let authorship = report
            .get_mut("authorship")
            .and_then(serde_yaml::Value::as_mapping_mut)
            .ok_or_else(|| AssuranceError::Invalid("report authorship is missing".to_owned()))?;
        changed |= set_yaml_string(authorship, "human_report_lead", &request.principal_id);
        changed |= set_yaml_string(authorship, "accountability_state", "assigned");
    }
    let review = report
        .get_mut("review")
        .and_then(serde_yaml::Value::as_mapping_mut)
        .ok_or_else(|| AssuranceError::Invalid("report review is missing".to_owned()))?;
    if request.assignments.build_maintainer {
        changed |= set_yaml_string(review, "build_maintainer_id", &request.principal_id);
    }
    if request.assignments.material_producer {
        let key = yaml_key("material_producer_ids");
        let producers = review
            .get_mut(&key)
            .and_then(serde_yaml::Value::as_sequence_mut)
            .ok_or_else(|| {
                AssuranceError::Invalid("material_producer_ids are missing".to_owned())
            })?;
        if !producers
            .iter()
            .any(|value| value.as_str() == Some(&request.principal_id))
        {
            producers.push(serde_yaml::Value::String(request.principal_id.clone()));
            producers.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
            changed = true;
        }
    }
    Ok(changed)
}

fn require_role_eligible_principal(
    root: &Path,
    principal_id: &str,
    assignments: &V2RoleAssignments,
) -> Result<()> {
    let path = PathBuf::from("assurance/v2/principals.yaml");
    let bytes = read_regular(root, &path)?;
    let registry: serde_yaml::Value = parse_yaml(&path, &bytes)?;
    let current = registry
        .get("principals")
        .and_then(serde_yaml::Value::as_sequence)
        .and_then(|principals| {
            principals
                .iter()
                .filter(|principal| {
                    principal.get("id").and_then(serde_yaml::Value::as_str) == Some(principal_id)
                })
                .max_by_key(|principal| {
                    principal
                        .get("record_version")
                        .and_then(serde_yaml::Value::as_u64)
                        .unwrap_or(0)
                })
        })
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown principal '{principal_id}'")))?;
    let roles = current
        .get("roles")
        .and_then(serde_yaml::Value::as_sequence)
        .ok_or_else(|| AssuranceError::Invalid("principal roles are missing".to_owned()))?;
    for (required, selected) in [
        ("report_lead", assignments.report_lead),
        ("material_producer", assignments.material_producer),
        ("build_maintainer", assignments.build_maintainer),
    ] {
        if selected && !roles.iter().any(|role| role.as_str() == Some(required)) {
            return Err(AssuranceError::Invalid(format!(
                "principal '{principal_id}' is not eligible for role '{required}'"
            )));
        }
    }
    Ok(())
}

fn no_op_receipt(
    root: &Path,
    operation: &str,
    impact_class: &str,
    affected_reports: Vec<String>,
    if_generation: Option<&str>,
) -> Result<V2AmendmentReceipt> {
    let lock = IdentityLock::load(root)?;
    lock.verify_files(root)?;
    require_expected_generation(&lock, if_generation)?;
    Ok(V2AmendmentReceipt {
        schema_version: 1,
        operation: operation.to_owned(),
        impact_class: impact_class.to_owned(),
        changed: false,
        old_generation_id: Some(lock.generation_id.clone()),
        new_generation_id: lock.generation_id,
        affected_reports,
        affected_paths: Vec::new(),
        invalidated_authority: Vec::new(),
        gate_ids: Vec::new(),
        gate_argv: Vec::new(),
    })
}

fn require_expected_generation(lock: &IdentityLock, expected: Option<&str>) -> Result<()> {
    if let Some(expected) = expected
        && lock.generation_id != expected
    {
        return Err(AssuranceError::Drift(format!(
            "amendment compare-and-swap rejected stale generation '{expected}'"
        )));
    }
    Ok(())
}
