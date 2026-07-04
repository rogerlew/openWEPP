//! Wave-1 sediment-continuity solver contract tests (SC-SED-001,
//! Increment-1 of the erosion port). Covers the handoff §5 unit surface:
//! RK4 vs analytic constant-coefficient solution, every `mshear` regime,
//! the un-clamped `tauchk` reconciliation, the interrill floor, the
//! analytic deposition kernels, a detachment->deposition transition, the
//! conservation round-trip, activation gating, fail-closed operands, and
//! the frame-level erosion span consuming the new state.

use super::direct_runtime_test_lock;
use crate::direct_runtime::direct_wave1_publication_projection;
use crate::{
    DIRECT_WAVE1_GRID_POINTS, DirectDayFrame, DirectPeakRunoffShadowProjection, DirectRunIdentity,
    DirectRuntimeError, DirectWave1ContinuityInputs, DirectWave1DailyState, DirectWave1OperandSeed,
    DirectWave1SlopeSegment, ErosionConsolidationBaselines, ErosionFrostRegime,
    ErosionParticleClass, Wave1ShearRegime, assemble_wave1_continuity_inputs_quantum,
    compute_direct_wave1_continuity, compute_direct_wave1_continuity_quantum,
    derive_wave1_slope_segments, wave1_depc, wave1_depend, wave1_depeqs, wave1_runge_step,
    wave1_xcrit,
};

/// Crafted ENABLED operand seed (real-shaped forest-class operands) for
/// assembly-level quantum tests: uniform 30% slope, clay-loam-class
/// erodibility, the crafted composition.
fn crafted_enabled_seed() -> DirectWave1OperandSeed {
    DirectWave1OperandSeed {
        enabled: true,
        is_cropland: false,
        segments: vec![DirectWave1SlopeSegment {
            xu: 0.0,
            xl: 1.0,
            a: 0.0,
            b: 1.0,
        }],
        slplen_m: 100.0,
        efflen_m: 100.0,
        cntlen_m: 100.0,
        rspace_m: 1.0,
        field_width_m: 30.0,
        avg_slope: 0.30,
        slpend: 0.30,
        sand: 0.25,
        classes: crafted_particle_classes(),
        veleff_m_s: 0.005,
        baselines: ErosionConsolidationBaselines {
            kicrat: 1.0,
            krcrat: 1.0,
            tccrat: 1.0,
            bconsd: 0.02,
        },
        kr_s_m: 5.0e-4,
        ki: 1.0e6,
        shcrit_pa: 1.0,
        hmax_m: 0.0,
        flivmx: 0.0,
        random_roughness_m: 0.01,
        initial_daydis: 100.0,
    }
}

/// Bare-cover daily state skeleton for quantum-assembly tests.
fn crafted_daily_state() -> DirectWave1DailyState {
    DirectWave1DailyState {
        peakro_m_s: 0.0,
        runoff_depth_m: 0.0,
        effdrn_s: 3600.0,
        qin_m2_s: 0.0,
        excess_intervals: Vec::new(),
        canopy_cover_fraction: 0.0,
        canopy_height_m: 0.0,
        interrill_cover_fraction: 0.0,
        rill_cover_fraction: 0.0,
        live_root_mass_kg_m2: 0.0,
        dead_root_mass_kg_m2: 0.0,
        buried_residue_mass_kg_m2: 0.0,
        random_roughness_m: 0.01,
        rill_width_prior_m: 0.0,
        days_since_disturbance: 100.0,
        frost_regime: ErosionFrostRegime::Unfrozen,
        theta_suppressed: false,
        beta: 1.0,
        strldn: 0.0,
    }
}

/// Crafted five-class `prtcmp`-shaped composition (fractions sum to 1);
/// diameters/densities/fall velocities are physically-shaped fillers —
/// the publication split reads only `frac`.
fn crafted_particle_classes() -> [ErosionParticleClass; 5] {
    let dia_m = [2.0e-6, 1.0e-5, 3.0e-5, 3.0e-4, 2.0e-4];
    let spg = [2.60, 2.65, 1.80, 1.60, 2.65];
    let frac = [0.05, 0.35, 0.25, 0.20, 0.15];
    let fall_m_s = [1.0e-6, 5.0e-5, 4.0e-4, 2.0e-2, 2.5e-2];
    core::array::from_fn(|i| ErosionParticleClass {
        dia_m: dia_m[i],
        spg: spg[i],
        frac: frac[i],
        fall_m_s: fall_m_s[i],
    })
}

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
fn wave1_continuity_inert_days_do_not_require_routed_operands() {
    // Production 1b shape (Codex review finding): on non-routed days the
    // runtime supplies zeroed sediment operands — WB16 publishes
    // `runoff_duration_s = 0` without runoff — and the solver must return
    // the inactive state, not a typed operand error. Legacy ordering:
    // `contin.for` gates on norun/passby BEFORE frcfac/xinflo/param run.

    // Plain no-runoff day: everything zeroed except the enable flag.
    let no_runoff_day = DirectWave1ContinuityInputs {
        enabled: true,
        ..DirectWave1ContinuityInputs::zero()
    };
    let state = compute_direct_wave1_continuity(&no_runoff_day)
        .expect("no-runoff day with zeroed routed operands must be inert");
    assert!(!state.active);
    assert_eq!(state.total_detachment_kg, 0.0);

    // WB16 tiny-average-rate floor shape: q_runoff = 1e-8 m,
    // peakro = 3.63e-8 m/s, runoff_duration_s = 0 (the real WB16 floor
    // branch output) — below both passby bounds, so inert.
    let wb16_floor_day = DirectWave1ContinuityInputs {
        enabled: true,
        runoff_depth_m: 1.0e-8,
        peakro_m_s: 3.63e-8,
        effdrn_s: 0.0,
        ..DirectWave1ContinuityInputs::zero()
    };
    let state = compute_direct_wave1_continuity(&wb16_floor_day)
        .expect("WB16 floor-rate day with zeroed routed operands must be inert");
    assert!(!state.active);

    // The activation operands themselves stay fail-closed even on days
    // that would gate to inert: a NaN runoff is a typed error, never a
    // silent pass through the `<= 0` activation branch.
    let nan_runoff_day = DirectWave1ContinuityInputs {
        enabled: true,
        runoff_depth_m: f64::NAN,
        ..DirectWave1ContinuityInputs::zero()
    };
    assert!(matches!(
        compute_direct_wave1_continuity(&nan_runoff_day),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
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

    // SC-SED-001 1b-C: the per-day assembly now owns the full
    // `wave1_continuity` population (including the runoff authority), and it
    // no-ops when the operand seed is disabled (as here). This test crafts
    // the continuity directly, so the storm runoff authority is set on the
    // crafted payload (matching the shadow projection below) rather than
    // relying on the removed Increment-1 r7d8 runoff-threading stopgap.
    let mut continuity = crafted_wave1_inputs();
    continuity.peakro_m_s = 1.0e-5;
    continuity.runoff_depth_m = 0.02;
    continuity.effdrn_s = 2000.0;
    *day.erosion_inputs.wave1_continuity = continuity;
    // E.1: an active continuity payload implies a seeded class table in
    // production (the assembly builds `enabled` from the seed); the
    // publication projection fail-closes on an unseeded composition.
    day.erosion_inputs.wave1_operand_seed.classes = crafted_particle_classes();
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

    // E.1 per-class publication: `sedcon_i = frac_i * conc` (the
    // `sloss.for:305-317` composition split) and the class sum conserves
    // the scalar toe concentration.
    let scalar_concentration = publication
        .hbp_sediment_concentration_kg_m3
        .expect("wave1 continuity must publish the scalar toe concentration");
    assert!(scalar_concentration > 0.0);
    let per_class = publication
        .sediment_concentration_kg_m3
        .expect("wave1 continuity must publish the per-class concentrations");
    let classes = crafted_particle_classes();
    let mut class_sum = 0.0;
    for (index, concentration) in per_class.iter().enumerate() {
        assert!(
            (concentration - classes[index].frac * scalar_concentration).abs() < 1.0e-12,
            "class {index} concentration must be frac * scalar concentration"
        );
        class_sum += concentration;
    }
    assert!(
        (class_sum - scalar_concentration).abs() < 1.0e-9 * scalar_concentration.max(1.0),
        "per-class concentrations must conserve the scalar toe concentration \
         (sum={class_sum}, scalar={scalar_concentration})"
    );
}

#[test]
fn wave1_full_reinfiltration_quantum_deposits_incoming_load_without_clamp() {
    // ADR-0036 D1 acceptance driver: an hour whose incoming runon fully
    // reinfiltrates (`qout = 0`, `qin > 0`) must SOLVE and deposit the
    // incoming sediment load — the legacy `xinflo.for:206` branch — with
    // no clamp anywhere in the path.
    let seed = crafted_enabled_seed();
    let mut daily = crafted_daily_state();
    daily.qin_m2_s = 1.0e-4;
    daily.strldn = 1.5;

    let inputs = assemble_wave1_continuity_inputs_quantum(&seed, &daily, true)
        .expect("full-reinfiltration quantum must assemble routed operands");
    assert!(
        inputs.shrsol_pa > 0.0,
        "qin-basis rill hydraulics must resolve (xinflo.for:206 qshear = qin*rspace)"
    );
    let state = compute_direct_wave1_continuity_quantum(&inputs, true)
        .expect("full-reinfiltration quantum must solve fail-closed-clean");
    assert!(state.active, "qin > 0 must activate the solve");
    assert!(
        state.total_deposition_kg > 0.0,
        "the reinfiltrating quantum must deposit incoming load, got tdep={}",
        state.total_deposition_kg
    );
    assert!(
        state.exported_sediment_kg_m <= state.inflow_sediment_kg_m,
        "full reinfiltration cannot export more than flowed in \
         (export={}, inflow={})",
        state.exported_sediment_kg_m,
        state.inflow_sediment_kg_m
    );
    // Telescoping conservation (the in-solve 1e-9 gate already enforced it;
    // restate externally).
    let residual = (state.exported_sediment_kg_m - state.inflow_sediment_kg_m)
        - (state.total_detachment_kg - state.total_deposition_kg) / seed.field_width_m;
    assert!(
        residual.abs() <= 1.0e-9 * state.inflow_sediment_kg_m.max(1.0e-9),
        "conservation identity must hold on the reinfiltration quantum \
         (residual {residual})"
    );
}

#[test]
fn wave1_decreasing_flow_quantum_deposits_on_falling_limb() {
    // The recession case: positive local flow smaller than the inflow
    // (`0 < qout < qin`) — the falling-limb hour deposits through the
    // solver's negative-qostar machinery, without the INV-RUNOFFPART-031
    // interim clamp semantics anywhere.
    let seed = crafted_enabled_seed();
    let mut daily = crafted_daily_state();
    daily.runoff_depth_m = 0.004; // below the day-level passby: hour semantics
    daily.peakro_m_s = 1.0e-6; // qout = 1e-4 m2/s
    daily.qin_m2_s = 3.0e-4; // qin > qout: decreasing flow
    daily.strldn = 1.5;

    let inputs = assemble_wave1_continuity_inputs_quantum(&seed, &daily, true)
        .expect("decreasing-flow quantum must assemble routed operands");
    // theta-suppressed regime (`qout <= qin`, param.for:540): the interrill
    // supply operands are legitimately zero.
    assert!(inputs.detinr_kg_s_m2 == 0.0);
    let state = compute_direct_wave1_continuity_quantum(&inputs, true)
        .expect("decreasing-flow quantum must solve fail-closed-clean");
    assert!(state.active);
    assert!(
        state.total_deposition_kg > 0.0,
        "the falling-limb quantum must deposit, got tdep={}",
        state.total_deposition_kg
    );
}

#[test]
fn wave1_span_hourly_plan_aggregates_and_publishes_paired_surfaces() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    // Production-shaped hourly path: an ENABLED seed + a two-hour excess
    // profile through the R7D8 assembly and the R7D6 hourly solve. The
    // publication must carry the paired hourly surfaces with their
    // integral closures (INV-SED-014).
    let identity = DirectRunIdentity::new(7, 2637, 1, 1)
        .expect("valid direct publication identity should construct");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("valid direct day should construct");
    *day.erosion_inputs.wave1_operand_seed = crafted_enabled_seed();
    day.wb14_hourly_excess_m[10] = 0.012;
    day.wb14_hourly_excess_m[11] = 0.006;
    day.wb14_hourly_rainfall_m[10] = 0.015;
    day.wb14_hourly_rainfall_m[11] = 0.008;
    day.forcing.precipitation_m = 0.023;
    day.peak_runoff_shadow_projection = Some(DirectPeakRunoffShadowProjection {
        lane_index: 0,
        day_index: 0,
        q_runoff_m: 0.018,
        peak_runoff_m3_s: 1.0e-5,
        runoff_duration_s: 2000.0,
        method_branch: 1.0,
        tstar: 0.0,
        qpstar: 0.0,
        vstar: 0.0,
    });
    // The frost gate requires a valid surface soil layer.
    day.subsurface_compute.layer_state_after = vec![crate::DirectSubsurfaceLayerState {
        depth_m: 0.2,
        theta_m: 0.04,
        field_capacity_theta: 0.25,
        ..crate::DirectSubsurfaceLayerState::neutral()
    }];

    let report = day
        .run_r7d6_erosion_span()
        .expect("hourly wave1 span should run");
    assert!(report.erosion_shadow_projection.publication_authority);
    let publication = report.erosion_shadow_projection.publication;

    let weights = publication
        .hourly_runoff_fraction
        .expect("hourly runoff fraction must publish on the enabled lane");
    let weight_sum: f64 = weights.iter().sum();
    assert!(
        (weight_sum - 1.0).abs() < 1.0e-9,
        "weights must be unit-normalized (sum {weight_sum})"
    );
    assert!(weights[10] > 0.0 && weights[11] > 0.0);
    assert_eq!(
        weights.iter().filter(|w| **w > 0.0).count(),
        2,
        "only the excess hours carry weight"
    );

    let hourly_sediment = publication
        .hourly_sediment_mass_kg
        .expect("hourly sediment mass must publish on the enabled lane");
    let sediment_sum: f64 = hourly_sediment.iter().sum();
    assert!(sediment_sum > 0.0, "the storm hours must export sediment");
    let state = day
        .erosion
        .wave1_continuity
        .as_ref()
        .expect("continuity state must be committed");
    assert!(state.active);
    let exported_kg =
        state.exported_sediment_kg_m * day.erosion_inputs.wave1_operand_seed.field_width_m;
    assert!(
        (sediment_sum - exported_kg).abs() <= 1.0e-9 * exported_kg.max(1.0e-9),
        "Σ S_h must equal the day's exported mass (Σ={sediment_sum}, day={exported_kg})"
    );
    // Day totals are the hour sums; on this zero-deposition profile the
    // E.1 reconstruction identity holds in hourly form too.
    let tdet = publication.total_detachment_kg.expect("tdet published");
    assert!(tdet > 0.0);
    if state.total_deposition_kg == 0.0 {
        assert!(
            (tdet - exported_kg).abs() <= 1.0e-9 * tdet.max(1.0e-9),
            "zero-deposition day: detached mass equals exported mass"
        );
    }
}

#[test]
fn wave1_publication_splits_concentration_by_detached_composition() {
    let inputs = crafted_wave1_inputs();
    let state = compute_direct_wave1_continuity(&inputs)
        .expect("crafted storm-day inputs must produce an active solve");
    assert!(state.active);
    assert!(state.sediment_concentration_kg_m3 > 0.0);

    let classes = crafted_particle_classes();
    let publication = direct_wave1_publication_projection(&state, &inputs, &classes)
        .expect("seeded composition must project the per-class split");
    let per_class = publication
        .sediment_concentration_kg_m3
        .expect("projection must publish the per-class array");
    let mut class_sum = 0.0;
    for (index, concentration) in per_class.iter().enumerate() {
        assert!(
            (concentration - classes[index].frac * state.sediment_concentration_kg_m3).abs()
                < 1.0e-12
        );
        class_sum += concentration;
    }
    assert!((class_sum - state.sediment_concentration_kg_m3).abs() < 1.0e-12);
}

#[test]
fn wave1_publication_fails_closed_on_unseeded_class_composition() {
    let inputs = crafted_wave1_inputs();
    let state = compute_direct_wave1_continuity(&inputs)
        .expect("crafted storm-day inputs must produce an active solve");
    assert!(state.active);

    // A zeroed class table (the `DirectWave1OperandSeed::disabled()` shape)
    // must be a typed error, never a silent all-zero "composition".
    let zeroed = [ErosionParticleClass {
        dia_m: 0.0,
        spg: 0.0,
        frac: 0.0,
        fall_m_s: 0.0,
    }; 5];
    let result = direct_wave1_publication_projection(&state, &inputs, &zeroed);
    assert!(matches!(
        result,
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.publication.class_fraction_sum"
        })
    ));
}

#[test]
fn wave1_span_dry_day_publishes_zero_authority_without_routed_operands() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    // The 1b production dry-day shape (Codex review finding): continuity
    // enabled, routed sediment operands zeroed, and the WB16 projection
    // reporting no runoff (`runoff_duration_s = 0`). The span must
    // complete and publish zero totals with authority — not fail on the
    // routed-operand validator.
    let identity = DirectRunIdentity::new(7, 2637, 1, 1)
        .expect("valid direct publication identity should construct");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("valid direct day should construct");
    day.erosion_inputs.wave1_continuity.enabled = true;
    day.peak_runoff_shadow_projection = Some(DirectPeakRunoffShadowProjection {
        lane_index: 0,
        day_index: 0,
        q_runoff_m: 0.0,
        peak_runoff_m3_s: 0.0,
        runoff_duration_s: 0.0,
        method_branch: 0.0,
        tstar: 0.0,
        qpstar: 0.0,
        vstar: 0.0,
    });

    let report = day
        .run_r7d6_erosion_span()
        .expect("dry day with zeroed routed operands must complete the span");
    assert!(report.erosion_shadow_projection.publication_authority);
    assert_eq!(
        report
            .erosion_shadow_projection
            .publication
            .total_detachment_kg,
        Some(0.0)
    );
    let state = day
        .erosion
        .wave1_continuity
        .as_ref()
        .expect("continuity state must be committed even on inert days");
    assert!(!state.active);
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
