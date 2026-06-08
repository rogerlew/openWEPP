use super::fixtures::*;
use super::*;

#[test]
fn active_slot_resolution_uses_year_three_perennial_slot() {
    #[derive(Default)]
    struct ProbeKernel {
        saw_decomp_perennial: bool,
        saw_annual_context: bool,
        saw_perennial_context: bool,
    }

    impl HillslopeKernel for ProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            match request.phase_class {
                HillslopeKernelPhaseClass::DecompositionTransition
                | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                    let context = request
                        .decomposition_context
                        .expect("decomposition phases should carry decomposition context");
                    self.saw_decomp_perennial = context.management_class
                        == HillslopeDecompositionManagementClass::Perennial;
                }
                HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                    self.saw_annual_context = request.growth_context.is_some();
                }
                HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                    self.saw_perennial_context = request.growth_context.is_some();
                }
                phase_class if phase_class.is_hydrology_phase() => {}
                _ => unreachable!("unexpected phase class for active-slot perennial test"),
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-ACTIVE-SLOT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_multislot_rotation_surface(3.0, 200.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("year-three slot resolution should succeed");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_decomp_perennial);
    assert!(!kernel.saw_annual_context);
    assert!(kernel.saw_perennial_context);
}

#[test]
fn active_slot_resolution_wraps_rotation_boundary_to_year_one() {
    #[derive(Default)]
    struct ProbeKernel {
        saw_decomp_annual: bool,
        saw_annual_context: bool,
        saw_perennial_context: bool,
    }

    impl HillslopeKernel for ProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            match request.phase_class {
                HillslopeKernelPhaseClass::DecompositionTransition
                | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                    let context = request
                        .decomposition_context
                        .expect("decomposition phases should carry decomposition context");
                    self.saw_decomp_annual = context.management_class
                        == HillslopeDecompositionManagementClass::AnnualOrFallow;
                }
                HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                    self.saw_annual_context = request.growth_context.is_some();
                }
                HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                    self.saw_perennial_context = request.growth_context.is_some();
                }
                phase_class if phase_class.is_hydrology_phase() => {}
                _ => unreachable!("unexpected phase class for active-slot annual test"),
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-ACTIVE-SLOT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_multislot_rotation_surface(4.0, 200.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("rotation-boundary slot resolution should succeed");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_decomp_annual);
    assert!(kernel.saw_annual_context);
    assert!(!kernel.saw_perennial_context);
}

#[test]
fn active_slot_resolution_rejects_ambiguous_slot_candidates() {
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
    let mut surface = seeded_multislot_rotation_surface(1.0, 200.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0002_year_in_rotation"),
        BoundaryValue::scalar(1.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("ambiguous slot candidate must return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-PLDISP-E-006"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn active_slot_resolution_rejects_missing_active_crop_for_day() {
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
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 30.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_0002_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(120.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(150.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdplt"),
        BoundaryValue::scalar(200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdharv"),
        BoundaryValue::scalar(240.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_rw"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0002_resmgt"),
        BoundaryValue::scalar(6.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing active crop must return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-PLDISP-E-008"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn hphys0250_zero_date_perennial_slot_remains_active_for_growth_dispatch() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-ZERO-DATE-PERENNIAL",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(2.0, 1.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdstop"),
        BoundaryValue::scalar(0.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("zero-date perennial slot should dispatch under baseline ptgrp semantics");

    assert!(
        report.scheduler_report.is_success(),
        "zero-date perennial dispatch should not fail active crop resolution: {:?}",
        report.scheduler_report.scheduler_status
    );
}

#[test]
fn active_slot_resolution_rejects_ambiguous_active_crops_for_day() {
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
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 210.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_0002_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(180.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(300.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdplt"),
        BoundaryValue::scalar(200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdharv"),
        BoundaryValue::scalar(240.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_rw"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0002_resmgt"),
        BoundaryValue::scalar(6.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("ambiguous active crop must return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-PLDISP-E-009"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}
