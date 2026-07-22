//! Canonical admission artifact for the LIGHT-to-HEAVY execution transition.

use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use serde_json::{Value, json};

use crate::canonical::{
    canonical_bytes, current_executable_sha256, derived_id, digest, parse_strict, sha256_bytes,
    validate_schema,
};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::executor::ExecutionClaims;
use crate::package_validation::validate_package;
use crate::planner::{reconstruct_plan_in, verify_plan_identity};
use crate::repository::remove_reconstruction_workspace;

pub const CHECK_IDS: [&str; 10] = [
    "PACKAGE_ADMISSION",
    "CHEAP_PREREQUISITES",
    "INVENTORY_AND_ARGUMENTS",
    "EXECUTION_IDENTITIES",
    "ATTEMPT_AND_OUTPUT_ISOLATION",
    "ROOTS_AND_EVIDENCE_REUSE",
    "COMBINED_FULL_COVERAGE",
    "ORDERING_RETRY_AND_HANDOFF",
    "DURABLE_ATTEMPT_LEDGER",
    "OPEN_TOOLING_DEFECTS",
];

const FAILURE_CHECK_INDEX_RULES: &[(&[&str], usize)] = &[
    (&["PACKAGE"], 0),
    (&["LIGHT", "DOC", "LINE"], 1),
    (&["INVENTORY", "PLAN"], 2),
    (&["EXECUT", "CLAIM"], 3),
    (&["ARTIFACT", "CHECKPOINT", "COLLISION"], 4),
    (&["ROOT", "CACHE"], 5),
    (&["COMBIN"], 6),
    (&["ORDER", "RETRY", "HANDOFF"], 7),
    (&["LEDGER"], 8),
];

/// An audit value constructed by the repository-owned implementation.
///
/// The private field prevents execution APIs from accepting caller-synthesized
/// READY JSON as proof that independent reconstruction occurred.
#[derive(Debug)]
pub struct ConstructedAudit(Value);

impl ConstructedAudit {
    #[must_use]
    pub fn as_value(&self) -> &Value {
        &self.0
    }
}

/// Construct and retain provenance for one audit decision.
///
/// # Errors
///
/// Returns an error only when neither the ordinary nor fallback audit can be
/// represented as a valid canonical document.
pub fn construct_audit(
    repo: &Path,
    plan: &Value,
    light_receipt: &Value,
    artifact_root: &Path,
    ledger: &Path,
) -> Result<ConstructedAudit> {
    let audit = match build_audit(repo, plan, light_receipt, artifact_root, ledger) {
        Ok(audit) => audit,
        Err(failure) => {
            build_failure_audit(repo, plan, light_receipt, artifact_root, ledger, &failure)?
        }
    };
    Ok(ConstructedAudit(audit))
}

/// Build the only artifact that may authorize the heavy execution stage.
///
/// # Errors
///
/// Returns a typed error when an input cannot be parsed or the produced report
/// violates its schema or derived identity.
#[allow(
    clippy::too_many_lines,
    reason = "the ten canonical checks must be assembled and sealed in one audit transaction"
)]
pub fn build_audit(
    repo: &Path,
    plan: &Value,
    light_receipt: &Value,
    artifact_root: &Path,
    ledger: &Path,
) -> Result<Value> {
    verify_plan_identity(plan)?;
    validate_stage_receipt(repo, plan, light_receipt, artifact_root, true)?;
    let ledger_head_sha256 = ledger_head(ledger).ok().flatten();

    let mut checks = Vec::with_capacity(CHECK_IDS.len());
    let package_admission = package_admission(repo, plan)?;
    checks.push(check(
        CHECK_IDS[0],
        package_admitted(plan, &package_admission),
        package_admission.clone(),
    )?);
    checks.push(check(
        CHECK_IDS[1],
        cheap_prerequisites(repo, plan, light_receipt),
        json!({"light_results": light_receipt["final_results"]}),
    )?);
    checks.push(check(
        CHECK_IDS[2],
        inventory_and_arguments_are_exact(repo, plan, artifact_root),
        json!({"nodes": plan["nodes"]}),
    )?);
    checks.push(check(
        CHECK_IDS[3],
        execution_identities(plan, light_receipt),
        json!({
            "policy": plan["policy"],
            "context": plan["execution_context"],
            "claims": light_receipt["claims"],
            "executor_binary_sha256": light_receipt["executor_binary_sha256"],
        }),
    )?);
    checks.push(check(
        CHECK_IDS[4],
        light_attempt_isolated(plan, light_receipt, artifact_root),
        json!({"artifact_root_sha256": path_digest(artifact_root)}),
    )?);
    checks.push(check(
        CHECK_IDS[5],
        separated_roots(plan),
        json!({"roots": plan["environment_roots"]}),
    )?);
    let combined_execution = plan["combined_quality"].clone();
    checks.push(check(
        CHECK_IDS[6],
        validate_combined_decision(plan),
        combined_execution.clone(),
    )?);
    checks.push(check(
        CHECK_IDS[7],
        valid_stage_order(plan),
        json!({"nodes": node_manifest(plan)?}),
    )?);
    checks.push(check(
        CHECK_IDS[8],
        durable_ledger(ledger),
        json!({"ledger_path_sha256": path_digest(ledger)}),
    )?);
    checks.push(check(
        CHECK_IDS[9],
        no_open_tooling_defect_at_head(ledger, ledger_head_sha256.as_deref()),
        json!({
            "ledger_path_sha256": path_digest(ledger),
            "ledger_head_sha256": ledger_head_sha256,
        }),
    )?);

    let reason_codes = checks
        .iter()
        .flat_map(|item| item["reason_codes"].as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let status = if checks.iter().any(|item| item["status"] == "INVALID") {
        "INVALID"
    } else if checks.iter().any(|item| item["status"] == "BLOCKED") {
        "BLOCKED"
    } else {
        "READY"
    };
    let mut audit = json!({
        "schema_version": "openwepp-pre-heavy-audit-v1",
        "audit_id": "0".repeat(64),
        "status": status,
        "reason_codes": reason_codes,
        "plan_id": plan["plan_id"],
        "plan_sha256": digest(plan)?,
        "execution_key": plan["execution_key"],
        "executor_binary_sha256": light_receipt["executor_binary_sha256"],
        "light_stage_receipt_id": light_receipt["stage_receipt_id"],
        "artifact_root_sha256": path_digest(artifact_root),
        "ledger_path_sha256": path_digest(ledger),
        "ledger_head_sha256": ledger_head_sha256,
        "node_manifest": node_manifest(plan)?,
        "package_admission": package_admission,
        "checks": checks,
        "combined_execution": combined_execution,
        "light_receipt": light_receipt,
    });
    audit["audit_id"] = Value::String(derived_id(&audit, "audit_id")?);
    let schema = read_json(&repo.join("gate-policy/v1/schemas/pre-heavy-audit.schema.json"))?;
    validate_schema(&schema, &audit, "pre-heavy audit")?;
    if derived_id(&audit, "audit_id")? != string(&audit, "audit_id")? {
        return Err(audit_error(
            "GATE-AUDIT-IDENTITY",
            "generated audit identity mismatch",
        ));
    }
    Ok(audit)
}

/// Build a schema-valid INVALID audit when a representable transaction fails
/// before the ordinary ten checks can be assembled.
///
/// # Errors
///
/// Returns a typed error only when the fallback artifact itself cannot be
/// canonicalized or validated against the audit schema.
pub fn build_failure_audit(
    repo: &Path,
    plan: &Value,
    light_receipt: &Value,
    artifact_root: &Path,
    ledger: &Path,
    failure: &GatePolicyError,
) -> Result<Value> {
    let plan_sha = digest(plan)?;
    let plan_id = plan["plan_id"]
        .as_str()
        .filter(|value| is_digest(value))
        .unwrap_or(&plan_sha);
    let execution_key = plan["execution_key"]
        .as_str()
        .filter(|value| is_digest(value))
        .unwrap_or(&plan_sha);
    let light_id = light_receipt["stage_receipt_id"]
        .as_str()
        .filter(|value| is_digest(value))
        .map(str::to_owned)
        .unwrap_or(digest(light_receipt)?);
    let binary = current_executable_sha256()?;
    let failed_index = failure_check_index(failure.code);
    let failed_status = failure_status(failure);
    let checks = CHECK_IDS
        .iter()
        .enumerate()
        .map(|(index, id)| {
            Ok(json!({
                "check_id": id,
                "status": if index == failed_index {failed_status} else {"BLOCKED"},
                "reason_codes": if index == failed_index {vec![failure.code]} else {vec!["PREREQUISITE_UNAVAILABLE"]},
                "evidence_sha256": digest(&json!({"failure_code": failure.code, "check_id": id}))?,
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    let mut audit = json!({
        "schema_version": "openwepp-pre-heavy-audit-v1",
        "audit_id": "0".repeat(64),
        "status": failed_status,
        "reason_codes": [failure.code],
        "plan_id": plan_id,
        "plan_sha256": plan_sha,
        "execution_key": execution_key,
        "executor_binary_sha256": binary,
        "light_stage_receipt_id": light_id,
        "artifact_root_sha256": path_digest(artifact_root),
        "ledger_path_sha256": path_digest(ledger),
        "ledger_head_sha256": ledger_head(ledger).ok().flatten(),
        "node_manifest": node_manifest(plan).unwrap_or_else(|_| Value::Array(Vec::new())),
        "package_admission": null,
        "checks": checks,
        "combined_execution": failure_combined_execution(plan),
        "light_receipt": if light_receipt.is_object() {light_receipt.clone()} else {json!({})},
    });
    audit["audit_id"] = Value::String(derived_id(&audit, "audit_id")?);
    let schema = read_json(&repo.join("gate-policy/v1/schemas/pre-heavy-audit.schema.json"))?;
    validate_schema(&schema, &audit, "invalid pre-heavy audit")?;
    Ok(audit)
}

fn failure_combined_execution(_plan: &Value) -> Value {
    json!({
        "decision": "NOT_APPLICABLE",
        "reason_code": "TRANSACTION_INVALID",
        "requested_proof_id": null,
        "accepted_proof_id": null,
        "proof_sha256": null,
        "baseline_count": 0
    })
}

fn failure_status(failure: &GatePolicyError) -> &'static str {
    if matches!(
        failure.class,
        ErrorClass::Identity | ErrorClass::Json | ErrorClass::Schema
    ) || failure.code.contains("PACKAGE")
        || failure.code.contains("IDENTITY")
        || failure.code.contains("SHAPE")
        || failure.code.contains("COLLISION")
        || failure.code.contains("SUBSTITUTION")
    {
        "INVALID"
    } else {
        "BLOCKED"
    }
}

fn failure_check_index(code: &str) -> usize {
    FAILURE_CHECK_INDEX_RULES
        .iter()
        .find_map(|(tokens, index)| {
            tokens
                .iter()
                .any(|token| code.contains(token))
                .then_some(*index)
        })
        .unwrap_or(9)
}

/// Verify a READY audit against current plan, inventory and artifact identity.
///
/// # Errors
///
/// Returns a typed execution error for a non-ready, stale, substituted, or
/// malformed audit.
pub fn validate_audit(
    repo: &Path,
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
) -> Result<()> {
    let schema = read_json(&repo.join("gate-policy/v1/schemas/pre-heavy-audit.schema.json"))?;
    validate_schema(&schema, audit, "pre-heavy audit")?;
    let current_package_admission = package_admission(repo, plan)?;
    if derived_id(audit, "audit_id")? != string(audit, "audit_id")?
        || audit["plan_id"] != plan["plan_id"]
        || audit["plan_sha256"] != digest(plan)?
        || audit["execution_key"] != plan["execution_key"]
        || audit["artifact_root_sha256"] != path_digest(artifact_root)
        || audit["node_manifest"] != node_manifest(plan)?
        || audit["combined_execution"] != plan["combined_quality"]
        || audit["package_admission"] != current_package_admission
    {
        return Err(audit_error("GATE-AUDIT-IDENTITY", "audit binding mismatch"));
    }
    if audit["status"] != "READY" {
        return Err(audit_error(
            "GATE-AUDIT-NOT-READY",
            audit["status"].to_string(),
        ));
    }
    let checks = audit["checks"]
        .as_array()
        .ok_or_else(|| audit_error("GATE-AUDIT-CHECK-SET", "checks must be an array"))?;
    if checks.len() != CHECK_IDS.len()
        || checks.iter().zip(CHECK_IDS).any(|(item, expected)| {
            item["check_id"] != expected
                || item["status"] != "PASS"
                || item["reason_codes"]
                    .as_array()
                    .is_none_or(|codes| !codes.is_empty())
        })
        || audit["reason_codes"]
            .as_array()
            .is_none_or(|codes| !codes.is_empty())
    {
        return Err(audit_error(
            "GATE-AUDIT-CHECK-SET",
            "READY requires the ordered canonical ten-check set, all PASS, with no reasons",
        ));
    }
    validate_stage_receipt(repo, plan, &audit["light_receipt"], artifact_root, false)?;
    if audit["light_stage_receipt_id"] != audit["light_receipt"]["stage_receipt_id"] {
        return Err(audit_error(
            "GATE-AUDIT-LIGHT-RECEIPT",
            "light receipt was substituted",
        ));
    }
    let current_inventory = node_manifest(plan)?;
    if current_inventory != audit["node_manifest"] {
        return Err(audit_error(
            "GATE-AUDIT-INVENTORY-DRIFT",
            "independent current inventory differs from admitted inventory",
        ));
    }
    Ok(())
}

/// Verify a READY audit and bind it to the exact executor image admitting HEAVY.
///
/// # Errors
///
/// Returns a typed error for every ordinary audit defect plus an executable
/// identity mismatch. Receipt-envelope verification deliberately uses
/// [`validate_audit`] because a verifier on another runner need not have the
/// byte-identical executable image used for execution.
pub fn validate_audit_for_execution(
    repo: &Path,
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
) -> Result<()> {
    validate_audit(repo, plan, audit, artifact_root)?;
    validate_executor_binary_binding(audit)?;
    validate_current_execution_context(repo, plan)?;
    validate_execution_claim_binding(audit, claims)
}

fn validate_executor_binary_binding(audit: &Value) -> Result<()> {
    let current = current_executable_sha256()?;
    if audit["executor_binary_sha256"] != current {
        return Err(audit_error(
            "GATE-AUDIT-EXECUTOR-BINARY-DRIFT",
            "HEAVY executor differs from the binary that emitted the LIGHT receipt",
        ));
    }
    Ok(())
}

fn validate_execution_claim_binding(audit: &Value, claims: &ExecutionClaims) -> Result<()> {
    let light = &audit["light_receipt"]["claims"];
    if !execution_claims_match(light, claims) {
        return Err(audit_error(
            "GATE-AUDIT-EXECUTION-CLAIM-DRIFT",
            "HEAVY workflow/job/runner/attempt differs from LIGHT",
        ));
    }
    Ok(())
}

pub(crate) fn validate_current_execution_context(repo: &Path, plan: &Value) -> Result<()> {
    execution_context_is_current(plan, &crate::planner::current_execution_context(repo)?)
}

fn execution_context_is_current(plan: &Value, current: &Value) -> Result<()> {
    if *current == plan["execution_context"] {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-EXECUTION-CONTEXT-DRIFT",
            "tool, environment, fixture, or configuration identity changed after READY",
        ))
    }
}

fn execution_claims_match(light: &Value, claims: &ExecutionClaims) -> bool {
    light["workflow"] == claims.workflow
        && light["job"] == claims.job
        && light["runner"] == claims.runner
        && light["attempt"] == claims.attempt
}

/// Verify that a resume ledger is the exact durable ledger admitted by audit.
///
/// # Errors
///
/// Returns a typed execution error for path substitution, ephemeral storage,
/// unreadable history, or an unresolved tooling defect.
pub fn validate_resume_ledger(
    repo: &Path,
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
    ledger: &Path,
    started_entry_sha256: &str,
    claims: &ExecutionClaims,
) -> Result<()> {
    validate_audit(repo, plan, audit, artifact_root)?;
    if audit["ledger_path_sha256"] != path_digest(ledger) {
        return Err(audit_error(
            "GATE-AUDIT-LEDGER-SUBSTITUTION",
            ledger.display().to_string(),
        ));
    }
    durable_ledger(ledger)?;
    no_open_tooling_defect(ledger)?;
    validate_started_successor(
        plan,
        audit,
        artifact_root,
        ledger,
        started_entry_sha256,
        claims,
    )
}

/// Admit only the caller-selected durable append target before a HEAVY
/// transaction is recorded. Full audit and resume admission happens after the
/// balanced lifecycle has begun.
///
/// # Errors
///
/// Returns a typed error when the selected ledger is absent, ephemeral, or has
/// an invalid predecessor chain.
pub fn admit_attempt_ledger(ledger: &Path) -> Result<()> {
    durable_ledger(ledger)
}

/// Append a terminal HEAVY failure. Tooling failures open a defect immediately;
/// infrastructure failures retain one declared retry before opening a defect.
///
/// # Errors
///
/// Returns a typed ledger error when history cannot be parsed or persisted.
pub fn record_heavy_failure(path: &Path, record: Value, cause_key: &str) -> Result<()> {
    let text = fs::read_to_string(path)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-READ", error.to_string()))?;
    let prior = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_strict(line.as_bytes()))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|item| {
            item["record_type"] == "STAGE_ATTEMPT"
                && item["status"] == "FAILED"
                && item["cause_key"] == cause_key
        })
        .count();
    append_attempt_record(path, record)?;
    let class = failure_class(cause_key);
    if class == "TOOLING" || prior >= 1 {
        let defect_key = sha256_bytes(cause_key.as_bytes());
        append_attempt_record(
            path,
            json!({
                "record_type": "TOOLING_DEFECT",
                "defect_id": format!("AUTO-{}", &defect_key[..16]),
                "status": "OPEN",
                "cause_key": cause_key,
                "failure_class": class,
                "reason_code": if prior >= 1 {"SAME_CAUSE_RECURRED_AFTER_ONE_RETRY"} else {"TOOLING_FAILURE_REQUIRES_CORRECTION"},
                "owner": "openwepp-maintainers",
                "reproducer": cause_key,
                "impact": "HEAVY admission or execution is not trustworthy",
                "correction_boundary": "resolve before the next pre-heavy audit",
            }),
        )?;
    }
    Ok(())
}

fn failure_class(cause_key: &str) -> &'static str {
    if cause_key.contains("SPAWN")
        || cause_key.contains("TIMEOUT")
        || cause_key.contains("RUNNER")
        || cause_key.contains("TERMINATED")
    {
        "INFRASTRUCTURE"
    } else {
        "TOOLING"
    }
}

/// Close HEAVY admissions whose process ended before recording an outcome.
///
/// # Errors
///
/// Returns a typed ledger error when history cannot be verified or amended.
pub fn reconcile_orphaned_attempts(path: &Path) -> Result<usize> {
    verify_ledger_chain(path)?;
    let text = fs::read_to_string(path)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-READ", error.to_string()))?;
    let records = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_strict(line.as_bytes()))
        .collect::<Result<Vec<_>>>()?;
    let terminal = records
        .iter()
        .filter_map(|item| item["started_entry_sha256"].as_str())
        .collect::<BTreeSet<_>>();
    let orphaned = records
        .iter()
        .filter(|item| {
            item["record_type"] == "STAGE_ATTEMPT"
                && item["status"] == "STARTED"
                && item["phase"] == "ADMISSION"
        })
        .filter(|item| {
            item["entry_sha256"]
                .as_str()
                .is_some_and(|entry| !terminal.contains(entry))
        })
        .cloned()
        .collect::<Vec<_>>();
    for started in &orphaned {
        let cause = "GATE-ATTEMPT-PREVIOUS-PROCESS-TERMINATED";
        record_heavy_failure(
            path,
            json!({
                "record_type": "STAGE_ATTEMPT",
                "status": "FAILED",
                "stage": "HEAVY",
                "plan_id": started["plan_id"],
                "audit_id": started["audit_id"],
                "artifact_root": started["artifact_root"],
                "recovery_root": started["recovery_root"],
                "workflow": started["workflow"],
                "job": started["job"],
                "runner": started["runner"],
                "attempt": started["attempt"],
                "result": null,
                "error_code": cause,
                "error_message": "the admitted HEAVY process ended without a terminal record",
                "cause_key": cause,
                "failure_class": "INFRASTRUCTURE",
                "wall_time_ms": null,
                "started_entry_sha256": started["entry_sha256"],
            }),
            cause,
        )?;
    }
    Ok(orphaned.len())
}

/// Durably append one predecessor-bound execution-attempt record.
///
/// # Errors
///
/// Returns a typed error when the existing chain is invalid, the record uses
/// reserved identity fields, or the append cannot be flushed to stable storage.
pub fn append_attempt_record(path: &Path, mut record: Value) -> Result<String> {
    verify_ledger_chain(path)?;
    let object = record
        .as_object_mut()
        .ok_or_else(|| audit_error("GATE-AUDIT-LEDGER-RECORD", "record must be an object"))?;
    if object.contains_key("previous_entry_sha256") || object.contains_key("entry_sha256") {
        return Err(audit_error(
            "GATE-AUDIT-LEDGER-RECORD",
            "record contains reserved chain fields",
        ));
    }
    let text = fs::read_to_string(path)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-READ", error.to_string()))?;
    let previous = text
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .map(|line| parse_strict(line.as_bytes()))
        .transpose()?
        .and_then(|item| item["entry_sha256"].as_str().map(str::to_owned));
    object.insert(
        "previous_entry_sha256".to_owned(),
        previous.map_or(Value::Null, Value::String),
    );
    let entry = digest(&record)?;
    record
        .as_object_mut()
        .ok_or_else(|| audit_error("GATE-AUDIT-LEDGER-RECORD", "record must be an object"))?
        .insert("entry_sha256".to_owned(), Value::String(entry.clone()));
    let mut bytes = canonical_bytes(&record)?;
    bytes.push(b'\n');
    let mut stream = OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-APPEND", error.to_string()))?;
    stream
        .write_all(&bytes)
        .and_then(|()| stream.sync_all())
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-APPEND", error.to_string()))?;
    Ok(entry)
}

#[allow(
    clippy::needless_pass_by_value,
    reason = "each ephemeral evidence value is consumed immediately into its canonical digest"
)]
fn check(id: &str, result: Result<()>, evidence: Value) -> Result<Value> {
    let (status, reason_codes) = match result {
        Ok(()) => ("PASS", Vec::new()),
        Err(error) if error.class == ErrorClass::Identity || error.class == ErrorClass::Schema => {
            ("INVALID", vec![error.code])
        }
        Err(error) => ("BLOCKED", vec![error.code]),
    };
    Ok(json!({
        "check_id": id,
        "status": status,
        "reason_codes": reason_codes,
        "evidence_sha256": digest(&evidence)?,
    }))
}

fn package_admission(repo: &Path, plan: &Value) -> Result<Value> {
    let packages = plan["authorized_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|path| path.starts_with("docs/work-packages/") && path.ends_with("/package.md"))
        .collect::<Vec<_>>();
    let base = string(&plan["source"], "base_commit")?;
    let candidates = packages
        .iter()
        .map(|package| validate_package(repo, base, Path::new(package)))
        .collect::<Result<Vec<_>>>()?;
    select_package_admission(plan, candidates, packages.len())
}

fn select_package_admission(
    plan: &Value,
    candidates: Vec<Value>,
    candidate_count: usize,
) -> Result<Value> {
    let mut admitted = candidates
        .into_iter()
        .filter(|candidate| package_admitted(plan, candidate).is_ok())
        .collect::<Vec<_>>();
    match admitted.len() {
        1 => Ok(admitted.remove(0)),
        0 => Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-PACKAGE-NOT-ADMITTED",
            format!("none of {candidate_count} changed package candidates admits the exact diff"),
        )),
        count => Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-PACKAGE-AMBIGUOUS",
            format!("{count} of {candidate_count} changed package candidates admit the exact diff"),
        )),
    }
}

fn package_admitted(plan: &Value, result: &Value) -> Result<()> {
    if result["status"] != "READY"
        || result["changed_paths"] != plan["authorized_paths"]
        || result["base_commit"] != plan["source"]["base_commit"]
    {
        return Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-PACKAGE-ADMISSION",
            format!(
                "package audit did not admit exact plan paths: {}",
                result["reason_codes"]
            ),
        ));
    }
    Ok(())
}

fn is_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn light_stage_passed(plan: &Value, receipt: &Value) -> Result<()> {
    nodes(plan)?
        .iter()
        .filter(|node| node["execution_cost_class"] == "LIGHT")
        .try_for_each(|node| require_light_node_pass(node, receipt))
}

fn require_light_node_pass(node: &Value, receipt: &Value) -> Result<()> {
    let id = string(node, "node_id")?;
    if receipt["final_results"][id] == "PASS" {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-LIGHT-NONPASS",
            format!("light node {id} did not pass"),
        ))
    }
}

fn cheap_prerequisites(repo: &Path, plan: &Value, receipt: &Value) -> Result<()> {
    light_stage_passed(plan, receipt)?;
    let base = string(&plan["source"], "base_commit")?;
    let diff = Command::new("git")
        .args(["diff", "--check", base, "--"])
        .current_dir(repo)
        .output()
        .map_err(|error| audit_error("GATE-AUDIT-DIFF-CHECK", error.to_string()))?;
    if !diff.status.success() {
        return Err(audit_error(
            "GATE-AUDIT-DIFF-HYGIENE",
            String::from_utf8_lossy(&diff.stdout),
        ));
    }
    for path in plan["authorized_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|path| {
            Path::new(path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("rs"))
        })
    {
        let lines = fs::read_to_string(repo.join(path))
            .map_err(|error| audit_error("GATE-AUDIT-LINE-COUNT", error.to_string()))?
            .lines()
            .count();
        if lines > 3_000 {
            return Err(audit_error(
                "GATE-AUDIT-RUST-FILE-OVER-3000",
                format!("{path}: {lines}"),
            ));
        }
    }
    documentation_scope_is_exact(plan)?;
    let package = plan["authorized_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .find(|path| path.starts_with("docs/work-packages/") && path.ends_with("/package.md"));
    if let Some(package) = package {
        let active = repo
            .join(package)
            .parent()
            .ok_or_else(|| audit_error("GATE-AUDIT-PROMPT-STATE", package))?
            .join("prompts/active");
        let count = fs::read_dir(active)
            .map_err(|error| audit_error("GATE-AUDIT-PROMPT-STATE", error.to_string()))?
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|value| value == "md"))
            .count();
        if count != 1 {
            return Err(audit_error(
                "GATE-AUDIT-PROMPT-STATE",
                format!("expected one active prompt, found {count}"),
            ));
        }
    }
    Ok(())
}

fn inventory_and_arguments_are_exact(
    repo: &Path,
    plan: &Value,
    artifact_root: &Path,
) -> Result<()> {
    validate_exact_node_shapes(plan)?;
    reconstruct_exact_plan(repo, plan, artifact_root)
}

fn validate_exact_node_shapes(plan: &Value) -> Result<()> {
    let mut ids = BTreeSet::new();
    for node in nodes(plan)? {
        let id = string(node, "node_id")?;
        if !ids.insert(id)
            || !node["arguments"].is_array()
            || node["expected_inventory"]["mode"] != "EXACT"
        {
            return Err(audit_error(
                "GATE-AUDIT-INVENTORY-INVALID",
                "node identity, arguments, or exact inventory is invalid",
            ));
        }
    }
    Ok(())
}

fn reconstruct_exact_plan(repo: &Path, plan: &Value, artifact_root: &Path) -> Result<()> {
    let reconstructed = with_disposable_audit_reconstruction(artifact_root, |reconstruction| {
        reconstruct_plan_in(repo, plan, reconstruction, false)
    })?;
    reconstructed_plan_is_exact(plan, &reconstructed)
}

fn audit_reconstruction_root(artifact_root: &Path) -> std::path::PathBuf {
    artifact_root.join(".work/audit-reconstruction")
}

fn with_disposable_audit_reconstruction<T>(
    artifact_root: &Path,
    reconstruct: impl FnOnce(&Path) -> Result<T>,
) -> Result<T> {
    let reconstruction = audit_reconstruction_root(artifact_root);
    let result = reconstruct(&reconstruction);
    remove_reconstruction_workspace(&reconstruction)?;
    result
}

fn reconstructed_plan_is_exact(plan: &Value, reconstructed: &Value) -> Result<()> {
    if digest(reconstructed)? == digest(plan)? {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-INVENTORY-DRIFT",
            "current source, policy, execution context, arguments, or independently enumerated inventory differs from the terminal plan",
        ))
    }
}

fn documentation_scope_is_exact(plan: &Value) -> Result<()> {
    let expected = plan["changed_objects"]
        .as_array()
        .ok_or_else(|| audit_error("GATE-AUDIT-DOC-SCOPE", "changed_objects"))?
        .iter()
        .filter(|change| change["change_kind"] != "DELETE")
        .filter_map(|change| change["path"].as_str())
        .filter(|path| {
            Path::new(path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        })
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let document_nodes = nodes(plan)?
        .iter()
        .filter(|node| node["gate_definition_id"] == "documentation-lint-v1")
        .collect::<Vec<_>>();
    if expected.is_empty() {
        if document_nodes.is_empty() {
            return Ok(());
        }
        return Err(audit_error(
            "GATE-AUDIT-DOC-SCOPE",
            "documentation lint is present without changed Markdown",
        ));
    }
    if document_nodes.len() != 1 {
        return Err(audit_error(
            "GATE-AUDIT-DOC-SCOPE",
            "changed Markdown requires exactly one documentation lint node",
        ));
    }
    let arguments = document_nodes[0]["arguments"]
        .as_array()
        .ok_or_else(|| audit_error("GATE-AUDIT-DOC-SCOPE", "arguments"))?
        .iter()
        .map(|argument| {
            argument
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| audit_error("GATE-AUDIT-DOC-SCOPE", "non-string argument"))
        })
        .collect::<Result<Vec<_>>>()?;
    if arguments.len() != 2 + expected.len() * 2
        || arguments.first().map(String::as_str) != Some("markdown-doc")
        || arguments.get(1).map(String::as_str) != Some("lint")
    {
        return Err(audit_error(
            "GATE-AUDIT-DOC-SCOPE",
            "documentation lint command shape differs from the canonical scoped form",
        ));
    }
    let mut unique = BTreeSet::new();
    let mut actual = Vec::new();
    for pair in arguments[2..].chunks_exact(2) {
        if pair[0] != "--path" || !unique.insert(pair[1].clone()) {
            return Err(audit_error(
                "GATE-AUDIT-DOC-SCOPE",
                "documentation lint paths are malformed or duplicated",
            ));
        }
        actual.push(pair[1].clone());
    }
    if actual == expected.into_iter().collect::<Vec<_>>() {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-DOC-SCOPE",
            "documentation lint paths differ from changed non-deleted Markdown",
        ))
    }
}

fn execution_identities(plan: &Value, receipt: &Value) -> Result<()> {
    require_execution_context_digests(plan)?;
    require_nonempty_execution_claims(receipt)?;
    require_positive_execution_attempt(receipt)?;
    require_executor_binary_digest(receipt)
}

fn require_execution_context_digests(plan: &Value) -> Result<()> {
    for field in [
        "configuration_sha256",
        "environment_manifest_sha256",
        "fixture_manifest_sha256",
        "tool_manifest_sha256",
    ] {
        if string(&plan["execution_context"], field)?.len() != 64 {
            return Err(audit_error("GATE-AUDIT-EXECUTION-IDENTITY", field));
        }
    }
    Ok(())
}

fn require_nonempty_execution_claims(receipt: &Value) -> Result<()> {
    for field in [
        "principal",
        "repository",
        "source_event",
        "source_ref",
        "workflow",
        "job",
        "runner",
    ] {
        if string(&receipt["claims"], field)?.is_empty() {
            return Err(audit_error("GATE-AUDIT-EXECUTION-CLAIM", field));
        }
    }
    Ok(())
}

fn require_positive_execution_attempt(receipt: &Value) -> Result<()> {
    if receipt["claims"]["attempt"]
        .as_u64()
        .is_none_or(|attempt| attempt == 0)
    {
        return Err(audit_error(
            "GATE-AUDIT-EXECUTION-CLAIM",
            "attempt must be positive",
        ));
    }
    Ok(())
}

fn require_executor_binary_digest(receipt: &Value) -> Result<()> {
    if string(receipt, "executor_binary_sha256")?.len() != 64 {
        return Err(audit_error(
            "GATE-AUDIT-EXECUTION-IDENTITY",
            "executor_binary_sha256",
        ));
    }
    Ok(())
}

fn artifact_identity(receipt: &Value, artifact_root: &Path) -> Result<()> {
    if receipt["artifact_root_sha256"] == path_digest(artifact_root) {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-ARTIFACT-COLLISION",
            "stage receipt belongs to another attempt root",
        ))
    }
}

fn light_attempt_isolated(plan: &Value, receipt: &Value, artifact_root: &Path) -> Result<()> {
    artifact_identity(receipt, artifact_root)?;
    for node in nodes(plan)?
        .iter()
        .filter(|node| node["execution_cost_class"] == "LIGHT")
    {
        let node_id = string(node, "node_id")?;
        let checkpoint = read_json(
            &artifact_root
                .join(".checkpoints")
                .join(format!("{node_id}.json")),
        )?;
        if checkpoint["node_sha256"] != digest(node)? || checkpoint["result"] != "PASS" {
            return Err(audit_error("GATE-AUDIT-CHECKPOINT-DRIFT", node_id));
        }
        for relative in node["output_paths"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            let expected = checkpoint["artifacts"]
                .as_array()
                .into_iter()
                .flatten()
                .find(|artifact| artifact["path"] == relative)
                .and_then(|artifact| artifact["sha256"].as_str())
                .ok_or_else(|| audit_error("GATE-AUDIT-CHECKPOINT-ARTIFACT", relative))?;
            if file_digest(&artifact_root.join(relative))? != expected {
                return Err(audit_error(
                    "GATE-AUDIT-CHECKPOINT-ARTIFACT-DRIFT",
                    relative,
                ));
            }
        }
    }
    Ok(())
}

fn separated_roots(plan: &Value) -> Result<()> {
    let roots = &plan["environment_roots"];
    let values = ["execution_root", "authority_root", "documentation_root"]
        .into_iter()
        .filter_map(|name| roots[name].as_str())
        .collect::<BTreeSet<_>>();
    if values.len() == 3 {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-ROOT-ALIAS",
            "execution, authority, and documentation roots must differ",
        ))
    }
}

fn valid_stage_order(plan: &Value) -> Result<()> {
    let mut seen = BTreeSet::new();
    for node in nodes(plan)? {
        validate_node_stage_order(node, &mut seen)?;
    }
    Ok(())
}

fn validate_node_stage_order(node: &Value, seen: &mut BTreeSet<String>) -> Result<()> {
    let class = string(node, "execution_cost_class")?;
    if !matches!(class, "LIGHT" | "HEAVY") {
        return Err(audit_error("GATE-AUDIT-COST-CLASS", class));
    }
    for dependency in node["prerequisites"].as_array().into_iter().flatten() {
        let dependency = dependency
            .as_str()
            .ok_or_else(|| audit_error("GATE-AUDIT-PREREQUISITE", "non-string prerequisite"))?;
        if !seen.contains(dependency) {
            return Err(audit_error("GATE-AUDIT-PREREQUISITE-ORDER", dependency));
        }
    }
    seen.insert(string(node, "node_id")?.to_owned());
    Ok(())
}

fn durable_ledger(path: &Path) -> Result<()> {
    let absolute = path
        .canonicalize()
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-MISSING", error.to_string()))?;
    if absolute.starts_with("/tmp") || absolute.starts_with("/t") || !absolute.is_file() {
        Err(audit_error(
            "GATE-AUDIT-LEDGER-EPHEMERAL",
            absolute.display().to_string(),
        ))
    } else {
        verify_ledger_chain(path)
    }
}

fn ledger_head(path: &Path) -> Result<Option<String>> {
    verify_ledger_chain(path)?;
    let text = fs::read_to_string(path)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-READ", error.to_string()))?;
    text.lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .map(|line| {
            let record = parse_strict(line.as_bytes())?;
            string(&record, "entry_sha256").map(str::to_owned)
        })
        .transpose()
}

fn validate_started_successor(
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
    ledger: &Path,
    started_entry_sha256: &str,
    claims: &ExecutionClaims,
) -> Result<()> {
    let text = fs::read_to_string(ledger)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-READ", error.to_string()))?;
    let started = text
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .map(|line| parse_strict(line.as_bytes()))
        .transpose()?
        .ok_or_else(|| audit_error("GATE-AUDIT-STARTED-MISSING", "ledger is empty"))?;
    let expected_previous = audit["ledger_head_sha256"].clone();
    let exact = started["entry_sha256"] == started_entry_sha256
        && started["previous_entry_sha256"] == expected_previous
        && started["record_type"] == "STAGE_ATTEMPT"
        && started["status"] == "STARTED"
        && started["stage"] == "HEAVY"
        && started["phase"] == "ADMISSION"
        && started["plan_id"] == plan["plan_id"]
        && started["audit_id"] == audit["audit_id"]
        && started["artifact_root"] == artifact_root.display().to_string()
        && started["workflow"] == claims.workflow
        && started["job"] == claims.job
        && started["runner"] == claims.runner
        && started["attempt"] == claims.attempt;
    if exact {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-LEDGER-SUCCESSOR",
            "current ledger must equal the audited prefix followed by this exact HEAVY STARTED record",
        ))
    }
}

fn verify_ledger_chain(path: &Path) -> Result<()> {
    let text = fs::read_to_string(path)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-READ", error.to_string()))?;
    let mut previous: Option<String> = None;
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let mut item = parse_strict(line.as_bytes())?;
        if item["previous_entry_sha256"]
            != previous.as_ref().map_or(Value::Null, |value| json!(value))
        {
            return Err(audit_error(
                "GATE-AUDIT-LEDGER-CHAIN",
                "predecessor digest mismatch",
            ));
        }
        let claimed = item["entry_sha256"]
            .as_str()
            .ok_or_else(|| audit_error("GATE-AUDIT-LEDGER-CHAIN", "entry_sha256"))?
            .to_owned();
        item.as_object_mut()
            .ok_or_else(|| audit_error("GATE-AUDIT-LEDGER-CHAIN", "record object"))?
            .remove("entry_sha256");
        if digest(&item)? != claimed {
            return Err(audit_error(
                "GATE-AUDIT-LEDGER-CHAIN",
                "entry digest mismatch",
            ));
        }
        previous = Some(claimed);
    }
    Ok(())
}

fn no_open_tooling_defect(path: &Path) -> Result<()> {
    let text = fs::read_to_string(path)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-READ", error.to_string()))?;
    reject_open_tooling_defects(tooling_defect_statuses(&text)?)
}

fn tooling_defect_statuses(text: &str) -> Result<std::collections::BTreeMap<String, String>> {
    let mut defects = std::collections::BTreeMap::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let item = parse_strict(line.as_bytes())?;
        if item["record_type"] == "TOOLING_DEFECT" {
            let defect_id = item["defect_id"]
                .as_str()
                .ok_or_else(|| audit_error("GATE-AUDIT-TOOLING-DEFECT-SHAPE", "defect_id"))?;
            let status = item["status"]
                .as_str()
                .ok_or_else(|| audit_error("GATE-AUDIT-TOOLING-DEFECT-SHAPE", "status"))?;
            defects.insert(defect_id.to_owned(), status.to_owned());
        }
    }
    Ok(defects)
}

fn reject_open_tooling_defects(defects: std::collections::BTreeMap<String, String>) -> Result<()> {
    for (defect_id, status) in defects {
        if status == "OPEN" {
            return Err(audit_error("GATE-AUDIT-OPEN-TOOLING-DEFECT", defect_id));
        }
    }
    Ok(())
}

fn no_open_tooling_defect_at_head(path: &Path, expected_head: Option<&str>) -> Result<()> {
    no_open_tooling_defect(path)?;
    if ledger_head(path)?.as_deref() == expected_head {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-LEDGER-DRIFT",
            "durable ledger changed while the pre-heavy audit was being constructed",
        ))
    }
}

fn validate_combined_decision(plan: &Value) -> Result<()> {
    let definitions = quality_definition_ids(plan);
    if combined_decision_is_valid(&plan["combined_quality"], &definitions) {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-COMBINED-DAG-MISMATCH",
            "combined-quality decision does not match the immutable DAG",
        ))
    }
}

fn quality_definition_ids(plan: &Value) -> BTreeSet<&str> {
    nodes(plan)
        .unwrap_or(&[])
        .iter()
        .filter_map(|node| node["gate_definition_id"].as_str())
        .collect()
}

fn combined_decision_is_valid(decision: &Value, definitions: &BTreeSet<&str>) -> bool {
    let combined = definitions.contains("combined-workspace-quality-v1");
    let separate = definitions.contains("workspace-full-nextest-v1")
        && definitions.contains("adjudicated-crap-v1");
    if decision["decision"] == "COMBINED" {
        combined
            && !separate
            && decision["accepted_proof_id"].as_str().is_some()
            && decision["proof_sha256"].as_str().is_some()
            && decision["baseline_count"] == 3
    } else if decision["decision"] == "SEPARATE" {
        separate && !combined && decision["accepted_proof_id"].is_null()
    } else if decision["decision"] == "NOT_APPLICABLE" {
        !combined && !separate && decision["accepted_proof_id"].is_null()
    } else {
        false
    }
}

fn validate_stage_receipt(
    repo: &Path,
    plan: &Value,
    receipt: &Value,
    artifact_root: &Path,
    enforce_current_binary: bool,
) -> Result<()> {
    let schema = read_json(&repo.join("gate-policy/v1/schemas/stage-receipt.schema.json"))?;
    validate_schema(&schema, receipt, "light stage receipt")?;
    if derived_id(receipt, "stage_receipt_id")? != string(receipt, "stage_receipt_id")?
        || receipt["plan_id"] != plan["plan_id"]
        || receipt["plan_sha256"] != digest(plan)?
        || receipt["execution_key"] != plan["execution_key"]
        || receipt["artifact_root_sha256"] != path_digest(artifact_root)
        || receipt["stage"] != "LIGHT"
    {
        return Err(audit_error(
            "GATE-AUDIT-STAGE-RECEIPT-IDENTITY",
            "light stage receipt binding mismatch",
        ));
    }
    if enforce_current_binary && receipt["executor_binary_sha256"] != current_executable_sha256()? {
        return Err(audit_error(
            "GATE-AUDIT-EXECUTOR-BINARY-DRIFT",
            "audit binary differs from the binary that emitted the LIGHT receipt",
        ));
    }
    Ok(())
}

fn node_manifest(plan: &Value) -> Result<Value> {
    let manifest = nodes(plan)?
        .iter()
        .map(|node| {
            Ok(json!({
                "node_id": string(node, "node_id")?,
                "execution_cost_class": string(node, "execution_cost_class")?,
                "node_sha256": digest(node)?,
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Value::Array(manifest))
}

fn nodes(plan: &Value) -> Result<&[Value]> {
    plan["nodes"]
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| audit_error("GATE-AUDIT-PLAN-SHAPE", "nodes"))
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field]
        .as_str()
        .ok_or_else(|| audit_error("GATE-AUDIT-SHAPE", field))
}

fn path_digest(path: &Path) -> String {
    sha256_bytes(path.as_os_str().as_encoded_bytes())
}

fn file_digest(path: &Path) -> Result<String> {
    fs::read(path)
        .map(|bytes| sha256_bytes(&bytes))
        .map_err(|error| audit_error("GATE-AUDIT-READ", error.to_string()))
}

fn read_json(path: &Path) -> Result<Value> {
    let bytes = fs::read(path)
        .map_err(|error| audit_error("GATE-AUDIT-READ", format!("{}: {error}", path.display())))?;
    parse_strict(&bytes)
}

fn audit_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, code, message)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde_json::json;

    use super::{
        append_attempt_record, audit_reconstruction_root, build_failure_audit, check,
        documentation_scope_is_exact, durable_ledger, execution_claims_match,
        execution_context_is_current, execution_identities, failure_check_index,
        light_stage_passed, no_open_tooling_defect, package_admission, package_admitted, read_json,
        reconcile_orphaned_attempts, reconstructed_plan_is_exact, record_heavy_failure,
        select_package_admission, valid_stage_order, validate_combined_decision,
        validate_started_successor, verify_ledger_chain, with_disposable_audit_reconstruction,
    };
    use crate::canonical::{parse_strict, validate_schema};
    use crate::error::{ErrorClass, GatePolicyError};
    use crate::executor::ExecutionClaims;

    #[test]
    fn audit_inventory_uses_a_disposable_target_distinct_from_execution() {
        let artifacts = std::env::temp_dir().join(format!(
            "openwepp-audit-disposable-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let audit_root = audit_reconstruction_root(&artifacts);
        assert_eq!(audit_root, artifacts.join(".work/audit-reconstruction"));
        assert_ne!(audit_root, artifacts.join(".work/cargo-target"));
        let value = with_disposable_audit_reconstruction(&artifacts, |root| {
            fs::create_dir_all(root).expect("create audit workspace");
            fs::write(root.join("compiled-test"), b"snapshot-bound").expect("write cache marker");
            Ok(7)
        })
        .expect("successful reconstruction");
        assert_eq!(value, 7);
        assert!(!audit_root.exists());

        let error = with_disposable_audit_reconstruction(&artifacts, |root| {
            fs::create_dir_all(root).expect("create failed audit workspace");
            fs::write(root.join("compiled-test"), b"snapshot-bound").expect("write cache marker");
            Err::<(), _>(GatePolicyError::new(
                ErrorClass::Execution,
                "GATE-AUDIT-TEST-FAILURE",
                "injected reconstruction failure",
            ))
        })
        .expect_err("reconstruction failure must be retained");
        assert_eq!(error.code, "GATE-AUDIT-TEST-FAILURE");
        assert!(!audit_root.exists());
        fs::remove_dir_all(artifacts).expect("remove fixture");
    }

    #[test]
    fn audit_schema_rejects_duplicate_canonical_check_ids() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let schema = read_json(&root.join("gate-policy/v1/schemas/pre-heavy-audit.schema.json"))
            .expect("audit schema");
        let mut audit = read_json(&root.join("gate-policy/v1/fixtures/valid/pre-heavy-audit.json"))
            .expect("valid audit fixture");
        validate_schema(&schema, &audit, "valid audit").expect("valid audit must pass schema");
        audit["checks"][1]["check_id"] = audit["checks"][0]["check_id"].clone();
        assert!(validate_schema(&schema, &audit, "duplicate audit").is_err());
    }

    #[test]
    fn rust_verifies_python_jcs_with_adversarial_unicode_keys() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        verify_ledger_chain(&root.join("tests/fixtures/testgate/python-ledger-unicode.jsonl"))
            .expect("Python-produced ledger must share Rust RFC 8785 ordering");
    }

    #[test]
    fn recurring_cause_opens_a_blocking_tooling_defect() {
        let path = std::env::temp_dir().join(format!(
            "openwepp-gate-recurrence-{}-{}.jsonl",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::write(&path, "").expect("empty ledger");
        for _ in 0..2 {
            record_heavy_failure(
                &path,
                json!({
                    "record_type": "STAGE_ATTEMPT",
                    "status": "FAILED",
                    "cause_key": "GATE-EXEC-SPAWN",
                }),
                "GATE-EXEC-SPAWN",
            )
            .expect("record failure");
        }
        let text = fs::read_to_string(&path).expect("ledger");
        assert!(text.contains("SAME_CAUSE_RECURRED_AFTER_ONE_RETRY"));
        assert!(text.contains("\"status\":\"OPEN\""));
        fs::remove_file(path).expect("remove ledger");
    }

    #[test]
    fn documentation_scope_is_exact_sorted_and_excludes_deletions() {
        let plan = json!({
            "changed_objects": [
                {"path": "docs/a.md", "change_kind": "MODIFY"},
                {"path": "docs/deleted.md", "change_kind": "DELETE"},
                {"path": "README.MD", "change_kind": "ADD"},
                {"path": "docs/schema.json", "change_kind": "MODIFY"}
            ],
            "nodes": [{
                "gate_definition_id": "documentation-lint-v1",
                "arguments": [
                    "markdown-doc", "lint", "--path", "README.MD", "--path", "docs/a.md"
                ]
            }]
        });
        documentation_scope_is_exact(&plan).expect("exact changed Markdown scope");
        for arguments in [
            json!(["markdown-doc", "lint", "--path", "docs/a.md"]),
            json!([
                "markdown-doc",
                "lint",
                "--path",
                "docs/a.md",
                "--path",
                "README.MD"
            ]),
            json!([
                "markdown-doc",
                "lint",
                "--path",
                "README.MD",
                "--path",
                "docs/deleted.md"
            ]),
        ] {
            let mut drifted = plan.clone();
            drifted["nodes"][0]["arguments"] = arguments;
            assert!(documentation_scope_is_exact(&drifted).is_err());
        }
    }

    #[test]
    fn independently_reconstructed_plan_must_match_all_identity_fields() {
        let plan = json!({
            "execution_context": {"configuration_sha256": "original"},
            "nodes": [{
            "node_id": "a", "arguments": ["cargo", "nextest", "run"],
            "expected_inventory": {"mode": "EXACT", "ids": ["one"]}
        }]});
        reconstructed_plan_is_exact(&plan, &plan).expect("exact reconstruction");
        for pointer in [
            "/execution_context/configuration_sha256",
            "/nodes/0/arguments/2",
            "/nodes/0/expected_inventory/ids/0",
        ] {
            let mut drifted = plan.clone();
            *drifted.pointer_mut(pointer).expect("mutation pointer") = json!("drift");
            assert!(reconstructed_plan_is_exact(&plan, &drifted).is_err());
        }
    }

    #[test]
    fn heavy_admission_rejects_every_execution_context_identity_breaker() {
        let context = json!({
            "environment_manifest_sha256": "environment",
            "runner_host_class": "runner",
            "runner_image_sha256": "image",
            "fixture_manifest_sha256": "fixtures",
            "tool_manifest_sha256": "tools",
            "configuration_sha256": "configuration"
        });
        let plan = json!({"execution_context": context});
        execution_context_is_current(&plan, &plan["execution_context"]).expect("unchanged context");
        for field in [
            "environment_manifest_sha256",
            "runner_host_class",
            "runner_image_sha256",
            "fixture_manifest_sha256",
            "tool_manifest_sha256",
            "configuration_sha256",
        ] {
            let mut drifted = plan["execution_context"].clone();
            drifted[field] = json!("drift");
            assert!(execution_context_is_current(&plan, &drifted).is_err());
        }
    }

    #[test]
    fn heavy_started_must_be_the_exact_successor_of_the_audited_ledger_head() {
        let path = std::env::temp_dir().join(format!(
            "openwepp-gate-started-successor-{}-{}.jsonl",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::write(&path, "").expect("empty ledger");
        let head = append_attempt_record(
            &path,
            json!({"record_type": "STAGE_ATTEMPT", "status": "CLOSED", "stage": "LIGHT"}),
        )
        .expect("audited ledger head");
        let plan = json!({"plan_id": "1".repeat(64)});
        let audit = json!({"audit_id": "2".repeat(64), "ledger_head_sha256": head});
        let artifacts = PathBuf::from("/external/evidence");
        let claims = ExecutionClaims {
            workflow: "workflow".to_owned(),
            job: "job".to_owned(),
            runner: "runner".to_owned(),
            attempt: 1,
            ..ExecutionClaims::default()
        };
        let started = append_attempt_record(
            &path,
            json!({
                "record_type": "STAGE_ATTEMPT", "status": "STARTED", "stage": "HEAVY",
                "phase": "ADMISSION", "plan_id": plan["plan_id"], "audit_id": audit["audit_id"],
                "artifact_root": artifacts.display().to_string(), "workflow": claims.workflow,
                "job": claims.job, "runner": claims.runner, "attempt": claims.attempt,
            }),
        )
        .expect("started successor");
        validate_started_successor(&plan, &audit, &artifacts, &path, &started, &claims)
            .expect("exact successor");
        append_attempt_record(&path, json!({"record_type": "ATTEMPT", "status": "CLOSED"}))
            .expect("intervening record");
        assert!(
            validate_started_successor(&plan, &audit, &artifacts, &path, &started, &claims)
                .is_err()
        );
        fs::remove_file(path).expect("remove ledger");
    }

    #[test]
    fn orphaned_admission_is_closed_once_and_recurrence_opens_defect() {
        let path = std::env::temp_dir().join(format!(
            "openwepp-gate-orphan-{}-{}.jsonl",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::write(&path, "").expect("empty ledger");
        for attempt in 1..=2 {
            append_attempt_record(
                &path,
                json!({
                    "record_type": "STAGE_ATTEMPT", "status": "STARTED",
                    "stage": "HEAVY", "phase": "ADMISSION", "attempt": attempt,
                    "plan_id": "1".repeat(64), "audit_id": "2".repeat(64),
                    "artifact_root": "/external/e", "recovery_root": "/history/recovery/r",
                    "workflow": "w", "job": "j", "runner": "r",
                }),
            )
            .expect("started");
            assert_eq!(reconcile_orphaned_attempts(&path).expect("reconcile"), 1);
            assert_eq!(reconcile_orphaned_attempts(&path).expect("idempotent"), 0);
        }
        let text = fs::read_to_string(&path).expect("ledger");
        assert_eq!(
            text.lines()
                .map(|line| parse_strict(line.as_bytes()).expect("record"))
                .filter(|item| item["status"] == "FAILED")
                .count(),
            2
        );
        assert!(text.contains("SAME_CAUSE_RECURRED_AFTER_ONE_RETRY"));
        fs::remove_file(path).expect("remove ledger");
    }

    #[test]
    fn representable_early_failure_emits_ten_check_invalid_audit() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let failure = GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-PLAN-IDENTITY",
            "injected identity failure",
        );
        let audit = build_failure_audit(
            &root,
            &json!({}),
            &json!({}),
            &root,
            &root.join("target/test-ledger.jsonl"),
            &failure,
        )
        .expect("invalid audit");
        assert_eq!(audit["status"], "INVALID");
        assert_eq!(audit["checks"].as_array().map(Vec::len), Some(10));
        assert_eq!(audit["reason_codes"], json!(["GATE-PLAN-IDENTITY"]));
        assert_eq!(audit["checks"][2]["status"], "INVALID");

        let blocked = GatePolicyError::new(
            ErrorClass::Io,
            "GATE-AUDIT-LEDGER-MISSING",
            "durable ledger unavailable",
        );
        let audit = build_failure_audit(
            &root,
            &json!({}),
            &json!({}),
            &root,
            &root.join("target/test-ledger.jsonl"),
            &blocked,
        )
        .expect("blocked audit");
        assert_eq!(audit["status"], "BLOCKED");
        assert_eq!(audit["checks"][8]["status"], "BLOCKED");
        assert_eq!(
            audit["checks"][8]["reason_codes"],
            json!(["GATE-AUDIT-LEDGER-MISSING"])
        );

        let malformed = build_failure_audit(
            &root,
            &json!({"plan_id": "z".repeat(64), "execution_key": "Z".repeat(64)}),
            &json!({"stage_receipt_id": "g".repeat(64)}),
            &root,
            &root.join("target/test-ledger.jsonl"),
            &failure,
        )
        .expect("malformed identities still yield schema-valid audit");
        for field in ["plan_id", "execution_key", "light_stage_receipt_id"] {
            let value = malformed[field].as_str().expect("digest field");
            assert_eq!(value.len(), 64);
            assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
            assert_eq!(value, value.to_ascii_lowercase());
        }
    }

    #[test]
    fn rejected_package_admission_is_an_identity_failure() {
        let error = package_admitted(
            &json!({
                "authorized_paths": ["docs/work-packages/p/package.md"],
                "source": {"base_commit": "base"},
            }),
            &json!({
                "status": "INVALID", "changed_paths": [], "base_commit": "base",
                "reason_codes": ["PACKAGE-UNDECLARED-PATH"],
            }),
        )
        .expect_err("authority substitution is invalid");
        assert_eq!(error.class, ErrorClass::Identity);
        assert_eq!(error.code, "GATE-AUDIT-PACKAGE-ADMISSION");
    }

    #[test]
    fn missing_ledger_is_reported_by_both_owning_checks_without_escape() {
        let path = std::env::temp_dir().join(format!(
            "openwepp-gate-missing-ledger-{}-{}.jsonl",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let durable = check(
            "DURABLE_ATTEMPT_LEDGER",
            durable_ledger(&path),
            json!({"ledger_path": path.display().to_string()}),
        )
        .expect("durable check artifact");
        let defects = check(
            "OPEN_TOOLING_DEFECTS",
            no_open_tooling_defect(&path),
            json!({"ledger_path": path.display().to_string(), "ledger_sha256": null}),
        )
        .expect("defect check artifact");
        assert_eq!(durable["status"], "BLOCKED");
        assert_eq!(
            durable["reason_codes"],
            json!(["GATE-AUDIT-LEDGER-MISSING"])
        );
        assert_eq!(defects["status"], "BLOCKED");
        assert_eq!(defects["reason_codes"], json!(["GATE-AUDIT-LEDGER-READ"]));
    }

    #[test]
    fn tooling_defect_ledger_uses_the_last_status_for_each_defect() {
        let path = std::env::temp_dir().join(format!(
            "openwepp-gate-tooling-defect-{}-{}.jsonl",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::write(&path, "").expect("empty ledger");
        append_attempt_record(
            &path,
            json!({"record_type": "TOOLING_DEFECT", "defect_id": "RTR-OPEN", "status": "OPEN"}),
        )
        .expect("open defect");
        assert_eq!(
            no_open_tooling_defect(&path)
                .expect_err("open defect blocks")
                .code,
            "GATE-AUDIT-OPEN-TOOLING-DEFECT"
        );
        append_attempt_record(
            &path,
            json!({"record_type": "TOOLING_DEFECT", "defect_id": "RTR-OPEN", "status": "CLOSED"}),
        )
        .expect("close defect");
        no_open_tooling_defect(&path).expect("latest CLOSED status admits");
        fs::remove_file(path).expect("remove ledger");
    }

    #[test]
    fn every_light_heavy_execution_claim_must_match() {
        let light = json!({"workflow": "w", "job": "j", "runner": "r", "attempt": 1});
        let baseline = ExecutionClaims {
            workflow: "w".to_owned(),
            job: "j".to_owned(),
            runner: "r".to_owned(),
            attempt: 1,
            ..ExecutionClaims::default()
        };
        assert!(execution_claims_match(&light, &baseline));
        for field in ["workflow", "job", "runner", "attempt"] {
            let mut mutated = light.clone();
            mutated[field] = if field == "attempt" {
                json!(2)
            } else {
                json!("other")
            };
            assert!(!execution_claims_match(&mutated, &baseline), "{field}");
        }
    }

    #[test]
    fn failure_check_index_preserves_first_matching_token_precedence() {
        for (code, expected) in [
            ("GATE-PACKAGE-LIGHT", 0),
            ("GATE-LIGHT-DOC", 1),
            ("GATE-INVENTORY-PLAN", 2),
            ("GATE-EXECUTION-CLAIM", 3),
            ("GATE-ARTIFACT-CHECKPOINT", 4),
            ("GATE-ROOT-CACHE", 5),
            ("GATE-COMBINED-ORDER", 6),
            ("GATE-ORDER-RETRY", 7),
            ("GATE-LEDGER-UNKNOWN", 8),
            ("GATE-UNKNOWN", 9),
        ] {
            assert_eq!(failure_check_index(code), expected, "{code}");
        }
    }

    #[test]
    fn light_stage_and_stage_order_reject_nonpass_or_forward_dependency() {
        let mut plan = json!({
            "nodes": [
                {"node_id": "light", "execution_cost_class": "LIGHT", "prerequisites": []},
                {"node_id": "heavy", "execution_cost_class": "HEAVY", "prerequisites": ["light"]}
            ]
        });
        let receipt = json!({"final_results": {"light": "PASS"}});
        light_stage_passed(&plan, &receipt).expect("passing LIGHT receipt");
        valid_stage_order(&plan).expect("ordered LIGHT then HEAVY");

        plan["nodes"][1]["prerequisites"] = json!(["missing"]);
        assert_eq!(
            valid_stage_order(&plan)
                .expect_err("forward dependency")
                .code,
            "GATE-AUDIT-PREREQUISITE-ORDER"
        );
        assert_eq!(
            light_stage_passed(&plan, &json!({"final_results": {"light": "FAIL"}}))
                .expect_err("LIGHT failure")
                .code,
            "GATE-AUDIT-LIGHT-NONPASS"
        );
    }

    #[test]
    fn combined_decision_requires_its_exact_dag_shape() {
        let separate = json!({
            "nodes": [
                {"gate_definition_id": "workspace-full-nextest-v1"},
                {"gate_definition_id": "adjudicated-crap-v1"}
            ],
            "combined_quality": {
                "decision": "SEPARATE", "accepted_proof_id": null
            }
        });
        validate_combined_decision(&separate).expect("separate quality DAG");

        let mut drifted = separate.clone();
        drifted["combined_quality"]["decision"] = json!("COMBINED");
        assert_eq!(
            validate_combined_decision(&drifted)
                .expect_err("decision/DAG mismatch")
                .code,
            "GATE-AUDIT-COMBINED-DAG-MISMATCH"
        );
    }

    #[test]
    fn execution_identity_requires_all_bound_digests_and_claims() {
        let digest = "a".repeat(64);
        let plan = json!({"execution_context": {
            "configuration_sha256": digest,
            "environment_manifest_sha256": "b".repeat(64),
            "fixture_manifest_sha256": "c".repeat(64),
            "tool_manifest_sha256": "d".repeat(64)
        }});
        let receipt = json!({
            "claims": {
                "principal": "principal", "repository": "repository",
                "source_event": "event", "source_ref": "ref", "workflow": "workflow",
                "job": "job", "runner": "runner", "attempt": 1
            },
            "executor_binary_sha256": "e".repeat(64)
        });
        execution_identities(&plan, &receipt).expect("complete execution identity");

        let mut malformed = receipt.clone();
        malformed["claims"]["attempt"] = json!(0);
        assert_eq!(
            execution_identities(&plan, &malformed)
                .expect_err("zero attempt")
                .code,
            "GATE-AUDIT-EXECUTION-CLAIM"
        );
    }

    #[test]
    fn package_admission_selects_exactly_one_independently_valid_authority() {
        let plan = json!({
            "source": {"base_commit": "a"},
            "authorized_paths": [
                "docs/work-packages/one/package.md",
                "docs/work-packages/two/package.md"
            ]
        });
        let ready = json!({
            "status": "READY",
            "base_commit": "a",
            "package_path": "docs/work-packages/one/package.md",
            "changed_paths": plan["authorized_paths"],
            "reason_codes": []
        });
        let invalid = json!({
            "status": "INVALID",
            "base_commit": "a",
            "package_path": "docs/work-packages/two/package.md",
            "changed_paths": plan["authorized_paths"],
            "reason_codes": ["UNDECLARED_CHANGED_PATH"]
        });
        let selected = select_package_admission(&plan, vec![ready.clone(), invalid.clone()], 2)
            .expect("one exact authority");
        assert_eq!(selected["package_path"], ready["package_path"]);

        let error = select_package_admission(&plan, vec![ready.clone(), ready], 2)
            .expect_err("multiple exact authorities must remain ambiguous");
        assert_eq!(error.code, "GATE-AUDIT-PACKAGE-AMBIGUOUS");
        assert_eq!(error.class, ErrorClass::Identity);

        let error = select_package_admission(&plan, vec![invalid], 1)
            .expect_err("no exact authority must fail closed");
        assert_eq!(error.code, "GATE-AUDIT-PACKAGE-NOT-ADMITTED");
        assert_eq!(error.class, ErrorClass::Identity);
    }

    static PACKAGE_FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct PackageFixture {
        root: PathBuf,
        base: String,
        paths: Vec<String>,
    }

    impl PackageFixture {
        fn new(owner_ready: bool, contender_ready: bool) -> Self {
            let sequence = PACKAGE_FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "openwepp-package-admission-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(root.join("gate-policy/v1/schemas")).expect("schema directory");
            fs::create_dir_all(root.join("docs/work-packages/owner")).expect("owner directory");
            fs::create_dir_all(root.join("docs/work-packages/contender"))
                .expect("contender directory");
            fs::create_dir_all(root.join("src")).expect("source directory");
            let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            fs::copy(
                repository.join("gate-policy/v1/schemas/package-audit.schema.json"),
                root.join("gate-policy/v1/schemas/package-audit.schema.json"),
            )
            .expect("copy package schema");
            Self::write_package(&root, "owner", owner_ready, "base owner");
            Self::write_package(&root, "contender", contender_ready, "base contender");
            fs::write(root.join("src/lib.rs"), "pub fn base() {}\n").expect("base source");
            Self::git(&root, &["init", "-q"]);
            Self::git(&root, &["config", "user.email", "test@example.invalid"]);
            Self::git(&root, &["config", "user.name", "Test"]);
            Self::git(&root, &["add", "."]);
            Self::git(&root, &["commit", "-qm", "base"]);
            let base = String::from_utf8(Self::git_output(&root, &["rev-parse", "HEAD"]))
                .expect("UTF-8 base")
                .trim()
                .to_owned();
            Self::write_package(&root, "owner", owner_ready, "changed owner");
            Self::write_package(&root, "contender", contender_ready, "changed contender");
            fs::write(root.join("src/lib.rs"), "pub fn changed() {}\n").expect("changed source");
            Self {
                root,
                base,
                paths: vec![
                    "docs/work-packages/contender/package.md".to_owned(),
                    "docs/work-packages/owner/package.md".to_owned(),
                    "src/lib.rs".to_owned(),
                ],
            }
        }

        fn write_package(root: &std::path::Path, name: &str, ready: bool, note: &str) {
            let write_set = if ready {
                "- `docs/work-packages/**`\n- `src/**`".to_owned()
            } else {
                format!("- `docs/work-packages/{name}/**`")
            };
            fs::write(
                root.join(format!("docs/work-packages/{name}/package.md")),
                format!("# {name}\n\n{note}\n\n## Declared Write Set\n\n{write_set}\n"),
            )
            .expect("write package");
        }

        fn plan(&self) -> serde_json::Value {
            json!({
                "source": {"base_commit": self.base},
                "authorized_paths": self.paths
            })
        }

        fn git(root: &std::path::Path, arguments: &[&str]) {
            assert!(
                Command::new("git")
                    .args(arguments)
                    .current_dir(root)
                    .status()
                    .expect("run git")
                    .success(),
                "git {arguments:?}"
            );
        }

        fn git_output(root: &std::path::Path, arguments: &[&str]) -> Vec<u8> {
            let output = Command::new("git")
                .args(arguments)
                .current_dir(root)
                .output()
                .expect("run git");
            assert!(output.status.success(), "git {arguments:?}");
            output.stdout
        }
    }

    impl Drop for PackageFixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.root).expect("remove package fixture");
        }
    }

    #[test]
    fn package_admission_reconstructs_real_candidate_authority_and_fails_closed() {
        let unique = PackageFixture::new(true, false);
        let selected = package_admission(&unique.root, &unique.plan()).expect("unique authority");
        assert_eq!(
            selected["package_path"],
            "docs/work-packages/owner/package.md"
        );

        let multiple = PackageFixture::new(true, true);
        assert_eq!(
            package_admission(&multiple.root, &multiple.plan())
                .expect_err("multiple authorities")
                .code,
            "GATE-AUDIT-PACKAGE-AMBIGUOUS"
        );
        let none = PackageFixture::new(false, false);
        assert_eq!(
            package_admission(&none.root, &none.plan())
                .expect_err("no authority")
                .code,
            "GATE-AUDIT-PACKAGE-NOT-ADMITTED"
        );

        let mut mismatched = unique.plan();
        mismatched["authorized_paths"] = json!(&unique.paths[..2]);
        assert_eq!(
            package_admission(&unique.root, &mismatched)
                .expect_err("changed-path mismatch")
                .code,
            "GATE-AUDIT-PACKAGE-NOT-ADMITTED"
        );
        let mut invalid_base = unique.plan();
        invalid_base["source"]["base_commit"] = json!("not-a-commit");
        assert_eq!(
            package_admission(&unique.root, &invalid_base)
                .expect_err("invalid base")
                .code,
            "GATE-PACKAGE-GIT"
        );
        for (path, code) in [
            ("docs/work-packages/../package.md", "GATE-PACKAGE-PATH"),
            ("docs/work-packages/missing/package.md", "GATE-PACKAGE-READ"),
        ] {
            let plan = json!({
                "source": {"base_commit": unique.base},
                "authorized_paths": [path]
            });
            assert_eq!(
                package_admission(&unique.root, &plan)
                    .expect_err("invalid candidate")
                    .code,
                code
            );
        }
        let schema = unique
            .root
            .join("gate-policy/v1/schemas/package-audit.schema.json");
        let held_schema = schema.with_extension("held");
        fs::rename(&schema, &held_schema).expect("hold schema");
        let error = package_admission(&unique.root, &unique.plan()).expect_err("missing schema");
        fs::rename(held_schema, schema).expect("restore schema");
        assert_eq!(error.code, "GATE-PACKAGE-SCHEMA-READ");
    }
}
