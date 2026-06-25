use super::*;

#[test]
fn r4b_storage_reconciliation_consumes_r4a_q_and_shadow_projects() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4B_STORAGE_RECONCILIATION_SPAN,
        [
            DirectPhaseKind::StorageReconciliation,
            DirectPhaseKind::ClosureDiagnostics
        ]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day = r4b_valid_day(identity);
    day.interception_m = 0.002;
    day.storage_reconciliation_inputs.interception_m = day.interception_m;
    day.publication.runoff_m = 0.125;
    day.water_ledger.diagnostic_residual_m = 0.1875;

    let report = day
        .run_r4b_storage_reconciliation_span()
        .expect("valid R4B storage reconciliation span should execute");

    let expected_state = r4b_expected_storage_state();
    assert_r4b_storage_result(&day, &report, expected_state);
    assert_r4b_storage_anti_aliases(expected_state, &day);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 13);
    assert_eq!(
        audit.direct_phase_entries,
        (DIRECT_R3A_PHASE_SPAN_COUNT
            + DIRECT_R4C_PHASE_SPAN_COUNT
            + DIRECT_R4M_PHASE_SPAN_COUNT
            + DIRECT_R4N_PHASE_SPAN_COUNT
            + DIRECT_R4O_PHASE_SPAN_COUNT
            + DIRECT_R4G_PHASE_SPAN_COUNT
            + DIRECT_R4I_PHASE_SPAN_COUNT
            + DIRECT_R4J_PHASE_SPAN_COUNT
            + DIRECT_R4K_PHASE_SPAN_COUNT
            + DIRECT_R4L_PHASE_SPAN_COUNT
            + DIRECT_R4A_PHASE_SPAN_COUNT
            + DIRECT_R4B_PHASE_SPAN_COUNT) as u64
    );
    assert_eq!(audit.direct_compute_operations, 13);
    assert_eq!(audit.direct_state_mutations, 13);
    assert_eq!(audit.downstream_operand_productions, 13);
    assert_eq!(audit.shadow_projections, 13);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

fn assert_r4b_storage_result(
    day: &DirectDayFrame,
    report: &crate::DirectStorageReconciliationSpanReport,
    expected_state: DirectStorageReconciliationState,
) {
    let expected_operands = DirectStorageDownstreamOperands::from(expected_state);
    let expected_shadow = r4b_expected_storage_shadow();

    assert_eq!(day.storage_reconciliation, expected_state);
    assert_eq!(day.water.soil_water_m.to_bits(), 1.076_125_f64.to_bits());
    assert_eq!(day.storage_downstream_operands, expected_operands);
    assert_eq!(day.storage_shadow_projection, Some(expected_shadow));
    assert_eq!(report.phase_count, DIRECT_R4B_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4B_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.storage_shadow_projection, expected_shadow);
    assert_eq!(
        day.storage_input,
        DirectStorageInputState {
            storage_initial_m: 1.0,
            precip_input_m: 0.25
        }
    );
    assert_eq!(
        day.deep_seepage,
        DirectDeepSeepageState {
            deep_seepage_m: 0.03125
        }
    );
    assert_eq!(
        day.subsurface_loss,
        DirectSubsurfaceLossState {
            subsurface_loss_m: 0.015_625
        }
    );
    assert_eq!(
        day.evapotranspiration_compute
            .evapotranspiration_m
            .to_bits(),
        0.0625_f64.to_bits()
    );
    assert_eq!(
        day.snow_coupling,
        DirectSnowCouplingState {
            snow_coupling_m: 0.125,
            ..DirectSnowCouplingState::zero()
        }
    );
}

fn r4b_expected_storage_state() -> DirectStorageReconciliationState {
    DirectStorageReconciliationState {
        storage_initial_m: 1.0,
        precip_input_m: 0.25,
        snow_coupling_m: 0.125,
        frost_liquid_delta_m: 0.0,
        runon_input_m: 0.140_625,
        interception_m: 0.002,
        q_runoff_m: 0.328_125,
        evapotranspiration_m: 0.0625,
        deep_seepage_m: 0.03125,
        subsurface_loss_m: 0.015_625,
        closure_tolerance_m: 0.0,
        storage_reconciled_m: 1.076_125,
        closure_residual_m: 0.0,
    }
}

fn r4b_expected_storage_shadow() -> DirectStorageShadowProjection {
    DirectStorageShadowProjection {
        lane_index: 0,
        day_index: 0,
        storage_initial_m: 1.0,
        precip_input_m: 0.25,
        snow_coupling_m: 0.125,
        frost_liquid_delta_m: 0.0,
        runon_input_m: 0.140_625,
        interception_m: 0.002,
        q_runoff_m: 0.328_125,
        evapotranspiration_m: 0.0625,
        deep_seepage_m: 0.03125,
        subsurface_loss_m: 0.015_625,
        storage_reconciled_m: 1.076_125,
        closure_residual_m: 0.0,
    }
}

fn assert_r4b_storage_anti_aliases(
    expected_state: DirectStorageReconciliationState,
    day: &DirectDayFrame,
) {
    let omitted_s_m = expected_state.storage_initial_m + expected_state.precip_input_m
        - expected_state.interception_m
        - expected_state.q_runoff_m
        - expected_state.evapotranspiration_m
        - expected_state.deep_seepage_m
        - expected_state.subsurface_loss_m;
    assert_ne!(
        expected_state.storage_reconciled_m.to_bits(),
        omitted_s_m.to_bits()
    );
    let wrong_q_sign_m = expected_state.storage_initial_m
        + expected_state.precip_input_m
        + expected_state.snow_coupling_m
        + expected_state.runon_input_m
        - expected_state.interception_m
        + expected_state.q_runoff_m
        - expected_state.evapotranspiration_m
        - expected_state.deep_seepage_m
        - expected_state.subsurface_loss_m;
    assert_ne!(
        expected_state.storage_reconciled_m.to_bits(),
        wrong_q_sign_m.to_bits()
    );
    let omitted_losses_m = expected_state.storage_initial_m
        + expected_state.precip_input_m
        + expected_state.snow_coupling_m
        + expected_state.runon_input_m
        - expected_state.interception_m
        - expected_state.q_runoff_m;
    assert_ne!(
        expected_state.storage_reconciled_m.to_bits(),
        omitted_losses_m.to_bits()
    );
    let omitted_interception_m = expected_state.storage_initial_m
        + expected_state.precip_input_m
        + expected_state.snow_coupling_m
        + expected_state.runon_input_m
        - expected_state.q_runoff_m
        - expected_state.evapotranspiration_m
        - expected_state.deep_seepage_m
        - expected_state.subsurface_loss_m;
    assert_ne!(
        expected_state.storage_reconciled_m.to_bits(),
        omitted_interception_m.to_bits()
    );
    let publication_q_alias_m = expected_state.storage_initial_m
        + expected_state.precip_input_m
        + expected_state.snow_coupling_m
        + expected_state.runon_input_m
        - expected_state.interception_m
        - day.publication.runoff_m
        - expected_state.evapotranspiration_m
        - expected_state.deep_seepage_m
        - expected_state.subsurface_loss_m;
    assert_ne!(
        expected_state.storage_reconciled_m.to_bits(),
        publication_q_alias_m.to_bits()
    );
    let ledger_q_alias_m = expected_state.storage_initial_m
        + expected_state.precip_input_m
        + expected_state.snow_coupling_m
        + expected_state.runon_input_m
        - expected_state.interception_m
        - day.water_ledger.diagnostic_residual_m
        - expected_state.evapotranspiration_m
        - expected_state.deep_seepage_m
        - expected_state.subsurface_loss_m;
    assert_ne!(
        expected_state.storage_reconciled_m.to_bits(),
        ledger_q_alias_m.to_bits()
    );
}

#[test]
fn r4b_storage_reconciliation_rejects_missing_upstream_producers() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");

    let mut missing_storage_input_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    assert_eq!(
        missing_storage_input_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("R4B should require R4C direct upstream execution"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4C storage input producer"
        }
    );

    let mut missing_percolation_day = r4b_day_after_r4c(identity);
    assert_eq!(
        missing_percolation_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("R4B should require R4M direct upstream execution"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4M percolation producer"
        }
    );

    let mut missing_subsurface_compute_day = r4b_day_after_r4m(identity);
    assert_eq!(
        missing_subsurface_compute_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("R4B should require R4O direct upstream execution"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4O subsurface compute producer"
        }
    );

    let mut missing_et_day = r4b_day_after_r4o(identity);
    assert_eq!(
        missing_et_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("R4B should require R4N direct upstream execution"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4N evapotranspiration/root-uptake producer"
        }
    );

    let mut missing_snow_day = r4b_day_after_r4n(identity);
    assert_eq!(
        missing_snow_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("R4B should require R4G direct upstream execution"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4G snow-coupling producer"
        }
    );

    let mut missing_runoff_partition_day = r4b_day_after_r4g(identity);
    assert_eq!(
        missing_runoff_partition_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("R4B should still require R4A direct upstream execution"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4A runoff partition"
        }
    );
}

#[test]
fn r4b_storage_reconciliation_rejects_invalid_values() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");

    let mut nonfinite_s_day = r4b_valid_day(identity);
    nonfinite_s_day
        .storage_reconciliation_inputs
        .snow_coupling_m = f64::NAN;
    assert_eq!(
        nonfinite_s_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("nonfinite S should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "storage_reconciliation.snow_coupling_m"
        }
    );

    let mut negative_loss_day = r4b_valid_day(identity);
    negative_loss_day
        .storage_reconciliation_inputs
        .subsurface_loss_m = -0.125;
    assert_eq!(
        negative_loss_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("negative Qd should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "storage_reconciliation.subsurface_loss_m"
        }
    );

    let mut negative_storage_day = r4b_valid_day(identity);
    negative_storage_day
        .storage_reconciliation_inputs
        .deep_seepage_m = 2.0;
    assert_eq!(
        negative_storage_day
            .run_r4b_storage_reconciliation_span()
            .expect_err("negative reconciled storage should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "storage_reconciliation.storage_reconciled_m"
        }
    );
}

#[test]
fn r4b_explicit_frost_storage_rebalance_debits_multiple_layers() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day = r4b_valid_day(identity);
    set_r4b_explicit_frost_projection_fixture(
        &mut day,
        vec![
            r4b_projection_layer(0.010, 0.0),
            r4b_projection_layer(0.050, 0.0),
        ],
        -0.030,
    );

    let report = day
        .run_r4b_storage_reconciliation_span()
        .expect("valid explicit frost storage debit should rebalance across layers");

    assert!(
        (day.storage_reconciliation.storage_reconciled_m - 0.030).abs() <= 1.0e-12,
        "storage reconciliation must preserve aggregate explicit frost debit"
    );
    assert!(
        (day.evapotranspiration_compute.soil_water_after_m - 0.030).abs() <= 1.0e-12,
        "R4N projection aggregate must track reconciled storage"
    );
    assert_eq!(
        day.evapotranspiration_compute.layer_state_after_root_uptake[0]
            .theta_m
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert!(
        (day.evapotranspiration_compute.layer_state_after_root_uptake[1].theta_m - 0.030).abs()
            <= 1.0e-12,
        "second layer must carry the residual explicit frost debit"
    );
    let shadow = day
        .evapotranspiration_compute_shadow_projection
        .as_ref()
        .expect("R4N shadow projection should remain present");
    assert!(
        (shadow.soil_water_after_m - 0.030).abs() <= 1.0e-12,
        "R4N shadow projection aggregate must track reconciled storage"
    );
    assert_eq!(
        shadow.layer_state_after_root_uptake,
        day.evapotranspiration_compute.layer_state_after_root_uptake
    );
    assert_eq!(report.state_mutation_count, 3);
}

#[test]
fn r4b_explicit_frost_storage_rebalance_rejects_insufficient_active_theta() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day = r4b_valid_day(identity);
    set_r4b_explicit_frost_projection_fixture(
        &mut day,
        vec![
            r4b_projection_layer(0.010, 0.020),
            r4b_projection_layer(0.005, 0.010),
        ],
        -0.030,
    );
    let layer_state_before = day
        .evapotranspiration_compute
        .layer_state_after_root_uptake
        .clone();
    let shadow_before = day.evapotranspiration_compute_shadow_projection.clone();

    assert_eq!(
        day.run_r4b_storage_reconciliation_span()
            .expect_err("material active-theta deficit must fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "storage_reconciliation.frost_storage_projection_theta_m"
        }
    );
    assert_eq!(
        day.evapotranspiration_compute.layer_state_after_root_uptake, layer_state_before,
        "insufficient active theta must fail before mutating layer projection"
    );
    assert_eq!(
        day.evapotranspiration_compute_shadow_projection, shadow_before,
        "insufficient active theta must fail before mutating shadow projection"
    );
}

fn r4b_day_after_r4c(identity: DirectRunIdentity) -> DirectDayFrame {
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.run_r3a_input_accounting_span()
        .expect("R3A upstream span should pass before R4C");
    day.run_r4c_storage_input_span()
        .expect("R4C upstream span should pass before R4B");
    day
}

fn r4b_day_after_r4m(identity: DirectRunIdentity) -> DirectDayFrame {
    let mut day = r4b_day_after_r4c(identity);
    day.percolation_inputs = r4b_percolation_inputs();
    day.run_r4m_percolation_span()
        .expect("R4M upstream span should pass before R4B");
    day
}

fn r4b_day_after_r4o(identity: DirectRunIdentity) -> DirectDayFrame {
    let mut day = r4b_day_after_r4m(identity);
    day.subsurface_compute_inputs = r4b_subsurface_inputs();
    day.run_r4o_subsurface_compute_span()
        .expect("R4O upstream span should pass before R4B");
    day
}

fn r4b_day_after_r4n(identity: DirectRunIdentity) -> DirectDayFrame {
    let mut day = r4b_day_after_r4m(identity);
    day.evapotranspiration_compute_inputs = r4b_evapotranspiration_compute_inputs();
    day.run_r4n_surface_et_span()
        .expect("R4N surface upstream span should pass before R4O");
    day.subsurface_compute_inputs = r4b_subsurface_inputs();
    day.run_r4o_subsurface_compute_span()
        .expect("R4O upstream span should pass before R4N root uptake");
    day.run_r4n_root_uptake_span()
        .expect("R4N root uptake span should pass before R4B");
    day
}

fn r4b_day_after_r4g(identity: DirectRunIdentity) -> DirectDayFrame {
    let mut day = r4b_day_after_r4n(identity);
    day.run_r4g_snow_coupling_span()
        .expect("R4G upstream span should pass before R4B");
    day
}

fn r4b_percolation_inputs() -> DirectPercolationInputs {
    DirectPercolationInputs {
        soil_water_initial_m: 0.046_875,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps: 1,
        restrictive_layer_enabled: false,
        restrictive_layer_conductivity_m_s: 0.0,
        restrictive_layer_thickness_m: 0.0,
        layers: vec![
            DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
                theta_m: 0.015_625,
                field_capacity_m: 1.0,
                upper_limit_m: 1.0,
                conductivity_m_s: 1.0,
                depth_m: 1.0,
                residual_theta: 0.0,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 1.0,
                field_capacity_theta: 0.5,
                coca: 1.0,
                lateral_conductivity_m_s: 1.0,
            }),
            DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
                theta_m: 0.03125,
                field_capacity_m: 0.0,
                upper_limit_m: 1.0,
                conductivity_m_s: 1.0,
                depth_m: 1.0,
                residual_theta: 0.0,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 1.0,
                field_capacity_theta: 0.5,
                coca: 1.0,
                lateral_conductivity_m_s: 1.0,
            }),
        ],
    }
}

fn r4b_subsurface_inputs() -> DirectSubsurfaceComputeInputs {
    DirectSubsurfaceComputeInputs {
        avg_slope: 0.0,
        slope_length_m: 1.0,
        lateral_anisotropy_ratio: 1.0,
        soil_depth_m: 2.0,
        solwpv_mode: 2006,
        mofe_hourly_carry_arrays_enabled: false,
        lane_substeps: 1,
        drainage_capacity_m: 0.015_625,
        drain_enabled: true,
        drain_depth_m: 1.5,
        drain_spacing_m: 2.0,
        drain_diameter_m: 0.1,
        layers: vec![
            DirectSubsurfaceLayerInputs {
                theta_m: 0.0,
                field_capacity_m: 0.0,
                upper_limit_m: 1.0,
                conductivity_m_s: 1.0,
                depth_m: 1.0,
                residual_theta: 0.0,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 1.0,
                field_capacity_theta: 0.5,
                coca: 1.0,
                lateral_conductivity_m_s: 1.0,
            },
            DirectSubsurfaceLayerInputs {
                theta_m: 0.0,
                field_capacity_m: 0.0,
                upper_limit_m: 1.0,
                conductivity_m_s: 1.0,
                depth_m: 1.0,
                residual_theta: 0.0,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 1.0,
                field_capacity_theta: 0.5,
                coca: 1.0,
                lateral_conductivity_m_s: 1.0,
            },
        ],
    }
}

fn r4b_evapotranspiration_compute_inputs() -> DirectEvapotranspirationComputeInputs {
    DirectEvapotranspirationComputeInputs {
        et_demand_m: 0.0625,
        leaf_area_index: 0.0,
        canopy_cover_fraction: 0.0,
        residue_interception_m: 0.0625,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: 0.0,
        plant_tolerance: 0.25,
        growth_context_required: false,
        stage_state: None,
        pmet: None,
        pmet_compute: None,
    }
}

fn r4b_valid_day(identity: DirectRunIdentity) -> DirectDayFrame {
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.25;
    day.water.soil_water_m = 1.0;
    day.runoff_partition_inputs = DirectRunoffPartitionInputs {
        liquid_input_m: 9.0,
        runon_input_m: 9.0,
        cumulative_infiltration_m: 9.0,
        depression_storage_delta_m: 9.0,
        surface_saturation_runoff_m: 9.0,
        frost_retained_local_liquid_m: 0.0,
        frost_preprojected_local_liquid_m: 0.0,
    };
    day.liquid_input_inputs = DirectLiquidInputInputs {
        liquid_input_handoff_m: 0.5,
    };
    day.runon_carry_inputs = DirectRunonCarryInputs {
        surface_runon_handoff_m: 0.125,
        subsurface_carry_handoff_m: 0.015_625,
    };
    day.infiltration_depression_inputs = DirectInfiltrationDepressionInputs {
        cumulative_infiltration_handoff_m: 0.25,
        depression_storage_delta_handoff_m: 0.0625,
        producer_inputs: None,
    };
    day.saturation_addback_inputs = DirectSaturationAddbackInputs {
        surface_saturation_runoff_handoff_m: 0.0,
    };
    day.deep_seepage_inputs = DirectDeepSeepageInputs {
        deep_seepage_handoff_m: 9.0,
    };
    day.subsurface_loss_inputs = DirectSubsurfaceLossInputs {
        subsurface_loss_handoff_m: 9.0,
    };
    day.percolation_inputs = r4b_percolation_inputs();
    day.subsurface_compute_inputs = r4b_subsurface_inputs();
    day.evapotranspiration_inputs = DirectEvapotranspirationInputs {
        evapotranspiration_handoff_m: 0.0625,
    };
    day.evapotranspiration_compute_inputs = r4b_evapotranspiration_compute_inputs();
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: 0.125,
        ..DirectSnowCouplingInputs::zero()
    };
    day.storage_reconciliation_inputs = DirectStorageReconciliationInputs {
        storage_initial_m: 9.0,
        precip_input_m: 9.0,
        snow_coupling_m: 9.0,
        frost_liquid_delta_m: 0.0,
        runon_input_m: 0.0,
        interception_m: 9.0,
        evapotranspiration_m: 9.0,
        deep_seepage_m: 9.0,
        subsurface_loss_m: 9.0,
        closure_tolerance_m: 0.0,
    };
    day.run_r3a_input_accounting_span()
        .expect("R3A upstream span should pass before R4C");
    day.run_r4c_storage_input_span()
        .expect("R4C upstream span should pass before R4B");
    day.run_r4m_percolation_span()
        .expect("R4M upstream span should pass before R4B");
    day.run_r4n_surface_et_span()
        .expect("R4N surface upstream span should pass before R4O");
    day.run_r4o_subsurface_compute_span()
        .expect("R4O upstream span should pass before R4N root uptake");
    day.run_r4n_root_uptake_span()
        .expect("R4N root uptake span should pass before R4B");
    day.run_r4g_snow_coupling_span()
        .expect("R4G upstream span should pass before R4B");
    day.run_r4i_liquid_input_span()
        .expect("R4I upstream span should pass before R4A");
    day.run_r4j_runon_carry_span()
        .expect("R4J upstream span should pass before R4A");
    day.run_r4k_infiltration_depression_span()
        .expect("R4K upstream span should pass before R4A");
    day.run_r4l_saturation_addback_span()
        .expect("R4L upstream span should pass before R4A");
    day.run_r4a_runoff_partition_span()
        .expect("R4A upstream span should pass before R4B");
    day
}

fn set_r4b_explicit_frost_projection_fixture(
    day: &mut DirectDayFrame,
    layers: Vec<DirectSubsurfaceLayerState>,
    frost_liquid_delta_m: f64,
) {
    let aggregate_m = layers
        .iter()
        .map(|layer| layer.theta_m + layer.residual_theta * layer.depth_m)
        .sum::<f64>();
    day.frost_storage_liquid_delta_m = Some(frost_liquid_delta_m);
    day.storage_reconciliation_inputs = DirectStorageReconciliationInputs {
        storage_initial_m: aggregate_m,
        precip_input_m: 0.0,
        snow_coupling_m: 0.0,
        frost_liquid_delta_m: 0.0,
        runon_input_m: 0.0,
        interception_m: 0.0,
        evapotranspiration_m: 0.0,
        deep_seepage_m: 0.0,
        subsurface_loss_m: 0.0,
        closure_tolerance_m: 0.0,
    };
    day.runoff_downstream_operands.q_runoff_m = 0.0;
    day.evapotranspiration_compute.soil_water_after_m = aggregate_m;
    day.evapotranspiration_compute
        .layer_state_after_root_uptake
        .clone_from(&layers);
    if let Some(shadow) = &mut day.evapotranspiration_compute_shadow_projection {
        shadow.soil_water_after_m = aggregate_m;
        shadow.layer_state_after_root_uptake = layers;
    }
}

fn r4b_projection_layer(theta_m: f64, residual_theta: f64) -> DirectSubsurfaceLayerState {
    DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m,
        field_capacity_m: 1.0,
        upper_limit_m: 1.0,
        conductivity_m_s: 1.0,
        depth_m: 1.0,
        residual_theta,
        frozen_depth_m: 0.0,
        frozen_water_m: 0.0,
        porosity: 1.0,
        field_capacity_theta: 0.5,
        coca: 1.0,
        lateral_conductivity_m_s: 1.0,
    })
}

#[test]
fn r3c_lane_transfer_span_projects_multilane_topology() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R3C_LANE_TRANSFER_SPAN,
        [
            DirectPhaseKind::LateralTransfer,
            DirectPhaseKind::RunoffReconciliation,
            DirectPhaseKind::ClosureDiagnostics
        ]
    );

    let mut frame = r3c_multilane_transfer_frame();

    let report = frame
        .run_r3c_lane_transfer_span()
        .expect("valid R3C lane transfer span should execute");

    let expected_ledger = r3c_expected_lane_transfer_ledger();
    let expected_shadow = r3c_expected_transfer_shadow_projection();
    let expected_operands = DirectRunTransferDownstreamOperands::from(expected_shadow);

    assert_eq!(frame.lane_transfer_ledger, expected_ledger);
    assert_eq!(frame.lane_transfer_downstream_operands, expected_operands);
    assert_eq!(frame.lane_transfer_shadow_projection, Some(expected_shadow));
    assert_eq!(report.phase_count, DIRECT_R3C_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R3C_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.transfer_shadow_projection, expected_shadow);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.run_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 1);
    assert_eq!(
        audit.direct_phase_entries,
        DIRECT_R3C_PHASE_SPAN_COUNT as u64
    );
    assert_eq!(audit.direct_compute_operations, 1);
    assert_eq!(audit.direct_state_mutations, 1);
    assert_eq!(audit.downstream_operand_productions, 1);
    assert_eq!(audit.shadow_projections, 1);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

fn r3c_multilane_transfer_frame() -> DirectRunFrame {
    let identity =
        DirectRunIdentity::new(7, 2637, 3, 1).expect("valid direct run identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("valid direct run frame should construct");
    frame.lanes[0].area_m2 = 64.0;
    frame.lanes[0].transfer.surface_carry_m[0] = 0.25;
    frame.lanes[0].transfer.lateral_carry_m[0] = 0.125;
    frame.lanes[1].area_m2 = 128.0;
    frame.lanes[1].upstream_area_ratio = 0.5;
    frame.lanes[1].transfer.surface_carry_m[0] = 0.0625;
    frame.lanes[1].transfer.lateral_carry_m[0] = 0.03125;
    frame.lanes[2].area_m2 = 256.0;
    frame.lanes[2].upstream_area_ratio = 0.25;
    frame.lanes[2].transfer.surface_carry_m[0] = 0.015_625;
    frame.lanes[2].transfer.lateral_carry_m[0] = 0.007_812_5;
    frame
}

fn r3c_expected_lane_transfer_ledger() -> Vec<DirectLaneTransferLedger> {
    vec![
        DirectLaneTransferLedger {
            lane_id: 1,
            upstream_lane_id: 0,
            downstream_lane_id: 2,
            upstream_area_ratio: 1.0,
            area_m2: 64.0,
            outgoing_surface_m: 0.25,
            outgoing_lateral_m: 0.125,
            received_surface_m: 0.0,
            received_lateral_m: 0.0,
            net_transfer_m: -0.375,
        },
        DirectLaneTransferLedger {
            lane_id: 2,
            upstream_lane_id: 1,
            downstream_lane_id: 3,
            upstream_area_ratio: 0.5,
            area_m2: 128.0,
            outgoing_surface_m: 0.0625,
            outgoing_lateral_m: 0.03125,
            received_surface_m: 0.125,
            received_lateral_m: 0.0625,
            net_transfer_m: 0.09375,
        },
        DirectLaneTransferLedger {
            lane_id: 3,
            upstream_lane_id: 2,
            downstream_lane_id: 0,
            upstream_area_ratio: 0.25,
            area_m2: 256.0,
            outgoing_surface_m: 0.015_625,
            outgoing_lateral_m: 0.007_812_5,
            received_surface_m: 0.015_625,
            received_lateral_m: 0.007_812_5,
            net_transfer_m: 0.0,
        },
    ]
}

fn r3c_expected_transfer_shadow_projection() -> DirectRunTransferShadowProjection {
    DirectRunTransferShadowProjection {
        lane_count: 3,
        outlet_lane_id: 3,
        total_outgoing_surface_m: 0.328_125,
        total_outgoing_lateral_m: 0.164_062_5,
        total_received_surface_m: 0.140_625,
        total_received_lateral_m: 0.070_312_5,
        total_net_transfer_m: -0.28125,
    }
}

#[test]
fn r3c_lane_transfer_span_rejects_invalid_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 2, 1).expect("valid direct run identity should construct");

    let mut negative_ratio_frame =
        DirectRunFrame::skeleton(identity).expect("valid direct run frame should construct");
    negative_ratio_frame.lanes[1].upstream_area_ratio = -0.5;
    assert_eq!(
        negative_ratio_frame
            .run_r3c_lane_transfer_span()
            .expect_err("negative upstream area ratio should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "lane.upstream_area_ratio"
        }
    );

    let mut invalid_downstream_frame =
        DirectRunFrame::skeleton(identity).expect("valid direct run frame should construct");
    invalid_downstream_frame.lanes[0].downstream_lane_id = 9;
    assert_eq!(
        invalid_downstream_frame
            .run_r3c_lane_transfer_span()
            .expect_err("invalid downstream lane id should fail closed"),
        DirectRuntimeError::InvalidLaneTopology {
            lane_index: 0,
            lane_id: 1,
            upstream_lane_id: 0,
            downstream_lane_id: 9
        }
    );

    let mut nonreciprocal_topology_frame =
        DirectRunFrame::skeleton(identity).expect("valid direct run frame should construct");
    nonreciprocal_topology_frame.lanes[1].upstream_lane_id = 0;
    assert_eq!(
        nonreciprocal_topology_frame
            .run_r3c_lane_transfer_span()
            .expect_err("nonreciprocal topology should fail closed"),
        DirectRuntimeError::InvalidLaneTopology {
            lane_index: 0,
            lane_id: 1,
            upstream_lane_id: 0,
            downstream_lane_id: 2
        }
    );

    let mut multiple_outlet_frame =
        DirectRunFrame::skeleton(identity).expect("valid direct run frame should construct");
    multiple_outlet_frame.lanes[0].downstream_lane_id = 0;
    multiple_outlet_frame.lanes[1].upstream_lane_id = 0;
    assert_eq!(
        multiple_outlet_frame
            .run_r3c_lane_transfer_span()
            .expect_err("multiple outlets should fail closed"),
        DirectRuntimeError::InvalidLaneOutletCount { outlet_count: 2 }
    );

    let mut sum_overflow_frame =
        DirectRunFrame::skeleton(identity).expect("valid direct run frame should construct");
    sum_overflow_frame.lanes[0].transfer.surface_carry_m[0] = f64::MAX;
    sum_overflow_frame.lanes[0].transfer.surface_carry_m[1] = f64::MAX;
    assert_eq!(
        sum_overflow_frame
            .run_r3c_lane_transfer_span()
            .expect_err("overflowed surface carry should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "transfer.surface_carry_m"
        }
    );

    let mut received_overflow_frame =
        DirectRunFrame::skeleton(identity).expect("valid direct run frame should construct");
    received_overflow_frame.lanes[0].transfer.surface_carry_m[0] = f64::MAX;
    received_overflow_frame.lanes[1].upstream_area_ratio = 2.0;
    assert_eq!(
        received_overflow_frame
            .run_r3c_lane_transfer_span()
            .expect_err("overflowed received surface transfer should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "lane_transfer.received_surface_m"
        }
    );
}
