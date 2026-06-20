use super::direct_runtime_test_lock;
use crate::{
    DIRECT_R4A_PHASE_SPAN_COUNT, DIRECT_R4A_RUNOFF_PARTITION_SPAN, DIRECT_R4I_LIQUID_INPUT_SPAN,
    DIRECT_R4I_PHASE_SPAN_COUNT, DIRECT_R4J_PHASE_SPAN_COUNT, DIRECT_R4J_RUNON_CARRY_SPAN,
    DIRECT_R4K_INFILTRATION_DEPRESSION_SPAN, DIRECT_R4K_PHASE_SPAN_COUNT,
    DIRECT_R4L_PHASE_SPAN_COUNT, DIRECT_R4L_SATURATION_ADDBACK_SPAN, DirectDayFrame,
    DirectInfiltrationDepressionDownstreamOperands, DirectInfiltrationDepressionInputs,
    DirectInfiltrationDepressionShadowProjection, DirectInfiltrationDepressionState,
    DirectLiquidInputDownstreamOperands, DirectLiquidInputInputs,
    DirectLiquidInputShadowProjection, DirectLiquidInputState, DirectPhaseKind, DirectRunIdentity,
    DirectRunoffDownstreamOperands, DirectRunoffPartitionInputs, DirectRunoffPartitionState,
    DirectRunoffShadowProjection, DirectRunonCarryDownstreamOperands, DirectRunonCarryInputs,
    DirectRunonCarryShadowProjection, DirectRunonCarryState, DirectRuntimeError,
    DirectSaturationAddbackDownstreamOperands, DirectSaturationAddbackInputs,
    DirectSaturationAddbackShadowProjection, DirectSaturationAddbackState,
    reset_direct_runtime_audit_counters,
};

#[test]
fn r4il_runoff_input_producers_feed_r4a_inputs_and_shadow_project() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_r4il_span_contracts();
    let mut day = r4il_seed_day();
    run_r4il_producers(&mut day);

    assert_r4il_producer_outputs(&day);
    assert_r4il_audit_counts();
}

fn assert_r4il_span_contracts() {
    assert_eq!(
        DIRECT_R4I_LIQUID_INPUT_SPAN,
        [
            DirectPhaseKind::Normalization,
            DirectPhaseKind::RunoffReconciliation
        ]
    );
    assert_eq!(
        DIRECT_R4J_RUNON_CARRY_SPAN,
        [
            DirectPhaseKind::LateralTransfer,
            DirectPhaseKind::RunoffReconciliation
        ]
    );
    assert_eq!(
        DIRECT_R4K_INFILTRATION_DEPRESSION_SPAN,
        [
            DirectPhaseKind::RunoffReconciliation,
            DirectPhaseKind::StorageReconciliation
        ]
    );
    assert_eq!(
        DIRECT_R4L_SATURATION_ADDBACK_SPAN,
        [
            DirectPhaseKind::RunoffReconciliation,
            DirectPhaseKind::StorageReconciliation
        ]
    );
}

fn assert_r4il_producer_outputs(day: &DirectDayFrame) {
    let expected_liquid = DirectLiquidInputState {
        liquid_input_m: 0.5,
    };
    let expected_runon = DirectRunonCarryState {
        runon_input_m: 0.125,
        subsurface_carry_m: 0.015_625,
    };
    let expected_infiltration = DirectInfiltrationDepressionState {
        cumulative_infiltration_m: 0.25,
        depression_storage_delta_m: 0.0625,
    };
    let expected_saturation = DirectSaturationAddbackState {
        surface_saturation_runoff_m: 0.03125,
    };

    assert_eq!(day.liquid_input, expected_liquid);
    assert_eq!(day.runon_carry, expected_runon);
    assert_eq!(day.infiltration_depression, expected_infiltration);
    assert_eq!(day.saturation_addback, expected_saturation);
    assert_eq!(
        day.liquid_input_downstream_operands,
        DirectLiquidInputDownstreamOperands::from(expected_liquid)
    );
    assert_eq!(
        day.runon_carry_downstream_operands,
        DirectRunonCarryDownstreamOperands::from(expected_runon)
    );
    assert_eq!(
        day.infiltration_depression_downstream_operands,
        DirectInfiltrationDepressionDownstreamOperands::from(expected_infiltration)
    );
    assert_eq!(
        day.saturation_addback_downstream_operands,
        DirectSaturationAddbackDownstreamOperands::from(expected_saturation)
    );
    assert_eq!(
        day.liquid_input_shadow_projection,
        Some(DirectLiquidInputShadowProjection {
            lane_index: 0,
            day_index: 0,
            liquid_input_m: 0.5
        })
    );
    assert_eq!(
        day.runon_carry_shadow_projection,
        Some(DirectRunonCarryShadowProjection {
            lane_index: 0,
            day_index: 0,
            runon_input_m: 0.125,
            subsurface_carry_m: 0.015_625
        })
    );
    assert_eq!(
        day.infiltration_depression_shadow_projection,
        Some(DirectInfiltrationDepressionShadowProjection {
            lane_index: 0,
            day_index: 0,
            cumulative_infiltration_m: 0.25,
            depression_storage_delta_m: 0.0625
        })
    );
    assert_eq!(
        day.saturation_addback_shadow_projection,
        Some(DirectSaturationAddbackShadowProjection {
            lane_index: 0,
            day_index: 0,
            surface_saturation_runoff_m: 0.03125
        })
    );
    assert_eq!(
        day.runoff_partition_inputs,
        DirectRunoffPartitionInputs {
            liquid_input_m: 0.5,
            runon_input_m: 0.125,
            cumulative_infiltration_m: 0.25,
            depression_storage_delta_m: 0.0625,
            surface_saturation_runoff_m: 0.03125
        }
    );
}

fn assert_r4il_audit_counts() {
    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 4);
    assert_eq!(
        audit.direct_phase_entries,
        (DIRECT_R4I_PHASE_SPAN_COUNT
            + DIRECT_R4J_PHASE_SPAN_COUNT
            + DIRECT_R4K_PHASE_SPAN_COUNT
            + DIRECT_R4L_PHASE_SPAN_COUNT) as u64
    );
    assert_eq!(audit.direct_compute_operations, 4);
    assert_eq!(audit.direct_state_mutations, 4);
    assert_eq!(audit.downstream_operand_productions, 4);
    assert_eq!(audit.shadow_projections, 4);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r4a_runoff_partition_consumes_r4il_producers_and_shadow_projects() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4A_RUNOFF_PARTITION_SPAN,
        [
            DirectPhaseKind::RunoffReconciliation,
            DirectPhaseKind::StorageReconciliation,
            DirectPhaseKind::ClosureDiagnostics
        ]
    );

    let mut day = r4a_ready_day();
    let report = day
        .run_r4a_runoff_partition_span()
        .expect("valid R4A runoff partition span should execute");

    let expected_state = DirectRunoffPartitionState {
        liquid_input_m: 0.5,
        runon_input_m: 0.125,
        cumulative_infiltration_m: 0.25,
        depression_storage_delta_m: 0.0625,
        surface_saturation_runoff_m: 0.03125,
        partition_runoff_m: 0.3125,
        q_runoff_m: 0.34375,
        closure_residual_m: 0.0,
    };
    let expected_shadow = DirectRunoffShadowProjection {
        lane_index: 0,
        day_index: 0,
        liquid_input_m: 0.5,
        runon_input_m: 0.125,
        cumulative_infiltration_m: 0.25,
        depression_storage_delta_m: 0.0625,
        surface_saturation_runoff_m: 0.03125,
        partition_runoff_m: 0.3125,
        q_runoff_m: 0.34375,
        closure_residual_m: 0.0,
    };

    assert_eq!(day.runoff_partition, expected_state);
    assert_eq!(day.water.infiltration_m.to_bits(), 0.25_f64.to_bits());
    assert_eq!(day.water.runoff_m.to_bits(), 0.34375_f64.to_bits());
    assert_eq!(
        day.runoff_downstream_operands,
        DirectRunoffDownstreamOperands::from(expected_state)
    );
    assert_eq!(day.runoff_shadow_projection, Some(expected_shadow));
    assert_eq!(report.phase_count, DIRECT_R4A_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4A_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(report.runoff_shadow_projection, expected_shadow);

    assert_r4il_runoff_anti_aliases(expected_state, &day);
}

#[test]
fn r4il_runoff_input_producers_reject_invalid_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut nonfinite_liquid = r4il_seed_day();
    nonfinite_liquid.liquid_input_inputs.liquid_input_handoff_m = f64::NAN;
    assert_eq!(
        nonfinite_liquid
            .run_r4i_liquid_input_span()
            .expect_err("nonfinite liquid handoff should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "liquid_input.liquid_input_handoff_m"
        }
    );

    let mut negative_runon = r4il_seed_day();
    negative_runon.runon_carry_inputs.surface_runon_handoff_m = -0.125;
    assert_eq!(
        negative_runon
            .run_r4j_runon_carry_span()
            .expect_err("negative surface runon should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "runon_carry.surface_runon_handoff_m"
        }
    );

    let mut negative_infiltration = r4il_seed_day();
    negative_infiltration
        .infiltration_depression_inputs
        .cumulative_infiltration_handoff_m = -0.25;
    assert_eq!(
        negative_infiltration
            .run_r4k_infiltration_depression_span()
            .expect_err("negative infiltration should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "infiltration_depression.cumulative_infiltration_handoff_m"
        }
    );

    let mut nonfinite_saturation = r4il_seed_day();
    nonfinite_saturation
        .saturation_addback_inputs
        .surface_saturation_runoff_handoff_m = f64::INFINITY;
    assert_eq!(
        nonfinite_saturation
            .run_r4l_saturation_addback_span()
            .expect_err("nonfinite saturation addback should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "saturation_addback.surface_saturation_runoff_handoff_m"
        }
    );
}

#[test]
fn r4a_runoff_partition_rejects_missing_r4il_upstreams_and_invalid_values() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut missing_liquid = r4il_seed_day();
    assert_eq!(
        missing_liquid
            .run_r4a_runoff_partition_span()
            .expect_err("R4A should require R4I liquid input"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4I liquid input"
        }
    );

    let mut missing_runon = r4il_seed_day();
    missing_runon
        .run_r4i_liquid_input_span()
        .expect("R4I should pass before missing R4J probe");
    assert_eq!(
        missing_runon
            .run_r4a_runoff_partition_span()
            .expect_err("R4A should require R4J runon/carry"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4J runon/carry"
        }
    );

    let mut missing_infiltration = r4il_seed_day();
    missing_infiltration
        .run_r4i_liquid_input_span()
        .expect("R4I should pass before missing R4K probe");
    missing_infiltration
        .run_r4j_runon_carry_span()
        .expect("R4J should pass before missing R4K probe");
    assert_eq!(
        missing_infiltration
            .run_r4a_runoff_partition_span()
            .expect_err("R4A should require R4K infiltration/depression"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4K infiltration/depression"
        }
    );

    let mut missing_saturation = r4il_seed_day();
    missing_saturation
        .run_r4i_liquid_input_span()
        .expect("R4I should pass before missing R4L probe");
    missing_saturation
        .run_r4j_runon_carry_span()
        .expect("R4J should pass before missing R4L probe");
    missing_saturation
        .run_r4k_infiltration_depression_span()
        .expect("R4K should pass before missing R4L probe");
    assert_eq!(
        missing_saturation
            .run_r4a_runoff_partition_span()
            .expect_err("R4A should require R4L saturation addback"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4L saturation addback"
        }
    );

    let mut negative_mutation = r4a_ready_day();
    negative_mutation
        .runoff_partition_inputs
        .depression_storage_delta_m = -0.125;
    assert_eq!(
        negative_mutation
            .run_r4a_runoff_partition_span()
            .expect_err("negative mutated depression storage should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "runoff_partition.depression_storage_delta_m"
        }
    );

    let mut overdraw = r4il_seed_day();
    overdraw.liquid_input_inputs.liquid_input_handoff_m = 0.125;
    overdraw
        .infiltration_depression_inputs
        .cumulative_infiltration_handoff_m = 0.25;
    run_r4il_producers(&mut overdraw);
    assert_eq!(
        overdraw
            .run_r4a_runoff_partition_span()
            .expect_err("overdrawn partition runoff should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "runoff_partition.partition_runoff_m"
        }
    );

    let mut runoff_overflow = r4il_seed_day();
    runoff_overflow.liquid_input_inputs.liquid_input_handoff_m = f64::MAX;
    runoff_overflow
        .saturation_addback_inputs
        .surface_saturation_runoff_handoff_m = f64::MAX;
    run_r4il_producers(&mut runoff_overflow);
    assert_eq!(
        runoff_overflow
            .run_r4a_runoff_partition_span()
            .expect_err("overflowed q runoff should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "runoff_partition.q_runoff_m"
        }
    );
}

fn r4a_ready_day() -> DirectDayFrame {
    let mut day = r4il_seed_day();
    run_r4il_producers(&mut day);
    day
}

fn r4il_seed_day() -> DirectDayFrame {
    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.75;
    day.publication.runoff_m = 0.21875;
    day.transfer.surface_carry_m[0] = 0.03125;
    day.transfer.lateral_carry_m[0] = 0.015_625;
    day.water_ledger.diagnostic_residual_m = 0.09375;
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
    day
}

fn run_r4il_producers(day: &mut DirectDayFrame) {
    day.run_r4i_liquid_input_span()
        .expect("R4I upstream span should pass before R4A");
    day.run_r4j_runon_carry_span()
        .expect("R4J upstream span should pass before R4A");
    day.run_r4k_infiltration_depression_span()
        .expect("R4K upstream span should pass before R4A");
    day.run_r4l_saturation_addback_span()
        .expect("R4L upstream span should pass before R4A");
}

fn assert_r4il_runoff_anti_aliases(
    expected_state: DirectRunoffPartitionState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.liquid_input_m.to_bits(),
        day.forcing.precipitation_m.to_bits()
    );
    assert_ne!(
        expected_state.runon_input_m.to_bits(),
        day.runon_carry.subsurface_carry_m.to_bits()
    );
    assert_ne!(
        expected_state.runon_input_m.to_bits(),
        day.transfer.lateral_carry_m[0].to_bits()
    );
    assert_ne!(
        expected_state.cumulative_infiltration_m.to_bits(),
        expected_state.depression_storage_delta_m.to_bits()
    );
    assert_ne!(
        expected_state.cumulative_infiltration_m.to_bits(),
        expected_state.surface_saturation_runoff_m.to_bits()
    );
    assert_ne!(
        expected_state.q_runoff_m.to_bits(),
        day.publication.runoff_m.to_bits()
    );
    assert_ne!(
        expected_state.q_runoff_m.to_bits(),
        day.water_ledger.diagnostic_residual_m.to_bits()
    );
}
