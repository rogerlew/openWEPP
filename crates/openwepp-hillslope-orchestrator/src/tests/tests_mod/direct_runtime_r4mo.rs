use super::direct_runtime_test_lock;
use crate::{
    DIRECT_R4M_PERCOLATION_SPAN, DIRECT_R4M_PHASE_SPAN_COUNT, DIRECT_R4O_PHASE_SPAN_COUNT,
    DIRECT_R4O_SUBSURFACE_SPAN, DirectDayFrame, DirectPercolationInputs, DirectPhaseKind,
    DirectRunIdentity, DirectRuntimeError, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState, reset_direct_runtime_audit_counters,
};

#[test]
fn r4mo_percolation_matches_wb18_kernel_authority_and_feeds_r4b_deep_seepage() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4M_PERCOLATION_SPAN,
        [
            DirectPhaseKind::PercolationDeepSeepage,
            DirectPhaseKind::StorageReconciliation
        ]
    );

    let mut day = seeded_day();
    day.percolation_inputs = daily_percolation_inputs(0.342_999_999_999_998_5, 1, false);
    day.percolation_inputs.layers[1].theta_m = 0.250_000_000_012;

    let report = day
        .run_r4m_percolation_span()
        .expect("valid direct WB18 percolation should execute");

    assert_eq!(report.phase_count, DIRECT_R4M_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4M_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);

    assert!(day.percolation.soil_water_after_m.is_finite());
    assert!(day.percolation.deep_seepage_m >= 0.0);
    assert!(day.percolation.recharge_m >= 0.0);
    assert!(day.percolation.per_layer_flux_m[1] >= 0.0);
    assert_close(
        day.storage_reconciliation_inputs.deep_seepage_m,
        day.percolation.deep_seepage_m,
    );
    assert_eq!(
        day.percolation_shadow_projection
            .as_ref()
            .expect("R4M must shadow project")
            .deep_seepage_m
            .to_bits(),
        day.deep_seepage.deep_seepage_m.to_bits()
    );
}

#[test]
fn r4mo_percolation_hourly_restrictive_branch_matches_wb18_kernel_authority() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = seeded_day();
    day.percolation_inputs = daily_percolation_inputs(0.463, 24, true);
    day.percolation_inputs.layers[1].theta_m = 0.32;

    day.run_r4m_percolation_span()
        .expect("hourly restrictive direct WB18 should execute");

    assert!(day.percolation.deep_seepage_m.is_finite());
    assert!(day.percolation.layer_state_after[1].theta_m.is_finite());
    assert!(
        day.percolation.deep_seepage_m > 0.0,
        "hourly restrictive fixture must keep the bottom-layer branch live"
    );
}

#[test]
fn r4mo_percolation_hourly_saturated_lower_boundary_uses_frozen_inclusive_storage() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = seeded_day();
    day.percolation_inputs =
        daily_percolation_inputs(0.20 + 0.45 + (0.05 * 0.30) + (0.07 * 0.40), 24, false);
    day.percolation_inputs.layers[0].theta_m = 0.20;
    day.percolation_inputs.layers[0].field_capacity_m = 0.10;
    day.percolation_inputs.layers[1].theta_m = 0.45;
    day.percolation_inputs.layers[1].field_capacity_m = 0.49;
    day.percolation_inputs.layers[1].frozen_water_m = 0.04;

    day.run_r4m_percolation_span()
        .expect("hourly saturated-lower direct WB18 should execute");

    assert!(day.percolation.per_layer_flux_m[0].is_finite());
    assert!(day.percolation.layer_state_after[0].theta_m.is_finite());
    assert!(
        day.percolation.per_layer_flux_m[0] > 0.010,
        "fixture must force the hourly saturated-lower fx=1 branch"
    );
}

#[test]
fn r4mo_subsurface_compute_feeds_qd_and_shadow_projection() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R4O_SUBSURFACE_SPAN,
        [
            DirectPhaseKind::Drainage,
            DirectPhaseKind::LateralTransfer,
            DirectPhaseKind::StorageReconciliation
        ]
    );

    let mut day = seeded_day();
    day.percolation_inputs = daily_percolation_inputs(0.663, 1, false);
    day.percolation_inputs.layers[0].theta_m = 0.24;
    day.percolation_inputs.layers[1].theta_m = 0.38;
    day.subsurface_compute_inputs = daily_subsurface_inputs(false, 1);
    day.subsurface_compute_inputs.layers[0].theta_m = 0.24;
    day.subsurface_compute_inputs.layers[1].theta_m = 0.38;

    day.run_r4m_percolation_span()
        .expect("R4M should execute before R4O");
    let report = day
        .run_r4o_subsurface_compute_span()
        .expect("valid direct WB19 subsurface compute should execute");

    assert_eq!(report.phase_count, DIRECT_R4O_PHASE_SPAN_COUNT);
    assert_eq!(report.phase_entry_count, DIRECT_R4O_PHASE_SPAN_COUNT as u64);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert!(
        day.subsurface_compute.lateral_flow_m > 0.0,
        "daily lateral fixture must keep q live"
    );
    assert_close(
        day.subsurface_compute.subsurface_loss_m,
        day.subsurface_compute.lateral_flow_m + day.subsurface_compute.tile_drainage_m,
    );
    assert_close(
        day.storage_reconciliation_inputs.subsurface_loss_m,
        day.subsurface_compute.subsurface_loss_m,
    );
    assert_eq!(
        day.subsurface_compute_shadow_projection
            .as_ref()
            .expect("R4O must shadow project")
            .subsurface_loss_m
            .to_bits(),
        day.subsurface_loss.subsurface_loss_m.to_bits()
    );
}

#[test]
fn r4mo_subsurface_hourly_drainage_runs_before_lateral_and_populates_carry_arrays() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = seeded_day();
    day.percolation_inputs = daily_percolation_inputs(0.753, 1, false);
    day.percolation_inputs.layers[0].theta_m = 0.30;
    day.percolation_inputs.layers[1].theta_m = 0.42;
    day.subsurface_compute_inputs = daily_subsurface_inputs(true, 24);
    day.subsurface_compute_inputs.layers[0].theta_m = 0.30;
    day.subsurface_compute_inputs.layers[1].theta_m = 0.42;

    day.run_r4m_percolation_span()
        .expect("R4M should execute before R4O");
    day.run_r4o_subsurface_compute_span()
        .expect("hourly direct WB19 subsurface compute should execute");

    assert!(
        day.subsurface_compute.tile_drainage_m > 0.0,
        "hourly drainage fixture must keep Qdd live"
    );
    assert!(
        day.subsurface_compute.lateral_flow_m > 0.0,
        "hourly lateral fixture must keep q live after drainage"
    );
    assert_close(
        day.subsurface_compute.subsurface_loss_m,
        day.subsurface_compute.lateral_flow_m + day.subsurface_compute.tile_drainage_m,
    );
    assert!(
        day.subsurface_compute
            .hourly_lateral_carry_m
            .iter()
            .any(|value| *value > 0.0),
        "hourly q carry array must be populated"
    );
    assert!(
        day.subsurface_compute
            .lateral_layer_withdrawal_m
            .iter()
            .sum::<f64>()
            <= day.subsurface_compute.lateral_target_m + 1.0e-12
    );
}

#[test]
fn r4mo_subsurface_requires_percolation_upstream_and_rejects_invalid_domains() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut missing_percolation = seeded_day();
    assert_eq!(
        missing_percolation
            .run_r4o_subsurface_compute_span()
            .expect_err("R4O should require R4M"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4M percolation"
        }
    );

    let mut invalid_percolation = seeded_day();
    invalid_percolation.percolation_inputs = daily_percolation_inputs(0.343, 0, false);
    assert_eq!(
        invalid_percolation
            .run_r4m_percolation_span()
            .expect_err("zero lane substeps should fail closed"),
        DirectRuntimeError::DirectDomainViolation {
            field: "percolation.lane_substeps"
        }
    );

    let mut invalid_subsurface = seeded_day();
    invalid_subsurface.percolation_inputs = daily_percolation_inputs(0.343, 1, false);
    invalid_subsurface
        .run_r4m_percolation_span()
        .expect("R4M should run before invalid R4O check");
    invalid_subsurface.subsurface_compute_inputs = daily_subsurface_inputs(false, 1);
    invalid_subsurface
        .subsurface_compute_inputs
        .lateral_anisotropy_ratio = 0.0;
    assert_eq!(
        invalid_subsurface
            .run_r4o_subsurface_compute_span()
            .expect_err("zero anisotropy should fail closed"),
        DirectRuntimeError::DirectDomainViolation {
            field: "subsurface.lateral_anisotropy_ratio"
        }
    );
}

#[test]
fn r4mo_anti_aliases_deep_lateral_drainage_and_qd() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = seeded_day();
    day.percolation_inputs = daily_percolation_inputs(0.753, 1, false);
    day.percolation_inputs.layers[0].theta_m = 0.30;
    day.percolation_inputs.layers[1].theta_m = 0.42;
    day.subsurface_compute_inputs = daily_subsurface_inputs(true, 24);
    day.subsurface_compute_inputs.layers[0].theta_m = 0.30;
    day.subsurface_compute_inputs.layers[1].theta_m = 0.42;

    day.run_r4m_percolation_span()
        .expect("R4M should execute before R4O");
    day.run_r4o_subsurface_compute_span()
        .expect("R4O should execute");

    assert_ne!(
        day.percolation.deep_seepage_m.to_bits(),
        day.subsurface_compute.lateral_flow_m.to_bits()
    );
    assert_ne!(
        day.percolation.deep_seepage_m.to_bits(),
        day.subsurface_compute.tile_drainage_m.to_bits()
    );
    assert_ne!(
        day.subsurface_compute.subsurface_loss_m.to_bits(),
        day.subsurface_compute.lateral_flow_m.to_bits()
    );
    assert_ne!(
        day.subsurface_compute.subsurface_loss_m.to_bits(),
        day.subsurface_compute.tile_drainage_m.to_bits()
    );
}

fn seeded_day() -> DirectDayFrame {
    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct identity should construct");
    DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct")
}

fn daily_percolation_inputs(
    soil_water_initial_m: f64,
    lane_substeps: usize,
    restrictive_layer_enabled: bool,
) -> DirectPercolationInputs {
    DirectPercolationInputs {
        soil_water_initial_m,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps,
        restrictive_layer_enabled,
        restrictive_layer_conductivity_m_s: 5.0e-7,
        restrictive_layer_thickness_m: 0.2,
        layers: vec![
            DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
                theta_m: 0.10,
                field_capacity_m: 0.15,
                upper_limit_m: 0.40,
                conductivity_m_s: 1.0e-6,
                depth_m: 0.30,
                residual_theta: 0.05,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 0.55,
                field_capacity_theta: 0.55,
                coca: 1.0,
                lateral_conductivity_m_s: 1.0e-6,
            }),
            DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
                theta_m: 0.20,
                field_capacity_m: 0.25,
                upper_limit_m: 0.50,
                conductivity_m_s: 1.0e-6,
                depth_m: 0.40,
                residual_theta: 0.07,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 0.55,
                field_capacity_theta: 0.55,
                coca: 1.0,
                lateral_conductivity_m_s: 1.0e-6,
            }),
        ],
    }
}

fn daily_subsurface_inputs(hourly: bool, lane_substeps: usize) -> DirectSubsurfaceComputeInputs {
    DirectSubsurfaceComputeInputs {
        avg_slope: 0.12,
        slope_length_m: 10.0,
        lateral_anisotropy_ratio: 1.5,
        soil_depth_m: 0.70,
        solwpv_mode: if hourly { 9002 } else { 2006 },
        mofe_hourly_carry_arrays_enabled: hourly,
        lane_substeps,
        drainage_capacity_m: if hourly { 0.05 } else { 0.0 },
        drain_enabled: hourly,
        drain_depth_m: 0.55,
        drain_spacing_m: 2.0,
        drain_diameter_m: 0.1,
        layers: vec![
            DirectSubsurfaceLayerInputs {
                theta_m: 0.20,
                field_capacity_m: 0.10,
                upper_limit_m: 0.30,
                conductivity_m_s: 1.0e-6,
                depth_m: 0.30,
                residual_theta: 0.05,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 0.55,
                field_capacity_theta: 0.383_333_333_333_333_36,
                coca: 1.0,
                lateral_conductivity_m_s: 1.2e-6,
            },
            DirectSubsurfaceLayerInputs {
                theta_m: 0.25,
                field_capacity_m: 0.10,
                upper_limit_m: 0.35,
                conductivity_m_s: 1.0e-6,
                depth_m: 0.40,
                residual_theta: 0.05,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 0.55,
                field_capacity_theta: 0.30,
                coca: 1.0,
                lateral_conductivity_m_s: 1.2e-6,
            },
        ],
    }
}

fn assert_close(observed: f64, expected: f64) {
    assert!(
        (observed - expected).abs() <= 1.0e-12,
        "observed {observed} expected {expected}"
    );
}
