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

const EXPECTED_WB14_INFILTRATION: f64 = 2.909_931_093_255_933;
const EXPECTED_WB14_Q: f64 = 0.290_068_906_744_067;
const WB14_TEST_TOLERANCE: f64 = 1.0e-6;
const WB14_KSATADJ_TOLERANCE: f64 = 1.0e-9;

fn state_scalar(surface: &HillslopeWritebackSurface, symbol: &str) -> f64 {
    surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .expect("required seeded symbol should be present")
        .as_f64()
}

fn run_wb14_reconciliation_outputs(surface: HillslopeWritebackSurface) -> (f64, f64) {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb14 execution should return typed report");
    if !report.scheduler_report.is_success() {
        let phase_trace = report
            .phase_reports
            .iter()
            .map(|phase| {
                (
                    phase.phase,
                    phase.decision_status.message_id().to_string(),
                    format!("{:?}", phase.decision_status),
                )
            })
            .collect::<Vec<_>>();
        panic!(
            "scheduler halted at {:?}; phase trace: {:?}",
            report.scheduler_report.halted_phase, phase_trace
        );
    }

    let infiltration = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_infiltration"))
        .expect("wb12_infiltration should be present")
        .as_f64();
    let q_runoff = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("Q should be present")
        .as_f64();
    (infiltration, q_runoff)
}

fn capture_pre_runoff_state_surface(
    mut surface: HillslopeWritebackSurface,
) -> HillslopeWritebackSurface {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    surface
        .state_surface
        .remove(&BoundarySymbol::from("timem_0002"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("runoff probe should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );
    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-001"
    );

    report.writeback_surface
}

fn wb14_expected_ke_9001(surface: &HillslopeWritebackSurface, ksatfac: f64, ksatrec: f64) -> f64 {
    let ssc = state_scalar(surface, "ssc");
    let theta_1 = state_scalar(surface, "wb18_perc_theta_0001");
    let theta_2 = state_scalar(surface, "wb18_perc_theta_0002");
    let ul_1 = state_scalar(surface, "wb18_perc_ul_0001");
    let ul_2 = state_scalar(surface, "wb18_perc_ul_0002");
    let sat_frac = ((theta_1 + theta_2) / (ul_1 + ul_2)).min(1.0);

    let upper_ks = ssc * 3.6e6;
    let lower_ks = upper_ks / ksatfac;
    let keff = ((upper_ks - lower_ks) / ((1.0 / ksatrec).exp() - 1.0))
        * ((sat_frac / ksatrec).exp() - 1.0)
        + lower_ks;
    keff / 3.6e6
}

fn wb14_expected_ke_9002_or_9003(surface: &HillslopeWritebackSurface, lkeff: Option<f64>) -> f64 {
    let ssc = state_scalar(surface, "ssc");
    let theta_1 = state_scalar(surface, "wb18_perc_theta_0001");
    let theta_2 = state_scalar(surface, "wb18_perc_theta_0002");
    let ul_1 = state_scalar(surface, "wb18_perc_ul_0001");
    let ul_2 = state_scalar(surface, "wb18_perc_ul_0002");
    let sat_frac = ((theta_1 + theta_2) / (ul_1 + ul_2)).min(1.0);

    let fc_1 = state_scalar(surface, "wb18_perc_fc_0001");
    let fc_2 = state_scalar(surface, "wb18_perc_fc_0002");
    let dg_1 = state_scalar(surface, "dg_0001");
    let dg_2 = state_scalar(surface, "dg_0002");
    let tillage_depth = dg_1 + dg_2;
    let avthetafc = (fc_1 + fc_2) / tillage_depth;
    let avthetadr = ((ul_1 - fc_1) + (ul_2 - fc_2)) / tillage_depth;

    let psi = (1500.0_f64.ln() - 33.0_f64.ln()) / (avthetafc.ln() - avthetadr.ln());
    let lambda = 1.0 / psi;
    let mut keff = (ssc * 3.6e6) * sat_frac.powf((2.0 * lambda) + 3.0);
    if let Some(lower_bound) = lkeff {
        if lower_bound > 0.0 && keff < lower_bound {
            keff = lower_bound;
        }
    }
    keff / 3.6e6
}

#[allow(clippy::too_many_lines)]
fn seeded_wb14_surface() -> HillslopeWritebackSurface {
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
        BoundaryValue::scalar(EXPECTED_WB14_Q),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(WB14_TEST_TOLERANCE),
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
        BoundaryValue::scalar(WB14_TEST_TOLERANCE),
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

#[test]
fn wb14_contract_conformance_computes_infiltration_from_hyetograph() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_wb14_surface())
        .expect("wb14 execution should return typed report");

    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let infiltration = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_infiltration"))
        .expect("wb12_infiltration should be present")
        .as_f64();
    assert!((infiltration - EXPECTED_WB14_INFILTRATION).abs() <= WB14_TEST_TOLERANCE);

    let q_runoff = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("Q should be present")
        .as_f64();
    assert!((q_runoff - EXPECTED_WB14_Q).abs() <= WB14_TEST_TOLERANCE);

    let runoff_closure = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("wb12_runoff_closure_delta"))
        .expect("wb12_runoff_closure_delta should be present")
        .as_f64();
    assert!(runoff_closure.abs() <= WB14_TEST_TOLERANCE);
}

#[test]
fn wb14_contract_conformance_normalizes_within_tolerance_negative_runoff_before_writeback() {
    let (_baseline_infiltration, baseline_q) =
        run_wb14_reconciliation_outputs(seeded_wb14_surface());

    let mut adjusted_surface = seeded_wb14_surface();
    let original_runon = state_scalar(&adjusted_surface, "wb12_runon_input");
    let target_runon = original_runon - baseline_q - 5.0e-13;
    adjusted_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(target_runon),
    );
    adjusted_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.0),
    );
    adjusted_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, adjusted_surface)
        .expect("wb14 execution should return typed report");
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let runoff_state = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_runoff_reconciled"))
        .expect("wb12_runoff_reconciled should be present")
        .as_f64();
    let runoff_flux = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("Q should be present")
        .as_f64();

    assert!(
        runoff_state >= 0.0,
        "wb12_runoff_reconciled must be non-negative"
    );
    assert!(runoff_flux >= 0.0, "Q must be non-negative");
    assert!(
        runoff_state <= WB14_TEST_TOLERANCE,
        "within-tolerance negative runoff should normalize to near-zero state; observed {runoff_state}"
    );
    assert!(
        runoff_flux <= WB14_TEST_TOLERANCE,
        "within-tolerance negative runoff should normalize to near-zero flux; observed {runoff_flux}"
    );
}

#[test]
fn wb14_contract_conformance_rejects_missing_hyetograph_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb14_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("timem_0002"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb14 failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-001"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn wb14_contract_conformance_rejects_non_monotone_hyetograph_time() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb14_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("timem_0002"),
        BoundaryValue::scalar(0.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb14 failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-003"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn wb14_contract_conformance_applies_ksatadj_9001_regime() {
    let mut ksatadj_surface = seeded_wb14_surface();
    ksatadj_surface.state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(9001.0),
    );
    ksatadj_surface
        .state_surface
        .insert(BoundarySymbol::from("ksatadj"), BoundaryValue::scalar(1.0));
    ksatadj_surface
        .state_surface
        .insert(BoundarySymbol::from("ksatfac"), BoundaryValue::scalar(5.0));
    ksatadj_surface
        .state_surface
        .insert(BoundarySymbol::from("ksatrec"), BoundaryValue::scalar(0.35));
    ksatadj_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let pre_runoff_surface = capture_pre_runoff_state_surface(ksatadj_surface.clone());
    let expected_ke = wb14_expected_ke_9001(&pre_runoff_surface, 5.0, 0.35);

    let mut expected_surface = ksatadj_surface.clone();
    expected_surface
        .state_surface
        .insert(BoundarySymbol::from("ksatadj"), BoundaryValue::scalar(0.0));
    expected_surface.state_surface.insert(
        BoundarySymbol::from("ssc"),
        BoundaryValue::scalar(expected_ke),
    );
    expected_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let (ksatadj_infiltration, ksatadj_q) = run_wb14_reconciliation_outputs(ksatadj_surface);
    let (expected_infiltration, expected_q) = run_wb14_reconciliation_outputs(expected_surface);

    assert!(
        (ksatadj_infiltration - expected_infiltration).abs() <= WB14_KSATADJ_TOLERANCE,
        "infiltration mismatch: actual={ksatadj_infiltration:.12}, expected={expected_infiltration:.12}, delta={:.12}",
        (ksatadj_infiltration - expected_infiltration).abs()
    );
    assert!(
        (ksatadj_q - expected_q).abs() <= WB14_KSATADJ_TOLERANCE,
        "runoff mismatch: actual={ksatadj_q:.12}, expected={expected_q:.12}, delta={:.12}",
        (ksatadj_q - expected_q).abs()
    );
}

#[test]
fn wb14_contract_conformance_applies_ksatadj_9002_regime() {
    let mut ksatadj_surface = seeded_wb14_surface();
    ksatadj_surface.state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(9002.0),
    );
    ksatadj_surface
        .state_surface
        .insert(BoundarySymbol::from("ksatadj"), BoundaryValue::scalar(1.0));
    ksatadj_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let pre_runoff_surface = capture_pre_runoff_state_surface(ksatadj_surface.clone());
    let expected_ke = wb14_expected_ke_9002_or_9003(&pre_runoff_surface, None);

    let mut expected_surface = ksatadj_surface.clone();
    expected_surface
        .state_surface
        .insert(BoundarySymbol::from("ksatadj"), BoundaryValue::scalar(0.0));
    expected_surface.state_surface.insert(
        BoundarySymbol::from("ssc"),
        BoundaryValue::scalar(expected_ke),
    );
    expected_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let (ksatadj_infiltration, ksatadj_q) = run_wb14_reconciliation_outputs(ksatadj_surface);
    let (expected_infiltration, expected_q) = run_wb14_reconciliation_outputs(expected_surface);

    assert!(
        (ksatadj_infiltration - expected_infiltration).abs() <= WB14_KSATADJ_TOLERANCE,
        "infiltration mismatch: actual={ksatadj_infiltration:.12}, expected={expected_infiltration:.12}, delta={:.12}",
        (ksatadj_infiltration - expected_infiltration).abs()
    );
    assert!(
        (ksatadj_q - expected_q).abs() <= WB14_KSATADJ_TOLERANCE,
        "runoff mismatch: actual={ksatadj_q:.12}, expected={expected_q:.12}, delta={:.12}",
        (ksatadj_q - expected_q).abs()
    );
}

#[test]
fn wb14_contract_conformance_applies_ksatadj_9003_burn_floor() {
    let mut ksatadj_surface = seeded_wb14_surface();
    ksatadj_surface.state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(9003.0),
    );
    ksatadj_surface
        .state_surface
        .insert(BoundarySymbol::from("ksatadj"), BoundaryValue::scalar(1.0));
    ksatadj_surface.state_surface.insert(
        BoundarySymbol::from("lkeff"),
        BoundaryValue::scalar(1_000_000.0),
    );
    ksatadj_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let pre_runoff_surface = capture_pre_runoff_state_surface(ksatadj_surface.clone());
    let expected_ke = wb14_expected_ke_9002_or_9003(&pre_runoff_surface, Some(1_000_000.0));

    let mut expected_surface = ksatadj_surface.clone();
    expected_surface
        .state_surface
        .insert(BoundarySymbol::from("ksatadj"), BoundaryValue::scalar(0.0));
    expected_surface.state_surface.insert(
        BoundarySymbol::from("ssc"),
        BoundaryValue::scalar(expected_ke),
    );
    expected_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let (ksatadj_infiltration, ksatadj_q) = run_wb14_reconciliation_outputs(ksatadj_surface);
    let (expected_infiltration, expected_q) = run_wb14_reconciliation_outputs(expected_surface);

    assert!(
        (ksatadj_infiltration - expected_infiltration).abs() <= WB14_KSATADJ_TOLERANCE,
        "infiltration mismatch: actual={ksatadj_infiltration:.12}, expected={expected_infiltration:.12}, delta={:.12}",
        (ksatadj_infiltration - expected_infiltration).abs()
    );
    assert!(
        (ksatadj_q - expected_q).abs() <= WB14_KSATADJ_TOLERANCE,
        "runoff mismatch: actual={ksatadj_q:.12}, expected={expected_q:.12}, delta={:.12}",
        (ksatadj_q - expected_q).abs()
    );
}

#[test]
fn wb14_contract_conformance_rejects_active_9001_zero_ksatrec() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb14_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(9001.0),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("ksatadj"), BoundaryValue::scalar(1.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("ksatfac"), BoundaryValue::scalar(5.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("ksatrec"), BoundaryValue::scalar(0.0));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb14 failure should return typed report");
    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-003"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}
