use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};
const AUTHORITY: &str = "docs/specifications/direct-hydrology-restart-v1.md";
const PACKAGE: &str =
    "docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/artifacts";
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
fn artifact(name: &str) -> Vec<u8> {
    fs::read(root().join(PACKAGE).join(name)).unwrap()
}
fn value(name: &str) -> Value {
    serde_json::from_slice(&artifact(name)).unwrap()
}
fn payload_digest(raw: &[u8]) -> String {
    let marker = b",\"payload_sha256\":";
    let end = raw.windows(marker.len()).position(|v| v == marker).unwrap();
    let mut input = raw[..end].to_vec();
    input.push(b'}');
    format!("{:x}", Sha256::digest(input))
}

#[test]
fn authority_binds_canonical_phase_union_atomicity_and_typed_failures() {
    let authority = fs::read_to_string(root().join(AUTHORITY)).unwrap();
    for marker in [
        "never serializes a Rust object or `DirectRunFrame` memory layout",
        "HexU128",
        "one non-fallible assignment",
        "unsupported_laned_active",
    ] {
        assert!(authority.contains(marker), "missing {marker}")
    }
}

#[test]
fn all_real_vectors_are_canonical_digest_bound_typed_checkpoints() {
    for name in [
        "checkpoint-vector.json",
        "checkpoint-in-progress-vector.json",
        "checkpoint-cross-midnight-vector.json",
        "checkpoint-multi-destination-vector.json",
    ] {
        let raw = artifact(name);
        assert!(!raw.contains(&b'\n'));
        let parsed: Value = serde_json::from_slice(&raw).unwrap();
        assert_eq!(
            parsed["schema"],
            "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1"
        );
        assert_eq!(parsed["version"], 1);
        assert_eq!(payload_digest(&raw), parsed["payload_sha256"]);
        assert!(
            parsed["direct_hydrology"]["lanes"]
                .as_array()
                .is_some_and(|v| !v.is_empty())
        )
    }
    assert_eq!(
        value("checkpoint-in-progress-vector.json")["phase"]["next_interval_index"],
        24
    );
    assert_eq!(
        value("checkpoint-vector.json")["phase"]["kind"],
        "between_days"
    );
    assert_eq!(
        value("checkpoint-in-progress-vector.json")["phase"]["kind"],
        "in_progress_day"
    );
    assert_eq!(
        value("checkpoint-multi-destination-vector.json")["direct_hydrology"]["lanes"]
            .as_array()
            .unwrap()
            .len(),
        3
    )
}

#[test]
fn structural_representation_poisons_are_executable() {
    let raw = artifact("checkpoint-in-progress-vector.json");
    let parsed: Value = serde_json::from_slice(&raw).unwrap();
    let mut missing = parsed.clone();
    missing.as_object_mut().unwrap().remove("phase");
    assert_ne!(serde_json::to_vec(&missing).unwrap(), raw);
    let mut extra = parsed.clone();
    extra["unexpected"] = Value::Bool(true);
    assert_ne!(serde_json::to_vec(&extra).unwrap(), raw);
    let mut wrong = parsed;
    wrong["version"] = Value::from(2);
    assert_ne!(wrong["version"], 1);
    let mut whitespace = raw;
    whitespace.insert(1, b' ');
    assert_ne!(
        serde_json::to_vec(&serde_json::from_slice::<Value>(&whitespace).unwrap()).unwrap(),
        whitespace
    )
}

#[test]
fn generated_metadata_ledger_and_poison_inventory_are_complete() {
    let ledger = fs::read_to_string(
        root()
            .join(PACKAGE)
            .join("direct-run-frame-field-classification.md"),
    )
    .unwrap();
    for owner in [
        "DirectRunFrame",
        "DirectLaneFrame",
        "DirectWaterState",
        "DirectTransferBuffers",
        "DirectLaneTransferLedger",
        "DirectRunTransferDownstreamOperands",
        "DirectSubsurfaceLayerState",
        "DirectEvapotranspirationStageState",
        "DirectGrowthStateSurface",
        "DirectWinterColumnState",
        "DirectSnowRuntimeCarry",
        "DirectFrostRuntimeCarry",
        "DirectErosionDownstreamOperands",
        "DirectErosionInflowIntake",
        "DirectErosionRuntimeCarry",
        "DirectGroundwaterRunState",
        "DirectSurfaceLiquidOwnedState",
    ] {
        assert!(ledger.contains(&format!("## `{owner}`")), "missing {owner}")
    }
    let metadata =
        fs::read_to_string(root().join(PACKAGE).join("generated-field-metadata.json")).unwrap();
    for term in [
        "source_operands",
        "operation",
        "comparison",
        "mismatch_poison",
    ] {
        assert!(metadata.contains(term))
    }
    let poison = fs::read_to_string(root().join(PACKAGE).join("poison-matrix.md")).unwrap();
    for category in [
        "schema",
        "unsupported_version",
        "noncanonical_bytes",
        "payload_digest",
        "missing_field",
        "extra_field",
        "reordered_field",
        "duplicate_field",
        "run_identity",
        "topology_identity",
        "configuration_identity",
        "owner_identity",
        "transaction_lineage",
        "scheduler_position",
        "provider_cursor",
        "gsi_receipt",
        "heterogeneous_lane_gsi_receipt",
        "forcing_receipt_cardinality",
        "forcing_receipt_order",
        "forcing_receipt_digest",
        "v10_v9_projection",
        "lse_v2_v1_projection",
        "owner_validation",
        "unsupported_laned_active",
        "canonical_order",
        "owner_omission",
        "child4_retained_liquid",
        "groundwater_total_area",
        "erosion_publication",
        "live_bytes_unchanged",
    ] {
        assert!(poison.contains(category), "missing {category}")
    }
}

#[test]
fn manifest_binds_every_schema_and_vector_byte_for_byte() {
    let manifest = value("artifact-manifest.json");
    let entries = manifest["artifacts"].as_array().unwrap();
    assert_eq!(entries.len(), 5);
    for entry in entries {
        let path = entry["path"].as_str().unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(artifact(path))),
            entry["sha256"].as_str().unwrap()
        )
    }
}
