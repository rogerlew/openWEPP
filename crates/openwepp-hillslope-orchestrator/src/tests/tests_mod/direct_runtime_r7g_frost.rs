use super::direct_runtime_test_lock;
use crate::{
    DirectActiveFrostPartitionInputs, DirectDayFrame, DirectExecutorMode, DirectFrameExecutor,
    DirectFrostControlInputs, DirectFrostFineLayerCarry, DirectFrostFineLayerProjection,
    DirectFrostHourlyForcing, DirectFrostLaneState, DirectFrostLayerInput,
    DirectFrostLayerShadowCarry, DirectFrostPriorStateInput, DirectFrostRuntimeCarry,
    DirectFrostThermalInputs, DirectHydrologyProjectionInputs, DirectLaneConstructorInputs,
    DirectPercolationInputs, DirectPublicationCalendarDay, DirectPublicationDayInput,
    DirectPublicationRunMetadata, DirectRunConstructorInputs, DirectRunFrame, DirectRunIdentity,
    DirectSubsurfaceComputeInputs, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
    DirectWinterFrostComputeInputs, Wb11HydrologyKernel, reset_direct_runtime_audit_counters,
};

#[test]
fn r7g_constructor_prefers_winter_column_frost_over_legacy_carry() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(91, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let mut lane = DirectLaneConstructorInputs::from_topology(0, 1, 1)
        .expect("single OFE lane constructor input should build");
    let canonical_carry = sample_frost_runtime_carry(0.050, 0.005, 0.123);
    let canonical_frost = DirectFrostLaneState::from(canonical_carry.clone());
    lane.winter_column.frost = canonical_frost.clone();
    lane.frost_runtime_carry = Some(sample_frost_runtime_carry(0.250, 0.025, 0.456));

    let frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
        identity,
        vec![lane],
    ))
    .expect("typed direct frame should construct from winter frost state");

    assert_eq!(frame.lanes[0].winter_column.frost, canonical_frost);
    assert_eq!(
        frame.lanes[0].frost_runtime_carry,
        Some(DirectFrostRuntimeCarry::from(canonical_frost))
    );
}

#[test]
fn r7g_legacy_constructor_frost_carry_migrates_into_winter_column() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(92, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let mut lane = DirectLaneConstructorInputs::from_topology(0, 1, 1)
        .expect("single OFE lane constructor input should build");
    let legacy_carry = sample_frost_runtime_carry(0.03125, 0.003_125, 0.222);
    lane.frost_runtime_carry = Some(legacy_carry.clone());

    let frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
        identity,
        vec![lane],
    ))
    .expect("typed direct frame should construct from legacy frost carry");

    assert_eq!(
        frame.lanes[0].winter_column.frost,
        DirectFrostLaneState::from(legacy_carry.clone())
    );
    assert_eq!(frame.lanes[0].frost_runtime_carry, Some(legacy_carry));
}

#[test]
fn r7g_r4a_frost_partition_mutates_winter_column_frost_state() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(93, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.percolation.layer_state_after = vec![sample_layer(0.200)];
    let winter_frost_compute_inputs = sample_winter_frost_compute_inputs(true);
    day.run_r4i_liquid_input_span()
        .expect("zero liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("zero infiltration/depression upstream span should execute");
    day.run_r4l_saturation_addback_span()
        .expect("zero saturation addback upstream span should execute");

    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
        .expect("typed winter frost compute should mutate direct frost state");

    assert!(day.winter_column.frost.active_frost_coupling);
    assert_eq!(
        day.winter_column.frost.dfrost_m.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        day.winter_column.frost.ws_frz_m.to_bits(),
        0.0_f64.to_bits()
    );
    assert!(!day.winter_column.frost.fine_layers.is_empty());
    assert_eq!(
        day.frost_runtime_carry,
        Some(DirectFrostRuntimeCarry::from(
            day.winter_column.frost.clone()
        ))
    );
}

#[test]
fn r7g_executor_commits_r4a_winter_column_frost_state_to_lane() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(94, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct publication frame should construct");
    frame.lanes[0].area_m2 = 100.0;
    let mut day_input = sample_publication_day_input();
    day_input.winter_frost_compute_inputs = Some(sample_winter_frost_compute_inputs(true));
    let metadata = DirectPublicationRunMetadata {
        run_name: "r7g_frost_state_commit".to_string(),
        runtime_selection: "direct-publication-frame-cutover-candidate".to_string(),
        output_policy: "test".to_string(),
    };

    DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_day_inputs(&mut frame, metadata, &[day_input])
        .expect("publication capture should commit typed frost state");

    assert!(frame.lanes[0].winter_column.frost.active_frost_coupling);
    assert_eq!(
        frame.lanes[0].winter_column.frost.dfrost_m.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        frame.lanes[0].winter_column.frost.ws_frz_m.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        frame.lanes[0].frost_runtime_carry,
        Some(DirectFrostRuntimeCarry::from(
            frame.lanes[0].winter_column.frost.clone()
        ))
    );
}

#[test]
fn r7g_typed_active_no_freeze_winter_outcome_has_no_coarse_projection() {
    let typed_partition = Wb11HydrologyKernel::compute_direct_winter_frost_partition(
        &no_freeze_typed_frost_inputs(true),
    )
    .expect("active no-freeze typed winter outcome should compute");

    assert!(typed_partition.active_frost_coupling);
    assert_eq!(
        typed_partition.frost_depth_after_m.to_bits(),
        0.0_f64.to_bits()
    );
    assert!(typed_partition.layer_projection.is_empty());
    assert!(!typed_partition.layer_shadow_projection.is_empty());
    assert!(!typed_partition.fine_layer_projection.is_empty());
}

#[test]
fn r7g_typed_inactive_winter_outcome_is_inert_without_material() {
    let typed_partition = Wb11HydrologyKernel::compute_direct_winter_frost_partition(
        &no_freeze_typed_frost_inputs(false),
    )
    .expect("inactive typed winter outcome should compute");

    assert!(!typed_partition.active_frost_coupling);
    assert!(typed_partition.layer_projection.is_empty());
    assert!(typed_partition.layer_shadow_projection.is_empty());
    assert!(typed_partition.fine_layer_projection.is_empty());
}

fn no_freeze_typed_frost_inputs(wint_red_enabled: bool) -> DirectActiveFrostPartitionInputs {
    let hourly = [DirectFrostHourlyForcing {
        radiation_mj_m2: 0.0,
        air_temperature_c: 5.0,
        cloud_fraction: 1.0,
    }; 24];
    DirectActiveFrostPartitionInputs {
        controls: DirectFrostControlInputs {
            frost_file_present: false,
            wint_red_enabled,
            fine_top_count: 10,
            fine_bot_count: 10,
            ksnowf: 1.0,
            kresf: 1.0,
            ksoilf: 1.0,
            kfactor1: 0.5,
            kfactor2: 0.5,
            kfactor3: 0.5,
            landuse_class_proxy: None,
        },
        thermal: DirectFrostThermalInputs {
            snow_depth_m: 0.0,
            snow_density_kg_m3: 0.0,
            residue_depth_m: 0.0,
            wind_m_s: 1.0,
            albedo: 0.20,
            canopy_height_m: 0.0,
            random_roughness_m: 0.0,
            day_of_year: 5.0,
            monthly_max_c: [8.0; 12],
            monthly_min_c: [2.0; 12],
        },
        profile_depth_m: 0.400,
        soil_water_m: 0.220,
        theta_residual: 0.050,
        theta_field_capacity: 0.250,
        soil_conductivity_m_s: 1.0e-6,
        prior_state: DirectFrostPriorStateInput::zero(),
        layers: vec![DirectFrostLayerInput {
            layer_index: 1,
            theta_m: 0.200,
            upper_limit_m: 0.500,
            depth_m: 0.400,
            residual_theta: 0.050,
            bulk_density_kg_m3: 1_300.0,
            frozen_depth_m: 0.0,
            frozen_water_m: 0.0,
        }],
        hourly,
    }
}

fn sample_winter_frost_compute_inputs(wint_red_enabled: bool) -> DirectWinterFrostComputeInputs {
    let active_inputs = no_freeze_typed_frost_inputs(wint_red_enabled);
    DirectWinterFrostComputeInputs {
        controls: active_inputs.controls,
        thermal: active_inputs.thermal,
        theta_residual: active_inputs.theta_residual,
        theta_field_capacity: active_inputs.theta_field_capacity,
        soil_conductivity_m_s: Some(active_inputs.soil_conductivity_m_s),
        layer_bulk_density_kg_m3: active_inputs
            .layers
            .iter()
            .map(|layer| layer.bulk_density_kg_m3)
            .collect(),
        hourly: active_inputs.hourly,
    }
}

fn sample_frost_runtime_carry(
    frost_depth_m: f64,
    frozen_water_m: f64,
    fine_liquid_theta: f64,
) -> DirectFrostRuntimeCarry {
    DirectFrostRuntimeCarry {
        active_frost_coupling: true,
        dfrost_m: frost_depth_m,
        dthaw_m: 0.0,
        nft: 1.0,
        ws_frz_m: frozen_water_m,
        infcap_frz_m_s: 1.0e-6,
        frwatc_soil_water_before_m: 0.200,
        frwatc_soil_water_after_m: 0.195,
        frwatc_frozen_water_before_m: 0.0,
        frwatc_frozen_water_after_m: frozen_water_m,
        frwatc_freeze_debit_m: frozen_water_m,
        frwatc_thaw_credit_m: 0.0,
        frwatc_net_liquid_delta_m: -frozen_water_m,
        frdp_m: frost_depth_m,
        thdp_m: 0.0,
        tfrdp_m: frost_depth_m,
        tthawd_m: 0.0,
        fgthwd_flag: 0.0,
        total_fine_layer_count: 1.0,
        conductivity_tilled_w_m_k: 1.58,
        conductivity_untilled_w_m_k: 1.75,
        conductivity_residue_w_m_k: 0.05,
        shadow_total_water_before_m: 0.200,
        shadow_total_water_after_m: 0.200,
        shadow_wb_delta_m: 0.0,
        shadow_frwatc_residual_m: 0.0,
        watpdg_m: 0.0,
        watbtm_m: 0.0,
        layer_shadows: vec![DirectFrostLayerShadowCarry {
            layer_index: 1,
            st_m: 0.200,
            soil_water_m: 0.195,
            frozen_depth_m: frost_depth_m,
            frozen_water_m,
            soilf_m: 0.0,
            yst_m: 0.200,
            nwfrzz_m: frozen_water_m,
        }],
        fine_layers: vec![DirectFrostFineLayerCarry {
            layer_index: 1,
            fine_index: 1,
            fgfrst: 1.0,
            slfsd_m: frost_depth_m,
            slsic_m: frozen_water_m,
            slsw_theta: fine_liquid_theta,
            sltime_s: 3_600.0,
        }],
    }
}

fn sample_publication_day_input() -> DirectPublicationDayInput {
    let layer_inputs = sample_layer_inputs(0.200);
    let base_layer = DirectSubsurfaceLayerState::from(layer_inputs.clone());
    let mut day_input = DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
        year: 2026,
        julian_day: 1,
        month: 1,
        day_of_month: 1,
        water_year: 2026,
    });
    day_input.initial_soil_water_m = Some(0.200);
    day_input.percolation_inputs = Some(DirectPercolationInputs {
        soil_water_initial_m: 0.200,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps: 1,
        restrictive_layer_enabled: false,
        restrictive_layer_conductivity_m_s: 0.0,
        restrictive_layer_thickness_m: 0.0,
        layers: vec![base_layer],
    });
    day_input.subsurface_compute_inputs = Some(DirectSubsurfaceComputeInputs {
        avg_slope: 0.0,
        slope_length_m: 1.0,
        lateral_anisotropy_ratio: 1.0,
        soil_depth_m: 0.400,
        solwpv_mode: 2006,
        mofe_hourly_carry_arrays_enabled: false,
        lane_substeps: 1,
        drainage_capacity_m: 0.0,
        drain_enabled: false,
        drain_depth_m: 0.5,
        drain_spacing_m: 1.0,
        drain_diameter_m: 0.1,
        layers: vec![layer_inputs],
    });
    day_input.hydrology_projection_inputs = Some(DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 1.0e-12,
        snow_water_m: 0.0,
        frozen_soil_water_m: 0.0,
        frost_depth_m: 0.0,
        profile_depth_m: Some(0.400),
        profile_porosity_cap_m: Some(0.200),
        profile_field_capacity_m: Some(0.100),
        profile_wilting_point_m: Some(0.050),
    });
    day_input
}

fn sample_layer(theta_m: f64) -> DirectSubsurfaceLayerState {
    DirectSubsurfaceLayerState::from(sample_layer_inputs(theta_m))
}

fn sample_layer_inputs(theta_m: f64) -> DirectSubsurfaceLayerInputs {
    DirectSubsurfaceLayerInputs {
        theta_m,
        field_capacity_m: 0.200,
        upper_limit_m: 0.500,
        conductivity_m_s: 1.0e-6,
        depth_m: 0.400,
        residual_theta: 0.0,
        frozen_depth_m: 0.0,
        frozen_water_m: 0.0,
        porosity: 0.5,
        field_capacity_theta: 0.25,
        coca: 1.0,
        lateral_conductivity_m_s: 1.0e-6,
    }
}
