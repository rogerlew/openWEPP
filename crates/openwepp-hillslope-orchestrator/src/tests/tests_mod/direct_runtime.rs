use super::*;
use crate::{
    DIRECT_PHASE_COUNT, DIRECT_R3A_INPUT_ACCOUNTING_SPAN, DIRECT_R3A_PHASE_SPAN_COUNT,
    DIRECT_R3B_PHASE_SPAN_COUNT, DIRECT_R3B_WATER_LEDGER_SPAN, DirectDayFrame,
    DirectDownstreamOperands, DirectExecutorMode, DirectFrameExecutor, DirectInputAccountingState,
    DirectLedgerDownstreamOperands, DirectLedgerShadowProjection, DirectPhaseKind, DirectRunFrame,
    DirectRunIdentity, DirectRuntimeError, DirectShadowProjection, DirectWaterLedgerState,
    reset_direct_runtime_audit_counters,
};
use std::sync::{Mutex, OnceLock};

#[test]
fn r2a_direct_skeleton_runs_noop_and_records_only_direct_audit_counters() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(7, 2637, 2, 10)
        .expect("valid direct skeleton identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct skeleton frame should construct");
    let executor = DirectFrameExecutor::new(DirectExecutorMode::Noop);

    let report = executor
        .run_skeleton(&mut frame)
        .expect("direct skeleton no-op execution should pass");

    assert_eq!(report.mode, DirectExecutorMode::Noop);
    assert_eq!(report.lane_count, 2);
    assert_eq!(report.day_count, 10);
    assert_eq!(report.planned_phase_count, DIRECT_PHASE_COUNT);
    assert_eq!(report.phase_view_count, (2 * DIRECT_PHASE_COUNT) as u64);
    assert_eq!(report.phase_span_run_count, 4);
    assert_eq!(
        report.direct_phase_entry_count,
        (2 * (DIRECT_R3A_PHASE_SPAN_COUNT + DIRECT_R3B_PHASE_SPAN_COUNT)) as u64
    );
    assert_eq!(report.direct_compute_count, 4);
    assert_eq!(report.state_mutation_count, 4);
    assert_eq!(report.downstream_operand_count, 4);
    assert_eq!(report.shadow_projection_count, 4);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.run_frame_constructions, 1);
    assert_eq!(audit.executor_constructions, 1);
    assert_eq!(audit.skeleton_runs, 1);
    assert_eq!(audit.day_frame_constructions, 2);
    assert_eq!(
        audit.phase_view_constructions,
        (2 * DIRECT_PHASE_COUNT) as u64
    );
    assert_eq!(audit.phase_span_runs, 4);
    assert_eq!(
        audit.direct_phase_entries,
        (2 * (DIRECT_R3A_PHASE_SPAN_COUNT + DIRECT_R3B_PHASE_SPAN_COUNT)) as u64
    );
    assert_eq!(audit.direct_compute_operations, 4);
    assert_eq!(audit.direct_state_mutations, 4);
    assert_eq!(audit.downstream_operand_productions, 4);
    assert_eq!(audit.shadow_projections, 4);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r2a_direct_skeleton_fails_closed_on_invalid_identity() {
    assert_eq!(
        DirectRunIdentity::new(7, 2637, 0, 1),
        Err(DirectRuntimeError::InvalidLaneCount { lane_count: 0 })
    );
    assert_eq!(
        DirectRunIdentity::new(7, 2637, 1, 0),
        Err(DirectRuntimeError::InvalidDayCount { day_count: 0 })
    );
}

#[test]
fn r2a_direct_runtime_source_excludes_compatibility_storage_tokens() {
    let direct_source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/direct_runtime.rs"
    ))
    .expect("direct runtime source should be readable");

    for forbidden in [
        "SymbolRegistry",
        "BoundarySymbol",
        "BoundaryValue",
        "Option<BoundaryValue>",
        "HillslopeWritebackSurface",
        "KernelWritebackPayload",
        "IndexedWritebackSurface",
        "HotSymbolTables",
        "HillslopeKernelRequest",
        "execute_with_kernel",
        "state_value_for_symbol",
        "flux_value_for_symbol",
        "dirty_state_ids",
        "dirty_flux_ids",
    ] {
        assert!(
            !direct_source.contains(forbidden),
            "direct runtime source must not contain forbidden token {forbidden}"
        );
    }
}

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

fn direct_runtime_test_lock() -> &'static Mutex<()> {
    static DIRECT_RUNTIME_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    DIRECT_RUNTIME_TEST_LOCK.get_or_init(|| Mutex::new(()))
}
