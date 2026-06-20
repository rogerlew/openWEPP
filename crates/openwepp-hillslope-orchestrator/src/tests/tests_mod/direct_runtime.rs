use super::{direct_runtime_test_lock, *};
use crate::{
    DIRECT_PHASE_COUNT, DIRECT_R3A_INPUT_ACCOUNTING_SPAN, DIRECT_R3A_PHASE_SPAN_COUNT,
    DIRECT_R3B_PHASE_SPAN_COUNT, DIRECT_R3B_WATER_LEDGER_SPAN, DIRECT_R3C_LANE_TRANSFER_SPAN,
    DIRECT_R3C_PHASE_SPAN_COUNT, DIRECT_R4A_PHASE_SPAN_COUNT, DIRECT_R4B_PHASE_SPAN_COUNT,
    DIRECT_R4B_STORAGE_RECONCILIATION_SPAN, DIRECT_R4C_PHASE_SPAN_COUNT,
    DIRECT_R4C_STORAGE_INPUT_SPAN, DIRECT_R4D_DEEP_SEEPAGE_SPAN, DIRECT_R4D_PHASE_SPAN_COUNT,
    DIRECT_R4E_PHASE_SPAN_COUNT, DIRECT_R4E_SUBSURFACE_LOSS_SPAN,
    DIRECT_R4F_EVAPOTRANSPIRATION_SPAN, DIRECT_R4F_PHASE_SPAN_COUNT, DIRECT_R4G_PHASE_SPAN_COUNT,
    DIRECT_R4G_SNOW_COUPLING_SPAN, DIRECT_R4I_PHASE_SPAN_COUNT, DIRECT_R4J_PHASE_SPAN_COUNT,
    DIRECT_R4K_PHASE_SPAN_COUNT, DIRECT_R4L_PHASE_SPAN_COUNT, DIRECT_R4M_PHASE_SPAN_COUNT,
    DIRECT_R4N_PHASE_SPAN_COUNT, DIRECT_R4O_PHASE_SPAN_COUNT, DIRECT_R4PQZ_PHASE_SPAN_COUNT,
    DirectDayFrame, DirectDeepSeepageDownstreamOperands, DirectDeepSeepageInputs,
    DirectDeepSeepageShadowProjection, DirectDeepSeepageState, DirectDownstreamOperands,
    DirectEvapotranspirationComputeInputs, DirectEvapotranspirationDownstreamOperands,
    DirectEvapotranspirationInputs, DirectEvapotranspirationShadowProjection,
    DirectEvapotranspirationState, DirectExecutorMode, DirectFrameExecutor,
    DirectInfiltrationDepressionInputs, DirectInputAccountingState, DirectLaneTransferLedger,
    DirectLedgerDownstreamOperands, DirectLedgerShadowProjection, DirectLiquidInputInputs,
    DirectPercolationInputs, DirectPhaseKind, DirectRunFrame, DirectRunIdentity,
    DirectRunTransferDownstreamOperands, DirectRunTransferShadowProjection,
    DirectRunoffPartitionInputs, DirectRunonCarryInputs, DirectRuntimeError,
    DirectSaturationAddbackInputs, DirectShadowProjection, DirectSnowCouplingDownstreamOperands,
    DirectSnowCouplingInputs, DirectSnowCouplingShadowProjection, DirectSnowCouplingState,
    DirectStorageDownstreamOperands, DirectStorageInputDownstreamOperands,
    DirectStorageInputShadowProjection, DirectStorageInputState, DirectStorageReconciliationInputs,
    DirectStorageReconciliationState, DirectStorageShadowProjection, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
    DirectSubsurfaceLossDownstreamOperands, DirectSubsurfaceLossInputs,
    DirectSubsurfaceLossShadowProjection, DirectSubsurfaceLossState, DirectWaterLedgerState,
    reset_direct_runtime_audit_counters,
};

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
    assert_eq!(report.phase_span_run_count, 31);
    assert_eq!(
        report.direct_phase_entry_count,
        (DIRECT_R3C_PHASE_SPAN_COUNT
            + 2 * (DIRECT_R3A_PHASE_SPAN_COUNT
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
                + DIRECT_R4B_PHASE_SPAN_COUNT
                + DIRECT_R4PQZ_PHASE_SPAN_COUNT
                + DIRECT_R3B_PHASE_SPAN_COUNT)) as u64
    );
    assert_eq!(report.direct_compute_count, 31);
    assert_eq!(report.state_mutation_count, 31);
    assert_eq!(report.downstream_operand_count, 31);
    assert_eq!(report.shadow_projection_count, 31);
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
    assert_eq!(audit.phase_span_runs, 31);
    assert_eq!(
        audit.direct_phase_entries,
        (DIRECT_R3C_PHASE_SPAN_COUNT
            + 2 * (DIRECT_R3A_PHASE_SPAN_COUNT
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
                + DIRECT_R4B_PHASE_SPAN_COUNT
                + DIRECT_R4PQZ_PHASE_SPAN_COUNT
                + DIRECT_R3B_PHASE_SPAN_COUNT)) as u64
    );
    assert_eq!(audit.direct_compute_operations, 31);
    assert_eq!(audit.direct_state_mutations, 31);
    assert_eq!(audit.downstream_operand_productions, 31);
    assert_eq!(audit.shadow_projections, 31);
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
    let direct_sources = [
        (
            "direct_runtime.rs",
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/direct_runtime.rs"
            ))
            .expect("direct runtime source should be readable"),
        ),
        (
            "direct_runtime/storage.rs",
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/direct_runtime/storage.rs"
            ))
            .expect("direct runtime storage source should be readable"),
        ),
        (
            "direct_runtime/runoff.rs",
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/direct_runtime/runoff.rs"
            ))
            .expect("direct runtime runoff source should be readable"),
        ),
        (
            "direct_runtime/subsurface.rs",
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/direct_runtime/subsurface.rs"
            ))
            .expect("direct runtime subsurface source should be readable"),
        ),
        (
            "direct_runtime/evapotranspiration.rs",
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/direct_runtime/evapotranspiration.rs"
            ))
            .expect("direct runtime evapotranspiration source should be readable"),
        ),
        (
            "direct_runtime/projection.rs",
            std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/direct_runtime/projection.rs"
            ))
            .expect("direct runtime projection source should be readable"),
        ),
    ];

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
        for (source_path, direct_source) in &direct_sources {
            assert!(
                !direct_source.contains(forbidden),
                "{source_path} must not contain forbidden token {forbidden}"
            );
        }
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
    };
    let expected_operands = DirectSnowCouplingDownstreamOperands::from(expected_state);
    let expected_shadow = DirectSnowCouplingShadowProjection {
        lane_index: 0,
        day_index: 0,
        snow_coupling_m: -0.09375,
    };

    assert_eq!(day.snow_coupling, expected_state);
    assert_eq!(day.snow_coupling_downstream_operands, expected_operands);
    assert_eq!(day.snow_coupling_shadow_projection, Some(expected_shadow));
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

    assert_r4g_snow_coupling_anti_aliases(expected_state, &day);
}

fn assert_r4g_snow_coupling_anti_aliases(
    expected_state: DirectSnowCouplingState,
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
    assert_eq!(day.water.soil_water_m.to_bits(), 0.921_875_f64.to_bits());
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
            snow_coupling_m: 0.125
        }
    );
}

fn r4b_expected_storage_state() -> DirectStorageReconciliationState {
    DirectStorageReconciliationState {
        storage_initial_m: 1.0,
        precip_input_m: 0.25,
        snow_coupling_m: 0.125,
        q_runoff_m: 0.34375,
        evapotranspiration_m: 0.0625,
        deep_seepage_m: 0.03125,
        subsurface_loss_m: 0.015_625,
        closure_tolerance_m: 0.0,
        storage_reconciled_m: 0.921_875,
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
        q_runoff_m: 0.34375,
        evapotranspiration_m: 0.0625,
        deep_seepage_m: 0.03125,
        subsurface_loss_m: 0.015_625,
        storage_reconciled_m: 0.921_875,
        closure_residual_m: 0.0,
    }
}

fn assert_r4b_storage_anti_aliases(
    expected_state: DirectStorageReconciliationState,
    day: &DirectDayFrame,
) {
    let omitted_s_m = expected_state.storage_initial_m + expected_state.precip_input_m
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
        - expected_state.q_runoff_m;
    assert_ne!(
        expected_state.storage_reconciled_m.to_bits(),
        omitted_losses_m.to_bits()
    );
    let publication_q_alias_m = expected_state.storage_initial_m
        + expected_state.precip_input_m
        + expected_state.snow_coupling_m
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
        stage_state: None,
        pmet: None,
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
    };
    day.saturation_addback_inputs = DirectSaturationAddbackInputs {
        surface_saturation_runoff_handoff_m: 0.03125,
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
    };
    day.storage_reconciliation_inputs = DirectStorageReconciliationInputs {
        storage_initial_m: 9.0,
        precip_input_m: 9.0,
        snow_coupling_m: 9.0,
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
