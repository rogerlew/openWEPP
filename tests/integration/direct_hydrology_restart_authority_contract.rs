use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const AUTHORITY: &str = "docs/specifications/direct-hydrology-restart-v1.md";
const PACKAGE: &str =
    "docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/artifacts";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn authority_binds_layout_independence_atomicity_and_typed_failures() {
    let authority = text(AUTHORITY);
    for marker in [
        "never serializes a Rust object or `DirectRunFrame` memory layout",
        "0x` followed by exactly sixteen hexadecimal digits",
        "heterogeneous_lane_gsi_receipt",
        "transient V9 and LSE-V1 projections",
        "one non-fallible assignment",
        "unsupported_laned_active",
    ] {
        assert!(
            authority.contains(marker),
            "missing authority marker {marker}"
        );
    }
}

#[test]
fn frozen_boundary_vector_satisfies_machine_schema_and_preserves_signed_zero() {
    let schema: serde_json::Value =
        serde_json::from_str(&text(&format!("{PACKAGE}/checkpoint-schema.json")))
            .expect("checkpoint schema");
    let bytes = text(&format!("{PACKAGE}/checkpoint-vector.json"));
    let vector: serde_json::Value = serde_json::from_str(&bytes).expect("checkpoint vector");
    jsonschema::draft202012::new(&schema)
        .expect("compile checkpoint schema")
        .validate(&vector)
        .expect("frozen vector validates");
    assert!(bytes.contains("[\"0x0000000000000000\",\"0x8000000000000000\"]"));
    let digest_member = ",\"payload_sha256\":";
    let prefix_end = bytes.find(digest_member).expect("payload digest member");
    let prefix = format!("{}}}", &bytes[..prefix_end]);
    assert_eq!(
        format!("{:x}", Sha256::digest(prefix.as_bytes())),
        vector["payload_sha256"].as_str().expect("payload digest")
    );
    let ordered_markers = [
        "\"schema\"",
        "\"version\"",
        "\"run_identity\"",
        "\"topology\"",
        "\"configuration_identities\"",
        "\"transaction_lineage\"",
        "\"scheduler\"",
        "\"owners\"",
        "\"in_progress_day\"",
        "\"payload_sha256\"",
    ];
    let mut prior = 0;
    for marker in ordered_markers {
        let position = bytes[prior..].find(marker).expect("ordered member") + prior;
        assert!(position >= prior);
        prior = position + marker.len();
    }
}

#[test]
fn structural_poisons_are_rejected_and_matrix_is_complete() {
    let schema: serde_json::Value =
        serde_json::from_str(&text(&format!("{PACKAGE}/checkpoint-schema.json")))
            .expect("checkpoint schema");
    let compiled = jsonschema::draft202012::new(&schema).expect("compile checkpoint schema");
    let vector: serde_json::Value =
        serde_json::from_str(&text(&format!("{PACKAGE}/checkpoint-vector.json")))
            .expect("checkpoint vector");

    let mut missing = vector.clone();
    missing.as_object_mut().expect("object").remove("owners");
    assert!(compiled.validate(&missing).is_err());

    let mut extra = vector.clone();
    extra["unexpected"] = serde_json::json!(true);
    assert!(compiled.validate(&extra).is_err());

    let mut wrong_version = vector.clone();
    wrong_version["version"] = serde_json::json!(2);
    assert!(compiled.validate(&wrong_version).is_err());

    let mut noncanonical_float = vector;
    noncanonical_float["run_identity"]["payload"][0] = serde_json::json!(0.0);
    assert!(compiled.validate(&noncanonical_float).is_err());

    let matrix = text(&format!("{PACKAGE}/poison-matrix.md"));
    for category in [
        "duplicate_field",
        "heterogeneous_lane_gsi_receipt",
        "forcing_receipt_order",
        "v10_v9_projection",
        "lse_v2_v1_projection",
        "unsupported_laned_active",
    ] {
        assert!(matrix.contains(category), "missing poison {category}");
    }
}
