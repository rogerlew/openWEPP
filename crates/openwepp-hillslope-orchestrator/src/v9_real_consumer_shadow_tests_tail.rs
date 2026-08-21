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
