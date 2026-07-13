use openwepp_hillslope_orchestrator::{
    DirectDayFrame, DirectPercolationShadowProjection, DirectRunIdentity, DirectRuntimeError,
    DirectSubsurfaceComputeInputs, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
};

fn seeded_day(soil_water_m: f64, drainage: bool) -> DirectDayFrame {
    let layer = DirectSubsurfaceLayerState::from(DirectSubsurfaceLayerInputs {
        theta_m: 1.0,
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
        soil_water_before_m: soil_water_m,
        soil_water_after_m: soil_water_m,
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
    day
}

#[test]
fn hphys0224_realized_withdrawals_subtract_exactly_from_soil_water() {
    for drainage in [false, true] {
        let mut day = seeded_day(10.5, drainage);
        day.run_r4o_subsurface_compute_span()
            .expect("in-domain withdrawal must pass");
        let withdrawal = day.subsurface_compute.subsurface_loss_m;
        assert!(withdrawal > 0.0);
        assert!((day.subsurface_compute.soil_water_after_m - (10.5 - withdrawal)).abs() <= 1.0e-12);
    }
}

#[test]
fn hphys0224_overwithdrawal_is_a_typed_domain_failure() {
    for drainage in [false, true] {
        let mut day = seeded_day(1.0e-6, drainage);
        assert_eq!(
            day.run_r4o_subsurface_compute_span()
                .expect_err("overdraw must fail closed"),
            DirectRuntimeError::DirectDomainViolation {
                field: "subsurface.withdrawal_m"
            }
        );
    }
}
