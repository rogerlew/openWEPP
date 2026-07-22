//! Verified cross-attempt import for current, target-reusable node receipts.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{Value, json};

use crate::artifact_contract::create_confined_directories;
use crate::canonical::{derived_id, digest, parse_strict, sha256_bytes};
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::executor::ExecutionClaims;
use crate::planner::verify_plan_identity;
use crate::pre_heavy::ConstructedAudit;
use crate::verifier::{DirectoryArtifacts, verify_receipt, verify_receipt_after_ready_audit};

pub struct ResumeCandidate {
    nodes: BTreeMap<String, CheckpointEvidence>,
}

struct CheckpointEvidence {
    checkpoint: Value,
    artifact_root: PathBuf,
    claims: Value,
    receipt_id: Value,
    provenance_id: Value,
}

struct RecoveryRoot {
    path: PathBuf,
    explicit: bool,
}

struct RecoveryArchive {
    root: PathBuf,
    artifact_root: PathBuf,
    provenance: Value,
}

struct RecoveryEnvelope {
    prior_plan: Value,
    accepted_receipt: Option<Value>,
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
pub fn load_candidate(
    repo: &Path,
    plan: &Value,
    ledger: &Path,
    claims: &ExecutionClaims,
) -> Result<Option<ResumeCandidate>> {
    load_candidate_internal(repo, plan, ledger, claims, false)
}

/// Load recovery evidence after the current plan was independently admitted by
/// an in-process READY audit.
///
/// # Errors
///
/// Returns the same typed recovery errors as [`load_candidate`].
pub fn load_candidate_after_ready_audit(
    repo: &Path,
    plan: &Value,
    ledger: &Path,
    claims: &ExecutionClaims,
    audit: &ConstructedAudit,
) -> Result<Option<ResumeCandidate>> {
    if audit.as_value()["status"] != "READY"
        || audit.as_value()["plan_id"] != plan["plan_id"]
        || audit.as_value()["plan_sha256"] != digest(plan)?
    {
        return Err(resume_error(
            "GATE-RESUME-AUDIT-BINDING",
            "recovery fast path requires the current constructed READY audit",
        ));
    }
    load_candidate_internal(repo, plan, ledger, claims, true)
}

fn load_candidate_internal(
    repo: &Path,
    plan: &Value,
    ledger: &Path,
    claims: &ExecutionClaims,
    current_plan_admitted: bool,
) -> Result<Option<ResumeCandidate>> {
    let text = fs::read_to_string(ledger)
        .map_err(|error| resume_error("GATE-RESUME-LEDGER", error.to_string()))?;
    let records = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_strict(line.as_bytes()))
        .collect::<Result<Vec<_>>>()?;
    let nodes = plan["nodes"]
        .as_array()
        .ok_or_else(|| resume_error("GATE-RESUME-PLAN-SHAPE", "nodes"))?;
    let mut admitted = BTreeMap::new();
    let mut seen_roots = BTreeSet::new();
    for item in records.iter().rev() {
        let Some(recovery_root) = recovery_root_from_record(item, ledger)? else {
            continue;
        };
        if seen_roots.contains(&recovery_root.path) {
            continue;
        }
        let Some(archive) = inspect_recovery_archive(ledger, claims, recovery_root)? else {
            continue;
        };
        seen_roots.insert(archive.root.clone());
        let envelope = load_recovery_envelope(repo, plan, &archive, current_plan_admitted)?;
        admit_archive_nodes(plan, nodes, &archive, &envelope, &mut admitted)?;
    }
    Ok((!admitted.is_empty()).then_some(ResumeCandidate { nodes: admitted }))
}

fn recovery_root_from_record(item: &Value, ledger: &Path) -> Result<Option<RecoveryRoot>> {
    if !matches!(
        item["record_type"].as_str(),
        Some("ATTEMPT" | "STAGE_ATTEMPT")
    ) {
        return Ok(None);
    }
    let explicit = match item.get("recovery_root") {
        None | Some(Value::Null) => None,
        Some(value) => Some(value.as_str().ok_or_else(|| {
            resume_error(
                "GATE-RESUME-PROVENANCE-PATH",
                "recovery_root must be a path",
            )
        })?),
    };
    let Some(path) = explicit
        .or_else(|| item["artifact_root"].as_str())
        .map(PathBuf::from)
    else {
        return Ok(None);
    };
    let recovery_parent = ledger
        .parent()
        .ok_or_else(|| resume_error("GATE-RESUME-PROVENANCE-PATH", ledger.display().to_string()))?
        .join("recovery");
    if path.parent() != Some(recovery_parent.as_path()) {
        if explicit.is_some() {
            return Err(resume_error(
                "GATE-RESUME-PROVENANCE-PATH",
                path.display().to_string(),
            ));
        }
        return Ok(None);
    }
    Ok(Some(RecoveryRoot {
        path,
        explicit: explicit.is_some(),
    }))
}

fn inspect_recovery_archive(
    ledger: &Path,
    claims: &ExecutionClaims,
    recovery: RecoveryRoot,
) -> Result<Option<RecoveryArchive>> {
    let artifact_root = if recovery.path.join(".checkpoints").is_dir() {
        recovery.path.clone()
    } else {
        recovery.path.join("execution")
    };
    if !artifact_root.is_dir() {
        if recovery.explicit {
            return Err(resume_error(
                "GATE-RESUME-PROVENANCE-PATH",
                recovery.path.display().to_string(),
            ));
        }
        return Ok(None);
    }
    let provenance = match verify_archive_provenance(ledger, &recovery.path, claims) {
        Ok(provenance) => provenance,
        Err(error) if error.code == "GATE-RESUME-PROVENANCE-MISSING" && !recovery.explicit => {
            return Ok(None);
        }
        Err(error) => return Err(error),
    };
    Ok(Some(RecoveryArchive {
        root: recovery.path,
        artifact_root,
        provenance,
    }))
}

fn load_recovery_envelope(
    repo: &Path,
    plan: &Value,
    archive: &RecoveryArchive,
    current_plan_admitted: bool,
) -> Result<RecoveryEnvelope> {
    let prior_plan_path = archive.root.join("plan.json");
    if !prior_plan_path.is_file() {
        return Err(resume_error(
            "GATE-RESUME-PROVENANCE-PLAN-MISSING",
            archive.root.display().to_string(),
        ));
    }
    let prior_plan = parse_strict(
        &fs::read(&prior_plan_path)
            .map_err(|error| resume_error("GATE-RESUME-PLAN", error.to_string()))?,
    )?;
    verify_plan_identity(&prior_plan)?;
    let receipt_path = archive.root.join("receipt.json");
    let accepted_receipt = if receipt_path.is_file() {
        Some(load_accepted_receipt(
            repo,
            plan,
            archive,
            &prior_plan,
            &receipt_path,
            current_plan_admitted,
        )?)
    } else {
        None
    };
    Ok(RecoveryEnvelope {
        prior_plan,
        accepted_receipt,
    })
}

fn load_accepted_receipt(
    repo: &Path,
    plan: &Value,
    archive: &RecoveryArchive,
    prior_plan: &Value,
    receipt_path: &Path,
    current_plan_admitted: bool,
) -> Result<Value> {
    let receipt = parse_strict(
        &fs::read(receipt_path)
            .map_err(|error| resume_error("GATE-RESUME-RECEIPT", error.to_string()))?,
    )?;
    let artifacts = DirectoryArtifacts::new(archive.artifact_root.clone());
    let verification = if current_plan_admitted && digest(prior_plan)? == digest(plan)? {
        verify_receipt_after_ready_audit(repo, prior_plan, &receipt, &artifacts)
    } else {
        verify_receipt(repo, prior_plan, &receipt, &artifacts)
    };
    verification.map_err(|error| {
        resume_error(
            "GATE-RESUME-RECEIPT-INVALID",
            format!("{}: {}", error.code, error.message),
        )
    })?;
    verify_provenance_claims(
        &receipt["claims"],
        &archive.provenance,
        "GATE-RESUME-PROVENANCE-RECEIPT-BINDING",
        &archive.root.display().to_string(),
    )?;
    Ok(receipt)
}

fn verify_provenance_claims(
    claims: &Value,
    provenance: &Value,
    code: &'static str,
    message: &str,
) -> Result<()> {
    if claims["workflow"] != provenance["workflow"]
        || claims["job"] != "openwepp/execute-increment"
        || claims["attempt"].as_u64()
            != provenance["run_attempt"]
                .as_str()
                .and_then(|value| value.parse().ok())
    {
        return Err(resume_error(code, message));
    }
    Ok(())
}

fn admit_archive_nodes(
    plan: &Value,
    nodes: &[Value],
    archive: &RecoveryArchive,
    recovery: &RecoveryEnvelope,
    admitted: &mut BTreeMap<String, CheckpointEvidence>,
) -> Result<()> {
    for node in nodes
        .iter()
        .filter(|node| node["execution_cost_class"] == "HEAVY")
    {
        let node_id = string(node, "node_id")?;
        if admitted.contains_key(node_id) {
            continue;
        }
        let checkpoint_path = archive
            .artifact_root
            .join(".checkpoints")
            .join(format!("{node_id}.json"));
        if !checkpoint_path.is_file() {
            continue;
        }
        let checkpoint = parse_strict(
            &fs::read(&checkpoint_path)
                .map_err(|error| resume_error("GATE-RESUME-CHECKPOINT", error.to_string()))?,
        )?;
        admit_checkpoint(plan, node, checkpoint, archive, recovery, admitted)?;
    }
    Ok(())
}

fn admit_checkpoint(
    plan: &Value,
    node: &Value,
    checkpoint: Value,
    archive: &RecoveryArchive,
    recovery: &RecoveryEnvelope,
    admitted: &mut BTreeMap<String, CheckpointEvidence>,
) -> Result<()> {
    let node_id = string(node, "node_id")?;
    verify_checkpoint_prior_plan(&checkpoint, node, &recovery.prior_plan)?;
    let envelope = recovery.accepted_receipt.clone().unwrap_or_else(|| {
        json!({
            "claims": checkpoint["execution_binding"]["claims"],
            "attempts": [checkpoint["attempt"].clone()],
            "artifacts": checkpoint["artifacts"].clone(),
        })
    });
    if recovery.accepted_receipt.is_none() {
        verify_provenance_claims(
            &checkpoint["execution_binding"]["claims"],
            &archive.provenance,
            "GATE-RESUME-PROVENANCE-CHECKPOINT-BINDING",
            node_id,
        )?;
    }
    verify_checkpoint(plan, node, &checkpoint, &envelope, &archive.artifact_root)?;
    admitted.insert(
        node_id.to_owned(),
        CheckpointEvidence {
            checkpoint,
            artifact_root: archive.artifact_root.clone(),
            claims: envelope["claims"].clone(),
            receipt_id: recovery
                .accepted_receipt
                .as_ref()
                .map_or(Value::Null, |receipt| receipt["receipt_id"].clone()),
            provenance_id: archive.provenance["index_sha256"].clone(),
        },
    );
    Ok(())
}

fn verify_archive_provenance(
    ledger: &Path,
    root: &Path,
    claims: &ExecutionClaims,
) -> Result<Value> {
    let name = root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| resume_error("GATE-RESUME-PROVENANCE-PATH", root.display().to_string()))?;
    let history = ledger
        .parent()
        .ok_or_else(|| resume_error("GATE-RESUME-PROVENANCE-PATH", ledger.display().to_string()))?;
    let recovery_parent = history.join("recovery");
    if root.parent() != Some(recovery_parent.as_path())
        || fs::symlink_metadata(root)
            .map(|metadata| metadata.file_type().is_symlink() || !metadata.is_dir())
            .unwrap_or(true)
    {
        return Err(resume_error(
            "GATE-RESUME-PROVENANCE-PATH",
            root.display().to_string(),
        ));
    }
    let provenance = history.join("provenance").join(name);
    let index_path = provenance.join("attempt-index.json");
    let predicate_path = provenance.join("recovery-predicate.json");
    let bundle_path = provenance.join("recovery-attestation.jsonl");
    if [&index_path, &predicate_path, &bundle_path]
        .iter()
        .any(|path| {
            fs::symlink_metadata(path)
                .map(|metadata| metadata.file_type().is_symlink() || !metadata.is_file())
                .unwrap_or(true)
        })
    {
        return Err(resume_error(
            "GATE-RESUME-PROVENANCE-MISSING",
            root.display().to_string(),
        ));
    }
    let index_bytes = fs::read(&index_path)
        .map_err(|error| resume_error("GATE-RESUME-PROVENANCE-READ", error.to_string()))?;
    let index = parse_strict(&index_bytes)?;
    let predicate = parse_strict(
        &fs::read(&predicate_path)
            .map_err(|error| resume_error("GATE-RESUME-PROVENANCE-READ", error.to_string()))?,
    )?;
    let repository = &claims.repository;
    if predicate["schema_version"] != "openwepp-testgate-recovery-provenance-v1"
        || predicate["index_sha256"] != sha256_bytes(&index_bytes)
        || predicate["run_id"].as_str().is_none()
        || predicate["run_attempt"].as_str().is_none()
        || index["provenance"]["repository"] != predicate["repository"]
        || index["provenance"]["workflow"] != predicate["workflow"]
        || index["provenance"]["run_id"] != predicate["run_id"]
        || index["provenance"]["run_attempt"] != predicate["run_attempt"]
        || index["provenance"]["head_sha"] != predicate["head_sha"]
        || predicate["repository"] != *repository
        || predicate["workflow"] != claims.workflow
        || predicate["source_ref"] != claims.source_ref
    {
        return Err(resume_error(
            "GATE-RESUME-PROVENANCE-IDENTITY",
            root.display().to_string(),
        ));
    }
    verify_indexed_recovery_root(&index, root, name)?;
    verify_native_attestation(&index_path, &bundle_path, &predicate, repository)?;
    Ok(predicate)
}

fn verify_native_attestation(
    index_path: &Path,
    bundle_path: &Path,
    predicate: &Value,
    repository: &str,
) -> Result<()> {
    #[cfg(test)]
    if fs::read(bundle_path).ok().as_deref() == Some(b"TEST-VALID-BUNDLE") {
        return Ok(());
    }
    let output = run_native_attestation(index_path, bundle_path, predicate, repository)?;
    verify_native_attestation_output(&output)
}

fn run_native_attestation(
    index_path: &Path,
    bundle_path: &Path,
    predicate: &Value,
    repository: &str,
) -> Result<Output> {
    let index_argument = index_path.to_string_lossy().into_owned();
    let bundle_argument = bundle_path.to_string_lossy().into_owned();
    let signer = format!("{repository}/.github/workflows/testgate-shadow.yml");
    Command::new("gh")
        .args([
            "attestation",
            "verify",
            &index_argument,
            "--repo",
            repository,
            "--signer-workflow",
            &signer,
            "--source-ref",
            string(predicate, "source_ref")?,
            "--source-digest",
            string(predicate, "head_sha")?,
            "--predicate-type",
            "https://openwepp.org/attestations/testgate-recovery/v1",
            "--deny-self-hosted-runners",
            "--bundle",
            &bundle_argument,
            "--format",
            "json",
        ])
        .output()
        .map_err(|error| resume_error("GATE-RESUME-PROVENANCE-VERIFY", error.to_string()))
}

fn verify_native_attestation_output(output: &Output) -> Result<()> {
    let verified = parse_strict(&output.stdout).map_err(|error| {
        resume_error(
            "GATE-RESUME-PROVENANCE-VERIFY",
            format!(
                "{}: {}",
                error.code,
                String::from_utf8_lossy(&output.stderr)
            ),
        )
    })?;
    if !output.status.success() || verified.as_array().is_none_or(Vec::is_empty) {
        return Err(resume_error(
            "GATE-RESUME-PROVENANCE-VERIFY",
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    Ok(())
}

fn verify_indexed_recovery_root(index: &Value, root: &Path, name: &str) -> Result<()> {
    let prefix = format!("recovery/{name}/");
    let indexed = index["files"]
        .as_array()
        .ok_or_else(|| resume_error("GATE-RESUME-PROVENANCE-INDEX", "files"))?
        .iter()
        .filter_map(|item| {
            item["path"]
                .as_str()
                .and_then(|path| path.strip_prefix(&prefix))
                .map(|path| (path.to_owned(), item["sha256"].clone()))
        })
        .collect::<BTreeMap<_, _>>();
    let mut actual = BTreeMap::new();
    collect_regular_files(root, root, &mut actual)?;
    if actual.len() != indexed.len()
        || actual.iter().any(|(path, bytes)| {
            indexed.get(path).and_then(Value::as_str) != Some(sha256_bytes(bytes).as_str())
        })
    {
        return Err(resume_error(
            "GATE-RESUME-PROVENANCE-FILESET",
            root.display().to_string(),
        ));
    }
    Ok(())
}

fn collect_regular_files(
    root: &Path,
    directory: &Path,
    files: &mut BTreeMap<String, Vec<u8>>,
) -> Result<()> {
    for entry in fs::read_dir(directory)
        .map_err(|error| resume_error("GATE-RESUME-PROVENANCE-READ", error.to_string()))?
    {
        let path = entry
            .map_err(|error| resume_error("GATE-RESUME-PROVENANCE-READ", error.to_string()))?
            .path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| resume_error("GATE-RESUME-PROVENANCE-READ", error.to_string()))?;
        if metadata.file_type().is_symlink() {
            return Err(resume_error(
                "GATE-RESUME-PROVENANCE-SYMLINK",
                path.display().to_string(),
            ));
        }
        if metadata.is_dir() {
            collect_regular_files(root, &path, files)?;
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|error| resume_error("GATE-RESUME-PROVENANCE-PATH", error.to_string()))?
                .to_string_lossy()
                .replace('\\', "/");
            files.insert(
                relative,
                fs::read(&path).map_err(|error| {
                    resume_error("GATE-RESUME-PROVENANCE-READ", error.to_string())
                })?,
            );
        } else {
            return Err(resume_error(
                "GATE-RESUME-PROVENANCE-FILETYPE",
                path.display().to_string(),
            ));
        }
    }
    Ok(())
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
            let mut imported = evidence.checkpoint.clone();
            imported["execution_binding"]["plan_id"] = plan["plan_id"].clone();
            imported["execution_binding"]["execution_key"] = plan["execution_key"].clone();
            imported["execution_binding"]["roots"] = plan["environment_roots"].clone();
            imported["execution_binding"]["execution_context"] = plan["execution_context"].clone();
            imported["execution_binding"]["policy"] = plan["policy"].clone();
            imported["execution_binding"]["claims"] = json!({
                "workflow": claims.workflow,
                "job": claims.job,
                "runner": claims.runner,
                "attempt": claims.attempt,
            });
            imported["checkpoint_id"] = Value::String(derived_id(&imported, "checkpoint_id")?);
            crate::checkpoint_mirror::mirror_node_checkpoint(current_root, node, &imported)?;
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
            "prior_receipt_id": evidence.map_or(Value::Null, |item| item.receipt_id.clone()),
            "prior_provenance_id": evidence.map_or(Value::Null, |item| item.provenance_id.clone()),
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
        if fs::symlink_metadata(&target_path).is_ok() {
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
        create_confined_directories(&target_root, parent).map_err(|error| {
            resume_error(error.code, format!("{}: {}", error.code, error.message))
        })?;
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

fn verify_checkpoint(
    plan: &Value,
    node: &Value,
    checkpoint: &Value,
    receipt: &Value,
    root: &Path,
) -> Result<()> {
    verify_checkpoint_identity(node, checkpoint)?;
    verify_checkpoint_roots(plan, node, checkpoint)?;
    verify_checkpoint_execution(plan, node, checkpoint)?;
    verify_checkpoint_receipt(node, checkpoint, receipt)?;
    verify_checkpoint_artifacts(checkpoint, receipt, root)
}

fn verify_checkpoint_identity(node: &Value, checkpoint: &Value) -> Result<()> {
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
    Ok(())
}

fn verify_checkpoint_roots(plan: &Value, node: &Value, checkpoint: &Value) -> Result<()> {
    let binding = &checkpoint["execution_binding"];
    let prior_roots = &binding["roots"];
    let current_roots = &plan["environment_roots"];
    if binding["boundary"] != plan["boundary"]
        || binding["execution_context"] != plan["execution_context"]
        || binding["policy"] != plan["policy"]
        || prior_roots["execution_root"] != current_roots["execution_root"]
        || prior_roots["authority_root"] != current_roots["authority_root"]
        || prior_roots["assurance_root"] != current_roots["assurance_root"]
    {
        return Err(resume_error(
            "GATE-RESUME-CHECKPOINT-ROOT-DRIFT",
            string(node, "node_id")?,
        ));
    }
    Ok(())
}

fn verify_checkpoint_execution(plan: &Value, node: &Value, checkpoint: &Value) -> Result<()> {
    let binding = &checkpoint["execution_binding"];
    if node["reuse_class"] == "SAME_EXECUTION"
        && (binding["plan_id"] != plan["plan_id"]
            || binding["execution_key"] != plan["execution_key"])
    {
        return Err(resume_error(
            "GATE-RESUME-CHECKPOINT-EXECUTION-DRIFT",
            string(node, "node_id")?,
        ));
    }
    Ok(())
}

fn verify_checkpoint_receipt(node: &Value, checkpoint: &Value, receipt: &Value) -> Result<()> {
    let binding = &checkpoint["execution_binding"];
    if !checkpoint_claims_match(&binding["claims"], &receipt["claims"])
        || !receipt["attempts"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|attempt| attempt == &checkpoint["attempt"])
    {
        return Err(resume_error(
            "GATE-RESUME-CHECKPOINT-RECEIPT-MISMATCH",
            string(node, "node_id")?,
        ));
    }
    Ok(())
}

fn verify_checkpoint_artifacts(checkpoint: &Value, receipt: &Value, root: &Path) -> Result<()> {
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
        if !receipt["artifacts"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|item| item["path"] == artifact["path"] && item["sha256"] == artifact["sha256"])
        {
            return Err(resume_error(
                "GATE-RESUME-CHECKPOINT-RECEIPT-MISMATCH",
                relative,
            ));
        }
    }
    Ok(())
}

fn verify_checkpoint_prior_plan(
    checkpoint: &Value,
    current_node: &Value,
    prior_plan: &Value,
) -> Result<()> {
    let node_id = string(current_node, "node_id")?;
    let prior_node = prior_plan["nodes"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|node| node["node_id"] == node_id)
        .ok_or_else(|| resume_error("GATE-RESUME-PRIOR-PLAN-NODE", node_id))?;
    let binding = &checkpoint["execution_binding"];
    if prior_node != current_node
        || checkpoint["node_sha256"] != digest(prior_node)?
        || binding["plan_id"] != prior_plan["plan_id"]
        || binding["execution_key"] != prior_plan["execution_key"]
        || binding["boundary"] != prior_plan["boundary"]
        || binding["roots"] != prior_plan["environment_roots"]
        || binding["execution_context"] != prior_plan["execution_context"]
        || binding["policy"] != prior_plan["policy"]
    {
        return Err(resume_error("GATE-RESUME-PRIOR-PLAN-BINDING", node_id));
    }
    Ok(())
}

fn checkpoint_claims_match(checkpoint: &Value, receipt: &Value) -> bool {
    ["workflow", "job", "runner", "attempt"]
        .into_iter()
        .all(|field| checkpoint[field] == receipt[field])
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
#[path = "resume_coverage_tests.rs"]
mod coverage_tests;

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::{
        apply_candidate, copy_outputs, load_candidate, reuse_reason, verify_checkpoint,
        verify_indexed_recovery_root,
    };
    use crate::canonical::{canonical_bytes, derived_id, digest, sha256_bytes};
    use crate::executor::ExecutionClaims;

    #[test]
    fn ordinary_current_attempt_artifact_root_is_not_a_recovery_claim() {
        let scratch = std::env::temp_dir().join(format!(
            "openwepp-resume-current-attempt-{}",
            std::process::id()
        ));
        let history = scratch.join("history");
        fs::create_dir_all(&history).expect("history");
        let ledger = history.join("attempts.jsonl");
        fs::write(
            &ledger,
            format!(
                "{}\n",
                serde_json::to_string(&json!({
                    "record_type": "STAGE_ATTEMPT",
                    "status": "STARTED",
                    "stage": "HEAVY",
                    "artifact_root": scratch.join("current-execution")
                }))
                .expect("record")
            ),
        )
        .expect("ledger");
        let candidate = load_candidate(
            &scratch,
            &json!({"nodes": []}),
            &ledger,
            &ExecutionClaims::default(),
        )
        .expect("ordinary artifact record is ignored");
        assert!(candidate.is_none());

        fs::write(
            &ledger,
            format!(
                "{}\n",
                serde_json::to_string(&json!({
                    "record_type": "STAGE_ATTEMPT",
                    "status": "CLOSED",
                    "stage": "HEAVY",
                    "recovery_root": scratch.join("forged-recovery")
                }))
                .expect("record")
            ),
        )
        .expect("ledger");
        let error = load_candidate(
            &scratch,
            &json!({"nodes": []}),
            &ledger,
            &ExecutionClaims::default(),
        )
        .err()
        .expect("explicit recovery roots remain strict");
        assert_eq!(error.code, "GATE-RESUME-PROVENANCE-PATH");

        let missing = history.join("recovery/missing");
        fs::create_dir_all(missing.parent().expect("recovery parent")).expect("recovery namespace");
        fs::write(
            &ledger,
            format!(
                "{}\n",
                serde_json::to_string(&json!({
                    "record_type": "STAGE_ATTEMPT",
                    "status": "CLOSED",
                    "stage": "HEAVY",
                    "recovery_root": missing
                }))
                .expect("record")
            ),
        )
        .expect("ledger");
        let error = load_candidate(
            &scratch,
            &json!({"nodes": []}),
            &ledger,
            &ExecutionClaims::default(),
        )
        .err()
        .expect("missing explicit recovery root must fail");
        assert_eq!(error.code, "GATE-RESUME-PROVENANCE-PATH");

        fs::create_dir_all(missing.join(".checkpoints")).expect("unattested recovery root");
        let error = load_candidate(
            &scratch,
            &json!({"nodes": []}),
            &ledger,
            &ExecutionClaims::default(),
        )
        .err()
        .expect("missing explicit recovery provenance must fail");
        assert_eq!(error.code, "GATE-RESUME-PROVENANCE-MISSING");
        fs::remove_dir_all(scratch).expect("remove scratch");
    }

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
    fn pre_receipt_checkpoint_is_retained_but_not_imported_without_receipt() {
        let root = std::env::temp_dir().join(format!(
            "openwepp-resume-{}-pre-receipt",
            std::process::id()
        ));
        let history = root.join("history");
        let recovery = history.join("recovery/prior");
        let prior = recovery.join("execution");
        fs::create_dir_all(prior.join(".checkpoints")).expect("checkpoint directory");
        fs::create_dir_all(prior.join("target/heavy")).expect("prior output directory");
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
            "execution_binding": {
                "plan_id": "prior-plan",
                "execution_key": "prior-key",
                "boundary": "INCREMENT",
                "roots": {"execution_root": "e", "authority_root": "a", "assurance_root": "s", "documentation_root": "old-docs"},
                "execution_context": {"tool": "fixed"},
                "policy": {"generation": 1},
                "claims": {"workflow": "w", "job": "j", "runner": "r", "attempt": 1},
            },
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
        let ledger = history.join("attempts.jsonl");
        fs::write(
            &ledger,
            format!(
                "{{\"artifact_root\":\"{}\",\"plan_id\":\"different-plan\",\"record_type\":\"STAGE_ATTEMPT\",\"status\":\"FAILED\"}}\n",
                recovery.display()
            ),
        )
        .expect("ledger");
        let plan = json!({
            "plan_id": "new-plan",
            "execution_key": "new-key",
            "boundary": "INCREMENT",
            "environment_roots": {"execution_root": "e", "authority_root": "a", "assurance_root": "s", "documentation_root": "new-docs"},
            "execution_context": {"tool": "fixed"},
            "policy": {"generation": 1},
            "nodes": [node.take()]
        });
        assert!(
            load_candidate(&root, &plan, &ledger, &ExecutionClaims::default())
                .expect("load candidate")
                .is_none()
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the recovery fixture binds provenance, checkpoint, prior plan, and documentation drift"
    )]
    fn hosted_attested_pre_receipt_checkpoint_survives_documentation_plan_change() {
        let root = std::env::temp_dir().join(format!(
            "openwepp-resume-{}-attested-pre-receipt",
            std::process::id()
        ));
        let history = root.join("history");
        let recovery = history.join("recovery/42-1");
        let checkpoints = recovery.join(".checkpoints");
        fs::create_dir_all(&checkpoints).expect("checkpoint directory");
        fs::create_dir_all(recovery.join("out")).expect("output directory");
        fs::write(recovery.join("out/result"), b"pass").expect("output");
        let mut node = json!({
            "node_id": "0".repeat(64), "execution_cost_class": "HEAVY",
            "reuse_class": "HERMETIC_CONTENT", "output_paths": ["out/result"],
            "expected_inventory": {"ids": ["case"]}, "prerequisites": [],
        });
        node["node_id"] = json!(derived_id(&node, "node_id").expect("node identity"));
        let mut prior_plan = json!({
            "plan_id": "0".repeat(64), "execution_key": "0".repeat(64),
            "boundary": "INCREMENT", "source": {"head_commit": "a".repeat(40)},
            "environment_roots": {"execution_root": "e", "authority_root": "a", "assurance_root": "s", "documentation_root": "old"},
            "execution_context": {"tool": "fixed"}, "policy": {"generation": 1},
            "nodes": [node.clone()],
        });
        prior_plan["plan_id"] =
            json!(crate::planner::derive_plan_id(&prior_plan).expect("prior plan identity"));
        prior_plan["execution_key"] = json!(
            crate::planner::derive_execution_key(&prior_plan).expect("prior execution identity")
        );
        let mut checkpoint = json!({
            "schema_version": "openwepp-gate-node-checkpoint-v1",
            "checkpoint_id": "0".repeat(64), "node_id": node["node_id"],
            "node_sha256": digest(&node).expect("node digest"), "result": "PASS",
            "attempt": {"node_id": node["node_id"], "result": "PASS"},
            "artifacts": [{"path": "out/result", "sha256": sha256_bytes(b"pass")}],
            "execution_binding": {
                "plan_id": prior_plan["plan_id"], "execution_key": prior_plan["execution_key"],
                "boundary": "INCREMENT",
                "roots": {"execution_root": "e", "authority_root": "a", "assurance_root": "s", "documentation_root": "old"},
                "execution_context": {"tool": "fixed"}, "policy": {"generation": 1},
                "claims": {"workflow": "testgate", "job": "openwepp/execute-increment", "runner": "forest1", "attempt": 1},
            },
        });
        checkpoint["checkpoint_id"] =
            json!(derived_id(&checkpoint, "checkpoint_id").expect("checkpoint ID"));
        let checkpoint_path = checkpoints.join(format!(
            "{}.json",
            node["node_id"].as_str().expect("node ID")
        ));
        fs::write(
            &checkpoint_path,
            canonical_bytes(&checkpoint).expect("checkpoint bytes"),
        )
        .expect("checkpoint");
        fs::write(
            recovery.join("plan.json"),
            canonical_bytes(&prior_plan).expect("prior plan bytes"),
        )
        .expect("prior plan");
        let files = [
            checkpoint_path,
            recovery.join("out/result"),
            recovery.join("plan.json"),
        ]
            .into_iter()
            .map(|path| {
                json!({
                    "path": format!("recovery/42-1/{}", path.strip_prefix(&recovery).expect("relative").display()),
                    "sha256": sha256_bytes(&fs::read(path).expect("indexed bytes")),
                })
            })
            .collect::<Vec<_>>();
        let index = json!({
            "schema_version": "openwepp-testgate-attempt-index-v1",
            "provenance": {"repository": "rogerlew/openWEPP", "workflow": "testgate", "run_id": "42", "run_attempt": "1", "head_sha": "b".repeat(40)},
            "files": files,
        });
        let provenance = history.join("provenance/42-1");
        fs::create_dir_all(&provenance).expect("provenance");
        let index_bytes = serde_json::to_vec(&index).expect("index bytes");
        fs::write(provenance.join("attempt-index.json"), &index_bytes).expect("index");
        fs::write(
            provenance.join("recovery-predicate.json"),
            serde_json::to_vec(&json!({
                "schema_version": "openwepp-testgate-recovery-provenance-v1",
                "index_sha256": sha256_bytes(&index_bytes), "repository": "rogerlew/openWEPP",
                "workflow": "testgate", "source_ref": "refs/heads/main", "run_id": "42",
                "run_attempt": "1", "head_sha": "b".repeat(40),
            }))
            .expect("predicate bytes"),
        )
        .expect("predicate");
        fs::write(
            provenance.join("recovery-attestation.jsonl"),
            b"TEST-VALID-BUNDLE",
        )
        .expect("test attestation");
        let ledger = history.join("attempts.jsonl");
        fs::write(
            &ledger,
            serde_json::to_vec(&json!({
                "record_type": "STAGE_ATTEMPT", "status": "FAILED",
                "recovery_root": recovery.display().to_string(),
            }))
            .expect("ledger bytes"),
        )
        .expect("ledger");
        let plan = json!({
            "plan_id": "new-plan", "execution_key": "new-key", "boundary": "INCREMENT",
            "environment_roots": {"execution_root": "e", "authority_root": "a", "assurance_root": "s", "documentation_root": "new"},
            "execution_context": {"tool": "fixed"}, "policy": {"generation": 1},
            "nodes": [node],
        });
        let claims = ExecutionClaims {
            workflow: "testgate".to_owned(),
            job: "openwepp/execute-increment".to_owned(),
            source_ref: "refs/heads/main".to_owned(),
            ..ExecutionClaims::default()
        };
        let candidate = load_candidate(&root, &plan, &ledger, &claims).expect("attested candidate");
        assert!(candidate.is_some());
        let current = root.join("current");
        fs::create_dir(&current).expect("current execution root");
        let seed = apply_candidate(&plan, &current, &claims, candidate.as_ref())
            .expect("import attested checkpoint");
        assert_eq!(seed.decisions[0]["decision"], "IMPORTED");
        assert_eq!(
            seed.decisions[0]["prior_provenance_id"],
            sha256_bytes(&index_bytes)
        );
        assert!(seed.decisions[0]["prior_receipt_id"].is_null());
        assert_eq!(
            fs::read(current.join("out/result")).expect("imported output"),
            b"pass"
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn self_hashed_pass_checkpoint_must_match_accepted_receipt_attempt() {
        let root = std::env::temp_dir().join(format!(
            "openwepp-resume-{}-receipt-binding",
            std::process::id()
        ));
        fs::create_dir_all(root.join("out")).expect("artifact root");
        fs::write(root.join("out/result"), b"pass").expect("artifact");
        let node = json!({
            "node_id": "1".repeat(64), "execution_cost_class": "HEAVY",
            "reuse_class": "HERMETIC_CONTENT", "output_paths": ["out/result"]
        });
        let attempt = json!({"node_id": node["node_id"], "attempt": 1, "result": "PASS"});
        let claims = json!({"workflow": "w", "job": "j", "runner": "r", "attempt": 1});
        let plan = json!({
            "plan_id": "2".repeat(64), "execution_key": "3".repeat(64),
            "boundary": "INCREMENT",
            "environment_roots": {"execution_root": "e", "authority_root": "a", "assurance_root": "s"},
            "execution_context": {"tool": "fixed"}, "policy": {"generation": 1}
        });
        let artifact = json!({"path": "out/result", "sha256": sha256_bytes(b"pass")});
        let mut checkpoint = json!({
            "schema_version": "openwepp-gate-node-checkpoint-v1",
            "checkpoint_id": "0".repeat(64), "node_id": node["node_id"],
            "node_sha256": digest(&node).expect("node digest"), "result": "PASS",
            "attempt": attempt, "artifacts": [artifact],
            "execution_binding": {
                "plan_id": plan["plan_id"], "execution_key": plan["execution_key"],
                "boundary": plan["boundary"], "roots": plan["environment_roots"],
                "execution_context": plan["execution_context"], "policy": plan["policy"],
                "claims": claims
            }
        });
        checkpoint["checkpoint_id"] = json!(derived_id(&checkpoint, "checkpoint_id").expect("ID"));
        let mut receipt = json!({
            "claims": {
                "principal": "developer", "repository": "owner/repo",
                "source_event": "push", "source_ref": "refs/heads/main",
                "trust_class": "LOCAL_UNTRUSTED",
                "workflow": claims["workflow"], "job": claims["job"],
                "runner": claims["runner"], "attempt": claims["attempt"]
            },
            "attempts": [checkpoint["attempt"].clone()],
            "artifacts": [artifact]
        });
        verify_checkpoint(&plan, &node, &checkpoint, &receipt, &root)
            .expect("exact receipt binding");
        receipt["attempts"][0]["result"] = json!("FAIL");
        let error = verify_checkpoint(&plan, &node, &checkpoint, &receipt, &root)
            .expect_err("forged checkpoint must not override failed receipt");
        assert_eq!(error.code, "GATE-RESUME-CHECKPOINT-RECEIPT-MISMATCH");
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn authenticated_archive_index_must_cover_exact_recovery_tree() {
        let root = std::env::temp_dir().join(format!(
            "openwepp-resume-{}-indexed-root",
            std::process::id()
        ));
        fs::create_dir_all(root.join("out")).expect("recovery root");
        fs::write(root.join("out/result"), b"pass").expect("artifact");
        let index = json!({"files": [{
            "path": "recovery/42-1/out/result",
            "sha256": sha256_bytes(b"pass"),
        }]});
        verify_indexed_recovery_root(&index, &root, "42-1").expect("exact indexed root");
        fs::write(root.join("unindexed"), b"forged").expect("unindexed artifact");
        let error = verify_indexed_recovery_root(&index, &root, "42-1")
            .expect_err("unindexed recovery bytes must fail closed");
        assert_eq!(error.code, "GATE-RESUME-PROVENANCE-FILESET");
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn resume_rejects_dangling_destination_symlink() {
        use std::os::unix::fs::symlink;

        let root =
            std::env::temp_dir().join(format!("openwepp-resume-{}-dangling", std::process::id()));
        let source = root.join("source");
        let target = root.join("target");
        fs::create_dir_all(source.join("out")).expect("source");
        fs::create_dir_all(target.join("out")).expect("target");
        fs::write(source.join("out/result"), b"pass").expect("source artifact");
        symlink(root.join("missing"), target.join("out/result")).expect("dangling symlink");
        let node = json!({"output_paths": ["out/result"]});
        let checkpoint = json!({"artifacts": [{
            "path": "out/result", "sha256": sha256_bytes(b"pass")
        }]});
        let error = copy_outputs(&node, &checkpoint, &source, &target)
            .expect_err("dangling destination must fail closed");
        assert_eq!(error.code, "GATE-RESUME-OUTPUT-COLLISION");
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn resume_rejects_ancestor_symlink_without_outside_mutation() {
        use std::os::unix::fs::symlink;

        let root =
            std::env::temp_dir().join(format!("openwepp-resume-{}-ancestor", std::process::id()));
        let source = root.join("source");
        let target = root.join("target");
        let outside = root.join("outside");
        fs::create_dir_all(source.join("out/nested")).expect("source");
        fs::create_dir_all(&target).expect("target");
        fs::create_dir_all(&outside).expect("outside");
        fs::write(source.join("out/nested/result"), b"pass").expect("source artifact");
        symlink(&outside, target.join("out")).expect("ancestor symlink");
        let node = json!({"output_paths": ["out/nested/result"]});
        let checkpoint = json!({"artifacts": [{
            "path": "out/nested/result", "sha256": sha256_bytes(b"pass")
        }]});
        let error = copy_outputs(&node, &checkpoint, &source, &target)
            .expect_err("ancestor symlink must fail closed");
        assert!(error.code.contains("SYMLINK") || error.code.contains("ESCAPE"));
        assert!(!outside.join("nested").exists());
        fs::remove_dir_all(root).expect("remove fixture");
    }
}
