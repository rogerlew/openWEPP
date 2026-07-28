use serde_yaml::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap_or_else(|error| {
        panic!("read {path}: {error}");
    })
}

fn scratch(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "openwepp-quality-contract-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("create scratch");
    path
}

#[test]
fn workflow_uses_direct_exact_source_identity_without_gate_control_plane() {
    let workflow_text = text(".github/workflows/quality-observatory.yml");
    let workflow: Value = serde_yaml::from_str(&workflow_text).expect("parse workflow");
    let dispatch = workflow
        .get("on")
        .or_else(|| workflow.get(Value::Bool(true)))
        .and_then(|on| on.get("workflow_dispatch"))
        .expect("workflow dispatch");
    let inputs = dispatch.get("inputs").expect("dispatch inputs");
    assert_eq!(
        inputs
            .get("source_sha")
            .and_then(|value| value.get("required"))
            .and_then(Value::as_bool),
        Some(true)
    );
    assert!(inputs.get("qualification_run_id").is_none());
    assert!(workflow_text.contains("test \"${WORKFLOW_SHA}\" = \"${SOURCE_SHA}\""));
    assert!(workflow_text.contains("test \"$(git rev-parse HEAD)\" = \"${SOURCE_SHA}\""));
    assert!(workflow_text.contains("QUALITY_HISTORY_ROOT: /quality-history/quality-observatory"));
    assert!(workflow_text.contains("group: openwepp-forest1-quality-observatory"));
    for forbidden in [
        "testgate",
        "openwepp-gate-plan",
        "workplan-lint",
        "qualification_run_id",
        "priority-preflight",
    ] {
        assert!(
            !workflow_text.to_ascii_lowercase().contains(forbidden),
            "retired control-plane token remains: {forbidden}"
        );
    }
}

#[test]
fn controller_is_self_consistent_and_observes_only_quality_runs() {
    let controller = text("tools/local_ci/quality_observatory_workflow.py");
    assert!(
        controller.contains("CURRENT_WORKFLOW = \".github/workflows/quality-observatory.yml\"")
    );
    assert!(controller.contains("\"DEFERRED_QUALITY_OBSERVATORY_PRIORITY\""));
    assert!(controller.contains("GITHUB_RUN_ID"));
    for forbidden in [
        "TESTGATE",
        "testgate",
        "openwepp-gate-plan",
        "workplan-lint",
    ] {
        assert!(
            !controller.contains(forbidden),
            "retired control-plane token remains: {forbidden}"
        );
    }
    let status = Command::new(".venv/bin/python")
        .arg("tools/local_ci/quality_observatory_workflow.py")
        .arg("self-test")
        .current_dir(root())
        .status()
        .expect("run controller self-test");
    assert!(status.success());
}

#[test]
fn deterministic_preflight_defers_a_competing_quality_run() {
    let temp = scratch("preflight");
    let fixture = temp.join("occupancy.json");
    let output = temp.join("receipt.json");
    fs::write(
        &fixture,
        r#"{"schema_version":"openwepp-quality-occupancy-v1","runs":[{"id":7,"repository":"openwepp/openwepp","workflow":".github/workflows/quality-observatory.yml","event":"workflow_dispatch","head_sha":"1111111111111111111111111111111111111111","status":"queued","conclusion":null,"jobs":[],"artifacts":0}]}"#,
    )
    .expect("write fixture");
    let status = Command::new(".venv/bin/python")
        .arg("tools/local_ci/quality_observatory_workflow.py")
        .args(["preflight", "--repository", "openwepp/openwepp"])
        .arg("--occupancy-fixture")
        .arg(&fixture)
        .arg("--output")
        .arg(&output)
        .current_dir(root())
        .status()
        .expect("run preflight");
    assert!(status.success());
    let receipt = fs::read_to_string(&output).expect("read receipt");
    assert!(receipt.contains("\"disposition\":\"DEFERRED_QUALITY_OBSERVATORY_PRIORITY\""));
    fs::remove_dir_all(temp).expect("remove scratch");
}

#[test]
fn runner_storage_and_docs_have_no_testgate_identity() {
    for path in [
        "tools/ci/omarchy-runner/README.md",
        "tools/ci/omarchy-runner/manage.sh",
    ] {
        let value = text(path);
        assert!(!value.to_ascii_lowercase().contains("testgate"));
        assert!(value.contains("quality-history") || value.contains("quality-observatory"));
    }
    assert!(
        !root()
            .join(".github/workflows/testgate-shadow.yml")
            .exists()
    );
}

#[test]
fn conservative_correctness_workflow_is_direct_and_planner_free() {
    let workflow = text(".github/workflows/conservative-correctness.yml");
    assert!(workflow.contains("name: conservative-correctness"));
    assert!(workflow.contains("bash tools/release/run_release_candidate_gates.sh"));
    assert!(workflow.contains("cargo-nextest"));
    for forbidden in ["testgate", "openwepp-gate-plan", "workplan-lint"] {
        assert!(
            !workflow.to_ascii_lowercase().contains(forbidden),
            "retired control-plane token remains: {forbidden}"
        );
    }
    assert!(
        !root()
            .join(".github/workflows/testgate-conservative.yml")
            .exists()
    );
}
