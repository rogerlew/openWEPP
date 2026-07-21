//! Source-state observation helpers for gate-plan execution.

use std::path::Path;
use std::process::Command;

use serde_json::{Value, json};

use crate::canonical::digest;
use crate::error::{ErrorClass, GatePolicyError, Result};
use crate::planner::manifest_roots;
use crate::repository::observe_dirty;

pub(crate) fn current_roots(repo: &Path, plan: &Value) -> Result<Value> {
    let revision = plan["source"]["head_commit"].as_str().unwrap_or("HEAD");
    manifest_roots(repo, revision, true)
}

pub(crate) fn git_text(repo: &Path, arguments: &[&str]) -> Result<String> {
    let bytes = git_bytes(repo, arguments)?;
    String::from_utf8(bytes)
        .map_err(|error| execution_error("GATE-EXEC-GIT-UTF8", error.to_string()))
}

pub(crate) fn git_bytes(repo: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
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

pub(crate) fn source_snapshot(plan: &Value) -> Result<String> {
    digest(&json!({
        "source": plan["source"],
        "roots": plan["environment_roots"]
    }))
}

pub(crate) fn observed_source_snapshot(repo: &Path, plan: &Value) -> Result<String> {
    let reference = plan["source"]["head_commit"]
        .as_str()
        .or_else(|| plan["source"]["base_commit"].as_str())
        .ok_or_else(|| execution_error("GATE-EXEC-SOURCE", "missing source revision"))?;
    let observed = observe_dirty(repo, reference)?;
    let matches_plan = if plan["source"]["head_commit"].is_string() {
        observed.changes.is_empty()
    } else {
        plan["source"]["dirty_tree_digest"].as_str() == observed.dirty_tree_digest.as_deref()
            && plan["source"]["index_digest"].as_str() == observed.index_digest.as_deref()
            && plan["source"]["worktree_digest"].as_str() == observed.worktree_digest.as_deref()
            && plan["source"]["untracked_digest"].as_str() == observed.untracked_digest.as_deref()
    };
    if matches_plan {
        source_snapshot(plan)
    } else {
        digest(&json!({
            "planned_snapshot": source_snapshot(plan)?,
            "observed": {
                "dirty_tree_digest": observed.dirty_tree_digest,
                "index_digest": observed.index_digest,
                "worktree_digest": observed.worktree_digest,
                "untracked_digest": observed.untracked_digest
            }
        }))
    }
}

fn execution_error(code: &'static str, message: impl Into<String>) -> GatePolicyError {
    GatePolicyError::new(ErrorClass::Execution, code, message)
}
