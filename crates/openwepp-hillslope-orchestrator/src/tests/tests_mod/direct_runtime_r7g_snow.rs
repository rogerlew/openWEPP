use super::direct_runtime_test_lock;
use crate::{
    DirectDayFrame, DirectExecutorMode, DirectFrameExecutor, DirectLaneConstructorInputs,
    DirectRunConstructorInputs, DirectRunFrame, DirectRunIdentity, DirectSnowCouplingInputs,
    DirectSnowLaneState, DirectSnowRuntimeCarry, reset_direct_runtime_audit_counters,
};

#[test]
fn r7g_constructor_prefers_winter_column_snow_over_legacy_carry() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(79, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let mut lane = DirectLaneConstructorInputs::from_topology(0, 1, 1)
        .expect("single OFE lane constructor input should build");
    let canonical_snow = DirectSnowLaneState::from_runtime_values(0.03125, 0.125, 275.0, 4.0);
    lane.winter_column.snow = canonical_snow.clone();
    lane.snow_runtime_carry = Some(DirectSnowRuntimeCarry {
        runtime_swe_m: 0.5,
        runtime_depth_m: 0.75,
        runtime_density_kg_m3: 400.0,
        runtime_settle_day_count: 8.0,
        coe_boundary_depth_m: 0.75,
        coe_boundary_density_kg_m3: 400.0,
        coe_boundary_settle_day_count: 8.0,
        liquid_water_retained_m: 0.0,
        snow_albedo_state: None,
        layers: Vec::new(),
    });

    let frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
        identity,
        vec![lane],
    ))
    .expect("typed direct frame should construct from winter snow state");

    assert_eq!(frame.lanes[0].winter_column.snow, canonical_snow);
    assert_eq!(
        frame.lanes[0].snow_runtime_carry.as_deref(),
        Some(&DirectSnowRuntimeCarry::from(canonical_snow))
    );
}

#[test]
fn r7g_legacy_constructor_snow_carry_migrates_into_winter_column() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(80, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let mut lane = DirectLaneConstructorInputs::from_topology(0, 1, 1)
        .expect("single OFE lane constructor input should build");
    lane.snow_runtime_carry = Some(DirectSnowRuntimeCarry {
        runtime_swe_m: 0.015_625,
        runtime_depth_m: 0.0625,
        runtime_density_kg_m3: 210.0,
        runtime_settle_day_count: 2.0,
        coe_boundary_depth_m: 0.0625,
        coe_boundary_density_kg_m3: 210.0,
        coe_boundary_settle_day_count: 2.0,
        liquid_water_retained_m: 0.0,
        snow_albedo_state: None,
        layers: Vec::new(),
    });

    let frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
        identity,
        vec![lane],
    ))
    .expect("typed direct frame should construct from legacy snow carry");

    assert_eq!(
        frame.lanes[0].winter_column.snow,
        DirectSnowLaneState::from(
            frame.lanes[0]
                .snow_runtime_carry
                .as_deref()
                .cloned()
                .expect("legacy mirror")
        )
    );
}

#[test]
fn r7g_r4g_snow_coupling_mutates_winter_column_snow_state() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: 0.015_625,
        snow_state_projected: true,
        active_snow_coupling: true,
        routed_melt_m: 0.003_906_25,
        post_winter_rain_m: 0.011_718_75,
        runtime_swe_after_m: 0.03125,
        runtime_depth_after_m: 0.125,
        runtime_density_after_kg_m3: 250.0,
        runtime_settle_day_count_after: 3.0,
        coe_boundary_depth_after_m: 0.125,
        coe_boundary_density_after_kg_m3: 250.0,
        coe_boundary_settle_day_count_after: 3.0,
        ..DirectSnowCouplingInputs::zero()
    };

    day.run_r4g_snow_coupling_span()
        .expect("projected snow coupling should execute");

    let expected_snow = DirectSnowLaneState::from_runtime_values(0.03125, 0.125, 250.0, 3.0);
    assert_eq!(day.winter_column.snow, expected_snow);
    assert_eq!(
        day.snow_runtime_carry,
        Some(DirectSnowRuntimeCarry::from(expected_snow))
    );
}

#[test]
fn r7g_executor_commits_r4g_winter_column_snow_state_to_lane() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity = DirectRunIdentity::new(81, 2637, 1, 1)
        .expect("valid direct constructor identity should construct");
    let mut lane = DirectLaneConstructorInputs::from_topology(0, 1, 1)
        .expect("single OFE lane constructor input should build");
    lane.day_inputs[0].snow_coupling_inputs = DirectSnowCouplingInputs {
        snow_coupling_handoff_m: 0.0,
        snow_state_projected: true,
        active_snow_coupling: false,
        routed_melt_m: 0.0,
        post_winter_rain_m: 0.0,
        runtime_swe_after_m: 0.046_875,
        runtime_depth_after_m: 0.1875,
        runtime_density_after_kg_m3: 300.0,
        runtime_settle_day_count_after: 5.0,
        coe_boundary_depth_after_m: 0.1875,
        coe_boundary_density_after_kg_m3: 300.0,
        coe_boundary_settle_day_count_after: 5.0,
        ..DirectSnowCouplingInputs::zero()
    };
    let mut frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
        identity,
        vec![lane],
    ))
    .expect("typed direct frame should construct");

    DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_skeleton(&mut frame)
        .expect("direct skeleton should commit snow lane state");

    let expected_snow = DirectSnowLaneState::from_runtime_values(0.046_875, 0.1875, 300.0, 5.0);
    assert_eq!(frame.lanes[0].winter_column.snow, expected_snow);
    assert_eq!(
        frame.lanes[0].snow_runtime_carry.as_deref(),
        Some(&DirectSnowRuntimeCarry::from(expected_snow))
    );
}
