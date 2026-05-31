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

const WB16_TEST_TOLERANCE: f64 = 1.0e-6;

#[allow(clippy::too_many_lines)]
fn seeded_wb16_surface() -> HillslopeWritebackSurface {
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
    // WB18 per-layer percolation inputs (WB11 compatibility lane).
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
        BoundaryValue::scalar(WB16_TEST_TOLERANCE),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(11.709_931_093_255_933),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(WB16_TEST_TOLERANCE),
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
        .expect("wb16 execution should return typed report")
}

fn wb16_tc_from_vstar(vstar: f64) -> f64 {
    let discriminant = 1.0 - (2.4 * (1.0 - vstar) * vstar);
    (1.0 - discriminant.sqrt()) / (1.2 * (1.0 - vstar))
}

fn assert_branch(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
    expected: f64,
) {
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let q = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("Q should be present")
        .as_f64();
    let peakro = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("peakro"))
        .expect("peakro should be present")
        .as_f64();
    let watdur = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("watdur"))
        .expect("watdur should be present")
        .as_f64();
    let branch = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb16_peak_method_branch"))
        .expect("wb16_peak_method_branch should be present")
        .as_f64();

    assert!((branch - expected).abs() <= f64::EPSILON);
    assert!(peakro.is_finite());
    assert!(watdur.is_finite());
    assert!(peakro > 0.0);
    assert!(watdur >= 0.0);
    assert!((watdur - (q / peakro)).abs() <= WB16_TEST_TOLERANCE);

    let tstar = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb16_tstar"))
        .expect("wb16_tstar should be present")
        .as_f64();
    let vstar = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb16_vstar"))
        .expect("wb16_vstar should be present")
        .as_f64();
    if (expected - 1.0).abs() <= f64::EPSILON {
        assert!(tstar >= 1.0 - WB16_TEST_TOLERANCE);
    } else if (expected - 2.0).abs() <= f64::EPSILON {
        let tc = wb16_tc_from_vstar(vstar);
        assert!(vstar < 1.0);
        assert!(tstar < 1.0 + WB16_TEST_TOLERANCE);
        assert!(tstar > tc - WB16_TEST_TOLERANCE);
    } else if (expected - 3.0).abs() <= f64::EPSILON {
        let tc = wb16_tc_from_vstar(vstar);
        assert!(vstar < 1.0);
        assert!(tstar > 0.0);
        assert!(tstar <= tc + WB16_TEST_TOLERANCE);
    } else if (expected - 4.0).abs() <= f64::EPSILON {
        assert!(vstar >= 1.0 - WB16_TEST_TOLERANCE);
        assert!(tstar < 1.0 + WB16_TEST_TOLERANCE);
    } else {
        panic!("unexpected WB16 branch id {expected}");
    }
}

#[test]
fn wb16_contract_conformance_emits_peak_runoff_outputs_with_branch_authority() {
    let mut branch1 = seeded_wb16_surface();
    branch1
        .state_surface
        .insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(2.0));

    let mut branch2 = seeded_wb16_surface();
    branch2
        .state_surface
        .insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(0.6));

    let mut branch3 = seeded_wb16_surface();
    branch3
        .state_surface
        .insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(0.01));

    let mut branch4 = seeded_wb16_surface();
    branch4.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.6),
    );
    branch4.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(10.0),
    );
    branch4.state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(10.0),
    );
    branch4.state_surface.insert(
        BoundarySymbol::from("intsty_0001"),
        BoundaryValue::scalar(0.05),
    );
    branch4.state_surface.insert(
        BoundarySymbol::from("intsty_0002"),
        BoundaryValue::scalar(0.05),
    );
    branch4.state_surface.insert(
        BoundarySymbol::from("intsty_0003"),
        BoundaryValue::scalar(0.05),
    );
    branch4
        .state_surface
        .insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(0.6));

    let report1 = run_surface(branch1);
    let report2 = run_surface(branch2);
    let report3 = run_surface(branch3);
    let report4 = run_surface(branch4);

    assert_branch(&report1, 1.0);
    assert_branch(&report2, 2.0);
    assert_branch(&report3, 3.0);
    assert_branch(&report4, 4.0);
}

#[test]
fn wb16_contract_conformance_rejects_missing_peak_symbol() {
    let mut surface = seeded_wb16_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("efflen"));

    let report = run_surface(surface);

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::ClosureDiagnostics)
        .expect("closure diagnostics report should exist");
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-WB16-PEAK-E-001"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn wb16_contract_conformance_rejects_non_finite_peak_symbol() {
    let mut surface = seeded_wb16_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(f64::NAN));

    let report = run_surface(surface);

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::ClosureDiagnostics)
        .expect("closure diagnostics report should exist");
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-WB16-PEAK-E-002"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn wb16_contract_conformance_rejects_out_of_domain_peak_symbol() {
    let mut surface = seeded_wb16_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(0.0));

    let report = run_surface(surface);

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::ClosureDiagnostics)
    );
    let phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::ClosureDiagnostics)
        .expect("closure diagnostics report should exist");
    assert_eq!(
        phase.decision_status.message_id(),
        "HKERNEL-WB16-PEAK-E-003"
    );
    assert_eq!(
        phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn wb16_contract_conformance_executes_without_timep_symbol() {
    let mut surface = seeded_wb16_surface();
    surface.state_surface.remove(&BoundarySymbol::from("timep"));

    let report = run_surface(surface);
    assert!(
        report.scheduler_report.is_success(),
        "wb16 execution should not require timep; halted_phase={:?}",
        report.scheduler_report.halted_phase
    );
}

#[test]
fn wb16_contract_conformance_accepts_near_zero_positive_runoff_with_floor_canonicalization() {
    let mut surface = seeded_wb16_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(5.0e-9),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(1.0),
    );

    let report = run_surface(surface);

    assert!(
        report.scheduler_report.is_success(),
        "near-zero runoff should remain compatibility-valid; halted_phase={:?}",
        report.scheduler_report.halted_phase
    );

    let peakro = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("peakro"))
        .expect("peakro should be present")
        .as_f64();
    let watdur = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("watdur"))
        .expect("watdur should be present")
        .as_f64();

    assert!(peakro.is_finite());
    assert!(peakro + WB16_TEST_TOLERANCE >= 3.63e-8);
    assert!(watdur.is_finite());
    assert!(watdur >= -WB16_TEST_TOLERANCE);
}
