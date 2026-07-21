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
use crate::planner::verify_plan_identity;

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

/// Build the only artifact that may authorize the heavy execution stage.
///
/// # Errors
///
/// Returns a typed error when an input cannot be parsed or the produced report
/// violates its schema or derived identity.
pub fn build_audit(
    repo: &Path,
    plan: &Value,
    light_receipt: &Value,
    artifact_root: &Path,
    ledger: &Path,
) -> Result<Value> {
    verify_plan_identity(plan)?;
    validate_stage_receipt(repo, plan, light_receipt, artifact_root, true)?;

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
        inventory_is_exact(plan),
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
        no_open_tooling_defect(ledger),
        json!({
            "ledger_path_sha256": path_digest(ledger),
            "ledger_sha256": fs::read(ledger).ok().map(|bytes| sha256_bytes(&bytes)),
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
    if code.contains("PACKAGE") {
        0
    } else if code.contains("LIGHT") || code.contains("DOC") || code.contains("LINE") {
        1
    } else if code.contains("INVENTORY") || code.contains("PLAN") {
        2
    } else if code.contains("EXECUT") || code.contains("CLAIM") {
        3
    } else if code.contains("ARTIFACT") || code.contains("CHECKPOINT") || code.contains("COLLISION")
    {
        4
    } else if code.contains("ROOT") || code.contains("CACHE") {
        5
    } else if code.contains("COMBIN") {
        6
    } else if code.contains("ORDER") || code.contains("RETRY") || code.contains("HANDOFF") {
        7
    } else if code.contains("LEDGER") {
        8
    } else {
        9
    }
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
    let current = current_executable_sha256()?;
    if audit["executor_binary_sha256"] != current {
        return Err(audit_error(
            "GATE-AUDIT-EXECUTOR-BINARY-DRIFT",
            "HEAVY executor differs from the binary that emitted the LIGHT receipt",
        ));
    }
    let light = &audit["light_receipt"]["claims"];
    if !execution_claims_match(light, claims) {
        return Err(audit_error(
            "GATE-AUDIT-EXECUTION-CLAIM-DRIFT",
            "HEAVY workflow/job/runner/attempt differs from LIGHT",
        ));
    }
    Ok(())
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
) -> Result<()> {
    if audit["ledger_path_sha256"] != path_digest(ledger) {
        return Err(audit_error(
            "GATE-AUDIT-LEDGER-SUBSTITUTION",
            ledger.display().to_string(),
        ));
    }
    durable_ledger(ledger)?;
    no_open_tooling_defect(ledger)?;
    let reconstructed = build_audit(repo, plan, &audit["light_receipt"], artifact_root, ledger)?;
    if &reconstructed != audit {
        return Err(audit_error(
            "GATE-AUDIT-RECONSTRUCTION-MISMATCH",
            "submitted audit differs from an independent reconstruction",
        ));
    }
    Ok(())
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
    if packages.len() != 1 {
        return Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-PACKAGE-AMBIGUOUS",
            format!(
                "expected exactly one package authority, found {}",
                packages.len()
            ),
        ));
    }
    let base = string(&plan["source"], "base_commit")?;
    validate_package(repo, base, Path::new(packages[0]))
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
    for node in nodes(plan)? {
        if node["execution_cost_class"] == "LIGHT" {
            let id = string(node, "node_id")?;
            if receipt["final_results"][id] != "PASS" {
                return Err(audit_error(
                    "GATE-AUDIT-LIGHT-NONPASS",
                    format!("light node {id} did not pass"),
                ));
            }
        }
    }
    Ok(())
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
        .filter(|path| path.ends_with(".rs"))
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
    if plan["authorized_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .any(|path| path.ends_with(".md"))
    {
        let lint = Command::new("markdown-doc")
            .args(["lint"])
            .current_dir(repo)
            .output()
            .map_err(|error| audit_error("GATE-AUDIT-DOC-LINT", error.to_string()))?;
        if !lint.status.success() {
            return Err(audit_error(
                "GATE-AUDIT-DOC-LINT",
                String::from_utf8_lossy(&lint.stdout),
            ));
        }
    }
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

fn inventory_is_exact(plan: &Value) -> Result<()> {
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

fn execution_identities(plan: &Value, receipt: &Value) -> Result<()> {
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
    if receipt["claims"]["attempt"]
        .as_u64()
        .is_none_or(|attempt| attempt == 0)
    {
        return Err(audit_error(
            "GATE-AUDIT-EXECUTION-CLAIM",
            "attempt must be positive",
        ));
    }
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
    let nodes = nodes(plan)?;
    let mut seen = BTreeSet::new();
    for node in nodes {
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
        seen.insert(string(node, "node_id")?);
    }
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
    for (defect_id, status) in defects {
        if status == "OPEN" {
            return Err(audit_error("GATE-AUDIT-OPEN-TOOLING-DEFECT", defect_id));
        }
    }
    Ok(())
}

fn validate_combined_decision(plan: &Value) -> Result<()> {
    let definitions = nodes(plan)
        .unwrap_or(&[])
        .iter()
        .filter_map(|node| node["gate_definition_id"].as_str())
        .collect::<BTreeSet<_>>();
    let decision = &plan["combined_quality"];
    let combined = definitions.contains("combined-workspace-quality-v1");
    let separate = definitions.contains("workspace-full-nextest-v1")
        && definitions.contains("adjudicated-crap-v1");
    let valid = if decision["decision"] == "COMBINED" {
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
    };
    if valid {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-COMBINED-DAG-MISMATCH",
            "combined-quality decision does not match the immutable DAG",
        ))
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

    use serde_json::json;

    use super::{
        append_attempt_record, build_failure_audit, check, durable_ledger, execution_claims_match,
        no_open_tooling_defect, package_admission, package_admitted, read_json,
        reconcile_orphaned_attempts, record_heavy_failure, verify_ledger_chain,
    };
    use crate::canonical::{parse_strict, validate_schema};
    use crate::error::{ErrorClass, GatePolicyError};
    use crate::executor::ExecutionClaims;

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
    fn multiple_changed_package_authorities_are_invalid() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let error = package_admission(
            &root,
            &json!({
                "authorized_paths": [
                    "docs/work-packages/one/package.md",
                    "docs/work-packages/two/package.md"
                ]
            }),
        )
        .expect_err("authority must not be inferred");
        assert_eq!(error.code, "GATE-AUDIT-PACKAGE-AMBIGUOUS");
        assert_eq!(error.class, ErrorClass::Identity);
    }
}
