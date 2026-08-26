use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowDiagnosticCapture, DirectSnowHourlyForcing,
    DirectSnowLayerState, DirectSnowLiquidPartition, DirectSnowSurfaceEnergyOptions,
    DirectSnowTurbulentGeometry, SnowDensityModel, SnowMeltModel, SnowPhasePartitionModel,
    SnowStage3EvaluationOperator, SnowStage3LiquidRoutingModel, SnowSurfaceLongwaveModel,
    SnowSurfaceSublimationModel, Wb11HydrologyKernel, snow_density_compaction_v1_constants,
};

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

fn stage3_diagnostics(
    result: &DirectSnowLiquidPartition,
) -> &openwepp_hillslope_orchestrator::DirectSnowStage3Diagnostics {
    &result
        .verbose_diagnostics
        .as_deref()
        .expect("public compatibility solve requests verbose diagnostics")
        .stage3
}

fn evaluation_diagnostics(
    result: &openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationResult,
) -> &openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationDiagnostics {
    result
        .evaluation
        .as_ref()
        .expect("enabled request carries evaluation diagnostics")
}

fn independently_compacted_density_after_day(
    initial_density_kg_m3: f64,
    overburden_kg_m2: f64,
    snow_temperature_c: f64,
) -> f64 {
    let constants = snow_density_compaction_v1_constants();
    let mut density_kg_m3 = initial_density_kg_m3;
    for _ in 0..24 {
        let rate = constants.compaction_rate_cos_amplitude
            * (std::f64::consts::PI * overburden_kg_m2 / constants.dry_compaction_swe_max_kg_m2)
                .cos()
            + constants.compaction_rate_offset;
        let c11 = (-constants.ptm_density_decay_m3_per_kg
            * (density_kg_m3 - constants.ptm_density_threshold_kg_m3))
            .exp();
        let freeze_minus_snow_temp = -snow_temperature_c.min(0.0);
        let destructive_metamorphism = constants.ptm_rate_per_hour
            * c11
            * (-constants.ptm_temperature_decay_per_c * freeze_minus_snow_temp).exp()
            / rate;
        let overburden_compaction = constants.poc_rate_per_hour
            * (-constants.poc_temperature_decay_per_c * freeze_minus_snow_temp).exp()
            * overburden_kg_m2
            * (-constants.poc_density_decay * (density_kg_m3 / 1_000.0)).exp()
            / rate;
        density_kg_m3 = (density_kg_m3
            + constants.dry_compaction_multiplier
                * (destructive_metamorphism + overburden_compaction)
                * density_kg_m3)
            .min(constants.dry_compaction_max_density_kg_m3);
    }
    density_kg_m3
}

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
            turbulent_geometry: DirectSnowTurbulentGeometry::CLIGEN_V1,
            complete_carrier_shadow: false,
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
        let diagnostics = stage3_diagnostics(&result);

        assert!((diagnostics.thermal_domain_suspended_seconds - 86_400.0).abs() <= f64::EPSILON);
        assert!(
            (diagnostics.minimum_unresolved_thermal_mass_kg_m2 - mass_swe_m * 1_000.0).abs()
                <= 1.0e-12
        );
        assert!(diagnostics.shortwave_energy_j_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.longwave_energy_j_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.latent_energy_j_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.vapor_mass_exchange_kg_m2.abs() <= f64::EPSILON);
        assert!(result.stage3_outcome().sublimation_m.abs() <= f64::EPSILON);
        assert!(diagnostics.conduction_energy_j_m2.abs() <= f64::EPSILON);
        assert!(diagnostics.surface_energy_j_m2.abs() <= f64::EPSILON);
        assert!(result.solid_to_liquid_ledger().liquid_handoff_m.abs() <= f64::EPSILON);
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
    let diagnostics = stage3_diagnostics(&result);

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
    let diagnostics = stage3_diagnostics(&result);

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
fn production_density_handoff_obeys_exact_mass_lifecycle_sides() {
    let boundary_swe_m = 1.0e-12_f64;
    for (fragment_mass_swe_m, expected_retained) in [
        (boundary_swe_m.next_down(), false),
        (boundary_swe_m, false),
        (boundary_swe_m.next_up(), true),
    ] {
        let mut candidate = inputs(
            SnowSurfaceLongwaveModel::Disabled,
            SnowSurfaceSublimationModel::Disabled,
        );
        candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
        let established = DirectSnowLayerState::new(0.18, 0.40, 450.0, 12.0);
        let fragment = DirectSnowLayerState::new(
            fragment_mass_swe_m,
            fragment_mass_swe_m * 1_000.0 / 500.0,
            500.0,
            7.0,
        );
        candidate.snow_layers = vec![established, fragment];
        candidate.runtime_swe_m = 0.18 + fragment_mass_swe_m;
        candidate.runtime_depth_m = 0.40 + fragment.thickness_m;
        candidate.runtime_density_kg_m3 =
            candidate.runtime_swe_m * 1_000.0 / candidate.runtime_depth_m;
        candidate.coe_boundary_depth_m = candidate.runtime_depth_m;
        candidate.coe_boundary_density_kg_m3 = candidate.runtime_density_kg_m3;

        let result =
            Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
                .expect("exact lifecycle sides must remain valid production states");

        assert_eq!(
            result
                .snow_layers_after
                .iter()
                .any(|layer| layer.mass_swe_m.to_bits() == fragment_mass_swe_m.to_bits()),
            expected_retained,
            "fragment SWE {fragment_mass_swe_m:e} around boundary {boundary_swe_m:e}: {:?}",
            result.snow_layers_after,
        );
    }
}

#[test]
fn density_handoff_retains_captured_subnanometer_swe_fragments_and_state() {
    for (fragment_mass_swe_m, fragment_depth_m, fragment_density_kg_m3) in [
        (
            5.260_584_353_128_359e-10,
            1.007_774_780_292_791_7e-9,
            521.999_999_999_998_6,
        ),
        (
            5.267_347_169_024_66e-10,
            1.088_162_587_814_523e-9,
            484.058_837_163_631_5,
        ),
    ] {
        let mut candidate = inputs(
            SnowSurfaceLongwaveModel::Disabled,
            SnowSurfaceSublimationModel::Disabled,
        );
        candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;

        let mut established = DirectSnowLayerState::new(0.13, 0.25, 520.0, 12.0);
        established.temperature_c = -5.0;
        established.cold_content_j_m2 = established.mass_swe_m * 1_000.0 * 2_100.0 * 5.0;

        let mut fragment = DirectSnowLayerState::new(
            fragment_mass_swe_m,
            fragment_depth_m,
            fragment_density_kg_m3,
            7.0,
        );
        fragment.temperature_c = -5.0;
        fragment.liquid_water_m = fragment_mass_swe_m * 0.1;
        fragment.refrozen_liquid_m = fragment_mass_swe_m * 0.2;
        fragment.cold_content_j_m2 = fragment_mass_swe_m * 1_000.0 * 2_100.0 * 5.0;

        candidate.snow_layers = vec![established, fragment];
        candidate.runtime_swe_m = candidate
            .snow_layers
            .iter()
            .map(|layer| layer.mass_swe_m)
            .sum();
        candidate.runtime_depth_m = candidate
            .snow_layers
            .iter()
            .map(|layer| layer.thickness_m)
            .sum();
        candidate.runtime_density_kg_m3 =
            candidate.runtime_swe_m * 1_000.0 / candidate.runtime_depth_m;
        candidate.coe_boundary_depth_m = candidate.runtime_depth_m;
        candidate.coe_boundary_density_kg_m3 = candidate.runtime_density_kg_m3;

        let result =
            Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
                .expect("represented fragment must survive density handoff");

        assert_eq!(result.snow_layers_after.len(), 2);
        let retained = result.snow_layers_after[1];
        let expected_density_kg_m3 = independently_compacted_density_after_day(
            fragment_density_kg_m3,
            established.mass_swe_m * 1_000.0,
            -5.0,
        )
        .min(522.0);
        let expected_depth_m = fragment_mass_swe_m * 1_000.0 / expected_density_kg_m3;
        assert!((retained.mass_swe_m - fragment_mass_swe_m).abs() <= 1.0e-15);
        assert!(
            (retained.thickness_m - expected_depth_m).abs() <= 1.0e-15,
            "retained depth {} != independently reconstructed {expected_depth_m}; density {} vs {expected_density_kg_m3}",
            retained.thickness_m,
            retained.density_kg_m3,
        );
        assert!((retained.density_kg_m3 - expected_density_kg_m3).abs() <= 1.0e-12);
        assert!((retained.liquid_water_m - fragment.liquid_water_m).abs() <= 1.0e-15);
        assert!((retained.refrozen_liquid_m - fragment.refrozen_liquid_m).abs() <= 1.0e-15);
        assert!((retained.cold_content_j_m2 - fragment.cold_content_j_m2).abs() <= 1.0e-15);
        assert!(
            (retained.settle_day_count - (fragment.settle_day_count + 1.0)).abs() <= f64::EPSILON
        );
        let mass_sum_m = result
            .snow_layers_after
            .iter()
            .map(|layer| layer.mass_swe_m)
            .sum::<f64>();
        let depth_sum_m = result
            .snow_layers_after
            .iter()
            .map(|layer| layer.thickness_m)
            .sum::<f64>();
        assert!((mass_sum_m - result.runtime_swe_after_m).abs() <= 1.0e-9);
        assert!((depth_sum_m - result.runtime_depth_after_m).abs() <= 1.0e-9);
    }
}

#[test]
fn density_handoff_still_rejects_material_depth_aggregate_mismatch() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::Disabled,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    candidate.runtime_depth_m += 1.0e-9_f64.next_up();
    candidate.coe_boundary_depth_m = candidate.runtime_depth_m;

    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&candidate)
        .expect_err("material physical-depth mismatch must remain typed");

    assert!(error.to_string().contains("prior_layers.thickness_m"));
    assert!(error.to_string().contains("does not match expected"));
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
    let diagnostics = stage3_diagnostics(&result);

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
    let diagnostics = stage3_diagnostics(&result);

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
        (stage3_diagnostics(&result).thermal_domain_suspended_seconds - 86_400.0).abs()
            <= f64::EPSILON
    );
}

fn assert_stage3_energy_reconstructs(result: &DirectSnowLiquidPartition) {
    let diagnostics = stage3_diagnostics(result);
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

    assert!(baseline.stage3_outcome().enabled);
    assert!(stage3_diagnostics(&baseline).longwave_energy_j_m2.abs() <= f64::EPSILON);
    assert!(baseline.stage3_outcome().sublimation_m.abs() <= f64::EPSILON);
    assert!(stage3_diagnostics(&longwave).longwave_energy_j_m2.abs() > f64::EPSILON);
    assert!(sublimation.stage3_outcome().sublimation_m > 0.0);
    assert!(combined.stage3_outcome().sublimation_m > 0.0);
    assert!(stage3_diagnostics(&combined).longwave_energy_j_m2.abs() > f64::EPSILON);
    assert!(stage3_diagnostics(&combined).latent_energy_j_m2.abs() > f64::EPSILON);
    assert!(combined.runtime_swe_after_m < longwave.runtime_swe_after_m);
    assert!(
        (baseline.runtime_swe_after_m
            - sublimation.runtime_swe_after_m
            - sublimation.stage3_outcome().sublimation_m)
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
    assert!(
        (sublimation.solid_to_liquid_ledger().liquid_handoff_m
            - baseline.solid_to_liquid_ledger().liquid_handoff_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (sublimation.solid_to_liquid_ledger().snowpack_swe_loss_m
            - baseline.solid_to_liquid_ledger().snowpack_swe_loss_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (sublimation.liquid_disposition_ledger().incoming_liquid_m
            - baseline.liquid_disposition_ledger().incoming_liquid_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (sublimation.liquid_disposition_ledger().routed_liquid_m
            - baseline.liquid_disposition_ledger().routed_liquid_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (sublimation
            .liquid_disposition_ledger()
            .retained_liquid_delta_m
            - baseline.liquid_disposition_ledger().retained_liquid_delta_m)
            .abs()
            <= 1.0e-12
    );
    assert!(
        (sublimation.liquid_disposition_ledger().refrozen_liquid_m
            - baseline.liquid_disposition_ledger().refrozen_liquid_m)
            .abs()
            <= 1.0e-12
    );
    for result in [&baseline, &longwave, &sublimation, &combined] {
        assert_stage3_energy_reconstructs(result);
    }
}

#[test]
fn complete_carrier_shadow_is_noninterfering_and_uses_typed_turbulence() {
    let authoritative_inputs = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    let shadow_inputs = authoritative_inputs.clone();

    let authoritative =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&authoritative_inputs)
            .expect("authoritative compatibility result");
    let shadow = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &shadow_inputs,
        SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
    )
    .expect("complete carrier shadow result");

    assert_eq!(
        shadow.authoritative.runtime_swe_after_m.to_bits(),
        authoritative.runtime_swe_after_m.to_bits()
    );
    assert_eq!(
        shadow.authoritative.runtime_depth_after_m.to_bits(),
        authoritative.runtime_depth_after_m.to_bits()
    );
    assert_eq!(
        shadow.authoritative.snow_layers_after,
        authoritative.snow_layers_after
    );
    assert_eq!(
        shadow.authoritative.solid_to_liquid_ledger(),
        authoritative.solid_to_liquid_ledger()
    );
    assert_eq!(
        shadow.authoritative.liquid_disposition_ledger(),
        authoritative.liquid_disposition_ledger()
    );
    let mut public_shadow = shadow.authoritative.clone();
    let mut public_authoritative = authoritative.clone();
    public_shadow.verbose_diagnostics = None;
    public_authoritative.verbose_diagnostics = None;
    assert_eq!(
        public_shadow, public_authoritative,
        "evaluation-only diagnostics must be the sole partition difference"
    );

    let evaluation = evaluation_diagnostics(&shadow);
    assert!(
        evaluation
            .hourly
            .iter()
            .all(|hour| hour.complete_carrier_evaluated)
    );
    assert!(evaluation.hourly.iter().any(|hour| {
        hour.sensible_flux_w_m2.abs() > f64::EPSILON
            && hour.latent_flux_w_m2.abs() > f64::EPSILON
            && hour.complete_energy_j_m2.abs() > f64::EPSILON
    }));
    for hour in evaluation
        .hourly
        .iter()
        .filter(|hour| hour.complete_carrier_evaluated)
    {
        let reconstructed = hour.cold_energy_change_j_m2
            + 333_600.0 * hour.melt_kg_m2
            + hour.unallocated_after_exhaustion_j_m2;
        assert!((hour.complete_energy_j_m2 - reconstructed).abs() <= 1.0e-6);
        assert!(hour.energy_closure_residual_j_m2.abs() <= 1.0e-6);
    }
    assert!(evaluation.complete_arm_maximum_thermodynamic_residual_j_m2 <= 1.0e-6);
    assert_eq!(
        evaluation.operator,
        SnowStage3EvaluationOperator::SequentialResolvedShadowV1
    );
    assert_eq!(evaluation.claim_class, "bounded_response_experiment");
    assert_eq!(evaluation.arm_count, 1);
    assert!(evaluation.coverage_fraction > 0.0 && evaluation.coverage_fraction <= 1.0);
    assert_ne!(evaluation.non_formulation_fingerprint, 0);
    assert!(!evaluation.surface_arm_applicable);
    assert_eq!(
        evaluation.surface_arm_total_j_m2.to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn same_state_pair_has_identical_support_and_reconstructable_named_arms() {
    let candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    let baseline = candidate.clone();

    let expected = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&baseline)
        .expect("paired baseline");
    let observed = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &candidate,
        SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
    )
    .expect("same-state pair");
    assert_eq!(
        observed.authoritative.runtime_swe_after_m.to_bits(),
        expected.runtime_swe_after_m.to_bits()
    );
    assert_eq!(
        observed.authoritative.snow_layers_after,
        expected.snow_layers_after
    );
    assert_eq!(
        observed.authoritative.solid_to_liquid_ledger(),
        expected.solid_to_liquid_ledger()
    );
    assert_eq!(
        observed.authoritative.liquid_disposition_ledger(),
        expected.liquid_disposition_ledger()
    );

    let evaluation = evaluation_diagnostics(&observed);
    assert_eq!(
        evaluation.operator,
        SnowStage3EvaluationOperator::SameStatePairedCarrierV1
    );
    assert_eq!(evaluation.pairing_id, Some("stage3_carrier_pair_v1"));
    assert_eq!(
        evaluation.arm_ids,
        ["stage3_surface_energy_v1", "stage3_complete_carrier_v1"]
    );
    assert_eq!(evaluation.arm_count, 2);
    assert_eq!(
        evaluation.requested_seconds.to_bits(),
        86_400.0_f64.to_bits()
    );
    assert_eq!(
        evaluation.evaluated_seconds.to_bits(),
        86_400.0_f64.to_bits()
    );
    assert_eq!(evaluation.coverage_fraction.to_bits(), 1.0_f64.to_bits());
    assert!(evaluation.surface_arm_applicable);
    assert_eq!(
        evaluation.surface_arm_non_formulation_fingerprint,
        evaluation.complete_arm_non_formulation_fingerprint
    );
    assert!(!evaluation.surface_arm_sensible_applicable);
    assert!(!evaluation.surface_arm_advected_applicable);
    assert!(!evaluation.complete_arm_internal_conduction_applicable);
    assert!(!evaluation.complete_arm_cold_content_export_applicable);
    assert!(!evaluation.complete_arm_available_ice_applicable);
    let surface_reconstructed = evaluation.surface_arm_shortwave_j_m2
        + evaluation.surface_arm_longwave_j_m2
        + evaluation.surface_arm_latent_j_m2;
    assert!((evaluation.surface_arm_total_j_m2 - surface_reconstructed).abs() <= 1.0e-6);
    let complete_reconstructed = evaluation.complete_arm_shortwave_j_m2
        + evaluation.complete_arm_longwave_j_m2
        + evaluation.complete_arm_sensible_j_m2
        + evaluation.complete_arm_latent_j_m2
        + evaluation.complete_arm_advected_j_m2;
    assert!((evaluation.complete_arm_total_j_m2 - complete_reconstructed).abs() <= 1.0e-6);
    assert!(evaluation.complete_arm_component_residual_j_m2.abs() <= 1.0e-6);
    assert!(evaluation.hourly.iter().all(|hour| {
        hour.requested_seconds.to_bits() == 3_600.0_f64.to_bits()
            && hour.evaluated_seconds.to_bits() == 3_600.0_f64.to_bits()
    }));
}

#[test]
fn complete_carrier_converts_geometric_snowfall_to_water_mass_once() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate.hyetograph_rainfall_m = 0.001;
    candidate.hourly[0] = DirectSnowHourlyForcing {
        active_precipitation_m: 0.001,
        snowfall_m: 0.01,
        air_temperature_c: -5.0,
        snow_fraction: 1.0,
        hydrometeor_temperature_c: Some(-5.0),
        ..DirectSnowHourlyForcing::zero()
    };

    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &candidate,
        SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
    )
    .expect("snowfall advected-heat shadow");
    let production_hour = &stage3_diagnostics(&result.authoritative).hourly_surface_energy[0];
    let hour = &evaluation_diagnostics(&result).hourly[0];
    let snow_specific_heat_j_kg_k =
        4.186_798_188 * (0.024_928 + 0.001_76 * (-5.0 + 273.16)) / 0.001;
    let expected_flux_w_m2 = snow_specific_heat_j_kg_k
        * (0.01 * 0.1 * 1_000.0 / 3_600.0)
        * (-5.0 - production_hour.surface_temperature_c);
    assert!((hour.advected_flux_w_m2 - expected_flux_w_m2).abs() <= 1.0e-12);
}

#[test]
fn sequential_shadow_gates_melt_on_cold_content_and_records_terminal_energy() {
    let mut cold = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    set_single_layer_mass(&mut cold, 0.50, -50.0);
    cold.tmax_c = -50.0;
    cold.tmin_c = -50.0;
    cold.dewpoint_c = -60.0;
    cold.surface_energy_options.daily_solar_radiation_mj_m2 = 0.1;
    cold.hourly = [DirectSnowHourlyForcing {
        radiation_mj_m2: 0.0,
        air_temperature_c: -50.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    let cold_result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &cold,
        SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
    )
    .expect("cold-content shadow");
    let cold_diagnostics = evaluation_diagnostics(&cold_result);
    assert!(
        cold_diagnostics.complete_arm_melt_kg_m2 <= 1.0e-9,
        "cold-pack numerical melt was {} kg m^-2",
        cold_diagnostics.complete_arm_melt_kg_m2
    );
    assert!(cold_diagnostics.complete_arm_cold_energy_change_j_m2.abs() > f64::EPSILON);

    let mut terminal = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    set_single_layer_mass(&mut terminal, 0.001_1, 0.0);
    terminal.surface_energy_options.daily_solar_radiation_mj_m2 = 48.0;
    terminal.hourly = [DirectSnowHourlyForcing {
        radiation_mj_m2: 1_000.0,
        air_temperature_c: 0.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    let terminal_baseline = terminal.clone();
    let baseline_result =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&terminal_baseline)
            .expect("terminal-energy authoritative baseline");
    let terminal_result =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
            &terminal,
            SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
        )
        .expect("terminal-energy shadow");
    let terminal_diagnostics = evaluation_diagnostics(&terminal_result);
    assert!(terminal_diagnostics.complete_arm_melt_kg_m2 > 0.0);
    assert!(terminal_diagnostics.complete_arm_terminal_unallocated_j_m2 > 0.0);
    assert!(terminal_diagnostics.complete_arm_maximum_thermodynamic_residual_j_m2 <= 1.0e-6);
    assert_eq!(
        terminal_result.authoritative.runtime_swe_after_m.to_bits(),
        baseline_result.runtime_swe_after_m.to_bits(),
        "shadow melt must not mutate authoritative SWE"
    );
    let evaluation = terminal_diagnostics;
    assert!(evaluation.evaluated_seconds < evaluation.requested_seconds);
    assert_eq!(
        evaluation
            .hourly
            .iter()
            .map(|hour| hour.requested_seconds)
            .sum::<f64>()
            .to_bits(),
        evaluation.requested_seconds.to_bits()
    );
    assert_eq!(
        evaluation
            .hourly
            .iter()
            .map(|hour| hour.evaluated_seconds)
            .sum::<f64>()
            .to_bits(),
        evaluation.evaluated_seconds.to_bits()
    );
    assert!(evaluation.hourly.iter().any(|hour| {
        hour.requested_seconds.to_bits() == 3_600.0_f64.to_bits()
            && hour.evaluated_seconds.to_bits() == 0.0_f64.to_bits()
    }));
}

#[test]
fn filtered_capture_skips_evaluation_primitives_and_preserves_authoritative_result() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate
        .surface_energy_options
        .turbulent_geometry
        .wind_speed_height_m = 0.001;
    candidate
        .surface_energy_options
        .turbulent_geometry
        .aerodynamic_roughness_length_m = 0.005;
    let baseline = candidate.clone();

    let expected = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_capture(
        &baseline,
        DirectSnowDiagnosticCapture::Disabled,
    )
    .expect("filtered authoritative baseline");
    let observed =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_capture_and_evaluation(
            &candidate,
            DirectSnowDiagnosticCapture::Disabled,
            Some(SnowStage3EvaluationOperator::SameStatePairedCarrierV1),
        )
        .expect("filtered request must not execute invalid evaluation geometry");
    assert_eq!(observed.authoritative, expected);
    assert!(observed.authoritative.verbose_diagnostics.is_none());
    assert!(observed.evaluation.is_none());
}

#[test]
fn selected_evaluator_on_empty_pack_emits_tagged_zero_coverage() {
    let mut candidate = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    candidate.runtime_swe_m = 0.0;
    candidate.runtime_depth_m = 0.0;
    candidate.runtime_density_kg_m3 = 0.0;
    candidate.coe_boundary_depth_m = 0.0;
    candidate.coe_boundary_density_kg_m3 = 0.0;
    candidate.snow_layers.clear();
    let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &candidate,
        SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
    )
    .expect("empty-pack evaluation row");
    let evaluation = evaluation_diagnostics(&result);
    assert_eq!(evaluation.evaluated_seconds.to_bits(), 0.0_f64.to_bits());
    assert_eq!(evaluation.coverage_fraction.to_bits(), 0.0_f64.to_bits());
    assert_eq!(
        evaluation
            .hourly
            .iter()
            .map(|hour| hour.requested_seconds)
            .sum::<f64>()
            .to_bits(),
        evaluation.requested_seconds.to_bits()
    );
}

#[test]
fn legacy_complete_carrier_field_remains_source_and_behavior_compatible() {
    let mut legacy = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    legacy.surface_energy_options.complete_carrier_shadow = true;
    let typed = legacy.clone();
    let mut typed = typed;
    typed.surface_energy_options.complete_carrier_shadow = false;
    let legacy_result =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&legacy)
            .expect("legacy complete-carrier request");
    let typed_result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &typed,
        SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
    )
    .expect("typed sequential request");
    let typed_baseline =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&typed)
            .expect("typed authoritative baseline");
    assert_eq!(typed_result.authoritative, typed_baseline);
    assert_eq!(
        stage3_diagnostics(&legacy_result)
            .shadow_complete_energy_j_m2
            .to_bits(),
        evaluation_diagnostics(&typed_result)
            .complete_arm_total_j_m2
            .to_bits()
    );
    let conflict = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &legacy,
        SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
    )
    .expect_err("legacy sequential plus explicit paired request must conflict");
    assert!(conflict.to_string().contains("evaluation_request_conflict"));
}

#[test]
fn paired_non_formulation_fingerprint_covers_material_shared_inputs() {
    fn fingerprint(candidate: &DirectActiveSnowPartitionInputs, label: &str) -> u64 {
        let result = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
            candidate,
            SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
        )
        .unwrap_or_else(|error| panic!("paired fingerprint candidate {label}: {error}"));
        let evaluation = evaluation_diagnostics(&result);
        assert_eq!(
            evaluation.surface_arm_non_formulation_fingerprint,
            evaluation.complete_arm_non_formulation_fingerprint
        );
        evaluation.non_formulation_fingerprint
    }

    let baseline_inputs = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    let baseline = fingerprint(&baseline_inputs, "baseline");
    let mut variants = Vec::new();
    let mut pressure = baseline_inputs.clone();
    pressure.surface_energy_options.atmospheric_pressure_pa += 10.0;
    variants.push(("pressure", pressure));
    let mut daily_solar = baseline_inputs.clone();
    daily_solar
        .surface_energy_options
        .daily_solar_radiation_mj_m2 += 0.1;
    variants.push(("daily_solar", daily_solar));
    let mut extraterrestrial = baseline_inputs.clone();
    extraterrestrial
        .surface_energy_options
        .daily_extraterrestrial_radiation_mj_m2 += 0.1;
    variants.push(("extraterrestrial", extraterrestrial));
    let mut forcing = baseline_inputs.clone();
    forcing.hyetograph_rainfall_m = 1.0e-6;
    forcing.hourly[0].active_precipitation_m = 1.0e-6;
    forcing.hourly[0].rain_m = 1.0e-6;
    forcing.hourly[0].rain_fraction = 1.0;
    forcing.hourly[0].hydrometeor_temperature_c = Some(-5.0);
    variants.push(("active_precipitation", forcing));
    let mut phase = baseline_inputs.clone();
    phase.hourly[0].phase_model = SnowPhasePartitionModel::HarderPomeroyHourly;
    variants.push(("phase_model", phase));
    let mut layer = baseline_inputs.clone();
    layer.snow_layers[0].settle_day_count += 1.0;
    variants.push(("settle_count", layer));
    let mut refrozen = baseline_inputs.clone();
    refrozen.snow_layers[0].refrozen_liquid_m = 1.0e-8;
    variants.push(("refrozen_liquid", refrozen));

    for (name, variant) in variants {
        assert_ne!(
            fingerprint(&variant, name),
            baseline,
            "fingerprint omitted {name}"
        );
    }
}

#[test]
fn complete_carrier_shadow_fails_closed_on_incomplete_or_invalid_geometry() {
    let missing_longwave = inputs(
        SnowSurfaceLongwaveModel::Disabled,
        SnowSurfaceSublimationModel::Disabled,
    );
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &missing_longwave,
        SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
    )
    .expect_err("complete shadow requires net longwave");
    assert!(
        error
            .to_string()
            .contains("shadow_requires_complete_longwave")
    );

    let mut invalid_geometry = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    invalid_geometry
        .surface_energy_options
        .turbulent_geometry
        .wind_speed_height_m = 0.0;
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &invalid_geometry,
        SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
    )
    .expect_err("invalid virtual instrument geometry must fail closed");
    assert!(error.to_string().contains("wind_speed_height_m"));

    let mut primitive_failure = inputs(
        SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
        SnowSurfaceSublimationModel::Disabled,
    );
    primitive_failure
        .surface_energy_options
        .turbulent_geometry
        .wind_speed_height_m = 0.001;
    primitive_failure
        .surface_energy_options
        .turbulent_geometry
        .aerodynamic_roughness_length_m = 0.005;
    let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &primitive_failure,
        SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
    )
    .expect_err("primitive geometry violation must retain its meteorology source");
    match error {
        openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationError::TurbulentTransfer(
            snapshot,
        ) => {
            assert_eq!(
                snapshot.operator,
                SnowStage3EvaluationOperator::SameStatePairedCarrierV1
            );
            assert!(!snapshot.source.to_string().is_empty());
        }
        openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationError::Kernel(error) => {
            panic!("expected typed turbulent transfer error, observed {error}")
        }
        openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationError::TerminalNumerics(
            error,
        ) => panic!("expected typed turbulent transfer error, observed {error}"),
        openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationError::TerminalCustody(
            error,
        ) => panic!("expected typed turbulent transfer error, observed {error}"),
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
        stage3_diagnostics(&result).hourly_surface_energy[0].applied_surface_energy_j_m2;

    assert!(applied_first_hour > event_layer_cold_content_j_m2 * 100.0);
    assert!(
        stage3_diagnostics(&result).hourly_surface_energy[0].unused_positive_energy_j_m2 <= 1.0e-9
    );
    assert!(stage3_diagnostics(&result).cold_content_after_j_m2 > 0.0);
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
    let first_hour = stage3_diagnostics(&result).hourly_surface_energy[0];

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
