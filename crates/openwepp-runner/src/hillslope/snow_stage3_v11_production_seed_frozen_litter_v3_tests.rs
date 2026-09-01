use std::path::PathBuf;

use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::FrozenLitterV3Resident;
use openwepp_land_surface_energy::{V3_MODEL_DEFINITION_SHA256, V3_MODEL_VERSION};
use openwepp_persisted_restart_v1::{ExpectedDirectHydrologyRestartContext, Sha256Hex};
use serde_json::Value;

use super::*;

fn successor_golden_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/snow_stage3_v11_owner_seed_frozen_litter_v3.json")
}

fn matching_frame(seed: &DirectSnowStage3V11ProductionSeedV1) -> DirectRunFrame {
    let source = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture()
        .owners
        .runtime
        .shadow
        .restart_authority_hydrology_frame()
        .clone();
    let committed = seed
        .day_zero_committed()
        .expect("successor committed owner");
    let surface_configuration = committed
        .surface_liquid_configuration
        .restore()
        .expect("successor surface configuration");
    let day_inputs = source
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
        .collect::<Vec<Sha256Hex>>();
    committed
        .scientific
        .direct_hydrology
        .restore(&ExpectedDirectHydrologyRestartContext {
            phase_plan: &source.phase_plan,
            phase_plan_sha256: &committed.scientific.direct_hydrology.phase_plan_sha256,
            day_inputs: &day_inputs,
            day_input_digests: &day_input_digests,
            surface_liquid_configuration: &surface_configuration,
        })
        .expect("successor live frame restored from sealed seed")
}

fn installed_resident(frame: &DirectRunFrame) -> &FrozenLitterV3Resident {
    frame
        .snow_stage3_v11_attachment
        .as_ref()
        .expect("Stage-3 attachment")
        .committed
        .real_consumer
        .frozen_litter_v3_resident()
        .expect("native frozen-litter V3 resident")
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
                        "rejection"
                    ]
                    .iter()
                    .any(|forbidden| key.contains(forbidden)),
                    "frozen-litter V3 seed persisted diagnostic key {key}"
                );
                assert_no_diagnostic_keys(nested);
            }
        }
        Value::Array(values) => values.iter().for_each(assert_no_diagnostic_keys),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

#[test]
fn successor_seed_installs_exact_v3_and_zero_ice_surface_owner() {
    let seed = DirectSnowStage3V11ProductionSeedV1::load_required(Some(&successor_golden_path()))
        .expect("successor seed retains strict V1 wire admission");
    let mut frame = matching_frame(&seed);
    seed.bootstrap(&mut frame)
        .expect("successor seed checked V3 bootstrap");
    let resident = installed_resident(&frame);

    assert_eq!(resident.lse_configuration().model_version, V3_MODEL_VERSION);
    assert_eq!(
        resident
            .lse_configuration()
            .model_definition_sha256
            .as_str(),
        V3_MODEL_DEFINITION_SHA256
    );
    resident
        .lse_state()
        .validate(resident.lse_configuration())
        .expect("native V3 model identity");
    let state = resident
        .surface_owner()
        .v2_state()
        .expect("native surface V2 owner");
    assert!(
        state
            .records()
            .iter()
            .all(|record| { record.litter_ice_kg_m2_tile.to_bits() == 0_f64.to_bits() })
    );
    for record in state.records() {
        let lse_tile = resident
            .lse_state()
            .0
            .tiles
            .iter()
            .find(|tile| tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id)
            .expect("joined LSE tile");
        assert_eq!(
            record.surface_enthalpy_j_m2_tile.to_bits(),
            lse_tile.surface_enthalpy_j_m2_tile_ground.to_bits()
        );
    }
}

#[test]
fn successor_seed_restart_is_exact_and_serializes_no_diagnostics() {
    let seed_path = successor_golden_path();
    let seed_bytes = fs::read(&seed_path).expect("successor seed bytes");
    assert_eq!(
        seed_bytes,
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/snow_stage3_v11_owner_seed_two_day.json"
        )),
        "the additive V3 bootstrap must preserve the admitted V1 seed bytes"
    );
    let seed = DirectSnowStage3V11ProductionSeedV1::load_required(Some(&seed_path))
        .expect("successor seed");
    let mut first = matching_frame(&seed);
    let mut replay = matching_frame(&seed);
    seed.bootstrap(&mut first).expect("first V3 bootstrap");
    seed.bootstrap(&mut replay).expect("replayed V3 bootstrap");
    let first = installed_resident(&first);
    let replay = installed_resident(&replay);

    assert_eq!(
        first.lse_state().to_json().expect("first LSE V3 bytes"),
        replay.lse_state().to_json().expect("replay LSE V3 bytes")
    );
    assert_eq!(
        first
            .surface_configuration()
            .canonical_bytes()
            .expect("first surface V2 configuration"),
        replay
            .surface_configuration()
            .canonical_bytes()
            .expect("replay surface V2 configuration")
    );
    assert_eq!(
        first
            .surface_owner()
            .canonical_bytes(
                first.surface_configuration().parent(),
                Some(first.surface_configuration()),
            )
            .expect("first surface V2 owner"),
        replay
            .surface_owner()
            .canonical_bytes(
                replay.surface_configuration().parent(),
                Some(replay.surface_configuration()),
            )
            .expect("replay surface V2 owner")
    );

    assert_no_diagnostic_keys(
        &serde_json::from_slice(&first.lse_state().to_json().expect("LSE V3 JSON"))
            .expect("LSE V3 value"),
    );
    assert_no_diagnostic_keys(
        &serde_json::from_slice(
            &first
                .surface_owner()
                .canonical_bytes(
                    first.surface_configuration().parent(),
                    Some(first.surface_configuration()),
                )
                .expect("surface V2 JSON"),
        )
        .expect("surface V2 value"),
    );
}
