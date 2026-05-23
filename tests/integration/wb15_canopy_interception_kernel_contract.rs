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

const WB15_TEST_TOLERANCE: f64 = 1.0e-9;
const EXPECTED_INTERCEPTION_I: f64 = 0.001_571_663_34;
const EXPECTED_INFILTRATION: f64 = 2.998_428_336_66;
const EXPECTED_RUNOFF_Q: f64 = 0.0;
const EXPECTED_STORAGE_RECONCILED: f64 = 10.248_428_336_66;

#[allow(clippy::too_many_lines)]
fn seeded_wb15_surface() -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(10.0));

    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.65));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.1));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.6));

    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(2.0),
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
        BoundaryValue::scalar(0.2),
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
        BoundaryValue::scalar(EXPECTED_RUNOFF_Q),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(WB15_TEST_TOLERANCE),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(EXPECTED_STORAGE_RECONCILED),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(WB15_TEST_TOLERANCE),
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

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: std::collections::BTreeMap::new(),
    }
}

#[test]
fn wb15_contract_conformance_couples_canopy_interception_into_runoff_and_storage_closure() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_wb15_surface())
        .expect("wb15 execution should return typed report");

    assert!(report.scheduler_report.is_success());

    let interception = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("I"))
        .expect("I should be present")
        .as_f64();
    assert!((interception - EXPECTED_INTERCEPTION_I).abs() <= WB15_TEST_TOLERANCE);

    let infiltration = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_infiltration"))
        .expect("wb12_infiltration should be present")
        .as_f64();
    assert!((infiltration - EXPECTED_INFILTRATION).abs() <= WB15_TEST_TOLERANCE);

    let runoff = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("Q should be present")
        .as_f64();
    assert!((runoff - EXPECTED_RUNOFF_Q).abs() <= WB15_TEST_TOLERANCE);

    let storage_reconciled = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_storage_reconciled"))
        .expect("wb12_storage_reconciled should be present")
        .as_f64();
    assert!((storage_reconciled - EXPECTED_STORAGE_RECONCILED).abs() <= WB15_TEST_TOLERANCE);
}

#[test]
fn wb15_contract_conformance_rejects_missing_canopy_state_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb15_surface();
    surface.state_surface.remove(&BoundarySymbol::from("lai"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb15 missing-symbol failure should return typed report");

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
fn wb15_contract_conformance_rejects_non_finite_canopy_state_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb15_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("vdmt"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb15 non-finite failure should return typed report");

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
        "HKERNEL-WB14-RUNOFF-E-002"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn wb15_contract_conformance_rejects_out_of_domain_canopy_state_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb15_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(1.2));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb15 domain failure should return typed report");

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
