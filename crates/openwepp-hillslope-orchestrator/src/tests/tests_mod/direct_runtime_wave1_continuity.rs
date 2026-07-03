//! Wave-1 sediment-continuity solver contract tests (SC-SED-001,
//! Increment-1 of the erosion port). Covers the handoff §5 unit surface:
//! RK4 vs analytic constant-coefficient solution, every `mshear` regime,
//! the un-clamped `tauchk` reconciliation, the interrill floor, the
//! analytic deposition kernels, a detachment->deposition transition, the
//! conservation round-trip, activation gating, fail-closed operands, and
//! the frame-level erosion span consuming the new state.

use super::direct_runtime_test_lock;
use crate::{
    DIRECT_WAVE1_GRID_POINTS, DirectDayFrame, DirectPeakRunoffShadowProjection, DirectRunIdentity,
    DirectRuntimeError, DirectWave1ContinuityInputs, DirectWave1SlopeSegment, Wave1ShearRegime,
    compute_direct_wave1_continuity, derive_wave1_slope_segments, wave1_depc, wave1_depend,
    wave1_depeqs, wave1_runge_step, wave1_xcrit,
};

/// Crafted single-segment concave OFE: normalized slope `s*(x) = -2x + 2`
/// (unit average), `qostar = 0`, so the shear polynomial is
/// `p(x) = -2x^2 + 2x` — transport capacity rises to mid-slope and falls
/// to zero at the toe, forcing a detachment -> deposition transition.
fn crafted_wave1_inputs() -> DirectWave1ContinuityInputs {
    DirectWave1ContinuityInputs {
        enabled: true,
        segments: vec![DirectWave1SlopeSegment {
            xu: 0.0,
            xl: 1.0,
            a: -2.0,
            b: 2.0,
        }],
        peakro_m_s: 1.0e-5,
        runoff_depth_m: 0.02,
        qin_m2_s: 0.0,
        efflen_m: 100.0,
        slplen_m: 100.0,
        cntlen_m: 100.0,
        rspace_m: 1.0,
        width_m: 0.2,
        field_width_m: 30.0,
        effdrn_s: 2000.0,
        effdrr_s: 1000.0,
        kr_s_m: 0.01,
        kradjf: 1.0,
        shcrit_pa: 1.0,
        tcadjf: 0.5,
        detinr_kg_s_m2: 0.001,
        shrsol_pa: 2.0,
        tcend_kg_s_m: 10.0,
        ktrato: 1.0,
        veleff_m_s: 0.02,
        beta: 0.5,
        strldn: 0.0,
        surface_frozen: false,
        theta_suppressed: false,
    }
}

#[test]
fn wave1_rk4_matches_analytic_constant_coefficient_solution() {
    // Constant coefficients: shear polynomial c = 1 (tau_f = 1), transport
    // ctc = 1, ktrato = 1 (Tc = 1), eata = 2, tauc = 0.5 (Dc = 1),
    // theta = 0.3. The continuity ODE is dG/dx = 1.3 - G with G(0) = 0,
    // whose exact solution is G(x) = 1.3 * (1 - exp(-x)).
    let (a, b, c) = (0.0, 0.0, 1.0);
    let (atc, btc, ctc) = (0.0, 0.0, 1.0);
    let (eata, tauc, theta, ktrato) = (2.0, 0.5, 0.3, 1.0);
    let mut load = 0.0;
    for step in 0..100_u32 {
        let x = f64::from(step) * 0.01;
        load = wave1_runge_step(
            a, b, c, atc, btc, ctc, eata, tauc, theta, ktrato, 0.01, x, load,
        );
    }
    let analytic = 1.3 * (1.0 - (-1.0_f64).exp());
    assert!(
        (load - analytic).abs() <= 1.0e-9,
        "RK4 101-point march must match the analytic constant-coefficient \
         solution to 1e-9: rk4={load}, analytic={analytic}"
    );
}

#[test]
fn wave1_rk4_interrill_floor_engages_when_rk_undershoots() {
    // Load far above transport capacity: the raw RK4 result drops below
    // ldold + theta*dx, so the `runge.for:219` floor must clamp exactly.
    let (a, b, c) = (0.0, 0.0, 1.0);
    let (atc, btc, ctc) = (0.0, 0.0, 1.0);
    let (eata, tauc, theta, ktrato) = (2.0, 0.5, 0.3, 1.0);
    let ldold = 5.0;
    let ldnew = wave1_runge_step(
        a, b, c, atc, btc, ctc, eata, tauc, theta, ktrato, 0.01, 0.0, ldold,
    );
    assert_eq!(
        ldnew,
        ldold + theta * 0.01,
        "interrill floor must clamp the RK4 undershoot to ldold + theta*dx"
    );
}

#[test]
fn wave1_xcrit_classifies_all_five_regimes() {
    // Rising cross on a uniform segment: xc1 = tauchk / b.
    let rising = wave1_xcrit(0.0, 1.0, 0.0, 0.5, 0.0, 1.0).expect("rising xcrit");
    assert_eq!(rising.regime, Wave1ShearRegime::RisingCross);
    let expected_xc1 = 0.5_f64.powf(1.5);
    assert!((rising.xc1 - expected_xc1).abs() < 1.0e-12);

    // Above-critical throughout (negative tauchk on a uniform segment).
    let above = wave1_xcrit(0.0, 1.0, 0.5, 0.5, 0.0, 1.0).expect("above xcrit");
    assert_eq!(above.regime, Wave1ShearRegime::AboveCritical);

    // Below-critical throughout.
    let below = wave1_xcrit(0.0, 1.0, 0.0, 2.0, 0.0, 1.0).expect("below xcrit");
    assert_eq!(below.regime, Wave1ShearRegime::BelowCritical);

    // Double cross on the concave crafted profile: p(x) = -2x^2 + 2x with
    // tauc = 0.25 crosses at x = (2 -+ sqrt(3)) / 4.
    let double = wave1_xcrit(-2.0, 2.0, 0.0, 0.25, 0.0, 1.0).expect("double xcrit");
    assert_eq!(double.regime, Wave1ShearRegime::DoubleCross);
    let tauchk = 0.25_f64.powf(1.5);
    let part = (4.0_f64 + 4.0 * (-2.0) * tauchk).sqrt();
    let xc1_expected = (2.0 - part) / 4.0;
    let xc2_expected = (2.0 + part) / 4.0;
    assert!((double.xc1 - xc1_expected).abs() < 1.0e-12);
    assert!((double.xc2 - xc2_expected).abs() < 1.0e-12);
}

#[test]
fn wave1_xcrit_falling_cross_uses_unclamped_tauchk() {
    // Decreasing shear p(x) = 1 - x^2 with tauc = 0.8: tauchk = 0.8^1.5 - 1
    // is negative. The un-clamped baseline (`xcrit.for:82` clamp commented
    // out) places the crossing at x = sqrt(1 - 0.8^1.5) ~ 0.5334; a
    // re-clamped tauchk (the deleted erod19 behavior) would collapse the
    // roots to zero and misclassify the crossing.
    let classification = wave1_xcrit(-1.0, 0.0, 1.0, 0.8, 0.0, 1.0).expect("falling xcrit");
    assert_eq!(classification.regime, Wave1ShearRegime::FallingCross);
    let expected = (1.0 - 0.8_f64.powf(1.5)).sqrt();
    assert!(
        (classification.xc1 - expected).abs() < 1.0e-9,
        "un-clamped tauchk crossing expected at {expected}, observed {}",
        classification.xc1
    );
    assert!(
        (classification.xc1 - expected).abs() < (classification.xc1 - 0.0).abs(),
        "clamped-tauchk root (x = 0) must not be reproduced"
    );
}

#[test]
fn wave1_deposition_constant_recovers_onset_rate() {
    // `depc` is defined so the analytic rate at the onset point equals the
    // incoming deposition rate: D(xu) = du.
    let (atc, btc) = (-1.5, 1.2);
    let (phi, theta, ktrato, qostar) = (40.0, 0.004, 1.05, 0.0);
    let (xu, du) = (0.3, -0.08);
    let cdep = wave1_depc(xu, atc, btc, phi, theta, du, ktrato, qostar);
    let rate_at_onset = wave1_depeqs(xu, cdep, atc, btc, phi, theta, xu, ktrato, qostar);
    assert!(
        (rate_at_onset - du).abs() < 1.0e-12,
        "depeqs at the onset point must return the depc-anchored rate: \
         expected {du}, observed {rate_at_onset}"
    );
}

#[test]
fn wave1_deposition_end_zeroes_the_analytic_rate() {
    // Transport capacity rising downslope (btc > 0): deposition entering at
    // the segment top dies out where D(x) crosses zero; `depend` must find
    // that point inside the segment with the legacy 0.001 residual bound.
    let (atc, btc) = (0.0, 1.0);
    let (phi, theta, ktrato, qostar) = (25.0, 0.0, 1.0, 0.0);
    let (xu, xl, du) = (0.05, 1.0, -0.2);
    let cdep = wave1_depc(xu, atc, btc, phi, theta, du, ktrato, qostar);
    let xdend = wave1_depend(xu, xl, atc, btc, cdep, phi, theta, ktrato, qostar);
    assert!(
        xdend > xu && xdend < xl,
        "xdend must fall inside the segment"
    );
    let residual = wave1_depeqs(xu, cdep, atc, btc, phi, theta, xdend, ktrato, qostar);
    assert!(
        residual.abs() <= 1.0e-3,
        "deposition rate at xdend must satisfy the depend residual bound, \
         observed {residual}"
    );
}

#[test]
fn wave1_continuity_transition_detachment_then_deposition() {
    let state = compute_direct_wave1_continuity(&crafted_wave1_inputs())
        .expect("crafted concave profile must solve");
    assert!(state.active);

    // Hand-checked `param.for` normalization (INV-SED-007).
    assert!((state.eta - 0.2).abs() < 1.0e-12, "eta = {}", state.eta);
    assert!(
        (state.taucn - 0.25).abs() < 1.0e-12,
        "taucn = {}",
        state.taucn
    );
    assert!(
        (state.theta - 0.005).abs() < 1.0e-12,
        "theta = {}",
        state.theta
    );
    assert!((state.phi - 1000.0).abs() < 1.0e-9, "phi = {}", state.phi);
    assert!((state.qostar - 0.0).abs() < 1.0e-12);

    // Regime structure: rill detachment on the upper limb, deposition on
    // the falling-transport limb near the toe (INV-SED-001 signs).
    let mid = 40;
    assert!(
        state.detach[mid] > 0.0,
        "mid-slope must detach: detach[{mid}] = {}",
        state.detach[mid]
    );
    let deposition_points = (1..DIRECT_WAVE1_GRID_POINTS)
        .filter(|&i| state.detach[i] < 0.0)
        .count();
    assert!(
        deposition_points > 0,
        "falling transport limb must produce deposition points"
    );
    // INV-SED-003: deposition only where load exceeds transport capacity
    // (the analytic construction G = Tc - D*(x+q*)/phi gives G >= Tc
    // wherever the rate D is negative, for q* >= 0).
    for i in 1..DIRECT_WAVE1_GRID_POINTS {
        if state.detach[i] < -1.0e-12 {
            assert!(
                state.load[i] >= state.tcap[i] - 1.0e-9,
                "deposition point {i} must sit on the G >= Tc branch: \
                 load = {}, tcap = {}",
                state.load[i],
                state.tcap[i]
            );
        }
        assert!(state.load[i] >= 0.0, "loads must stay nonnegative");
        assert!(
            state.tcap[i] >= 0.0,
            "transport capacity must be clamped >= 0"
        );
    }

    assert!(state.total_detachment_kg > 0.0);
    assert!(state.total_deposition_kg > 0.0);
}

#[test]
fn wave1_continuity_conservation_round_trip() {
    let inputs = crafted_wave1_inputs();
    let state = compute_direct_wave1_continuity(&inputs).expect("crafted profile must solve");
    assert!(state.active);

    // Hard identity: exported - inflow = detachment - deposition, with the
    // per-metre totals recovered from the published kg totals.
    let detach_kg_m = state.total_detachment_kg / inputs.field_width_m;
    let depos_kg_m = state.total_deposition_kg / inputs.field_width_m;
    let identity_residual =
        (state.exported_sediment_kg_m - state.inflow_sediment_kg_m) - (detach_kg_m - depos_kg_m);
    let scale = state
        .exported_sediment_kg_m
        .abs()
        .max(detach_kg_m.abs())
        .max(1.0e-9);
    assert!(
        identity_residual.abs() <= 1.0e-9 * scale,
        "publication conservation identity residual {identity_residual} \
         exceeds tolerance at scale {scale}"
    );
    assert!(
        state.publication_closure_residual_kg_m.abs() <= 1.0e-9 * scale,
        "reported closure residual must satisfy the hard gate"
    );

    // Continuity flux residual (INV-SED-001) must be within the named
    // discretization bound and reported.
    assert!(state.flux_closure_scale > 0.0);
    assert!(
        state.flux_closure_residual <= 1.0e-3 * state.flux_closure_scale,
        "flux residual {} exceeds the discretization gate at scale {}",
        state.flux_closure_residual,
        state.flux_closure_scale
    );
}

#[test]
fn wave1_continuity_inactive_without_runoff_or_below_event_gate() {
    let mut no_runoff = crafted_wave1_inputs();
    no_runoff.runoff_depth_m = 0.0;
    no_runoff.peakro_m_s = 0.0;
    let state = compute_direct_wave1_continuity(&no_runoff).expect("no-runoff day must be inert");
    assert!(!state.active);
    assert_eq!(state.total_detachment_kg, 0.0);

    // `contin.for:977` passby: both bounds must be crossed to route.
    let mut tiny_event = crafted_wave1_inputs();
    tiny_event.runoff_depth_m = 0.005;
    tiny_event.peakro_m_s = 1.0e-6;
    let state = compute_direct_wave1_continuity(&tiny_event).expect("sub-gate event must be inert");
    assert!(!state.active);

    let mut disabled = crafted_wave1_inputs();
    disabled.enabled = false;
    let state = compute_direct_wave1_continuity(&disabled).expect("disabled payload is inert");
    assert!(!state.active);
}

#[test]
fn wave1_continuity_fails_closed_on_missing_operands() {
    // Missing transport normalization (ktrato) must be a typed hard error,
    // not a defaulted solve.
    let mut missing_ktrato = crafted_wave1_inputs();
    missing_ktrato.ktrato = 0.0;
    assert!(matches!(
        compute_direct_wave1_continuity(&missing_ktrato),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));

    // INV-SED-006 sandy adjustment floor.
    let mut bad_tcadjf = crafted_wave1_inputs();
    bad_tcadjf.tcadjf = 0.1;
    assert!(matches!(
        compute_direct_wave1_continuity(&bad_tcadjf),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));

    // Segments must tile [0, 1].
    let mut short_profile = crafted_wave1_inputs();
    short_profile.segments = vec![DirectWave1SlopeSegment {
        xu: 0.0,
        xl: 0.5,
        a: 0.0,
        b: 1.0,
    }];
    assert!(matches!(
        compute_direct_wave1_continuity(&short_profile),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));

    let mut empty_profile = crafted_wave1_inputs();
    empty_profile.segments = Vec::new();
    assert!(matches!(
        compute_direct_wave1_continuity(&empty_profile),
        Err(DirectRuntimeError::DirectDomainViolation { .. })
    ));
}

#[test]
fn wave1_profil_segment_fit_matches_legacy_normalization() {
    // `profil.for`: sstar = slope/avgslp, xstar = x/slen, linear fit per
    // segment. Two-segment profile with average slope 0.3.
    let points = [(0.0, 0.4), (50.0, 0.4), (100.0, 0.2)];
    // avgslp = total drop / length = (0.4*50 + 0.3*50) / 100 = 0.35.
    let avgslp = 0.35;
    let segments =
        derive_wave1_slope_segments(&points, 100.0, avgslp).expect("valid profile must fit");
    assert_eq!(segments.len(), 2);
    assert!((segments[0].xu - 0.0).abs() < 1.0e-12);
    assert!((segments[0].xl - 0.5).abs() < 1.0e-12);
    // Uniform first segment: a = 0, b = 0.4/0.35.
    assert!(segments[0].a.abs() < 1.0e-12);
    assert!((segments[0].b - 0.4 / 0.35).abs() < 1.0e-12);
    // Falling second segment: a = (0.2/0.35 - 0.4/0.35) / 0.5.
    let sstar_u = 0.4 / 0.35;
    let sstar_l = 0.2 / 0.35;
    let expected_a = (sstar_l - sstar_u) / 0.5;
    assert!((segments[1].a - expected_a).abs() < 1.0e-12);
    assert!((segments[1].b - (sstar_u - expected_a * 0.5)).abs() < 1.0e-12);
}

#[test]
fn wave1_span_publishes_continuity_totals_through_the_frame() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let identity = DirectRunIdentity::new(7, 2637, 1, 1)
        .expect("valid direct publication identity should construct");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("valid direct day should construct");

    // The runoff authority overrides peakro/runoff/effdrn from the WB16
    // shadow projection (r7d8); leave them zeroed in the seed payload.
    let mut continuity = crafted_wave1_inputs();
    continuity.peakro_m_s = 0.0;
    continuity.runoff_depth_m = 0.0;
    continuity.effdrn_s = 0.0;
    *day.erosion_inputs.wave1_continuity = continuity;
    day.peak_runoff_shadow_projection = Some(DirectPeakRunoffShadowProjection {
        lane_index: 0,
        day_index: 0,
        q_runoff_m: 0.02,
        peak_runoff_m3_s: 1.0e-5,
        runoff_duration_s: 2000.0,
        method_branch: 1.0,
        tstar: 0.0,
        qpstar: 0.0,
        vstar: 0.0,
    });

    let report = day
        .run_r7d6_erosion_span()
        .expect("wave1 continuity span should run");
    assert!(report.erosion_shadow_projection.wave1_active);
    assert!(report.erosion_shadow_projection.publication_authority);
    let publication = report.erosion_shadow_projection.publication;
    let total_detachment = publication
        .total_detachment_kg
        .expect("wave1 continuity must publish total detachment");
    let total_deposition = publication
        .total_deposition_kg
        .expect("wave1 continuity must publish total deposition");
    assert!(
        total_detachment > 0.0,
        "storm-day crafted profile must publish nonzero detachment"
    );
    assert!(total_deposition > 0.0);
    let state = day
        .erosion
        .wave1_continuity
        .as_ref()
        .expect("continuity state must be committed to the frame");
    assert!(state.active);
    assert!((state.total_detachment_kg - total_detachment).abs() < 1.0e-12);
}

#[test]
fn wave1_span_requires_peak_runoff_upstream_when_enabled() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let identity = DirectRunIdentity::new(7, 2637, 1, 1)
        .expect("valid direct publication identity should construct");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("valid direct day should construct");
    *day.erosion_inputs.wave1_continuity = crafted_wave1_inputs();
    day.peak_runoff_shadow_projection = None;
    assert!(matches!(
        day.run_r7d6_erosion_span(),
        Err(DirectRuntimeError::MissingDirectUpstream { .. })
    ));
}
