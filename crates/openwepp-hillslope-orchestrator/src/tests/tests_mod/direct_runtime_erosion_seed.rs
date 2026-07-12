//! Wave-1 operand assembly tests (erosion port Increment-1b-C flip core).
//! Exercises the full static-seed + daily-state → `DirectWave1ContinuityInputs`
//! pipeline and drives it through the continuity solver, on McKenzie
//! clay-loam-shaped operands, to prove the production operand pipeline
//! conserves.
#![allow(clippy::doc_markdown)]

use crate::{
    DirectRuntimeError, DirectWave1DailyState, DirectWave1OperandSeed, DirectWave1SlopeSegment,
    ErosionConsolidationInputs, ErosionExcessInterval, ErosionFrostRegime, ErosionTextureInputs,
    Wave1InflowOperands, assemble_wave1_continuity_inputs,
    assemble_wave1_continuity_inputs_quantum, compute_direct_wave1_continuity,
    erosion_consolidation_baselines, erosion_effective_particle, erosion_falvel,
    erosion_particle_composition,
};

fn clay_loam_seed() -> DirectWave1OperandSeed {
    let texture = ErosionTextureInputs {
        sand: 0.25,
        clay: 0.30,
        silt: 0.45,
        orgmat: 0.05,
    };
    let classes = erosion_particle_composition(&texture).expect("composition");
    let (diaeff, spgeff) = erosion_effective_particle(&classes).expect("effective particle");
    let veleff = erosion_falvel(spgeff, diaeff);
    let baselines = erosion_consolidation_baselines(&ErosionConsolidationInputs {
        sand: 0.25,
        silt: 0.45,
        orgmat: 0.05,
        thetfc: 0.2833,
        rock_fragment_fraction: 0.05,
        ki: 1.5e6,
        kr: 6.0e-5,
        shcrit: 0.5,
    })
    .expect("baselines");
    // Single concave segment (rising-then-falling transport) so the solve
    // exercises detachment -> deposition, like the erod16 fixture.
    let avgslp = 0.43;
    let segment = DirectWave1SlopeSegment {
        xu: 0.0,
        xl: 1.0,
        a: -2.0,
        b: 2.0,
    };
    let slpend = (segment.a + segment.b) * avgslp;
    DirectWave1OperandSeed {
        enabled: true,
        is_cropland: false,
        segments: vec![segment],
        slplen_m: 200.0,
        efflen_m: 200.0,
        cntlen_m: 200.0,
        rspace_m: 1.0,
        field_width_m: 30.0,
        avg_slope: avgslp,
        slpend,
        sand: 0.25,
        ssasol: 6.0,
        classes,
        veleff_m_s: veleff,
        baselines,
        kr_s_m: 6.0e-5,
        ki: 1.5e6,
        shcrit_pa: 0.5,
        hmax_m: 0.0,
        flivmx: 0.0,
        random_roughness_m: 0.006,
        initial_daydis: 30.0,
    }
}

fn storm_daily_state() -> DirectWave1DailyState {
    // A storm day: peak runoff above the passby gate, one excess interval.
    DirectWave1DailyState {
        inflow: None,
        peakro_m_s: 5.0e-5,
        runoff_depth_m: 0.03,
        effdrn_s: 600.0,
        qin_m2_s: 0.0,
        excess_intervals: vec![ErosionExcessInterval {
            duration_s: 600.0,
            rainfall_intensity_m_s: 8.0e-5,
            excess_m: 0.03,
            snowmelt_active: false,
        }],
        canopy_cover_fraction: 0.0,
        canopy_height_m: 0.0,
        interrill_cover_fraction: 0.0,
        rill_cover_fraction: 0.0,
        live_root_mass_kg_m2: 0.0,
        dead_root_mass_kg_m2: 0.0,
        buried_residue_mass_kg_m2: 0.0,
        random_roughness_m: 0.006,
        rill_width_prior_m: 0.0,
        days_since_disturbance: 30.0,
        frost_regime: ErosionFrostRegime::Unfrozen,
        theta_suppressed: false,
        beta: 0.5,
        strldn: 0.0,
    }
}

#[test]
fn assembled_storm_day_produces_a_conserving_solve() {
    let seed = clay_loam_seed();
    let daily = storm_daily_state();
    let inputs = assemble_wave1_continuity_inputs(&seed, &daily)
        .expect("production operand assembly must resolve");

    // The assembly produced real, finite operands (not the erod16
    // test-harness approximations).
    assert!(inputs.enabled);
    assert!(inputs.shrsol_pa > 0.0 && inputs.shrsol_pa.is_finite());
    assert!(inputs.tcend_kg_s_m > 0.0);
    assert!(inputs.ktrato > 0.0);
    assert!(inputs.detinr_kg_s_m2 >= 0.0);
    assert!(inputs.effdrr_s > 0.0);
    assert!(inputs.width_m > 0.0);
    // Day-30 unfrozen bare burn: adjustment factors are in range.
    assert!(inputs.kradjf >= 0.03 && inputs.kradjf <= 1.0 + 1.0e-9);
    assert!(inputs.tcadjf >= 0.30 && inputs.tcadjf <= 2.0 + 1.0e-9);

    // Drive the assembled operands through the solver: it must conserve.
    let state =
        compute_direct_wave1_continuity(&inputs).expect("assembled operands must solve clean");
    assert!(state.active);
    let detach_kg_m = state.total_detachment_kg / seed.field_width_m;
    let depos_kg_m = state.total_deposition_kg / seed.field_width_m;
    let residual =
        (state.exported_sediment_kg_m - state.inflow_sediment_kg_m) - (detach_kg_m - depos_kg_m);
    let scale = state
        .exported_sediment_kg_m
        .abs()
        .max(detach_kg_m.abs())
        .max(1.0e-9);
    assert!(
        residual.abs() <= 1.0e-9 * scale,
        "assembled-operand solve must conserve: residual {residual} scale {scale}"
    );
    assert!(state.total_detachment_kg > 0.0, "storm day must detach");
}

#[test]
fn assembled_effint_uses_rainfall_intensity_not_excess_rate() {
    // The production assembly must use the faithful effint (rainfall
    // intensity over excess periods), distinct from the mean excess rate.
    let seed = clay_loam_seed();
    let daily = storm_daily_state();
    let inputs = assemble_wave1_continuity_inputs(&seed, &daily).expect("assembly");
    // effint feeds detinr; with the single interval effint = 8.0e-5
    // (rainfall rate), whereas runoff/effdrr = 0.03/600 = 5e-5 (the old
    // approximation). detinr scales with effint, so a nonzero detinr on a
    // bare day confirms the interrill driver is live.
    assert!(inputs.effdrr_s > 0.0);
    // No cover/roots on the bare burn day, so detinr should be positive
    // (interrill supply is active).
    assert!(inputs.detinr_kg_s_m2 > 0.0);
}

#[test]
fn assembled_thawing_day_fails_closed() {
    // The actively-thawing frost regime must propagate the 1b-B
    // fail-closed error through the assembly (winter fcycle absent).
    let seed = clay_loam_seed();
    let mut daily = storm_daily_state();
    daily.frost_regime = ErosionFrostRegime::Thawing;
    assert!(matches!(
        assemble_wave1_continuity_inputs(&seed, &daily),
        Err(DirectRuntimeError::MissingDirectUpstream { .. })
    ));
}

#[test]
fn assembled_frozen_surface_sets_surface_frozen_flag() {
    let seed = clay_loam_seed();
    let mut daily = storm_daily_state();
    daily.frost_regime = ErosionFrostRegime::FrozenSurface;
    let inputs = assemble_wave1_continuity_inputs(&seed, &daily).expect("frozen assembly");
    assert!(inputs.surface_frozen, "frozen surface must flag the solver");
}

#[test]
fn assembled_dry_day_gates_before_operands_and_solves_inactive() {
    // Codex round-1 High: a no-runoff day (peakro = 0) must NOT hit the
    // routed-operand pipeline (zero-width rill hydraulics would hard-error).
    // The assembly gates first and returns the inert payload; the solver
    // returns inactive.
    let seed = clay_loam_seed();
    let dry = DirectWave1DailyState {
        peakro_m_s: 0.0,
        runoff_depth_m: 0.0,
        effdrn_s: 0.0,
        excess_intervals: Vec::new(),
        ..storm_daily_state()
    };
    let inputs = assemble_wave1_continuity_inputs(&seed, &dry)
        .expect("dry day must assemble without touching routed operands");
    let state = compute_direct_wave1_continuity(&inputs).expect("dry day must solve inactive");
    assert!(!state.active);

    // A tiny sub-passby event (both bounds below the gate) is also inert.
    let tiny = DirectWave1DailyState {
        peakro_m_s: 1.0e-6,
        runoff_depth_m: 0.005,
        effdrn_s: 5000.0,
        excess_intervals: Vec::new(),
        ..storm_daily_state()
    };
    let inputs = assemble_wave1_continuity_inputs(&seed, &tiny).expect("sub-gate assembly");
    assert!(!compute_direct_wave1_continuity(&inputs).unwrap().active);

    // A NaN activation operand still fails closed.
    let nan_day = DirectWave1DailyState {
        peakro_m_s: f64::NAN,
        ..storm_daily_state()
    };
    assert!(matches!(
        assemble_wave1_continuity_inputs(&seed, &nan_day),
        Err(DirectRuntimeError::NonFiniteDirectValue { .. })
    ));
}

#[test]
fn assembled_rill_width_carries_persistent_state() {
    // Codex round-1 High: the Gilley width is persistent state grown by
    // shears, reset only at disturbance. A later smaller storm must NOT
    // shrink the width below the prior storm's grown value.
    let seed = clay_loam_seed();

    // A large storm grows the width from a zero seed.
    let big = DirectWave1DailyState {
        peakro_m_s: 1.0e-4,
        runoff_depth_m: 0.05,
        effdrn_s: 500.0,
        rill_width_prior_m: 0.0,
        ..storm_daily_state()
    };
    let big_inputs = assemble_wave1_continuity_inputs(&seed, &big).expect("big storm");
    let grown_width = big_inputs.width_m;
    assert!(grown_width > 0.0);

    // A later smaller storm, carrying the grown width, must retain it
    // (the smaller storm's own Gilley width would be narrower).
    let small_fresh = DirectWave1DailyState {
        peakro_m_s: 2.0e-5,
        runoff_depth_m: 0.02,
        effdrn_s: 800.0,
        rill_width_prior_m: 0.0,
        ..storm_daily_state()
    };
    let small_fresh_width = assemble_wave1_continuity_inputs(&seed, &small_fresh)
        .expect("small fresh")
        .width_m;
    let small_carried = DirectWave1DailyState {
        rill_width_prior_m: grown_width,
        ..small_fresh.clone()
    };
    let small_carried_width = assemble_wave1_continuity_inputs(&seed, &small_carried)
        .expect("small carried")
        .width_m;
    assert!(
        small_fresh_width < grown_width,
        "the small storm's own width must be narrower than the big storm's"
    );
    assert!(
        (small_carried_width - grown_width).abs() < 1.0e-12,
        "carrying the prior width must retain it (monotone growth): \
         carried {small_carried_width}, grown {grown_width}"
    );
    // detinr scales with 1/width, so the carried (wider) width yields a
    // smaller detinr than the fresh (narrower) width — the fidelity point.
    let fresh_detinr = assemble_wave1_continuity_inputs(&seed, &small_fresh)
        .unwrap()
        .detinr_kg_s_m2;
    let carried_detinr = assemble_wave1_continuity_inputs(&seed, &small_carried)
        .unwrap()
        .detinr_kg_s_m2;
    assert!(carried_detinr < fresh_detinr);
}

#[test]
fn assembled_inflow_derivations_use_the_receiver_basis() {
    // INV-SED-016 (c) alias-separation: the receiver-side derivations
    // must use the RECEIVER's scales for `strldn` (`param.for:243`) and
    // the PRIOR lane's slopes for the boundary shear (`param.for:187-189`)
    // — aliasing either to the wrong OFE basis is the regression class.
    let seed = clay_loam_seed();
    let mut daily = storm_daily_state();
    daily.inflow = Some(Wave1InflowOperands {
        qin_m2_s: 2.0e-4,
        qsout_kg_m_s: 1.0e-3,
        prior_slpend: 0.30,
        prior_cnslp: 0.25,
        prior_end_shear: (0.5, 0.5, 0.0),
        prior_end_transport: (0.5, 0.5, 0.0),
        exit_fractions: [0.3, 0.3, 0.2, 0.1, 0.1],
    });
    let inputs = assemble_wave1_continuity_inputs(&seed, &daily).expect("inflow assembly");
    assert!(
        (inputs.qin_m2_s - 2.0e-4).abs() < 1.0e-18,
        "qin must come from the handoff"
    );
    // Receiver-basis strldn identity against the SAME assembled payload's
    // own tcend/width/rspace (`strldn = qsout * rspace / (tcend * width)`).
    let expected_strldn =
        1.0e-3 * inputs.rspace_m / inputs.tcend_kg_s_m.max(1.0e-10) / inputs.width_m;
    assert!(
        (inputs.strldn - expected_strldn).abs() <= 1.0e-12 * expected_strldn.abs().max(1.0e-12),
        "strldn must be normalized on the receiver's scales \
         (observed {}, expected {expected_strldn})",
        inputs.strldn
    );
    let inter_ofe = inputs
        .inter_ofe
        .expect("an inflow assembly must derive the continuity operands");
    assert_eq!(inter_ofe.prior_shear_last, (0.5, 0.5, 0.0));

    // The boundary shear derives from the PRIOR lane's slopes: changing
    // ONLY `prior_cnslp` must move `shrspv`; a receiver-slope alias
    // would leave it unchanged.
    let mut steeper = storm_daily_state();
    steeper.inflow = Some(Wave1InflowOperands {
        qin_m2_s: 2.0e-4,
        qsout_kg_m_s: 1.0e-3,
        prior_slpend: 0.30,
        prior_cnslp: 0.55,
        prior_end_shear: (0.5, 0.5, 0.0),
        prior_end_transport: (0.5, 0.5, 0.0),
        exit_fractions: [0.3, 0.3, 0.2, 0.1, 0.1],
    });
    let steeper_inputs =
        assemble_wave1_continuity_inputs(&seed, &steeper).expect("steeper inflow assembly");
    let steeper_inter_ofe = steeper_inputs.inter_ofe.expect("continuity operands");
    assert!(
        (steeper_inter_ofe.shrspv_pa - inter_ofe.shrspv_pa).abs()
            > 1.0e-9 * inter_ofe.shrspv_pa.abs(),
        "shrspv must respond to the PRIOR lane's average slope \
         ({} vs {})",
        steeper_inter_ofe.shrspv_pa,
        inter_ofe.shrspv_pa
    );
}

#[test]
fn hb03_d_f_h_qin_authority_rejects_conflict_nonfinite_and_negative_values() {
    let seed = clay_loam_seed();
    let mut conflict = storm_daily_state();
    conflict.qin_m2_s = 1.0e-4;
    conflict.inflow = Some(Wave1InflowOperands {
        qin_m2_s: 2.0e-4,
        qsout_kg_m_s: 1.0e-3,
        prior_slpend: 0.30,
        prior_cnslp: 0.25,
        prior_end_shear: (0.5, 0.5, 0.0),
        prior_end_transport: (0.5, 0.5, 0.0),
        exit_fractions: [0.3, 0.3, 0.2, 0.1, 0.1],
    });
    assert_eq!(
        assemble_wave1_continuity_inputs_quantum(&seed, &conflict, true),
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.assemble.qin_conflict"
        })
    );

    for qin_m2_s in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let malformed = DirectWave1DailyState {
            qin_m2_s,
            ..storm_daily_state()
        };
        assert!(matches!(
            assemble_wave1_continuity_inputs_quantum(&seed, &malformed, true),
            Err(DirectRuntimeError::NonFiniteDirectValue {
                field: "erosion.assemble.qin"
            })
        ));

        let mut malformed_handoff = storm_daily_state();
        malformed_handoff.inflow = Some(Wave1InflowOperands {
            qin_m2_s,
            qsout_kg_m_s: 1.0e-3,
            prior_slpend: 0.30,
            prior_cnslp: 0.25,
            prior_end_shear: (0.5, 0.5, 0.0),
            prior_end_transport: (0.5, 0.5, 0.0),
            exit_fractions: [0.3, 0.3, 0.2, 0.1, 0.1],
        });
        assert!(matches!(
            assemble_wave1_continuity_inputs_quantum(&seed, &malformed_handoff, true),
            Err(DirectRuntimeError::NonFiniteDirectValue {
                field: "erosion.assemble.inflow_qin"
            })
        ));
    }

    let negative = DirectWave1DailyState {
        qin_m2_s: -1.0e-4,
        ..storm_daily_state()
    };
    assert_eq!(
        assemble_wave1_continuity_inputs_quantum(&seed, &negative, true),
        Err(DirectRuntimeError::NegativeDirectValue {
            field: "erosion.assemble.qin"
        })
    );

    let mut negative_handoff = storm_daily_state();
    negative_handoff.inflow = Some(Wave1InflowOperands {
        qin_m2_s: -1.0e-4,
        qsout_kg_m_s: 1.0e-3,
        prior_slpend: 0.30,
        prior_cnslp: 0.25,
        prior_end_shear: (0.5, 0.5, 0.0),
        prior_end_transport: (0.5, 0.5, 0.0),
        exit_fractions: [0.3, 0.3, 0.2, 0.1, 0.1],
    });
    assert_eq!(
        assemble_wave1_continuity_inputs_quantum(&seed, &negative_handoff, true),
        Err(DirectRuntimeError::NegativeDirectValue {
            field: "erosion.assemble.inflow_qin"
        })
    );

    negative_handoff.qin_m2_s = 1.0e-4;
    assert_eq!(
        assemble_wave1_continuity_inputs_quantum(&seed, &negative_handoff, true),
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.assemble.qin_conflict"
        }),
        "dual-authority conflict must retain priority over handoff negativity"
    );
}

#[test]
fn hb03_a_b_c_g_passby_exempt_reinfiltration_routes_only_upstream_authority() {
    let mut seed = clay_loam_seed();
    seed.avg_slope = 0.30;
    seed.slpend = 0.30;
    seed.segments = vec![DirectWave1SlopeSegment {
        xu: 0.0,
        xl: 1.0,
        a: 0.0,
        b: 1.0,
    }];
    let mut daily = DirectWave1DailyState {
        peakro_m_s: 0.0,
        runoff_depth_m: 0.0,
        effdrn_s: 3600.0,
        excess_intervals: Vec::new(),
        theta_suppressed: true,
        ..storm_daily_state()
    };
    daily.inflow = Some(Wave1InflowOperands {
        qin_m2_s: 2.0e-4,
        qsout_kg_m_s: 1.0e-6,
        prior_slpend: 0.30,
        prior_cnslp: 0.25,
        prior_end_shear: (0.5, 0.5, 0.0),
        prior_end_transport: (0.5, 0.5, 0.0),
        exit_fractions: [0.3, 0.3, 0.2, 0.1, 0.1],
    });
    let inputs = assemble_wave1_continuity_inputs_quantum(&seed, &daily, true)
        .expect("positive erosion handoff must activate a reinfiltration quantum");
    assert_eq!(inputs.peakro_m_s, 0.0);
    assert_eq!(inputs.runoff_depth_m, 0.0);
    assert_eq!(inputs.qin_m2_s, 2.0e-4);
    assert_eq!(inputs.detinr_kg_s_m2, 0.0);
    assert!(inputs.strldn > 0.0);
    assert!(inputs.inter_ofe.is_some());
    assert_eq!(
        inputs
            .enrichment
            .as_deref()
            .and_then(|enrichment| enrichment.inflow_fractions),
        Some([0.3, 0.3, 0.2, 0.1, 0.1])
    );

    let assembled_strldn = inputs.strldn;
    let assembled_inter_ofe = inputs.inter_ofe.expect("inter-OFE lineage");
    let state = compute_direct_wave1_continuity(&inputs)
        .expect("real continuity consumer must route the reinfiltrating load");
    assert!(state.active);
    assert!(state.inflow_sediment_kg_m > 0.0);
    assert_eq!(inputs.detinr_kg_s_m2, 0.0, "no local interrill supply");
    assert!(state.total_deposition_kg > 0.0);
    assert!(
        state.exported_sediment_kg_m
            <= state.inflow_sediment_kg_m + state.total_detachment_kg / seed.field_width_m
    );
    assert!(assembled_strldn > 0.0);
    assert_eq!(state.qout_m2_s, 0.0);
    assert_eq!(
        inputs.inter_ofe.expect("inter-OFE inputs").prior_shear_last,
        assembled_inter_ofe.prior_shear_last
    );
    assert!(state.exit_class_fractions.is_some());
    let closure = (state.exported_sediment_kg_m - state.inflow_sediment_kg_m)
        - (state.total_detachment_kg - state.total_deposition_kg) / seed.field_width_m;
    assert!(closure.abs() <= 1.0e-9 * state.inflow_sediment_kg_m.max(1.0e-9));
}

#[test]
fn hb03_a_c_positive_standalone_qin_is_preserved_without_handoff() {
    let mut seed = clay_loam_seed();
    seed.avg_slope = 0.30;
    seed.slpend = 0.30;
    seed.segments = vec![DirectWave1SlopeSegment {
        xu: 0.0,
        xl: 1.0,
        a: 0.0,
        b: 1.0,
    }];
    let daily = DirectWave1DailyState {
        peakro_m_s: 0.0,
        runoff_depth_m: 0.0,
        effdrn_s: 3600.0,
        qin_m2_s: 2.0e-4,
        strldn: 1.5,
        excess_intervals: Vec::new(),
        theta_suppressed: true,
        ..storm_daily_state()
    };
    let inputs = assemble_wave1_continuity_inputs_quantum(&seed, &daily, true)
        .expect("standalone qin must remain a valid crafted-quantum authority");
    assert_eq!(inputs.qin_m2_s, daily.qin_m2_s);
    assert_eq!(inputs.strldn, daily.strldn);
    assert!(inputs.inter_ofe.is_none());
    assert_eq!(inputs.detinr_kg_s_m2, 0.0);
    // This crafted standalone payload is preserved for diagnostic callers;
    // the handoff vector above supplies the canonically valid real solve.
}
