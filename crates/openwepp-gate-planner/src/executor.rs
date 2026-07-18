//! Fail-closed, shell-free execution of independently reconstructed gate plans.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::canonical::{derived_id, digest, parse_strict, sha256_bytes, validate_schema};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::planner::{
    environment_record, inventory_for_node, manifest_roots, reconstruct_plan, tool_records,
    verify_plan_identity,
};

/// Provenance labels for a local, unsigned execution receipt.
#[derive(Debug, Clone)]
pub struct ExecutionClaims {
    pub principal: String,
    pub repository: String,
    pub source_event: String,
    pub source_ref: String,
    pub workflow: String,
    pub job: String,
    pub runner: String,
    pub attempt: u64,
}

impl Default for ExecutionClaims {
    fn default() -> Self {
        Self {
            principal: "developer".to_owned(),
            repository: "rogerlew/openWEPP".to_owned(),
            source_event: "local".to_owned(),
            source_ref: "refs/heads/main".to_owned(),
            workflow: "local-shell".to_owned(),
            job: "testgate-shadow".to_owned(),
            runner: "local".to_owned(),
            attempt: 1,
        }
    }
}

#[derive(Debug)]
struct NodeRun {
    attempt: Value,
    result: String,
    log_path: PathBuf,
    artifact_source: Option<PathBuf>,
}

/// Execute a terminal plan and construct an unsigned local receipt.
///
/// The plan is schema-checked, identity-checked, and independently
/// reconstructed before the first process starts. Outputs are confined to an
/// external artifact root. The returned receipt still requires the independent
/// receipt verifier before use.
///
/// # Errors
///
/// Returns a typed execution error for an invalid or stale plan, unsupported
/// executor, inventory drift, path/environment escape, output collision,
/// process I/O failure, or source mutation.
pub fn execute_plan(
    repo: &Path,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
) -> Result<Value> {
    let repository = canonical_directory(repo, "GATE-EXEC-REPOSITORY")?;
    let artifacts = canonical_directory(artifact_root, "GATE-EXEC-ARTIFACT-ROOT")?;
    if artifacts.starts_with(&repository) {
        return Err(execution_error(
            "GATE-EXEC-ARTIFACT-IN-REPOSITORY",
            "artifact root must be outside the repository",
        ));
    }
    validate_plan(&repository, plan)?;
    verify_execution_checkout(&repository, plan)?;
    preflight(&repository, &artifacts, plan)?;

    let started_at = timestamp()?;
    let source_snapshot = source_snapshot(plan)?;
    let roots_before = current_roots(&repository, plan)?;
    if roots_before != plan["environment_roots"] {
        return Err(execution_error(
            "GATE-EXEC-SOURCE-DRIFT",
            "execution roots differ from the verified plan",
        ));
    }

    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-PLAN-SHAPE", "nodes must be an array"))?;
    let mut final_results = BTreeMap::<String, String>::new();
    let mut attempts = Vec::new();
    for node in nodes {
        let run = execute_node(&repository, &artifacts, node, &final_results)?;
        let node_id = required_string(node, "node_id")?.to_owned();
        final_results.insert(node_id, run.result.clone());
        write_node_artifacts(&repository, &artifacts, node, &run)?;
        attempts.push(run.attempt);
    }
    if final_results.values().any(|result| result != "PASS") {
        return Err(execution_error(
            "GATE-EXEC-NONPASS",
            "one or more planned nodes did not pass; no receipt was issued",
        ));
    }

    let roots_after = current_roots(&repository, plan)?;
    if roots_after != roots_before {
        return Err(execution_error(
            "GATE-EXEC-SOURCE-MUTATION",
            "repository authority changed during execution",
        ));
    }
    let finished_at = timestamp()?;
    build_receipt(
        &repository,
        plan,
        &artifacts,
        &attempts,
        &final_results,
        &started_at,
        &finished_at,
        &source_snapshot,
        claims,
    )
}

fn validate_plan(repo: &Path, plan: &Value) -> Result<()> {
    let schema = read_json(&repo.join("gate-policy/v1/schemas/gate-plan.schema.json"))?;
    validate_schema(&schema, plan, "executor gate plan")?;
    verify_plan_identity(plan)?;
    let reconstructed = reconstruct_plan(repo, plan)?;
    if digest(&reconstructed)? != digest(plan)? {
        return Err(execution_error(
            "GATE-EXEC-PLAN-RECONSTRUCTION",
            "current policy and source do not reconstruct the supplied plan",
        ));
    }
    if plan["planning_stage"] != "TERMINAL" {
        return Err(execution_error(
            "GATE-EXEC-NONTERMINAL",
            "only a terminal plan may execute",
        ));
    }
    Ok(())
}

fn verify_execution_checkout(repo: &Path, plan: &Value) -> Result<()> {
    let Some(planned_head) = plan["source"]["head_commit"].as_str() else {
        return Ok(());
    };
    let head = git_text(repo, &["rev-parse", "--verify", "HEAD^{commit}"])?;
    if head.trim() != planned_head {
        return Err(execution_error(
            "GATE-EXEC-CHECKOUT-HEAD",
            format!("planned {planned_head}, observed {}", head.trim()),
        ));
    }
    let status = git_bytes(
        repo,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    if status.is_empty() {
        Ok(())
    } else {
        Err(execution_error(
            "GATE-EXEC-CHECKOUT-DIRTY",
            "committed plans require an exact clean checkout",
        ))
    }
}

fn preflight(repo: &Path, artifact_root: &Path, plan: &Value) -> Result<()> {
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-PLAN-SHAPE", "nodes must be an array"))?;
    let mut outputs = BTreeSet::new();
    for node in nodes {
        supported_executor(node)?;
        if node["retry"]["maximum_attempts"] != 1 {
            return Err(execution_error(
                "GATE-EXEC-RETRY-UNSUPPORTED",
                "v1 executor admits exactly one attempt",
            ));
        }
        confined_working_directory(repo, node)?;
        allowed_environment(node)?;
        let current_inventory = inventory_for_node(repo, node)?;
        let expected = string_array(&node["expected_inventory"]["ids"], "inventory")?;
        if current_inventory != expected {
            return Err(execution_error(
                "GATE-EXEC-INVENTORY-DRIFT",
                required_string(node, "gate_definition_id")?,
            ));
        }
        for path in string_array(&node["output_paths"], "output_paths")? {
            if !outputs.insert(path.clone()) {
                return Err(execution_error("GATE-EXEC-OUTPUT-COLLISION", path));
            }
            let destination = confined_output_path(artifact_root, &path)?;
            if fs::symlink_metadata(&destination).is_ok() {
                return Err(execution_error("GATE-EXEC-OUTPUT-COLLISION", path));
            }
        }
    }
    Ok(())
}

fn supported_executor(node: &Value) -> Result<()> {
    match required_string(&node["executor"], "kind")? {
        "PROCESS_V1" | "NEXTEST_V1" | "SCHEMA_V1" | "LEGACY_ADAPTER_V1" => Ok(()),
        kind => Err(execution_error("GATE-EXEC-UNSUPPORTED", kind)),
    }
}

fn confined_working_directory(repo: &Path, node: &Value) -> Result<PathBuf> {
    let relative = Path::new(required_string(node, "working_directory")?);
    require_relative_path(relative, true)?;
    let directory = fs::canonicalize(repo.join(relative))
        .map_err(|error| execution_error("GATE-EXEC-WORKING-DIRECTORY", error.to_string()))?;
    if !directory.starts_with(repo) || !directory.is_dir() {
        return Err(execution_error(
            "GATE-EXEC-WORKING-DIRECTORY",
            relative.display().to_string(),
        ));
    }
    Ok(directory)
}

fn allowed_environment(node: &Value) -> Result<BTreeMap<String, String>> {
    let mut environment = BTreeMap::new();
    for key in string_array(&node["environment_allowlist"], "environment_allowlist")? {
        let value = std::env::var(&key)
            .map_err(|_| execution_error("GATE-EXEC-ENVIRONMENT-MISSING", key.clone()))?;
        environment.insert(key, value);
    }
    Ok(environment)
}

fn execute_node(
    repo: &Path,
    artifact_root: &Path,
    node: &Value,
    final_results: &BTreeMap<String, String>,
) -> Result<NodeRun> {
    let node_id = required_string(node, "node_id")?;
    let started_at = timestamp()?;
    let log_path = attempt_log_path(artifact_root, node_id)?;
    let prerequisite_failed = string_array(&node["prerequisites"], "prerequisites")?
        .iter()
        .any(|id| final_results.get(id).is_none_or(|result| result != "PASS"));
    let artifact_source = if prerequisite_failed {
        None
    } else {
        prepare_real_artifact(repo, node)?
    };
    let (exit_code, result) = if prerequisite_failed {
        File::create(&log_path)
            .map_err(|error| execution_error("GATE-EXEC-LOG-CREATE", error.to_string()))?;
        (None, "BLOCKED".to_owned())
    } else {
        run_process(repo, node, &log_path)?
    };
    if result == "PASS" {
        validate_success_artifact(node, artifact_source.as_deref())?;
    }
    let finished_at = timestamp()?;
    Ok(NodeRun {
        attempt: json!({
            "node_id": node_id,
            "attempt": 1,
            "arguments": node["arguments"],
            "started_at": started_at,
            "finished_at": finished_at,
            "exit_code": exit_code,
            "result": result,
            "retry_reason": null
        }),
        result,
        log_path,
        artifact_source,
    })
}

fn validate_success_artifact(node: &Value, source: Option<&Path>) -> Result<()> {
    match required_string(node, "artifact_contract")? {
        "nextest-junit-v1" => {
            let source = source.ok_or_else(|| {
                execution_error("GATE-EXEC-REAL-ARTIFACT-MISSING", "nextest JUnit")
            })?;
            let actual = junit_inventory(source)?;
            let expected = string_array(&node["expected_inventory"]["ids"], "inventory")?
                .into_iter()
                .collect::<BTreeSet<_>>();
            if actual == expected {
                Ok(())
            } else {
                Err(execution_error(
                    "GATE-EXEC-JUNIT-INVENTORY",
                    format!("expected {}, observed {}", expected.len(), actual.len()),
                ))
            }
        }
        "adjudicated-crap-v1" => {
            let source = source
                .ok_or_else(|| execution_error("GATE-EXEC-REAL-ARTIFACT-MISSING", "CRAP report"))?;
            let report = read_json(source)?;
            if report["status"] == "PASS" {
                Ok(())
            } else {
                Err(execution_error(
                    "GATE-EXEC-CRAP-REPORT",
                    "adapter exited successfully without a PASS report",
                ))
            }
        }
        _ => Ok(()),
    }
}

fn junit_inventory(path: &Path) -> Result<BTreeSet<String>> {
    let text = fs::read_to_string(path)
        .map_err(|error| execution_error("GATE-EXEC-JUNIT-READ", error.to_string()))?;
    let mut inventory = BTreeSet::new();
    for line in text
        .lines()
        .filter(|line| line.trim_start().starts_with("<testcase "))
    {
        let name = xml_attribute(line, "name")?;
        let class = xml_attribute(line, "classname")?;
        inventory.insert(sha256_bytes(
            format!("rust-suites::{class}\0{name}").as_bytes(),
        ));
    }
    if inventory.is_empty() {
        Err(execution_error(
            "GATE-EXEC-JUNIT-EMPTY",
            path.display().to_string(),
        ))
    } else {
        Ok(inventory)
    }
}

fn xml_attribute(line: &str, name: &str) -> Result<String> {
    let marker = format!("{name}=\"");
    let value = line
        .split_once(&marker)
        .and_then(|(_, tail)| tail.split_once('"').map(|(value, _)| value))
        .ok_or_else(|| execution_error("GATE-EXEC-JUNIT-SHAPE", name))?;
    Ok(value
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&"))
}

fn prepare_real_artifact(repo: &Path, node: &Value) -> Result<Option<PathBuf>> {
    let source = match required_string(node, "artifact_contract")? {
        "nextest-junit-v1" => Some(nextest_junit_path(repo, node)?),
        "adjudicated-crap-v1" => Some(adjudicated_crap_report_path(repo, node)?),
        _ => None,
    };
    if let Some(path) = &source {
        match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(execution_error(
                    "GATE-EXEC-REAL-ARTIFACT-SYMLINK",
                    path.display().to_string(),
                ));
            }
            Ok(metadata) if metadata.is_file() => fs::remove_file(path).map_err(|error| {
                execution_error("GATE-EXEC-REAL-ARTIFACT-RESET", error.to_string())
            })?,
            Ok(_) => {
                return Err(execution_error(
                    "GATE-EXEC-REAL-ARTIFACT-TYPE",
                    path.display().to_string(),
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(execution_error(
                    "GATE-EXEC-REAL-ARTIFACT-METADATA",
                    error.to_string(),
                ));
            }
        }
    }
    Ok(source)
}

fn nextest_junit_path(repo: &Path, node: &Value) -> Result<PathBuf> {
    let arguments = string_array(&node["arguments"], "arguments")?;
    let profile = arguments
        .windows(2)
        .find(|pair| pair[0] == "--profile")
        .map_or("default", |pair| pair[1].as_str());
    require_identifier(profile, "GATE-EXEC-NEXTEST-PROFILE")?;
    Ok(repo.join("target/nextest").join(profile).join("junit.xml"))
}

fn adjudicated_crap_report_path(repo: &Path, node: &Value) -> Result<PathBuf> {
    let arguments = string_array(&node["arguments"], "arguments")?;
    let output = arguments
        .windows(2)
        .find(|pair| pair[0] == "--output-dir")
        .map_or("target/adjudicated-crap", |pair| pair[1].as_str());
    let relative = Path::new(output);
    require_relative_path(relative, false)?;
    Ok(repo.join(relative).join("adjudicated-crap-report.json"))
}

fn run_process(repo: &Path, node: &Value, log_path: &Path) -> Result<(Option<i32>, String)> {
    let arguments = string_array(&node["arguments"], "arguments")?;
    let program = arguments
        .first()
        .ok_or_else(|| execution_error("GATE-EXEC-ARGUMENTS", "missing executable"))?;
    let log = File::create(log_path)
        .map_err(|error| execution_error("GATE-EXEC-LOG-CREATE", error.to_string()))?;
    let stderr = log
        .try_clone()
        .map_err(|error| execution_error("GATE-EXEC-LOG-CLONE", error.to_string()))?;
    let environment = allowed_environment(node)?;
    let mut command = Command::new(program);
    command
        .args(&arguments[1..])
        .current_dir(confined_working_directory(repo, node)?)
        .env_clear()
        .envs(environment)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(stderr));
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            fs::write(log_path, format!("process spawn failed: {error}\n")).map_err(
                |write_error| execution_error("GATE-EXEC-LOG-WRITE", write_error.to_string()),
            )?;
            return Ok((None, "BLOCKED".to_owned()));
        }
    };
    let timeout = required_u64(node, "timeout_seconds")?;
    let status = wait_with_timeout(&mut child, Duration::from_secs(timeout))?;
    Ok(match status {
        Some(status) => {
            let code = status.code();
            let expected = node["acceptance"]["expected"]
                .as_i64()
                .ok_or_else(|| execution_error("GATE-EXEC-ACCEPTANCE", "expected exit code"))?;
            let result = if code.map(i64::from) == Some(expected) {
                "PASS"
            } else {
                "FAIL"
            };
            (code, result.to_owned())
        }
        None => (None, "BLOCKED".to_owned()),
    })
}

fn wait_with_timeout(
    child: &mut std::process::Child,
    timeout: Duration,
) -> Result<Option<ExitStatus>> {
    let started = Instant::now();
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| execution_error("GATE-EXEC-WAIT", error.to_string()))?
        {
            return Ok(Some(status));
        }
        if started.elapsed() >= timeout {
            kill_process_tree(child)?;
            child
                .wait()
                .map_err(|error| execution_error("GATE-EXEC-WAIT", error.to_string()))?;
            return Ok(None);
        }
        thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(unix)]
fn kill_process_tree(child: &mut std::process::Child) -> Result<()> {
    use rustix::process::{Pid, Signal, kill_process_group};

    let raw = i32::try_from(child.id())
        .map_err(|error| execution_error("GATE-EXEC-PID", error.to_string()))?;
    let pid = Pid::from_raw(raw)
        .ok_or_else(|| execution_error("GATE-EXEC-PID", "child process ID is zero"))?;
    kill_process_group(pid, Signal::KILL)
        .map_err(|error| execution_error("GATE-EXEC-KILL-GROUP", error.to_string()))
}

#[cfg(not(unix))]
fn kill_process_tree(child: &mut std::process::Child) -> Result<()> {
    child
        .kill()
        .map_err(|error| execution_error("GATE-EXEC-KILL", error.to_string()))
}

#[allow(
    clippy::too_many_arguments,
    reason = "receipt fields mirror the v1 wire contract"
)]
fn build_receipt(
    repo: &Path,
    plan: &Value,
    artifact_root: &Path,
    attempts: &[Value],
    final_results: &BTreeMap<String, String>,
    started_at: &str,
    finished_at: &str,
    source_snapshot: &str,
    claims: &ExecutionClaims,
) -> Result<Value> {
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-PLAN-SHAPE", "nodes must be an array"))?;
    let planned_inventory = nodes
        .iter()
        .flat_map(|node| {
            node["expected_inventory"]["ids"]
                .as_array()
                .into_iter()
                .flatten()
        })
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let dag_nodes = nodes
        .iter()
        .map(|node| {
            let mut snapshot = node.clone();
            snapshot
                .as_object_mut()
                .ok_or_else(|| execution_error("GATE-EXEC-PLAN-SHAPE", "node"))?
                .insert("plan_node_sha256".to_owned(), Value::String(digest(node)?));
            Ok(snapshot)
        })
        .collect::<Result<Vec<_>>>()?;
    let artifacts = receipt_artifacts(artifact_root, nodes)?;
    let authority_outcomes = authority_outcomes(nodes, final_results)?;
    let (passed, failed, blocked) = result_counts(final_results);
    let result = aggregate_result(passed, failed, blocked);
    let tools = tool_records(repo)?;
    let target = nodes
        .first()
        .and_then(|node| node["matrix"]["target"].as_str())
        .unwrap_or("x86_64-unknown-linux-gnu");
    let environment = environment_record(repo, target)?;
    let mut receipt = json!({
        "schema_version": "openwepp-gate-receipt-v1",
        "receipt_id": "0000000000000000000000000000000000000000000000000000000000000000",
        "plan_id": plan["plan_id"],
        "plan_sha256": digest(plan)?,
        "execution_key": plan["execution_key"],
        "boundary": plan["boundary"],
        "campaign_id": plan["campaign_id"],
        "source": {
            "base_commit": plan["source"]["base_commit"],
            "head_commit": plan["source"]["head_commit"],
            "tree_sha256": digest(&plan["environment_roots"])? ,
            "dirty_tree_digest": plan["source"]["dirty_tree_digest"]
        },
        "roots": plan["environment_roots"],
        "dag_sha256": digest(&plan["nodes"])? ,
        "dag_nodes": dag_nodes,
        "zero_work": nodes.is_empty(),
        "attempts": attempts,
        "planned_inventory": planned_inventory,
        "executed_inventory": planned_inventory,
        "tools": tools,
        "environment": environment,
        "started_at": started_at,
        "finished_at": finished_at,
        "counts": {"passed": passed, "failed": failed, "skipped": 0, "blocked": blocked, "retried": 0},
        "authority_outcomes": authority_outcomes,
        "artifacts": artifacts,
        "unavailable_items": [],
        "source_mutation_check": {
            "required": true,
            "before_sha256": source_snapshot,
            "after_sha256": source_snapshot,
            "unchanged": true
        },
        "result": result,
        "claims": {
            "trust_class": "LOCAL_UNTRUSTED",
            "principal": claims.principal,
            "repository": claims.repository,
            "source_event": claims.source_event,
            "source_ref": claims.source_ref,
            "workflow": claims.workflow,
            "job": claims.job,
            "runner": claims.runner,
            "attempt": claims.attempt
        }
    });
    receipt["receipt_id"] = Value::String(derived_id(&receipt, "receipt_id")?);
    let schema = read_json(&repo.join("gate-policy/v1/schemas/gate-receipt.schema.json"))?;
    validate_schema(&schema, &receipt, "executor receipt")?;
    Ok(receipt)
}

fn authority_outcomes(
    nodes: &[Value],
    final_results: &BTreeMap<String, String>,
) -> Result<Vec<Value>> {
    let mut gates = BTreeMap::<String, (String, Vec<String>)>::new();
    for node in nodes {
        let gate = required_string(node, "gate_definition_id")?.to_owned();
        let authority = required_string(node, "authority_class")?.to_owned();
        let result = final_results
            .get(required_string(node, "node_id")?)
            .cloned()
            .ok_or_else(|| execution_error("GATE-EXEC-RESULT-MISSING", gate.clone()))?;
        let entry = gates.entry(gate).or_insert((authority.clone(), Vec::new()));
        if entry.0 != authority {
            return Err(execution_error(
                "GATE-EXEC-AUTHORITY-MISMATCH",
                "one gate has multiple authority classes",
            ));
        }
        entry.1.push(result);
    }
    gates
        .into_iter()
        .map(|(gate_id, (authority_class, results))| {
            if authority_class != "NONE" {
                return Err(execution_error("GATE-EXEC-AUTHORITY-UNSUPPORTED", gate_id));
            }
            Ok(json!({
                "gate_id": gate_id,
                "authority_class": authority_class,
                "execution_integrity": aggregate_node_results(&results),
                "admission_outcome": null,
                "scientific_outcome": null,
                "outcome_policy_generation": 1,
                "investigation_record_id": null
            }))
        })
        .collect()
}

fn aggregate_node_results(results: &[String]) -> &'static str {
    for candidate in ["INVALID", "FAIL", "BLOCKED"] {
        if results.iter().any(|result| result == candidate) {
            return candidate;
        }
    }
    "PASS"
}

fn receipt_artifacts(artifact_root: &Path, nodes: &[Value]) -> Result<Vec<Value>> {
    let mut artifacts = Vec::new();
    for node in nodes {
        let kind = artifact_kind(required_string(node, "artifact_contract")?);
        for path in string_array(&node["output_paths"], "output_paths")? {
            let bytes = fs::read(confined_output_path(artifact_root, &path)?).map_err(|error| {
                execution_error("GATE-EXEC-ARTIFACT-READ", format!("{path}: {error}"))
            })?;
            artifacts.push(json!({
                "artifact_id": format!("artifact-{}", artifacts.len() + 1),
                "kind": kind,
                "path": path,
                "sha256": sha256_bytes(&bytes)
            }));
        }
    }
    Ok(artifacts)
}

fn artifact_kind(contract: &str) -> &'static str {
    match contract {
        "nextest-junit-v1" => "JUNIT",
        "adjudicated-crap-v1" => "CRAP",
        "schema-validation-v1" => "SCHEMA",
        _ => "LOG",
    }
}

fn write_node_artifacts(
    repo: &Path,
    artifact_root: &Path,
    node: &Value,
    run: &NodeRun,
) -> Result<()> {
    let log_sha256 = fs::read(&run.log_path)
        .map(|bytes| sha256_bytes(&bytes))
        .map_err(|error| execution_error("GATE-EXEC-LOG-READ", error.to_string()))?;
    for path in string_array(&node["output_paths"], "output_paths")? {
        let destination = confined_output_path(artifact_root, &path)?;
        let parent = destination
            .parent()
            .ok_or_else(|| execution_error("GATE-EXEC-OUTPUT-PATH", path.clone()))?;
        create_confined_directories(artifact_root, parent)?;
        let bytes = artifact_bytes(repo, artifact_root, node, run, &log_sha256)?;
        write_atomic(&destination, &bytes)?;
    }
    Ok(())
}

fn write_atomic(destination: &Path, bytes: &[u8]) -> Result<()> {
    let parent = destination.parent().ok_or_else(|| {
        execution_error("GATE-EXEC-ARTIFACT-PATH", destination.display().to_string())
    })?;
    let name = destination.file_name().ok_or_else(|| {
        execution_error("GATE-EXEC-ARTIFACT-PATH", destination.display().to_string())
    })?;
    let temporary = parent.join(format!(
        ".{}.tmp-{}",
        name.to_string_lossy(),
        std::process::id()
    ));
    let mut file = File::options()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|error| execution_error("GATE-EXEC-ARTIFACT-CREATE", error.to_string()))?;
    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
        let _ = fs::remove_file(&temporary);
        return Err(execution_error(
            "GATE-EXEC-ARTIFACT-WRITE",
            error.to_string(),
        ));
    }
    drop(file);
    fs::rename(&temporary, destination).map_err(|error| {
        let _ = fs::remove_file(&temporary);
        execution_error("GATE-EXEC-ARTIFACT-RENAME", error.to_string())
    })?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| execution_error("GATE-EXEC-ARTIFACT-SYNC", error.to_string()))
}

fn artifact_bytes(
    repo: &Path,
    artifact_root: &Path,
    node: &Value,
    run: &NodeRun,
    log_sha256: &str,
) -> Result<Vec<u8>> {
    if let Some(source) = &run.artifact_source {
        match fs::symlink_metadata(source) {
            Ok(_) => return read_real_artifact(repo, artifact_root, source),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && run.result != "PASS" => {}
            Err(error) => {
                return Err(execution_error(
                    "GATE-EXEC-REAL-ARTIFACT-MISSING",
                    format!("{}: {error}", source.display()),
                ));
            }
        }
    }
    if required_string(node, "artifact_contract")? == "nextest-junit-v1" {
        let failed = usize::from(run.result != "PASS");
        return Ok(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><testsuite name=\"openwepp-gate\" tests=\"1\" failures=\"{failed}\"><testcase name=\"{}\"/></testsuite>\n",
            required_string(node, "node_id")?
        )
        .into_bytes());
    }
    serde_json::to_vec(&json!({
        "schema_version": "openwepp-gate-process-artifact-v1",
        "node_id": node["node_id"],
        "gate_definition_id": node["gate_definition_id"],
        "result": run.result,
        "attempt_log_sha256": log_sha256
    }))
    .map_err(|error| execution_error("GATE-EXEC-ARTIFACT-SERIALIZE", error.to_string()))
}

fn read_real_artifact(repo: &Path, artifact_root: &Path, source: &Path) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(source).map_err(|error| {
        execution_error(
            "GATE-EXEC-REAL-ARTIFACT-MISSING",
            format!("{}: {error}", source.display()),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(execution_error(
            "GATE-EXEC-REAL-ARTIFACT-TYPE",
            source.display().to_string(),
        ));
    }
    let canonical = fs::canonicalize(source)
        .map_err(|error| execution_error("GATE-EXEC-REAL-ARTIFACT-PATH", error.to_string()))?;
    let target = fs::canonicalize(repo.join("target"))
        .map_err(|error| execution_error("GATE-EXEC-REAL-ARTIFACT-PATH", error.to_string()))?;
    if !canonical.starts_with(target) || canonical.starts_with(artifact_root) {
        return Err(execution_error(
            "GATE-EXEC-REAL-ARTIFACT-ESCAPE",
            source.display().to_string(),
        ));
    }
    fs::read(canonical)
        .map_err(|error| execution_error("GATE-EXEC-REAL-ARTIFACT-READ", error.to_string()))
}

fn attempt_log_path(root: &Path, node_id: &str) -> Result<PathBuf> {
    let directory = root.join(".attempts");
    create_confined_directories(root, &directory)?;
    let path = directory.join(format!("{node_id}-1.log"));
    if fs::symlink_metadata(&path).is_ok() {
        return Err(execution_error(
            "GATE-EXEC-OUTPUT-COLLISION",
            path.display().to_string(),
        ));
    }
    Ok(path)
}

fn create_confined_directories(root: &Path, directory: &Path) -> Result<()> {
    if !directory.starts_with(root) {
        return Err(execution_error(
            "GATE-EXEC-OUTPUT-ESCAPE",
            directory.display().to_string(),
        ));
    }
    fs::create_dir_all(directory)
        .map_err(|error| execution_error("GATE-EXEC-OUTPUT-DIRECTORY", error.to_string()))?;
    let canonical = fs::canonicalize(directory)
        .map_err(|error| execution_error("GATE-EXEC-OUTPUT-DIRECTORY", error.to_string()))?;
    if canonical.starts_with(root) {
        Ok(())
    } else {
        Err(execution_error(
            "GATE-EXEC-OUTPUT-ESCAPE",
            directory.display().to_string(),
        ))
    }
}

fn confined_output_path(root: &Path, relative: &str) -> Result<PathBuf> {
    let path = Path::new(relative);
    require_relative_path(path, false)?;
    Ok(root.join(path))
}

fn require_relative_path(path: &Path, allow_dot: bool) -> Result<()> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(execution_error(
            "GATE-EXEC-PATH",
            path.display().to_string(),
        ));
    }
    let valid = path.components().all(|component| match component {
        Component::Normal(_) => true,
        Component::CurDir => allow_dot,
        _ => false,
    });
    if valid {
        Ok(())
    } else {
        Err(execution_error(
            "GATE-EXEC-PATH",
            path.display().to_string(),
        ))
    }
}

fn canonical_directory(path: &Path, code: &'static str) -> Result<PathBuf> {
    let canonical = fs::canonicalize(path)
        .map_err(|error| execution_error(code, format!("{}: {error}", path.display())))?;
    if canonical.is_dir() {
        Ok(canonical)
    } else {
        Err(execution_error(code, path.display().to_string()))
    }
}

fn current_roots(repo: &Path, plan: &Value) -> Result<Value> {
    let revision = plan["source"]["head_commit"].as_str().unwrap_or("HEAD");
    manifest_roots(repo, revision, true)
}

fn git_text(repo: &Path, arguments: &[&str]) -> Result<String> {
    let bytes = git_bytes(repo, arguments)?;
    String::from_utf8(bytes)
        .map_err(|error| execution_error("GATE-EXEC-GIT-UTF8", error.to_string()))
}

fn git_bytes(repo: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(repo)
        .env_clear()
        .env(
            "PATH",
            std::env::var("PATH")
                .map_err(|_| execution_error("GATE-EXEC-ENVIRONMENT-MISSING", "PATH"))?,
        )
        .output()
        .map_err(|error| execution_error("GATE-EXEC-GIT", error.to_string()))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(execution_error(
            "GATE-EXEC-GIT",
            String::from_utf8_lossy(&output.stderr).trim(),
        ))
    }
}

fn source_snapshot(plan: &Value) -> Result<String> {
    digest(&json!({
        "source": plan["source"],
        "roots": plan["environment_roots"]
    }))
}

fn result_counts(results: &BTreeMap<String, String>) -> (u64, u64, u64) {
    let passed = results
        .values()
        .filter(|result| result.as_str() == "PASS")
        .count() as u64;
    let failed = results
        .values()
        .filter(|result| result.as_str() == "FAIL")
        .count() as u64;
    let blocked = results
        .values()
        .filter(|result| matches!(result.as_str(), "BLOCKED" | "INVALID"))
        .count() as u64;
    (passed, failed, blocked)
}

fn aggregate_result(_passed: u64, failed: u64, blocked: u64) -> &'static str {
    if failed > 0 {
        "FAIL"
    } else if blocked > 0 {
        "BLOCKED"
    } else {
        "PASS"
    }
}

fn timestamp() -> Result<String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| execution_error("GATE-EXEC-TIMESTAMP", error.to_string()))
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field]
        .as_str()
        .ok_or_else(|| execution_error("GATE-EXEC-SHAPE", field))
}

fn required_u64(value: &Value, field: &str) -> Result<u64> {
    value[field]
        .as_u64()
        .ok_or_else(|| execution_error("GATE-EXEC-SHAPE", field))
}

fn require_identifier(value: &str, code: &'static str) -> Result<()> {
    if !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        Ok(())
    } else {
        Err(execution_error(code, value))
    }
}

fn string_array(value: &Value, label: &str) -> Result<Vec<String>> {
    value
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-SHAPE", label))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| execution_error("GATE-EXEC-SHAPE", label))
        })
        .collect()
}

fn read_json(path: &Path) -> Result<Value> {
    let bytes = fs::read(path).map_err(|error| {
        execution_error("GATE-EXEC-READ", format!("{}: {error}", path.display()))
    })?;
    parse_strict(&bytes)
}

fn execution_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, code, message)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use serde_json::json;

    use super::{allowed_environment, require_relative_path, supported_executor};

    #[test]
    fn rejects_path_escape_and_absolute_paths() {
        for path in ["../escape", "/absolute", "nested/../escape", ""] {
            assert!(require_relative_path(Path::new(path), false).is_err());
        }
        assert!(require_relative_path(Path::new("target/gate/result.json"), false).is_ok());
    }

    #[test]
    fn rejects_unknown_executor_before_spawn() {
        let node = json!({"executor": {"kind": "SHELL_V1"}});
        let error = supported_executor(&node).expect_err("unknown executor must fail");
        assert_eq!(error.code, "GATE-EXEC-UNSUPPORTED");
    }

    #[test]
    fn environment_is_exactly_allowlisted() {
        let environment = allowed_environment(&json!({"environment_allowlist": ["PATH"]}))
            .expect("allowlisted environment");
        assert_eq!(environment.len(), 1);
        assert_eq!(environment.get("PATH"), std::env::var("PATH").ok().as_ref());
        assert!(!environment.contains_key("HOME"));
    }
}
