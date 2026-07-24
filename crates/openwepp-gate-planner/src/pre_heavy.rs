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
use crate::package_validation::validate_package_chain;
use crate::planner::{reconstruct_plan_in, verify_plan_identity};
use crate::repository::remove_reconstruction_workspace;

pub const CHECK_IDS: [&str; 10] = [
    "PACKAGE_ADMISSION",
    "CHEAP_PREREQUISITES",
    "INVENTORY_AND_ARGUMENTS",
    "EXECUTION_IDENTITIES",
    "ATTEMPT_AND_OUTPUT_ISOLATION",
    "ROOTS_AND_EVIDENCE_REUSE",
    "QUALITY_DEFERRAL",
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
    (&["QUALITY"], 6),
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
pub fn build_audit(
    repo: &Path,
    plan: &Value,
    light_receipt: &Value,
    artifact_root: &Path,
    ledger: &Path,
) -> Result<Value> {
    verify_plan_identity(plan)?;
    validate_stage_receipt(repo, plan, light_receipt, artifact_root, true)?;
    let audit = build_unsealed_audit(repo, plan, light_receipt, artifact_root, ledger)?;
    seal_audit(repo, audit)
}

fn build_unsealed_audit(
    repo: &Path,
    plan: &Value,
    light_receipt: &Value,
    artifact_root: &Path,
    ledger: &Path,
) -> Result<Value> {
    let ledger_head_sha256 = ledger_head(ledger).ok().flatten();
    let package_admission = package_admission(repo, plan)?;
    let quality_disposition = plan["quality_disposition"].clone();
    let checks = build_audit_checks(&AuditCheckInputs {
        repo,
        plan,
        light_receipt,
        artifact_root,
        ledger,
        package_admission: &package_admission,
        quality_disposition: &quality_disposition,
        ledger_head_sha256: ledger_head_sha256.as_deref(),
    })?;
    let reason_codes = audit_reason_codes(&checks);
    let status = audit_status(&checks);
    let audit = audit_document(AuditDocumentInputs {
        plan,
        light_receipt,
        artifact_root,
        ledger,
        ledger_head_sha256,
        package_admission,
        checks,
        quality_disposition,
        status,
        reason_codes,
    })?;
    Ok(audit)
}

struct AuditCheckInputs<'a> {
    repo: &'a Path,
    plan: &'a Value,
    light_receipt: &'a Value,
    artifact_root: &'a Path,
    ledger: &'a Path,
    package_admission: &'a Value,
    quality_disposition: &'a Value,
    ledger_head_sha256: Option<&'a str>,
}

fn build_audit_checks(inputs: &AuditCheckInputs<'_>) -> Result<Vec<Value>> {
    let mut checks = audit_check_prefix(
        inputs.repo,
        inputs.plan,
        inputs.light_receipt,
        inputs.artifact_root,
        inputs.package_admission,
    )?;
    checks.extend(audit_check_middle(
        inputs.plan,
        inputs.light_receipt,
        inputs.artifact_root,
        inputs.quality_disposition,
    )?);
    checks.extend(audit_check_suffix(
        inputs.plan,
        inputs.ledger,
        inputs.ledger_head_sha256,
    )?);
    Ok(checks)
}

fn audit_check_prefix(
    repo: &Path,
    plan: &Value,
    light_receipt: &Value,
    artifact_root: &Path,
    package_admission: &Value,
) -> Result<Vec<Value>> {
    Ok(vec![
        check(
            CHECK_IDS[0],
            package_admitted(plan, package_admission),
            package_admission.clone(),
        )?,
        check(
            CHECK_IDS[1],
            cheap_prerequisites(repo, plan, light_receipt, package_admission),
            json!({"light_results": light_receipt["final_results"]}),
        )?,
        check(
            CHECK_IDS[2],
            inventory_and_arguments_are_exact(repo, plan, artifact_root),
            json!({"nodes": plan["nodes"]}),
        )?,
        check(
            CHECK_IDS[3],
            execution_identities(plan, light_receipt),
            json!({
                "policy": plan["policy"],
                "context": plan["execution_context"],
                "claims": light_receipt["claims"],
                "executor_binary_sha256": light_receipt["executor_binary_sha256"],
            }),
        )?,
    ])
}

fn audit_check_middle(
    plan: &Value,
    light_receipt: &Value,
    artifact_root: &Path,
    quality_disposition: &Value,
) -> Result<Vec<Value>> {
    Ok(vec![
        check(
            CHECK_IDS[4],
            light_attempt_isolated(plan, light_receipt, artifact_root),
            json!({"artifact_root_sha256": path_digest(artifact_root)}),
        )?,
        check(
            CHECK_IDS[5],
            separated_roots(plan),
            json!({"roots": plan["environment_roots"]}),
        )?,
        check(
            CHECK_IDS[6],
            validate_quality_deferral(plan),
            quality_disposition.clone(),
        )?,
    ])
}

fn audit_check_suffix(
    plan: &Value,
    ledger: &Path,
    ledger_head_sha256: Option<&str>,
) -> Result<Vec<Value>> {
    Ok(vec![
        check(
            CHECK_IDS[7],
            valid_stage_order(plan),
            json!({"nodes": node_manifest(plan)?}),
        )?,
        check(
            CHECK_IDS[8],
            durable_ledger(ledger),
            json!({"ledger_path_sha256": path_digest(ledger)}),
        )?,
        check(
            CHECK_IDS[9],
            no_open_tooling_defect_at_head(ledger, ledger_head_sha256),
            json!({
                "ledger_path_sha256": path_digest(ledger),
                "ledger_head_sha256": ledger_head_sha256,
            }),
        )?,
    ])
}

fn audit_reason_codes(checks: &[Value]) -> Vec<String> {
    checks
        .iter()
        .flat_map(|item| item["reason_codes"].as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn audit_status(checks: &[Value]) -> &'static str {
    if checks.iter().any(|item| item["status"] == "INVALID") {
        "INVALID"
    } else if checks.iter().any(|item| item["status"] == "BLOCKED") {
        "BLOCKED"
    } else {
        "READY"
    }
}

struct AuditDocumentInputs<'a> {
    plan: &'a Value,
    light_receipt: &'a Value,
    artifact_root: &'a Path,
    ledger: &'a Path,
    ledger_head_sha256: Option<String>,
    package_admission: Value,
    checks: Vec<Value>,
    quality_disposition: Value,
    status: &'a str,
    reason_codes: Vec<String>,
}

fn audit_document(inputs: AuditDocumentInputs<'_>) -> Result<Value> {
    let AuditDocumentInputs {
        plan,
        light_receipt,
        artifact_root,
        ledger,
        ledger_head_sha256,
        package_admission,
        checks,
        quality_disposition,
        status,
        reason_codes,
    } = inputs;
    Ok(json!({
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
        "quality_disposition": quality_disposition,
        "light_receipt": light_receipt,
    }))
}

fn seal_audit(repo: &Path, mut audit: Value) -> Result<Value> {
    audit["audit_id"] = Value::String(derived_id(&audit, "audit_id")?);
    let schema = read_json(&repo.join("gate-policy/v1/schemas/pre-heavy-audit.schema.json"))?;
    validate_schema(&schema, &audit, "pre-heavy audit")?;
    validate_sealed_audit_identity(&audit)?;
    Ok(audit)
}

fn validate_sealed_audit_identity(audit: &Value) -> Result<()> {
    if derived_id(audit, "audit_id")? != string(audit, "audit_id")? {
        return Err(audit_error(
            "GATE-AUDIT-IDENTITY",
            "generated audit identity mismatch",
        ));
    }
    Ok(())
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
        "quality_disposition": expected_quality_disposition(),
        "light_receipt": if light_receipt.is_object() {light_receipt.clone()} else {json!({})},
    });
    audit["audit_id"] = Value::String(derived_id(&audit, "audit_id")?);
    let schema = read_json(&repo.join("gate-policy/v1/schemas/pre-heavy-audit.schema.json"))?;
    validate_schema(&schema, &audit, "invalid pre-heavy audit")?;
    Ok(audit)
}

fn expected_quality_disposition() -> Value {
    json!({
        "status": "DEFERRED_TO_QUALITY_CI",
        "observations": ["COVERAGE", "CRAP"],
        "owner": "openwepp-quality-observatory",
        "trigger": "OPTIONAL_OPERATOR_DISPATCH",
        "closure_eligible": true,
        "prohibited_gate_definition_ids": [
            "affected-adjudicated-crap-v1",
            "adjudicated-crap-v1",
            "combined-workspace-quality-v1"
        ]
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
    validate_audit_schema(repo, audit)?;
    validate_audit_admission(repo, plan, audit, artifact_root)?;
    validate_audit_evidence(repo, plan, audit, artifact_root)
}

/// Verify a READY audit after its evidence archive has moved to another runner.
///
/// The execution-time audit and LIGHT receipt remain mutually bound to the
/// original attempt-root identity. The independent verifier checks that sealed
/// identity instead of hashing its unrelated extraction pathname.
///
/// # Errors
///
/// Returns a typed execution error for a malformed, substituted, stale, or
/// internally inconsistent audit or LIGHT receipt.
pub fn validate_relocated_audit(repo: &Path, plan: &Value, audit: &Value) -> Result<()> {
    validate_audit_schema(repo, audit)?;
    let current_package_admission = package_admission(repo, plan)?;
    validate_audit_core_binding(plan, audit)?;
    validate_relocated_artifact_binding(plan, audit)?;
    validate_audit_policy_fields(plan, audit, &current_package_admission)?;
    validate_ready_audit(audit)?;
    validate_relocated_light_receipt(repo, plan, audit)?;
    validate_current_audit_inventory(plan, audit)
}

fn validate_audit_admission(
    repo: &Path,
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
) -> Result<()> {
    let current_package_admission = package_admission(repo, plan)?;
    validate_audit_bindings(plan, audit, artifact_root, &current_package_admission)
}

fn validate_audit_evidence(
    repo: &Path,
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
) -> Result<()> {
    validate_ready_audit(audit)?;
    validate_embedded_light_receipt(repo, plan, audit, artifact_root)?;
    validate_current_audit_inventory(plan, audit)
}

fn validate_audit_schema(repo: &Path, audit: &Value) -> Result<()> {
    let schema = read_json(&repo.join("gate-policy/v1/schemas/pre-heavy-audit.schema.json"))?;
    validate_schema(&schema, audit, "pre-heavy audit")
}

fn validate_audit_bindings(
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
    current_package_admission: &Value,
) -> Result<()> {
    validate_audit_core_binding(plan, audit)?;
    validate_audit_context_binding(plan, audit, artifact_root, current_package_admission)
}

fn validate_ready_audit(audit: &Value) -> Result<()> {
    require_ready_audit_status(audit)?;
    validate_ready_check_set(audit)
}

fn validate_embedded_light_receipt(
    repo: &Path,
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
) -> Result<()> {
    validate_stage_receipt(repo, plan, &audit["light_receipt"], artifact_root, false)?;
    validate_embedded_light_receipt_id(audit)
}

fn validate_relocated_light_receipt(repo: &Path, plan: &Value, audit: &Value) -> Result<()> {
    let receipt = &audit["light_receipt"];
    let schema = read_json(&repo.join("gate-policy/v1/schemas/stage-receipt.schema.json"))?;
    validate_schema(&schema, receipt, "light stage receipt")?;
    validate_stage_receipt_plan_binding(plan, receipt)?;
    validate_stage_receipt_binary_binding(receipt, false)?;
    if receipt["execution_key"] != plan["execution_key"] || receipt["stage"] != "LIGHT" {
        return invalid_stage_receipt_binding();
    }
    validate_embedded_light_receipt_id(audit)
}

fn validate_audit_core_binding(plan: &Value, audit: &Value) -> Result<()> {
    validate_audit_identity_fields(plan, audit)?;
    validate_audit_plan_fields(plan, audit)
}

fn validate_audit_identity_fields(plan: &Value, audit: &Value) -> Result<()> {
    require_audit_binding(derived_id(audit, "audit_id")? == string(audit, "audit_id")?)?;
    require_audit_binding(audit["plan_id"] == plan["plan_id"])
}

fn validate_audit_plan_fields(plan: &Value, audit: &Value) -> Result<()> {
    require_audit_binding(audit["plan_sha256"] == digest(plan)?)?;
    require_audit_binding(audit["execution_key"] == plan["execution_key"])
}

fn validate_audit_context_binding(
    plan: &Value,
    audit: &Value,
    artifact_root: &Path,
    current_package_admission: &Value,
) -> Result<()> {
    validate_audit_artifact_fields(plan, audit, artifact_root)?;
    validate_audit_policy_fields(plan, audit, current_package_admission)
}

fn validate_audit_artifact_fields(plan: &Value, audit: &Value, artifact_root: &Path) -> Result<()> {
    require_audit_binding(audit["artifact_root_sha256"] == path_digest(artifact_root))?;
    require_audit_binding(audit["node_manifest"] == node_manifest(plan)?)
}

fn validate_relocated_artifact_binding(plan: &Value, audit: &Value) -> Result<()> {
    require_audit_binding(
        audit["artifact_root_sha256"] == audit["light_receipt"]["artifact_root_sha256"],
    )?;
    require_audit_binding(audit["node_manifest"] == node_manifest(plan)?)
}

fn validate_audit_policy_fields(
    plan: &Value,
    audit: &Value,
    current_package_admission: &Value,
) -> Result<()> {
    require_audit_binding(audit["quality_disposition"] == plan["quality_disposition"])?;
    require_audit_binding(audit["package_admission"] == *current_package_admission)
}

fn require_audit_binding(matches: bool) -> Result<()> {
    if matches {
        Ok(())
    } else {
        Err(audit_error("GATE-AUDIT-IDENTITY", "audit binding mismatch"))
    }
}

fn require_ready_audit_status(audit: &Value) -> Result<()> {
    if audit["status"] != "READY" {
        return Err(audit_error(
            "GATE-AUDIT-NOT-READY",
            audit["status"].to_string(),
        ));
    }
    Ok(())
}

fn validate_ready_check_set(audit: &Value) -> Result<()> {
    let checks = ready_checks(audit)?;
    validate_ready_checks(checks)?;
    validate_ready_reason_codes(audit)
}

fn ready_checks(audit: &Value) -> Result<&[Value]> {
    let checks = audit["checks"]
        .as_array()
        .ok_or_else(|| audit_error("GATE-AUDIT-CHECK-SET", "checks must be an array"))?;
    if checks.len() == CHECK_IDS.len() {
        Ok(checks)
    } else {
        invalid_ready_check_set()
    }
}

fn validate_ready_checks(checks: &[Value]) -> Result<()> {
    for (item, expected) in checks.iter().zip(CHECK_IDS) {
        validate_ready_check(item, expected)?;
    }
    Ok(())
}

fn validate_ready_reason_codes(audit: &Value) -> Result<()> {
    let reasons = audit["reason_codes"].as_array();
    if reasons.is_none_or(|codes| !codes.is_empty()) {
        invalid_ready_check_set()
    } else {
        Ok(())
    }
}

fn validate_ready_check(item: &Value, expected: &str) -> Result<()> {
    let reasons = item["reason_codes"].as_array();
    if item["check_id"] == expected
        && item["status"] == "PASS"
        && reasons.is_some_and(Vec::is_empty)
    {
        Ok(())
    } else {
        invalid_ready_check_set()
    }
}

fn invalid_ready_check_set<T>() -> Result<T> {
    Err(audit_error(
        "GATE-AUDIT-CHECK-SET",
        "READY requires the ordered canonical ten-check set, all PASS, with no reasons",
    ))
}

fn validate_embedded_light_receipt_id(audit: &Value) -> Result<()> {
    if audit["light_stage_receipt_id"] == audit["light_receipt"]["stage_receipt_id"] {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-LIGHT-RECEIPT",
            "light receipt was substituted",
        ))
    }
}

fn validate_current_audit_inventory(plan: &Value, audit: &Value) -> Result<()> {
    let current_inventory = node_manifest(plan)?;
    if current_inventory == audit["node_manifest"] {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-INVENTORY-DRIFT",
            "independent current inventory differs from admitted inventory",
        ))
    }
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

/// Close one exact open tooling defect and optionally invalidate its failed
/// recovery root.
///
/// # Errors
///
/// Returns a typed ledger error unless the ledger is valid, the named defect's
/// latest state is open, and every closure binding is canonical.
pub fn close_tooling_defect(
    path: &Path,
    defect_id: &str,
    correction_commit: &str,
    closure_evidence: &str,
    invalidated_recovery_root: Option<&Path>,
) -> Result<String> {
    admit_attempt_ledger(path)?;
    if defect_id.is_empty()
        || closure_evidence.trim().is_empty()
        || correction_commit.len() != 40
        || !correction_commit
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(audit_error("GATE-AUDIT-DEFECT-CLOSURE-SHAPE", defect_id));
    }
    let records = fs::read_to_string(path)
        .map_err(|error| audit_error("GATE-AUDIT-LEDGER-READ", error.to_string()))?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_strict(line.as_bytes()))
        .collect::<Result<Vec<_>>>()?;
    let (latest_index, latest) = records
        .iter()
        .enumerate()
        .rev()
        .find(|(_, item)| item["record_type"] == "TOOLING_DEFECT" && item["defect_id"] == defect_id)
        .ok_or_else(|| audit_error("GATE-AUDIT-DEFECT-CLOSURE-UNKNOWN", defect_id))?;
    if tooling_defect_status(latest)? != "OPEN" {
        return Err(audit_error("GATE-AUDIT-DEFECT-CLOSURE-NOT-OPEN", defect_id));
    }
    let mut record = json!({
        "record_type": "TOOLING_DEFECT",
        "defect_id": defect_id,
        "status": "CLOSED",
        "owner": "openwepp-maintainers",
        "correction_commit": correction_commit,
        "closure_evidence": closure_evidence,
    });
    if let Some(root) = invalidated_recovery_root {
        let cause_key = latest["cause_key"]
            .as_str()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| audit_error("GATE-AUDIT-DEFECT-CLOSURE-SHAPE", "cause_key"))?;
        let expected_parent = path
            .parent()
            .ok_or_else(|| {
                audit_error("GATE-AUDIT-DEFECT-CLOSURE-PATH", path.display().to_string())
            })?
            .join("recovery");
        if !root.is_absolute()
            || root.parent() != Some(expected_parent.as_path())
            || !matches!(
                root.components().next_back(),
                Some(std::path::Component::Normal(_))
            )
        {
            return Err(audit_error(
                "GATE-AUDIT-DEFECT-CLOSURE-PATH",
                root.display().to_string(),
            ));
        }
        let associated = records[..latest_index].iter().rev().any(|item| {
            item["record_type"] == "STAGE_ATTEMPT"
                && item["stage"] == "HEAVY"
                && item["status"] == "FAILED"
                && item["cause_key"] == cause_key
                && item["recovery_root"] == root.display().to_string()
        });
        if !associated {
            return Err(audit_error(
                "GATE-AUDIT-DEFECT-CLOSURE-UNASSOCIATED",
                root.display().to_string(),
            ));
        }
        record["cause_key"] = json!(cause_key);
        record["invalidated_recovery_root"] = json!(root);
    }
    append_attempt_record(path, record)
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
    let base = string(&plan["source"], "base_commit")?;
    let head = string(&plan["source"], "head_commit")?;
    let package = string(&plan["package_authority"], "intent_package_path")?;
    let chain = validate_package_chain(repo, base, Some(head), Path::new(package))?;
    package_admitted(plan, &chain)?;
    Ok(chain)
}

fn package_admitted(plan: &Value, result: &Value) -> Result<()> {
    if result["status"] != "READY"
        || result["changed_paths"] != plan["authorized_paths"]
        || result["base_commit"] != plan["source"]["base_commit"]
        || result["head_commit"] != plan["source"]["head_commit"]
        || result["package_authority_chain_id"] != plan["package_authority"]["chain_id"]
        || result["intent_package_path"] != plan["package_authority"]["intent_package_path"]
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

fn cheap_prerequisites(
    repo: &Path,
    plan: &Value,
    receipt: &Value,
    package_admission: &Value,
) -> Result<()> {
    light_stage_passed(plan, receipt)?;
    require_clean_diff(repo, plan)?;
    enforce_authorized_rust_line_limit(repo, plan)?;
    documentation_scope_is_exact(plan)?;
    require_bound_active_prompt(repo, package_admission)
}

fn require_clean_diff(repo: &Path, plan: &Value) -> Result<()> {
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
    Ok(())
}

fn enforce_authorized_rust_line_limit(repo: &Path, plan: &Value) -> Result<()> {
    let deleted_paths = plan["changed_objects"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|change| change["change_kind"] == "DELETE")
        .filter_map(|change| change["path"].as_str())
        .collect::<BTreeSet<_>>();
    for path in plan["authorized_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|path| !deleted_paths.contains(path))
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
    Ok(())
}

fn require_bound_active_prompt(repo: &Path, admission: &Value) -> Result<()> {
    let prompt = string(&admission["prompt_owner"], "prompt_path")?;
    let prompt_path = Path::new(prompt);
    let active = prompt_path
        .parent()
        .ok_or_else(|| audit_error("GATE-AUDIT-PROMPT-STATE", prompt))?;
    let entries = fs::read_dir(repo.join(active))
        .map_err(|error| audit_error("GATE-AUDIT-PROMPT-STATE", error.to_string()))?;
    let mut markdown = Vec::new();
    for entry in entries {
        let entry =
            entry.map_err(|error| audit_error("GATE-AUDIT-PROMPT-STATE", error.to_string()))?;
        if entry.path().extension().is_some_and(|value| value == "md") {
            let kind = entry
                .file_type()
                .map_err(|error| audit_error("GATE-AUDIT-PROMPT-STATE", error.to_string()))?;
            if !kind.is_file() {
                return Err(audit_error(
                    "GATE-AUDIT-PROMPT-STATE",
                    "active Markdown prompt must be a regular file",
                ));
            }
            markdown.push(entry.path());
        }
    }
    let bound = repo.join(prompt_path);
    if markdown.len() != 1 || markdown[0] != bound {
        return Err(audit_error(
            "GATE-AUDIT-PROMPT-STATE",
            "active directory does not contain exactly the bound prompt",
        ));
    }
    let bytes = fs::read(bound)
        .map_err(|error| audit_error("GATE-AUDIT-PROMPT-STATE", error.to_string()))?;
    if sha256_bytes(&bytes) == string(&admission["prompt_owner"], "prompt_sha256")? {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-PROMPT-STATE",
            "bound active prompt digest changed",
        ))
    }
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
        validate_exact_node_shape(node, &mut ids)?;
    }
    Ok(())
}

fn validate_exact_node_shape<'a>(node: &'a Value, ids: &mut BTreeSet<&'a str>) -> Result<()> {
    let id = string(node, "node_id")?;
    if ids.insert(id)
        && node["arguments"].is_array()
        && node["expected_inventory"]["mode"] == "EXACT"
    {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-INVENTORY-INVALID",
            "node identity, arguments, or exact inventory is invalid",
        ))
    }
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
        validate_light_node_checkpoint(node, artifact_root)?;
    }
    Ok(())
}

fn validate_light_node_checkpoint(node: &Value, artifact_root: &Path) -> Result<()> {
    let node_id = string(node, "node_id")?;
    let checkpoint = read_json(
        &artifact_root
            .join(".checkpoints")
            .join(format!("{node_id}.json")),
    )?;
    validate_checkpoint_identity(node, &checkpoint, node_id)?;
    validate_checkpoint_artifacts(node, &checkpoint, artifact_root)
}

fn validate_checkpoint_identity(node: &Value, checkpoint: &Value, node_id: &str) -> Result<()> {
    if checkpoint["node_sha256"] == digest(node)? && checkpoint["result"] == "PASS" {
        Ok(())
    } else {
        Err(audit_error("GATE-AUDIT-CHECKPOINT-DRIFT", node_id))
    }
}

fn validate_checkpoint_artifacts(
    node: &Value,
    checkpoint: &Value,
    artifact_root: &Path,
) -> Result<()> {
    for relative in node["output_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        validate_checkpoint_artifact(checkpoint, artifact_root, relative)?;
    }
    Ok(())
}

fn validate_checkpoint_artifact(
    checkpoint: &Value,
    artifact_root: &Path,
    relative: &str,
) -> Result<()> {
    let expected = checkpoint["artifacts"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|artifact| artifact["path"] == relative)
        .and_then(|artifact| artifact["sha256"].as_str())
        .ok_or_else(|| audit_error("GATE-AUDIT-CHECKPOINT-ARTIFACT", relative))?;
    if file_digest(&artifact_root.join(relative))? == expected {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-CHECKPOINT-ARTIFACT-DRIFT",
            relative,
        ))
    }
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

fn validate_node_stage_order<'a>(node: &'a Value, seen: &mut BTreeSet<&'a str>) -> Result<()> {
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
    Ok(())
}

fn durable_ledger(path: &Path) -> Result<()> {
    validate_ledger_path_nofollow(path)?;
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

fn validate_ledger_path_nofollow(path: &Path) -> Result<()> {
    if !path.is_absolute()
        || path.components().any(|component| {
            !matches!(
                component,
                std::path::Component::RootDir | std::path::Component::Normal(_)
            )
        })
    {
        return Err(audit_error(
            "GATE-AUDIT-LEDGER-PATH",
            path.display().to_string(),
        ));
    }
    for (index, ancestor) in path.ancestors().enumerate() {
        let metadata = fs::symlink_metadata(ancestor)
            .map_err(|error| audit_error("GATE-AUDIT-LEDGER-MISSING", error.to_string()))?;
        let invalid = metadata.file_type().is_symlink()
            || index == 0 && !metadata.is_file()
            || index != 0 && !metadata.is_dir();
        if invalid {
            return Err(audit_error(
                "GATE-AUDIT-LEDGER-PATH",
                ancestor.display().to_string(),
            ));
        }
    }
    Ok(())
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
            let status = tooling_defect_status(&item)?;
            defects.insert(defect_id.to_owned(), status.to_owned());
        }
    }
    Ok(defects)
}

pub(crate) fn tooling_defect_status(item: &Value) -> Result<&str> {
    match item["status"].as_str() {
        Some(status @ ("OPEN" | "CLOSED")) => Ok(status),
        _ => Err(audit_error("GATE-AUDIT-TOOLING-DEFECT-SHAPE", "status")),
    }
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

fn validate_quality_deferral(plan: &Value) -> Result<()> {
    if plan["quality_disposition"] != expected_quality_disposition()
        || plan.get("combined_quality").is_some()
        || plan.get("quality_scope").is_some()
    {
        return Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-QUALITY-DISPOSITION",
            "quality disposition does not match immutable policy",
        ));
    }
    let has_prohibited_node = nodes(plan).unwrap_or(&[]).iter().any(|node| {
        matches!(
            node["gate_definition_id"].as_str(),
            Some(
                "affected-adjudicated-crap-v1"
                    | "adjudicated-crap-v1"
                    | "combined-workspace-quality-v1"
            )
        ) || matches!(
            node["gate_family"].as_str(),
            Some("coverage-complexity" | "combined-quality")
        ) || node["artifact_contract"] == "adjudicated-crap-v1"
    });
    if has_prohibited_node {
        Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-AUDIT-QUALITY-NODE-PROHIBITED",
            "coverage/CRAP execution cannot enter the TESTGATE DAG",
        ))
    } else {
        Ok(())
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
    validate_stage_receipt_plan_binding(plan, receipt)?;
    validate_stage_receipt_execution_binding(plan, receipt, artifact_root)?;
    validate_stage_receipt_binary_binding(receipt, enforce_current_binary)
}

fn validate_stage_receipt_plan_binding(plan: &Value, receipt: &Value) -> Result<()> {
    validate_stage_receipt_identity(receipt)?;
    validate_stage_receipt_plan_fields(plan, receipt)
}

fn validate_stage_receipt_identity(receipt: &Value) -> Result<()> {
    if derived_id(receipt, "stage_receipt_id")? == string(receipt, "stage_receipt_id")? {
        Ok(())
    } else {
        invalid_stage_receipt_binding()
    }
}

fn validate_stage_receipt_plan_fields(plan: &Value, receipt: &Value) -> Result<()> {
    if receipt["plan_id"] == plan["plan_id"] && receipt["plan_sha256"] == digest(plan)? {
        Ok(())
    } else {
        invalid_stage_receipt_binding()
    }
}

fn invalid_stage_receipt_binding<T>() -> Result<T> {
    Err(audit_error(
        "GATE-AUDIT-STAGE-RECEIPT-IDENTITY",
        "light stage receipt binding mismatch",
    ))
}

fn validate_stage_receipt_execution_binding(
    plan: &Value,
    receipt: &Value,
    artifact_root: &Path,
) -> Result<()> {
    if receipt["execution_key"] == plan["execution_key"]
        && receipt["artifact_root_sha256"] == path_digest(artifact_root)
        && receipt["stage"] == "LIGHT"
    {
        Ok(())
    } else {
        Err(audit_error(
            "GATE-AUDIT-STAGE-RECEIPT-IDENTITY",
            "light stage receipt binding mismatch",
        ))
    }
}

fn validate_stage_receipt_binary_binding(
    receipt: &Value,
    enforce_current_binary: bool,
) -> Result<()> {
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
#[path = "pre_heavy_coverage_tests.rs"]
mod coverage_tests;

#[cfg(test)]
#[path = "pre_heavy_tests.rs"]
mod tests;
