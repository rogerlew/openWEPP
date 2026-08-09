use super::direct_runtime_test_lock;
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
    DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT, DIRECT_R5B_NORMALIZATION_SPAN,
    DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT, DIRECT_R5B_STORAGE_BOUNDS_SPAN,
    DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT, DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT,
    DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT, DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT,
    DirectDayConstructorInputs, DirectDayForcing, DirectDayFrame,
    DirectDeepSeepageDownstreamOperands, DirectDeepSeepageInputs,
    DirectDeepSeepageShadowProjection, DirectDeepSeepageState, DirectDownstreamOperands,
    DirectErod13Inputs, DirectErosionInputs, DirectEvapotranspirationComputeInputs,
    DirectEvapotranspirationDownstreamOperands, DirectEvapotranspirationInputs,
    DirectEvapotranspirationPmetInputs, DirectEvapotranspirationShadowProjection,
    DirectEvapotranspirationState, DirectExecutorMode, DirectFrameExecutor,
    DirectFrostLayerCarryProjection, DirectGroundwaterAuthority, DirectGroundwaterRunState,
    DirectHydrologyProjectionInputs, DirectInfiltrationDepressionInputs,
    DirectInputAccountingState, DirectLaneConstructorInputs, DirectLaneTransferLedger,
    DirectLedgerDownstreamOperands, DirectLedgerShadowProjection, DirectLiquidInputInputs,
    DirectNormalizationDownstreamOperands, DirectNormalizationInputs,
    DirectNormalizationShadowProjection, DirectNormalizationState, DirectPercolationInputs,
    DirectPhaseKind, DirectPhaseLifecycleStatus, DirectPublicationCalendarDay,
    DirectPublicationDayInput, DirectPublicationRunMetadata, DirectRunConstructorInputs,
    DirectRunFrame, DirectRunIdentity, DirectRunTransferDownstreamOperands,
    DirectRunTransferShadowProjection, DirectRunoffPartitionInputs, DirectRunonCarryInputs,
    DirectRuntimeError, DirectSaturationAddbackInputs, DirectShadowProjection,
    DirectSnowCouplingDownstreamOperands, DirectSnowCouplingInputs,
    DirectSnowCouplingShadowProjection, DirectSnowCouplingState,
    DirectStorageBoundsDownstreamOperands, DirectStorageBoundsInputs,
    DirectStorageBoundsShadowProjection, DirectStorageBoundsState, DirectStorageDownstreamOperands,
    DirectStorageInputDownstreamOperands, DirectStorageInputInputs,
    DirectStorageInputShadowProjection, DirectStorageInputState, DirectStorageReconciliationInputs,
    DirectStorageReconciliationState, DirectStorageShadowProjection, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
    DirectSubsurfaceLossDownstreamOperands, DirectSubsurfaceLossInputs,
    DirectSubsurfaceLossShadowProjection, DirectSubsurfaceLossState, DirectWaterLedgerState,
    DirectWave1ContinuityInputs, DirectWave1OperandSeed, DirectWb14HyetographInterval,
    direct_runtime_audit_snapshot, record_direct_runtime_compatibility_edge_invocation,
    record_direct_runtime_ksatadj_effective_conductivity_evaluation,
    reset_direct_runtime_audit_counters,
};

#[test]
fn cqr_direct_runtime_audit_api_records_specialized_events() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    record_direct_runtime_compatibility_edge_invocation();
    record_direct_runtime_ksatadj_effective_conductivity_evaluation();

    let audit = direct_runtime_audit_snapshot();
    assert_eq!(audit.compatibility_edge_invocations, 1);
    assert_eq!(audit.ksatadj_effective_conductivity_evaluations, 1);
    reset_direct_runtime_audit_counters();
}

fn r5c_day_span_run_count() -> u64 {
    22
}

fn r5c_day_phase_entry_count() -> u64 {
    (DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT
        + DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT
        + DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT
        + DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT
        + DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT
        + DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT
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
        + 3
        + DIRECT_R4B_PHASE_SPAN_COUNT
        + DIRECT_R4PQZ_PHASE_SPAN_COUNT
        + 3
        + DIRECT_R3B_PHASE_SPAN_COUNT) as u64
}

fn r5c_expected_phase_status(phase: DirectPhaseKind) -> DirectPhaseLifecycleStatus {
    match phase {
        DirectPhaseKind::Normalization
        | DirectPhaseKind::StorageBounds
        | DirectPhaseKind::DecompositionTransition
        | DirectPhaseKind::ResiduePartitionTransition
        | DirectPhaseKind::AnnualGrowthTransition
        | DirectPhaseKind::PerennialGrowthTransition
        | DirectPhaseKind::PercolationDeepSeepage
        | DirectPhaseKind::Evapotranspiration
        | DirectPhaseKind::Drainage
        | DirectPhaseKind::LateralTransfer
        | DirectPhaseKind::PlantRootUptake
        | DirectPhaseKind::RunoffReconciliation
        | DirectPhaseKind::StorageReconciliation
        | DirectPhaseKind::ClosureDiagnostics => DirectPhaseLifecycleStatus::Executed,
    }
}

fn assert_r5c_phase_status_counts(counts: &[crate::DirectPhaseStatusCount], expected_count: u64) {
    assert_eq!(counts.len(), DIRECT_PHASE_COUNT);
    for (index, count) in counts.iter().enumerate() {
        let expected_phase = DirectPhaseKind::ORDERED[index];
        assert_eq!(count.phase, expected_phase);
        assert_eq!(count.status, r5c_expected_phase_status(expected_phase));
        assert_eq!(count.count, expected_count);
    }
}

#[test]
fn r5a_direct_skeleton_runs_all_days_and_lanes_with_lifecycle_counters() {
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
    let expected_day_frames = (identity.lane_count * identity.day_count) as u64;
    assert_eq!(
        report.phase_view_count,
        expected_day_frames * DIRECT_PHASE_COUNT as u64
    );
    assert_eq!(
        report.canonical_phase_entry_count,
        expected_day_frames * DIRECT_PHASE_COUNT as u64
    );
    assert_r5c_phase_status_counts(&report.phase_status_counts, expected_day_frames);
    let expected_dynamic_transfers =
        identity.day_count as u64 * identity.lane_count.saturating_sub(1) as u64;
    assert_eq!(
        report.phase_span_run_count,
        1 + expected_day_frames * r5c_day_span_run_count()
    );
    assert_eq!(
        report.direct_phase_entry_count,
        DIRECT_R3C_PHASE_SPAN_COUNT as u64 + expected_day_frames * r5c_day_phase_entry_count()
    );
    assert_eq!(
        report.direct_compute_count,
        1 + expected_day_frames * r5c_day_span_run_count()
    );
    assert_eq!(
        report.state_mutation_count,
        1 + expected_day_frames * r5c_day_span_run_count() + expected_dynamic_transfers
    );
    assert_eq!(
        report.downstream_operand_count,
        1 + expected_day_frames * r5c_day_span_run_count() + expected_dynamic_transfers
    );
    assert_eq!(
        report.shadow_projection_count,
        1 + expected_day_frames * r5c_day_span_run_count()
    );
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.day_frame_commit_count, expected_day_frames);
    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.run_frame_constructions, 1);
    assert_eq!(audit.executor_constructions, 1);
    assert_eq!(audit.skeleton_runs, 1);
    assert_eq!(audit.day_frame_constructions, expected_day_frames);
    assert_eq!(audit.day_frame_commits, expected_day_frames);
    assert_eq!(
        audit.phase_view_constructions,
        expected_day_frames * DIRECT_PHASE_COUNT as u64
    );
    assert_eq!(
        audit.phase_span_runs,
        1 + expected_day_frames * r5c_day_span_run_count()
    );
    assert_eq!(
        audit.direct_phase_entries,
        DIRECT_R3C_PHASE_SPAN_COUNT as u64 + expected_day_frames * r5c_day_phase_entry_count()
    );
    assert_eq!(
        audit.direct_compute_operations,
        1 + expected_day_frames * r5c_day_span_run_count()
    );
    assert_eq!(
        audit.direct_state_mutations,
        1 + expected_day_frames * r5c_day_span_run_count() + expected_dynamic_transfers
    );
    assert_eq!(
        audit.downstream_operand_productions,
        1 + expected_day_frames * r5c_day_span_run_count() + expected_dynamic_transfers
    );
    assert_eq!(
        audit.shadow_projections,
        1 + expected_day_frames * r5c_day_span_run_count()
    );
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r6a_publication_capture_records_run_bound_rows_without_publication_alias() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(11, 2637, 2, 2)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct frame should construct for capture");
    for (index, lane) in frame.lanes.iter_mut().enumerate() {
        lane.area_m2 = if index == 0 { 100.0 } else { 101.0 };
        lane.publication.runoff_m = 0.875;
        lane.publication.evapotranspiration_m = 0.25;
        lane.publication.drainage_m = 0.125;
        lane.publication.lateral_flow_m = 0.0625;
    }
    let calendar_days = [
        DirectPublicationCalendarDay {
            year: 2026,
            julian_day: 274,
            month: 10,
            day_of_month: 1,
            water_year: 2027,
        },
        DirectPublicationCalendarDay {
            year: 2026,
            julian_day: 275,
            month: 10,
            day_of_month: 2,
            water_year: 2027,
        },
    ];
    let metadata = DirectPublicationRunMetadata {
        run_name: "r6a_capture".to_string(),
        runtime_selection: "direct-publication-frame-shadow".to_string(),
        output_policy: "test".to_string(),
    };

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture(&mut frame, metadata, &calendar_days)
        .expect("direct publication capture should execute");

    assert_eq!(execution.publication_frame.rows().len(), 4);
    let first_row = execution
        .publication_frame
        .first_day()
        .expect("capture should include first row");
    assert_eq!(first_row.calendar.julian_day, 274);
    assert_eq!(first_row.area_m2.to_bits(), 100.0_f64.to_bits());
    assert_ne!(first_row.runoff.q_mm.to_bits(), 875.0_f64.to_bits());
    assert_eq!(first_row.runoff.q_mm.to_bits(), 0.0_f64.to_bits());
    assert_eq!(execution.report.compatibility_edge_invocation_count, 0);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.publication_capture_runs, 1);
    assert_eq!(audit.skeleton_runs, 0);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r6f_publication_capture_accepts_typed_process_inputs_and_carries_layers() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(17, 501, 1, 2)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct frame should construct for capture");
    frame.lanes[0].area_m2 = 100.0;
    let day_inputs = r6f_typed_publication_day_inputs();
    let metadata = DirectPublicationRunMetadata {
        run_name: "r6f_typed_inputs".to_string(),
        runtime_selection: "direct-publication-frame-cutover-candidate".to_string(),
        output_policy: "test".to_string(),
    };

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_day_inputs(&mut frame, metadata, &day_inputs)
        .expect("typed direct publication capture should execute");
    let rows = execution.publication_frame.rows();

    assert_eq!(rows.len(), 2);
    assert!((rows[0].evaporation.es_mm - 10.0).abs() < 1.0e-12);
    assert!((rows[0].storage.total_soil_mm - 190.0).abs() < 1.0e-12);
    assert_eq!(rows[0].profile.depth_mm, Some(400.0));
    assert_eq!(rows[0].profile.porosity_cap_mm, Some(200.0));
    assert_eq!(rows[0].profile.fc_store_mm, Some(100.0));
    assert_eq!(rows[0].profile.wp_store_mm, Some(50.0));
    assert!((rows[1].storage.total_soil_mm - 190.0).abs() < 1.0e-12);
    assert_eq!(frame.lanes[0].subsurface_layers.len(), 1);
    assert!((frame.lanes[0].subsurface_layers[0].theta_m - 0.190).abs() < 1.0e-12);
}

#[test]
fn r7d4_publication_qofe_equals_q_with_independent_runvol_basis() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(17, 501, 1, 1)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct frame should construct for capture");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[0].runoff_publication_q_scale = 0.25;
    frame.lanes[0].runoff_publication_qofe_scale = 1.0;
    frame.lanes[0].runoff_publication_efflen_m = 1.0;
    frame.lanes[0].runoff_publication_cumulative_length_m = 4.0;
    frame.lanes[0].runoff_publication_ofe_length_m = 1.0;
    let metadata = DirectPublicationRunMetadata {
        run_name: "r7d4_runoff_publication_scale".to_string(),
        runtime_selection: "direct-publication-frame-cutover-candidate".to_string(),
        output_policy: "test".to_string(),
    };
    let day_input = r7d4_mofe_carry_publication_day(0.700, true);

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_day_inputs(&mut frame, metadata, &[day_input])
        .expect("direct publication capture should execute");
    let row = execution
        .publication_frame
        .first_day()
        .expect("capture should include first row");

    // MOFEFID-B02 (INV-RUNOFFPART-032): QOFE is published as Q (both
    // cumulative-length normalized) — no longer the per-OFE local-length
    // value. With ofe_length (1.0) != cumulative_length (4.0), the retained
    // runvol basis is independent of the published QOFE=Q, so runvol_m3 is
    // NOT the naive QOFE x area (it reflects the per-OFE ofe_length geometry).
    assert!(row.runoff.qofe_mm > 0.0);
    assert_eq!(row.runoff.q_mm.to_bits(), row.runoff.qofe_mm.to_bits());
    let naive_qofe_volume_m3 = row.runoff.qofe_mm * 0.001 * 100.0;
    assert!(
        (row.runoff.runvol_m3 - naive_qofe_volume_m3).abs() > 1.0e-9,
        "runvol must retain the independent per-OFE basis, not QOFE=Q x area: runvol={} naive={}",
        row.runoff.runvol_m3,
        naive_qofe_volume_m3
    );
    assert_eq!(execution.report.compatibility_edge_invocation_count, 0);
}

#[test]
fn r7d4_publication_interception_is_external_to_published_soil_storage() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(17, 501, 1, 1)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct frame should construct for capture");
    frame.lanes[0].area_m2 = 100.0;
    let metadata = DirectPublicationRunMetadata {
        run_name: "r7d4_interception_publication".to_string(),
        runtime_selection: "direct-publication-frame-cutover-candidate".to_string(),
        output_policy: "test".to_string(),
    };
    let layer_inputs = r6f_typed_base_layer_inputs();
    let base_layer = DirectSubsurfaceLayerState::from(layer_inputs.clone());
    let mut day_input =
        r6f_typed_first_day(base_layer, layer_inputs, r6f_typed_process_projection());
    day_input.precipitation_m = 0.002;
    day_input.interception_m = 0.002;
    day_input.storage_input_inputs = Some(DirectStorageInputInputs {
        precip_input_handoff_m: Some(0.002),
    });
    day_input.liquid_input_inputs = Some(DirectLiquidInputInputs {
        liquid_input_handoff_m: 0.0,
    });
    day_input.evapotranspiration_compute_inputs = Some(r6h_typed_evapotranspiration_inputs(0.0));

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_day_inputs(&mut frame, metadata, &[day_input])
        .expect("direct publication capture should execute");
    let row = execution
        .publication_frame
        .first_day()
        .expect("capture should include first row");
    assert!((row.interception.interception_mm - 2.0).abs() < 1.0e-12);
    assert!((row.storage.total_soil_mm - 200.0).abs() < 1.0e-12);
    assert_eq!(
        frame.lanes[0].water.soil_water_m.to_bits(),
        0.200_f64.to_bits()
    );
    assert_eq!(execution.report.compatibility_edge_invocation_count, 0);
}

#[test]
fn r6h_publication_capture_builds_lane_day_inputs_after_direct_commit() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(17, 501, 2, 2)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct frame should construct for capture");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[1].area_m2 = 125.0;
    let process_projection = r6f_typed_process_projection();
    let first_day_theta = [0.200, 0.190];
    let mut observed_carried_theta = Vec::new();
    let metadata = DirectPublicationRunMetadata {
        run_name: "r6h_interleaved_inputs".to_string(),
        runtime_selection: "direct-publication-frame-cutover-candidate".to_string(),
        output_policy: "test".to_string(),
    };

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_interleaved_day_inputs(
            &mut frame,
            metadata,
            |frame, day_index, lane_index| {
                if day_index == 0 {
                    return Ok(r6h_typed_first_day_for_lane(
                        first_day_theta[lane_index],
                        process_projection,
                    ));
                }
                let carried_theta = frame.lanes[lane_index]
                    .subsurface_layers
                    .first()
                    .expect("prior direct day must commit lane layer state")
                    .theta_m;
                observed_carried_theta.push((lane_index, carried_theta));
                Ok(r6h_typed_second_day_from_carried(
                    carried_theta,
                    process_projection,
                ))
            },
        )
        .expect("interleaved typed direct publication capture should execute");

    let rows = execution.publication_frame.rows();
    assert_eq!(rows.len(), 4);
    assert_eq!(observed_carried_theta.len(), 2);
    assert_eq!(observed_carried_theta[0].0, 0);
    assert_eq!(observed_carried_theta[1].0, 1);
    assert!((observed_carried_theta[0].1 - 0.190).abs() < 1.0e-12);
    assert!((observed_carried_theta[1].1 - 0.180).abs() < 1.0e-12);
    assert!((rows[2].evaporation.es_mm - 1.9).abs() < 1.0e-12);
    assert!((rows[3].evaporation.es_mm - 1.8).abs() < 1.0e-12);
    assert_eq!(execution.report.compatibility_edge_invocation_count, 0);
}

#[test]
fn r7d4_publication_capture_copies_mofe_carry_to_downstream_lane_before_r4j() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(17, 501, 2, 1)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct frame should construct for capture");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[1].area_m2 = 50.0;
    frame.lanes[1].upstream_area_ratio = 2.0;
    let metadata = DirectPublicationRunMetadata {
        run_name: "r7d4_dynamic_mofe_transfer".to_string(),
        runtime_selection: "direct-publication-frame-cutover-candidate".to_string(),
        output_policy: "test".to_string(),
    };

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_interleaved_day_inputs(
            &mut frame,
            metadata,
            |_frame, _day_index, lane_index| {
                Ok(if lane_index == 0 {
                    r7d4_mofe_carry_publication_day(0.700, true)
                } else {
                    r7d4_mofe_carry_publication_day(0.200, false)
                })
            },
        )
        .expect("dynamic MOFE transfer should reach downstream lane before R4J");

    let rows = execution.publication_frame.rows();
    assert_eq!(rows.len(), 2);
    let downstream_surface_m = frame.lanes[1].transfer.surface_carry_m.iter().sum::<f64>();
    let downstream_lateral_m = frame.lanes[1].transfer.lateral_carry_m.iter().sum::<f64>();
    assert!(
        downstream_surface_m > 0.0,
        "upstream ui_SCrunf must become downstream ui_SUrunf"
    );
    assert!(
        downstream_lateral_m > 0.0,
        "upstream ui_LfCrf must become downstream ui_LfUrf"
    );
    let upstream_q_runoff_m = rows[0].runoff.q_mm / 1_000.0;
    assert!(
        upstream_q_runoff_m > downstream_lateral_m,
        "fixture must anti-alias surface runoff from lateral carry"
    );
    assert!((downstream_surface_m - upstream_q_runoff_m).abs() < 1.0e-12);
    let scaled_downstream_surface_m = downstream_surface_m * frame.lanes[1].upstream_area_ratio;
    let scaled_downstream_lateral_m = downstream_lateral_m * frame.lanes[1].upstream_area_ratio;
    assert!(
        (rows[1].transfer.upstream_surface_mm / 1_000.0 - scaled_downstream_surface_m).abs()
            < 1.0e-12
    );
    assert!(
        (rows[1].transfer.upstream_lateral_mm / 1_000.0 - scaled_downstream_lateral_m).abs()
            < 1.0e-12
    );
    assert!(
        (rows[1].runoff.q_mm / 1_000.0
            - (scaled_downstream_surface_m + scaled_downstream_lateral_m))
            .abs()
            < 1.0e-9
    );
    assert_eq!(execution.report.compatibility_edge_invocation_count, 0);
}

#[test]
fn r6i_direct_frost_carry_projection_skips_inactive_frost_storage() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(17, 501, 1, 1)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct frame should construct for capture");
    frame.lanes[0].area_m2 = 100.0;
    let mut layer_inputs = r6f_typed_base_layer_inputs();
    layer_inputs.theta_m = 0.023_876_766_951_720_32;
    layer_inputs.depth_m = 0.200;
    layer_inputs.residual_theta = 0.140_679_649_464_459_6;
    layer_inputs.field_capacity_m = 0.050;
    layer_inputs.upper_limit_m = 0.100;
    let base_layer = DirectSubsurfaceLayerState::from(layer_inputs.clone());
    let mut process_projection = r6f_typed_process_projection();
    process_projection.profile_depth_m = Some(0.200);
    process_projection.profile_porosity_cap_m = Some(0.100);
    let mut day_input = r6f_typed_first_day(base_layer, layer_inputs, process_projection);
    let initial_soil_water_m = 0.023_876_766_951_720_32 + 0.140_679_649_464_459_6 * 0.200;
    day_input.initial_soil_water_m = Some(initial_soil_water_m);
    if let Some(percolation_inputs) = day_input.percolation_inputs.as_mut() {
        percolation_inputs.soil_water_initial_m = initial_soil_water_m;
    }
    day_input.evapotranspiration_compute_inputs = Some(r6h_typed_evapotranspiration_inputs(0.0));
    day_input.frost_layer_carry_projection = Some(vec![DirectFrostLayerCarryProjection {
        layer_index: 1,
        fine_layer_count: 10,
        fine_layer_thickness_m: 0.020,
    }]);
    let metadata = DirectPublicationRunMetadata {
        run_name: "r6i_frost_carry_projection".to_string(),
        runtime_selection: "direct-publication-frame-cutover-candidate".to_string(),
        output_policy: "test".to_string(),
    };

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_day_inputs(&mut frame, metadata, &[day_input])
        .expect("typed direct publication capture should execute");

    assert_eq!(execution.report.compatibility_edge_invocation_count, 0);
    assert_eq!(
        frame.lanes[0].subsurface_layers[0].theta_m.to_bits(),
        0.023_876_766_951_720_32_f64.to_bits()
    );
}

#[test]
fn r7h_followup_publication_layers_supersede_carried_layers() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(18, 501, 1, 2)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct frame should construct for capture");
    frame.lanes[0].area_m2 = 100.0;

    let process_projection = r6f_typed_process_projection();
    let base_layer_inputs = r6f_typed_base_layer_inputs();
    let mut first_day = r6f_typed_first_day(
        DirectSubsurfaceLayerState::from(base_layer_inputs.clone()),
        base_layer_inputs,
        process_projection,
    );
    first_day.evapotranspiration_compute_inputs = Some(r6h_typed_evapotranspiration_inputs(0.0));

    let mut replacement_layer_inputs = r6f_typed_base_layer_inputs();
    replacement_layer_inputs.theta_m = 0.123;
    let replacement_layer = DirectSubsurfaceLayerState::from(replacement_layer_inputs.clone());
    let mut second_day = r6f_typed_calendar_day(2);
    second_day.initial_soil_water_m = Some(replacement_layer.theta_m);
    second_day.percolation_inputs = Some(DirectPercolationInputs {
        soil_water_initial_m: replacement_layer.theta_m,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps: 1,
        restrictive_layer_enabled: false,
        restrictive_layer_conductivity_m_s: 0.0,
        restrictive_layer_thickness_m: 0.0,
        layers: vec![replacement_layer],
    });
    second_day.subsurface_compute_inputs =
        Some(r6f_typed_subsurface_inputs(replacement_layer_inputs));
    second_day.evapotranspiration_compute_inputs = Some(r6h_typed_evapotranspiration_inputs(0.0));
    second_day.hydrology_projection_inputs = Some(process_projection);

    let metadata = DirectPublicationRunMetadata {
        run_name: "r7h_followup_publication_layers".to_string(),
        runtime_selection: "direct-publication-frame-cutover-candidate".to_string(),
        output_policy: "test".to_string(),
    };

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_day_inputs(&mut frame, metadata, &[first_day, second_day])
        .expect("typed direct publication capture should execute");

    assert_eq!(execution.report.compatibility_edge_invocation_count, 0);
    assert_eq!(
        frame.lanes[0].subsurface_layers[0].theta_m.to_bits(),
        0.123_f64.to_bits(),
        "nonempty follow-up publication layers must not be overwritten by carried lane layers"
    );
}

#[test]
fn r7d5_erosion_active_publication_fails_closed_without_direct_sediment_producer() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(7, 2637, 1, 1)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct publication frame should construct");
    frame.lanes[0].area_m2 = 100.0;
    let mut day_input = r6f_typed_publication_day_inputs()[0].clone();
    day_input.erosion_producer_required = true;
    let metadata = DirectPublicationRunMetadata {
        run_name: "r7d5_erosion_required".to_string(),
        runtime_selection: "direct-production-executor".to_string(),
        output_policy: "test".to_string(),
    };

    let error = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
        .run_publication_capture_with_day_inputs(&mut frame, metadata, &[day_input])
        .expect_err("erosion-active direct publication must not emit zero sediment authority");

    assert_eq!(
        error,
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R7D5 direct Wave-1 sediment producer"
        }
    );
}

#[test]
fn r7g_peak_runoff_tiny_hourly_depth_retains_positive_rate_without_floor() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut weights = [0.0; 24];
    weights[6] = 1.0;
    let (peak_rate_m_s, duration_s, peak_hour) =
        crate::direct_runtime::test_hourly_peak_runoff_depth_rate_m_s(1.0e-15, &weights)
            .expect("positive near-zero hourly runoff remains representable");

    assert!((peak_rate_m_s - 1.0e-15 / 3_600.0).abs() < f64::EPSILON);
    assert!((duration_s - 3_600.0).abs() < 1.0e-9);
    assert_eq!(peak_hour, 6);
}

#[test]
// E.3 2e: this test previously proved the EROD14 qin accepted a committed
// typed zero qout from an inactive upstream lane. With the Wave-2 arm
// deleted, the equivalent inter-OFE guarantee is that an inactive upstream
// lane publishes NO erosion inflow intake (the Wave-1 chain never
// fabricates a handoff) and the run completes with the committed zero.
fn r7g_zero_upstream_lane_publishes_no_erosion_inflow_intake() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(8, 2637, 2, 1)
        .expect("valid direct publication identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct publication frame should construct");
    frame.lanes[0].area_m2 = 100.0;
    frame.lanes[1].area_m2 = 100.0;
    let base_day = r6f_typed_publication_day_inputs()[0].clone();
    let lane0_day = base_day.clone();
    let mut lane1_day = base_day;
    // 2e: no Wave-2 producer exists; the crafted inputs carry a disabled
    // Wave-1 seed, so this lane publishes zero-authority erosion.
    lane1_day.erosion_inputs = Some(r7d6_typed_erosion_inputs());
    let lane_days = [lane0_day, lane1_day];
    let metadata = DirectPublicationRunMetadata {
        run_name: "r7g_zero_upstream_erosion_qout".to_string(),
        runtime_selection: "direct-production-executor".to_string(),
        output_policy: "test".to_string(),
    };

    let execution = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
        .run_publication_capture_with_interleaved_day_inputs(
            &mut frame,
            metadata,
            |_frame, _day_index, lane_index| Ok(lane_days[lane_index].clone()),
        )
        .expect(
            "the chain must accept a committed typed zero handoff from an inactive upstream lane",
        );

    assert_eq!(execution.report.compatibility_edge_invocation_count, 0);
    assert!(
        frame.lanes[1].erosion_inflow_intake.is_none(),
        "an erosion-inactive upstream lane must not publish an inflow intake"
    );
    assert_eq!(execution.publication_frame.rows().len(), 2);
}

fn r7d4_mofe_carry_publication_day(theta_m: f64, carry_enabled: bool) -> DirectPublicationDayInput {
    let mut day = r6f_typed_calendar_day(1);
    let layer_inputs = r7d4_mofe_transfer_layer_inputs(theta_m, carry_enabled);
    let layer_state = DirectSubsurfaceLayerState::from(layer_inputs.clone());
    day.precipitation_m = if carry_enabled { 0.050 } else { 0.0 };
    day.initial_soil_water_m = Some(theta_m);
    day.liquid_input_inputs = Some(DirectLiquidInputInputs {
        liquid_input_handoff_m: if carry_enabled { 0.050 } else { 0.0 },
    });
    day.percolation_inputs = Some(DirectPercolationInputs {
        soil_water_initial_m: theta_m,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps: 24,
        restrictive_layer_enabled: false,
        restrictive_layer_conductivity_m_s: 0.0,
        restrictive_layer_thickness_m: 0.0,
        layers: vec![layer_state],
    });
    day.subsurface_compute_inputs = Some(DirectSubsurfaceComputeInputs {
        avg_slope: if carry_enabled { 0.10 } else { 0.0 },
        slope_length_m: 1.0,
        lateral_anisotropy_ratio: 1.0,
        soil_depth_m: 0.400,
        solwpv_mode: 2006,
        mofe_hourly_carry_arrays_enabled: carry_enabled,
        lane_substeps: 24,
        drainage_capacity_m: 0.0,
        drain_enabled: false,
        drain_depth_m: 0.5,
        drain_spacing_m: 1.0,
        drain_diameter_m: 0.1,
        layers: vec![layer_inputs],
    });
    day.infiltration_depression_inputs = Some(DirectInfiltrationDepressionInputs {
        cumulative_infiltration_handoff_m: 0.0,
        depression_storage_delta_handoff_m: 0.0,
        producer_inputs: Some(crate::DirectWb14InfiltrationProducerInputs {
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: if carry_enabled { 0.050 / 3_600.0 } else { 0.0 },
            }],
            hourly_additional_supply_m: [0.0; 24],
            effective_conductivity_m_s: 1.0e-5,
            matric_potential_m: 0.05,
            storage_capacity_m: 0.0,
            depression_storage_capacity_m: 0.0,
        }),
    });
    day.evapotranspiration_compute_inputs = Some(r6h_typed_evapotranspiration_inputs(0.0));
    day.hydrology_projection_inputs = Some(r6f_typed_process_projection());
    day
}

fn r5a_peak_runoff_day_inputs() -> DirectDayConstructorInputs {
    let mut inputs = DirectDayConstructorInputs::zero();
    inputs.infiltration_depression_inputs.producer_inputs =
        Some(crate::DirectWb14InfiltrationProducerInputs {
            hyetograph: vec![DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 3_600.0,
                intensity_m_s: 0.0,
            }],
            hourly_additional_supply_m: [0.0; 24],
            effective_conductivity_m_s: 1.0e-5,
            matric_potential_m: 0.05,
            storage_capacity_m: 0.0,
            depression_storage_capacity_m: 0.0,
        });
    inputs
}

fn r7d6_typed_erosion_inputs() -> DirectErosionInputs {
    DirectErosionInputs {
        wave1_enabled: true,
        wave1_continuity: Box::new(DirectWave1ContinuityInputs::zero()),
        wave1_operand_seed: Box::new(DirectWave1OperandSeed::disabled()),
        hydrograph_shape_authority: crate::DirectErosionHydrographShapeAuthority::Dc01SourceShape,
        routed_hydrograph_runoff_fraction: None,
        wave1: DirectErod13Inputs {
            ie_m_s: 0.000_01,
            te_s: 60.0,
            fs: 0.5,
            ft: 1.0,
            taufe_pa: 4.0,
            q_m2_s: 0.001,
            g_kg_s_m: 1.0,
            di_kg_s_m2: 0.002,
            beta: 0.5,
            vf_m_s: 0.2,
            dgdx_kg_s_m2: 0.0195,
            cntlen_m: 10.0,
            kr_s_m: 0.01,
            kradjf: 1.0,
            tcadjf: 0.5,
            shrsol_pa: 2.0,
            tcend_kg_s_m: 10.0,
            shcrit_pa: 1.0,
            detinr_kg_s_m2: 0.001,
            effdrr_m: 1.0,
            effdrn_m: 1.0,
            veleff_m_s: 0.2,
            pkro_m3_s: 0.001,
            tc_k: 2.0,
            tc_m: 1.0,
            q_runoff_m: 0.01,
            peakro_m_s: 0.001,
            watdur_s: 10.0,
        },
    }
}

fn r7d4_mofe_transfer_layer_inputs(
    theta_m: f64,
    carry_enabled: bool,
) -> DirectSubsurfaceLayerInputs {
    DirectSubsurfaceLayerInputs {
        theta_m,
        field_capacity_m: 0.100,
        upper_limit_m: 0.500,
        conductivity_m_s: 1.0e-10,
        depth_m: 0.400,
        residual_theta: 0.0,
        frozen_depth_m: 0.0,
        frozen_water_m: 0.0,
        porosity: 0.5,
        field_capacity_theta: 0.25,
        coca: 1.0,
        lateral_conductivity_m_s: if carry_enabled { 1.0e-8 } else { 1.0e-10 },
    }
}

fn r6f_typed_publication_day_inputs() -> [DirectPublicationDayInput; 2] {
    let base_layer = DirectSubsurfaceLayerState::from(r6f_typed_base_layer_inputs());
    let base_layer_inputs = DirectSubsurfaceLayerInputs::from(base_layer.clone());
    let process_projection = r6f_typed_process_projection();
    [
        r6f_typed_first_day(base_layer, base_layer_inputs, process_projection),
        r6f_typed_second_day(process_projection),
    ]
}

fn r6h_typed_first_day_for_lane(
    theta_m: f64,
    process_projection: DirectHydrologyProjectionInputs,
) -> DirectPublicationDayInput {
    let mut layer_inputs = r6f_typed_base_layer_inputs();
    layer_inputs.theta_m = theta_m;
    let base_layer = DirectSubsurfaceLayerState::from(layer_inputs.clone());
    let mut first_day = r6f_typed_first_day(base_layer, layer_inputs, process_projection);
    first_day.initial_soil_water_m = Some(theta_m);
    if let Some(percolation_inputs) = first_day.percolation_inputs.as_mut() {
        percolation_inputs.soil_water_initial_m = theta_m;
    }
    first_day
}

fn r6h_typed_second_day_from_carried(
    carried_theta_m: f64,
    process_projection: DirectHydrologyProjectionInputs,
) -> DirectPublicationDayInput {
    let mut second_day = r6f_typed_calendar_day(2);
    let soil_evaporation_m = carried_theta_m / 100.0;
    second_day.evapotranspiration_compute_inputs =
        Some(r6h_typed_evapotranspiration_inputs(soil_evaporation_m));
    second_day.hydrology_projection_inputs = Some(process_projection);
    second_day
}

fn r6f_typed_base_layer_inputs() -> DirectSubsurfaceLayerInputs {
    DirectSubsurfaceLayerInputs {
        theta_m: 0.200,
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

fn r6f_typed_process_projection() -> DirectHydrologyProjectionInputs {
    DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 1.0e-12,
        snow_water_m: 0.0,
        frozen_soil_water_m: 0.0,
        frost_depth_m: 0.0,
        profile_depth_m: Some(0.400),
        profile_porosity_cap_m: Some(0.200),
        profile_field_capacity_m: Some(0.100),
        profile_wilting_point_m: Some(0.050),
    }
}

fn r6f_typed_first_day(
    base_layer: DirectSubsurfaceLayerState,
    base_layer_inputs: DirectSubsurfaceLayerInputs,
    process_projection: DirectHydrologyProjectionInputs,
) -> DirectPublicationDayInput {
    let mut first_day = r6f_typed_calendar_day(1);
    first_day.initial_soil_water_m = Some(0.200);
    first_day.percolation_inputs = Some(DirectPercolationInputs {
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
    first_day.subsurface_compute_inputs = Some(r6f_typed_subsurface_inputs(base_layer_inputs));
    first_day.evapotranspiration_compute_inputs = Some(r6f_typed_evapotranspiration_inputs());
    first_day.hydrology_projection_inputs = Some(process_projection);
    first_day
}

fn r6f_typed_second_day(
    process_projection: DirectHydrologyProjectionInputs,
) -> DirectPublicationDayInput {
    let mut second_day = r6f_typed_calendar_day(2);
    second_day.hydrology_projection_inputs = Some(process_projection);
    second_day
}

fn r6f_typed_calendar_day(day: u16) -> DirectPublicationDayInput {
    DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
        year: 2026,
        julian_day: day,
        month: 1,
        day_of_month: if day == 1 { 1 } else { 2 },
        water_year: 2026,
    })
}

fn r6f_typed_subsurface_inputs(
    base_layer_inputs: DirectSubsurfaceLayerInputs,
) -> DirectSubsurfaceComputeInputs {
    DirectSubsurfaceComputeInputs {
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
        layers: vec![base_layer_inputs],
    }
}

fn r6f_typed_evapotranspiration_inputs() -> DirectEvapotranspirationComputeInputs {
    r6h_typed_evapotranspiration_inputs(0.010)
}

fn r6h_typed_evapotranspiration_inputs(
    soil_evaporation_m: f64,
) -> DirectEvapotranspirationComputeInputs {
    DirectEvapotranspirationComputeInputs {
        et_demand_m: soil_evaporation_m,
        leaf_area_index: 0.0,
        canopy_height_m: 0.0,
        canopy_cover_fraction: 0.0,
        residue_interception_m: 0.0,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: 0.0,
        plant_tolerance: 0.25,
        growth_context_required: false,
        stage_state: None,
        pmet: Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m,
            plant_transpiration_m: 0.0,
            soil_evaporation_storage_return_m: 0.0,
        }),
        pmet_compute: None,
    }
}

#[test]
fn r5e_direct_endpoint_records_exactly_ordered_fourteen_phase_entries() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(7, 2637, 2, 3)
        .expect("valid direct skeleton identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct skeleton frame should construct");
    let executor = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly);

    let report = executor
        .run_skeleton(&mut frame)
        .expect("R5E direct endpoint skeleton should execute");

    let expected_day_frames = (identity.lane_count * identity.day_count) as u64;
    assert_eq!(report.planned_phase_count, DIRECT_PHASE_COUNT);
    assert_eq!(
        report.canonical_phase_entry_count,
        expected_day_frames * DIRECT_PHASE_COUNT as u64
    );
    assert_eq!(report.phase_view_count, report.canonical_phase_entry_count);
    assert_r5c_phase_status_counts(&report.phase_status_counts, expected_day_frames);
    assert!(
        report.direct_phase_entry_count > report.canonical_phase_entry_count,
        "sub-operation counters must remain distinct from canonical phase entries"
    );
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.day_frame_commit_count, expected_day_frames);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(
        audit.phase_view_constructions,
        report.canonical_phase_entry_count
    );
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r5a_direct_skeleton_commits_day_state_back_to_lane_state() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(7, 2637, 1, 3)
        .expect("valid direct skeleton identity should construct");
    let mut frame =
        DirectRunFrame::skeleton(identity).expect("direct skeleton frame should construct");
    frame.lanes[0].transfer.surface_carry_m[5] = 0.25;
    frame.lanes[0].transfer.upstream_flow_m = 0.125;
    frame.lanes[0].transfer.surface_hourly_weights[5] = 1.0;
    frame.lanes[0].day_inputs = vec![r5a_peak_runoff_day_inputs(); 3];
    let executor = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly);

    let report = executor
        .run_skeleton(&mut frame)
        .expect("direct skeleton lifecycle should commit all day frames");

    assert_eq!(report.day_frame_commit_count, 3);
    assert!((frame.lanes[0].transfer.surface_carry_m[5] - 0.25).abs() < 1.0e-12);
    assert!((frame.lanes[0].transfer.upstream_flow_m - 0.125).abs() < 1.0e-12);
    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 3);
    assert_eq!(audit.day_frame_commits, 3);
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
    let direct_source_paths = [
        "direct_runtime.rs",
        "direct_runtime/00_core_frames.rs",
        "direct_runtime/01_publication.rs",
        "direct_runtime/02_state_reports.rs",
        "direct_runtime/03_executor.rs",
        "direct_runtime/04_audit_error_helpers.rs",
        "direct_runtime/decomposition.rs",
        "direct_runtime/storage.rs",
        "direct_runtime/runoff.rs",
        "direct_runtime/subsurface.rs",
        "direct_runtime/evapotranspiration.rs",
        "direct_runtime/normalization.rs",
        "direct_runtime/growth.rs",
        "direct_runtime/projection.rs",
    ];
    let direct_sources = direct_source_paths
        .iter()
        .map(|source_path| {
            let path = format!("{}/src/{source_path}", env!("CARGO_MANIFEST_DIR"));
            (
                *source_path,
                std::fs::read_to_string(&path).unwrap_or_else(|err| {
                    panic!("direct runtime source {path} should be readable: {err}")
                }),
            )
        })
        .collect::<Vec<_>>();

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
fn r7b_typed_run_constructor_roundtrips_single_ofe_sidecar_defaults() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(77, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let inputs = DirectRunConstructorInputs::new(
        identity,
        vec![
            DirectLaneConstructorInputs::from_topology(0, 1, 1)
                .expect("single OFE lane constructor input should build"),
        ],
    );

    let frame = DirectRunFrame::from_constructor_inputs(inputs)
        .expect("typed single OFE direct frame should construct");

    assert_eq!(frame.identity, identity);
    assert_eq!(frame.lanes.len(), 1);
    assert_eq!(frame.lanes[0].lane_id, 1);
    assert_r7b_close(frame.lanes[0].area_m2, 1.0);
    assert_eq!(frame.lanes[0].day_inputs.len(), 1);
    assert_eq!(
        frame.lanes[0].day_inputs[0].forcing,
        DirectDayForcing::zero()
    );
    assert_eq!(
        frame.lanes[0].day_inputs[0].snow_coupling_inputs,
        DirectSnowCouplingInputs::zero()
    );
    assert_eq!(
        frame.lanes[0].day_inputs[0].frost_layer_carry_projection,
        None
    );
    assert_eq!(
        frame.lanes[0].day_inputs[0]
            .evapotranspiration_compute_inputs
            .pmet,
        None
    );
}

#[test]
fn r7b_typed_run_constructor_roundtrips_multiofe_daily_parsed_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(78, 2637, 2, 2)
        .expect("valid direct constructor identity should construct");
    let mut lanes = vec![
        DirectLaneConstructorInputs::from_topology(0, 2, 2)
            .expect("upstream lane constructor input should build"),
        DirectLaneConstructorInputs::from_topology(1, 2, 2)
            .expect("outlet lane constructor input should build"),
    ];

    lanes[0].area_m2 = 125.0;
    lanes[0].water.soil_water_m = 0.21;
    lanes[0].transfer.surface_carry_m[3] = 0.004;
    lanes[0].publication.runoff_m = 0.015;
    lanes[0].subsurface_layers = vec![r7b_constructor_layer(0.18, 0.05, 0.40, 0.30, 0.02)];
    lanes[0].day_inputs[0] = r7b_breakpoint_management_pmet_day();
    lanes[0].day_inputs[1].forcing = DirectDayForcing {
        precipitation_m: 0.003,
        effective_temperature_c: -1.25,
    };

    lanes[1].area_m2 = 130.0;
    lanes[1].upstream_area_ratio = 1.04;
    lanes[1].subsurface_layers = vec![r7b_constructor_layer(0.16, 0.04, 0.38, 0.28, 0.0)];
    lanes[1].day_inputs[0].snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: 0.006,
        ..DirectSnowCouplingInputs::zero()
    };
    lanes[1].day_inputs[0]
        .hydrology_projection_inputs
        .snow_water_m = 0.011;
    lanes[1].day_inputs[0]
        .hydrology_projection_inputs
        .frozen_soil_water_m = 0.002;
    lanes[1].day_inputs[0].frost_layer_carry_projection =
        Some(vec![DirectFrostLayerCarryProjection {
            layer_index: 1,
            fine_layer_count: 2,
            fine_layer_thickness_m: 0.10,
        }]);

    let frame =
        DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(identity, lanes))
            .expect("typed multi OFE direct frame should construct");

    assert_eq!(frame.identity, identity);
    assert_eq!(frame.lanes.len(), 2);
    assert_eq!(frame.lanes[0].downstream_lane_id, 2);
    assert_eq!(frame.lanes[1].upstream_lane_id, 1);
    assert_r7b_close(frame.lanes[0].water.soil_water_m, 0.21);
    assert_r7b_close(frame.lanes[0].transfer.surface_carry_m[3], 0.004);
    assert_eq!(
        frame.lanes[0].day_inputs[0]
            .evapotranspiration_compute_inputs
            .pmet,
        Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m: 0.001,
            plant_transpiration_m: 0.002,
            soil_evaporation_storage_return_m: 0.0004,
        })
    );
    assert_r7b_close(
        frame.lanes[0].day_inputs[0]
            .residue_partition_inputs
            .cover_fraction,
        0.35,
    );
    assert_eq!(
        frame.lanes[1].day_inputs[0].frost_layer_carry_projection,
        Some(vec![DirectFrostLayerCarryProjection {
            layer_index: 1,
            fine_layer_count: 2,
            fine_layer_thickness_m: 0.10,
        }])
    );
}

#[test]
fn gwbaseflow_linear_reservoir_recurrence_uses_prior_day_exports() {
    let authority = DirectGroundwaterAuthority::linear_reservoir(0.010, 0.10, 0.05, 0.0)
        .expect("valid groundwater authority should construct");
    let mut state = DirectGroundwaterRunState::from_authority(authority, 1_000.0)
        .expect("groundwater state should initialize");

    let day1 = state
        .run_day(2.0, 1_000.0)
        .expect("day 1 groundwater should run");
    assert!(day1.enabled);
    assert_r7b_close(day1.storage_before_m3, 10.0);
    assert_r7b_close(day1.storage_after_m3, 12.0);
    assert_r7b_close(day1.baseflow_m3, 1.2);
    assert_r7b_close(day1.deep_seepage_m3, 0.6);

    let day2 = state
        .run_day(4.0, 1_000.0)
        .expect("day 2 groundwater should run");
    assert_r7b_close(day2.storage_before_m3, 12.0);
    assert_r7b_close(day2.storage_after_m3, 14.2);
    assert_r7b_close(day2.baseflow_m3, 1.42);
    assert_r7b_close(day2.deep_seepage_m3, 0.71);
}

#[test]
fn gwbaseflow_mofe_recharge_aggregates_lane_deep_percolation() {
    let identity = DirectRunIdentity::new(79, 2637, 2, 1)
        .expect("valid direct constructor identity should construct");
    let mut lanes = vec![
        DirectLaneConstructorInputs::from_topology(0, 2, 1)
            .expect("upstream lane constructor input should build"),
        DirectLaneConstructorInputs::from_topology(1, 2, 1)
            .expect("outlet lane constructor input should build"),
    ];
    lanes[0].area_m2 = 100.0;
    lanes[1].area_m2 = 300.0;
    let mut frame =
        DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(identity, lanes))
            .expect("typed multi OFE direct frame should construct");
    frame
        .configure_groundwater(
            DirectGroundwaterAuthority::linear_reservoir(0.001, 0.10, 0.20, 0.0)
                .expect("valid groundwater authority should construct"),
        )
        .expect("groundwater should configure");

    let mut day_frames = vec![
        DirectDayFrame::seed(identity, 0, 0).expect("lane 1 day frame should seed"),
        DirectDayFrame::seed(identity, 1, 0).expect("lane 2 day frame should seed"),
    ];
    day_frames[0].hydrology_projection.deep_percolation_m = 0.002;
    day_frames[1].hydrology_projection.deep_percolation_m = 0.003;

    let output = frame
        .run_groundwater_day_from_lane_frames(0, &mut day_frames)
        .expect("groundwater day should run");

    assert_r7b_close(output.recharge_m3, 1.1);
    assert_r7b_close(output.storage_before_m3, 0.4);
    assert_r7b_close(output.storage_after_m3, 1.5);
    assert_r7b_close(output.baseflow_m3, 0.15);
    assert_r7b_close(output.deep_seepage_m3, 0.30);
    assert_eq!(day_frames[0].groundwater_output, output);
    assert_eq!(day_frames[1].groundwater_output, output);
}

#[test]
fn gwbaseflow_exports_over_accepted_storage_fail_closed() {
    let authority = DirectGroundwaterAuthority::linear_reservoir(0.001, 0.80, 0.30, 0.0)
        .expect("coefficient parser permits nonnegative coefficients");
    let mut state = DirectGroundwaterRunState::from_authority(authority, 100.0)
        .expect("groundwater state should initialize");

    let error = state
        .run_day(0.0, 100.0)
        .expect_err("outflow over accepted storage should fail");
    assert!(matches!(
        error,
        DirectRuntimeError::DirectKernelGuardFailure {
            phase: "groundwater_linear_reservoir",
            ..
        }
    ));
}

#[test]
fn r7b_typed_day_constructor_supplies_r4_r5_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(79, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let day = DirectDayFrame::from_constructor_inputs(
        identity,
        0,
        0,
        r7b_breakpoint_management_pmet_day(),
    )
    .expect("typed day constructor should construct");

    assert_r7b_close(day.forcing.precipitation_m, 0.012);
    assert_r7b_close(day.normalization_inputs.precipitation_m, 0.012);
    assert_r7b_close(day.liquid_input_inputs.liquid_input_handoff_m, 0.011);
    assert_r7b_close(day.runon_carry_inputs.surface_runon_handoff_m, 0.0015);
    assert_r7b_close(day.runoff_partition_inputs.liquid_input_m, 0.011);
    assert_r7b_close(day.storage_reconciliation_inputs.precip_input_m, 0.012);
    assert_r7b_close(day.residue_partition_inputs.cover_fraction, 0.35);
    assert_eq!(
        day.evapotranspiration_compute_inputs.pmet,
        Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m: 0.001,
            plant_transpiration_m: 0.002,
            soil_evaporation_storage_return_m: 0.0004,
        })
    );
}

#[test]
fn r7b_typed_constructors_fail_closed_on_invalid_domains() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(80, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let mut lane = DirectLaneConstructorInputs::from_topology(0, 1, 1)
        .expect("lane constructor input should build");
    lane.area_m2 = 0.0;
    assert_eq!(
        DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
            identity,
            vec![lane],
        )),
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "constructor.area_m2"
        })
    );

    let mut day = DirectDayConstructorInputs::zero();
    day.forcing.precipitation_m = f64::NAN;
    assert_eq!(
        DirectDayFrame::from_constructor_inputs(identity, 0, 0, day),
        Err(DirectRuntimeError::NonFiniteDirectValue {
            field: "constructor.forcing.precipitation_m"
        })
    );
}

#[test]
fn r7b_constructor_source_excludes_forbidden_compatibility_storage_tokens() {
    let path = format!(
        "{}/src/direct_runtime/00_core_frames.rs",
        env!("CARGO_MANIFEST_DIR")
    );
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("direct core-frame source should be readable: {err}"));
    let constructor_source = source
        .split("#[derive(Debug, Clone, PartialEq)]\npub struct DirectRunConstructorInputs")
        .nth(1)
        .expect("constructor declaration block should exist")
        .split("#[derive(Debug, Clone, PartialEq)]\npub struct DirectRunFrame")
        .next()
        .expect("constructor declaration block should terminate before DirectRunFrame");

    for forbidden in [
        "HillslopeWritebackSurface",
        "BoundarySymbol",
        "BoundaryValue",
        "SymbolRegistry",
        "IndexedWritebackSurface",
        "HillslopeKernelRequest",
        "KernelWritebackPayload",
        "Wb13",
        "WB13",
    ] {
        assert!(
            !constructor_source.contains(forbidden),
            "R7B typed constructor declarations must not contain forbidden compatibility token {forbidden}"
        );
    }
}

#[test]
fn r7b_constructor_type_size_layout_is_bounded() {
    let run_constructor = std::mem::size_of::<DirectRunConstructorInputs>();
    let lane_constructor = std::mem::size_of::<DirectLaneConstructorInputs>();
    let day_constructor = std::mem::size_of::<DirectDayConstructorInputs>();
    let run_frame = std::mem::size_of::<DirectRunFrame>();
    let lane_frame = std::mem::size_of::<crate::DirectLaneFrame>();
    let day_frame = std::mem::size_of::<DirectDayFrame>();

    println!(
        "R7B type sizes: DirectRunConstructorInputs={run_constructor}; DirectLaneConstructorInputs={lane_constructor}; DirectDayConstructorInputs={day_constructor}; DirectRunFrame={run_frame}; DirectLaneFrame={lane_frame}; DirectDayFrame={day_frame}"
    );

    assert!(run_constructor <= 128);
    // DC01 (INV-RUNOFFPART-031): lane constructor inputs also carry the 24-slot
    // surface hourly-weights transfer channel (+192 B).
    assert!(lane_constructor <= 1_216);
    // D16 row-crop canhgt publication adds daily canopy height to two growth
    // inputs and the ET operand bundle.
    assert!(day_constructor <= 4_160);
    assert!(run_frame <= 512);
    // R7G carries typed snow runtime state plus SNOWDENSITY-07 CoE boundary carry and
    // SNOWDENSITY-10.3.8 retained-liquid storage at lane scope.
    // DC01 (INV-RUNOFFPART-031) adds the 24-slot surface hourly-weights channel
    // to the transfer buffers carried at lane scope (+192 B).
    // ADR-0036 (E.2): the erosion publication operands gain the paired
    // hourly surfaces (`hourly_runoff_fraction` + `hourly_sediment_mass_kg`,
    // two Option<[f64; 24]>), which ride the lane-scope publication/
    // downstream-operand embeddings (+352 B observed).
    // E.3: the boxed inter-OFE erosion inflow intake pointer (+8 B).
    assert!(lane_frame <= 1_768);
    // FROST RESIDUE-COVER IMPLEMENTATION carries dynamic residue-depth operands;
    // the direct-cutover correction carries PMET storage-return closure operands.
    // DC01 (INV-RUNOFFPART-031) adds the WB14 hourly excess profile plus the
    // transfer weights channel at day-frame scope, and the WB14 producer inputs
    // gain the 24-slot runon hourly supply (+576 B total).
    // SC-SED-001 1b-C adds the parallel WB14 hourly RAINFALL profile
    // (24-slot, +192 B, feeds the erosion `effint`) and the persistent
    // erosion runtime carry (`rfcum`/`daydis`/`ifrost`/rill width, +~32 B).
    // ADR-0036 (E.2): the day frame gains the hourly-runoff weights
    // (`wave1_hourly_weights`, +192 B), the hourly solve plan Vec header
    // (+24 B), and the paired hourly publication surfaces embedded in the
    // erosion operand/shadow/publication rows (+2,200 B total observed).
    // E.3: the boxed inter-OFE erosion inflow intake pointer (+8 B).
    // D16 row-crop canhgt publication adds daily canopy height to the ET
    // operand bundle (+8 B).
    // GWBASEFLOW M-T2B adds the per-day groundwater output carried from the
    // run-level linear-reservoir recurrence into terminal-row publication (+48 B).
    // CANOPY-PHENOLOGY-02 adds two exact consumed-value observations for the
    // erosion canopy and frost residue-depth seams (+32 B including Option
    // discriminants); these are contract closure evidence, not producers.
    // CAL04B-NATIVE-001 adds the paired erosion/frost canopy-height
    // observations (+32 B) needed to prove those consumers read the identical
    // post-growth height.
    assert!(day_frame <= 15_600);
}

fn r7b_breakpoint_management_pmet_day() -> DirectDayConstructorInputs {
    let mut day = DirectDayConstructorInputs::zero();
    day.forcing = DirectDayForcing {
        precipitation_m: 0.012,
        effective_temperature_c: 4.5,
    };
    day.normalization_inputs = DirectNormalizationInputs {
        precipitation_m: 0.012,
        effective_temperature_c: 4.5,
        storage_initial_m: 0.21,
        surface_transfer_m: 0.0015,
        lateral_transfer_m: 0.0005,
        upstream_flow_m: 0.0,
        subsurface_input_m: 0.0,
    };
    day.storage_bounds_inputs = DirectStorageBoundsInputs {
        storage_initial_m: 0.21,
        total_accounted_input_m: 0.014,
        closure_tolerance_m: 1.0e-9,
    };
    day.liquid_input_inputs = DirectLiquidInputInputs {
        liquid_input_handoff_m: 0.011,
    };
    day.runon_carry_inputs = DirectRunonCarryInputs {
        surface_runon_handoff_m: 0.0015,
        subsurface_carry_handoff_m: 0.0005,
    };
    day.runoff_partition_inputs = DirectRunoffPartitionInputs {
        liquid_input_m: 0.011,
        runon_input_m: 0.0015,
        cumulative_infiltration_m: 0.006,
        depression_storage_delta_m: 0.0005,
        surface_saturation_runoff_m: 0.0008,
        frost_retained_local_liquid_m: 0.0,
        frost_preprojected_local_liquid_m: 0.0,
    };
    day.percolation_inputs.soil_water_initial_m = 0.21;
    day.percolation_inputs.layers = vec![r7b_constructor_layer(0.18, 0.05, 0.40, 0.30, 0.0)];
    day.deep_seepage_inputs = DirectDeepSeepageInputs {
        deep_seepage_handoff_m: 0.001,
    };
    day.subsurface_loss_inputs = DirectSubsurfaceLossInputs {
        subsurface_loss_handoff_m: 0.0007,
    };
    day.evapotranspiration_inputs = DirectEvapotranspirationInputs {
        evapotranspiration_handoff_m: 0.003,
    };
    day.evapotranspiration_compute_inputs = DirectEvapotranspirationComputeInputs {
        et_demand_m: 0.004,
        leaf_area_index: 1.2,
        canopy_height_m: 0.35,
        canopy_cover_fraction: 0.45,
        residue_interception_m: 0.0002,
        same_pass_infiltration_m: 0.006,
        outside_water_depth_m: 0.0,
        root_depth_m: 0.35,
        plant_tolerance: 0.2,
        growth_context_required: false,
        stage_state: None,
        pmet: Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m: 0.001,
            plant_transpiration_m: 0.002,
            soil_evaporation_storage_return_m: 0.0004,
        }),
        pmet_compute: None,
    };
    day.residue_partition_inputs = crate::DirectResiduePartitionInputs {
        rescov_interrill_weight: 0.0,
        standing_residue_kg_m2: 0.02,
        flat_residue_offset_kg_m2: 0.01,
        buried_residue_kg_m2: 0.005,
        cover_fraction: 0.35,
    };
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: 0.002,
        ..DirectSnowCouplingInputs::zero()
    };
    day.storage_reconciliation_inputs = DirectStorageReconciliationInputs {
        storage_initial_m: 0.21,
        precip_input_m: 0.012,
        snow_coupling_m: 0.002,
        frost_liquid_delta_m: 0.0,
        runon_input_m: 0.0,
        interception_m: 0.0,
        evapotranspiration_m: 0.003,
        evapotranspiration_storage_return_m: 0.0,
        deep_seepage_m: 0.001,
        subsurface_loss_m: 0.0007,
        closure_tolerance_m: 1.0e-9,
    };
    day.hydrology_projection_inputs.snow_water_m = 0.004;
    day.hydrology_projection_inputs.frozen_soil_water_m = 0.001;
    day.hydrology_projection_inputs.profile_depth_m = Some(0.30);
    day
}

fn assert_r7b_close(observed: f64, expected: f64) {
    assert!(
        (observed - expected).abs() <= 1.0e-12,
        "observed {observed} differs from expected {expected}"
    );
}

fn r7b_constructor_layer(
    theta_m: f64,
    field_capacity_m: f64,
    upper_limit_m: f64,
    depth_m: f64,
    frozen_depth_m: f64,
) -> DirectSubsurfaceLayerState {
    DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m,
        field_capacity_m,
        upper_limit_m,
        depth_m,
        frozen_depth_m,
        ..DirectSubsurfaceLayerInputs::neutral()
    })
}

#[path = "direct_runtime_r3_r4.rs"]
mod direct_runtime_r3_r4;

#[path = "direct_runtime_r3c_r4b.rs"]
mod direct_runtime_r3c_r4b;
