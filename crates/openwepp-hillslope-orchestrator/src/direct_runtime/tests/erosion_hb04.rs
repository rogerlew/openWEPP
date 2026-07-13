use super::*;

fn enrichment_inputs() -> super::super::Wave1EnrichmentInputs {
    let class = crate::ErosionParticleClass {
        dia_m: 1.0e-5,
        spg: 2.65,
        frac: 0.2,
        fall_m_s: 1.0e-4,
        frcly: 0.2,
        frslt: 0.4,
        frsnd: 0.4,
        frorg: 0.0,
    };
    super::super::Wave1EnrichmentInputs {
        classes: [class; 5],
        tcf1: [0.2; 5],
        fidel: [0.2; 5],
        ssasol: 5.0,
        inflow_fractions: Some([0.2; 5]),
    }
}

fn continuity_inputs() -> DirectWave1ContinuityInputs {
    DirectWave1ContinuityInputs {
        enabled: true,
        inter_ofe: None,
        enrichment: None,
        segments: vec![DirectWave1SlopeSegment {
            xu: 0.0,
            xl: 1.0,
            a: 0.0,
            b: 1.0,
        }],
        peakro_m_s: 1.0e-5,
        runoff_depth_m: 0.02,
        qin_m2_s: 1.0e-4,
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
        shrsol_pa: 1.0,
        tcend_kg_s_m: 1.0,
        ktrato: 1.0,
        veleff_m_s: 0.02,
        beta: 0.5,
        strldn: 0.0,
        surface_frozen: false,
        theta_suppressed: false,
    }
}

fn segment(a: f64, b: f64, c: f64, atc: f64, btc: f64, ctc: f64) -> Wave1SegmentCoefficients {
    Wave1SegmentCoefficients {
        xu: 0.0,
        xl: 1.0,
        a,
        b,
        c,
        atc,
        btc,
        ctc,
    }
}

fn drivers(qostar: f64, qout_m2_s: f64) -> Wave1Drivers {
    Wave1Drivers {
        eta: 1.0,
        taucn: 0.25,
        theta: 0.01,
        phi: 0.5,
        ktrato: 1.0,
        qostar,
        qout_m2_s,
        pkro: 1.0e-5,
        beta: 1.0,
    }
}

#[test]
fn enrichment_trace_load_floor_preserves_nonnegative_unit_composition() {
    let ldtop = 3.807_345_127_696_808e-12;
    let ldbot = 1.590_759_142_146_721_6e-19;
    let mut inputs = enrichment_inputs();
    inputs.tcf1 = [0.0; 5];
    inputs.fidel = [0.2; 5];
    inputs.inflow_fractions = Some([
        0.000_131_325_105_350_369_68,
        0.249_967_168_723_662_4,
        0.249_967_168_723_662_4,
        0.249_967_168_723_662_4,
        0.249_967_168_723_662_4,
    ]);
    let mut state = super::super::Wave1EnrichmentState::initialize(&inputs, true, true, ldtop);
    let operands = super::super::Wave1EnrichmentRegionOperands {
        atc: 0.0,
        btc: 0.0,
        ctc: 0.0,
        ktrato: 1.0,
        qostar: 1.0,
        theta: 0.0,
        beta: 0.5,
        pkro: 1.0e-4,
        qout_m2_s: 1.0e-5,
    };

    state
        .deposition_region(&inputs, &operands, 0.0, 1.0, ldtop, ldbot)
        .expect("trace deposition must retain a physical class composition");

    assert!(state.frcflw.iter().all(|fraction| fraction.is_finite()));
    assert!(
        state.frcflw.iter().all(|fraction| *fraction >= 0.0),
        "class fractions must remain nonnegative: {:?}",
        state.frcflw
    );
    let sum: f64 = state.frcflw.iter().sum();
    assert!((sum - 1.0).abs() <= 1.0e-12, "fraction sum was {sum}");
}

#[test]
fn erod_onset_characterizes_all_legacy_flags_and_bracket_updates() {
    let seg = segment(0.0, 0.0, 1.0, 0.0, 0.0, 1.0);
    let drv = drivers(0.0, 1.0e-3);
    for flag in [1_u8, 2, 5] {
        let (x, load) = wave1_erod_onset(&seg, 1.0, &drv, flag, 0.0, 0.5, 1.0, -0.5, 0.1);
        assert!(
            (0.0..=1.0).contains(&x),
            "flag {flag} onset must remain bracketed"
        );
        assert!(
            load.is_finite() && load >= 0.0,
            "flag {flag} load must remain physical"
        );
    }

    // Zero transport/load drives the flag-2 load floor and flag-1
    // transport floor without permitting a non-finite secant state.
    let zero = segment(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for flag in [1_u8, 2] {
        let (x, load) = wave1_erod_onset(&zero, 0.0, &drv, flag, 0.0, 1.0, 1.0, -1.0, 0.0);
        assert!(x.is_finite() && load.is_finite());
        assert!(load >= 0.0);
    }
}

#[test]
fn erod_march_characterizes_flow_end_zero_capacity_and_grid_end() {
    let mut flow_end = Wave1RouteGrid::new();
    flow_end.ilast = 19;
    flow_end.load[19] = 0.2;
    let flow_drv = drivers(-0.19, 0.0);
    let seg = segment(0.0, 1.0, 0.0, 0.0, 1.0, 0.0);
    let marched = wave1_erod_march(&mut flow_end, &seg, 1.0, &flow_drv, 0.19, 1.0, 20, 0.2);
    assert_eq!(marched.kflag, 4);
    assert_eq!(flow_end.region[20], Wave1PointRegion::FlowEnd);
    assert!(flow_end.load[20].abs() <= f64::EPSILON);

    let mut zero_capacity = Wave1RouteGrid::new();
    let zero_seg = segment(0.0, 1.0, 1.0, 0.0, 0.0, 0.0);
    let zero_drv = Wave1Drivers {
        theta: 0.0,
        ..drivers(0.0, 1.0e-3)
    };
    let marched = wave1_erod_march(
        &mut zero_capacity,
        &zero_seg,
        1.0,
        &zero_drv,
        0.0,
        0.02,
        1,
        0.0,
    );
    assert_eq!(marched.kflag, 3);
    assert!(zero_capacity.load[1].abs() <= f64::EPSILON);

    let mut at_end = Wave1RouteGrid::new();
    at_end.ilast = DIRECT_WAVE1_GRID_POINTS - 2;
    at_end.load[at_end.ilast] = 0.1;
    let marched = wave1_erod_march(
        &mut at_end,
        &seg,
        1.0,
        &drivers(0.0, 1.0e-3),
        0.99,
        1.0,
        100,
        0.1,
    );
    assert_eq!(marched.currpt, DIRECT_WAVE1_GRID_POINTS - 1);
    assert!(at_end.load[100].is_finite());
}

#[test]
fn erod_characterizes_segment_end_and_deposition_onset_outcomes() {
    let detaching = segment(0.0, 1.0, 0.1, 0.0, 1.0, 0.1);
    let drv = drivers(0.0, 1.0e-3);
    let mut grid = Wave1RouteGrid::new();
    let (mut dl, mut load) = (0.0, 0.0);
    let outcome = wave1_erod(
        &mut grid, &detaching, 1.0, &drv, 0.0, 0.005, &mut dl, &mut load,
    );
    assert!(!outcome.ndep);
    assert!(load >= 0.0 && dl.is_finite());

    let depositing = segment(0.0, 1.0, 1.0, 0.0, 0.0, 0.05);
    let mut grid = Wave1RouteGrid::new();
    grid.load[0] = 1.0;
    grid.tcap[0] = 0.05;
    let (mut dl, mut load) = (0.0, 1.0);
    let outcome = wave1_erod(
        &mut grid,
        &depositing,
        0.0,
        &drv,
        0.0,
        1.0,
        &mut dl,
        &mut load,
    );
    assert!(outcome.ndep);
    assert!((0.0..=1.0).contains(&outcome.xdbeg));
    assert!(load.is_finite() && load >= 0.0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn erod_preparation_characterizes_complete_and_onset_variants() {
    let drv = drivers(0.0, 1.0e-3);
    let seg = segment(0.0, 1.0, 1.0, 0.0, 1.0, 1.0);
    let mut grid = Wave1RouteGrid::new();
    grid.load[0] = 0.1;
    grid.tcap[0] = 1.0;

    let mut dl = 1.0;
    let mut load = 0.1;
    let flow_end = Wave1ErodMarchOutcome {
        currpt: 1,
        kflag: 4,
        ldrat: 0.0,
        ldrat2: 0.0,
        outcome: Wave1ErodOutcome {
            ndep: false,
            xdbeg: 0.0,
        },
    };
    assert!(matches!(
        wave1_erod_prepare(
            &grid, &seg, 1.0, &drv, 0.0, 1.0, 1, flow_end, &mut dl, &mut load
        ),
        Wave1ErodPreparation::Complete(Wave1ErodOutcome { ndep: false, .. })
    ));
    assert!(dl.abs() <= f64::EPSILON && load.abs() <= f64::EPSILON);

    let positive = Wave1ErodMarchOutcome {
        currpt: 1,
        kflag: 1,
        ldrat: 0.5,
        ldrat2: 0.0,
        outcome: Wave1ErodOutcome {
            ndep: false,
            xdbeg: 0.0,
        },
    };
    let mut dl = 0.0;
    let mut load = 0.0;
    assert!(matches!(
        wave1_erod_prepare(
            &grid, &seg, 1.0, &drv, 0.0, 0.0, 1, positive, &mut dl, &mut load
        ),
        Wave1ErodPreparation::Complete(Wave1ErodOutcome { ndep: false, .. })
    ));
    assert!(load >= 0.0 && dl >= 0.0);

    let zero_seg = segment(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let zero = Wave1ErodMarchOutcome {
        currpt: 1,
        kflag: 3,
        ldrat: 0.0,
        ldrat2: 0.0,
        outcome: Wave1ErodOutcome {
            ndep: false,
            xdbeg: 0.0,
        },
    };
    let mut zero_grid = Wave1RouteGrid::new();
    zero_grid.load[0] = 0.0;
    let mut dl = 1.0;
    let mut load = 0.0;
    assert!(matches!(
        wave1_erod_prepare(
            &zero_grid, &zero_seg, 0.0, &drv, 0.0, 0.0, 1, zero, &mut dl, &mut load
        ),
        Wave1ErodPreparation::Complete(_)
    ));
    assert!(dl.abs() <= f64::EPSILON);

    let top = Wave1ErodMarchOutcome {
        currpt: 1,
        kflag: 3,
        ldrat: 0.0,
        ldrat2: 0.0,
        outcome: Wave1ErodOutcome {
            ndep: true,
            xdbeg: 0.0,
        },
    };
    let mut dl = 0.0;
    let mut load = 0.0;
    assert!(matches!(
        wave1_erod_prepare(
            &zero_grid, &zero_seg, 0.0, &drv, 0.0, 1.0, 1, top, &mut dl, &mut load,
        ),
        Wave1ErodPreparation::Complete(Wave1ErodOutcome { ndep: true, .. })
    ));

    for flag in [1_u8, 2] {
        let onset = Wave1ErodMarchOutcome {
            currpt: 2,
            kflag: flag,
            ldrat: -0.25,
            ldrat2: -0.25,
            outcome: Wave1ErodOutcome {
                ndep: true,
                xdbeg: 0.0,
            },
        };
        let mut onset_grid = Wave1RouteGrid::new();
        onset_grid.ilast = 1;
        onset_grid.load[1] = 0.5;
        onset_grid.tcap[1] = 0.25;
        let mut dl = 0.0;
        let mut load = 0.5;
        match wave1_erod_prepare(
            &onset_grid,
            &seg,
            1.0,
            &drv,
            0.0,
            1.0,
            1,
            onset,
            &mut dl,
            &mut load,
        ) {
            Wave1ErodPreparation::Onset(outcome, bracket) => {
                assert!(outcome.ndep);
                assert_eq!(bracket.kflag, flag);
                assert!(bracket.xlast <= bracket.xfrt);
                assert!(bracket.ldlast >= 0.0);
            }
            Wave1ErodPreparation::Complete(_) => {
                panic!("flag {flag} must retain an onset bracket")
            }
        }
    }

    // `erod.for:282-389`: the committed x=.01 point remains below
    // transport capacity (G=.5 < Tc=1), but Tc falls linearly to .1
    // over the half-cell tail while zero Dc/theta leave G unchanged.
    // Deposition therefore begins only at xe=.015 and must publish a
    // flag-2 bracket pairing the committed point with the tail load.
    let tail_seg = segment(0.0, 0.0, 0.0, 0.0, -180.0, 2.8);
    let mut tail_grid = Wave1RouteGrid::new();
    tail_grid.ilast = 1;
    tail_grid.load[1] = 0.5;
    tail_grid.tcap[1] = 1.0;
    let tail_march = Wave1ErodMarchOutcome {
        currpt: 2,
        kflag: 1,
        ldrat: 0.5,
        ldrat2: 0.0,
        outcome: Wave1ErodOutcome {
            ndep: false,
            xdbeg: 0.0,
        },
    };
    let mut dl = 0.0;
    let mut load = 0.5;
    match wave1_erod_prepare(
        &tail_grid,
        &tail_seg,
        0.0,
        &Wave1Drivers { theta: 0.0, ..drv },
        0.0,
        0.015,
        2,
        tail_march,
        &mut dl,
        &mut load,
    ) {
        Wave1ErodPreparation::Onset(outcome, bracket) => {
            assert!(outcome.ndep);
            assert_eq!(bracket.kflag, 2);
            assert!((bracket.xlast - 0.01).abs() <= f64::EPSILON);
            assert!((bracket.xfrt - 0.015).abs() <= f64::EPSILON);
            assert!((bracket.detlst - 1.0).abs() <= f64::EPSILON);
            assert!((bracket.detfrt - (-0.8)).abs() <= 1.0e-12);
            assert!((bracket.ldlast - 0.5).abs() <= f64::EPSILON);
        }
        Wave1ErodPreparation::Complete(_) => {
            panic!("segment-tail capacity crossing must create an onset bracket")
        }
    }

    let mut rewind_grid = Wave1RouteGrid::new();
    rewind_grid.ilast = 1;
    rewind_grid.load[0] = 0.25;
    rewind_grid.tcap[0] = 0.5;
    rewind_grid.load[1] = 0.5;
    rewind_grid.tcap[1] = 1.0;
    let rewind_march = Wave1ErodMarchOutcome {
        currpt: 2,
        kflag: 1,
        ldrat: 0.5,
        ldrat2: 0.0,
        outcome: Wave1ErodOutcome {
            ndep: false,
            xdbeg: 0.0,
        },
    };
    let rewind_seg = segment(0.0, 0.0, 0.0, 0.0, 0.0, 0.1);
    let mut dl = 0.0;
    let mut load = 0.5;
    match wave1_erod_prepare(
        &rewind_grid,
        &rewind_seg,
        0.0,
        &Wave1Drivers { theta: 0.0, ..drv },
        0.0,
        0.01,
        2,
        rewind_march,
        &mut dl,
        &mut load,
    ) {
        Wave1ErodPreparation::Onset(outcome, bracket) => {
            assert!(outcome.ndep);
            assert!(bracket.xlast.abs() <= f64::EPSILON);
            assert!((bracket.xfrt - 0.01).abs() <= f64::EPSILON);
            assert!((bracket.detlst - 1.0).abs() <= f64::EPSILON);
        }
        Wave1ErodPreparation::Complete(_) => panic!("exact endpoint must rewind its bracket"),
    }
}

#[test]
fn route_characterizes_typed_empty_error_flow_end_and_deposition_paths() {
    let error = wave1_route(&[], &drivers(0.0, 1.0e-3), 0.0, None)
        .expect_err("empty coefficient payload must fail closed");
    assert!(matches!(
        error,
        DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.segments"
        }
    ));

    let skipped = Wave1SegmentCoefficients {
        xu: 0.2,
        xl: 1.0,
        ..segment(0.0, 1.0, 0.1, 0.0, 1.0, 0.1)
    };
    let toe_error = wave1_route(&[skipped], &drivers(-0.1, 0.0), 0.0, None)
        .expect_err("a profile wholly below the flow end must not publish an untouched toe");
    assert!(matches!(
        toe_error,
        DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.route_toe_uncomputed"
        }
    ));

    let profiles = [
        (
            segment(0.0, 1.0, 0.1, 0.0, 1.0, 0.1),
            drivers(0.0, 1.0e-3),
            0.0,
        ),
        (
            segment(-2.0, 2.0, 0.0, -2.0, 2.0, 0.0),
            drivers(0.0, 1.0e-3),
            0.0,
        ),
        (
            segment(0.0, 1.0, 0.1, 0.0, 0.2, 0.1),
            drivers(-0.5, 0.0),
            0.5,
        ),
        (
            segment(1.0, 0.1, 0.1, 1.0, 0.1, 0.1),
            drivers(0.5, 1.0e-3),
            0.2,
        ),
    ];
    let mut deposition_points = 0_usize;
    let mut detachment_points = 0_usize;
    let mut flow_end_points = 0_usize;
    for (seg, drv, inflow) in profiles {
        let grid = wave1_route(&[seg], &drv, inflow, None).expect("characterized route must solve");
        let toe = DIRECT_WAVE1_GRID_POINTS - 1;
        assert_ne!(grid.region[toe], Wave1PointRegion::Untouched);
        assert!(grid.load[toe].is_finite() && grid.load[toe] >= 0.0);
        assert!(grid.tcap[toe].is_finite() && grid.tcap[toe] >= 0.0);
        deposition_points += grid
            .region
            .iter()
            .filter(|region| **region == Wave1PointRegion::Deposition)
            .count();
        detachment_points += grid
            .region
            .iter()
            .filter(|region| **region == Wave1PointRegion::Detachment)
            .count();
        flow_end_points += grid
            .region
            .iter()
            .filter(|region| **region == Wave1PointRegion::FlowEnd)
            .count();
    }
    assert!(deposition_points > 0, "matrix must characterize deposition");
    assert!(detachment_points > 0, "matrix must characterize detachment");
    assert!(
        flow_end_points > 0,
        "matrix must characterize case-4 flow end"
    );

    let inputs = enrichment_inputs();
    let mut state = super::super::Wave1EnrichmentState::initialize(&inputs, true, true, 0.5);
    let enriched = wave1_route(
        &[segment(0.0, 1.0, 0.1, 0.0, 0.0, 0.1)],
        &drivers(0.0, 1.0e-3),
        0.5,
        Some((&inputs, &mut state)),
    )
    .expect("depositing route must update the enrichment consumer");
    assert_ne!(
        enriched.region[DIRECT_WAVE1_GRID_POINTS - 1],
        Wave1PointRegion::Untouched
    );
    assert!(state.lddend >= 0.0 && state.xdetst >= 0.0);
    assert!(state.enrichment_ratio.is_some());
}

#[test]
fn xcrit_nonconvex_rising_subinterval_is_characterized() {
    let rising = wave1_xcrit(-2.0, 2.0, 0.0, 0.25, 0.0, 0.4)
        .expect("concave rising subinterval must classify");
    assert_eq!(rising.regime, Wave1ShearRegime::RisingCross);
    assert!((0.0..=0.4).contains(&rising.xc1));

    // Both endpoints and the short interval remain below critical while
    // the real quadratic roots lie outside this interval. This is the
    // legacy non-double-cross fallback, not a discriminant failure.
    let outside = wave1_xcrit(-2.0, 2.0, 0.0, 0.25, 0.0, 0.05)
        .expect("out-of-interval roots must classify below critical");
    assert_eq!(outside.regime, Wave1ShearRegime::BelowCritical);
}

#[test]
fn inter_ofe_continuity_characterizes_bounded_ratio_denominators() {
    let inputs = continuity_inputs();
    for inter in [
        Wave1InterOfeContinuity {
            shrspv_pa: 1.0,
            tcprev_kg_s_m: 1.0,
            ktrprv: 1.0,
            prior_shear_last: (0.0, 1.0, 0.0),
            prior_transport_last: (0.0, 1.0, 0.0),
        },
        Wave1InterOfeContinuity {
            shrspv_pa: 1.0e-300,
            tcprev_kg_s_m: 0.0,
            ktrprv: 0.0,
            prior_shear_last: (0.0, 1.0, 0.0),
            prior_transport_last: (0.0, 0.0, 0.0),
        },
    ] {
        let mut coefficients = vec![segment(0.0, 1.0, 0.0, 0.0, 1.0, 0.0)];
        wave1_apply_inter_ofe_continuity(&mut coefficients, &inputs, &inter, 0.25)
            .expect("bounded legacy denominator case must rewrite finitely");
        let rewritten = coefficients[0];
        assert!(rewritten.a.is_finite());
        assert!(rewritten.b.is_finite());
        assert!(rewritten.c.is_finite());
        assert!(rewritten.atc.is_finite());
        assert!(rewritten.btc.is_finite());
        assert!(rewritten.ctc.is_finite());
    }
}

#[test]
fn geometry_validation_rejects_each_missing_positive_operand_at_its_call_site() {
    type GeometryCase = (&'static str, fn(&mut DirectWave1ContinuityInputs));
    let cases: [GeometryCase; 6] = [
        (
            "erosion.wave1.efflen_m",
            |inputs: &mut DirectWave1ContinuityInputs| inputs.efflen_m = 0.0,
        ),
        (
            "erosion.wave1.slplen_m",
            |inputs: &mut DirectWave1ContinuityInputs| inputs.slplen_m = 0.0,
        ),
        (
            "erosion.wave1.cntlen_m",
            |inputs: &mut DirectWave1ContinuityInputs| inputs.cntlen_m = 0.0,
        ),
        (
            "erosion.wave1.rspace_m",
            |inputs: &mut DirectWave1ContinuityInputs| inputs.rspace_m = 0.0,
        ),
        (
            "erosion.wave1.width_m",
            |inputs: &mut DirectWave1ContinuityInputs| inputs.width_m = 0.0,
        ),
        (
            "erosion.wave1.field_width_m",
            |inputs: &mut DirectWave1ContinuityInputs| inputs.field_width_m = 0.0,
        ),
    ];
    for (field, mutate) in cases {
        let mut inputs = continuity_inputs();
        mutate(&mut inputs);
        assert!(matches!(
            validate_wave1_geometry_scalars(&inputs),
            Err(DirectRuntimeError::DirectDomainViolation { field: actual }) if actual == field
        ));
    }
}

#[test]
fn dispatch_detachment_characterizes_every_shear_regime_and_start_branch() {
    let seg = segment(0.0, 1.0, 0.1, 0.0, 1.0, 0.1);
    let drv = drivers(0.0, 1.0e-3);
    let cases = [
        (Wave1ShearRegime::BelowCritical, 0.3, 0.7, 0.0),
        (Wave1ShearRegime::AboveCritical, 0.3, 0.7, 0.0),
        (Wave1ShearRegime::RisingCross, 0.3, 0.7, 0.0),
        (Wave1ShearRegime::RisingCross, 0.3, 0.7, 0.5),
        (Wave1ShearRegime::FallingCross, 0.3, 0.7, 0.0),
        (Wave1ShearRegime::FallingCross, 0.3, 0.7, 0.5),
        (Wave1ShearRegime::DoubleCross, 0.3, 0.7, 0.0),
        (Wave1ShearRegime::DoubleCross, 0.3, 0.7, 0.5),
        (Wave1ShearRegime::DoubleCross, 0.3, 0.7, 0.8),
    ];
    for (regime, xc1, xc2, start_x) in cases {
        let mut grid = Wave1RouteGrid::new();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let start_index = (start_x / WAVE1_GRID_DX) as usize;
        grid.ilast = start_index;
        grid.load[start_index] = 0.0;
        grid.tcap[start_index] = 1.0;
        let mut dl = 0.0;
        let mut load = 0.0;
        let outcome = wave1_dispatch_detachment(
            &mut grid,
            &seg,
            &Wave1ShearClassification { regime, xc1, xc2 },
            1.0,
            &drv,
            start_x,
            &mut dl,
            &mut load,
        );
        assert!(load.is_finite() && load >= 0.0, "{regime:?} load");
        assert!(dl.is_finite(), "{regime:?} terminal rate");
        for index in 0..=grid.ilast {
            if grid.region[index] != Wave1PointRegion::Untouched {
                assert!(
                    grid.load[index].is_finite() && grid.load[index] >= 0.0,
                    "{regime:?} load[{index}]"
                );
                assert!(
                    grid.tcap[index].is_finite() && grid.tcap[index] >= 0.0,
                    "{regime:?} tcap[{index}]"
                );
            }
        }
        if outcome.ndep {
            assert!((start_x..=seg.xl).contains(&outcome.xdbeg));
        } else {
            assert!(grid.ilast > 0, "{regime:?} must commit routed mass state");
        }
    }
}
