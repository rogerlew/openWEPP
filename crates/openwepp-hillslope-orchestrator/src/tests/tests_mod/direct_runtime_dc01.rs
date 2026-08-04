use crate::{
    DirectDayFrame, DirectRunIdentity, DirectRuntimeError, DirectSnowCouplingInputs,
    DirectSnowLiquidDispositionLedger, DirectSnowMassTransitionLedgers,
    DirectSnowSolidToLiquidLedger, DirectSnowStage3Outcome, DirectWb14HyetographInterval,
    DirectWb14InfiltrationProducerInputs,
};

#[test]
fn dc01_hourly_supply_basis_merges_rain_and_runon() {
    let hyetograph = vec![DirectWb14HyetographInterval {
        start_s: 0.0,
        end_s: 7_200.0,
        intensity_m_s: 2.0e-6,
    }];
    let mut runon = [0.0_f64; 24];
    runon[5] = 0.003;
    let basis = crate::direct_runtime::dc01_test_hourly_supply_basis(&hyetograph, &runon);
    assert_eq!(basis.len(), 24);
    let depth = |i: usize| basis[i].intensity_m_s * 3_600.0;
    assert!((depth(0) - 2.0e-6 * 3_600.0).abs() < 1.0e-15);
    assert!((depth(1) - 2.0e-6 * 3_600.0).abs() < 1.0e-15);
    assert!((depth(5) - 0.003).abs() < 1.0e-15);
    let total: f64 = (0..24).map(depth).sum();
    assert!((total - (2.0e-6 * 7_200.0 + 0.003)).abs() < 1.0e-12);
}

#[test]
fn dc01_runon_supply_distribution_uses_shapes_with_uniform_fallback() {
    let mut weights = [0.0_f64; 24];
    weights[3] = 0.75;
    weights[10] = 0.25;
    let mut lateral = [0.0_f64; 24];
    lateral[8] = 0.002;
    let supply = DirectDayFrame::dc01_distribute_runon_supply(0.01, 0.004, &weights, &lateral);
    assert!((supply[3] - 0.0075).abs() < 1.0e-15);
    assert!((supply[10] - 0.0025).abs() < 1.0e-15);
    assert!((supply[8] - 0.004).abs() < 1.0e-15);
    let total: f64 = supply.iter().sum();
    assert!((total - 0.014).abs() < 1.0e-12);
    let uniform_supply =
        DirectDayFrame::dc01_distribute_runon_supply(0.024, 0.0, &[0.0; 24], &[0.0; 24]);
    assert!((uniform_supply[0] - 0.001).abs() < 1.0e-15);
    assert!((uniform_supply[23] - 0.001).abs() < 1.0e-15);
}

#[test]
fn dc01_surface_shape_uses_routed_melt_limb_without_uniform_fallback() {
    let wb14 = [0.0_f64; 24];
    let saturation = [0.0_f64; 24];
    let mut melt = [0.0_f64; 24];
    melt[6] = 0.003;
    melt[7] = 0.001;

    let weights = crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
        0.002,
        &wb14,
        &saturation,
        &melt,
    )
    .expect("melt-only source shape should normalize");

    assert!((weights[6] - 0.75).abs() < 1.0e-15);
    assert!((weights[7] - 0.25).abs() < 1.0e-15);
    assert!((weights.iter().sum::<f64>() - 1.0).abs() < 1.0e-15);
    assert_ne!(weights, [1.0 / 24.0; 24]);
}

#[test]
fn dc01_surface_shape_rejects_invalid_routed_melt_limb() {
    let wb14 = [0.0_f64; 24];
    let saturation = [0.0_f64; 24];
    let mut melt = [0.0_f64; 24];
    melt[3] = -1.0e-6;

    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
            0.002,
            &wb14,
            &saturation,
            &melt,
        )
        .expect_err("negative melt limb must fail closed"),
        DirectRuntimeError::NegativeDirectValue {
            field: "dc01_surface_shape.hourly_routed_melt_m"
        }
    );
}

#[test]
fn dc01_surface_shape_rejects_nonfinite_inputs() {
    let mut wb14 = [0.0_f64; 24];
    let mut saturation = [0.0_f64; 24];
    let mut melt = [0.0_f64; 24];

    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
            f64::NAN,
            &wb14,
            &saturation,
            &melt,
        )
        .expect_err("nonfinite runoff scalar must fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "dc01_surface_shape.q_runoff_m"
        }
    );

    wb14[0] = f64::INFINITY;
    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
            0.001,
            &wb14,
            &saturation,
            &melt,
        )
        .expect_err("nonfinite WB14 limb must fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "dc01_surface_shape.wb14_hourly_excess_m"
        }
    );
    wb14[0] = 0.0;

    saturation[1] = f64::NAN;
    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
            0.001,
            &wb14,
            &saturation,
            &melt,
        )
        .expect_err("nonfinite saturation-carry limb must fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "dc01_surface_shape.hourly_saturation_carry_m"
        }
    );
    saturation[1] = 0.0;

    melt[2] = f64::INFINITY;
    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
            0.001,
            &wb14,
            &saturation,
            &melt,
        )
        .expect_err("nonfinite routed-melt limb must fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "dc01_surface_shape.hourly_routed_melt_m"
        }
    );
}

#[test]
fn dc01_surface_shape_returns_zero_weights_without_runoff() {
    let mut wb14 = [0.0_f64; 24];
    let saturation = [0.0_f64; 24];
    let mut melt = [0.0_f64; 24];
    wb14[1] = 0.001;
    melt[4] = 0.002;

    let weights = crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
        0.0,
        &wb14,
        &saturation,
        &melt,
    )
    .expect("dry runoff surface should still validate source limbs");

    assert_eq!(weights, [0.0; 24]);
}

#[test]
fn r4g_rejects_hourly_routed_melt_daily_nonclosure() {
    let identity =
        DirectRunIdentity::new(912, 2637, 1, 1).expect("valid direct identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    let mut hourly = [0.0_f64; 24];
    hourly[0] = 0.009;
    day.snow_coupling_inputs = DirectSnowCouplingInputs {
        mass_transition_ledgers: Box::new(
            DirectSnowMassTransitionLedgers::try_from_parts(
                DirectSnowSolidToLiquidLedger {
                    snowpack_swe_loss_m: 0.010,
                    liquid_handoff_m: 0.010,
                    ..DirectSnowSolidToLiquidLedger::default()
                },
                DirectSnowLiquidDispositionLedger::default(),
                DirectSnowStage3Outcome::default(),
            )
            .expect("valid disabled Stage-3 mass transition"),
        ),
        hourly_routed_melt_m: hourly,
        ..DirectSnowCouplingInputs::zero()
    };

    assert_eq!(
        day.run_r4g_snow_coupling_span()
            .expect_err("hourly routed melt must close to daily scalar"),
        DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "snow_coupling.hourly_routed_melt_m"
        }
    );
}

#[test]
fn dc01_wb14_supply_admission_increases_infiltration_with_runon() {
    let base_inputs = DirectWb14InfiltrationProducerInputs {
        runon_hourly_supply_m: [0.0; 24],
        hyetograph: vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 3_600.0,
            intensity_m_s: 5.0e-7,
        }],
        effective_conductivity_m_s: 1.0e-6,
        matric_potential_m: 0.2,
        storage_capacity_m: 0.5,
        depression_storage_capacity_m: 0.0,
    };
    let without = crate::direct_runtime::dc01_test_wb14_with_profile(&base_inputs)
        .expect("baseline WB14 must compute");
    let mut with_runon = base_inputs;
    with_runon.runon_hourly_supply_m[2] = 0.004;
    let with = crate::direct_runtime::dc01_test_wb14_with_profile(&with_runon)
        .expect("runon WB14 must compute");
    assert!(
        with.state.cumulative_infiltration_m > without.state.cumulative_infiltration_m,
        "runon must be admitted into the infiltration supply: {} vs {}",
        with.state.cumulative_infiltration_m,
        without.state.cumulative_infiltration_m
    );
    let excess_total: f64 = with.hourly_excess_m.iter().sum();
    let supply_total = 5.0e-7 * 3_600.0 + 0.004;
    assert!(
        (supply_total - with.state.cumulative_infiltration_m - excess_total).abs() < 1.0e-12,
        "supply must split exactly into infiltration + excess"
    );
}

#[test]
fn dc01_dry_runon_day_still_infiltrates() {
    let mut inputs = DirectWb14InfiltrationProducerInputs {
        runon_hourly_supply_m: [0.0; 24],
        hyetograph: vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 3_600.0,
            intensity_m_s: 0.0,
        }],
        effective_conductivity_m_s: 1.0e-6,
        matric_potential_m: 0.2,
        storage_capacity_m: 0.5,
        depression_storage_capacity_m: 0.0,
    };
    let dry =
        crate::direct_runtime::dc01_test_wb14_with_profile(&inputs).expect("dry day computes");
    assert_eq!(dry.state.cumulative_infiltration_m, 0.0);
    inputs.runon_hourly_supply_m[6] = 0.005;
    let with_runon = crate::direct_runtime::dc01_test_wb14_with_profile(&inputs)
        .expect("dry-runon day computes");
    assert!(
        with_runon.state.cumulative_infiltration_m > 0.0,
        "acceptance criterion 1: a dry-runon day must infiltrate, got {}",
        with_runon.state.cumulative_infiltration_m
    );
    let excess: f64 = with_runon.hourly_excess_m.iter().sum();
    assert!(
        (0.005 - with_runon.state.cumulative_infiltration_m - excess).abs() < 1.0e-12,
        "runon must split exactly into infiltration + excess on a dry day"
    );
}
