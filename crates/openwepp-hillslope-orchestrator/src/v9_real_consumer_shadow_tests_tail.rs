/// Phase-1 evidence sweep for the model-specific positive-support domain.
/// This is intentionally ignored: it is an evidence generator, not a
/// release gate. A failed duration is caught at the actual LSE boundary.
#[test]
#[ignore = "deterministic support-domain evidence sweep"]
fn v11_support_domain_evidence_sweep() {
    let durations_ns = [
        1_800_000_000_000_u128,
        600_000_000_000,
        60_000_000_000,
        6_000_000_000,
        600_000_000,
        60_000_000,
        600_000_000,
        601_000_000,
        700_000_000,
        1_000_000_000,
        6_000_000,
        10_000_000,
        15_000_000,
        20_000_000,
        25_000_000,
        30_000_000,
        40_000_000,
        50_000_000,
        32_000_000,
        35_000_000,
        38_000_000,
        42_000_000,
        45_000_000,
        48_000_000,
        55_000_000,
        59_999_999,
        60_000_000,
        600_000,
        60_000,
        6_000,
        600,
        60,
        6,
        1,
    ];
    let (shadow, fixture) = v10_shadow_fixture();
    let interval = day_input(&fixture).intervals[0].clone();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for duration_ns in durations_ns {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = run_actual_v11_segments(&shadow, &interval, &[duration_ns], &[0.0]);
        }));
        eprintln!(
            "V11_SUPPORT_SWEEP fixture=v10_actual duration_ns={} result={}",
            duration_ns,
            if result.is_ok() { "PASS" } else { "LSE_REJECT" }
        );
    }
    for duration_ns in (30_000_000_u128..=60_000_000).step_by(1_000_000) {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = run_actual_v11_segments(&shadow, &interval, &[duration_ns], &[0.0]);
        }));
        eprintln!(
            "V11_SUPPORT_SWEEP fixture=v10_actual_boundary duration_ns={} result={}",
            duration_ns,
            if result.is_ok() { "PASS" } else { "LSE_REJECT" }
        );
    }
    std::panic::set_hook(previous_hook);
}

#[test]
fn imported_v10_snow_free_successor_installs_only_v10_inactive_anchor_authority() {
    let (shadow, _) = v10_shadow_fixture();
    let mut covered_predecessor = shadow.clone();
    covered_predecessor.inner.authority = CoveredColumnAuthority::V11SnowCovered;
    let mut expected = covered_predecessor.clone();
    expected.inner.authority = CoveredColumnAuthority::V10NonpositiveAssimilation;

    install_imported_v10_snow_free_authority(&mut covered_predecessor);

    assert_eq!(covered_predecessor, expected);
    assert_eq!(
        covered_predecessor.inner.authority,
        CoveredColumnAuthority::V10NonpositiveAssimilation
    );
    assert!(
        covered_predecessor
            .inner
            .authority
            .admits_nonpositive_assimilation()
    );
    assert_ne!(
        covered_predecessor.inner.authority,
        CoveredColumnAuthority::HistoricalV8
    );
    assert_ne!(
        covered_predecessor.inner.authority,
        CoveredColumnAuthority::V11SnowCovered,
        "the snow-free successor must not retain a Stage-3 lower-boundary authority"
    );
}

#[test]
fn repository_day_receipt_mismatch_discards_both_candidates() {
    let (mut shadow, fixture) = shadow_fixture();
    let mut production = fixture.hydrology.beginning_frame().clone();
    let production_before = production.clone();
    let shadow_before = shadow.clone();
    let shadow_input = day_input(&fixture);
    let mut actual_input = production_day_input();
    actual_input.precipitation_m = f64::from_bits(actual_input.precipitation_m.to_bits() ^ 1);
    let mut published_row_count = 0_usize;
    let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_stream_with_v9_real_consumer_shadow(
            &mut production,
            DirectPublicationRunMetadata {
                run_name: "v9-provider-poison".into(),
                runtime_selection: "direct-default-off-shadow-test".into(),
                output_policy: "test-only".into(),
            },
            |_, _, _| Ok(actual_input.clone()),
            |_, _, _| Ok(shadow_input.clone()),
            |_, _| {
                published_row_count += 1;
                Ok(())
            },
            &mut shadow,
        )
        .expect_err("repository receipt mismatch");
    assert!(matches!(
        error,
        crate::DirectRuntimeError::V9RealConsumerShadowFailure {
            category: "identity",
            ..
        }
    ));
    assert_eq!(published_row_count, 0);
    assert_eq!(production, production_before);
    assert_eq!(shadow, shadow_before);
}

#[test]
fn downstream_scheduler_failure_discards_production_and_complete_shadow_candidate() {
    let (mut shadow, fixture) = shadow_fixture();
    let mut production = fixture.hydrology.beginning_frame().clone();
    let production_before = production.clone();
    let shadow_before = shadow.clone();
    let shadow_input = day_input(&fixture);
    let production_input = production_day_input();
    let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_stream_with_v9_real_consumer_shadow(
            &mut production,
            DirectPublicationRunMetadata {
                run_name: "v9-shadow-rollback".into(),
                runtime_selection: "direct-default-off-shadow-test".into(),
                output_policy: "test-only".into(),
            },
            |_, _, _| Ok(production_input.clone()),
            |_, _, _| Ok(shadow_input.clone()),
            |_, _| {
                Err(crate::DirectRuntimeError::PublicationSinkFailure {
                    detail: "injected after shadow day".into(),
                })
            },
            &mut shadow,
        )
        .expect_err("injected downstream failure");
    assert!(matches!(
        error,
        crate::DirectRuntimeError::PublicationSinkFailure { .. }
    ));
    assert_eq!(production, production_before);
    assert_eq!(shadow, shadow_before);
}

#[test]
fn active_routing_is_typed_unsupported_before_any_shadow_or_production_change() {
    let (mut shadow, fixture) = shadow_fixture();
    let mut production = fixture.hydrology.beginning_frame().clone();
    production.laned_active = Some(Box::new(DirectLanedActiveConfig {
        lanes: vec![DirectLanedActiveLaneConfig {
            slplen_m: 10.0,
            width_m: 10.0,
            mean_gradient: 0.01,
            skin_friction_coefficient_ko: 500.0,
            form_drag_coefficient: 0.0,
            roughness_element_height_m: 0.0,
            roughness_concentration: 0.0,
            vegetation_drag_coefficient: 0.0,
            canopy_height_m: None,
        }],
        mesh_policy: DirectLanedActiveMeshPolicy::FixedCells { cells: 10 },
        max_dt_s: 300.0,
        trace_enabled: false,
        trace_detail_filter: None,
        step_trace_enabled: false,
    }));
    let production_before = production.clone();
    let shadow_before = shadow.clone();
    let shadow_input = day_input(&fixture);
    let production_input = production_day_input();
    let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_stream_with_v9_real_consumer_shadow(
            &mut production,
            DirectPublicationRunMetadata {
                run_name: "v9-active-unsupported".into(),
                runtime_selection: "direct-default-off-shadow-test".into(),
                output_policy: "test-only".into(),
            },
            |_, _, _| Ok(production_input.clone()),
            |_, _, _| Ok(shadow_input.clone()),
            |_, _| Ok(()),
            &mut shadow,
        )
        .expect_err("active routing must reject");
    assert!(matches!(
        error,
        crate::DirectRuntimeError::DirectDomainViolation {
            field: "v9_shadow.laned_active_unsupported"
        }
    ));
    assert_eq!(production, production_before);
    assert_eq!(shadow, shadow_before);
}
