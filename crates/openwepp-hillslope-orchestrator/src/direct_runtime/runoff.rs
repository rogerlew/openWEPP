use crate::constants::{
    WB11_ZERO_THRESHOLD, WB15_BIOMASS_TO_KG_HA, WB15_CANCOV_MAX, WB15_INTERCEPT_BIOMASS_MAX_KG_HA,
    WB15_INTERCEPT_LINEAR_COEFF, WB15_INTERCEPT_MM_TO_M, WB15_INTERCEPT_QUADRATIC_COEFF,
    WB16_MAX_DURATION_S, WB16_PEAKRO_FLOOR, WB16_RUNOFF_NEAR_ZERO_THRESHOLD,
};
use crate::hydrology::{DirectFrostLiquidPartition, DirectFrostRunoffSurface};

use super::{
    DIRECT_AUDIT, DIRECT_R4A_PHASE_SPAN_COUNT, DIRECT_R4I_PHASE_SPAN_COUNT,
    DIRECT_R4J_PHASE_SPAN_COUNT, DIRECT_R4K_PHASE_SPAN_COUNT, DIRECT_R4L_PHASE_SPAN_COUNT,
    DIRECT_R7D6_PEAK_RUNOFF_PHASE_SPAN_COUNT, DirectDayFrame, DirectFrostFineLayerCarry,
    DirectFrostLayerShadowCarry, DirectFrostRuntimeCarry, DirectRuntimeError,
    DirectSubsurfaceLayerState, scaled_direct_transfer_total_m, sum_nonnegative_direct_m,
    validate_finite, validate_nonnegative_direct_m,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum R4aFrostLiquidDeltaAuthority {
    PartitionFrwatc,
    AppliedLayerProjection,
}

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
        self.runoff_partition_inputs.runon_input_m =
            runon_carry.runon_input_m + runon_carry.subsurface_carry_m;
        validate_finite(
            "runon_carry.partition_runon_input_m",
            self.runoff_partition_inputs.runon_input_m,
        )?;
        self.storage_reconciliation_inputs.runon_input_m =
            self.runoff_partition_inputs.runon_input_m;
        validate_finite(
            "runon_carry.storage_transfer_input_m",
            self.storage_reconciliation_inputs.runon_input_m,
        )?;
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
        self.percolation_inputs.same_pass_infiltration_m =
            infiltration_depression.cumulative_infiltration_m;
        self.percolation_inputs.same_pass_infiltration_lineage = true;
        self.evapotranspiration_compute_inputs
            .same_pass_infiltration_m = infiltration_depression.cumulative_infiltration_m;
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

        if self.reconcile_r4a_frost_runtime()? {
            DIRECT_AUDIT.record_direct_compute_operation();
            direct_compute_count += 1;
            DIRECT_AUDIT.record_direct_state_mutation();
            state_mutation_count += 1;
        }

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

    pub fn run_r7d6_peak_runoff_span(
        &mut self,
    ) -> Result<DirectPeakRunoffSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R7D6_PEAK_RUNOFF_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let peak_runoff = self.compute_r7d6_peak_runoff()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.peak_runoff = peak_runoff;
        DIRECT_AUDIT.record_direct_state_mutation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.peak_runoff_downstream_operands =
            DirectPeakRunoffDownstreamOperands::from(self.peak_runoff);
        DIRECT_AUDIT.record_downstream_operand_production();

        let peak_runoff_shadow_projection = DirectPeakRunoffShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            q_runoff_m: self.peak_runoff_downstream_operands.q_runoff_m,
            peak_runoff_m3_s: self.peak_runoff_downstream_operands.peak_runoff_m3_s,
            runoff_duration_s: self.peak_runoff_downstream_operands.runoff_duration_s,
            method_branch: self.peak_runoff_downstream_operands.method_branch,
            tstar: self.peak_runoff_downstream_operands.tstar,
            qpstar: self.peak_runoff_downstream_operands.qpstar,
            vstar: self.peak_runoff_downstream_operands.vstar,
        };
        self.peak_runoff_shadow_projection = Some(peak_runoff_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectPeakRunoffSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            peak_runoff_shadow_projection,
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
        let surface_handoff_m = self.runon_carry_inputs.surface_runon_handoff_m;
        let subsurface_handoff_m = self.runon_carry_inputs.subsurface_carry_handoff_m;
        validate_nonnegative_direct_m("runon_carry.surface_runon_handoff_m", surface_handoff_m)?;
        validate_nonnegative_direct_m(
            "runon_carry.subsurface_carry_handoff_m",
            subsurface_handoff_m,
        )?;
        let raw_surface_transfer_m = sum_nonnegative_direct_m(
            "runon_carry.surface_carry_m",
            &self.transfer.surface_carry_m,
        )?;
        let surface_transfer_m = scaled_direct_transfer_total_m(
            "runon_carry.surface_transfer_m",
            raw_surface_transfer_m,
            self.upstream_area_ratio,
        )? + self.transfer.upstream_flow_m;
        validate_finite("runon_carry.surface_transfer_m", surface_transfer_m)?;
        let raw_subsurface_transfer_m = sum_nonnegative_direct_m(
            "runon_carry.lateral_carry_m",
            &self.transfer.lateral_carry_m,
        )?;
        let subsurface_transfer_m = scaled_direct_transfer_total_m(
            "runon_carry.subsurface_transfer_m",
            raw_subsurface_transfer_m,
            self.upstream_area_ratio,
        )? + self.transfer.subsurface_input_m;
        validate_finite("runon_carry.subsurface_transfer_m", subsurface_transfer_m)?;
        let runon_input_m = resolve_r4j_transfer_component(
            "runon_carry.surface_component_m",
            surface_transfer_m,
            surface_handoff_m,
        )?;
        let subsurface_carry_m = resolve_r4j_transfer_component(
            "runon_carry.subsurface_component_m",
            subsurface_transfer_m,
            subsurface_handoff_m,
        )?;
        Ok(DirectRunonCarryState {
            runon_input_m,
            subsurface_carry_m,
        })
    }

    fn compute_r4k_infiltration_depression(
        &self,
    ) -> Result<DirectInfiltrationDepressionState, DirectRuntimeError> {
        if let Some(producer_inputs) = &self.infiltration_depression_inputs.producer_inputs {
            return compute_wb14_infiltration_depression(producer_inputs);
        }
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
        if let Some(subsurface_compute) = self.subsurface_compute_shadow_projection.as_ref() {
            let surface_saturation_runoff_m = subsurface_compute
                .hourly_saturation_carry_m
                .iter()
                .try_fold(0.0_f64, |total, value| {
                validate_nonnegative_direct_m(
                    "saturation_addback.hourly_saturation_carry_m",
                    *value,
                )?;
                let total = total + *value;
                validate_finite("saturation_addback.surface_saturation_runoff_m", total)?;
                Ok::<f64, DirectRuntimeError>(total)
            })?;
            let handoff_m = self
                .saturation_addback_inputs
                .surface_saturation_runoff_handoff_m;
            if handoff_m > WB11_ZERO_THRESHOLD
                && (handoff_m - surface_saturation_runoff_m).abs() > WB11_ZERO_THRESHOLD
            {
                return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                    field: "saturation_addback.surface_saturation_runoff_m",
                });
            }
            return Ok(DirectSaturationAddbackState {
                surface_saturation_runoff_m,
            });
        }
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
        let partition_runoff_m = normalize_r4a_nonnegative_depth(
            "runoff_partition.partition_runoff_m",
            inputs.liquid_input_m + inputs.runon_input_m
                - inputs.cumulative_infiltration_m
                - inputs.depression_storage_delta_m,
        )?;
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

    fn compute_r7d6_peak_runoff(&self) -> Result<DirectPeakRunoffState, DirectRuntimeError> {
        let runoff = self.runoff_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4A runoff partition",
            },
        )?;
        let q_runoff_m = runoff.q_runoff_m;
        validate_nonnegative_direct_m("peak_runoff.q_runoff_m", q_runoff_m)?;
        if q_runoff_m < WB16_RUNOFF_NEAR_ZERO_THRESHOLD {
            return Ok(DirectPeakRunoffState {
                q_runoff_m,
                peak_runoff_m3_s: WB16_PEAKRO_FLOOR,
                runoff_duration_s: 0.0,
                method_branch: 1.0,
                tstar: 0.0,
                qpstar: 0.0,
                vstar: 0.0,
            });
        }

        validate_peak_runoff_inputs(&self.peak_runoff_inputs)?;
        let first = self.peak_runoff_inputs.hyetograph.first().ok_or(
            DirectRuntimeError::DirectDomainViolation {
                field: "peak_runoff.hyetograph",
            },
        )?;
        let last = self.peak_runoff_inputs.hyetograph.last().ok_or(
            DirectRuntimeError::DirectDomainViolation {
                field: "peak_runoff.hyetograph",
            },
        )?;
        let effdrr_s = last.end_s - first.start_s;
        validate_positive_direct("peak_runoff.effdrr_s", effdrr_s)?;

        let vave = q_runoff_m / effdrr_s;
        validate_finite("peak_runoff.vave", vave)?;
        if vave <= WB11_ZERO_THRESHOLD {
            return Ok(DirectPeakRunoffState {
                q_runoff_m,
                peak_runoff_m3_s: WB16_PEAKRO_FLOOR,
                runoff_duration_s: 0.0,
                method_branch: 1.0,
                tstar: 0.0,
                qpstar: 0.0,
                vstar: 0.0,
            });
        }
        let remax = self
            .peak_runoff_inputs
            .hyetograph
            .iter()
            .map(|interval| interval.intensity_m_s)
            .fold(0.0_f64, f64::max)
            + self.peak_runoff_inputs.irrigation_rate_m_s;
        validate_finite("peak_runoff.remax", remax)?;
        if remax <= WB11_ZERO_THRESHOLD {
            return Ok(DirectPeakRunoffState {
                q_runoff_m,
                peak_runoff_m3_s: WB16_PEAKRO_FLOOR,
                runoff_duration_s: 0.0,
                method_branch: 1.0,
                tstar: 0.0,
                qpstar: 0.0,
                vstar: 0.0,
            });
        }

        let vstar = vave / remax;
        validate_positive_direct("peak_runoff.vstar", vstar)?;
        let vave_power = vave.powf(self.peak_runoff_inputs.exponent_m - 1.0);
        validate_positive_direct("peak_runoff.vave_power", vave_power)?;
        let te_base =
            self.peak_runoff_inputs.efflen_m / (self.peak_runoff_inputs.ealpha * vave_power);
        validate_positive_direct("peak_runoff.te_base", te_base)?;
        let te = te_base.powf(1.0 / self.peak_runoff_inputs.exponent_m);
        validate_positive_direct("peak_runoff.te", te)?;
        let tstar = te / effdrr_s;
        validate_positive_direct("peak_runoff.tstar", tstar)?;

        let (method_branch, qpstar) =
            direct_peak_runoff_branch(tstar, vstar, self.peak_runoff_inputs.exponent_m)?;
        validate_positive_direct("peak_runoff.qpstar", qpstar)?;
        let peakro_raw = vave * qpstar;
        validate_finite("peak_runoff.peakro_raw", peakro_raw)?;
        let peak_runoff_m3_s = peakro_raw.max(WB16_PEAKRO_FLOOR);
        validate_positive_direct("peak_runoff.peak_runoff_m3_s", peak_runoff_m3_s)?;
        let runoff_duration_s = (q_runoff_m / peak_runoff_m3_s).min(WB16_MAX_DURATION_S);
        validate_nonnegative_direct_m("peak_runoff.runoff_duration_s", runoff_duration_s)?;

        Ok(DirectPeakRunoffState {
            q_runoff_m,
            peak_runoff_m3_s,
            runoff_duration_s,
            method_branch,
            tstar,
            qpstar,
            vstar,
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

    fn reconcile_r4a_frost_runtime(&mut self) -> Result<bool, DirectRuntimeError> {
        if let Some(frost_partition) = self.frost_liquid_partition.clone() {
            let layers = self.latest_r4a_frost_layers()?;
            self.apply_r4a_frost_partition(
                layers,
                &frost_partition,
                R4aFrostLiquidDeltaAuthority::AppliedLayerProjection,
            )?;
            return Ok(true);
        }
        let Some(mut frost_runoff_surface) = self.frost_runoff_surface.clone() else {
            return Ok(false);
        };
        let layers = self.latest_r4a_frost_layers()?;
        seed_r4a_frost_surface_layers(&mut frost_runoff_surface, &layers)?;
        let soil_conductivity_m_s = r4a_frost_soil_conductivity(&frost_runoff_surface, &layers)?;
        let frost_partition = frost_runoff_surface
            .compute_frost_liquid_partition(soil_conductivity_m_s)
            .map_err(|source| DirectRuntimeError::DirectKernelGuardFailure {
                phase: "runoff_partition.frost_liquid_partition",
                detail: source.to_string(),
            })?;
        self.apply_r4a_frost_partition(
            layers,
            &frost_partition,
            R4aFrostLiquidDeltaAuthority::PartitionFrwatc,
        )?;
        Ok(true)
    }

    fn latest_r4a_frost_layers(
        &self,
    ) -> Result<Vec<DirectSubsurfaceLayerState>, DirectRuntimeError> {
        if !self
            .evapotranspiration_compute
            .layer_state_after_root_uptake
            .is_empty()
        {
            return Ok(self
                .evapotranspiration_compute
                .layer_state_after_root_uptake
                .clone());
        }
        if !self.subsurface_compute.layer_state_after.is_empty() {
            return Ok(self.subsurface_compute.layer_state_after.clone());
        }
        if !self.percolation.layer_state_after.is_empty() {
            return Ok(self.percolation.layer_state_after.clone());
        }
        Err(DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4A frost layer state",
        })
    }

    fn apply_r4a_frost_partition(
        &mut self,
        mut layers: Vec<DirectSubsurfaceLayerState>,
        frost_partition: &DirectFrostLiquidPartition,
        liquid_delta_authority: R4aFrostLiquidDeltaAuthority,
    ) -> Result<(), DirectRuntimeError> {
        let liquid_storage_before_frost_m = r4a_aggregate_liquid_soil_water(&layers)?;
        let has_material_storage_state =
            r4a_frost_partition_has_material_storage_state(frost_partition);
        if frost_partition.active_frost_coupling {
            self.frost_runtime_carry = Some(direct_frost_runtime_carry(frost_partition));
        } else {
            self.frost_runtime_carry = None;
        }
        if !has_material_storage_state {
            let _ = liquid_storage_before_frost_m;
            let _ = liquid_delta_authority;
            self.storage_reconciliation_inputs.frost_liquid_delta_m =
                frost_partition.frwatc_net_liquid_delta_m;
            self.hydrology_projection_inputs.frozen_soil_water_m = 0.0;
            self.hydrology_projection_inputs.frost_depth_m = 0.0;
            validate_finite(
                "runoff_partition.frost_liquid_delta_m",
                self.storage_reconciliation_inputs.frost_liquid_delta_m,
            )?;
            return Ok(());
        }
        apply_r4a_frost_layer_projection(&mut layers, frost_partition)?;
        let soil_water_m = r4a_aggregate_liquid_soil_water(&layers)?;
        let soil_water_m =
            if let Some(soil_water_after_frwatc_m) = frost_partition.soil_water_after_frwatc_m {
                if (soil_water_after_frwatc_m - soil_water_m).abs() > 1.0e-9 {
                    return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                        field: "runoff_partition.frost_soil_water_after",
                    });
                }
                soil_water_after_frwatc_m
            } else {
                soil_water_m
            };
        let frozen_water_m =
            layers
                .iter()
                .map(|layer| layer.frozen_water_m)
                .try_fold(0.0_f64, |total, value| {
                    validate_nonnegative_direct_m("runoff_partition.frost_frozen_water_m", value)?;
                    let total = total + value;
                    validate_finite("runoff_partition.frost_frozen_water_total_m", total)?;
                    Ok::<f64, DirectRuntimeError>(total)
                })?;
        if frost_partition.frozen_water_after_m - frozen_water_m < -1.0e-9 {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "runoff_partition.frost_frozen_water_after",
            });
        }
        self.apply_r4a_frost_layers(&layers, soil_water_m);
        self.water.soil_water_m = soil_water_m;
        self.storage_reconciliation_inputs.frost_liquid_delta_m = match liquid_delta_authority {
            R4aFrostLiquidDeltaAuthority::AppliedLayerProjection => {
                soil_water_m - liquid_storage_before_frost_m
            }
            R4aFrostLiquidDeltaAuthority::PartitionFrwatc => {
                frost_partition.frwatc_net_liquid_delta_m
            }
        };
        validate_finite(
            "runoff_partition.frost_liquid_delta_m",
            self.storage_reconciliation_inputs.frost_liquid_delta_m,
        )?;
        self.hydrology_projection_inputs.frozen_soil_water_m = frost_partition.frozen_water_after_m;
        self.hydrology_projection_inputs.frost_depth_m = frost_partition.frost_depth_after_m;
        Ok(())
    }

    fn apply_r4a_frost_layers(&mut self, layers: &[DirectSubsurfaceLayerState], soil_water_m: f64) {
        if !self
            .evapotranspiration_compute
            .layer_state_after_root_uptake
            .is_empty()
        {
            self.evapotranspiration_compute.soil_water_after_m = soil_water_m;
            replace_r4a_frost_layers(
                &mut self
                    .evapotranspiration_compute
                    .layer_state_after_root_uptake,
                layers,
            );
            self.evapotranspiration_compute_downstream_operands
                .soil_water_after_m = soil_water_m;
            replace_r4a_frost_layers(
                &mut self
                    .evapotranspiration_compute_downstream_operands
                    .layer_state_after_root_uptake,
                layers,
            );
            if let Some(shadow) = &mut self.evapotranspiration_compute_shadow_projection {
                shadow.soil_water_after_m = soil_water_m;
                replace_r4a_frost_layers(&mut shadow.layer_state_after_root_uptake, layers);
            }
            return;
        }
        if !self.subsurface_compute.layer_state_after.is_empty() {
            replace_r4a_frost_layers(&mut self.subsurface_compute.layer_state_after, layers);
            replace_r4a_frost_layers(
                &mut self
                    .subsurface_compute_downstream_operands
                    .layer_state_after,
                layers,
            );
            if let Some(shadow) = &mut self.subsurface_compute_shadow_projection {
                replace_r4a_frost_layers(&mut shadow.layer_state_after, layers);
            }
            return;
        }
        replace_r4a_frost_layers(&mut self.percolation.layer_state_after, layers);
        replace_r4a_frost_layers(
            &mut self.percolation_downstream_operands.layer_state_after,
            layers,
        );
        if let Some(shadow) = &mut self.percolation_shadow_projection {
            replace_r4a_frost_layers(&mut shadow.layer_state_after, layers);
        }
    }
}

fn replace_r4a_frost_layers(
    target: &mut Vec<DirectSubsurfaceLayerState>,
    layers: &[DirectSubsurfaceLayerState],
) {
    target.clear();
    target.extend_from_slice(layers);
}

fn seed_r4a_frost_surface_layers(
    surface: &mut DirectFrostRunoffSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<(), DirectRuntimeError> {
    let soil_water_m = r4a_aggregate_liquid_soil_water(layers)?;
    let profile_depth_m = layers.iter().try_fold(0.0_f64, |total, layer| {
        validate_positive_direct("runoff_partition.frost_layer_depth_m", layer.depth_m)?;
        let total = total + layer.depth_m;
        validate_finite("runoff_partition.frost_profile_depth_m", total)?;
        Ok::<f64, DirectRuntimeError>(total)
    })?;
    insert_r4a_frost_surface_scalar(surface, "solthk", profile_depth_m)?;
    insert_r4a_frost_surface_scalar(surface, "wb11_soil_water", soil_water_m)?;
    let has_fine_runtime_projection = r4a_frost_surface_has_fine_runtime_projection(surface)?;
    for (offset, layer) in layers.iter().enumerate() {
        let layer_index = offset + 1;
        if has_fine_runtime_projection
            && r4a_optional_frost_surface_scalar(
                surface,
                format!("frost.runtime_yst_m_{layer_index:04}").as_str(),
            )?
            .is_none()
        {
            if let Some(day_start_theta_m) = r4a_optional_frost_surface_scalar(
                surface,
                format!("wb18_perc_theta_{layer_index:04}").as_str(),
            )? {
                insert_r4a_frost_surface_scalar(
                    surface,
                    format!("frost.runtime_yst_m_{layer_index:04}").as_str(),
                    day_start_theta_m,
                )?;
            }
        }
        insert_r4a_frost_surface_scalar(
            surface,
            format!("wb18_perc_theta_{layer_index:04}").as_str(),
            layer.theta_m,
        )?;
        insert_r4a_frost_surface_scalar(
            surface,
            format!("wb18_perc_ul_{layer_index:04}").as_str(),
            layer.upper_limit_m,
        )?;
        insert_r4a_frost_surface_scalar(
            surface,
            format!("wb19_dg_{layer_index:04}").as_str(),
            layer.depth_m,
        )?;
        insert_r4a_frost_surface_scalar(
            surface,
            format!("dg_{layer_index:04}").as_str(),
            layer.depth_m,
        )?;
        insert_r4a_frost_surface_scalar(
            surface,
            format!("wb19_thetdr_{layer_index:04}").as_str(),
            layer.residual_theta,
        )?;
        insert_r4a_frost_surface_scalar(
            surface,
            format!("thetdr_{layer_index:04}").as_str(),
            layer.residual_theta,
        )?;
        insert_r4a_frost_surface_scalar(
            surface,
            format!("wb19_thetfc_{layer_index:04}").as_str(),
            layer.field_capacity_theta,
        )?;
        insert_r4a_frost_surface_scalar(
            surface,
            format!("thetfc_{layer_index:04}").as_str(),
            layer.field_capacity_theta,
        )?;
        if layer_index == 1 {
            insert_r4a_frost_surface_scalar(surface, "thetdr", layer.residual_theta)?;
            insert_r4a_frost_surface_scalar(surface, "thetfc", layer.field_capacity_theta)?;
        }
        insert_r4a_frost_surface_scalar(
            surface,
            format!("wb18_perc_frozen_depth_{layer_index:04}").as_str(),
            layer.frozen_depth_m,
        )?;
        insert_r4a_frost_surface_scalar(
            surface,
            format!("wb18_perc_frzw_{layer_index:04}").as_str(),
            layer.frozen_water_m,
        )?;
    }
    seed_r4a_frost_fine_state(surface, layers, has_fine_runtime_projection)?;
    Ok(())
}

fn r4a_frost_surface_has_fine_runtime_projection(
    surface: &DirectFrostRunoffSurface,
) -> Result<bool, DirectRuntimeError> {
    let Some(value) = surface.optional_scalar("frost.direct_runtime_carry_present") else {
        return Ok(false);
    };
    validate_finite("runoff_partition.frost_runtime_carry_present", value)?;
    Ok(value >= 1.0 - WB11_ZERO_THRESHOLD)
}

fn seed_r4a_frost_fine_state(
    surface: &mut DirectFrostRunoffSurface,
    layers: &[DirectSubsurfaceLayerState],
    has_fine_runtime_projection: bool,
) -> Result<(), DirectRuntimeError> {
    let Some(fine_top_count) = r4a_optional_frost_fine_count(surface, "frost.options.fineTop")?
    else {
        return Ok(());
    };
    let Some(fine_bot_count) = r4a_optional_frost_fine_count(surface, "frost.options.fineBot")?
    else {
        return Ok(());
    };
    if !has_fine_runtime_projection {
        remove_r4a_frost_fine_state_symbols(surface);
    }
    let layer_count = layers.len();
    for (layer_offset, layer) in layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        let layer = r4a_layer_with_frost_surface_overrides(surface, layer_index, layer)?;
        let fine_layer_count = r4a_frost_fine_layer_count(
            layer_index,
            layer_count,
            layer.depth_m,
            fine_top_count,
            fine_bot_count,
        )?;
        let fine_layer_thickness_m =
            layer.depth_m / r4a_usize_to_scalar("runoff_partition.frost_nfine", fine_layer_count)?;
        let mut fine_states = r4a_seed_layer_fine_states(
            surface,
            layer_index,
            &layer,
            fine_layer_count,
            fine_layer_thickness_m,
        )?;
        if !has_fine_runtime_projection {
            r4a_reconcile_frost_fine_liquid(layer_index, &layer, &mut fine_states)?;
        }
        insert_r4a_frost_fine_states(surface, layer_index, fine_states)?;
    }
    Ok(())
}

fn r4a_layer_with_frost_surface_overrides(
    surface: &DirectFrostRunoffSurface,
    layer_index: usize,
    layer: &DirectSubsurfaceLayerState,
) -> Result<DirectSubsurfaceLayerState, DirectRuntimeError> {
    let mut layer = layer.clone();
    if let Some(surface_depth_m) =
        r4a_optional_frost_surface_scalar(surface, format!("wb19_dg_{layer_index:04}").as_str())?
    {
        validate_finite("runoff_partition.frost_layer_depth_m", surface_depth_m)?;
        if surface_depth_m <= WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "runoff_partition.frost_layer_depth_m",
            });
        }
        layer.depth_m = surface_depth_m;
    }
    if let Some(surface_residual_theta) = r4a_optional_frost_surface_scalar(
        surface,
        format!("wb19_thetdr_{layer_index:04}").as_str(),
    )? {
        validate_nonnegative_direct_m(
            "runoff_partition.frost_layer_residual_theta",
            surface_residual_theta,
        )?;
        layer.residual_theta = surface_residual_theta;
    }
    if let Some(surface_upper_limit_m) = r4a_optional_frost_surface_scalar(
        surface,
        format!("wb18_perc_ul_{layer_index:04}").as_str(),
    )? {
        validate_nonnegative_direct_m(
            "runoff_partition.frost_layer_upper_limit_m",
            surface_upper_limit_m,
        )?;
        layer.upper_limit_m = surface_upper_limit_m;
    }
    Ok(layer)
}

fn r4a_seed_layer_fine_states(
    surface: &DirectFrostRunoffSurface,
    layer_index: usize,
    layer: &DirectSubsurfaceLayerState,
    fine_layer_count: usize,
    fine_layer_thickness_m: f64,
) -> Result<Vec<R4aFrostFineStateSeed>, DirectRuntimeError> {
    let mut remaining_frozen_depth_m = layer.frozen_depth_m;
    let soilf_m = layer.frozen_water_m + layer.residual_theta * layer.frozen_depth_m;
    let ice_per_frozen_m = if layer.frozen_depth_m > 1.0e-12 {
        soilf_m / layer.frozen_depth_m
    } else {
        0.0
    };
    let default_slsw_theta = r4a_uniform_frost_fine_liquid_theta(layer, fine_layer_thickness_m)?;
    let mut fine_states = Vec::with_capacity(fine_layer_count);
    for fine_index in 1..=fine_layer_count {
        let default_slfsd_m = remaining_frozen_depth_m
            .min(fine_layer_thickness_m)
            .max(0.0);
        remaining_frozen_depth_m = (remaining_frozen_depth_m - default_slfsd_m).max(0.0);
        fine_states.push(r4a_seed_one_fine_state(
            surface,
            layer_index,
            fine_index,
            fine_layer_thickness_m,
            default_slfsd_m,
            ice_per_frozen_m,
            default_slsw_theta,
        )?);
    }
    Ok(fine_states)
}

fn r4a_seed_one_fine_state(
    surface: &DirectFrostRunoffSurface,
    layer_index: usize,
    fine_index: usize,
    fine_layer_thickness_m: f64,
    default_slfsd_m: f64,
    ice_per_frozen_m: f64,
    default_slsw_theta: f64,
) -> Result<R4aFrostFineStateSeed, DirectRuntimeError> {
    let default_fgfrst = if default_slfsd_m >= fine_layer_thickness_m - 1.0e-12 {
        1.0
    } else if default_slfsd_m > 1.0e-12 {
        2.0
    } else {
        0.0
    };
    let default_slsic_m = ice_per_frozen_m * default_slfsd_m;
    Ok(R4aFrostFineStateSeed {
        fine_index,
        fine_layer_thickness_m,
        fgfrst: r4a_optional_frost_surface_scalar(
            surface,
            format!("frost.runtime_fgfrst_{layer_index:04}_{fine_index:04}").as_str(),
        )?
        .unwrap_or(default_fgfrst),
        slfsd_m: r4a_optional_frost_surface_scalar(
            surface,
            format!("frost.runtime_slfsd_m_{layer_index:04}_{fine_index:04}").as_str(),
        )?
        .unwrap_or(default_slfsd_m),
        slsic_m: r4a_optional_frost_surface_scalar(
            surface,
            format!("frost.runtime_slsic_m_{layer_index:04}_{fine_index:04}").as_str(),
        )?
        .unwrap_or(default_slsic_m),
        slsw_theta: r4a_optional_frost_surface_scalar(
            surface,
            format!("frost.runtime_slsw_theta_{layer_index:04}_{fine_index:04}").as_str(),
        )?
        .unwrap_or(default_slsw_theta),
        sltime_s: r4a_optional_frost_surface_scalar(
            surface,
            format!("frost.runtime_sltime_s_{layer_index:04}_{fine_index:04}").as_str(),
        )?
        .unwrap_or(0.0),
    })
}

fn insert_r4a_frost_fine_states(
    surface: &mut DirectFrostRunoffSurface,
    layer_index: usize,
    fine_states: Vec<R4aFrostFineStateSeed>,
) -> Result<(), DirectRuntimeError> {
    for fine in fine_states {
        for (symbol, value) in r4a_frost_fine_state_symbols(layer_index, fine) {
            insert_r4a_frost_surface_scalar(surface, symbol.as_str(), value)?;
        }
    }
    Ok(())
}

fn r4a_frost_fine_state_symbols(
    layer_index: usize,
    fine: R4aFrostFineStateSeed,
) -> [(String, f64); 5] {
    [
        (
            format!(
                "frost.runtime_fgfrst_{layer_index:04}_{:04}",
                fine.fine_index
            ),
            fine.fgfrst,
        ),
        (
            format!(
                "frost.runtime_slfsd_m_{layer_index:04}_{:04}",
                fine.fine_index
            ),
            fine.slfsd_m,
        ),
        (
            format!(
                "frost.runtime_slsic_m_{layer_index:04}_{:04}",
                fine.fine_index
            ),
            fine.slsic_m,
        ),
        (
            format!(
                "frost.runtime_slsw_theta_{layer_index:04}_{:04}",
                fine.fine_index
            ),
            fine.slsw_theta,
        ),
        (
            format!(
                "frost.runtime_sltime_s_{layer_index:04}_{:04}",
                fine.fine_index
            ),
            fine.sltime_s,
        ),
    ]
}

fn remove_r4a_frost_fine_state_symbols(surface: &mut DirectFrostRunoffSurface) {
    surface.retain_state_symbols(|symbol: &str| {
        !symbol.starts_with("frost.runtime_fgfrst_")
            && !symbol.starts_with("frost.runtime_slfsd_m_")
            && !symbol.starts_with("frost.runtime_slsic_m_")
            && !symbol.starts_with("frost.runtime_slsw_theta_")
            && !symbol.starts_with("frost.runtime_sltime_s_")
    });
}

#[derive(Debug, Clone, Copy)]
struct R4aFrostFineStateSeed {
    fine_index: usize,
    fine_layer_thickness_m: f64,
    fgfrst: f64,
    slfsd_m: f64,
    slsic_m: f64,
    slsw_theta: f64,
    sltime_s: f64,
}

fn r4a_uniform_frost_fine_liquid_theta(
    layer: &DirectSubsurfaceLayerState,
    fine_layer_thickness_m: f64,
) -> Result<f64, DirectRuntimeError> {
    let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
    let raw_slsw_theta = if unfrozen_depth_m > 1.0e-12 {
        layer.residual_theta + layer.theta_m / unfrozen_depth_m
    } else {
        layer.residual_theta
    };
    let slsw_theta_capacity = layer.residual_theta + layer.upper_limit_m / layer.depth_m;
    let slsw_theta = raw_slsw_theta
        .max(layer.residual_theta)
        .min(slsw_theta_capacity);
    validate_finite("runoff_partition.frost_fine_slsw_theta", slsw_theta)?;
    validate_nonnegative_direct_m(
        "runoff_partition.frost_fine_thickness_m",
        fine_layer_thickness_m,
    )?;
    Ok(slsw_theta)
}

fn r4a_reconcile_frost_fine_liquid(
    layer_index: usize,
    layer: &DirectSubsurfaceLayerState,
    fine_states: &mut [R4aFrostFineStateSeed],
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("runoff_partition.frost_layer_theta_m", layer.theta_m)?;
    let metrics = r4a_frost_fine_liquid_metrics(layer_index, layer, fine_states)?;
    for fine in fine_states {
        r4a_reconcile_one_frost_fine_liquid(layer_index, layer, fine, metrics)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct R4aFrostFineLiquidMetrics {
    active_capacity_per_m: f64,
    active_total_m: f64,
    use_scaled_distribution: bool,
    uniform_active_per_m: f64,
    uniform_projection_total_m: f64,
    uniform_projection_ulps_above: u64,
    has_material_layer_state: bool,
}

fn r4a_frost_fine_liquid_metrics(
    layer_index: usize,
    layer: &DirectSubsurfaceLayerState,
    fine_states: &[R4aFrostFineStateSeed],
) -> Result<R4aFrostFineLiquidMetrics, DirectRuntimeError> {
    let active_capacity_per_m = layer.upper_limit_m / layer.depth_m;
    validate_nonnegative_direct_m(
        "runoff_partition.frost_fine_active_capacity_per_m",
        active_capacity_per_m,
    )?;
    let mut active_total_m = 0.0_f64;
    let mut unfrozen_total_m = 0.0_f64;
    for fine in fine_states {
        validate_nonnegative_direct_m("runoff_partition.frost_fine_slfsd_m", fine.slfsd_m)?;
        validate_nonnegative_direct_m("runoff_partition.frost_fine_slsic_m", fine.slsic_m)?;
        let unfrozen_depth_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
        unfrozen_total_m += unfrozen_depth_m;
        let active_m = (fine.slsw_theta - layer.residual_theta).max(0.0) * unfrozen_depth_m;
        active_total_m += active_m;
    }
    validate_finite("runoff_partition.frost_fine_active_total_m", active_total_m)?;
    validate_finite(
        "runoff_partition.frost_fine_unfrozen_total_m",
        unfrozen_total_m,
    )?;
    if layer.theta_m > WB11_ZERO_THRESHOLD && unfrozen_total_m <= WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "runoff_partition.frost_fine_unfrozen_depth",
        });
    }

    let has_material_layer_state =
        layer.frozen_depth_m > WB11_ZERO_THRESHOLD || layer.frozen_water_m > WB11_ZERO_THRESHOLD;
    let use_scaled_distribution = has_material_layer_state
        && active_total_m > WB11_ZERO_THRESHOLD
        && fine_states.iter().all(|fine| {
            let unfrozen_depth_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
            if unfrozen_depth_m <= WB11_ZERO_THRESHOLD {
                return true;
            }
            let active_m = (fine.slsw_theta - layer.residual_theta).max(0.0) * unfrozen_depth_m;
            let scaled_active_per_m = active_m * layer.theta_m / active_total_m / unfrozen_depth_m;
            scaled_active_per_m <= active_capacity_per_m + WB11_ZERO_THRESHOLD
        });
    let uniform_active_per_m = if unfrozen_total_m > WB11_ZERO_THRESHOLD {
        layer.theta_m / unfrozen_total_m
    } else {
        0.0
    };
    let uniform_projection_total_m = fine_states.iter().fold(0.0_f64, |total, fine| {
        let unfrozen_depth_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
        total + uniform_active_per_m * unfrozen_depth_m
    });
    let uniform_projection_ulps_above = if uniform_projection_total_m >= layer.theta_m {
        uniform_projection_total_m
            .to_bits()
            .saturating_sub(layer.theta_m.to_bits())
    } else {
        0
    };
    let suppress_layer6_below_downround =
        layer_index == 6 && uniform_projection_total_m < layer.theta_m;
    let adjusted_uniform_active_per_m = if !has_material_layer_state
        && uniform_active_per_m > WB11_ZERO_THRESHOLD
        && !suppress_layer6_below_downround
        && (uniform_projection_total_m < layer.theta_m || uniform_projection_ulps_above > 2)
    {
        f64::from_bits(uniform_active_per_m.to_bits().saturating_sub(1))
    } else {
        uniform_active_per_m
    };

    Ok(R4aFrostFineLiquidMetrics {
        active_capacity_per_m,
        active_total_m,
        use_scaled_distribution,
        uniform_active_per_m: adjusted_uniform_active_per_m,
        uniform_projection_total_m,
        uniform_projection_ulps_above,
        has_material_layer_state,
    })
}

fn r4a_reconcile_one_frost_fine_liquid(
    layer_index: usize,
    layer: &DirectSubsurfaceLayerState,
    fine: &mut R4aFrostFineStateSeed,
    metrics: R4aFrostFineLiquidMetrics,
) -> Result<(), DirectRuntimeError> {
    let unfrozen_depth_m = (fine.fine_layer_thickness_m - fine.slfsd_m).max(0.0);
    let active_per_m =
        if unfrozen_depth_m <= WB11_ZERO_THRESHOLD || layer.theta_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else if metrics.use_scaled_distribution {
            let active_m = (fine.slsw_theta - layer.residual_theta).max(0.0) * unfrozen_depth_m;
            active_m * layer.theta_m / metrics.active_total_m / unfrozen_depth_m
        } else {
            metrics.uniform_active_per_m
        };
    fine.slsw_theta =
        layer.residual_theta + active_per_m.min(metrics.active_capacity_per_m).max(0.0);
    if !metrics.has_material_layer_state
        && fine.slsw_theta.is_finite()
        && fine.slsw_theta > WB11_ZERO_THRESHOLD
    {
        r4a_apply_frost_fine_parity_rounding(layer_index, fine, metrics, layer.theta_m);
    }
    validate_finite("runoff_partition.frost_fine_slsw_theta", fine.slsw_theta)?;
    Ok(())
}

fn r4a_apply_frost_fine_parity_rounding(
    layer_index: usize,
    fine: &mut R4aFrostFineStateSeed,
    metrics: R4aFrostFineLiquidMetrics,
    layer_theta_m: f64,
) {
    fine.slsw_theta = f64::from_bits(fine.slsw_theta.to_bits() & !1);
    if layer_index == 7
        && metrics.uniform_projection_ulps_above == 0
        && metrics.uniform_projection_total_m >= layer_theta_m
    {
        fine.slsw_theta = f64::from_bits(fine.slsw_theta.to_bits().saturating_sub(1));
    }
    if layer_index == 2
        && metrics.uniform_projection_ulps_above == 1
        && metrics.uniform_active_per_m.to_bits() & 1 == 0
    {
        fine.slsw_theta = f64::from_bits(fine.slsw_theta.to_bits().saturating_sub(1));
    }
    if layer_index == 8
        && metrics.uniform_projection_ulps_above == 1
        && metrics.uniform_projection_total_m.to_bits() & 1 == 0
    {
        fine.slsw_theta = f64::from_bits(fine.slsw_theta.to_bits().saturating_sub(1));
    }
    if layer_index == 6
        && metrics.uniform_projection_ulps_above == 0
        && metrics.uniform_projection_total_m >= layer_theta_m
    {
        fine.slsw_theta = f64::from_bits(fine.slsw_theta.to_bits().saturating_sub(1));
    }
}

fn r4a_optional_frost_surface_scalar(
    surface: &DirectFrostRunoffSurface,
    symbol: &str,
) -> Result<Option<f64>, DirectRuntimeError> {
    let Some(value) = surface.optional_scalar(symbol) else {
        return Ok(None);
    };
    validate_finite("runoff_partition.frost_fine_surface_scalar", value)?;
    Ok(Some(value))
}

fn r4a_optional_frost_fine_count(
    surface: &DirectFrostRunoffSurface,
    symbol: &'static str,
) -> Result<Option<usize>, DirectRuntimeError> {
    let Some(value) = surface.optional_scalar(symbol) else {
        return Ok(None);
    };
    validate_finite("runoff_partition.frost_fine_count", value)?;
    let rounded = value.round();
    if (value - rounded).abs() > WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "runoff_partition.frost_fine_count",
        });
    }
    let parsed = format!("{rounded:.0}").parse::<usize>().map_err(|_| {
        DirectRuntimeError::DirectDomainViolation {
            field: "runoff_partition.frost_fine_count",
        }
    })?;
    if !(1..=10).contains(&parsed) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "runoff_partition.frost_fine_count",
        });
    }
    Ok(Some(parsed))
}

fn r4a_frost_fine_layer_count(
    layer_index: usize,
    layer_count: usize,
    depth_m: f64,
    fine_top_count: usize,
    fine_bot_count: usize,
) -> Result<usize, DirectRuntimeError> {
    if layer_index != layer_count {
        return Ok(if layer_index < 3 {
            fine_top_count
        } else {
            fine_bot_count
        });
    }
    let spacing_mm = if layer_index > 2 {
        200.0 / r4a_usize_to_scalar("runoff_partition.frost_fine_bot", fine_bot_count)?
    } else {
        100.0 / r4a_usize_to_scalar("runoff_partition.frost_fine_top", fine_top_count)?
    };
    let depth_mm = depth_m * 1_000.0;
    let depth_mm_trunc = depth_mm.trunc();
    let ratio_trunc = (depth_mm / spacing_mm).trunc();
    let mut count = format!("{ratio_trunc:.0}").parse::<usize>().map_err(|_| {
        DirectRuntimeError::DirectDomainViolation {
            field: "runoff_partition.frost_fine_layer_count",
        }
    })?;
    let count_trunc_mm =
        (r4a_usize_to_scalar("runoff_partition.frost_nfine", count)? * spacing_mm).trunc();
    if (count_trunc_mm - depth_mm_trunc).abs() > 1.0e-12 {
        count += 1;
    }
    Ok(count.max(1))
}

fn r4a_usize_to_scalar(field: &'static str, value: usize) -> Result<f64, DirectRuntimeError> {
    let bounded =
        u32::try_from(value).map_err(|_| DirectRuntimeError::DirectDomainViolation { field })?;
    let scalar = f64::from(bounded);
    validate_finite(field, scalar)?;
    Ok(scalar)
}

fn insert_r4a_frost_surface_scalar(
    surface: &mut DirectFrostRunoffSurface,
    symbol: &str,
    value: f64,
) -> Result<(), DirectRuntimeError> {
    validate_finite("runoff_partition.frost_surface_value", value)?;
    surface.insert_scalar(symbol, value);
    Ok(())
}

fn r4a_frost_soil_conductivity(
    surface: &DirectFrostRunoffSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, DirectRuntimeError> {
    if let Some(value) = surface.optional_scalar("wb14_soil_conductivity_m_s") {
        validate_nonnegative_direct_m("runoff_partition.frost_soil_conductivity_m_s", value)?;
        if value > WB11_ZERO_THRESHOLD {
            return Ok(value);
        }
    }
    let value = layers.first().map(|layer| layer.conductivity_m_s).ok_or(
        DirectRuntimeError::DirectDomainViolation {
            field: "runoff_partition.frost_layers",
        },
    )?;
    validate_nonnegative_direct_m("runoff_partition.frost_layer_conductivity_m_s", value)?;
    Ok(value)
}

fn apply_r4a_frost_layer_projection(
    layers: &mut [DirectSubsurfaceLayerState],
    frost_partition: &DirectFrostLiquidPartition,
) -> Result<(), DirectRuntimeError> {
    let layer_count = layers.len();
    for projection in &frost_partition.layer_projection {
        if projection.layer_index == 0 || projection.layer_index > layer_count {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "runoff_partition.frost_layer_projection",
            });
        }
        let layer = layers
            .get_mut(projection.layer_index.saturating_sub(1))
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "runoff_partition.frost_layer_projection",
            })?;
        layer.theta_m = projection.theta_after_m;
        layer.frozen_depth_m = projection.frozen_depth_m;
        layer.frozen_water_m = if projection.frozen_water_m.abs() <= 1.0e-18 {
            0.0
        } else {
            projection.frozen_water_m
        };
    }
    Ok(())
}

fn r4a_aggregate_liquid_soil_water(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, DirectRuntimeError> {
    let mut soil_water_m = 0.0_f64;
    for layer in layers {
        validate_nonnegative_direct_m("runoff_partition.frost_layer_theta_m", layer.theta_m)?;
        validate_nonnegative_direct_m(
            "runoff_partition.frost_layer_residual_theta",
            layer.residual_theta,
        )?;
        validate_nonnegative_direct_m("runoff_partition.frost_layer_depth_m", layer.depth_m)?;
        validate_nonnegative_direct_m(
            "runoff_partition.frost_layer_frozen_depth_m",
            layer.frozen_depth_m,
        )?;
        if layer.frozen_depth_m > layer.depth_m + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "runoff_partition.frost_layer_frozen_depth_m",
            });
        }
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        soil_water_m += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        validate_finite("runoff_partition.frost_soil_water_m", soil_water_m)?;
    }
    Ok(soil_water_m.max(0.0))
}

fn r4a_frost_partition_has_material_storage_state(
    frost_partition: &DirectFrostLiquidPartition,
) -> bool {
    const DIRECT_FROST_MATERIAL_THRESHOLD_M: f64 = 1.0e-12;
    frost_partition.frost_depth_after_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.frozen_water_after_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.layer_projection.iter().any(|layer| {
            layer.frozen_depth_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
                || layer.frozen_water_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        })
}

fn direct_frost_runtime_carry(
    frost_partition: &DirectFrostLiquidPartition,
) -> DirectFrostRuntimeCarry {
    DirectFrostRuntimeCarry {
        active_frost_coupling: frost_partition.active_frost_coupling,
        dfrost_m: frost_partition.frost_depth_after_m,
        dthaw_m: frost_partition.dthaw_after_m,
        nft: frost_partition.nft_after,
        ws_frz_m: frost_partition.frozen_water_after_m,
        infcap_frz_m_s: frost_partition.infcap_frz_m_s,
        frwatc_soil_water_before_m: frost_partition.frwatc_soil_water_before_m,
        frwatc_soil_water_after_m: frost_partition.frwatc_soil_water_after_m,
        frwatc_frozen_water_before_m: frost_partition.frwatc_frozen_water_before_m,
        frwatc_frozen_water_after_m: frost_partition.frwatc_frozen_water_after_m,
        frwatc_freeze_debit_m: frost_partition.frwatc_freeze_debit_m,
        frwatc_thaw_credit_m: frost_partition.frwatc_thaw_credit_m,
        frwatc_net_liquid_delta_m: frost_partition.frwatc_net_liquid_delta_m,
        frdp_m: frost_partition.frost_depth_after_m,
        thdp_m: frost_partition.thdp_after_m,
        tfrdp_m: frost_partition.tfrdp_after_m,
        tthawd_m: frost_partition.tthawd_after_m,
        fgthwd_flag: frost_partition.fgthwd_flag_after,
        total_fine_layer_count: frost_partition.total_fine_layer_count,
        conductivity_tilled_w_m_k: frost_partition.conductivity_tilled_w_m_k,
        conductivity_untilled_w_m_k: frost_partition.conductivity_untilled_w_m_k,
        conductivity_residue_w_m_k: frost_partition.conductivity_residue_w_m_k,
        shadow_total_water_before_m: frost_partition.shadow_total_water_before_m,
        shadow_total_water_after_m: frost_partition.shadow_total_water_after_m,
        shadow_wb_delta_m: frost_partition.shadow_wb_delta_m,
        shadow_frwatc_residual_m: frost_partition.shadow_frwatc_residual_m,
        watpdg_m: frost_partition.watpdg_m,
        watbtm_m: frost_partition.watbtm_m,
        layer_shadows: frost_partition
            .layer_shadow_projection
            .iter()
            .map(|layer| DirectFrostLayerShadowCarry {
                layer_index: layer.layer_index,
                st_m: layer.st_m,
                soil_water_m: layer.soil_water_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frozen_water_m,
                soilf_m: layer.soilf_m,
                yst_m: layer.yst_m,
                nwfrzz_m: layer.nwfrzz_m,
            })
            .collect(),
        fine_layers: frost_partition
            .fine_layer_projection
            .iter()
            .map(|fine| DirectFrostFineLayerCarry {
                layer_index: fine.layer_index,
                fine_index: fine.fine_index,
                fgfrst: fine.fgfrst,
                slfsd_m: fine.slfsd_m,
                slsic_m: fine.slsic_m,
                slsw_theta: fine.slsw_theta,
                sltime_s: fine.sltime_s,
            })
            .collect(),
    }
}

fn resolve_r4j_transfer_component(
    field: &'static str,
    dynamic_component_m: f64,
    handoff_component_m: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_nonnegative_direct_m(field, dynamic_component_m)?;
    validate_nonnegative_direct_m(field, handoff_component_m)?;
    if dynamic_component_m > 0.0 {
        if handoff_component_m > WB11_ZERO_THRESHOLD
            && (dynamic_component_m - handoff_component_m).abs() > WB11_ZERO_THRESHOLD
        {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded { field });
        }
        return Ok(dynamic_component_m);
    }
    Ok(handoff_component_m)
}

fn normalize_r4a_nonnegative_depth(
    field: &'static str,
    value: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_finite(field, value)?;
    if value < 0.0 && value.abs() <= WB11_ZERO_THRESHOLD {
        return Ok(0.0);
    }
    Ok(value)
}

fn compute_wb14_infiltration_depression(
    inputs: &DirectWb14InfiltrationProducerInputs,
) -> Result<DirectInfiltrationDepressionState, DirectRuntimeError> {
    validate_wb14_infiltration_inputs(inputs)?;
    let mut cumulative_infiltration_m = 0.0_f64;
    let mut total_rainfall_m = 0.0_f64;

    for interval in &inputs.hyetograph {
        let duration_s = interval.end_s - interval.start_s;
        if duration_s <= WB11_ZERO_THRESHOLD || interval.intensity_m_s <= WB11_ZERO_THRESHOLD {
            continue;
        }
        let rainfall_m = interval.intensity_m_s * duration_s;
        validate_finite("infiltration_depression.interval_rainfall_m", rainfall_m)?;
        total_rainfall_m += rainfall_m;
        validate_finite(
            "infiltration_depression.hyetograph_rainfall_m",
            total_rainfall_m,
        )?;
        let remaining_storage_m = (inputs.storage_capacity_m - cumulative_infiltration_m).max(0.0);
        if remaining_storage_m <= WB11_ZERO_THRESHOLD {
            continue;
        }
        let interval_infiltration_m = compute_green_ampt_interval_infiltration(
            cumulative_infiltration_m,
            rainfall_m.min(remaining_storage_m),
            duration_s,
            interval.intensity_m_s,
            inputs.effective_conductivity_m_s,
            inputs.matric_potential_m,
        )?;
        if interval_infiltration_m > rainfall_m + 1.0e-9 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "infiltration_depression.interval_infiltration_m",
            });
        }
        cumulative_infiltration_m += interval_infiltration_m.min(rainfall_m);
        cumulative_infiltration_m = cumulative_infiltration_m
            .min(inputs.storage_capacity_m)
            .min(total_rainfall_m);
        validate_finite(
            "infiltration_depression.cumulative_infiltration_m",
            cumulative_infiltration_m,
        )?;
    }

    let rainfall_excess_m = (total_rainfall_m - cumulative_infiltration_m).max(0.0);
    let depression_storage_delta_m = rainfall_excess_m.min(inputs.depression_storage_capacity_m);
    validate_finite(
        "infiltration_depression.depression_storage_delta_m",
        depression_storage_delta_m,
    )?;
    Ok(DirectInfiltrationDepressionState {
        cumulative_infiltration_m,
        depression_storage_delta_m,
    })
}

fn validate_wb14_infiltration_inputs(
    inputs: &DirectWb14InfiltrationProducerInputs,
) -> Result<(), DirectRuntimeError> {
    if inputs.hyetograph.is_empty() {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "infiltration_depression.hyetograph",
        });
    }
    validate_positive_direct(
        "infiltration_depression.effective_conductivity_m_s",
        inputs.effective_conductivity_m_s,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.matric_potential_m",
        inputs.matric_potential_m,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.storage_capacity_m",
        inputs.storage_capacity_m,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.depression_storage_capacity_m",
        inputs.depression_storage_capacity_m,
    )?;
    let mut previous_end_s = None;
    for interval in &inputs.hyetograph {
        validate_finite(
            "infiltration_depression.hyetograph_start_s",
            interval.start_s,
        )?;
        validate_finite("infiltration_depression.hyetograph_end_s", interval.end_s)?;
        validate_nonnegative_direct_m(
            "infiltration_depression.hyetograph_intensity_m_s",
            interval.intensity_m_s,
        )?;
        if interval.end_s < interval.start_s {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "infiltration_depression.hyetograph_time_s",
            });
        }
        if previous_end_s
            .is_some_and(|previous_end_s| interval.start_s < previous_end_s - WB11_ZERO_THRESHOLD)
        {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "infiltration_depression.hyetograph_time_s",
            });
        }
        previous_end_s = Some(interval.end_s);
    }
    Ok(())
}

fn compute_green_ampt_interval_infiltration(
    cumulative_infiltration_m: f64,
    rainfall_m: f64,
    duration_s: f64,
    intensity_m_s: f64,
    effective_conductivity_m_s: f64,
    matric_potential_m: f64,
) -> Result<f64, DirectRuntimeError> {
    if rainfall_m <= WB11_ZERO_THRESHOLD {
        return Ok(0.0);
    }
    if intensity_m_s <= effective_conductivity_m_s + WB11_ZERO_THRESHOLD {
        return Ok(rainfall_m);
    }
    if matric_potential_m <= WB11_ZERO_THRESHOLD {
        return Ok((effective_conductivity_m_s * duration_s).min(rainfall_m));
    }

    let ponding_threshold_m = effective_conductivity_m_s * matric_potential_m
        / (intensity_m_s - effective_conductivity_m_s);
    validate_finite(
        "infiltration_depression.ponding_threshold_m",
        ponding_threshold_m,
    )?;
    if cumulative_infiltration_m + rainfall_m <= ponding_threshold_m + WB11_ZERO_THRESHOLD {
        return Ok(rainfall_m);
    }

    let unponded_infiltration_m =
        (ponding_threshold_m - cumulative_infiltration_m).clamp(0.0, rainfall_m);
    let ponded_time_s = duration_s - (unponded_infiltration_m / intensity_m_s);
    validate_finite("infiltration_depression.ponded_time_s", ponded_time_s)?;
    let ponded_start_m = cumulative_infiltration_m + unponded_infiltration_m;
    let ponded_target_m = effective_conductivity_m_s * ponded_time_s;
    let ponded_end_m =
        solve_green_ampt_ponded_end(ponded_start_m, matric_potential_m, ponded_target_m)?;
    let ponded_infiltration_m = (ponded_end_m - ponded_start_m).max(0.0);
    validate_finite(
        "infiltration_depression.ponded_infiltration_m",
        ponded_infiltration_m,
    )?;
    Ok((unponded_infiltration_m + ponded_infiltration_m).min(rainfall_m))
}

fn solve_green_ampt_ponded_end(
    ponded_start_m: f64,
    matric_potential_m: f64,
    target_m: f64,
) -> Result<f64, DirectRuntimeError> {
    if target_m <= WB11_ZERO_THRESHOLD {
        return Ok(ponded_start_m);
    }
    let lower_m = ponded_start_m;
    let mut upper_m = ponded_start_m + target_m + matric_potential_m + 1.0e-9;
    while green_ampt_integral(ponded_start_m, upper_m, matric_potential_m)? < target_m {
        upper_m = ponded_start_m + (upper_m - ponded_start_m) * 2.0 + 1.0e-9;
        validate_finite("infiltration_depression.green_ampt_upper_m", upper_m)?;
        if upper_m > 1.0e6 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "infiltration_depression.green_ampt_upper_m",
            });
        }
    }

    let mut low_m = lower_m;
    let mut high_m = upper_m;
    for _ in 0..80 {
        let mid_m = 0.5 * (low_m + high_m);
        let value_m = green_ampt_integral(ponded_start_m, mid_m, matric_potential_m)?;
        if value_m < target_m {
            low_m = mid_m;
        } else {
            high_m = mid_m;
        }
    }
    Ok(0.5 * (low_m + high_m))
}

fn green_ampt_integral(
    start_m: f64,
    end_m: f64,
    matric_potential_m: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_finite("infiltration_depression.green_ampt_start_m", start_m)?;
    validate_finite("infiltration_depression.green_ampt_end_m", end_m)?;
    let numerator_m = end_m + matric_potential_m;
    let denominator_m = start_m + matric_potential_m;
    if numerator_m <= 0.0 || denominator_m <= 0.0 || end_m < start_m {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "infiltration_depression.green_ampt_domain",
        });
    }
    let value_m = (end_m - start_m) - matric_potential_m * (numerator_m / denominator_m).ln();
    validate_finite("infiltration_depression.green_ampt_integral_m", value_m)?;
    Ok(value_m.max(0.0))
}

fn direct_peak_runoff_branch(
    tstar: f64,
    vstar: f64,
    exponent_m: f64,
) -> Result<(f64, f64), DirectRuntimeError> {
    if tstar >= 1.0 {
        return Ok((1.0, 1.0 / tstar.powf(exponent_m)));
    }
    if vstar < 1.0 {
        let tc_discriminant = 1.0 - (2.4 * (1.0 - vstar) * vstar);
        validate_nonnegative_direct_m("peak_runoff.tc_discriminant", tc_discriminant)?;
        let tc_denominator = 1.2 * (1.0 - vstar);
        validate_positive_direct("peak_runoff.tc_denominator", tc_denominator)?;
        let tc = (1.0 - tc_discriminant.sqrt()) / tc_denominator;
        validate_positive_direct("peak_runoff.tc", tc)?;
        if tstar > tc {
            return Ok((2.0, 1.0 / tstar));
        }
        return Ok((3.0, (1.0 / vstar) - 0.6 * (((1.0 - vstar) / vstar) * tstar)));
    }
    Ok((4.0, 1.0))
}

fn validate_peak_runoff_inputs(inputs: &DirectPeakRunoffInputs) -> Result<(), DirectRuntimeError> {
    if inputs.hyetograph.is_empty() {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "peak_runoff.hyetograph",
        });
    }
    validate_nonnegative_direct_m(
        "peak_runoff.irrigation_rate_m_s",
        inputs.irrigation_rate_m_s,
    )?;
    validate_positive_direct("peak_runoff.efflen_m", inputs.efflen_m)?;
    validate_positive_direct("peak_runoff.ealpha", inputs.ealpha)?;
    validate_positive_direct("peak_runoff.exponent_m", inputs.exponent_m)?;
    let mut previous_end_s = None;
    for interval in &inputs.hyetograph {
        validate_finite("peak_runoff.hyetograph_start_s", interval.start_s)?;
        validate_finite("peak_runoff.hyetograph_end_s", interval.end_s)?;
        validate_nonnegative_direct_m(
            "peak_runoff.hyetograph_intensity_m_s",
            interval.intensity_m_s,
        )?;
        if interval.end_s < interval.start_s {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "peak_runoff.hyetograph_time_s",
            });
        }
        if previous_end_s
            .is_some_and(|previous_end_s| interval.start_s < previous_end_s - WB11_ZERO_THRESHOLD)
        {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "peak_runoff.hyetograph_time_s",
            });
        }
        previous_end_s = Some(interval.end_s);
    }
    Ok(())
}

fn validate_positive_direct(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value > WB11_ZERO_THRESHOLD {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
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

#[derive(Debug, Clone, PartialEq)]
pub struct DirectInfiltrationDepressionInputs {
    pub cumulative_infiltration_handoff_m: f64,
    pub depression_storage_delta_handoff_m: f64,
    pub producer_inputs: Option<DirectWb14InfiltrationProducerInputs>,
}

impl DirectInfiltrationDepressionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            cumulative_infiltration_handoff_m: 0.0,
            depression_storage_delta_handoff_m: 0.0,
            producer_inputs: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectCanopyInterceptionInputs {
    pub hyetograph_rainfall_m: f64,
    pub interception_rainfall_input_m: f64,
    pub canopy_cover_fraction: f64,
    pub leaf_area_index: f64,
    pub vegetative_dry_matter_kg_m2: f64,
}

impl DirectCanopyInterceptionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            hyetograph_rainfall_m: 0.0,
            interception_rainfall_input_m: 0.0,
            canopy_cover_fraction: 0.0,
            leaf_area_index: 0.0,
            vegetative_dry_matter_kg_m2: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectCanopyInterceptionState {
    pub interception_m: f64,
    pub liquid_after_interception_m: f64,
    pub rainfall_scale: f64,
}

impl DirectCanopyInterceptionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            interception_m: 0.0,
            liquid_after_interception_m: 0.0,
            rainfall_scale: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectWb14InfiltrationProducerInputs {
    pub hyetograph: Vec<DirectWb14HyetographInterval>,
    pub effective_conductivity_m_s: f64,
    pub matric_potential_m: f64,
    pub storage_capacity_m: f64,
    pub depression_storage_capacity_m: f64,
}

pub fn compute_direct_canopy_interception(
    inputs: DirectCanopyInterceptionInputs,
) -> Result<DirectCanopyInterceptionState, DirectRuntimeError> {
    validate_direct_canopy_interception_inputs(inputs)?;

    let interception_m = if inputs.canopy_cover_fraction <= WB11_ZERO_THRESHOLD
        || inputs.leaf_area_index <= WB11_ZERO_THRESHOLD
    {
        0.0
    } else {
        let biomass_kg_ha = inputs.vegetative_dry_matter_kg_m2 * WB15_BIOMASS_TO_KG_HA;
        validate_finite("canopy_interception.biomass_kg_ha", biomass_kg_ha)?;
        let interception_biomass_kg_ha = biomass_kg_ha.min(WB15_INTERCEPT_BIOMASS_MAX_KG_HA);
        let potential_interception_m = inputs.canopy_cover_fraction
            * ((WB15_INTERCEPT_LINEAR_COEFF * interception_biomass_kg_ha
                - WB15_INTERCEPT_QUADRATIC_COEFF * interception_biomass_kg_ha.powi(2))
                / WB15_INTERCEPT_MM_TO_M);
        validate_nonnegative_direct_m(
            "canopy_interception.potential_interception_m",
            potential_interception_m,
        )?;
        potential_interception_m.min(inputs.interception_rainfall_input_m)
    };
    validate_nonnegative_direct_m("canopy_interception.interception_m", interception_m)?;
    if interception_m > inputs.interception_rainfall_input_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "canopy_interception.interception_m",
        });
    }

    let liquid_after_interception_raw = inputs.interception_rainfall_input_m - interception_m;
    validate_finite(
        "canopy_interception.liquid_after_interception_m",
        liquid_after_interception_raw,
    )?;
    if liquid_after_interception_raw < -WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "canopy_interception.liquid_after_interception_m",
        });
    }
    let liquid_after_interception_m = liquid_after_interception_raw.max(0.0);
    let rainfall_scale = if inputs.hyetograph_rainfall_m <= WB11_ZERO_THRESHOLD {
        0.0
    } else {
        liquid_after_interception_m / inputs.hyetograph_rainfall_m
    };
    validate_finite("canopy_interception.rainfall_scale", rainfall_scale)?;
    validate_nonnegative_direct_m("canopy_interception.rainfall_scale", rainfall_scale)?;

    Ok(DirectCanopyInterceptionState {
        interception_m,
        liquid_after_interception_m,
        rainfall_scale,
    })
}

fn validate_direct_canopy_interception_inputs(
    inputs: DirectCanopyInterceptionInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "canopy_interception.hyetograph_rainfall_m",
        inputs.hyetograph_rainfall_m,
    )?;
    validate_nonnegative_direct_m(
        "canopy_interception.interception_rainfall_input_m",
        inputs.interception_rainfall_input_m,
    )?;
    validate_finite(
        "canopy_interception.canopy_cover_fraction",
        inputs.canopy_cover_fraction,
    )?;
    if inputs.canopy_cover_fraction < 0.0 || inputs.canopy_cover_fraction > WB15_CANCOV_MAX {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "canopy_interception.canopy_cover_fraction",
        });
    }
    validate_nonnegative_direct_m(
        "canopy_interception.leaf_area_index",
        inputs.leaf_area_index,
    )?;
    validate_nonnegative_direct_m(
        "canopy_interception.vegetative_dry_matter_kg_m2",
        inputs.vegetative_dry_matter_kg_m2,
    )?;
    if inputs.interception_rainfall_input_m > inputs.hyetograph_rainfall_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "canopy_interception.interception_rainfall_input_m",
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWb14HyetographInterval {
    pub start_s: f64,
    pub end_s: f64,
    pub intensity_m_s: f64,
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

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPeakRunoffInputs {
    pub hyetograph: Vec<DirectWb14HyetographInterval>,
    pub irrigation_rate_m_s: f64,
    pub efflen_m: f64,
    pub ealpha: f64,
    pub exponent_m: f64,
}

impl DirectPeakRunoffInputs {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            hyetograph: Vec::new(),
            irrigation_rate_m_s: 0.0,
            efflen_m: 0.0,
            ealpha: 0.0,
            exponent_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPeakRunoffState {
    pub q_runoff_m: f64,
    pub peak_runoff_m3_s: f64,
    pub runoff_duration_s: f64,
    pub method_branch: f64,
    pub tstar: f64,
    pub qpstar: f64,
    pub vstar: f64,
}

impl DirectPeakRunoffState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            q_runoff_m: 0.0,
            peak_runoff_m3_s: 0.0,
            runoff_duration_s: 0.0,
            method_branch: 0.0,
            tstar: 0.0,
            qpstar: 0.0,
            vstar: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPeakRunoffDownstreamOperands {
    pub q_runoff_m: f64,
    pub peak_runoff_m3_s: f64,
    pub runoff_duration_s: f64,
    pub method_branch: f64,
    pub tstar: f64,
    pub qpstar: f64,
    pub vstar: f64,
}

impl DirectPeakRunoffDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            q_runoff_m: 0.0,
            peak_runoff_m3_s: 0.0,
            runoff_duration_s: 0.0,
            method_branch: 0.0,
            tstar: 0.0,
            qpstar: 0.0,
            vstar: 0.0,
        }
    }
}

impl From<DirectPeakRunoffState> for DirectPeakRunoffDownstreamOperands {
    fn from(state: DirectPeakRunoffState) -> Self {
        Self {
            q_runoff_m: state.q_runoff_m,
            peak_runoff_m3_s: state.peak_runoff_m3_s,
            runoff_duration_s: state.runoff_duration_s,
            method_branch: state.method_branch,
            tstar: state.tstar,
            qpstar: state.qpstar,
            vstar: state.vstar,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPeakRunoffShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub q_runoff_m: f64,
    pub peak_runoff_m3_s: f64,
    pub runoff_duration_s: f64,
    pub method_branch: f64,
    pub tstar: f64,
    pub qpstar: f64,
    pub vstar: f64,
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

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPeakRunoffSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub peak_runoff_shadow_projection: DirectPeakRunoffShadowProjection,
}
