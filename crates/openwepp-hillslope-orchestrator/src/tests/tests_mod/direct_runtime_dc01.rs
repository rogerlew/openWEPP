use crate::{
    DirectDayFrame, DirectRunIdentity, DirectRunonCarryDownstreamOperands, DirectRuntimeError,
    DirectSnowCouplingInputs, DirectSnowLiquidDispositionLedger, DirectSnowMassTransitionLedgers,
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
fn dc01_runon_supply_distribution_uses_produced_shapes_and_rejects_missing_shapes() {
    let mut weights = [0.0_f64; 24];
    weights[3] = 0.75;
    weights[10] = 0.25;
    let mut lateral = [0.0_f64; 24];
    lateral[8] = 0.002;
    let supply = DirectDayFrame::dc01_distribute_runon_supply(0.01, 0.004, &weights, &lateral)
        .expect("produced runon shapes should distribute");
    assert!((supply[3] - 0.0075).abs() < 1.0e-15);
    assert!((supply[10] - 0.0025).abs() < 1.0e-15);
    assert!((supply[8] - 0.004).abs() < 1.0e-15);
    let total: f64 = supply.iter().sum();
    assert!((total - 0.014).abs() < 1.0e-12);
    assert_eq!(
        DirectDayFrame::dc01_distribute_runon_supply(0.024, 0.0, &[0.0; 24], &[0.0; 24],)
            .expect_err("positive runon without an hourly shape must fail closed"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "hourly surface-runon transfer shape"
        }
    );
}

#[test]
fn routed_melt_enters_wb14_once_and_carries_peak_timing_after_partition() {
    let mut inputs = DirectWb14InfiltrationProducerInputs {
        hourly_additional_supply_m: [0.0; 24],
        hyetograph: vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 3_600.0,
            intensity_m_s: 0.0,
        }],
        effective_conductivity_m_s: 1.0e-6,
        matric_potential_m: 0.2,
        storage_capacity_m: 0.0,
        depression_storage_capacity_m: 0.0,
    };
    inputs.hourly_additional_supply_m[6] = 0.003;
    inputs.hourly_additional_supply_m[7] = 0.001;
    let outcome = crate::direct_runtime::dc01_test_wb14_with_profile(&inputs)
        .expect("melt-only WB14 supply should partition");
    let saturation = [0.0_f64; 24];

    let weights = crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
        0.004,
        &outcome.hourly_excess_m,
        &saturation,
    )
    .expect("post-partition melt supply should close");

    assert!((weights[6] - 0.75).abs() < 1.0e-15);
    assert!((weights[7] - 0.25).abs() < 1.0e-15);
    assert!((weights.iter().sum::<f64>() - 1.0).abs() < 1.0e-15);
    assert_ne!(weights, [1.0 / 24.0; 24]);
    let (_, duration_s, peak_hour) =
        crate::direct_runtime::test_source_complete_hourly_peak_runoff_depth_rate_m_s(
            0.004,
            &outcome.hourly_excess_m,
            &saturation,
        )
        .expect("melt-only source must carry production peak timing");
    assert!((duration_s - 4_800.0).abs() < 1.0e-9);
    assert_eq!(peak_hour, 6);
}

#[test]
fn routed_melt_can_infiltrate_without_creating_a_runoff_limb() {
    let mut inputs = DirectWb14InfiltrationProducerInputs {
        hourly_additional_supply_m: [0.0; 24],
        hyetograph: vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 3_600.0,
            intensity_m_s: 0.0,
        }],
        effective_conductivity_m_s: 1.0e-4,
        matric_potential_m: 0.2,
        storage_capacity_m: 0.5,
        depression_storage_capacity_m: 0.0,
    };
    inputs.hourly_additional_supply_m[6] = 0.004;
    let outcome = crate::direct_runtime::dc01_test_wb14_with_profile(&inputs)
        .expect("melt-only supply should pass WB14");
    assert!((outcome.state.cumulative_infiltration_m - 0.004).abs() < 1.0e-12);
    assert_eq!(outcome.hourly_excess_m, [0.0; 24]);
}

#[test]
fn wb14_preserves_tiny_positive_supply_and_rejects_invalid_supply_bins() {
    let mut inputs = DirectWb14InfiltrationProducerInputs {
        hourly_additional_supply_m: [0.0; 24],
        hyetograph: vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 3_600.0,
            intensity_m_s: 0.0,
        }],
        effective_conductivity_m_s: 1.0e-6,
        matric_potential_m: 0.2,
        storage_capacity_m: 0.0,
        depression_storage_capacity_m: 0.0,
    };
    inputs.hourly_additional_supply_m[9] = 1.0e-15;
    let outcome = crate::direct_runtime::dc01_test_wb14_with_profile(&inputs)
        .expect("every positive source-backed depth remains represented");
    assert_eq!(outcome.hourly_excess_m[9].to_bits(), 1.0e-15_f64.to_bits());

    for invalid in [-1.0e-15, f64::NAN] {
        inputs.hourly_additional_supply_m = [0.0; 24];
        inputs.hourly_additional_supply_m[9] = invalid;
        assert!(
            crate::direct_runtime::dc01_test_wb14_with_profile(&inputs).is_err(),
            "invalid hourly additional supply {invalid} must fail"
        );
    }
}

#[test]
fn hourly_peak_uses_saturation_return_in_its_produced_hour() {
    let wb14 = [0.0_f64; 24];
    let mut saturation = [0.0_f64; 24];
    saturation[11] = 0.004;
    let (peak_rate_m_s, duration_s, peak_hour) =
        crate::direct_runtime::test_source_complete_hourly_peak_runoff_depth_rate_m_s(
            0.004,
            &wb14,
            &saturation,
        )
        .expect("saturation-only source must carry production peak timing");
    assert!((peak_rate_m_s - 0.004 / 3_600.0).abs() < 1.0e-15);
    assert!((duration_s - 3_600.0).abs() < 1.0e-9);
    assert_eq!(peak_hour, 11);
}

#[test]
fn hourly_peak_uses_runon_only_wb14_excess_timing() {
    let mut inputs = DirectWb14InfiltrationProducerInputs {
        hourly_additional_supply_m: [0.0; 24],
        hyetograph: vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 3_600.0,
            intensity_m_s: 0.0,
        }],
        effective_conductivity_m_s: 1.0e-6,
        matric_potential_m: 0.2,
        storage_capacity_m: 0.0,
        depression_storage_capacity_m: 0.0,
    };
    inputs.hourly_additional_supply_m[8] = 0.003;
    let outcome = crate::direct_runtime::dc01_test_wb14_with_profile(&inputs)
        .expect("runon-only WB14 must compute");
    let (peak_rate_m_s, duration_s, peak_hour) =
        crate::direct_runtime::test_source_complete_hourly_peak_runoff_depth_rate_m_s(
            0.003,
            &outcome.hourly_excess_m,
            &[0.0; 24],
        )
        .expect("runon-only excess must carry production peak timing");
    assert!((peak_rate_m_s - 0.003 / 3_600.0).abs() < 1.0e-15);
    assert!((duration_s - 3_600.0).abs() < 1.0e-9);
    assert_eq!(peak_hour, 8);
}

#[test]
fn hourly_peak_fails_closed_for_positive_runoff_without_source_timing() {
    let error = crate::direct_runtime::test_source_complete_hourly_peak_runoff_depth_rate_m_s(
        0.003, &[0.0; 24], &[0.0; 24],
    );
    let DirectRuntimeError::HydrologyKernelGuard(source) =
        error.expect_err("positive runoff without source timing must fail closed")
    else {
        panic!("WB16 missing timing must use the typed hydrology guard")
    };
    assert_eq!(source.code(), "HKERNEL-WB16-PEAK-E-001");
}

#[test]
fn hourly_peak_typed_guard_covers_nonfinite_and_closure_failures() {
    let mut nonfinite = [0.0; 24];
    nonfinite[3] = f64::NAN;
    let DirectRuntimeError::HydrologyKernelGuard(nonfinite_guard) =
        crate::direct_runtime::test_source_complete_hourly_peak_runoff_depth_rate_m_s(
            0.003, &nonfinite, &[0.0; 24],
        )
        .expect_err("non-finite hourly depth must fail")
    else {
        panic!("WB16 non-finite depth must use the typed hydrology guard")
    };
    assert_eq!(nonfinite_guard.code(), "HKERNEL-WB16-PEAK-E-002");

    let mut nonclosing = [0.0; 24];
    nonclosing[3] = 0.002;
    let DirectRuntimeError::HydrologyKernelGuard(closure_guard) =
        crate::direct_runtime::test_source_complete_hourly_peak_runoff_depth_rate_m_s(
            0.003,
            &nonclosing,
            &[0.0; 24],
        )
        .expect_err("non-closing hourly depth must fail")
    else {
        panic!("WB16 closure failure must use the typed hydrology guard")
    };
    assert_eq!(closure_guard.code(), "HKERNEL-WB16-PEAK-E-003");
}

#[test]
fn partition_runoff_only_canonicalizes_source_free_roundoff() {
    let zero_source = [0.0; 24];
    assert_eq!(
        crate::direct_runtime::test_source_informed_partition_runoff_canonicalization(
            5.0e-13,
            &zero_source,
        )
        .expect("source-free roundoff canonicalization"),
        0.0
    );
    let mut tiny_source = [0.0; 24];
    tiny_source[4] = 5.0e-13;
    assert_eq!(
        crate::direct_runtime::test_source_informed_partition_runoff_canonicalization(
            5.0e-13,
            &tiny_source,
        )
        .expect("source-backed tiny runoff remains represented"),
        5.0e-13
    );
    assert_eq!(
        crate::direct_runtime::test_source_informed_partition_runoff_canonicalization(
            2.0e-12,
            &zero_source,
        )
        .expect("material source-free runoff remains for peak fail-closed handling"),
        2.0e-12
    );
}

#[test]
fn hourly_partition_reconciliation_clears_full_frost_retention_without_timing() {
    let mut hourly = [0.0; 24];
    hourly[2] = 0.006;
    hourly[8] = 0.004;
    let reconciled_m = crate::direct_runtime::test_reconcile_hourly_partition_runoff_profile(
        &mut hourly,
        0.0,
        0.01,
    )
    .expect("full frost retention leaves no positive peak timing to claim");

    assert_eq!(reconciled_m.to_bits(), 0.0_f64.to_bits());
    assert_eq!(hourly, [0.0; 24]);
}

#[test]
fn hourly_partition_reconciliation_rejects_partial_daily_frost_timing() {
    let mut hourly = [0.0; 24];
    hourly[2] = 0.006;
    hourly[8] = 0.004;
    assert_eq!(
        crate::direct_runtime::test_reconcile_hourly_partition_runoff_profile(
            &mut hourly,
            0.008,
            0.002,
        )
        .expect_err("partial daily frost retention cannot invent positive runoff timing"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "hourly frost-retention timing for partial positive runoff"
        }
    );
}

#[test]
fn hourly_partition_reconciliation_rejects_positive_empty_ledger() {
    for partition_runoff_m in [2.0e-12, 24.0e-9] {
        let mut hourly = [0.0; 24];
        assert_eq!(
            crate::direct_runtime::test_reconcile_hourly_partition_runoff_profile(
                &mut hourly,
                partition_runoff_m,
                0.0,
            )
            .expect_err("a positive daily scalar cannot supply missing hourly timing"),
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "hourly WB14 runoff ledger for positive partition runoff"
            }
        );
    }
}

#[test]
fn mixed_local_same_pass_infiltration_and_runon_requires_source_tagged_timing() {
    assert_eq!(
        crate::direct_runtime::test_ensure_hourly_same_pass_source_custody(0.002, 0.003)
            .expect_err("a local-only daily debit cannot be applied to merged runon"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "hourly source-tagged local/runon same-pass infiltration custody"
        }
    );
    crate::direct_runtime::test_ensure_hourly_same_pass_source_custody(0.002, 0.0)
        .expect("a local-only ledger preserves source custody");
}

#[test]
fn positive_runon_without_a_wb14_producer_fails_closed() {
    let identity = DirectRunIdentity::new(1, 1, 1, 1).expect("identity");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("day frame");
    day.runon_carry_downstream_operands = DirectRunonCarryDownstreamOperands {
        runon_input_m: 1.0e-15,
        subsurface_carry_m: 0.0,
    };
    assert_eq!(
        day.run_r4k_infiltration_depression_span()
            .expect_err("positive runon requires a WB14 producer"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "WB14 producer for positive hourly runon supply"
        }
    );
}

#[test]
fn hourly_partition_reconciliation_is_bounded_and_hourly_authoritative() {
    let aggregate_bound_m = 24.0e-9;
    let mut at_bound = [0.0; 24];
    at_bound[4] = 0.01;
    let reconciled_m = crate::direct_runtime::test_reconcile_hourly_partition_runoff_profile(
        &mut at_bound,
        0.01 + aggregate_bound_m,
        0.0,
    )
    .expect("the exact 24-interval aggregate bound is within TOL-WATBAL-009");
    assert_eq!(reconciled_m.to_bits(), 0.01_f64.to_bits());
    assert_eq!(at_bound[4].to_bits(), 0.01_f64.to_bits());

    let mut outside_bound = [0.0; 24];
    outside_bound[4] = 0.01;
    assert_eq!(
        crate::direct_runtime::test_reconcile_hourly_partition_runoff_profile(
            &mut outside_bound,
            0.01 + aggregate_bound_m + 1.0e-12,
            0.0,
        )
        .expect_err("a daily/hourly mismatch outside the bound must fail closed"),
        DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "runoff_partition.hourly_partition_runoff_m"
        }
    );
}

#[test]
fn dc01_surface_shape_rejects_hourly_source_total_mismatch() {
    let mut wb14 = [0.0_f64; 24];
    wb14[3] = 0.004;
    let saturation = [0.0_f64; 24];

    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(0.002, &wb14, &saturation,)
            .expect_err("hourly runoff depths must independently close to Q"),
        DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "peak_runoff.hourly_source_total_m"
        }
    );
}

#[test]
fn dc01_surface_shape_rejects_nonfinite_inputs() {
    let mut wb14 = [0.0_f64; 24];
    let mut saturation = [0.0_f64; 24];

    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(
            f64::NAN,
            &wb14,
            &saturation,
        )
        .expect_err("nonfinite runoff scalar must fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "dc01_surface_shape.q_runoff_m"
        }
    );

    wb14[0] = f64::INFINITY;
    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(0.001, &wb14, &saturation,)
            .expect_err("nonfinite WB14 limb must fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "peak_runoff.wb14_hourly_excess_m"
        }
    );
    wb14[0] = 0.0;

    saturation[1] = f64::NAN;
    assert_eq!(
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(0.001, &wb14, &saturation,)
            .expect_err("nonfinite saturation-carry limb must fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "peak_runoff.hourly_saturation_carry_m"
        }
    );
}

#[test]
fn dc01_surface_shape_returns_zero_weights_without_runoff_and_sources() {
    let wb14 = [0.0_f64; 24];
    let saturation = [0.0_f64; 24];

    let weights =
        crate::direct_runtime::dc01_test_surface_runoff_hourly_weights(0.0, &wb14, &saturation)
            .expect("dry runoff surface should still validate source limbs");

    assert_eq!(weights, [0.0; 24]);
}

#[test]
fn hourly_peak_orders_equal_volume_concentrated_and_spread_shapes() {
    let q_runoff_m = 0.024;
    let mut concentrated = [0.0; 24];
    concentrated[7] = 1.0;
    let spread = [1.0 / 24.0; 24];

    let (concentrated_rate, concentrated_duration, concentrated_hour) =
        crate::direct_runtime::test_hourly_peak_runoff_depth_rate_m_s(q_runoff_m, &concentrated)
            .expect("concentrated hourly peak");
    let (spread_rate, spread_duration, spread_hour) =
        crate::direct_runtime::test_hourly_peak_runoff_depth_rate_m_s(q_runoff_m, &spread)
            .expect("spread hourly peak");

    assert!((concentrated_rate - q_runoff_m / 3_600.0).abs() < 1.0e-15);
    assert!((spread_rate - q_runoff_m / 86_400.0).abs() < 1.0e-15);
    assert!((concentrated_rate / spread_rate - 24.0).abs() < 1.0e-12);
    assert!((concentrated_duration - 3_600.0).abs() < 1.0e-9);
    assert!((spread_duration - 86_400.0).abs() < 1.0e-9);
    assert_eq!(concentrated_hour, 7);
    assert_eq!(spread_hour, 0, "ties select the earliest hour");
}

#[test]
fn hourly_peak_fails_closed_when_weights_do_not_reconstruct_event_runoff() {
    let mut incomplete = [0.0; 24];
    incomplete[2] = 0.4;
    assert_eq!(
        crate::direct_runtime::test_hourly_peak_runoff_depth_rate_m_s(0.01, &incomplete)
            .expect_err("incomplete weights must fail closed"),
        DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "peak_runoff.hourly_weight_total"
        }
    );
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
        hourly_additional_supply_m: [0.0; 24],
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
    with_runon.hourly_additional_supply_m[2] = 0.004;
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
        hourly_additional_supply_m: [0.0; 24],
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
    inputs.hourly_additional_supply_m[6] = 0.005;
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

#[test]
fn dc01_zero_storage_capacity_preserves_all_supply_as_hourly_excess() {
    let inputs = DirectWb14InfiltrationProducerInputs {
        hourly_additional_supply_m: [0.0; 24],
        hyetograph: vec![DirectWb14HyetographInterval {
            start_s: 7_200.0,
            end_s: 10_800.0,
            intensity_m_s: 0.006 / 3_600.0,
        }],
        effective_conductivity_m_s: 1.0e-6,
        matric_potential_m: 0.2,
        storage_capacity_m: 0.0,
        depression_storage_capacity_m: 0.0,
    };
    let outcome = crate::direct_runtime::dc01_test_wb14_with_profile(&inputs)
        .expect("zero-capacity WB14 must compute");
    assert_eq!(outcome.state.cumulative_infiltration_m, 0.0);
    assert!((outcome.hourly_excess_m[2] - 0.006).abs() < 1.0e-12);
    assert!(
        outcome
            .hourly_excess_m
            .iter()
            .enumerate()
            .all(|(hour, depth)| hour == 2 || *depth == 0.0)
    );
}
