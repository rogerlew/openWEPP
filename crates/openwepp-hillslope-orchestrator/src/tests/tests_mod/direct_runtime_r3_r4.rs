use super::*;
use crate::DirectSnowMassTransitionLedgers;

#[test]
fn r3a_input_accounting_span_computes_mutates_downstream_and_shadow_projects() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R3A_INPUT_ACCOUNTING_SPAN,
        [
            DirectPhaseKind::Normalization,
            DirectPhaseKind::LateralTransfer
        ]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.125;
    day.forcing.effective_temperature_c = -2.5;
    day.transfer.surface_carry_m[0] = 0.25;
    day.transfer.surface_carry_m[1] = 0.125;
    day.transfer.lateral_carry_m[0] = 0.0625;
    day.transfer.upstream_flow_m = 0.03125;
    day.transfer.subsurface_input_m = 0.015_625;

    let report = day
        .run_r3a_input_accounting_span()
        .expect("valid direct phase span should execute");

    let expected_state = DirectInputAccountingState {
        precipitation_m: 0.125,
        surface_transfer_m: 0.375,
        lateral_transfer_m: 0.0625,
        upstream_flow_m: 0.03125,
        subsurface_input_m: 0.015_625,
        transfer_input_m: 0.484_375,
        total_accounted_input_m: 0.609_375,
    };
    let expected_operands = DirectDownstreamOperands::from(expected_state);
    let expected_shadow = DirectShadowProjection {
        lane_index: 0,
        day_index: 0,
        precipitation_m: 0.125,
        transfer_input_m: 0.484_375,
        total_accounted_input_m: 0.609_375,
    };

    assert_eq!(day.input_accounting, expected_state);
    assert_eq!(day.downstream_operands, expected_operands);
    assert_eq!(day.shadow_projection, Some(expected_shadow));
    assert_eq!(report.phase_count, DIRECT_R3A_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R3A_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.shadow_projection, expected_shadow);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 1);
    assert_eq!(
        audit.direct_phase_entries,
        DIRECT_R3A_PHASE_SPAN_COUNT as u64
    );
    assert_eq!(audit.direct_compute_operations, 1);
    assert_eq!(audit.direct_state_mutations, 1);
    assert_eq!(audit.downstream_operand_productions, 1);
    assert_eq!(audit.shadow_projections, 1);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r3a_input_accounting_span_rejects_invalid_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut nonfinite_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    nonfinite_day.forcing.precipitation_m = f64::NAN;
    assert_eq!(
        nonfinite_day
            .run_r3a_input_accounting_span()
            .expect_err("nonfinite precipitation should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "forcing.precipitation_m"
        }
    );

    let mut negative_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    negative_day.transfer.surface_carry_m[0] = -0.125;
    assert_eq!(
        negative_day
            .run_r3a_input_accounting_span()
            .expect_err("negative surface carry should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "transfer.surface_carry_m"
        }
    );

    let mut sum_overflow_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    sum_overflow_day.transfer.surface_carry_m[0] = f64::MAX;
    sum_overflow_day.transfer.surface_carry_m[1] = f64::MAX;
    assert_eq!(
        sum_overflow_day
            .run_r3a_input_accounting_span()
            .expect_err("overflowed surface carry sum should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "transfer.surface_carry_m"
        }
    );

    let mut derived_overflow_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    derived_overflow_day.transfer.surface_carry_m[0] = f64::MAX;
    derived_overflow_day.transfer.upstream_flow_m = f64::MAX;
    assert_eq!(
        derived_overflow_day
            .run_r3a_input_accounting_span()
            .expect_err("overflowed transfer total should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "input_accounting.transfer_input_m"
        }
    );
}

#[test]
fn r5b_normalization_phase_computes_mutates_downstream_and_shadow_projects() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R5B_NORMALIZATION_SPAN,
        [DirectPhaseKind::Normalization]
    );
    assert_eq!(
        &[
            DirectPhaseKind::Normalization,
            DirectPhaseKind::StorageBounds,
            DirectPhaseKind::DecompositionTransition,
        ],
        &DirectPhaseKind::ORDERED[0..3]
    );

    let mut day = r5b_normalization_fixture_day();
    let report = day
        .run_r5b_normalization_phase()
        .expect("valid R5B normalization phase should execute");
    let (expected_inputs, expected_state, expected_operands, expected_shadow, r3_shadow) =
        r5b_expected_normalization_outputs();

    assert_eq!(day.normalization_inputs, expected_inputs);
    assert_eq!(day.normalization, expected_state);
    assert_eq!(
        day.input_accounting,
        DirectInputAccountingState::from(expected_state)
    );
    assert_eq!(day.normalization_downstream_operands, expected_operands);
    assert_eq!(
        day.downstream_operands,
        DirectDownstreamOperands::from(day.input_accounting)
    );
    assert_eq!(day.normalization_shadow_projection, Some(expected_shadow));
    assert_eq!(day.shadow_projection, Some(r3_shadow));
    assert_eq!(
        day.storage_reconciliation_inputs
            .storage_initial_m
            .to_bits(),
        1.25_f64.to_bits()
    );
    assert_eq!(
        day.storage_reconciliation_inputs.precip_input_m.to_bits(),
        0.125_f64.to_bits()
    );
    assert_eq!(
        report.phase_count,
        DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT
    );
    assert_eq!(report.phase_entry_count, 1);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.normalization_shadow_projection, expected_shadow);

    assert_r5b_normalization_anti_aliases(expected_state, &day);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 1);
    assert_eq!(audit.direct_phase_entries, 1);
    assert_eq!(audit.direct_compute_operations, 1);
    assert_eq!(audit.direct_state_mutations, 1);
    assert_eq!(audit.downstream_operand_productions, 1);
    assert_eq!(audit.shadow_projections, 1);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

fn r5b_normalization_fixture_day() -> DirectDayFrame {
    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.125;
    day.forcing.effective_temperature_c = -2.5;
    day.water.soil_water_m = 1.25;
    day.transfer.surface_carry_m[0] = 0.25;
    day.transfer.surface_carry_m[1] = 0.125;
    day.transfer.lateral_carry_m[0] = 0.0625;
    day.transfer.upstream_flow_m = 0.03125;
    day.transfer.subsurface_input_m = 0.015_625;
    day.publication.infiltration_m = 0.007_812_5;
    day
}

fn r5b_expected_normalization_outputs() -> (
    DirectNormalizationInputs,
    DirectNormalizationState,
    DirectNormalizationDownstreamOperands,
    DirectNormalizationShadowProjection,
    DirectShadowProjection,
) {
    let expected_inputs = DirectNormalizationInputs {
        precipitation_m: 0.125,
        effective_temperature_c: -2.5,
        storage_initial_m: 1.25,
        surface_transfer_m: 0.375,
        lateral_transfer_m: 0.0625,
        upstream_flow_m: 0.03125,
        subsurface_input_m: 0.015_625,
    };
    let expected_state = DirectNormalizationState {
        precipitation_m: 0.125,
        effective_temperature_c: -2.5,
        storage_initial_m: 1.25,
        surface_transfer_m: 0.375,
        lateral_transfer_m: 0.0625,
        upstream_flow_m: 0.03125,
        subsurface_input_m: 0.015_625,
        transfer_input_m: 0.484_375,
        total_accounted_input_m: 0.609_375,
    };
    let expected_shadow = DirectNormalizationShadowProjection {
        lane_index: 0,
        day_index: 0,
        precipitation_m: 0.125,
        storage_initial_m: 1.25,
        transfer_input_m: 0.484_375,
        total_accounted_input_m: 0.609_375,
    };
    let r3_shadow = DirectShadowProjection {
        lane_index: 0,
        day_index: 0,
        precipitation_m: 0.125,
        transfer_input_m: 0.484_375,
        total_accounted_input_m: 0.609_375,
    };

    (
        expected_inputs,
        expected_state,
        DirectNormalizationDownstreamOperands::from(expected_state),
        expected_shadow,
        r3_shadow,
    )
}

fn assert_r5b_normalization_anti_aliases(
    expected_state: DirectNormalizationState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.precipitation_m.to_bits(),
        expected_state.storage_initial_m.to_bits()
    );
    assert_ne!(
        expected_state.precipitation_m.to_bits(),
        expected_state.transfer_input_m.to_bits()
    );
    assert_ne!(
        expected_state.precipitation_m.to_bits(),
        expected_state.total_accounted_input_m.to_bits()
    );
    assert_ne!(
        expected_state.storage_initial_m.to_bits(),
        expected_state.total_accounted_input_m.to_bits()
    );
    assert_ne!(
        expected_state.precipitation_m.to_bits(),
        day.publication.infiltration_m.to_bits()
    );
}

#[test]
fn r5b_storage_bounds_phase_consumes_normalization_and_shadow_projects() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R5B_STORAGE_BOUNDS_SPAN,
        [DirectPhaseKind::StorageBounds]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.25;
    day.water.soil_water_m = 1.0;
    day.transfer.surface_carry_m[0] = 0.5;
    day.transfer.lateral_carry_m[0] = 0.125;
    day.transfer.upstream_flow_m = 0.03125;
    day.transfer.subsurface_input_m = 0.015_625;
    day.storage_reconciliation_inputs.closure_tolerance_m = 1.0e-12;
    day.storage_reconciliation.closure_residual_m = 0.5;

    day.run_r5b_normalization_phase()
        .expect("R5B normalization should pass before storage bounds");
    let report = day
        .run_r5b_storage_bounds_phase()
        .expect("valid R5B storage bounds phase should execute");

    let expected_inputs = DirectStorageBoundsInputs {
        storage_initial_m: 1.0,
        total_accounted_input_m: 0.921_875,
        closure_tolerance_m: 1.0e-12,
    };
    let expected_state = DirectStorageBoundsState {
        storage_bounded_m: 1.0,
        total_accounted_input_m: 0.921_875,
        closure_tolerance_m: 1.0e-12,
    };
    let expected_operands = DirectStorageBoundsDownstreamOperands::from(expected_state);
    let expected_shadow = DirectStorageBoundsShadowProjection {
        lane_index: 0,
        day_index: 0,
        storage_bounded_m: 1.0,
        total_accounted_input_m: 0.921_875,
        closure_tolerance_m: 1.0e-12,
    };

    assert_eq!(day.storage_bounds_inputs, expected_inputs);
    assert_eq!(day.storage_bounds, expected_state);
    assert_eq!(day.storage_bounds_downstream_operands, expected_operands);
    assert_eq!(day.storage_bounds_shadow_projection, Some(expected_shadow));
    assert_eq!(day.water.soil_water_m.to_bits(), 1.0_f64.to_bits());
    assert_eq!(
        day.storage_reconciliation_inputs
            .storage_initial_m
            .to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        report.phase_count,
        DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT
    );
    assert_eq!(report.phase_entry_count, 1);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.storage_bounds_shadow_projection, expected_shadow);

    assert_r5b_storage_bounds_anti_aliases(expected_state, &day);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 2);
    assert_eq!(audit.direct_phase_entries, 2);
    assert_eq!(audit.direct_compute_operations, 2);
    assert_eq!(audit.direct_state_mutations, 2);
    assert_eq!(audit.downstream_operand_productions, 2);
    assert_eq!(audit.shadow_projections, 2);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

fn assert_r5b_storage_bounds_anti_aliases(
    expected_state: DirectStorageBoundsState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.storage_bounded_m.to_bits(),
        expected_state.total_accounted_input_m.to_bits()
    );
    assert_ne!(
        expected_state.storage_bounded_m.to_bits(),
        day.forcing.precipitation_m.to_bits()
    );
    assert_ne!(
        expected_state.storage_bounded_m.to_bits(),
        day.storage_reconciliation.closure_residual_m.to_bits()
    );
    assert_ne!(
        expected_state.total_accounted_input_m.to_bits(),
        day.storage_reconciliation.closure_residual_m.to_bits()
    );
}

#[test]
fn r5b_storage_bounds_phase_rejects_missing_normalization_and_invalid_storage() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");

    let mut missing_normalization_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    assert_eq!(
        missing_normalization_day
            .run_r5b_storage_bounds_phase()
            .expect_err("storage bounds should require R5B normalization"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R5B normalization phase"
        }
    );

    let mut negative_storage_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    negative_storage_day.water.soil_water_m = -0.125;
    assert_eq!(
        negative_storage_day
            .run_r5b_normalization_phase()
            .expect_err("normalization should reject negative storage"),
        DirectRuntimeError::NegativeDirectValue {
            field: "normalization.storage_initial_m"
        }
    );

    let mut invalid_bounded_storage_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    invalid_bounded_storage_day
        .run_r5b_normalization_phase()
        .expect("valid normalization should execute before injected invalid storage");
    invalid_bounded_storage_day
        .normalization_downstream_operands
        .storage_initial_m = -0.125;
    assert_eq!(
        invalid_bounded_storage_day
            .run_r5b_storage_bounds_phase()
            .expect_err("storage bounds should reject invalid normalized storage"),
        DirectRuntimeError::NegativeDirectValue {
            field: "storage_bounds.storage_initial_m"
        }
    );

    let mut invalid_tolerance_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    invalid_tolerance_day
        .run_r5b_normalization_phase()
        .expect("valid normalization should execute before invalid tolerance");
    invalid_tolerance_day
        .storage_reconciliation_inputs
        .closure_tolerance_m = f64::NAN;
    assert_eq!(
        invalid_tolerance_day
            .run_r5b_storage_bounds_phase()
            .expect_err("storage bounds should reject nonfinite closure tolerance"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "storage_bounds.closure_tolerance_m"
        }
    );
}

#[test]
fn r3b_water_ledger_span_consumes_r3a_state_and_shadow_projects() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R3B_WATER_LEDGER_SPAN,
        [
            DirectPhaseKind::RunoffReconciliation,
            DirectPhaseKind::StorageReconciliation,
            DirectPhaseKind::ClosureDiagnostics
        ]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.5;
    day.transfer.surface_carry_m[0] = 0.25;
    day.transfer.lateral_carry_m[0] = 0.125;
    day.transfer.upstream_flow_m = 0.0625;
    day.transfer.subsurface_input_m = 0.03125;
    day.water.soil_water_m = 0.25;
    day.water.infiltration_m = 0.125;
    day.water.runoff_m = 0.0625;
    day.water.evapotranspiration_m = 0.03125;
    day.water.drainage_m = 0.015_625;
    day.water.lateral_flow_m = 0.007_812_5;
    day.publication.infiltration_m = 0.015_625;
    day.publication.runoff_m = 0.03125;
    day.publication.evapotranspiration_m = 0.007_812_5;
    day.publication.drainage_m = 0.003_906_25;
    day.publication.lateral_flow_m = 0.001_953_125;

    day.run_r3a_input_accounting_span()
        .expect("R3A upstream span should pass before R3B");
    let report = day
        .run_r3b_water_ledger_span()
        .expect("valid R3B ledger span should execute");

    let expected_state = DirectWaterLedgerState {
        total_accounted_input_m: 0.96875,
        soil_water_m: 0.25,
        available_water_m: 1.21875,
        direct_flux_m: 0.242_187_5,
        publication_flux_m: 0.060_546_875,
        direct_publication_delta_m: 0.181_640_625,
        diagnostic_residual_m: 0.976_562_5,
    };
    let expected_operands = DirectLedgerDownstreamOperands::from(expected_state);
    let expected_shadow = DirectLedgerShadowProjection {
        lane_index: 0,
        day_index: 0,
        available_water_m: 1.21875,
        direct_flux_m: 0.242_187_5,
        publication_flux_m: 0.060_546_875,
        direct_publication_delta_m: 0.181_640_625,
        diagnostic_residual_m: 0.976_562_5,
    };

    assert_eq!(day.water_ledger, expected_state);
    assert_eq!(day.ledger_downstream_operands, expected_operands);
    assert_eq!(day.ledger_shadow_projection, Some(expected_shadow));
    assert_eq!(report.phase_count, DIRECT_R3B_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R3B_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.ledger_shadow_projection, expected_shadow);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 2);
    assert_eq!(
        audit.direct_phase_entries,
        (DIRECT_R3A_PHASE_SPAN_COUNT + DIRECT_R3B_PHASE_SPAN_COUNT) as u64
    );
    assert_eq!(audit.direct_compute_operations, 2);
    assert_eq!(audit.direct_state_mutations, 2);
    assert_eq!(audit.downstream_operand_productions, 2);
    assert_eq!(audit.shadow_projections, 2);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r3b_water_ledger_span_allows_signed_diagnostic_residual() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.125;
    day.water.runoff_m = 0.25;

    day.run_r3a_input_accounting_span()
        .expect("R3A upstream span should pass before R3B");
    let report = day
        .run_r3b_water_ledger_span()
        .expect("finite signed residual should be valid");

    let expected_state = DirectWaterLedgerState {
        total_accounted_input_m: 0.125,
        soil_water_m: 0.0,
        available_water_m: 0.125,
        direct_flux_m: 0.25,
        publication_flux_m: 0.0,
        direct_publication_delta_m: 0.25,
        diagnostic_residual_m: -0.125,
    };
    let expected_shadow = DirectLedgerShadowProjection {
        lane_index: 0,
        day_index: 0,
        available_water_m: 0.125,
        direct_flux_m: 0.25,
        publication_flux_m: 0.0,
        direct_publication_delta_m: 0.25,
        diagnostic_residual_m: -0.125,
    };

    assert_eq!(day.water_ledger, expected_state);
    assert_eq!(report.ledger_shadow_projection, expected_shadow);
}

#[test]
fn r3b_water_ledger_span_rejects_invalid_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");

    let mut nonfinite_input_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    nonfinite_input_day.input_accounting.total_accounted_input_m = f64::NAN;
    assert_eq!(
        nonfinite_input_day
            .run_r3b_water_ledger_span()
            .expect_err("nonfinite input accounting should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "input_accounting.total_accounted_input_m"
        }
    );

    let mut negative_publication_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    negative_publication_day.publication.runoff_m = -0.125;
    assert_eq!(
        negative_publication_day
            .run_r3b_water_ledger_span()
            .expect_err("negative publication runoff should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "publication.runoff_m"
        }
    );

    let mut direct_flux_overflow_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    direct_flux_overflow_day.water.infiltration_m = f64::MAX;
    direct_flux_overflow_day.water.runoff_m = f64::MAX;
    assert_eq!(
        direct_flux_overflow_day
            .run_r3b_water_ledger_span()
            .expect_err("overflowed direct flux should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "water_ledger.direct_flux_m"
        }
    );

    let mut available_water_overflow_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    available_water_overflow_day
        .input_accounting
        .total_accounted_input_m = f64::MAX;
    available_water_overflow_day.water.soil_water_m = f64::MAX;
    assert_eq!(
        available_water_overflow_day
            .run_r3b_water_ledger_span()
            .expect_err("overflowed available water should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "water_ledger.available_water_m"
        }
    );
}

#[test]
fn r4c_storage_input_producer_consumes_r3a_precipitation_and_direct_storage() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4C_STORAGE_INPUT_SPAN,
        [
            DirectPhaseKind::Normalization,
            DirectPhaseKind::StorageReconciliation
        ]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.25;
    day.water.soil_water_m = 1.0;
    day.transfer.surface_carry_m[0] = 0.5;
    day.transfer.lateral_carry_m[0] = 0.125;
    day.runoff_partition_inputs.liquid_input_m = 0.875;
    day.publication.infiltration_m = 0.5;
    day.water_ledger.soil_water_m = 0.75;

    day.run_r3a_input_accounting_span()
        .expect("R3A upstream span should pass before R4C");
    let report = day
        .run_r4c_storage_input_span()
        .expect("valid R4C storage input span should execute");

    let expected_state = DirectStorageInputState {
        storage_initial_m: 1.0,
        precip_input_m: 0.25,
    };
    let expected_operands = DirectStorageInputDownstreamOperands::from(expected_state);
    let expected_shadow = DirectStorageInputShadowProjection {
        lane_index: 0,
        day_index: 0,
        storage_initial_m: 1.0,
        precip_input_m: 0.25,
    };

    assert_eq!(day.storage_input, expected_state);
    assert_eq!(day.storage_input_downstream_operands, expected_operands);
    assert_eq!(day.storage_input_shadow_projection, Some(expected_shadow));
    assert_eq!(
        day.storage_reconciliation_inputs
            .storage_initial_m
            .to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        day.storage_reconciliation_inputs.precip_input_m.to_bits(),
        0.25_f64.to_bits()
    );
    assert_eq!(report.phase_count, DIRECT_R4C_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4C_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.storage_input_shadow_projection, expected_shadow);

    assert_r4c_storage_input_anti_aliases(expected_state, &day);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 2);
    assert_eq!(
        audit.direct_phase_entries,
        (DIRECT_R3A_PHASE_SPAN_COUNT + DIRECT_R4C_PHASE_SPAN_COUNT) as u64
    );
    assert_eq!(audit.direct_compute_operations, 2);
    assert_eq!(audit.direct_state_mutations, 2);
    assert_eq!(audit.downstream_operand_productions, 2);
    assert_eq!(audit.shadow_projections, 2);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

fn assert_r4c_storage_input_anti_aliases(
    expected_state: DirectStorageInputState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.precip_input_m.to_bits(),
        day.downstream_operands.transfer_input_m.to_bits()
    );
    assert_ne!(
        expected_state.precip_input_m.to_bits(),
        day.downstream_operands.total_accounted_input_m.to_bits()
    );
    assert_ne!(
        expected_state.precip_input_m.to_bits(),
        day.runoff_partition_inputs.liquid_input_m.to_bits()
    );
    assert_ne!(
        expected_state.storage_initial_m.to_bits(),
        day.publication.infiltration_m.to_bits()
    );
    assert_ne!(
        expected_state.storage_initial_m.to_bits(),
        day.water_ledger.soil_water_m.to_bits()
    );
}

#[test]
fn r4c_storage_input_producer_rejects_invalid_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");

    let mut missing_r3a_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    assert_eq!(
        missing_r3a_day
            .run_r4c_storage_input_span()
            .expect_err("R4C should require R3A direct upstream execution"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R3A input accounting"
        }
    );

    let mut negative_storage_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    negative_storage_day.water.soil_water_m = -0.125;
    assert_eq!(
        negative_storage_day
            .run_r3a_input_accounting_span()
            .expect_err("R3A should reject negative storage before R4C"),
        DirectRuntimeError::NegativeDirectValue {
            field: "water.soil_water_m"
        }
    );

    let mut nonfinite_precip_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    nonfinite_precip_day
        .run_r3a_input_accounting_span()
        .expect("R3A zero input should pass before mutation probe");
    nonfinite_precip_day.downstream_operands.precipitation_m = f64::NAN;
    assert_eq!(
        nonfinite_precip_day
            .run_r4c_storage_input_span()
            .expect_err("nonfinite direct precipitation should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "storage_input.precip_input_m"
        }
    );
}

#[test]
fn r4d_deep_seepage_producer_consumes_direct_handoff_and_updates_r4b_input() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4D_DEEP_SEEPAGE_SPAN,
        [
            DirectPhaseKind::PercolationDeepSeepage,
            DirectPhaseKind::StorageReconciliation
        ]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.deep_seepage_inputs = DirectDeepSeepageInputs {
        deep_seepage_handoff_m: 0.03125,
    };
    day.publication.drainage_m = 0.125;
    day.storage_reconciliation_inputs.snow_coupling_m = 0.25;
    day.storage_reconciliation_inputs.evapotranspiration_m = 0.0625;
    day.storage_reconciliation_inputs.subsurface_loss_m = 0.015_625;
    day.forcing.precipitation_m = 0.5;
    day.water.runoff_m = 0.34375;
    day.water_ledger.diagnostic_residual_m = 0.1875;
    day.storage_reconciliation.closure_residual_m = 0.09375;

    let report = day
        .run_r4d_deep_seepage_span()
        .expect("valid R4D deep-seepage span should execute");

    let expected_state = DirectDeepSeepageState {
        deep_seepage_m: 0.03125,
    };
    let expected_operands = DirectDeepSeepageDownstreamOperands::from(expected_state);
    let expected_shadow = DirectDeepSeepageShadowProjection {
        lane_index: 0,
        day_index: 0,
        deep_seepage_m: 0.03125,
    };

    assert_eq!(day.deep_seepage, expected_state);
    assert_eq!(day.deep_seepage_downstream_operands, expected_operands);
    assert_eq!(day.deep_seepage_shadow_projection, Some(expected_shadow));
    assert_eq!(
        day.storage_reconciliation_inputs.deep_seepage_m.to_bits(),
        0.03125_f64.to_bits()
    );
    assert_eq!(report.phase_count, DIRECT_R4D_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4D_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.deep_seepage_shadow_projection, expected_shadow);

    assert_r4d_deep_seepage_anti_aliases(expected_state, &day);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 1);
    assert_eq!(
        audit.direct_phase_entries,
        DIRECT_R4D_PHASE_SPAN_COUNT as u64
    );
    assert_eq!(audit.direct_compute_operations, 1);
    assert_eq!(audit.direct_state_mutations, 1);
    assert_eq!(audit.downstream_operand_productions, 1);
    assert_eq!(audit.shadow_projections, 1);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

fn assert_r4d_deep_seepage_anti_aliases(
    expected_state: DirectDeepSeepageState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.deep_seepage_m.to_bits(),
        day.publication.drainage_m.to_bits()
    );
    assert_ne!(
        expected_state.deep_seepage_m.to_bits(),
        day.storage_reconciliation_inputs
            .subsurface_loss_m
            .to_bits()
    );
    assert_ne!(
        expected_state.deep_seepage_m.to_bits(),
        day.storage_reconciliation_inputs
            .evapotranspiration_m
            .to_bits()
    );
    assert_ne!(
        expected_state.deep_seepage_m.to_bits(),
        day.storage_reconciliation_inputs.snow_coupling_m.to_bits()
    );
    assert_ne!(
        expected_state.deep_seepage_m.to_bits(),
        day.forcing.precipitation_m.to_bits()
    );
    assert_ne!(
        expected_state.deep_seepage_m.to_bits(),
        day.water.runoff_m.to_bits()
    );
    assert_ne!(
        expected_state.deep_seepage_m.to_bits(),
        day.water_ledger.diagnostic_residual_m.to_bits()
    );
    assert_ne!(
        expected_state.deep_seepage_m.to_bits(),
        day.storage_reconciliation.closure_residual_m.to_bits()
    );
}

#[test]
fn r4d_deep_seepage_producer_rejects_invalid_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");

    let mut negative_handoff_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    negative_handoff_day
        .deep_seepage_inputs
        .deep_seepage_handoff_m = -0.125;
    assert_eq!(
        negative_handoff_day
            .run_r4d_deep_seepage_span()
            .expect_err("negative deep-seepage handoff should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "deep_seepage.deep_seepage_handoff_m"
        }
    );

    let mut nonfinite_handoff_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    nonfinite_handoff_day
        .deep_seepage_inputs
        .deep_seepage_handoff_m = f64::NAN;
    assert_eq!(
        nonfinite_handoff_day
            .run_r4d_deep_seepage_span()
            .expect_err("nonfinite deep-seepage handoff should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "deep_seepage.deep_seepage_handoff_m"
        }
    );
}

#[test]
fn r4e_subsurface_loss_producer_consumes_direct_handoff_and_updates_r4b_input() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4E_SUBSURFACE_LOSS_SPAN,
        [
            DirectPhaseKind::Drainage,
            DirectPhaseKind::LateralTransfer,
            DirectPhaseKind::StorageReconciliation
        ]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.subsurface_loss_inputs = DirectSubsurfaceLossInputs {
        subsurface_loss_handoff_m: 0.015_625,
    };
    day.water.lateral_flow_m = 0.125;
    day.water.drainage_m = 0.0625;
    day.deep_seepage_inputs.deep_seepage_handoff_m = 0.03125;
    day.storage_reconciliation_inputs.evapotranspiration_m = 0.046_875;
    day.storage_reconciliation_inputs.snow_coupling_m = 0.09375;
    day.publication.lateral_flow_m = 0.1875;
    day.publication.drainage_m = 0.21875;
    day.storage_reconciliation.closure_residual_m = 0.25;

    let report = day
        .run_r4e_subsurface_loss_span()
        .expect("valid R4E subsurface-loss span should execute");

    let expected_state = DirectSubsurfaceLossState {
        subsurface_loss_m: 0.015_625,
    };
    let expected_operands = DirectSubsurfaceLossDownstreamOperands::from(expected_state);
    let expected_shadow = DirectSubsurfaceLossShadowProjection {
        lane_index: 0,
        day_index: 0,
        subsurface_loss_m: 0.015_625,
    };

    assert_eq!(day.subsurface_loss, expected_state);
    assert_eq!(day.subsurface_loss_downstream_operands, expected_operands);
    assert_eq!(day.subsurface_loss_shadow_projection, Some(expected_shadow));
    assert_eq!(
        day.storage_reconciliation_inputs
            .subsurface_loss_m
            .to_bits(),
        0.015_625_f64.to_bits()
    );
    assert_eq!(report.phase_count, DIRECT_R4E_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4E_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.subsurface_loss_shadow_projection, expected_shadow);

    assert_r4e_subsurface_loss_anti_aliases(expected_state, &day);
}

fn assert_r4e_subsurface_loss_anti_aliases(
    expected_state: DirectSubsurfaceLossState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.subsurface_loss_m.to_bits(),
        day.water.lateral_flow_m.to_bits()
    );
    assert_ne!(
        expected_state.subsurface_loss_m.to_bits(),
        day.water.drainage_m.to_bits()
    );
    assert_ne!(
        expected_state.subsurface_loss_m.to_bits(),
        day.deep_seepage_inputs.deep_seepage_handoff_m.to_bits()
    );
    assert_ne!(
        expected_state.subsurface_loss_m.to_bits(),
        day.storage_reconciliation_inputs
            .evapotranspiration_m
            .to_bits()
    );
    assert_ne!(
        expected_state.subsurface_loss_m.to_bits(),
        day.storage_reconciliation_inputs.snow_coupling_m.to_bits()
    );
    assert_ne!(
        expected_state.subsurface_loss_m.to_bits(),
        day.publication.lateral_flow_m.to_bits()
    );
    assert_ne!(
        expected_state.subsurface_loss_m.to_bits(),
        day.publication.drainage_m.to_bits()
    );
    assert_ne!(
        expected_state.subsurface_loss_m.to_bits(),
        day.storage_reconciliation.closure_residual_m.to_bits()
    );
}

#[test]
fn r4f_evapotranspiration_producer_consumes_direct_handoff_and_updates_r4b_input() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4F_EVAPOTRANSPIRATION_SPAN,
        [
            DirectPhaseKind::Evapotranspiration,
            DirectPhaseKind::StorageReconciliation
        ]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.evapotranspiration_inputs = DirectEvapotranspirationInputs {
        evapotranspiration_handoff_m: 0.0625,
    };
    day.publication.evapotranspiration_m = 0.125;
    day.water.drainage_m = 0.03125;
    day.water.runoff_m = 0.34375;
    day.forcing.precipitation_m = 0.25;
    day.storage_reconciliation_inputs.subsurface_loss_m = 0.015_625;
    day.storage_reconciliation_inputs.snow_coupling_m = 0.09375;
    day.storage_reconciliation.closure_residual_m = 0.1875;

    let report = day
        .run_r4f_evapotranspiration_span()
        .expect("valid R4F evapotranspiration span should execute");

    let expected_state = DirectEvapotranspirationState {
        evapotranspiration_m: 0.0625,
    };
    let expected_operands = DirectEvapotranspirationDownstreamOperands::from(expected_state);
    let expected_shadow = DirectEvapotranspirationShadowProjection {
        lane_index: 0,
        day_index: 0,
        evapotranspiration_m: 0.0625,
    };

    assert_eq!(day.evapotranspiration, expected_state);
    assert_eq!(
        day.water.evapotranspiration_m.to_bits(),
        0.0625_f64.to_bits()
    );
    assert_eq!(
        day.evapotranspiration_downstream_operands,
        expected_operands
    );
    assert_eq!(
        day.evapotranspiration_shadow_projection,
        Some(expected_shadow)
    );
    assert_eq!(
        day.storage_reconciliation_inputs
            .evapotranspiration_m
            .to_bits(),
        0.0625_f64.to_bits()
    );
    assert_eq!(report.phase_count, DIRECT_R4F_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4F_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.evapotranspiration_shadow_projection, expected_shadow);

    assert_r4f_evapotranspiration_anti_aliases(expected_state, &day);
}

fn assert_r4f_evapotranspiration_anti_aliases(
    expected_state: DirectEvapotranspirationState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.evapotranspiration_m.to_bits(),
        day.publication.evapotranspiration_m.to_bits()
    );
    assert_ne!(
        expected_state.evapotranspiration_m.to_bits(),
        day.water.drainage_m.to_bits()
    );
    assert_ne!(
        expected_state.evapotranspiration_m.to_bits(),
        day.water.runoff_m.to_bits()
    );
    assert_ne!(
        expected_state.evapotranspiration_m.to_bits(),
        day.forcing.precipitation_m.to_bits()
    );
    assert_ne!(
        expected_state.evapotranspiration_m.to_bits(),
        day.storage_reconciliation_inputs
            .subsurface_loss_m
            .to_bits()
    );
    assert_ne!(
        expected_state.evapotranspiration_m.to_bits(),
        day.storage_reconciliation_inputs.snow_coupling_m.to_bits()
    );
    assert_ne!(
        expected_state.evapotranspiration_m.to_bits(),
        day.storage_reconciliation.closure_residual_m.to_bits()
    );
}

#[test]
fn r4g_snow_coupling_producer_consumes_signed_handoff_and_updates_r4b_input() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4G_SNOW_COUPLING_SPAN,
        [
            DirectPhaseKind::Normalization,
            DirectPhaseKind::StorageReconciliation
        ]
    );

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: -0.09375,
        ..DirectSnowCouplingInputs::zero()
    };
    day.forcing.precipitation_m = 0.25;
    day.runoff_downstream_operands.q_runoff_m = 0.34375;
    day.evapotranspiration_inputs.evapotranspiration_handoff_m = 0.0625;
    day.deep_seepage_inputs.deep_seepage_handoff_m = 0.03125;
    day.subsurface_loss_inputs.subsurface_loss_handoff_m = 0.015_625;
    day.publication.runoff_m = 0.125;
    day.publication.evapotranspiration_m = 0.1875;
    day.storage_reconciliation.closure_residual_m = 0.21875;

    let report = day
        .run_r4g_snow_coupling_span()
        .expect("valid signed R4G snow-coupling span should execute");

    let expected_state = DirectSnowCouplingState {
        snow_coupling_m: -0.09375,
        ..DirectSnowCouplingState::zero()
    };
    let expected_operands = DirectSnowCouplingDownstreamOperands::from_state_and_hourly_routed_melt(
        expected_state.clone(),
        [0.0; 24],
    );
    let expected_shadow = DirectSnowCouplingShadowProjection {
        lane_index: 0,
        day_index: 0,
        snow_coupling_m: -0.09375,
        active_snow_coupling: false,
        mass_transition_ledgers: DirectSnowMassTransitionLedgers::zero(),
        sublimation_m: 0.0,
        post_winter_rain_m: 0.0,
        runtime_swe_after_m: 0.0,
        runtime_depth_after_m: 0.0,
        runtime_density_after_kg_m3: 0.0,
        runtime_settle_day_count_after: 0.0,
        coe_boundary_depth_after_m: 0.0,
        coe_boundary_density_after_kg_m3: 0.0,
        coe_boundary_settle_day_count_after: 0.0,
        snow_albedo_state_after: None,
    };

    assert_eq!(day.snow_coupling, expected_state);
    assert_eq!(day.snow_coupling_downstream_operands, expected_operands);
    assert_eq!(
        day.snow_coupling_shadow_projection,
        Some(Box::new(expected_shadow.clone()))
    );
    assert_eq!(
        day.storage_reconciliation_inputs.snow_coupling_m.to_bits(),
        (-0.09375_f64).to_bits()
    );
    assert_eq!(report.phase_count, DIRECT_R4G_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4G_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.snow_coupling_shadow_projection, expected_shadow);

    assert_r4g_snow_coupling_anti_aliases(&expected_state, &day);
}

fn assert_r4g_snow_coupling_anti_aliases(
    expected_state: &DirectSnowCouplingState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.snow_coupling_m.to_bits(),
        day.forcing.precipitation_m.to_bits()
    );
    assert_ne!(
        expected_state.snow_coupling_m.to_bits(),
        day.runoff_downstream_operands.q_runoff_m.to_bits()
    );
    assert_ne!(
        expected_state.snow_coupling_m.to_bits(),
        day.evapotranspiration_inputs
            .evapotranspiration_handoff_m
            .to_bits()
    );
    assert_ne!(
        expected_state.snow_coupling_m.to_bits(),
        day.deep_seepage_inputs.deep_seepage_handoff_m.to_bits()
    );
    assert_ne!(
        expected_state.snow_coupling_m.to_bits(),
        day.subsurface_loss_inputs
            .subsurface_loss_handoff_m
            .to_bits()
    );
    assert_ne!(
        expected_state.snow_coupling_m.to_bits(),
        day.publication.runoff_m.to_bits()
    );
    assert_ne!(
        expected_state.snow_coupling_m.to_bits(),
        day.publication.evapotranspiration_m.to_bits()
    );
    assert_ne!(
        expected_state.snow_coupling_m.to_bits(),
        day.storage_reconciliation.closure_residual_m.to_bits()
    );
}

#[test]
fn r4eh_storage_budget_handoff_producers_reject_invalid_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");

    let mut negative_qd_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    negative_qd_day
        .subsurface_loss_inputs
        .subsurface_loss_handoff_m = -0.125;
    assert_eq!(
        negative_qd_day
            .run_r4e_subsurface_loss_span()
            .expect_err("negative subsurface-loss handoff should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "subsurface_loss.subsurface_loss_handoff_m"
        }
    );

    let mut nonfinite_qd_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    nonfinite_qd_day
        .subsurface_loss_inputs
        .subsurface_loss_handoff_m = f64::NAN;
    assert_eq!(
        nonfinite_qd_day
            .run_r4e_subsurface_loss_span()
            .expect_err("nonfinite subsurface-loss handoff should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "subsurface_loss.subsurface_loss_handoff_m"
        }
    );

    let mut negative_et_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    negative_et_day
        .evapotranspiration_inputs
        .evapotranspiration_handoff_m = -0.125;
    assert_eq!(
        negative_et_day
            .run_r4f_evapotranspiration_span()
            .expect_err("negative ET handoff should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "evapotranspiration.evapotranspiration_handoff_m"
        }
    );

    let mut nonfinite_et_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    nonfinite_et_day
        .evapotranspiration_inputs
        .evapotranspiration_handoff_m = f64::INFINITY;
    assert_eq!(
        nonfinite_et_day
            .run_r4f_evapotranspiration_span()
            .expect_err("nonfinite ET handoff should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "evapotranspiration.evapotranspiration_handoff_m"
        }
    );

    let mut nonfinite_snow_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    nonfinite_snow_day
        .snow_coupling_inputs
        .snow_coupling_handoff_m = f64::NAN;
    assert_eq!(
        nonfinite_snow_day
            .run_r4g_snow_coupling_span()
            .expect_err("nonfinite signed snow coupling should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "snow_coupling.snow_coupling_handoff_m"
        }
    );
}
