use super::{
    DIRECT_AUDIT, DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT,
    DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT, DirectDayFrame, DirectDownstreamOperands,
    DirectInputAccountingState, DirectRuntimeError, DirectShadowProjection,
    DirectStorageReconciliationInputs, scaled_direct_transfer_total_m, sum_nonnegative_direct_m,
    validate_finite, validate_nonnegative_direct_m,
};

impl DirectDayFrame {
    pub fn run_r5b_normalization_phase(
        &mut self,
    ) -> Result<DirectNormalizationSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        DIRECT_AUDIT.record_direct_phase_entry();

        let normalization = self.compute_r5b_normalization()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        self.normalization_inputs = DirectNormalizationInputs::from_frame(self)?;
        self.normalization = normalization;
        self.input_accounting = DirectInputAccountingState::from(normalization);
        self.storage_reconciliation_inputs.storage_initial_m = normalization.storage_initial_m;
        self.storage_reconciliation_inputs.precip_input_m = normalization.precipitation_m;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.normalization_downstream_operands =
            DirectNormalizationDownstreamOperands::from(normalization);
        self.downstream_operands = DirectDownstreamOperands::from(self.input_accounting);
        DIRECT_AUDIT.record_downstream_operand_production();

        let normalization_shadow_projection = DirectNormalizationShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            precipitation_m: self.normalization_downstream_operands.precipitation_m,
            storage_initial_m: self.normalization_downstream_operands.storage_initial_m,
            transfer_input_m: self.normalization_downstream_operands.transfer_input_m,
            total_accounted_input_m: self
                .normalization_downstream_operands
                .total_accounted_input_m,
        };
        self.normalization_shadow_projection = Some(normalization_shadow_projection);
        self.shadow_projection = Some(DirectShadowProjection {
            lane_index: normalization_shadow_projection.lane_index,
            day_index: normalization_shadow_projection.day_index,
            precipitation_m: normalization_shadow_projection.precipitation_m,
            transfer_input_m: normalization_shadow_projection.transfer_input_m,
            total_accounted_input_m: normalization_shadow_projection.total_accounted_input_m,
        });
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectNormalizationSpanReport {
            phase_count: DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT,
            phase_entry_count: 1,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            normalization_shadow_projection,
        })
    }

    pub fn run_r5b_storage_bounds_phase(
        &mut self,
    ) -> Result<DirectStorageBoundsSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        DIRECT_AUDIT.record_direct_phase_entry();

        let storage_bounds = self.compute_r5b_storage_bounds()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        self.storage_bounds_inputs = DirectStorageBoundsInputs::from_frame(self)?;
        self.storage_bounds = storage_bounds;
        self.water.soil_water_m = storage_bounds.storage_bounded_m;
        self.storage_reconciliation_inputs.storage_initial_m = storage_bounds.storage_bounded_m;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.storage_bounds_downstream_operands =
            DirectStorageBoundsDownstreamOperands::from(storage_bounds);
        DIRECT_AUDIT.record_downstream_operand_production();

        let storage_bounds_shadow_projection = DirectStorageBoundsShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            storage_bounded_m: self.storage_bounds_downstream_operands.storage_bounded_m,
            total_accounted_input_m: self
                .storage_bounds_downstream_operands
                .total_accounted_input_m,
            closure_tolerance_m: self.storage_bounds_downstream_operands.closure_tolerance_m,
        };
        self.storage_bounds_shadow_projection = Some(storage_bounds_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectStorageBoundsSpanReport {
            phase_count: DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT,
            phase_entry_count: 1,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            storage_bounds_shadow_projection,
        })
    }

    fn compute_r5b_normalization(&self) -> Result<DirectNormalizationState, DirectRuntimeError> {
        let inputs = DirectNormalizationInputs::from_frame(self)?;
        let transfer_input_m = inputs.surface_transfer_m
            + inputs.lateral_transfer_m
            + inputs.upstream_flow_m
            + inputs.subsurface_input_m;
        validate_finite("normalization.transfer_input_m", transfer_input_m)?;
        validate_nonnegative_direct_m("normalization.transfer_input_m", transfer_input_m)?;
        let total_accounted_input_m = inputs.precipitation_m + transfer_input_m;
        validate_finite(
            "normalization.total_accounted_input_m",
            total_accounted_input_m,
        )?;
        validate_nonnegative_direct_m(
            "normalization.total_accounted_input_m",
            total_accounted_input_m,
        )?;

        Ok(DirectNormalizationState {
            precipitation_m: inputs.precipitation_m,
            effective_temperature_c: inputs.effective_temperature_c,
            storage_initial_m: inputs.storage_initial_m,
            surface_transfer_m: inputs.surface_transfer_m,
            lateral_transfer_m: inputs.lateral_transfer_m,
            upstream_flow_m: inputs.upstream_flow_m,
            subsurface_input_m: inputs.subsurface_input_m,
            transfer_input_m,
            total_accounted_input_m,
        })
    }

    fn compute_r5b_storage_bounds(&self) -> Result<DirectStorageBoundsState, DirectRuntimeError> {
        if self.normalization_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R5B normalization phase",
            });
        }
        let inputs = DirectStorageBoundsInputs::from_frame(self)?;
        validate_nonnegative_direct_m(
            "storage_bounds.storage_initial_m",
            inputs.storage_initial_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_bounds.total_accounted_input_m",
            inputs.total_accounted_input_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_bounds.closure_tolerance_m",
            inputs.closure_tolerance_m,
        )?;

        Ok(DirectStorageBoundsState {
            storage_bounded_m: inputs.storage_initial_m,
            total_accounted_input_m: inputs.total_accounted_input_m,
            closure_tolerance_m: inputs.closure_tolerance_m,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectNormalizationInputs {
    pub precipitation_m: f64,
    pub effective_temperature_c: f64,
    pub storage_initial_m: f64,
    pub surface_transfer_m: f64,
    pub lateral_transfer_m: f64,
    pub upstream_flow_m: f64,
    pub subsurface_input_m: f64,
}

impl DirectNormalizationInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            precipitation_m: 0.0,
            effective_temperature_c: 0.0,
            storage_initial_m: 0.0,
            surface_transfer_m: 0.0,
            lateral_transfer_m: 0.0,
            upstream_flow_m: 0.0,
            subsurface_input_m: 0.0,
        }
    }

    fn from_frame(frame: &DirectDayFrame) -> Result<Self, DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "normalization.precipitation_m",
            frame.forcing.precipitation_m,
        )?;
        validate_finite(
            "normalization.effective_temperature_c",
            frame.forcing.effective_temperature_c,
        )?;
        validate_nonnegative_direct_m("normalization.storage_initial_m", frame.water.soil_water_m)?;
        validate_nonnegative_direct_m(
            "normalization.upstream_flow_m",
            frame.transfer.upstream_flow_m,
        )?;
        validate_nonnegative_direct_m(
            "normalization.subsurface_input_m",
            frame.transfer.subsurface_input_m,
        )?;
        validate_nonnegative_direct_m(
            "normalization.publication.runoff_m",
            frame.publication.runoff_m,
        )?;
        validate_nonnegative_direct_m(
            "normalization.publication.infiltration_m",
            frame.publication.infiltration_m,
        )?;
        validate_nonnegative_direct_m(
            "normalization.publication.evapotranspiration_m",
            frame.publication.evapotranspiration_m,
        )?;
        validate_nonnegative_direct_m(
            "normalization.publication.drainage_m",
            frame.publication.drainage_m,
        )?;
        validate_nonnegative_direct_m(
            "normalization.publication.lateral_flow_m",
            frame.publication.lateral_flow_m,
        )?;

        let raw_surface_transfer_m = sum_nonnegative_direct_m(
            "normalization.surface_carry_m",
            &frame.transfer.surface_carry_m,
        )?;
        let raw_lateral_transfer_m = sum_nonnegative_direct_m(
            "normalization.lateral_carry_m",
            &frame.transfer.lateral_carry_m,
        )?;
        let surface_transfer_m = scaled_direct_transfer_total_m(
            "normalization.surface_transfer_m",
            raw_surface_transfer_m,
            frame.upstream_area_ratio,
        )?;
        let lateral_transfer_m = scaled_direct_transfer_total_m(
            "normalization.lateral_transfer_m",
            raw_lateral_transfer_m,
            frame.upstream_area_ratio,
        )?;

        Ok(Self {
            precipitation_m: frame.forcing.precipitation_m,
            effective_temperature_c: frame.forcing.effective_temperature_c,
            storage_initial_m: frame.water.soil_water_m,
            surface_transfer_m,
            lateral_transfer_m,
            upstream_flow_m: frame.transfer.upstream_flow_m,
            subsurface_input_m: frame.transfer.subsurface_input_m,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectNormalizationState {
    pub precipitation_m: f64,
    pub effective_temperature_c: f64,
    pub storage_initial_m: f64,
    pub surface_transfer_m: f64,
    pub lateral_transfer_m: f64,
    pub upstream_flow_m: f64,
    pub subsurface_input_m: f64,
    pub transfer_input_m: f64,
    pub total_accounted_input_m: f64,
}

impl DirectNormalizationState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            precipitation_m: 0.0,
            effective_temperature_c: 0.0,
            storage_initial_m: 0.0,
            surface_transfer_m: 0.0,
            lateral_transfer_m: 0.0,
            upstream_flow_m: 0.0,
            subsurface_input_m: 0.0,
            transfer_input_m: 0.0,
            total_accounted_input_m: 0.0,
        }
    }
}

impl From<DirectNormalizationState> for DirectInputAccountingState {
    fn from(state: DirectNormalizationState) -> Self {
        Self {
            precipitation_m: state.precipitation_m,
            surface_transfer_m: state.surface_transfer_m,
            lateral_transfer_m: state.lateral_transfer_m,
            upstream_flow_m: state.upstream_flow_m,
            subsurface_input_m: state.subsurface_input_m,
            transfer_input_m: state.transfer_input_m,
            total_accounted_input_m: state.total_accounted_input_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectNormalizationDownstreamOperands {
    pub precipitation_m: f64,
    pub effective_temperature_c: f64,
    pub storage_initial_m: f64,
    pub surface_transfer_m: f64,
    pub lateral_transfer_m: f64,
    pub upstream_flow_m: f64,
    pub subsurface_input_m: f64,
    pub transfer_input_m: f64,
    pub total_accounted_input_m: f64,
}

impl DirectNormalizationDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            precipitation_m: 0.0,
            effective_temperature_c: 0.0,
            storage_initial_m: 0.0,
            surface_transfer_m: 0.0,
            lateral_transfer_m: 0.0,
            upstream_flow_m: 0.0,
            subsurface_input_m: 0.0,
            transfer_input_m: 0.0,
            total_accounted_input_m: 0.0,
        }
    }
}

impl From<DirectNormalizationState> for DirectNormalizationDownstreamOperands {
    fn from(state: DirectNormalizationState) -> Self {
        Self {
            precipitation_m: state.precipitation_m,
            effective_temperature_c: state.effective_temperature_c,
            storage_initial_m: state.storage_initial_m,
            surface_transfer_m: state.surface_transfer_m,
            lateral_transfer_m: state.lateral_transfer_m,
            upstream_flow_m: state.upstream_flow_m,
            subsurface_input_m: state.subsurface_input_m,
            transfer_input_m: state.transfer_input_m,
            total_accounted_input_m: state.total_accounted_input_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectNormalizationShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub precipitation_m: f64,
    pub storage_initial_m: f64,
    pub transfer_input_m: f64,
    pub total_accounted_input_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectNormalizationSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub normalization_shadow_projection: DirectNormalizationShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectStorageBoundsInputs {
    pub storage_initial_m: f64,
    pub total_accounted_input_m: f64,
    pub closure_tolerance_m: f64,
}

impl DirectStorageBoundsInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            total_accounted_input_m: 0.0,
            closure_tolerance_m: 0.0,
        }
    }

    fn from_frame(frame: &DirectDayFrame) -> Result<Self, DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "storage_bounds.storage_initial_m",
            frame.normalization_downstream_operands.storage_initial_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_bounds.total_accounted_input_m",
            frame
                .normalization_downstream_operands
                .total_accounted_input_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_bounds.closure_tolerance_m",
            frame.storage_reconciliation_inputs.closure_tolerance_m,
        )?;

        Ok(Self {
            storage_initial_m: frame.normalization_downstream_operands.storage_initial_m,
            total_accounted_input_m: frame
                .normalization_downstream_operands
                .total_accounted_input_m,
            closure_tolerance_m: frame.storage_reconciliation_inputs.closure_tolerance_m,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageBoundsState {
    pub storage_bounded_m: f64,
    pub total_accounted_input_m: f64,
    pub closure_tolerance_m: f64,
}

impl DirectStorageBoundsState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_bounded_m: 0.0,
            total_accounted_input_m: 0.0,
            closure_tolerance_m: 0.0,
        }
    }
}

impl From<DirectStorageBoundsState> for DirectStorageReconciliationInputs {
    fn from(state: DirectStorageBoundsState) -> Self {
        Self {
            storage_initial_m: state.storage_bounded_m,
            precip_input_m: 0.0,
            snow_coupling_m: 0.0,
            frost_liquid_delta_m: 0.0,
            runon_input_m: 0.0,
            interception_m: 0.0,
            evapotranspiration_m: 0.0,
            evapotranspiration_storage_return_m: 0.0,
            deep_seepage_m: 0.0,
            subsurface_loss_m: 0.0,
            closure_tolerance_m: state.closure_tolerance_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageBoundsDownstreamOperands {
    pub storage_bounded_m: f64,
    pub total_accounted_input_m: f64,
    pub closure_tolerance_m: f64,
}

impl DirectStorageBoundsDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_bounded_m: 0.0,
            total_accounted_input_m: 0.0,
            closure_tolerance_m: 0.0,
        }
    }
}

impl From<DirectStorageBoundsState> for DirectStorageBoundsDownstreamOperands {
    fn from(state: DirectStorageBoundsState) -> Self {
        Self {
            storage_bounded_m: state.storage_bounded_m,
            total_accounted_input_m: state.total_accounted_input_m,
            closure_tolerance_m: state.closure_tolerance_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageBoundsShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub storage_bounded_m: f64,
    pub total_accounted_input_m: f64,
    pub closure_tolerance_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageBoundsSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub storage_bounds_shadow_projection: DirectStorageBoundsShadowProjection,
}
