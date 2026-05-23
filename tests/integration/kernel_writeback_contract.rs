use openwepp_hillslope_orchestrator::{HillslopePhaseScheduler, HillslopeWritebackSurface};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeKernel, HillslopeKernelRequest, KernelRunResponse,
    KernelWritebackPayload, WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID, WritebackDecisionOutcome,
    WritebackError, WritebackField, apply_kernel_writeback, evaluate_kernel_writeback,
};
use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, SimulationStatus};
use openwepp_topology::{
    ContributorTriplet, TopologyContributors, TopologyGraph, TopologyNode, TopologyNodeKey,
    TopologyNodeKind, parse_topology_fixture_str, validate_pre_execution_topology,
};
use openwepp_watershed_orchestrator::{
    WatershedWritebackSurface, execute_watershed_dispatch_with_kernel,
};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

#[test]
fn hillslope_writeback_success_applies_updates() {
    #[derive(Default)]
    struct NominalKernel {
        call_index: u32,
    }

    impl HillslopeKernel for NominalKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.call_index += 1;
            let status = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                format!("IT-HKERNEL-OK-{}", self.call_index),
            )
            .expect("status should construct");
            let writeback = KernelWritebackPayload::with_updates(
                vec![WritebackField::bounded(
                    "st",
                    f64::from(self.call_index),
                    Some(0.0),
                    Some(1000.0),
                )],
                vec![WritebackField::bounded(
                    "qout",
                    f64::from(self.call_index) * 0.1,
                    Some(0.0),
                    None,
                )],
            );
            KernelRunResponse::new(status, writeback)
        }
    }

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NominalKernel::default();

    let report = scheduler
        .execute_with_kernel(
            &topology_report,
            &mut kernel,
            HillslopeWritebackSurface::default(),
        )
        .expect("kernel execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(report.phase_reports.len(), 13);
    assert!(
        report
            .phase_reports
            .iter()
            .all(|phase| phase.decision_outcome == WritebackDecisionOutcome::Apply)
    );
    assert_eq!(
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from("st"))
            .copied(),
        Some(BoundaryValue::from(13.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("qout"))
            .copied(),
        Some(BoundaryValue::from(1.3))
    );
}

#[test]
fn watershed_writeback_reject_keeps_orchestrator_surface_unchanged() {
    struct RejectKernel;

    impl openwepp_kernel_contract::WatershedKernel for RejectKernel {
        fn run_watershed_node(
            &mut self,
            _request: &openwepp_kernel_contract::WatershedKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = SimulationStatus::ok(SimulationPhase::WatershedKernel, "IT-WKERNEL-OK")
                .expect("status should construct");
            let writeback = KernelWritebackPayload::with_updates(
                vec![WritebackField::unbounded("channel_storage", f64::NAN)],
                Vec::new(),
            );
            KernelRunResponse::new(status, writeback)
        }
    }

    let graph = TopologyGraph::new(
        1,
        1,
        0,
        vec![TopologyNode::new(
            TopologyNodeKey::new(TopologyNodeKind::Channel, 1),
            TopologyContributors::new(
                ContributorTriplet::new(1, 0, 0),
                ContributorTriplet::new(0, 0, 0),
                ContributorTriplet::new(0, 0, 0),
            ),
        )],
    );
    let topology_validation =
        validate_pre_execution_topology(&graph).expect("topology validation should construct");
    let mut kernel = RejectKernel;

    let report = execute_watershed_dispatch_with_kernel(
        &graph,
        &topology_validation,
        &mut kernel,
        WatershedWritebackSurface::default(),
    )
    .expect("execution should return typed report");

    assert!(!report.dispatch_report.is_success());
    assert_eq!(report.step_reports.len(), 1);
    assert_eq!(
        report.step_reports[0].decision_status.message_id(),
        WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
    );
    assert_eq!(
        report.dispatch_report.dispatch_status.message_id(),
        WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
    );
    assert!(report.writeback_surface.state_surface.is_empty());
}

#[test]
fn apply_reject_path_propagates_typed_error() {
    let payload = KernelWritebackPayload::with_updates(
        vec![WritebackField::unbounded("soil_storage", f64::INFINITY)],
        Vec::new(),
    );

    let decision =
        evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload).expect("decision ok");
    assert_eq!(decision.outcome, WritebackDecisionOutcome::Reject);

    let mut state_surface = std::collections::BTreeMap::new();
    let mut flux_surface = std::collections::BTreeMap::new();

    let error = apply_kernel_writeback(
        SimulationPhase::HillslopeKernel,
        &decision,
        &payload,
        &mut state_surface,
        &mut flux_surface,
    )
    .expect_err("reject decision must not apply");

    match error {
        WritebackError::DecisionNotAccept { outcome } => {
            assert_eq!(outcome, WritebackDecisionOutcome::Reject);
        }
        WritebackError::Status(_) => panic!("unexpected status construction error"),
    }
    assert!(state_surface.is_empty());
    assert!(flux_surface.is_empty());
}

#[test]
fn reject_status_has_failure_domain_signal() {
    let payload = KernelWritebackPayload::with_updates(
        vec![WritebackField::bounded("st", -1.0, Some(0.0), None)],
        Vec::new(),
    );

    let decision =
        evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload).expect("decision ok");

    assert_eq!(decision.outcome, WritebackDecisionOutcome::Reject);
    assert_eq!(
        decision.status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}
