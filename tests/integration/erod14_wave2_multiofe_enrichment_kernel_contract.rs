use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseScheduler, HillslopeWritebackSurface, Wb11HydrologyKernel,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use openwepp_sim_contract::status::BoundaryClass;
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

const TEST_TOLERANCE: f64 = 1.0e-6;
const WAVE2_TEST_CLASSES: usize = 3;
const WAVE2_TEST_CLASSES_SCALAR: f64 = 3.0;

#[allow(clippy::too_many_lines)]
fn seeded_surface() -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(0.5));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.0));

    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(2.0e-6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(4.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0002"),
        BoundaryValue::scalar(2.0e-5),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("nslpts"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("xinput_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("slpinp_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(39.653_865_297_983_295),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_depth"),
        BoundaryValue::scalar(0.15),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_spacing"),
        BoundaryValue::scalar(0.285),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_diameter"),
        BoundaryValue::scalar(0.1),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_lateral_fraction"),
        BoundaryValue::scalar(0.25),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainage_fraction"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainage_coefficient"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(2.0),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(3.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.290_068_906_744_067),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(TEST_TOLERANCE),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(9.959_931_093_255_933),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(TEST_TOLERANCE),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(3.0),
    );

    state_surface.insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(4.0));
    state_surface.insert(
        BoundarySymbol::from("timem_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("timem_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("timem_0003"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("timem_0004"),
        BoundaryValue::scalar(3.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0003"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0004"),
        BoundaryValue::scalar(0.0),
    );

    state_surface.insert(BoundarySymbol::from("timep"), BoundaryValue::scalar(0.25));
    state_surface.insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));

    state_surface.insert(
        BoundarySymbol::from("erod13_core_enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("Ie"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("te"), BoundaryValue::scalar(3.0));
    state_surface.insert(BoundarySymbol::from("fs"), BoundaryValue::scalar(0.6));
    state_surface.insert(BoundarySymbol::from("ft"), BoundaryValue::scalar(1.2));
    state_surface.insert(BoundarySymbol::from("taufe"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.4));

    state_surface.insert(BoundarySymbol::from("G"), BoundaryValue::scalar(0.2));
    state_surface.insert(BoundarySymbol::from("Di"), BoundaryValue::scalar(0.05));
    state_surface.insert(BoundarySymbol::from("beta"), BoundaryValue::scalar(0.5));
    state_surface.insert(BoundarySymbol::from("vf"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("dGdx"), BoundaryValue::scalar(0.8816));

    state_surface.insert(BoundarySymbol::from("cntlen"), BoundaryValue::scalar(10.0));
    state_surface.insert(BoundarySymbol::from("kr"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("kradjf"), BoundaryValue::scalar(1.1));
    state_surface.insert(BoundarySymbol::from("tcadjf"), BoundaryValue::scalar(0.5));
    state_surface.insert(BoundarySymbol::from("shrsol"), BoundaryValue::scalar(0.8));
    state_surface.insert(BoundarySymbol::from("tcend"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("shcrit"), BoundaryValue::scalar(0.4));
    state_surface.insert(BoundarySymbol::from("detinr"), BoundaryValue::scalar(0.2));
    state_surface.insert(BoundarySymbol::from("effdrr"), BoundaryValue::scalar(1.2));
    state_surface.insert(BoundarySymbol::from("effdrn"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("veleff"), BoundaryValue::scalar(0.9));
    state_surface.insert(BoundarySymbol::from("pkro"), BoundaryValue::scalar(1.5));
    state_surface.insert(
        BoundarySymbol::from("erod13_tc_k"),
        BoundaryValue::scalar(2.5),
    );
    state_surface.insert(
        BoundarySymbol::from("erod13_tc_m"),
        BoundaryValue::scalar(1.2),
    );

    state_surface.insert(
        BoundarySymbol::from("erod14_wave2_enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_class_count"),
        BoundaryValue::scalar(WAVE2_TEST_CLASSES_SCALAR),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_xtop"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_xbot"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_xdetst"),
        BoundaryValue::scalar(0.1),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_ldtop"),
        BoundaryValue::scalar(0.8),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_ldbot"),
        BoundaryValue::scalar(0.6),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_lddend"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_qout"),
        BoundaryValue::scalar(1.2),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_qin"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_qostar"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(BoundarySymbol::from("qostar"), BoundaryValue::scalar(0.2));
    state_surface.insert(
        BoundarySymbol::from("erod14_slplen"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_ktrato"),
        BoundaryValue::scalar(1.1),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_ainftc"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_binftc"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_cinftc"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(BoundarySymbol::from("xdetst"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("lddend"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("xu_0002"), BoundaryValue::scalar(0.2));
    state_surface.insert(BoundarySymbol::from("xl_0002"), BoundaryValue::scalar(0.5));
    state_surface.insert(
        BoundarySymbol::from("ainf_0002"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("binf_0002"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("cinf_0002"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("ainftc_0002"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("binftc_0002"),
        BoundaryValue::scalar(0.3),
    );
    state_surface.insert(
        BoundarySymbol::from("cinftc_0002"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_beta"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_Qj_minus_1"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_Vj"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_Qj"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_Fh"),
        BoundaryValue::scalar(0.8),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_Fp"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_case"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("erod14_ssa_soil"),
        BoundaryValue::scalar(5.0),
    );

    for (index, (fall, frcflw, frac, fidel, tcf1, ssa_class)) in [
        (0.02, 0.3, 0.3, 0.25, 0.4, 1.5),
        (0.01, 0.4, 0.4, 0.35, 0.3, 4.0),
        (0.005, 0.3, 0.3, 0.40, 0.2, 8.0),
    ]
    .into_iter()
    .enumerate()
    {
        let class = index + 1;
        state_surface.insert(
            BoundarySymbol::from(format!("erod14_fall_{class:04}")),
            BoundaryValue::scalar(fall),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("erod14_frcflw_{class:04}")),
            BoundaryValue::scalar(frcflw),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("erod14_frac_{class:04}")),
            BoundaryValue::scalar(frac),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("erod14_fidel_{class:04}")),
            BoundaryValue::scalar(fidel),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("erod14_tcf1_{class:04}")),
            BoundaryValue::scalar(tcf1),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("erod14_ssa_class_{class:04}")),
            BoundaryValue::scalar(ssa_class),
        );
    }

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: std::collections::BTreeMap::new(),
    }
}

fn run_surface(
    surface: HillslopeWritebackSurface,
) -> openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("erod14 execution should return typed report")
}

fn closure_phase_report(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
) -> &openwepp_hillslope_orchestrator::HillslopeKernelPhaseReport {
    report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::ClosureDiagnostics)
        .expect("closure diagnostics report should exist")
}

#[test]
#[allow(clippy::too_many_lines)]
fn erod14_contract_vector_nominal_multiofe_enrichment_emits_outputs() {
    let report = run_surface(seeded_surface());
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let mut sed_frac_sum = 0.0;
    let mut particle_flow_fraction_sum = 0.0;
    for class in 1..=WAVE2_TEST_CLASSES {
        let gend = report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(format!("erod14_gend_{class:04}")))
            .expect("class gend should be present")
            .as_f64();
        let sedmax = report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(format!("erod14_sedmax_{class:04}")))
            .expect("class sedmax should be present")
            .as_f64();
        let sed_frac = report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(format!("sed_frac_{class:04}")))
            .expect("class sed_frac should be present")
            .as_f64();
        let concentration = report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(format!(
                "sediment_concentration_kg_m3_{class:04}"
            )))
            .expect("class concentration should be present")
            .as_f64();
        let particle_flow_fraction = report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(format!(
                "particle_flow_fraction_{class:04}"
            )))
            .expect("class particle flow fraction should be present")
            .as_f64();

        assert!(gend.is_finite() && gend >= 0.0, "gend[{class}]={gend}");
        assert!(
            sedmax.is_finite() && sedmax >= 0.0,
            "sedmax[{class}]={sedmax}"
        );
        assert!(
            gend <= sedmax + TEST_TOLERANCE,
            "gend[{class}]={gend} exceeds sedmax={sedmax}"
        );
        assert!(
            sed_frac.is_finite() && sed_frac >= 0.0,
            "sed_frac[{class}]={sed_frac}"
        );
        assert!(
            concentration.is_finite() && concentration >= 0.0,
            "concentration[{class}]={concentration}"
        );
        assert!(
            particle_flow_fraction.is_finite() && particle_flow_fraction >= 0.0,
            "particle_flow_fraction[{class}]={particle_flow_fraction}"
        );
        sed_frac_sum += sed_frac;
        particle_flow_fraction_sum += particle_flow_fraction;
    }
    assert!(
        (sed_frac_sum - 1.0).abs() <= TEST_TOLERANCE,
        "sed_frac_sum={sed_frac_sum}"
    );
    assert!(
        (particle_flow_fraction_sum - 1.0).abs() <= TEST_TOLERANCE,
        "particle_flow_fraction_sum={particle_flow_fraction_sum}"
    );

    let sumg = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("erod14_sumg"))
        .expect("erod14_sumg should be present")
        .as_f64();
    let er = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("ER"))
        .expect("ER should be present")
        .as_f64();
    let total_detachment = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("total_detachment_kg"))
        .expect("total_detachment_kg should be present")
        .as_f64();
    let total_deposition = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("total_deposition_kg"))
        .expect("total_deposition_kg should be present")
        .as_f64();
    let particle_class_count = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("particle_class_count"))
        .expect("particle_class_count should be present")
        .as_f64();

    assert!(sumg.is_finite() && sumg > 0.0, "sumg={sumg}");
    assert!(er.is_finite() && er > 0.0, "ER={er}");
    assert!(
        (total_detachment - sumg).abs() <= TEST_TOLERANCE,
        "total_detachment_kg must match erod14_sumg"
    );
    assert!(total_deposition.is_finite() && total_deposition >= 0.0);
    assert!(
        (particle_class_count - WAVE2_TEST_CLASSES_SCALAR).abs() <= TEST_TOLERANCE,
        "particle_class_count={particle_class_count}"
    );
}

#[test]
fn erod14_contract_vector_case_four_zero_outflow_emits_zero_class_fractions() {
    let mut surface = seeded_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_case"),
        BoundaryValue::scalar(4.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_Vj"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_Qj"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fh"),
        BoundaryValue::scalar(0.1),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fp"),
        BoundaryValue::scalar(0.2),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_qout"),
        BoundaryValue::scalar(0.0),
    );

    let report = run_surface(surface);
    assert!(report.scheduler_report.is_success());

    for class in 1..=WAVE2_TEST_CLASSES {
        let sed_frac = report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(format!("sed_frac_{class:04}")))
            .expect("sed_frac should be present")
            .as_f64();
        let concentration = report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(format!(
                "sediment_concentration_kg_m3_{class:04}"
            )))
            .expect("concentration should be present")
            .as_f64();
        let particle_flow_fraction = report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(format!(
                "particle_flow_fraction_{class:04}"
            )))
            .expect("particle_flow_fraction should be present")
            .as_f64();
        assert!(
            sed_frac.abs() <= TEST_TOLERANCE,
            "sed_frac[{class}]={sed_frac}"
        );
        assert!(
            concentration.abs() <= TEST_TOLERANCE,
            "concentration[{class}]={concentration}"
        );
        assert!(
            particle_flow_fraction.abs() <= TEST_TOLERANCE,
            "particle_flow_fraction[{class}]={particle_flow_fraction}"
        );
    }

    let total_detachment = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("total_detachment_kg"))
        .expect("total_detachment_kg should be present")
        .as_f64();
    let total_deposition = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("total_deposition_kg"))
        .expect("total_deposition_kg should be present")
        .as_f64();
    let particle_class_count = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("particle_class_count"))
        .expect("particle_class_count should be present")
        .as_f64();
    assert!(total_detachment.abs() <= TEST_TOLERANCE);
    assert!((total_deposition - 0.3).abs() <= TEST_TOLERANCE);
    assert!(
        (particle_class_count - WAVE2_TEST_CLASSES_SCALAR).abs() <= TEST_TOLERANCE,
        "particle_class_count={particle_class_count}"
    );
}

#[test]
fn erod14_contract_vector_rejects_missing_required_symbol() {
    let mut surface = seeded_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("erod14_xbot"));

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD14-WAVE2-E-001"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn erod14_contract_vector_rejects_non_finite_required_symbol() {
    let mut surface = seeded_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_qostar"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD14-WAVE2-E-002"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn erod14_contract_vector_rejects_case_classification_mismatch() {
    let mut surface = seeded_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_case"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_Vj"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fh"),
        BoundaryValue::scalar(0.1),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fp"),
        BoundaryValue::scalar(0.2),
    );

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD14-WAVE2-E-003"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn erod14_contract_vector_rejects_unreproportionable_mass_request() {
    let mut surface = seeded_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("erod14_ldbot"),
        BoundaryValue::scalar(10.0),
    );

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD14-WAVE2-E-003"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn erod18_contract_route_topology_rejects_missing_required_symbol() {
    let mut surface = seeded_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("ainf_0002"));

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD18-ROUTE-E-001"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn erod18_contract_route_topology_rejects_non_finite_required_symbol() {
    let mut surface = seeded_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("xu_0002"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD18-ROUTE-E-002"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn erod18_contract_route_topology_rejects_domain_violation() {
    let mut surface = seeded_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("xu_0002"), BoundaryValue::scalar(0.6));
    surface
        .state_surface
        .insert(BoundarySymbol::from("xl_0002"), BoundaryValue::scalar(0.5));

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD18-ROUTE-E-003"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

fn require_state_scalar(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
    symbol: &str,
) -> f64 {
    report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing expected state symbol {symbol}"))
        .as_f64()
}

#[test]
#[ignore = "EROD19 route segment migration pending"]
fn erod17_contract_mshear_dispatch_vector_requires_segment_case_publication() {
    let mut low_shear = seeded_surface();
    low_shear.state_surface.insert(
        BoundarySymbol::from("ainf_0002"),
        BoundaryValue::scalar(0.01),
    );
    low_shear.state_surface.insert(
        BoundarySymbol::from("binf_0002"),
        BoundaryValue::scalar(0.01),
    );
    low_shear.state_surface.insert(
        BoundarySymbol::from("cinf_0002"),
        BoundaryValue::scalar(0.01),
    );

    let mut high_shear = seeded_surface();
    high_shear.state_surface.insert(
        BoundarySymbol::from("ainf_0002"),
        BoundaryValue::scalar(10.0),
    );
    high_shear.state_surface.insert(
        BoundarySymbol::from("binf_0002"),
        BoundaryValue::scalar(10.0),
    );
    high_shear.state_surface.insert(
        BoundarySymbol::from("cinf_0002"),
        BoundaryValue::scalar(10.0),
    );

    let low_report = run_surface(low_shear);
    let high_report = run_surface(high_shear);
    assert!(low_report.scheduler_report.is_success());
    assert!(high_report.scheduler_report.is_success());

    let low_case = require_state_scalar(&low_report, "mshear");
    let high_case = require_state_scalar(&high_report, "mshear");
    assert!(
        (low_case - high_case).abs() > TEST_TOLERANCE,
        "route mshear branch publication must distinguish low/high shear segment vectors"
    );
}

#[test]
#[ignore = "EROD19 route segment migration pending"]
fn erod17_contract_deposition_end_vector_requires_xdend_publication() {
    let mut in_segment = seeded_surface();
    in_segment
        .state_surface
        .insert(BoundarySymbol::from("qostar"), BoundaryValue::scalar(0.3));
    in_segment
        .state_surface
        .insert(BoundarySymbol::from("xu_0002"), BoundaryValue::scalar(0.10));
    in_segment
        .state_surface
        .insert(BoundarySymbol::from("xl_0002"), BoundaryValue::scalar(0.60));

    let mut extends_to_end = seeded_surface();
    extends_to_end
        .state_surface
        .insert(BoundarySymbol::from("qostar"), BoundaryValue::scalar(-0.3));
    extends_to_end
        .state_surface
        .insert(BoundarySymbol::from("xu_0002"), BoundaryValue::scalar(0.10));
    extends_to_end
        .state_surface
        .insert(BoundarySymbol::from("xl_0002"), BoundaryValue::scalar(0.60));

    let in_segment_report = run_surface(in_segment);
    let extends_report = run_surface(extends_to_end);
    assert!(in_segment_report.scheduler_report.is_success());
    assert!(extends_report.scheduler_report.is_success());

    let xdend_in_segment = require_state_scalar(&in_segment_report, "xdend");
    let xdend_extends = require_state_scalar(&extends_report, "xdend");
    let xl_extends = require_state_scalar(&extends_report, "xl_0002");

    assert!(
        xdend_in_segment < xl_extends - TEST_TOLERANCE,
        "deposition-end-in-segment vector must publish xdend strictly inside segment"
    );
    assert!(
        (xdend_extends - xl_extends).abs() <= TEST_TOLERANCE,
        "deposition-extends-to-end vector must publish xdend equal to segment end"
    );
}

#[test]
#[ignore = "EROD19 route segment migration pending"]
fn erod17_contract_ndep_followup_vector_requires_post_detachment_deposition_path() {
    let mut no_followup = seeded_surface();
    no_followup
        .state_surface
        .insert(BoundarySymbol::from("G"), BoundaryValue::scalar(0.2));

    let mut followup = seeded_surface();
    followup
        .state_surface
        .insert(BoundarySymbol::from("G"), BoundaryValue::scalar(0.25));

    let no_followup_report = run_surface(no_followup);
    let followup_report = run_surface(followup);

    let ndep_no_followup = require_state_scalar(&no_followup_report, "ndep");
    let ndep_followup = require_state_scalar(&followup_report, "ndep");
    let lddend_no_followup = require_state_scalar(&no_followup_report, "lddend");
    let lddend_followup = require_state_scalar(&followup_report, "lddend");

    assert!(
        ndep_no_followup <= TEST_TOLERANCE,
        "no-followup vector should keep ndep at zero"
    );
    assert!(
        ndep_followup > 0.0,
        "followup vector must publish non-zero ndep when detachment transitions into deposition"
    );
    assert!(
        (lddend_followup - lddend_no_followup).abs() > TEST_TOLERANCE,
        "post-detachment deposition path should alter lddend"
    );
}

#[test]
#[ignore = "EROD19 route segment migration pending"]
fn erod17_contract_qostar_threshold_vector_requires_upper_boundary_branch_divergence() {
    let mut near_zero = seeded_surface();
    near_zero.state_surface.insert(
        BoundarySymbol::from("qostar"),
        BoundaryValue::scalar(0.0005),
    );

    let mut non_zero = seeded_surface();
    non_zero
        .state_surface
        .insert(BoundarySymbol::from("qostar"), BoundaryValue::scalar(0.02));

    let near_zero_report = run_surface(near_zero);
    let non_zero_report = run_surface(non_zero);
    assert!(near_zero_report.scheduler_report.is_success());
    assert!(non_zero_report.scheduler_report.is_success());

    let dl_near_zero = require_state_scalar(&near_zero_report, "dl");
    let dl_non_zero = require_state_scalar(&non_zero_report, "dl");

    assert!(
        (dl_near_zero - dl_non_zero).abs() > TEST_TOLERANCE,
        "near-zero qostar threshold branch should publish distinct upper-boundary deposition-rate behavior"
    );
}

#[test]
fn erod17_contract_route_branch_seam_vector_requires_core_publication_family() {
    let report = run_surface(seeded_surface());
    assert!(report.scheduler_report.is_success());

    for symbol in [
        "mshear", "xc1", "xc2", "du", "dl", "xdbeg", "xdend", "xdetst", "ndep", "lddend",
    ] {
        let _ = require_state_scalar(&report, symbol);
    }
}
