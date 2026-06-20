use super::{
    DIRECT_AUDIT, DIRECT_R4B_PHASE_SPAN_COUNT, DIRECT_R4C_PHASE_SPAN_COUNT, DirectDayFrame,
    DirectRuntimeError, validate_finite, validate_nonnegative_direct_m,
};

impl DirectDayFrame {
    pub fn run_r4c_storage_input_span(
        &mut self,
    ) -> Result<DirectStorageInputSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4C_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let storage_input = self.compute_r4c_storage_input()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.storage_input = storage_input;
        self.storage_reconciliation_inputs.storage_initial_m = storage_input.storage_initial_m;
        self.storage_reconciliation_inputs.precip_input_m = storage_input.precip_input_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.storage_input_downstream_operands =
            DirectStorageInputDownstreamOperands::from(storage_input);
        DIRECT_AUDIT.record_downstream_operand_production();

        let storage_input_shadow_projection = DirectStorageInputShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            storage_initial_m: self.storage_input_downstream_operands.storage_initial_m,
            precip_input_m: self.storage_input_downstream_operands.precip_input_m,
        };
        self.storage_input_shadow_projection = Some(storage_input_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectStorageInputSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            storage_input_shadow_projection,
        })
    }

    pub fn run_r4b_storage_reconciliation_span(
        &mut self,
    ) -> Result<DirectStorageReconciliationSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4B_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let storage_reconciliation = self.compute_r4b_storage_reconciliation()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.storage_reconciliation = storage_reconciliation;
        self.water.soil_water_m = storage_reconciliation.storage_reconciled_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.storage_downstream_operands =
            DirectStorageDownstreamOperands::from(storage_reconciliation);
        DIRECT_AUDIT.record_downstream_operand_production();

        let storage_shadow_projection = DirectStorageShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            storage_initial_m: self.storage_downstream_operands.storage_initial_m,
            precip_input_m: self.storage_downstream_operands.precip_input_m,
            snow_coupling_m: self.storage_downstream_operands.snow_coupling_m,
            q_runoff_m: self.storage_downstream_operands.q_runoff_m,
            evapotranspiration_m: self.storage_downstream_operands.evapotranspiration_m,
            deep_seepage_m: self.storage_downstream_operands.deep_seepage_m,
            subsurface_loss_m: self.storage_downstream_operands.subsurface_loss_m,
            storage_reconciled_m: self.storage_downstream_operands.storage_reconciled_m,
            closure_residual_m: self.storage_downstream_operands.closure_residual_m,
        };
        self.storage_shadow_projection = Some(storage_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectStorageReconciliationSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            storage_shadow_projection,
        })
    }

    fn compute_r4c_storage_input(&self) -> Result<DirectStorageInputState, DirectRuntimeError> {
        self.validate_r4c_storage_input_domain()?;
        Ok(DirectStorageInputState {
            storage_initial_m: self.water.soil_water_m,
            precip_input_m: self.downstream_operands.precipitation_m,
        })
    }

    fn compute_r4b_storage_reconciliation(
        &self,
    ) -> Result<DirectStorageReconciliationState, DirectRuntimeError> {
        self.validate_r4b_storage_reconciliation_domain()?;
        let inputs = self.storage_reconciliation_inputs;
        let q_runoff_m = self.runoff_downstream_operands.q_runoff_m;
        let storage_reconciled_m =
            inputs.storage_initial_m + inputs.precip_input_m + inputs.snow_coupling_m
                - q_runoff_m
                - inputs.evapotranspiration_m
                - inputs.deep_seepage_m
                - inputs.subsurface_loss_m;
        validate_finite(
            "storage_reconciliation.storage_reconciled_m",
            storage_reconciled_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.storage_reconciled_m",
            storage_reconciled_m,
        )?;
        let closure_residual_m =
            inputs.storage_initial_m + inputs.precip_input_m + inputs.snow_coupling_m
                - q_runoff_m
                - inputs.evapotranspiration_m
                - inputs.deep_seepage_m
                - inputs.subsurface_loss_m
                - storage_reconciled_m;
        validate_finite(
            "storage_reconciliation.closure_residual_m",
            closure_residual_m,
        )?;
        if closure_residual_m.abs() > inputs.closure_tolerance_m {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "storage_reconciliation.closure_residual_m",
            });
        }

        Ok(DirectStorageReconciliationState {
            storage_initial_m: inputs.storage_initial_m,
            precip_input_m: inputs.precip_input_m,
            snow_coupling_m: inputs.snow_coupling_m,
            q_runoff_m,
            evapotranspiration_m: inputs.evapotranspiration_m,
            deep_seepage_m: inputs.deep_seepage_m,
            subsurface_loss_m: inputs.subsurface_loss_m,
            closure_tolerance_m: inputs.closure_tolerance_m,
            storage_reconciled_m,
            closure_residual_m,
        })
    }

    fn validate_r4c_storage_input_domain(&self) -> Result<(), DirectRuntimeError> {
        if self.shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R3A input accounting",
            });
        }
        validate_nonnegative_direct_m("storage_input.storage_initial_m", self.water.soil_water_m)?;
        validate_nonnegative_direct_m(
            "storage_input.precip_input_m",
            self.downstream_operands.precipitation_m,
        )?;
        Ok(())
    }

    fn validate_r4b_storage_reconciliation_domain(&self) -> Result<(), DirectRuntimeError> {
        if self.storage_input_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4C storage input producer",
            });
        }
        if self.runoff_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4A runoff partition",
            });
        }
        validate_nonnegative_direct_m(
            "storage_reconciliation.storage_initial_m",
            self.storage_reconciliation_inputs.storage_initial_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.precip_input_m",
            self.storage_reconciliation_inputs.precip_input_m,
        )?;
        validate_finite(
            "storage_reconciliation.snow_coupling_m",
            self.storage_reconciliation_inputs.snow_coupling_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.q_runoff_m",
            self.runoff_downstream_operands.q_runoff_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.evapotranspiration_m",
            self.storage_reconciliation_inputs.evapotranspiration_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.deep_seepage_m",
            self.storage_reconciliation_inputs.deep_seepage_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.subsurface_loss_m",
            self.storage_reconciliation_inputs.subsurface_loss_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.closure_tolerance_m",
            self.storage_reconciliation_inputs.closure_tolerance_m,
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageInputState {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
}

impl DirectStorageInputState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            precip_input_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageInputDownstreamOperands {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
}

impl DirectStorageInputDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            precip_input_m: 0.0,
        }
    }
}

impl From<DirectStorageInputState> for DirectStorageInputDownstreamOperands {
    fn from(state: DirectStorageInputState) -> Self {
        Self {
            storage_initial_m: state.storage_initial_m,
            precip_input_m: state.precip_input_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageInputShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageReconciliationInputs {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub evapotranspiration_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub closure_tolerance_m: f64,
}

impl DirectStorageReconciliationInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            precip_input_m: 0.0,
            snow_coupling_m: 0.0,
            evapotranspiration_m: 0.0,
            deep_seepage_m: 0.0,
            subsurface_loss_m: 0.0,
            closure_tolerance_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageReconciliationState {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub closure_tolerance_m: f64,
    pub storage_reconciled_m: f64,
    pub closure_residual_m: f64,
}

impl DirectStorageReconciliationState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            precip_input_m: 0.0,
            snow_coupling_m: 0.0,
            q_runoff_m: 0.0,
            evapotranspiration_m: 0.0,
            deep_seepage_m: 0.0,
            subsurface_loss_m: 0.0,
            closure_tolerance_m: 0.0,
            storage_reconciled_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageDownstreamOperands {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub storage_reconciled_m: f64,
    pub closure_residual_m: f64,
}

impl DirectStorageDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            precip_input_m: 0.0,
            snow_coupling_m: 0.0,
            q_runoff_m: 0.0,
            evapotranspiration_m: 0.0,
            deep_seepage_m: 0.0,
            subsurface_loss_m: 0.0,
            storage_reconciled_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

impl From<DirectStorageReconciliationState> for DirectStorageDownstreamOperands {
    fn from(state: DirectStorageReconciliationState) -> Self {
        Self {
            storage_initial_m: state.storage_initial_m,
            precip_input_m: state.precip_input_m,
            snow_coupling_m: state.snow_coupling_m,
            q_runoff_m: state.q_runoff_m,
            evapotranspiration_m: state.evapotranspiration_m,
            deep_seepage_m: state.deep_seepage_m,
            subsurface_loss_m: state.subsurface_loss_m,
            storage_reconciled_m: state.storage_reconciled_m,
            closure_residual_m: state.closure_residual_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub storage_reconciled_m: f64,
    pub closure_residual_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageInputSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub storage_input_shadow_projection: DirectStorageInputShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageReconciliationSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub storage_shadow_projection: DirectStorageShadowProjection,
}
