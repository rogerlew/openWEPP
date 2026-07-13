use std::fs;
use std::path::Path;

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

#[test]
fn auth04_package_and_authority_model_capture_lane_policy() {
    let package = repo_file(
        "docs/work-packages/20260531-auth04-release-gate-authority-stack-integration-001/package.md",
    );
    let authority_model = repo_file("docs/specifications/correctness-authority-model.md");

    assert!(
        package.contains("Wire required constitutive suites into release-gates CI path.")
            && package.contains("Add periodic/manual lanes for heavier suite classes"),
        "AUTH04 package must preserve required + periodic/manual lane scope"
    );
    assert!(
        authority_model.contains("## Release/CI Lane Enforcement (Normative)")
            && authority_model.contains("required")
            && authority_model.contains("periodic")
            && authority_model.contains("manual"),
        "correctness authority model must define release/CI lane enforcement policy"
    );
    assert!(
        authority_model.contains("hard-fail")
            && authority_model.contains("investigation")
            && authority_model.contains("gate exits non-zero"),
        "authority model must define failure-class gate behavior"
    );
}

#[test]
fn auth04_release_gate_script_enforces_lane_and_failure_class_semantics() {
    let script = repo_file("tools/release/run_release_candidate_gates.sh");

    assert!(
        script.contains("--skip-authority-required")
            && script.contains("--run-authority-periodic")
            && script.contains("--run-authority-manual"),
        "release gate script must expose authority lane flags"
    );
    assert!(
        script.contains("authority_lane_rows")
            && script.contains("gate_lane")
            && script.contains("failure_class"),
        "release gate script must derive suite runs from registry lane/failure metadata"
    );
    assert!(
        script.contains("run_authority_lane \"required\" \"hard-fail\"")
            && script.contains("run_authority_lane \"periodic\" \"hard-fail\"")
            && script.contains("run_authority_lane \"manual\" \"hard-fail\""),
        "release gate script must wire required/periodic/manual hard-fail execution paths"
    );
    assert!(
        script.contains("AUTHORITY_INVESTIGATION_FAILURES=1")
            && script.contains("failure_class=investigation"),
        "release gate script must record investigation failures as non-blocking outcomes"
    );
}

#[test]
fn intval_rel001_release_workspace_gate_uses_nextest_process_isolation() {
    let script = repo_file("tools/release/run_release_candidate_gates.sh");

    assert!(
        script.contains("cargo nextest run --workspace --profile full"),
        "INTVAL-REL-001: canonical workspace closure must use the full nextest profile"
    );
    assert!(
        !script.contains("\ncargo test --workspace\n"),
        "INTVAL-REL-001: threaded workspace libtest violates H2637 process isolation"
    );
}

#[test]
fn auth04_workflow_exposes_periodic_and_manual_authority_lane_triggers() {
    let workflow = repo_file(".github/workflows/release-gates.yml");

    assert!(
        workflow.contains("schedule:")
            && workflow.contains("cron: \"0 9 * * 1\"")
            && workflow.contains("run_authority_periodic")
            && workflow.contains("run_authority_manual"),
        "release workflow must expose scheduled/dispatch triggers for periodic/manual authority lanes"
    );
    assert!(
        workflow.contains("gate_cmd+=(--run-authority-periodic)")
            && workflow.contains("gate_cmd+=(--run-authority-manual)"),
        "release workflow must route trigger inputs into release-gate authority lane flags"
    );
}

#[test]
fn auth04_release_runbook_and_release_readme_document_authority_stack_outputs() {
    let runbook = repo_file("docs/governance/openwepp-release-procedure-draft.md");
    let release_readme = repo_file("tools/release/README.md");

    assert!(
        runbook.contains("## Authority-Stack Gate Policy")
            && runbook.contains("authority_suite_results.md"),
        "release runbook must document authority lane policy and report artifact"
    );
    assert!(
        release_readme.contains("Required authority lane runs by default")
            || release_readme.contains("required lane runs by default"),
        "release tooling README must document default required-lane behavior"
    );
    assert!(
        release_readme.contains("--run-authority-periodic")
            && release_readme.contains("--run-authority-manual")
            && release_readme.contains("--authority-report"),
        "release tooling README must document authority lane and reporting flags"
    );
}
