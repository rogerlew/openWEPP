use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).expect("read repository file")
}

fn scratch(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "openwepp-quality-workflow-{name}-{}-{stamp}",
        std::process::id()
    ));
    fs::create_dir(&path).expect("create scratch directory");
    path
}

fn write_fixture(path: &Path, body: &str) {
    fs::write(path, body).expect("write occupancy fixture");
}

#[test]
fn quality_workflow_is_manual_forest1_specific_and_nonblocking() {
    let workflow = text(".github/workflows/quality-observatory.yml");
    let document: serde_yaml::Value =
        serde_yaml::from_str(&workflow).expect("quality workflow YAML");
    let events = document
        .get("on")
        .and_then(serde_yaml::Value::as_mapping)
        .expect("workflow event map");
    assert_eq!(events.len(), 1);
    let dispatch = events
        .get("workflow_dispatch")
        .and_then(serde_yaml::Value::as_mapping)
        .expect("manual dispatch only");
    let source = dispatch
        .get("inputs")
        .and_then(serde_yaml::Value::as_mapping)
        .and_then(|inputs| inputs.get("source_sha"))
        .and_then(serde_yaml::Value::as_mapping)
        .expect("required source SHA");
    assert_eq!(source.get("required"), Some(&serde_yaml::Value::Bool(true)));
    assert!(!source.contains_key("default"));
    assert!(workflow.contains("group: openwepp-forest1-quality-observatory"));
    assert!(!workflow.contains("group: openwepp-forest1-testgate"));
    assert!(workflow.contains("runs-on: [self-hosted, Linux, X64, openwepp, forest1, trusted]"));
    assert!(workflow.contains("actions: read"));
    assert!(workflow.contains("[[ \"${SOURCE_SHA}\" =~ ^[0-9a-f]{40}$ ]]"));
    assert!(workflow.contains("test \"$(git rev-parse HEAD)\" = \"${SOURCE_SHA}\""));
    assert!(workflow.contains("test \"$(git rev-parse refs/remotes/origin/main)\""));
    assert!(workflow.contains("quality_observatory_workflow.py preflight"));
    assert!(workflow.contains("quality_observatory_workflow.py supervise"));
    assert!(workflow.contains("run_quality_observatory_child.sh"));
    assert!(workflow.contains("actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02"));
    assert!(workflow.contains("overwrite: false"));
    assert!(workflow.contains("retention-days: 30"));
    assert!(!workflow.contains("openwepp/increment-gates"));
}

#[test]
fn workflow_controller_self_test_and_python_compile_pass() {
    let compile = Command::new(root().join(".venv/bin/python"))
        .args([
            "-m",
            "py_compile",
            "tools/local_ci/quality_observatory.py",
            "tools/local_ci/quality_observatory_workflow.py",
        ])
        .current_dir(root())
        .status()
        .expect("compile Python controllers");
    assert!(compile.success());
    let check = Command::new(root().join(".venv/bin/python"))
        .args([
            "tools/local_ci/quality_observatory_workflow.py",
            "self-test",
        ])
        .current_dir(root())
        .status()
        .expect("run workflow controller self-test");
    assert!(check.success());
}

#[test]
fn occupancy_preflight_defers_live_and_unknown_but_ignores_exact_omarchy() {
    let temp = scratch("preflight");
    let script = root().join("tools/local_ci/quality_observatory_workflow.py");
    let python = root().join(".venv/bin/python");
    let repository = "openwepp/openwepp";
    let cases = [
        (
            "live",
            r#"{"schema_version":"openwepp-quality-occupancy-v1","runs":[{"id":7,"repository":"openwepp/openwepp","workflow":".github/workflows/testgate-shadow.yml","event":"workflow_dispatch","head_sha":"1111111111111111111111111111111111111111","status":"queued","conclusion":null,"jobs":[],"artifacts":0}]}"#,
            "DEFERRED_TESTGATE_PRIORITY",
        ),
        (
            "omarchy",
            r#"{"schema_version":"openwepp-quality-occupancy-v1","runs":[{"id":29673299308,"repository":"openwepp/openwepp","workflow":".github/workflows/testgate-shadow.yml","event":"workflow_dispatch","head_sha":"850f7f6f10044c078299718d8e9c46b77d278a86","status":"completed","conclusion":"cancelled","jobs":[],"artifacts":0}]}"#,
            "READY",
        ),
        (
            "unknown",
            r#"{"schema_version":"wrong","runs":[]}"#,
            "DEFERRED_OCCUPANCY_UNKNOWN",
        ),
        (
            "requested",
            r#"{"schema_version":"openwepp-quality-occupancy-v1","runs":[{"id":8,"repository":"openwepp/openwepp","workflow":".github/workflows/testgate-shadow.yml","event":"workflow_dispatch","head_sha":"1111111111111111111111111111111111111111","status":"requested","conclusion":null,"jobs":[],"artifacts":0}]}"#,
            "DEFERRED_TESTGATE_PRIORITY",
        ),
        (
            "repo-drift",
            r#"{"schema_version":"openwepp-quality-occupancy-v1","runs":[{"id":8,"repository":"spoof/openwepp","workflow":".github/workflows/testgate-shadow.yml","event":"workflow_dispatch","head_sha":"1111111111111111111111111111111111111111","status":"queued","conclusion":null,"jobs":[],"artifacts":0}]}"#,
            "DEFERRED_OCCUPANCY_UNKNOWN",
        ),
        (
            "omarchy-drift",
            r#"{"schema_version":"openwepp-quality-occupancy-v1","runs":[{"id":29673299308,"repository":"openwepp/openwepp","workflow":".github/workflows/testgate-shadow.yml","event":"workflow_dispatch","head_sha":"850f7f6f10044c078299718d8e9c46b77d278a86","status":"completed","conclusion":"cancelled","jobs":[],"artifacts":1}]}"#,
            "DEFERRED_OCCUPANCY_UNKNOWN",
        ),
    ];
    for (name, fixture, expected) in cases {
        let fixture_path = temp.join(format!("{name}.json"));
        let output = temp.join(format!("{name}-receipt.json"));
        write_fixture(&fixture_path, fixture);
        let status = Command::new(&python)
            .arg(&script)
            .args(["preflight", "--repository", repository])
            .arg("--occupancy-fixture")
            .arg(&fixture_path)
            .arg("--output")
            .arg(&output)
            .current_dir(root())
            .status()
            .expect("run deterministic preflight");
        assert!(status.success());
        let receipt = fs::read_to_string(output).expect("read preflight receipt");
        assert!(
            receipt.contains(&format!("\"disposition\":\"{expected}\"")),
            "{name} receipt: {receipt}"
        );
    }
    fs::remove_dir_all(temp).expect("remove preflight scratch");
}

#[test]
fn supervisor_yields_to_testgate_and_removes_partial_publication() {
    let temp = scratch("yield");
    let fixture = temp.join("race.json");
    write_fixture(
        &fixture,
        r#"{"snapshots":[
{"schema_version":"openwepp-quality-occupancy-v1","runs":[]},
{"schema_version":"openwepp-quality-occupancy-v1","runs":[]},
{"after_path":"stage-full.done","snapshot":{"schema_version":"openwepp-quality-occupancy-v1","runs":[{"id":9,"repository":"openwepp/openwepp","workflow":".github/workflows/testgate-shadow.yml","event":"workflow_dispatch","head_sha":"1111111111111111111111111111111111111111","status":"in_progress","conclusion":null,"jobs":[{"name":"openwepp/execute-increment","status":"in_progress","labels":["self-hosted","Linux","X64","openwepp","forest1","trusted"]}],"artifacts":0}]}}
]}"#,
    );
    let attempt = temp.join("attempt");
    let control = temp.join("control");
    let lease = temp.join("forest1.lock");
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root())
        .output()
        .expect("resolve head");
    assert!(head.status.success());
    let head = String::from_utf8(head.stdout)
        .expect("UTF-8 head")
        .trim()
        .to_owned();
    let status = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/quality_observatory_workflow.py"))
        .args([
            "supervise",
            "--repo",
            ".",
            "--repository",
            "openwepp/openwepp",
            "--source-sha",
            &head,
            "--workflow-revision",
            &head,
            "--workflow-sha256",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "--poll-seconds",
            "0.01",
            "--grace-seconds",
            "0.1",
        ])
        .arg("--attempt-root")
        .arg(&attempt)
        .arg("--control-root")
        .arg(&control)
        .arg("--lease")
        .arg(&lease)
        .arg("--occupancy-fixture")
        .arg(&fixture)
        .args([
            "--",
            "bash",
            "-c",
            "mkdir -p \"$1/published\"; printf partial > \"$1/published/raw.lcov\"; touch \"$1/stage-full.done\"; sleep 10",
            "_",
        ])
        .arg(&attempt)
        .current_dir(root())
        .status()
        .expect("run supervised race fixture");
    assert!(status.success());
    assert!(!attempt.join("published").exists());
    let receipt = fs::read_to_string(control.join("quality-control-receipt.json"))
        .expect("read control receipt");
    assert!(receipt.contains("\"disposition\":\"DEFERRED_TESTGATE_PRIORITY\""));
    assert!(control.join("quality-partial-index.json").is_file());
    fs::remove_dir_all(temp).expect("remove supervisor scratch");
}

#[test]
fn supervisor_yields_after_science_before_crap() {
    let temp = scratch("before-crap");
    let fixture = temp.join("race.json");
    write_fixture(
        &fixture,
        r#"{"snapshots":[
{"schema_version":"openwepp-quality-occupancy-v1","runs":[]},
{"schema_version":"openwepp-quality-occupancy-v1","runs":[]},
{"after_path":"stage-science.done","snapshot":{"schema_version":"openwepp-quality-occupancy-v1","runs":[{"id":10,"repository":"openwepp/openwepp","workflow":".github/workflows/testgate-shadow.yml","event":"workflow_dispatch","head_sha":"1111111111111111111111111111111111111111","status":"queued","conclusion":null,"jobs":[],"artifacts":0}]}}
]}"#,
    );
    let attempt = temp.join("attempt");
    let control = temp.join("control");
    let lease = temp.join("forest1.lock");
    let head_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root())
        .output()
        .expect("resolve head");
    let head = String::from_utf8(head_output.stdout)
        .expect("UTF-8 head")
        .trim()
        .to_owned();
    let status = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/quality_observatory_workflow.py"))
        .args([
            "supervise",
            "--repo",
            ".",
            "--repository",
            "openwepp/openwepp",
            "--source-sha",
            &head,
            "--workflow-revision",
            &head,
            "--workflow-sha256",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "--poll-seconds",
            "0.01",
            "--grace-seconds",
            "0.1",
        ])
        .arg("--attempt-root")
        .arg(&attempt)
        .arg("--control-root")
        .arg(&control)
        .arg("--lease")
        .arg(&lease)
        .arg("--occupancy-fixture")
        .arg(&fixture)
        .args([
            "--",
            "bash",
            "-c",
            "mkdir -p \"$1/local\"; touch \"$1/stage-science.done\"; sleep 10",
            "_",
        ])
        .arg(&attempt)
        .current_dir(root())
        .status()
        .expect("run before-CRAP race fixture");
    assert!(status.success());
    let receipt = fs::read_to_string(control.join("quality-control-receipt.json"))
        .expect("read control receipt");
    assert!(receipt.contains("\"disposition\":\"DEFERRED_TESTGATE_PRIORITY\""));
    let names = fs::read_dir(&control)
        .expect("read control directory")
        .map(|entry| entry.expect("control entry").file_name())
        .collect::<Vec<_>>();
    assert_eq!(names.len(), 2, "control artifact set must remain exact");
    fs::remove_dir_all(temp).expect("remove before-CRAP scratch");
}

#[test]
fn corrupt_complete_candidate_fails_closed_and_cleans_raw_state() {
    let temp = scratch("corrupt");
    let fixture = temp.join("clear.json");
    write_fixture(
        &fixture,
        r#"{"schema_version":"openwepp-quality-occupancy-v1","runs":[]}"#,
    );
    let attempt = temp.join("attempt");
    let control = temp.join("control");
    let head_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root())
        .output()
        .expect("resolve head");
    let head = String::from_utf8(head_output.stdout)
        .expect("UTF-8 head")
        .trim()
        .to_owned();
    let status = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/quality_observatory_workflow.py"))
        .args([
            "supervise",
            "--repo",
            ".",
            "--repository",
            "openwepp/openwepp",
            "--source-sha",
            &head,
            "--workflow-revision",
            &head,
            "--workflow-sha256",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "--poll-seconds",
            "0.01",
            "--grace-seconds",
            "0.1",
        ])
        .arg("--attempt-root")
        .arg(&attempt)
        .arg("--control-root")
        .arg(&control)
        .arg("--lease")
        .arg(temp.join("forest1.lock"))
        .arg("--occupancy-fixture")
        .arg(&fixture)
        .args([
            "--",
            "bash",
            "-c",
            "mkdir -p \"$1/local\" \"$1/published\"; printf raw > \"$1/local/raw.lcov\"; printf '{}' > \"$1/published/run-status.json\"",
            "_",
        ])
        .arg(&attempt)
        .current_dir(root())
        .status()
        .expect("run corrupt publication fixture");
    assert!(!status.success());
    assert!(!attempt.join("local").exists());
    assert!(!attempt.join("published").exists());
    let receipt = fs::read_to_string(control.join("quality-control-receipt.json"))
        .expect("read failure receipt");
    assert!(receipt.contains("\"disposition\":\"EXECUTION_FAILED\""));
    fs::remove_dir_all(temp).expect("remove corrupt scratch");
}

#[test]
fn workflow_contract_keeps_exact_publication_and_bounded_priority_intervals() {
    let controller = text("tools/local_ci/quality_observatory_workflow.py");
    let collector = text("tools/local_ci/quality_observatory.py");
    let child = text("tools/local_ci/run_quality_observatory_child.sh");
    for name in [
        "quality-envelope.json",
        "quality-payload.json",
        "run-status.json",
        "inventory-full.json",
        "inventory-science-manual.json",
        "inventory-workspace.json",
        "junit-full.xml",
        "junit-science-manual.xml",
        "adjudicated-crap-report.json",
        "adjudicated-crap-report.md",
        "coverage-summary.json",
    ] {
        assert!(controller.contains(name));
        assert!(collector.contains(name));
    }
    assert!(controller.contains("MAX_PUBLISHED_BYTES = 100 * 1024 * 1024"));
    assert!(controller.contains("MAX_CONTROL_BYTES = 1024 * 1024"));
    assert!(controller.contains("default=30.0"));
    assert!(controller.contains("default=60.0"));
    assert!(controller.contains("signal.SIGTERM"));
    assert!(controller.contains("signal.SIGKILL"));
    assert!(controller.contains("DEFERRED_OCCUPANCY_UNKNOWN"));
    assert!(controller.contains("DEFERRED_TESTGATE_PRIORITY"));
    assert!(child.contains("--admission-mode workflow"));
    assert!(child.contains("quality_observatory.py transition"));
}
