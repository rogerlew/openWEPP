use std::path::PathBuf;

use openwepp_hillslope_orchestrator::STAGE3_V11_PARENT_SUPPORT_NS;
use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{
    ExactDyadicEnthalpy, Sha256Digest, refuse_soil_thermal_v2_to_v1_downgrade,
};
use openwepp_persisted_restart_v1::{
    ExpectedDirectHydrologyRestartContext, Sha256Hex, from_canonical_bytes, to_canonical_bytes,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::*;

const V1_GOLDEN_SHA256: &str = "e1d9d6164d4fe47a31e29266de12ca3908e3ecd8972efb0b45d1bbf56b890a4b";

fn golden_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/snow_stage3_v11_owner_seed_two_day.json")
}

fn golden_frame() -> DirectRunFrame {
    openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture()
        .owners
        .runtime
        .shadow
        .restart_authority_hydrology_frame()
        .clone()
}

fn golden_matching_frame(seed: &DirectSnowStage3V11ProductionSeedV1) -> DirectRunFrame {
    let source_inputs = golden_frame();
    let committed = seed.day_zero_committed().expect("golden committed owner");
    let surface_configuration = committed
        .surface_liquid_configuration
        .restore()
        .expect("golden surface configuration");
    let day_inputs = source_inputs
        .lanes
        .iter()
        .map(|lane| lane.day_inputs.clone())
        .collect::<Vec<_>>();
    let day_input_digests = committed
        .scientific
        .direct_hydrology
        .lanes
        .iter()
        .map(|lane| lane.day_inputs_sha256.clone())
        .collect::<Vec<_>>();
    committed
        .scientific
        .direct_hydrology
        .restore(&ExpectedDirectHydrologyRestartContext {
            phase_plan: &source_inputs.phase_plan,
            phase_plan_sha256: &committed.scientific.direct_hydrology.phase_plan_sha256,
            day_inputs: &day_inputs,
            day_input_digests: &day_input_digests,
            surface_liquid_configuration: &surface_configuration,
        })
        .expect("golden live frame restored from its sealed V1 sidecar")
}

fn assert_no_diagnostic_keys(value: &Value) {
    match value {
        Value::Object(fields) => {
            for (key, nested) in fields {
                let key = key.to_ascii_lowercase();
                assert!(
                    ![
                        "diagnostic",
                        "microstep",
                        "iteration",
                        "solver",
                        "rejection",
                    ]
                    .iter()
                    .any(|forbidden| key.contains(forbidden)),
                    "production V2 owner persisted diagnostic key {key}"
                );
                assert_no_diagnostic_keys(nested);
            }
        }
        Value::Array(values) => values.iter().for_each(assert_no_diagnostic_keys),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

#[test]
fn unchanged_v1_golden_bootstraps_one_zero_carry_v2_resident() {
    let bytes = fs::read(golden_path()).expect("V1 golden bytes");
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), V1_GOLDEN_SHA256);
    let artifact: DirectSnowStage3V11ProductionSeedArtifactV1 =
        from_canonical_bytes(&bytes).expect("strict V1 golden artifact");
    assert_eq!(artifact.schema, SCHEMA);
    assert_eq!(artifact.version, VERSION);
    assert_eq!(
        to_canonical_bytes(&artifact).expect("canonical V1 re-encoding"),
        bytes,
        "V2 bootstrap must not change the admitted V1 wire bytes"
    );

    let seed = DirectSnowStage3V11ProductionSeedV1::load_required(Some(&golden_path()))
        .expect("unchanged V1 golden sidecar admission");
    let mut frame = golden_matching_frame(&seed);
    let committed = seed.day_zero_committed().expect("day-zero V1 owner");
    let v1_restart = &committed.scientific.soil_thermal;
    let owner_id = ResourceOwnerId::try_new(v1_restart.owner_id.clone()).expect("owner ID");
    let configuration_sha256 =
        Sha256Digest::try_new(v1_restart.configuration_sha256.as_str().to_owned())
            .expect("configuration digest");
    let v1_owner = v1_restart
        .restore(&owner_id, &configuration_sha256)
        .expect("sealed V1 owner");

    seed.bootstrap(&mut frame)
        .expect("checked V1-to-V2 production bootstrap");
    let consumer = &frame
        .snow_stage3_v11_attachment
        .as_ref()
        .expect("installed Stage-3 attachment")
        .committed
        .real_consumer;
    assert!(
        consumer.restart_authority_soil_thermal().is_err(),
        "the V2 host must not expose or retain a V1 soil owner"
    );
    let resident = consumer.soil_thermal_v2().expect("sole native V2 resident");
    assert!(resident.latest_accepted().is_none());
    assert!(resident.receipt_free_seals().is_some());
    let v2_owner = resident.owner();
    assert_eq!(v2_owner.parent_v1_state_sha256, v1_owner.state_sha256);
    assert_eq!(
        v2_owner.receipt_chain_sha256.as_str(),
        v1_restart.restart_payload_sha256.as_str()
    );
    assert_eq!(v2_owner.transaction_id, TransactionId(41));
    assert_eq!(
        v2_owner.expected_predecessor_transaction_id,
        v1_owner.last_accepted_transaction_id
    );
    assert_eq!(v2_owner.support_start_ns, 0);
    assert_eq!(v2_owner.support_end_ns, STAGE3_V11_PARENT_SUPPORT_NS);
    assert_eq!(v2_owner.state.ofes.len(), v1_owner.ofes.len());
    for (v2_ofe, v1_ofe) in v2_owner.state.ofes.iter().zip(&v1_owner.ofes) {
        assert_eq!(v2_ofe.ofe_id, v1_ofe.ofe_id);
        assert_eq!(v2_ofe.ordered_layers.len(), v1_ofe.ordered_layers.len());
        for (v2_layer, v1_layer) in v2_ofe.ordered_layers.iter().zip(&v1_ofe.ordered_layers) {
            assert_eq!(v2_layer.layer_id, v1_layer.layer_id);
            assert_eq!(
                v2_layer.temperature_k.to_bits(),
                v1_layer.temperature_k.to_bits()
            );
            assert_eq!(
                v2_layer.enthalpy_hi_j_m2_ofe_ground.to_bits(),
                v1_layer.enthalpy_j_m2_ofe_ground.to_bits()
            );
            assert_eq!(v2_layer.enthalpy_carry, ExactDyadicEnthalpy::zero());
        }
    }

    let resident_bytes = consumer
        .soil_thermal_resident()
        .canonical_active_owner_bytes()
        .expect("canonical active V2 resident");
    let resident_text = std::str::from_utf8(&resident_bytes).expect("V2 resident JSON");
    assert!(resident_text.contains("OPENWEPP_DIRECT_V10_SOIL_THERMAL_RESIDENT_V2"));
    assert!(!resident_text.contains("snapshot_sha256"));
    let resident_json: Value = serde_json::from_slice(&resident_bytes).expect("V2 resident JSON");
    assert_no_diagnostic_keys(&resident_json);
}

#[test]
fn v2_bootstrap_refuses_mixed_configuration_and_truncated_topology() {
    let frame = golden_frame();
    let seed = explicit_repository_test_seed(&frame, None)
        .expect("V1 seed authored from the live test owner");
    let committed = seed.day_zero_committed().expect("day-zero V1 owner");
    let soil_configuration = &seed.artifact.lse_configuration.soil_thermal_configuration;
    let v1_owner = committed
        .scientific
        .soil_thermal
        .restore(
            &soil_configuration.owner_id,
            &soil_configuration.configuration_sha256,
        )
        .expect("sealed V1 owner");
    let receipt_chain_sha256 = Sha256Digest::try_new(
        committed
            .scientific
            .soil_thermal
            .restart_payload_sha256
            .as_str()
            .to_owned(),
    )
    .expect("receipt-chain digest");
    let transaction_id = seed
        .artifact
        .support_static_authority
        .interval_template
        .lse_forcing
        .transaction_id;
    let run_id = committed.static_forcing_configuration.run_id.clone();

    let mut mixed_configuration = seed.artifact.lse_configuration.clone();
    mixed_configuration.soil_thermal_configuration.owner_id =
        ResourceOwnerId::try_new("mixed-soil-owner").expect("poison owner");
    assert!(
        bootstrap_soil_thermal_v2(
            &v1_owner,
            &mixed_configuration,
            frame.lanes.len(),
            transaction_id,
            &run_id,
            receipt_chain_sha256.clone(),
        )
        .is_err()
    );

    let mut truncated_topology = seed.artifact.lse_configuration.clone();
    truncated_topology.ofes[0].soil_interface_layers.pop();
    assert!(
        bootstrap_soil_thermal_v2(
            &v1_owner,
            &truncated_topology,
            frame.lanes.len(),
            transaction_id,
            &run_id,
            receipt_chain_sha256,
        )
        .is_err()
    );
}

#[test]
fn v1_admission_refuses_wrong_tag_and_digest() {
    let bytes = fs::read(golden_path()).expect("V1 golden bytes");
    let artifact: DirectSnowStage3V11ProductionSeedArtifactV1 =
        from_canonical_bytes(&bytes).expect("strict V1 golden artifact");

    let mut wrong_tag = artifact.clone();
    wrong_tag.schema = "OPENWEPP_SNOW_STAGE3_V11_PRODUCTION_SEED_V2".to_owned();
    assert!(
        DirectSnowStage3V11ProductionSeedV1 {
            artifact: wrong_tag,
        }
        .validate_envelope()
        .is_err()
    );

    let mut wrong_digest = artifact;
    wrong_digest.checkpoint.payload_sha256 =
        Sha256Hex::try_new("0".repeat(64)).expect("poison digest");
    assert!(
        DirectSnowStage3V11ProductionSeedV1 {
            artifact: wrong_digest,
        }
        .validate_envelope()
        .is_err()
    );
}

#[test]
fn v2_bootstrap_refuses_replay_and_every_downgrade() {
    let mut accepted_frame = golden_frame();
    let accepted_seed = explicit_repository_test_seed(&accepted_frame, None)
        .expect("V1 seed authored from the live test owner");
    accepted_seed
        .bootstrap(&mut accepted_frame)
        .expect("single checked bootstrap");
    let accepted_state = &accepted_frame
        .snow_stage3_v11_attachment
        .as_ref()
        .expect("installed attachment")
        .committed
        .real_consumer
        .soil_thermal_v2()
        .expect("native V2 resident")
        .owner()
        .state;
    assert!(
        refuse_soil_thermal_v2_to_v1_downgrade(accepted_state).is_err(),
        "zero carry does not authorize a V2-to-V1 route"
    );

    let mut frame = golden_frame();
    let mut seed = explicit_repository_test_seed(&frame, None)
        .expect("V1 seed authored from the live test owner");
    let predecessor = seed
        .day_zero_committed()
        .expect("day-zero V1 owner")
        .scientific
        .soil_thermal
        .last_accepted_transaction_id
        .as_ref()
        .expect("golden predecessor transaction")
        .to_u128();
    seed.artifact
        .support_static_authority
        .interval_template
        .lse_forcing
        .transaction_id = TransactionId(predecessor);
    assert!(seed.bootstrap(&mut frame).is_err());
    assert!(
        frame.snow_stage3_v11_attachment.is_none(),
        "a replay refusal must not install any resident"
    );
}
