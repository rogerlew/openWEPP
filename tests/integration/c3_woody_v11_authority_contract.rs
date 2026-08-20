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
    ] {
        assert!(vegetation.contains(required), "missing {required}");
    }
    for required in [
        "INV-VEGTRANSACTION-009",
        "INV-VEGTRANSACTION-013",
        "accepted-segment hierarchy",
        "one complete parent commit",
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
        "sub_ns_nonroundtrip",
        "600_1200",
        "1200_600",
        "one_ns_first",
        "one_ns_last",
        "three_unequal",
        "gap",
        "overlap",
        "overbook",
        "two_increments",
        "rejected_attempt_noop",
        "scheduled_twice",
        "restart_replay",
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
    assert_eq!(result["results"].as_array().unwrap().len(), 22);
}
