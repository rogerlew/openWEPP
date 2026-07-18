use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).expect("contract source must be readable")
}

#[test]
fn shadow_executor_and_affected_quality_preserve_conservative_fallback() {
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

    let executor = text("crates/openwepp-gate-planner/src/executor.rs");
    for required in [
        "reconstruct_plan(repo, plan)",
        ".env_clear()",
        "Command::new(program)",
        "GATE-EXEC-INVENTORY-DRIFT",
        "GATE-EXEC-SOURCE-MUTATION",
        "GATE-EXEC-OUTPUT-COLLISION",
        "GATE-EXEC-UNSUPPORTED",
    ] {
        assert!(
            executor.contains(required),
            "missing executor guard: {required}"
        );
    }
    assert!(
        !executor.contains("sh -c"),
        "executor must not evaluate a shell string"
    );
    let cli = text("crates/openwepp-gate-planner/src/main.rs");
    assert!(cli.contains("verify_receipt(repo, &plan, &receipt, &artifacts)"));

    let affected_driver = text("tools/release/run_adjudicated_crap_gate.sh");
    assert!(affected_driver.contains("SCOPE=\"global\""));
    assert!(affected_driver.contains("affected scope requires fresh acquisition"));
    assert!(affected_driver.contains("--expected-package"));

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
}
