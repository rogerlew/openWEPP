use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const AUTHORITY: &str = "docs/specifications/direct-hydrology-restart-v1.md";
const PACKAGE: &str =
    "docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/artifacts";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
fn bytes(path: &str) -> Vec<u8> {
    fs::read(root().join(path)).unwrap_or_else(|error| panic!("read {path}: {error}"))
}
fn text(path: &str) -> String {
    String::from_utf8(bytes(path)).expect("UTF-8 artifact")
}
fn artifact(name: &str) -> Vec<u8> {
    bytes(&format!("{PACKAGE}/{name}"))
}
fn schema() -> serde_json::Value {
    serde_json::from_slice(&artifact("checkpoint-schema.json")).expect("checkpoint schema")
}

fn assert_payload_digest(raw: &[u8], value: &serde_json::Value) {
    let marker = b",\"payload_sha256\":";
    let prefix_end = raw
        .windows(marker.len())
        .position(|window| window == marker)
        .expect("final payload digest member");
    let mut prefix = raw[..prefix_end].to_vec();
    prefix.extend_from_slice(b"}");
    assert_eq!(
        format!("{:x}", Sha256::digest(&prefix)),
        value["payload_sha256"].as_str().expect("payload digest")
    );
}

fn validate_semantics(raw: &[u8]) -> serde_json::Value {
    assert_eq!(raw.last(), Some(&b'\n'), "canonical artifact has one LF");
    assert!(!raw[..raw.len() - 1].contains(&b'\n'));
    assert!(!raw.contains(&b' '));
    let value: serde_json::Value = serde_json::from_slice(raw).expect("strict JSON syntax");
    jsonschema::draft202012::new(&schema())
        .expect("compile schema")
        .validate(&value)
        .expect("machine schema");
    assert_payload_digest(raw, &value);
    assert_eq!(
        value["schema"],
        "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1"
    );
    assert_eq!(value["version"], 1);
    let phase = &value["phase"];
    match phase["kind"].as_str().expect("phase kind") {
        "between_days" => assert!(phase.get("staged_candidate_owners").is_none()),
        "in_progress_day" => {
            let next = phase["next_interval_index"].as_u64().expect("interval");
            assert!((1..=47).contains(&next));
            let receipts = phase["validated_forcing_day_receipts"]
                .as_array()
                .expect("destination receipts");
            assert!(!receipts.is_empty());
            let mut previous: Option<(&str, &str)> = None;
            for destination in receipts {
                let identity = (
                    destination["ofe_id"].as_str().expect("OFE"),
                    destination["tile_id"].as_str().expect("tile"),
                );
                if let Some(prior) = previous {
                    assert!(prior < identity);
                }
                previous = Some(identity);
                let intervals = destination["intervals"].as_array().expect("intervals");
                assert_eq!(intervals.len(), 48);
                for (index, interval) in intervals.iter().enumerate() {
                    assert_eq!(interval["interval_index"], index);
                }
            }
        }
        other => panic!("unsupported phase {other}"),
    }
    let owner_key = if phase["kind"] == "between_days" {
        "committed_owners"
    } else {
        "committed_beginning_owners"
    };
    let owners = &phase[owner_key];
    assert!(owners.get("surface_liquid_state").is_none());
    assert!(
        owners["direct_hydrology"]
            .get("surface_liquid_owned_state")
            .is_some()
    );
    value
}

#[test]
fn authority_binds_canonical_phase_union_atomicity_and_typed_failures() {
    let authority = text(AUTHORITY);
    for marker in [
        "never serializes a Rust object or `DirectRunFrame` memory layout",
        "HexU128",
        "never has\na third or duplicated owner set",
        "one non-fallible assignment",
        "unsupported_laned_active",
        "exact input\nbytes equal canonical output bytes",
    ] {
        assert!(
            authority.contains(marker),
            "missing authority marker {marker}"
        );
    }
}

#[test]
fn all_real_vectors_satisfy_schema_digest_phase_and_owner_semantics() {
    for name in [
        "checkpoint-vector.json",
        "checkpoint-in-progress-vector.json",
        "checkpoint-cross-midnight-vector.json",
        "checkpoint-multi-destination-vector.json",
    ] {
        let raw = artifact(name);
        let value = validate_semantics(&raw);
        assert!(
            raw.windows(18)
                .any(|window| window == b"0x0000000000000000")
        );
        assert!(
            raw.windows(18)
                .any(|window| window == b"0x8000000000000000")
        );
        assert!(!raw.windows(5).any(|window| window == b"usize"));
        assert!(!raw.windows(6).any(|window| window == b"Debug("));
        assert_eq!(value.as_object().expect("checkpoint").len(), 8);
    }
    let in_progress = validate_semantics(&artifact("checkpoint-in-progress-vector.json"));
    assert_eq!(in_progress["phase"]["next_interval_index"], 24);
    assert_eq!(
        in_progress["phase"]["validated_forcing_day_receipts"]
            .as_array()
            .expect("destinations")
            .len(),
        2
    );
    let cross = validate_semantics(&artifact("checkpoint-cross-midnight-vector.json"));
    assert!(
        !cross["phase"]["ending_provider_cursor"]["pending_carry"]
            .as_array()
            .expect("carry")
            .is_empty()
    );
}

#[test]
fn structural_and_representation_poisons_are_executable() {
    let compiled = jsonschema::draft202012::new(&schema()).expect("compile schema");
    let vector: serde_json::Value =
        serde_json::from_slice(&artifact("checkpoint-in-progress-vector.json")).expect("vector");
    let mut missing = vector.clone();
    missing.as_object_mut().expect("object").remove("phase");
    assert!(compiled.validate(&missing).is_err());
    let mut extra = vector.clone();
    extra["unexpected"] = serde_json::json!(true);
    assert!(compiled.validate(&extra).is_err());
    let mut wrong_version = vector.clone();
    wrong_version["version"] = serde_json::json!(2);
    assert!(compiled.validate(&wrong_version).is_err());
    let mut platform_width = vector.clone();
    platform_width["phase"]["next_interval_index"] = serde_json::json!(4_294_967_296_u64);
    assert!(compiled.validate(&platform_width).is_err());
    for bad in [0, 48] {
        let mut interval = vector.clone();
        interval["phase"]["next_interval_index"] = serde_json::json!(bad);
        assert!(compiled.validate(&interval).is_err());
    }
    let mut wrong_count = vector.clone();
    wrong_count["phase"]["validated_forcing_day_receipts"][0]["intervals"]
        .as_array_mut()
        .expect("intervals")
        .pop();
    assert!(compiled.validate(&wrong_count).is_err());
    let mut native_float = vector;
    native_float["phase"]["staged_candidate_owners"]["direct_hydrology"]["lanes"][0]["water"]["surface_runoff_kg_m2"] =
        serde_json::json!(1.375);
    assert!(compiled.validate(&native_float).is_err());

    let matrix = text(&format!("{PACKAGE}/poison-matrix.md"));
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
    ] {
        assert!(matrix.contains(category), "missing poison {category}");
    }
}

#[test]
fn generated_field_ledger_covers_required_owners_and_special_dispositions() {
    let ledger = text(&format!(
        "{PACKAGE}/direct-run-frame-field-classification.md"
    ));
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
        assert!(ledger.contains(&format!("## `{owner}`")), "missing {owner}");
    }
    for disposition in [
        "phase-plan configuration digest",
        "empty publication scratch",
        "ledger plus topology",
        "bound day-input digest",
        "typed rejection before serialization",
    ] {
        assert!(ledger.contains(disposition), "missing {disposition}");
    }
}

#[test]
fn manifest_binds_every_schema_and_vector_byte_for_byte() {
    let manifest: serde_json::Value =
        serde_json::from_slice(&artifact("artifact-manifest.json")).expect("manifest");
    let entries = manifest["artifacts"].as_array().expect("artifact entries");
    assert_eq!(entries.len(), 5);
    for entry in entries {
        let path = entry["path"].as_str().expect("artifact path");
        assert_eq!(
            format!("{:x}", Sha256::digest(artifact(path))),
            entry["sha256"].as_str().expect("artifact SHA-256")
        );
    }
}
