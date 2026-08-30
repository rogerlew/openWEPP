use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowLayerState,
    SnowDensityModel, SnowMeltModel, SnowStage3LiquidRoutingModel, Wb11HydrologyKernel,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001/package.md";
const HYDROLOGY_IMPL: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs";
const HYDROLOGY_SOLVER: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs";
const HYDROLOGY_EVALUATION: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs";
const OBSERVED_GATE_TOOL: &str =
    "tools/snowfreeze_observed/paradigm2_stage3_decouple_water_temperature.py";

#[test]
fn stage3_decouple_contract_package_and_selector_are_bound() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_id: SC-SNOWFREEZE-001",
        "REF-SNOWFREEZE-PARADIGM2-STAGE3-DECOUPLE",
        "INV-SNOWFREEZE-081",
        "OBL-SNOWFREEZE-P-056",
        "bulk-equivalent layer stack",
        "15` robust fails / `179` score",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "PARADIGM-2 Stage 3-Decouple",
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1",
        "physics_bulk_density_compaction_v1",
        "No Stage 1 per-layer densification requirement",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let hydrology_impl = format!(
        "{}\n{}\n{}",
        read(HYDROLOGY_IMPL),
        read(HYDROLOGY_SOLVER),
        read(HYDROLOGY_EVALUATION)
    );
    for marker in [
        "prepare_stage3_layer_stack",
        "stage3_requires_bulk_or_multilayer_density_model",
        "stage3_bulk_equivalent_layer_swe_residual_m",
        "stage3_bulk_equivalent_layer_depth_residual_m",
    ] {
        assert_contains(&hydrology_impl, marker, HYDROLOGY_IMPL);
    }

    let observed_gate_tool = read(OBSERVED_GATE_TOOL);
    for marker in [
        "paradigm2-stage3-decouple-water-temperature-gates-v1",
        "INV-SNOWFREEZE-081",
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL",
        "stage3_decoupled_bulk_equivalent",
        "snow_guardrail_equals_default",
    ] {
        assert_contains(&observed_gate_tool, marker, OBSERVED_GATE_TOOL);
    }
}

#[test]
fn stage3_v11_bulk_fixture_rejects_retired_partition_entrypoint() {
    let mut inputs = warm_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1);
    inputs.snow_density_model = SnowDensityModel::PhysicsBulkDensityCompactionV1;
    inputs.snow_layers.clear();
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
fn stage3_v11_layer_geometry_fixture_cannot_bypass_retired_partition_entrypoint() {
    let mut inputs = warm_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1);
    inputs.snow_density_model = SnowDensityModel::PhysicsBulkDensityCompactionV1;
    inputs.snow_layers = vec![
        DirectSnowLayerState::new(0.05, 0.20, 250.0, 2.0)
            .with_stage3_thermal_liquid_state(-2.0, 0.0, 0.0, 0.0),
        DirectSnowLayerState::new(0.08, 0.18, 444.444_444_444, 8.0)
            .with_stage3_thermal_liquid_state(-0.5, 0.0, 0.0, 0.0),
    ];

    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect_err("adaptive Stage3 V11 geometry cannot enter the retired WB11 path");
    assert!(
        error
            .to_string()
            .contains("snow.adaptive_stage3_legacy_shortwave_entry"),
        "unexpected V11 cutover error: {error}"
    );
}

fn warm_inputs(
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
        snow_layers: Vec::new(),
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
