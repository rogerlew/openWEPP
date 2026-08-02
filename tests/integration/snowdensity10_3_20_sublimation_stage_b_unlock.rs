use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, SnowDensityModel, SnowMeltModel,
    Wb11HydrologyKernel,
};
use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = concat!(
    "docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/",
    "package.md"
);
const BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00c_day_input_builder_impl.rs"
);
const TOOL: &str = "tools/snowfreeze_observed/sublimation_stage_b_unlock.py";
const REPORT: &str = concat!(
    "docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/",
    "artifacts/sublimation-stage-b-unlock.json"
);
const TOL: f64 = 1.0e-12;

#[test]
fn contract_and_package_bind_stage_b_unlock_authority() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 120",
        "REF-SNOWFREEZE-SNOWDENSITY1020",
        "REF-SNOWFREEZE-LIBSNOBAL-CC0",
        "INV-SNOWFREEZE-076",
        "OBL-SNOWFREEZE-P-051",
        "coe_open_sublimation_stage_b_v1",
        "snow_sublimation_surface_temperature_c",
        "snow_sublimation_surface_layer_depth_m",
        "SNOWDENSITY-10.3.20 Sublimation Stage B Unlock Addendum",
        "bf8b41c71e3e54ae654ae04005ddf72566c47ee6",
        "license=\"CC0 1.0\"",
        "Promotion is allowed only if",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.20",
        "Stage A degradation",
        "partition+sublimation composition",
        "coe_open_sublimation_stage_b_v1",
        "No `.run` disable option",
        "Subagent authorization",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn stage_b_selector_is_opt_in_and_no_user_surface_is_added() {
    let builder = read(BUILDER);
    for marker in [
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "SnowMeltModel::CoeLiquidHoldingCapacityV1",
        "SnowMeltModel::CoeOpenSublimationStageAV1",
        "SnowMeltModel::CoeOpenSublimationStageBV1",
        "coe_open_sublimation_stage_b_v1",
        "must be legacy_coe, coe_liquid_holding_capacity_v1, coe_open_sublimation_stage_a_v1, or coe_open_sublimation_stage_b_v1",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }
    assert!(
        !builder.contains("snow-melt-model") && !builder.contains("run_file_disable_option"),
        "Stage B must not add parser/runfile/user CLI or .run disable surfaces"
    );
}

#[test]
fn stage_b_surface_temperature_gate_reduces_stage_a_sublimation_without_liquid_routing() {
    let stage_a = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &dry_windy_open_inputs(SnowMeltModel::CoeOpenSublimationStageAV1),
    )
    .expect("Stage A should compute");
    let stage_b = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &dry_windy_open_inputs(SnowMeltModel::CoeOpenSublimationStageBV1),
    )
    .expect("Stage B should compute");

    assert!(stage_a.sublimation_m > 0.0);
    assert!(stage_b.sublimation_m > 0.0);
    assert!(
        stage_b.sublimation_m < stage_a.sublimation_m,
        "cold surface-layer vapor pressure should reduce Stage A sublimation"
    );
    assert!((stage_b.routed_melt_m - stage_a.routed_melt_m).abs() <= TOL);
    assert!((stage_b.snowpack_swe_loss_m - stage_a.snowpack_swe_loss_m).abs() <= TOL);

    let available_swe_m = dry_windy_open_inputs(SnowMeltModel::CoeOpenSublimationStageBV1)
        .runtime_swe_m
        + stage_b.rain_retained_m;
    let closure = available_swe_m
        - stage_b.snowpack_swe_loss_m
        - stage_b.sublimation_m
        - stage_b.runtime_swe_after_m;
    assert!(
        closure.abs() <= TOL,
        "Stage B vapor closure failed: {closure}"
    );
}

#[test]
fn tool_and_report_bind_primary_rubric_and_conservation_gates() {
    let tool = read(TOOL);
    for marker in [
        "snowdensity10-3-20-sublimation-stage-b-unlock-v1",
        "stage_a_legacy_phase_10_3_16",
        "partition_sublimation_stage_a",
        "stage_b_surface_layer",
        "coe_open_sublimation_stage_b_v1",
        "primary_gate_beats_current_default",
        "sublimation_vapor_conservation_ok",
        "run_file_disable_option_added",
    ] {
        assert_contains(&tool, marker, TOOL);
    }

    let report: Value = serde_json::from_str(&read(REPORT)).expect("10.3.20 report should parse");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-20-sublimation-stage-b-unlock-v1"
    );
    assert_eq!(
        report["protected_boundaries"]["production_default_changed"],
        false
    );
    assert_eq!(
        report["protected_boundaries"]["run_file_disable_option_added"],
        false
    );
    assert_eq!(
        report["trace_proof"]["stage_b_surface_layer"]["expected"]["snow_melt_model"],
        "coe_open_sublimation_stage_b_v1"
    );
    assert_eq!(
        report["trace_proof"]["stage_b_surface_layer"]["selector_trace_ok"],
        true
    );
    assert_eq!(
        report["trace_proof"]["stage_b_surface_layer"]["sublimation_vapor_conservation_ok"],
        true
    );
    assert_eq!(
        report["trace_proof"]["stage_b_surface_layer"]["partition_conservation_ok"],
        true
    );
}

fn dry_windy_open_inputs(model: SnowMeltModel) -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        cloud_fraction: 0.25,
        air_temperature_c: -8.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    for hour in &mut hourly {
        hour.radiation_mj_m2 = 0.2;
        hour.air_temperature_c = -8.0;
        hour.cloud_fraction = 0.25;
    }

    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.45,
        runtime_depth_m: 1.0,
        runtime_density_kg_m3: 450.0,
        runtime_settle_day_count: 30.0,
        liquid_water_retained_m: 0.0,
        tmax_c: -8.0,
        tmin_c: -8.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 6.0,
        dewpoint_c: -18.0,
        snow_melt_model: model,
        snow_density_model: SnowDensityModel::LegacyWepp,
        stage3_liquid_routing_model:
            openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled,
        surface_energy_options:
            openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions::default(),
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 1.0,
        coe_boundary_density_kg_m3: 450.0,
        coe_boundary_settle_day_count: 30.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: Vec::new(),
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

fn read(path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn assert_contains(haystack: &str, needle: &str, context: &str) {
    assert!(
        haystack.contains(needle),
        "{context} missing required marker: {needle}"
    );
}
