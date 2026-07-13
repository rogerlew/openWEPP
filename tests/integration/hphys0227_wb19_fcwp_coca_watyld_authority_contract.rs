use openwepp_hillslope_orchestrator::{
    DirectDayFrame, DirectPercolationShadowProjection, DirectRunIdentity,
    DirectSubsurfaceComputeInputs, DirectSubsurfaceComputeState, DirectSubsurfaceLayerInputs,
    DirectSubsurfaceLayerState,
};

fn run_case(field_capacity_theta: f64, residual_theta: f64) -> DirectSubsurfaceComputeState {
    let layer = DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m: 0.8,
        field_capacity_m: 0.2,
        upper_limit_m: 1.0,
        conductivity_m_s: 1.0e-5,
        depth_m: 1.0,
        residual_theta,
        porosity: 0.8,
        field_capacity_theta,
        coca: 0.5,
        lateral_conductivity_m_s: 1.0e-5,
        ..DirectSubsurfaceLayerInputs::neutral()
    });
    let identity = DirectRunIdentity::new(1, 1, 1, 1).expect("valid identity");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("valid day");
    day.percolation_shadow_projection = Some(DirectPercolationShadowProjection {
        lane_index: 0,
        day_index: 0,
        soil_water_before_m: 2.0,
        soil_water_after_m: 2.0,
        deep_seepage_m: 0.0,
        recharge_m: 0.0,
        per_layer_flux_m: vec![0.0],
        layer_state_after: vec![layer.clone()],
    });
    day.subsurface_compute_inputs = DirectSubsurfaceComputeInputs {
        avg_slope: 0.1,
        slope_length_m: 10.0,
        lateral_anisotropy_ratio: 2.0,
        soil_depth_m: 1.0,
        solwpv_mode: 2005,
        layers: vec![layer.into()],
        ..DirectSubsurfaceComputeInputs::neutral()
    };
    day.run_r4o_subsurface_compute_span()
        .expect("FC/WP water-yield case must pass");
    day.subsurface_compute
}

#[test]
fn hphys0227_fcwp_theta_lineage_controls_watyld_and_legacy_depth_response() {
    let low_fc = run_case(0.2, 0.05);
    let high_fc = run_case(0.25, 0.1);
    assert!((low_fc.lateral_flow_m - high_fc.lateral_flow_m).abs() <= 1.0e-12);
    assert!((low_fc.water_yield_m - 0.1).abs() <= 1.0e-12);
    assert!((high_fc.water_yield_m - 0.05).abs() <= 1.0e-12);
    assert!(high_fc.water_yield_m < low_fc.water_yield_m);
    assert!(high_fc.saturated_depth_m < low_fc.saturated_depth_m);
    assert!(high_fc.unsaturated_depth_m > low_fc.unsaturated_depth_m);
}
