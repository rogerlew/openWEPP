use super::direct_runtime_test_lock;
use crate::{
    DIRECT_R4PQZ_HYDROLOGY_PROJECTION_SPAN, DIRECT_R4PQZ_PHASE_SPAN_COUNT, DirectDayFrame,
    DirectEvapotranspirationComputeShadowProjection, DirectHydrologyProjectionInputs,
    DirectHydrologyProjectionShadowProjection, DirectHydrologyProjectionState,
    DirectPercolationShadowProjection, DirectPhaseKind, DirectRunIdentity,
    DirectRunoffShadowProjection, DirectRunonCarryShadowProjection, DirectRuntimeError,
    DirectSnowCouplingShadowProjection, DirectStorageShadowProjection,
    DirectSubsurfaceComputeShadowProjection, DirectSubsurfaceLayerInputs,
    DirectSubsurfaceLayerState, reset_direct_runtime_audit_counters,
};

#[test]
fn r4pqz_projection_recomputes_storage_from_final_layers_and_projects_direct_operands() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4PQZ_HYDROLOGY_PROJECTION_SPAN,
        [
            DirectPhaseKind::StorageReconciliation,
            DirectPhaseKind::ClosureDiagnostics
        ]
    );

    let mut day = projectable_day();
    let report = day
        .run_r4pqz_hydrology_projection_span()
        .expect("valid R4P/Q/Z projection should execute");

    let expected = expected_projection_state();
    assert_eq!(day.hydrology_projection, expected);
    assert_eq!(
        day.hydrology_projection_downstream_operands
            .aggregate_storage_from_layers_m
            .to_bits(),
        expected.aggregate_storage_from_layers_m.to_bits()
    );
    assert_eq!(
        day.hydrology_projection_shadow_projection,
        Some(expected_projection_shadow())
    );
    assert_eq!(report.phase_count, DIRECT_R4PQZ_PHASE_SPAN_COUNT);
    assert_eq!(
        report.phase_entry_count,
        DIRECT_R4PQZ_PHASE_SPAN_COUNT as u64
    );
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_eq!(
        report.hydrology_projection_shadow_projection,
        expected_projection_shadow()
    );

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.phase_span_runs, 1);
    assert_eq!(
        audit.direct_phase_entries,
        DIRECT_R4PQZ_PHASE_SPAN_COUNT as u64
    );
    assert_eq!(audit.direct_compute_operations, 1);
    assert_eq!(audit.direct_state_mutations, 1);
    assert_eq!(audit.downstream_operand_productions, 1);
    assert_eq!(audit.shadow_projections, 1);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r4pqz_projection_is_shadow_only_and_anti_aliases_publication_values() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = projectable_day();
    day.publication.runoff_m = 0.999;
    day.publication.evapotranspiration_m = 0.888;
    day.publication.drainage_m = 0.777;
    day.publication.lateral_flow_m = 0.666;

    day.run_r4pqz_hydrology_projection_span()
        .expect("publication comparison values should not become authoritative");

    let projection = day
        .hydrology_projection_shadow_projection
        .expect("projection shadow should exist");
    assert_eq!(projection.q_runoff_m.to_bits(), 0.125_f64.to_bits());
    assert_ne!(
        projection.q_runoff_m.to_bits(),
        projection.publication_runoff_m.to_bits()
    );
    assert_eq!(
        projection.evapotranspiration_m.to_bits(),
        0.09375_f64.to_bits()
    );
    assert_ne!(
        projection.evapotranspiration_m.to_bits(),
        projection.publication_evapotranspiration_m.to_bits()
    );
    assert!(!projection.public_output_cutover);
}

#[test]
fn r4pqz_projection_preserves_mofe_transfer_identity_fields() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = projectable_day_at(17, 41);
    day.runon_carry_shadow_projection = Some(DirectRunonCarryShadowProjection {
        lane_index: 17,
        day_index: 41,
        runon_input_m: 0.1875,
        subsurface_carry_m: 0.046_875,
    });

    day.run_r4pqz_hydrology_projection_span()
        .expect("MOFE-style transfer/carry projection should execute");

    let projection = day
        .hydrology_projection_shadow_projection
        .expect("projection shadow should exist");
    assert_eq!(projection.lane_index, 17);
    assert_eq!(projection.day_index, 41);
    assert_eq!(projection.runon_input_m.to_bits(), 0.1875_f64.to_bits());
    assert_eq!(
        projection.subsurface_carry_m.to_bits(),
        0.046_875_f64.to_bits()
    );
    assert_ne!(
        projection.runon_input_m.to_bits(),
        projection.subsurface_carry_m.to_bits()
    );
}

#[test]
fn r4pqz_projection_fails_closed_on_missing_upstream_and_storage_mismatch() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct identity should construct");
    let mut missing_storage =
        DirectDayFrame::seed(identity, 0, 0).expect("valid day should construct");
    assert_eq!(
        missing_storage
            .run_r4pqz_hydrology_projection_span()
            .expect_err("projection should require R4B"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4B storage reconciliation"
        }
    );

    let mut mismatched = projectable_day();
    mismatched
        .storage_shadow_projection
        .as_mut()
        .expect("storage shadow exists")
        .storage_reconciled_m = 0.5;
    assert_eq!(
        mismatched
            .run_r4pqz_hydrology_projection_span()
            .expect_err("layer aggregate mismatch should fail closed"),
        DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "hydrology_projection.aggregate_storage_delta_m"
        }
    );
}

#[test]
fn r4pqz_projection_rejects_invalid_projection_domains() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut bad_profile = projectable_day();
    bad_profile
        .hydrology_projection_inputs
        .profile_field_capacity_m = Some(-0.125);
    assert_eq!(
        bad_profile
            .run_r4pqz_hydrology_projection_span()
            .expect_err("negative profile capacity placeholder should fail"),
        DirectRuntimeError::NegativeDirectValue {
            field: "hydrology_projection.profile_field_capacity_m"
        }
    );

    let mut bad_layer = projectable_day();
    bad_layer
        .evapotranspiration_compute_shadow_projection
        .as_mut()
        .expect("ET shadow exists")
        .layer_state_after_root_uptake[0]
        .theta_m = f64::NAN;
    assert_eq!(
        bad_layer
            .run_r4pqz_hydrology_projection_span()
            .expect_err("nonfinite final layer storage should fail"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "hydrology_projection.layer.theta_m"
        }
    );
}

fn projectable_day() -> DirectDayFrame {
    projectable_day_at(0, 0)
}

fn projectable_day_at(lane_index: usize, day_index: usize) -> DirectDayFrame {
    let identity =
        DirectRunIdentity::new(7, 2637, 19, 12419).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, lane_index, day_index).expect("valid day should construct");
    day.hydrology_projection_inputs = DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 0.0,
        snow_water_m: 0.03125,
        frozen_soil_water_m: 0.015_625,
        profile_field_capacity_m: Some(0.6),
        profile_wilting_point_m: Some(0.2),
    };
    day.publication.runoff_m = 0.25;
    day.publication.evapotranspiration_m = 0.125;
    day.publication.drainage_m = 0.0625;
    day.publication.lateral_flow_m = 0.03125;
    day.runoff_shadow_projection = Some(DirectRunoffShadowProjection {
        lane_index,
        day_index,
        liquid_input_m: 0.5,
        runon_input_m: 0.03125,
        cumulative_infiltration_m: 0.25,
        depression_storage_delta_m: 0.0625,
        surface_saturation_runoff_m: 0.03125,
        partition_runoff_m: 0.125,
        q_runoff_m: 0.125,
        closure_residual_m: 0.0,
    });
    day.percolation_shadow_projection = Some(DirectPercolationShadowProjection {
        lane_index,
        day_index,
        soil_water_before_m: 0.875,
        soil_water_after_m: 0.8125,
        deep_seepage_m: 0.0625,
        recharge_m: 0.0,
        per_layer_flux_m: vec![0.0625, 0.0],
        layer_state_after: vec![layer(0.5, 0.0625), layer(0.3125, 0.03125)],
    });
    day.subsurface_compute_shadow_projection = Some(DirectSubsurfaceComputeShadowProjection {
        lane_index,
        day_index,
        soil_water_before_m: 0.8125,
        soil_water_after_m: 0.765_625,
        lateral_flow_m: 0.03125,
        tile_drainage_m: 0.015_625,
        subsurface_loss_m: 0.046_875,
        lateral_target_m: 0.03125,
        drainage_target_m: 0.015_625,
        lateral_capacity_m: 0.5,
        hourly_lateral_carry_m: [0.0; 24],
        hourly_saturation_carry_m: [0.0; 24],
        layer_state_after: vec![layer(0.484_375, 0.0625), layer(0.28125, 0.03125)],
        lateral_layer_withdrawal_m: vec![0.015_625, 0.03125],
    });
    day.evapotranspiration_compute_shadow_projection =
        Some(DirectEvapotranspirationComputeShadowProjection {
            lane_index,
            day_index,
            soil_water_before_root_uptake_m: 0.765_625,
            soil_water_after_m: 0.75,
            evapotranspiration_m: 0.09375,
            soil_evaporation_m: 0.03125,
            residue_evaporation_m: 0.015_625,
            plant_transpiration_m: 0.046_875,
            transpiration_demand_m: 0.046_875,
            water_stress: 0.75,
            uptake_potential_m: 0.046_875,
            uptake_actual_m: 0.015_625,
            effective_plant_tolerance: 0.25,
            layer_uptake_potential_m: vec![0.015_625, 0.03125],
            layer_uptake_actual_m: vec![0.0, 0.015_625],
            layer_state_after_root_uptake: vec![layer(0.5, 0.0625), layer(0.25, 0.03125)],
        });
    day.snow_coupling_shadow_projection = Some(DirectSnowCouplingShadowProjection {
        lane_index,
        day_index,
        snow_coupling_m: 0.007_812_5,
    });
    day.storage_shadow_projection = Some(DirectStorageShadowProjection {
        lane_index,
        day_index,
        storage_initial_m: 0.875,
        precip_input_m: 0.25,
        snow_coupling_m: 0.007_812_5,
        q_runoff_m: 0.125,
        evapotranspiration_m: 0.09375,
        deep_seepage_m: 0.0625,
        subsurface_loss_m: 0.046_875,
        storage_reconciled_m: 0.75,
        closure_residual_m: 0.0,
    });
    day.runon_carry_shadow_projection = Some(DirectRunonCarryShadowProjection {
        lane_index,
        day_index,
        runon_input_m: 0.03125,
        subsurface_carry_m: 0.003_906_25,
    });
    day
}

fn expected_projection_state() -> DirectHydrologyProjectionState {
    DirectHydrologyProjectionState {
        aggregate_storage_from_layers_m: 0.75,
        storage_reconciled_m: 0.75,
        aggregate_storage_delta_m: 0.0,
        q_runoff_m: 0.125,
        q_ofe_m: 0.125,
        deep_percolation_m: 0.0625,
        lateral_flow_m: 0.03125,
        tile_drainage_m: 0.015_625,
        subsurface_loss_m: 0.046_875,
        evapotranspiration_m: 0.09375,
        soil_evaporation_m: 0.03125,
        residue_evaporation_m: 0.015_625,
        plant_transpiration_m: 0.046_875,
        water_stress: 0.75,
        snow_frost_coupling_m: 0.007_812_5,
        snow_water_m: 0.03125,
        frozen_soil_water_m: 0.109_375,
        total_soil_m: 0.859_375,
        soil_water_total_m: 0.859_375,
        runon_input_m: 0.03125,
        subsurface_carry_m: 0.003_906_25,
        profile_field_capacity_m: Some(0.6),
        profile_wilting_point_m: Some(0.2),
        publication_runoff_m: 0.25,
        publication_evapotranspiration_m: 0.125,
        publication_deep_percolation_m: 0.0625,
        publication_lateral_flow_m: 0.03125,
        public_output_cutover: false,
    }
}

fn expected_projection_shadow() -> DirectHydrologyProjectionShadowProjection {
    let state = expected_projection_state();
    DirectHydrologyProjectionShadowProjection {
        lane_index: 0,
        day_index: 0,
        aggregate_storage_from_layers_m: state.aggregate_storage_from_layers_m,
        storage_reconciled_m: state.storage_reconciled_m,
        aggregate_storage_delta_m: state.aggregate_storage_delta_m,
        q_runoff_m: state.q_runoff_m,
        q_ofe_m: state.q_ofe_m,
        deep_percolation_m: state.deep_percolation_m,
        lateral_flow_m: state.lateral_flow_m,
        tile_drainage_m: state.tile_drainage_m,
        subsurface_loss_m: state.subsurface_loss_m,
        evapotranspiration_m: state.evapotranspiration_m,
        soil_evaporation_m: state.soil_evaporation_m,
        residue_evaporation_m: state.residue_evaporation_m,
        plant_transpiration_m: state.plant_transpiration_m,
        water_stress: state.water_stress,
        snow_frost_coupling_m: state.snow_frost_coupling_m,
        snow_water_m: state.snow_water_m,
        frozen_soil_water_m: state.frozen_soil_water_m,
        total_soil_m: state.total_soil_m,
        soil_water_total_m: state.soil_water_total_m,
        runon_input_m: state.runon_input_m,
        subsurface_carry_m: state.subsurface_carry_m,
        profile_field_capacity_m: state.profile_field_capacity_m,
        profile_wilting_point_m: state.profile_wilting_point_m,
        publication_runoff_m: state.publication_runoff_m,
        publication_evapotranspiration_m: state.publication_evapotranspiration_m,
        publication_deep_percolation_m: state.publication_deep_percolation_m,
        publication_lateral_flow_m: state.publication_lateral_flow_m,
        public_output_cutover: false,
    }
}

fn layer(theta_m: f64, frozen_water_m: f64) -> DirectSubsurfaceLayerState {
    DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m,
        field_capacity_m: 0.1,
        upper_limit_m: 1.0,
        conductivity_m_s: 1.0,
        depth_m: 1.0,
        residual_theta: 0.0,
        frozen_depth_m: if frozen_water_m > 0.0 { 0.25 } else { 0.0 },
        frozen_water_m,
        porosity: 1.0,
        field_capacity_theta: 0.5,
        coca: 1.0,
        lateral_conductivity_m_s: 1.0,
    })
}
