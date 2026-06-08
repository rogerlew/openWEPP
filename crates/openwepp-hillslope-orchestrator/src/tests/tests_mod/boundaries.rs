use super::fixtures::*;
use super::*;

#[test]
fn decomposition_boundary_missing_required_symbol_returns_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface.state_surface.remove(&BoundarySymbol::from(
        "pl_decomp_slot_0001_crop_0001_resmgt",
    ));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed decomposition guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(kernel.invocation_count, 2);
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-001"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn decomposition_boundary_invalid_ordering_flag_returns_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_order_decomp_before_soil"),
        BoundaryValue::scalar(0.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed decomposition guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(kernel.invocation_count, 2);
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-003"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn decomposition_boundary_rejects_negative_oratea_with_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_oratea"),
        BoundaryValue::scalar(-0.1),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed decomposition guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(kernel.invocation_count, 2);
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-010"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn pl12_contract_conformance_rejects_missing_perennial_cutday_payload() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface(2.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_mgtopt"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncut"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
        BoundaryValue::scalar(0.0),
    );
    for symbol in [
        "pl_decomp_slot_0001_crop_0001_gday_0001",
        "pl_decomp_slot_0001_crop_0001_gend_0001",
        "pl_decomp_slot_0001_crop_0001_animal_0001",
        "pl_decomp_slot_0001_crop_0001_bodywt_0001",
        "pl_decomp_slot_0001_crop_0001_area_0001",
        "pl_decomp_slot_0001_crop_0001_digest_0001",
    ] {
        surface.state_surface.remove(&BoundarySymbol::from(symbol));
    }

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing perennial cutday payload should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-007"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn pl12_contract_conformance_rejects_invalid_perennial_grazing_window() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface(2.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_mgtopt"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncut"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gday_0001"),
        BoundaryValue::scalar(220.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gend_0001"),
        BoundaryValue::scalar(200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_animal_0001"),
        BoundaryValue::scalar(20.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_bodywt_0001"),
        BoundaryValue::scalar(450.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_area_0001"),
        BoundaryValue::scalar(1200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_digest_0001"),
        BoundaryValue::scalar(0.62),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("invalid perennial grazing window should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-009"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn pl13_contract_conformance_rejects_missing_growth_state_surface() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface
        .state_surface
        .remove(&BoundarySymbol::from("sumgdd"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing growth transition state should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-001"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn pl13_contract_conformance_rejects_growth_state_domain_violation() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface
        .state_surface
        .insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(1.1));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("invalid growth transition state should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-007"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn growth_boundary_missing_required_symbol_returns_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface
        .state_surface
        .remove(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed growth guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(kernel.invocation_count, 4);
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-001"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn growth_boundary_non_finite_ordering_flag_returns_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_order_watbal_after_growth"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed growth guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(kernel.invocation_count, 4);
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-002"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}
