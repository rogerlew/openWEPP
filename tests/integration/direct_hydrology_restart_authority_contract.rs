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
fn restart_evidence_access_is_nondefault_and_has_no_production_selector() {
    let cargo =
        fs::read_to_string(root().join("crates/openwepp-hillslope-orchestrator/Cargo.toml"))
            .unwrap();
    assert!(cargo.contains("default = []"));
    assert!(cargo.contains("restart-authority-evidence = []"));
    let shadow = fs::read_to_string(
        root().join("crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs"),
    )
    .unwrap();
    assert!(
        shadow
            .matches("#[cfg(feature = \"restart-authority-evidence\")]")
            .count()
            >= 10
    );
    for forbidden in [
        "restart_selector",
        "restart_checkpoint_path",
        "resume_from_checkpoint",
    ] {
        assert!(!shadow.contains(forbidden));
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
        let scientific = if parsed["phase"]["kind"] == "between_days" {
            &parsed["phase"]["committed"]["scientific"]
        } else {
            &parsed["phase"]["staged_scientific"]
        };
        assert!(
            scientific["direct_hydrology"]["lanes"]
                .as_array()
                .is_some_and(|v| !v.is_empty())
        );
        for owner in [
            "vegetation_v10",
            "lse_v2",
            "direct_hydrology",
            "soil_thermal",
            "biogeochemistry",
        ] {
            assert!(!scientific[owner].is_null(), "{name} omits {owner}");
        }
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
    let multi = value("checkpoint-multi-destination-vector.json");
    let receipts = multi["phase"]["validated_forcing_day_receipts"]
        .as_array()
        .unwrap();
    assert_eq!(receipts.len(), 2);
    assert!(
        receipts
            .iter()
            .all(|receipt| receipt["intervals"].as_array().unwrap().len() == 48)
    );
}

#[test]
fn poison_inventory_is_bound_to_typed_admission_and_live_byte_evidence() {
    let source = fs::read_to_string(root().join(
        "docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/\
         tools/restart-authority-reference/src/checkpoint.rs",
    ))
    .unwrap();
    assert!(
        source
            .contains("complete_checkpoint_poison_matrix_is_typed_and_preserves_actual_live_bytes")
    );
    assert!(source.contains("admit_checkpoint_into_owner_store_v1(&bytes, &context, &mut live)"));
    assert!(source.contains("let before = to_canonical_bytes(&live).unwrap();"));
    assert!(source.contains("assert_eq!(to_canonical_bytes(&live).unwrap(), before)"));
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
        "DirectFrostControlInputs",
        "DirectFrostThermalInputs",
        "DirectFrostHourlyForcing",
        "DirectEvapotranspirationPmetInputs",
        "DirectEvapotranspirationPmetComputeInputs",
        "DirectWave1ContinuityInputs",
        "DirectWinterFrostPartitionOutcome",
        "DirectFrostLayerCarryProjection",
        "DirectPublicationFrame",
    ] {
        assert!(ledger.contains(&format!("## `{owner}`")), "missing {owner}")
    }
    let publication = ledger.split("## `DirectPublicationFrame`").nth(1).unwrap();
    assert!(
        publication
            .lines()
            .take(15)
            .any(|line| line.contains("excluded scratch"))
    );
    for reconstructed in [
        "DirectWinterFrostPartitionOutcome",
        "DirectFrostLayerCarryProjection",
    ] {
        let section = ledger
            .split(&format!("## `{reconstructed}`"))
            .nth(1)
            .unwrap();
        assert!(
            section
                .lines()
                .take(15)
                .any(|line| line.contains("reconstructed cache"))
        );
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
    ] {
        assert!(poison.contains(category), "missing {category}")
    }
}

#[test]
fn manifest_binds_every_schema_and_vector_byte_for_byte() {
    let manifest = value("artifact-manifest.json");
    let entries = manifest["artifacts"].as_array().unwrap();
    assert_eq!(entries.len(), 8);
    for entry in entries {
        let path = entry["path"].as_str().unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(artifact(path))),
            entry["sha256"].as_str().unwrap()
        )
    }
}

#[test]
fn generated_schema_accepts_every_frozen_vector_and_rejects_wire_bounds() {
    let schema = value("checkpoint-schema.json");
    let validator = jsonschema::validator_for(&schema).unwrap();
    for name in [
        "checkpoint-vector.json",
        "checkpoint-in-progress-vector.json",
        "checkpoint-cross-midnight-vector.json",
        "checkpoint-multi-destination-vector.json",
    ] {
        let candidate = value(name);
        let errors = validator
            .iter_errors(&candidate)
            .map(|error| error.to_string())
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            let phase_errors = schema["properties"]["phase"]["oneOf"]
                .as_array()
                .unwrap()
                .iter()
                .map(|branch| {
                    jsonschema::validator_for(branch)
                        .unwrap()
                        .iter_errors(&candidate["phase"])
                        .map(|error| format!("{} @ {}", error, error.instance_path()))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            panic!("schema rejects {name}: {errors:?}; phase={phase_errors:?}");
        }
    }
    let mut invalid = value("checkpoint-in-progress-vector.json");
    invalid["phase"]["next_interval_index"] = Value::from(48);
    assert!(!validator.is_valid(&invalid));
    invalid = value("checkpoint-in-progress-vector.json");
    invalid["phase"]["next_interval_index"] = Value::from(0);
    assert!(!validator.is_valid(&invalid));
    invalid = value("checkpoint-in-progress-vector.json");
    invalid["phase"]["kind"] = Value::from("invented_phase");
    assert!(!validator.is_valid(&invalid));
    invalid = value("checkpoint-in-progress-vector.json");
    invalid["phase"]["kind"] = Value::from("between_days");
    assert!(!validator.is_valid(&invalid));

    // Topology-bound owner cardinalities are not inferred from the four
    // examples. Typed admission, not the wire schema, validates their joins.
    let mut flexible = value("checkpoint-in-progress-vector.json");
    let lane = flexible["phase"]["staged_scientific"]["direct_hydrology"]["lanes"][0].clone();
    flexible["phase"]["staged_scientific"]["direct_hydrology"]["lanes"]
        .as_array_mut()
        .unwrap()
        .push(lane);
    let forcing = flexible["phase"]["validated_forcing_day_receipts"][0].clone();
    flexible["phase"]["validated_forcing_day_receipts"]
        .as_array_mut()
        .unwrap()
        .push(forcing);
    assert!(validator.is_valid(&flexible));

    let hex = Value::from("0x3ff0000000000000");
    let groundwater =
        &mut flexible["phase"]["staged_scientific"]["direct_hydrology"]["groundwater"];
    groundwater["authority"] = serde_json::json!({
        "authority":"linear_reservoir",
        "initial_storage_depth_m":hex,
        "baseflow_coeff_per_day":"0x3ff0000000000000",
        "deep_seepage_coeff_per_day":"0x3ff0000000000000",
        "baseflow_threshold_area_ha":"0x3ff0000000000000"
    });
    groundwater["initialized_area_m2"] = Value::from("0x4059000000000000");
    assert!(validator.is_valid(&flexible));

    let mut later_day = value("checkpoint-vector.json");
    later_day["phase"]["committed"]["gsi_state"]["history_oldest_first"] =
        serde_json::json!(["0x3ff0000000000000"]);
    later_day["phase"]["committed"]["gsi_state"]["last_date"] =
        serde_json::json!({"year":2026,"ordinal_day":231});
    assert!(validator.is_valid(&later_day));

    let mut legal_optionals = value("checkpoint-in-progress-vector.json");
    legal_optionals["phase"]["staged_scientific"]["direct_hydrology"]["lanes"][0]["erosion_downstream_operands"]
        ["publication"]["peak_runoff_rate_m_s"] = Value::Null;
    legal_optionals["phase"]["staged_scientific"]["direct_hydrology"]["surface_liquid_owned_state"]
        ["continuations"][0]["next_interval_index"] = Value::from(48);
    assert!(validator.is_valid(&legal_optionals));

    let mut typed_optionals = value("checkpoint-in-progress-vector.json");
    let lane = &mut typed_optionals["phase"]["staged_scientific"]["direct_hydrology"]["lanes"][0];
    lane["evapotranspiration_stage_state"] = serde_json::json!({
        "s1_m":"0x0000000000000000","s2_m":"0x0000000000000000",
        "threshold_m":"0x3ff0000000000000","counter":"0x0000000000000000"
    });
    lane["winter_column"]["snow"]["snow_albedo_state"] = serde_json::json!({
        "model":"brock_2000_temperature_age_v1","albedo":"0x3fe0000000000000",
        "accumulated_positive_temperature_c_day":"0x0000000000000000"
    });
    assert!(validator.is_valid(&typed_optionals));
    typed_optionals["phase"]["staged_scientific"]["direct_hydrology"]["lanes"][0]["evapotranspiration_stage_state"] =
        serde_json::json!({});
    assert!(!validator.is_valid(&typed_optionals));
    let mut nested_arrays = value("checkpoint-in-progress-vector.json");
    nested_arrays["phase"]["staged_scientific"]["direct_hydrology"]["lanes"][0]["winter_column"]
        ["snow"]["layers"] = serde_json::json!([{}]);
    assert!(!validator.is_valid(&nested_arrays));
    nested_arrays["phase"]["staged_scientific"]["direct_hydrology"]["lanes"][0]["winter_column"]
        ["snow"]["layers"] = serde_json::json!([{
        "mass_swe_m":"0x0000000000000000","thickness_m":"0x0000000000000000",
        "density_kg_m3":"0x0000000000000000","settle_day_count":"0x0000000000000000",
        "temperature_c":"0x0000000000000000","liquid_water_m":"0x0000000000000000",
        "cold_content_j_m2":"0x0000000000000000","refrozen_liquid_m":"0x0000000000000000"
    }]);
    assert!(validator.is_valid(&nested_arrays));
}
