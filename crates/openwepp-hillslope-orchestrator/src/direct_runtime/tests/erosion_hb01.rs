use super::*;
use crate::{DirectPeakRunoffShadowProjection, DirectRunIdentity};

fn nominal_inputs() -> DirectErod13Inputs {
    DirectErod13Inputs {
        ie_m_s: 1.0e-5,
        te_s: 60.0,
        fs: 0.5,
        ft: 1.0,
        taufe_pa: 4.0,
        q_m2_s: 0.001,
        g_kg_s_m: 1.0,
        di_kg_s_m2: 0.002,
        beta: 0.5,
        vf_m_s: 0.2,
        dgdx_kg_s_m2: 0.0195,
        cntlen_m: 10.0,
        kr_s_m: 0.01,
        kradjf: 1.0,
        tcadjf: 0.5,
        shrsol_pa: 2.0,
        tcend_kg_s_m: 10.0,
        shcrit_pa: 1.0,
        detinr_kg_s_m2: 0.001,
        effdrr_m: 1.0,
        effdrn_m: 1.0,
        veleff_m_s: 0.2,
        pkro_m3_s: 0.001,
        tc_k: 2.0,
        tc_m: 1.0,
        q_runoff_m: 0.01,
        peakro_m3_s: 0.001,
        watdur_s: 10.0,
    }
}

#[test]
fn hb01_a_g_nominal_detachment_satisfies_formulas_and_continuity() {
    let state = compute_direct_erod13(&nominal_inputs()).expect("nominal vector must solve");
    assert_eq!(state.tau_f_pa, 2.0);
    assert_eq!(state.eta, 0.02);
    assert_eq!(state.taucn, 0.25);
    assert_eq!(state.theta, 0.001);
    assert_eq!(state.phi, 100.0);
    assert_eq!(state.tc_kg_s_m, 2.0);
    assert!((state.dc_kg_s_m2 - 0.035).abs() <= 1.0e-15);
    assert!((state.df_kg_s_m2 - 0.0175).abs() <= 1.0e-15);
    assert!(
        (state.df_kg_s_m2 + nominal_inputs().di_kg_s_m2 - nominal_inputs().dgdx_kg_s_m2).abs()
            <= DIRECT_EROD13_CONTINUITY_TOLERANCE
    );
}

#[test]
fn hb01_b_c_threshold_and_deposition_branches_are_exact() {
    let mut threshold = nominal_inputs();
    threshold.shcrit_pa = 8.0;
    threshold.dgdx_kg_s_m2 = threshold.di_kg_s_m2;
    let threshold_state = compute_direct_erod13(&threshold).expect("threshold vector must solve");
    assert_eq!(threshold_state.tau_f_pa, threshold_state.taucn);
    assert_eq!(threshold_state.dc_kg_s_m2, 0.0);
    assert_eq!(threshold_state.df_kg_s_m2, 0.0);

    let mut deposition = nominal_inputs();
    deposition.g_kg_s_m = 3.0;
    deposition.dgdx_kg_s_m2 = -99.998;
    let deposition_state =
        compute_direct_erod13(&deposition).expect("deposition vector must solve");
    assert_eq!(deposition_state.dc_kg_s_m2, 0.0);
    assert!((deposition_state.df_kg_s_m2 + 100.0).abs() <= 1.0e-12);
}

#[test]
fn hb01_b_d_h_threshold_domain_guards_fail_closed_with_exact_fields() {
    let cases: [(&str, fn(&mut DirectErod13Inputs)); 10] = [
        ("erosion.erod13.te_s", |v| v.te_s = 0.0),
        ("erosion.erod13.ft", |v| v.ft = 0.0),
        ("erosion.erod13.fs", |v| v.fs = v.ft + 1.0),
        ("erosion.erod13.tcadjf", |v| v.tcadjf = 0.29),
        ("erosion.erod13.shrsol_pa", |v| v.shrsol_pa = 0.0),
        ("erosion.erod13.tcend_kg_s_m", |v| v.tcend_kg_s_m = 0.0),
        ("erosion.erod13.effdrn_m", |v| v.effdrn_m = 0.0),
        ("erosion.erod13.pkro_m3_s", |v| v.pkro_m3_s = 0.0),
        ("erosion.erod13.peakro_m3_s", |v| v.peakro_m3_s = 0.0),
        ("erosion.erod13.watdur_s", |v| v.watdur_s = 0.0),
    ];
    for (field, mutate) in cases {
        let mut inputs = nominal_inputs();
        mutate(&mut inputs);
        assert_eq!(
            compute_direct_erod13(&inputs),
            Err(DirectRuntimeError::DirectDomainViolation { field })
        );
    }
}

#[test]
fn hb01_f_h_every_required_scalar_rejects_all_nonfinite_values() {
    let fields: [(&str, fn(&mut DirectErod13Inputs, f64)); 28] = [
        ("erosion.erod13.ie_m_s", |v, x| v.ie_m_s = x),
        ("erosion.erod13.te_s", |v, x| v.te_s = x),
        ("erosion.erod13.fs", |v, x| v.fs = x),
        ("erosion.erod13.ft", |v, x| v.ft = x),
        ("erosion.erod13.taufe_pa", |v, x| v.taufe_pa = x),
        ("erosion.erod13.q_m2_s", |v, x| v.q_m2_s = x),
        ("erosion.erod13.g_kg_s_m", |v, x| v.g_kg_s_m = x),
        ("erosion.erod13.di_kg_s_m2", |v, x| v.di_kg_s_m2 = x),
        ("erosion.erod13.beta", |v, x| v.beta = x),
        ("erosion.erod13.vf_m_s", |v, x| v.vf_m_s = x),
        ("erosion.erod13.dgdx_kg_s_m2", |v, x| v.dgdx_kg_s_m2 = x),
        ("erosion.erod13.cntlen_m", |v, x| v.cntlen_m = x),
        ("erosion.erod13.kr_s_m", |v, x| v.kr_s_m = x),
        ("erosion.erod13.kradjf", |v, x| v.kradjf = x),
        ("erosion.erod13.tcadjf", |v, x| v.tcadjf = x),
        ("erosion.erod13.shrsol_pa", |v, x| v.shrsol_pa = x),
        ("erosion.erod13.tcend_kg_s_m", |v, x| v.tcend_kg_s_m = x),
        ("erosion.erod13.shcrit_pa", |v, x| v.shcrit_pa = x),
        ("erosion.erod13.detinr_kg_s_m2", |v, x| v.detinr_kg_s_m2 = x),
        ("erosion.erod13.effdrr_m", |v, x| v.effdrr_m = x),
        ("erosion.erod13.effdrn_m", |v, x| v.effdrn_m = x),
        ("erosion.erod13.veleff_m_s", |v, x| v.veleff_m_s = x),
        ("erosion.erod13.pkro_m3_s", |v, x| v.pkro_m3_s = x),
        ("erosion.erod13.tc_k", |v, x| v.tc_k = x),
        ("erosion.erod13.tc_m", |v, x| v.tc_m = x),
        ("erosion.erod13.q_runoff_m", |v, x| v.q_runoff_m = x),
        ("erosion.erod13.peakro_m3_s", |v, x| v.peakro_m3_s = x),
        ("erosion.erod13.watdur_s", |v, x| v.watdur_s = x),
    ];
    for (field, mutate) in fields {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut inputs = nominal_inputs();
            mutate(&mut inputs, value);
            assert_eq!(
                compute_direct_erod13(&inputs),
                Err(DirectRuntimeError::NonFiniteDirectValue { field })
            );
        }
    }
}

#[test]
fn hb01_g_h_watdur_and_sediment_residuals_fail_closed() {
    let mut watdur = nominal_inputs();
    watdur.watdur_s += 1.0e-6;
    assert_eq!(
        compute_direct_erod13(&watdur),
        Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod13.watdur_s"
        })
    );

    let mut sediment = nominal_inputs();
    sediment.dgdx_kg_s_m2 += 1.0e-6;
    assert_eq!(
        compute_direct_erod13(&sediment),
        Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod13.dgdx"
        })
    );
}

#[test]
fn hb01_h_first_invalid_input_retains_guard_priority() {
    let mut inputs = nominal_inputs();
    inputs.ie_m_s = f64::NAN;
    inputs.te_s = 0.0;
    inputs.watdur_s = 11.0;
    assert_eq!(
        compute_direct_erod13(&inputs),
        Err(DirectRuntimeError::NonFiniteDirectValue {
            field: "erosion.erod13.ie_m_s"
        })
    );
}

#[test]
fn hb01_a_real_r7d6_consumer_publishes_wave1_state_and_operands() {
    let identity = DirectRunIdentity::new(1, 1, 1, 1).expect("valid identity");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("valid day frame");
    day.erosion_inputs.wave1_enabled = true;
    day.erosion_inputs.wave1 = nominal_inputs();
    day.peak_runoff_shadow_projection = Some(DirectPeakRunoffShadowProjection {
        lane_index: 0,
        day_index: 0,
        q_runoff_m: 0.01,
        peak_runoff_m3_s: 0.001,
        runoff_duration_s: 10.0,
        method_branch: 1.0,
        tstar: 0.0,
        qpstar: 0.0,
        vstar: 0.0,
    });

    let report = day
        .run_r7d6_erosion_span()
        .expect("real direct erosion span must consume valid EROD13 inputs");
    let wave1 = day.erosion.wave1.expect("Wave-1 state must be stored");
    assert_eq!(wave1.tc_kg_s_m, 2.0);
    assert_eq!(
        day.erosion_downstream_operands.publication,
        day.erosion.publication
    );
    assert!(report.erosion_shadow_projection.wave1_active);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
}
