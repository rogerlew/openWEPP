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

#[allow(clippy::too_many_lines)]
fn seeded_surface() -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2006.0),
    );
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(0.5));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("pltol"), BoundaryValue::scalar(0.25));

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
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(50.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
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
        BoundarySymbol::from("thetfc_0002"),
        BoundaryValue::scalar(40.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.0),
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
    state_surface.insert(
        BoundarySymbol::from("por_0001"),
        BoundaryValue::scalar(0.55),
    );
    state_surface.insert(
        BoundarySymbol::from("por_0002"),
        BoundaryValue::scalar(0.55),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0002"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0002"),
        BoundaryValue::scalar(1.0),
    );
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
        BoundaryValue::scalar(13.169_814_232_504_201),
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
        .expect("erod13 execution should return typed report")
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
fn erod13_contract_vector_nominal_detachment_emits_core_outputs() {
    let report = run_surface(seeded_surface());
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );
    let dc = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("Dc"))
        .expect("Dc should be present")
        .as_f64();
    let tc = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("Tc"))
        .expect("Tc should be present")
        .as_f64();
    let df = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("Df"))
        .expect("Df should be present")
        .as_f64();

    assert!((dc - 0.99).abs() <= TEST_TOLERANCE, "Dc={dc}");
    assert!((tc - 1.25).abs() <= TEST_TOLERANCE, "Tc={tc}");
    assert!((df - 0.8316).abs() <= TEST_TOLERANCE, "Df={df}");
}

#[test]
fn erod13_contract_vector_threshold_branch_sets_zero_df() {
    let mut surface = seeded_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("taufe"), BoundaryValue::scalar(0.1));
    surface
        .state_surface
        .insert(BoundarySymbol::from("G"), BoundaryValue::scalar(0.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("dGdx"), BoundaryValue::scalar(0.05));

    let report = run_surface(surface);
    if !report.scheduler_report.is_success() {
        let phase = closure_phase_report(&report);
        panic!(
            "expected success, got {} ({:?})",
            phase.decision_status.message_id(),
            phase.decision_status.boundary_class()
        );
    }
    let df = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("Df"))
        .expect("Df should be present")
        .as_f64();
    assert!(df.abs() <= TEST_TOLERANCE, "Df={df}");
}

#[test]
fn erod13_contract_vector_deposition_branch_emits_negative_df() {
    let mut surface = seeded_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("G"), BoundaryValue::scalar(2.0));
    surface.state_surface.insert(
        BoundarySymbol::from("dGdx"),
        BoundaryValue::scalar(-0.04375),
    );

    let report = run_surface(surface);
    assert!(report.scheduler_report.is_success());
    let df = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("Df"))
        .expect("Df should be present")
        .as_f64();
    assert!(df < 0.0, "Df={df}");
}

#[test]
fn erod13_contract_vector_rejects_missing_required_symbol() {
    let mut surface = seeded_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("cntlen"));

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD13-CORE-E-001"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn erod13_contract_vector_rejects_non_finite_required_symbol() {
    let mut surface = seeded_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("shcrit"),
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
        "HKERNEL-EROD13-CORE-E-002"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn erod13_contract_vector_rejects_domain_violation() {
    let mut surface = seeded_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("tcadjf"), BoundaryValue::scalar(0.25));

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD13-CORE-E-003"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn erod13_contract_vector_rejects_continuity_residual_violation() {
    let mut surface = seeded_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("dGdx"), BoundaryValue::scalar(9.0));

    let report = run_surface(surface);
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = closure_phase_report(&report);
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-EROD13-CORE-E-003"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}
