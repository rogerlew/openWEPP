use super::direct_runtime_test_lock;
use crate::{
    DIRECT_R4N_ROOT_PHASE_SPAN_COUNT, DIRECT_R4N_ROOT_UPTAKE_SPAN, DIRECT_R4N_SURFACE_ET_SPAN,
    DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT, DirectDayFrame, DirectEvapotranspirationComputeInputs,
    DirectEvapotranspirationPmetInputs, DirectPhaseKind, DirectRunIdentity, DirectRuntimeError,
    DirectSubsurfaceComputeInputs, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
    reset_direct_runtime_audit_counters,
};

const TOL: f64 = 1.0e-12;

#[test]
fn r4n_surface_et_matches_wb17_soil_evaporation_layer_mutation_fixture() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4N_SURFACE_ET_SPAN,
        [DirectPhaseKind::Evapotranspiration]
    );

    let mut day = seeded_day_with_layers(vec![
        layer_state(0.03, 0.05, 1.0),
        layer_state(0.08, 0.20, 1.0),
    ]);
    day.evapotranspiration_compute_inputs = DirectEvapotranspirationComputeInputs {
        et_demand_m: 0.04,
        leaf_area_index: 0.0,
        canopy_cover_fraction: 0.0,
        residue_interception_m: 0.0,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: 0.0,
        plant_tolerance: 0.25,
        growth_context_required: false,
        stage_state: None,
        pmet: None,
        pmet_compute: None,
    };
    day.run_r4m_percolation_span()
        .expect("R4M no-loss percolation should seed R4N layers");

    let report = day
        .run_r4n_surface_et_span()
        .expect("valid direct WB17 surface ET should execute");

    let expected_es = 0.04 * (-0.05_f64).exp();
    let expected_layer_1 = 0.0;
    let expected_layer_2 = 0.08 - (expected_es - 0.03);
    let expected_soil_water = expected_layer_1 + expected_layer_2;

    assert_eq!(report.phase_count, DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT);
    assert_eq!(
        report.phase_entry_count,
        DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT as u64
    );
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_close(
        day.evapotranspiration_surface.soil_evaporation_m,
        expected_es,
    );
    assert_close(
        day.evapotranspiration_surface.layer_state_after_soil_evap[0].theta_m,
        expected_layer_1,
    );
    assert_close(
        day.evapotranspiration_surface.layer_state_after_soil_evap[1].theta_m,
        expected_layer_2,
    );
    assert_close(
        day.evapotranspiration_surface.soil_water_after_soil_evap_m,
        expected_soil_water,
    );
    assert_close(day.water.soil_water_m, expected_soil_water);
}

#[test]
fn r4n_root_uptake_matches_swu_fixture_and_finalizes_aggregate_et() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4N_ROOT_UPTAKE_SPAN,
        [
            DirectPhaseKind::PlantRootUptake,
            DirectPhaseKind::StorageReconciliation
        ]
    );

    let mut day = seeded_day_with_layers(vec![
        layer_state(0.004, 0.10, 0.02),
        layer_state(0.004, 0.10, 0.02),
    ]);
    day.evapotranspiration_compute_inputs = DirectEvapotranspirationComputeInputs {
        et_demand_m: 0.0,
        leaf_area_index: 0.0,
        canopy_cover_fraction: 0.0,
        residue_interception_m: 0.0,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: 0.20,
        plant_tolerance: 0.25,
        growth_context_required: false,
        stage_state: None,
        pmet: Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m: 0.0,
            plant_transpiration_m: 0.006,
            soil_evaporation_storage_return_m: 0.0,
        }),
        pmet_compute: None,
    };
    day.run_r4m_percolation_span()
        .expect("R4M no-loss percolation should seed R4N layers");
    day.run_r4n_surface_et_span()
        .expect("R4N surface ET should seed transpiration demand");
    day.subsurface_compute_inputs = no_loss_subsurface_inputs(&day.percolation.layer_state_after);
    day.run_r4o_subsurface_compute_span()
        .expect("R4O no-loss subsurface should pass root layers through");

    let report = day
        .run_r4n_root_uptake_span()
        .expect("valid direct WB17 SWU root uptake should execute");

    let expected_layer_1_uptake = 0.003_947_385_293_021_583_f64;
    let expected_layer_2_uptake = 0.000_852_615_503_174_695_3_f64;
    let expected_ep = expected_layer_1_uptake + expected_layer_2_uptake;

    assert_eq!(report.phase_count, DIRECT_R4N_ROOT_PHASE_SPAN_COUNT);
    assert_eq!(
        report.phase_entry_count,
        DIRECT_R4N_ROOT_PHASE_SPAN_COUNT as u64
    );
    assert_close(
        day.evapotranspiration_compute.layer_uptake_actual_m[0],
        expected_layer_1_uptake,
    );
    assert_close(
        day.evapotranspiration_compute.layer_uptake_actual_m[1],
        expected_layer_2_uptake,
    );
    assert_close(
        day.evapotranspiration_compute.plant_transpiration_m,
        expected_ep,
    );
    assert_close(
        day.evapotranspiration_compute.evapotranspiration_m,
        expected_ep,
    );
    assert_close(
        day.evapotranspiration_compute.water_stress,
        expected_ep / 0.006,
    );
    assert_close(
        day.evapotranspiration_compute.layer_state_after_root_uptake[0].theta_m,
        0.004 - expected_layer_1_uptake,
    );
    assert_close(
        day.evapotranspiration_compute.layer_state_after_root_uptake[1].theta_m,
        0.004 - expected_layer_2_uptake,
    );
    assert_close(
        day.storage_reconciliation_inputs.evapotranspiration_m,
        expected_ep,
    );
}

#[test]
fn r4o_consumes_r4n_surface_et_mutated_layer_state_when_present() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = seeded_day_with_layers(vec![
        layer_state(0.03, 0.05, 1.0),
        layer_state(0.08, 0.20, 1.0),
    ]);
    day.evapotranspiration_compute_inputs = DirectEvapotranspirationComputeInputs {
        et_demand_m: 0.04,
        leaf_area_index: 0.0,
        canopy_cover_fraction: 0.0,
        residue_interception_m: 0.0,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: 0.0,
        plant_tolerance: 0.25,
        growth_context_required: false,
        stage_state: None,
        pmet: None,
        pmet_compute: None,
    };
    day.run_r4m_percolation_span()
        .expect("R4M no-loss percolation should seed R4N layers");
    let percolation_soil_water = day.percolation.soil_water_after_m;
    day.run_r4n_surface_et_span()
        .expect("R4N surface ET should mutate layers before R4O");
    let surface_soil_water = day.evapotranspiration_surface.soil_water_after_soil_evap_m;
    day.subsurface_compute_inputs =
        no_loss_subsurface_inputs(&day.evapotranspiration_surface.layer_state_after_soil_evap);

    day.run_r4o_subsurface_compute_span()
        .expect("R4O no-loss subsurface should consume R4N surface layers");

    assert_ne!(
        percolation_soil_water.to_bits(),
        surface_soil_water.to_bits()
    );
    assert_close(
        day.subsurface_compute.soil_water_before_m,
        surface_soil_water,
    );
    assert_close(
        day.subsurface_compute.layer_state_after[0].theta_m,
        day.evapotranspiration_surface.layer_state_after_soil_evap[0].theta_m,
    );
    assert_close(
        day.subsurface_compute.layer_state_after[1].theta_m,
        day.evapotranspiration_surface.layer_state_after_soil_evap[1].theta_m,
    );
}

#[test]
fn r4n_feeds_r4b_final_et_not_handoff_or_publication_aliases() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = seeded_day_with_layers(vec![
        layer_state(0.004, 0.10, 0.02),
        layer_state(0.004, 0.10, 0.02),
    ]);
    day.evapotranspiration_inputs.evapotranspiration_handoff_m = 0.5;
    day.publication.evapotranspiration_m = 0.75;
    day.storage_reconciliation.closure_residual_m = 0.875;
    day.evapotranspiration_compute_inputs = DirectEvapotranspirationComputeInputs {
        et_demand_m: 0.0,
        leaf_area_index: 0.0,
        canopy_cover_fraction: 0.0,
        residue_interception_m: 0.01,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: 0.20,
        plant_tolerance: 0.25,
        growth_context_required: false,
        stage_state: None,
        pmet: Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m: 0.04,
            plant_transpiration_m: 0.006,
            soil_evaporation_storage_return_m: 0.0,
        }),
        pmet_compute: None,
    };
    day.run_r4m_percolation_span()
        .expect("R4M no-loss percolation should seed R4N layers");
    day.run_r4n_surface_et_span()
        .expect("R4N surface ET should execute");
    day.subsurface_compute_inputs =
        no_loss_subsurface_inputs(&day.evapotranspiration_surface.layer_state_after_soil_evap);
    day.run_r4o_subsurface_compute_span()
        .expect("R4O no-loss subsurface should pass root layers through");
    day.run_r4n_root_uptake_span()
        .expect("R4N root uptake should finalize aggregate ET");

    let final_et = day.evapotranspiration_compute.evapotranspiration_m;
    assert_close(
        day.storage_reconciliation_inputs.evapotranspiration_m,
        final_et,
    );
    assert_ne!(
        final_et.to_bits(),
        day.evapotranspiration_inputs
            .evapotranspiration_handoff_m
            .to_bits()
    );
    assert_ne!(
        final_et.to_bits(),
        day.publication.evapotranspiration_m.to_bits()
    );
    assert_ne!(
        final_et.to_bits(),
        day.evapotranspiration_compute.soil_evaporation_m.to_bits()
    );
    assert_ne!(
        final_et.to_bits(),
        day.evapotranspiration_compute
            .residue_evaporation_m
            .to_bits()
    );
    assert_ne!(
        final_et.to_bits(),
        day.evapotranspiration_compute
            .plant_transpiration_m
            .to_bits()
    );
    assert_ne!(
        final_et.to_bits(),
        day.storage_reconciliation.closure_residual_m.to_bits()
    );
}

#[test]
fn r4n_fails_closed_on_missing_upstream_and_invalid_domain() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut missing_r4m_day =
        DirectDayFrame::seed(identity, 0, 0).expect("direct day should construct");
    assert_eq!(
        missing_r4m_day
            .run_r4n_surface_et_span()
            .expect_err("R4N surface ET should require R4M"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4M percolation producer"
        }
    );

    let mut invalid_day = seeded_day_with_layers(vec![layer_state(0.03, 0.10, 1.0)]);
    invalid_day
        .run_r4m_percolation_span()
        .expect("R4M no-loss percolation should seed invalid R4N test");
    invalid_day
        .evapotranspiration_compute_inputs
        .canopy_cover_fraction = f64::NAN;
    assert_eq!(
        invalid_day
            .run_r4n_surface_et_span()
            .expect_err("nonfinite canopy cover should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "evapotranspiration.canopy_cover_fraction"
        }
    );

    let mut root_without_subsurface_day =
        seeded_day_with_layers(vec![layer_state(0.004, 0.10, 0.02)]);
    root_without_subsurface_day.evapotranspiration_compute_inputs =
        DirectEvapotranspirationComputeInputs {
            et_demand_m: 0.0,
            leaf_area_index: 0.0,
            canopy_cover_fraction: 0.0,
            residue_interception_m: 0.0,
            same_pass_infiltration_m: 0.0,
            outside_water_depth_m: 0.0,
            root_depth_m: 0.10,
            plant_tolerance: 0.25,
            growth_context_required: false,
            stage_state: None,
            pmet: Some(DirectEvapotranspirationPmetInputs {
                soil_evaporation_m: 0.0,
                plant_transpiration_m: 0.006,
                soil_evaporation_storage_return_m: 0.0,
            }),
            pmet_compute: None,
        };
    root_without_subsurface_day
        .run_r4m_percolation_span()
        .expect("R4M no-loss percolation should seed surface ET");
    root_without_subsurface_day
        .run_r4n_surface_et_span()
        .expect("R4N surface ET should seed root uptake");
    assert_eq!(
        root_without_subsurface_day
            .run_r4n_root_uptake_span()
            .expect_err("R4N root uptake should require R4O"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4O subsurface compute producer"
        }
    );
}

fn seeded_day_with_layers(layers: Vec<DirectSubsurfaceLayerState>) -> DirectDayFrame {
    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let soil_water_initial_m = layers
        .iter()
        .map(|layer| layer.theta_m + layer.residual_theta * layer.depth_m)
        .sum::<f64>();
    day.water.soil_water_m = soil_water_initial_m;
    day.percolation_inputs.soil_water_initial_m = soil_water_initial_m;
    day.percolation_inputs.layers = layers;
    day
}

fn layer_state(theta_m: f64, depth_m: f64, upper_limit_m: f64) -> DirectSubsurfaceLayerState {
    DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m,
        field_capacity_m: upper_limit_m,
        upper_limit_m,
        conductivity_m_s: 1.0,
        depth_m,
        residual_theta: 0.0,
        frozen_depth_m: 0.0,
        frozen_water_m: 0.0,
        porosity: 1.0,
        field_capacity_theta: 0.5,
        coca: 1.0,
        lateral_conductivity_m_s: 1.0,
    })
}

fn no_loss_subsurface_inputs(
    layers: &[DirectSubsurfaceLayerState],
) -> DirectSubsurfaceComputeInputs {
    DirectSubsurfaceComputeInputs {
        avg_slope: 0.0,
        slope_length_m: 1.0,
        lateral_anisotropy_ratio: 1.0,
        soil_depth_m: layers.iter().map(|layer| layer.depth_m).sum::<f64>(),
        solwpv_mode: 2006,
        mofe_hourly_carry_arrays_enabled: false,
        lane_substeps: 1,
        drainage_capacity_m: 0.0,
        drain_enabled: false,
        drain_depth_m: 0.5,
        drain_spacing_m: 1.0,
        drain_diameter_m: 0.1,
        layers: layers.iter().map(layer_inputs_from_state).collect(),
    }
}

fn layer_inputs_from_state(layer: &DirectSubsurfaceLayerState) -> DirectSubsurfaceLayerInputs {
    DirectSubsurfaceLayerInputs {
        theta_m: layer.theta_m,
        field_capacity_m: layer.field_capacity_m,
        upper_limit_m: layer.upper_limit_m,
        conductivity_m_s: layer.conductivity_m_s,
        depth_m: layer.depth_m,
        residual_theta: layer.residual_theta,
        frozen_depth_m: layer.frozen_depth_m,
        frozen_water_m: layer.frozen_water_m,
        porosity: layer.porosity,
        field_capacity_theta: layer.field_capacity_theta,
        coca: layer.coca,
        lateral_conductivity_m_s: layer.lateral_conductivity_m_s,
    }
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= TOL,
        "expected {actual} to equal {expected}"
    );
}
