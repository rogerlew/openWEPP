use super::direct_runtime_test_lock;
use crate::{
    DirectActiveFrostPartitionInputs, DirectDayFrame,
    DirectEvapotranspirationSurfaceDownstreamOperands,
    DirectEvapotranspirationSurfaceShadowProjection, DirectEvapotranspirationSurfaceState,
    DirectExecutorMode, DirectFrameExecutor, DirectFrostControlInputs, DirectFrostFineLayerCarry,
    DirectFrostHourlyForcing, DirectFrostLaneState, DirectFrostLayerInput,
    DirectFrostLayerShadowCarry, DirectFrostPriorStateInput, DirectFrostRuntimeCarry,
    DirectFrostThermalInputs, DirectHydrologyProjectionInputs, DirectLaneConstructorInputs,
    DirectPercolationInputs, DirectPublicationCalendarDay, DirectPublicationDayInput,
    DirectPublicationRunMetadata, DirectRunConstructorInputs, DirectRunFrame, DirectRunIdentity,
    DirectSnowCouplingInputs, DirectSubsurfaceComputeInputs, DirectSubsurfaceLayerInputs,
    DirectSubsurfaceLayerState, DirectWb14HyetographInterval, DirectWb14InfiltrationProducerInputs,
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
    assert_eq!(
        day.storage_reconciliation_inputs
            .frost_liquid_delta_m
            .to_bits(),
        0.0_f64.to_bits(),
        "active no-material frost carry must not mutate coarse storage"
    );
    assert_eq!(
        day.hydrology_projection_inputs
            .frozen_soil_water_m
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        day.hydrology_projection_inputs.frost_depth_m.to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn r7g_inactive_no_material_frost_clears_stale_coarse_projection_without_storage_delta() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(95, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.percolation.layer_state_after = vec![sample_layer_with_frost(0.200, 0.100, 0.005)];
    let storage_before_no_material_frost =
        aggregate_test_layer_storage(&day.percolation.layer_state_after[0]);
    let winter_frost_compute_inputs = sample_winter_frost_compute_inputs(false);
    day.run_r4i_liquid_input_span()
        .expect("zero liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("zero infiltration/depression upstream span should execute");
    day.run_r4l_saturation_addback_span()
        .expect("zero saturation addback upstream span should execute");

    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
        .expect("inactive no-material frost should clear stale coarse projection");

    assert!(!day.winter_column.frost.active_frost_coupling);
    assert_eq!(
        day.storage_reconciliation_inputs
            .frost_liquid_delta_m
            .to_bits(),
        0.0_f64.to_bits(),
        "inactive no-material frost must not mutate coarse storage"
    );
    assert_eq!(
        day.hydrology_projection_inputs
            .frozen_soil_water_m
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        day.hydrology_projection_inputs.frost_depth_m.to_bits(),
        0.0_f64.to_bits()
    );
    let cleared_layer = &day.percolation.layer_state_after[0];
    assert_eq!(cleared_layer.frozen_depth_m.to_bits(), 0.0_f64.to_bits());
    assert_eq!(cleared_layer.frozen_water_m.to_bits(), 0.0_f64.to_bits());
    assert!(
        (aggregate_test_layer_storage(cleared_layer) - storage_before_no_material_frost).abs()
            <= 1.0e-12,
        "inactive no-material frost clear must preserve aggregate layer storage"
    );
}

#[test]
fn r7g_r4a_prior_frozen_water_thaw_credits_liquid_storage() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(96, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let prior_frost_depth_m = 0.005;
    let prior_frozen_water_m = 0.001;
    day.percolation.layer_state_after = vec![sample_layer_with_frost(
        0.200,
        prior_frost_depth_m,
        prior_frozen_water_m,
    )];
    day.winter_column.frost = DirectFrostLaneState::from(sample_frost_runtime_carry(
        prior_frost_depth_m,
        prior_frozen_water_m,
        0.123,
    ));
    let liquid_storage_before_thaw =
        aggregate_test_layer_storage(&day.percolation.layer_state_after[0]);
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
        .expect("warm active frost step should thaw prior frozen water into liquid storage");

    assert_eq!(
        day.winter_column.frost.ws_frz_m.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        day.hydrology_projection_inputs
            .frozen_soil_water_m
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert!(
        (day.storage_reconciliation_inputs.frost_liquid_delta_m
            - day.winter_column.frost.frwatc_net_liquid_delta_m)
            .abs()
            <= 1.0e-12,
        "WB12 liquid storage delta must consume the frwatc net ledger: delta={} thaw={} net={} soil_after={} before={}",
        day.storage_reconciliation_inputs.frost_liquid_delta_m,
        day.winter_column.frost.frwatc_thaw_credit_m,
        day.winter_column.frost.frwatc_net_liquid_delta_m,
        day.water.soil_water_m,
        liquid_storage_before_thaw
    );
    assert!(
        (day.water.soil_water_m
            - liquid_storage_before_thaw
            - day.winter_column.frost.frwatc_net_liquid_delta_m)
            .abs()
            <= 1.0e-12,
        "liquid storage must include the frwatc net liquid delta after frost clears"
    );
    assert_eq!(
        day.winter_column.frost.frwatc_thaw_credit_m.to_bits(),
        prior_frozen_water_m.to_bits()
    );
    assert_eq!(
        day.winter_column.frost.frwatc_net_liquid_delta_m.to_bits(),
        day.storage_reconciliation_inputs
            .frost_liquid_delta_m
            .to_bits()
    );
    let thawed_layer = &day.percolation.layer_state_after[0];
    assert_eq!(thawed_layer.frozen_depth_m.to_bits(), 0.0_f64.to_bits());
    assert_eq!(thawed_layer.frozen_water_m.to_bits(), 0.0_f64.to_bits());
    assert!(
        (aggregate_test_layer_storage(thawed_layer) - day.water.soil_water_m).abs() <= 1.0e-12,
        "thawed/no-final-frost layer projection must clear before it can seed the next day"
    );
}

#[test]
fn r7h_explicit_frost_storage_source_does_not_rewrite_r4a_layer_projection() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(123, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let prior_frost_depth_m = 0.005;
    let prior_frozen_water_m = 0.001;
    day.percolation.layer_state_after = vec![sample_layer_with_frost(
        0.200,
        prior_frost_depth_m,
        prior_frozen_water_m,
    )];
    day.winter_column.frost = DirectFrostLaneState::from(sample_frost_runtime_carry(
        prior_frost_depth_m,
        prior_frozen_water_m,
        0.123,
    ));
    let explicit_storage_source_m = 0.004;
    day.frost_storage_liquid_delta_m = Some(explicit_storage_source_m);
    let liquid_storage_before_thaw =
        aggregate_test_layer_storage(&day.percolation.layer_state_after[0]);
    let winter_frost_compute_inputs = sample_winter_frost_compute_inputs(true);
    day.water.soil_water_m = liquid_storage_before_thaw;
    day.run_r4i_liquid_input_span()
        .expect("zero liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("zero infiltration/depression upstream span should execute");
    day.run_r4l_saturation_addback_span()
        .expect("zero saturation addback upstream span should execute");

    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
        .expect("typed frost partition should accept explicit storage source");

    let local_projection_delta_m = day.water.soil_water_m - liquid_storage_before_thaw;
    assert!(
        (local_projection_delta_m - day.winter_column.frost.frwatc_net_liquid_delta_m).abs()
            <= 1.0e-12,
        "layer/state projection must remain tied to the local frost partition"
    );
    assert!(
        (day.storage_reconciliation_inputs.frost_liquid_delta_m - local_projection_delta_m).abs()
            <= 1.0e-12,
        "R4A must keep the local frost partition delta for projection rebalance"
    );
    assert!(
        (explicit_storage_source_m - local_projection_delta_m).abs() > 1.0e-6,
        "test setup must distinguish storage-source authority from layer projection authority"
    );
    assert_eq!(
        day.frost_storage_liquid_delta_m,
        Some(explicit_storage_source_m),
        "R4A must preserve the explicit WB12 frost source for the later storage phase"
    );
}

#[test]
fn r7h_r4a_frost_uses_local_partition_excess_without_rewriting_wb14_capacity() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(97, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let base_soil_water_m = 0.200;
    let prior_frost_depth_m = 0.005;
    let prior_frozen_water_m = 0.001;
    let liquid_input_m = 0.010;
    day.percolation.layer_state_after = vec![sample_layer(base_soil_water_m)];
    day.winter_column.frost = DirectFrostLaneState::from(sample_frost_runtime_carry(
        prior_frost_depth_m,
        prior_frozen_water_m,
        0.123,
    ));
    day.liquid_input_inputs.liquid_input_handoff_m = liquid_input_m;
    day.infiltration_depression_inputs.producer_inputs =
        Some(DirectWb14InfiltrationProducerInputs {
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: liquid_input_m / 3_600.0,
            }],
            effective_conductivity_m_s: 1.0e-9,
            matric_potential_m: 0.100,
            storage_capacity_m: 0.020,
            depression_storage_capacity_m: 0.0,
        });
    let mut winter_frost_compute_inputs = sample_winter_frost_compute_inputs(true);
    winter_frost_compute_inputs.soil_conductivity_m_s = Some(1.0e-8);

    day.run_r4i_liquid_input_span()
        .expect("liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("infiltration/depression upstream span should execute");
    let cumulative_infiltration_m = day.runoff_partition_inputs.cumulative_infiltration_m;
    let wb14_effective_conductivity_m_s = day
        .infiltration_depression_inputs
        .producer_inputs
        .as_ref()
        .expect("producer inputs should remain present")
        .effective_conductivity_m_s;
    day.run_r4l_saturation_addback_span()
        .expect("zero saturation addback upstream span should execute");

    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
        .expect("runoff-stage frost should retain local liquid and partition runoff");

    assert_eq!(
        day.runoff_partition.cumulative_infiltration_m.to_bits(),
        cumulative_infiltration_m.to_bits(),
        "R4A must consume the already-computed WB14 infiltration operand"
    );
    assert!(
        (day.infiltration_depression_inputs
            .producer_inputs
            .as_ref()
            .expect("producer inputs should remain present")
            .effective_conductivity_m_s
            - wb14_effective_conductivity_m_s)
            .abs()
            <= 1.0e-12,
        "R4A must not rewrite WB14 conductivity after downstream spans have consumed it"
    );

    let local_partition_excess_m = liquid_input_m
        - cumulative_infiltration_m
        - day.runoff_partition.depression_storage_delta_m;
    assert!(
        local_partition_excess_m > 0.0,
        "test setup should retain local pre-partition liquid excess"
    );
    assert!(
        (day.winter_column.frost.frwatc_soil_water_before_m
            - base_soil_water_m
            - local_partition_excess_m)
            .abs()
            <= 1.0e-12,
        "runoff-stage frost must consume local pre-partition liquid before final Q"
    );
    assert!(
        (day.runoff_partition.q_runoff_m
            - (liquid_input_m
                - cumulative_infiltration_m
                - day.runoff_partition.depression_storage_delta_m
                - local_partition_excess_m))
            .abs()
            <= 1.0e-12,
        "final Q must exclude local liquid retained by runoff-stage frost"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn r7h_winter_local_liquid_projects_after_surface_et_before_saturation() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(99, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let base_soil_water_m = 0.200;
    let prior_frost_depth_m = 0.005;
    let prior_frozen_water_m = 0.001;
    let liquid_input_m = 0.010;
    day.percolation.layer_state_after = vec![sample_layer(base_soil_water_m)];
    day.winter_column.frost = DirectFrostLaneState::from(sample_frost_runtime_carry(
        prior_frost_depth_m,
        prior_frozen_water_m,
        0.123,
    ));
    day.liquid_input_inputs.liquid_input_handoff_m = liquid_input_m;
    day.infiltration_depression_inputs.producer_inputs =
        Some(DirectWb14InfiltrationProducerInputs {
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: liquid_input_m / 3_600.0,
            }],
            effective_conductivity_m_s: 1.0e-9,
            matric_potential_m: 0.100,
            storage_capacity_m: 0.020,
            depression_storage_capacity_m: 0.0,
        });

    day.run_r4i_liquid_input_span()
        .expect("liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("infiltration/depression upstream span should execute");
    let cumulative_infiltration_m = day.runoff_partition_inputs.cumulative_infiltration_m;
    let local_partition_excess_m = liquid_input_m
        - cumulative_infiltration_m
        - day.runoff_partition_inputs.depression_storage_delta_m;
    assert!(
        local_partition_excess_m > 0.0,
        "test setup should retain local pre-partition liquid excess"
    );

    let surface = DirectEvapotranspirationSurfaceState {
        soil_water_before_m: base_soil_water_m,
        soil_water_after_soil_evap_m: base_soil_water_m,
        evapotranspiration_seed_m: 0.0,
        transpiration_demand_m: 0.0,
        soil_evaporation_m: 0.0,
        residue_evaporation_m: 0.0,
        soil_evaporation_storage_return_m: 0.0,
        residue_interception_after_m: 0.0,
        stage_state_after: None,
        layer_soil_evaporation_withdrawal_m: vec![0.0],
        layer_state_after_soil_evap: vec![sample_layer(base_soil_water_m)],
    };
    day.evapotranspiration_surface_downstream_operands =
        DirectEvapotranspirationSurfaceDownstreamOperands::from(surface.clone());
    day.evapotranspiration_surface_shadow_projection =
        Some(DirectEvapotranspirationSurfaceShadowProjection {
            lane_index: 0,
            day_index: 0,
            soil_water_before_m: surface.soil_water_before_m,
            soil_water_after_soil_evap_m: surface.soil_water_after_soil_evap_m,
            evapotranspiration_seed_m: surface.evapotranspiration_seed_m,
            transpiration_demand_m: surface.transpiration_demand_m,
            soil_evaporation_m: surface.soil_evaporation_m,
            residue_evaporation_m: surface.residue_evaporation_m,
            soil_evaporation_storage_return_m: surface.soil_evaporation_storage_return_m,
            residue_interception_after_m: surface.residue_interception_after_m,
            layer_soil_evaporation_withdrawal_m: surface
                .layer_soil_evaporation_withdrawal_m
                .clone(),
            layer_state_after_soil_evap: surface.layer_state_after_soil_evap.clone(),
        });
    day.evapotranspiration_surface = surface;

    day.project_r4x_winter_local_liquid_before_saturation(Some(
        &sample_winter_frost_compute_inputs(true),
    ))
    .expect("winter-local liquid should project after surface ET");

    assert_eq!(
        day.percolation_inputs.same_pass_infiltration_m.to_bits(),
        cumulative_infiltration_m.to_bits(),
        "WB18 must consume the WB14 same-pass infiltration operand without frost-retained liquid"
    );
    assert_eq!(
        day.evapotranspiration_compute_inputs
            .same_pass_infiltration_m
            .to_bits(),
        cumulative_infiltration_m.to_bits(),
        "ET must consume the WB14 same-pass infiltration operand without frost-retained liquid"
    );
    assert!(
        (day.runoff_partition_inputs
            .frost_preprojected_local_liquid_m
            - local_partition_excess_m)
            .abs()
            <= 1.0e-12,
        "frost-retained liquid must be marked preprojected before saturation"
    );
    assert!(
        (day.evapotranspiration_surface.layer_state_after_soil_evap[0].theta_m
            - base_soil_water_m
            - local_partition_excess_m)
            .abs()
            <= 1.0e-12,
        "surface ET layer state should receive retained local liquid before R4O"
    );
    assert!(
        (day.evapotranspiration_surface_shadow_projection
            .as_ref()
            .expect("surface shadow should exist")
            .soil_water_after_soil_evap_m
            - base_soil_water_m
            - local_partition_excess_m)
            .abs()
            <= 1.0e-12,
        "surface shadow should receive retained local liquid before R4O"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn r7h_active_snowmelt_local_liquid_routes_through_wb18_same_pass() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(97, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let base_soil_water_m = 0.200;
    let liquid_input_m = 0.010;
    day.liquid_input_inputs.liquid_input_handoff_m = liquid_input_m;
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: liquid_input_m,
        snow_state_projected: true,
        active_snow_coupling: true,
        routed_melt_m: liquid_input_m,
        post_winter_rain_m: 0.0,
        runtime_swe_after_m: 0.0,
        runtime_depth_after_m: 0.0,
        runtime_density_after_kg_m3: 0.0,
        runtime_settle_day_count_after: 0.0,
        ..DirectSnowCouplingInputs::zero()
    };
    day.infiltration_depression_inputs.producer_inputs =
        Some(DirectWb14InfiltrationProducerInputs {
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: liquid_input_m / 3_600.0,
            }],
            effective_conductivity_m_s: 1.0e-9,
            matric_potential_m: 0.100,
            storage_capacity_m: 0.020,
            depression_storage_capacity_m: 0.0,
        });

    day.run_r4i_liquid_input_span()
        .expect("liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("snowmelt infiltration/depression upstream span should execute");
    let cumulative_infiltration_m = day.runoff_partition_inputs.cumulative_infiltration_m;
    assert_eq!(
        cumulative_infiltration_m.to_bits(),
        liquid_input_m.to_bits(),
        "active snowmelt must promote reconstructed same-pass infiltration to the downstream cumulative operand"
    );
    assert_eq!(
        day.percolation_inputs.same_pass_infiltration_m.to_bits(),
        liquid_input_m.to_bits(),
        "active snowmelt WB18 same-pass must use direct liquid input after depression storage"
    );
    assert_eq!(
        day.evapotranspiration_compute_inputs
            .same_pass_infiltration_m
            .to_bits(),
        liquid_input_m.to_bits(),
        "active snowmelt ET same-pass must use the same reconstructed liquid input"
    );

    let surface = DirectEvapotranspirationSurfaceState {
        soil_water_before_m: base_soil_water_m,
        soil_water_after_soil_evap_m: base_soil_water_m,
        evapotranspiration_seed_m: 0.0,
        transpiration_demand_m: 0.0,
        soil_evaporation_m: 0.0,
        residue_evaporation_m: 0.0,
        soil_evaporation_storage_return_m: 0.0,
        residue_interception_after_m: 0.0,
        stage_state_after: None,
        layer_soil_evaporation_withdrawal_m: vec![0.0],
        layer_state_after_soil_evap: vec![sample_layer(base_soil_water_m)],
    };
    day.evapotranspiration_surface_downstream_operands =
        DirectEvapotranspirationSurfaceDownstreamOperands::from(surface.clone());
    day.evapotranspiration_surface_shadow_projection =
        Some(DirectEvapotranspirationSurfaceShadowProjection {
            lane_index: 0,
            day_index: 0,
            soil_water_before_m: surface.soil_water_before_m,
            soil_water_after_soil_evap_m: surface.soil_water_after_soil_evap_m,
            evapotranspiration_seed_m: surface.evapotranspiration_seed_m,
            transpiration_demand_m: surface.transpiration_demand_m,
            soil_evaporation_m: surface.soil_evaporation_m,
            residue_evaporation_m: surface.residue_evaporation_m,
            soil_evaporation_storage_return_m: surface.soil_evaporation_storage_return_m,
            residue_interception_after_m: surface.residue_interception_after_m,
            layer_soil_evaporation_withdrawal_m: surface
                .layer_soil_evaporation_withdrawal_m
                .clone(),
            layer_state_after_soil_evap: surface.layer_state_after_soil_evap.clone(),
        });
    day.evapotranspiration_surface = surface;

    day.project_r4x_winter_local_liquid_before_saturation(Some(
        &sample_winter_frost_compute_inputs(true),
    ))
    .expect("snow-consumed local liquid should not require deferred projection");

    assert_eq!(
        day.runoff_partition_inputs
            .frost_preprojected_local_liquid_m
            .to_bits(),
        0.0_f64.to_bits(),
        "snowmelt local excess already consumed by WB18 must not be preprojected again"
    );
    assert_eq!(
        day.evapotranspiration_surface.layer_state_after_soil_evap[0]
            .theta_m
            .to_bits(),
        base_soil_water_m.to_bits(),
        "surface ET layer state must not receive duplicate local snowmelt"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn r7h_mixed_rain_snowmelt_uses_wb14_same_pass_infiltration() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(97, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let base_soil_water_m = 0.200;
    let liquid_input_m = 0.010;
    let routed_melt_m = 0.004;
    let post_winter_rain_m = liquid_input_m - routed_melt_m;
    day.liquid_input_inputs.liquid_input_handoff_m = liquid_input_m;
    day.storage_reconciliation_inputs.precip_input_m = post_winter_rain_m;
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: routed_melt_m,
        snow_state_projected: true,
        active_snow_coupling: true,
        routed_melt_m,
        post_winter_rain_m,
        runtime_swe_after_m: 0.0,
        runtime_depth_after_m: 0.0,
        runtime_density_after_kg_m3: 0.0,
        runtime_settle_day_count_after: 0.0,
        ..DirectSnowCouplingInputs::zero()
    };
    day.infiltration_depression_inputs.producer_inputs =
        Some(DirectWb14InfiltrationProducerInputs {
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: liquid_input_m / 3_600.0,
            }],
            effective_conductivity_m_s: 1.0e-9,
            matric_potential_m: 0.100,
            storage_capacity_m: 0.020,
            depression_storage_capacity_m: 0.0,
        });

    day.run_r4i_liquid_input_span()
        .expect("liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("mixed rain/snowmelt infiltration span should execute");
    let cumulative_infiltration_m = day.runoff_partition_inputs.cumulative_infiltration_m;
    assert!(
        cumulative_infiltration_m < liquid_input_m,
        "test setup should leave WB14-limited mixed-event excess"
    );
    assert_eq!(
        day.percolation_inputs.same_pass_infiltration_m.to_bits(),
        cumulative_infiltration_m.to_bits(),
        "mixed rain plus snowmelt must keep WB14 cumulative infiltration as the same-pass operand"
    );
    assert_eq!(
        day.evapotranspiration_compute_inputs
            .same_pass_infiltration_m
            .to_bits(),
        cumulative_infiltration_m.to_bits(),
        "mixed rain plus snowmelt ET same-pass must not promote full liquid input"
    );

    let surface = DirectEvapotranspirationSurfaceState {
        soil_water_before_m: base_soil_water_m,
        soil_water_after_soil_evap_m: base_soil_water_m,
        evapotranspiration_seed_m: 0.0,
        transpiration_demand_m: 0.0,
        soil_evaporation_m: 0.0,
        residue_evaporation_m: 0.0,
        soil_evaporation_storage_return_m: 0.0,
        residue_interception_after_m: 0.0,
        stage_state_after: None,
        layer_soil_evaporation_withdrawal_m: vec![0.0],
        layer_state_after_soil_evap: vec![sample_layer(base_soil_water_m)],
    };
    day.evapotranspiration_surface_downstream_operands =
        DirectEvapotranspirationSurfaceDownstreamOperands::from(surface.clone());
    day.evapotranspiration_surface_shadow_projection =
        Some(DirectEvapotranspirationSurfaceShadowProjection {
            lane_index: 0,
            day_index: 0,
            soil_water_before_m: surface.soil_water_before_m,
            soil_water_after_soil_evap_m: surface.soil_water_after_soil_evap_m,
            evapotranspiration_seed_m: surface.evapotranspiration_seed_m,
            transpiration_demand_m: surface.transpiration_demand_m,
            soil_evaporation_m: surface.soil_evaporation_m,
            residue_evaporation_m: surface.residue_evaporation_m,
            soil_evaporation_storage_return_m: surface.soil_evaporation_storage_return_m,
            residue_interception_after_m: surface.residue_interception_after_m,
            layer_soil_evaporation_withdrawal_m: surface
                .layer_soil_evaporation_withdrawal_m
                .clone(),
            layer_state_after_soil_evap: surface.layer_state_after_soil_evap.clone(),
        });
    day.evapotranspiration_surface = surface;

    day.project_r4x_winter_local_liquid_before_saturation(Some(
        &sample_winter_frost_compute_inputs(true),
    ))
    .expect("mixed rain/snowmelt excess must not require winter local projection");

    assert_eq!(
        day.runoff_partition_inputs
            .frost_preprojected_local_liquid_m
            .to_bits(),
        0.0_f64.to_bits(),
        "mixed rain/snowmelt excess must remain available for runoff partitioning"
    );
    assert_eq!(
        day.evapotranspiration_surface.layer_state_after_soil_evap[0]
            .theta_m
            .to_bits(),
        base_soil_water_m.to_bits(),
        "mixed rain/snowmelt excess must not be projected into storage before saturation"
    );
}

#[test]
fn r7h_r4a_no_material_rainfall_excess_remains_runoff() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(98, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let base_soil_water_m = 0.200;
    let liquid_input_m = 0.010;
    day.percolation.layer_state_after = vec![sample_layer(base_soil_water_m)];
    let storage_before_m = aggregate_test_layer_storage(&day.percolation.layer_state_after[0]);
    day.liquid_input_inputs.liquid_input_handoff_m = liquid_input_m;

    day.run_r4i_liquid_input_span()
        .expect("liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("zero infiltration/depression upstream span should execute");
    day.run_r4l_saturation_addback_span()
        .expect("zero saturation addback upstream span should execute");

    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&sample_winter_frost_compute_inputs(
        false,
    )))
    .expect("ordinary no-material rainfall excess should remain runoff");

    let retained_liquid_m = day.runoff_partition_inputs.frost_retained_local_liquid_m;
    assert_eq!(
        retained_liquid_m.to_bits(),
        0.0_f64.to_bits(),
        "no-material ordinary rainfall excess must not be classified as frost-retained liquid"
    );
    assert!(
        (day.runoff_partition.q_runoff_m - liquid_input_m).abs() <= 1.0e-12,
        "ordinary rainfall excess should remain R4A partition runoff"
    );
    assert!(
        (aggregate_test_layer_storage(&day.percolation.layer_state_after[0]) - storage_before_m)
            .abs()
            <= 1.0e-12,
        "no-material ordinary rainfall excess must not be projected into typed storage layers"
    );
}

#[test]
fn r7h_r4a_material_frost_retained_liquid_projects_to_storage_layers() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(98, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let base_soil_water_m = 0.200;
    let prior_frost_depth_m = 0.005;
    let prior_frozen_water_m = 0.001;
    let liquid_input_m = 0.010;
    day.percolation.layer_state_after = vec![sample_layer_with_frost(
        base_soil_water_m,
        prior_frost_depth_m,
        prior_frozen_water_m,
    )];
    day.winter_column.frost = DirectFrostLaneState::from(sample_frost_runtime_carry(
        prior_frost_depth_m,
        prior_frozen_water_m,
        0.123,
    ));
    let storage_before_m = aggregate_test_layer_storage(&day.percolation.layer_state_after[0]);
    day.liquid_input_inputs.liquid_input_handoff_m = liquid_input_m;

    day.run_r4i_liquid_input_span()
        .expect("liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("zero infiltration/depression upstream span should execute");
    day.run_r4l_saturation_addback_span()
        .expect("zero saturation addback upstream span should execute");

    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&sample_winter_frost_compute_inputs(
        false,
    )))
    .expect("material prior frost path should retain local winter liquid");

    let retained_liquid_m = day.runoff_partition_inputs.frost_retained_local_liquid_m;
    assert!(
        (retained_liquid_m - liquid_input_m).abs() <= 1.0e-12,
        "test setup should retain the full local winter liquid excess"
    );
    assert_eq!(day.runoff_partition.q_runoff_m.to_bits(), 0.0_f64.to_bits());
    assert_eq!(
        day.storage_reconciliation_inputs
            .frost_liquid_delta_m
            .to_bits(),
        0.0_f64.to_bits(),
        "retained snowmelt is already a storage input, not a freeze/thaw delta"
    );
    assert!(
        (aggregate_test_layer_storage(&day.percolation.layer_state_after[0])
            - storage_before_m
            - retained_liquid_m)
            .abs()
            <= 1.0e-12,
        "R4A must project retained winter liquid into typed layer storage even without material freeze/thaw"
    );
    assert!(
        (day.water.soil_water_m - storage_before_m - retained_liquid_m).abs() <= 1.0e-12,
        "frame soil water must match retained-liquid layer projection"
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

fn sample_layer_with_frost(
    theta_m: f64,
    frozen_depth_m: f64,
    frozen_water_m: f64,
) -> DirectSubsurfaceLayerState {
    DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        frozen_depth_m,
        frozen_water_m,
        ..sample_layer_inputs(theta_m)
    })
}

fn aggregate_test_layer_storage(layer: &DirectSubsurfaceLayerState) -> f64 {
    layer.theta_m + layer.residual_theta * (layer.depth_m - layer.frozen_depth_m).max(0.0)
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
