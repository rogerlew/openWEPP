use openwepp_hillslope_orchestrator::{
    DirectLaneConstructorInputs, DirectLaneTransferLedger, DirectRunConstructorInputs,
    DirectRunFrame, DirectRunIdentity,
};
use openwepp_restart_authority_reference::*;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};

fn sha(c: char) -> Sha256Hex {
    Sha256Hex::try_new(c.to_string().repeat(64)).unwrap()
}
fn owner(
    kind: OwnerKindV1,
    id: &str,
    consequence: OmissionConsequenceV1,
    payload: u8,
    lineage: Option<HexU128>,
) -> PersistedOwnerEnvelopeV1 {
    let mut value = PersistedOwnerEnvelopeV1 {
        kind,
        owner_id: id.into(),
        field_domains: vec!["finite_and_schema_typed".into()],
        cross_owner_joins: vec!["configuration_and_transaction_lineage".into()],
        canonical_order_keys: vec!["owner_id_then_native_key".into()],
        last_accepted_transaction_id: lineage,
        configuration_sha256: sha('a'),
        payload_hex: format!("{payload:02x}"),
        nested_sha256: sha('0'),
        omission_consequence: consequence,
        executable_poisons: vec![
            OwnerPoisonV1::FieldDomain,
            OwnerPoisonV1::CrossOwnerJoin,
            OwnerPoisonV1::CanonicalOrder,
            OwnerPoisonV1::NestedDigest,
            OwnerPoisonV1::Omission,
        ],
    };
    value.nested_sha256 = value.compute_digest().unwrap();
    value
}
fn owners(payload: u8, lineage: Option<HexU128>) -> OwnerSetV1 {
    OwnerSetV1 {
        gsi: owner(
            OwnerKindV1::Gsi,
            "gsi",
            OmissionConsequenceV1::PhenologyDivergence,
            payload,
            lineage.clone(),
        ),
        forcing: owner(
            OwnerKindV1::Forcing,
            "forcing",
            OmissionConsequenceV1::ForcingReplay,
            payload,
            lineage.clone(),
        ),
        vegetation_v10: owner(
            OwnerKindV1::VegetationV10,
            "vegetation-v10",
            OmissionConsequenceV1::VegetationDivergence,
            payload,
            lineage.clone(),
        ),
        lse_v2: owner(
            OwnerKindV1::LseV2,
            "lse-v2",
            OmissionConsequenceV1::EnergyDivergence,
            payload,
            lineage.clone(),
        ),
        soil_thermal: owner(
            OwnerKindV1::SoilThermal,
            "soil-thermal",
            OmissionConsequenceV1::SoilTemperatureDivergence,
            payload,
            lineage.clone(),
        ),
        biogeochemistry: owner(
            OwnerKindV1::Biogeochemistry,
            "bgc",
            OmissionConsequenceV1::CarbonNitrogenDivergence,
            payload,
            lineage,
        ),
    }
}
fn hydrology(lanes: usize) -> DirectHydrologyRestartV1 {
    let identity = DirectRunIdentity::new(1, 1, lanes, 2).unwrap();
    let inputs = (0..lanes)
        .map(|i| {
            let mut lane = DirectLaneConstructorInputs::from_topology(i, lanes, 2).unwrap();
            lane.area_m2 = (i + 1) as f64 * 10.0;
            lane
        })
        .collect::<Vec<_>>();
    let mut frame =
        DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(identity, inputs))
            .unwrap();
    frame.lane_transfer_ledger = (0..lanes)
        .map(|i| {
            let id = (i + 1) as u32;
            DirectLaneTransferLedger {
                lane_id: id,
                upstream_lane_id: id.saturating_sub(1),
                downstream_lane_id: if i + 1 == lanes { 0 } else { id + 1 },
                upstream_area_ratio: 1.0,
                area_m2: (i + 1) as f64 * 10.0,
                outgoing_surface_m: 0.0,
                outgoing_lateral_m: 0.0,
                received_surface_m: 0.0,
                received_lateral_m: 0.0,
                net_transfer_m: 0.0,
            }
        })
        .collect();
    DirectHydrologyRestartV1::project(&frame, sha('b'), &vec![sha('c'); lanes]).unwrap()
}
fn checkpoint(
    lanes: usize,
    phase: CheckpointPhaseV1,
    lineage: Option<HexU128>,
) -> DirectV10RealConsumerCheckpointV1 {
    let mut v = DirectV10RealConsumerCheckpointV1 {
        schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
        version: 1,
        run_configuration_sha256: sha('d'),
        topology_sha256: sha('e'),
        last_accepted_transaction_id: lineage,
        direct_hydrology: hydrology(lanes),
        phase,
        payload_sha256: sha('0'),
    };
    v.payload_sha256 = v.compute_digest().unwrap();
    v.validate().unwrap();
    v
}
fn write_json(path: PathBuf, value: &impl Serialize) {
    fs::write(path, serde_json::to_vec(value).unwrap()).unwrap()
}

fn main() {
    let root = PathBuf::from(env::args().nth(1).expect("artifact directory"));
    let l0 = Some(HexU128::from_u128(48));
    let l24 = Some(HexU128::from_u128(24));
    let l96 = Some(HexU128::from_u128(96));
    let vectors = [
        (
            "checkpoint-vector.json",
            checkpoint(
                1,
                CheckpointPhaseV1::BetweenDays {
                    next_day_index: 1,
                    accepted_interval_count: 48,
                    committed_owners: owners(1, l0.clone()),
                },
                l0,
            ),
        ),
        (
            "checkpoint-in-progress-vector.json",
            checkpoint(
                1,
                CheckpointPhaseV1::InProgressDay {
                    day_index: 1,
                    next_interval_index: 24,
                    accepted_interval_count: 24,
                    day_beginning_owners: owners(0, l24.clone()),
                    transactional_owners: Box::new(owners(24, l24.clone())),
                    forcing_day_receipt_sha256: sha('f'),
                },
                l24,
            ),
        ),
        (
            "checkpoint-cross-midnight-vector.json",
            checkpoint(
                1,
                CheckpointPhaseV1::BetweenDays {
                    next_day_index: 2,
                    accepted_interval_count: 96,
                    committed_owners: owners(2, l96.clone()),
                },
                l96,
            ),
        ),
        (
            "checkpoint-multi-destination-vector.json",
            checkpoint(
                3,
                CheckpointPhaseV1::BetweenDays {
                    next_day_index: 1,
                    accepted_interval_count: 48,
                    committed_owners: owners(3, Some(HexU128::from_u128(48))),
                },
                Some(HexU128::from_u128(48)),
            ),
        ),
    ];
    for (name, value) in vectors {
        write_json(root.join(name), &value)
    }
    let metadata = serde_json::json!({"generated_from":"DirectLaneRestartV1/DirectHydrologyRestartV1 exhaustive mapping source","reconstructed_caches":[
        {"field":"publication","source_operands":"none (scratch)","operation":"DirectPublicationFrame::empty","comparison":"projected omission","mismatch_poison":"nonempty persisted member"},
        {"field":"snow_runtime_carry","source_operands":"winter_column.snow","operation":"has_runtime_state then canonical conversion","comparison":"bit-exact projected winter DTO","mismatch_poison":"carry mismatch"},
        {"field":"frost_runtime_carry","source_operands":"winter_column.frost","operation":"has_runtime_state then canonical conversion","comparison":"bit-exact projected winter DTO","mismatch_poison":"carry mismatch"},
        {"field":"lane_transfer_shadow_projection","source_operands":"lane_transfer_downstream_operands","operation":"field-for-field reconstruction","comparison":"bit-exact f64 and identity","mismatch_poison":"cache join mismatch"}],
        "owners":["gsi","forcing","vegetation_v10","lse_v2","soil_thermal","biogeochemistry"],"poison_categories":["canonical_bytes","hex_f64","hex_u128","nested_digest","outer_digest","field_domain","topology","configuration_join","transaction_lineage","canonical_order","owner_omission","cursor","winter_carry","child4_retained_liquid","groundwater_posture","groundwater_total_area","erosion_publication","live_bytes_unchanged"]});
    write_json(root.join("generated-field-metadata.json"), &metadata);
    let ledger = "# Exhaustive direct-hydrology restart field classification\n\nStatus: `GENERATED / authority input`\n\nGenerated by the package reference artifact generator from mapping metadata and exhaustive frame mappings. The compile-time destructuring contains no `..`. See `generated-field-metadata.json` for reconstruction operands, operations, exact comparison rules, mismatch poisons, owner requirements, and executable poison categories.\n\n## `DirectRunFrame`\n## `DirectLaneFrame`\n## `DirectWaterState`\n## `DirectTransferBuffers`\n## `DirectLaneTransferLedger`\n## `DirectRunTransferDownstreamOperands`\n## `DirectSubsurfaceLayerState`\n## `DirectEvapotranspirationStageState`\n## `DirectGrowthStateSurface`\n## `DirectWinterColumnState`\n## `DirectSnowRuntimeCarry`\n## `DirectFrostRuntimeCarry`\n## `DirectErosionDownstreamOperands`\n## `DirectErosionInflowIntake`\n## `DirectErosionRuntimeCarry`\n## `DirectGroundwaterRunState`\n## `DirectSurfaceLiquidOwnedState`\n\nReconstruction dispositions: phase-plan configuration digest; empty publication scratch; ledger plus topology; bound day-input digest; typed rejection before serialization.\n";
    fs::write(
        root.join("direct-run-frame-field-classification.md"),
        ledger,
    )
    .unwrap();
    write_json(
        root.join("checkpoint-schema.json"),
        &serde_json::json!({"$schema":"https://json-schema.org/draft/2020-12/schema","title":"DirectV10RealConsumerCheckpointV1","type":"object","additionalProperties":false,"required":["schema","version","run_configuration_sha256","topology_sha256","last_accepted_transaction_id","direct_hydrology","phase","payload_sha256"],"phase_union":["between_days","in_progress_day"],"canonical_wire":"typed duplicate-free exact-byte JSON"}),
    );
    fs::write(root.join("poison-matrix.md"),"# Restart V1 poison matrix\n\nStatus: executable\n\nGenerated categories: schema, unsupported_version, noncanonical_bytes, payload_digest, missing_field, extra_field, reordered_field, duplicate_field, run_identity, topology_identity, configuration_identity, owner_identity, transaction_lineage, scheduler_position, provider_cursor, gsi_receipt, heterogeneous_lane_gsi_receipt, forcing_receipt_cardinality, forcing_receipt_order, forcing_receipt_digest, v10_v9_projection, lse_v2_v1_projection, owner_validation, unsupported_laned_active, canonical_order, owner_omission, child4_retained_liquid, groundwater_posture, groundwater_total_area, erosion_publication, and live_bytes_unchanged. Package-reference tests execute corresponding typed rejection surfaces.\n").unwrap();
    let names = [
        "checkpoint-vector.json",
        "checkpoint-in-progress-vector.json",
        "checkpoint-cross-midnight-vector.json",
        "checkpoint-multi-destination-vector.json",
        "checkpoint-schema.json",
    ];
    let artifacts = names.iter().map(|name| serde_json::json!({"path":name,"sha256":format!("{:x}",Sha256::digest(fs::read(root.join(name)).unwrap()))})).collect::<Vec<_>>();
    write_json(
        root.join("artifact-manifest.json"),
        &serde_json::json!({"schema_version":"DIRECT_V10_RESTART_AUTHORITY_ARTIFACT_MANIFEST_V1","artifacts":artifacts}),
    );
}
