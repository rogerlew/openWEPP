use super::*;

fn reconciliation_inputs() -> DirectActiveSnowPartitionInputs {
    let mut layer = DirectSnowLayerState::new(0.18, 0.40, 450.0, 12.0);
    layer.temperature_c = -8.0;
    layer.cold_content_j_m2 = 0.18 * 1_000.0 * 2_100.0 * 8.0;
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
        surface_energy_options: DirectSnowSurfaceEnergyOptions {
            longwave_model: SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
            sublimation_model: SnowSurfaceSublimationModel::NeutralBulkStage3V1,
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
        snow_layers: vec![layer],
        underlying_surface_albedo: 0.2,
        hourly: [DirectSnowHourlyForcing {
            radiation_mj_m2: 0.0,
            air_temperature_c: -5.0,
            ..DirectSnowHourlyForcing::zero()
        }; 24],
    }
}

#[test]
fn paired_arm_fingerprint_mismatch_fails_closed() {
    let tag = Stage3EvaluationTag::new(SnowStage3EvaluationOperator::SameStatePairedCarrierV1);
    let mut summary = Stage3ShadowSummary::new(tag);
    summary.source_fingerprint = 1;
    summary.forcing_fingerprint = 2;
    summary.geometry_fingerprint = 3;
    summary.non_formulation_fingerprint = 4;
    summary.surface_arm_non_formulation_fingerprint = 5;
    summary.complete_arm_non_formulation_fingerprint = 6;
    summary.evaluated_seconds = summary.requested_seconds;
    for hour in &mut summary.hourly {
        hour.requested_seconds = STAGE3_SECONDS_PER_HOUR;
        hour.evaluated_seconds = STAGE3_SECONDS_PER_HOUR;
    }

    let error = Wb11HydrologyKernel::validate_stage3_shadow_summary(
        HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
        &summary,
    )
    .expect_err("paired fingerprint mismatch must fail");
    assert!(
        error
            .to_string()
            .contains("stage3_evaluation_paired_fingerprint_equality")
    );
}

#[test]
fn operator_tags_have_exact_distinct_cadence() {
    let paired = Stage3EvaluationTag::new(SnowStage3EvaluationOperator::SameStatePairedCarrierV1);
    let sequential =
        Stage3EvaluationTag::new(SnowStage3EvaluationOperator::SequentialResolvedShadowV1);
    assert_eq!(
        paired.cadence_id,
        "stage3_fixed_hourly_immutable_snapshot_v1"
    );
    assert_eq!(
        sequential.cadence_id,
        "stage3_dynamic_substep_with_hourly_forcing_v1"
    );
}

#[test]
fn reconciliation_validator_rejects_global_order_reason_and_projection_mutations() {
    let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
    let inputs = reconciliation_inputs();
    let cold = vec![inputs.snow_layers[0].cold_content_j_m2];
    let tag = Stage3EvaluationTag::new(SnowStage3EvaluationOperator::SameStatePairedCarrierV1);
    let valid = Wb11HydrologyKernel::evaluate_stage3_same_state_paired_carrier(
        phase,
        tag,
        &inputs,
        &inputs.snow_layers,
        &cold,
    )
    .expect("valid same-state reconciliation");
    Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &valid)
        .expect("unmodified reconciliation must pass");

    let mut reordered = valid.clone();
    reordered.reconciliation.tuples.swap(0, 1);
    assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &reordered).is_err());

    let mut bad_reason = valid.clone();
    bad_reason.reconciliation.hourly_status[0].reason = "invented";
    assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &bad_reason).is_err());

    let mut bad_projection = valid;
    bad_projection.reconciliation.tuples[0].projection_id = "aligned_active_dynamic";
    assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &bad_projection).is_err());
}

#[test]
fn sequential_reconciliation_mass_guard_uses_mass_scale_tolerance() {
    let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
    let inputs = reconciliation_inputs();
    let cold = vec![inputs.snow_layers[0].cold_content_j_m2];
    let tag = Stage3EvaluationTag::new(SnowStage3EvaluationOperator::SequentialResolvedShadowV1);
    let supports = inputs
        .hourly
        .iter()
        .copied()
        .map(|forcing| DirectSnowStage3SupportInput {
            forcing,
            duration_seconds: STAGE3_SECONDS_PER_HOUR,
        })
        .collect::<Vec<_>>();
    let mut summary = Wb11HydrologyKernel::evaluate_stage3_sequential_melt_shadow(
        phase,
        tag,
        &inputs,
        &supports,
        inputs.snow_layers.clone(),
        cold,
        None,
        0.0,
        None,
        None,
    )
    .expect("valid sequential reconciliation");
    Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &summary)
        .expect("unmodified sequential reconciliation must pass");
    summary.reconciliation.tuples[0].total_ice_mass_after_kg_m2 += 1.0e-8;
    assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &summary).is_err());
}

#[test]
fn sequential_reconciliation_serializes_exact_transition_continuity() {
    let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
    let mut inputs = reconciliation_inputs();
    inputs.runtime_swe_m = 0.02;
    inputs.runtime_depth_m = 0.04;
    inputs.runtime_density_kg_m3 = 500.0;
    inputs.coe_boundary_depth_m = 0.04;
    inputs.coe_boundary_density_kg_m3 = 500.0;
    inputs.snow_layers[0].mass_swe_m = 0.02;
    inputs.snow_layers[0].thickness_m = 0.04;
    inputs.snow_layers[0].density_kg_m3 = 500.0;
    inputs.snow_layers[0].cold_content_j_m2 = 0.02 * 1_000.0 * 2_100.0 * 8.0;
    let cold = vec![inputs.snow_layers[0].cold_content_j_m2];
    let tag = Stage3EvaluationTag::new(SnowStage3EvaluationOperator::SequentialResolvedShadowV1);
    let supports = inputs
        .hourly
        .iter()
        .copied()
        .map(|forcing| DirectSnowStage3SupportInput {
            forcing,
            duration_seconds: STAGE3_SECONDS_PER_HOUR,
        })
        .collect::<Vec<_>>();
    let mut summary = Wb11HydrologyKernel::evaluate_stage3_sequential_melt_shadow(
        phase,
        tag,
        &inputs,
        &supports,
        inputs.snow_layers.clone(),
        cold,
        None,
        0.0,
        None,
        None,
    )
    .expect("valid sequential reconciliation");
    assert!(summary.reconciliation.tuples.len() > 1);
    assert_eq!(summary.reconciliation.tuples[0].hour_index, 0);
    assert_eq!(summary.reconciliation.tuples[0].substep_index, 0);
    assert_eq!(summary.reconciliation.tuples[1].hour_index, 0);
    assert_eq!(summary.reconciliation.tuples[1].substep_index, 1);
    for pair in summary.reconciliation.tuples.windows(2) {
        let (previous, next) = (&pair[0], &pair[1]);
        assert_eq!(
            previous.total_layer_state_fingerprint_after_fnv1a64,
            next.total_layer_state_fingerprint_before_fnv1a64
        );
        assert_eq!(
            previous.total_cold_after_j_m2.to_bits(),
            next.total_cold_before_j_m2.to_bits()
        );
        assert_eq!(
            previous.total_ice_mass_after_kg_m2.to_bits(),
            next.total_ice_mass_before_kg_m2.to_bits()
        );
    }
    Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &summary)
        .expect("exactly continuous reconciliation must pass");

    summary.reconciliation.tuples[1].total_layer_state_fingerprint_before_fnv1a64 ^= 1;
    assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &summary).is_err());
}

#[test]
fn same_state_suppresses_day_start_pack_at_resolved_mass_boundary() {
    let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
    let mut inputs = reconciliation_inputs();
    inputs.runtime_swe_m = STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
    inputs.runtime_depth_m = 0.002;
    inputs.runtime_density_kg_m3 = 500.0;
    inputs.snow_layers[0].mass_swe_m = STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
    inputs.snow_layers[0].thickness_m = 0.002;
    inputs.snow_layers[0].density_kg_m3 = 500.0;
    inputs.snow_layers[0].cold_content_j_m2 = 0.0;
    let tag = Stage3EvaluationTag::new(SnowStage3EvaluationOperator::SameStatePairedCarrierV1);
    let summary = Wb11HydrologyKernel::evaluate_stage3_same_state_paired_carrier(
        phase,
        tag,
        &inputs,
        &inputs.snow_layers,
        &[0.0],
    )
    .expect("boundary pack is a valid non-evaluated state");
    assert!(summary.reconciliation.tuples.is_empty());
    assert!(
        summary.reconciliation.hourly_status.iter().all(|status| {
            !status.evaluated && status.reason == "no_resolved_snow_at_day_start"
        })
    );
}

#[test]
fn surface_projection_crosses_density_boundary_and_uses_active_cold_content() {
    let layers = vec![
        DirectSnowLayerState {
            mass_swe_m: 0.02,
            thickness_m: 0.10,
            density_kg_m3: 200.0,
            settle_day_count: 1.0,
            temperature_c: -10.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 420_000.0,
            refrozen_liquid_m: 0.0,
        },
        DirectSnowLayerState {
            mass_swe_m: 0.14,
            thickness_m: 0.30,
            density_kg_m3: 466.666_666_666_666_7,
            settle_day_count: 2.0,
            temperature_c: -2.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 588_000.0,
            refrozen_liquid_m: 0.0,
        },
    ];
    let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(17, layers)
        .expect("multilayer persistent state");
    let surface = Wb11HydrologyKernel::project_stage3_surface_state_v1(&state)
        .expect("canonical active-volume surface");
    assert_eq!(surface.active_depth_m.to_bits(), 0.25_f64.to_bits());
    assert!((surface.active_mass_kg_m2 - 90.0).abs() <= 1.0e-12);
    assert!((surface.surface_temperature_k - 269.372_222_222_222_2).abs() <= 1.0e-12);
    assert_ne!(
        surface.surface_temperature_k.to_bits(),
        (state.layers[0].temperature_c + 273.15).to_bits()
    );
    assert!(matches!(
        surface.selected_substep_seconds,
        1_800.0 | 900.0 | 60.0
    ));
    assert_ne!(
        surface.active_lower_partition_sha256,
        openwepp_coupled_time::Digest32::zero()
    );
    assert_ne!(
        surface.beginning_stage3_state_sha256,
        openwepp_coupled_time::Digest32::zero()
    );
}

#[test]
fn surface_projection_selects_parent_medium_and_small_cadence() {
    for (lane_id, mass_swe_m, expected_seconds) in [
        (1, 0.08, 1_800.0_f64),
        (2, 0.02, 900.0_f64),
        (3, 0.005, 60.0_f64),
    ] {
        let temperature_c = -3.0;
        let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            lane_id,
            vec![DirectSnowLayerState {
                mass_swe_m,
                thickness_m: mass_swe_m * 2.0,
                density_kg_m3: 500.0,
                settle_day_count: 1.0,
                temperature_c,
                liquid_water_m: 0.0,
                cold_content_j_m2: mass_swe_m
                    * STAGE3_RHO_WATER_KG_M3
                    * STAGE3_SPECIFIC_HEAT_ICE_J_KG_K
                    * -temperature_c,
                refrozen_liquid_m: 0.0,
            }],
        )
        .expect("cadence persistent state");
        let surface = Wb11HydrologyKernel::project_stage3_surface_state_v1(&state)
            .expect("cadence surface projection");
        assert_eq!(
            surface.selected_substep_seconds.to_bits(),
            expected_seconds.to_bits()
        );
    }
}

#[test]
fn bottom_volume_projection_uses_whole_pack_for_one_layer() {
    let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(
        21,
        vec![DirectSnowLayerState {
            mass_swe_m: 0.08,
            thickness_m: 0.20,
            density_kg_m3: 400.0,
            settle_day_count: 1.0,
            temperature_c: -4.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 672_000.0,
            refrozen_liquid_m: 0.0,
        }],
    )
    .expect("one-layer persistent state");
    let projection = Wb11HydrologyKernel::project_stage3_bottom_volume_v1(&state, 101_324.6)
        .expect("one-layer bottom volume");

    assert_eq!(projection.thickness_m.to_bits(), 0.20_f64.to_bits());
    assert_eq!(projection.temperature_k.to_bits(), 269.15_f64.to_bits());
    assert!(projection.thermal_conductivity_w_m_k.is_finite());
    assert!(projection.thermal_conductivity_w_m_k > 0.0);
    assert_eq!(
        projection.beginning_stage3_state_sha256,
        openwepp_coupled_time::digest_bytes(
            &Wb11HydrologyKernel::serialize_stage3_persistent_state(&state)
                .expect("serialize beginning state"),
        )
    );
}

#[test]
fn bottom_volume_projection_selects_lower_partition_for_multiple_layers() {
    let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(
        22,
        vec![
            DirectSnowLayerState {
                mass_swe_m: 0.02,
                thickness_m: 0.10,
                density_kg_m3: 200.0,
                settle_day_count: 1.0,
                temperature_c: -10.0,
                liquid_water_m: 0.0,
                cold_content_j_m2: 420_000.0,
                refrozen_liquid_m: 0.0,
            },
            DirectSnowLayerState {
                mass_swe_m: 0.14,
                thickness_m: 0.30,
                density_kg_m3: 466.666_666_666_666_7,
                settle_day_count: 2.0,
                temperature_c: -2.0,
                liquid_water_m: 0.0,
                cold_content_j_m2: 588_000.0,
                refrozen_liquid_m: 0.0,
            },
        ],
    )
    .expect("multilayer persistent state");
    let projection = Wb11HydrologyKernel::project_stage3_bottom_volume_v1(&state, 101_324.6)
        .expect("lower bottom volume");

    assert!((projection.thickness_m - 0.15).abs() <= 1.0e-12);
    assert!((projection.temperature_k - 271.15).abs() <= 1.0e-12);
    assert!(projection.thermal_conductivity_w_m_k.is_finite());
    assert!(projection.thermal_conductivity_w_m_k > 0.0);
}

#[test]
fn bottom_volume_projection_rejects_invalid_state_and_pressure() {
    let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(
        23,
        vec![DirectSnowLayerState {
            mass_swe_m: 0.08,
            thickness_m: 0.20,
            density_kg_m3: 400.0,
            settle_day_count: 1.0,
            temperature_c: -4.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 672_000.0,
            refrozen_liquid_m: 0.0,
        }],
    )
    .expect("valid persistent state");
    let mut corrupt = state.clone();
    corrupt.layers[0].thickness_m = f64::NAN;

    assert!(Wb11HydrologyKernel::project_stage3_bottom_volume_v1(&corrupt, 101_324.6,).is_err());
    assert!(Wb11HydrologyKernel::project_stage3_bottom_volume_v1(&state, f64::NAN).is_err());
    assert!(Wb11HydrologyKernel::project_stage3_bottom_volume_v1(&state, 0.0).is_err());
}

#[test]
fn stage3_intake_rejects_obsolete_physics_selectors() {
    let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
    let tag = Stage3EvaluationTag::new(SnowStage3EvaluationOperator::SameStatePairedCarrierV1);
    let mut inputs = reconciliation_inputs();
    inputs.snow_melt_model = SnowMeltModel::CoeLiquidHoldingCapacityV1;
    let error = Wb11HydrologyKernel::evaluate_stage3_same_state_paired_carrier(
        phase,
        tag,
        &inputs,
        &inputs.snow_layers,
        &[inputs.snow_layers[0].cold_content_j_m2],
    )
    .err()
    .expect("legacy CoE selector must not enter Stage 3");
    assert!(
        error
            .to_string()
            .contains("snow.stage3_obsolete_snow_model_selector")
    );

    inputs.snow_melt_model = SnowMeltModel::AdaptiveCompositionalStage3V1;
    inputs.surface_energy_options.sublimation_model = SnowSurfaceSublimationModel::Disabled;
    let error = Wb11HydrologyKernel::evaluate_stage3_same_state_paired_carrier(
        phase,
        tag,
        &inputs,
        &inputs.snow_layers,
        &[inputs.snow_layers[0].cold_content_j_m2],
    )
    .err()
    .expect("disabled vapor transfer must not enter Stage 3");
    assert!(
        error
            .to_string()
            .contains("snow.stage3_obsolete_sublimation_selector")
    );
}

#[test]
fn stage3_neutral_bulk_vapor_transfer_preserves_bounded_mass_and_latent_custody() {
    let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
    let inputs = reconciliation_inputs();
    let hourly = inputs.hourly[0];
    let surface_temperature_c = -8.0;
    let snow_depth_m = inputs.runtime_depth_m;
    let snow_density_kg_m3 = inputs.runtime_density_kg_m3;
    let raw_sublimation_m = Wb11HydrologyKernel::stage3_neutral_bulk_vapor_transfer_hour_m(
        phase,
        inputs.canopy_cover_fraction,
        inputs.wind_m_s,
        hourly.air_temperature_c,
        inputs.dewpoint_c,
        snow_depth_m,
        surface_temperature_c,
    )
    .expect("contract neutral-bulk equation");
    assert!(raw_sublimation_m > 0.0);

    let carrier = Wb11HydrologyKernel::stage3_hourly_surface_energy(
        phase,
        &inputs,
        hourly,
        Stage3SurfaceInterval {
            surface_temperature_c,
            snow_depth_m,
            snow_density_kg_m3,
            duration_seconds: STAGE3_SECONDS_PER_HOUR,
            forcing_duration_seconds: STAGE3_SECONDS_PER_HOUR,
            boundary: None,
        },
        Some(SnowStage3EvaluationOperator::SameStatePairedCarrierV1),
        DirectSnowDiagnosticCapture::Verbose,
    )
    .expect("sole Stage3 neutral-bulk carrier");
    let available_swe_m = snow_depth_m * snow_density_kg_m3 / STAGE3_RHO_WATER_KG_M3;
    assert_eq!(
        carrier.sublimation_m.to_bits(),
        raw_sublimation_m.min(available_swe_m).to_bits()
    );
    let reconstructed_vapor_kg_m2 = -carrier.sublimation_m * STAGE3_RHO_WATER_KG_M3;
    assert!(
        (carrier.vapor_mass_exchange_kg_m2 - reconstructed_vapor_kg_m2).abs()
            <= MASS_ABSOLUTE_TOLERANCE_KG_M2
    );
    let diagnostics = carrier.diagnostics.expect("verbose vapor custody");
    let reconstructed_latent_j_m2 = reconstructed_vapor_kg_m2 * diagnostics.latent_heat_j_kg;
    assert!(
        (carrier.latent_j_m2 - reconstructed_latent_j_m2).abs() <= ENERGY_ABSOLUTE_TOLERANCE_J_M2
    );
    assert!(carrier.mass_latent_identity_residual_j_m2.abs() <= ENERGY_ABSOLUTE_TOLERANCE_J_M2);
    assert_eq!(
        carrier
            .reconciliation
            .expect("neutral-bulk receipt identity")
            .sublimation_model_id,
        STAGE3_NEUTRAL_BULK_VAPOR_TRANSFER_ID
    );
}

#[test]
fn nonexhausting_melt_has_exact_zero_terminal_energy() {
    let excess_energy_j_m2 = 8.423_781_023_441_192_e-14;
    let ice_available_kg_m2 = 1.0e-3;
    let (melt_kg_m2, unallocated_energy_j_m2) =
        super::evaluation::canonical_melt_and_unallocated_energy(
            excess_energy_j_m2,
            ice_available_kg_m2,
        );
    assert!(melt_kg_m2 > 0.0);
    assert!(melt_kg_m2 < ice_available_kg_m2);
    assert_eq!(unallocated_energy_j_m2.to_bits(), 0.0_f64.to_bits());

    let exhausting_energy_j_m2 = STAGE3_LATENT_HEAT_FUSION_J_KG * ice_available_kg_m2 + 2.0;
    let (exhausting_melt, exhausting_unallocated) =
        super::evaluation::canonical_melt_and_unallocated_energy(
            exhausting_energy_j_m2,
            ice_available_kg_m2,
        );
    assert_eq!(exhausting_melt.to_bits(), ice_available_kg_m2.to_bits());
    assert_eq!(exhausting_unallocated.to_bits(), 2.0_f64.to_bits());
}

#[test]
fn thermal_fragment_rejoin_preserves_exact_settling_chronology_bits() {
    let settle_day_count = 12.0_f64;
    let mut layers = vec![
        DirectSnowLayerState {
            mass_swe_m: 0.1125,
            thickness_m: 0.25,
            density_kg_m3: 450.0,
            settle_day_count,
            temperature_c: -4.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 945_000.0,
            refrozen_liquid_m: 0.0,
        },
        DirectSnowLayerState {
            mass_swe_m: 0.0675,
            thickness_m: 0.15,
            density_kg_m3: 450.0,
            settle_day_count,
            temperature_c: -4.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 567_000.0,
            refrozen_liquid_m: 0.0,
        },
    ];
    let mut cold_content_by_layer = layers
        .iter()
        .map(|layer| layer.cold_content_j_m2)
        .collect::<Vec<_>>();

    let active_layer_count = Wb11HydrologyKernel::coalesce_stage3_thermal_fragments_for_test(
        &mut layers,
        &mut cold_content_by_layer,
        2,
    );

    assert_eq!(active_layer_count, 1);
    assert_eq!(layers.len(), 1);
    assert_eq!(layers[0].settle_day_count.to_bits(), settle_day_count.to_bits());
}

#[test]
fn thermal_fragment_rejoin_rejects_adjacent_ulp_settling_chronology() {
    let settle_day_count = 12.0_f64;
    let adjacent_settle_day_count = f64::from_bits(settle_day_count.to_bits() + 1);
    let mut layers = vec![
        DirectSnowLayerState {
            mass_swe_m: 0.1125,
            thickness_m: 0.25,
            density_kg_m3: 450.0,
            settle_day_count,
            temperature_c: -4.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 945_000.0,
            refrozen_liquid_m: 0.0,
        },
        DirectSnowLayerState {
            mass_swe_m: 0.0675,
            thickness_m: 0.15,
            density_kg_m3: 450.0,
            settle_day_count: adjacent_settle_day_count,
            temperature_c: -4.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 567_000.0,
            refrozen_liquid_m: 0.0,
        },
    ];
    let mut cold_content_by_layer = layers
        .iter()
        .map(|layer| layer.cold_content_j_m2)
        .collect::<Vec<_>>();

    let active_layer_count = Wb11HydrologyKernel::coalesce_stage3_thermal_fragments_for_test(
        &mut layers,
        &mut cold_content_by_layer,
        2,
    );

    assert_eq!(active_layer_count, 2);
    assert_eq!(layers.len(), 2);
    assert_eq!(layers[0].settle_day_count.to_bits(), settle_day_count.to_bits());
    assert_eq!(
        layers[1].settle_day_count.to_bits(),
        adjacent_settle_day_count.to_bits()
    );
}

#[path = "stage3_solver/stage3_evaluation_validation_tests/persistent_tests.rs"]
mod persistent_tests;
