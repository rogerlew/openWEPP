#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
pub(crate) fn run_storage_reconciliation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyStorageReconciliation;
        let storage_initial =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_INITIAL)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_INITIAL,
            storage_initial,
            Some(0.0),
            None,
        )?;

        let forward_solver_lane =
            Self::resolve_wb20_forward_solver_lane_enabled(request, phase_class)?;

        let closure_tolerance = Self::require_state_scalar(
            request,
            phase_class,
            WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE,
        )?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE,
            closure_tolerance,
            Some(0.0),
            None,
        )?;

        let precip_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_PRECIP_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_PRECIP_INPUT,
            precip_input,
            Some(0.0),
            None,
        )?;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;

        let snow_coupling_s =
            Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_SNOW_COUPLING_S)?;

        let interception_i =
            Self::require_flux_scalar(request, phase_class, WB15_SYMBOL_INTERCEPTION_I)?;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception_i,
            Some(0.0),
            None,
        )?;
        let irrigation_input =
            Self::optional_flux_scalar(request, phase_class, IRRIG_SYMBOL_DAILY_IRRIGATION)?
                .unwrap_or(0.0);
        Self::require_flux_range(
            phase_class,
            IRRIG_SYMBOL_DAILY_IRRIGATION,
            irrigation_input,
            Some(0.0),
            None,
        )?;
        let runon_input = Self::resolve_storage_runon_input(request, phase_class)?;

        let et = Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_ET)?;
        Self::require_flux_range(phase_class, WB11_SYMBOL_ET, et, Some(0.0), None)?;

        let percolation_loss =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_PERC_LOSS_D)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_LOSS_D,
            percolation_loss,
            Some(0.0),
            None,
        )?;
        let frost_bottom_overflow_symbol = BoundarySymbol::from("frost.runtime_watbtm_m");
        let frost_bottom_overflow = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &frost_bottom_overflow_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_state_range_for_symbol(
            phase_class,
            &frost_bottom_overflow_symbol,
            frost_bottom_overflow,
            Some(0.0),
            None,
        )?;

        let subsurface_loss =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_SUBHYD_QD)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_SUBHYD_QD,
            subsurface_loss,
            Some(0.0),
            None,
        )?;
        let frost_liquid_exchange = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("frost.runtime_frwatc_net_liquid_delta_m"),
        )?
        .unwrap_or(0.0);

        let storage_reconciled = Self::compute_storage_reconciled_with_interception(
            phase_class,
            storage_initial,
            precip_input,
            snow_coupling_s,
            irrigation_input,
            runon_input,
            interception_i,
            q_runoff,
            et,
            percolation_loss + frost_bottom_overflow,
            subsurface_loss,
            frost_liquid_exchange,
        )?;

        let closure_delta = if forward_solver_lane {
            let solver_closure =
                storage_initial + precip_input + snow_coupling_s + irrigation_input + runon_input
                    + frost_liquid_exchange
                    - interception_i
                    - q_runoff
                    - et
                    - percolation_loss
                    - frost_bottom_overflow
                    - subsurface_loss;
            solver_closure - storage_reconciled
        } else {
            let storage_observed =
                Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_OBSERVED)?;
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_STORAGE_OBSERVED,
                storage_observed,
                Some(0.0),
                None,
            )?;
            storage_reconciled - storage_observed
        };
        if closure_delta.abs() > closure_tolerance + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_STORAGE_CLOSURE_DELTA),
                value: closure_delta,
                minimum: Some(-closure_tolerance),
                maximum: Some(closure_tolerance),
            });
        }

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB12-STORAGE-OK-001",
        ) else {
            unreachable!("status message ids are non-empty WB12 constants")
        };
        let mut state_updates = vec![WritebackField::bounded(
            WB12_SYMBOL_STORAGE_RECONCILED,
            storage_reconciled,
            Some(0.0),
            None,
        )];
        if Self::resolve_mofe_hourly_carry_arrays_enabled(request, phase_class)? {
            state_updates.push(WritebackField::bounded(
                WB11_SYMBOL_SOIL_WATER,
                storage_reconciled,
                Some(0.0),
                None,
            ));
        }

        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![WritebackField::unbounded(
                WB12_SYMBOL_STORAGE_CLOSURE_DELTA,
                closure_delta,
            )],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    fn resolve_storage_runon_input(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let carryover_symbol = BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CARRYOVER);
        if let Some(carryover) =
            Self::optional_flux_scalar_for_symbol(request, phase_class, &carryover_symbol)?
        {
            Self::require_flux_range_for_symbol(
                phase_class,
                &carryover_symbol,
                carryover,
                Some(0.0),
                None,
            )?;
            return Ok(Self::normalize_non_negative_within_tolerance(carryover));
        }

        Self::resolve_runoff_carryover_input(request, phase_class)
    }

}
