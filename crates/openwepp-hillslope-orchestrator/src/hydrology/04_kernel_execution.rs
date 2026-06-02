impl HillslopeKernel for Wb11HydrologyKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        let response_result = match request.phase_class {
            HillslopeKernelPhaseClass::HydrologyEvapotranspiration => {
                Self::run_evapotranspiration(request)
            }
            HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage => {
                Self::run_percolation(request)
            }
            HillslopeKernelPhaseClass::HydrologyLateralTransfer => {
                Self::run_lateral_transfer(request)
            }
            HillslopeKernelPhaseClass::HydrologyDrainage => Self::run_drainage(request),
            HillslopeKernelPhaseClass::HydrologyPlantRootUptake => {
                Self::run_plant_root_uptake(request)
            }
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation => {
                Self::run_runoff_reconciliation(request)
            }
            HillslopeKernelPhaseClass::HydrologyStorageReconciliation => {
                Self::run_storage_reconciliation(request)
            }
            HillslopeKernelPhaseClass::HydrologyPeakRunoff => Self::run_peak_runoff(request),
            HillslopeKernelPhaseClass::DecompositionTransition
            | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                Ok(Self::run_decomposition_transition(request))
            }
            HillslopeKernelPhaseClass::GrowthAnnualTransition
            | HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                Ok(Self::run_growth_transition(request))
            }
            HillslopeKernelPhaseClass::Hydrology => {
                let Ok(status) =
                    SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-NOP-001")
                else {
                    unreachable!("status message ids are non-empty WB11 constants")
                };
                Ok(KernelRunResponse::new(
                    status,
                    KernelWritebackPayload::empty(),
                ))
            }
        };

        match response_result {
            Ok(response) => response,
            Err(error) => KernelRunResponse::new(
                Self::status_from_guard_error(&error),
                KernelWritebackPayload::empty(),
            ),
        }
    }
}

impl Wb11HydrologyKernel {
    fn run_decomposition_transition(request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        let Some(context) = request.decomposition_context else {
            let status = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB11-DECOMP-NOP-001",
            )
            .expect("status message ids are non-empty WB11 constants");
            return KernelRunResponse::new(status, KernelWritebackPayload::empty());
        };
        let Some(payload) = context.transition_payload else {
            let status = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB11-DECOMP-NOP-002",
            )
            .expect("status message ids are non-empty WB11 constants");
            return KernelRunResponse::new(status, KernelWritebackPayload::empty());
        };

        let status = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB11-DECOMP-OK-001",
        )
        .expect("status message ids are non-empty WB11 constants");
        let writeback = KernelWritebackPayload::with_updates(
            vec![
                WritebackField::bounded(
                    PL_DECOMP_IRESD_SEED_SYMBOL,
                    payload.iresd_seed,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(
                    PL_DECOMP_SUMRTM_SEED_SYMBOL,
                    payload.sumrtm_seed,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(
                    PL_DECOMP_SUMSRM_SEED_SYMBOL,
                    payload.sumsrm_seed,
                    Some(0.0),
                    None,
                ),
            ],
            Vec::new(),
        );
        KernelRunResponse::new(status, writeback)
    }

    fn run_growth_transition(request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        let Some(context) = request.growth_context else {
            let status = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB11-GROWTH-NOP-001",
            )
            .expect("status message ids are non-empty WB11 constants");
            return KernelRunResponse::new(status, KernelWritebackPayload::empty());
        };
        let Some(payload) = context.transition_payload else {
            let status = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB11-GROWTH-NOP-002",
            )
            .expect("status message ids are non-empty WB11 constants");
            return KernelRunResponse::new(status, KernelWritebackPayload::empty());
        };

        let state = payload.state_after;
        let status = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB11-GROWTH-OK-001",
        )
        .expect("status message ids are non-empty WB11 constants");
        let writeback = KernelWritebackPayload::with_updates(
            vec![
                WritebackField::bounded(
                    PL_GROWTH_STATE_SUMGDD_SYMBOL,
                    state.sumgdd,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(
                    PL_GROWTH_STATE_VDMT_SYMBOL,
                    state.vdmt,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(
                    PL_GROWTH_STATE_CANCOV_SYMBOL,
                    state.cancov,
                    Some(0.0),
                    Some(PL_GROWTH_CANCOV_MAX),
                ),
                WritebackField::bounded(
                    PL_GROWTH_STATE_LAI_SYMBOL,
                    state.lai,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(
                    PL_GROWTH_STATE_RTMASS_SYMBOL,
                    state.rtmass,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(
                    PL_GROWTH_STATE_RTD_SYMBOL,
                    state.rtd,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(
                    PL_GROWTH_STATE_HIA_SYMBOL,
                    state.hia,
                    Some(0.0),
                    Some(1.0),
                ),
            ],
            Vec::new(),
        );
        KernelRunResponse::new(status, writeback)
    }
}
