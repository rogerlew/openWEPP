use super::fixtures::*;
use super::*;

#[test]
fn execute_with_kernel_applies_writeback_updates() {
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
            let call_value = f64::from(self.call_index);
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                format!("HKERNEL-PHASE-OK-{}", self.call_index),
            )
            .expect("status should construct");
            let writeback = KernelWritebackPayload::with_updates(
                vec![WritebackField::bounded(
                    "soil_storage",
                    call_value,
                    Some(0.0),
                    Some(1000.0),
                )],
                vec![WritebackField::bounded(
                    "runoff_total",
                    call_value * 0.25,
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
    assert_eq!(
        report.scheduler_report.executed_phases(),
        Vec::from(HillslopePhaseGraph::canonical_order())
    );
    assert_eq!(
        report.phase_reports.len(),
        HillslopePhaseGraph::canonical_order().len()
    );
    assert!(report.phase_reports.iter().all(|phase| {
        phase.decision_outcome == WritebackDecisionOutcome::Apply && phase.apply_result.is_some()
    }));
    let phase_count =
        u32::try_from(HillslopePhaseGraph::canonical_order().len()).expect("phase count fits u32");
    let final_call_value = f64::from(phase_count);
    assert_eq!(
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from("soil_storage"))
            .copied(),
        Some(BoundaryValue::from(final_call_value))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("runoff_total"))
            .copied(),
        Some(BoundaryValue::from(final_call_value * 0.25))
    );
}

#[test]
fn execute_with_kernel_lends_stable_surface_references() {
    #[derive(Default)]
    struct PointerProbeKernel {
        call_index: u32,
        state_surface_ptrs: Vec<usize>,
        flux_surface_ptrs: Vec<usize>,
    }

    impl HillslopeKernel for PointerProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.call_index += 1;
            self.state_surface_ptrs
                .push(std::ptr::from_ref(request.state_surface) as usize);
            self.flux_surface_ptrs
                .push(std::ptr::from_ref(request.flux_surface) as usize);
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                format!("HKERNEL-PHASE-POINTER-{}", self.call_index),
            )
            .expect("status should construct");

            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = PointerProbeKernel::default();

    let report = scheduler
        .execute_with_kernel(
            &topology_report,
            &mut kernel,
            HillslopeWritebackSurface::default(),
        )
        .expect("kernel execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.state_surface_ptrs.len(),
        HillslopePhaseGraph::canonical_order().len()
    );
    assert_eq!(
        kernel.flux_surface_ptrs.len(),
        HillslopePhaseGraph::canonical_order().len()
    );
    assert!(
        kernel
            .state_surface_ptrs
            .windows(2)
            .all(|pair| pair[0] == pair[1]),
        "state surface reference should remain stable across phase calls"
    );
    assert!(
        kernel
            .flux_surface_ptrs
            .windows(2)
            .all(|pair| pair[0] == pair[1]),
        "flux surface reference should remain stable across phase calls"
    );
}

#[test]
fn execute_with_kernel_rejects_non_finite_writeback() {
    struct RejectKernel;

    impl HillslopeKernel for RejectKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-PHASE-OK-REJECT",
            )
            .expect("status should construct");
            let writeback = KernelWritebackPayload::with_updates(
                vec![WritebackField::unbounded("soil_storage", f64::NAN)],
                Vec::new(),
            );
            KernelRunResponse::new(status, writeback)
        }
    }

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = RejectKernel;

    let report = scheduler
        .execute_with_kernel(
            &topology_report,
            &mut kernel,
            HillslopeWritebackSurface::default(),
        )
        .expect("execution should return typed report");

    assert_eq!(
        report.scheduler_report.outcome_class,
        SchedulerOutcomeClass::PhaseFailure
    );
    assert_eq!(report.phase_reports.len(), 1);
    assert_eq!(
        report.phase_reports[0].decision_outcome,
        WritebackDecisionOutcome::Reject
    );
    assert_eq!(
        report.phase_reports[0].decision_status.message_id(),
        WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
    );
    assert!(
        !report
            .writeback_surface
            .state_surface
            .contains_key(&BoundarySymbol::from("soil_storage")),
        "rejected payload must not mutate orchestrator writeback state"
    );
}

#[test]
fn execute_with_kernel_rejects_kernel_phase_mismatch() {
    struct PhaseMismatchKernel;

    impl HillslopeKernel for PhaseMismatchKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::PreExecutionValidation,
                "HKERNEL-PHASE-INVALID",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = PhaseMismatchKernel;

    let report = scheduler
        .execute_with_kernel(
            &topology_report,
            &mut kernel,
            HillslopeWritebackSurface::default(),
        )
        .expect("execution should return typed report");

    assert_eq!(
        report.scheduler_report.outcome_class,
        SchedulerOutcomeClass::PhaseFailure
    );
    assert_eq!(
        report.scheduler_report.scheduler_status.boundary_class(),
        BoundaryClass::ModeMismatch
    );
    assert_eq!(report.phase_reports.len(), 1);
    assert_eq!(
        report.phase_reports[0].decision_outcome,
        WritebackDecisionOutcome::Reject
    );
}
