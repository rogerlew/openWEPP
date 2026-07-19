use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).expect("contract source must be readable")
}

fn assert_receipt_runtime_guards() {
    let executor = text("crates/openwepp-gate-planner/src/executor.rs");
    for required in [
        "reconstruct_plan_in(",
        ".env_clear()",
        "Command::new(program)",
        "GATE-EXEC-INVENTORY-DRIFT",
        "GATE-EXEC-SOURCE-MUTATION",
        "GATE-EXEC-OUTPUT-COLLISION",
        "GATE-EXEC-UNSUPPORTED",
        "observed_source_snapshot",
        "OPENWEPP_GATE_ARTIFACT_ROOT",
        "CARGO_TARGET_DIR",
        "PREREQUISITE_NONPASS",
        "PROCESS_SPAWN_FAILED",
        "TIMEOUT",
        "SOURCE_MUTATION_DETECTED",
        "TEST_NOT_EXECUTED",
        "GATE-EXEC-SHELL-STRING",
    ] {
        assert!(
            executor.contains(required),
            "missing executor guard: {required}"
        );
    }
    assert!(!executor.contains("sh -c"));
    let cli = text("crates/openwepp-gate-planner/src/main.rs");
    assert!(cli.contains("verify_receipt(repo, &plan, &receipt, &artifacts)"));
    assert!(cli.contains("Some(\"FAIL\" | \"BLOCKED\" | \"INVALID\")"));

    let verifier = text("crates/openwepp-gate-planner/src/verifier.rs");
    for required in [
        "GATE-RECEIPT-EXECUTED-INVENTORY",
        "GATE-RECEIPT-UNAVAILABLE",
        "GATE-RECEIPT-PREREQUISITE",
        "verifier_accepts_truthful_fail_and_blocked_receipts",
    ] {
        assert!(
            verifier.contains(required),
            "missing verifier proof: {required}"
        );
    }
}

#[test]
fn blocking_executor_and_affected_quality_preserve_manual_rollback() {
    assert_receipt_runtime_guards();
    let definitions: Value = serde_json::from_str(&text("gate-policy/v1/gate-definitions.json"))
        .expect("gate definitions JSON");
    assert_eq!(definitions["enforcement_status"], "BLOCKING");
    let entries = definitions["definitions"].as_array().expect("definitions");
    let affected = entries
        .iter()
        .find(|entry| entry["gate_definition_id"] == "affected-adjudicated-crap-v1")
        .expect("affected CRAP definition");
    assert_eq!(
        affected["risk_classes"],
        serde_json::json!(["BOUNDED_COMPONENT", "INTEGRATED_DOMAIN"])
    );
    assert!(
        affected["arguments_template"]
            .as_array()
            .expect("affected arguments")
            .iter()
            .any(|argument| argument == "affected")
    );
    let global = entries
        .iter()
        .find(|entry| entry["gate_definition_id"] == "adjudicated-crap-v1")
        .expect("global CRAP definition");
    assert_eq!(global["risk_classes"], serde_json::json!(["CRITICAL"]));
    for (id, authority_class) in [
        ("authority-admission-v1", "A0"),
        ("required-authority-v1", "A3"),
    ] {
        let authority = entries
            .iter()
            .find(|entry| entry["gate_definition_id"] == id)
            .unwrap_or_else(|| panic!("missing authority definition {id}"));
        assert_eq!(authority["authority_class"], authority_class);
        assert_eq!(authority["risk_classes"], serde_json::json!(["CRITICAL"]));
    }
    let full = entries
        .iter()
        .find(|entry| entry["gate_definition_id"] == "workspace-full-nextest-v1")
        .expect("full workspace definition");
    assert_eq!(full["authority_class"], "NONE");

    let plan_schema = text("gate-policy/v1/schemas/gate-plan.schema.json");
    assert!(plan_schema.contains("quality_scope"));
    assert!(plan_schema.contains("covering_inventory_ids"));
    assert!(plan_schema.contains("ESCALATED_GLOBAL"));

    let affected_driver = text("tools/release/run_adjudicated_crap_gate.sh");
    assert!(affected_driver.contains("SCOPE=\"global\""));
    assert!(affected_driver.contains("affected scope requires fresh acquisition"));
    assert!(affected_driver.contains("--expected-package"));
    assert!(affected_driver.contains("cargo llvm-cov show-env --sh"));
    assert!(affected_driver.contains("cargo nextest run"));
    assert!(affected_driver.contains("--config-file \"${NEXTEST_CONFIG}\""));
    assert!(affected_driver.contains("OPENWEPP_GATE_ARTIFACT_ROOT"));
    assert!(affected_driver.contains("for package in \"${PACKAGES[@]}\""));

    let profiles = text(".config/nextest.toml");
    for profile in ["affected", "checkpoint", "campaign", "release", "full"] {
        assert!(
            profiles.contains(&format!("[profile.{profile}]")),
            "missing lifecycle profile {profile}"
        );
    }

    let workflow = text(".github/workflows/testgate-shadow.yml");
    for context in [
        "increment-gates:",
        "verify-increment:",
        "name: openwepp/verify-increment",
        "name: openwepp/increment-gates",
        "name: openwepp/execute-increment",
        "runs-on: [self-hosted, Linux, X64, openwepp, omarchy, trusted]",
        "runs-on: ubuntu-24.04",
        "bootstrap_dependencies.sh",
        "tools/local_ci/testgate.py",
        "--boundary INCREMENT",
        "actions/attest@f7c74d28b9d84cb8768d0b8ca14a4bac6ef463e6",
        "attestation-predicate.json",
        "github-attestation.jsonl",
        "gh attestation verify",
        "verify-receipt-envelope",
        "Independently admit comparison base",
        "_intent_authorization",
        "--job openwepp/execute-increment",
        "--signer-workflow",
        "--source-digest",
        "--deny-self-hosted-runners",
        "actions/download-artifact@d3f86a106a0bac45b974a628896c90dbdf5c8093",
        "id-token: write",
        "attestations: write",
        "cargo-nextest@0.9.138",
        "--artifact-root \"${EVIDENCE_DIR}/execution\"",
        "if: ${{ always() }}",
    ] {
        assert!(
            workflow.contains(context),
            "missing blocking workflow contract {context}"
        );
    }
    assert!(!workflow.contains("pull_request:"));
    assert!(!workflow.contains("pull_request_target:"));
    assert!(!workflow.contains("schedule:"));
    assert!(!workflow.contains("ubuntu-latest"));
    assert!(!workflow.contains("conservative-rollback:"));
    assert!(!workflow.contains("inputs.boundary"));
    assert!(!workflow.contains("inputs.mode"));
    assert!(!workflow.contains("cargo install"));
    assert!(!workflow.contains("continue-on-error: true"));
    assert!(workflow.contains("persist-credentials: false"));
    assert!(workflow.contains("git merge-base --is-ancestor"));
    assert!(workflow.contains("permissions:\n  contents: read"));
    assert!(workflow.contains("execute-increment:\n    name: openwepp/execute-increment"));
    assert!(workflow.contains(
        "increment-gates:\n    name: openwepp/increment-gates\n    needs: [execute-increment, verify-increment]\n    if: ${{ always() }}"
    ));
    let signer = workflow
        .split_once("  increment-gates:")
        .expect("signer job")
        .1;
    assert!(!signer.contains("actions/checkout"));
    assert!(!signer.contains("cargo build"));
    assert!(!signer.contains("python3"));

    let conservative = text(".github/workflows/testgate-conservative.yml");
    assert!(conservative.contains("conservative-rollback:"));
    assert!(conservative.contains("name: openwepp/conservative-rollback"));
    assert!(conservative.contains("runs-on: ubuntu-24.04"));
    assert!(conservative.contains("run_release_candidate_gates.sh"));
    assert!(conservative.contains("--mode validate"));
    assert!(!conservative.contains("--skip-authority-required"));
    assert!(conservative.contains("--skip-stability"));
    assert!(!conservative.contains("self-hosted"));

    let rollback =
        text("docs/work-packages/20260718-testgate-ci-shadow-executor-001/artifacts/rollback.md");
    assert!(rollback.contains("entire nonrequired shadow workflow"));
    assert!(rollback.contains("required only after provider-side cutover"));
}

#[test]
fn runner_container_has_no_host_or_privileged_mounts() {
    let manager = text("tools/ci/omarchy-runner/manage.sh");
    let image = text("tools/ci/omarchy-runner/Dockerfile");
    assert!(manager.contains("--security-opt no-new-privileges=true"));
    assert!(manager.contains("--cap-drop ALL"));
    assert!(manager.contains("--read-only"));
    assert!(manager.contains("dst=/runner-state,readonly"));
    assert!(manager.contains("--tmpfs"));
    assert!(manager.contains("/cache/target:rw,exec,nosuid,nodev"));
    assert!(manager.contains("/cache/cargo:rw,nosuid,nodev"));
    assert!(manager.contains("job-completed-hook.sh"));
    assert!(!manager.contains("/var/run/docker.sock"));
    assert!(!manager.contains("--privileged"));
    assert!(!manager.contains("--network host"));
    assert!(manager.contains("registration_token"));
    assert!(manager.contains("printf '%s\\n' \"${registration_token}\""));
    assert!(image.contains("RUSTUP_TOOLCHAIN=1.92.0-x86_64-unknown-linux-gnu"));
    assert!(image.contains(
        "ACTIONS_RUNNER_HOOK_JOB_COMPLETED=/usr/local/bin/openwepp-job-completed-hook.sh"
    ));
    let hook = text("tools/ci/omarchy-runner/job-completed-hook.sh");
    assert!(hook.contains("/runner-work /cache/cargo /cache/target /home/runner /tmp"));
    assert!(hook.contains("/runner-state/_diag"));
    assert!(hook.contains("for round in {1..10}"));
}
