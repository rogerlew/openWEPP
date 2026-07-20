//! Verified cross-attempt import for current, target-reusable node receipts.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::canonical::parse_strict;
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::executor::ExecutionClaims;
use crate::verifier::{DirectoryArtifacts, verify_receipt};

pub struct ResumeCandidate {
    receipt: Value,
    artifact_root: PathBuf,
}

pub struct ResumeSeed {
    pub attempts: Vec<Value>,
    pub final_results: BTreeMap<String, String>,
    pub executed_inventory: BTreeSet<String>,
    pub decisions: Vec<Value>,
}

/// Find and independently verify the newest prior aggregate receipt named by
/// the append-only attempt ledger.
///
/// # Errors
///
/// Returns a typed error for malformed ledger records or a receipt that claims
/// the current plan but fails independent verification.
pub fn load_candidate(repo: &Path, plan: &Value, ledger: &Path) -> Result<Option<ResumeCandidate>> {
    let text = fs::read_to_string(ledger)
        .map_err(|error| resume_error("GATE-RESUME-LEDGER", error.to_string()))?;
    for line in text.lines().rev().filter(|line| !line.trim().is_empty()) {
        let item = parse_strict(line.as_bytes())?;
        if item["record_type"] != "ATTEMPT" || item["plan_id"] != plan["plan_id"] {
            continue;
        }
        let Some(root) = item["artifact_root"].as_str().map(PathBuf::from) else {
            continue;
        };
        let receipt_path = root.join("receipt.json");
        let artifact_root = root.join("execution");
        if !receipt_path.is_file() || !artifact_root.is_dir() {
            continue;
        }
        let receipt = parse_strict(&fs::read(&receipt_path).map_err(|error| {
            resume_error(
                "GATE-RESUME-RECEIPT",
                format!("{}: {error}", receipt_path.display()),
            )
        })?)?;
        let artifacts = DirectoryArtifacts::new(artifact_root.clone());
        verify_receipt(repo, plan, &receipt, &artifacts).map_err(|error| {
            resume_error(
                "GATE-RESUME-RECEIPT-INVALID",
                format!("{}: {}", error.code, error.message),
            )
        })?;
        return Ok(Some(ResumeCandidate {
            receipt,
            artifact_root,
        }));
    }
    Ok(None)
}

/// Import eligible PASS attempts and their verified artifacts into a fresh
/// attempt root while recording every import or rejection reason.
///
/// # Errors
///
/// Returns a typed error for artifact substitution, output collision, or an
/// invalid node shape.
pub fn apply_candidate(
    plan: &Value,
    current_root: &Path,
    claims: &ExecutionClaims,
    candidate: Option<&ResumeCandidate>,
) -> Result<ResumeSeed> {
    let mut seed = ResumeSeed {
        attempts: Vec::new(),
        final_results: BTreeMap::new(),
        executed_inventory: BTreeSet::new(),
        decisions: Vec::new(),
    };
    let Some(candidate) = candidate else {
        return Ok(seed);
    };
    let attempts = candidate.receipt["attempts"]
        .as_array()
        .ok_or_else(|| resume_error("GATE-RESUME-RECEIPT-SHAPE", "attempts"))?;
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| resume_error("GATE-RESUME-PLAN-SHAPE", "nodes"))?;
    for node in nodes
        .iter()
        .filter(|node| node["execution_cost_class"] == "HEAVY")
    {
        let node_id = string(node, "node_id")?;
        let previous = attempts
            .iter()
            .rev()
            .find(|item| item["node_id"] == node_id);
        let reason = reuse_reason(node, previous, &candidate.receipt, claims)?;
        if reason == "IMPORTED_CURRENT_PASS" {
            let attempt = previous.ok_or_else(|| {
                resume_error("GATE-RESUME-INTERNAL", "eligible attempt disappeared")
            })?;
            copy_outputs(
                node,
                &candidate.receipt,
                &candidate.artifact_root,
                current_root,
            )?;
            seed.attempts.push(attempt.clone());
            seed.final_results
                .insert(node_id.to_owned(), "PASS".to_owned());
            for item in node["expected_inventory"]["ids"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
            {
                seed.executed_inventory.insert(item.to_owned());
            }
        }
        seed.decisions.push(json!({
            "node_id": node_id,
            "prior_receipt_id": candidate.receipt["receipt_id"],
            "decision": if reason == "IMPORTED_CURRENT_PASS" {"IMPORTED"} else {"RERUN"},
            "reason_code": reason,
        }));
    }
    Ok(seed)
}

fn reuse_reason(
    node: &Value,
    attempt: Option<&Value>,
    receipt: &Value,
    claims: &ExecutionClaims,
) -> Result<&'static str> {
    let Some(attempt) = attempt else {
        return Ok("NO_PRIOR_NODE_RECEIPT");
    };
    if attempt["result"] != "PASS" {
        return Ok("PRIOR_NODE_NONPASS");
    }
    match string(node, "reuse_class")? {
        "NON_REUSABLE" => Ok("NON_REUSABLE_POLICY"),
        "HERMETIC_CONTENT" => Ok("IMPORTED_CURRENT_PASS"),
        "SAME_EXECUTION" => {
            let prior = &receipt["claims"];
            if prior["workflow"] != claims.workflow {
                Ok("SAME_EXECUTION_WORKFLOW_MISMATCH")
            } else if prior["job"] != claims.job {
                Ok("SAME_EXECUTION_JOB_MISMATCH")
            } else if prior["runner"] != claims.runner {
                Ok("SAME_EXECUTION_RUNNER_MISMATCH")
            } else if prior["attempt"] != claims.attempt {
                Ok("SAME_EXECUTION_ATTEMPT_MISMATCH")
            } else {
                Ok("IMPORTED_CURRENT_PASS")
            }
        }
        value => Err(resume_error("GATE-RESUME-CLASS", value)),
    }
}

fn copy_outputs(node: &Value, receipt: &Value, source: &Path, target: &Path) -> Result<()> {
    for relative in node["output_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        let source_path = source.join(relative);
        let target_path = target.join(relative);
        if target_path.exists() {
            return Err(resume_error("GATE-RESUME-OUTPUT-COLLISION", relative));
        }
        let bytes = fs::read(&source_path).map_err(|error| {
            resume_error("GATE-RESUME-ARTIFACT", format!("{relative}: {error}"))
        })?;
        let expected = receipt["artifacts"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|artifact| artifact["path"] == relative)
            .and_then(|artifact| artifact["sha256"].as_str())
            .ok_or_else(|| resume_error("GATE-RESUME-ARTIFACT-RECEIPT", relative))?;
        if crate::canonical::sha256_bytes(&bytes) != expected {
            return Err(resume_error("GATE-RESUME-ARTIFACT-DIGEST", relative));
        }
        let parent = target_path
            .parent()
            .ok_or_else(|| resume_error("GATE-RESUME-OUTPUT-PATH", relative))?;
        fs::create_dir_all(parent)
            .map_err(|error| resume_error("GATE-RESUME-MKDIR", error.to_string()))?;
        fs::write(&target_path, bytes)
            .map_err(|error| resume_error("GATE-RESUME-WRITE", error.to_string()))?;
    }
    Ok(())
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field]
        .as_str()
        .ok_or_else(|| resume_error("GATE-RESUME-SHAPE", field))
}

fn resume_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, code, message)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::reuse_reason;
    use crate::executor::ExecutionClaims;

    #[test]
    fn same_execution_rejects_runner_and_attempt_changes() {
        let node = json!({"reuse_class": "SAME_EXECUTION"});
        let attempt = json!({"result": "PASS"});
        let receipt = json!({"claims": {
            "workflow": "workflow", "job": "job", "runner": "runner", "attempt": 1
        }});
        let mut claims = ExecutionClaims {
            workflow: "workflow".to_owned(),
            job: "job".to_owned(),
            runner: "other".to_owned(),
            ..ExecutionClaims::default()
        };
        assert_eq!(
            reuse_reason(&node, Some(&attempt), &receipt, &claims).expect("runner decision"),
            "SAME_EXECUTION_RUNNER_MISMATCH"
        );
        claims.runner = "runner".to_owned();
        claims.attempt = 2;
        assert_eq!(
            reuse_reason(&node, Some(&attempt), &receipt, &claims).expect("attempt decision"),
            "SAME_EXECUTION_ATTEMPT_MISMATCH"
        );
    }

    #[test]
    fn non_reusable_pass_retains_exact_policy_reason() {
        let node = json!({"reuse_class": "NON_REUSABLE"});
        let attempt = json!({"result": "PASS"});
        let receipt = json!({"claims": {}});
        assert_eq!(
            reuse_reason(&node, Some(&attempt), &receipt, &ExecutionClaims::default())
                .expect("reuse decision"),
            "NON_REUSABLE_POLICY"
        );
    }
}
