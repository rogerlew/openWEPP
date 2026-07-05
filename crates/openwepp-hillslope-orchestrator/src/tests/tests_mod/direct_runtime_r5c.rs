use super::direct_runtime_test_lock;
use crate::{
    DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT, DIRECT_R5C_DECOMPOSITION_SPAN,
    DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT, DIRECT_R5C_RESIDUE_PARTITION_SPAN,
    DirectDayFrame, DirectDecompositionAction, DirectDecompositionActiveContext,
    DirectDecompositionDownstreamOperands, DirectDecompositionInputs,
    DirectDecompositionShadowProjection, DirectDecompositionState, DirectPhaseKind,
    DirectResiduePartitionDownstreamOperands, DirectResiduePartitionInputs,
    DirectResiduePartitionShadowProjection, DirectResiduePartitionState, DirectRunIdentity,
    DirectRuntimeError, reset_direct_runtime_audit_counters, residue_ground_cover_fraction,
};

const EPS: f64 = 1.0e-12;

#[test]
fn r5c_decomposition_phase_computes_mutates_downstream_and_shadow_projects_annual_cut() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R5C_DECOMPOSITION_SPAN,
        [DirectPhaseKind::DecompositionTransition]
    );
    assert_eq!(
        &[
            DirectPhaseKind::StorageBounds,
            DirectPhaseKind::DecompositionTransition,
            DirectPhaseKind::ResiduePartitionTransition,
        ],
        &DirectPhaseKind::ORDERED[1..4]
    );

    let mut day = r5c_day_after_storage_bounds();
    let inputs = annual_cut_inputs();
    day.decomposition_inputs = inputs;

    let report = day
        .run_r5c_decomposition_phase()
        .expect("valid annual/fallow R5C decomposition should execute");
    let expected_state = expected_annual_cut_state(inputs);
    let expected_operands = DirectDecompositionDownstreamOperands::from(expected_state);
    let expected_shadow = DirectDecompositionShadowProjection {
        lane_index: 0,
        day_index: 0,
        active_context: inputs.active_context,
        active_action: inputs.active_action,
        surface_residue_kg_m2: expected_state.surface_residue_kg_m2,
        root_residue_kg_m2: expected_state.root_residue_kg_m2,
        surface_litter_input_kg_m2: expected_state.surface_litter_input_kg_m2,
        residue_depth_m: expected_state.residue_depth_m,
        environment_index: expected_state.environment_index,
        surface_decay_factor: expected_state.surface_decay_factor,
        root_decay_factor: expected_state.root_decay_factor,
    };

    assert_eq!(day.decomposition_inputs, inputs);
    assert_decomposition_state_close(day.decomposition, expected_state);
    assert_decomposition_operands_close(day.decomposition_downstream_operands, expected_operands);
    assert_decomposition_shadow_close(day.decomposition_shadow_projection, expected_shadow);
    assert_eq!(
        report.phase_count,
        DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT
    );
    assert_eq!(report.phase_entry_count, 1);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_decomposition_shadow_value_close(
        report.decomposition_shadow_projection,
        expected_shadow,
    );

    assert_r5c_decomposition_anti_aliases(expected_state, &day);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.day_frame_constructions, 1);
    assert_eq!(audit.phase_span_runs, 3);
    assert_eq!(audit.direct_phase_entries, 3);
    assert_eq!(audit.direct_compute_operations, 3);
    assert_eq!(audit.direct_state_mutations, 3);
    assert_eq!(audit.downstream_operand_productions, 3);
    assert_eq!(audit.shadow_projections, 3);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r5c_decomposition_phase_supports_perennial_grazing_and_zero_decay() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = r5c_day_after_storage_bounds();
    let inputs = DirectDecompositionInputs {
        interrill_ground_seed_kg_m2: 0.0,
        rill_ground_seed_kg_m2: 0.0,
        residue_cover_factor: 0.0,
        active_context: DirectDecompositionActiveContext::Perennial {
            active_slot_index: 2,
            active_crop_slot_index: 1,
            runtime_day_of_year: 210,
        },
        active_action: DirectDecompositionAction::Grazing,
        residue_type_selector: 3.0,
        surface_residue_seed_kg_m2: 0.6,
        root_residue_seed_kg_m2: 0.2,
        surface_litter_input_kg_m2: 0.0,
        residue_depth_conversion_m_per_kg_m2: 0.0,
        temperature_max_c: 5.0,
        temperature_min_c: 1.0,
        precipitation_m: 0.01,
        water_stress_fraction: 1.0,
        surface_decomposition_rate: 0.0,
        root_decomposition_rate: 0.0,
        burn_surface_fraction: 0.0,
        remove_surface_fraction: 0.0,
        cut_transfer_fraction: 0.0,
        grazing_digest_fraction: 0.4,
    };
    day.decomposition_inputs = inputs;

    day.run_r5c_decomposition_phase()
        .expect("valid perennial grazing decomposition should execute");

    assert_close(day.decomposition.surface_decay_factor, 1.0);
    assert_close(day.decomposition.root_decay_factor, 1.0);
    assert_close(day.decomposition.surface_residue_kg_m2, 0.36);
    assert_close(day.decomposition.root_residue_kg_m2, 0.2);
    assert_eq!(
        day.decomposition_shadow_projection
            .expect("perennial decomposition should project")
            .active_context,
        inputs.active_context
    );
}

#[test]
fn r5c_decomposition_phase_rejects_missing_upstream_context_and_invalid_domains() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut missing_upstream_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    missing_upstream_day.decomposition_inputs = annual_cut_inputs();
    assert_eq!(
        missing_upstream_day
            .run_r5c_decomposition_phase()
            .expect_err("R5C decomposition should require R5B storage bounds"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R5B storage bounds phase"
        }
    );

    let mut missing_context_day = r5c_day_after_storage_bounds();
    missing_context_day.decomposition_inputs = DirectDecompositionInputs {
        active_context: DirectDecompositionActiveContext::Missing,
        ..annual_cut_inputs()
    };
    assert_eq!(
        missing_context_day
            .run_r5c_decomposition_phase()
            .expect_err("missing active context should fail closed"),
        DirectRuntimeError::DirectDomainViolation {
            field: "decomposition.active_context"
        }
    );

    let mut ambiguous_context_day = r5c_day_after_storage_bounds();
    ambiguous_context_day.decomposition_inputs = DirectDecompositionInputs {
        active_context: DirectDecompositionActiveContext::Ambiguous,
        ..annual_cut_inputs()
    };
    assert_eq!(
        ambiguous_context_day
            .run_r5c_decomposition_phase()
            .expect_err("ambiguous active context should fail closed"),
        DirectRuntimeError::DirectDomainViolation {
            field: "decomposition.active_context"
        }
    );

    let mut invalid_action_day = r5c_day_after_storage_bounds();
    invalid_action_day.decomposition_inputs = DirectDecompositionInputs {
        active_action: DirectDecompositionAction::Grazing,
        ..annual_cut_inputs()
    };
    assert_eq!(
        invalid_action_day
            .run_r5c_decomposition_phase()
            .expect_err("annual branch should reject grazing action"),
        DirectRuntimeError::DirectDomainViolation {
            field: "decomposition.active_action"
        }
    );

    let mut negative_pool_day = r5c_day_after_storage_bounds();
    negative_pool_day.decomposition_inputs = DirectDecompositionInputs {
        surface_residue_seed_kg_m2: -0.1,
        ..annual_cut_inputs()
    };
    assert_eq!(
        negative_pool_day
            .run_r5c_decomposition_phase()
            .expect_err("negative seed pool should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "decomposition.surface_residue_seed_kg_m2"
        }
    );

    let mut invalid_fraction_day = r5c_day_after_storage_bounds();
    invalid_fraction_day.decomposition_inputs = DirectDecompositionInputs {
        cut_transfer_fraction: 1.25,
        ..annual_cut_inputs()
    };
    assert_eq!(
        invalid_fraction_day
            .run_r5c_decomposition_phase()
            .expect_err("invalid event fraction should fail closed"),
        DirectRuntimeError::DirectDomainViolation {
            field: "decomposition.cut_transfer_fraction"
        }
    );
}

#[test]
fn r5c_residue_partition_consumes_decomposition_and_shadow_projects() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R5C_RESIDUE_PARTITION_SPAN,
        [DirectPhaseKind::ResiduePartitionTransition]
    );

    let mut day = r5c_day_after_storage_bounds();
    let mut inputs = annual_cut_inputs();
    // INV-RESIDUE-020: give the ground-cover pathway real operands so the
    // computed covers are nonzero and the composite blend is meaningful.
    inputs.interrill_ground_seed_kg_m2 = 0.2;
    inputs.rill_ground_seed_kg_m2 = 0.1;
    inputs.residue_cover_factor = 3.5;
    day.decomposition_inputs = inputs;
    day.run_r5c_decomposition_phase()
        .expect("decomposition should execute before residue partition");
    day.residue_partition_inputs = DirectResiduePartitionInputs {
        rescov_interrill_weight: 0.4,
        standing_residue_kg_m2: 0.12,
        flat_residue_offset_kg_m2: 0.07,
        buried_residue_kg_m2: 0.03,
        cover_fraction: 0.41,
    };

    let report = day
        .run_r5c_residue_partition_phase()
        .expect("valid residue partition should execute");
    let flat_residue_kg_m2 = 0.07 + expected_annual_cut_state(inputs).surface_residue_kg_m2;
    let root_residue_kg_m2 = expected_annual_cut_state(inputs).root_residue_kg_m2;
    let total_residue_kg_m2 = 0.12 + flat_residue_kg_m2 + 0.03 + root_residue_kg_m2;
    // INV-RESIDUE-020: the partition covers derive from the evolved
    // ground pools (the decomposition phase above owns their evolution;
    // this test checks the partition CONSUMES them) and the composite is
    // the `rescov` blend — the input `cover_fraction` pass-through is
    // superseded.
    let expected_interrill_cover =
        residue_ground_cover_fraction(3.5, day.decomposition.interrill_ground_residue_kg_m2)
            .expect("interrill cover");
    let expected_rill_cover =
        residue_ground_cover_fraction(3.5, day.decomposition.rill_ground_residue_kg_m2)
            .expect("rill cover");
    assert!(expected_interrill_cover > 0.0 && expected_rill_cover > 0.0);
    let expected_state = DirectResiduePartitionState {
        interrill_cover_fraction: expected_interrill_cover,
        rill_cover_fraction: expected_rill_cover,
        standing_residue_kg_m2: 0.12,
        flat_residue_kg_m2,
        buried_residue_kg_m2: 0.03,
        root_residue_kg_m2,
        total_residue_kg_m2,
        cover_fraction: 0.4 * expected_interrill_cover + 0.6 * expected_rill_cover,
    };
    let expected_operands = DirectResiduePartitionDownstreamOperands::from(expected_state);
    let expected_shadow = DirectResiduePartitionShadowProjection {
        lane_index: 0,
        day_index: 0,
        standing_residue_kg_m2: 0.12,
        flat_residue_kg_m2,
        buried_residue_kg_m2: 0.03,
        root_residue_kg_m2,
        total_residue_kg_m2,
        cover_fraction: 0.4 * expected_interrill_cover + 0.6 * expected_rill_cover,
    };

    assert_residue_state_close(day.residue_partition, expected_state);
    assert_residue_operands_close(day.residue_partition_downstream_operands, expected_operands);
    assert_residue_shadow_close(day.residue_partition_shadow_projection, expected_shadow);
    assert_eq!(
        report.phase_count,
        DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT
    );
    assert_eq!(report.phase_entry_count, 1);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_residue_shadow_value_close(report.residue_partition_shadow_projection, expected_shadow);

    assert_r5c_residue_partition_anti_aliases(expected_state, &day);
}

#[test]
fn r5c_residue_partition_rejects_missing_decomposition_and_invalid_inputs() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut missing_decomposition_day = r5c_day_after_storage_bounds();
    assert_eq!(
        missing_decomposition_day
            .run_r5c_residue_partition_phase()
            .expect_err("residue partition should require decomposition"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R5C decomposition transition"
        }
    );

    let mut negative_standing_day = r5c_day_after_decomposition();
    negative_standing_day.residue_partition_inputs = DirectResiduePartitionInputs {
        rescov_interrill_weight: 0.0,
        standing_residue_kg_m2: -0.1,
        flat_residue_offset_kg_m2: 0.0,
        buried_residue_kg_m2: 0.0,
        cover_fraction: 0.0,
    };
    assert_eq!(
        negative_standing_day
            .run_r5c_residue_partition_phase()
            .expect_err("negative standing residue should fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "residue_partition.standing_residue_kg_m2"
        }
    );

    let mut invalid_cover_day = r5c_day_after_decomposition();
    invalid_cover_day.residue_partition_inputs = DirectResiduePartitionInputs {
        rescov_interrill_weight: 0.0,
        standing_residue_kg_m2: 0.0,
        flat_residue_offset_kg_m2: 0.0,
        buried_residue_kg_m2: 0.0,
        cover_fraction: 1.25,
    };
    assert_eq!(
        invalid_cover_day
            .run_r5c_residue_partition_phase()
            .expect_err("invalid cover should fail closed"),
        DirectRuntimeError::DirectDomainViolation {
            field: "residue_partition.cover_fraction"
        }
    );

    // The rescov weight fails closed at the same boundary — out-of-range
    // and NaN both reject (guarding against a return of the silent
    // clamp-canonicalization path).
    for bad_weight in [1.5, -0.1, f64::NAN] {
        let mut invalid_weight_day = r5c_day_after_decomposition();
        invalid_weight_day.residue_partition_inputs = DirectResiduePartitionInputs {
            rescov_interrill_weight: bad_weight,
            standing_residue_kg_m2: 0.0,
            flat_residue_offset_kg_m2: 0.0,
            buried_residue_kg_m2: 0.0,
            cover_fraction: 0.0,
        };
        assert_eq!(
            invalid_weight_day
                .run_r5c_residue_partition_phase()
                .expect_err("invalid rescov weight should fail closed"),
            DirectRuntimeError::DirectDomainViolation {
                field: "residue_partition.rescov_interrill_weight"
            }
        );
    }
}

fn r5c_day_after_decomposition() -> DirectDayFrame {
    let mut day = r5c_day_after_storage_bounds();
    day.decomposition_inputs = annual_cut_inputs();
    day.run_r5c_decomposition_phase()
        .expect("R5C decomposition should execute for residue partition fixture");
    day
}

fn r5c_day_after_storage_bounds() -> DirectDayFrame {
    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.125;
    day.water.soil_water_m = 1.25;
    day.storage_reconciliation_inputs.closure_tolerance_m = 1.0e-12;
    day.run_r5b_normalization_phase()
        .expect("R5B normalization should pass before R5C");
    day.run_r5b_storage_bounds_phase()
        .expect("R5B storage bounds should pass before R5C");
    day
}

fn annual_cut_inputs() -> DirectDecompositionInputs {
    DirectDecompositionInputs {
        interrill_ground_seed_kg_m2: 0.0,
        rill_ground_seed_kg_m2: 0.0,
        residue_cover_factor: 0.0,
        active_context: DirectDecompositionActiveContext::AnnualOrFallow {
            active_slot_index: 1,
            active_crop_slot_index: 2,
            runtime_day_of_year: 123,
        },
        active_action: DirectDecompositionAction::Cut,
        residue_type_selector: 4.0,
        surface_residue_seed_kg_m2: 0.8,
        root_residue_seed_kg_m2: 0.3,
        surface_litter_input_kg_m2: 0.0,
        residue_depth_conversion_m_per_kg_m2: 0.0,
        temperature_max_c: 22.0,
        temperature_min_c: 10.0,
        precipitation_m: 0.002,
        water_stress_fraction: 0.5,
        surface_decomposition_rate: 0.2,
        root_decomposition_rate: 0.1,
        burn_surface_fraction: 0.1,
        remove_surface_fraction: 0.2,
        cut_transfer_fraction: 0.25,
        grazing_digest_fraction: 0.3,
    }
}

fn expected_annual_cut_state(inputs: DirectDecompositionInputs) -> DirectDecompositionState {
    let tave = f64::midpoint(inputs.temperature_max_c, inputs.temperature_min_c);
    let t1 = (tave + 6.1).powi(2);
    let temperature_factor = t1 * (2.0 * 1528.81 - t1) / 1528.81_f64.powi(2);
    let surface_water_factor = inputs.precipitation_m / 0.004;
    let flat_water_factor = inputs.water_stress_fraction;
    let environment_index = temperature_factor.min(flat_water_factor);
    let surface_decay_factor = (-environment_index * inputs.surface_decomposition_rate).exp();
    let root_decay_factor = (-environment_index * inputs.root_decomposition_rate).exp();
    let surface_after_decay = inputs.surface_residue_seed_kg_m2 * surface_decay_factor;
    let root_after_decay = inputs.root_residue_seed_kg_m2 * root_decay_factor;
    let cut_transfer = surface_after_decay * inputs.cut_transfer_fraction;
    DirectDecompositionState {
        interrill_ground_residue_kg_m2: 0.0,
        rill_ground_residue_kg_m2: 0.0,
        residue_cover_factor: 0.0,
        active_context: inputs.active_context,
        active_action: inputs.active_action,
        residue_type_selector: inputs.residue_type_selector,
        surface_residue_seed_kg_m2: inputs.surface_residue_seed_kg_m2,
        root_residue_seed_kg_m2: inputs.root_residue_seed_kg_m2,
        surface_litter_input_kg_m2: inputs.surface_litter_input_kg_m2,
        residue_depth_conversion_m_per_kg_m2: inputs.residue_depth_conversion_m_per_kg_m2,
        temperature_factor,
        surface_water_factor,
        flat_water_factor,
        environment_index,
        surface_decay_factor,
        root_decay_factor,
        surface_residue_kg_m2: surface_after_decay - cut_transfer,
        root_residue_kg_m2: root_after_decay + cut_transfer,
        residue_depth_m: (surface_after_decay - cut_transfer)
            * inputs.residue_depth_conversion_m_per_kg_m2,
    }
}

fn assert_r5c_decomposition_anti_aliases(
    expected_state: DirectDecompositionState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.surface_residue_kg_m2.to_bits(),
        expected_state.surface_residue_seed_kg_m2.to_bits()
    );
    assert_ne!(
        expected_state.root_residue_kg_m2.to_bits(),
        expected_state.root_residue_seed_kg_m2.to_bits()
    );
    assert_ne!(
        expected_state.surface_residue_kg_m2.to_bits(),
        day.storage_bounds.storage_bounded_m.to_bits()
    );
    assert_ne!(
        expected_state.environment_index.to_bits(),
        day.forcing.precipitation_m.to_bits()
    );
    assert_ne!(
        expected_state.surface_decay_factor.to_bits(),
        day.decomposition_inputs.cut_transfer_fraction.to_bits()
    );
}

fn assert_r5c_residue_partition_anti_aliases(
    expected_state: DirectResiduePartitionState,
    day: &DirectDayFrame,
) {
    assert_ne!(
        expected_state.flat_residue_kg_m2.to_bits(),
        expected_state.standing_residue_kg_m2.to_bits()
    );
    assert_ne!(
        expected_state.flat_residue_kg_m2.to_bits(),
        expected_state.buried_residue_kg_m2.to_bits()
    );
    assert_ne!(
        expected_state.flat_residue_kg_m2.to_bits(),
        expected_state.total_residue_kg_m2.to_bits()
    );
    assert_ne!(
        expected_state.total_residue_kg_m2.to_bits(),
        day.storage_bounds.storage_bounded_m.to_bits()
    );
    assert_ne!(
        expected_state.cover_fraction.to_bits(),
        day.publication.runoff_m.to_bits()
    );
}

fn assert_decomposition_state_close(
    observed: DirectDecompositionState,
    expected: DirectDecompositionState,
) {
    assert_eq!(observed.active_context, expected.active_context);
    assert_eq!(observed.active_action, expected.active_action);
    assert_close(
        observed.residue_type_selector,
        expected.residue_type_selector,
    );
    assert_close(
        observed.surface_residue_seed_kg_m2,
        expected.surface_residue_seed_kg_m2,
    );
    assert_close(
        observed.root_residue_seed_kg_m2,
        expected.root_residue_seed_kg_m2,
    );
    assert_close(
        observed.surface_litter_input_kg_m2,
        expected.surface_litter_input_kg_m2,
    );
    assert_close(
        observed.residue_depth_conversion_m_per_kg_m2,
        expected.residue_depth_conversion_m_per_kg_m2,
    );
    assert_close(observed.temperature_factor, expected.temperature_factor);
    assert_close(observed.surface_water_factor, expected.surface_water_factor);
    assert_close(observed.flat_water_factor, expected.flat_water_factor);
    assert_close(observed.environment_index, expected.environment_index);
    assert_close(observed.surface_decay_factor, expected.surface_decay_factor);
    assert_close(observed.root_decay_factor, expected.root_decay_factor);
    assert_close(
        observed.surface_residue_kg_m2,
        expected.surface_residue_kg_m2,
    );
    assert_close(observed.root_residue_kg_m2, expected.root_residue_kg_m2);
    assert_close(observed.residue_depth_m, expected.residue_depth_m);
}

fn assert_decomposition_operands_close(
    observed: DirectDecompositionDownstreamOperands,
    expected: DirectDecompositionDownstreamOperands,
) {
    assert_eq!(observed.active_context, expected.active_context);
    assert_eq!(observed.active_action, expected.active_action);
    assert_close(
        observed.residue_type_selector,
        expected.residue_type_selector,
    );
    assert_close(
        observed.surface_residue_kg_m2,
        expected.surface_residue_kg_m2,
    );
    assert_close(observed.root_residue_kg_m2, expected.root_residue_kg_m2);
    assert_close(
        observed.surface_litter_input_kg_m2,
        expected.surface_litter_input_kg_m2,
    );
    assert_close(observed.residue_depth_m, expected.residue_depth_m);
    assert_close(observed.temperature_factor, expected.temperature_factor);
    assert_close(observed.surface_water_factor, expected.surface_water_factor);
    assert_close(observed.flat_water_factor, expected.flat_water_factor);
    assert_close(observed.environment_index, expected.environment_index);
    assert_close(observed.surface_decay_factor, expected.surface_decay_factor);
    assert_close(observed.root_decay_factor, expected.root_decay_factor);
}

fn assert_decomposition_shadow_close(
    observed: Option<DirectDecompositionShadowProjection>,
    expected: DirectDecompositionShadowProjection,
) {
    assert_decomposition_shadow_value_close(
        observed.expect("decomposition should produce shadow projection"),
        expected,
    );
}

fn assert_decomposition_shadow_value_close(
    observed: DirectDecompositionShadowProjection,
    expected: DirectDecompositionShadowProjection,
) {
    assert_eq!(observed.lane_index, expected.lane_index);
    assert_eq!(observed.day_index, expected.day_index);
    assert_eq!(observed.active_context, expected.active_context);
    assert_eq!(observed.active_action, expected.active_action);
    assert_close(
        observed.surface_residue_kg_m2,
        expected.surface_residue_kg_m2,
    );
    assert_close(observed.root_residue_kg_m2, expected.root_residue_kg_m2);
    assert_close(
        observed.surface_litter_input_kg_m2,
        expected.surface_litter_input_kg_m2,
    );
    assert_close(observed.residue_depth_m, expected.residue_depth_m);
    assert_close(observed.environment_index, expected.environment_index);
    assert_close(observed.surface_decay_factor, expected.surface_decay_factor);
    assert_close(observed.root_decay_factor, expected.root_decay_factor);
}

fn assert_residue_state_close(
    observed: DirectResiduePartitionState,
    expected: DirectResiduePartitionState,
) {
    assert_close(
        observed.standing_residue_kg_m2,
        expected.standing_residue_kg_m2,
    );
    assert_close(observed.flat_residue_kg_m2, expected.flat_residue_kg_m2);
    assert_close(observed.buried_residue_kg_m2, expected.buried_residue_kg_m2);
    assert_close(observed.root_residue_kg_m2, expected.root_residue_kg_m2);
    assert_close(observed.total_residue_kg_m2, expected.total_residue_kg_m2);
    assert_close(observed.cover_fraction, expected.cover_fraction);
}

fn assert_residue_operands_close(
    observed: DirectResiduePartitionDownstreamOperands,
    expected: DirectResiduePartitionDownstreamOperands,
) {
    assert_close(
        observed.standing_residue_kg_m2,
        expected.standing_residue_kg_m2,
    );
    assert_close(observed.flat_residue_kg_m2, expected.flat_residue_kg_m2);
    assert_close(observed.buried_residue_kg_m2, expected.buried_residue_kg_m2);
    assert_close(observed.root_residue_kg_m2, expected.root_residue_kg_m2);
    assert_close(observed.total_residue_kg_m2, expected.total_residue_kg_m2);
    assert_close(observed.cover_fraction, expected.cover_fraction);
}

fn assert_residue_shadow_close(
    observed: Option<DirectResiduePartitionShadowProjection>,
    expected: DirectResiduePartitionShadowProjection,
) {
    assert_residue_shadow_value_close(
        observed.expect("residue partition should produce shadow projection"),
        expected,
    );
}

fn assert_residue_shadow_value_close(
    observed: DirectResiduePartitionShadowProjection,
    expected: DirectResiduePartitionShadowProjection,
) {
    assert_eq!(observed.lane_index, expected.lane_index);
    assert_eq!(observed.day_index, expected.day_index);
    assert_close(
        observed.standing_residue_kg_m2,
        expected.standing_residue_kg_m2,
    );
    assert_close(observed.flat_residue_kg_m2, expected.flat_residue_kg_m2);
    assert_close(observed.buried_residue_kg_m2, expected.buried_residue_kg_m2);
    assert_close(observed.root_residue_kg_m2, expected.root_residue_kg_m2);
    assert_close(observed.total_residue_kg_m2, expected.total_residue_kg_m2);
    assert_close(observed.cover_fraction, expected.cover_fraction);
}

fn assert_close(observed: f64, expected: f64) {
    assert!(
        (observed - expected).abs() <= EPS,
        "observed {observed}, expected {expected}"
    );
}

#[test]
fn ground_cover_seed_round_trips_the_declared_cover() {
    // GAP-SED-009 closure identity: seeding a pool by the `init1.for`
    // inverse and re-deriving cover by the `covcal.for` forward form
    // reproduces the declared cover exactly (the 0.999 clamp bounds it).
    let cover_factor = 3.5;
    for declared in [0.10_f64, 0.5, 0.85, 0.999] {
        let pool = (1.0 - declared).ln() / -cover_factor;
        let cover = residue_ground_cover_fraction(cover_factor, pool).expect("covcal forward form");
        assert!(
            (cover - declared).abs() < 1.0e-12,
            "seed/derive round trip must be exact ({cover} vs {declared})"
        );
    }
    // Zero pool and zero factor both yield zero cover (pre-fix behavior
    // for managements that declare no cover).
    assert_eq!(residue_ground_cover_fraction(3.5, 0.0).unwrap(), 0.0);
    assert_eq!(residue_ground_cover_fraction(0.0, 5.0).unwrap(), 0.0);
    // Clamp: enormous mass caps at the legacy 0.999.
    assert_eq!(residue_ground_cover_fraction(3.5, 1.0e6).unwrap(), 0.999);
}

#[test]
fn ground_pools_hold_constant_without_decay_or_litter() {
    // The no-decomp forest scenario: decay factor 1 and no litter input
    // must carry the pools (and therefore the covers) unchanged — the
    // legacy behavior that holds declared covers constant for decades.
    let inputs = DirectDecompositionInputs {
        interrill_ground_seed_kg_m2: 0.542,
        rill_ground_seed_kg_m2: 0.542,
        residue_cover_factor: 3.5,
        active_context: DirectDecompositionActiveContext::AnnualOrFallow {
            active_slot_index: 1,
            active_crop_slot_index: 1,
            runtime_day_of_year: 210,
        },
        active_action: DirectDecompositionAction::None,
        residue_type_selector: 1.0,
        surface_residue_seed_kg_m2: 0.1,
        root_residue_seed_kg_m2: 0.1,
        surface_litter_input_kg_m2: 0.0,
        residue_depth_conversion_m_per_kg_m2: 0.0,
        temperature_max_c: 5.0,
        temperature_min_c: 1.0,
        precipitation_m: 0.01,
        water_stress_fraction: 1.0,
        surface_decomposition_rate: 0.0,
        root_decomposition_rate: 0.0,
        burn_surface_fraction: 0.0,
        remove_surface_fraction: 0.0,
        cut_transfer_fraction: 0.0,
        grazing_digest_fraction: 0.0,
    };
    let state = inputs.compute_state().expect("no-decay state");
    assert!(
        (state.interrill_ground_residue_kg_m2 - 0.542).abs() < 1.0e-12
            && (state.rill_ground_residue_kg_m2 - 0.542).abs() < 1.0e-12,
        "zero decay + zero litter must hold the ground pools"
    );
}
