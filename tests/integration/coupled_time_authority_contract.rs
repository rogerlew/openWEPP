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
    for id in ["parent_interval_identity_kat", "parent_transaction_identity_kat", "event_receipt_identity_kat", "ambiguous_length_left", "ambiguous_length_right", "event_parent_start", "event_inside", "event_parent_end", "two_events_same_tick", "participant_transition", "compatible_equal_constraints", "conflicting_equal_constraints", "retry_reduce", "restart_after_rejection", "duration_above_2p53_ns", "duration_u128_max", "quantize_exact_half_ties_even", "quantize_above_2p53_ns", "quantize_to_u128_max", "quantize_addition_overflow", "quantize_magnitude_overflow", "publication_before_commit", "transaction_successor_overflow", "direct_v10_hash_protection"] {
        assert!(ids.contains(&id), "missing executable vector {id}");
    }
    for case in cases {
        assert_ne!(case["op"], "forced_error", "{} may not bypass executable semantics", case["id"]);
        let expected = case.get("expected").and_then(Value::as_object).expect("every case has expected object");
        assert!(expected.contains_key("status"));
        if expected["status"] == "rejected" {
            assert!(expected.contains_key("error"));
            assert_eq!(expected["before_sha256"], expected["after_sha256"], "{} must prove atomic no-op", case["id"]);
        }
    }
    assert_ne!(
        cases.iter().find(|c| c["id"] == "ambiguous_length_left").unwrap()["expected"]["sha256"],
        cases.iter().find(|c| c["id"] == "ambiguous_length_right").unwrap()["expected"]["sha256"],
        "length framing must separate ambiguous concatenations",
    );
    let model: Value = serde_json::from_str(&read(
        "docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/model-definition.json",
    ))
    .expect("model definition parses");
    for domain in model["identity_domain_fields"].as_object().expect("closed identity domains").keys() {
        let kat_id = format!("{}_identity_kat", domain.replace('-', "_"));
        assert!(ids.contains(&kat_id.as_str()), "missing exact framed KAT for {domain}");
        let case = cases.iter().find(|c| c["id"] == kat_id).unwrap();
        let declared = model["identity_domain_fields"][domain].as_array().unwrap();
        assert_eq!(case["fields"].as_array().unwrap().len(), declared.len(), "{domain} field cardinality");
        for (field, declaration) in case["fields"].as_array().unwrap().iter().zip(declared) {
            let declaration = declaration.as_str().unwrap();
            let (tag, kind) = declaration.split_once(':').unwrap();
            assert_eq!(field["tag"], tag);
            assert_eq!(field["type"], kind);
        }
    }
    for id in [
        "restart_immediately_before_event",
        "restart_immediately_after_event",
        "restart_poison_run_id",
        "restart_poison_controller_policy",
        "restart_poison_accepted_event_receipts",
        "restart_poison_scheduled_once_receipts",
        "restart_poison_reduction_state",
        "restart_poison_publication_outbox",
        "restart_uninterrupted_equivalence_before_event",
        "restart_uninterrupted_equivalence_after_event",
        "owner_ledger_join_success",
        "owner_cardinality_join_failure",
        "ledger_join_failure",
        "outbox_crash_retains_receipt",
        "outbox_idempotent_redelivery",
        "outbox_ack",
        "reduction_accepted_only",
        "reduction_rejected_attempt_alias",
        "reduction_nominal_duration_alias",
        "reduction_precommit_alias",
        "authority_tuple_legacy_valid",
        "authority_tuple_richards_valid",
        "authority_tuple_richards_nonpersistent_lane_d",
        "authority_tuple_richards_legacy_r4l",
    ] {
        assert!(ids.contains(&id), "missing executable authority case {id}");
    }
}

#[test]
fn independent_reference_model_passes_without_rust_expected_value_calls() {
    let reference = root().join("docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/reference_model.py");
    let source = fs::read_to_string(&reference).expect("reference source");
    assert!(!source.contains("subprocess"));
    assert!(!source.contains("import openwepp"));
    assert!(!source.contains("forced_error"));
    let output = Command::new("python3").arg(reference).output().expect("python runs");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let actual: Value = serde_json::from_slice(&output.stdout).expect("reference emits JSON");
    assert_eq!(actual["schema"], "OPENWEPP_COUPLED_TIME_REFERENCE_RESULTS_V1");
    let vectors: Value = serde_json::from_str(&read("docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts/coupled-time-vectors.json")).expect("vectors parse");
    assert_eq!(format!("{:x}", Sha256::digest(&output.stdout)), vectors["expected_reference_results_sha256"].as_str().unwrap());
    let results = actual["results"].as_array().expect("results");
    let cases = vectors["cases"].as_array().expect("cases");
    assert_eq!(results.len(), cases.len());
    for (index, (case, result)) in cases.iter().zip(results).enumerate() {
        assert_eq!(result["id"], case["id"]);
        let mut expected = case["expected"].as_object().expect("expected").clone();
        expected.insert("id".into(), case["id"].clone());
        assert_eq!(result.as_object().expect("result"), &expected, "structural reference mismatch at case {index}: {}", case["id"]);
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

#[test]
fn independent_semantic_schema_validator_rejects_poison_population() {
    let artifacts = root().join(
        "docs/work-packages/20260820-coupled-time-authority-implementation-001/artifacts",
    );
    let validator = artifacts.join("semantic_schema_validator.py");
    let poisons = artifacts.join("semantic-schema-poisons.json");
    let source = fs::read_to_string(&validator).expect("validator source");
    assert!(!source.contains("import openwepp"));
    assert!(!source.contains("subprocess"));
    let output = Command::new("python3")
        .arg(validator)
        .arg("--poisons")
        .arg(poisons)
        .output()
        .expect("semantic validator runs");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let report: Value = serde_json::from_slice(&output.stdout).expect("validator emits JSON");
    assert_eq!(report["schema"], "OPENWEPP_COUPLED_TIME_SEMANTIC_VALIDATION_RESULTS_V1");
    let results = report["results"].as_array().expect("result array");
    assert!(results.len() >= 30, "poison population must remain broad");
    for id in [
        "u128_overflow", "leading_zero", "equal_support", "owner_digest_corrupt",
        "owner_set_digest_corrupt", "participant_not_owner", "controller_bytes_corrupt",
        "missing_policy", "candidate_omitted", "candidate_wrong_owner",
        "candidate_wrong_disposition", "candidate_bad_base64",
        "candidate_state_digest_mismatch", "duplicate_owner_id", "unresolved_ledger",
        "candidate_support_mismatch", "candidate_duration_mismatch", "field_order_poison",
        "whitespace_poison", "duplicate_field_poison", "future_scheduled_receipt",
        "invalid_scheduled_receipt_id", "outbox_records_digest_corrupt",
        "outbox_state_invalid", "outbox_sequence_overflow",
    ] {
        let result = results.iter().find(|result| result["id"] == id).unwrap_or_else(|| panic!("missing poison {id}"));
        assert_eq!(result["status"], "rejected", "poison {id} must fail closed");
    }
}
