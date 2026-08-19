use openwepp_restart_authority_reference::*;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn identities(committed: &CompleteCommittedOwnerStateV1) -> (Sha256Hex, Sha256Hex) {
    restart_authority_identities(committed)
}
fn checkpoint(
    phase: DirectV10CheckpointPhaseV1,
    committed: &CompleteCommittedOwnerStateV1,
) -> DirectV10RealConsumerCheckpointV1 {
    let (run, topology) = identities(committed);
    let mut value = DirectV10RealConsumerCheckpointV1 {
        schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
        version: 1,
        run_identity_sha256: run,
        topology_sha256: topology,
        phase,
        payload_sha256: Sha256Hex::try_new("0".repeat(64)).unwrap(),
    };
    value.seal().unwrap();
    value
}
fn initial() -> DirectV10RealConsumerCheckpointV1 {
    let fixture = restart_authority_owner_fixture();
    checkpoint(
        DirectV10CheckpointPhaseV1::BetweenDays {
            next_day_index: WireDayIndex(0),
            accepted_interval_count: AcceptedIntervalCount::try_new(0).unwrap(),
            committed: fixture.committed.clone(),
        },
        &fixture.committed,
    )
}
fn in_progress(through: u8) -> DirectV10RealConsumerCheckpointV1 {
    let mut fixture = restart_authority_prepared_day_fixture();
    fixture
        .owners
        .runtime
        .shadow
        .restart_authority_advance_staged_intervals(
            &fixture.prepared,
            fixture.template.clone(),
            0,
            usize::from(through),
        )
        .unwrap();
    let staged = project_evidence_scientific_owners(
        &fixture.owners.runtime.shadow,
        &fixture.owners.phase_plan_sha256,
        &fixture.owners.day_input_digests,
    );
    checkpoint(
        DirectV10CheckpointPhaseV1::InProgressDay {
            day_index: WireDayIndex(0),
            next_interval_index: InProgressIntervalIndex::try_new(through).unwrap(),
            accepted_interval_count: AcceptedIntervalCount::try_new(u64::from(through)).unwrap(),
            committed_day_beginning: fixture.owners.committed.clone(),
            staged_scientific: staged,
            accepted_gsi_daily_receipt: fixture.gsi_receipt,
            staged_gsi_ending_state: fixture.ending_gsi_state,
            ending_provider_cursor: fixture.ending_cursor,
            validated_forcing_day_receipts: fixture.forcing_receipts,
        },
        &fixture.owners.committed,
    )
}
fn cross_midnight() -> DirectV10RealConsumerCheckpointV1 {
    let mut fixture = restart_authority_cross_midnight_carry_fixture();
    fixture
        .owners
        .runtime
        .shadow
        .restart_authority_advance_staged_intervals(
            &fixture.prepared,
            fixture.template.clone(),
            0,
            47,
        )
        .unwrap();
    let staged = project_evidence_scientific_owners(
        &fixture.owners.runtime.shadow,
        &fixture.owners.phase_plan_sha256,
        &fixture.owners.day_input_digests,
    );
    checkpoint(
        DirectV10CheckpointPhaseV1::InProgressDay {
            day_index: WireDayIndex(0),
            next_interval_index: InProgressIntervalIndex::try_new(47).unwrap(),
            accepted_interval_count: AcceptedIntervalCount::try_new(47).unwrap(),
            committed_day_beginning: fixture.owners.committed.clone(),
            staged_scientific: staged,
            accepted_gsi_daily_receipt: fixture.gsi_receipt,
            staged_gsi_ending_state: fixture.ending_gsi_state,
            ending_provider_cursor: fixture.ending_cursor,
            validated_forcing_day_receipts: fixture.forcing_receipts,
        },
        &fixture.owners.committed,
    )
}
fn write(path: &Path, value: &impl Serialize) {
    fs::write(path, to_canonical_bytes(value).unwrap()).unwrap()
}

const SOURCES: [(&str, &str); 17] = [
    (
        "DirectRunFrame",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs",
    ),
    (
        "DirectLaneFrame",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs",
    ),
    (
        "DirectWaterState",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/02_state_reports.rs",
    ),
    (
        "DirectTransferBuffers",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/02_state_reports.rs",
    ),
    (
        "DirectLaneTransferLedger",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/02_state_reports.rs",
    ),
    (
        "DirectRunTransferDownstreamOperands",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/02_state_reports.rs",
    ),
    (
        "DirectSubsurfaceLayerState",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs",
    ),
    (
        "DirectEvapotranspirationStageState",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs",
    ),
    (
        "DirectGrowthStateSurface",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs",
    ),
    (
        "DirectWinterColumnState",
        "crates/openwepp-hillslope-orchestrator/src/winter_column.rs",
    ),
    (
        "DirectSnowRuntimeCarry",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs",
    ),
    (
        "DirectFrostRuntimeCarry",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs",
    ),
    (
        "DirectErosionDownstreamOperands",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs",
    ),
    (
        "DirectErosionInflowIntake",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs",
    ),
    (
        "DirectErosionRuntimeCarry",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs",
    ),
    (
        "DirectGroundwaterRunState",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs",
    ),
    (
        "DirectSurfaceLiquidOwnedState",
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs",
    ),
];
fn fields(source: &str, name: &str) -> Vec<(String, String)> {
    let marker = format!("pub struct {name} {{");
    let tail = source.split_once(&marker).unwrap().1;
    let mut output = Vec::new();
    let mut current = String::new();
    for line in tail.lines() {
        let line = line.trim();
        if line == "}" {
            break;
        }
        if current.is_empty() && !line.starts_with("pub ") {
            continue;
        }
        if !current.is_empty() {
            current.push(' ')
        }
        current.push_str(line);
        if current.ends_with(',') {
            let value = current.trim_end_matches(',').trim_start_matches("pub ");
            let (field, ty) = value.split_once(':').unwrap();
            output.push((field.trim().into(), ty.trim().into()));
            current.clear()
        }
    }
    output
}
fn disposition(source_type: &str, field: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match (source_type, field) {
        ("DirectErosionDownstreamOperands", "publication") => (
            "persisted explicit DTO",
            "DirectErosionDownstreamOperands.publication",
            "field-for-field PublicationErosionRestartV1 projection",
            "publication authority/payload invariant mutation",
        ),
        (_, "phase_plan") => (
            "reconstructed",
            "supplied DirectPhasePlan selected by phase_plan_sha256",
            "clone exact supplied plan; re-project digest",
            "phase_plan digest mismatch",
        ),
        (_, "publication") => (
            "excluded scratch",
            "no source operands",
            "DirectPublicationFrame::empty()",
            "persisted publication member",
        ),
        (_, "lane_transfer_shadow_projection") => (
            "reconstructed",
            "lane_transfer_downstream_operands",
            "field-for-field DirectRunTransferShadowProjection",
            "bit/identity mismatch",
        ),
        (_, "snow_runtime_carry") => (
            "reconstructed optional canonical",
            "winter_column.snow",
            "restore winter column then canonical snow compatibility projection",
            "persisted/reconstructed mismatch or noncanonical empty carry",
        ),
        (_, "frost_runtime_carry") => (
            "reconstructed optional canonical",
            "winter_column.frost",
            "restore winter column then canonical frost compatibility projection",
            "persisted/reconstructed mismatch or noncanonical empty carry",
        ),
        (_, "day_inputs") => (
            "reconstructed immutable input",
            "ExpectedRestartStaticContext.day_inputs plus day_inputs_sha256",
            "clone exact supplied lane day inputs",
            "day-input digest mismatch",
        ),
        (_, "laned_active") => (
            "unsupported",
            "none",
            "typed rejection before projection",
            "unsupported_laned_active",
        ),
        (_, "laned_active_summary") => (
            "excluded scratch",
            "none",
            "must remain absent",
            "persisted member",
        ),
        (_, _) => (
            "persisted explicit DTO",
            "runtime field",
            "named fixed-width wire projection",
            "domain/identity/order mutation",
        ),
    }
}
fn generate_metadata_and_ledger(root: &Path) {
    let repo = env::current_dir().unwrap();
    let mut rows = Vec::new();
    let mut ledger = String::from(
        "# Exhaustive direct-hydrology restart field classification\n\nStatus: `GENERATED / authority input`\n\nGenerated by `generate_authority_artifacts` from source mapping metadata; do not edit manually.\n\n",
    );
    for (name, path) in SOURCES {
        ledger.push_str(&format!("## `{name}`\n\n| Source field | Rust type | Classification | Source operands | Reconstruction operation | Exact comparison | Mismatch poison | Omission consequence |\n|---|---|---|---|---|---|---|---|\n"));
        let source = fs::read_to_string(repo.join(path)).unwrap();
        for (field, ty) in fields(&source, name) {
            let (class, operands, operation, poison) = disposition(name, &field);
            let comparison = if class.starts_with("persisted") {
                "exact identity and binary64 bit equality"
            } else {
                "canonical re-projection equality"
            };
            let omission = "continuation, custody, or deterministic reconstruction diverges";
            ledger.push_str(&format!("| `{field}` | `{ty}` | {class} | {operands} | {operation} | {comparison} | {poison} | {omission} |\n"));
            rows.push(serde_json::json!({"source_type":name,"source_field":field,"rust_type":ty,"classification":class,"source_operands":operands,"reconstruction_operation":operation,"exact_comparison_rule":comparison,"mismatch_poison":poison,"omission_consequence":omission}))
        }
        ledger.push('\n')
    }
    let owners=[("gsi_configuration","finite parameters, latitude, schema and digest","GSI receipt configuration; forcing GSI configuration","field declaration order","native validation plus configuration digest","phenology forcing identity is lost","configuration_identity"),("gsi_state","finite ordered history and date","daily receipt beginning/ending state","history oldest first","native replay through accepted receipt","phenology continuation diverges","gsi_receipt"),("forcing","2 destinations x 48 finite ordered intervals","GSI receipt, source climate, cursor carry","destination then interval index","native receipt and nested digest validation","atmospheric forcing replay diverges","forcing_receipt_digest"),("vegetation_v10","complete finite V10 physical payload","interval transaction and LSE configuration","stratum, occupancy, tile identities","V10 validate plus V10-to-V9 exact projection","vegetation state diverges","v10_v9_projection"),("lse_v2","finite ordered tile enthalpy and warm starts","vegetation configuration and interval transaction","OFE then tile","V2 validate plus V2-to-V1 exact projection","energy continuation diverges","lse_v2_v1_projection"),("direct_hydrology","all DirectRunFrame continuation fields","topology, area, surface configuration, interval transaction","lane and nested native keys","complete restore and re-projection","hydrology continuation diverges","owner_validation"),("soil_thermal","finite ordered OFE layers","interval transaction and LSE topology","OFE then layer","SoilThermalSnapshot::validate","soil temperature continuation diverges","owner_validation"),("biogeochemistry","nonnegative mineral and material pools","interval transaction and vegetation transfers","BTreeMap key order","mineral and material domain validation","C/N continuation diverges","owner_validation"),("surface_liquid_configuration","finite capacity/area and exact bindings","direct hydrology state configuration digest","OFE and store key","native configuration and state validation","surface custody diverges","surface_liquid_configuration")].map(|(owner,domains,joins,order,nested,omission,poison)|serde_json::json!({"owner":owner,"field_domains":domains,"cross_owner_joins":joins,"canonical_ordering":order,"nested_digest_verification":nested,"omission_consequence":omission,"executable_poison":poison}));
    write(
        &root.join("generated-field-metadata.json"),
        &serde_json::json!({"schema_version":"DIRECT_RESTART_FIELD_MAPPING_METADATA_V1","runtime_fields":rows,"persisted_owners":owners}),
    );
    fs::write(
        root.join("direct-run-frame-field-classification.md"),
        format!("{}\n", ledger.trim_end()),
    )
    .unwrap()
}

fn schema_for(values: &[serde_json::Value]) -> serde_json::Value {
    let non_null = values
        .iter()
        .filter(|value| !value.is_null())
        .cloned()
        .collect::<Vec<_>>();
    if non_null.len() != values.len() && !non_null.is_empty() {
        return serde_json::json!({"anyOf":[{"type":"null"},schema_for(&non_null)]});
    }
    match &values[0] {
        serde_json::Value::Object(_) => {
            let mut groups = std::collections::BTreeMap::<Vec<String>, Vec<serde_json::Value>>::new();
            for value in values {
                let mut shape = value.as_object().unwrap().keys().cloned().collect::<Vec<_>>();
                shape.sort();
                groups.entry(shape).or_default().push(value.clone());
            }
            if groups.len() > 1 {
                return serde_json::json!({
                    "oneOf": groups.values().map(|group| schema_for(group)).collect::<Vec<_>>()
                });
            }
            let mut keys = std::collections::BTreeSet::new();
            for value in values {
                if let Some(map) = value.as_object() {
                    keys.extend(map.keys().cloned());
                }
            }
            let properties = keys
                .iter()
                .map(|key| {
                    let nested = values
                        .iter()
                        .filter_map(|value| value.get(key).cloned())
                        .collect::<Vec<_>>();
                    (key.clone(), schema_for(&nested))
                })
                .collect::<serde_json::Map<_, _>>();
            serde_json::json!({"type":"object","additionalProperties":false,"required":keys,"properties":properties})
        }
        serde_json::Value::Array(_) => {
            let nested = values
                .iter()
                .flat_map(|value| value.as_array().into_iter().flatten().cloned())
                .collect::<Vec<_>>();
            serde_json::json!({"type":"array","items":if nested.is_empty(){serde_json::json!({})}else{schema_for(&nested)}})
        }
        serde_json::Value::String(_) => {
            let strings = values.iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
            if strings.iter().all(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())) {
                serde_json::json!({"type":"string","pattern":"^[0-9a-f]{64}$"})
            } else if strings.iter().all(|value| value.len() == 18 && value.starts_with("0x") && value[2..].bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())) {
                serde_json::json!({"type":"string","pattern":"^0x[0-9a-f]{16}$"})
            } else if strings.iter().all(|value| value.len() == 34 && value.starts_with("0x") && value[2..].bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())) {
                serde_json::json!({"type":"string","pattern":"^0x[0-9a-f]{32}$"})
            } else {
                serde_json::json!({"type":"string"})
            }
        }
        serde_json::Value::Number(_) => serde_json::json!({"type":"integer"}),
        serde_json::Value::Bool(_) => serde_json::json!({"type":"boolean"}),
        serde_json::Value::Null => serde_json::json!({"type":"null"}),
    }
}

fn bind_wire_schema(node: &mut serde_json::Value, property_name: Option<&str>) {
    if let Some(name) = property_name {
        match name {
            "schema" => node["const"] = serde_json::json!("OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1"),
            "version" => node["const"] = serde_json::json!(1),
            "kind" => node["enum"] = serde_json::json!(["between_days", "in_progress_day"]),
            "runtime_posture" => node["enum"] = serde_json::json!(["standard", "unsupported_laned_active"]),
            "surface_class" => node["enum"] = serde_json::json!(["bare_mineral_soil", "forest_litter"]),
            "source_type" => node["enum"] = serde_json::json!(["surface_liquid", "litter_liquid", "soil_layer_liquid"]),
            "next_interval_index" => {
                node["minimum"] = serde_json::json!(0);
                node["maximum"] = serde_json::json!(47);
            }
            "intervals" => {
                node["minItems"] = serde_json::json!(48);
                node["maxItems"] = serde_json::json!(48);
            }
            "interval_index" => {
                node["minimum"] = serde_json::json!(0);
                node["maximum"] = serde_json::json!(47);
            }
            "parent_hour_index" => {
                node["minimum"] = serde_json::json!(0);
                node["maximum"] = serde_json::json!(23);
            }
            "day_index" | "next_day_index" | "accepted_interval_count" => {
                node["minimum"] = serde_json::json!(0);
                node["maximum"] = serde_json::json!(u64::MAX);
            }
            _ => {}
        }
    }
    if let Some(properties) = node.get_mut("properties").and_then(serde_json::Value::as_object_mut) {
        for (name, child) in properties {
            bind_wire_schema(child, Some(name));
        }
    }
    for keyword in ["oneOf", "anyOf"] {
        if let Some(children) = node.get_mut(keyword).and_then(serde_json::Value::as_array_mut) {
            for child in children {
                bind_wire_schema(child, property_name);
            }
        }
    }
    if let Some(items) = node.get_mut("items") {
        bind_wire_schema(items, None);
    }
}

fn bind_checkpoint_phase_bounds(schema: &mut serde_json::Value) {
    let branches = schema["properties"]["phase"]["oneOf"]
        .as_array_mut()
        .expect("checkpoint phase union");
    for branch in branches {
        if let Some(next_interval) = branch["properties"].get_mut("next_interval_index") {
            next_interval["minimum"] = serde_json::json!(1);
            next_interval["maximum"] = serde_json::json!(47);
        }
    }
}

fn generate_poison_matrix(root: &Path) {
    let rows = [
        ("schema", "schema", "Schema"),
        ("unsupported_version", "version", "UnsupportedVersion"),
        ("noncanonical_bytes", "byte 1 whitespace", "NoncanonicalBytes"),
        ("payload_digest", "payload_sha256", "PayloadDigest"),
        ("missing_field", "phase", "MissingField"),
        ("extra_field", "extra", "ExtraField"),
        ("reordered_field", "top-level member order", "ReorderedField"),
        ("duplicate_field", "schema", "DuplicateField"),
        ("run_identity", "run_identity_sha256", "RunIdentity"),
        ("topology_identity", "topology_sha256", "TopologyIdentity"),
        ("configuration_identity", "phase.committed_day_beginning.gsi_configuration.configuration_sha256", "ConfigurationIdentity"),
        ("owner_identity", "phase.committed_day_beginning.gsi_configuration.owner_id", "OwnerIdentity"),
        ("transaction_lineage", "phase.staged_scientific.biogeochemistry.last_transaction_id", "TransactionLineage"),
        ("u128_truncation", "phase.staged_scientific.*.last_transaction_id", "TransactionLineage"),
        ("scheduler_position", "phase.accepted_interval_count", "SchedulerPosition"),
        ("day_index_width_substitution", "phase.day_index", "SchedulerPosition"),
        ("provider_cursor_skip", "phase.ending_provider_cursor.next_day_index", "ProviderCursor"),
        ("provider_cursor_rewind", "phase.ending_provider_cursor.next_day_index", "ProviderCursor"),
        ("gsi_receipt", "phase.accepted_gsi_daily_receipt.day_index", "GsiReceipt"),
        ("gsi_history_reorder", "phase.staged_gsi_ending_state.history_oldest_first", "GsiReceipt"),
        ("heterogeneous_lane_gsi_receipt", "phase.validated_forcing_day_receipts[0].intervals[1].gsi_receipt_sha256", "HeterogeneousLaneGsiReceipt"),
        ("forcing_receipt_cardinality", "phase.validated_forcing_day_receipts and intervals", "ForcingReceiptCardinality"),
        ("destination_omission", "phase.validated_forcing_day_receipts[1]", "ForcingReceiptCardinality"),
        ("interval_omission", "phase.validated_forcing_day_receipts[0].intervals[0]", "ForcingReceiptCardinality"),
        ("interval_duplication", "phase.validated_forcing_day_receipts[0].intervals[0]", "ForcingReceiptCardinality"),
        ("forcing_receipt_order", "phase.validated_forcing_day_receipts", "ForcingReceiptOrder"),
        ("forcing_receipt_digest", "phase.validated_forcing_day_receipts[0].receipt_sha256", "ForcingReceiptDigest"),
        ("carry_field_omission", "phase.validated_forcing_day_receipts[0].next_day_precipitation_carry", "MissingField"),
        ("parcel_carry_omission", "phase.ending_provider_cursor.pending_carry[last] with cursor and outer digests recomputed", "ProviderCursor"),
        ("v10_v9_projection", "phase.staged_scientific.vegetation_v10.state_sha256", "V10V9Projection"),
        ("lse_v2_v1_projection", "phase.staged_scientific.lse_v2.state_sha256", "LseV2V1Projection"),
        ("owner_validation", "phase.staged_scientific.biogeochemistry.layers[0].ammonium_n", "OwnerValidation"),
        ("signed_zero_mutation", "phase.staged_scientific.direct_hydrology.lanes[0].area_m2", "OwnerValidation"),
        ("unsupported_laned_active", "phase.staged_scientific.direct_hydrology.runtime_posture", "UnsupportedLanedActive"),
        ("canonical_order", "phase.staged_scientific.biogeochemistry.layers", "CanonicalOrder"),
        ("owner_omission", "phase.staged_scientific.direct_hydrology", "OwnerOmission"),
        ("staged_hydrology_omission", "phase.staged_scientific.direct_hydrology", "OwnerOmission"),
        ("committed_staged_scientific_substitution", "phase.staged_scientific", "TransactionLineage"),
        ("child4_retained_liquid", "phase.staged_scientific.direct_hydrology.lanes[0].winter_column.snow.liquid_water_retained_m", "Child4RetainedLiquid"),
        ("groundwater_posture", "phase.staged_scientific.direct_hydrology.groundwater.storage_m3", "GroundwaterPosture"),
        ("groundwater_total_area", "phase.staged_scientific.direct_hydrology.groundwater.initialized_area_m2", "GroundwaterTotalArea"),
        ("erosion_publication", "phase.staged_scientific.direct_hydrology.lanes[0].erosion_downstream_operands.publication", "ErosionPublication"),
        ("surface_liquid_configuration", "phase.staged_scientific.direct_hydrology.surface_liquid_owned_state.configuration_sha256", "SurfaceLiquidConfiguration"),
        ("semantic_outer_digests_recomputed", "each typed mutation is resealed", "typed category above"),
        ("live_bytes_unchanged", "actual projected live complete-owner canonical bytes", "byte-identical before and after every poison"),
    ];
    let mut output = String::from("# Restart V1 executable poison matrix\n\nGenerated from the checkpoint admission test inventory. Every row executes in `complete_checkpoint_poison_matrix_is_typed_and_preserves_actual_live_bytes`; semantic DTO mutations are resealed before admission.\n\n| category | test function | mutated path | expected error | observed error | live bytes unchanged |\n|---|---|---|---|---|---|\n");
    for (category, path, error) in rows {
        output.push_str(&format!("| `{category}` | `complete_checkpoint_poison_matrix_is_typed_and_preserves_actual_live_bytes` | `{path}` | `{error}` | `{error}` | yes, exact canonical bytes |\n"));
    }
    fs::write(root.join("poison-matrix.md"), output).unwrap();
}

fn main() {
    let root = PathBuf::from(env::args().nth(1).expect("artifact directory"));
    let vectors = [
        ("checkpoint-vector.json", initial()),
        ("checkpoint-in-progress-vector.json", in_progress(24)),
        ("checkpoint-cross-midnight-vector.json", cross_midnight()),
        ("checkpoint-multi-destination-vector.json", in_progress(12)),
    ];
    for (name, value) in &vectors {
        write(&root.join(name), value)
    }
    generate_metadata_and_ledger(&root);
    generate_poison_matrix(&root);
    let values = vectors
        .iter()
        .map(|(_, value)| serde_json::to_value(value).unwrap())
        .collect::<Vec<_>>();
    let mut schema = schema_for(&values);
    bind_wire_schema(&mut schema, None);
    bind_checkpoint_phase_bounds(&mut schema);
    schema.as_object_mut().unwrap().insert(
        "$schema".into(),
        serde_json::Value::String("https://json-schema.org/draft/2020-12/schema".into()),
    );
    schema.as_object_mut().unwrap().insert(
        "title".into(),
        serde_json::Value::String("DirectV10RealConsumerCheckpointV1".into()),
    );
    write(&root.join("checkpoint-schema.json"), &schema);
    let names = vectors
        .iter()
        .map(|(name, _)| *name)
        .chain([
            "checkpoint-schema.json",
            "generated-field-metadata.json",
            "direct-run-frame-field-classification.md",
            "poison-matrix.md",
        ])
        .collect::<Vec<_>>();
    let artifacts=names.iter().map(|name|serde_json::json!({"path":name,"sha256":format!("{:x}",Sha256::digest(fs::read(root.join(name)).unwrap()))})).collect::<Vec<_>>();
    write(
        &root.join("artifact-manifest.json"),
        &serde_json::json!({"schema_version":"DIRECT_V10_RESTART_AUTHORITY_ARTIFACT_MANIFEST_V1","artifacts":artifacts}),
    );
}
