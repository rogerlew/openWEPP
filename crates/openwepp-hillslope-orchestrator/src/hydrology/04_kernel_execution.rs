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
            _ => {
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
