use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::canonical::{derived_id, digest, parse_strict, sha256_bytes, validate_schema};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::executor::authority_adapter_supported;

pub trait ArtifactProvider {
    /// Read one confined artifact by repository-relative receipt path.
    ///
    /// # Errors
    ///
    /// Returns an artifact error when bytes cannot be resolved exactly.
    fn read(&self, path: &str) -> Result<Vec<u8>>;

    fn workspace_root(&self) -> Option<&Path> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct DirectoryArtifacts {
    root: PathBuf,
}

impl DirectoryArtifacts {
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl ArtifactProvider for DirectoryArtifacts {
    fn read(&self, path: &str) -> Result<Vec<u8>> {
        if path.starts_with('/') || path.split('/').any(|part| matches!(part, "" | "." | "..")) {
            return Err(verification_error("GATE-ARTIFACT-PATH", path));
        }
        read_confined(&self.root, path).map_err(|error| {
            GatePolicyError::new(
                ErrorClass::Receipt,
                "GATE-ARTIFACT-READ",
                format!("{path}: {error}"),
            )
        })
    }

    fn workspace_root(&self) -> Option<&Path> {
        Some(&self.root)
    }
}

#[cfg(target_os = "linux")]
fn read_confined(root: &Path, path: &str) -> std::io::Result<Vec<u8>> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};

    let no_links = ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS;
    let root_fd = openat2(
        rustix::fs::CWD,
        root,
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        no_links,
    )?;
    let artifact_fd = openat2(
        &root_fd,
        path,
        OFlags::RDONLY | OFlags::CLOEXEC,
        Mode::empty(),
        no_links | ResolveFlags::BENEATH,
    )?;
    let mut file = fs::File::from(artifact_fd);
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

#[cfg(not(target_os = "linux"))]
fn read_confined(_root: &Path, _path: &str) -> std::io::Result<Vec<u8>> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "no-follow artifact reads are unavailable on this platform",
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptVerdict {
    receipt_id: String,
    receipt_sha256: String,
    plan_id: String,
    plan_sha256: String,
    execution_key: String,
    roots_sha256: String,
    boundary: String,
    result: String,
    trust_class: String,
    claimed_trust_class: String,
}

impl ReceiptVerdict {
    #[must_use]
    pub fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    #[must_use]
    pub fn result(&self) -> &str {
        &self.result
    }

    #[must_use]
    pub fn trust_class(&self) -> &str {
        &self.trust_class
    }

    #[must_use]
    pub fn claimed_trust_class(&self) -> &str {
        &self.claimed_trust_class
    }
}

/// Verify an unsigned receipt against its plan and artifact bytes.
///
/// # Errors
///
/// Returns a receipt error for any schema, identity, source, DAG, inventory, or artifact mismatch.
#[allow(
    clippy::too_many_lines,
    reason = "sequential receipt audit follows the wire contract"
)]
pub fn verify_receipt(
    repo: &Path,
    plan: &Value,
    receipt: &Value,
    artifacts: &dyn ArtifactProvider,
) -> Result<ReceiptVerdict> {
    verify_receipt_identity(repo, plan, receipt)?;
    let reconstructed = reconstruct_receipt_plan(repo, plan, receipt, artifacts)?;
    if digest(&reconstructed)? != digest(plan)? {
        return Err(verification_error(
            "GATE-RECEIPT-PLAN-RECONSTRUCTION",
            "observed source, selection, or node contract differs from the supplied plan",
        ));
    }
    verify_source_roots(repo, plan, receipt)?;
    verify_tool_environment(repo, plan, receipt)?;
    let plan_nodes = array(plan, "/nodes")?;
    verify_dag(plan_nodes, receipt)?;
    let counts = verify_attempts(plan_nodes, receipt)?;
    verify_artifacts(plan_nodes, receipt, artifacts)?;
    verify_inventory(plan_nodes, receipt, artifacts)?;
    verify_receipt_summary(repo, plan, receipt, &counts)?;
    verify_authority_outcomes(plan_nodes, receipt)?;
    receipt_verdict(plan, receipt, &counts)
}

/// Verify the immutable receipt/plan/artifact envelope without trusting the
/// execution host's live tool installation.
///
/// This is the cross-host signing boundary: it recomputes schemas, identities,
/// source roots, DAG/results, artifacts, inventory, summary, and authority
/// outcomes. Full execution-host verification additionally reconstructs the
/// plan and compares the live tool/environment projection.
///
/// # Errors
///
/// Returns a receipt error for any immutable envelope mismatch.
pub fn verify_receipt_envelope(
    repo: &Path,
    plan: &Value,
    receipt: &Value,
    artifacts: &dyn ArtifactProvider,
) -> Result<ReceiptVerdict> {
    verify_receipt_identity(repo, plan, receipt)?;
    let root = artifacts.workspace_root().ok_or_else(|| {
        verification_error(
            "GATE-RECEIPT-RECONSTRUCTION-ROOT",
            "artifact provider must supply an external reconstruction root",
        )
    })?;
    let reconstructed = crate::planner::reconstruct_plan_in_with_bound_context(
        repo,
        plan,
        &root.join(".envelope-verification"),
        receipt["source_mutation_check"]["unchanged"] == false,
    )?;
    if digest(&reconstructed)? != digest(plan)? {
        return Err(verification_error(
            "GATE-RECEIPT-PLAN-RECONSTRUCTION",
            "source selection or node contract differs from the supplied plan",
        ));
    }
    verify_source_roots(repo, plan, receipt)?;
    verify_tool_environment_bindings(plan, receipt)?;
    let plan_nodes = array(plan, "/nodes")?;
    verify_dag(plan_nodes, receipt)?;
    let counts = verify_attempts(plan_nodes, receipt)?;
    verify_artifacts(plan_nodes, receipt, artifacts)?;
    verify_inventory(plan_nodes, receipt, artifacts)?;
    verify_receipt_summary(repo, plan, receipt, &counts)?;
    verify_authority_outcomes(plan_nodes, receipt)?;
    receipt_verdict(plan, receipt, &counts)
}

fn receipt_verdict(
    plan: &Value,
    receipt: &Value,
    counts: &ReceiptCounts,
) -> Result<ReceiptVerdict> {
    Ok(ReceiptVerdict {
        receipt_id: string(receipt, "/receipt_id")?.to_owned(),
        receipt_sha256: digest(receipt)?,
        plan_id: string(plan, "/plan_id")?.to_owned(),
        plan_sha256: digest(plan)?,
        execution_key: string(plan, "/execution_key")?.to_owned(),
        roots_sha256: digest(&plan["environment_roots"])?,
        boundary: string(plan, "/boundary")?.to_owned(),
        result: counts.result().to_owned(),
        trust_class: "LOCAL_UNTRUSTED".to_owned(),
        claimed_trust_class: string(receipt, "/claims/trust_class")?.to_owned(),
    })
}

fn reconstruct_receipt_plan(
    repo: &Path,
    plan: &Value,
    receipt: &Value,
    artifacts: &dyn ArtifactProvider,
) -> Result<Value> {
    let after_source_mutation = receipt["source_mutation_check"]["unchanged"] == false;
    let root = artifacts.workspace_root().ok_or_else(|| {
        verification_error(
            "GATE-RECEIPT-RECONSTRUCTION-ROOT",
            "artifact provider must supply an external reconstruction root",
        )
    })?;
    crate::planner::reconstruct_plan_in(repo, plan, &root.join(".work"), after_source_mutation)
}

fn verify_tool_environment(repo: &Path, plan: &Value, receipt: &Value) -> Result<()> {
    verify_tool_environment_bindings(plan, receipt)?;
    let tools = crate::planner::tool_records(repo)?;
    let environment =
        crate::planner::environment_record(repo, string(receipt, "/environment/target_triple")?)?;
    if digest(&tools)? != plan["execution_context"]["tool_manifest_sha256"]
        || receipt["tools"] != tools
        || digest(&environment)? != plan["execution_context"]["environment_manifest_sha256"]
        || receipt["environment"] != environment
    {
        Err(verification_error(
            "GATE-RECEIPT-EXECUTION-CONTEXT",
            "tool or environment projection differs from the verified plan",
        ))
    } else {
        Ok(())
    }
}

fn verify_tool_environment_bindings(plan: &Value, receipt: &Value) -> Result<()> {
    if digest(&receipt["tools"])? != plan["execution_context"]["tool_manifest_sha256"]
        || digest(&receipt["environment"])?
            != plan["execution_context"]["environment_manifest_sha256"]
    {
        Err(verification_error(
            "GATE-RECEIPT-EXECUTION-CONTEXT",
            "immutable tool or environment evidence differs from the verified plan",
        ))
    } else {
        Ok(())
    }
}

fn verify_receipt_identity(repo: &Path, plan: &Value, receipt: &Value) -> Result<()> {
    validate_document(repo, "gate-plan", plan)?;
    validate_document(repo, "gate-receipt", receipt)?;
    crate::planner::verify_plan_identity(plan)?;
    verify_derived_id(receipt, "receipt_id", "GATE-RECEIPT-ID")?;

    equal(receipt, "/plan_id", plan, "/plan_id", "GATE-RECEIPT-PLAN")?;
    equal(
        receipt,
        "/execution_key",
        plan,
        "/execution_key",
        "GATE-RECEIPT-EXECUTION",
    )?;
    equal(
        receipt,
        "/boundary",
        plan,
        "/boundary",
        "GATE-RECEIPT-BOUNDARY",
    )?;
    equal(
        receipt,
        "/campaign_id",
        plan,
        "/campaign_id",
        "GATE-RECEIPT-CAMPAIGN",
    )?;
    equal(
        receipt,
        "/roots",
        plan,
        "/environment_roots",
        "GATE-RECEIPT-ROOTS",
    )?;
    if receipt["plan_sha256"] != digest(plan)? {
        return Err(verification_error(
            "GATE-RECEIPT-PLAN-DIGEST",
            "plan digest mismatch",
        ));
    }
    for field in ["base_commit", "head_commit", "dirty_tree_digest"] {
        equal(
            receipt,
            &format!("/source/{field}"),
            plan,
            &format!("/source/{field}"),
            "GATE-RECEIPT-SOURCE",
        )?;
    }
    Ok(())
}

fn verify_source_roots(repo: &Path, plan: &Value, receipt: &Value) -> Result<()> {
    let root_revision = plan["source"]["head_commit"].as_str().unwrap_or("HEAD");
    let reconstructed_roots = crate::planner::manifest_roots(
        repo,
        root_revision,
        plan["source"]["head_commit"].is_null(),
    )?;
    if reconstructed_roots != plan["environment_roots"] {
        return Err(verification_error(
            "GATE-RECEIPT-ROOT-RECONSTRUCTION",
            "current repository manifests do not reconstruct the planned roots",
        ));
    }
    if receipt["source"]["tree_sha256"] != digest(&plan["environment_roots"])? {
        return Err(verification_error(
            "GATE-RECEIPT-TREE",
            "source tree/root digest mismatch",
        ));
    }
    Ok(())
}

fn verify_dag(plan_nodes: &[Value], receipt: &Value) -> Result<()> {
    let receipt_nodes = array(receipt, "/dag_nodes")?;
    if receipt_nodes.len() != plan_nodes.len() {
        return Err(verification_error(
            "GATE-RECEIPT-DAG-SIZE",
            "DAG node count mismatch",
        ));
    }
    if receipt["zero_work"] != plan_nodes.is_empty() {
        return Err(verification_error(
            "GATE-RECEIPT-ZERO-WORK",
            "zero-work flag does not match the plan",
        ));
    }
    if receipt["dag_sha256"] != digest(&Value::Array(plan_nodes.to_vec()))? {
        return Err(verification_error(
            "GATE-RECEIPT-DAG-DIGEST",
            "DAG digest mismatch",
        ));
    }
    for (plan_node, receipt_node) in plan_nodes.iter().zip(receipt_nodes) {
        let mut snapshot = receipt_node.clone();
        let snapshot_object = snapshot.as_object_mut().ok_or_else(|| {
            verification_error("GATE-RECEIPT-DAG", "node snapshot is not an object")
        })?;
        let plan_node_sha = snapshot_object
            .remove("plan_node_sha256")
            .ok_or_else(|| verification_error("GATE-RECEIPT-DAG", "missing plan node digest"))?;
        if plan_node_sha != digest(plan_node)? || snapshot != *plan_node {
            return Err(verification_error(
                "GATE-RECEIPT-DAG",
                "node snapshot differs from plan",
            ));
        }
    }
    Ok(())
}

fn verify_inventory(
    plan_nodes: &[Value],
    receipt: &Value,
    artifacts: &dyn ArtifactProvider,
) -> Result<()> {
    let expected_inventory = plan_nodes
        .iter()
        .flat_map(|node| {
            node["expected_inventory"]["ids"]
                .as_array()
                .into_iter()
                .flatten()
        })
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let planned_inventory = array(receipt, "/planned_inventory")?
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let executed_inventory = array(receipt, "/executed_inventory")?
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    if expected_inventory != planned_inventory {
        return Err(verification_error(
            "GATE-RECEIPT-INVENTORY",
            "planned inventory differs from the exact gate-node inventory",
        ));
    }
    if !plan_nodes.is_empty() && planned_inventory.is_empty() {
        return Err(verification_error(
            "GATE-RECEIPT-EMPTY-INVENTORY",
            "nonempty plan has empty inventory",
        ));
    }
    let attempts = array(receipt, "/attempts")?;
    let expected_executed = expected_executed_inventory(plan_nodes, attempts, artifacts)?;
    if executed_inventory != expected_executed {
        return Err(verification_error(
            "GATE-RECEIPT-EXECUTED-INVENTORY",
            "executed inventory differs from nodes with observed process exits",
        ));
    }
    let expected_unavailable = planned_inventory
        .difference(&executed_inventory)
        .cloned()
        .collect::<BTreeSet<_>>();
    let unavailable = array(receipt, "/unavailable_items")?;
    let observed_unavailable = unavailable
        .iter()
        .map(|item| string(item, "/item_id").map(str::to_owned))
        .collect::<std::result::Result<BTreeSet<_>, _>>()?;
    let allowed_reasons = [
        "PREREQUISITE_NONPASS",
        "PROCESS_SPAWN_FAILED",
        "SOURCE_MUTATION_DETECTED",
        "TEST_NOT_EXECUTED",
        "TIMEOUT",
    ];
    if expected_unavailable != observed_unavailable
        || unavailable.iter().any(|item| {
            item["policy_disposition"] != "BLOCK"
                || item["reason_code"]
                    .as_str()
                    .is_none_or(|reason| !allowed_reasons.contains(&reason))
        })
    {
        return Err(verification_error(
            "GATE-RECEIPT-UNAVAILABLE",
            "unavailable items do not exactly partition planned and executed inventory",
        ));
    }
    Ok(())
}

fn expected_executed_inventory(
    plan_nodes: &[Value],
    attempts: &[Value],
    artifacts: &dyn ArtifactProvider,
) -> Result<BTreeSet<String>> {
    let mut executed = BTreeSet::new();
    for node in plan_nodes {
        let attempt = attempts
            .iter()
            .rev()
            .find(|attempt| attempt["node_id"] == node["node_id"])
            .ok_or_else(|| verification_error("GATE-RECEIPT-ATTEMPT-MISSING", "node"))?;
        let planned = node["expected_inventory"]["ids"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        let observed = if node["gate_definition_id"] == "required-authority-v1" {
            receipt_authority_inventory(node, artifacts, attempt["result"] == "PASS")?
        } else if node_has_junit_evidence(node) {
            receipt_junit_inventory(node, artifacts)?
        } else if !attempt["exit_code"].is_null() || !attempt["termination_signal"].is_null() {
            planned.clone()
        } else {
            BTreeSet::new()
        };
        if !observed.is_subset(&planned) {
            return Err(verification_error(
                "GATE-RECEIPT-EXECUTED-INVENTORY",
                "JUnit inventory is outside the terminal plan",
            ));
        }
        executed.extend(observed);
    }
    Ok(executed)
}

fn receipt_authority_inventory(
    node: &Value,
    artifacts: &dyn ArtifactProvider,
    require_pass: bool,
) -> Result<BTreeSet<String>> {
    let path = node["output_paths"]
        .as_array()
        .and_then(|paths| paths.first())
        .and_then(Value::as_str)
        .ok_or_else(|| verification_error("GATE-RECEIPT-AUTHORITY-REPORT", "output path"))?;
    let bytes = artifacts.read(path)?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|error| verification_error("GATE-RECEIPT-AUTHORITY-REPORT", error.to_string()))?;
    let mut observed = BTreeSet::new();
    for line in text.lines().filter(|line| {
        line.starts_with("- lane=required failure_class=hard-fail ") && line.contains(" status=")
    }) {
        let status = line
            .split_ascii_whitespace()
            .find_map(|field| field.strip_prefix("status="))
            .ok_or_else(|| verification_error("GATE-RECEIPT-AUTHORITY-REPORT", line))?;
        if require_pass && status != "pass" {
            return Err(verification_error(
                "GATE-RECEIPT-AUTHORITY-REPORT",
                "PASS attempt contains a nonpassing required suite",
            ));
        }
        let suites = line
            .split_ascii_whitespace()
            .find_map(|field| field.strip_prefix("suites="))
            .ok_or_else(|| verification_error("GATE-RECEIPT-AUTHORITY-REPORT", line))?;
        observed.extend(suites.split(',').map(str::to_owned));
    }
    Ok(observed)
}

fn node_has_junit_evidence(node: &Value) -> bool {
    node["artifact_contract"] == "nextest-junit-v1" && node["executor"]["kind"] == "NEXTEST_V1"
        || node["artifact_contract"] == "adjudicated-crap-v1"
            && node["gate_definition_id"] == "affected-adjudicated-crap-v1"
}

fn receipt_junit_inventory(
    node: &Value,
    artifacts: &dyn ArtifactProvider,
) -> Result<BTreeSet<String>> {
    let path = node["output_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .find(|path| {
            Path::new(path)
                .extension()
                .is_some_and(|extension| extension == "xml")
        })
        .or_else(|| node["output_paths"].as_array()?.first()?.as_str())
        .ok_or_else(|| verification_error("GATE-RECEIPT-JUNIT", "missing JUnit output"))?;
    junit_inventory_bytes(&artifacts.read(path)?)
}

fn junit_inventory_bytes(bytes: &[u8]) -> Result<BTreeSet<String>> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| verification_error("GATE-RECEIPT-JUNIT", error.to_string()))?;
    let mut inventory = BTreeSet::new();
    for line in text
        .lines()
        .filter(|line| line.trim_start().starts_with("<testcase "))
    {
        let name = junit_attribute(line, "name")?;
        let class = junit_attribute(line, "classname")?;
        inventory.insert(sha256_bytes(
            format!("rust-suites::{class}\0{name}").as_bytes(),
        ));
    }
    Ok(inventory)
}

fn junit_attribute(line: &str, name: &str) -> Result<String> {
    let marker = format!(" {name}=\"");
    let value = line
        .split_once(&marker)
        .and_then(|(_, tail)| tail.split_once('"').map(|(value, _)| value))
        .ok_or_else(|| verification_error("GATE-RECEIPT-JUNIT", name))?;
    Ok(value
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&"))
}

#[derive(Debug, Default)]
struct ReceiptCounts {
    passed: u64,
    failed: u64,
    blocked: u64,
    invalid: bool,
    retried: u64,
}

impl ReceiptCounts {
    fn result(&self) -> &'static str {
        if self.invalid {
            "INVALID"
        } else if self.failed > 0 {
            "FAIL"
        } else if self.blocked > 0 {
            "BLOCKED"
        } else if self.retried > 0 {
            "PASS_WITH_RETRY"
        } else {
            "PASS"
        }
    }
}

fn verify_attempts(plan_nodes: &[Value], receipt: &Value) -> Result<ReceiptCounts> {
    let nodes_by_id = plan_nodes
        .iter()
        .map(|node| string(node, "/node_id").map(|id| (id, node)))
        .collect::<std::result::Result<BTreeMap<_, _>, _>>()?;
    let attempts_by_node = index_attempts(&nodes_by_id, array(receipt, "/attempts")?)?;
    let mut counts = ReceiptCounts::default();
    for (node_id, node) in &nodes_by_id {
        let attempts = attempts_by_node
            .get(node_id)
            .ok_or_else(|| verification_error("GATE-RECEIPT-MISSING-ATTEMPT", *node_id))?;
        verify_node_attempts(
            node_id,
            node,
            attempts,
            receipt["source_mutation_check"]["unchanged"] == false,
            &mut counts,
        )?;
    }
    verify_prerequisite_results(plan_nodes, receipt)?;
    Ok(counts)
}

fn verify_prerequisite_results(plan_nodes: &[Value], receipt: &Value) -> Result<()> {
    let attempts = array(receipt, "/attempts")?;
    let nodes = plan_nodes
        .iter()
        .map(|node| string(node, "/node_id").map(|id| (id, node)))
        .collect::<std::result::Result<BTreeMap<_, _>, _>>()?;
    for node in plan_nodes {
        let prerequisite_nonpass = array(node, "/prerequisites")?.iter().any(|id| {
            id.as_str()
                .and_then(|id| nodes.get(id))
                .and_then(|prerequisite| final_node_result(prerequisite, attempts).ok())
                .is_none_or(|result| !matches!(result, "PASS" | "PASS_WITH_RETRY"))
        });
        if prerequisite_nonpass && final_node_result(node, attempts)? != "BLOCKED" {
            return Err(verification_error(
                "GATE-RECEIPT-PREREQUISITE",
                string(node, "/node_id")?,
            ));
        }
    }
    Ok(())
}

fn index_attempts<'a>(
    nodes_by_id: &BTreeMap<&str, &Value>,
    attempts: &'a [Value],
) -> Result<BTreeMap<&'a str, Vec<&'a Value>>> {
    let mut attempts_by_node: BTreeMap<&str, Vec<&Value>> = BTreeMap::new();
    for attempt in attempts {
        let node_id = string(attempt, "/node_id")?;
        if !nodes_by_id.contains_key(node_id) {
            return Err(verification_error("GATE-RECEIPT-ATTEMPT-NODE", node_id));
        }
        attempts_by_node.entry(node_id).or_default().push(attempt);
    }
    Ok(attempts_by_node)
}

fn verify_node_attempts(
    node_id: &str,
    node: &Value,
    attempts: &[&Value],
    source_mutated: bool,
    counts: &mut ReceiptCounts,
) -> Result<()> {
    let maximum = node["retry"]["maximum_attempts"]
        .as_u64()
        .ok_or_else(|| verification_error("GATE-RECEIPT-RETRY", node_id))?;
    if attempts.len() > usize::try_from(maximum).unwrap_or(usize::MAX) {
        return Err(verification_error("GATE-RECEIPT-ATTEMPT-COUNT", node_id));
    }
    for (index, attempt) in attempts.iter().enumerate() {
        verify_attempt(node_id, node, attempt, index, source_mutated)?;
    }
    counts.retried += u64::try_from(attempts.len().saturating_sub(1)).unwrap_or(u64::MAX);
    update_result_counts(
        counts,
        string(attempts[attempts.len() - 1], "/result")?,
        node_id,
    )
}

fn verify_attempt(
    node_id: &str,
    node: &Value,
    attempt: &Value,
    index: usize,
    source_mutated: bool,
) -> Result<()> {
    if attempt["attempt"] != u64::try_from(index + 1).unwrap_or(u64::MAX)
        || attempt["arguments"] != node["arguments"]
    {
        return Err(verification_error("GATE-RECEIPT-ATTEMPT", node_id));
    }
    if index == 0 {
        return verify_attempt_acceptance(node_id, node, attempt, source_mutated);
    }
    let reason = string(attempt, "/retry_reason")?;
    let permitted = node["retry"]["permitted_reasons"]
        .as_array()
        .is_some_and(|reasons| reasons.iter().any(|candidate| candidate == reason));
    if permitted {
        verify_attempt_acceptance(node_id, node, attempt, source_mutated)
    } else {
        Err(verification_error("GATE-RECEIPT-RETRY-POLICY", reason))
    }
}

fn verify_attempt_acceptance(
    node_id: &str,
    node: &Value,
    attempt: &Value,
    source_mutated: bool,
) -> Result<()> {
    if attempt["result"] == "INVALID" && source_mutated {
        return Ok(());
    }
    let acceptance = &node["acceptance"];
    if acceptance["kind"] != "EXIT_CODE" || acceptance["operator"] != "EQUALS" {
        return Err(verification_error(
            "GATE-RECEIPT-ACCEPTANCE-UNSUPPORTED",
            node_id,
        ));
    }
    let expected = acceptance["expected"].as_i64().ok_or_else(|| {
        verification_error("GATE-RECEIPT-ACCEPTANCE", "expected exit code is missing")
    })?;
    let actual = attempt["exit_code"].as_i64();
    let signal = attempt["termination_signal"].as_i64();
    if actual.is_some() && signal.is_some() {
        return Err(verification_error("GATE-RECEIPT-ACCEPTANCE", node_id));
    }
    let expected_result = match (actual, signal) {
        (Some(code), None) if code == expected => "PASS",
        (Some(_), None) | (None, Some(_)) => "FAIL",
        (None, None) => "BLOCKED",
        (Some(_), Some(_)) => unreachable!(),
    };
    if attempt["result"] == expected_result {
        Ok(())
    } else {
        Err(verification_error("GATE-RECEIPT-ACCEPTANCE", node_id))
    }
}

fn update_result_counts(counts: &mut ReceiptCounts, result: &str, node_id: &str) -> Result<()> {
    match result {
        "PASS" => counts.passed += 1,
        "FAIL" => counts.failed += 1,
        "BLOCKED" => counts.blocked += 1,
        "INVALID" => counts.invalid = true,
        _ => return Err(verification_error("GATE-RECEIPT-RESULT", node_id)),
    }
    Ok(())
}

fn verify_receipt_summary(
    repo: &Path,
    plan: &Value,
    receipt: &Value,
    counts: &ReceiptCounts,
) -> Result<()> {
    let expected_result = counts.result();
    if string(receipt, "/result")? != expected_result {
        return Err(verification_error(
            "GATE-RECEIPT-AGGREGATE",
            "result precedence mismatch",
        ));
    }
    for (field, expected) in [
        ("passed", counts.passed),
        ("failed", counts.failed),
        ("blocked", counts.blocked),
        ("retried", counts.retried),
    ] {
        if receipt["counts"][field] != expected {
            return Err(verification_error("GATE-RECEIPT-COUNTS", field));
        }
    }
    let unavailable_count =
        u64::try_from(array(receipt, "/unavailable_items")?.len()).unwrap_or(u64::MAX);
    if receipt["counts"]["skipped"] != unavailable_count
        || unavailable_count > 0 && expected_result == "PASS"
    {
        return Err(verification_error(
            "GATE-RECEIPT-UNAVAILABLE",
            "passing receipt contains unavailable/skipped work",
        ));
    }
    let expected_source_snapshot = crate::executor::source_snapshot(plan)?;
    let observed_source_snapshot = crate::executor::observed_source_snapshot(repo, plan)?;
    let source_unchanged = expected_source_snapshot == observed_source_snapshot;
    if receipt["source_mutation_check"]["required"] != true
        || receipt["source_mutation_check"]["unchanged"] != source_unchanged
        || receipt["source_mutation_check"]["before_sha256"] != expected_source_snapshot
        || receipt["source_mutation_check"]["after_sha256"] != observed_source_snapshot
        || !source_unchanged && expected_result != "INVALID"
    {
        return Err(verification_error(
            "GATE-RECEIPT-SOURCE-MUTATION",
            "source changed during execution",
        ));
    }
    Ok(())
}

fn verify_authority_outcomes(plan_nodes: &[Value], receipt: &Value) -> Result<()> {
    let expected_gate_ids = plan_nodes
        .iter()
        .filter_map(|node| node["gate_definition_id"].as_str())
        .collect::<BTreeSet<_>>();
    let authority_gate_ids = array(receipt, "/authority_outcomes")?
        .iter()
        .map(|outcome| string(outcome, "/gate_id"))
        .collect::<std::result::Result<BTreeSet<_>, _>>()?;
    if expected_gate_ids != authority_gate_ids {
        return Err(verification_error(
            "GATE-RECEIPT-AUTHORITY-OUTCOMES",
            "authority outcome coverage differs from the planned gates",
        ));
    }
    for outcome in array(receipt, "/authority_outcomes")? {
        verify_authority_outcome(plan_nodes, receipt, outcome)?;
    }
    Ok(())
}

fn verify_authority_outcome(plan_nodes: &[Value], receipt: &Value, outcome: &Value) -> Result<()> {
    let gate_id = string(outcome, "/gate_id")?;
    let authority_class = string(outcome, "/authority_class")?;
    if !authority_adapter_supported(gate_id, authority_class) {
        return Err(verification_error(
            "GATE-RECEIPT-AUTHORITY-UNSUPPORTED",
            gate_id,
        ));
    }
    let nodes = plan_nodes
        .iter()
        .filter(|node| node["gate_definition_id"] == gate_id)
        .collect::<Vec<_>>();
    if nodes.is_empty()
        || nodes
            .iter()
            .any(|node| node["authority_class"] != outcome["authority_class"])
    {
        return Err(verification_error("GATE-RECEIPT-AUTHORITY-CLASS", gate_id));
    }
    let attempts = array(receipt, "/attempts")?;
    let results = nodes
        .iter()
        .map(|node| final_node_result(node, attempts))
        .collect::<Result<Vec<_>>>()?;
    let expected = aggregate_gate_results(&results);
    let outcome_matches = if matches!(expected, "PASS" | "PASS_WITH_RETRY") {
        nodes
            .iter()
            .all(|node| authority_outcome_accepted(node, outcome))
    } else {
        nodes
            .iter()
            .all(|node| truthful_nonpass_authority_outcome(node, outcome, expected))
    };
    if outcome["execution_integrity"] == expected && outcome_matches {
        Ok(())
    } else {
        Err(verification_error(
            "GATE-RECEIPT-AUTHORITY-INTEGRITY",
            gate_id,
        ))
    }
}

fn truthful_nonpass_authority_outcome(node: &Value, outcome: &Value, result: &str) -> bool {
    match node["authority_class"].as_str() {
        Some("NONE") => no_authority_outcome(outcome),
        Some("A0") => {
            outcome["scientific_outcome"].is_null()
                && outcome["admission_outcome"]
                    == match result {
                        "FAIL" => "REJECTED",
                        "BLOCKED" => "BLOCKED",
                        _ => "INVALID",
                    }
        }
        Some(_) => {
            outcome["admission_outcome"].is_null()
                && outcome["scientific_outcome"] == "NOT_EVALUATED"
                && outcome["investigation_record_id"].is_null()
        }
        None => false,
    }
}

fn authority_outcome_accepted(node: &Value, outcome: &Value) -> bool {
    match node["authority_class"].as_str() {
        Some("NONE") => no_authority_outcome(outcome),
        Some("A0") => outcome["admission_outcome"] == "ADMITTED",
        Some(_) if node["outcome_policy"] == "BLOCKING" => blocking_outcome_accepted(outcome),
        Some(_) => advisory_outcome_accepted(outcome),
        None => false,
    }
}

fn no_authority_outcome(outcome: &Value) -> bool {
    outcome["admission_outcome"].is_null() && outcome["scientific_outcome"].is_null()
}

fn blocking_outcome_accepted(outcome: &Value) -> bool {
    outcome["scientific_outcome"] == "CONFORMS"
}

fn advisory_outcome_accepted(outcome: &Value) -> bool {
    match outcome["scientific_outcome"].as_str() {
        Some("CONFORMS") => true,
        Some("DIVERGES" | "INCONCLUSIVE") => !outcome["investigation_record_id"].is_null(),
        _ => false,
    }
}

fn final_node_result<'a>(node: &Value, attempts: &'a [Value]) -> Result<&'a str> {
    let node_id = string(node, "/node_id")?;
    let node_attempts = attempts
        .iter()
        .filter(|attempt| attempt["node_id"] == node_id)
        .collect::<Vec<_>>();
    let last = node_attempts
        .last()
        .ok_or_else(|| verification_error("GATE-RECEIPT-MISSING-ATTEMPT", node_id))?;
    let result = string(last, "/result")?;
    if result == "PASS" && node_attempts.len() > 1 {
        Ok("PASS_WITH_RETRY")
    } else {
        Ok(result)
    }
}

fn aggregate_gate_results(results: &[&str]) -> &'static str {
    for candidate in ["INVALID", "FAIL", "BLOCKED", "PASS_WITH_RETRY"] {
        if results.contains(&candidate) {
            return candidate;
        }
    }
    "PASS"
}

fn verify_artifacts(
    plan_nodes: &[Value],
    receipt: &Value,
    artifacts: &dyn ArtifactProvider,
) -> Result<()> {
    let expected_artifact_paths = plan_nodes
        .iter()
        .flat_map(|node| node["output_paths"].as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .collect::<BTreeSet<_>>();
    let receipt_artifact_paths = array(receipt, "/artifacts")?
        .iter()
        .map(|artifact| string(artifact, "/path"))
        .collect::<std::result::Result<BTreeSet<_>, _>>()?;
    if expected_artifact_paths != receipt_artifact_paths {
        return Err(verification_error(
            "GATE-RECEIPT-ARTIFACT-INVENTORY",
            "artifact paths differ from the plan",
        ));
    }
    for artifact in array(receipt, "/artifacts")? {
        let path = string(artifact, "/path")?;
        let node = plan_nodes
            .iter()
            .find(|node| {
                node["output_paths"]
                    .as_array()
                    .is_some_and(|paths| paths.contains(&Value::String(path.to_owned())))
            })
            .ok_or_else(|| verification_error("GATE-RECEIPT-ARTIFACT-CONTRACT", path))?;
        if artifact["kind"] != expected_artifact_kind(string(node, "/artifact_contract")?, path) {
            return Err(verification_error("GATE-RECEIPT-ARTIFACT-CONTRACT", path));
        }
        if artifact["sha256"] != sha256_bytes(&artifacts.read(path)?) {
            return Err(verification_error("GATE-RECEIPT-ARTIFACT", path));
        }
    }
    Ok(())
}

fn expected_artifact_kind(contract: &str, path: &str) -> &'static str {
    match (
        contract,
        Path::new(path).extension().and_then(|value| value.to_str()),
    ) {
        ("adjudicated-crap-v1", Some("lcov")) => "LCOV",
        ("adjudicated-crap-v1", Some("xml")) | ("nextest-junit-v1", _) => "JUNIT",
        ("adjudicated-crap-v1", _) => "CRAP",
        ("schema-validation-v1", _) => "SCHEMA",
        _ => "LOG",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttestationIdentity {
    pub principal_id: String,
    pub trust_root_id: String,
    pub repository: String,
    pub source_commit: String,
    pub source_ref: String,
    pub workflow: String,
    pub workflow_sha256: String,
    pub job: String,
    pub runner_image_sha256: String,
    pub attempt: u64,
    pub plan_id: String,
    pub execution_key: String,
    pub receipt_id: String,
    pub receipt_sha256: String,
    pub artifacts: BTreeSet<(String, String)>,
}

pub trait AttestationVerifier {
    /// Cryptographically verify an offline attestation bundle.
    ///
    /// # Errors
    ///
    /// Returns a trust error when signature or provenance verification fails.
    fn verify(&self, format: &str, bundle: &[u8]) -> Result<AttestationIdentity>;
}

#[derive(Debug, Clone)]
pub struct TrustedIssuer {
    pub principal_id: String,
    pub trust_root_id: String,
    pub trust_class: String,
    pub policy_generation: u64,
    pub revocation_generation: u64,
    pub revoked: bool,
    pub repository: String,
    pub source_ref: String,
    pub workflow: String,
    pub workflow_sha256: String,
    pub job: String,
    pub runner_image_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvelopeVerdict {
    envelope_id: String,
    envelope_sha256: String,
    receipt_id: String,
    receipt_sha256: String,
    trust_class: String,
    policy_generation: u64,
    identity: AttestationIdentity,
}

impl EnvelopeVerdict {
    #[must_use]
    pub fn envelope_id(&self) -> &str {
        &self.envelope_id
    }

    #[must_use]
    pub fn envelope_sha256(&self) -> &str {
        &self.envelope_sha256
    }

    #[must_use]
    pub fn trust_class(&self) -> &str {
        &self.trust_class
    }
}

/// Verify a nonrecursive envelope against receipt, bundle, and current issuer authority.
///
/// # Errors
///
/// Returns a trust/receipt error for any subject, signature, issuer, or revocation mismatch.
pub fn verify_envelope(
    repo: &Path,
    receipt: &Value,
    envelope: &Value,
    bundle: &[u8],
    verifier: &dyn AttestationVerifier,
    issuer: &TrustedIssuer,
) -> Result<EnvelopeVerdict> {
    validate_document(repo, "attestation-envelope", envelope)?;
    verify_derived_id(envelope, "envelope_id", "GATE-ENVELOPE-ID")?;
    verify_envelope_subject(receipt, envelope)?;
    verify_envelope_provenance(receipt, envelope)?;
    verify_envelope_artifacts(receipt, envelope)?;
    verify_envelope_bundle(envelope, bundle)?;
    verify_issuer_current(issuer)?;
    let identity = verifier.verify(string(envelope, "/signature/format")?, bundle)?;
    verify_attestation_identity(envelope, &identity, issuer)?;
    verify_issuer_authority(receipt, envelope, issuer)?;
    verify_attestation_claims(receipt, envelope, &identity)?;
    Ok(EnvelopeVerdict {
        envelope_id: string(envelope, "/envelope_id")?.to_owned(),
        envelope_sha256: digest(envelope)?,
        receipt_id: string(receipt, "/receipt_id")?.to_owned(),
        receipt_sha256: digest(receipt)?,
        trust_class: issuer.trust_class.clone(),
        policy_generation: issuer.policy_generation,
        identity,
    })
}

fn verify_envelope_subject(receipt: &Value, envelope: &Value) -> Result<()> {
    if envelope["receipt_subject"]
        != json!({
            "kind": "RECEIPT",
            "receipt_id": receipt["receipt_id"],
            "sha256": digest(receipt)?
        })
    {
        return Err(verification_error(
            "GATE-ENVELOPE-SUBJECT",
            "receipt subject mismatch",
        ));
    }
    Ok(())
}

fn verify_envelope_provenance(receipt: &Value, envelope: &Value) -> Result<()> {
    if envelope["provenance"]["plan_id"] != receipt["plan_id"]
        || envelope["provenance"]["execution_key"] != receipt["execution_key"]
    {
        return Err(verification_error(
            "GATE-ENVELOPE-PROVENANCE",
            "plan or execution identity mismatch",
        ));
    }
    Ok(())
}

fn verify_envelope_bundle(envelope: &Value, bundle: &[u8]) -> Result<()> {
    if envelope["signature"]["bundle_sha256"] != sha256_bytes(bundle) {
        return Err(verification_error(
            "GATE-ENVELOPE-BUNDLE",
            "attestation bundle digest mismatch",
        ));
    }
    Ok(())
}

fn verify_issuer_current(issuer: &TrustedIssuer) -> Result<()> {
    if issuer.revoked {
        return Err(verification_error(
            "GATE-ISSUER-REVOKED",
            &issuer.principal_id,
        ));
    }
    Ok(())
}

fn verify_attestation_identity(
    envelope: &Value,
    identity: &AttestationIdentity,
    issuer: &TrustedIssuer,
) -> Result<()> {
    for (actual, expected, code) in [
        (
            identity.principal_id.as_str(),
            issuer.principal_id.as_str(),
            "GATE-ISSUER-PRINCIPAL",
        ),
        (
            identity.trust_root_id.as_str(),
            issuer.trust_root_id.as_str(),
            "GATE-ISSUER-ROOT",
        ),
        (
            identity.repository.as_str(),
            string(envelope, "/provenance/repository")?,
            "GATE-ISSUER-REPOSITORY",
        ),
        (
            identity.source_commit.as_str(),
            string(envelope, "/provenance/source_commit")?,
            "GATE-ISSUER-SOURCE",
        ),
        (
            identity.workflow_sha256.as_str(),
            string(envelope, "/provenance/workflow_sha256")?,
            "GATE-ISSUER-WORKFLOW",
        ),
    ] {
        if actual != expected {
            return Err(verification_error(
                code,
                "verified attestation identity mismatch",
            ));
        }
    }
    Ok(())
}

fn require_string_binding(actual: &Value, expected: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(verification_error(
            "GATE-ISSUER-AUTHORITY",
            "issuer registry mismatch",
        ))
    }
}

fn require_generation_binding(actual: &Value, expected: u64) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(verification_error(
            "GATE-ISSUER-AUTHORITY",
            "issuer registry mismatch",
        ))
    }
}

fn verify_issuer_authority(
    receipt: &Value,
    envelope: &Value,
    issuer: &TrustedIssuer,
) -> Result<()> {
    require_string_binding(&envelope["issuer"]["principal_id"], &issuer.principal_id)?;
    require_string_binding(&envelope["issuer"]["trust_root_id"], &issuer.trust_root_id)?;
    require_string_binding(&envelope["issuer"]["trust_class"], &issuer.trust_class)?;
    require_generation_binding(
        &envelope["issuer"]["policy_generation"],
        issuer.policy_generation,
    )?;
    require_generation_binding(
        &envelope["issuer"]["revocation_generation"],
        issuer.revocation_generation,
    )?;
    require_string_binding(&receipt["claims"]["principal"], &issuer.principal_id)?;
    require_string_binding(&receipt["claims"]["trust_class"], &issuer.trust_class)?;
    require_string_binding(&envelope["provenance"]["repository"], &issuer.repository)?;
    require_string_binding(&envelope["provenance"]["source_ref"], &issuer.source_ref)?;
    require_string_binding(&envelope["provenance"]["workflow"], &issuer.workflow)?;
    require_string_binding(
        &envelope["provenance"]["workflow_sha256"],
        &issuer.workflow_sha256,
    )?;
    require_string_binding(&envelope["provenance"]["job"], &issuer.job)?;
    require_string_binding(
        &envelope["provenance"]["runner_image_sha256"],
        &issuer.runner_image_sha256,
    )
}

fn verify_envelope_artifacts(receipt: &Value, envelope: &Value) -> Result<()> {
    if artifact_subjects(receipt)? == artifact_subjects(envelope)? {
        Ok(())
    } else {
        Err(verification_error(
            "GATE-ENVELOPE-ARTIFACTS",
            "attested artifact subjects differ from the receipt",
        ))
    }
}

fn artifact_subjects(value: &Value) -> Result<BTreeSet<(String, String)>> {
    array(value, "/artifacts")?
        .iter()
        .map(|artifact| {
            Ok((
                string(artifact, "/artifact_id")?.to_owned(),
                string(artifact, "/sha256")?.to_owned(),
            ))
        })
        .collect()
}

fn verify_attestation_claims(
    receipt: &Value,
    envelope: &Value,
    identity: &AttestationIdentity,
) -> Result<()> {
    let receipt_digest = digest(receipt)?;
    for (actual, expected, code) in [
        (
            identity.source_ref.as_str(),
            string(envelope, "/provenance/source_ref")?,
            "GATE-ATTESTATION-REF",
        ),
        (
            identity.workflow.as_str(),
            string(envelope, "/provenance/workflow")?,
            "GATE-ATTESTATION-WORKFLOW",
        ),
        (
            identity.job.as_str(),
            string(envelope, "/provenance/job")?,
            "GATE-ATTESTATION-JOB",
        ),
        (
            identity.runner_image_sha256.as_str(),
            string(envelope, "/provenance/runner_image_sha256")?,
            "GATE-ATTESTATION-RUNNER",
        ),
        (
            identity.plan_id.as_str(),
            string(envelope, "/provenance/plan_id")?,
            "GATE-ATTESTATION-PLAN",
        ),
        (
            identity.execution_key.as_str(),
            string(envelope, "/provenance/execution_key")?,
            "GATE-ATTESTATION-EXECUTION",
        ),
        (
            identity.receipt_id.as_str(),
            string(receipt, "/receipt_id")?,
            "GATE-ATTESTATION-RECEIPT",
        ),
        (
            identity.receipt_sha256.as_str(),
            receipt_digest.as_str(),
            "GATE-ATTESTATION-RECEIPT-DIGEST",
        ),
    ] {
        if actual != expected {
            return Err(verification_error(code, "attestation claim mismatch"));
        }
    }
    verify_receipt_provenance(receipt, envelope, identity)
}

fn verify_receipt_provenance(
    receipt: &Value,
    envelope: &Value,
    identity: &AttestationIdentity,
) -> Result<()> {
    let claims = &receipt["claims"];
    let provenance = &envelope["provenance"];
    let consistent = receipt["source"]["head_commit"] == provenance["source_commit"]
        && claims["repository"] == provenance["repository"]
        && claims["source_ref"] == provenance["source_ref"]
        && claims["workflow"] == provenance["workflow"]
        && claims["job"] == provenance["job"]
        && claims["attempt"] == provenance["attempt"]
        && receipt["environment"]["runner_image_sha256"] == provenance["runner_image_sha256"]
        && identity.attempt == provenance["attempt"]
        && identity.artifacts == artifact_subjects(receipt)?;
    if consistent {
        Ok(())
    } else {
        Err(verification_error(
            "GATE-ATTESTATION-PROVENANCE",
            "receipt, envelope, and verified bundle provenance differ",
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReuseContext<'a> {
    pub target_boundary: &'a str,
    pub target_envelope: &'a EnvelopeVerdict,
}

/// Decide whether verified receipt evidence is reusable for the supplied plan.
///
/// # Errors
///
/// Returns a receipt error when identity, execution, trust, or confinement proof is insufficient.
pub fn verify_reuse(
    plan: &Value,
    receipt: &Value,
    receipt_verdict: &ReceiptVerdict,
    envelope_verdict: &EnvelopeVerdict,
    context: ReuseContext<'_>,
) -> Result<()> {
    verify_reuse_identity(plan, receipt, receipt_verdict, envelope_verdict)?;
    verify_reuse_currency(plan, receipt_verdict, envelope_verdict, context)?;
    for node in array(plan, "/nodes")? {
        verify_node_reuse(node, envelope_verdict, context.target_envelope)?;
    }
    Ok(())
}

fn verify_reuse_identity(
    plan: &Value,
    receipt: &Value,
    receipt_verdict: &ReceiptVerdict,
    envelope_verdict: &EnvelopeVerdict,
) -> Result<()> {
    verify_reuse_document_digests(plan, receipt, receipt_verdict)?;
    verify_reuse_plan_bindings(plan, receipt_verdict)?;
    verify_reuse_envelope_bindings(plan, receipt, receipt_verdict, envelope_verdict)
}

fn verify_reuse_document_digests(
    plan: &Value,
    receipt: &Value,
    receipt_verdict: &ReceiptVerdict,
) -> Result<()> {
    require_reuse_identity(digest(plan)? == receipt_verdict.plan_sha256)?;
    require_reuse_identity(digest(receipt)? == receipt_verdict.receipt_sha256)
}

fn verify_reuse_plan_bindings(plan: &Value, receipt_verdict: &ReceiptVerdict) -> Result<()> {
    require_reuse_identity(receipt_verdict.plan_id == plan["plan_id"])?;
    require_reuse_identity(receipt_verdict.execution_key == plan["execution_key"])?;
    require_reuse_identity(receipt_verdict.roots_sha256 == digest(&plan["environment_roots"])?)?;
    require_reuse_identity(receipt_verdict.boundary == plan["boundary"])
}

fn verify_reuse_envelope_bindings(
    plan: &Value,
    receipt: &Value,
    receipt_verdict: &ReceiptVerdict,
    envelope_verdict: &EnvelopeVerdict,
) -> Result<()> {
    verify_reuse_origin_bindings(receipt_verdict, envelope_verdict)?;
    verify_reuse_receipt_bindings(plan, receipt, receipt_verdict, envelope_verdict)
}

fn verify_reuse_origin_bindings(
    receipt_verdict: &ReceiptVerdict,
    envelope_verdict: &EnvelopeVerdict,
) -> Result<()> {
    require_reuse_identity(envelope_verdict.receipt_sha256 == receipt_verdict.receipt_sha256)?;
    require_reuse_identity(
        envelope_verdict.identity.receipt_sha256 == receipt_verdict.receipt_sha256,
    )?;
    require_reuse_identity(envelope_verdict.identity.plan_id == receipt_verdict.plan_id)?;
    require_reuse_identity(envelope_verdict.identity.execution_key == receipt_verdict.execution_key)
}

fn verify_reuse_receipt_bindings(
    plan: &Value,
    receipt: &Value,
    receipt_verdict: &ReceiptVerdict,
    envelope_verdict: &EnvelopeVerdict,
) -> Result<()> {
    require_reuse_identity(plan["execution_key"] == receipt["execution_key"])?;
    require_reuse_identity(plan["environment_roots"] == receipt["roots"])?;
    require_reuse_identity(receipt_verdict.receipt_id == receipt["receipt_id"])?;
    require_reuse_identity(envelope_verdict.receipt_id == receipt_verdict.receipt_id)
}

fn require_reuse_identity(binding: bool) -> Result<()> {
    if binding {
        Ok(())
    } else {
        Err(verification_error(
            "GATE-REUSE-IDENTITY",
            "execution identity or roots changed",
        ))
    }
}

fn verify_reuse_currency(
    plan: &Value,
    receipt_verdict: &ReceiptVerdict,
    envelope_verdict: &EnvelopeVerdict,
    context: ReuseContext<'_>,
) -> Result<()> {
    require_reuse_currency(matches!(
        receipt_verdict.result.as_str(),
        "PASS" | "PASS_WITH_RETRY"
    ))?;
    require_reuse_currency(
        envelope_verdict.policy_generation == context.target_envelope.policy_generation,
    )?;
    require_reuse_currency(plan["boundary"] == context.target_boundary)
}

fn require_reuse_currency(current: bool) -> Result<()> {
    if current {
        Ok(())
    } else {
        Err(verification_error(
            "GATE-REUSE-CURRENCY",
            "result, policy generation, or target boundary is not current",
        ))
    }
}

fn verify_node_reuse(
    node: &Value,
    envelope: &EnvelopeVerdict,
    target: &EnvelopeVerdict,
) -> Result<()> {
    verify_reuse_trust(node, envelope)?;
    match string(node, "/reuse_class")? {
        "NON_REUSABLE" => Err(verification_error(
            "GATE-REUSE-NONREUSABLE",
            string(node, "/node_id")?,
        )),
        "SAME_EXECUTION" => verify_same_execution(envelope, target),
        "HERMETIC_CONTENT" => Err(verification_error(
            "GATE-REUSE-HERMETIC",
            "v1 has no verifier-issued confinement capability",
        )),
        value => Err(verification_error("GATE-REUSE-CLASS", value)),
    }
}

fn verify_reuse_trust(node: &Value, envelope: &EnvelopeVerdict) -> Result<()> {
    let available = trust_rank(&envelope.trust_class)?;
    let required = trust_rank(string(node, "/trust_requirement")?)?;
    if available < required {
        return Err(verification_error(
            "GATE-REUSE-TRUST",
            string(node, "/node_id")?,
        ));
    }
    Ok(())
}

fn verify_same_execution(envelope: &EnvelopeVerdict, target: &EnvelopeVerdict) -> Result<()> {
    if envelope.envelope_id == target.envelope_id && envelope.identity == target.identity {
        Ok(())
    } else {
        Err(verification_error(
            "GATE-REUSE-EXECUTION",
            "origin and target verified execution identities differ",
        ))
    }
}

fn trust_rank(value: &str) -> Result<u8> {
    match value {
        "LOCAL_UNTRUSTED" => Ok(0),
        "REPOSITORY_REVIEWED" => Ok(1),
        "PROTECTED_CI" => Ok(2),
        _ => Err(verification_error("GATE-REUSE-TRUST", value)),
    }
}

fn validate_document(repo: &Path, stem: &str, value: &Value) -> Result<()> {
    let path = repo.join(format!("gate-policy/v1/schemas/{stem}.schema.json"));
    let bytes = fs::read(&path).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-SCHEMA-READ",
            format!("{}: {error}", path.display()),
        )
    })?;
    validate_schema(&parse_strict(&bytes)?, value, stem)
}

fn verify_derived_id(value: &Value, field: &str, code: &'static str) -> Result<()> {
    if value[field] == derived_id(value, field)? {
        Ok(())
    } else {
        Err(verification_error(code, "derived identity mismatch"))
    }
}

fn equal(
    left: &Value,
    left_pointer: &str,
    right: &Value,
    right_pointer: &str,
    code: &'static str,
) -> Result<()> {
    if left.pointer(left_pointer) == right.pointer(right_pointer) {
        Ok(())
    } else {
        Err(verification_error(
            code,
            format!("{left_pointer} != {right_pointer}"),
        ))
    }
}

fn array<'a>(value: &'a Value, pointer: &str) -> Result<&'a [Value]> {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| verification_error("GATE-RECEIPT-SHAPE", pointer))
}

fn string<'a>(value: &'a Value, pointer: &str) -> Result<&'a str> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| verification_error("GATE-RECEIPT-SHAPE", pointer))
}

fn verification_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Receipt, code, message)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fmt::Write as _;
    use std::sync::OnceLock;
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde_json::{Value, json};

    use super::{
        ArtifactProvider, AttestationIdentity, AttestationVerifier, EnvelopeVerdict,
        ReceiptVerdict, ReuseContext, TrustedIssuer, authority_outcome_accepted,
        expected_executed_inventory, truthful_nonpass_authority_outcome, verify_envelope,
        verify_node_reuse, verify_receipt, verify_reuse,
    };
    use crate::canonical::{derived_id, digest, sha256_bytes};
    use crate::error::{ErrorClass, GatePolicyError, Result};
    use crate::planner::{NextestInventory, PlanRequest, Planner, PlanningStage};

    #[test]
    #[allow(clippy::too_many_lines)]
    fn reuse_fails_closed_for_nonreusable_failed_wrong_trust_and_stale_evidence() {
        let plan = json!({"plan_id": "plan", "execution_key": "a", "environment_roots": {}, "boundary": "INCREMENT", "nodes": [{"node_id": "n", "reuse_class": "NON_REUSABLE", "trust_requirement": "REPOSITORY_REVIEWED"}]});
        let receipt = json!({"receipt_id": "r", "execution_key": "a", "roots": {}});
        let receipt_sha256 = digest(&receipt).expect("receipt digest");
        let receipt_verdict = ReceiptVerdict {
            receipt_id: "r".to_owned(),
            receipt_sha256: receipt_sha256.clone(),
            plan_id: "plan".to_owned(),
            plan_sha256: digest(&plan).expect("plan digest"),
            execution_key: "a".to_owned(),
            roots_sha256: digest(&json!({})).expect("roots digest"),
            boundary: "INCREMENT".to_owned(),
            result: "PASS".to_owned(),
            trust_class: "LOCAL_UNTRUSTED".to_owned(),
            claimed_trust_class: "REPOSITORY_REVIEWED".to_owned(),
        };
        let envelope_verdict = EnvelopeVerdict {
            envelope_id: "e".to_owned(),
            envelope_sha256: "0".repeat(64),
            receipt_id: "r".to_owned(),
            receipt_sha256: receipt_sha256.clone(),
            trust_class: "REPOSITORY_REVIEWED".to_owned(),
            policy_generation: 1,
            identity: AttestationIdentity {
                principal_id: "p".to_owned(),
                trust_root_id: "t".to_owned(),
                repository: "r".to_owned(),
                source_commit: "0".repeat(40),
                source_ref: "refs/heads/main".to_owned(),
                workflow: "w".to_owned(),
                workflow_sha256: "0".repeat(64),
                job: "j".to_owned(),
                runner_image_sha256: "0".repeat(64),
                attempt: 1,
                plan_id: "plan".to_owned(),
                execution_key: "a".to_owned(),
                receipt_id: "r".to_owned(),
                receipt_sha256,
                artifacts: BTreeSet::new(),
            },
        };
        assert!(
            verify_reuse(
                &plan,
                &receipt,
                &receipt_verdict,
                &envelope_verdict,
                ReuseContext {
                    target_boundary: "INCREMENT",
                    target_envelope: &envelope_verdict,
                }
            )
            .is_err()
        );

        let mut node = json!({
            "node_id": "n",
            "reuse_class": "NON_REUSABLE",
            "trust_requirement": "REPOSITORY_REVIEWED"
        });
        let error = verify_node_reuse(&node, &envelope_verdict, &envelope_verdict)
            .expect_err("non-reusable nodes must fail closed");
        assert_eq!(error.code, "GATE-REUSE-NONREUSABLE");

        node["reuse_class"] = json!("SAME_EXECUTION");
        verify_node_reuse(&node, &envelope_verdict, &envelope_verdict)
            .expect("the exact verified execution is reusable");
        let different_target = EnvelopeVerdict {
            envelope_id: "different".to_owned(),
            ..envelope_verdict.clone()
        };
        let error = verify_node_reuse(&node, &envelope_verdict, &different_target)
            .expect_err("different execution identities must fail closed");
        assert_eq!(error.code, "GATE-REUSE-EXECUTION");

        node["reuse_class"] = json!("HERMETIC_CONTENT");
        let error = verify_node_reuse(&node, &envelope_verdict, &envelope_verdict)
            .expect_err("v1 has no hermetic reuse capability");
        assert_eq!(error.code, "GATE-REUSE-HERMETIC");

        node["reuse_class"] = json!("UNKNOWN");
        let error = verify_node_reuse(&node, &envelope_verdict, &envelope_verdict)
            .expect_err("unknown reuse classes must fail closed");
        assert_eq!(error.code, "GATE-REUSE-CLASS");

        let mut reusable_plan = plan;
        reusable_plan["nodes"][0]["reuse_class"] = json!("SAME_EXECUTION");
        let reusable_verdict = ReceiptVerdict {
            plan_sha256: digest(&reusable_plan).expect("reusable plan digest"),
            ..receipt_verdict.clone()
        };
        verify_reuse(
            &reusable_plan,
            &receipt,
            &reusable_verdict,
            &envelope_verdict,
            ReuseContext {
                target_boundary: "INCREMENT",
                target_envelope: &envelope_verdict,
            },
        )
        .expect("exact same-execution evidence is reusable");
        let failed = ReceiptVerdict {
            result: "FAIL".to_owned(),
            ..reusable_verdict.clone()
        };
        assert!(
            verify_reuse(
                &reusable_plan,
                &receipt,
                &failed,
                &envelope_verdict,
                ReuseContext {
                    target_boundary: "INCREMENT",
                    target_envelope: &envelope_verdict,
                }
            )
            .is_err()
        );
        reusable_plan["nodes"][0]["trust_requirement"] = json!("PROTECTED_CI");
        let protected_verdict = ReceiptVerdict {
            plan_sha256: digest(&reusable_plan).expect("protected plan digest"),
            ..receipt_verdict.clone()
        };
        assert!(
            verify_reuse(
                &reusable_plan,
                &receipt,
                &protected_verdict,
                &envelope_verdict,
                ReuseContext {
                    target_boundary: "INCREMENT",
                    target_envelope: &envelope_verdict,
                }
            )
            .is_err()
        );
        reusable_plan["nodes"][0]["trust_requirement"] = json!("REPOSITORY_REVIEWED");
        let current_verdict = ReceiptVerdict {
            plan_sha256: digest(&reusable_plan).expect("current plan digest"),
            ..receipt_verdict
        };
        let stale_envelope = EnvelopeVerdict {
            policy_generation: 2,
            ..envelope_verdict.clone()
        };
        assert!(
            verify_reuse(
                &reusable_plan,
                &receipt,
                &current_verdict,
                &envelope_verdict,
                ReuseContext {
                    target_boundary: "INCREMENT",
                    target_envelope: &stale_envelope,
                }
            )
            .is_err()
        );
    }

    #[test]
    fn authority_outcome_contract_distinguishes_admission_blocking_and_investigation() {
        let mut node = json!({"authority_class": "NONE", "outcome_policy": "BLOCKING"});
        let mut outcome = json!({
            "admission_outcome": null,
            "scientific_outcome": null,
            "investigation_record_id": null
        });
        assert!(authority_outcome_accepted(&node, &outcome));

        node["authority_class"] = json!("A0");
        outcome["admission_outcome"] = json!("ADMITTED");
        assert!(authority_outcome_accepted(&node, &outcome));

        node["authority_class"] = json!("A1");
        outcome["admission_outcome"] = Value::Null;
        outcome["scientific_outcome"] = json!("CONFORMS");
        assert!(authority_outcome_accepted(&node, &outcome));
        outcome["scientific_outcome"] = json!("DIVERGES");
        assert!(!authority_outcome_accepted(&node, &outcome));

        node["outcome_policy"] = json!("INVESTIGATION");
        assert!(!authority_outcome_accepted(&node, &outcome));
        outcome["investigation_record_id"] = json!("investigation");
        assert!(authority_outcome_accepted(&node, &outcome));
        outcome["scientific_outcome"] = json!("NOT_EVALUATED");
        assert!(!authority_outcome_accepted(&node, &outcome));

        node["outcome_policy"] = json!("BLOCKING");
        outcome["investigation_record_id"] = Value::Null;
        assert!(truthful_nonpass_authority_outcome(&node, &outcome, "FAIL"));
        node["authority_class"] = json!("A0");
        outcome["scientific_outcome"] = Value::Null;
        outcome["admission_outcome"] = json!("BLOCKED");
        assert!(truthful_nonpass_authority_outcome(
            &node, &outcome, "BLOCKED"
        ));
    }

    static ARTIFACT_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct MemoryArtifacts {
        bytes: BTreeMap<String, Vec<u8>>,
        workspace: std::path::PathBuf,
    }

    impl MemoryArtifacts {
        fn new(bytes: BTreeMap<String, Vec<u8>>) -> Self {
            let sequence = ARTIFACT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let workspace = std::env::temp_dir().join(format!(
                "openwepp-verifier-artifacts-{}-{sequence}",
                std::process::id()
            ));
            std::fs::create_dir(&workspace).expect("create verifier artifact workspace");
            Self { bytes, workspace }
        }
    }

    impl Drop for MemoryArtifacts {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.workspace)
                .expect("remove precise verifier artifact workspace");
        }
    }

    impl ArtifactProvider for MemoryArtifacts {
        fn read(&self, path: &str) -> Result<Vec<u8>> {
            self.bytes.get(path).cloned().ok_or_else(|| {
                GatePolicyError::new(ErrorClass::Receipt, "TEST-ARTIFACT-MISSING", path)
            })
        }

        fn workspace_root(&self) -> Option<&std::path::Path> {
            Some(&self.workspace)
        }
    }

    struct RootlessArtifacts;

    impl ArtifactProvider for RootlessArtifacts {
        fn read(&self, path: &str) -> Result<Vec<u8>> {
            Err(GatePolicyError::new(
                ErrorClass::Receipt,
                "TEST-ARTIFACT-UNUSED",
                path,
            ))
        }
    }

    #[test]
    fn verifier_rejects_provider_without_reconstruction_root() {
        let error = super::reconstruct_receipt_plan(
            &repo(),
            &json!({}),
            &json!({"source_mutation_check": {"unchanged": true}}),
            &RootlessArtifacts,
        )
        .expect_err("rootless artifact provider must fail closed");
        assert_eq!(error.code, "GATE-RECEIPT-RECONSTRUCTION-ROOT");
    }

    #[test]
    fn verifier_derives_partial_failed_inventory_from_junit_bytes() {
        let observed_id = sha256_bytes(b"rust-suites::fixture\0works");
        let missing_id = sha256_bytes(b"rust-suites::fixture\0missing");
        let node = json!({
            "node_id": "node",
            "gate_definition_id": "nextest-v1",
            "executor": {"kind": "NEXTEST_V1"},
            "artifact_contract": "nextest-junit-v1",
            "expected_inventory": {"ids": [observed_id.clone(), missing_id]},
            "output_paths": ["junit.xml"]
        });
        let attempt = json!({
            "node_id": "node",
            "exit_code": 1,
            "termination_signal": null
        });
        let artifacts = MemoryArtifacts::new(BTreeMap::from([(
            "junit.xml".to_owned(),
            b"<testsuite>\n<testcase classname=\"fixture\" name=\"works\"/>\n</testsuite>\n"
                .to_vec(),
        )]));
        let executed = expected_executed_inventory(&[node], &[attempt], &artifacts)
            .expect("partial executed inventory");
        assert_eq!(executed, BTreeSet::from([observed_id]));
    }

    struct FixedAttestation(AttestationIdentity);

    impl AttestationVerifier for FixedAttestation {
        fn verify(&self, _format: &str, _bundle: &[u8]) -> Result<AttestationIdentity> {
            Ok(self.0.clone())
        }
    }

    fn repo() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    fn collect_junit_cases(value: &Value, prefix: &str, output: &mut Vec<(String, String)>) {
        match value {
            Value::Object(object) => {
                if let Some(testcases) = object.get("testcases").and_then(Value::as_object) {
                    output.extend(
                        testcases
                            .keys()
                            .map(|name| (prefix.to_owned(), name.to_owned())),
                    );
                }
                for (key, child) in object {
                    let next = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}::{key}")
                    };
                    collect_junit_cases(child, &next, output);
                }
            }
            Value::Array(array) => {
                for child in array {
                    collect_junit_cases(child, prefix, output);
                }
            }
            _ => {}
        }
    }

    fn xml_escape(value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('"', "&quot;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\'', "&apos;")
    }

    fn nextest_listing_selectors(node: &Value) -> Vec<Vec<String>> {
        let arguments = node["arguments"].as_array().expect("node arguments");
        let mut packages = Vec::new();
        for pair in arguments.windows(2) {
            let option = pair[0].as_str();
            let value = pair[1].as_str().map(str::to_owned);
            match (option, value) {
                (Some("--test"), Some(target)) => return vec![vec!["--test".to_owned(), target]],
                (Some("-p" | "--package"), Some(package)) => packages.push(package),
                _ => {}
            }
        }
        if packages.is_empty() {
            vec![vec!["--workspace".to_owned()]]
        } else {
            packages
                .into_iter()
                .map(|package| vec!["-p".to_owned(), package])
                .collect()
        }
    }

    fn junit_bytes_for_node(root: &std::path::Path, node: &Value) -> Vec<u8> {
        let mut cases = Vec::new();
        for selector in nextest_listing_selectors(node) {
            let output = crate::repository::neutral_cargo_command()
                .args([
                    "nextest",
                    "list",
                    "--locked",
                    "--offline",
                    "--message-format",
                    "json",
                ])
                .args(selector)
                .current_dir(root)
                .output()
                .expect("list test inventory for JUnit fixture");
            assert!(
                output.status.success(),
                "nextest list must succeed: status={}\nstdout:\n{}\nstderr:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let listing: Value =
                serde_json::from_slice(&output.stdout).expect("nextest listing JSON");
            collect_junit_cases(&listing, "", &mut cases);
        }
        let planned = node["expected_inventory"]["ids"]
            .as_array()
            .expect("planned inventory")
            .iter()
            .map(|item| item.as_str().expect("inventory ID"))
            .collect::<BTreeSet<_>>();
        cases.sort();
        cases.dedup();
        let retained = cases
            .into_iter()
            .filter(|(class, name)| {
                planned.contains(sha256_bytes(format!("{class}\0{name}").as_bytes()).as_str())
            })
            .collect::<Vec<_>>();
        assert_eq!(
            retained.len(),
            planned.len(),
            "JUnit fixture must be exact for {}",
            node["gate_definition_id"]
        );
        let mut xml = String::from("<testsuite>\n");
        for (class, name) in retained {
            let junit_class = class
                .strip_prefix("rust-suites::")
                .expect("Nextest inventory class prefix");
            writeln!(
                xml,
                "<testcase classname=\"{}\" name=\"{}\"/>",
                xml_escape(junit_class),
                xml_escape(&name)
            )
            .expect("write JUnit fixture");
        }
        xml.push_str("</testsuite>\n");
        let parsed = super::junit_inventory_bytes(xml.as_bytes()).expect("parse JUnit fixture");
        assert_eq!(
            parsed.iter().map(String::as_str).collect::<BTreeSet<_>>(),
            planned
        );
        xml.into_bytes()
    }

    #[allow(
        clippy::too_many_lines,
        reason = "the fixture mirrors the closed receipt wire contract field by field"
    )]
    fn normalized_plan_and_receipt() -> (Value, Value, MemoryArtifacts) {
        static PLAN: OnceLock<Value> = OnceLock::new();
        let root = repo();
        let plan = PLAN
            .get_or_init(|| {
                let head = crate::repository::resolve_commit(&root, "HEAD")
                    .expect("resolved fixture head");
                let base = crate::repository::resolve_commit(&root, "HEAD^")
                    .expect("resolved fixture base");
                let source =
                    crate::repository::observe_committed_after_mutation(&root, &base, &head)
                        .expect("observed committed source");
                let authorized_paths = source
                    .changes
                    .iter()
                    .map(|change| change.path.clone())
                    .collect();
                Planner::new(NextestInventory)
                    .build(
                        &root,
                        &PlanRequest {
                            stage: PlanningStage::Intent,
                            predecessor_intent_plan_id: None,
                            boundary: "INCREMENT".to_owned(),
                            campaign_id: Some("TESTGATE-PLAN-01".to_owned()),
                            authorized_paths,
                            source,
                        },
                    )
                    .expect("fixture plan")
            })
            .clone();
        let target = plan["nodes"][0]["matrix"]["target"]
            .as_str()
            .expect("target")
            .to_owned();
        let tools = crate::planner::tool_records(&root).expect("tool records");
        let environment =
            crate::planner::environment_record(&root, &target).expect("environment record");

        let mut receipt: Value = serde_json::from_slice(
            &std::fs::read(root.join("gate-policy/v1/fixtures/valid/gate-receipt.json"))
                .expect("receipt fixture"),
        )
        .expect("receipt JSON");
        receipt["plan_id"] = plan["plan_id"].clone();
        receipt["plan_sha256"] = Value::String(digest(&plan).expect("plan digest"));
        receipt["execution_key"] = plan["execution_key"].clone();
        receipt["boundary"] = plan["boundary"].clone();
        receipt["campaign_id"] = plan["campaign_id"].clone();
        receipt["source"]["base_commit"] = plan["source"]["base_commit"].clone();
        receipt["source"]["head_commit"] = plan["source"]["head_commit"].clone();
        receipt["source"]["dirty_tree_digest"] = plan["source"]["dirty_tree_digest"].clone();
        receipt["roots"] = plan["environment_roots"].clone();
        receipt["tools"] = tools;
        receipt["environment"] = environment;
        receipt["source"]["tree_sha256"] =
            Value::String(digest(&plan["environment_roots"]).expect("tree digest"));
        receipt["dag_sha256"] = Value::String(digest(&plan["nodes"]).expect("DAG digest"));
        let nodes = plan["nodes"].as_array().expect("nodes");
        receipt["dag_nodes"] = Value::Array(
            nodes
                .iter()
                .map(|node| {
                    let mut snapshot = node.clone();
                    snapshot.as_object_mut().expect("node object").insert(
                        "plan_node_sha256".to_owned(),
                        Value::String(digest(node).expect("node digest")),
                    );
                    snapshot
                })
                .collect(),
        );
        let attempt_template = receipt["attempts"][0].clone();
        receipt["attempts"] = Value::Array(
            nodes
                .iter()
                .map(|node| {
                    let mut attempt = attempt_template.clone();
                    attempt["node_id"] = node["node_id"].clone();
                    attempt["arguments"] = node["arguments"].clone();
                    attempt["exit_code"] = node["acceptance"]["expected"].clone();
                    attempt["termination_signal"] = Value::Null;
                    attempt["result"] = json!("PASS");
                    attempt
                })
                .collect(),
        );
        let inventory = nodes
            .iter()
            .flat_map(|node| {
                node["expected_inventory"]["ids"]
                    .as_array()
                    .expect("inventory")
            })
            .map(|id| id.as_str().expect("inventory ID").to_owned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(Value::String)
            .collect::<Vec<_>>();
        receipt["planned_inventory"] = Value::Array(inventory);
        receipt["executed_inventory"] = receipt["planned_inventory"].clone();
        receipt["counts"] = json!({
            "passed": nodes.len(),
            "failed": 0,
            "blocked": 0,
            "skipped": 0,
            "retried": 0
        });
        let final_results = nodes
            .iter()
            .map(|node| {
                (
                    node["node_id"].as_str().expect("node ID").to_owned(),
                    "PASS".to_owned(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        receipt["authority_outcomes"] = Value::Array(
            crate::executor::authority_outcomes(nodes, &final_results).expect("authority outcomes"),
        );
        let mut artifact_bytes = BTreeMap::new();
        let mut receipt_artifacts = Vec::new();
        for (index, node) in nodes.iter().enumerate() {
            for output in node["output_paths"].as_array().expect("outputs") {
                let path = output.as_str().expect("output").to_owned();
                let bytes = if node["gate_definition_id"] == "required-authority-v1" {
                    let suites = node["expected_inventory"]["ids"]
                        .as_array()
                        .expect("authority inventory")
                        .iter()
                        .map(|suite| suite.as_str().expect("suite ID"))
                        .collect::<Vec<_>>()
                        .join(",");
                    format!(
                        "- lane=required failure_class=hard-fail blocking=true test=required-authority suites={suites} status=pass\n"
                    )
                    .into_bytes()
                } else if super::node_has_junit_evidence(node)
                    && std::path::Path::new(&path)
                        .extension()
                        .is_some_and(|extension| extension == "xml")
                {
                    junit_bytes_for_node(&root, node)
                } else {
                    format!("verified artifact {index}: {path}").into_bytes()
                };
                receipt_artifacts.push(json!({
                    "artifact_id": format!("artifact-{index}"),
                    "kind": super::expected_artifact_kind(
                        node["artifact_contract"].as_str().expect("contract"),
                        &path
                    ),
                    "path": path,
                    "sha256": sha256_bytes(&bytes)
                }));
                artifact_bytes.insert(path, bytes);
            }
        }
        receipt["artifacts"] = Value::Array(receipt_artifacts);
        let source_snapshot = digest(&json!({
            "source": plan["source"],
            "roots": plan["environment_roots"]
        }))
        .expect("source snapshot");
        receipt["source_mutation_check"]["before_sha256"] = json!(source_snapshot);
        receipt["source_mutation_check"]["after_sha256"] =
            receipt["source_mutation_check"]["before_sha256"].clone();
        receipt["receipt_id"] =
            Value::String(derived_id(&receipt, "receipt_id").expect("receipt ID"));
        (plan, receipt, MemoryArtifacts::new(artifact_bytes))
    }

    fn set_gate_integrity(receipt: &mut Value, gate_id: &Value, result: &str) {
        let outcomes = receipt["authority_outcomes"]
            .as_array_mut()
            .expect("authority outcomes");
        let outcome = outcomes
            .iter_mut()
            .find(|outcome| outcome["gate_id"] == *gate_id)
            .expect("gate outcome");
        outcome["execution_integrity"] = json!(result);
        match outcome["authority_class"].as_str() {
            Some("A0") => {
                outcome["admission_outcome"] = json!(match result {
                    "FAIL" => "REJECTED",
                    "BLOCKED" => "BLOCKED",
                    "INVALID" => "INVALID",
                    _ => "ADMITTED",
                });
            }
            Some("NONE") | None => {}
            Some(_) if !matches!(result, "PASS" | "PASS_WITH_RETRY") => {
                outcome["scientific_outcome"] = json!("NOT_EVALUATED");
            }
            Some(_) => {
                outcome["scientific_outcome"] = json!("CONFORMS");
            }
        }
    }

    fn refresh_receipt_id(receipt: &mut Value) {
        receipt["receipt_id"] = json!(derived_id(receipt, "receipt_id").expect("receipt identity"));
    }

    #[test]
    fn verifier_accepts_truthful_fail_and_blocked_receipts() {
        let (plan, mut failed, artifacts) = normalized_plan_and_receipt();
        let mut blocked = failed.clone();
        let nodes = plan["nodes"].as_array().expect("nodes");
        let expected = nodes[0]["acceptance"]["expected"]
            .as_i64()
            .expect("expected exit");
        failed["attempts"][0]["exit_code"] = json!(expected + 1);
        failed["attempts"][0]["result"] = json!("FAIL");
        failed["counts"]["passed"] = json!(nodes.len() - 1);
        failed["counts"]["failed"] = json!(1);
        failed["result"] = json!("FAIL");
        set_gate_integrity(&mut failed, &nodes[0]["gate_definition_id"], "FAIL");
        refresh_receipt_id(&mut failed);
        let verdict =
            verify_receipt(&repo(), &plan, &failed, &artifacts).expect("truthful FAIL receipt");
        assert_eq!(verdict.result(), "FAIL");

        let nodes = plan["nodes"].as_array().expect("nodes");
        let blocked_index = nodes.len() - 1;
        blocked["attempts"][blocked_index]["exit_code"] = Value::Null;
        blocked["attempts"][blocked_index]["result"] = json!("BLOCKED");
        let executed = nodes
            .iter()
            .take(blocked_index)
            .flat_map(|node| {
                node["expected_inventory"]["ids"]
                    .as_array()
                    .expect("inventory")
            })
            .map(|item| item.as_str().expect("inventory ID").to_owned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(Value::String)
            .collect::<Vec<_>>();
        let executed_ids = executed
            .iter()
            .map(|item| item.as_str().expect("inventory ID"))
            .collect::<BTreeSet<_>>();
        let unavailable = blocked["planned_inventory"]
            .as_array()
            .expect("planned inventory")
            .iter()
            .filter(|item| !executed_ids.contains(item.as_str().expect("planned inventory ID")))
            .map(|item| {
                json!({
                    "item_id": item,
                    "reason_code": "TIMEOUT",
                    "policy_disposition": "BLOCK"
                })
            })
            .collect::<Vec<_>>();
        blocked["executed_inventory"] = Value::Array(executed);
        blocked["unavailable_items"] = Value::Array(unavailable.clone());
        blocked["counts"]["passed"] = json!(nodes.len() - 1);
        blocked["counts"]["blocked"] = json!(1);
        blocked["counts"]["skipped"] = json!(unavailable.len());
        blocked["result"] = json!("BLOCKED");
        set_gate_integrity(
            &mut blocked,
            &nodes[blocked_index]["gate_definition_id"],
            "BLOCKED",
        );
        refresh_receipt_id(&mut blocked);
        let verdict =
            verify_receipt(&repo(), &plan, &blocked, &artifacts).expect("truthful BLOCKED receipt");
        assert_eq!(verdict.result(), "BLOCKED");
    }

    #[test]
    fn normalized_junit_artifacts_reconstruct_exact_inventory() {
        let (plan, receipt, artifacts) = normalized_plan_and_receipt();
        let observed = expected_executed_inventory(
            plan["nodes"].as_array().expect("nodes"),
            receipt["attempts"].as_array().expect("attempts"),
            &artifacts,
        )
        .expect("artifact-derived inventory");
        let claimed = receipt["executed_inventory"]
            .as_array()
            .expect("executed inventory")
            .iter()
            .map(|item| item.as_str().expect("inventory ID").to_owned())
            .collect::<BTreeSet<_>>();
        assert_eq!(observed, claimed);
    }

    #[test]
    fn receipt_verification_reconstructs_identity_dag_inventory_and_artifacts() {
        let (plan, receipt, artifacts) = normalized_plan_and_receipt();
        let verdict = verify_receipt(&repo(), &plan, &receipt, &artifacts).expect("receipt");
        assert_eq!(verdict.result, "PASS");
        let envelope_verdict = super::verify_receipt_envelope(&repo(), &plan, &receipt, &artifacts)
            .expect("immutable envelope");
        assert_eq!(envelope_verdict.receipt_id, verdict.receipt_id);

        let mut drifted = receipt;
        drifted["executed_inventory"] = json!([]);
        assert!(verify_receipt(&repo(), &plan, &drifted, &artifacts).is_err());

        let (_plan, mut dishonest, artifacts) = normalized_plan_and_receipt();
        dishonest["attempts"][0]["result"] = json!("FAIL");
        dishonest["result"] = json!("FAIL");
        dishonest["counts"]["passed"] = json!(0);
        dishonest["counts"]["failed"] = json!(1);
        dishonest["authority_outcomes"][0]["execution_integrity"] = json!("FAIL");
        dishonest["receipt_id"] = json!(derived_id(&dishonest, "receipt_id").expect("receipt ID"));
        assert!(verify_receipt(&repo(), &plan, &dishonest, &artifacts).is_err());
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn envelope_requires_exact_subject_bundle_current_issuer_and_external_proof() {
        let (_plan, mut receipt, _artifacts) = normalized_plan_and_receipt();
        let root = repo();
        let mut envelope: Value = serde_json::from_slice(
            &std::fs::read(root.join("gate-policy/v1/fixtures/valid/attestation-envelope.json"))
                .expect("envelope fixture"),
        )
        .expect("envelope JSON");
        let bundle = b"offline attestation bundle";
        receipt["source"]["head_commit"] = envelope["provenance"]["source_commit"].clone();
        receipt["claims"]["trust_class"] = envelope["issuer"]["trust_class"].clone();
        receipt["claims"]["principal"] = envelope["issuer"]["principal_id"].clone();
        for field in ["repository", "source_ref", "workflow", "job", "attempt"] {
            receipt["claims"][field] = envelope["provenance"][field].clone();
        }
        receipt["environment"]["runner_image_sha256"] =
            envelope["provenance"]["runner_image_sha256"].clone();
        receipt["receipt_id"] =
            Value::String(derived_id(&receipt, "receipt_id").expect("receipt ID"));
        envelope["receipt_subject"] = json!({
            "kind": "RECEIPT", "receipt_id": receipt["receipt_id"],
            "sha256": digest(&receipt).expect("receipt digest")
        });
        envelope["provenance"]["plan_id"] = receipt["plan_id"].clone();
        envelope["provenance"]["execution_key"] = receipt["execution_key"].clone();
        envelope["artifacts"] = Value::Array(
            receipt["artifacts"]
                .as_array()
                .expect("artifacts")
                .iter()
                .map(|artifact| {
                    json!({
                        "kind": "ARTIFACT",
                        "artifact_id": artifact["artifact_id"],
                        "sha256": artifact["sha256"]
                    })
                })
                .collect(),
        );
        envelope["signature"]["bundle_sha256"] = Value::String(sha256_bytes(bundle));
        envelope["envelope_id"] =
            Value::String(derived_id(&envelope, "envelope_id").expect("envelope ID"));
        let identity = AttestationIdentity {
            principal_id: "evidence-publisher".to_owned(),
            trust_root_id: "github-attestations".to_owned(),
            repository: "rogerlew/openWEPP".to_owned(),
            source_commit: envelope["provenance"]["source_commit"]
                .as_str()
                .expect("commit")
                .to_owned(),
            source_ref: envelope["provenance"]["source_ref"]
                .as_str()
                .expect("source ref")
                .to_owned(),
            workflow: envelope["provenance"]["workflow"]
                .as_str()
                .expect("workflow")
                .to_owned(),
            workflow_sha256: envelope["provenance"]["workflow_sha256"]
                .as_str()
                .expect("workflow")
                .to_owned(),
            job: envelope["provenance"]["job"]
                .as_str()
                .expect("job")
                .to_owned(),
            runner_image_sha256: envelope["provenance"]["runner_image_sha256"]
                .as_str()
                .expect("runner")
                .to_owned(),
            attempt: envelope["provenance"]["attempt"].as_u64().expect("attempt"),
            plan_id: envelope["provenance"]["plan_id"]
                .as_str()
                .expect("plan")
                .to_owned(),
            execution_key: envelope["provenance"]["execution_key"]
                .as_str()
                .expect("execution")
                .to_owned(),
            receipt_id: receipt["receipt_id"].as_str().expect("receipt").to_owned(),
            receipt_sha256: digest(&receipt).expect("receipt digest"),
            artifacts: super::artifact_subjects(&receipt).expect("artifact subjects"),
        };
        let mut issuer = TrustedIssuer {
            principal_id: "evidence-publisher".to_owned(),
            trust_root_id: "github-attestations".to_owned(),
            trust_class: "PROTECTED_CI".to_owned(),
            policy_generation: 1,
            revocation_generation: 1,
            revoked: false,
            repository: envelope["provenance"]["repository"]
                .as_str()
                .expect("repository")
                .to_owned(),
            source_ref: envelope["provenance"]["source_ref"]
                .as_str()
                .expect("source ref")
                .to_owned(),
            workflow: envelope["provenance"]["workflow"]
                .as_str()
                .expect("workflow")
                .to_owned(),
            workflow_sha256: envelope["provenance"]["workflow_sha256"]
                .as_str()
                .expect("workflow digest")
                .to_owned(),
            job: envelope["provenance"]["job"]
                .as_str()
                .expect("job")
                .to_owned(),
            runner_image_sha256: envelope["provenance"]["runner_image_sha256"]
                .as_str()
                .expect("runner")
                .to_owned(),
        };
        verify_envelope(
            &root,
            &receipt,
            &envelope,
            bundle,
            &FixedAttestation(identity.clone()),
            &issuer,
        )
        .expect("verified envelope");
        issuer.revoked = true;
        assert!(
            verify_envelope(
                &root,
                &receipt,
                &envelope,
                bundle,
                &FixedAttestation(identity),
                &issuer,
            )
            .is_err()
        );
    }
}
