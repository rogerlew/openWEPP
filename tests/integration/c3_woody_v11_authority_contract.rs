use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    fs::read_to_string(root().join(path)).expect("authority artifact")
}

#[test]
fn canonical_contracts_bind_v11_without_mutating_v10() {
    let vegetation = read("docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md");
    let transaction =
        read("docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md");
    for required in [
        "OPENWEPP_C3_WOODY_V11",
        "INV-VEGETATION-121",
        "INV-VEGETATION-128",
        "OPENWEPP_C3_WOODY_V11_RESTART_V1",
        "duration_s_bits",
        "one parent material batch",
        "occupancy-scoped debit receipt",
        "shared-resource owner transition",
        "OPENWEPP_C3_WOODY_V11_RESTART_V3",
    ] {
        assert!(vegetation.contains(required), "missing {required}");
    }
    for required in [
        "INV-VEGTRANSACTION-009",
        "INV-VEGTRANSACTION-013",
        "accepted-segment hierarchy",
        "one complete parent commit",
        "shared owner predecessor records",
    ] {
        assert!(transaction.contains(required), "missing {required}");
    }
    assert!(vegetation.contains("V10 configuration/state/model/vector bytes remain immutable"));
}

#[test]
fn frozen_vectors_are_executable_and_cover_required_aliases() {
    let base = "docs/work-packages/20260820-c3-woody-v11-segmented-support-001/artifacts";
    let migration: Value =
        serde_json::from_str(&read(&format!("{base}/v10-v11-migration-vectors.json"))).unwrap();
    let segmented: Value =
        serde_json::from_str(&read(&format!("{base}/segmented-support-vectors.json"))).unwrap();
    let ids = migration["cases"]
        .as_array()
        .unwrap()
        .iter()
        .chain(segmented["cases"].as_array().unwrap())
        .map(|case| case["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    for id in [
        "nominal_1800",
        "half_ns_tie_even_zero",
        "one_ns_lower_neighbor",
        "u128_range_overflow",
        "600_1200_forcing_order",
        "1200_600_forcing_order",
        "one_ns_first",
        "one_ns_last",
        "three_unequal",
        "event_at_start",
        "event_interior",
        "event_at_end_zero_remainder_skip",
        "restart_before_event_equivalent",
        "restart_after_event_equivalent",
        "consecutive_parent_1",
        "consecutive_parent_2",
        "gap",
        "overlap",
        "wrong_slab_receipt",
        "participant_mismatch",
        "duration_nominal_alias",
        "water_overbook",
        "nh4_overbook",
        "no3_overbook",
        "two_increments",
        "per_segment_commit",
        "rejected_attempt_noop",
        "scheduled_twice",
        "restart_replay",
        "event_replay",
        "publish_before_commit",
        "parent_abort_rollback",
        "nonassociative_three_resource_chain",
        "wrong_regrouped_water_ending",
        "regrouped_owner_alias",
        "two_occupancy_shared_water_owner_chain",
        "shared_water_wrong_occupancy_alias",
        "shared_water_missing_debit_link",
        "shared_water_forged_owner_transition",
        "shared_water_authorization_overbook",
    ] {
        assert!(ids.contains(&id), "missing {id}");
    }
}

#[test]
fn independent_reference_accepts_frozen_population() {
    let path = root().join(
        "docs/work-packages/20260820-c3-woody-v11-segmented-support-001/artifacts/reference_calculator.py",
    );
    let source = fs::read_to_string(&path).unwrap();
    assert!(!source.contains("import openwepp"));
    assert!(!source.contains("subprocess"));
    let output = Command::new("python3").arg(path).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["results"].as_array().unwrap().len(), 54);
    let control = result["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == "nonassociative_three_resource_chain")
        .expect("nonassociative control");
    assert_eq!(control["actual"]["status"], "accepted");
    assert_eq!(
        control["actual"]["resource_endings"]["water"],
        "241296780.79251432"
    );
    let shared = result["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == "two_occupancy_shared_water_owner_chain")
        .expect("shared owner control");
    assert_eq!(
        shared["actual"]["shared_owner_endings"]["water|hydrology|ofe-1|layer-1|soil"],
        "6.5"
    );
}

#[test]
fn binary64_sequential_owner_ending_is_not_the_regrouped_alias() {
    let beginning = 497_355_953.965_941_8_f64;
    let amounts = [
        108_987_197.969_511_36_f64,
        119_731_815.493_540_45_f64,
        27_340_159.710_375_622_f64,
    ];
    let sequential = amounts
        .iter()
        .fold(beginning, |staged, amount| staged - amount);
    let cumulative = amounts.iter().fold(0.0_f64, |total, amount| total + amount);
    let regrouped = beginning - cumulative;
    assert_eq!(sequential.to_bits(), 241_296_780.792_514_32_f64.to_bits());
    assert_eq!(regrouped.to_bits(), 241_296_780.792_514_38_f64.to_bits());
    assert_ne!(sequential.to_bits(), regrouped.to_bits());
}

#[test]
fn successor_schemas_are_closed_and_do_not_hide_v10_physics_in_blobs() {
    let base = "docs/work-packages/20260820-c3-woody-v11-segmented-support-001/artifacts";
    for name in [
        "v11-configuration-schema.json",
        "v11-state-schema.json",
        "v11-restart-schema.json",
        "v11-resource-custody-schema.json",
        "v11-restart-v3-schema.json",
    ] {
        let source = read(&format!("{base}/{name}"));
        let _: Value = serde_json::from_str(&source).unwrap();
        assert!(source.contains("\"additionalProperties\":false"));
        assert!(!source.contains("physical_state_base64"));
        assert!(!source.contains("physical_configuration_base64"));
    }
    let ledger = read(&format!("{base}/full-support-compatibility-ledger.md"));
    for required in [
        "unmatched",
        "synthetic unknown leaf",
        "every projection root",
    ] {
        assert!(
            ledger.contains(required),
            "missing fail-closed rule {required}"
        );
    }
}

#[test]
fn restart_v3_closes_resource_custody_without_mutating_v2() {
    let base = "docs/work-packages/20260820-c3-woody-v11-segmented-support-001/artifacts";
    for (name, expected) in [
        (
            "v11-restart-v2-schema.json",
            "af9314c3f1abd70c40b849c6f466046e3c5e519583a837eefca9edbf43d02441",
        ),
        (
            "restart_v2_reference.py",
            "13f3d009221a60cc2af094103255c5d8c3be2dbee657bb87144b2fee476bbf7c",
        ),
        (
            "restart-v2-poisons.json",
            "fa5ae93f8b8e109b851f37946070bff71b5f5182b5df818c80f0d4de9990ad34",
        ),
    ] {
        let output = Command::new("sha256sum")
            .arg(root().join(format!("{base}/{name}")))
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout)
                .split_whitespace()
                .next(),
            Some(expected)
        );
    }
    let path = root().join(format!("{base}/restart_v3_reference.py"));
    let output = Command::new("python3").arg(path).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        result["schema"],
        "OPENWEPP_C3_WOODY_V11_RESTART_V3_RESULTS_V2"
    );
    assert_eq!(result["accepted"]["accepted_prefix_slabs"], 1);
    assert_eq!(result["accepted"]["candidate_count"], 7);
    assert_eq!(result["accepted"]["transition_count"], 3);
    assert_eq!(
        result["accepted"]["complete_suffix_sha256"],
        "0b2ff7b0182c756d6d706016b164459d5d55e99e148bd776aca1c0d1d6341096"
    );
    assert_eq!(result["poisons"].as_array().unwrap().len(), 13);
}

#[test]
fn typed_semantic_authority_reconstructs_receipts_restart_and_atomic_commit() {
    let base = "docs/work-packages/20260820-c3-woody-v11-segmented-support-001/artifacts";
    for name in [
        "parent-candidate-schema.json",
        "owner-descriptor-manifest-schema.json",
        "imported-canonical-fixtures.json",
        "semantic-schema-poisons.json",
    ] {
        let _: Value = serde_json::from_str(&read(&format!("{base}/{name}"))).unwrap();
    }
    let path = root().join(format!("{base}/semantic_schema_validator.py"));
    let source = fs::read_to_string(&path).unwrap();
    assert!(!source.contains("import openwepp"));
    assert!(!source.contains("subprocess"));
    for required in [
        "framed(",
        "class AtomicStore",
        "payload_sha256",
        "duration_s_bits",
    ] {
        assert!(
            source.contains(required),
            "missing executable authority {required}"
        );
    }
    let output = Command::new("python3").arg(path).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        result["schema"],
        "OPENWEPP_C3_WOODY_V11_SEMANTIC_RESULTS_V1"
    );
    assert_eq!(result["restore_equivalent"], true);
    assert_eq!(result["atomic_commit_owner_count"], 7);
    assert_eq!(result["publication_count"], 1);
    assert_eq!(result["valid"]["receipt_count"], 13);
    assert_eq!(
        result["valid"]["ending_resource_bits"]["water"],
        "0000000000000000"
    );
    assert_eq!(result["poisons"].as_array().unwrap().len(), 37);
    let ids = result["poisons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    for id in [
        "wrong_digest",
        "broken_event_custody",
        "material_reordered",
        "local_duration_conversion",
        "rejected_attempt_leakage",
        "stale_clock_commit",
        "partial_owner_commit",
        "late_failure",
        "commit_consumed_twice",
        "unknown_receipt_body_field",
        "forged_hydrology_ending",
        "forged_live_beginning",
        "checkpoint_rejected_leakage",
        "checkpoint_event_replay",
    ] {
        assert!(ids.contains(&id), "missing semantic poison {id}");
    }
}

#[test]
fn restart_v2_reference_closes_complete_custody_amendment() {
    let output = Command::new("python3")
        .arg("docs/work-packages/20260820-c3-woody-v11-segmented-support-001/artifacts/restart_v2_reference.py")
        .output()
        .expect("run restart V2 reference");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).expect("restart V2 result JSON");
    assert_eq!(result["poisons"].as_array().unwrap().len(), 54);
    assert_eq!(
        result["accepted"]["complete_continuation_sha256"]
            .as_str()
            .unwrap()
            .len(),
        64
    );
    assert_eq!(
        result["accepted"]["ending_state"],
        result["accepted"]["uninterrupted_state"]
    );
    assert_eq!(result["accepted"]["suffix_slabs"], 1);
}
