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
    let committed = seed.day_zero_committed().expect("V4 committed owner");
    let configuration = committed
        .surface_liquid_configuration
        .restore()
        .expect("V4 surface configuration");
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
        .expect("V4 live frame")
}

#[test]
fn successor_seed_installs_mandatory_exact_surface_owner() {
    let seed = DirectSnowStage3V11ProductionSeedV1::load_required(Some(&successor_golden_path()))
        .expect("V4 seed");
    let mut frame = matching_frame(&seed);
    seed.bootstrap(&mut frame).expect("V4 bootstrap");
    let consumer = &frame
        .snow_stage3_v11_attachment
        .as_ref()
        .expect("Stage-3 attachment")
        .committed
        .real_consumer;
    let physical = consumer
        .frozen_litter_v3_resident()
        .expect("parallel immutable V3 physical resident");
    let exact = consumer
        .frozen_litter_v4_resident()
        .expect("mandatory V4 exact resident")
        .exact_surface_owner();
    exact
        .validate_frozen_parent_join(
            physical.lse_configuration(),
            physical.lse_state(),
            physical.surface_configuration(),
            physical.surface_owner(),
        )
        .expect("exact owner/high-mirror join");
    assert_eq!(exact.records().len(), physical.lse_state().0.tiles.len());
    assert!(exact.records().iter().all(|record| {
        record.enthalpy_carry == openwepp_land_surface_energy::ExactDyadicEnthalpy::zero()
    }));
    assert!(
        consumer
            .frozen_litter_v4_resident()
            .expect("V4 resident")
            .accepted_publication_supports_canonical_bytes()
            .is_empty(),
        "bootstrap must not fabricate an accepted support publication"
    );
}

#[test]
fn successor_seed_v4_fixture_preserves_frozen_v1_wire() {
    assert_eq!(
        std::fs::read(successor_golden_path()).expect("V4 fixture bytes"),
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/snow_stage3_v11_owner_seed_frozen_litter_v3.json"
        ))
    );
}
