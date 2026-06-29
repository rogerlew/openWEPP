use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectDayFrame, DirectRunIdentity, DirectSnowCouplingInputs, DirectSnowLayerState,
    SnowDensityError, SnowDensityModel, SnowDensityRuntimeInputs,
    update_snow_density_runtime_state,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const BUILDER: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs";
const RUNNER_BINS: &str = "crates/openwepp-runner/src/bin";
const PACKAGE: &str =
    "docs/work-packages/20260628-paradigm-2-stage-1-layered-snow-density-001/package.md";

#[test]
fn stage1_contract_and_selector_are_package_bound() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 110",
        "REF-SNOWFREEZE-PARADIGM2-STAGE1",
        "INV-SNOWFREEZE-078",
        "OBL-SNOWFREEZE-P-053",
        "snow_layers",
        "snow_layer_local_overburden",
        "physics_bulk_multilayer_density_v1",
        "local overburden",
        "must close `HOLD` or non-promotion without activation",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let builder = read(BUILDER);
    assert_contains(&builder, "physics_bulk_multilayer_density_v1", BUILDER);
    assert_contains(
        &builder,
        "SnowDensityModel::PhysicsBulkMultilayerDensityV1",
        BUILDER,
    );
    assert_contains(
        &builder,
        "must be legacy_wepp, physics_bulk_density_compaction_v1, physics_bulk_shallow_guard_v1, physics_bulk_climate_class_density_v1, or physics_bulk_multilayer_density_v1",
        BUILDER,
    );
    assert_runner_bins_do_not_expose_selector();

    let package = read(PACKAGE);
    assert_contains(
        &package,
        "Status: `HOLD-GATE-FAILURE-NON-PROMOTION`",
        PACKAGE,
    );
}

#[test]
fn multilayer_density_uses_local_overburden_top_to_bottom() {
    let prior_layers = vec![
        DirectSnowLayerState::new(0.05, 0.25, 200.0, 3.0),
        DirectSnowLayerState::new(0.25, 0.75, 333.333_333_333_333_3, 8.0),
    ];
    let prior_swe_m = sum_swe(&prior_layers);
    let prior_depth_m = sum_depth(&prior_layers);
    let prior_density_kg_m3 = prior_swe_m * 1_000.0 / prior_depth_m;

    let outcome = update_snow_density_runtime_state(&SnowDensityRuntimeInputs {
        model: SnowDensityModel::PhysicsBulkMultilayerDensityV1,
        prior_swe_m,
        prior_depth_m,
        prior_density_kg_m3,
        prior_settle_day_count: 8.0,
        prior_layers: prior_layers.clone(),
        boundary_swe_after_m: prior_swe_m,
        boundary_depth_after_m: prior_depth_m,
        boundary_density_after_kg_m3: prior_density_kg_m3,
        snow_input_m: 0.0,
        liquid_for_compaction_m: 0.0,
        mean_air_temperature_c: -5.0,
        runtime_density_cap_kg_m3: 522.0,
        sturm_climate_class: None,
        sturm_day_of_year: None,
    })
    .expect("multilayer density update should run");

    assert_eq!(
        outcome.model,
        SnowDensityModel::PhysicsBulkMultilayerDensityV1
    );
    assert_eq!(outcome.layers_after.len(), 2);
    let top_delta = outcome.layers_after[0].density_kg_m3 - prior_layers[0].density_kg_m3;
    let bottom_delta = outcome.layers_after[1].density_kg_m3 - prior_layers[1].density_kg_m3;
    assert!(
        top_delta > 0.0,
        "destructive metamorphism should still affect the surface layer"
    );
    assert!(
        bottom_delta > top_delta,
        "lower layer should densify more because local overburden includes the surface layer"
    );
    assert_close(outcome.runtime_swe_after_m, prior_swe_m, 1.0e-12);
    assert_close(
        sum_swe(&outcome.layers_after),
        outcome.runtime_swe_after_m,
        1.0e-12,
    );
    assert_close(
        sum_depth(&outcome.layers_after),
        outcome.runtime_depth_after_m,
        1.0e-12,
    );
}

#[test]
fn multilayer_density_adds_new_snow_as_surface_layer() {
    let outcome = update_snow_density_runtime_state(&SnowDensityRuntimeInputs {
        model: SnowDensityModel::PhysicsBulkMultilayerDensityV1,
        prior_swe_m: 0.10,
        prior_depth_m: 0.40,
        prior_density_kg_m3: 250.0,
        prior_settle_day_count: 4.0,
        prior_layers: Vec::new(),
        boundary_swe_after_m: 0.12,
        boundary_depth_after_m: 0.48,
        boundary_density_after_kg_m3: 250.0,
        snow_input_m: 0.02,
        liquid_for_compaction_m: 0.0,
        mean_air_temperature_c: -6.0,
        runtime_density_cap_kg_m3: 522.0,
        sturm_climate_class: None,
        sturm_day_of_year: None,
    })
    .expect("first opt-in use should synthesize prior layer and add fresh surface layer");

    assert_eq!(outcome.layers_after.len(), 2);
    assert_close(outcome.layers_after[0].mass_swe_m, 0.02, 1.0e-12);
    assert_close(sum_swe(&outcome.layers_after), 0.12, 1.0e-12);
}

#[test]
fn multilayer_density_fails_closed_on_bad_layer_aggregate() {
    let err = update_snow_density_runtime_state(&SnowDensityRuntimeInputs {
        model: SnowDensityModel::PhysicsBulkMultilayerDensityV1,
        prior_swe_m: 0.20,
        prior_depth_m: 0.50,
        prior_density_kg_m3: 400.0,
        prior_settle_day_count: 4.0,
        prior_layers: vec![DirectSnowLayerState::new(0.10, 0.25, 400.0, 4.0)],
        boundary_swe_after_m: 0.20,
        boundary_depth_after_m: 0.50,
        boundary_density_after_kg_m3: 400.0,
        snow_input_m: 0.0,
        liquid_for_compaction_m: 0.0,
        mean_air_temperature_c: -4.0,
        runtime_density_cap_kg_m3: 522.0,
        sturm_climate_class: None,
        sturm_day_of_year: None,
    })
    .expect_err("layer aggregate mismatch must fail closed");

    assert!(matches!(
        err,
        SnowDensityError::LayerAggregateMismatch {
            symbol: "prior_layers.mass_swe_m",
            ..
        }
    ));
}

#[test]
fn multilayer_density_respects_aggregate_density_cap() {
    let cap = 522.0;
    let prior_swe_m = 0.10;
    let prior_depth_m = prior_swe_m * 1_000.0 / cap;
    let outcome = update_snow_density_runtime_state(&SnowDensityRuntimeInputs {
        model: SnowDensityModel::PhysicsBulkMultilayerDensityV1,
        prior_swe_m,
        prior_depth_m,
        prior_density_kg_m3: cap,
        prior_settle_day_count: 6.0,
        prior_layers: vec![DirectSnowLayerState::new(
            prior_swe_m,
            prior_depth_m,
            cap,
            6.0,
        )],
        boundary_swe_after_m: prior_swe_m,
        boundary_depth_after_m: prior_depth_m,
        boundary_density_after_kg_m3: cap,
        snow_input_m: 0.0,
        liquid_for_compaction_m: 0.0,
        mean_air_temperature_c: -1.0,
        runtime_density_cap_kg_m3: cap,
        sturm_climate_class: None,
        sturm_day_of_year: None,
    })
    .expect("cap-bound multilayer update should remain domain-valid");

    assert!(
        outcome.runtime_density_after_kg_m3 <= cap,
        "aggregate density must not exceed the direct-domain density cap"
    );
    assert!(
        outcome
            .layers_after
            .iter()
            .all(|layer| layer.density_kg_m3 <= cap)
    );
}

#[test]
fn r4g_snow_coupling_persists_layer_stack_into_winter_column_and_carry() {
    let identity = DirectRunIdentity::new(90, 2637, 1, 1)
        .expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let layers = vec![
        DirectSnowLayerState::new(0.04, 0.20, 200.0, 1.0),
        DirectSnowLayerState::new(0.16, 0.40, 400.0, 6.0),
    ];
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: 0.0,
        snow_state_projected: true,
        active_snow_coupling: true,
        runtime_swe_after_m: 0.20,
        runtime_depth_after_m: 0.60,
        runtime_density_after_kg_m3: 333.333_333_333_333_3,
        runtime_settle_day_count_after: 6.0,
        coe_boundary_depth_after_m: 0.60,
        coe_boundary_density_after_kg_m3: 333.333_333_333_333_3,
        coe_boundary_settle_day_count_after: 6.0,
        snow_layers_after: layers.clone(),
        ..DirectSnowCouplingInputs::zero()
    };

    day.run_r4g_snow_coupling_span()
        .expect("projected snow coupling should execute");

    assert_eq!(day.winter_column.snow.layers, layers);
    assert_eq!(
        &day.snow_runtime_carry
            .as_ref()
            .expect("snow carry should be present")
            .layers,
        &day.winter_column.snow.layers
    );
}

fn sum_swe(layers: &[DirectSnowLayerState]) -> f64 {
    layers.iter().map(|layer| layer.mass_swe_m).sum()
}

fn sum_depth(layers: &[DirectSnowLayerState]) -> f64 {
    layers.iter().map(|layer| layer.thickness_m).sum()
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {actual} to be within {tolerance} of {expected}"
    );
}

fn read(path: &str) -> String {
    fs::read_to_string(Path::new(path)).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_runner_bins_do_not_expose_selector() {
    for entry in fs::read_dir(RUNNER_BINS)
        .unwrap_or_else(|err| panic!("failed to read runner binary directory: {err}"))
    {
        let path = entry
            .unwrap_or_else(|err| panic!("failed to read runner binary entry: {err}"))
            .path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        assert!(
            !text.contains("physics_bulk_multilayer_density_v1"),
            "Stage 1 selector must remain package-bound, not CLI/user-facing: {}",
            path.display()
        );
    }
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}
