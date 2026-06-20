use super::{
    DIRECT_AUDIT, DIRECT_R4A_PHASE_SPAN_COUNT, DIRECT_R4I_PHASE_SPAN_COUNT,
    DIRECT_R4J_PHASE_SPAN_COUNT, DIRECT_R4K_PHASE_SPAN_COUNT, DIRECT_R4L_PHASE_SPAN_COUNT,
    DirectDayFrame, DirectRuntimeError, validate_finite, validate_nonnegative_direct_m,
};

impl DirectDayFrame {
    pub fn run_r4i_liquid_input_span(
        &mut self,
    ) -> Result<DirectLiquidInputSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4I_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let liquid_input = self.compute_r4i_liquid_input()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.liquid_input = liquid_input;
        self.runoff_partition_inputs.liquid_input_m = liquid_input.liquid_input_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.liquid_input_downstream_operands =
            DirectLiquidInputDownstreamOperands::from(liquid_input);
        DIRECT_AUDIT.record_downstream_operand_production();

        let liquid_input_shadow_projection = DirectLiquidInputShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            liquid_input_m: self.liquid_input_downstream_operands.liquid_input_m,
        };
        self.liquid_input_shadow_projection = Some(liquid_input_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectLiquidInputSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            liquid_input_shadow_projection,
        })
    }

    pub fn run_r4j_runon_carry_span(
        &mut self,
    ) -> Result<DirectRunonCarrySpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4J_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let runon_carry = self.compute_r4j_runon_carry()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.runon_carry = runon_carry;
        self.runoff_partition_inputs.runon_input_m = runon_carry.runon_input_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.runon_carry_downstream_operands =
            DirectRunonCarryDownstreamOperands::from(runon_carry);
        DIRECT_AUDIT.record_downstream_operand_production();

        let runon_carry_shadow_projection = DirectRunonCarryShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            runon_input_m: self.runon_carry_downstream_operands.runon_input_m,
            subsurface_carry_m: self.runon_carry_downstream_operands.subsurface_carry_m,
        };
        self.runon_carry_shadow_projection = Some(runon_carry_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectRunonCarrySpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            runon_carry_shadow_projection,
        })
    }

    pub fn run_r4k_infiltration_depression_span(
        &mut self,
    ) -> Result<DirectInfiltrationDepressionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4K_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let infiltration_depression = self.compute_r4k_infiltration_depression()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.infiltration_depression = infiltration_depression;
        self.runoff_partition_inputs.cumulative_infiltration_m =
            infiltration_depression.cumulative_infiltration_m;
        self.runoff_partition_inputs.depression_storage_delta_m =
            infiltration_depression.depression_storage_delta_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.infiltration_depression_downstream_operands =
            DirectInfiltrationDepressionDownstreamOperands::from(infiltration_depression);
        DIRECT_AUDIT.record_downstream_operand_production();

        let infiltration_depression_shadow_projection =
            DirectInfiltrationDepressionShadowProjection {
                lane_index: self.lane_index,
                day_index: self.day_index,
                cumulative_infiltration_m: self
                    .infiltration_depression_downstream_operands
                    .cumulative_infiltration_m,
                depression_storage_delta_m: self
                    .infiltration_depression_downstream_operands
                    .depression_storage_delta_m,
            };
        self.infiltration_depression_shadow_projection =
            Some(infiltration_depression_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectInfiltrationDepressionSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            infiltration_depression_shadow_projection,
        })
    }

    pub fn run_r4l_saturation_addback_span(
        &mut self,
    ) -> Result<DirectSaturationAddbackSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4L_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let saturation_addback = self.compute_r4l_saturation_addback()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.saturation_addback = saturation_addback;
        self.runoff_partition_inputs.surface_saturation_runoff_m =
            saturation_addback.surface_saturation_runoff_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.saturation_addback_downstream_operands =
            DirectSaturationAddbackDownstreamOperands::from(saturation_addback);
        DIRECT_AUDIT.record_downstream_operand_production();

        let saturation_addback_shadow_projection = DirectSaturationAddbackShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            surface_saturation_runoff_m: self
                .saturation_addback_downstream_operands
                .surface_saturation_runoff_m,
        };
        self.saturation_addback_shadow_projection = Some(saturation_addback_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectSaturationAddbackSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            saturation_addback_shadow_projection,
        })
    }

    pub fn run_r4a_runoff_partition_span(
        &mut self,
    ) -> Result<DirectRunoffPartitionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4A_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;
        let mut direct_compute_count = 0_u64;
        let mut state_mutation_count = 0_u64;
        let mut downstream_operand_count = 0_u64;
        let mut shadow_projection_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let runoff_partition = self.compute_r4a_runoff_partition()?;
        DIRECT_AUDIT.record_direct_compute_operation();
        direct_compute_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.runoff_partition = runoff_partition;
        self.water.infiltration_m = runoff_partition.cumulative_infiltration_m;
        self.water.runoff_m = runoff_partition.q_runoff_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        state_mutation_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.runoff_downstream_operands = DirectRunoffDownstreamOperands::from(runoff_partition);
        DIRECT_AUDIT.record_downstream_operand_production();
        downstream_operand_count += 1;

        let runoff_shadow_projection = DirectRunoffShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            liquid_input_m: self.runoff_downstream_operands.liquid_input_m,
            runon_input_m: self.runoff_downstream_operands.runon_input_m,
            cumulative_infiltration_m: self.runoff_downstream_operands.cumulative_infiltration_m,
            depression_storage_delta_m: self.runoff_downstream_operands.depression_storage_delta_m,
            surface_saturation_runoff_m: self
                .runoff_downstream_operands
                .surface_saturation_runoff_m,
            partition_runoff_m: self.runoff_downstream_operands.partition_runoff_m,
            q_runoff_m: self.runoff_downstream_operands.q_runoff_m,
            closure_residual_m: self.runoff_downstream_operands.closure_residual_m,
        };
        self.runoff_shadow_projection = Some(runoff_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();
        shadow_projection_count += 1;

        Ok(DirectRunoffPartitionSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count,
            state_mutation_count,
            downstream_operand_count,
            shadow_projection_count,
            compatibility_edge_invocation_count: 0,
            runoff_shadow_projection,
        })
    }

    fn compute_r4i_liquid_input(&self) -> Result<DirectLiquidInputState, DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "liquid_input.liquid_input_handoff_m",
            self.liquid_input_inputs.liquid_input_handoff_m,
        )?;
        Ok(DirectLiquidInputState {
            liquid_input_m: self.liquid_input_inputs.liquid_input_handoff_m,
        })
    }

    fn compute_r4j_runon_carry(&self) -> Result<DirectRunonCarryState, DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "runon_carry.surface_runon_handoff_m",
            self.runon_carry_inputs.surface_runon_handoff_m,
        )?;
        validate_nonnegative_direct_m(
            "runon_carry.subsurface_carry_handoff_m",
            self.runon_carry_inputs.subsurface_carry_handoff_m,
        )?;
        Ok(DirectRunonCarryState {
            runon_input_m: self.runon_carry_inputs.surface_runon_handoff_m,
            subsurface_carry_m: self.runon_carry_inputs.subsurface_carry_handoff_m,
        })
    }

    fn compute_r4k_infiltration_depression(
        &self,
    ) -> Result<DirectInfiltrationDepressionState, DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "infiltration_depression.cumulative_infiltration_handoff_m",
            self.infiltration_depression_inputs
                .cumulative_infiltration_handoff_m,
        )?;
        validate_nonnegative_direct_m(
            "infiltration_depression.depression_storage_delta_handoff_m",
            self.infiltration_depression_inputs
                .depression_storage_delta_handoff_m,
        )?;
        Ok(DirectInfiltrationDepressionState {
            cumulative_infiltration_m: self
                .infiltration_depression_inputs
                .cumulative_infiltration_handoff_m,
            depression_storage_delta_m: self
                .infiltration_depression_inputs
                .depression_storage_delta_handoff_m,
        })
    }

    fn compute_r4l_saturation_addback(
        &self,
    ) -> Result<DirectSaturationAddbackState, DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "saturation_addback.surface_saturation_runoff_handoff_m",
            self.saturation_addback_inputs
                .surface_saturation_runoff_handoff_m,
        )?;
        Ok(DirectSaturationAddbackState {
            surface_saturation_runoff_m: self
                .saturation_addback_inputs
                .surface_saturation_runoff_handoff_m,
        })
    }

    fn compute_r4a_runoff_partition(
        &self,
    ) -> Result<DirectRunoffPartitionState, DirectRuntimeError> {
        self.ensure_r4il_runoff_inputs_ready()?;
        self.validate_r4a_runoff_partition_domain()?;
        let inputs = self.runoff_partition_inputs;
        let liquid_and_runon_m = inputs.liquid_input_m + inputs.runon_input_m;
        validate_finite("runoff_partition.liquid_and_runon_m", liquid_and_runon_m)?;
        let retained_m = inputs.cumulative_infiltration_m + inputs.depression_storage_delta_m;
        validate_finite("runoff_partition.retained_m", retained_m)?;
        let partition_runoff_m = liquid_and_runon_m - retained_m;
        validate_finite("runoff_partition.partition_runoff_m", partition_runoff_m)?;
        validate_nonnegative_direct_m("runoff_partition.partition_runoff_m", partition_runoff_m)?;
        let q_runoff_m = partition_runoff_m + inputs.surface_saturation_runoff_m;
        validate_finite("runoff_partition.q_runoff_m", q_runoff_m)?;
        validate_nonnegative_direct_m("runoff_partition.q_runoff_m", q_runoff_m)?;
        let closure_residual_m =
            inputs.liquid_input_m + inputs.runon_input_m + inputs.surface_saturation_runoff_m
                - inputs.cumulative_infiltration_m
                - inputs.depression_storage_delta_m
                - q_runoff_m;
        validate_finite("runoff_partition.closure_residual_m", closure_residual_m)?;

        Ok(DirectRunoffPartitionState {
            liquid_input_m: inputs.liquid_input_m,
            runon_input_m: inputs.runon_input_m,
            cumulative_infiltration_m: inputs.cumulative_infiltration_m,
            depression_storage_delta_m: inputs.depression_storage_delta_m,
            surface_saturation_runoff_m: inputs.surface_saturation_runoff_m,
            partition_runoff_m,
            q_runoff_m,
            closure_residual_m,
        })
    }

    fn ensure_r4il_runoff_inputs_ready(&self) -> Result<(), DirectRuntimeError> {
        if self.liquid_input_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4I liquid input",
            });
        }
        if self.runon_carry_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4J runon/carry",
            });
        }
        if self.infiltration_depression_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4K infiltration/depression",
            });
        }
        if self.saturation_addback_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4L saturation addback",
            });
        }
        Ok(())
    }

    fn validate_r4a_runoff_partition_domain(&self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "runoff_partition.liquid_input_m",
            self.runoff_partition_inputs.liquid_input_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.runon_input_m",
            self.runoff_partition_inputs.runon_input_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.cumulative_infiltration_m",
            self.runoff_partition_inputs.cumulative_infiltration_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.depression_storage_delta_m",
            self.runoff_partition_inputs.depression_storage_delta_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.surface_saturation_runoff_m",
            self.runoff_partition_inputs.surface_saturation_runoff_m,
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputInputs {
    pub liquid_input_handoff_m: f64,
}

impl DirectLiquidInputInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputState {
    pub liquid_input_m: f64,
}

impl DirectLiquidInputState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputDownstreamOperands {
    pub liquid_input_m: f64,
}

impl DirectLiquidInputDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
        }
    }
}

impl From<DirectLiquidInputState> for DirectLiquidInputDownstreamOperands {
    fn from(state: DirectLiquidInputState) -> Self {
        Self {
            liquid_input_m: state.liquid_input_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub liquid_input_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarryInputs {
    pub surface_runon_handoff_m: f64,
    pub subsurface_carry_handoff_m: f64,
}

impl DirectRunonCarryInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_runon_handoff_m: 0.0,
            subsurface_carry_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarryState {
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
}

impl DirectRunonCarryState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            runon_input_m: 0.0,
            subsurface_carry_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarryDownstreamOperands {
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
}

impl DirectRunonCarryDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            runon_input_m: 0.0,
            subsurface_carry_m: 0.0,
        }
    }
}

impl From<DirectRunonCarryState> for DirectRunonCarryDownstreamOperands {
    fn from(state: DirectRunonCarryState) -> Self {
        Self {
            runon_input_m: state.runon_input_m,
            subsurface_carry_m: state.subsurface_carry_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarryShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionInputs {
    pub cumulative_infiltration_handoff_m: f64,
    pub depression_storage_delta_handoff_m: f64,
}

impl DirectInfiltrationDepressionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            cumulative_infiltration_handoff_m: 0.0,
            depression_storage_delta_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionState {
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
}

impl DirectInfiltrationDepressionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionDownstreamOperands {
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
}

impl DirectInfiltrationDepressionDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
        }
    }
}

impl From<DirectInfiltrationDepressionState> for DirectInfiltrationDepressionDownstreamOperands {
    fn from(state: DirectInfiltrationDepressionState) -> Self {
        Self {
            cumulative_infiltration_m: state.cumulative_infiltration_m,
            depression_storage_delta_m: state.depression_storage_delta_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackInputs {
    pub surface_saturation_runoff_handoff_m: f64,
}

impl DirectSaturationAddbackInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_saturation_runoff_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackState {
    pub surface_saturation_runoff_m: f64,
}

impl DirectSaturationAddbackState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_saturation_runoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackDownstreamOperands {
    pub surface_saturation_runoff_m: f64,
}

impl DirectSaturationAddbackDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_saturation_runoff_m: 0.0,
        }
    }
}

impl From<DirectSaturationAddbackState> for DirectSaturationAddbackDownstreamOperands {
    fn from(state: DirectSaturationAddbackState) -> Self {
        Self {
            surface_saturation_runoff_m: state.surface_saturation_runoff_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub surface_saturation_runoff_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffPartitionInputs {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
}

impl DirectRunoffPartitionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffPartitionState {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

impl DirectRunoffPartitionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
            partition_runoff_m: 0.0,
            q_runoff_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffDownstreamOperands {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

impl DirectRunoffDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
            partition_runoff_m: 0.0,
            q_runoff_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

impl From<DirectRunoffPartitionState> for DirectRunoffDownstreamOperands {
    fn from(state: DirectRunoffPartitionState) -> Self {
        Self {
            liquid_input_m: state.liquid_input_m,
            runon_input_m: state.runon_input_m,
            cumulative_infiltration_m: state.cumulative_infiltration_m,
            depression_storage_delta_m: state.depression_storage_delta_m,
            surface_saturation_runoff_m: state.surface_saturation_runoff_m,
            partition_runoff_m: state.partition_runoff_m,
            q_runoff_m: state.q_runoff_m,
            closure_residual_m: state.closure_residual_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub liquid_input_shadow_projection: DirectLiquidInputShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarrySpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub runon_carry_shadow_projection: DirectRunonCarryShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub infiltration_depression_shadow_projection: DirectInfiltrationDepressionShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub saturation_addback_shadow_projection: DirectSaturationAddbackShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffPartitionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub runoff_shadow_projection: DirectRunoffShadowProjection,
}
