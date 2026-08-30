use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowLayerState,
    SnowDensityModel, SnowMeltModel, SnowStage3LiquidRoutingModel, Wb11HydrologyKernel,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = "docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001/package.md";
const RUNNER_BUILDER: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs";
const RUNNER_STAGE3_AUTHORITY: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_stage3_canopy_authority.rs";
const RUNNER_IMPL: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs";
const HYDROLOGY_IMPL: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs";
const HYDROLOGY_SOLVER: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs";
const HYDROLOGY_EVALUATION: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs";
const RUNNER_BINS: &str = "crates/openwepp-runner/src/bin";
const OBSERVED_GATE_TOOL: &str =
    "tools/snowfreeze_observed/paradigm2_stage3_liquid_routing_meltwater_temperature.py";
const V11_PRODUCTION_TESTS: &str = concat!(
    "crates/openwepp-hillslope-orchestrator/src/",
    "snow_stage3_v11_adaptive_production_tests.rs"
);

#[test]
fn stage3_contract_package_and_selector_are_bound() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_id: SC-SNOWFREEZE-001",
        "REF-SNOWFREEZE-PARADIGM2-STAGE3",
        "INV-SNOWFREEZE-080",
        "OBL-SNOWFREEZE-P-055",
        "snow_stage3_liquid_routing_model",
        "snow_meltwater_flux_temperature",
        "layered_thermal_liquid_v1",
        "CoE melt/rain mass path remains authoritative",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "PARADIGM-2 Stage 3",
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1",
        "No replacement of CoE melt with energy-balance melt",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let builder = format!(
        "{}\n{}",
        read(RUNNER_BUILDER),
        read(RUNNER_STAGE3_AUTHORITY)
    );
    for marker in [
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL",
        "reject_retired_stage3_snow_selector_envs",
        "retired snow selector",
        "SnowMeltModel::AdaptiveCompositionalStage3V1",
        "SnowDensityModel::PhysicsBulkDensityCompactionV1",
        "SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1",
    ] {
        assert_contains(
            &builder,
            marker,
            "split runner Stage-3 builder/authority sources",
        );
    }
    assert!(!builder.contains("\"disabled\" => Ok"));

    let runner_impl = read(RUNNER_IMPL);
    assert_contains(
        &runner_impl,
        "stage3_liquid_routing_model: self.stage3_liquid_routing_model",
        RUNNER_IMPL,
    );
    assert_contains(&runner_impl, "mass_transition_ledgers", RUNNER_IMPL);

    let hydrology_impl = format!(
        "{}\n{}\n{}",
        read(HYDROLOGY_IMPL),
        read(HYDROLOGY_SOLVER),
        read(HYDROLOGY_EVALUATION)
    );
    for marker in [
        "TemperatureCelsius",
        "surface_energy_balance",
        "net_shortwave_radiation",
        "conductive_heat_flux",
        "stage3_liquid_closure_residual_m",
        "stage3_energy_residual_j_m2",
    ] {
        assert_contains(&hydrology_impl, marker, HYDROLOGY_IMPL);
    }

    let observed_gate_tool = read(OBSERVED_GATE_TOOL);
    for marker in [
        "paradigm2-stage3-liquid-routing-meltwater-temperature-gates-v1",
        "INV-SNOWFREEZE-080",
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL",
        "layered_thermal_liquid_v1",
        "stage1_layered_density_disabled_stage3",
        "runoff_timing_guardrail_vs_stage1",
    ] {
        assert_contains(&observed_gate_tool, marker, OBSERVED_GATE_TOOL);
    }

    let v11_production_tests = read(V11_PRODUCTION_TESTS);
    for marker in [
        "stable_minimum_production_support_accepts_one_direct_trial",
        "stable adaptive receipt validates",
        "accepted_microsteps.len(), 1",
        "odd_quanta_tile_exactly_and_never_call_carrier_below_floor",
    ] {
        assert_contains(&v11_production_tests, marker, V11_PRODUCTION_TESTS);
    }
}

#[test]
fn stage3_internal_selector_not_user_cli_exposed() {
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
            !text.contains("layered_thermal_liquid_v1"),
            "Stage 3 selector must remain internal, not CLI/user-facing: {}",
            path.display()
        );
    }
}

#[test]
fn retired_stage3_disabled_selector_fails_closed() {
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &warm_layered_inputs(SnowStage3LiquidRoutingModel::Disabled),
    )
    .expect_err("retired disabled Stage 3 selector must fail closed");
    assert!(
        error
            .to_string()
            .contains("snow.adaptive_stage3_legacy_shortwave_entry"),
        "unexpected retired-selector error: {error}"
    );
}

#[test]
fn stage3_v11_fixture_rejects_retired_partition_entrypoint() {
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &warm_layered_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1),
    )
    .expect_err("adaptive Stage3 V11 cannot enter the retired WB11 partition path");
    assert!(
        error
            .to_string()
            .contains("snow.adaptive_stage3_legacy_shortwave_entry"),
        "unexpected V11 cutover error: {error}"
    );
}

#[test]
fn stage3_v11_bulk_density_fixture_rejects_retired_partition_entrypoint() {
    let mut inputs = warm_layered_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1);
    inputs.snow_density_model = SnowDensityModel::PhysicsBulkDensityCompactionV1;
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect_err("adaptive Stage3 V11 cannot enter the retired WB11 partition path");
    assert!(
        error
            .to_string()
            .contains("snow.adaptive_stage3_legacy_shortwave_entry"),
        "unexpected V11 cutover error: {error}"
    );
}

#[test]
fn stage3_v11_cutover_precedes_retired_cold_content_partition_guard() {
    let mut inputs = warm_layered_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1);
    inputs.snow_layers[0].cold_content_j_m2 = 1.0e12;
    inputs.snow_layers[1].cold_content_j_m2 = 1.0e12;

    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect_err("retired partition entrypoint must fail before historical cold-content solve");

    let rendered = error.to_string();
    assert!(
        rendered.contains("snow.adaptive_stage3_legacy_shortwave_entry"),
        "unexpected V11 cutover error: {error}"
    );
}

fn warm_layered_inputs(
    stage3_liquid_routing_model: SnowStage3LiquidRoutingModel,
) -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        air_temperature_c: 6.0,
        cloud_fraction: 0.2,
        radiation_mj_m2: 0.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    for hour in hourly.iter_mut().take(2) {
        hour.air_temperature_c = 2.0;
        hour.radiation_mj_m2 = 0.0;
    }
    hourly[2].rain_m = 0.05;
    hourly[2].active_precipitation_m = 0.05;
    hourly[2].rain_fraction = 1.0;
    hourly[2].air_temperature_c = 4.0;

    let layers = vec![
        DirectSnowLayerState::new(0.09, 0.20, 450.0, 12.0)
            .with_stage3_thermal_liquid_state(-0.2, 0.0, 0.0, 0.0),
        DirectSnowLayerState::new(0.09, 0.20, 450.0, 12.0),
    ];

    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.05,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.18,
        runtime_depth_m: 0.40,
        runtime_density_kg_m3: 450.0,
        runtime_settle_day_count: 12.0,
        liquid_water_retained_m: 0.0,
        tmax_c: 6.0,
        tmin_c: 2.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 2.0,
        dewpoint_c: 0.0,
        snow_melt_model: SnowMeltModel::AdaptiveCompositionalStage3V1,
        snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
        stage3_liquid_routing_model,
        surface_energy_options:
            openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions::default(),
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 0.40,
        coe_boundary_density_kg_m3: 450.0,
        coe_boundary_settle_day_count: 12.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: layers,
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(Path::new(path)).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}
