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

use crate::artifact_contract::{
    artifact_kind, create_confined_directories, has_output_extension, node_has_junit_evidence,
};
use crate::canonical::{
    current_executable_sha256, derived_id, digest, parse_strict, sha256_bytes, validate_schema,
};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::execution_nextest::derive_execution_config;
use crate::execution_temp::with_process_temp;
use crate::executor_source::{current_roots, git_bytes, git_text};
pub(crate) use crate::executor_source::{observed_source_snapshot, source_snapshot};
use crate::planner::{
    environment_record, inventory_for_node, reconstruct_plan_in, tool_records, verify_plan_identity,
};
use crate::pre_heavy::{
    ConstructedAudit, validate_audit_for_execution, validate_current_execution_context,
};
use crate::repository::remove_reconstruction_workspace;
use crate::resume::{ResumeCandidate, apply_candidate};

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
            job: "openwepp/increment-gates".to_owned(),
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
    executed_inventory: BTreeSet<String>,
    unavailable_reason: Option<String>,
}

struct ProcessOutcome {
    exit_code: Option<i32>,
    termination_signal: Option<i32>,
    result: String,
    unavailable_reason: Option<String>,
}

struct ExecutionRecord {
    final_results: BTreeMap<String, String>,
    attempts: Vec<Value>,
    executed_inventory: BTreeSet<String>,
    unavailable: BTreeMap<String, String>,
    resume_decisions: Vec<Value>,
}

impl ExecutionRecord {
    fn empty() -> Self {
        Self {
            final_results: BTreeMap::new(),
            attempts: Vec::new(),
            executed_inventory: BTreeSet::new(),
            unavailable: BTreeMap::new(),
            resume_decisions: Vec::new(),
        }
    }

    fn from_stage_receipt(receipt: &Value) -> Result<Self> {
        Ok(Self {
            final_results: stage_final_results(receipt)?,
            attempts: stage_attempts(receipt)?,
            executed_inventory: stage_executed_inventory(receipt)?,
            unavailable: stage_unavailable_items(receipt)?,
            resume_decisions: Vec::new(),
        })
    }
}

fn stage_final_results(receipt: &Value) -> Result<BTreeMap<String, String>> {
    receipt["final_results"]
        .as_object()
        .ok_or_else(|| execution_error("GATE-EXEC-STAGE-RECEIPT", "final_results"))?
        .iter()
        .map(|(node_id, result)| {
            result
                .as_str()
                .map(|value| (node_id.clone(), value.to_owned()))
                .ok_or_else(|| execution_error("GATE-EXEC-STAGE-RECEIPT", "non-string result"))
        })
        .collect()
}

fn stage_attempts(receipt: &Value) -> Result<Vec<Value>> {
    receipt["attempts"]
        .as_array()
        .cloned()
        .ok_or_else(|| execution_error("GATE-EXEC-STAGE-RECEIPT", "attempts"))
}

fn stage_executed_inventory(receipt: &Value) -> Result<BTreeSet<String>> {
    Ok(
        string_array(&receipt["executed_inventory"], "stage executed inventory")?
            .into_iter()
            .collect(),
    )
}

fn stage_unavailable_items(receipt: &Value) -> Result<BTreeMap<String, String>> {
    receipt["unavailable_items"]
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-STAGE-RECEIPT", "unavailable_items"))?
        .iter()
        .map(|item| {
            Ok((
                required_string(item, "item_id")?.to_owned(),
                required_string(item, "reason_code")?.to_owned(),
            ))
        })
        .collect()
}

/// Execute a validated terminal plan and construct an unsigned local receipt.
///
/// Fails closed on invalid or stale plans, execution-contract violations, or
/// source mutation. The receipt still requires independent verification.
///
/// # Errors
/// Returns a typed execution-contract error.
pub fn execute_plan(
    repo: &Path,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
) -> Result<Value> {
    if has_cost_class(plan, "HEAVY")? {
        return Err(execution_error(
            "GATE-EXEC-HEAVY-REQUIRES-AUDIT",
            "a plan containing HEAVY nodes must use the staged executor",
        ));
    }
    execute_plan_stage(repo, plan, artifact_root, claims, "FINAL_LIGHT", None, None)
}

/// Execute one authenticated plan stage.
///
/// # Errors
/// Returns a typed error on invalid bindings or execution failure.
pub fn execute_plan_stage(
    repo: &Path,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
    stage: &str,
    audit: Option<&ConstructedAudit>,
    resume: Option<&ResumeCandidate>,
) -> Result<Value> {
    let repository = canonical_directory(repo, "GATE-EXEC-REPOSITORY")?;
    let artifacts = canonical_directory(artifact_root, "GATE-EXEC-ARTIFACT-ROOT")?;
    if artifacts.starts_with(&repository) {
        return Err(execution_error(
            "GATE-EXEC-ARTIFACT-IN-REPOSITORY",
            "artifact root must be outside the repository",
        ));
    }
    create_confined_directories(&artifacts, &work_root(&artifacts))?;
    create_confined_directories(&artifacts, &cargo_target_root(&artifacts))?;
    validate_plan(&repository, &artifacts, plan, stage != "HEAVY")?;
    verify_execution_checkout(&repository, plan)?;
    let StageAdmission {
        mut imported,
        admitted_audit,
    } = admit_stage(&repository, plan, &artifacts, claims, stage, audit, resume)?;

    let started_at = timestamp()?;
    let source_snapshot = source_snapshot(plan)?;
    let observed_before = observed_source_snapshot(&repository, plan)?;
    if observed_before != source_snapshot {
        return Err(execution_error(
            "GATE-EXEC-SOURCE-DRIFT",
            "repository state differs from the verified pre-execution snapshot",
        ));
    }
    let roots_before = current_roots(&repository, plan)?;
    if roots_before != plan["environment_roots"] {
        return Err(execution_error(
            "GATE-EXEC-SOURCE-DRIFT",
            "execution roots differ from the verified plan",
        ));
    }

    let execution_class = if stage == "FINAL_LIGHT" {
        "LIGHT"
    } else {
        stage
    };
    if execution_class == "HEAVY" {
        validate_current_execution_context(&repository, plan)?;
    }
    let execution = execute_nodes_for(
        &repository,
        &artifacts,
        plan,
        &roots_before,
        &observed_before,
        execution_class,
        claims,
        &mut imported,
    )?;

    finalize_stage_execution(
        &StageFinalization {
            repository: &repository,
            plan,
            artifacts: &artifacts,
            roots_before: &roots_before,
            observed_before: &observed_before,
            started_at: &started_at,
            source_snapshot: &source_snapshot,
            claims,
            stage,
            admitted_audit,
        },
        execution,
    )
}

struct StageAdmission<'a> {
    imported: ExecutionRecord,
    admitted_audit: Option<&'a Value>,
}

fn admit_stage<'a>(
    repository: &Path,
    plan: &Value,
    artifacts: &Path,
    claims: &ExecutionClaims,
    stage: &str,
    audit: Option<&'a ConstructedAudit>,
    resume: Option<&ResumeCandidate>,
) -> Result<StageAdmission<'a>> {
    let admitted_audit = if stage == "HEAVY" {
        let audit = audit.ok_or_else(|| {
            execution_error("GATE-EXEC-AUDIT-REQUIRED", "heavy stage requires an audit")
        })?;
        validate_audit_for_execution(repository, plan, audit.as_value(), artifacts, claims)?;
        Some(audit.as_value())
    } else {
        None
    };
    let allowed_existing = admitted_light_nodes(admitted_audit)?;
    preflight(repository, artifacts, plan, &allowed_existing, false)?;

    let mut imported = match stage {
        "LIGHT" | "FINAL_LIGHT" => ExecutionRecord::empty(),
        "HEAVY" => {
            let audit = admitted_audit.ok_or_else(|| {
                execution_error("GATE-EXEC-AUDIT-REQUIRED", "heavy stage requires an audit")
            })?;
            ExecutionRecord::from_stage_receipt(&audit["light_receipt"])?
        }
        _ => return Err(execution_error("GATE-EXEC-STAGE", stage)),
    };
    if stage == "HEAVY" {
        let seed = apply_candidate(plan, artifacts, claims, resume)?;
        imported.attempts.extend(seed.attempts);
        imported.final_results.extend(seed.final_results);
        imported.executed_inventory.extend(seed.executed_inventory);
        imported.resume_decisions = seed.decisions;
    }
    Ok(StageAdmission {
        imported,
        admitted_audit,
    })
}

#[derive(Clone, Copy)]
struct StageFinalization<'a> {
    repository: &'a Path,
    plan: &'a Value,
    artifacts: &'a Path,
    roots_before: &'a Value,
    observed_before: &'a str,
    started_at: &'a str,
    source_snapshot: &'a str,
    claims: &'a ExecutionClaims,
    stage: &'a str,
    admitted_audit: Option<&'a Value>,
}

fn finalize_stage_execution(
    context: &StageFinalization<'_>,
    mut execution: ExecutionRecord,
) -> Result<Value> {
    let StageFinalization {
        repository,
        plan,
        artifacts,
        roots_before,
        observed_before,
        started_at,
        source_snapshot,
        claims,
        stage,
        admitted_audit,
    } = *context;
    let roots_after = current_roots(repository, plan)?;
    let observed_after = observed_source_snapshot(repository, plan)?;
    let source_unchanged = roots_after == *roots_before && observed_after == observed_before;
    if !source_unchanged
        && !execution
            .attempts
            .iter()
            .any(|attempt| attempt["result"] == "INVALID")
    {
        mark_source_mutation(&mut execution)?;
    }
    let unavailable_items = unavailable_items(&mut execution);
    let finished_at = timestamp()?;
    if stage == "LIGHT" {
        return build_stage_receipt(
            repository,
            plan,
            artifacts,
            &execution,
            &unavailable_items,
            started_at,
            &finished_at,
            claims,
        );
    }
    build_receipt(
        repository,
        plan,
        artifacts,
        &execution.attempts,
        &execution.final_results,
        &execution.executed_inventory,
        &unavailable_items,
        started_at,
        &finished_at,
        source_snapshot,
        &observed_after,
        source_unchanged,
        claims,
        admitted_audit,
        &execution.resume_decisions,
    )
}

#[cfg(test)]
fn execute_nodes(
    repo: &Path,
    artifact_root: &Path,
    plan: &Value,
    roots_before: &Value,
    observed_before: &str,
) -> Result<ExecutionRecord> {
    let mut record = ExecutionRecord::empty();
    execute_nodes_for(
        repo,
        artifact_root,
        plan,
        roots_before,
        observed_before,
        "ALL",
        &ExecutionClaims::default(),
        &mut record,
    )
}
#[allow(clippy::too_many_arguments)] // Independently authenticated inputs.
fn execute_nodes_for(
    repo: &Path,
    artifact_root: &Path,
    plan: &Value,
    roots_before: &Value,
    observed_before: &str,
    cost_class: &str,
    claims: &ExecutionClaims,
    record: &mut ExecutionRecord,
) -> Result<ExecutionRecord> {
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-PLAN-SHAPE", "nodes must be an array"))?;
    let mut source_invalid = false;
    for node in nodes {
        if cost_class != "ALL" && node["execution_cost_class"] != cost_class {
            continue;
        }
        let node_id = required_string(node, "node_id")?;
        if record
            .final_results
            .get(node_id)
            .is_some_and(|result| result == "PASS")
        {
            continue;
        }
        let forced_reason = source_invalid.then_some("SOURCE_MUTATION_DETECTED");
        let mut run = execute_node(
            repo,
            artifact_root,
            node,
            &record.final_results,
            forced_reason,
        )?;
        if !source_invalid
            && (current_roots(repo, plan)? != *roots_before
                || observed_source_snapshot(repo, plan)? != observed_before)
        {
            run.attempt["result"] = Value::String("INVALID".to_owned());
            "INVALID".clone_into(&mut run.result);
            run.unavailable_reason = Some("SOURCE_MUTATION_DETECTED".to_owned());
            source_invalid = true;
        }
        let node_id = node_id.to_owned();
        record.final_results.insert(node_id, run.result.clone());
        write_node_artifacts(artifact_root, plan, roots_before, claims, node, &run)?;
        record
            .executed_inventory
            .extend(run.executed_inventory.iter().cloned());
        record_unavailable(node, &run, &mut record.unavailable)?;
        record.attempts.push(run.attempt);
    }
    Ok(std::mem::replace(record, ExecutionRecord::empty()))
}

fn record_unavailable(
    node: &Value,
    run: &NodeRun,
    unavailable: &mut BTreeMap<String, String>,
) -> Result<()> {
    if let Some(reason) = &run.unavailable_reason {
        for item in string_array(&node["expected_inventory"]["ids"], "inventory")? {
            unavailable.entry(item).or_insert_with(|| reason.clone());
        }
    }
    Ok(())
}

fn mark_source_mutation(record: &mut ExecutionRecord) -> Result<()> {
    let last_attempt = record.attempts.last_mut().ok_or_else(|| {
        execution_error(
            "GATE-EXEC-SOURCE-MUTATION",
            "source changed without an attributable node attempt",
        )
    })?;
    last_attempt["result"] = Value::String("INVALID".to_owned());
    let node_id = required_string(last_attempt, "node_id")?.to_owned();
    record.final_results.insert(node_id, "INVALID".to_owned());
    Ok(())
}

fn has_cost_class(plan: &Value, class: &str) -> Result<bool> {
    Ok(plan["nodes"]
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-PLAN-SHAPE", "nodes must be an array"))?
        .iter()
        .any(|node| node["execution_cost_class"] == class))
}

fn unavailable_items(record: &mut ExecutionRecord) -> Vec<Value> {
    record
        .unavailable
        .retain(|item, _| !record.executed_inventory.contains(item));
    std::mem::take(&mut record.unavailable)
        .into_iter()
        .map(|(item_id, reason_code)| {
            json!({
                "item_id": item_id,
                "reason_code": reason_code,
                "policy_disposition": "BLOCK"
            })
        })
        .collect()
}

fn validate_plan(repo: &Path, artifact_root: &Path, plan: &Value, reconstruct: bool) -> Result<()> {
    let schema = read_json(&repo.join("gate-policy/v1/schemas/gate-plan.schema.json"))?;
    validate_schema(&schema, plan, "executor gate plan")?;
    verify_plan_identity(plan)?;
    if reconstruct {
        let reconstruction = work_root(artifact_root).join("reconstruction");
        let reconstructed = reconstruct_plan_in(repo, plan, &reconstruction, false);
        remove_reconstruction_workspace(&reconstruction)?;
        let reconstructed = reconstructed?;
        if digest(&reconstructed)? != digest(plan)? {
            return Err(execution_error(
                "GATE-EXEC-PLAN-RECONSTRUCTION",
                "current policy and source do not reconstruct the supplied plan",
            ));
        }
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

fn preflight(
    repo: &Path,
    artifact_root: &Path,
    plan: &Value,
    allowed_existing: &BTreeSet<String>,
    enumerate_inventory: bool,
) -> Result<()> {
    validate_quality_scope(plan)?;
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-PLAN-SHAPE", "nodes must be an array"))?;
    let mut outputs = BTreeSet::new();
    for node in nodes {
        supported_executor(node)?;
        reject_shell_string(node)?;
        if node["retry"]["maximum_attempts"] != 1 {
            return Err(execution_error(
                "GATE-EXEC-RETRY-UNSUPPORTED",
                "v1 executor admits exactly one attempt",
            ));
        }
        confined_working_directory(repo, node)?;
        allowed_environment(node)?;
        if enumerate_inventory {
            let current_inventory =
                inventory_for_node(repo, node, Some(&cargo_target_root(artifact_root)))?;
            let expected = string_array(&node["expected_inventory"]["ids"], "inventory")?;
            if current_inventory != expected {
                return Err(execution_error(
                    "GATE-EXEC-INVENTORY-DRIFT",
                    required_string(node, "gate_definition_id")?,
                ));
            }
        }
        for path in string_array(&node["output_paths"], "output_paths")? {
            if !outputs.insert(path.clone()) {
                return Err(execution_error("GATE-EXEC-OUTPUT-COLLISION", path));
            }
            let destination = confined_output_path(artifact_root, &path)?;
            if fs::symlink_metadata(&destination).is_ok() {
                let node_id = required_string(node, "node_id")?;
                if !allowed_existing.contains(node_id) {
                    return Err(execution_error("GATE-EXEC-OUTPUT-COLLISION", path));
                }
                verify_checkpoint_artifact(artifact_root, node, &path)?;
            }
        }
    }
    Ok(())
}

fn admitted_light_nodes(audit: Option<&Value>) -> Result<BTreeSet<String>> {
    let Some(audit) = audit else {
        return Ok(BTreeSet::new());
    };
    audit["light_receipt"]["final_results"]
        .as_object()
        .ok_or_else(|| execution_error("GATE-EXEC-AUDIT-LIGHT-RESULTS", "final_results"))?
        .iter()
        .filter(|(_, result)| *result == "PASS")
        .map(|(node_id, _)| Ok(node_id.clone()))
        .collect()
}

fn verify_checkpoint_artifact(artifact_root: &Path, node: &Value, relative: &str) -> Result<()> {
    let node_id = required_string(node, "node_id")?;
    let checkpoint_path = artifact_root
        .join(".checkpoints")
        .join(format!("{node_id}.json"));
    let checkpoint = read_json(&checkpoint_path)?;
    let expected = checkpoint["artifacts"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|artifact| artifact["path"] == relative)
        .and_then(|artifact| artifact["sha256"].as_str())
        .ok_or_else(|| execution_error("GATE-EXEC-CHECKPOINT-ARTIFACT", relative))?;
    let bytes = fs::read(confined_output_path(artifact_root, relative)?)
        .map_err(|error| execution_error("GATE-EXEC-CHECKPOINT-ARTIFACT", error.to_string()))?;
    if checkpoint["node_sha256"] == digest(node)? && sha256_bytes(&bytes) == expected {
        Ok(())
    } else {
        Err(execution_error(
            "GATE-EXEC-CHECKPOINT-ARTIFACT-DRIFT",
            relative,
        ))
    }
}

fn reject_shell_string(node: &Value) -> Result<()> {
    let arguments = string_array(&node["arguments"], "arguments")?;
    let uses_shell_string = arguments.iter().enumerate().any(|(index, argument)| {
        !argument.contains('=')
            && Path::new(argument)
                .extension()
                .is_none_or(|extension| extension != "sh")
            && Path::new(argument)
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    let name = name.to_ascii_lowercase();
                    name == "sh" || name.ends_with("sh") || name == "powershell"
                })
            && shell_uses_command_string(&arguments[index + 1..])
    });
    if uses_shell_string {
        Err(execution_error(
            "GATE-EXEC-SHELL-STRING",
            "plan arguments cannot contain an inline shell program",
        ))
    } else {
        Ok(())
    }
}

fn shell_uses_command_string(arguments: &[String]) -> bool {
    for argument in arguments {
        let normalized = argument.to_ascii_lowercase();
        if normalized == "--command"
            || normalized.starts_with("--command=")
            || normalized == "-c"
            || normalized.starts_with('-')
                && !normalized.starts_with("--")
                && normalized[1..].contains('c')
        {
            return true;
        }
        let path = Path::new(argument);
        if argument.contains('/') || path.extension().is_some_and(|extension| extension == "sh") {
            return false;
        }
    }
    false
}

fn validate_quality_scope(plan: &Value) -> Result<()> {
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| execution_error("GATE-EXEC-PLAN-SHAPE", "nodes must be an array"))?;
    let affected_nodes = nodes
        .iter()
        .filter(|node| node["gate_definition_id"] == "affected-adjudicated-crap-v1")
        .collect::<Vec<_>>();
    let global_nodes = nodes
        .iter()
        .filter(|node| node["gate_definition_id"] == "adjudicated-crap-v1")
        .collect::<Vec<_>>();
    match required_string(&plan["quality_scope"], "mode")? {
        "NOT_APPLICABLE" if affected_nodes.is_empty() => Ok(()),
        "GLOBAL" if affected_nodes.is_empty() && global_nodes.len() == 1 => Ok(()),
        "AFFECTED" if affected_nodes.len() == 1 && global_nodes.is_empty() => {
            validate_affected_quality_scope(&plan["quality_scope"], affected_nodes[0], nodes)
        }
        mode => Err(execution_error(
            "GATE-EXEC-QUALITY-SCOPE",
            format!("quality scope {mode} does not match the planned measurement nodes"),
        )),
    }
}

fn validate_affected_quality_scope(scope: &Value, affected: &Value, nodes: &[Value]) -> Result<()> {
    if scope["completeness"] != "COMPLETE" {
        return Err(execution_error(
            "GATE-EXEC-QUALITY-INCOMPLETE",
            "affected quality requires complete contribution evidence",
        ));
    }
    validate_affected_quality_packages(scope, affected)?;
    let covering_nodes = affected_quality_covering_nodes(scope, nodes)?;
    validate_affected_quality_inventory(scope, &covering_nodes)
}

fn validate_affected_quality_packages(scope: &Value, affected: &Value) -> Result<()> {
    let planned_packages = string_array(&scope["production_packages"], "production_packages")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let argument_packages = argument_values(affected, "--package")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    if planned_packages != argument_packages {
        return Err(execution_error(
            "GATE-EXEC-QUALITY-PACKAGES",
            "affected measurement arguments differ from terminal production packages",
        ));
    }
    Ok(())
}

fn affected_quality_covering_nodes<'a>(
    scope: &Value,
    nodes: &'a [Value],
) -> Result<Vec<&'a Value>> {
    let covering_ids = string_array(&scope["covering_node_ids"], "covering_node_ids")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let covering_nodes = nodes
        .iter()
        .filter(|node| {
            node["node_id"]
                .as_str()
                .is_some_and(|id| covering_ids.contains(id))
        })
        .collect::<Vec<_>>();
    if covering_nodes.len() != covering_ids.len()
        || covering_nodes
            .iter()
            .any(|node| node["gate_definition_id"] != "affected-adjudicated-crap-v1")
    {
        return Err(execution_error(
            "GATE-EXEC-QUALITY-COVERING-NODES",
            "covering node identity is not the combined affected measurement",
        ));
    }
    Ok(covering_nodes)
}

fn validate_affected_quality_inventory(scope: &Value, covering_nodes: &[&Value]) -> Result<()> {
    let observed_inventory = covering_nodes
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
    let planned_inventory =
        string_array(&scope["covering_inventory_ids"], "covering_inventory_ids")?
            .into_iter()
            .collect::<BTreeSet<_>>();
    if observed_inventory == planned_inventory && !planned_inventory.is_empty() {
        Ok(())
    } else {
        Err(execution_error(
            "GATE-EXEC-QUALITY-INVENTORY",
            "covering inventory differs from terminal package test closure",
        ))
    }
}

fn argument_values(node: &Value, flag: &str) -> Result<Vec<String>> {
    let arguments = string_array(&node["arguments"], "arguments")?;
    let mut values = Vec::new();
    let mut index = 0;
    while index < arguments.len() {
        if arguments[index] == flag {
            let value = arguments.get(index + 1).ok_or_else(|| {
                execution_error("GATE-EXEC-QUALITY-ARGUMENT", format!("{flag} has no value"))
            })?;
            values.push(value.clone());
            index += 2;
        } else {
            index += 1;
        }
    }
    Ok(values)
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
        match std::env::var(&key) {
            Ok(value) => {
                environment.insert(key, value);
            }
            Err(std::env::VarError::NotPresent) if key != "PATH" => {}
            Err(std::env::VarError::NotPresent) => {
                return Err(execution_error("GATE-EXEC-ENVIRONMENT-MISSING", key));
            }
            Err(std::env::VarError::NotUnicode(_)) => {
                return Err(execution_error("GATE-EXEC-ENVIRONMENT-NONUTF8", key));
            }
        }
    }
    Ok(environment)
}

fn execute_node(
    repo: &Path,
    artifact_root: &Path,
    node: &Value,
    final_results: &BTreeMap<String, String>,
    forced_block_reason: Option<&str>,
) -> Result<NodeRun> {
    let node_id = required_string(node, "node_id")?;
    let started_at = timestamp()?;
    let log_path = attempt_log_path(artifact_root, node_id)?;
    let prerequisite_failed = string_array(&node["prerequisites"], "prerequisites")?
        .iter()
        .any(|id| final_results.get(id).is_none_or(|result| result != "PASS"));
    if forced_block_reason.is_none() && !prerequisite_failed {
        prepare_real_artifacts(artifact_root, node)?;
    }
    let outcome = if let Some(reason) = forced_block_reason {
        File::create(&log_path)
            .map_err(|error| execution_error("GATE-EXEC-LOG-CREATE", error.to_string()))?;
        ProcessOutcome {
            exit_code: None,
            termination_signal: None,
            result: "BLOCKED".to_owned(),
            unavailable_reason: Some(reason.to_owned()),
        }
    } else if prerequisite_failed {
        File::create(&log_path)
            .map_err(|error| execution_error("GATE-EXEC-LOG-CREATE", error.to_string()))?;
        ProcessOutcome {
            exit_code: None,
            termination_signal: None,
            result: "BLOCKED".to_owned(),
            unavailable_reason: Some("PREREQUISITE_NONPASS".to_owned()),
        }
    } else {
        run_process(repo, artifact_root, node, &log_path)?
    };
    if outcome.result == "PASS" {
        validate_success_artifacts(artifact_root, node)?;
    }
    let (executed_inventory, unavailable_reason) =
        observed_inventory(artifact_root, node, &outcome)?;
    let finished_at = timestamp()?;
    Ok(NodeRun {
        attempt: json!({
            "node_id": node_id,
            "attempt": 1,
            "arguments": node["arguments"],
            "started_at": started_at,
            "finished_at": finished_at,
            "exit_code": outcome.exit_code,
            "termination_signal": outcome.termination_signal,
            "result": outcome.result,
            "retry_reason": null
        }),
        result: outcome.result,
        log_path,
        executed_inventory,
        unavailable_reason,
    })
}

fn observed_inventory(
    artifact_root: &Path,
    node: &Value,
    outcome: &ProcessOutcome,
) -> Result<(BTreeSet<String>, Option<String>)> {
    let planned = string_array(&node["expected_inventory"]["ids"], "inventory")?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let observed = if node["gate_definition_id"] == "required-authority-v1" {
        observed_authority_inventory(artifact_root, node, outcome.result.as_str())?
    } else if node_has_junit_evidence(node) {
        observed_junit_inventory(artifact_root, node, outcome.result.as_str())?
    } else if outcome.exit_code.is_some() || outcome.termination_signal.is_some() {
        planned.clone()
    } else {
        BTreeSet::new()
    };
    if !observed.is_subset(&planned) {
        return Err(execution_error(
            "GATE-EXEC-JUNIT-INVENTORY",
            "observed JUnit contains inventory outside the terminal plan",
        ));
    }
    let reason = if observed == planned {
        None
    } else {
        outcome
            .unavailable_reason
            .clone()
            .or_else(|| Some("TEST_NOT_EXECUTED".to_owned()))
    };
    Ok((observed, reason))
}

fn observed_authority_inventory(
    artifact_root: &Path,
    node: &Value,
    result: &str,
) -> Result<BTreeSet<String>> {
    let report = authority_report_path(artifact_root, node)?;
    let contents = match fs::read_to_string(&report) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && result != "PASS" => {
            return Ok(BTreeSet::new());
        }
        Err(error) => {
            return Err(execution_error(
                "GATE-EXEC-AUTHORITY-REPORT",
                format!("{}: {error}", report.display()),
            ));
        }
    };
    let mut observed = BTreeSet::new();
    for line in contents.lines().filter(|line| {
        line.starts_with("- lane=required failure_class=hard-fail ") && line.contains(" status=")
    }) {
        let suites = line
            .split_ascii_whitespace()
            .find_map(|field| field.strip_prefix("suites="))
            .ok_or_else(|| execution_error("GATE-EXEC-AUTHORITY-REPORT", line))?;
        for suite in suites.split(',') {
            require_identifier(suite, "GATE-EXEC-AUTHORITY-SUITE")?;
            observed.insert(suite.to_owned());
        }
    }
    Ok(observed)
}

fn observed_junit_inventory(
    artifact_root: &Path,
    node: &Value,
    result: &str,
) -> Result<BTreeSet<String>> {
    let path = nextest_junit_path(artifact_root, node)?;
    match fs::symlink_metadata(&path) {
        Ok(_) => junit_inventory(&path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && result != "PASS" => {
            Ok(BTreeSet::new())
        }
        Err(error) => Err(execution_error(
            "GATE-EXEC-JUNIT-READ",
            format!("{}: {error}", path.display()),
        )),
    }
}

fn validate_success_artifacts(artifact_root: &Path, node: &Value) -> Result<()> {
    match required_string(node, "artifact_contract")? {
        "nextest-junit-v1" => validate_junit_artifact(artifact_root, node),
        "adjudicated-crap-v1" => validate_crap_artifacts(artifact_root, node),
        _ => Ok(()),
    }
}

fn validate_crap_artifacts(artifact_root: &Path, node: &Value) -> Result<()> {
    let report = validated_crap_report_bytes(artifact_root, node)?;
    if has_output_extension(node, "xml") {
        validate_junit_artifact(artifact_root, node)?;
    }
    if has_output_extension(node, "lcov") {
        let report = parse_strict(&report)?;
        let lcov = read_real_artifact(artifact_root, &affected_lcov_path(artifact_root, node)?)?;
        if report["acquisition_mode"] != "fresh"
            || report["closure_eligible"] != true
            || report["lcov_sha256"] != sha256_bytes(&lcov)
        {
            return Err(execution_error(
                "GATE-EXEC-CRAP-LCOV-LINEAGE",
                "CRAP report does not bind the fresh published LCOV bytes",
            ));
        }
    }
    Ok(())
}

fn validated_crap_report_bytes(artifact_root: &Path, node: &Value) -> Result<Vec<u8>> {
    let status_path = adjudicated_crap_status_path(artifact_root, node)?;
    let status = parse_strict(&read_real_artifact(artifact_root, &status_path)?)?;
    if status["result"] != "PASS" || status["exit_status"].as_i64() != Some(0) {
        return Err(execution_error(
            "GATE-EXEC-CRAP-CONTROL",
            "adapter exited successfully without a PASS control envelope",
        ));
    }
    let expected_sha256 = required_string(&status, "adjudicated_crap_report_sha256")?;
    let report_path = adjudicated_crap_report_path(artifact_root, node)?;
    let report = read_real_artifact(artifact_root, &report_path)?;
    let observed_sha256 = sha256_bytes(&report);
    if observed_sha256 != expected_sha256 {
        return Err(execution_error(
            "GATE-EXEC-CRAP-REPORT-DIGEST",
            format!("expected {expected_sha256}, observed {observed_sha256}"),
        ));
    }
    Ok(report)
}

fn validate_junit_artifact(artifact_root: &Path, node: &Value) -> Result<()> {
    let actual = junit_inventory(&nextest_junit_path(artifact_root, node)?)?;
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
    Ok(inventory)
}

fn xml_attribute(line: &str, name: &str) -> Result<String> {
    let marker = format!(" {name}=\"");
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

fn prepare_real_artifacts(artifact_root: &Path, node: &Value) -> Result<()> {
    for path in real_artifact_sources(artifact_root, node)? {
        reset_real_artifact(&path)?;
    }
    Ok(())
}

fn reset_real_artifact(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(execution_error(
            "GATE-EXEC-REAL-ARTIFACT-SYMLINK",
            path.display().to_string(),
        )),
        Ok(metadata) if metadata.is_file() => fs::remove_file(path)
            .map_err(|error| execution_error("GATE-EXEC-REAL-ARTIFACT-RESET", error.to_string())),
        Ok(_) => Err(execution_error(
            "GATE-EXEC-REAL-ARTIFACT-TYPE",
            path.display().to_string(),
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(execution_error(
            "GATE-EXEC-REAL-ARTIFACT-METADATA",
            error.to_string(),
        )),
    }
}

fn real_artifact_sources(artifact_root: &Path, node: &Value) -> Result<Vec<PathBuf>> {
    match required_string(node, "artifact_contract")? {
        "nextest-junit-v1" => Ok(vec![nextest_junit_path(artifact_root, node)?]),
        "adjudicated-crap-v1" if has_output_extension(node, "xml") => Ok(vec![
            adjudicated_crap_report_path(artifact_root, node)?,
            adjudicated_crap_status_path(artifact_root, node)?,
            nextest_junit_path(artifact_root, node)?,
            affected_lcov_path(artifact_root, node)?,
        ]),
        "adjudicated-crap-v1" => Ok(vec![
            adjudicated_crap_report_path(artifact_root, node)?,
            adjudicated_crap_status_path(artifact_root, node)?,
        ]),
        "authority-suite-report-v1" => Ok(vec![authority_report_path(artifact_root, node)?]),
        _ => Ok(Vec::new()),
    }
}

fn nextest_junit_path(artifact_root: &Path, node: &Value) -> Result<PathBuf> {
    let arguments = string_array(&node["arguments"], "arguments")?;
    let profile = arguments
        .windows(2)
        .find(|pair| matches!(pair[0].as_str(), "--profile" | "--nextest-profile"))
        .map_or_else(
            || {
                if node["artifact_contract"] == "adjudicated-crap-v1" {
                    "full"
                } else {
                    "default"
                }
            },
            |pair| pair[1].as_str(),
        );
    require_identifier(profile, "GATE-EXEC-NEXTEST-PROFILE")?;
    let target = if node["artifact_contract"] == "adjudicated-crap-v1" {
        let report = adjudicated_crap_report_path(artifact_root, node)?;
        report
            .parent()
            .ok_or_else(|| execution_error("GATE-EXEC-REAL-ARTIFACT-PATH", "Nextest output"))?
            .join("nextest")
    } else {
        work_root(artifact_root).join("nextest")
    };
    Ok(target.join(profile).join("junit.xml"))
}

fn affected_lcov_path(artifact_root: &Path, node: &Value) -> Result<PathBuf> {
    let report = adjudicated_crap_report_path(artifact_root, node)?;
    let directory = report
        .parent()
        .ok_or_else(|| execution_error("GATE-EXEC-REAL-ARTIFACT-PATH", "CRAP output"))?;
    Ok(directory.join("workspace.lcov"))
}

fn adjudicated_crap_report_path(artifact_root: &Path, node: &Value) -> Result<PathBuf> {
    let arguments = string_array(&node["arguments"], "arguments")?;
    let output = arguments
        .windows(2)
        .find(|pair| pair[0] == "--output-dir")
        .map_or("target/adjudicated-crap", |pair| pair[1].as_str());
    let relative = Path::new(output);
    require_relative_path(relative, false)?;
    Ok(work_root(artifact_root)
        .join(relative)
        .join("adjudicated-crap-report.json"))
}

fn adjudicated_crap_status_path(artifact_root: &Path, node: &Value) -> Result<PathBuf> {
    let report = adjudicated_crap_report_path(artifact_root, node)?;
    let directory = report
        .parent()
        .ok_or_else(|| execution_error("GATE-EXEC-REAL-ARTIFACT-PATH", "CRAP control output"))?;
    Ok(directory.join("run-status.json"))
}

fn authority_report_path(artifact_root: &Path, node: &Value) -> Result<PathBuf> {
    let path = string_array(&node["output_paths"], "output_paths")?
        .into_iter()
        .next()
        .ok_or_else(|| execution_error("GATE-EXEC-AUTHORITY-REPORT", "missing output path"))?;
    let relative = Path::new(&path);
    require_relative_path(relative, false)?;
    Ok(work_root(artifact_root).join(relative))
}

fn run_process(
    repo: &Path,
    artifact_root: &Path,
    node: &Value,
    log_path: &Path,
) -> Result<ProcessOutcome> {
    with_process_temp(|temporary| {
        let arguments = runtime_arguments(repo, artifact_root, node)?;
        let program = arguments
            .first()
            .ok_or_else(|| execution_error("GATE-EXEC-ARGUMENTS", "missing executable"))?;
        let log = File::create(log_path)
            .map_err(|error| execution_error("GATE-EXEC-LOG-CREATE", error.to_string()))?;
        let stderr = log
            .try_clone()
            .map_err(|error| execution_error("GATE-EXEC-LOG-CLONE", error.to_string()))?;
        let mut environment = allowed_environment(node)?;
        environment.insert(
            "CARGO_TARGET_DIR".to_owned(),
            cargo_target_root(artifact_root).display().to_string(),
        );
        environment.insert(
            "OPENWEPP_GATE_ARTIFACT_ROOT".to_owned(),
            work_root(artifact_root).display().to_string(),
        );
        environment.insert("TMPDIR".to_owned(), temporary.display().to_string());
        if node["artifact_contract"] == "adjudicated-crap-v1" {
            environment.insert(
                "OPENWEPP_GATE_NEXTEST_CONFIG".to_owned(),
                external_nextest_config(repo, artifact_root)?
                    .display()
                    .to_string(),
            );
        }
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
                return Ok(ProcessOutcome {
                    exit_code: None,
                    termination_signal: None,
                    result: "BLOCKED".to_owned(),
                    unavailable_reason: Some("PROCESS_SPAWN_FAILED".to_owned()),
                });
            }
        };
        let timeout = required_u64(node, "timeout_seconds")?;
        let status = wait_with_timeout(&mut child, Duration::from_secs(timeout))?;
        Ok(match status {
            Some(status) => {
                let code = status.code();
                #[cfg(unix)]
                let signal = {
                    use std::os::unix::process::ExitStatusExt;
                    status.signal()
                };
                #[cfg(not(unix))]
                let signal = None;
                let expected = node["acceptance"]["expected"]
                    .as_i64()
                    .ok_or_else(|| execution_error("GATE-EXEC-ACCEPTANCE", "expected exit code"))?;
                let result = if code.map(i64::from) == Some(expected) {
                    "PASS"
                } else {
                    "FAIL"
                };
                ProcessOutcome {
                    exit_code: code,
                    termination_signal: signal,
                    result: result.to_owned(),
                    unavailable_reason: None,
                }
            }
            None => ProcessOutcome {
                exit_code: None,
                termination_signal: None,
                result: "BLOCKED".to_owned(),
                unavailable_reason: Some("TIMEOUT".to_owned()),
            },
        })
    })
}

fn runtime_arguments(repo: &Path, artifact_root: &Path, node: &Value) -> Result<Vec<String>> {
    let mut arguments = string_array(&node["arguments"], "arguments")?;
    if required_string(&node["executor"], "kind")? == "NEXTEST_V1" {
        let config = external_nextest_config(repo, artifact_root)?;
        arguments.extend([
            "--target-dir".to_owned(),
            cargo_target_root(artifact_root).display().to_string(),
            "--config-file".to_owned(),
            config.display().to_string(),
        ]);
    }
    Ok(arguments)
}

fn external_nextest_config(repo: &Path, artifact_root: &Path) -> Result<PathBuf> {
    let source = fs::read_to_string(repo.join(".config/nextest.toml"))
        .map_err(|error| execution_error("GATE-EXEC-NEXTEST-CONFIG", error.to_string()))?;
    let store = work_root(artifact_root).join("nextest");
    let contents = derive_execution_config(&source, &store)?;
    let destination = work_root(artifact_root).join("nextest.toml");
    write_atomic(&destination, contents.as_bytes())?;
    Ok(destination)
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
fn build_stage_receipt(
    repo: &Path,
    plan: &Value,
    artifact_root: &Path,
    execution: &ExecutionRecord,
    unavailable_items: &[Value],
    started_at: &str,
    finished_at: &str,
    claims: &ExecutionClaims,
) -> Result<Value> {
    let (passed, failed, blocked, invalid) = result_counts(&execution.final_results);
    let mut receipt = json!({
        "schema_version": "openwepp-gate-stage-receipt-v1",
        "stage_receipt_id": "0".repeat(64),
        "stage": "LIGHT",
        "plan_id": plan["plan_id"],
        "plan_sha256": digest(plan)?,
        "execution_key": plan["execution_key"],
        "executor_binary_sha256": current_executable_sha256()?,
        "artifact_root_sha256": sha256_bytes(artifact_root.as_os_str().as_encoded_bytes()),
        "roots": plan["environment_roots"],
        "attempts": execution.attempts,
        "final_results": execution.final_results,
        "executed_inventory": execution.executed_inventory,
        "unavailable_items": unavailable_items,
        "started_at": started_at,
        "finished_at": finished_at,
        "claims": {
            "principal": claims.principal,
            "repository": claims.repository,
            "source_event": claims.source_event,
            "source_ref": claims.source_ref,
            "workflow": claims.workflow,
            "job": claims.job,
            "runner": claims.runner,
            "attempt": claims.attempt
        },
        "result": aggregate_result(failed, blocked, invalid),
        "counts": {"passed": passed, "failed": failed, "blocked": blocked}
    });
    receipt
        .as_object_mut()
        .ok_or_else(|| execution_error("GATE-EXEC-STAGE-RECEIPT", "object"))?
        .remove("counts");
    receipt["stage_receipt_id"] = Value::String(derived_id(&receipt, "stage_receipt_id")?);
    let schema = read_json(&repo.join("gate-policy/v1/schemas/stage-receipt.schema.json"))?;
    validate_schema(&schema, &receipt, "executor stage receipt")?;
    Ok(receipt)
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
    executed_inventory: &BTreeSet<String>,
    unavailable_items: &[Value],
    started_at: &str,
    finished_at: &str,
    source_snapshot: &str,
    observed_after: &str,
    source_unchanged: bool,
    claims: &ExecutionClaims,
    pre_heavy_audit: Option<&Value>,
    resume_decisions: &[Value],
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
    let (passed, failed, blocked, invalid) = result_counts(final_results);
    let result = aggregate_result(failed, blocked, invalid);
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
        "executed_inventory": executed_inventory,
        "tools": tools,
        "environment": environment,
        "started_at": started_at,
        "finished_at": finished_at,
        "counts": {"passed": passed, "failed": failed, "skipped": unavailable_items.len(), "blocked": blocked, "retried": 0},
        "authority_outcomes": authority_outcomes,
        "artifacts": artifacts,
        "unavailable_items": unavailable_items,
        "source_mutation_check": {
            "required": true,
            "before_sha256": source_snapshot,
            "after_sha256": observed_after,
            "unchanged": source_unchanged
        },
        "result": result,
        "pre_heavy_audit": pre_heavy_audit,
        "resume_decisions": resume_decisions,
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

pub(crate) fn authority_outcomes(
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
            if !authority_adapter_supported(&gate_id, &authority_class) {
                return Err(execution_error(
                    "GATE-EXEC-AUTHORITY-UNSUPPORTED",
                    format!("{gate_id} cannot claim authority class {authority_class}"),
                ));
            }
            let execution_integrity = aggregate_node_results(&results);
            let admission_outcome = if authority_class == "A0" {
                Some(match execution_integrity {
                    "PASS" | "PASS_WITH_RETRY" => "ADMITTED",
                    "FAIL" => "REJECTED",
                    "BLOCKED" => "BLOCKED",
                    _ => "INVALID",
                })
            } else {
                None
            };
            let scientific_outcome = if matches!(
                authority_class.as_str(),
                "A1" | "A2" | "A3" | "A4" | "A5" | "A6"
            ) {
                Some(
                    if matches!(execution_integrity, "PASS" | "PASS_WITH_RETRY") {
                        "CONFORMS"
                    } else {
                        "NOT_EVALUATED"
                    },
                )
            } else {
                None
            };
            Ok(json!({
                "gate_id": gate_id,
                "authority_class": authority_class,
                "execution_integrity": execution_integrity,
                "admission_outcome": admission_outcome,
                "scientific_outcome": scientific_outcome,
                "outcome_policy_generation": 1,
                "investigation_record_id": null
            }))
        })
        .collect()
}

pub(crate) fn authority_adapter_supported(gate_id: &str, authority_class: &str) -> bool {
    authority_class == "NONE"
        || authority_class == "A1"
        || matches!(
            (gate_id, authority_class),
            ("authority-admission-v1", "A0") | ("required-authority-v1", "A3")
        )
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
        for path in string_array(&node["output_paths"], "output_paths")? {
            let bytes = fs::read(confined_output_path(artifact_root, &path)?).map_err(|error| {
                execution_error("GATE-EXEC-ARTIFACT-READ", format!("{path}: {error}"))
            })?;
            artifacts.push(json!({
                "artifact_id": format!("artifact-{}", artifacts.len() + 1),
                "kind": artifact_kind(required_string(node, "artifact_contract")?, &path),
                "path": path,
                "sha256": sha256_bytes(&bytes)
            }));
        }
    }
    Ok(artifacts)
}

fn write_node_artifacts(
    artifact_root: &Path,
    plan: &Value,
    roots: &Value,
    claims: &ExecutionClaims,
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
        let bytes = artifact_bytes(artifact_root, node, run, &log_sha256, &path)?;
        write_atomic(&destination, &bytes)?;
    }
    write_node_checkpoint(artifact_root, plan, roots, claims, node, run)?;
    Ok(())
}

fn write_node_checkpoint(
    artifact_root: &Path,
    plan: &Value,
    roots: &Value,
    claims: &ExecutionClaims,
    node: &Value,
    run: &NodeRun,
) -> Result<()> {
    let node_id = required_string(node, "node_id")?;
    let artifacts = string_array(&node["output_paths"], "output_paths")?
        .into_iter()
        .map(|path| {
            let bytes = fs::read(confined_output_path(artifact_root, &path)?).map_err(|error| {
                execution_error("GATE-EXEC-CHECKPOINT-ARTIFACT", format!("{path}: {error}"))
            })?;
            Ok(json!({"path": path, "sha256": sha256_bytes(&bytes)}))
        })
        .collect::<Result<Vec<_>>>()?;
    let mut checkpoint = json!({
        "schema_version": "openwepp-gate-node-checkpoint-v1",
        "checkpoint_id": "0".repeat(64),
        "node_id": node_id,
        "node_sha256": digest(node)?,
        "reuse_class": node["reuse_class"],
        "result": run.result,
        "attempt": run.attempt,
        "artifacts": artifacts,
        "execution_binding": {
            "plan_id": plan["plan_id"],
            "execution_key": plan["execution_key"],
            "boundary": plan["boundary"],
            "roots": roots,
            "execution_context": plan["execution_context"],
            "policy": plan["policy"],
            "claims": {
                "workflow": claims.workflow,
                "job": claims.job,
                "runner": claims.runner,
                "attempt": claims.attempt,
            }
        },
    });
    checkpoint["checkpoint_id"] = Value::String(derived_id(&checkpoint, "checkpoint_id")?);
    let directory = artifact_root.join(".checkpoints");
    create_confined_directories(artifact_root, &directory)?;
    write_atomic(
        &directory.join(format!("{node_id}.json")),
        &crate::canonical::canonical_bytes(&checkpoint)?,
    )?;
    crate::checkpoint_mirror::mirror_node_checkpoint(artifact_root, node, &checkpoint)
}

pub(crate) fn write_atomic(destination: &Path, bytes: &[u8]) -> Result<()> {
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
    artifact_root: &Path,
    node: &Value,
    run: &NodeRun,
    log_sha256: &str,
    output_path: &str,
) -> Result<Vec<u8>> {
    let contract = required_string(node, "artifact_contract")?;
    if run.result == "PASS"
        && contract == "adjudicated-crap-v1"
        && artifact_kind(contract, output_path) == "CRAP"
    {
        return validated_crap_report_bytes(artifact_root, node);
    }
    if let Some(source) = real_source_for_output(artifact_root, node, output_path)? {
        match fs::symlink_metadata(&source) {
            Ok(_) => return read_real_artifact(artifact_root, &source),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && run.result != "PASS" => {}
            Err(error) => {
                return Err(execution_error(
                    "GATE-EXEC-REAL-ARTIFACT-MISSING",
                    format!("{}: {error}", source.display()),
                ));
            }
        }
    }
    let kind = artifact_kind(contract, output_path);
    if kind == "JUNIT" {
        let failed = usize::from(run.result != "PASS");
        return Ok(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><testsuite name=\"openwepp-gate\" tests=\"0\" failures=\"{failed}\"></testsuite>\n"
        )
        .into_bytes());
    }
    if kind == "LCOV" {
        return Ok(b"TN:\n".to_vec());
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

fn real_source_for_output(
    artifact_root: &Path,
    node: &Value,
    output_path: &str,
) -> Result<Option<PathBuf>> {
    match (
        required_string(node, "artifact_contract")?,
        Path::new(output_path)
            .extension()
            .and_then(|value| value.to_str()),
    ) {
        ("nextest-junit-v1", _) | ("adjudicated-crap-v1", Some("xml")) => {
            nextest_junit_path(artifact_root, node).map(Some)
        }
        ("adjudicated-crap-v1", Some("lcov")) => affected_lcov_path(artifact_root, node).map(Some),
        ("adjudicated-crap-v1", _) => adjudicated_crap_report_path(artifact_root, node).map(Some),
        ("authority-suite-report-v1", _) => authority_report_path(artifact_root, node).map(Some),
        _ => Ok(None),
    }
}

fn read_real_artifact(artifact_root: &Path, source: &Path) -> Result<Vec<u8>> {
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
    let work = fs::canonicalize(work_root(artifact_root))
        .map_err(|error| execution_error("GATE-EXEC-REAL-ARTIFACT-PATH", error.to_string()))?;
    if !canonical.starts_with(work) {
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

pub(crate) fn confined_output_path(root: &Path, relative: &str) -> Result<PathBuf> {
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

fn work_root(artifact_root: &Path) -> PathBuf {
    artifact_root.join(".work")
}

fn cargo_target_root(artifact_root: &Path) -> PathBuf {
    work_root(artifact_root).join("cargo-target")
}

fn result_counts(results: &BTreeMap<String, String>) -> (u64, u64, u64, bool) {
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
        .filter(|result| result.as_str() == "BLOCKED")
        .count() as u64;
    let invalid = results.values().any(|result| result == "INVALID");
    (passed, failed, blocked, invalid)
}

fn aggregate_result(failed: u64, blocked: u64, invalid: bool) -> &'static str {
    if invalid {
        "INVALID"
    } else if failed > 0 {
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

pub(crate) fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
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

pub(crate) fn string_array(value: &Value, label: &str) -> Result<Vec<String>> {
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
#[path = "executor_coverage_tests.rs"]
mod coverage_tests;

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde_json::{Value, json};

    use super::{
        ExecutionClaims, NodeRun, ProcessOutcome, allowed_environment, artifact_bytes,
        create_confined_directories, execute_node, execute_nodes, execute_plan, junit_inventory,
        nextest_junit_path, observed_inventory, observed_source_snapshot, preflight,
        prepare_real_artifacts, read_real_artifact, reject_shell_string, require_relative_path,
        run_process, runtime_arguments, source_snapshot, supported_executor, validate_plan,
        validate_success_artifacts, work_root,
    };
    use crate::canonical::sha256_bytes;
    use crate::planner::{NextestInventory, PlanRequest, Planner, PlanningStage};
    use crate::repository::observe_committed;

    use crate::verifier::{DirectoryArtifacts, verify_receipt};

    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    pub(crate) struct TempDirectory(PathBuf);

    impl TempDirectory {
        pub(crate) fn new(label: &str) -> Self {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "openwepp-gate-executor-{label}-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir(&path).expect("create test directory");
            Self(path)
        }

        pub(crate) fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDirectory {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).expect("remove precise test directory");
        }
    }

    fn process_node(arguments: &[&str], timeout_seconds: u64) -> serde_json::Value {
        json!({
            "node_id": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "executor": {"kind": "PROCESS_V1"},
            "arguments": arguments,
            "working_directory": ".",
            "environment_allowlist": ["PATH"],
            "prerequisites": [],
            "expected_inventory": {"ids": ["bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"]},
            "acceptance": {"expected": 0},
            "timeout_seconds": timeout_seconds,
            "artifact_contract": "process-exit-v1",
            "output_paths": ["result.json"]
        })
    }

    fn prepare_artifacts(label: &str) -> TempDirectory {
        let artifacts = TempDirectory::new(label);
        create_confined_directories(artifacts.path(), &work_root(artifacts.path()))
            .expect("external work root");
        create_confined_directories(
            artifacts.path(),
            &work_root(artifacts.path()).join("cargo-target"),
        )
        .expect("external cargo target");
        artifacts
    }

    fn source_repo() -> PathBuf {
        fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .expect("canonical source repository")
    }

    pub(crate) fn gate_definition(id: &str, arguments: &[&str], prerequisites: &[&str]) -> Value {
        let inventory_source = if id == "affected-adjudicated-crap-v1" {
            "NEXTEST_PACKAGES"
        } else {
            "COMMAND"
        };
        json!({
            "gate_definition_id": id,
            "gate_family": "executor-contract",
            "execution_cost_class": "LIGHT",
            "target_template": "WORKSPACE",
            "risk_classes": ["EDITORIAL", "BOUNDED_COMPONENT"],
            "executor": {"kind": "PROCESS_V1", "version": "process-1", "adapter_sha256": null},
            "arguments_template": arguments,
            "environment_allowlist": ["PATH"],
            "authority_class": "NONE",
            "outcome_policy": "BLOCKING",
            "failure_classification": "HARD_FAIL",
            "owner": "openwepp-maintainers",
            "investigation_owner": "openwepp-maintainers",
            "boundary": "INCREMENT",
            "trust_requirement": "REPOSITORY_REVIEWED",
            "reuse_class": "SAME_EXECUTION",
            "inventory_mode": "EXACT",
            "inventory_source": inventory_source,
            "minimum_count": 1,
            "acceptance": {"kind": "EXIT_CODE", "operator": "EQUALS", "expected": 0, "children": []},
            "timeout_seconds": 5,
            "maximum_attempts": 1,
            "permitted_retry_reasons": [],
            "artifact_contract": "process-exit-v1",
            "output_paths": [format!("target/e2e/{id}.json")],
            "blocks_transition": "INCREMENT",
            "identity_breakers": ["rust-toolchain"],
            "prerequisite_definition_ids": prerequisites
        })
    }

    pub(crate) fn global_quality_gate_definition(
        arguments: &[&str],
        prerequisites: &[&str],
    ) -> Value {
        gate_definition("adjudicated-crap-v1", arguments, prerequisites)
    }

    fn write_json(path: &Path, value: &Value) {
        fs::write(
            path,
            serde_json::to_vec_pretty(value).expect("serialize JSON"),
        )
        .expect("write JSON");
    }

    fn copy_schemas(repo: &Path) {
        let source = source_repo().join("gate-policy/v1/schemas");
        let destination = repo.join("gate-policy/v1/schemas");
        fs::create_dir_all(&destination).expect("create schema directory");
        for entry in fs::read_dir(source).expect("read schemas") {
            let entry = entry.expect("schema entry");
            fs::copy(entry.path(), destination.join(entry.file_name())).expect("copy schema");
        }
    }

    fn git(repo: &Path, arguments: &[&str]) -> String {
        let output = Command::new("git")
            .args(arguments)
            .current_dir(repo)
            .output()
            .expect("run git");
        assert!(
            output.status.success(),
            "git {arguments:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .expect("UTF-8 git output")
            .trim()
            .to_owned()
    }

    fn commit(repo: &Path, message: &str) -> String {
        git(repo, &["add", "."]);
        git(
            repo,
            &[
                "-c",
                "user.name=Codex Test",
                "-c",
                "user.email=codex@example.invalid",
                "commit",
                "-q",
                "-m",
                message,
            ],
        );
        git(repo, &["rev-parse", "HEAD"])
    }

    #[allow(
        clippy::too_many_lines,
        reason = "the isolated repository fixture declares the complete policy wire contract"
    )]
    pub(crate) fn execution_fixture(label: &str, definitions: &[Value]) -> (TempDirectory, Value) {
        let repo = TempDirectory::new(label);
        fs::create_dir_all(repo.path().join("src")).expect("create source directory");
        fs::create_dir_all(repo.path().join("docs/standards")).expect("create standards directory");
        fs::create_dir_all(
            repo.path()
                .join("docs/work-packages/executor/prompts/active"),
        )
        .expect("create package prompt directory");
        fs::create_dir_all(repo.path().join("assurance/v2")).expect("create assurance directory");
        fs::create_dir_all(repo.path().join("gate-policy/v1")).expect("create policy directory");
        fs::create_dir_all(repo.path().join("tools")).expect("create fixture tools");
        for (name, body) in [
            ("pass.sh", "#!/bin/sh\nexit 0\n"),
            ("fail.sh", "#!/bin/sh\nexit 1\n"),
            (
                "mutate.sh",
                "#!/bin/sh\nmkdir -p .github\nprintf 'name: mutation\\n' > .github/probe.yml\n",
            ),
            (
                "mark.sh",
                "#!/bin/sh\nprintf 'ran\\n' > independent-marker\n",
            ),
        ] {
            let path = repo.path().join("tools").join(name);
            fs::write(&path, body).expect("write fixture tool");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
                    .expect("make fixture tool executable");
            }
        }
        fs::write(
            repo.path().join("Cargo.toml"),
            "[package]\nname = \"executor-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .expect("write Cargo manifest");
        fs::write(
            repo.path().join("Cargo.lock"),
            "# This file is automatically @generated by Cargo.\nversion = 4\n\n[[package]]\nname = \"executor-fixture\"\nversion = \"0.1.0\"\n",
        )
        .expect("write Cargo lock");
        fs::write(repo.path().join(".gitignore"), "/target/\n").expect("write ignore rules");
        fs::write(
            repo.path().join("src/lib.rs"),
            "pub fn fixture() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn fixture_works() { super::fixture(); }\n}\n",
        )
        .expect("write source");
        let strategy = b"# Testing strategy\n\nExecutor contract fixture.\n";
        fs::write(
            repo.path()
                .join("docs/standards/testing-and-gate-strategy.md"),
            strategy,
        )
        .expect("write strategy");
        fs::write(
            repo.path()
                .join("docs/work-packages/executor/prompts/active/kickoff.md"),
            "# Executor fixture kickoff\n",
        )
        .expect("write active prompt");
        fs::write(
            repo.path()
                .join("docs/work-packages/executor/package.md"),
            "# Executor fixture\n\nStatus: ACTIVE\n\nbase\n\n## Declared Write Set\n\n- `docs/work-packages/**`\n- `src/**`\n",
        )
        .expect("write base package");
        copy_schemas(repo.path());
        fs::write(
            repo.path().join("assurance/v2/catalog.yaml"),
            "reports:\n- id: executor-fixture-report\n",
        )
        .expect("write assurance catalog");
        fs::write(
            repo.path().join("assurance/v2/principals.yaml"),
            "principals: []\n",
        )
        .expect("write assurance principals");
        fs::create_dir_all(
            repo.path()
                .join("assurance/v2/reports/executor-fixture-report"),
        )
        .expect("create fixture report directory");
        fs::write(
            repo.path()
                .join("assurance/v2/reports/executor-fixture-report/report.yaml"),
            "id: executor-fixture-report\nauthorship:\n  human_report_lead: null\n",
        )
        .expect("write fixture report lifecycle");
        write_json(
            &repo
                .path()
                .join("assurance/v2/reports/executor-fixture-report/review.lock.json"),
            &json!({
                "report_id": "executor-fixture-report",
                "science_root": "1".repeat(64),
                "preapproval_realization_root": "2".repeat(64),
                "realization_root": null
            }),
        );
        write_json(
            &repo.path().join("gate-policy/v1/assurance-registry.json"),
            &json!({
                "schema_version": "openwepp-assurance-registry-v1",
                "policy_id": "ADR-0039",
                "generation": 1,
                "reports": [{
                    "report_id": "executor-fixture-report",
                    "watch_generation": 1,
                    "source_root": "1".repeat(64),
                    "assessed_realization_root": "2".repeat(64),
                    "resolution_authority": {
                        "principal_id": null,
                        "role_id": "assurance_steward",
                        "role_record_sha256": null
                    },
                    "watches": [{
                        "watch_id": "executor-fixture-watch",
                        "owner": "fixture owner",
                        "kind": "exact_path",
                        "match_value": "assurance/fixture-only",
                        "lifecycle_boundary": "CAMPAIGN_CLOSURE"
                    }]
                }]
            }),
        );
        fs::copy(
            source_repo().join("gate-policy/v1/execution-matrix.json"),
            repo.path().join("gate-policy/v1/execution-matrix.json"),
        )
        .expect("copy execution matrix");
        let gate_ids = definitions
            .iter()
            .map(|definition| definition["gate_definition_id"].clone())
            .collect::<Vec<_>>();
        write_json(
            &repo.path().join("gate-policy/v1/impact-map.json"),
            &json!({
                "schema_version": "openwepp-gate-impact-map-v1",
                "policy_id": "ADR-0039",
                "policy_sha256": sha256_bytes(strategy),
                "generation": 1,
                "enforcement_status": "BLOCKING",
                "unknown_path_action": "ESCALATE_CRITICAL",
                "entries": [{
                    "entry_id": "executor-fixture",
                    "matcher": {"kind": "exact_path", "value": "src/lib.rs"},
                    "owner": "openwepp-maintainers",
                    "semantic_surface": "executor-contract",
                    "risk_floor": "BOUNDED_COMPONENT",
                    "reason_codes": ["EXECUTOR_FIXTURE_CHANGED"],
                    "affected_packages": ["executor-fixture"],
                    "test_targets": [],
                    "covering_test_targets": [],
                    "contracts": [],
                    "authority_suites": [],
                    "assurance_watches": [],
                    "gate_definition_ids": gate_ids,
                    "documentation_paths": []
                }, {
                    "entry_id": "executor-package-fixture",
                    "matcher": {"kind": "exact_path", "value": "docs/work-packages/executor/package.md"},
                    "owner": "openwepp-maintainers",
                    "semantic_surface": "executor-contract",
                    "risk_floor": "BOUNDED_COMPONENT",
                    "reason_codes": ["EXECUTOR_FIXTURE_CHANGED"],
                    "affected_packages": ["executor-fixture"],
                    "test_targets": [],
                    "covering_test_targets": [],
                    "contracts": [],
                    "authority_suites": [],
                    "assurance_watches": [],
                    "gate_definition_ids": gate_ids,
                    "documentation_paths": []
                }]
            }),
        );
        write_json(
            &repo.path().join("gate-policy/v1/gate-definitions.json"),
            &json!({
                "schema_version": "openwepp-gate-definitions-v1",
                "generation": 1,
                "enforcement_status": "BLOCKING",
                "definitions": definitions
            }),
        );
        git(repo.path(), &["init", "-q"]);
        let base = commit(repo.path(), "baseline");
        fs::write(
            repo.path().join("src/lib.rs"),
            "pub fn fixture() { let _changed = true; }\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn fixture_works() { super::fixture(); }\n}\n",
        )
        .expect("change fixture");
        fs::write(
            repo.path()
                .join("docs/work-packages/executor/package.md"),
            "# Executor fixture\n\nStatus: ACTIVE\n\nchanged\n\n## Declared Write Set\n\n- `docs/work-packages/**`\n- `src/**`\n",
        )
        .expect("change package");
        let head = commit(repo.path(), "change fixture");
        let authority = crate::package_validation::validate_package_chain(
            repo.path(),
            &base,
            Some(&head),
            Path::new("docs/work-packages/executor/package.md"),
        )
        .expect("fixture package authority");
        let source =
            observe_committed(repo.path(), &base, &head).expect("observe committed source");
        let plan = Planner::new(NextestInventory)
            .build(
                repo.path(),
                &PlanRequest {
                    stage: PlanningStage::Terminal,
                    predecessor_intent_plan_id: Some("11".repeat(32)),
                    boundary: "INCREMENT".to_owned(),
                    campaign_id: Some("TESTGATE-CI-01".to_owned()),
                    combined_quality_proof_id: None,
                    authorized_paths: vec![
                        "docs/work-packages/executor/package.md".to_owned(),
                        "src/lib.rs".to_owned(),
                    ],
                    package_authority_chain_id: authority["package_authority_chain_id"]
                        .as_str()
                        .expect("authority chain ID")
                        .to_owned(),
                    intent_package_path: "docs/work-packages/executor/package.md".to_owned(),
                    source,
                },
            )
            .expect("build terminal plan");
        (repo, plan)
    }

    fn execute_and_verify(repo: &Path, plan: &Value, artifacts: &Path) -> Value {
        assert_eq!(
            observed_source_snapshot(repo, plan).expect("observed pre-execution snapshot"),
            source_snapshot(plan).expect("planned pre-execution snapshot"),
            "fixture status: {}",
            git(repo, &["status", "--short"])
        );
        let receipt = execute_plan(repo, plan, artifacts, &ExecutionClaims::default())
            .expect("execute terminal plan");
        let verdict = verify_receipt(
            repo,
            plan,
            &receipt,
            &DirectoryArtifacts::new(artifacts.to_owned()),
        )
        .expect("verify execution receipt");
        assert_eq!(
            verdict.result(),
            receipt["result"].as_str().expect("result")
        );
        receipt
    }

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
    fn rejects_inline_shell_programs_before_spawn() {
        for arguments in [
            json!(["sh", "-c", "exit 0"]),
            json!(["bash", "-lc", "exit 0"]),
            json!(["bash", "-o", "pipefail", "-c", "exit 0"]),
            json!(["bash", "--command", "exit 0"]),
            json!(["busybox", "sh", "-c", "exit 0"]),
            json!(["busybox", "ash", "-c", "exit 0"]),
            json!(["ksh", "-c", "exit 0"]),
            json!(["bash", "-O", "extglob", "-c", "exit 0"]),
            json!(["bash", "--init-file", "file", "-c", "exit 0"]),
            json!(["env", "-i", "bash", "-c", "exit 0"]),
            json!(["env", "FOO=crash", "/bin/bash", "-c", "exit 0"]),
            json!(["powershell", "-Command", "exit 0"]),
        ] {
            let error = reject_shell_string(&json!({"arguments": arguments}))
                .expect_err("inline shell must fail closed");
            assert_eq!(error.code, "GATE-EXEC-SHELL-STRING");
        }
        reject_shell_string(&json!({"arguments": ["bash", "adapter.sh", "-c"]}))
            .expect("script path with its own -c option is not an inline shell string");
    }

    #[test]
    fn identity_inventory_and_output_collisions_fail_before_spawn() {
        let (repo, plan) = execution_fixture(
            "preflight-repo",
            &[
                global_quality_gate_definition(&["./tools/pass.sh"], &[]),
                gate_definition("fixture-command-v1", &["./tools/pass.sh"], &[]),
            ],
        );
        let artifacts = prepare_artifacts("preflight-artifacts");

        let mut malformed = plan.clone();
        malformed["plan_id"] = json!("0".repeat(64));
        let error = validate_plan(repo.path(), artifacts.path(), &malformed, true)
            .expect_err("malformed identity must fail");
        assert_eq!(error.code, "GATE-PLAN-IDENTITY");

        let mut drifted = plan.clone();
        drifted["nodes"][0]["expected_inventory"]["ids"] = json!(["0".repeat(64)]);
        let error = preflight(
            repo.path(),
            artifacts.path(),
            &drifted,
            &BTreeSet::new(),
            true,
        )
        .expect_err("inventory drift must fail");
        assert_eq!(error.code, "GATE-EXEC-INVENTORY-DRIFT");

        let mut colliding = plan;
        colliding["nodes"][1]["output_paths"] = colliding["nodes"][0]["output_paths"].clone();
        let error = preflight(
            repo.path(),
            artifacts.path(),
            &colliding,
            &BTreeSet::new(),
            false,
        )
        .expect_err("output collision must fail");
        assert_eq!(error.code, "GATE-EXEC-OUTPUT-COLLISION");
    }

    #[test]
    fn zero_work_dispatches_no_process_attempts() {
        let repo = TempDirectory::new("zero-work-repo");
        let artifacts = prepare_artifacts("zero-work-artifacts");
        let record = execute_nodes(
            repo.path(),
            artifacts.path(),
            &json!({"nodes": []}),
            &json!({}),
            "unchanged",
        )
        .expect("zero-work execution");
        assert!(record.attempts.is_empty());
        assert!(record.final_results.is_empty());
        assert!(record.executed_inventory.is_empty());
    }

    #[test]
    fn environment_is_exactly_allowlisted() {
        let environment = allowed_environment(&json!({"environment_allowlist": ["PATH"]}))
            .expect("allowlisted environment");
        assert_eq!(environment.len(), 1);
        assert_eq!(environment.get("PATH"), std::env::var("PATH").ok().as_ref());
        assert!(!environment.contains_key("HOME"));
    }

    #[test]
    fn process_fail_spawn_block_and_timeout_are_observed() {
        let repo = fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .expect("canonical repository");

        let failed_artifacts = prepare_artifacts("fail");
        let failed_log = failed_artifacts.path().join("failed.log");
        let failed = run_process(
            &repo,
            failed_artifacts.path(),
            &process_node(&["false"], 5),
            &failed_log,
        )
        .expect("failing process outcome");
        assert_eq!(failed.result, "FAIL");
        assert_eq!(failed.exit_code, Some(1));
        assert_eq!(failed.termination_signal, None);

        let spawn_artifacts = prepare_artifacts("spawn");
        let spawn_log = spawn_artifacts.path().join("spawn.log");
        let blocked = run_process(
            &repo,
            spawn_artifacts.path(),
            &process_node(&["openwepp-command-that-does-not-exist"], 5),
            &spawn_log,
        )
        .expect("spawn failure outcome");
        assert_eq!(blocked.result, "BLOCKED");
        assert_eq!(
            blocked.unavailable_reason.as_deref(),
            Some("PROCESS_SPAWN_FAILED")
        );

        let timeout_artifacts = prepare_artifacts("timeout");
        let timeout_log = timeout_artifacts.path().join("timeout.log");
        let timed_out = run_process(
            &repo,
            timeout_artifacts.path(),
            &process_node(&["sleep", "2"], 1),
            &timeout_log,
        )
        .expect("timeout outcome");
        assert_eq!(timed_out.result, "BLOCKED");
        assert_eq!(timed_out.unavailable_reason.as_deref(), Some("TIMEOUT"));

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let signal_artifacts = prepare_artifacts("signal");
            let script = signal_artifacts.path().join("terminate.sh");
            fs::write(&script, "#!/bin/sh\nkill -TERM $$\n").expect("signal script");
            fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
                .expect("signal script executable");
            let script_text = script.to_str().expect("UTF-8 signal script");
            let signalled = run_process(
                &repo,
                signal_artifacts.path(),
                &process_node(&[script_text], 5),
                &signal_artifacts.path().join("signal.log"),
            )
            .expect("signal outcome");
            assert_eq!(signalled.result, "FAIL");
            assert_eq!(signalled.exit_code, None);
            assert_eq!(signalled.termination_signal, Some(15));
        }
    }

    #[test]
    fn failed_junit_node_reports_only_observed_inventory() {
        let artifacts = prepare_artifacts("partial-junit");
        let mut node = process_node(&["false"], 5);
        node["artifact_contract"] = json!("nextest-junit-v1");
        node["executor"] = json!({"kind": "NEXTEST_V1"});
        node["arguments"] = json!(["cargo", "nextest", "run", "--profile", "affected"]);
        let observed_id = sha256_bytes(b"rust-suites::fixture\0works");
        let missing_id = sha256_bytes(b"rust-suites::fixture\0missing");
        node["expected_inventory"]["ids"] = json!([observed_id, missing_id]);
        let junit = nextest_junit_path(artifacts.path(), &node).expect("JUnit path");
        fs::create_dir_all(junit.parent().expect("JUnit parent")).expect("JUnit directory");
        fs::write(
            &junit,
            "<testsuite>\n<testcase classname=\"fixture\" name=\"works\"/>\n</testsuite>\n",
        )
        .expect("partial JUnit");
        let outcome = ProcessOutcome {
            exit_code: Some(1),
            termination_signal: None,
            result: "FAIL".to_owned(),
            unavailable_reason: None,
        };
        let (executed, reason) =
            observed_inventory(artifacts.path(), &node, &outcome).expect("observed inventory");
        assert_eq!(executed, BTreeSet::from([observed_id]));
        assert_eq!(reason.as_deref(), Some("TEST_NOT_EXECUTED"));
    }

    #[test]
    fn blocked_prerequisite_emits_an_observed_attempt() {
        let repo = fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .expect("canonical repository");
        let artifacts = prepare_artifacts("prerequisite");
        let prerequisite = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
        let mut node = process_node(&["true"], 5);
        node["prerequisites"] = json!([prerequisite]);
        let results =
            std::collections::BTreeMap::from([(prerequisite.to_owned(), "FAIL".to_owned())]);
        let run = execute_node(&repo, artifacts.path(), &node, &results, None)
            .expect("blocked dependent attempt");
        assert_eq!(run.result, "BLOCKED");
        assert!(run.attempt["exit_code"].is_null());
        assert_eq!(
            run.unavailable_reason.as_deref(),
            Some("PREREQUISITE_NONPASS")
        );
    }

    #[test]
    fn executor_injects_only_external_work_paths() {
        let repo = TempDirectory::new("external-repo");
        let artifacts = prepare_artifacts("external-artifacts");
        let log = artifacts.path().join("environment.log");
        let outcome = run_process(
            repo.path(),
            artifacts.path(),
            &process_node(&["env"], 5),
            &log,
        )
        .expect("environment process");
        assert_eq!(outcome.result, "PASS");
        let environment = fs::read_to_string(log).expect("environment log");
        assert!(environment.contains(&format!(
            "CARGO_TARGET_DIR={}",
            artifacts.path().join(".work/cargo-target").display()
        )));
        assert!(environment.contains(&format!(
            "OPENWEPP_GATE_ARTIFACT_ROOT={}",
            artifacts.path().join(".work").display()
        )));
        let temporary = environment
            .lines()
            .find_map(|line| line.strip_prefix("TMPDIR="))
            .expect("process temporary root");
        #[cfg(unix)]
        assert!(temporary.len() <= 40);
        assert!(!Path::new(temporary).exists());
        assert!(!repo.path().join("target").exists());
    }

    #[test]
    fn executor_binds_nested_crap_nextest_to_the_qualified_config() {
        let repo = source_repo();
        let artifacts = prepare_artifacts("nested-crap-config");
        let log = artifacts.path().join("environment.log");
        let mut node = process_node(&["env"], 5);
        node["artifact_contract"] = json!("adjudicated-crap-v1");
        let outcome = run_process(&repo, artifacts.path(), &node, &log)
            .expect("nested CRAP environment process");
        assert_eq!(outcome.result, "PASS");
        let environment = fs::read_to_string(log).expect("environment log");
        let config = environment
            .lines()
            .find_map(|line| line.strip_prefix("OPENWEPP_GATE_NEXTEST_CONFIG="))
            .map(PathBuf::from)
            .expect("nested Nextest config");
        assert_eq!(config, artifacts.path().join(".work/nextest.toml"));
        let contents = fs::read_to_string(config).expect("qualified nested config");
        assert!(contents.contains("[test-groups.assurance-publication]\nmax-threads = 2"));
        assert!(contents.contains(&format!(
            "dir = \"{}\"",
            artifacts.path().join(".work/nextest").display()
        )));
    }

    #[test]
    fn nextest_runtime_arguments_confine_build_and_report_stores() {
        let repo = source_repo();
        let artifacts = prepare_artifacts("nextest-config");
        let node = json!({
            "executor": {"kind": "NEXTEST_V1"},
            "arguments": ["cargo", "nextest", "run", "--workspace"]
        });
        let arguments =
            runtime_arguments(&repo, artifacts.path(), &node).expect("external Nextest arguments");
        assert!(
            arguments.contains(
                &artifacts
                    .path()
                    .join(".work/cargo-target")
                    .display()
                    .to_string()
            )
        );
        let config = artifacts.path().join(".work/nextest.toml");
        assert!(arguments.contains(&config.display().to_string()));
        let contents = fs::read_to_string(config).expect("external Nextest config");
        assert!(contents.contains(&format!(
            "dir = \"{}\"",
            artifacts.path().join(".work/nextest").display()
        )));
        assert!(contents.contains("[test-groups.assurance-publication]\nmax-threads = 2"));
        assert!(!contents.contains("[test-groups.assurance-publication]\nmax-threads = 4"));
    }

    #[test]
    #[allow(clippy::too_many_lines)] // Complete combined artifact contract fixture.
    fn combined_quality_artifacts_are_external_resettable_and_inventory_checked() {
        let artifacts = prepare_artifacts("combined-artifacts");
        let mut node = json!({
            "gate_definition_id": "combined-workspace-quality-v1",
            "artifact_contract": "adjudicated-crap-v1",
            "arguments": [
                "bash", "adapter.sh", "--nextest-profile", "full",
                "--output-dir", "target/combined-quality"
            ],
            "expected_inventory": {"ids": ["placeholder"]},
            "output_paths": ["report.json", "junit.xml", "workspace.lcov"]
        });
        let output = artifacts.path().join(".work/target/combined-quality");
        let junit = output.join("nextest/full/junit.xml");
        fs::create_dir_all(junit.parent().expect("JUnit parent")).expect("create JUnit parent");
        fs::write(
            &junit,
            "<testsuite>\n<testcase classname=\"fixture\" name=\"works\"/>\n</testsuite>\n",
        )
        .expect("write JUnit");
        let report = output.join("adjudicated-crap-report.json");
        let control = output.join("run-status.json");
        let report_bytes = b"{\"acquisition_mode\":\"fresh\",\"closure_eligible\":true,\"lcov_sha256\":\"ad78bcf9de2caa140900bda1f8c4979af5f3f5c069f5eda785475f7427a306e0\",\"status\":\"PASS\"}\n";
        fs::write(&report, report_bytes).expect("write CRAP report");
        fs::write(
            &control,
            serde_json::to_vec(&json!({
                "acquisition_mode": "fresh",
                "adjudicated_crap_report_sha256": sha256_bytes(report_bytes),
                "exit_status": 0,
                "finished_utc": "2026-07-19T00:00:01Z",
                "result": "PASS",
                "started_utc": "2026-07-19T00:00:00Z"
            }))
            .expect("serialize CRAP control"),
        )
        .expect("write CRAP control");
        fs::write(output.join("workspace.lcov"), "TN:\n").expect("write LCOV");
        let inventory = junit_inventory(&junit).expect("JUnit inventory");
        assert_eq!(inventory.len(), 1);
        node["expected_inventory"]["ids"] = json!(inventory);
        validate_success_artifacts(artifacts.path(), &node).expect("combined artifacts");
        fs::write(&report, b"{\"status\":\"PASS\",\"coverage\":1.0}\n")
            .expect("tamper CRAP report");
        let run = NodeRun {
            attempt: json!({}),
            result: "PASS".to_owned(),
            log_path: output.join("unused.log"),
            executed_inventory: BTreeSet::new(),
            unavailable_reason: None,
        };
        let error = artifact_bytes(artifacts.path(), &node, &run, "unused", "report.json")
            .expect_err("published bytes must be revalidated after prior success validation");
        assert_eq!(error.code, "GATE-EXEC-CRAP-REPORT-DIGEST");
        fs::write(&report, report_bytes).expect("restore CRAP report");
        fs::write(output.join("workspace.lcov"), "TN:tampered\n").expect("tamper LCOV");
        let error = validate_success_artifacts(artifacts.path(), &node)
            .expect_err("published LCOV must match the CRAP report");
        assert_eq!(error.code, "GATE-EXEC-CRAP-LCOV-LINEAGE");
        fs::write(output.join("workspace.lcov"), "TN:\n").expect("restore LCOV");
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;

            fs::remove_file(&control).expect("remove CRAP control");
            symlink(&report, &control).expect("symlink CRAP control");
            let error = validate_success_artifacts(artifacts.path(), &node)
                .expect_err("control symlinks must fail confinement");
            assert_eq!(error.code, "GATE-EXEC-REAL-ARTIFACT-TYPE");
            fs::remove_file(&control).expect("remove CRAP control symlink");
        }
        fs::write(
            &control,
            format!(
                "{{\"adjudicated_crap_report_sha256\":\"{}\",\"exit_status\":0.0,\"result\":\"PASS\"}}\n",
                sha256_bytes(report_bytes)
            ),
        )
        .expect("write floating control");
        let error = validate_success_artifacts(artifacts.path(), &node)
            .expect_err("control envelope must remain integer-only");
        assert_eq!(error.code, "GATE-JSON-INVALID");
        fs::write(
            &control,
            serde_json::to_vec(&json!({
                "adjudicated_crap_report_sha256": sha256_bytes(report_bytes),
                "exit_status": 2,
                "result": "FAIL"
            }))
            .expect("serialize failed CRAP control"),
        )
        .expect("write failed CRAP control");
        let error = validate_success_artifacts(artifacts.path(), &node)
            .expect_err("non-PASS control must fail closed");
        assert_eq!(error.code, "GATE-EXEC-CRAP-CONTROL");
        prepare_real_artifacts(artifacts.path(), &node).expect("reset real artifacts");
        assert!(!junit.exists());
        assert!(!report.exists());
        assert!(!control.exists());
        prepare_real_artifacts(artifacts.path(), &node).expect("absent artifacts remain reset");
        let retained = artifacts.path().join(".work/retained.log");
        fs::write(&retained, "retained\n").expect("write retained artifact");
        assert_eq!(
            read_real_artifact(artifacts.path(), &retained).expect("read confined artifact"),
            b"retained\n"
        );
    }

    #[test]
    fn terminal_plan_executes_and_independent_verifier_accepts_pass_receipt() {
        let (repo, plan) = execution_fixture(
            "e2e-pass-repo",
            &[global_quality_gate_definition(&["./tools/pass.sh"], &[])],
        );
        let artifacts = TempDirectory::new("e2e-pass-artifacts");
        let receipt = execute_and_verify(repo.path(), &plan, artifacts.path());
        assert_eq!(receipt["result"], "PASS");
        assert_eq!(receipt["counts"]["passed"], 1);
        assert!(!repo.path().join("target").exists());
        assert!(artifacts.path().join(".work/cargo-target").is_dir());
        assert!(artifacts.path().join(".work/graph-snapshots").is_dir());
        assert!(artifacts.path().join(".work/inventory-snapshots").is_dir());
        assert!(!artifacts.path().join(".work/reconstruction").exists());
        assert!(!artifacts.path().join(".verification").exists());
    }

    #[test]
    fn terminal_plan_preserves_fail_and_blocked_attempts_in_verified_receipt() {
        let (repo, plan) = execution_fixture(
            "e2e-nonpass-repo",
            &[
                global_quality_gate_definition(&["./tools/fail.sh"], &[]),
                gate_definition("fixture-dependent-v1", &["true"], &["adjudicated-crap-v1"]),
            ],
        );
        let artifacts = TempDirectory::new("e2e-nonpass-artifacts");
        let receipt = execute_and_verify(repo.path(), &plan, artifacts.path());
        assert_eq!(receipt["result"], "FAIL");
        assert_eq!(receipt["counts"]["failed"], 1);
        assert_eq!(receipt["counts"]["blocked"], 1);
        assert_eq!(receipt["counts"]["skipped"], 1);
        let results = receipt["attempts"]
            .as_array()
            .expect("attempts")
            .iter()
            .map(|attempt| attempt["result"].as_str().expect("attempt result"))
            .collect::<Vec<_>>();
        assert_eq!(results, ["FAIL", "BLOCKED"]);
        assert_eq!(
            receipt["unavailable_items"][0]["reason_code"],
            "PREREQUISITE_NONPASS"
        );
    }

    #[test]
    fn terminal_plan_detects_out_of_manifest_source_mutation_and_verifies_invalid_receipt() {
        let (repo, plan) = execution_fixture(
            "e2e-mutation-repo",
            &[
                global_quality_gate_definition(&["./tools/mutate.sh"], &[]),
                gate_definition("fixture-independent-v1", &["./tools/mark.sh"], &[]),
            ],
        );
        let artifacts = TempDirectory::new("e2e-mutation-artifacts");
        let receipt = execute_and_verify(repo.path(), &plan, artifacts.path());
        assert_eq!(receipt["result"], "INVALID");
        assert_eq!(receipt["attempts"][0]["result"], "INVALID");
        assert_eq!(receipt["attempts"][1]["result"], "BLOCKED");
        assert_eq!(receipt["source_mutation_check"]["unchanged"], false);
        assert_ne!(
            receipt["source_mutation_check"]["before_sha256"],
            receipt["source_mutation_check"]["after_sha256"]
        );
        assert!(repo.path().join(".github/probe.yml").is_file());
        assert!(!repo.path().join("independent-marker").exists());
    }

    #[test]
    fn mutation_snapshot_covers_paths_outside_manifest_filters() {
        let repo = TempDirectory::new("mutation-repo");
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo.path())
            .status()
            .expect("git init");
        fs::write(repo.path().join("README.md"), "baseline\n").expect("baseline file");
        Command::new("git")
            .args(["add", "README.md"])
            .current_dir(repo.path())
            .status()
            .expect("git add");
        Command::new("git")
            .args([
                "-c",
                "user.name=Codex Test",
                "-c",
                "user.email=codex@example.invalid",
                "commit",
                "-q",
                "-m",
                "baseline",
            ])
            .current_dir(repo.path())
            .status()
            .expect("git commit");
        let head = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(repo.path())
            .output()
            .expect("git head");
        let head = String::from_utf8(head.stdout)
            .expect("UTF-8 head")
            .trim()
            .to_owned();
        let plan = json!({
            "source": {
                "base_commit": head,
                "head_commit": head,
                "dirty_tree_digest": null,
                "index_digest": null,
                "worktree_digest": null,
                "untracked_digest": null
            },
            "environment_roots": {
                "execution_root": "11".repeat(32),
                "authority_root": "22".repeat(32),
                "documentation_root": "33".repeat(32),
                "assurance_root": null
            }
        });
        assert_eq!(
            observed_source_snapshot(repo.path(), &plan).expect("clean snapshot"),
            source_snapshot(&plan).expect("planned snapshot")
        );
        fs::create_dir(repo.path().join(".github")).expect("workflow directory");
        fs::write(repo.path().join(".github/probe.yml"), "name: mutation\n")
            .expect("untracked workflow mutation");
        assert_ne!(
            observed_source_snapshot(repo.path(), &plan).expect("mutated snapshot"),
            source_snapshot(&plan).expect("planned snapshot")
        );
    }
}
