use openwepp_sim_contract::closure::{
    ClosureSeverity, ClosureViolationDetails, ClosureViolationKind, check_balance_residual,
    check_equal_count, check_finite, check_min, check_range, check_unit_interval,
};

#[test]
fn finite_check_accepts_finite_values() {
    let result = check_finite("INV-WATBAL-001", "WB-OK-FINITE-001", "runoff", 1.25);

    assert!(result.is_ok());
}

#[test]
fn finite_check_rejects_nan_with_typed_violation() {
    let error = *check_finite("INV-WATBAL-001", "WB-E-FINITE-001", "runoff", f64::NAN)
        .expect_err("non-finite scalar must fail");

    assert_eq!(error.kind, ClosureViolationKind::NonFinite);
    assert_eq!(error.severity, ClosureSeverity::HardFail);
    assert_eq!(error.message_id, "WB-E-FINITE-001");
}

#[test]
fn min_check_rejects_negative_values_with_typed_violation() {
    let error = *check_min("INV-WATBAL-002", "WB-E-DOMAIN-001", "runoff", -0.01, 0.0)
        .expect_err("negative runoff must fail");

    assert_eq!(error.kind, ClosureViolationKind::DomainLowerBound);
    assert_eq!(error.subject, "runoff");
    assert_eq!(error.message_id, "WB-E-DOMAIN-001");
}

#[test]
fn range_and_unit_interval_checks_enforce_domain_bounds() {
    check_range("INV-WATBAL-004", "WB-OK-RANGE-001", "ws", 0.5, 0.0, 1.0)
        .expect("in-range scalar should pass");

    let range_error = check_range("INV-WATBAL-004", "WB-E-RANGE-001", "ws", 1.2, 0.0, 1.0)
        .expect_err("out-of-range scalar should fail");
    assert_eq!(range_error.kind, ClosureViolationKind::DomainRange);

    let interval_error = check_unit_interval("INV-WATBAL-004", "WB-E-UNIT-001", "ws", -0.2)
        .expect_err("unit interval should reject negative values");
    assert_eq!(interval_error.kind, ClosureViolationKind::DomainRange);
}

#[test]
fn residual_check_enforces_closure_tolerance() {
    check_balance_residual(
        "INV-WATBAL-001",
        "WB-OK-CLOSURE-001",
        "daily_balance",
        10.0,
        9.0,
        1.0,
        1e-9,
    )
    .expect("exact closure should pass");

    let error = *check_balance_residual(
        "INV-WATBAL-001",
        "WB-E-CLOSURE-001",
        "daily_balance",
        10.0,
        9.1,
        0.7,
        1e-6,
    )
    .expect_err("residual above tolerance should fail");

    assert_eq!(error.kind, ClosureViolationKind::ResidualExceeded);

    match error.details {
        ClosureViolationDetails::Residual {
            residual,
            tolerance,
            ..
        } => {
            assert!((residual - 0.2).abs() < 1e-12);
            assert!((tolerance - 1e-6).abs() < 1e-18);
        }
        other => panic!("unexpected closure details: {other:?}"),
    }
}

#[test]
fn count_check_rejects_cardinality_mismatch() {
    let error = *check_equal_count(
        "INV-SYSTEM-COUNT-001",
        "SYS-E-COUNT-001",
        "channel_count",
        4,
        3,
    )
    .expect_err("count mismatch should fail");

    assert_eq!(error.kind, ClosureViolationKind::CardinalityMismatch);
    assert_eq!(error.severity, ClosureSeverity::HardFail);
}
