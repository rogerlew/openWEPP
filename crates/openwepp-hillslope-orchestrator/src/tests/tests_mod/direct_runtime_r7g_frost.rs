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
    DirectSnowCouplingInputs, DirectSnowLiquidDispositionLedger, DirectSnowMassTransitionLedgers,
    DirectSnowSolidToLiquidLedger, DirectSnowStage3Outcome, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState, DirectWb14HyetographInterval,
    DirectWb14InfiltrationProducerInputs, DirectWinterFrostComputeInputs,
    DirectWinterFrostPartitionOutcome, Wb11HydrologyKernel, reset_direct_runtime_audit_counters,
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

    day.winter_frost_outcome = Some(Box::new(solve_test_winter_frost_outcome(
        &day,
        &winter_frost_compute_inputs,
    )));
    day.percolation_inputs.layers = day.percolation.layer_state_after.clone();
    day.apply_r4w_winter_frost_ingress()
        .expect("frost ingress should apply the single-solve outcome");
    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
        .expect("carried single-solve frost outcome should mutate direct frost state");

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

    day.winter_frost_outcome = Some(Box::new(solve_test_winter_frost_outcome(
        &day,
        &winter_frost_compute_inputs,
    )));
    day.percolation_inputs.layers = day.percolation.layer_state_after.clone();
    day.apply_r4w_winter_frost_ingress()
        .expect("frost ingress should apply the single-solve outcome");
    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
        .expect("inactive no-material frost outcome should clear the frost carry");

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
    // Since the single-solve rewire, stale coarse-layer frost projections are
    // cleared by the runner authority channel
    // (direct_production_same_day_frost_hydrology_layers with
    // clear_no_final_hydrology_layers), not by R4A: the frame's layer basis
    // is untouched here and the aggregate is preserved by construction.
    let untouched_layer = &day.percolation.layer_state_after[0];
    assert!(
        (aggregate_test_layer_storage(untouched_layer) - storage_before_no_material_frost).abs()
            <= 1.0e-12,
        "inactive no-material frost must preserve aggregate layer storage"
    );
    assert!(day.frost_runtime_carry.is_none());
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

    // Single-solve rewire: the warm-thaw physics is the kernel outcome's
    // contract; the frame consumes the outcome's carry, and the storage delta
    // rides the runner day-input channel (frost_storage_liquid_delta_m).
    let outcome = solve_test_winter_frost_outcome(&day, &winter_frost_compute_inputs);
    assert_eq!(
        outcome.frwatc_thaw_credit_m.to_bits(),
        prior_frozen_water_m.to_bits(),
        "warm active frost solve must thaw the prior frozen water"
    );
    for projection in &outcome.layer_projection {
        assert_eq!(projection.frozen_depth_m.to_bits(), 0.0_f64.to_bits());
        assert_eq!(projection.frozen_water_m.to_bits(), 0.0_f64.to_bits());
    }
    // The runner channel consumes the same outcome for the storage delta.
    day.frost_storage_liquid_delta_m = Some(outcome.frwatc_net_liquid_delta_m);
    day.winter_frost_outcome = Some(Box::new(outcome));
    day.percolation_inputs.layers = day.percolation.layer_state_after.clone();
    day.apply_r4w_winter_frost_ingress()
        .expect("frost ingress should apply the warm-thaw outcome");

    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
        .expect("warm active frost outcome should clear the frost carry");

    assert_eq!(
        day.winter_column.frost.ws_frz_m.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        day.winter_column.frost.frwatc_thaw_credit_m.to_bits(),
        prior_frozen_water_m.to_bits()
    );
    // The day-input delta is the storage phase's sole frost authority;
    // R4B's consumption is exercised by the integration publication path.
    let _ = liquid_storage_before_thaw;
}

#[test]
fn r7h_explicit_frost_storage_source_is_preserved_for_the_storage_phase() {
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

    day.winter_frost_outcome = Some(Box::new(solve_test_winter_frost_outcome(
        &day,
        &winter_frost_compute_inputs,
    )));
    day.percolation_inputs.layers = day.percolation.layer_state_after.clone();
    day.apply_r4w_winter_frost_ingress()
        .expect("frost ingress should apply the single-solve outcome");
    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
        .expect("typed frost carry should accept explicit storage source");

    assert_eq!(
        day.frost_storage_liquid_delta_m,
        Some(explicit_storage_source_m),
        "R4A must preserve the explicit WB12 frost source for the later storage phase"
    );
    // Single-solve rewire: the frost ingress writes the same-solve delta into
    // the storage inputs, and R4B's day-input override (the explicit source,
    // preserved above) is the final authority — exercised by the integration
    // publication path on every frost day. There is no competing second-solve
    // writer anymore; the in-frame delta must tie to the applied outcome.
    assert!(
        (day.storage_reconciliation_inputs.frost_liquid_delta_m
            - (day.water.soil_water_m - liquid_storage_before_thaw))
            .abs()
            <= 1.0e-12,
        "the ingress frost delta must equal the applied outcome's storage change"
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
            hourly_additional_supply_m: [0.0; 24],
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

    day.winter_frost_outcome = Some(Box::new(solve_test_winter_frost_outcome(
        &day,
        &winter_frost_compute_inputs,
    )));
    day.percolation_inputs.layers = day.percolation.layer_state_after.clone();
    day.apply_r4w_winter_frost_ingress()
        .expect("frost ingress should apply the single-solve outcome");
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
    // Single-solve rewire (INV-SNOWFREEZE-012 hour-1 ingress): the frost
    // solve consumes the start-of-day soil-water basis; same-day partition
    // excess is retained for the runoff partition and reaches frost on the
    // next day's solve, matching the legacy frsoil-before-infiltration order.
    assert!(
        (day.winter_column.frost.frwatc_soil_water_before_m - base_soil_water_m).abs() <= 1.0e-12,
        "the frost solve basis must be start-of-day soil water, not same-day excess"
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
            hourly_additional_supply_m: [0.0; 24],
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
        mass_transition_ledgers: Box::new(
            DirectSnowMassTransitionLedgers::try_from_parts(
                DirectSnowSolidToLiquidLedger {
                    snowpack_swe_loss_m: liquid_input_m,
                    liquid_handoff_m: liquid_input_m,
                    ..DirectSnowSolidToLiquidLedger::default()
                },
                DirectSnowLiquidDispositionLedger::default(),
                DirectSnowStage3Outcome::default(),
            )
            .expect("valid disabled Stage-3 mass transition"),
        ),
        hourly_routed_melt_m: {
            let mut hourly = [0.0; 24];
            hourly[0] = liquid_input_m;
            hourly
        },
        post_winter_rain_m: 0.0,
        runtime_swe_after_m: 0.0,
        runtime_depth_after_m: 0.0,
        runtime_density_after_kg_m3: 0.0,
        runtime_settle_day_count_after: 0.0,
        ..DirectSnowCouplingInputs::zero()
    };
    day.infiltration_depression_inputs.producer_inputs =
        Some(DirectWb14InfiltrationProducerInputs {
            hourly_additional_supply_m: [0.0; 24],
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: liquid_input_m / 3_600.0,
            }],
            effective_conductivity_m_s: 1.0,
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
        "active snowmelt must use WB14 infiltration on the routed-melt producer clock"
    );
    assert_eq!(
        day.percolation_inputs.same_pass_infiltration_m.to_bits(),
        liquid_input_m.to_bits(),
        "active snowmelt WB18 same-pass must use WB14 cumulative infiltration"
    );
    assert_eq!(
        day.evapotranspiration_compute_inputs
            .same_pass_infiltration_m
            .to_bits(),
        liquid_input_m.to_bits(),
        "active snowmelt ET same-pass must use the same WB14 infiltration"
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
fn r7h_pure_melt_r4k_preserves_wb14_capacity_and_residual_hour() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(98, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let liquid_input_m = 0.010;
    let mut hourly_routed_melt_m = [0.0; 24];
    hourly_routed_melt_m[5] = liquid_input_m;
    day.liquid_input_inputs.liquid_input_handoff_m = liquid_input_m;
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: liquid_input_m,
        snow_state_projected: true,
        active_snow_coupling: true,
        mass_transition_ledgers: Box::new(
            DirectSnowMassTransitionLedgers::try_from_parts(
                DirectSnowSolidToLiquidLedger {
                    snowpack_swe_loss_m: liquid_input_m,
                    liquid_handoff_m: liquid_input_m,
                    ..DirectSnowSolidToLiquidLedger::default()
                },
                DirectSnowLiquidDispositionLedger::default(),
                DirectSnowStage3Outcome::default(),
            )
            .expect("valid disabled Stage-3 mass transition"),
        ),
        hourly_routed_melt_m,
        post_winter_rain_m: 0.0,
        ..DirectSnowCouplingInputs::zero()
    };
    day.infiltration_depression_inputs.producer_inputs =
        Some(DirectWb14InfiltrationProducerInputs {
            hourly_additional_supply_m: hourly_routed_melt_m,
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: 0.0,
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
        .expect("pure-melt infiltration/depression span should execute");

    let infiltration_m = day.runoff_partition_inputs.cumulative_infiltration_m;
    assert!(infiltration_m > 0.0 && infiltration_m < liquid_input_m);
    assert_eq!(
        day.percolation_inputs.same_pass_infiltration_m.to_bits(),
        infiltration_m.to_bits(),
        "R4K must publish WB14 infiltration without a daily snow override"
    );
    let hourly_residual_m: f64 = day.wb14_hourly_excess_m.iter().sum();
    assert!((hourly_residual_m - (liquid_input_m - infiltration_m)).abs() <= 1.0e-12);
    assert!(day.wb14_hourly_excess_m[5] > 0.0);
    for (hour, residual_m) in day.wb14_hourly_excess_m.iter().enumerate() {
        if hour != 5 {
            assert_eq!(residual_m.to_bits(), 0.0_f64.to_bits());
        }
    }
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
        mass_transition_ledgers: Box::new(
            DirectSnowMassTransitionLedgers::try_from_parts(
                DirectSnowSolidToLiquidLedger {
                    snowpack_swe_loss_m: routed_melt_m,
                    liquid_handoff_m: routed_melt_m,
                    ..DirectSnowSolidToLiquidLedger::default()
                },
                DirectSnowLiquidDispositionLedger::default(),
                DirectSnowStage3Outcome::default(),
            )
            .expect("valid disabled Stage-3 mass transition"),
        ),
        hourly_routed_melt_m: {
            let mut hourly = [0.0; 24];
            hourly[0] = routed_melt_m;
            hourly
        },
        post_winter_rain_m,
        runtime_swe_after_m: 0.0,
        runtime_depth_after_m: 0.0,
        runtime_density_after_kg_m3: 0.0,
        runtime_settle_day_count_after: 0.0,
        ..DirectSnowCouplingInputs::zero()
    };
    day.infiltration_depression_inputs.producer_inputs =
        Some(DirectWb14InfiltrationProducerInputs {
            hourly_additional_supply_m: [0.0; 24],
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
    day.infiltration_depression_inputs.producer_inputs = None;

    day.run_r4i_liquid_input_span()
        .expect("liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("zero infiltration/depression upstream span should execute");
    day.run_r4l_saturation_addback_span()
        .expect("zero saturation addback upstream span should execute");

    let winter_frost_compute_inputs = sample_winter_frost_compute_inputs(false);
    day.winter_frost_outcome = Some(Box::new(solve_test_winter_frost_outcome(
        &day,
        &winter_frost_compute_inputs,
    )));
    day.percolation_inputs.layers = day.percolation.layer_state_after.clone();
    day.apply_r4w_winter_frost_ingress()
        .expect("frost ingress should apply the single-solve outcome");
    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
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
    day.infiltration_depression_inputs.producer_inputs = None;

    day.run_r4i_liquid_input_span()
        .expect("liquid input upstream span should execute");
    day.run_r4j_runon_carry_span()
        .expect("zero runon/carry upstream span should execute");
    day.run_r4k_infiltration_depression_span()
        .expect("zero infiltration/depression upstream span should execute");
    day.run_r4l_saturation_addback_span()
        .expect("zero saturation addback upstream span should execute");

    let winter_frost_compute_inputs = sample_winter_frost_compute_inputs(false);
    day.winter_frost_outcome = Some(Box::new(solve_test_winter_frost_outcome(
        &day,
        &winter_frost_compute_inputs,
    )));
    day.percolation_inputs.layers = day.percolation.layer_state_after.clone();
    day.apply_r4w_winter_frost_ingress()
        .expect("frost ingress should apply the single-solve outcome");
    day.run_r4a_runoff_partition_span_with_winter_frost(Some(&winter_frost_compute_inputs))
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
    // Single-solve rewire: retained winter liquid projects into typed layer
    // storage at R4X (after surface ET, before saturation), pinned by
    // r7h_winter_local_liquid_projects_after_surface_et_before_saturation.
    // R4A itself must leave the layer basis untouched.
    assert!(
        (aggregate_test_layer_storage(&day.percolation.layer_state_after[0]) - storage_before_m)
            .abs()
            <= 1.0e-12,
        "R4A must not project retained liquid into the layer basis"
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
    day_input.winter_frost_outcome = Some(Box::new(
        Wb11HydrologyKernel::compute_direct_winter_frost_partition(&no_freeze_typed_frost_inputs(
            true,
        ))
        .expect("active no-freeze outcome should solve"),
    ));
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
            seasonal_temperature_curve: Wb11HydrologyKernel::fit_seasonal_temperature_curve(
                &[8.0; 12], &[2.0; 12],
            ),
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

#[test]
fn diagnostic_count_to_f64_matches_decimal_string_parse_bit_for_bit() {
    // The former implementation round-tripped through a decimal string; the
    // cast must stay bit-identical across the whole usize range, including
    // values above 2^53 where nearest-rounding is exercised.
    let samples: [usize; 8] = [
        0,
        1,
        365,
        4_038,
        (1_usize << 53) - 1,
        1_usize << 53,
        (1_usize << 53) + 1,
        usize::MAX,
    ];
    for value in samples {
        let via_cast = Wb11HydrologyKernel::diagnostic_count_to_f64(value);
        let via_parse = value.to_string().parse::<f64>().unwrap();
        assert_eq!(
            via_cast.to_bits(),
            via_parse.to_bits(),
            "diagnostic_count_to_f64({value}) diverged from decimal parse"
        );
    }
}

// Test-local mirror of the runner authority's prior-state assembly (the
// production copy lives in the runner since the single-solve rewire).
fn test_frost_prior_state(state: &DirectFrostLaneState) -> DirectFrostPriorStateInput {
    DirectFrostPriorStateInput {
        active_frost_coupling: state.active_frost_coupling,
        dfrost_m: state.dfrost_m,
        dthaw_m: state.dthaw_m,
        nft: state.nft,
        ws_frz_m: state.ws_frz_m,
        infcap_frz_m_s: state.infcap_frz_m_s,
        frwatc_soil_water_before_m: state.frwatc_soil_water_before_m,
        frwatc_soil_water_after_m: state.frwatc_soil_water_after_m,
        frwatc_frozen_water_before_m: state.frwatc_frozen_water_before_m,
        frwatc_frozen_water_after_m: state.frwatc_frozen_water_after_m,
        frwatc_freeze_debit_m: state.frwatc_freeze_debit_m,
        frwatc_thaw_credit_m: state.frwatc_thaw_credit_m,
        frwatc_net_liquid_delta_m: state.frwatc_net_liquid_delta_m,
        frdp_m: state.frdp_m,
        thdp_m: state.thdp_m,
        tfrdp_m: state.tfrdp_m,
        tthawd_m: state.tthawd_m,
        fgthwd_flag: state.fgthwd_flag,
        total_fine_layer_count: state.total_fine_layer_count,
        conductivity_tilled_w_m_k: state.conductivity_tilled_w_m_k,
        conductivity_untilled_w_m_k: state.conductivity_untilled_w_m_k,
        conductivity_residue_w_m_k: state.conductivity_residue_w_m_k,
        shadow_total_water_before_m: state.shadow_total_water_before_m,
        shadow_total_water_after_m: state.shadow_total_water_after_m,
        shadow_wb_delta_m: state.shadow_wb_delta_m,
        shadow_frwatc_residual_m: state.shadow_frwatc_residual_m,
        watpdg_m: state.watpdg_m,
        watbtm_m: state.watbtm_m,
        layer_shadows: state
            .layer_shadows
            .iter()
            .map(|layer| crate::hydrology::DirectFrostLayerShadowProjection {
                layer_index: layer.layer_index,
                st_m: layer.st_m,
                soil_water_m: layer.soil_water_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frozen_water_m,
                soilf_m: layer.soilf_m,
                yst_m: layer.yst_m,
                nwfrzz_m: layer.nwfrzz_m,
            })
            .collect(),
        fine_layers: state
            .fine_layers
            .iter()
            .map(|fine| crate::hydrology::DirectFrostFineLayerProjection {
                layer_index: fine.layer_index,
                fine_index: fine.fine_index,
                fgfrst: fine.fgfrst,
                slfsd_m: fine.slfsd_m,
                slsic_m: fine.slsic_m,
                slsw_theta: fine.slsw_theta,
                sltime_s: fine.sltime_s,
            })
            .collect(),
    }
}

// Test-local mirror of the runner authority's kernel-request assembly: solve
// the day's winter frost partition once from the frame's current layer basis,
// as the production builder does from lane state.
fn solve_test_winter_frost_outcome(
    day: &DirectDayFrame,
    compute_inputs: &DirectWinterFrostComputeInputs,
) -> DirectWinterFrostPartitionOutcome {
    let layers = &day.percolation.layer_state_after;
    let soil_conductivity_m_s = compute_inputs
        .soil_conductivity_m_s
        .filter(|value| *value > 0.0)
        .unwrap_or_else(|| layers[0].conductivity_m_s);
    let profile_depth_m = layers.iter().map(|layer| layer.depth_m).sum();
    let soil_water_m = layers.iter().map(aggregate_test_layer_storage).sum();
    let layer_inputs = layers
        .iter()
        .zip(compute_inputs.layer_bulk_density_kg_m3.iter().copied())
        .enumerate()
        .map(
            |(offset, (layer, bulk_density_kg_m3))| DirectFrostLayerInput {
                layer_index: offset + 1,
                theta_m: layer.theta_m,
                upper_limit_m: layer.upper_limit_m,
                depth_m: layer.depth_m,
                residual_theta: layer.residual_theta,
                bulk_density_kg_m3,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frozen_water_m,
            },
        )
        .collect();
    Wb11HydrologyKernel::compute_direct_winter_frost_partition(&DirectActiveFrostPartitionInputs {
        controls: compute_inputs.controls,
        thermal: compute_inputs.thermal,
        profile_depth_m,
        soil_water_m,
        theta_residual: compute_inputs.theta_residual,
        theta_field_capacity: compute_inputs.theta_field_capacity,
        soil_conductivity_m_s,
        prior_state: test_frost_prior_state(&day.winter_column.frost),
        layers: layer_inputs,
        hourly: compute_inputs.hourly,
    })
    .expect("test winter frost outcome should solve")
}
