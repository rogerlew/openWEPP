use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowLayerState,
    DirectSnowStage3Diagnostics, SnowDensityModel, SnowMeltModel, SnowStage3LiquidRoutingModel,
    Wb11HydrologyKernel,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = "docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001/package.md";
const RUNNER_BUILDER: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs";
const RUNNER_IMPL: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs";
const HYDROLOGY_IMPL: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs";
const RUNNER_BINS: &str = "crates/openwepp-runner/src/bin";
const OBSERVED_GATE_TOOL: &str =
    "tools/snowfreeze_observed/paradigm2_stage3_liquid_routing_meltwater_temperature.py";

#[test]
fn stage3_contract_package_and_selector_are_bound() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 112",
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

    let builder = read(RUNNER_BUILDER);
    for marker in [
        "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL",
        "paradigm2_stage3_liquid_routing_model",
        "disabled",
        "layered_thermal_liquid_v1",
    ] {
        assert_contains(&builder, marker, RUNNER_BUILDER);
    }

    let runner_impl = read(RUNNER_IMPL);
    assert_contains(
        &runner_impl,
        "stage3_liquid_routing_model: self.stage3_liquid_routing_model",
        RUNNER_IMPL,
    );
    assert_contains(&runner_impl, "stage3_diagnostics", RUNNER_IMPL);

    let hydrology_impl = read(HYDROLOGY_IMPL);
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
fn stage3_disabled_default_leaves_diagnostics_off() {
    let partition = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &warm_layered_inputs(SnowStage3LiquidRoutingModel::Disabled),
    )
    .expect("disabled Stage 3 partition should compute");
    assert_eq!(
        partition.stage3_diagnostics,
        DirectSnowStage3Diagnostics::disabled()
    );
    assert!(
        partition.routed_melt_m > 0.0,
        "test setup should exercise the existing CoE melt path"
    );
}

#[test]
fn stage3_routes_liquid_closes_energy_and_produces_typed_temperature() {
    let partition = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        &warm_layered_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1),
    )
    .expect("Stage 3 opt-in partition should compute");

    let diagnostics = partition.stage3_diagnostics;
    assert!(diagnostics.enabled);
    assert!(diagnostics.incoming_liquid_m > 0.0);
    assert!(diagnostics.routed_liquid_m > 0.0);
    assert!(diagnostics.refrozen_liquid_m > 0.0);
    assert!(diagnostics.retained_liquid_m > 0.0);
    assert!(diagnostics.liquid_closure_residual_m.abs() <= 1.0e-9);
    assert!(diagnostics.energy_closure_residual_j_m2.abs() <= 1.0e-6);
    let meltwater_temperature_c = diagnostics
        .meltwater_temperature_c
        .expect("positive routed liquid should carry typed temperature")
        .as_celsius();
    assert!(meltwater_temperature_c.abs() <= 1.0e-12);
    assert!((partition.routed_melt_m - diagnostics.incoming_liquid_m).abs() <= 1.0e-12);
    assert!(
        partition
            .snow_layers_after
            .iter()
            .all(|layer| layer.temperature_c <= 0.0)
    );
}

#[test]
fn stage3_accepts_bulk_density_model_after_decouple() {
    let mut inputs = warm_layered_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1);
    inputs.snow_density_model = SnowDensityModel::PhysicsBulkDensityCompactionV1;
    let mut disabled_inputs = inputs.clone();
    disabled_inputs.stage3_liquid_routing_model = SnowStage3LiquidRoutingModel::Disabled;

    let disabled =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&disabled_inputs)
            .expect("bulk-density disabled Stage 3 partition should compute");
    let partition = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("Stage 3 must run on the bulk-density model after decoupling");

    assert!(partition.stage3_diagnostics.enabled);
    assert!((partition.runtime_swe_after_m - disabled.runtime_swe_after_m).abs() <= 1.0e-12);
    assert!((partition.runtime_depth_after_m - disabled.runtime_depth_after_m).abs() <= 1.0e-12);
    assert!(
        (partition.runtime_density_after_kg_m3 - disabled.runtime_density_after_kg_m3).abs()
            <= 1.0e-12
    );
    assert!(partition.snow_layers_after.iter().all(|layer| {
        (layer.density_kg_m3 - partition.runtime_density_after_kg_m3).abs() <= 1.0e-12
    }));
}

#[test]
fn stage3_caps_persisted_cold_content_to_current_layer_mass() {
    let mut inputs = warm_layered_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1);
    inputs.snow_layers[0].cold_content_j_m2 = 1.0e12;
    inputs.snow_layers[1].cold_content_j_m2 = 1.0e12;

    let partition = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("physically capped Stage 3 cold-content carry should compute");

    assert!(partition.stage3_diagnostics.enabled);
    assert!(partition.snow_layers_after.iter().all(|layer| {
        (-273.15..=0.0).contains(&layer.temperature_c)
            && layer.cold_content_j_m2.is_finite()
            && layer.cold_content_j_m2 >= 0.0
    }));
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
        snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
        snow_density_model: SnowDensityModel::PhysicsBulkMultilayerDensityV1,
        stage3_liquid_routing_model,
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
