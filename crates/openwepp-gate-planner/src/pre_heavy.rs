//! Canonical admission artifact for the LIGHT-to-HEAVY execution transition.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::{Value, json};

use crate::canonical::{derived_id, digest, parse_strict, sha256_bytes, validate_schema};
use crate::error::{ErrorClass, GatePolicyError, Result};
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
    validate_stage_receipt(repo, plan, light_receipt, artifact_root)?;

    let mut checks = Vec::with_capacity(CHECK_IDS.len());
    checks.push(check(
        CHECK_IDS[0],
        package_admitted(repo, plan),
        json!({"base": plan["source"]["base_commit"], "paths": plan["authorized_paths"]}),
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
        Ok(()),
        json!({"policy": plan["policy"], "context": plan["execution_context"]}),
    )?);
    checks.push(check(
        CHECK_IDS[4],
        artifact_identity(light_receipt, artifact_root),
        json!({"artifact_root_sha256": path_digest(artifact_root)}),
    )?);
    checks.push(check(
        CHECK_IDS[5],
        separated_roots(plan),
        json!({"roots": plan["environment_roots"]}),
    )?);
    let combined_execution = combined_decision(plan);
    checks.push(check(CHECK_IDS[6], Ok(()), combined_execution.clone())?);
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
        json!({"ledger_sha256": file_digest(ledger)?}),
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
        "light_stage_receipt_id": light_receipt["stage_receipt_id"],
        "artifact_root_sha256": path_digest(artifact_root),
        "ledger_path_sha256": path_digest(ledger),
        "node_manifest": node_manifest(plan)?,
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
    if derived_id(audit, "audit_id")? != string(audit, "audit_id")?
        || audit["plan_id"] != plan["plan_id"]
        || audit["plan_sha256"] != digest(plan)?
        || audit["execution_key"] != plan["execution_key"]
        || audit["artifact_root_sha256"] != path_digest(artifact_root)
        || audit["node_manifest"] != node_manifest(plan)?
    {
        return Err(audit_error("GATE-AUDIT-IDENTITY", "audit binding mismatch"));
    }
    if audit["status"] != "READY" {
        return Err(audit_error(
            "GATE-AUDIT-NOT-READY",
            audit["status"].to_string(),
        ));
    }
    validate_stage_receipt(repo, plan, &audit["light_receipt"], artifact_root)?;
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

/// Verify that a resume ledger is the exact durable ledger admitted by audit.
///
/// # Errors
///
/// Returns a typed execution error for path substitution, ephemeral storage,
/// unreadable history, or an unresolved tooling defect.
pub fn validate_resume_ledger(audit: &Value, ledger: &Path) -> Result<()> {
    if audit["ledger_path_sha256"] != path_digest(ledger) {
        return Err(audit_error(
            "GATE-AUDIT-LEDGER-SUBSTITUTION",
            ledger.display().to_string(),
        ));
    }
    durable_ledger(ledger)?;
    no_open_tooling_defect(ledger)
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

fn package_admitted(repo: &Path, plan: &Value) -> Result<()> {
    let package = plan["authorized_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .find(|path| path.starts_with("docs/work-packages/") && path.ends_with("/package.md"))
        .ok_or_else(|| audit_error("GATE-AUDIT-PACKAGE-MISSING", "no package path admitted"))?;
    let base = string(&plan["source"], "base_commit")?;
    let object = format!("{base}:{package}");
    let output = Command::new("git")
        .args(["cat-file", "-e", &object])
        .current_dir(repo)
        .output()
        .map_err(|error| audit_error("GATE-AUDIT-GIT", error.to_string()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(audit_error(
            "SCAFFOLD_COMMIT_REQUIRED",
            "package does not exist in authenticated base",
        ))
    }
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
    if absolute.starts_with("/tmp") || !absolute.is_file() {
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

fn combined_decision(plan: &Value) -> Value {
    let definitions = nodes(plan)
        .unwrap_or(&[])
        .iter()
        .filter_map(|node| node["gate_definition_id"].as_str())
        .collect::<BTreeSet<_>>();
    if definitions.contains("workspace-full-nextest-v1")
        && definitions.contains("adjudicated-crap-v1")
    {
        json!({
            "decision": "SEPARATE",
            "reason_code": "COMBINATION_NOT_ADOPTED_INSUFFICIENT_COMPATIBLE_HISTORY"
        })
    } else {
        json!({"decision": "NOT_APPLICABLE", "reason_code": "NO_DUPLICATE_FULL_INVENTORY"})
    }
}

fn validate_stage_receipt(
    repo: &Path,
    plan: &Value,
    receipt: &Value,
    artifact_root: &Path,
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
