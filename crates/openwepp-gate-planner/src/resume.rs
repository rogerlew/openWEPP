//! Verified cross-attempt import for current, target-reusable node receipts.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::canonical::{derived_id, digest, parse_strict, sha256_bytes};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::executor::ExecutionClaims;

pub struct ResumeCandidate {
    nodes: BTreeMap<String, CheckpointEvidence>,
}

struct CheckpointEvidence {
    checkpoint: Value,
    artifact_root: PathBuf,
    claims: Value,
}

pub struct ResumeSeed {
    pub attempts: Vec<Value>,
    pub final_results: BTreeMap<String, String>,
    pub executed_inventory: BTreeSet<String>,
    pub decisions: Vec<Value>,
}

/// Find and independently verify current per-node checkpoints named by the
/// append-only attempt ledger, including attempts without an aggregate receipt.
///
/// # Errors
///
/// Returns a typed error for malformed ledger records or substituted checkpoint
/// artifacts. Plan identity may change across documentation-only edits; node
/// identity and every output digest may not.
pub fn load_candidate(repo: &Path, plan: &Value, ledger: &Path) -> Result<Option<ResumeCandidate>> {
    let _ = repo;
    let text = fs::read_to_string(ledger)
        .map_err(|error| resume_error("GATE-RESUME-LEDGER", error.to_string()))?;
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| resume_error("GATE-RESUME-PLAN-SHAPE", "nodes"))?;
    let mut admitted = BTreeMap::new();
    for line in text.lines().rev().filter(|line| !line.trim().is_empty()) {
        let item = parse_strict(line.as_bytes())?;
        if !matches!(
            item["record_type"].as_str(),
            Some("ATTEMPT" | "STAGE_ATTEMPT")
        ) {
            continue;
        }
        let Some(root) = item["artifact_root"].as_str().map(PathBuf::from) else {
            continue;
        };
        let artifact_root = if root.join(".checkpoints").is_dir() {
            root.clone()
        } else {
            root.join("execution")
        };
        if !artifact_root.is_dir() {
            continue;
        }
        let claims = prior_claims(&item, &root)?;
        for node in nodes
            .iter()
            .filter(|node| node["execution_cost_class"] == "HEAVY")
        {
            let node_id = string(node, "node_id")?;
            if admitted.contains_key(node_id) {
                continue;
            }
            let checkpoint_path = artifact_root
                .join(".checkpoints")
                .join(format!("{node_id}.json"));
            if !checkpoint_path.is_file() {
                continue;
            }
            let checkpoint = parse_strict(
                &fs::read(&checkpoint_path)
                    .map_err(|error| resume_error("GATE-RESUME-CHECKPOINT", error.to_string()))?,
            )?;
            verify_checkpoint(node, &checkpoint, &artifact_root)?;
            admitted.insert(
                node_id.to_owned(),
                CheckpointEvidence {
                    checkpoint,
                    artifact_root: artifact_root.clone(),
                    claims: claims.clone(),
                },
            );
        }
    }
    Ok((!admitted.is_empty()).then_some(ResumeCandidate { nodes: admitted }))
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
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| resume_error("GATE-RESUME-PLAN-SHAPE", "nodes"))?;
    for node in nodes
        .iter()
        .filter(|node| node["execution_cost_class"] == "HEAVY")
    {
        let node_id = string(node, "node_id")?;
        let evidence = candidate.nodes.get(node_id);
        let previous = evidence.map(|item| &item.checkpoint["attempt"]);
        let prior_claims = evidence.map_or(&Value::Null, |item| &item.claims);
        let reason = reuse_reason(node, previous, prior_claims, claims)?;
        if reason == "IMPORTED_CURRENT_PASS" {
            let evidence = evidence.ok_or_else(|| {
                resume_error("GATE-RESUME-INTERNAL", "eligible attempt disappeared")
            })?;
            copy_outputs(
                node,
                &evidence.checkpoint,
                &evidence.artifact_root,
                current_root,
            )?;
            seed.attempts.push(evidence.checkpoint["attempt"].clone());
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
            "prior_receipt_id": evidence.map_or(Value::Null, |item| item.checkpoint["checkpoint_id"].clone()),
            "decision": if reason == "IMPORTED_CURRENT_PASS" {"IMPORTED"} else {"RERUN"},
            "reason_code": reason,
        }));
    }
    Ok(seed)
}

fn reuse_reason(
    node: &Value,
    attempt: Option<&Value>,
    prior_claims: &Value,
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
            if prior_claims["workflow"] != claims.workflow {
                Ok("SAME_EXECUTION_WORKFLOW_MISMATCH")
            } else if prior_claims["job"] != claims.job {
                Ok("SAME_EXECUTION_JOB_MISMATCH")
            } else if prior_claims["runner"] != claims.runner {
                Ok("SAME_EXECUTION_RUNNER_MISMATCH")
            } else if prior_claims["attempt"] != claims.attempt {
                Ok("SAME_EXECUTION_ATTEMPT_MISMATCH")
            } else {
                Ok("IMPORTED_CURRENT_PASS")
            }
        }
        value => Err(resume_error("GATE-RESUME-CLASS", value)),
    }
}

fn copy_outputs(node: &Value, checkpoint: &Value, source: &Path, target: &Path) -> Result<()> {
    let source_root = source
        .canonicalize()
        .map_err(|error| resume_error("GATE-RESUME-SOURCE-ROOT", error.to_string()))?;
    let target_root = target
        .canonicalize()
        .map_err(|error| resume_error("GATE-RESUME-TARGET-ROOT", error.to_string()))?;
    for relative in node["output_paths"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        let confined = Path::new(relative);
        if confined.is_absolute()
            || confined
                .components()
                .any(|part| !matches!(part, std::path::Component::Normal(_)))
        {
            return Err(resume_error("GATE-RESUME-OUTPUT-PATH", relative));
        }
        let source_path = source_root.join(confined);
        let target_path = target_root.join(confined);
        if target_path.exists() {
            return Err(resume_error("GATE-RESUME-OUTPUT-COLLISION", relative));
        }
        let canonical_source = source_path
            .canonicalize()
            .map_err(|error| resume_error("GATE-RESUME-ARTIFACT", error.to_string()))?;
        if !canonical_source.starts_with(&source_root) || !canonical_source.is_file() {
            return Err(resume_error("GATE-RESUME-SOURCE-ESCAPE", relative));
        }
        let bytes = fs::read(&canonical_source).map_err(|error| {
            resume_error("GATE-RESUME-ARTIFACT", format!("{relative}: {error}"))
        })?;
        let expected = checkpoint["artifacts"]
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
        let canonical_parent = parent
            .canonicalize()
            .map_err(|error| resume_error("GATE-RESUME-OUTPUT-PATH", error.to_string()))?;
        if !canonical_parent.starts_with(&target_root) {
            return Err(resume_error("GATE-RESUME-OUTPUT-ESCAPE", relative));
        }
        fs::write(&target_path, bytes)
            .map_err(|error| resume_error("GATE-RESUME-WRITE", error.to_string()))?;
    }
    Ok(())
}

fn verify_checkpoint(node: &Value, checkpoint: &Value, root: &Path) -> Result<()> {
    if checkpoint["schema_version"] != "openwepp-gate-node-checkpoint-v1"
        || checkpoint["checkpoint_id"] != derived_id(checkpoint, "checkpoint_id")?
        || checkpoint["node_id"] != node["node_id"]
        || checkpoint["node_sha256"] != digest(node)?
        || checkpoint["result"] != "PASS"
    {
        return Err(resume_error(
            "GATE-RESUME-CHECKPOINT-IDENTITY",
            string(node, "node_id")?,
        ));
    }
    for artifact in checkpoint["artifacts"]
        .as_array()
        .ok_or_else(|| resume_error("GATE-RESUME-CHECKPOINT-SHAPE", "artifacts"))?
    {
        let relative = string(artifact, "path")?;
        let bytes = fs::read(root.join(relative))
            .map_err(|error| resume_error("GATE-RESUME-ARTIFACT", error.to_string()))?;
        if artifact["sha256"] != sha256_bytes(&bytes) {
            return Err(resume_error("GATE-RESUME-ARTIFACT-DIGEST", relative));
        }
    }
    Ok(())
}

fn prior_claims(item: &Value, root: &Path) -> Result<Value> {
    if item["workflow"].is_string() {
        return Ok(json!({
            "workflow": item["workflow"],
            "job": item["job"],
            "runner": item["runner"],
            "attempt": item["attempt"],
        }));
    }
    let receipt_path = root.join("receipt.json");
    if receipt_path.is_file() {
        let receipt = parse_strict(
            &fs::read(receipt_path)
                .map_err(|error| resume_error("GATE-RESUME-RECEIPT", error.to_string()))?,
        )?;
        return Ok(receipt["claims"].clone());
    }
    Ok(Value::Null)
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
    use std::fs;

    use serde_json::json;

    use super::{apply_candidate, load_candidate, reuse_reason};
    use crate::canonical::{canonical_bytes, derived_id, digest, sha256_bytes};
    use crate::executor::ExecutionClaims;

    #[test]
    fn same_execution_rejects_runner_and_attempt_changes() {
        let node = json!({"reuse_class": "SAME_EXECUTION"});
        let attempt = json!({"result": "PASS"});
        let prior_claims = json!({
            "workflow": "workflow", "job": "job", "runner": "runner", "attempt": 1
        });
        let mut claims = ExecutionClaims {
            workflow: "workflow".to_owned(),
            job: "job".to_owned(),
            runner: "other".to_owned(),
            ..ExecutionClaims::default()
        };
        assert_eq!(
            reuse_reason(&node, Some(&attempt), &prior_claims, &claims).expect("runner decision"),
            "SAME_EXECUTION_RUNNER_MISMATCH"
        );
        claims.runner = "runner".to_owned();
        claims.attempt = 2;
        assert_eq!(
            reuse_reason(&node, Some(&attempt), &prior_claims, &claims).expect("attempt decision"),
            "SAME_EXECUTION_ATTEMPT_MISMATCH"
        );
    }

    #[test]
    fn non_reusable_pass_retains_exact_policy_reason() {
        let node = json!({"reuse_class": "NON_REUSABLE"});
        let attempt = json!({"result": "PASS"});
        let prior_claims = json!({});
        assert_eq!(
            reuse_reason(
                &node,
                Some(&attempt),
                &prior_claims,
                &ExecutionClaims::default()
            )
            .expect("reuse decision"),
            "NON_REUSABLE_POLICY"
        );
    }

    #[test]
    fn pre_receipt_checkpoint_is_imported_across_plan_identity_change() {
        let root = std::env::temp_dir().join(format!(
            "openwepp-resume-{}-pre-receipt",
            std::process::id()
        ));
        let prior = root.join("prior/execution");
        let current = root.join("current");
        fs::create_dir_all(prior.join(".checkpoints")).expect("checkpoint directory");
        fs::create_dir_all(prior.join("target/heavy")).expect("prior output directory");
        fs::create_dir_all(&current).expect("current output directory");
        let output = b"verified output\n";
        fs::write(prior.join("target/heavy/result.json"), output).expect("prior output");
        let mut node = json!({
            "node_id": "1".repeat(64),
            "execution_cost_class": "HEAVY",
            "reuse_class": "HERMETIC_CONTENT",
            "output_paths": ["target/heavy/result.json"],
            "expected_inventory": {"ids": ["case"]},
        });
        let node_sha = digest(&node).expect("node digest");
        let mut checkpoint = json!({
            "schema_version": "openwepp-gate-node-checkpoint-v1",
            "checkpoint_id": "0".repeat(64),
            "node_id": node["node_id"],
            "node_sha256": node_sha,
            "result": "PASS",
            "reuse_class": "HERMETIC_CONTENT",
            "attempt": {"node_id": node["node_id"], "result": "PASS"},
            "artifacts": [{"path": "target/heavy/result.json", "sha256": sha256_bytes(output)}],
        });
        checkpoint["checkpoint_id"] = derived_id(&checkpoint, "checkpoint_id")
            .expect("checkpoint identity")
            .into();
        fs::write(
            prior.join(".checkpoints").join(format!(
                "{}.json",
                node["node_id"].as_str().expect("node id")
            )),
            canonical_bytes(&checkpoint).expect("checkpoint bytes"),
        )
        .expect("checkpoint");
        let ledger = root.join("attempts.jsonl");
        fs::write(
            &ledger,
            format!(
                "{{\"artifact_root\":\"{}\",\"plan_id\":\"different-plan\",\"record_type\":\"STAGE_ATTEMPT\",\"status\":\"FAILED\"}}\n",
                root.join("prior").display()
            ),
        )
        .expect("ledger");
        let plan = json!({"nodes": [node.take()]});
        let candidate = load_candidate(&root, &plan, &ledger)
            .expect("load candidate")
            .expect("candidate exists");
        let seed = apply_candidate(
            &plan,
            &current,
            &ExecutionClaims::default(),
            Some(&candidate),
        )
        .expect("apply checkpoint");
        assert_eq!(seed.decisions[0]["decision"], "IMPORTED");
        assert_eq!(
            fs::read(current.join("target/heavy/result.json")).expect("imported output"),
            output
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }
}
