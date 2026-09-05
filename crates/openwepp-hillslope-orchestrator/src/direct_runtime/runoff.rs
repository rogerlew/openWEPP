use crate::constants::{
    WB11_ZERO_THRESHOLD, WB15_BIOMASS_TO_KG_HA, WB15_CANCOV_MAX, WB15_INTERCEPT_BIOMASS_MAX_KG_HA,
    WB15_INTERCEPT_LINEAR_COEFF, WB15_INTERCEPT_MM_TO_M, WB15_INTERCEPT_QUADRATIC_COEFF,
};
use crate::hydrology::{
    DirectWinterFrostComputeInputs, DirectWinterFrostPartitionOutcome,
    Wb11HydrologyKernelGuardError,
};
use crate::winter_column::DirectFrostLaneState;

pub(super) use super::surface_liquid_wb14::{
    DirectWb14ContinuationIntervalInputs, advance_wb14_continuation_interval,
};
use super::surface_liquid_wb14::{DirectWb14IntervalTransitionInputs, advance_wb14_interval_state};
use super::{
    DIRECT_AUDIT, DIRECT_R4A_PHASE_SPAN_COUNT, DIRECT_R4I_PHASE_SPAN_COUNT,
    DIRECT_R4J_PHASE_SPAN_COUNT, DIRECT_R4K_PHASE_SPAN_COUNT, DIRECT_R4L_PHASE_SPAN_COUNT,
    DIRECT_R7D6_PEAK_RUNOFF_PHASE_SPAN_COUNT, DirectDayFrame, DirectFrostFineLayerCarry,
    DirectFrostLayerShadowCarry, DirectFrostRuntimeCarry, DirectRuntimeError,
    DirectSubsurfaceLayerState, scaled_direct_transfer_total_m, sum_nonnegative_direct_m,
    validate_finite, validate_nonnegative_direct_m,
};

/// Per-interval arithmetic allowance for the 24-bin WB14 ledger
/// (`SC-WATBAL-001#TOL-WATBAL-009`).
const WB14_INTERVAL_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;

fn r7h_runoff_trace_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.17}")
    } else {
        "null".to_string()
    }
}

fn r7h_runoff_append_trace_line(path: &std::path::Path, line: &str) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = std::io::Write::write_all(&mut file, line.as_bytes());
    }
}

// WP-2 frost-solve diagnostic trace: since the single-solve rewire the only
// writer is the runner day-input builder (the R4A-side hook died with the
// deleted re-solve); one JSONL row per (lane, day) solve when
// OPENWEPP_WP2_FROST_PAIR_TRACE_PATH is set. Inert (no construction, no I/O)
// when the variable is unset.
static WP2_FROST_PAIR_TRACE_PATH: std::sync::OnceLock<Option<std::path::PathBuf>> =
    std::sync::OnceLock::new();

pub fn wp2_frost_pair_trace_path() -> Option<&'static std::path::PathBuf> {
    WP2_FROST_PAIR_TRACE_PATH
        .get_or_init(|| {
            let path = std::env::var_os("OPENWEPP_WP2_FROST_PAIR_TRACE_PATH")?;
            if path.is_empty() {
                return None;
            }
            Some(std::path::PathBuf::from(path))
        })
        .as_ref()
}

#[allow(clippy::too_many_arguments)]
pub fn write_wp2_frost_pair_trace(
    path: &std::path::Path,
    source: &str,
    lane_index: usize,
    day_index: usize,
    in_soil_water_m: f64,
    in_layer_theta_sum_m: f64,
    in_layer_frozen_water_sum_m: f64,
    in_prior_dfrost_m: f64,
    in_prior_ws_frz_m: f64,
    outcome: &crate::hydrology::DirectWinterFrostPartitionOutcome,
) {
    let n = r7h_runoff_trace_number;
    let line = format!(
        concat!(
            "{{\"schema\":\"openwepp-wp2-frost-pair-trace-v1\",\"source\":\"{}\",",
            "\"lane_index\":{},\"day_index\":{},",
            "\"in_soil_water_m\":{},\"in_layer_theta_sum_m\":{},",
            "\"in_layer_frozen_water_sum_m\":{},\"in_prior_dfrost_m\":{},",
            "\"in_prior_ws_frz_m\":{},\"out_active\":{},",
            "\"out_infcap_frz_m_s\":{},\"out_frost_depth_after_m\":{},",
            "\"out_frozen_water_after_m\":{},\"out_thdp_after_m\":{},",
            "\"out_dthaw_after_m\":{},\"out_tfrdp_after_m\":{},",
            "\"out_tthawd_after_m\":{},\"out_fgthwd_flag_after\":{},",
            "\"out_frwatc_net_liquid_delta_m\":{},",
            "\"out_total_fine_layer_count\":{}}}\n"
        ),
        source,
        lane_index,
        day_index,
        n(in_soil_water_m),
        n(in_layer_theta_sum_m),
        n(in_layer_frozen_water_sum_m),
        n(in_prior_dfrost_m),
        n(in_prior_ws_frz_m),
        outcome.active_frost_coupling,
        n(outcome.infcap_frz_m_s),
        n(outcome.frost_depth_after_m),
        n(outcome.frozen_water_after_m),
        n(outcome.thdp_after_m),
        n(outcome.dthaw_after_m),
        n(outcome.tfrdp_after_m),
        n(outcome.tthawd_after_m),
        n(outcome.fgthwd_flag_after),
        n(outcome.frwatc_net_liquid_delta_m),
        n(outcome.total_fine_layer_count),
    );
    r7h_runoff_append_trace_line(path, &line);
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
        self.apply_dc01_runon_supply_admission()?;
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4K_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let infiltration_depression =
            if let Some(producer_inputs) = &self.infiltration_depression_inputs.producer_inputs {
                let outcome = compute_wb14_infiltration_depression_with_profile(producer_inputs)?;
                self.wb14_hourly_excess_m = outcome.hourly_excess_m;
                self.wb14_hourly_rainfall_m = outcome.hourly_rainfall_m;
                outcome.state
            } else {
                self.compute_r4k_infiltration_depression()?
            };
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        // WB14 owns both the daily infiltration total and its hourly runoff
        // residual. A later daily-only snow reconstruction cannot increase
        // infiltration without inventing which hourly runoff supply was
        // removed. Routed melt already enters WB14 on its producer clock.
        let same_pass_infiltration_m = infiltration_depression.cumulative_infiltration_m;
        self.infiltration_depression = infiltration_depression;
        self.runoff_partition_inputs.cumulative_infiltration_m =
            infiltration_depression.cumulative_infiltration_m;
        self.runoff_partition_inputs.depression_storage_delta_m =
            infiltration_depression.depression_storage_delta_m;
        self.percolation_inputs.same_pass_infiltration_m = same_pass_infiltration_m;
        self.percolation_inputs.same_pass_infiltration_lineage = true;
        self.evapotranspiration_compute_inputs
            .same_pass_infiltration_m = same_pass_infiltration_m;
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

    pub(crate) fn project_r4x_winter_local_liquid_before_saturation(
        &mut self,
        winter_frost_compute_inputs: Option<&DirectWinterFrostComputeInputs>,
    ) -> Result<(), DirectRuntimeError> {
        if winter_frost_compute_inputs.is_none() {
            return Ok(());
        }
        let deferred_local_liquid_m = self.r4a_deferred_local_partition_excess_m()?;
        if deferred_local_liquid_m <= WB11_ZERO_THRESHOLD {
            return Ok(());
        }
        validate_nonnegative_direct_m(
            "runoff_partition.winter_local_liquid_projection_before_saturation_m",
            deferred_local_liquid_m,
        )?;
        // WB18 and ET consume the WB12/WB14 same-pass infiltration producer
        // lineage. Frost-retained local liquid enters typed storage after
        // surface ET and before saturation/subsurface so WB18 fluxes stay
        // lineage-clean while saturation runoff sees the retained top water.
        self.evapotranspiration_surface.soil_water_after_soil_evap_m += deferred_local_liquid_m;
        project_r4x_local_liquid_to_top_layer(
            &mut self.evapotranspiration_surface.layer_state_after_soil_evap,
            deferred_local_liquid_m,
        )?;
        self.evapotranspiration_surface_downstream_operands
            .soil_water_after_soil_evap_m += deferred_local_liquid_m;
        project_r4x_local_liquid_to_top_layer(
            &mut self
                .evapotranspiration_surface_downstream_operands
                .layer_state_after_soil_evap,
            deferred_local_liquid_m,
        )?;
        let surface_shadow = self
            .evapotranspiration_surface_shadow_projection
            .as_mut()
            .ok_or(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4N surface evapotranspiration",
            })?;
        surface_shadow.soil_water_after_soil_evap_m += deferred_local_liquid_m;
        project_r4x_local_liquid_to_top_layer(
            &mut surface_shadow.layer_state_after_soil_evap,
            deferred_local_liquid_m,
        )?;
        self.water.soil_water_m = self.evapotranspiration_surface.soil_water_after_soil_evap_m;
        self.runoff_partition_inputs
            .frost_preprojected_local_liquid_m = deferred_local_liquid_m;
        Ok(())
    }

    pub fn run_r4a_runoff_partition_span(
        &mut self,
    ) -> Result<DirectRunoffPartitionSpanReport, DirectRuntimeError> {
        self.run_r4a_runoff_partition_span_with_winter_frost(None)
    }

    pub fn run_r4a_runoff_partition_span_with_winter_frost(
        &mut self,
        winter_frost_compute_inputs: Option<&DirectWinterFrostComputeInputs>,
    ) -> Result<DirectRunoffPartitionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4A_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;
        let mut direct_compute_count = 0_u64;
        let mut state_mutation_count = 0_u64;
        let mut downstream_operand_count = 0_u64;
        let mut shadow_projection_count = 0_u64;

        let frost_reconciled = self.reconcile_r4a_frost_runtime(winter_frost_compute_inputs)?;
        if frost_reconciled {
            DIRECT_AUDIT.record_direct_compute_operation();
            direct_compute_count += 1;
            DIRECT_AUDIT.record_direct_state_mutation();
            state_mutation_count += 1;
        }

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
            peak_runoff_rate_m_s: self.peak_runoff_downstream_operands.peak_runoff_rate_m_s,
            runoff_duration_s: self.peak_runoff_downstream_operands.runoff_duration_s,
            peak_hour_index: self.peak_runoff_downstream_operands.peak_hour_index,
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

    /// DC01 (INV-RUNOFFPART-031): distribute the R4J-resolved runon totals
    /// onto hour bins — surface via the published upstream hourly weights,
    /// lateral via the hourly lateral-carry shape — and
    /// inject them into the WB14 producer inputs before the R4K compute.
    /// Using the R4J-resolved totals keeps the partition identity consistent
    /// by construction. Zero-runon (single-OFE) lanes are untouched.
    fn apply_dc01_runon_supply_admission(&mut self) -> Result<(), DirectRuntimeError> {
        let surface_total_m = self.runon_carry_downstream_operands.runon_input_m;
        let lateral_total_m = self.runon_carry_downstream_operands.subsurface_carry_m;
        validate_nonnegative_direct_m("runon_admission.surface_total_m", surface_total_m)?;
        validate_nonnegative_direct_m("runon_admission.lateral_total_m", lateral_total_m)?;
        if surface_total_m == 0.0 && lateral_total_m == 0.0 {
            return Ok(());
        }
        if self
            .infiltration_depression_inputs
            .producer_inputs
            .is_none()
        {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "WB14 producer for positive hourly runon supply",
            });
        }
        let supply = Self::dc01_distribute_runon_supply(
            surface_total_m,
            lateral_total_m,
            &self.transfer.surface_hourly_weights,
            &self.transfer.lateral_carry_m,
        )?;
        if let Some(producer_inputs) = &mut self.infiltration_depression_inputs.producer_inputs {
            for (hourly_supply_m, runon_supply_m) in producer_inputs
                .hourly_additional_supply_m
                .iter_mut()
                .zip(supply)
            {
                validate_nonnegative_direct_m(
                    "infiltration_depression.hourly_additional_supply_m",
                    *hourly_supply_m,
                )?;
                *hourly_supply_m += runon_supply_m;
                validate_finite(
                    "infiltration_depression.hourly_additional_supply_m",
                    *hourly_supply_m,
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn dc01_distribute_runon_supply(
        surface_total_m: f64,
        lateral_total_m: f64,
        surface_hourly_weights: &[f64; DC01_HOUR_BIN_COUNT],
        lateral_carry_m: &[f64; DC01_HOUR_BIN_COUNT],
    ) -> Result<[f64; DC01_HOUR_BIN_COUNT], DirectRuntimeError> {
        // (shared-shape note: the SURFACE weights consumed here are produced
        // by `dc01_surface_runoff_hourly_weights` below — one authority.)
        validate_nonnegative_direct_m("runon_admission.surface_total_m", surface_total_m)?;
        validate_nonnegative_direct_m("runon_admission.lateral_total_m", lateral_total_m)?;
        let mut supply = [0.0_f64; DC01_HOUR_BIN_COUNT];
        let weight_total = sum_nonnegative_direct_m(
            "runon_admission.surface_hourly_weights",
            surface_hourly_weights,
        )?;
        if surface_total_m > 0.0 {
            if weight_total <= 0.0 {
                return Err(DirectRuntimeError::MissingDirectUpstream {
                    upstream: "hourly surface-runon transfer shape",
                });
            }
            for (hour, slot) in supply.iter_mut().enumerate() {
                let weight = surface_hourly_weights[hour] / weight_total;
                *slot += surface_total_m * weight;
            }
        }
        let lateral_shape_total_m =
            sum_nonnegative_direct_m("runon_admission.lateral_carry_m", lateral_carry_m)?;
        if lateral_total_m > 0.0 {
            if lateral_shape_total_m <= 0.0 {
                return Err(DirectRuntimeError::MissingDirectUpstream {
                    upstream: "hourly lateral-runon transfer shape",
                });
            }
            for (hour, slot) in supply.iter_mut().enumerate() {
                let weight = lateral_carry_m[hour] / lateral_shape_total_m;
                *slot += lateral_total_m * weight;
            }
        }
        Ok(supply)
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
        &mut self,
    ) -> Result<DirectRunoffPartitionState, DirectRuntimeError> {
        self.ensure_r4il_runoff_inputs_ready()?;
        self.validate_r4a_runoff_partition_domain()?;
        let inputs = self.runoff_partition_inputs;
        let partition_runoff_m = normalize_r4a_nonnegative_depth(
            "runoff_partition.partition_runoff_m",
            inputs.liquid_input_m + inputs.runon_input_m
                - inputs.cumulative_infiltration_m
                - inputs.depression_storage_delta_m
                - inputs.frost_retained_local_liquid_m,
        )?;
        let partition_runoff_m = source_informed_partition_runoff_canonicalization(
            partition_runoff_m,
            &self.wb14_hourly_excess_m,
        )?;
        validate_finite("runoff_partition.partition_runoff_m", partition_runoff_m)?;
        validate_nonnegative_direct_m("runoff_partition.partition_runoff_m", partition_runoff_m)?;
        let partition_runoff_m = if self
            .infiltration_depression_inputs
            .producer_inputs
            .is_some()
        {
            reconcile_hourly_partition_runoff_profile(
                &mut self.wb14_hourly_excess_m,
                partition_runoff_m,
                inputs.frost_retained_local_liquid_m,
            )?
        } else {
            partition_runoff_m
        };
        let q_runoff_m = partition_runoff_m + inputs.surface_saturation_runoff_m;
        validate_finite("runoff_partition.q_runoff_m", q_runoff_m)?;
        validate_nonnegative_direct_m("runoff_partition.q_runoff_m", q_runoff_m)?;
        let closure_residual_m =
            inputs.liquid_input_m + inputs.runon_input_m + inputs.surface_saturation_runoff_m
                - inputs.cumulative_infiltration_m
                - inputs.depression_storage_delta_m
                - inputs.frost_retained_local_liquid_m
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
        let runoff = self.runoff_shadow_projection.as_ref().ok_or_else(|| {
            DirectRuntimeError::HydrologyKernelGuard(Box::new(
                Wb11HydrologyKernelGuardError::peak_missing_flux("R4A runoff partition"),
            ))
        })?;
        let q_runoff_m = runoff.q_runoff_m;
        wb16_validate_nonnegative_flux("peak_runoff.q_runoff_m", q_runoff_m)
            .map_err(|guard| DirectRuntimeError::HydrologyKernelGuard(Box::new(guard)))?;
        if q_runoff_m == 0.0 {
            return Ok(DirectPeakRunoffState {
                q_runoff_m,
                peak_runoff_rate_m_s: 0.0,
                runoff_duration_s: 0.0,
                peak_hour_index: None,
                method_branch: 5.0,
                tstar: 0.0,
                qpstar: 0.0,
                vstar: 0.0,
            });
        }

        let subsurface = self
            .subsurface_compute_shadow_projection
            .as_ref()
            .ok_or_else(|| {
                DirectRuntimeError::HydrologyKernelGuard(Box::new(
                    Wb11HydrologyKernelGuardError::peak_missing_flux(
                        "R4O hourly saturation-return producer",
                    ),
                ))
            })?;
        let (peak_runoff_rate_m_s, runoff_duration_s, peak_hour_index) =
            source_complete_hourly_peak_runoff_depth_rate_m_s(
                q_runoff_m,
                &self.wb14_hourly_excess_m,
                &subsurface.hourly_saturation_carry_m,
            )?;

        Ok(DirectPeakRunoffState {
            q_runoff_m,
            peak_runoff_rate_m_s,
            runoff_duration_s,
            peak_hour_index: Some(peak_hour_index),
            method_branch: 5.0,
            tstar: 0.0,
            qpstar: 0.0,
            vstar: 0.0,
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
        validate_nonnegative_direct_m(
            "runoff_partition.frost_retained_local_liquid_m",
            self.runoff_partition_inputs.frost_retained_local_liquid_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.frost_preprojected_local_liquid_m",
            self.runoff_partition_inputs
                .frost_preprojected_local_liquid_m,
        )?;
        Ok(())
    }

    fn reconcile_r4a_frost_runtime(
        &mut self,
        winter_frost_compute_inputs: Option<&DirectWinterFrostComputeInputs>,
    ) -> Result<bool, DirectRuntimeError> {
        if winter_frost_compute_inputs.is_none() {
            return Ok(false);
        }
        // Single-solve authority (WP-2): the frost partition solved once at
        // day ingress (apply_r4w_winter_frost_ingress) owns every frost
        // mutation; R4A keeps only the retained-local-liquid bookkeeping for
        // the runoff partition.
        self.runoff_partition_inputs.frost_retained_local_liquid_m =
            self.r4a_deferred_local_partition_excess_m()?;
        Ok(true)
    }

    // Single-solve authority (WP-2): apply the day's frost partition outcome
    // (solved once by the runner authority from start-of-day lane state) to
    // the day's layer-input basis before any water-moving phase — the legacy
    // frsoil-before-infiltration ordering and the INV-SNOWFREEZE-012 hour-1
    // ingress handoff. Runs between R4C (which captures the pre-frost
    // storage-initial scalar) and R4I, so the daily identity
    // initial + fluxes + frost_delta = final closes by construction.
    pub(crate) fn apply_r4w_winter_frost_ingress(&mut self) -> Result<bool, DirectRuntimeError> {
        let Some(frost_partition) = self.winter_frost_outcome.take() else {
            return Ok(false);
        };
        let mut layers = self.percolation_inputs.layers.clone();
        let liquid_storage_before_frost_m = r4w_aggregate_liquid_soil_water(&layers)?;
        let has_material_storage_state =
            r4w_frost_partition_has_material_storage_state(&frost_partition);
        if frost_partition.active_frost_coupling {
            let carry = direct_frost_runtime_carry(&frost_partition);
            self.winter_column.frost = carry.clone().into();
            self.frost_runtime_carry = Some(carry);
        } else {
            self.winter_column.frost = DirectFrostLaneState::zero();
            self.frost_runtime_carry = None;
        }
        if !has_material_storage_state {
            if r4a_layers_have_coarse_frost_projection(&layers)? {
                clear_r4w_nonmaterial_frost_layer_projection(&mut layers)?;
                let liquid_storage_after_clear_m = r4w_aggregate_liquid_soil_water(&layers)?;
                if (liquid_storage_after_clear_m - liquid_storage_before_frost_m).abs() > 1.0e-9 {
                    return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                        field: "frost_ingress.no_material_frost_storage_clear",
                    });
                }
                self.apply_r4w_frost_layers(layers, liquid_storage_before_frost_m);
            }
            self.storage_reconciliation_inputs.frost_liquid_delta_m = 0.0;
            self.hydrology_projection_inputs.frozen_soil_water_m = 0.0;
            self.hydrology_projection_inputs.frost_depth_m = 0.0;
            return Ok(true);
        }
        apply_r4w_frost_layer_projection(&mut layers, &frost_partition)?;
        let soil_water_m = r4w_aggregate_liquid_soil_water(&layers)?;
        let soil_water_m =
            if let Some(soil_water_after_frwatc_m) = frost_partition.soil_water_after_frwatc_m {
                if (soil_water_after_frwatc_m - soil_water_m).abs() > 1.0e-9 {
                    return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                        field: "frost_ingress.frost_soil_water_after",
                    });
                }
                soil_water_after_frwatc_m
            } else {
                soil_water_m
            };
        if !r4w_frost_partition_has_final_frozen_projection(&frost_partition)
            && r4a_layers_have_coarse_frost_projection(&layers)?
        {
            clear_r4w_nonmaterial_frost_layer_projection(&mut layers)?;
            let soil_water_after_clear_m = r4w_aggregate_liquid_soil_water(&layers)?;
            if (soil_water_after_clear_m - soil_water_m).abs() > 1.0e-9 {
                return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                    field: "frost_ingress.thawed_frost_storage_clear",
                });
            }
        }
        let frozen_water_m =
            layers
                .iter()
                .map(|layer| layer.frozen_water_m)
                .try_fold(0.0_f64, |total, value| {
                    validate_nonnegative_direct_m("frost_ingress.frost_frozen_water_m", value)?;
                    let total = total + value;
                    validate_finite("frost_ingress.frost_frozen_water_total_m", total)?;
                    Ok::<f64, DirectRuntimeError>(total)
                })?;
        if frost_partition.frozen_water_after_m - frozen_water_m < -1.0e-9 {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "frost_ingress.frost_frozen_water_after",
            });
        }
        self.storage_reconciliation_inputs.frost_liquid_delta_m =
            soil_water_m - liquid_storage_before_frost_m;
        validate_finite(
            "frost_ingress.frost_liquid_delta_m",
            self.storage_reconciliation_inputs.frost_liquid_delta_m,
        )?;
        self.hydrology_projection_inputs.frozen_soil_water_m = frost_partition.frozen_water_after_m;
        self.hydrology_projection_inputs.frost_depth_m = frost_partition.frost_depth_after_m;
        self.apply_r4w_frost_layers(layers, soil_water_m);
        Ok(true)
    }

    fn apply_r4w_frost_layers(
        &mut self,
        layers: Vec<DirectSubsurfaceLayerState>,
        soil_water_m: f64,
    ) {
        self.subsurface_compute_inputs.layers = layers.iter().cloned().map(Into::into).collect();
        self.percolation_inputs.layers = layers;
        self.percolation_inputs.soil_water_initial_m = soil_water_m;
        self.water.soil_water_m = soil_water_m;
    }

    fn r4a_local_partition_excess_m(&self) -> Result<f64, DirectRuntimeError> {
        let inputs = self.runoff_partition_inputs;
        let local_excess_m = normalize_r4a_nonnegative_depth(
            "runoff_partition.local_partition_excess_m",
            inputs.liquid_input_m
                - inputs.cumulative_infiltration_m
                - inputs.depression_storage_delta_m,
        )?;
        validate_finite("runoff_partition.local_partition_excess_m", local_excess_m)?;
        Ok(local_excess_m)
    }

    fn r4a_deferred_local_partition_excess_m(&self) -> Result<f64, DirectRuntimeError> {
        if !self.r4a_has_material_winter_local_liquid_context()? {
            return Ok(0.0);
        }
        let local_partition_excess_m = self.r4a_local_partition_excess_m()?;
        let same_pass_consumed_local_m = normalize_r4a_nonnegative_depth(
            "runoff_partition.same_pass_consumed_local_liquid_m",
            self.percolation_inputs.same_pass_infiltration_m
                - self.runoff_partition_inputs.cumulative_infiltration_m,
        )?;
        validate_finite(
            "runoff_partition.same_pass_consumed_local_liquid_m",
            same_pass_consumed_local_m,
        )?;
        let deferred_local_liquid_m = (local_partition_excess_m
            - same_pass_consumed_local_m.min(local_partition_excess_m))
        .max(0.0);
        validate_finite(
            "runoff_partition.deferred_local_partition_excess_m",
            deferred_local_liquid_m,
        )?;
        Ok(deferred_local_liquid_m)
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

    fn r4a_has_material_winter_local_liquid_context(&self) -> Result<bool, DirectRuntimeError> {
        if self.snow_coupling_inputs.active_snow_coupling
            && self
                .snow_coupling_inputs
                .mass_transition_ledgers
                .solid_to_liquid()
                .liquid_handoff_m
                > WB11_ZERO_THRESHOLD
            && self.snow_coupling_inputs.post_winter_rain_m <= WB11_ZERO_THRESHOLD
            && self.storage_reconciliation_inputs.precip_input_m <= WB11_ZERO_THRESHOLD
        {
            return Ok(true);
        }
        if r4a_frost_lane_has_material_storage_state(&self.winter_column.frost) {
            return Ok(true);
        }
        let Ok(layers) = self.latest_r4a_frost_layers() else {
            return Ok(false);
        };
        r4a_layers_have_coarse_frost_projection(&layers)
    }
}

fn project_r4x_local_liquid_to_top_layer(
    layers: &mut [DirectSubsurfaceLayerState],
    local_partition_excess_m: f64,
) -> Result<(), DirectRuntimeError> {
    let Some(top_layer) = layers.first_mut() else {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "runoff_partition.winter_local_liquid_projection_layers",
        });
    };
    top_layer.theta_m += local_partition_excess_m;
    validate_nonnegative_direct_m(
        "runoff_partition.winter_local_liquid_projection_theta_m",
        top_layer.theta_m,
    )?;
    Ok(())
}

fn r4a_layers_have_coarse_frost_projection(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<bool, DirectRuntimeError> {
    for layer in layers {
        validate_nonnegative_direct_m(
            "runoff_partition.frost_layer_frozen_depth_m",
            layer.frozen_depth_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.frost_layer_frozen_water_m",
            layer.frozen_water_m,
        )?;
        if layer.frozen_depth_m > WB11_ZERO_THRESHOLD || layer.frozen_water_m > WB11_ZERO_THRESHOLD
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn r4w_aggregate_liquid_soil_water(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, DirectRuntimeError> {
    let mut soil_water_m = 0.0_f64;
    for layer in layers {
        validate_nonnegative_direct_m("frost_ingress.frost_layer_theta_m", layer.theta_m)?;
        validate_nonnegative_direct_m(
            "frost_ingress.frost_layer_residual_theta",
            layer.residual_theta,
        )?;
        validate_nonnegative_direct_m("frost_ingress.frost_layer_depth_m", layer.depth_m)?;
        validate_nonnegative_direct_m(
            "frost_ingress.frost_layer_frozen_depth_m",
            layer.frozen_depth_m,
        )?;
        if layer.frozen_depth_m > layer.depth_m + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "frost_ingress.frost_layer_frozen_depth_m",
            });
        }
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        soil_water_m += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        validate_finite("runoff_partition.frost_soil_water_m", soil_water_m)?;
    }
    Ok(soil_water_m.max(0.0))
}

fn r4w_frost_partition_has_material_storage_state(
    frost_partition: &DirectWinterFrostPartitionOutcome,
) -> bool {
    const DIRECT_FROST_MATERIAL_THRESHOLD_M: f64 = 1.0e-12;
    frost_partition.frost_depth_after_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.frozen_water_after_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.frwatc_freeze_debit_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.frwatc_thaw_credit_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.frwatc_net_liquid_delta_m.abs() > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.watpdg_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.watbtm_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.soil_water_after_frwatc_m.is_some()
        || frost_partition.layer_projection.iter().any(|layer| {
            layer.frozen_depth_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
                || layer.frozen_water_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        })
}

fn r4w_frost_partition_has_final_frozen_projection(
    frost_partition: &DirectWinterFrostPartitionOutcome,
) -> bool {
    const DIRECT_FROST_MATERIAL_THRESHOLD_M: f64 = 1.0e-12;
    frost_partition.frost_depth_after_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_partition.frozen_water_after_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
}

fn clear_r4w_nonmaterial_frost_layer_projection(
    layers: &mut [DirectSubsurfaceLayerState],
) -> Result<(), DirectRuntimeError> {
    for layer in layers {
        validate_nonnegative_direct_m("frost_ingress.frost_layer_theta_m", layer.theta_m)?;
        validate_nonnegative_direct_m(
            "frost_ingress.frost_layer_residual_theta",
            layer.residual_theta,
        )?;
        validate_nonnegative_direct_m("frost_ingress.frost_layer_depth_m", layer.depth_m)?;
        validate_nonnegative_direct_m(
            "frost_ingress.frost_layer_frozen_depth_m",
            layer.frozen_depth_m,
        )?;
        if layer.frozen_depth_m > layer.depth_m + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "frost_ingress.frost_layer_frozen_depth_m",
            });
        }
        if layer.frozen_depth_m <= WB11_ZERO_THRESHOLD
            && layer.frozen_water_m <= WB11_ZERO_THRESHOLD
        {
            layer.frozen_depth_m = 0.0;
            layer.frozen_water_m = 0.0;
            continue;
        }
        let aggregate_before_m =
            layer.theta_m + layer.residual_theta * (layer.depth_m - layer.frozen_depth_m).max(0.0);
        validate_finite(
            "runoff_partition.no_material_frost_layer_storage_before_m",
            aggregate_before_m,
        )?;
        layer.frozen_depth_m = 0.0;
        layer.frozen_water_m = 0.0;
        layer.theta_m = aggregate_before_m - layer.residual_theta * layer.depth_m;
        if layer.theta_m < 0.0 && layer.theta_m.abs() <= WB11_ZERO_THRESHOLD {
            layer.theta_m = 0.0;
        }
        validate_nonnegative_direct_m(
            "runoff_partition.no_material_frost_layer_theta_m",
            layer.theta_m,
        )?;
    }
    Ok(())
}

fn apply_r4w_frost_layer_projection(
    layers: &mut [DirectSubsurfaceLayerState],
    frost_partition: &DirectWinterFrostPartitionOutcome,
) -> Result<(), DirectRuntimeError> {
    let layer_count = layers.len();
    for projection in &frost_partition.layer_projection {
        if projection.layer_index == 0 || projection.layer_index > layer_count {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "frost_ingress.frost_layer_projection",
            });
        }
        let layer = layers
            .get_mut(projection.layer_index.saturating_sub(1))
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "frost_ingress.frost_layer_projection",
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

fn r4a_frost_lane_has_material_storage_state(frost_state: &DirectFrostLaneState) -> bool {
    const DIRECT_FROST_MATERIAL_THRESHOLD_M: f64 = 1.0e-12;
    frost_state.frdp_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_state.ws_frz_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_state.frwatc_freeze_debit_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_state.frwatc_thaw_credit_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_state.frwatc_net_liquid_delta_m.abs() > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_state.watpdg_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_state.watbtm_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        || frost_state.layer_shadows.iter().any(|layer| {
            layer.frozen_depth_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
                || layer.frozen_water_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        })
        || frost_state.fine_layers.iter().any(|layer| {
            layer.slfsd_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
                || layer.slsic_m > DIRECT_FROST_MATERIAL_THRESHOLD_M
        })
}

fn direct_frost_runtime_carry(
    frost_partition: &DirectWinterFrostPartitionOutcome,
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

pub(crate) fn source_informed_partition_runoff_canonicalization(
    partition_runoff_m: f64,
    wb14_hourly_excess_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> Result<f64, DirectRuntimeError> {
    validate_nonnegative_direct_m("runoff_partition.partition_runoff_m", partition_runoff_m)?;
    let hourly_excess_total_m = sum_nonnegative_direct_m(
        "runoff_partition.wb14_hourly_excess_m",
        wb14_hourly_excess_m,
    )?;
    if partition_runoff_m <= WB11_ZERO_THRESHOLD && hourly_excess_total_m == 0.0 {
        return Ok(0.0);
    }
    Ok(partition_runoff_m)
}

fn scale_aware_depth_closure_tolerance_m(values: &[f64]) -> f64 {
    let scale_m = values
        .iter()
        .copied()
        .fold(1.0_f64, |scale, value| scale.max(value.abs()));
    WB11_ZERO_THRESHOLD * scale_m
}

/// Reconcile the WB14 post-depression hourly lineage to the R4A local runoff
/// operand. A daily-only frost-retention amount may clear the entire hourly
/// surface series, because no peak timing remains to claim. Partial retention
/// with positive residual runoff requires an hourly frost-retention producer
/// and fails closed here; distributing a daily debit would invent peak timing.
pub(super) fn reconcile_hourly_partition_runoff_profile(
    hourly_runoff_m: &mut [f64; DC01_HOUR_BIN_COUNT],
    partition_runoff_m: f64,
    frost_retained_local_liquid_m: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_nonnegative_direct_m("runoff_partition.partition_runoff_m", partition_runoff_m)?;
    validate_nonnegative_direct_m(
        "runoff_partition.frost_retained_local_liquid_m",
        frost_retained_local_liquid_m,
    )?;
    let before_m = sum_nonnegative_direct_m(
        "runoff_partition.hourly_post_depression_runoff_m",
        hourly_runoff_m,
    )?;
    // WB14 solves as many as 24 interval bins and permits 1e-9 m interval
    // arithmetic closure. This explicit aggregate bound reconciles only the
    // independently accumulated daily scalar; the hourly ledger remains the
    // authoritative runoff operand.
    let tolerance_m = WB14_INTERVAL_CLOSURE_TOLERANCE_M
        * DC01_HOUR_BIN_COUNT_F64
        * [before_m, partition_runoff_m, frost_retained_local_liquid_m]
            .iter()
            .copied()
            .fold(1.0_f64, |scale, value| scale.max(value.abs()));
    if before_m == 0.0 && partition_runoff_m == 0.0 && frost_retained_local_liquid_m <= tolerance_m
    {
        *hourly_runoff_m = [0.0; DC01_HOUR_BIN_COUNT];
        return Ok(0.0);
    }
    if before_m == 0.0 && partition_runoff_m > 0.0 {
        return Err(DirectRuntimeError::MissingDirectUpstream {
            upstream: "hourly WB14 runoff ledger for positive partition runoff",
        });
    }
    let hourly_partition_runoff_m = (before_m - frost_retained_local_liquid_m).max(0.0);
    if (hourly_partition_runoff_m - partition_runoff_m).abs() > tolerance_m {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "runoff_partition.hourly_partition_runoff_m",
        });
    }
    if frost_retained_local_liquid_m > tolerance_m {
        if partition_runoff_m == 0.0 {
            *hourly_runoff_m = [0.0; DC01_HOUR_BIN_COUNT];
            return Ok(0.0);
        }
        return Err(DirectRuntimeError::MissingDirectUpstream {
            upstream: "hourly frost-retention timing for partial positive runoff",
        });
    }
    Ok(before_m)
}

/// ADR-0036 `REF-SED-DC01-SHAPE`: the SINGLE hourly-runoff shape authority.
/// Unit-normalized hourly distribution of the day's surface runoff (WB14
/// post-partition runoff profile + hourly saturation return). Routed melt and
/// runon have already passed through WB14 and are therefore not separate
/// runoff limbs. Positive runoff without a closing profile fails closed;
/// all-zero when there is no runoff. Consumed by (1) the MOFE
/// dynamic-transfer publication, (2) the hydrograph-resolved Wave-1 erosion
/// substrate (`INV-SED-013`), and (3) the serialized `V_h` HBP surface
/// (`V_h = runvol · w_h`) — one shape, three consumers, so the solve, the
/// interchange, and downstream runon admission see the same hydrograph.
pub(crate) fn dc01_surface_runoff_hourly_weights(
    q_runoff_m: f64,
    wb14_hourly_excess_m: &[f64; DC01_HOUR_BIN_COUNT],
    hourly_saturation_carry_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> Result<[f64; DC01_HOUR_BIN_COUNT], DirectRuntimeError> {
    validate_nonnegative_direct_m("dc01_surface_shape.q_runoff_m", q_runoff_m)?;
    let (hourly_runoff_m, source_total_m) =
        closing_hourly_runoff_depths_m(wb14_hourly_excess_m, hourly_saturation_carry_m)?;
    ensure_hourly_runoff_source_closure(q_runoff_m, source_total_m)?;
    if q_runoff_m == 0.0 {
        return Ok([0.0; DC01_HOUR_BIN_COUNT]);
    }
    let mut weights = [0.0; DC01_HOUR_BIN_COUNT];
    for (weight, hourly_m) in weights.iter_mut().zip(hourly_runoff_m) {
        *weight = hourly_m / source_total_m;
    }
    Ok(weights)
}

pub(crate) fn source_complete_hourly_peak_runoff_depth_rate_m_s(
    q_runoff_m: f64,
    wb14_hourly_excess_m: &[f64; DC01_HOUR_BIN_COUNT],
    hourly_saturation_carry_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> Result<(f64, f64, usize), DirectRuntimeError> {
    wb16_source_complete_hourly_peak_runoff_depth_rate_m_s(
        q_runoff_m,
        wb14_hourly_excess_m,
        hourly_saturation_carry_m,
    )
    .map_err(|guard| DirectRuntimeError::HydrologyKernelGuard(Box::new(guard)))
}

fn wb16_source_complete_hourly_peak_runoff_depth_rate_m_s(
    q_runoff_m: f64,
    wb14_hourly_excess_m: &[f64; DC01_HOUR_BIN_COUNT],
    hourly_saturation_carry_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> Result<(f64, f64, usize), Wb11HydrologyKernelGuardError> {
    wb16_validate_finite_flux("peak_runoff.q_runoff_m", q_runoff_m)?;
    if q_runoff_m <= 0.0 {
        return Err(Wb11HydrologyKernelGuardError::peak_flux_out_of_range(
            "peak_runoff.q_runoff_m",
            q_runoff_m,
            Some(f64::MIN_POSITIVE),
            None,
        ));
    }
    let (hourly_runoff_m, source_total_m) =
        wb16_closing_hourly_runoff_depths_m(wb14_hourly_excess_m, hourly_saturation_carry_m)?;
    wb16_ensure_hourly_runoff_source_closure(q_runoff_m, source_total_m)?;
    wb16_hourly_peak_runoff_from_closing_depths_m_s(q_runoff_m, &hourly_runoff_m)
}

fn wb16_validate_finite_flux(
    symbol: &'static str,
    value: f64,
) -> Result<(), Wb11HydrologyKernelGuardError> {
    if value.is_finite() {
        return Ok(());
    }
    Err(Wb11HydrologyKernelGuardError::peak_nonfinite_flux(
        symbol, value,
    ))
}

fn wb16_validate_nonnegative_flux(
    symbol: &'static str,
    value: f64,
) -> Result<(), Wb11HydrologyKernelGuardError> {
    wb16_validate_finite_flux(symbol, value)?;
    if value >= 0.0 {
        return Ok(());
    }
    Err(Wb11HydrologyKernelGuardError::peak_flux_out_of_range(
        symbol,
        value,
        Some(0.0),
        None,
    ))
}

fn wb16_closing_hourly_runoff_depths_m(
    wb14_hourly_excess_m: &[f64; DC01_HOUR_BIN_COUNT],
    hourly_saturation_carry_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> Result<([f64; DC01_HOUR_BIN_COUNT], f64), Wb11HydrologyKernelGuardError> {
    for hour in 0..DC01_HOUR_BIN_COUNT {
        for (symbol, value) in [
            (
                "peak_runoff.wb14_hourly_excess_m",
                wb14_hourly_excess_m[hour],
            ),
            (
                "peak_runoff.hourly_saturation_carry_m",
                hourly_saturation_carry_m[hour],
            ),
        ] {
            wb16_validate_nonnegative_flux(symbol, value)?;
        }
    }
    let (hourly_runoff_m, total_m) =
        assemble_closing_hourly_runoff_depths_m(wb14_hourly_excess_m, hourly_saturation_carry_m);
    wb16_validate_finite_flux("peak_runoff.hourly_source_total_m", total_m)?;
    Ok((hourly_runoff_m, total_m))
}

fn wb16_ensure_hourly_runoff_source_closure(
    q_runoff_m: f64,
    source_total_m: f64,
) -> Result<(), Wb11HydrologyKernelGuardError> {
    let tolerance_m = scale_aware_depth_closure_tolerance_m(&[q_runoff_m, source_total_m]);
    if q_runoff_m > 0.0 && source_total_m <= 0.0 {
        return Err(Wb11HydrologyKernelGuardError::peak_missing_flux(
            "post-partition hourly runoff timing for positive runoff",
        ));
    }
    if (source_total_m - q_runoff_m).abs() > tolerance_m {
        return Err(Wb11HydrologyKernelGuardError::peak_flux_out_of_range(
            "peak_runoff.hourly_source_total_m",
            source_total_m,
            Some(q_runoff_m - tolerance_m),
            Some(q_runoff_m + tolerance_m),
        ));
    }
    Ok(())
}

fn wb16_hourly_peak_runoff_from_closing_depths_m_s(
    q_runoff_m: f64,
    hourly_runoff_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> Result<(f64, f64, usize), Wb11HydrologyKernelGuardError> {
    let mut peak_rate_m_s = 0.0;
    let mut peak_hour_index = 0;
    for (hour, hourly_depth_m) in hourly_runoff_m.iter().copied().enumerate() {
        wb16_validate_nonnegative_flux("peak_runoff.hourly_depth_m", hourly_depth_m)?;
        let hourly_rate_m_s = hourly_depth_m / DC01_HOUR_BIN_SECONDS;
        wb16_validate_nonnegative_flux("peak_runoff.hourly_rate_m_s", hourly_rate_m_s)?;
        if hourly_rate_m_s > peak_rate_m_s {
            peak_rate_m_s = hourly_rate_m_s;
            peak_hour_index = hour;
        }
    }
    if peak_rate_m_s <= 0.0 {
        return Err(Wb11HydrologyKernelGuardError::peak_flux_out_of_range(
            "peak_runoff.peak_runoff_rate_m_s",
            peak_rate_m_s,
            Some(f64::MIN_POSITIVE),
            None,
        ));
    }
    let runoff_duration_s = q_runoff_m / peak_rate_m_s;
    wb16_validate_finite_flux("peak_runoff.runoff_duration_s", runoff_duration_s)?;
    let maximum_duration_s = DC01_HOUR_BIN_SECONDS * DC01_HOUR_BIN_COUNT_F64;
    if runoff_duration_s <= 0.0
        || runoff_duration_s > maximum_duration_s * (1.0 + WB11_ZERO_THRESHOLD)
    {
        return Err(Wb11HydrologyKernelGuardError::peak_flux_out_of_range(
            "peak_runoff.runoff_duration_s",
            runoff_duration_s,
            Some(f64::MIN_POSITIVE),
            Some(maximum_duration_s * (1.0 + WB11_ZERO_THRESHOLD)),
        ));
    }
    Ok((peak_rate_m_s, runoff_duration_s, peak_hour_index))
}

pub(crate) fn closing_hourly_runoff_depths_m(
    wb14_hourly_excess_m: &[f64; DC01_HOUR_BIN_COUNT],
    hourly_saturation_carry_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> Result<([f64; DC01_HOUR_BIN_COUNT], f64), DirectRuntimeError> {
    sum_nonnegative_direct_m("peak_runoff.wb14_hourly_excess_m", wb14_hourly_excess_m)?;
    sum_nonnegative_direct_m(
        "peak_runoff.hourly_saturation_carry_m",
        hourly_saturation_carry_m,
    )?;
    let (hourly_runoff_m, total_m) =
        assemble_closing_hourly_runoff_depths_m(wb14_hourly_excess_m, hourly_saturation_carry_m);
    validate_finite("peak_runoff.hourly_source_total_m", total_m)?;
    Ok((hourly_runoff_m, total_m))
}

fn assemble_closing_hourly_runoff_depths_m(
    wb14_hourly_excess_m: &[f64; DC01_HOUR_BIN_COUNT],
    hourly_saturation_carry_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> ([f64; DC01_HOUR_BIN_COUNT], f64) {
    let mut hourly_runoff_m = [0.0; DC01_HOUR_BIN_COUNT];
    let mut total_m = 0.0;
    for hour in 0..DC01_HOUR_BIN_COUNT {
        hourly_runoff_m[hour] = wb14_hourly_excess_m[hour] + hourly_saturation_carry_m[hour];
        total_m += hourly_runoff_m[hour];
    }
    (hourly_runoff_m, total_m)
}

fn ensure_hourly_runoff_source_closure(
    q_runoff_m: f64,
    source_total_m: f64,
) -> Result<(), DirectRuntimeError> {
    let tolerance_m = scale_aware_depth_closure_tolerance_m(&[q_runoff_m, source_total_m]);
    if q_runoff_m > 0.0 && source_total_m <= 0.0 {
        return Err(DirectRuntimeError::MissingDirectUpstream {
            upstream: "post-partition hourly runoff timing for positive runoff",
        });
    }
    if (source_total_m - q_runoff_m).abs() > tolerance_m {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "peak_runoff.hourly_source_total_m",
        });
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn hourly_peak_runoff_depth_rate_m_s(
    q_runoff_m: f64,
    hourly_weights: &[f64; DC01_HOUR_BIN_COUNT],
) -> Result<(f64, f64, usize), DirectRuntimeError> {
    validate_finite("peak_runoff.q_runoff_m", q_runoff_m)?;
    if q_runoff_m <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "peak_runoff.q_runoff_m",
        });
    }
    let mut weight_total = 0.0;
    let mut hourly_depths_m = [0.0; DC01_HOUR_BIN_COUNT];
    for (hour, weight) in hourly_weights.iter().copied().enumerate() {
        validate_nonnegative_direct_m("peak_runoff.hourly_weight", weight)?;
        weight_total += weight;
        validate_finite("peak_runoff.hourly_weight_total", weight_total)?;
        hourly_depths_m[hour] = q_runoff_m * weight;
        validate_nonnegative_direct_m("peak_runoff.hourly_depth_m", hourly_depths_m[hour])?;
    }
    if (weight_total - 1.0).abs() > WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "peak_runoff.hourly_weight_total",
        });
    }
    wb16_hourly_peak_runoff_from_closing_depths_m_s(q_runoff_m, &hourly_depths_m)
        .map_err(|guard| DirectRuntimeError::HydrologyKernelGuard(Box::new(guard)))
}

pub(crate) const DC01_HOUR_BIN_COUNT: usize = 24;
pub(crate) const DC01_HOUR_BIN_SECONDS: f64 = 3_600.0;
pub(crate) const DC01_HOUR_BIN_COUNT_F64: f64 = 24.0;
pub(crate) const WAT5_INTERVAL_SECONDS: f64 = 300.0;
pub(crate) const WAT5_INTERVALS_PER_HOUR: usize = 12;
pub(crate) const WAT5_INTERVALS_PER_DAY: usize = 288;
const WAT5_DAY_SECONDS: f64 = 86_400.0;

/// DC01 (INV-RUNOFFPART-031): WB14 outcome carrying the per-hour
/// infiltration-excess profile alongside the classic state. The profile is
/// the hourly distribution of non-infiltrated supply, used to publish an
/// hourly surface-transfer profile to the downstream lane.
pub(crate) struct DirectWb14OutcomeWithProfile {
    pub state: DirectInfiltrationDepressionState,
    pub hourly_excess_m: [f64; DC01_HOUR_BIN_COUNT],
    /// SC-SED-001 1b-C: per-hour RAINFALL depth (m), binned on the same
    /// basis as `hourly_excess_m`. Feeds the erosion `effint`; unread
    /// until the Wave-1 seed is enabled.
    pub hourly_rainfall_m: [f64; DC01_HOUR_BIN_COUNT],
}

#[allow(clippy::struct_field_names)]
pub(crate) struct DirectWb14SubhourlyProfile {
    pub rainfall_m: [f64; WAT5_INTERVALS_PER_DAY],
    pub additional_supply_m: [f64; WAT5_INTERVALS_PER_DAY],
    pub infiltration_m: [f64; WAT5_INTERVALS_PER_DAY],
    pub depression_storage_retention_m: [f64; WAT5_INTERVALS_PER_DAY],
    pub post_depression_excess_m: [f64; WAT5_INTERVALS_PER_DAY],
    pub depression_storage_delta_m: f64,
    pub canonical_source_pieces: Vec<DirectWb14HyetographInterval>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Wat5BoundedPartitionLedgerReconciliationV1 {
    pub hour_index: usize,
    pub authoritative_generation_m: f64,
    pub raw_generation_m: f64,
    pub closing_residual_m: f64,
    pub selected_piece_start_s: f64,
    pub selected_piece_end_s: f64,
    pub selected_bin_index: usize,
    pub closing_ledger_m: [f64; WAT5_INTERVALS_PER_HOUR],
}

fn wat5_zero_raw_reconciliation_error() -> DirectRuntimeError {
    wat5_zero_raw_reconciliation_error_at(
        "WAT5-E-002 bounded source-supported partition-ledger reconciliation",
    )
}

fn wat5_zero_raw_reconciliation_error_at(upstream: &'static str) -> DirectRuntimeError {
    DirectRuntimeError::MissingDirectUpstream { upstream }
}

impl Wat5BoundedPartitionLedgerReconciliationV1 {
    pub(crate) fn validate_against(
        &self,
        accepted_hourly_generation_m: &[f64; DC01_HOUR_BIN_COUNT],
        accepted_partition_runoff_m: f64,
        raw: &DirectWb14SubhourlyProfile,
    ) -> Result<(), DirectRuntimeError> {
        let hour = self.hour_index;
        if hour >= DC01_HOUR_BIN_COUNT {
            return Err(wat5_zero_raw_reconciliation_error());
        }
        let start = hour * WAT5_INTERVALS_PER_HOUR;
        let end = start + WAT5_INTERVALS_PER_HOUR;
        let sum_checked = |field: &'static str, values: &[f64]| {
            values.iter().try_fold(0.0_f64, |sum, value| {
                validate_nonnegative_direct_m(field, *value)?;
                let next = sum + *value;
                validate_nonnegative_direct_m(field, next)?;
                Ok(next)
            })
        };
        let rainfall_m = sum_checked(
            "wat5.bounded.hourly_rainfall_m",
            &raw.rainfall_m[start..end],
        )?;
        let additional_m = sum_checked(
            "wat5.bounded.hourly_additional_supply_m",
            &raw.additional_supply_m[start..end],
        )?;
        let source_m = rainfall_m + additional_m;
        validate_nonnegative_direct_m("wat5.bounded.hourly_source_m", source_m)?;
        let infiltration_m = sum_checked(
            "wat5.bounded.hourly_infiltration_m",
            &raw.infiltration_m[start..end],
        )?;
        let depression_m = sum_checked(
            "wat5.bounded.hourly_depression_storage_m",
            &raw.depression_storage_retention_m[start..end],
        )?;
        let raw_generation_m = sum_checked(
            "wat5.bounded.hourly_raw_generation_m",
            &raw.post_depression_excess_m[start..end],
        )?;
        let authoritative_m = accepted_hourly_generation_m[hour];
        validate_nonnegative_direct_m(
            "wat5.bounded.authoritative_hourly_generation_m",
            authoritative_m,
        )?;
        validate_nonnegative_direct_m(
            "wat5.bounded.accepted_partition_runoff_m",
            accepted_partition_runoff_m,
        )?;
        let accepted_total_m = sum_checked(
            "wat5.bounded.accepted_hourly_generation_m",
            accepted_hourly_generation_m,
        )?;
        let accepted_tolerance_m = 24.0
            * 1.0e-9
            * accepted_total_m
                .abs()
                .max(accepted_partition_runoff_m.abs())
                .max(1.0);
        if (accepted_total_m - accepted_partition_runoff_m).abs() > accepted_tolerance_m {
            return Err(wat5_zero_raw_reconciliation_error_at(
                "WAT5-E-002 accepted hourly/partition runoff closure",
            ));
        }
        let raw_accounted_m = infiltration_m + depression_m + raw_generation_m;
        validate_nonnegative_direct_m("wat5.bounded.hourly_raw_accounted_m", raw_accounted_m)?;
        let raw_tolerance_m = 1.0e-12 * source_m.abs().max(raw_accounted_m.abs()).max(1.0);
        if (source_m - raw_accounted_m).abs() > raw_tolerance_m
            || authoritative_m <= 0.0
            || raw_generation_m != 0.0
            || source_m <= 0.0
        {
            return Err(wat5_zero_raw_reconciliation_error_at(
                "WAT5-E-002 positive source-backed exact-zero raw generation eligibility",
            ));
        }
        let epsilon_m = authoritative_m - raw_generation_m;
        validate_nonnegative_direct_m("wat5.bounded.closing_residual_m", epsilon_m)?;
        let epsilon_tolerance_m = 1.0e-12
            * source_m
                .abs()
                .max(infiltration_m.abs())
                .max(depression_m.abs())
                .max(authoritative_m.abs())
                .max(1.0);
        if epsilon_m <= 0.0 || epsilon_m > epsilon_tolerance_m {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "WAT5-E-002",
                detail: format!(
                    "bounded closing residual tolerance: hour={hour} epsilon_m={epsilon_m:?} tolerance_m={epsilon_tolerance_m:?} source_m={source_m:?} infiltration_m={infiltration_m:?} depression_m={depression_m:?} authoritative_m={authoritative_m:?} raw_generation_m={raw_generation_m:?}",
                ),
            });
        }

        let hour_start_s = f64::from(u32::try_from(hour).unwrap_or(u32::MAX)) * 3_600.0;
        let hour_end_s = hour_start_s + 3_600.0;
        let mut positive_pieces = raw
            .canonical_source_pieces
            .iter()
            .filter(|piece| {
                piece.intensity_m_s > 0.0
                    && piece.start_s >= hour_start_s
                    && piece.end_s <= hour_end_s
                    && piece.end_s > piece.start_s
            })
            .collect::<Vec<_>>();
        positive_pieces.sort_by(|left, right| {
            left.end_s
                .total_cmp(&right.end_s)
                .then_with(|| left.start_s.total_cmp(&right.start_s))
        });
        let selected = positive_pieces
            .last()
            .ok_or_else(wat5_zero_raw_reconciliation_error)?;
        if positive_pieces
            .iter()
            .filter(|piece| {
                piece.start_s.to_bits() == selected.start_s.to_bits()
                    && piece.end_s.to_bits() == selected.end_s.to_bits()
            })
            .count()
            != 1
        {
            return Err(wat5_zero_raw_reconciliation_error_at(
                "WAT5-E-002 unique latest positive source piece",
            ));
        }
        let selected_bin_index = (start..end)
            .filter(|bin| {
                let bin_start_s =
                    f64::from(u32::try_from(*bin).unwrap_or(u32::MAX)) * WAT5_INTERVAL_SECONDS;
                let bin_end_s = bin_start_s + WAT5_INTERVAL_SECONDS;
                selected.start_s >= bin_start_s && selected.end_s <= bin_end_s
            })
            .next()
            .ok_or_else(wat5_zero_raw_reconciliation_error)?;
        if self.authoritative_generation_m.to_bits() != authoritative_m.to_bits()
            || self.raw_generation_m.to_bits() != raw_generation_m.to_bits()
            || self.closing_residual_m.to_bits() != epsilon_m.to_bits()
            || self.selected_piece_start_s.to_bits() != selected.start_s.to_bits()
            || self.selected_piece_end_s.to_bits() != selected.end_s.to_bits()
            || self.selected_bin_index != selected_bin_index
        {
            return Err(wat5_zero_raw_reconciliation_error_at(
                "WAT5-E-002 closing-reconciliation identity replay",
            ));
        }
        for (offset, value) in self.closing_ledger_m.iter().copied().enumerate() {
            validate_nonnegative_direct_m("wat5.bounded.closing_ledger_m", value)?;
            let expected = if start + offset == selected_bin_index {
                epsilon_m
            } else {
                0.0
            };
            if value.to_bits() != expected.to_bits() {
                return Err(wat5_zero_raw_reconciliation_error_at(
                    "WAT5-E-002 single-bin closing-ledger placement",
                ));
            }
        }
        let closed_m = sum_checked("wat5.bounded.closed_generation_m", &self.closing_ledger_m)?;
        if closed_m.to_bits() != authoritative_m.to_bits() {
            return Err(wat5_zero_raw_reconciliation_error_at(
                "WAT5-E-002 exact closed-hour reconstruction",
            ));
        }
        Ok(())
    }
}

pub(crate) fn reconcile_wat5_zero_raw_generation_hour_v1(
    hour_index: usize,
    accepted_hourly_generation_m: &[f64; DC01_HOUR_BIN_COUNT],
    accepted_partition_runoff_m: f64,
    raw: &DirectWb14SubhourlyProfile,
) -> Result<Wat5BoundedPartitionLedgerReconciliationV1, DirectRuntimeError> {
    if hour_index >= DC01_HOUR_BIN_COUNT {
        return Err(wat5_zero_raw_reconciliation_error());
    }
    let start = hour_index * WAT5_INTERVALS_PER_HOUR;
    let end = start + WAT5_INTERVALS_PER_HOUR;
    let authoritative_generation_m = accepted_hourly_generation_m[hour_index];
    let raw_generation_m = raw.post_depression_excess_m[start..end].iter().sum();
    let hour_start_s = f64::from(u32::try_from(hour_index).unwrap_or(u32::MAX)) * 3_600.0;
    let hour_end_s = hour_start_s + 3_600.0;
    let selected = raw
        .canonical_source_pieces
        .iter()
        .filter(|piece| {
            piece.intensity_m_s > 0.0
                && piece.start_s >= hour_start_s
                && piece.end_s <= hour_end_s
                && piece.end_s > piece.start_s
        })
        .max_by(|left, right| {
            left.end_s
                .total_cmp(&right.end_s)
                .then_with(|| left.start_s.total_cmp(&right.start_s))
        })
        .ok_or_else(wat5_zero_raw_reconciliation_error)?;
    let selected_bin_index = (start..end)
        .find(|bin| {
            let bin_start_s =
                f64::from(u32::try_from(*bin).unwrap_or(u32::MAX)) * WAT5_INTERVAL_SECONDS;
            let bin_end_s = bin_start_s + WAT5_INTERVAL_SECONDS;
            selected.start_s >= bin_start_s && selected.end_s <= bin_end_s
        })
        .ok_or_else(wat5_zero_raw_reconciliation_error)?;
    let closing_residual_m = authoritative_generation_m - raw_generation_m;
    let mut closing_ledger_m = [0.0; WAT5_INTERVALS_PER_HOUR];
    closing_ledger_m[selected_bin_index - start] = closing_residual_m;
    let reconciliation = Wat5BoundedPartitionLedgerReconciliationV1 {
        hour_index,
        authoritative_generation_m,
        raw_generation_m,
        closing_residual_m,
        selected_piece_start_s: selected.start_s,
        selected_piece_end_s: selected.end_s,
        selected_bin_index,
        closing_ledger_m,
    };
    reconciliation.validate_against(
        accepted_hourly_generation_m,
        accepted_partition_runoff_m,
        raw,
    )?;
    Ok(reconciliation)
}

fn add_depth_to_fixed_bins(
    bins: &mut [f64],
    bin_duration_s: f64,
    start_s: f64,
    end_s: f64,
    depth_m: f64,
) {
    let duration_s = end_s - start_s;
    if duration_s <= 0.0 || depth_m <= 0.0 {
        return;
    }
    for (hour, bin) in bins.iter_mut().enumerate() {
        let bin_start_s = f64::from(u32::try_from(hour).unwrap_or(u32::MAX)) * bin_duration_s;
        let bin_end_s = bin_start_s + bin_duration_s;
        let overlap_s = (end_s.min(bin_end_s) - start_s.max(bin_start_s)).max(0.0);
        if overlap_s > 0.0 {
            *bin += depth_m * overlap_s / duration_s;
        }
    }
}

pub(crate) fn dc01_hourly_supply_basis(
    hyetograph: &[DirectWb14HyetographInterval],
    hourly_additional_supply_m: &[f64; DC01_HOUR_BIN_COUNT],
) -> Vec<DirectWb14HyetographInterval> {
    let mut bins = [0.0_f64; DC01_HOUR_BIN_COUNT];
    for interval in hyetograph {
        let duration_s = interval.end_s - interval.start_s;
        if duration_s > 0.0 && interval.intensity_m_s > 0.0 {
            add_depth_to_fixed_bins(
                &mut bins,
                DC01_HOUR_BIN_SECONDS,
                interval.start_s,
                interval.end_s,
                interval.intensity_m_s * duration_s,
            );
        }
    }
    for (bin, runon_m) in bins.iter_mut().zip(hourly_additional_supply_m.iter()) {
        *bin += runon_m;
    }
    bins.iter()
        .enumerate()
        .map(|(hour, depth_m)| DirectWb14HyetographInterval {
            start_s: f64::from(u32::try_from(hour).unwrap_or(u32::MAX)) * DC01_HOUR_BIN_SECONDS,
            end_s: f64::from(u32::try_from(hour + 1).unwrap_or(u32::MAX)) * DC01_HOUR_BIN_SECONDS,
            intensity_m_s: depth_m / DC01_HOUR_BIN_SECONDS,
        })
        .collect()
}

fn wb14_hourly_local_rainfall_m(
    hyetograph: &[DirectWb14HyetographInterval],
) -> [f64; DC01_HOUR_BIN_COUNT] {
    let mut hourly_rainfall_m = [0.0_f64; DC01_HOUR_BIN_COUNT];
    for interval in hyetograph {
        let duration_s = interval.end_s - interval.start_s;
        if duration_s > 0.0 && interval.intensity_m_s > 0.0 {
            add_depth_to_fixed_bins(
                &mut hourly_rainfall_m,
                DC01_HOUR_BIN_SECONDS,
                interval.start_s,
                interval.end_s,
                interval.intensity_m_s * duration_s,
            );
        }
    }
    hourly_rainfall_m
}

#[allow(clippy::too_many_lines)]
pub(crate) fn compute_wb14_infiltration_depression_with_profile(
    inputs: &DirectWb14InfiltrationProducerInputs,
) -> Result<DirectWb14OutcomeWithProfile, DirectRuntimeError> {
    compute_wb14_infiltration_depression_with_optional_subhourly(inputs, false)
        .map(|(outcome, _)| outcome)
}

#[cfg(test)]
pub(crate) fn compute_wb14_subhourly_profile(
    inputs: &DirectWb14InfiltrationProducerInputs,
) -> Result<DirectWb14SubhourlyProfile, DirectRuntimeError> {
    compute_wb14_subhourly_profile_with_exact_segments(inputs, &[], None)
}

fn wat5_exact_segment_error() -> DirectRuntimeError {
    DirectRuntimeError::DirectDomainViolation {
        field: "WAT5-E-001 exact accepted additional-supply segment custody",
    }
}

pub(crate) fn wat5_additional_supply_source_receipt_sha256(
    source_kind: Wat5AdditionalSupplySourceKindV1,
    source_identity: &str,
    transaction_id: &str,
    destination_ofe_id: &openwepp_land_surface_energy::OfeId,
    start_s: f64,
    end_s: f64,
    depth_m_ofe_ground: f64,
) -> Result<String, DirectRuntimeError> {
    let source_kind = format!("{source_kind:?}");
    let start_bits = start_s.to_bits().to_be_bytes();
    let end_bits = end_s.to_bits().to_be_bytes();
    let depth_bits = depth_m_ofe_ground.to_bits().to_be_bytes();
    let digest = openwepp_coupled_time::framed_sha256(
        "wat5-exact-accepted-additional-source-receipt-v1",
        &[
            openwepp_coupled_time::FramedField {
                tag: "source_kind",
                value: source_kind.as_bytes(),
            },
            openwepp_coupled_time::FramedField {
                tag: "source_identity",
                value: source_identity.as_bytes(),
            },
            openwepp_coupled_time::FramedField {
                tag: "transaction_id",
                value: transaction_id.as_bytes(),
            },
            openwepp_coupled_time::FramedField {
                tag: "destination_ofe_id",
                value: destination_ofe_id.as_str().as_bytes(),
            },
            openwepp_coupled_time::FramedField {
                tag: "start_s_bits",
                value: &start_bits,
            },
            openwepp_coupled_time::FramedField {
                tag: "end_s_bits",
                value: &end_bits,
            },
            openwepp_coupled_time::FramedField {
                tag: "depth_m_ofe_ground_bits",
                value: &depth_bits,
            },
        ],
    )
    .map_err(|_| wat5_exact_segment_error())?;
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(64);
    for byte in digest.as_bytes() {
        text.push(char::from(HEX[usize::from(byte >> 4)]));
        text.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    Ok(text)
}

fn wat5_canonical_additional_supply_segments(
    segments: &[Wat5AdditionalSupplySegmentV1],
    expected_destination_ofe_id: Option<&openwepp_land_surface_energy::OfeId>,
) -> Result<Vec<Wat5AdditionalSupplySegmentV1>, DirectRuntimeError> {
    let mut canonical = segments.to_vec();
    for segment in &canonical {
        if !segment.start_s.is_finite()
            || !segment.end_s.is_finite()
            || segment.start_s < 0.0
            || segment.end_s <= segment.start_s
            || segment.end_s > WAT5_DAY_SECONDS
            || !segment.depth_m_ofe_ground.is_finite()
            || segment.depth_m_ofe_ground <= 0.0
            || segment.source_identity.is_empty()
            || segment.source_receipt_sha256.len() != 64
            || !segment
                .source_receipt_sha256
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
            || segment.transaction_id.is_empty()
            || expected_destination_ofe_id
                .is_some_and(|expected| expected != &segment.destination_ofe_id)
        {
            return Err(wat5_exact_segment_error());
        }
        let expected_receipt = wat5_additional_supply_source_receipt_sha256(
            segment.source_kind,
            &segment.source_identity,
            &segment.transaction_id,
            &segment.destination_ofe_id,
            segment.start_s,
            segment.end_s,
            segment.depth_m_ofe_ground,
        )?;
        if expected_receipt != segment.source_receipt_sha256 {
            return Err(wat5_exact_segment_error());
        }
    }
    canonical.sort_by(|left, right| {
        left.start_s
            .total_cmp(&right.start_s)
            .then_with(|| left.end_s.total_cmp(&right.end_s))
            .then_with(|| left.source_kind.cmp(&right.source_kind))
            .then_with(|| left.source_receipt_sha256.cmp(&right.source_receipt_sha256))
            .then_with(|| left.transaction_id.cmp(&right.transaction_id))
            .then_with(|| left.destination_ofe_id.cmp(&right.destination_ofe_id))
    });
    for (index, segment) in canonical.iter().enumerate() {
        if canonical[..index].iter().any(|earlier| {
            earlier.source_receipt_sha256 == segment.source_receipt_sha256
                && (earlier.start_s < segment.end_s && segment.start_s < earlier.end_s)
        }) {
            return Err(wat5_exact_segment_error());
        }
    }
    Ok(canonical)
}

pub(crate) fn wat5_hourly_additional_supply_from_segments(
    segments: &[Wat5AdditionalSupplySegmentV1],
    expected_destination_ofe_id: Option<&openwepp_land_surface_energy::OfeId>,
) -> Result<[f64; DC01_HOUR_BIN_COUNT], DirectRuntimeError> {
    let canonical =
        wat5_canonical_additional_supply_segments(segments, expected_destination_ofe_id)?;
    let mut hourly = [0.0; DC01_HOUR_BIN_COUNT];
    for segment in canonical {
        add_depth_to_fixed_bins(
            &mut hourly,
            DC01_HOUR_BIN_SECONDS,
            segment.start_s,
            segment.end_s,
            segment.depth_m_ofe_ground,
        );
        for depth_m in &hourly {
            validate_nonnegative_direct_m("wat5.exact_segment_hourly_depth_m", *depth_m)?;
        }
    }
    Ok(hourly)
}

fn wat5_piecewise_combined_supply(
    rain: &[DirectWb14HyetographInterval],
    canonical_additional: &[Wat5AdditionalSupplySegmentV1],
) -> Result<Vec<DirectWb14HyetographInterval>, DirectRuntimeError> {
    let mut boundaries = rain
        .iter()
        .flat_map(|interval| [interval.start_s, interval.end_s])
        .chain(
            canonical_additional
                .iter()
                .flat_map(|segment| [segment.start_s, segment.end_s]),
        )
        .chain((0..=WAT5_INTERVALS_PER_DAY).map(|index| {
            f64::from(u32::try_from(index).unwrap_or(u32::MAX)) * WAT5_INTERVAL_SECONDS
        }))
        .collect::<Vec<_>>();
    boundaries.sort_by(f64::total_cmp);
    boundaries.dedup_by(|left, right| left.to_bits() == right.to_bits());

    let mut combined = Vec::with_capacity(boundaries.len().saturating_sub(1));
    for bounds in boundaries.windows(2) {
        let start_s = bounds[0];
        let end_s = bounds[1];
        if end_s <= start_s {
            continue;
        }
        let rain_rate_m_s = rain
            .iter()
            .filter(|interval| interval.start_s <= start_s && interval.end_s >= end_s)
            .try_fold(0.0, |sum, interval| {
                let next = sum + interval.intensity_m_s;
                validate_nonnegative_direct_m("wat5.rain_rate_m_s", next)?;
                Ok(next)
            })?;
        let additional_rate_m_s = canonical_additional
            .iter()
            .filter(|segment| segment.start_s <= start_s && segment.end_s >= end_s)
            .try_fold(0.0, |sum, segment| {
                let rate_m_s = segment.depth_m_ofe_ground / (segment.end_s - segment.start_s);
                validate_nonnegative_direct_m("wat5.additional_rate_m_s", rate_m_s)?;
                let next = sum + rate_m_s;
                validate_nonnegative_direct_m("wat5.additional_rate_m_s", next)?;
                Ok(next)
            })?;
        let intensity_m_s = rain_rate_m_s + additional_rate_m_s;
        validate_nonnegative_direct_m("wat5.combined_supply_rate_m_s", intensity_m_s)?;
        combined.push(DirectWb14HyetographInterval {
            start_s,
            end_s,
            intensity_m_s,
        });
    }
    Ok(combined)
}

pub(crate) fn compute_wb14_subhourly_profile_with_exact_segments(
    inputs: &DirectWb14InfiltrationProducerInputs,
    segments: &[Wat5AdditionalSupplySegmentV1],
    expected_destination_ofe_id: Option<&openwepp_land_surface_energy::OfeId>,
) -> Result<DirectWb14SubhourlyProfile, DirectRuntimeError> {
    let source_profile =
        wat5_source_profile_with_exact_segments(inputs, segments, expected_destination_ofe_id)?;
    let mut split_inputs = inputs.clone();
    split_inputs
        .hyetograph
        .clone_from(&source_profile.canonical_source_pieces);
    split_inputs.hourly_additional_supply_m = [0.0; DC01_HOUR_BIN_COUNT];
    let (outcome, mut subhourly) =
        compute_wb14_infiltration_depression_with_optional_subhourly(&split_inputs, true)?;
    if let Some(profile) = subhourly.as_mut() {
        profile.depression_storage_delta_m = outcome.state.depression_storage_delta_m;
        profile.canonical_source_pieces = source_profile.canonical_source_pieces;
        profile.rainfall_m = source_profile.rainfall_m;
        profile.additional_supply_m = source_profile.additional_supply_m;
    }
    subhourly.ok_or(DirectRuntimeError::MissingDirectUpstream {
        upstream: "requested WAT5 subhourly WB14 profile",
    })
}

pub(crate) fn wat5_source_profile_with_exact_segments(
    inputs: &DirectWb14InfiltrationProducerInputs,
    segments: &[Wat5AdditionalSupplySegmentV1],
    expected_destination_ofe_id: Option<&openwepp_land_surface_energy::OfeId>,
) -> Result<DirectWb14SubhourlyProfile, DirectRuntimeError> {
    validate_wb14_infiltration_inputs(inputs)?;
    if inputs.hyetograph.iter().any(|interval| {
        interval.start_s < 0.0 || interval.end_s < 0.0 || interval.end_s > WAT5_DAY_SECONDS
    }) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "WAT5-E-003 hyetograph support lies outside the simulation day",
        });
    }
    let canonical =
        wat5_canonical_additional_supply_segments(segments, expected_destination_ofe_id)?;
    let reconstructed_hourly =
        wat5_hourly_additional_supply_from_segments(&canonical, expected_destination_ofe_id)?;
    if reconstructed_hourly
        .iter()
        .zip(inputs.hourly_additional_supply_m)
        .any(|(reconstructed, authoritative)| reconstructed.to_bits() != authoritative.to_bits())
    {
        return Err(wat5_exact_segment_error());
    }

    let canonical_source_pieces = wat5_piecewise_combined_supply(&inputs.hyetograph, &canonical)?;
    let mut profile = DirectWb14SubhourlyProfile {
        rainfall_m: [0.0; WAT5_INTERVALS_PER_DAY],
        additional_supply_m: [0.0; WAT5_INTERVALS_PER_DAY],
        infiltration_m: [0.0; WAT5_INTERVALS_PER_DAY],
        depression_storage_retention_m: [0.0; WAT5_INTERVALS_PER_DAY],
        post_depression_excess_m: [0.0; WAT5_INTERVALS_PER_DAY],
        depression_storage_delta_m: 0.0,
        canonical_source_pieces,
    };
    for interval in &inputs.hyetograph {
        add_depth_to_fixed_bins(
            &mut profile.rainfall_m,
            WAT5_INTERVAL_SECONDS,
            interval.start_s,
            interval.end_s,
            interval.intensity_m_s * (interval.end_s - interval.start_s),
        );
    }
    for segment in &canonical {
        add_depth_to_fixed_bins(
            &mut profile.additional_supply_m,
            WAT5_INTERVAL_SECONDS,
            segment.start_s,
            segment.end_s,
            segment.depth_m_ofe_ground,
        );
    }
    Ok(profile)
}

#[allow(clippy::too_many_lines)]
fn compute_wb14_infiltration_depression_with_optional_subhourly(
    inputs: &DirectWb14InfiltrationProducerInputs,
    collect_subhourly: bool,
) -> Result<
    (
        DirectWb14OutcomeWithProfile,
        Option<DirectWb14SubhourlyProfile>,
    ),
    DirectRuntimeError,
> {
    validate_wb14_infiltration_inputs(inputs)?;
    let mut cumulative_infiltration_m = 0.0_f64;
    let mut total_rainfall_m = 0.0_f64;
    let mut hourly_excess_m = [0.0_f64; DC01_HOUR_BIN_COUNT];
    let mut subhourly = if collect_subhourly {
        Some(DirectWb14SubhourlyProfile {
            rainfall_m: [0.0; WAT5_INTERVALS_PER_DAY],
            additional_supply_m: [0.0; WAT5_INTERVALS_PER_DAY],
            infiltration_m: [0.0; WAT5_INTERVALS_PER_DAY],
            depression_storage_retention_m: [0.0; WAT5_INTERVALS_PER_DAY],
            post_depression_excess_m: [0.0; WAT5_INTERVALS_PER_DAY],
            depression_storage_delta_m: 0.0,
            canonical_source_pieces: Vec::new(),
        })
    } else {
        None
    };

    // SC-SED-001 1b-C: the erosion rainfall surface is PURE local rainfall
    // (the erosion `effint` is a rainfall-intensity driver, not a supply
    // driver). Bin it from the local hyetograph directly, independent of
    // the runon-combined excess basis below — otherwise a downstream OFE's
    // runon would contaminate `effint` (a MOFE-erosion prerequisite).
    let hourly_rainfall_m = wb14_hourly_local_rainfall_m(&inputs.hyetograph);

    // With material additional supply, re-bin onto the 24 x 1 h basis: local
    // hyetograph depth plus producer-timed routed melt and inter-OFE runon.
    // This is the baseline hourly xfin loop shape. A direct-rain-only lane
    // keeps its exact interval basis.
    let additional_supply_total_m = sum_nonnegative_direct_m(
        "infiltration_depression.hourly_additional_supply_m",
        &inputs.hourly_additional_supply_m,
    )?;
    let hourly_basis: Vec<DirectWb14HyetographInterval>;
    let intervals: &[DirectWb14HyetographInterval] = if additional_supply_total_m > 0.0 {
        hourly_basis =
            dc01_hourly_supply_basis(&inputs.hyetograph, &inputs.hourly_additional_supply_m);
        &hourly_basis
    } else {
        &inputs.hyetograph
    };

    for interval in intervals {
        let duration_s = interval.end_s - interval.start_s;
        if duration_s <= 0.0 || interval.intensity_m_s <= 0.0 {
            continue;
        }
        let rainfall_m = interval.intensity_m_s * duration_s;
        if let Some(profile) = subhourly.as_mut() {
            add_depth_to_fixed_bins(
                &mut profile.rainfall_m,
                WAT5_INTERVAL_SECONDS,
                interval.start_s,
                interval.end_s,
                rainfall_m,
            );
        }
        validate_finite("infiltration_depression.interval_rainfall_m", rainfall_m)?;
        let transition = advance_wb14_interval_state(DirectWb14IntervalTransitionInputs {
            cumulative_supply_m: total_rainfall_m,
            cumulative_infiltration_m,
            interval_supply_m: rainfall_m,
            interval_duration_s: duration_s,
            interval_intensity_m_s: interval.intensity_m_s,
            effective_conductivity_m_s: inputs.effective_conductivity_m_s,
            matric_potential_m: inputs.matric_potential_m,
            storage_capacity_m: inputs.storage_capacity_m,
        })?;
        total_rainfall_m = transition.cumulative_supply_m;
        cumulative_infiltration_m = transition.cumulative_infiltration_m;
        let interval_infiltration_m = transition.interval_infiltration_m;
        let interval_excess_m = transition.interval_excess_m;
        if let Some(profile) = subhourly.as_mut() {
            add_depth_to_fixed_bins(
                &mut profile.infiltration_m,
                WAT5_INTERVAL_SECONDS,
                interval.start_s,
                interval.end_s,
                interval_infiltration_m,
            );
            add_depth_to_fixed_bins(
                &mut profile.post_depression_excess_m,
                WAT5_INTERVAL_SECONDS,
                interval.start_s,
                interval.end_s,
                interval_excess_m,
            );
        }
        add_depth_to_fixed_bins(
            &mut hourly_excess_m,
            DC01_HOUR_BIN_SECONDS,
            interval.start_s,
            interval.end_s,
            interval_excess_m,
        );
    }

    let rainfall_excess_m = (total_rainfall_m - cumulative_infiltration_m).max(0.0);
    let depression_storage_delta_m = rainfall_excess_m.min(inputs.depression_storage_capacity_m);
    validate_finite(
        "infiltration_depression.depression_storage_delta_m",
        depression_storage_delta_m,
    )?;
    // Depression storage is filled by the earliest non-infiltrated supply.
    // Remove that retained depth from the hourly runoff lineage here so the
    // profile, rather than a later normalized proxy, closes to the WB14
    // post-depression runoff operand.
    remove_depth_from_bins_earliest(
        &mut hourly_excess_m,
        depression_storage_delta_m,
        "infiltration_depression.hourly_post_depression_runoff_m",
    )?;
    if let Some(profile) = subhourly.as_mut() {
        let pre_depression_excess_m = profile.post_depression_excess_m;
        remove_depth_from_bins_earliest(
            &mut profile.post_depression_excess_m,
            depression_storage_delta_m,
            "infiltration_depression.subhourly_post_depression_runoff_m",
        )?;
        for ((retained_m, before_m), after_m) in profile
            .depression_storage_retention_m
            .iter_mut()
            .zip(pre_depression_excess_m)
            .zip(profile.post_depression_excess_m)
        {
            *retained_m = before_m - after_m;
            validate_nonnegative_direct_m(
                "infiltration_depression.subhourly_depression_storage_retention_m",
                *retained_m,
            )?;
        }
    }
    Ok((
        DirectWb14OutcomeWithProfile {
            state: DirectInfiltrationDepressionState {
                cumulative_infiltration_m,
                depression_storage_delta_m,
            },
            hourly_excess_m,
            hourly_rainfall_m,
        },
        subhourly,
    ))
}

fn remove_depth_from_bins_earliest(
    bins: &mut [f64],
    depth_m: f64,
    field: &'static str,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(field, depth_m)?;
    let mut remaining_m = depth_m;
    for bin_m in bins {
        validate_nonnegative_direct_m(field, *bin_m)?;
        let removed_m = remaining_m.min(*bin_m);
        *bin_m -= removed_m;
        remaining_m -= removed_m;
        if remaining_m <= WB11_ZERO_THRESHOLD {
            remaining_m = 0.0;
            break;
        }
    }
    if remaining_m > WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded { field });
    }
    Ok(())
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
    sum_nonnegative_direct_m(
        "infiltration_depression.hourly_additional_supply_m",
        &inputs.hourly_additional_supply_m,
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

pub(super) fn compute_green_ampt_interval_infiltration(
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

fn validate_positive_direct(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value > WB11_ZERO_THRESHOLD {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

include!("runoff_result_types.rs");
