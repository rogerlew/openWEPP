use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowLayerState,
    DirectSnowLiquidPartition, DirectSnowSurfaceEnergyOptions, SnowDensityModel, SnowMeltModel,
    SnowStage3LiquidRoutingModel, SnowSurfaceLongwaveModel, SnowSurfaceSublimationModel,
    Wb11HydrologyKernel,
};

fn inputs(
    longwave_model: SnowSurfaceLongwaveModel,
    sublimation_model: SnowSurfaceSublimationModel,
) -> DirectActiveSnowPartitionInputs {
    let hourly = [DirectSnowHourlyForcing {
        radiation_mj_m2: 0.0,
        air_temperature_c: -5.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
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
        snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
        snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
        stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
        surface_energy_options: DirectSnowSurfaceEnergyOptions {
            longwave_model,
            sublimation_model,
            daily_solar_radiation_mj_m2: 5.0,
            daily_extraterrestrial_radiation_mj_m2: 10.0,
            daylight: true,
            atmospheric_pressure_pa: 101_324.6,
        },
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 0.40,
        coe_boundary_density_kg_m3: 450.0,
        coe_boundary_settle_day_count: 12.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: vec![surface],
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

fn set_single_layer_mass(
    inputs: &mut DirectActiveSnowPartitionInputs,
    mass_swe_m: f64,
    temperature_c: f64,
) {
    let density_kg_m3 = 500.0;
    let depth_m = mass_swe_m * 1_000.0 / density_kg_m3;
    let cold_content_j_m2 = -mass_swe_m * 1_000.0 * 2_100.0 * temperature_c;
    let mut layer = DirectSnowLayerState::new(mass_swe_m, depth_m, density_kg_m3, 12.0);
    layer.temperature_c = temperature_c;
    layer.cold_content_j_m2 = cold_content_j_m2;
    inputs.runtime_swe_m = mass_swe_m;
    inputs.runtime_depth_m = depth_m;
    inputs.runtime_density_kg_m3 = density_kg_m3;
    inputs.coe_boundary_depth_m = depth_m;
    inputs.coe_boundary_density_kg_m3 = density_kg_m3;
    inputs.snow_layers = vec![layer];
}

#[test]
fn minimum_resolved_thermal_mass_preserves_state_and_suspends_exchange() {
    for mass_swe_m in [0.000_999_f64, 0.001_f64] {
        let mut candidate = inputs(
            SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
            SnowSurfaceSublimationModel::NeutralBulkStage3V1,
        );
        set_single_layer_mass(&mut candidate, mass_swe_m, -20.0);
        candidate.snow_layers[0].refrozen_liquid_m = mass_swe_m * 0.1;
        let expected = candidate.snow_layers[0];

        let result =
            Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
                .expect("sub-resolution Stage 3 domain");
        let diagnostics = result.stage3_diagnostics;

        assert!((diagnostics.thermal_domain_suspended_seconds - 86_400.0).abs() <= f64::EPSILON);
        assert!(
            (diagnostics.minimum_unresolved_thermal_mass_kg_m2 - mass_swe_m * 1_000.0).abs()
                <= 1.0e-12
        );
        assert!(diagnostics.shortwave_energy_j_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.longwave_energy_j_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.latent_energy_j_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.vapor_mass_exchange_kg_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.sublimation_m.abs() <= f64::EPSILON);
        assert!(diagnostics.conduction_energy_j_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.surface_energy_j_m2.abs() <= f64::EPSILON);
        assert!(result.routed_melt_m.abs() <= f64::EPSILON);
        assert_eq!(result.snow_layers_after.len(), 1);
        assert!((result.snow_layers_after[0].mass_swe_m - expected.mass_swe_m).abs() <= 1.0e-12);
        assert!(
            (result.snow_layers_after[0].cold_content_j_m2 - expected.cold_content_j_m2).abs()
                <= 1.0e-9
        );
        assert!(
            (result.snow_layers_after[0].temperature_c - expected.temperature_c).abs()
                <= f64::EPSILON
        );
        assert!(
            (result.snow_layers_after[0].refrozen_liquid_m - expected.refrozen_liquid_m).abs()
                <= f64::EPSILON
        );
    }
}

#[test]
fn mass_above_resolved_boundary_resumes_existing_stage3_exchange() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    set_single_layer_mass(&mut candidate, 0.001_001, -20.0);

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect("resolved Stage 3 domain");
    let diagnostics = result.stage3_diagnostics;

    assert!(diagnostics.thermal_domain_suspended_seconds.abs() <= f64::EPSILON);
    assert!(diagnostics.minimum_unresolved_thermal_mass_kg_m2.abs() <= f64::EPSILON);
    assert!(diagnostics.longwave_energy_j_m2.abs() > f64::EPSILON);
    assert!(
        diagnostics
            .hourly_surface_energy
            .iter()
            .any(|hour| hour.substep_count > 0)
    );
}

#[test]
fn authoritative_mass_increase_resumes_from_retained_unresolved_state() {
    let mut unresolved = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    unresolved.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    set_single_layer_mass(&mut unresolved, 0.001, -20.0);
    let suspended =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&unresolved)
            .expect("exact-boundary state must suspend");
    let retained_cold_content_j_m2 = suspended.snow_layers_after[0].cold_content_j_m2;

    let mut resumed = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    resumed.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    let added = DirectSnowLayerState::new(0.000_1, 0.000_2, 500.0, 1.0);
    resumed.snow_layers = suspended.snow_layers_after;
    resumed.snow_layers.insert(0, added);
    resumed.runtime_swe_m = resumed
        .snow_layers
        .iter()
        .map(|layer| layer.mass_swe_m)
        .sum();
    resumed.runtime_depth_m = resumed
        .snow_layers
        .iter()
        .map(|layer| layer.thickness_m)
        .sum();
    resumed.runtime_density_kg_m3 = resumed.runtime_swe_m * 1_000.0 / resumed.runtime_depth_m;
    resumed.coe_boundary_depth_m = resumed.runtime_depth_m;
    resumed.coe_boundary_density_kg_m3 = resumed.runtime_density_kg_m3;

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&resumed)
        .expect("authoritative mass increase must resume the retained state");
    let diagnostics = result.stage3_diagnostics;

    assert!(diagnostics.thermal_domain_suspended_seconds.abs() <= f64::EPSILON);
    assert!((diagnostics.cold_content_before_j_m2 - retained_cold_content_j_m2).abs() <= 1.0e-9);
    assert!(diagnostics.longwave_energy_j_m2.abs() > f64::EPSILON);
    assert!(
        diagnostics
            .hourly_surface_energy
            .iter()
            .any(|hour| hour.substep_count > 0)
    );
}

#[test]
fn invalid_temperature_guard_remains_fail_closed_above_total_mass_boundary() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    set_single_layer_mass(&mut candidate, 0.001_001, -300.0);

    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect_err("resolved mass must retain the absolute-zero guard");

    assert!(error.to_string().contains("must be above absolute zero"));
}

#[test]
fn sub_resolution_lower_control_volume_collapses_to_one_resolved_volume() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    let mut active = DirectSnowLayerState::new(0.130_5, 0.25, 522.0, 12.0);
    active.temperature_c = -8.0;
    active.cold_content_j_m2 = active.mass_swe_m * 1_000.0 * 2_100.0 * 8.0;
    active.liquid_water_m = 0.000_1;
    active.refrozen_liquid_m = 0.000_2;
    let mut lower = DirectSnowLayerState::new(0.000_5, 0.000_5 * 1_000.0 / 522.0, 522.0, 12.0);
    lower.temperature_c = -8.0;
    lower.cold_content_j_m2 = lower.mass_swe_m * 1_000.0 * 2_100.0 * 8.0;
    lower.liquid_water_m = 0.000_000_1;
    lower.refrozen_liquid_m = 0.000_000_2;
    candidate.runtime_swe_m = active.mass_swe_m + lower.mass_swe_m;
    candidate.runtime_depth_m = active.thickness_m + lower.thickness_m;
    candidate.runtime_density_kg_m3 = 522.0;
    candidate.coe_boundary_depth_m = candidate.runtime_depth_m;
    candidate.coe_boundary_density_kg_m3 = candidate.runtime_density_kg_m3;
    candidate.snow_layers = vec![active, lower];
    let expected_mass_m = candidate
        .snow_layers
        .iter()
        .map(|layer| layer.mass_swe_m)
        .sum::<f64>();
    let expected_liquid_m = candidate
        .snow_layers
        .iter()
        .map(|layer| layer.liquid_water_m)
        .sum::<f64>();
    let expected_refrozen_m = candidate
        .snow_layers
        .iter()
        .map(|layer| layer.refrozen_liquid_m)
        .sum::<f64>();
    let expected_cold_content_j_m2 = candidate
        .snow_layers
        .iter()
        .map(|layer| layer.cold_content_j_m2)
        .sum::<f64>();

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect("a sub-resolution lower thermal volume must collapse to one volume");
    let diagnostics = result.stage3_diagnostics;

    assert!(diagnostics.thermal_domain_suspended_seconds.abs() <= f64::EPSILON);
    assert!((diagnostics.lower_thermal_volume_collapsed_seconds - 86_400.0).abs() <= f64::EPSILON);
    assert!((diagnostics.minimum_collapsed_lower_mass_kg_m2 - 0.5).abs() <= 1.0e-6);
    assert!(diagnostics.surface_energy_j_m2.abs() > f64::EPSILON);
    assert!(diagnostics.conduction_energy_j_m2.abs() <= f64::EPSILON);
    assert!((diagnostics.cold_content_before_j_m2 - expected_cold_content_j_m2).abs() <= 1.0e-9);
    assert!(
        (result
            .snow_layers_after
            .iter()
            .map(|layer| layer.mass_swe_m)
            .sum::<f64>()
            - expected_mass_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (result
            .snow_layers_after
            .iter()
            .map(|layer| layer.liquid_water_m)
            .sum::<f64>()
            - expected_liquid_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (result
            .snow_layers_after
            .iter()
            .map(|layer| layer.refrozen_liquid_m)
            .sum::<f64>()
            - expected_refrozen_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        diagnostics
            .hourly_surface_energy
            .iter()
            .all(|hour| hour.lower_layer_present_fraction.abs() <= f64::EPSILON)
    );
    assert_stage3_energy_reconstructs(&result);
}

#[test]
fn one_kg_m2_lower_control_volume_remains_a_two_volume_solve() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    let mut active = DirectSnowLayerState::new(0.130_5, 0.25, 522.0, 12.0);
    active.temperature_c = -8.0;
    active.cold_content_j_m2 = active.mass_swe_m * 1_000.0 * 2_100.0 * 8.0;
    let mut lower = DirectSnowLayerState::new(0.001, 0.001 * 1_000.0 / 522.0, 522.0, 12.0);
    lower.temperature_c = -8.0;
    lower.cold_content_j_m2 = lower.mass_swe_m * 1_000.0 * 2_100.0 * 8.0;
    candidate.runtime_swe_m = active.mass_swe_m + lower.mass_swe_m;
    candidate.runtime_depth_m = active.thickness_m + lower.thickness_m;
    candidate.runtime_density_kg_m3 = 522.0;
    candidate.coe_boundary_depth_m = candidate.runtime_depth_m;
    candidate.coe_boundary_density_kg_m3 = candidate.runtime_density_kg_m3;
    candidate.snow_layers = vec![active, lower];

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect("an exact 1 kg m-2 lower volume remains resolved");
    let diagnostics = result.stage3_diagnostics;

    assert!(diagnostics.thermal_domain_suspended_seconds.abs() <= f64::EPSILON);
    assert!(diagnostics.lower_thermal_volume_collapsed_seconds.abs() <= f64::EPSILON);
    assert!(
        diagnostics
            .hourly_surface_energy
            .iter()
            .any(|hour| hour.lower_layer_present_fraction > 0.0)
    );
}

#[test]
fn deep_unresolved_pack_preserves_topology_and_stored_state_before_partition() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::NeutralBulkStage3V1,
    );
    candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    let mut layer = DirectSnowLayerState::new(0.001, 0.5, 2.0, 12.0);
    layer.temperature_c = -5.0;
    layer.cold_content_j_m2 = 0.0;
    layer.liquid_water_m = 0.000_01;
    layer.refrozen_liquid_m = 0.000_02;
    candidate.runtime_swe_m = layer.mass_swe_m;
    candidate.runtime_depth_m = layer.thickness_m;
    candidate.runtime_density_kg_m3 = layer.density_kg_m3;
    candidate.coe_boundary_depth_m = layer.thickness_m;
    candidate.coe_boundary_density_kg_m3 = layer.density_kg_m3;
    candidate.snow_layers = vec![layer];

    let mut baseline = candidate.clone();
    baseline.stage3_liquid_routing_model = SnowStage3LiquidRoutingModel::Disabled;
    baseline.surface_energy_options.longwave_model = SnowSurfaceLongwaveModel::Disabled;
    baseline.surface_energy_options.sublimation_model = SnowSurfaceSublimationModel::Disabled;
    let projected = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&baseline)
        .expect("CoE projection baseline");

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect("unresolved total mass must branch before thermal partition");

    assert_eq!(result.snow_layers_after, projected.snow_layers_after);
    assert!(
        (result.stage3_diagnostics.thermal_domain_suspended_seconds - 86_400.0).abs()
            <= f64::EPSILON
    );
}

fn assert_stage3_energy_reconstructs(result: &DirectSnowLiquidPartition) {
    let diagnostics = result.stage3_diagnostics;
    let independently_reconstructed_energy_residual = diagnostics.surface_energy_j_m2
        + diagnostics.conduction_energy_j_m2
        + diagnostics.latent_refreeze_energy_j_m2
        + diagnostics.cold_content_export_j_m2
        - (diagnostics.cold_content_before_j_m2 - diagnostics.cold_content_after_j_m2);
    assert!(independently_reconstructed_energy_residual.abs() <= 1.0e-6);
    assert!(
        (independently_reconstructed_energy_residual - diagnostics.energy_closure_residual_j_m2)
            .abs()
            <= 1.0e-12
    );
    assert!(diagnostics.mass_latent_identity_residual_j_m2.abs() <= 1.0e-6);
    for hour in diagnostics.hourly_surface_energy {
        let longwave_residual =
            hour.subcanopy_longwave_w_m2 - hour.outgoing_longwave_w_m2 - hour.net_longwave_w_m2;
        assert!(longwave_residual.abs() <= 1.0e-10);
        let latent_residual = hour.latent_flux_w_m2 * 3_600.0
            - hour.vapor_mass_exchange_kg_m2 * hour.latent_heat_j_kg;
        assert!(latent_residual.abs() <= 1.0e-6);
        let surface_energy_residual = hour.potential_surface_energy_j_m2
            - (hour.net_shortwave_w_m2 + hour.net_longwave_w_m2 + hour.latent_flux_w_m2) * 3_600.0;
        assert!(surface_energy_residual.abs() <= 1.0e-6);
        assert!(hour.unused_positive_energy_j_m2 >= 0.0);
        assert!(hour.canopy_temperature_equals_air);
        if hour.substep_count > 0 {
            assert!(hour.active_layer_depth_m > 0.0);
            if diagnostics.lower_thermal_volume_collapsed_seconds > 0.0 {
                assert!(hour.lower_layer_present_fraction.abs() <= f64::EPSILON);
            } else {
                assert!(hour.active_layer_depth_m <= 0.25 + 1.0e-12);
            }
            assert!([60.0, 900.0, 3_600.0].contains(&hour.minimum_substep_seconds));
            assert!(hour.maximum_active_energy_closure_residual_j_m2 <= 1.0e-6);
            assert!(hour.maximum_lower_energy_closure_residual_j_m2 <= 1.0e-6);
            assert!(hour.maximum_conduction_cancellation_residual_j_m2 <= 1.0e-12);
        }
    }
}

#[test]
fn orthogonal_cells_share_stage3_and_compose_additively() {
    let baseline = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs(
        SnowSurfaceLongwaveModel::Disabled,
        SnowSurfaceSublimationModel::Disabled,
    ))
    .expect("baseline cell");
    let longwave = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    ))
    .expect("longwave cell");
    let sublimation =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs(
            SnowSurfaceLongwaveModel::Disabled,
            SnowSurfaceSublimationModel::NeutralBulkStage3V1,
        ))
        .expect("sublimation cell");
    let combined = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::NeutralBulkStage3V1,
    ))
    .expect("combined cell");

    assert!(baseline.stage3_diagnostics.enabled);
    assert!(baseline.stage3_diagnostics.longwave_energy_j_m2.abs() <= f64::EPSILON);
    assert!(baseline.stage3_diagnostics.sublimation_m.abs() <= f64::EPSILON);
    assert!(longwave.stage3_diagnostics.longwave_energy_j_m2.abs() > f64::EPSILON);
    assert!(sublimation.stage3_diagnostics.sublimation_m > 0.0);
    assert!(combined.stage3_diagnostics.sublimation_m > 0.0);
    assert!(combined.stage3_diagnostics.longwave_energy_j_m2.abs() > f64::EPSILON);
    assert!(combined.stage3_diagnostics.latent_energy_j_m2.abs() > f64::EPSILON);
    assert!(combined.runtime_swe_after_m < longwave.runtime_swe_after_m);
    assert!(
        (baseline.runtime_swe_after_m
            - sublimation.runtime_swe_after_m
            - sublimation.stage3_diagnostics.sublimation_m)
            .abs()
            <= 1.0e-9
    );
    assert!(
        (sublimation
            .snow_layers_after
            .iter()
            .map(|layer| layer.mass_swe_m)
            .sum::<f64>()
            - sublimation.runtime_swe_after_m)
            .abs()
            <= 1.0e-9
    );
    assert!((sublimation.routed_melt_m - baseline.routed_melt_m).abs() <= 1.0e-12);
    assert!((sublimation.snowpack_swe_loss_m - baseline.snowpack_swe_loss_m).abs() <= 1.0e-12);
    assert!(
        (sublimation.stage3_diagnostics.incoming_liquid_m
            - baseline.stage3_diagnostics.incoming_liquid_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (sublimation.stage3_diagnostics.routed_liquid_m
            - baseline.stage3_diagnostics.routed_liquid_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (sublimation.stage3_diagnostics.retained_liquid_m
            - baseline.stage3_diagnostics.retained_liquid_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (sublimation.stage3_diagnostics.refrozen_liquid_m
            - baseline.stage3_diagnostics.refrozen_liquid_m)
            .abs()
            <= 1.0e-12
    );
    for result in [&baseline, &longwave, &sublimation, &combined] {
        assert_stage3_energy_reconstructs(result);
    }
}

#[test]
fn enabled_mechanism_requires_stage3_provider() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate.stage3_liquid_routing_model = SnowStage3LiquidRoutingModel::Disabled;
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect_err("missing provider must fail");
    assert!(
        error
            .to_string()
            .contains("surface_energy_requires_stage3_provider")
    );
}

#[test]
fn polar_night_and_double_sublimation_fail_closed() {
    let mut polar = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    polar.surface_energy_options.daylight = false;
    polar
        .surface_energy_options
        .daily_extraterrestrial_radiation_mj_m2 = 0.0;
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&polar)
        .expect_err("polar night must fail");
    assert!(error.to_string().contains("snow.cloud_forcing_unavailable"));

    let mut duplicate = inputs(
        SnowSurfaceLongwaveModel::Disabled,
        SnowSurfaceSublimationModel::NeutralBulkStage3V1,
    );
    duplicate.snow_melt_model = SnowMeltModel::CoeOpenSublimationStageBV1;
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&duplicate)
        .expect_err("duplicate sublimation must fail");
    assert!(
        error
            .to_string()
            .contains("incompatible_sublimation_selectors")
    );
}

#[test]
fn exhausted_surface_layer_promotes_lower_layer_and_preserves_aggregate_identity() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::Disabled,
        SnowSurfaceSublimationModel::NeutralBulkStage3V1,
    );
    candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    candidate.wind_m_s = 500.0;
    let mut top = DirectSnowLayerState::new(0.000_001, 0.000_004, 250.0, 1.0);
    top.temperature_c = -5.0;
    top.cold_content_j_m2 = top.mass_swe_m * 1_000.0 * 2_100.0 * 5.0;
    let mut lower = DirectSnowLayerState::new(0.179_999, 0.359_998, 500.0, 12.0);
    lower.temperature_c = -8.0;
    lower.cold_content_j_m2 = lower.mass_swe_m * 1_000.0 * 2_100.0 * 8.0;
    candidate.snow_layers = vec![top, lower];
    candidate.runtime_depth_m = top.thickness_m + lower.thickness_m;
    candidate.runtime_density_kg_m3 = candidate.runtime_swe_m * 1_000.0 / candidate.runtime_depth_m;
    candidate.coe_boundary_depth_m = candidate.runtime_depth_m;
    candidate.coe_boundary_density_kg_m3 = candidate.runtime_density_kg_m3;

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect("layer exhaustion must promote lower snow");
    // The persistent depositional layers may contain one additional split at
    // the 0.25 m active/lower thermal boundary.
    assert!(result.snow_layers_after.len() <= 3);
    assert!(result.snow_layers_after[0].mass_swe_m > 0.0);
    let reconstructed_density = result.runtime_swe_after_m * 1_000.0 / result.runtime_depth_after_m;
    assert!((result.runtime_density_after_kg_m3 - reconstructed_density).abs() <= 1.0e-9);
}

#[test]
fn shallow_pack_uses_the_whole_active_thermal_volume_not_the_event_layer() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::Disabled,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    candidate.hourly[0].radiation_mj_m2 = 0.20;
    candidate.runtime_swe_m = 0.040;
    candidate.runtime_depth_m = 0.200;
    candidate.runtime_density_kg_m3 = 200.0;
    candidate.coe_boundary_depth_m = candidate.runtime_depth_m;
    candidate.coe_boundary_density_kg_m3 = candidate.runtime_density_kg_m3;

    let mut event = DirectSnowLayerState::new(0.000_010, 0.000_040, 250.0, 1.0);
    event.temperature_c = -5.0;
    event.cold_content_j_m2 = event.mass_swe_m * 1_000.0 * 2_100.0 * 5.0;
    let event_layer_cold_content_j_m2 = event.cold_content_j_m2;
    let mut established = DirectSnowLayerState::new(0.039_990, 0.199_960, 199.99, 12.0);
    established.temperature_c = -5.0;
    established.cold_content_j_m2 = established.mass_swe_m * 1_000.0 * 2_100.0 * 5.0;
    candidate.snow_layers = vec![event, established];

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect("a shallow pack must use one active thermal control volume");
    let applied_first_hour =
        result.stage3_diagnostics.hourly_surface_energy[0].applied_surface_energy_j_m2;

    assert!(applied_first_hour > event_layer_cold_content_j_m2 * 100.0);
    assert!(
        result.stage3_diagnostics.hourly_surface_energy[0].unused_positive_energy_j_m2 <= 1.0e-9
    );
    assert!(result.stage3_diagnostics.cold_content_after_j_m2 > 0.0);
    assert_stage3_energy_reconstructs(&result);
}

#[test]
fn unequal_depositional_temperatures_project_to_persistent_active_and_lower_states() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::Disabled,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    let mut surface = DirectSnowLayerState::new(0.050, 0.100, 500.0, 3.0);
    surface.temperature_c = -2.0;
    surface.cold_content_j_m2 = surface.mass_swe_m * 1_000.0 * 2_100.0 * 2.0;
    let mut basal = DirectSnowLayerState::new(0.130, 0.300, 433.333_333_333, 12.0);
    basal.temperature_c = -10.0;
    basal.cold_content_j_m2 = basal.mass_swe_m * 1_000.0 * 2_100.0 * 10.0;
    candidate.snow_layers = vec![surface, basal];

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect("unequal depositional temperatures must form coupled thermal states");
    let first_hour = result.stage3_diagnostics.hourly_surface_energy[0];

    assert!(first_hour.lower_layer_present_fraction > 0.0);
    assert!(first_hour.active_layer_temperature_c < -2.0);
    assert!(first_hour.active_layer_temperature_c > -10.0);
    assert!(first_hour.active_lower_conduction_w_m2 < 0.0);
    assert_eq!(first_hour.substep_count, 1);
    let reconstructed_g0_w_m2 = 2.0
        * first_hour.active_layer_effective_conductivity_w_m_k
        * first_hour.lower_layer_effective_conductivity_w_m_k
        * (first_hour.lower_layer_temperature_c - first_hour.active_layer_temperature_c)
        / (first_hour.lower_layer_effective_conductivity_w_m_k * first_hour.active_layer_depth_m
            + first_hour.active_layer_effective_conductivity_w_m_k
                * first_hour.lower_layer_depth_m);
    assert!((first_hour.active_lower_conduction_w_m2 - reconstructed_g0_w_m2).abs() <= 1.0e-10);
    assert!(
        (first_hour.requested_active_lower_conduction_w_m2
            - first_hour.active_lower_conduction_w_m2
            - first_hour.rejected_active_lower_conduction_w_m2)
            .abs()
            <= 1.0e-12
    );
    let peak_reconstructed_g0_w_m2 = 2.0
        * first_hour.peak_substep_active_conductivity_w_m_k
        * first_hour.peak_substep_lower_conductivity_w_m_k
        * (first_hour.peak_substep_lower_temperature_c
            - first_hour.peak_substep_active_temperature_c)
        / (first_hour.peak_substep_lower_conductivity_w_m_k
            * first_hour.peak_substep_active_depth_m
            + first_hour.peak_substep_active_conductivity_w_m_k
                * first_hour.peak_substep_lower_depth_m);
    assert!(
        (first_hour.peak_substep_requested_g0_w_m2 - peak_reconstructed_g0_w_m2).abs() <= 1.0e-10
    );
    assert!(
        (first_hour.peak_substep_requested_g0_w_m2
            - first_hour.peak_substep_applied_g0_w_m2
            - first_hour.peak_substep_rejected_g0_w_m2)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (first_hour.peak_substep_active_resistance_m2_k_w
            - first_hour.peak_substep_active_depth_m
                / first_hour.peak_substep_active_conductivity_w_m_k)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (first_hour.peak_substep_lower_resistance_m2_k_w
            - first_hour.peak_substep_lower_depth_m
                / first_hour.peak_substep_lower_conductivity_w_m_k)
            .abs()
            <= 1.0e-12
    );
    assert!(first_hour.atmospheric_pressure_pa > 0.0);
    assert!(first_hour.maximum_active_energy_closure_residual_j_m2 <= 1.0e-6);
    assert!(first_hour.maximum_lower_energy_closure_residual_j_m2 <= 1.0e-6);

    let mut active_temperatures = Vec::new();
    let mut lower_temperatures = Vec::new();
    let mut depth_m = 0.0;
    for layer in &result.snow_layers_after {
        depth_m += layer.thickness_m;
        if depth_m <= 0.25 + 1.0e-9 {
            active_temperatures.push(layer.temperature_c);
        } else {
            lower_temperatures.push(layer.temperature_c);
        }
    }
    assert!(active_temperatures.len() >= 2);
    assert!(!lower_temperatures.is_empty());
    assert!(
        active_temperatures
            .windows(2)
            .all(|pair| { (pair[0] - pair[1]).abs() <= 1.0e-9 })
    );
    assert_stage3_energy_reconstructs(&result);
}
