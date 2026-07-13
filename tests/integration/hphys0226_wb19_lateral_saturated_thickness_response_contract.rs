use openwepp_hillslope_orchestrator::{
    DirectDayFrame, DirectPercolationShadowProjection, DirectRunIdentity,
    DirectSubsurfaceComputeInputs, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
};

fn lateral_flow(theta_m: f64) -> f64 {
    let layer = DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m,
        field_capacity_m: 0.2,
        upper_limit_m: 1.2,
        conductivity_m_s: 1.0e-5,
        depth_m: 1.0,
        porosity: 0.8,
        field_capacity_theta: 0.2,
        coca: 1.0,
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
        lateral_anisotropy_ratio: 1.0e6,
        soil_depth_m: 1.0,
        layers: vec![layer.into()],
        ..DirectSubsurfaceComputeInputs::neutral()
    };
    day.run_r4o_subsurface_compute_span()
        .expect("lateral response case must pass");
    day.subsurface_compute.lateral_flow_m
}

#[test]
fn hphys0226_lateral_flow_increases_with_available_saturated_storage() {
    let low = lateral_flow(0.6);
    let high = lateral_flow(1.0);
    assert!(high > low);
    assert!(high - low >= 0.2 - 1.0e-12);
    assert!(low <= 0.4 + 1.0e-12 && high <= 0.8 + 1.0e-12);
}
