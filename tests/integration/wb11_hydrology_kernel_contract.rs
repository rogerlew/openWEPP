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

#[allow(clippy::too_many_lines)]
fn seeded_wb11_surface() -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    // Seed soil family sentinels used by consumer-boundary guards.
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.0));

    // WB11 kernel state inputs.
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

    // WB12 reconciliation inputs so canonical scheduler completion can proceed
    // beyond WB11 hydrology phases in the nominal success vector.
    state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(4.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(1.0e-6),
    );
    state_surface.insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(3.0));
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
        BoundarySymbol::from("intsty_0001"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0002"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0003"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(BoundarySymbol::from("timep"), BoundaryValue::scalar(0.25));
    state_surface.insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(10.75),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(1.0e-6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(4.0),
    );

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: std::collections::BTreeMap::new(),
    }
}

#[test]
fn wb11_contract_conformance_kernel_updates_et_perc_lateral_drain_surfaces() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_wb11_surface())
        .expect("wb11 execution should return typed report");

    assert!(report.scheduler_report.is_success());

    assert_eq!(
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from("wb11_soil_water"))
            .copied(),
        Some(BoundaryValue::scalar(9.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from("wb11_drainable_storage"))
            .copied(),
        Some(BoundaryValue::scalar(1.25))
    );

    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("ET"))
            .copied(),
        Some(BoundaryValue::scalar(2.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("Ws"))
            .copied(),
        Some(BoundaryValue::scalar(1.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("D"))
            .copied(),
        Some(BoundaryValue::scalar(1.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("Pe"))
            .copied(),
        Some(BoundaryValue::scalar(1.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("q"))
            .copied(),
        Some(BoundaryValue::scalar(0.75))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("Qdd"))
            .copied(),
        Some(BoundaryValue::scalar(1.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("Qd"))
            .copied(),
        Some(BoundaryValue::scalar(1.75))
    );
}

#[test]
fn wb11_contract_conformance_rejects_non_finite_et_demand() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb11_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb11 failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::Evapotranspiration)
    );

    let et_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::Evapotranspiration)
        .expect("evapotranspiration phase report should exist");
    assert_eq!(
        et_phase.decision_status.message_id(),
        "HKERNEL-WB11-ET-E-002"
    );
    assert_eq!(
        et_phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn wb11_contract_conformance_rejects_invalid_percolation_fraction() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb11_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(1.2),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb11 failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::PercolationDeepSeepage)
    );

    let perc_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::PercolationDeepSeepage)
        .expect("percolation phase report should exist");
    assert_eq!(
        perc_phase.decision_status.message_id(),
        "HKERNEL-WB11-PERC-E-003"
    );
    assert_eq!(
        perc_phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}
