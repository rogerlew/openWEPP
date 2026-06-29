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
const OBSERVED_GATE_TOOL: &str =
    "tools/snowfreeze_observed/paradigm2_stage3_decouple_water_temperature.py";

#[test]
fn stage3_decouple_contract_package_and_selector_are_bound() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 115",
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

    let hydrology_impl = read(HYDROLOGY_IMPL);
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
fn stage3_decouple_synthesizes_bulk_equivalent_layers_without_stage1_density() {
    let mut inputs = warm_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1);
    inputs.snow_density_model = SnowDensityModel::PhysicsBulkDensityCompactionV1;
    inputs.snow_layers.clear();
    let mut disabled_inputs = inputs.clone();
    disabled_inputs.stage3_liquid_routing_model = SnowStage3LiquidRoutingModel::Disabled;

    let disabled =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&disabled_inputs)
            .expect("bulk-density disabled partition should compute");
    let partition = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("decoupled Stage 3 should synthesize bulk-equivalent layers");

    assert!(partition.stage3_diagnostics.enabled);
    assert!((partition.runtime_swe_after_m - disabled.runtime_swe_after_m).abs() <= 1.0e-12);
    assert!((partition.runtime_depth_after_m - disabled.runtime_depth_after_m).abs() <= 1.0e-12);
    assert!(
        (partition.runtime_density_after_kg_m3 - disabled.runtime_density_after_kg_m3).abs()
            <= 1.0e-12
    );
    assert!(!partition.snow_layers_after.is_empty());
    assert_bulk_equivalent_layers(&partition);
    assert!(
        partition
            .stage3_diagnostics
            .meltwater_temperature_c
            .is_none_or(|temperature| { temperature.as_celsius().abs() <= 1.0e-12 })
    );
    assert!(partition.stage3_diagnostics.liquid_closure_residual_m.abs() <= 1.0e-9);
    assert!(
        partition
            .stage3_diagnostics
            .energy_closure_residual_j_m2
            .abs()
            <= 1.0e-6
    );
}

#[test]
fn stage3_decouple_carries_geometry_but_removes_density_gradient() {
    let mut inputs = warm_inputs(SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1);
    inputs.snow_density_model = SnowDensityModel::PhysicsBulkDensityCompactionV1;
    inputs.snow_layers = vec![
        DirectSnowLayerState::new(0.05, 0.20, 250.0, 2.0)
            .with_stage3_thermal_liquid_state(-2.0, 0.0, 0.0, 0.0),
        DirectSnowLayerState::new(0.08, 0.18, 444.444_444_444, 8.0)
            .with_stage3_thermal_liquid_state(-0.5, 0.0, 0.0, 0.0),
    ];

    let partition = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("decoupled Stage 3 should preserve geometry with bulk density");

    assert!(partition.stage3_diagnostics.enabled);
    assert!(partition.snow_layers_after.len() >= 2);
    assert_bulk_equivalent_layers(&partition);
    let surface_density = partition
        .snow_layers_after
        .first()
        .expect("surface layer")
        .density_kg_m3;
    let basal_density = partition
        .snow_layers_after
        .last()
        .expect("basal layer")
        .density_kg_m3;
    assert!((basal_density - surface_density).abs() <= 1.0e-12);
}

fn assert_bulk_equivalent_layers(
    partition: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) {
    let layer_swe_sum_m = partition
        .snow_layers_after
        .iter()
        .map(|layer| layer.mass_swe_m)
        .sum::<f64>();
    let layer_depth_sum_m = partition
        .snow_layers_after
        .iter()
        .map(|layer| layer.thickness_m)
        .sum::<f64>();
    assert!((layer_swe_sum_m - partition.runtime_swe_after_m).abs() <= 1.0e-9);
    assert!((layer_depth_sum_m - partition.runtime_depth_after_m).abs() <= 1.0e-9);
    for layer in &partition.snow_layers_after {
        assert!((layer.density_kg_m3 - partition.runtime_density_after_kg_m3).abs() <= 1.0e-12);
        assert!(layer.temperature_c <= 0.0);
    }
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
        snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
        snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
        stage3_liquid_routing_model,
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
