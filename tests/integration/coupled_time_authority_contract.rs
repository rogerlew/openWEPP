use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative)).expect("authority file must exist")
}

#[test]
fn canonical_contract_carries_complete_time_authority() {
    let contract = read("docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md");
    for required in [
        "SC-COUPLEDTIME-001",
        "OPENWEPP_COUPLED_TIME_SUPPORT_V1",
        "ModelTimeNs",
        "zero-duration event transition",
        "complete parent owner set",
        "active participant set",
        "ScheduledOnce",
        "DiagnosticReduction",
        "ControllerPolicyMismatch",
        "PublicationBeforeParentCommit",
        "CALIBRATION_NOT_APPLICABLE",
    ] {
        assert!(contract.contains(required), "contract missing {required}");
    }
}

#[test]
fn frozen_vectors_have_separating_event_constraint_and_duration_cases() {
    let path = "docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/coupled-time-vectors.json";
    let vectors: Value = serde_json::from_str(&read(path)).expect("vectors parse");
    let cases = vectors["cases"].as_array().expect("case array");
    assert!(cases.len() >= 45, "authority population must stay broad");
    let ids: Vec<_> = cases.iter().map(|v| v["id"].as_str().unwrap()).collect();
    for id in ["event_parent_start", "event_inside", "event_parent_end", "two_events_same_tick", "participant_transition", "event_no_progress_cycle_new_ordinals", "compatible_equal_constraints", "conflicting_equal_constraints", "retry_reduce", "restart_after_rejection", "duration_above_2p53_ns", "duration_u128_max", "publication_before_commit", "publication_after_rollback", "direct_v10_hash_protection"] {
        assert!(ids.contains(&id), "missing executable vector {id}");
    }
    for case in cases {
        let expected = case.get("expected").and_then(Value::as_object).expect("every case has expected object");
        assert!(expected.contains_key("status"));
        if expected["status"] == "rejected" {
            assert!(expected.contains_key("error"));
            assert_eq!(expected["before_sha256"], expected["after_sha256"], "{} must prove atomic no-op", case["id"]);
        }
    }
}

#[test]
fn independent_reference_model_passes_without_rust_expected_value_calls() {
    let reference = root().join("docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/reference_model.py");
    let source = fs::read_to_string(&reference).expect("reference source");
    assert!(!source.contains("subprocess"));
    assert!(!source.contains("import openwepp"));
    let output = Command::new("python3").arg(reference).output().expect("python runs");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let actual: Value = serde_json::from_slice(&output.stdout).expect("reference emits JSON");
    assert_eq!(actual["schema"], "OPENWEPP_COUPLED_TIME_REFERENCE_RESULTS_V1");
    let vectors: Value = serde_json::from_str(&read("docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/coupled-time-vectors.json")).expect("vectors parse");
    assert_eq!(format!("{:x}", Sha256::digest(&output.stdout)), vectors["expected_reference_results_sha256"].as_str().unwrap());
    let results = actual["results"].as_array().expect("results");
    let cases = vectors["cases"].as_array().expect("cases");
    assert_eq!(results.len(), cases.len());
    for (case, result) in cases.iter().zip(results) {
        assert_eq!(result["id"], case["id"]);
        let mut expected = case["expected"].as_object().expect("expected").clone();
        expected.insert("id".into(), case["id"].clone());
        assert_eq!(result.as_object().expect("result"), &expected, "reference mismatch for {}", case["id"]);
    }
}

#[test]
fn restart_schema_is_additive_and_direct_v10_is_protected() {
    let schema = read("docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/restart-schema.json");
    assert!(schema.contains("OPENWEPP_COUPLED_TIME_RESTART_V1"));
    assert!(schema.contains("publication_outbox"));
    assert!(schema.contains("accepted_event_receipts"));
    let package = read("docs/work-packages/20260820-coupled-time-authority-implementation-001/package.md");
    assert!(package.contains("DirectV10 restart V1 schema, vectors, manifest, and bytes are protected"));
    assert!(package.contains("additive and versioned"));
}
