use super::fixtures::*;
use super::*;

#[test]
fn canonical_graph_order_is_deterministic() {
    let graph = HillslopePhaseGraph::canonical();
    let order = graph
        .topological_order()
        .expect("canonical graph should always topologically sort");

    assert_eq!(
        order,
        Vec::from(HillslopePhaseGraph::canonical_order()),
        "ARCH05 requires explicit deterministic scheduler order"
    );
    assert_eq!(graph.dependency_edges().len(), 13);
}

#[test]
fn topology_precondition_failure_blocks_phase_execution() {
    let graph = parse_topology_fixture_str(INVALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    assert_eq!(
        topology_report.status.classification(),
        StatusClassification::Failure
    );

    let scheduler = HillslopePhaseScheduler::canonical();
    let call_count = Cell::new(0_usize);

    let report = scheduler
        .execute_with(&topology_report, |_| {
            call_count.set(call_count.get() + 1);
            HillslopePhaseScheduler::nominal_phase_status(HillslopePhase::Normalization)
                .expect("nominal status should build")
        })
        .expect("scheduler should not error");

    assert_eq!(call_count.get(), 0);
    assert_eq!(
        report.outcome_class,
        SchedulerOutcomeClass::TopologyPreconditionFailed
    );
    assert_eq!(
        report.scheduler_status.classification(),
        StatusClassification::Failure
    );
    assert_eq!(
        report.scheduler_status.boundary_class(),
        BoundaryClass::TopologyInvalid
    );
}

#[test]
fn phase_failure_is_typed_and_fail_fast() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();

    let report = scheduler
        .execute_with(&topology_report, |phase| {
            if phase == HillslopePhase::PercolationDeepSeepage {
                return openwepp_sim_contract::status::SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::DomainViolation,
                    "HSCHED-PHASE-E-004",
                )
                .expect("failure status should build");
            }

            HillslopePhaseScheduler::nominal_phase_status(phase)
                .expect("nominal status should build")
        })
        .expect("scheduler should not error");

    assert_eq!(report.outcome_class, SchedulerOutcomeClass::PhaseFailure);
    assert_eq!(
        report.scheduler_status.classification(),
        StatusClassification::Failure
    );
    assert_eq!(
        report.scheduler_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
    assert_eq!(
        report.executed_phases(),
        vec![
            HillslopePhase::Normalization,
            HillslopePhase::StorageBounds,
            HillslopePhase::DecompositionTransition,
            HillslopePhase::ResiduePartitionTransition,
            HillslopePhase::AnnualGrowthTransition,
            HillslopePhase::PerennialGrowthTransition,
            HillslopePhase::PercolationDeepSeepage,
        ]
    );
    assert_eq!(
        report.halted_phase,
        Some(HillslopePhase::PercolationDeepSeepage)
    );
}

#[test]
fn phase_status_phase_mismatch_returns_mode_mismatch_failure() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();

    let report = scheduler
        .execute_with(&topology_report, |_| {
            openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::PreExecutionValidation,
                "HSCHED-PHASE-INVALID-STATUS",
            )
            .expect("status should build")
        })
        .expect("scheduler should not error");

    assert_eq!(
        report.outcome_class,
        SchedulerOutcomeClass::SchedulerInvariantFailure
    );
    assert_eq!(
        report.scheduler_status.classification(),
        StatusClassification::Failure
    );
    assert_eq!(
        report.scheduler_status.boundary_class(),
        BoundaryClass::ModeMismatch
    );
    assert_eq!(report.halted_phase, Some(HillslopePhase::Normalization));
}

#[test]
fn nominal_execution_completes_in_canonical_order() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();

    let report = scheduler
        .execute_with(&topology_report, |phase| {
            HillslopePhaseScheduler::nominal_phase_status(phase)
                .expect("nominal status should build")
        })
        .expect("scheduler should not error");

    assert!(report.is_success());
    assert_eq!(report.outcome_class, SchedulerOutcomeClass::Completed);
    assert_eq!(report.halted_phase, None);
    assert_eq!(
        report.executed_phases(),
        Vec::from(HillslopePhaseGraph::canonical_order())
    );
    assert_eq!(
        report.scheduler_status.phase(),
        SimulationPhase::HillslopeKernel
    );
    assert_eq!(
        report.scheduler_status.classification(),
        StatusClassification::Nominal
    );
}

#[test]
fn consumer_adapter_mapping_matches_phase_contract() {
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::Normalization),
        HillslopeConsumerAdapter::Soil
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::StorageBounds),
        HillslopeConsumerAdapter::Soil
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::DecompositionTransition),
        HillslopeConsumerAdapter::Decomposition
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::ResiduePartitionTransition),
        HillslopeConsumerAdapter::Decomposition
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::AnnualGrowthTransition),
        HillslopeConsumerAdapter::Growth
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::PerennialGrowthTransition),
        HillslopeConsumerAdapter::Growth
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::Evapotranspiration),
        HillslopeConsumerAdapter::Watbal
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::PercolationDeepSeepage),
        HillslopeConsumerAdapter::Perc
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::LateralTransfer),
        HillslopeConsumerAdapter::Watbal
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::PlantRootUptake),
        HillslopeConsumerAdapter::Watbal
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::Drainage),
        HillslopeConsumerAdapter::Perc
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::RunoffReconciliation),
        HillslopeConsumerAdapter::Runoff
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::StorageReconciliation),
        HillslopeConsumerAdapter::Watbal
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::ClosureDiagnostics),
        HillslopeConsumerAdapter::Watbal
    );
}

#[test]
fn wb10_contract_conformance_hydrology_phase_classes_are_not_generic() {
    #[derive(Default)]
    struct ProbeKernel {
        observed_phase_classes: BTreeMap<String, String>,
    }

    impl HillslopeKernel for ProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if matches!(
                request.phase_name,
                "evapotranspiration"
                    | "percolation_deep_seepage"
                    | "lateral_transfer"
                    | "drainage"
                    | "plant_root_uptake"
                    | "runoff_reconciliation"
                    | "storage_reconciliation"
                    | "closure_diagnostics"
            ) {
                self.observed_phase_classes.insert(
                    request.phase_name.to_owned(),
                    request.phase_class.as_str().to_owned(),
                );
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-WB10-PHASE-CLASS",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_growth_runtime_surface(1.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb10 phase-class conformance probe should execute");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.observed_phase_classes.get("evapotranspiration"),
        Some(&"hydrology_evapotranspiration".to_owned())
    );
    assert_eq!(
        kernel
            .observed_phase_classes
            .get("percolation_deep_seepage"),
        Some(&"hydrology_percolation_deep_seepage".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("lateral_transfer"),
        Some(&"hydrology_lateral_transfer".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("drainage"),
        Some(&"hydrology_drainage".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("plant_root_uptake"),
        Some(&"hydrology_plant_root_uptake".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("runoff_reconciliation"),
        Some(&"hydrology_runoff_reconciliation".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("storage_reconciliation"),
        Some(&"hydrology_storage_reconciliation".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("closure_diagnostics"),
        Some(&"hydrology_peak_runoff".to_owned())
    );
}

#[test]
fn wb10_contract_conformance_rejects_unsupported_hydrology_phase_class() {
    let error = super::hydrology_phase_dispatch_for_phase(
        HillslopePhase::Evapotranspiration,
        HillslopeKernelPhaseClass::Hydrology,
    )
    .expect_err("evapotranspiration must not accept generic hydrology class");

    assert_eq!(error.code(), "HS-HYDRO-E-001");
    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn required_consumer_symbols_are_empty_without_slope_or_soil_families() {
    let empty_surface = BTreeMap::new();

    for phase in HillslopePhaseGraph::canonical_order() {
        let required = required_hillslope_consumer_state_symbols(phase, &empty_surface);
        assert!(
            required.is_empty(),
            "phase {} should not require slope/soil symbols when neither family is seeded",
            phase.as_str()
        );
        validate_hillslope_consumer_boundary(phase, &empty_surface)
            .expect("empty non-slope/non-soil surface should not trigger consumer guard");
    }
}

#[test]
fn consumer_boundary_reports_typed_missing_symbol_for_seeded_family() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.25));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.31));
    state_surface.insert(
        BoundarySymbol::from("ssc"),
        BoundaryValue::scalar(0.000_004),
    );

    let error = validate_hillslope_consumer_boundary(HillslopePhase::Normalization, &state_surface)
        .expect_err("missing thetdr must fail with typed consumer boundary error");
    assert_eq!(error.code(), "HS-CONSUMER-E-001");
    assert!(matches!(
        error,
        super::HillslopeConsumerBoundaryError::MissingRequiredStateSymbol {
            phase: HillslopePhase::Normalization,
            adapter: HillslopeConsumerAdapter::Soil,
            symbol,
        } if symbol.as_str() == "thetdr"
    ));
}

#[test]
fn annual_growth_phase_emits_typed_growth_context() {
    #[derive(Default)]
    struct ProbeKernel {
        decomp: usize,
        annual: usize,
        perennial: usize,
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
                    assert_eq!(
                        context.management_class,
                        HillslopeDecompositionManagementClass::AnnualOrFallow
                    );
                    let transition_payload = context
                        .transition_payload
                        .expect("decomposition context should carry transition payload");
                    assert!(matches!(
                        transition_payload.control,
                        HillslopeDecompositionTransitionControl::Annual(
                            HillslopeAnnualDecompositionControl {
                                active_action: HillslopeAnnualDecompositionAction::Herbicide,
                                ..
                            }
                        )
                    ));
                    assert!(request.growth_context.is_none());
                    self.decomp += 1;
                }
                HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                    let context = request
                        .growth_context
                        .expect("annual growth phase should carry growth context");
                    assert_eq!(
                        context.management_class,
                        HillslopeGrowthManagementClass::AnnualOrFallow
                    );
                    let transition_payload = context
                        .transition_payload
                        .expect("annual growth context should carry transition payload");
                    assert!(matches!(
                        transition_payload.control,
                        HillslopeGrowthTransitionControl::Annual(HillslopeAnnualGrowthControl {
                            active_action: HillslopeAnnualGrowthAction::None,
                            ..
                        })
                    ));
                    self.annual += 1;
                }
                HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                    assert!(
                        request.growth_context.is_none(),
                        "perennial phase should skip context when annual branch is active"
                    );
                    self.perennial += 1;
                }
                phase_class if phase_class.is_hydrology_phase() => {
                    assert!(request.growth_context.is_none());
                    assert!(request.decomposition_context.is_none());
                }
                _ => unreachable!("unexpected phase class for annual growth test"),
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GROWTH-CONTEXT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_growth_runtime_surface(1.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("annual growth context execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(kernel.decomp, 2);
    assert_eq!(kernel.annual, 1);
    assert_eq!(kernel.perennial, 1);
}

#[test]
fn perennial_growth_phase_emits_typed_growth_context() {
    #[derive(Default)]
    struct ProbeKernel {
        decomp: usize,
        annual: usize,
        perennial: usize,
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
                    assert_eq!(
                        context.management_class,
                        HillslopeDecompositionManagementClass::Perennial
                    );
                    let transition_payload = context
                        .transition_payload
                        .expect("decomposition context should carry transition payload");
                    assert!(matches!(
                        transition_payload.control,
                        HillslopeDecompositionTransitionControl::Perennial(
                            HillslopePerennialDecompositionControl {
                                active_action: HillslopePerennialDecompositionAction::Grazing {
                                    cycle_index: 1
                                },
                                ..
                            }
                        )
                    ));
                    assert!(request.growth_context.is_none());
                    self.decomp += 1;
                }
                HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                    assert!(
                        request.growth_context.is_none(),
                        "annual phase should skip context when perennial branch is active"
                    );
                    self.annual += 1;
                }
                HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                    let context = request
                        .growth_context
                        .expect("perennial growth phase should carry growth context");
                    assert_eq!(
                        context.management_class,
                        HillslopeGrowthManagementClass::Perennial
                    );
                    let transition_payload = context
                        .transition_payload
                        .expect("perennial growth context should carry transition payload");
                    assert!(matches!(
                        transition_payload.control,
                        HillslopeGrowthTransitionControl::Perennial(
                            HillslopePerennialGrowthControl {
                                active_action: HillslopePerennialGrowthAction::None,
                                ..
                            }
                        )
                    ));
                    self.perennial += 1;
                }
                phase_class if phase_class.is_hydrology_phase() => {
                    assert!(request.growth_context.is_none());
                    assert!(request.decomposition_context.is_none());
                }
                _ => unreachable!("unexpected phase class for perennial growth test"),
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GROWTH-CONTEXT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_growth_runtime_surface(2.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("perennial growth context execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(kernel.decomp, 2);
    assert_eq!(kernel.annual, 1);
    assert_eq!(kernel.perennial, 1);
}

#[test]
fn pl16_annual_growth_accepts_zero_gddmax_sentinel_for_summer_branch() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GDDMAX-SUMMER",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 200.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_gddmax"),
        BoundaryValue::scalar(0.0),
    );
    seed_legacy_monthly_temperature_vectors(&mut surface);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("annual gddmax sentinel branch should execute");

    assert!(report.scheduler_report.is_success());
}

#[test]
fn pl16_annual_growth_accepts_zero_gddmax_sentinel_for_winter_branch() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GDDMAX-WINTER",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 20.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(300.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(100.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_gddmax"),
        BoundaryValue::scalar(0.0),
    );
    seed_legacy_monthly_temperature_vectors(&mut surface);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("winter annual gddmax sentinel branch should execute");

    assert!(report.scheduler_report.is_success());
}

#[test]
fn pl16_perennial_growth_accepts_zero_gddmax_sentinel() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GDDMAX-PERENNIAL",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(2.0, 200.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_gddmax"),
        BoundaryValue::scalar(0.0),
    );
    seed_legacy_monthly_temperature_vectors(&mut surface);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("perennial gddmax sentinel branch should execute");

    assert!(report.scheduler_report.is_success());
}

#[test]
fn pl16_gddmax_sentinel_requires_monthly_temperature_vectors() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GDDMAX-MISSING-MONTHLY",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 200.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_gddmax"),
        BoundaryValue::scalar(0.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing monthly temperature vectors should return typed failure report");

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
