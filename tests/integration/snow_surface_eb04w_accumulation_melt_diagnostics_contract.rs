use std::fs;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, SnowDensityModel, SnowMeltModel,
    SnowPhasePartitionModel, Wb11HydrologyKernel,
};

const STORAGE_TOLERANCE_M: f64 = 1.0e-9;

fn repo_text(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn contract_binds_phase_accumulation_and_melt_diagnostics_without_promotion() {
    let contract =
        repo_text("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    for binding in [
        "contract_id: SC-SNOWFREEZE-001",
        "INV-SNOWFREEZE-088",
        "OBL-SNOWFREEZE-P-062",
        "TOL-SNOWFREEZE-013",
        "snowfall SWE",
        "amelt",
        "bmelt",
        "cmelt",
        "dmelt",
        "physical site redistribution remains unknown",
        "diagnostic-only",
    ] {
        assert!(
            contract.contains(binding),
            "missing contract binding: {binding}"
        );
    }
}

fn zero_pack_inputs(hourly: &[DirectSnowHourlyForcing; 24]) -> DirectActiveSnowPartitionInputs {
    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: hourly.iter().map(|hour| hour.active_precipitation_m).sum(),
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 350.0,
        runtime_swe_m: 0.0,
        runtime_depth_m: 0.0,
        runtime_density_kg_m3: 0.0,
        runtime_settle_day_count: 0.0,
        liquid_water_retained_m: 0.0,
        tmax_c: 6.0,
        tmin_c: 0.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 0.0,
        dewpoint_c: 0.0,
        snow_melt_model: SnowMeltModel::LegacyCoe,
        snow_density_model: SnowDensityModel::LegacyWepp,
        stage3_liquid_routing_model:
            openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled,
        surface_energy_options:
            openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions::default(),
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 0.0,
        coe_boundary_density_kg_m3: 0.0,
        coe_boundary_settle_day_count: 0.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: Vec::new(),
        underlying_surface_albedo: 0.2,
        hourly: *hourly,
    }
}

fn snowfall_hour(snowfall_depth_m: f64, rain_m: f64) -> DirectSnowHourlyForcing {
    let snowfall_swe_m = snowfall_depth_m * 0.1;
    let total_m = snowfall_swe_m + rain_m;
    DirectSnowHourlyForcing {
        active_precipitation_m: total_m,
        rain_m,
        snowfall_m: snowfall_depth_m,
        air_temperature_c: -2.0,
        phase_model: SnowPhasePartitionModel::HarderPomeroyHourly,
        rain_fraction: rain_m / total_m,
        snow_fraction: snowfall_swe_m / total_m,
        hydrometeor_temperature_c: Some(-1.0),
        ..DirectSnowHourlyForcing::zero()
    }
}

fn independent_residual(
    inputs: &DirectActiveSnowPartitionInputs,
    outcome: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> f64 {
    let typed_snowfall_swe_m: f64 = inputs.hourly.iter().map(|hour| hour.snowfall_m * 0.1).sum();
    inputs.runtime_swe_m + typed_snowfall_swe_m + outcome.rain_retained_m
        - outcome.solid_to_liquid_ledger().snowpack_swe_loss_m
        - outcome.sublimation_m
        - outcome.runtime_swe_after_m
}

#[test]
fn eb04w2b_contracts_bind_activation_and_independent_closure() {
    let snow = repo_text("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    let runoff = repo_text("docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md");
    for marker in [
        "INV-SNOWFREEZE-089",
        "TOL-SNOWFREEZE-014",
        "sum(hourly snowfall_m * 0.1)",
        "Warm all-rain, zero-pack input remains inactive",
    ] {
        assert!(
            snow.contains(marker),
            "missing snow contract marker {marker}"
        );
    }
    for marker in [
        "contract_version: 49",
        "INV-RUNOFFPART-033",
        "INV-RUNOFFPART-034",
        "OBL-RUNOFFPART-C-005",
    ] {
        assert!(
            runoff.contains(marker),
            "missing runoff contract marker {marker}"
        );
    }
}

#[test]
fn typed_snow_activation_uses_the_strict_physical_depth_threshold() {
    let threshold_m: f64 = 1.0e-12;
    let just_above_m = f64::from_bits(threshold_m.to_bits() + 1);
    for (snowfall_depth_m, expected_active) in [(threshold_m, false), (just_above_m, true)] {
        let mut hourly = [DirectSnowHourlyForcing::zero(); 24];
        hourly[6] = snowfall_hour(snowfall_depth_m, 0.0);
        let inputs = zero_pack_inputs(&hourly);
        let outcome = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
            .expect("activation-threshold probe must compute");
        assert_eq!(outcome.active_snow_coupling, expected_active);
    }
}

#[test]
fn warm_mean_zero_pack_typed_snow_and_mixed_event_activate_and_close() {
    for (snowfall_depth_m, rain_m) in [(0.10, 0.0), (0.08, 0.002)] {
        let mut hourly = [DirectSnowHourlyForcing::zero(); 24];
        hourly[6] = snowfall_hour(snowfall_depth_m, rain_m);
        let inputs = zero_pack_inputs(&hourly);
        let outcome = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
            .expect("valid warm-day typed snowfall must compute");

        assert!(outcome.active_snow_coupling);
        assert!((outcome.accumulation_m - snowfall_depth_m * 0.1).abs() <= STORAGE_TOLERANCE_M);
        assert!(independent_residual(&inputs, &outcome).abs() <= STORAGE_TOLERANCE_M);
    }
}

#[test]
fn warm_all_rain_zero_pack_remains_inactive() {
    let mut hourly = [DirectSnowHourlyForcing::zero(); 24];
    hourly[6] = DirectSnowHourlyForcing {
        active_precipitation_m: 0.01,
        rain_m: 0.01,
        air_temperature_c: 3.0,
        phase_model: SnowPhasePartitionModel::HarderPomeroyHourly,
        rain_fraction: 1.0,
        snow_fraction: 0.0,
        hydrometeor_temperature_c: Some(2.0),
        ..DirectSnowHourlyForcing::zero()
    };
    let inputs = zero_pack_inputs(&hourly);
    let outcome = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("warm all-rain input must remain valid");

    assert!(!outcome.active_snow_coupling);
    assert_eq!(outcome.accumulation_m.abs().to_bits(), 0.0_f64.to_bits());
    assert!(independent_residual(&inputs, &outcome).abs() <= STORAGE_TOLERANCE_M);
}

#[test]
fn typed_runtime_and_real_consumer_expose_required_non_aliased_fields() {
    let forcing = repo_text(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    );
    let melt = repo_text(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs",
    );
    let projection = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs",
    );
    let consumer = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
    );
    let trace_helper = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs",
    );
    for field in [
        "active_precipitation_m",
        "rain_fraction",
        "snow_fraction",
        "phase_model",
        "hydrometeor_temperature_c",
        "snowfall_swe_m",
        "coe_melt_amelt_m",
        "coe_melt_bmelt_m",
        "coe_melt_cmelt_m",
        "coe_melt_dmelt_m",
        "coe_melt_uncapped_m",
        "coe_melt_cap_adjustment_m",
        "coe_melt_applied_m",
        "modeled_wind_redistribution_m",
    ] {
        assert!(
            forcing.contains(field) || melt.contains(field),
            "typed runtime missing {field}"
        );
        assert!(
            projection.contains(field) || consumer.contains(field) || trace_helper.contains(field),
            "real projection/consumer missing {field}"
        );
    }
    assert!(consumer.contains("openwepp-r7h-direct-production-snow-trace-v4"));
}

#[test]
fn stage3_trace_contract_binds_exact_operands_and_behavior_neutrality() {
    let contract =
        repo_text("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    for binding in [
        "contract_id: SC-SNOWFREEZE-001",
        "REF-SNOWFREEZE-STAGE3-TRACE-CLOSURE",
        "INV-SNOWFREEZE-090",
        "OBL-SNOWFREEZE-P-063",
        "TOL-SNOWFREEZE-015",
        "incoming - routed - retained_delta - refrozen = residual",
        "WAT/HBP/PASS schemas and values",
        "all pre-v4 trace field values",
    ] {
        assert!(
            contract.contains(binding),
            "missing Stage-3 trace contract binding: {binding}"
        );
    }
}

#[test]
fn stage3_liquid_identity_rejects_adjacent_alias_formulas() {
    let incoming_m = 0.021_f64;
    let stage3_routed_m = 0.009_f64;
    let retained_delta_m = 0.004_f64;
    let refrozen_m = 0.006_f64;
    let producer_residual_m = 0.002_f64;
    let top_level_coe_routed_m = 0.007_f64;
    let coe_retained_store_m = 0.003_f64;
    let tolerance_m = 1.0e-9_f64;

    let reconstruct = |routed_m: f64, retained_m: f64, refrozen_multiplier: f64| {
        incoming_m - routed_m - retained_m - refrozen_multiplier * refrozen_m
    };
    let accepted = reconstruct(stage3_routed_m, retained_delta_m, 1.0);
    assert!((accepted - producer_residual_m).abs() <= tolerance_m);

    let aliases = [
        reconstruct(stage3_routed_m, 0.0, 1.0),
        reconstruct(top_level_coe_routed_m, retained_delta_m, 1.0),
        reconstruct(stage3_routed_m, coe_retained_store_m, 1.0),
        reconstruct(stage3_routed_m, retained_delta_m, 2.0),
    ];
    for alias in aliases {
        assert!((alias - producer_residual_m).abs() > tolerance_m);
    }
    assert!(producer_residual_m.abs() > tolerance_m);
}

#[test]
fn schema_v4_real_consumer_names_exact_stage3_and_signed_hour_operands() {
    let carrier = repo_text(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    );
    let consumer = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
    );
    let hourly = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs",
    );
    assert!(consumer.contains("openwepp-r7h-direct-production-snow-trace-v4"));
    for field in [
        "stage3_incoming_liquid_m",
        "stage3_routed_liquid_m",
        "stage3_retained_liquid_delta_m",
        "stage3_refrozen_liquid_m",
        "stage3_liquid_closure_residual_m",
        "stage3_hourly_active_mass_kg_m2",
        "stage3_hourly_active_depth_m",
        "stage3_hourly_active_temperature_c",
        "stage3_hourly_active_cold_content_j_m2",
        "stage3_hourly_lower_present_fraction",
        "stage3_hourly_lower_mass_kg_m2",
        "stage3_hourly_lower_depth_m",
        "stage3_hourly_lower_temperature_c",
        "stage3_hourly_lower_cold_content_j_m2",
    ] {
        assert!(consumer.contains(field), "real consumer missing {field}");
    }
    for field in [
        "wind_m_s",
        "dewpoint_c",
        "canopy_cover_fraction",
        "hourly_air_temperature_c",
        "hourly_radiation_mj_m2",
        "hourly_cloud_fraction",
        "hourly_routed_melt_m",
        "hourly_liquid_holding_capacity_m",
        "hourly_liquid_water_retained_before_m",
        "hourly_liquid_water_retained_after_m",
        "hourly_liquid_water_released_m",
        "hourly_rain_released_m",
        "hourly_sublimation_m",
        "hourly_pack_depth_before_m",
        "hourly_pack_depth_after_m",
        "hourly_pack_density_before_kg_m3",
        "hourly_pack_density_after_kg_m3",
    ] {
        assert!(
            carrier.contains(field),
            "typed signed-hour carrier missing {field}"
        );
        assert!(hourly.contains(field), "hourly formatter missing {field}");
    }
}
