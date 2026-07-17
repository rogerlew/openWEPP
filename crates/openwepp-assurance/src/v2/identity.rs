use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::confined::{read_regular_confined, validate_relative};
use crate::{AssuranceError, Result, sha256_bytes};

pub(super) const IDENTITY_LOCK_PATH: &str = "assurance/v2/identity.lock.json";
pub(super) const IDENTITY_ALGORITHM: &str = "openwepp-assurance-generated-identity-v2";
const TOOL_IDENTITY: &str = "openwepp-assurance:0.1.0";

const SCIENCE_REPORT_FIELDS: &[&str] = &[
    "id",
    "version",
    "trust_domain",
    "dependencies",
    "units",
    "claims",
    "methods",
    "results",
    "value_bindings",
    "tables",
    "figures",
    "references",
    "research_objects",
];
const COMMUNICATION_REPORT_FIELDS: &[&str] =
    &["title", "reader_metadata", "manuscript", "supplement"];
const GOVERNANCE_REPORT_FIELDS: &[&str] = &[
    "schema_version",
    "contract_version",
    "id",
    "version",
    "owner",
    "trust_domain",
    "fixture_only",
    "authorship",
    "agent_assistance",
    "review",
];
const EVENT_GOVERNED_REPORT_FIELDS: &[&str] = &["lifecycle", "publication"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct IdentityLock {
    pub(super) machine_owned: bool,
    pub(super) format_version: u32,
    pub(super) identity_algorithm_version: String,
    pub(super) tool_identity: String,
    pub(super) genesis: Option<MigrationGenesis>,
    pub(super) previous_generation_id: Option<String>,
    pub(super) sources: BTreeMap<String, String>,
    pub(super) review_locks: BTreeMap<String, String>,
    pub(super) generation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MigrationGenesis {
    pub(super) base_ref: String,
    pub(super) legacy_catalog_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReviewLock {
    pub(super) machine_owned: bool,
    pub(super) format_version: u32,
    pub(super) identity_algorithm_version: String,
    pub(super) report_id: String,
    pub(super) lifecycle: String,
    pub(super) legacy_subject_root: Option<String>,
    pub(super) science_root: String,
    pub(super) communication_root: String,
    pub(super) attribution_root: String,
    pub(super) review_governance_root: String,
    pub(super) content_review_subject_root: String,
    pub(super) finding_ledger_root: Option<String>,
    pub(super) preapproval_realization_root: String,
    pub(super) pre_steward_approval_root: Option<String>,
    pub(super) approval_lock_root: Option<String>,
    pub(super) realization_root: Option<String>,
    pub(super) release_transfer_root: Option<String>,
    pub(super) event_ids: Vec<String>,
    pub(super) invalidated_event_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReviewEvent {
    pub(super) event_id: String,
    pub(super) schema_version: u32,
    pub(super) event_type: String,
    pub(super) report_id: String,
    pub(super) principal_id: String,
    pub(super) decision: String,
    pub(super) rationale: String,
    pub(super) recorded_on: String,
    pub(super) bound_roots: BTreeMap<String, String>,
    pub(super) predecessor_event_ids: Vec<String>,
    pub(super) authority_source: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub(super) inputs: BTreeMap<String, String>,
}

#[derive(Serialize)]
struct GenerationPayload<'a> {
    machine_owned: bool,
    format_version: u32,
    identity_algorithm_version: &'a str,
    tool_identity: &'a str,
    genesis: &'a Option<MigrationGenesis>,
    previous_generation_id: &'a Option<String>,
    sources: &'a BTreeMap<String, String>,
    review_locks: &'a BTreeMap<String, String>,
}

impl IdentityLock {
    pub(super) fn successor(
        previous: &Self,
        sources: BTreeMap<String, String>,
        review_locks: BTreeMap<String, String>,
    ) -> Result<Self> {
        let mut lock = Self {
            machine_owned: true,
            format_version: 1,
            identity_algorithm_version: IDENTITY_ALGORITHM.to_owned(),
            tool_identity: TOOL_IDENTITY.to_owned(),
            genesis: previous.genesis.clone(),
            previous_generation_id: Some(previous.generation_id.clone()),
            sources,
            review_locks,
            generation_id: String::new(),
        };
        lock.generation_id = lock.calculate_generation_id()?;
        Ok(lock)
    }

    pub(super) fn load(root: &Path) -> Result<Self> {
        let path = Path::new(IDENTITY_LOCK_PATH);
        let bytes = read_regular_confined(root, path)?;
        Self::parse(path, &bytes)
    }

    pub(super) fn parse(path: &Path, bytes: &[u8]) -> Result<Self> {
        let lock: Self = serde_json::from_slice(bytes).map_err(|error| AssuranceError::Parse {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;
        lock.validate_structure()?;
        if lock.render()? != bytes {
            return Err(AssuranceError::Invalid(
                "generated identity lock is not canonical JSON".to_owned(),
            ));
        }
        Ok(lock)
    }

    pub(super) fn render(&self) -> Result<Vec<u8>> {
        let mut bytes = serde_json::to_vec_pretty(self).map_err(|error| {
            AssuranceError::Invalid(format!("identity lock serialization failed: {error}"))
        })?;
        bytes.push(b'\n');
        Ok(bytes)
    }

    pub(super) fn verify_files(&self, root: &Path) -> Result<()> {
        self.verify_files_except(root, &BTreeSet::new())
    }

    pub(super) fn verify_files_except(
        &self,
        root: &Path,
        exceptions: &BTreeSet<&str>,
    ) -> Result<()> {
        let mut paths = BTreeSet::new();
        for (path, expected) in self.sources.iter().chain(&self.review_locks) {
            validate_lock_member(path, expected, &mut paths)?;
            if exceptions.contains(path.as_str()) {
                continue;
            }
            let relative = Path::new(path);
            let observed = sha256_bytes(&read_regular_confined(root, relative)?);
            if observed != *expected {
                return Err(AssuranceError::Drift(format!(
                    "generated identity member changed: {path}"
                )));
            }
        }
        Ok(())
    }

    pub(super) fn digest_for(&self, path: &Path) -> Result<&str> {
        let key = path.to_str().ok_or_else(|| {
            AssuranceError::Invalid(format!("identity path is not UTF-8: {}", path.display()))
        })?;
        self.sources
            .get(key)
            .or_else(|| self.review_locks.get(key))
            .map(String::as_str)
            .ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "generated identity lock does not admit '{}'",
                    path.display()
                ))
            })
    }

    pub(super) fn hydrate_yaml(&self, value: &mut serde_yaml::Value) -> Result<()> {
        hydrate_value(value, self)
    }

    fn calculate_generation_id(&self) -> Result<String> {
        let payload = GenerationPayload {
            machine_owned: self.machine_owned,
            format_version: self.format_version,
            identity_algorithm_version: &self.identity_algorithm_version,
            tool_identity: &self.tool_identity,
            genesis: &self.genesis,
            previous_generation_id: &self.previous_generation_id,
            sources: &self.sources,
            review_locks: &self.review_locks,
        };
        let bytes = serde_json::to_vec(&payload).map_err(|error| {
            AssuranceError::Invalid(format!("generation payload serialization failed: {error}"))
        })?;
        let mut material = self.identity_algorithm_version.as_bytes().to_vec();
        material.push(0);
        material.extend_from_slice(&bytes);
        Ok(sha256_bytes(&material))
    }

    fn validate_structure(&self) -> Result<()> {
        self.validate_header()?;
        self.validate_genesis()?;
        if let Some(previous) = &self.previous_generation_id {
            validate_digest(previous, "previous generation ID")?;
        }
        if self.sources.is_empty() {
            return Err(AssuranceError::Invalid(
                "generated identity lock has no admitted sources".to_owned(),
            ));
        }
        validate_digest(&self.generation_id, "generation ID")?;
        if self.calculate_generation_id()? != self.generation_id {
            return Err(AssuranceError::Drift(
                "generated identity lock generation ID is stale".to_owned(),
            ));
        }
        Ok(())
    }

    fn validate_header(&self) -> Result<()> {
        if !self.machine_owned
            || self.format_version != 1
            || self.identity_algorithm_version != IDENTITY_ALGORITHM
            || self.tool_identity != TOOL_IDENTITY
        {
            return Err(AssuranceError::Invalid(
                "generated identity lock header is invalid".to_owned(),
            ));
        }
        Ok(())
    }

    fn validate_genesis(&self) -> Result<()> {
        match &self.genesis {
            Some(genesis) => {
                require_text(&genesis.base_ref, "migration base ref")?;
                validate_digest(&genesis.legacy_catalog_sha256, "legacy catalog digest")?;
                Ok(())
            }
            None => Err(AssuranceError::Invalid(
                "identity lock requires a retained migration genesis anchor".to_owned(),
            )),
        }
    }
}

impl ReviewLock {
    pub(super) fn render(&self) -> Result<Vec<u8>> {
        canonical_pretty_json(self, "review lock")
    }

    pub(super) fn parse(path: &Path, bytes: &[u8]) -> Result<Self> {
        let lock: Self = serde_json::from_slice(bytes).map_err(|error| AssuranceError::Parse {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;
        if lock.render()? != bytes {
            return Err(AssuranceError::Invalid(format!(
                "review lock is not canonical JSON: {}",
                path.display()
            )));
        }
        Ok(lock)
    }
}

pub(super) fn load_review_state(
    root: &Path,
    identity: &IdentityLock,
    report_id: &str,
) -> Result<(ReviewLock, Vec<ReviewEvent>)> {
    let path = PathBuf::from(format!("assurance/v2/reports/{report_id}/review.lock.json"));
    let bytes = read_regular_confined(root, &path)?;
    if sha256_bytes(&bytes) != identity.digest_for(&path)? {
        return Err(AssuranceError::Drift(format!(
            "generated review lock changed: {}",
            path.display()
        )));
    }
    let lock = ReviewLock::parse(&path, &bytes)?;
    if lock.report_id != report_id {
        return Err(AssuranceError::Invalid(format!(
            "review lock does not match report '{report_id}'"
        )));
    }
    let events = load_review_events(root, report_id, &lock.event_ids, &BTreeMap::new())?;
    Ok((lock, events))
}

pub(super) fn verify_review_lock_current(
    root: &Path,
    identity: &IdentityLock,
    report_id: &str,
    report: &serde_yaml::Value,
    principals: &serde_yaml::Value,
) -> Result<()> {
    let (current, _) = load_review_state(root, identity, report_id)?;
    let mut calculated = calculate_review_lock(
        root,
        report_id,
        report,
        principals,
        &BTreeMap::new(),
        current.legacy_subject_root.clone(),
        current.event_ids.clone(),
    )?;
    calculated
        .invalidated_event_ids
        .clone_from(&current.invalidated_event_ids);
    if calculated == current {
        Ok(())
    } else {
        Err(AssuranceError::Drift(format!(
            "generated review lock is stale for report '{report_id}'"
        )))
    }
}

impl ReviewEvent {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        event_type: String,
        report_id: String,
        principal_id: String,
        decision: String,
        rationale: String,
        recorded_on: String,
        bound_roots: BTreeMap<String, String>,
        predecessor_event_ids: Vec<String>,
        authority_source: String,
        inputs: BTreeMap<String, String>,
    ) -> Result<Self> {
        let mut event = Self {
            event_id: String::new(),
            schema_version: 1,
            event_type,
            report_id,
            principal_id,
            decision,
            rationale,
            recorded_on,
            bound_roots,
            predecessor_event_ids,
            authority_source,
            inputs,
        };
        let mut value = serde_json::to_value(&event).map_err(|error| {
            AssuranceError::Invalid(format!("review event serialization failed: {error}"))
        })?;
        value
            .as_object_mut()
            .ok_or_else(|| AssuranceError::Invalid("review event is not an object".to_owned()))?
            .remove("event_id");
        event.event_id = digest_serialized("openwepp-assurance-review-event-v1", &value)?;
        Ok(event)
    }

    pub(super) fn render(&self) -> Result<Vec<u8>> {
        canonical_pretty_json(self, "review event")
    }

    fn validate(&self, report_id: &str) -> Result<()> {
        if self.schema_version != 1 || self.report_id != report_id {
            return Err(AssuranceError::Invalid(
                "review event header does not match its report".to_owned(),
            ));
        }
        let rebuilt = Self::new(
            self.event_type.clone(),
            self.report_id.clone(),
            self.principal_id.clone(),
            self.decision.clone(),
            self.rationale.clone(),
            self.recorded_on.clone(),
            self.bound_roots.clone(),
            self.predecessor_event_ids.clone(),
            self.authority_source.clone(),
            self.inputs.clone(),
        )?;
        if rebuilt.event_id != self.event_id {
            return Err(AssuranceError::Drift(
                "review event identity is stale".to_owned(),
            ));
        }
        for (name, value) in [
            ("event_type", self.event_type.as_str()),
            ("principal_id", self.principal_id.as_str()),
            ("decision", self.decision.as_str()),
            ("rationale", self.rationale.as_str()),
            ("recorded_on", self.recorded_on.as_str()),
            ("authority_source", self.authority_source.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(AssuranceError::Invalid(format!(
                    "review event {name} cannot be empty"
                )));
            }
        }
        validate_event_decision(&self.event_type, &self.decision)?;
        if self
            .predecessor_event_ids
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            != self.predecessor_event_ids.len()
        {
            return Err(AssuranceError::Invalid(
                "review event repeats a predecessor event ID".to_owned(),
            ));
        }
        Ok(())
    }
}

fn validate_event_decision(event_type: &str, decision: &str) -> Result<()> {
    let valid = match event_type {
        "review_entry" => decision == "entered_pending_review",
        "finding" => decision == "open",
        "disposition" => matches!(decision, "resolved_and_verified" | "rejected"),
        "scientific_approval" | "reproduction_approval" | "steward_approval" => {
            decision == "approved"
        }
        "withdrawal" => decision == "withdrawn",
        "supersession" => decision == "superseded",
        "release_transfer" => decision == "approved",
        "role_assignment" => decision == "roles_assigned",
        "principal_version" => decision == "authority_or_eligibility_updated",
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "review event type '{event_type}' rejects decision '{decision}'"
        )))
    }
}

struct ProjectionRoots {
    science: String,
    communication: String,
    attribution: String,
    governance: String,
    subject: String,
}

struct ApprovalRoots {
    preapproval_realization: String,
    pre_steward: Option<String>,
    approval_lock: Option<String>,
    realization: Option<String>,
    release_transfer: Option<String>,
}

pub(super) fn calculate_review_lock(
    root: &Path,
    report_id: &str,
    report: &serde_yaml::Value,
    principals: &serde_yaml::Value,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
    legacy_subject_root: Option<String>,
    event_ids: Vec<String>,
) -> Result<ReviewLock> {
    validate_report_projection_classification()?;
    let lifecycle = yaml_text(report, "lifecycle")?.to_owned();
    let roots = calculate_projection_roots(root, report, principals, replacements)?;
    let events = load_review_events(root, report_id, &event_ids, replacements)?;
    let finding_ledger_root = calculate_finding_ledger(&lifecycle, &events, &roots.subject)?;
    let approvals = calculate_approval_roots(&events, &roots, finding_ledger_root.as_deref())?;
    if lifecycle == "APPROVED" && approvals.approval_lock.is_none() {
        return Err(AssuranceError::Invalid(
            "APPROVED lifecycle requires current approval authority".to_owned(),
        ));
    }
    Ok(ReviewLock {
        machine_owned: true,
        format_version: 1,
        identity_algorithm_version: IDENTITY_ALGORITHM.to_owned(),
        report_id: report_id.to_owned(),
        lifecycle,
        legacy_subject_root,
        science_root: roots.science,
        communication_root: roots.communication,
        attribution_root: roots.attribution,
        review_governance_root: roots.governance,
        content_review_subject_root: roots.subject,
        finding_ledger_root,
        preapproval_realization_root: approvals.preapproval_realization,
        pre_steward_approval_root: approvals.pre_steward,
        approval_lock_root: approvals.approval_lock,
        realization_root: approvals.realization,
        release_transfer_root: approvals.release_transfer,
        event_ids,
        invalidated_event_ids: Vec::new(),
    })
}

fn calculate_projection_roots(
    root: &Path,
    report: &serde_yaml::Value,
    principals: &serde_yaml::Value,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<ProjectionRoots> {
    let science = science_projection(root, report, replacements)?;
    let science = digest_serialized("openwepp-assurance-science-v1", &science)?;
    let communication = communication_root(root, report, replacements)?;
    let attribution = attribution_projection(report, principals)?;
    let attribution = digest_serialized("openwepp-assurance-attribution-v1", &attribution)?;
    let governance = governance_projection(report, principals)?;
    let governance = digest_serialized("openwepp-assurance-review-governance-v1", &governance)?;
    let subject = digest_serialized(
        "openwepp-assurance-content-review-subject-v1",
        &serde_json::json!({
            "science_root": science,
            "communication_root": communication,
            "review_governance_root": governance,
        }),
    )?;
    Ok(ProjectionRoots {
        science,
        communication,
        attribution,
        governance,
        subject,
    })
}

fn is_finding_ledger_event(event: &ReviewEvent) -> bool {
    matches!(
        event.event_type.as_str(),
        "review_entry" | "finding" | "disposition" | "withdrawal" | "supersession"
    )
}

fn calculate_finding_ledger(
    lifecycle: &str,
    events: &[ReviewEvent],
    subject_root: &str,
) -> Result<Option<String>> {
    for event in events.iter().filter(|event| is_finding_ledger_event(event)) {
        require_bound(event, "content_review_subject_root", subject_root)?;
    }
    let finding_event_ids = events
        .iter()
        .filter(|event| is_finding_ledger_event(event))
        .map(|event| event.event_id.clone())
        .collect::<Vec<_>>();
    (lifecycle != "DRAFT")
        .then(|| {
            digest_serialized(
                "openwepp-assurance-finding-ledger-v1",
                &serde_json::json!({
                    "content_review_subject_root": subject_root,
                    "review_event_ids": &finding_event_ids,
                }),
            )
        })
        .transpose()
}

fn calculate_approval_roots(
    events: &[ReviewEvent],
    roots: &ProjectionRoots,
    finding_ledger_root: Option<&str>,
) -> Result<ApprovalRoots> {
    let preapproval_realization = digest_serialized(
        "openwepp-assurance-preapproval-realization-v2",
        &serde_json::json!({
            "science_root": roots.science,
            "communication_root": roots.communication,
            "review_governance_root": roots.governance,
            "content_review_subject_root": roots.subject,
            "builder_identity": "openwepp-assurance-assembly:1",
            "implementation_digest": assurance_implementation_digest(),
        }),
    )?;
    let scientific = approval_events(events, "scientific_approval");
    let reproduction = approval_events(events, "reproduction_approval");
    let steward = approval_events(events, "steward_approval");
    validate_finding_closure(events, !scientific.is_empty() || !reproduction.is_empty())?;
    let approvals = scientific
        .iter()
        .chain(reproduction.iter())
        .map(|event| event.event_id.clone())
        .collect::<Vec<_>>();
    let pre_steward = (!approvals.is_empty())
        .then(|| {
            digest_serialized(
                "openwepp-assurance-pre-steward-approval-v1",
                &serde_json::json!({"approval_event_ids": approvals}),
            )
        })
        .transpose()?;
    let approval_lock = match (pre_steward.as_ref(), steward.as_slice()) {
        (Some(pre_steward), [event]) => Some(digest_serialized(
            "openwepp-assurance-approval-lock-v1",
            &serde_json::json!({
                "pre_steward_approval_root": pre_steward,
                "steward_event_id": event.event_id,
                "predecessor_event_ids": event.predecessor_event_ids,
            }),
        )?),
        (_, []) => None,
        _ => {
            return Err(AssuranceError::Invalid(
                "approval ledger permits at most one active steward approval".to_owned(),
            ));
        }
    };
    validate_approval_bindings(
        &scientific,
        &reproduction,
        &steward,
        roots,
        finding_ledger_root,
        &preapproval_realization,
        pre_steward.as_deref(),
    )?;
    let realization = approval_lock
        .as_ref()
        .map(|approval| {
            digest_serialized(
                "openwepp-assurance-realization-v2",
                &serde_json::json!({
                    "preapproval_realization_root": preapproval_realization,
                    "attribution_root": roots.attribution,
                    "approval_lock_root": approval,
                    "implementation_digest": assurance_implementation_digest(),
                }),
            )
        })
        .transpose()?;
    let release_transfer = calculate_release_transfer_root(
        events,
        &steward,
        approval_lock.as_deref(),
        realization.as_deref(),
    )?;
    Ok(ApprovalRoots {
        preapproval_realization,
        pre_steward,
        approval_lock,
        realization,
        release_transfer,
    })
}

fn calculate_release_transfer_root(
    events: &[ReviewEvent],
    steward: &[&ReviewEvent],
    approval_lock: Option<&str>,
    realization: Option<&str>,
) -> Result<Option<String>> {
    let release_events = approval_events(events, "release_transfer");
    Ok(match release_events.as_slice() {
        [] => None,
        [event] => {
            let approval = approval_lock.ok_or_else(|| {
                AssuranceError::Invalid("release transfer requires approval authority".to_owned())
            })?;
            let realization = realization.ok_or_else(|| {
                AssuranceError::Invalid("release transfer requires a realization".to_owned())
            })?;
            require_bound(event, "approval_lock_root", approval)?;
            require_bound(event, "realization_root", realization)?;
            let steward_ids = steward
                .iter()
                .map(|event| event.event_id.as_str())
                .collect::<BTreeSet<_>>();
            require_predecessors(event, &steward_ids)?;
            let [steward_event] = steward else {
                return Err(AssuranceError::Invalid(
                    "release transfer requires exactly one steward approval".to_owned(),
                ));
            };
            if event.inputs.get("assurance_steward_id").map(String::as_str)
                != Some(steward_event.principal_id.as_str())
            {
                return Err(AssuranceError::Invalid(
                    "release transfer names a steward other than its approval predecessor"
                        .to_owned(),
                ));
            }
            for field in [
                "target_release_commit",
                "target_release_configuration",
                "assurance_steward_id",
                "publication_date",
                "public_path",
            ] {
                require_event_input(event, field)?;
            }
            Some(digest_serialized(
                "openwepp-assurance-release-transfer-v1",
                &serde_json::json!({
                    "event_id": event.event_id,
                    "approval_lock_root": approval,
                    "realization_root": realization,
                    "inputs": event.inputs,
                }),
            )?)
        }
        _ => {
            return Err(AssuranceError::Invalid(
                "release-transfer ledger permits at most one active event".to_owned(),
            ));
        }
    })
}

fn validate_finding_closure(events: &[ReviewEvent], approvals_present: bool) -> Result<()> {
    let findings = events
        .iter()
        .filter(|event| event.event_type == "finding")
        .map(|event| event.event_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut disposed = BTreeSet::new();
    for event in events
        .iter()
        .filter(|event| event.event_type == "disposition")
    {
        if event.predecessor_event_ids.len() != 1 {
            return Err(AssuranceError::Invalid(
                "a disposition must bind exactly one finding event".to_owned(),
            ));
        }
        let finding = event.predecessor_event_ids[0].as_str();
        if !findings.contains(finding) {
            return Err(AssuranceError::Invalid(
                "a disposition binds an unknown finding event".to_owned(),
            ));
        }
        if !matches!(
            event.decision.as_str(),
            "resolved" | "resolved_and_verified" | "rejected" | "follow_up"
        ) {
            return Err(AssuranceError::Invalid(
                "a disposition decision must be resolved, resolved_and_verified, rejected, or follow_up"
                    .to_owned(),
            ));
        }
        if !disposed.insert(finding) {
            return Err(AssuranceError::Invalid(
                "a finding has more than one active disposition".to_owned(),
            ));
        }
    }
    if approvals_present && findings != disposed {
        return Err(AssuranceError::Invalid(
            "approval is prohibited while an active finding lacks disposition".to_owned(),
        ));
    }
    Ok(())
}

fn require_event_input<'a>(event: &'a ReviewEvent, name: &str) -> Result<&'a str> {
    event
        .inputs
        .get(name)
        .map(String::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            AssuranceError::Invalid(format!(
                "{} event '{}' omits required input {name}",
                event.event_type, event.event_id
            ))
        })
}

pub(super) fn collect_regular_sources(
    root: &Path,
    external: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>> {
    let mut sources = external.clone();
    collect_tree(root, &root.join("assurance/v2"), &mut sources)?;
    sources.remove(IDENTITY_LOCK_PATH);
    sources.retain(|path, _| !is_generated_or_receipt(path));
    Ok(sources)
}

fn load_review_events(
    root: &Path,
    report_id: &str,
    event_ids: &[String],
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<Vec<ReviewEvent>> {
    let mut events = Vec::new();
    let mut seen = BTreeSet::new();
    for event_id in event_ids {
        if !seen.insert(event_id) {
            return Err(AssuranceError::Invalid(format!(
                "duplicate active review event '{event_id}'"
            )));
        }
        let path = PathBuf::from(format!(
            "assurance/v2/reports/{report_id}/review-events/{event_id}.json"
        ));
        let bytes = match replacements.get(&path) {
            Some(bytes) => bytes.clone(),
            None => read_regular_confined(root, &path)?,
        };
        let event: ReviewEvent =
            serde_json::from_slice(&bytes).map_err(|error| AssuranceError::Parse {
                path: path.clone(),
                message: error.to_string(),
            })?;
        event.validate(report_id)?;
        if event.render()? != bytes {
            return Err(AssuranceError::Invalid(format!(
                "review event is not canonical JSON: {}",
                path.display()
            )));
        }
        events.push(event);
    }
    Ok(events)
}

fn approval_events<'a>(events: &'a [ReviewEvent], kind: &str) -> Vec<&'a ReviewEvent> {
    events
        .iter()
        .filter(|event| event.event_type == kind)
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn validate_approval_bindings(
    scientific: &[&ReviewEvent],
    reproduction: &[&ReviewEvent],
    steward: &[&ReviewEvent],
    roots: &ProjectionRoots,
    ledger_root: Option<&str>,
    realization_root: &str,
    pre_steward_root: Option<&str>,
) -> Result<()> {
    let ledger = if scientific.is_empty() && reproduction.is_empty() {
        ledger_root.unwrap_or("")
    } else {
        ledger_root.ok_or_else(|| {
            AssuranceError::Invalid("approval events require an active finding ledger".to_owned())
        })?
    };
    for event in scientific {
        validate_scientific_approval(event, roots, ledger)?;
    }
    let scientific_ids = scientific
        .iter()
        .map(|event| event.event_id.as_str())
        .collect::<BTreeSet<_>>();
    for event in reproduction {
        validate_reproduction_approval(event, roots, ledger, realization_root, &scientific_ids)?;
    }
    let predecessor_ids = scientific
        .iter()
        .chain(reproduction.iter())
        .map(|event| event.event_id.as_str())
        .collect::<BTreeSet<_>>();
    for event in steward {
        validate_steward_approval(
            event,
            roots,
            ledger,
            realization_root,
            pre_steward_root,
            &predecessor_ids,
        )?;
    }
    Ok(())
}

fn validate_common_approval(
    event: &ReviewEvent,
    roots: &ProjectionRoots,
    ledger: &str,
) -> Result<()> {
    require_event_input(event, "competence_basis")?;
    require_event_input(event, "independence_attestation")?;
    require_bound(event, "science_root", &roots.science)?;
    require_bound(event, "communication_root", &roots.communication)?;
    require_bound(event, "review_governance_root", &roots.governance)?;
    require_bound(event, "content_review_subject_root", &roots.subject)?;
    require_bound(event, "finding_ledger_root", ledger)
}

fn validate_scientific_approval(
    event: &ReviewEvent,
    roots: &ProjectionRoots,
    ledger: &str,
) -> Result<()> {
    validate_common_approval(event, roots, ledger)
}

fn validate_reproduction_approval(
    event: &ReviewEvent,
    roots: &ProjectionRoots,
    ledger: &str,
    realization_root: &str,
    scientific_ids: &BTreeSet<&str>,
) -> Result<()> {
    validate_common_approval(event, roots, ledger)?;
    require_bound(event, "preapproval_realization_root", realization_root)?;
    require_predecessors(event, scientific_ids)
}

fn validate_steward_approval(
    event: &ReviewEvent,
    roots: &ProjectionRoots,
    ledger: &str,
    realization_root: &str,
    pre_steward_root: Option<&str>,
    predecessor_ids: &BTreeSet<&str>,
) -> Result<()> {
    validate_common_approval(event, roots, ledger)?;
    require_bound(event, "preapproval_realization_root", realization_root)?;
    require_bound(event, "attribution_root", &roots.attribution)?;
    let pre_steward = pre_steward_root.ok_or_else(|| {
        AssuranceError::Invalid("steward approval requires predecessor approvals".to_owned())
    })?;
    require_bound(event, "pre_steward_approval_root", pre_steward)?;
    require_predecessors(event, predecessor_ids)
}

fn require_bound(event: &ReviewEvent, name: &str, expected: &str) -> Result<()> {
    if event.bound_roots.get(name).map(String::as_str) == Some(expected) {
        Ok(())
    } else {
        let observed = event.bound_roots.get(name).map(String::as_str);
        Err(AssuranceError::Invalid(format!(
            "{} event '{}' does not bind current {name}: observed={observed:?}, expected={expected}",
            event.event_type, event.event_id,
        )))
    }
}

fn require_predecessors(event: &ReviewEvent, expected: &BTreeSet<&str>) -> Result<()> {
    let observed = event
        .predecessor_event_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if observed == *expected {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "{} event '{}' has incomplete predecessor bindings",
            event.event_type, event.event_id
        )))
    }
}

fn collect_tree(root: &Path, current: &Path, output: &mut BTreeMap<String, String>) -> Result<()> {
    let mut entries = fs::read_dir(current)
        .map_err(|error| AssuranceError::io(current, error))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| AssuranceError::io(current, error))?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let metadata =
            fs::symlink_metadata(&path).map_err(|error| AssuranceError::io(&path, error))?;
        if metadata.file_type().is_symlink() {
            return Err(AssuranceError::Invalid(format!(
                "identity source tree contains a symlink: {}",
                path.display()
            )));
        }
        if metadata.is_dir() {
            collect_tree(root, &path, output)?;
        } else if metadata.is_file() {
            let relative = path.strip_prefix(root).map_err(|_| {
                AssuranceError::Invalid(format!("identity path escaped root: {}", path.display()))
            })?;
            let key = relative.to_str().ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "identity path is not UTF-8: {}",
                    relative.display()
                ))
            })?;
            let bytes = read_regular_confined(root, relative)?;
            output.insert(key.to_owned(), sha256_bytes(&bytes));
        } else {
            return Err(AssuranceError::Invalid(format!(
                "identity source is not a regular file: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

fn hydrate_value(value: &mut serde_yaml::Value, lock: &IdentityLock) -> Result<()> {
    match value {
        serde_yaml::Value::Mapping(mapping) => {
            hydrate_mapping(mapping, lock)?;
            for child in mapping.values_mut() {
                hydrate_value(child, lock)?;
            }
        }
        serde_yaml::Value::Sequence(values) => {
            for child in values {
                hydrate_value(child, lock)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn hydrate_mapping(mapping: &mut serde_yaml::Mapping, lock: &IdentityLock) -> Result<()> {
    let path_key = serde_yaml::Value::String("path".to_owned());
    let digest_key = serde_yaml::Value::String("sha256".to_owned());
    if !mapping.contains_key(&digest_key) {
        if let Some(path) = mapping.get(&path_key).and_then(serde_yaml::Value::as_str) {
            super::validate_relative(Path::new(path))?;
            let digest = lock.sources.get(path).ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "generated identity lock omits identified source '{path}'"
                ))
            })?;
            mapping.insert(digest_key, serde_yaml::Value::String(digest.clone()));
        }
    }
    let manifest_key = serde_yaml::Value::String("manifest_path".to_owned());
    let manifest_digest_key = serde_yaml::Value::String("manifest_sha256".to_owned());
    if !mapping.contains_key(&manifest_digest_key) {
        if let Some(path) = mapping
            .get(&manifest_key)
            .and_then(serde_yaml::Value::as_str)
        {
            super::validate_relative(Path::new(path))?;
            let digest = lock.sources.get(path).ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "generated identity lock omits report manifest '{path}'"
                ))
            })?;
            mapping.insert(
                manifest_digest_key,
                serde_yaml::Value::String(digest.clone()),
            );
        }
    }
    Ok(())
}

fn validate_lock_member(path: &str, digest: &str, paths: &mut BTreeSet<String>) -> Result<()> {
    let relative = Path::new(path);
    validate_relative(relative)?;
    validate_digest(digest, "identity member digest")?;
    if path == IDENTITY_LOCK_PATH || is_receipt(path) {
        return Err(AssuranceError::Invalid(format!(
            "identity lock contains a self-referential or receipt path: {path}"
        )));
    }
    if !paths.insert(path.to_owned()) {
        return Err(AssuranceError::Invalid(format!(
            "identity member appears in more than one layer: {path}"
        )));
    }
    Ok(())
}

fn is_generated_or_receipt(path: &str) -> bool {
    path.ends_with("/review.lock.json") || is_receipt(path)
}

fn is_receipt(path: &str) -> bool {
    path.starts_with("assurance/v2/transactions/")
}

fn validate_digest(value: &str, label: &str) -> Result<()> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "{label} must be lowercase SHA-256"
        )))
    }
}

fn require_text(value: &str, label: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(AssuranceError::Invalid(format!("{label} cannot be empty")))
    } else {
        Ok(())
    }
}

fn validate_report_projection_classification() -> Result<()> {
    let classified = SCIENCE_REPORT_FIELDS
        .iter()
        .chain(COMMUNICATION_REPORT_FIELDS)
        .chain(GOVERNANCE_REPORT_FIELDS)
        .chain(EVENT_GOVERNED_REPORT_FIELDS)
        .copied()
        .collect::<BTreeSet<_>>();
    let admitted = super::REPORT_FIELDS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if classified == admitted {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "report identity projection classification disagrees with the typed schema: classified={classified:?}, admitted={admitted:?}"
        )))
    }
}

fn assurance_implementation_digest() -> String {
    let mut material = b"openwepp-assurance-review-realization-implementation-v1\0".to_vec();
    for source in [
        include_bytes!("../v2.rs").as_slice(),
        include_bytes!("assembly.rs").as_slice(),
        include_bytes!("identity.rs").as_slice(),
        include_bytes!("lifecycle.rs").as_slice(),
        include_bytes!("planner.rs").as_slice(),
        include_bytes!("publication.rs").as_slice(),
    ] {
        material.extend_from_slice(&(source.len() as u64).to_be_bytes());
        material.extend_from_slice(source);
    }
    sha256_bytes(&material)
}

fn science_projection(
    root: &Path,
    report: &serde_yaml::Value,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<serde_json::Value> {
    let mapping = report.as_mapping().ok_or_else(|| {
        AssuranceError::Invalid("report science projection requires an object".to_owned())
    })?;
    let mut projected = serde_yaml::Mapping::new();
    for &field in SCIENCE_REPORT_FIELDS {
        let key = serde_yaml::Value::String(field.to_owned());
        let value = mapping.get(&key).ok_or_else(|| {
            AssuranceError::Invalid(format!("science projection omits required field '{field}'"))
        })?;
        projected.insert(key, value.clone());
    }
    let structure = serde_json::to_value(serde_yaml::Value::Mapping(projected))
        .map_err(|error| AssuranceError::Invalid(format!("science projection failed: {error}")))?;
    let mut files = BTreeMap::new();
    for field in ["dependencies", "results", "research_objects"] {
        if let Some(records) = report.get(field).and_then(serde_yaml::Value::as_sequence) {
            for record in records {
                if let Some(path) = record.get("path").and_then(serde_yaml::Value::as_str) {
                    let path = PathBuf::from(path);
                    let bytes = match replacements.get(&path) {
                        Some(bytes) => bytes.clone(),
                        None => read_regular_confined(root, &path)?,
                    };
                    files.insert(path.to_string_lossy().into_owned(), sha256_bytes(&bytes));
                }
            }
        }
    }
    Ok(serde_json::json!({
        "structure": structure,
        "identified_files": files,
    }))
}

fn communication_root(
    root: &Path,
    report: &serde_yaml::Value,
    replacements: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<String> {
    let structure = project_fields(report, COMMUNICATION_REPORT_FIELDS, "communication")?;
    let mut content = BTreeMap::new();
    for field in ["manuscript", "supplement"] {
        let path = report
            .get(field)
            .and_then(|value| value.get("path"))
            .and_then(serde_yaml::Value::as_str)
            .ok_or_else(|| AssuranceError::Invalid(format!("report {field} path is missing")))?;
        let relative = PathBuf::from(path);
        let bytes = match replacements.get(&relative) {
            Some(bytes) => bytes.clone(),
            None => read_regular_confined(root, &relative)?,
        };
        content.insert(field.to_owned(), sha256_bytes(&bytes));
    }
    digest_serialized(
        "openwepp-assurance-communication-v2",
        &serde_json::json!({"structure": structure, "identified_files": content}),
    )
}

fn attribution_projection(
    report: &serde_yaml::Value,
    principals: &serde_yaml::Value,
) -> Result<serde_json::Value> {
    let mut bibliography = Vec::new();
    let used = used_principal_ids(report);
    for (id, principal) in latest_principals(principals)? {
        if !used.contains(id) {
            continue;
        }
        bibliography.push(project_fields(
            principal,
            &["id", "record_version", "display_name", "affiliations"],
            "principal bibliography",
        )?);
    }
    Ok(serde_json::json!({
        "authorship": yaml_json_field(report, "authorship")?,
        "principals": bibliography,
    }))
}

fn governance_projection(
    report: &serde_yaml::Value,
    principals: &serde_yaml::Value,
) -> Result<serde_json::Value> {
    let report_governance = project_fields(
        report,
        &[
            "schema_version",
            "contract_version",
            "id",
            "version",
            "owner",
            "trust_domain",
            "fixture_only",
            "authorship",
            "agent_assistance",
        ],
        "report governance",
    )?;
    let review = report
        .get("review")
        .ok_or_else(|| AssuranceError::Invalid("report review is missing".to_owned()))?;
    let mut authority = Vec::new();
    let used = used_principal_ids(report);
    for (id, principal) in latest_principals(principals)? {
        if !used.contains(id) {
            continue;
        }
        authority.push(project_fields(
            principal,
            &[
                "id",
                "kind",
                "identity_authority",
                "identity_reference",
                "roles",
            ],
            "principal authority",
        )?);
    }
    Ok(serde_json::json!({
        "report": report_governance,
        "review": project_fields(
            review,
            &[
                "review_charge",
                "build_maintainer_id",
                "material_producer_ids",
                "independence_assessment",
            ],
            "review governance",
        )?,
        "event_governed_fields": EVENT_GOVERNED_REPORT_FIELDS,
        "principals": authority,
        "identity_algorithm": IDENTITY_ALGORITHM,
    }))
}

fn used_principal_ids(report: &serde_yaml::Value) -> BTreeSet<&str> {
    let mut ids = BTreeSet::new();
    if let Some(authorship) = report.get("authorship") {
        for field in ["human_report_lead", "scientific_approver"] {
            if let Some(id) = authorship.get(field).and_then(serde_yaml::Value::as_str) {
                ids.insert(id);
            }
        }
    }
    if let Some(review) = report.get("review") {
        if let Some(id) = review
            .get("build_maintainer_id")
            .and_then(serde_yaml::Value::as_str)
        {
            ids.insert(id);
        }
        if let Some(producers) = review
            .get("material_producer_ids")
            .and_then(serde_yaml::Value::as_sequence)
        {
            for producer in producers {
                if let Some(id) = producer.as_str() {
                    ids.insert(id);
                }
            }
        }
    }
    ids
}

fn latest_principals(principals: &serde_yaml::Value) -> Result<BTreeMap<&str, &serde_yaml::Value>> {
    let values = principals
        .get("principals")
        .and_then(serde_yaml::Value::as_sequence)
        .ok_or_else(|| AssuranceError::Invalid("principal list is missing".to_owned()))?;
    let mut latest = BTreeMap::<&str, (u64, &serde_yaml::Value)>::new();
    for principal in values {
        let id = principal
            .get("id")
            .and_then(serde_yaml::Value::as_str)
            .ok_or_else(|| AssuranceError::Invalid("principal ID is missing".to_owned()))?;
        let version = principal
            .get("record_version")
            .and_then(serde_yaml::Value::as_u64)
            .ok_or_else(|| {
                AssuranceError::Invalid("principal record_version is missing".to_owned())
            })?;
        match latest.get(id) {
            Some((current, _)) if *current >= version => {}
            _ => {
                latest.insert(id, (version, principal));
            }
        }
    }
    Ok(latest
        .into_iter()
        .map(|(id, (_, principal))| (id, principal))
        .collect())
}

fn project_fields(
    value: &serde_yaml::Value,
    fields: &[&str],
    label: &str,
) -> Result<serde_json::Value> {
    let mapping = value
        .as_mapping()
        .ok_or_else(|| AssuranceError::Invalid(format!("{label} is not an object")))?;
    let mut projected = serde_yaml::Mapping::new();
    for field in fields {
        let key = serde_yaml::Value::String((*field).to_owned());
        let child = mapping.get(&key).ok_or_else(|| {
            AssuranceError::Invalid(format!("{label} omits required field '{field}'"))
        })?;
        projected.insert(key, child.clone());
    }
    serde_json::to_value(serde_yaml::Value::Mapping(projected))
        .map_err(|error| AssuranceError::Invalid(format!("{label} projection failed: {error}")))
}

fn yaml_json_field(value: &serde_yaml::Value, field: &str) -> Result<serde_json::Value> {
    let child = value
        .get(field)
        .ok_or_else(|| AssuranceError::Invalid(format!("report field '{field}' is missing")))?;
    serde_json::to_value(child)
        .map_err(|error| AssuranceError::Invalid(format!("field projection failed: {error}")))
}

fn yaml_text<'a>(value: &'a serde_yaml::Value, field: &str) -> Result<&'a str> {
    value
        .get(field)
        .and_then(serde_yaml::Value::as_str)
        .ok_or_else(|| AssuranceError::Invalid(format!("report field '{field}' is missing")))
}

fn digest_serialized(domain: &str, value: &impl Serialize) -> Result<String> {
    let bytes = serde_json::to_vec(value).map_err(|error| {
        AssuranceError::Invalid(format!("identity projection serialization failed: {error}"))
    })?;
    let mut material = domain.as_bytes().to_vec();
    material.push(0);
    material.extend_from_slice(&bytes);
    Ok(sha256_bytes(&material))
}

fn canonical_pretty_json(value: &impl Serialize, label: &str) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|error| {
        AssuranceError::Invalid(format!("{label} serialization failed: {error}"))
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}
