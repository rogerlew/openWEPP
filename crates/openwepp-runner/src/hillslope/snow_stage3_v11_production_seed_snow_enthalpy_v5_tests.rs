//! Runner bootstrap posture for the additive V56 snow-enthalpy owner.

use std::path::PathBuf;

use openwepp_persisted_restart_v1::{ExpectedDirectHydrologyRestartContext, Sha256Hex};

use super::*;

fn successor_golden_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/snow_stage3_v11_owner_seed_frozen_litter_v4.json")
}

fn matching_frame(seed: &DirectSnowStage3V11ProductionSeedV1) -> DirectRunFrame {
    let source = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture()
        .owners
        .runtime
        .shadow
        .restart_authority_hydrology_frame()
        .clone();
    let committed = seed.day_zero_committed().expect("V5 committed owner");
    let configuration = committed
        .surface_liquid_configuration
        .restore()
        .expect("V5 surface configuration");
    let day_inputs = source
        .lanes
        .iter()
        .map(|lane| lane.day_inputs.clone())
        .collect::<Vec<_>>();
    let digests = committed
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
            day_input_digests: &digests,
            surface_liquid_configuration: &configuration,
        })
        .expect("V5 live frame")
}

#[test]
fn production_seed_does_not_invent_v56_material_or_receipt_authority() {
    let seed = DirectSnowStage3V11ProductionSeedV1::load_required(Some(&successor_golden_path()))
        .expect("V4 seed");
    let mut frame = matching_frame(&seed);
    seed.bootstrap(&mut frame).expect("production bootstrap");
    let committed = &frame
        .snow_stage3_v11_attachment
        .as_ref()
        .expect("Stage-3 attachment")
        .committed;
    assert!(
        committed.snow_enthalpy_material_owner.is_none(),
        "a seed predating an accepted strictly frozen V56 support cannot invent a compound owner"
    );
    assert!(
        committed.snow_enthalpy_material_owner_chronology.is_empty(),
        "bootstrap cannot fabricate accepted V56 carry receipts"
    );
}
