use std::fs;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowLayerState,
    DirectSnowSurfaceEnergyOptions, DirectSnowTurbulentGeometry, SnowDensityModel, SnowMeltModel,
    SnowStage3LiquidRoutingModel, SnowSurfaceLongwaveModel, SnowSurfaceSublimationModel,
    Wb11HydrologyKernel,
};

const RUNNER_BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00c_day_input_builder_impl.rs"
);
const STAGE3_CANOPY_AUTHORITY: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00c_stage3_canopy_authority.rs"
);
const V11_PRODUCTION_TESTS: &str = concat!(
    "crates/openwepp-hillslope-orchestrator/src/",
    "snow_stage3_v11_adaptive_production_tests.rs"
);
const V11_EVALUATION_TESTS: &str = concat!(
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/",
    "runoff_reconciliation/stage3_evaluation_validation_tests.rs"
);
const RETIRED_PARTITION_PIPELINE: &str = concat!(
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/",
    "infiltration_reconciliation.rs"
);

#[test]
fn cligen_virtual_instrument_geometry_is_contract_bound() {
    let geometry = DirectSnowTurbulentGeometry::CLIGEN_V1;
    assert_eq!(
        geometry.air_temperature_height_m.to_bits(),
        5.0_f64.to_bits()
    );
    assert_eq!(
        geometry.vapor_pressure_height_m.to_bits(),
        5.0_f64.to_bits()
    );
    assert_eq!(geometry.wind_speed_height_m.to_bits(), 5.0_f64.to_bits());
    assert_eq!(
        geometry.aerodynamic_roughness_length_m.to_bits(),
        0.005_f64.to_bits()
    );
    assert_eq!(
        DirectSnowSurfaceEnergyOptions::default().turbulent_geometry,
        geometry
    );
}

#[test]
fn sole_stage3_v11_owner_and_real_success_closure_evidence_are_source_bound() {
    let builder = format!(
        "{}\n{}",
        read(RUNNER_BUILDER),
        read(STAGE3_CANOPY_AUTHORITY)
    );
    for marker in [
        "reject_retired_stage3_snow_selector_envs",
        "retired snow selector",
        "SnowMeltModel::AdaptiveCompositionalStage3V1",
        "SnowDensityModel::PhysicsBulkDensityCompactionV1",
        "SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1",
    ] {
        assert_contains(&builder, marker, RUNNER_BUILDER);
    }

    let production = read(V11_PRODUCTION_TESTS);
    for marker in [
        "stable_minimum_production_support_accepts_one_direct_trial",
        "stable adaptive receipt validates",
        "accepted_microsteps.len(), 1",
        "odd_quanta_tile_exactly_and_never_call_carrier_below_floor",
        "assert_eq!(accepted_ns, receipt.parent_support.duration_ns())",
    ] {
        assert_contains(&production, marker, V11_PRODUCTION_TESTS);
    }

    let evaluation = read(V11_EVALUATION_TESTS);
    for marker in [
        "stage3_intake_rejects_obsolete_physics_selectors",
        "stage3_neutral_bulk_vapor_transfer_preserves_bounded_mass_and_latent_custody",
        "snow.stage3_obsolete_snow_model_selector",
        "snow.stage3_obsolete_sublimation_selector",
    ] {
        assert_contains(&evaluation, marker, V11_EVALUATION_TESTS);
    }
    let retired_pipeline = read(RETIRED_PARTITION_PIPELINE);
    for marker in [
        "snow.adaptive_stage3_legacy_shortwave_entry",
        "snow.adaptive_stage3_legacy_sublimation_entry",
    ] {
        assert_contains(&retired_pipeline, marker, RETIRED_PARTITION_PIPELINE);
    }
}

#[test]
fn adaptive_owner_rejects_retired_wb11_partition_entrypoint() {
    let inputs = v11_inputs(DirectSnowSurfaceEnergyOptions::default());
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect_err("adaptive owner must not execute the retired WB11 partition path");
    assert!(
        error
            .to_string()
            .contains("snow.adaptive_stage3_legacy_sublimation_entry"),
        "unexpected cutover error: {error}"
    );
}

#[test]
fn adaptive_owner_rejects_retired_wb11_sublimation_entrypoint() {
    let inputs = v11_inputs(DirectSnowSurfaceEnergyOptions {
        longwave_model: SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        sublimation_model: SnowSurfaceSublimationModel::NeutralBulkStage3V1,
        daily_solar_radiation_mj_m2: 5.0,
        daily_extraterrestrial_radiation_mj_m2: 10.0,
        daylight: true,
        ..DirectSnowSurfaceEnergyOptions::default()
    });
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect_err("adaptive owner must not execute the retired WB11 sublimation path");
    assert!(
        error
            .to_string()
            .contains("snow.adaptive_stage3_legacy_sublimation_entry"),
        "unexpected cutover error: {error}"
    );
}

#[test]
fn historical_snow_model_selector_remains_fail_closed() {
    let mut inputs = v11_inputs(DirectSnowSurfaceEnergyOptions::default());
    inputs.snow_melt_model = SnowMeltModel::CoeLiquidHoldingCapacityV1;
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect_err("historical CoE selector must not enter Stage3 V11");
    assert!(
        error
            .to_string()
            .contains("snow.stage3_obsolete_snow_model_selector"),
        "unexpected selector error: {error}"
    );
}

fn v11_inputs(
    surface_energy_options: DirectSnowSurfaceEnergyOptions,
) -> DirectActiveSnowPartitionInputs {
    let mut surface = DirectSnowLayerState::new(0.18, 0.40, 450.0, 12.0);
    surface.temperature_c = -8.0;
    surface.cold_content_j_m2 = 0.18 * 1_000.0 * 2_100.0 * 8.0;
    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.18,
        runtime_depth_m: 0.40,
        runtime_density_kg_m3: 450.0,
        runtime_settle_day_count: 12.0,
        liquid_water_retained_m: 0.0,
        tmax_c: -3.0,
        tmin_c: -7.0,
        canopy_cover_fraction: 0.45,
        wind_m_s: 3.0,
        dewpoint_c: -15.0,
        snow_melt_model: SnowMeltModel::AdaptiveCompositionalStage3V1,
        snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
        stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
        surface_energy_options,
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 0.40,
        coe_boundary_density_kg_m3: 450.0,
        coe_boundary_settle_day_count: 12.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: vec![surface],
        underlying_surface_albedo: 0.2,
        hourly: [DirectSnowHourlyForcing {
            radiation_mj_m2: 0.0,
            air_temperature_c: -5.0,
            ..DirectSnowHourlyForcing::zero()
        }; 24],
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}
