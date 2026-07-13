use openwepp_hillslope_orchestrator::{
    DirectDayFrame, DirectPercolationShadowProjection, DirectRunIdentity,
    DirectSubsurfaceComputeInputs, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
};

fn run_case(drainage: bool, legacy_scalar: f64) -> (f64, f64) {
    assert!(legacy_scalar.is_finite());
    let layer = DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m: 0.6,
        field_capacity_m: 0.2,
        upper_limit_m: 1.0,
        conductivity_m_s: 1.0e-4,
        depth_m: 1.0,
        porosity: 0.8,
        field_capacity_theta: 0.2,
        coca: 1.0,
        lateral_conductivity_m_s: 1.0e-4,
        ..DirectSubsurfaceLayerInputs::neutral()
    });
    let identity = DirectRunIdentity::new(1, 1, 1, 1).expect("valid identity");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("valid day");
    day.percolation_shadow_projection = Some(DirectPercolationShadowProjection {
        lane_index: 0,
        day_index: 0,
        soil_water_before_m: 10.5,
        soil_water_after_m: 10.5,
        deep_seepage_m: 0.0,
        recharge_m: 0.0,
        per_layer_flux_m: vec![0.0],
        layer_state_after: vec![layer.clone()],
    });
    day.subsurface_compute_inputs = DirectSubsurfaceComputeInputs {
        avg_slope: if drainage { 0.0 } else { 1.0 },
        slope_length_m: 1.0,
        lateral_anisotropy_ratio: 100.0,
        soil_depth_m: 1.0,
        drainage_capacity_m: if drainage { 0.1 } else { 0.0 },
        drain_enabled: drainage,
        drain_depth_m: 1.0,
        drain_spacing_m: 1.0,
        drain_diameter_m: 0.1,
        layers: vec![layer.into()],
        ..DirectSubsurfaceComputeInputs::neutral()
    };
    day.run_r4o_subsurface_compute_span()
        .expect("typed layer-pool case must pass");
    (
        day.subsurface_compute.subsurface_loss_m,
        day.subsurface_compute.soil_water_after_m,
    )
}

#[test]
fn hphys0225_deleted_legacy_scalar_cannot_expand_typed_layer_pool() {
    for drainage in [false, true] {
        let low = run_case(drainage, 2.5);
        let high = run_case(drainage, 50.0);
        assert_eq!(low, high);
        assert!(low.0 > 0.0 && low.0 <= 0.4 + 1.0e-12);
        assert!((low.1 - (10.5 - low.0)).abs() <= 1.0e-12);
    }
}
