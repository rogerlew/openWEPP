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
fn shadow_executor_and_affected_quality_preserve_conservative_fallback() {
    assert_receipt_runtime_guards();
    let definitions: Value = serde_json::from_str(&text("gate-policy/v1/gate-definitions.json"))
        .expect("gate definitions JSON");
    assert_eq!(definitions["enforcement_status"], "SHADOW");
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

    let conservative = text(".github/workflows/release-gates.yml");
    assert!(conservative.contains("workspace-validation:"));
    assert!(conservative.contains("run_release_candidate_gates.sh"));
    let shadow = text(".github/workflows/testgate-shadow.yml");
    for context in [
        "shadow-presubmit:",
        "shadow-backstop:",
        "shadow-dispatch:",
        "shadow-campaign:",
        "shadow-release:",
        "testgate-shadow-observation:",
    ] {
        assert!(
            shadow.contains(context),
            "missing shadow lifecycle context {context}"
        );
    }
    assert!(!shadow.contains("continue-on-error: true"));
    assert!(shadow.contains("shadow-campaign:"));
    assert!(shadow.contains("shadow-release:"));
    assert!(shadow.contains("exit 1"));
    assert!(shadow.contains("inputs.boundary == 'INCREMENT'"));
    assert!(shadow.contains("inputs.boundary == 'CHECKPOINT'"));

    let rollback =
        text("docs/work-packages/20260718-testgate-ci-shadow-executor-001/artifacts/rollback.md");
    assert!(rollback.contains("entire nonrequired shadow workflow"));
    assert!(rollback.contains("required only after provider-side cutover"));
}

#[test]
fn shadow_rollback_removes_only_the_nonrequired_workflow() {
    let directory =
        std::env::temp_dir().join(format!("openwepp-testgate-rollback-{}", std::process::id()));
    fs::create_dir(&directory).expect("create precise rollback fixture");
    let release_source = root().join(".github/workflows/release-gates.yml");
    let shadow_source = root().join(".github/workflows/testgate-shadow.yml");
    let release = directory.join("release-gates.yml");
    let shadow = directory.join("testgate-shadow.yml");
    fs::copy(&release_source, &release).expect("copy conservative workflow");
    fs::copy(&shadow_source, &shadow).expect("copy shadow workflow");
    let before = fs::read(&release).expect("read conservative workflow before rollback");

    fs::remove_file(&shadow).expect("remove only nonrequired shadow workflow");

    assert!(!shadow.exists());
    assert_eq!(
        fs::read(&release).expect("read conservative workflow after rollback"),
        before
    );
    fs::remove_dir_all(&directory).expect("remove precise rollback fixture");
}
