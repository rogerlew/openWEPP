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

#[test]
fn mofe01_me2_sequential_executor_carries_first_ofe_arrays_to_second_lane() {
    #[derive(Default)]
    struct TransferProbeKernel {
        normalization_call_count: usize,
        observed_inputs: Vec<(f64, f64, f64, f64)>,
    }

    impl HillslopeKernel for TransferProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_name == "normalization" {
                self.normalization_call_count += 1;
                self.observed_inputs.push((
                    request_state_scalar(request, "UpStrmQ"),
                    request_state_scalar(request, "SubRIn"),
                    request_state_scalar(request, "ui_SUrunf_0001"),
                    request_state_scalar(request, "ui_LfUrf_0004"),
                ));
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-MOFE-ME2-TRANSFER",
            )
            .expect("status should construct");

            if request.phase_name != "closure_diagnostics" {
                return KernelRunResponse::new(status, KernelWritebackPayload::empty());
            }

            let state_updates = match self.normalization_call_count {
                1 => transfer_current_state_updates(&[(1, 0.25), (2, 0.50)], &[(4, 0.75)]),
                2 => transfer_current_state_updates(&[], &[]),
                lane => panic!("unexpected lane count {lane}"),
            };
            KernelRunResponse::new(
                status,
                KernelWritebackPayload::with_updates(state_updates, Vec::new()),
            )
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = TransferProbeKernel::default();
    let lane_inputs = vec![
        OfeLaneExecutionInput::new(1, stale_current_transfer_surface()),
        OfeLaneExecutionInput::new(2, stale_downstream_transfer_surface()),
    ];

    let report = scheduler
        .execute_ofe_sequence_with_kernel(&topology_report, &mut kernel, lane_inputs)
        .expect("two-OFE transfer sequence should execute");

    assert_eq!(report.lane_count(), 2);
    assert_eq!(
        kernel.observed_inputs,
        vec![(0.0, 0.0, 0.0, 0.0), (0.75, 0.75, 0.25, 0.75)],
        "OFE 2 must receive only the explicit OFE 1 transfer arrays"
    );
    assert_eq!(report.lane_reports[0].ofe_id, 1);
    assert_eq!(
        report.lane_reports[0]
            .current_transfer_output
            .recipient_ofe_id,
        Some(2)
    );
    assert!((report.lane_reports[0].current_transfer_output.qofe - 0.75).abs() < 1.0e-12);
    assert!(
        (report.lane_reports[0]
            .current_transfer_output
            .lateral_export
            - 0.75)
            .abs()
            < 1.0e-12
    );
    assert_eq!(
        report.lane_reports[1].upstream_transfer_input.source_ofe_id,
        Some(1)
    );
    assert!((report.lane_reports[1].upstream_transfer_input.upstrmq - 0.75).abs() < 1.0e-12);
    assert!((report.lane_reports[1].upstream_transfer_input.subrin - 0.75).abs() < 1.0e-12);
    assert_eq!(
        report.lane_reports[1]
            .current_transfer_output
            .recipient_ofe_id,
        None
    );
}

#[test]
fn mofe01_me2_sequential_executor_applies_downstream_area_ratio() {
    #[derive(Default)]
    struct ScalingProbeKernel {
        normalization_call_count: usize,
        observed_inputs: Vec<(f64, f64)>,
    }

    impl HillslopeKernel for ScalingProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_name == "normalization" {
                self.normalization_call_count += 1;
                self.observed_inputs.push((
                    request_state_scalar(request, "UpStrmQ"),
                    request_state_scalar(request, "SubRIn"),
                ));
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-MOFE-ME2-SCALING",
            )
            .expect("status should construct");

            if request.phase_name != "closure_diagnostics" {
                return KernelRunResponse::new(status, KernelWritebackPayload::empty());
            }

            let state_updates = match self.normalization_call_count {
                1 => transfer_current_state_updates(&[(1, 0.25)], &[(1, 0.50)]),
                2 => transfer_current_state_updates(&[], &[]),
                lane => panic!("unexpected lane count {lane}"),
            };
            KernelRunResponse::new(
                status,
                KernelWritebackPayload::with_updates(state_updates, Vec::new()),
            )
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ScalingProbeKernel::default();
    let lane_inputs = vec![
        OfeLaneExecutionInput::new(1, HillslopeWritebackSurface::default()),
        OfeLaneExecutionInput::with_upstream_area_ratio(
            2,
            2.0,
            HillslopeWritebackSurface::default(),
        ),
    ];

    let report = scheduler
        .execute_ofe_sequence_with_kernel(&topology_report, &mut kernel, lane_inputs)
        .expect("two-OFE transfer sequence should execute with area scaling");

    assert_eq!(kernel.observed_inputs, vec![(0.0, 0.0), (0.50, 1.0)]);
    assert!((report.lane_reports[1].upstream_transfer_input.area_ratio - 2.0).abs() < 1.0e-12);
    assert!((report.lane_reports[1].upstream_transfer_input.upstrmq - 0.50).abs() < 1.0e-12);
    assert!((report.lane_reports[1].upstream_transfer_input.subrin - 1.0).abs() < 1.0e-12);
}

#[test]
fn mofe01_me2_sequential_executor_rejects_stale_current_output_arrays() {
    struct NoCurrentOutputKernel;

    impl HillslopeKernel for NoCurrentOutputKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-MOFE-ME2-NO-CURRENT-OUTPUT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoCurrentOutputKernel;
    let lane_inputs = vec![
        OfeLaneExecutionInput::new(1, stale_current_transfer_surface()),
        OfeLaneExecutionInput::new(2, HillslopeWritebackSurface::default()),
    ];

    let error = scheduler
        .execute_ofe_sequence_with_kernel(&topology_report, &mut kernel, lane_inputs)
        .expect_err("stale current output arrays must be cleared before extraction");

    assert!(matches!(
        error,
        OfeLaneSequenceError::InvalidTransferValue {
            ofe_id: 1,
            hour: Some(1),
            value,
            ..
        } if value.is_nan()
    ));
}

#[test]
fn mofe01_me2_sequential_executor_rejects_malformed_transfer_arrays() {
    struct MalformedTransferKernel;

    impl HillslopeKernel for MalformedTransferKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-MOFE-ME2-MALFORMED",
            )
            .expect("status should construct");

            if request.phase_name != "closure_diagnostics" {
                return KernelRunResponse::new(status, KernelWritebackPayload::empty());
            }

            let mut state_updates = transfer_current_state_updates(&[], &[]);
            state_updates.push(WritebackField::unbounded("ui_SCrunf_0002", -0.10));
            KernelRunResponse::new(
                status,
                KernelWritebackPayload::with_updates(state_updates, Vec::new()),
            )
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = MalformedTransferKernel;
    let lane_inputs = vec![
        OfeLaneExecutionInput::new(1, HillslopeWritebackSurface::default()),
        OfeLaneExecutionInput::new(2, HillslopeWritebackSurface::default()),
    ];

    let error = scheduler
        .execute_ofe_sequence_with_kernel(&topology_report, &mut kernel, lane_inputs)
        .expect_err("negative transfer array slot must fail closed");

    assert!(matches!(
        error,
        OfeLaneSequenceError::InvalidTransferValue {
            ofe_id: 1,
            hour: Some(2),
            value,
            ..
        } if (value + 0.10).abs() < 1.0e-12
    ));
}

#[test]
fn mofe01_me2_sequential_executor_rejects_transfer_total_overflow() {
    struct OverflowTransferKernel;

    impl HillslopeKernel for OverflowTransferKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-MOFE-ME2-OVERFLOW",
            )
            .expect("status should construct");

            if request.phase_name != "closure_diagnostics" {
                return KernelRunResponse::new(status, KernelWritebackPayload::empty());
            }

            KernelRunResponse::new(
                status,
                KernelWritebackPayload::with_updates(
                    transfer_current_state_updates_uniform(f64::MAX, 0.0),
                    Vec::new(),
                ),
            )
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = OverflowTransferKernel;
    let lane_inputs = vec![
        OfeLaneExecutionInput::new(1, HillslopeWritebackSurface::default()),
        OfeLaneExecutionInput::new(2, HillslopeWritebackSurface::default()),
    ];

    let error = scheduler
        .execute_ofe_sequence_with_kernel(&topology_report, &mut kernel, lane_inputs)
        .expect_err("overflowed transfer totals must fail closed");

    assert!(matches!(
        error,
        OfeLaneSequenceError::InvalidTransferValue {
            ofe_id: 1,
            hour: None,
            value,
            ..
        } if value.is_infinite()
    ));
}

#[test]
fn mofe01_me2_sequential_executor_rejects_nonsequential_lane_ids() {
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-MOFE-ME2-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let lane_inputs = vec![OfeLaneExecutionInput::new(
        2,
        HillslopeWritebackSurface::default(),
    )];

    let error = scheduler
        .execute_ofe_sequence_with_kernel(&topology_report, &mut kernel, lane_inputs)
        .expect_err("sequence must start at OFE 1");

    assert!(matches!(
        error,
        OfeLaneSequenceError::NonSequentialLaneOfeId {
            expected_ofe_id: 1,
            observed_ofe_id: 2
        }
    ));
}

#[test]
fn mofe01_me3_persistent_sequence_carries_lane_state_across_days_without_bleed() {
    #[derive(Default)]
    struct PersistentProbeKernel {
        normalization_call_count: usize,
        current_lane: usize,
        observed_markers: Vec<(usize, usize, f64)>,
    }

    impl HillslopeKernel for PersistentProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-MOFE-ME3-PERSIST",
            )
            .expect("status should construct");

            if request.phase_name == "normalization" {
                self.normalization_call_count += 1;
                self.current_lane = ((self.normalization_call_count - 1) % 2) + 1;
                let current_day = ((self.normalization_call_count - 1) / 2) + 1;
                self.observed_markers.push((
                    current_day,
                    self.current_lane,
                    request_state_scalar(request, "me3_storage_marker"),
                ));
            }

            if request.phase_name != "closure_diagnostics" {
                return KernelRunResponse::new(status, KernelWritebackPayload::empty());
            }

            let marker = request_state_scalar(request, "me3_storage_marker");
            let lane_increment = match self.current_lane {
                1 => 1.0,
                2 => 2.0,
                _ => 0.0,
            };
            let mut state_updates = vec![WritebackField::unbounded(
                "me3_storage_marker",
                marker + lane_increment,
            )];
            state_updates.extend(transfer_current_state_updates(&[], &[]));

            KernelRunResponse::new(
                status,
                KernelWritebackPayload::with_updates(state_updates, Vec::new()),
            )
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = PersistentProbeKernel::default();
    let mut lane_state = OfeLanePersistentStateSequence::new(vec![
        OfeLanePersistentState::new(1, marker_surface(10.0)),
        OfeLanePersistentState::new(2, marker_surface(100.0)),
    ])
    .expect("persistent state should construct");

    let first_day = scheduler
        .execute_persistent_ofe_sequence_day_with_kernel(
            &topology_report,
            &mut kernel,
            &mut lane_state,
        )
        .expect("first persistent day should execute");
    let second_day = scheduler
        .execute_persistent_ofe_sequence_day_with_kernel(
            &topology_report,
            &mut kernel,
            &mut lane_state,
        )
        .expect("second persistent day should execute");

    assert_eq!(first_day.lane_count(), 2);
    assert_eq!(second_day.lane_count(), 2);
    assert_eq!(
        kernel.observed_markers,
        vec![(1, 1, 10.0), (1, 2, 100.0), (2, 1, 11.0), (2, 2, 102.0)]
    );
    assert!((persistent_lane_marker(&lane_state, 1) - 12.0).abs() < 1.0e-12);
    assert!((persistent_lane_marker(&lane_state, 2) - 104.0).abs() < 1.0e-12);
}

#[test]
fn mofe01_me3_persistent_sequence_keeps_prior_state_when_day_fails() {
    #[derive(Default)]
    struct SecondDayFailureKernel {
        normalization_call_count: usize,
        current_lane: usize,
    }

    impl HillslopeKernel for SecondDayFailureKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-MOFE-ME3-FAIL-CLOSED",
            )
            .expect("status should construct");

            if request.phase_name == "normalization" {
                self.normalization_call_count += 1;
                self.current_lane = ((self.normalization_call_count - 1) % 2) + 1;
            }

            if request.phase_name != "closure_diagnostics" {
                return KernelRunResponse::new(status, KernelWritebackPayload::empty());
            }

            let marker = request_state_scalar(request, "me3_storage_marker");
            let mut state_updates = vec![WritebackField::unbounded(
                "me3_storage_marker",
                marker + 1.0,
            )];
            if self.normalization_call_count <= 2 {
                state_updates.extend(transfer_current_state_updates(&[], &[]));
            }

            KernelRunResponse::new(
                status,
                KernelWritebackPayload::with_updates(state_updates, Vec::new()),
            )
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = SecondDayFailureKernel::default();
    let mut lane_state = OfeLanePersistentStateSequence::new(vec![
        OfeLanePersistentState::new(1, marker_surface(1.0)),
        OfeLanePersistentState::new(2, marker_surface(2.0)),
    ])
    .expect("persistent state should construct");

    scheduler
        .execute_persistent_ofe_sequence_day_with_kernel(
            &topology_report,
            &mut kernel,
            &mut lane_state,
        )
        .expect("first persistent day should execute");
    let day_one_lane_one_marker = persistent_lane_marker(&lane_state, 1);
    let day_one_lane_two_marker = persistent_lane_marker(&lane_state, 2);

    let error = scheduler
        .execute_persistent_ofe_sequence_day_with_kernel(
            &topology_report,
            &mut kernel,
            &mut lane_state,
        )
        .expect_err("missing transfer arrays on second day should fail closed");

    assert!(matches!(
        error,
        OfeLaneSequenceError::InvalidTransferValue {
            ofe_id: 1,
            hour: Some(1),
            value,
            ..
        } if value.is_nan()
    ));
    assert!((day_one_lane_one_marker - 2.0).abs() < 1.0e-12);
    assert!((day_one_lane_two_marker - 3.0).abs() < 1.0e-12);
    assert!((persistent_lane_marker(&lane_state, 1) - day_one_lane_one_marker).abs() < 1.0e-12);
    assert!((persistent_lane_marker(&lane_state, 2) - day_one_lane_two_marker).abs() < 1.0e-12);
}

#[test]
fn mofe01_me3_persistent_sequence_rejects_nonsequential_initial_state() {
    let error = OfeLanePersistentStateSequence::new(vec![OfeLanePersistentState::new(
        2,
        HillslopeWritebackSurface::default(),
    )])
    .expect_err("persistent sequence must start at OFE 1");

    assert!(matches!(
        error,
        OfeLaneSequenceError::NonSequentialLaneOfeId {
            expected_ofe_id: 1,
            observed_ofe_id: 2
        }
    ));
}

fn marker_surface(marker: f64) -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface::default();
    surface.state_surface.insert(
        BoundarySymbol::from("me3_storage_marker"),
        BoundaryValue::scalar(marker),
    );
    surface
}

fn persistent_lane_marker(lane_state: &OfeLanePersistentStateSequence, ofe_id: usize) -> f64 {
    lane_state
        .lane_surface(ofe_id)
        .and_then(|surface| {
            surface
                .state_surface
                .get(&BoundarySymbol::from("me3_storage_marker"))
        })
        .copied()
        .map_or(0.0, BoundaryValue::as_f64)
}

fn stale_current_transfer_surface() -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface::default();
    for hour in 1..=24 {
        surface.state_surface.insert(
            BoundarySymbol::from(hourly_symbol("ui_SCrunf", hour)),
            BoundaryValue::scalar(999.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(hourly_symbol("ui_LfCrf", hour)),
            BoundaryValue::scalar(999.0),
        );
    }
    surface
}

fn request_state_scalar(request: &HillslopeKernelRequest<'_>, symbol: &str) -> f64 {
    request
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .copied()
        .map_or(0.0, BoundaryValue::as_f64)
}

fn transfer_current_state_updates(
    surface_values: &[(usize, f64)],
    lateral_values: &[(usize, f64)],
) -> Vec<WritebackField> {
    let mut updates = Vec::new();
    for hour in 1..=24 {
        updates.push(WritebackField::bounded(
            hourly_symbol("ui_SCrunf", hour),
            hourly_value(surface_values, hour),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            hourly_symbol("ui_LfCrf", hour),
            hourly_value(lateral_values, hour),
            Some(0.0),
            None,
        ));
    }
    updates
}

fn transfer_current_state_updates_uniform(
    surface_value: f64,
    lateral_value: f64,
) -> Vec<WritebackField> {
    let mut updates = Vec::new();
    for hour in 1..=24 {
        updates.push(WritebackField::bounded(
            hourly_symbol("ui_SCrunf", hour),
            surface_value,
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            hourly_symbol("ui_LfCrf", hour),
            lateral_value,
            Some(0.0),
            None,
        ));
    }
    updates
}

fn stale_downstream_transfer_surface() -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface::default();
    surface.state_surface.insert(
        BoundarySymbol::from("UpStrmQ"),
        BoundaryValue::scalar(999.0),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("SubRIn"), BoundaryValue::scalar(999.0));
    for hour in 1..=24 {
        surface.state_surface.insert(
            BoundarySymbol::from(hourly_symbol("ui_SUrunf", hour)),
            BoundaryValue::scalar(999.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(hourly_symbol("ui_LfUrf", hour)),
            BoundaryValue::scalar(999.0),
        );
    }
    surface
}

fn hourly_value(values: &[(usize, f64)], hour: usize) -> f64 {
    values
        .iter()
        .find_map(|(candidate_hour, value)| {
            if *candidate_hour == hour {
                Some(*value)
            } else {
                None
            }
        })
        .unwrap_or(0.0)
}

fn hourly_symbol(root: &str, hour: usize) -> String {
    format!("{root}_{hour:04}")
}
