use crate::constants::{
    WB11_ZERO_THRESHOLD, WB15_BIOMASS_TO_KG_HA, WB15_CANCOV_MAX, WB15_INTERCEPT_BIOMASS_MAX_KG_HA,
    WB15_INTERCEPT_LINEAR_COEFF, WB15_INTERCEPT_MM_TO_M, WB15_INTERCEPT_QUADRATIC_COEFF,
};
use crate::hydrology::{
    DirectWinterFrostComputeInputs, DirectWinterFrostPartitionOutcome,
    Wb11HydrologyKernelGuardError,
};
use crate::winter_column::DirectFrostLaneState;

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

fn closing_hourly_runoff_depths_m(
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
    pub infiltration_m: [f64; WAT5_INTERVALS_PER_DAY],
    pub depression_storage_retention_m: [f64; WAT5_INTERVALS_PER_DAY],
    pub post_depression_excess_m: [f64; WAT5_INTERVALS_PER_DAY],
    pub depression_storage_delta_m: f64,
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

pub(crate) fn compute_wb14_subhourly_profile(
    inputs: &DirectWb14InfiltrationProducerInputs,
) -> Result<DirectWb14SubhourlyProfile, DirectRuntimeError> {
    validate_wb14_infiltration_inputs(inputs)?;
    if inputs.hyetograph.iter().any(|interval| {
        interval.start_s < 0.0 || interval.end_s < 0.0 || interval.end_s > WAT5_DAY_SECONDS
    }) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "WAT5-E-003 hyetograph support lies outside the simulation day",
        });
    }
    if inputs
        .hourly_additional_supply_m
        .iter()
        .any(|depth_m| *depth_m > 0.0)
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "WAT5-E-001 positive additional supply lacks 300-second timing",
        });
    }

    // WAT5 is a chronological Green-Ampt replay, not a proportional
    // redistribution of whole source-interval outcomes. Split every source
    // interval at exact 300-second boundaries so each piece advances the
    // cumulative infiltration state before the next piece is solved.
    let mut split_inputs = inputs.clone();
    split_inputs.hyetograph = wat5_boundary_split_hyetograph(&inputs.hyetograph);
    let (outcome, mut subhourly) =
        compute_wb14_infiltration_depression_with_optional_subhourly(&split_inputs, true)?;
    if let Some(profile) = subhourly.as_mut() {
        profile.depression_storage_delta_m = outcome.state.depression_storage_delta_m;
    }
    subhourly.ok_or(DirectRuntimeError::MissingDirectUpstream {
        upstream: "requested WAT5 subhourly WB14 profile",
    })
}

fn wat5_boundary_split_hyetograph(
    hyetograph: &[DirectWb14HyetographInterval],
) -> Vec<DirectWb14HyetographInterval> {
    let mut split = Vec::with_capacity(hyetograph.len());
    for interval in hyetograph {
        if interval.end_s <= interval.start_s {
            split.push(*interval);
            continue;
        }
        let mut piece_start_s = interval.start_s;
        while piece_start_s < interval.end_s {
            let next_boundary_s =
                ((piece_start_s / WAT5_INTERVAL_SECONDS).floor() + 1.0) * WAT5_INTERVAL_SECONDS;
            let piece_end_s = interval.end_s.min(next_boundary_s);
            split.push(DirectWb14HyetographInterval {
                start_s: piece_start_s,
                end_s: piece_end_s,
                intensity_m_s: interval.intensity_m_s,
            });
            piece_start_s = piece_end_s;
        }
    }
    split
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
            infiltration_m: [0.0; WAT5_INTERVALS_PER_DAY],
            depression_storage_retention_m: [0.0; WAT5_INTERVALS_PER_DAY],
            post_depression_excess_m: [0.0; WAT5_INTERVALS_PER_DAY],
            depression_storage_delta_m: 0.0,
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
        total_rainfall_m += rainfall_m;
        validate_finite(
            "infiltration_depression.hyetograph_rainfall_m",
            total_rainfall_m,
        )?;
        let remaining_storage_m = (inputs.storage_capacity_m - cumulative_infiltration_m).max(0.0);
        if remaining_storage_m <= WB11_ZERO_THRESHOLD {
            add_depth_to_fixed_bins(
                &mut hourly_excess_m,
                DC01_HOUR_BIN_SECONDS,
                interval.start_s,
                interval.end_s,
                rainfall_m,
            );
            if let Some(profile) = subhourly.as_mut() {
                add_depth_to_fixed_bins(
                    &mut profile.post_depression_excess_m,
                    WAT5_INTERVAL_SECONDS,
                    interval.start_s,
                    interval.end_s,
                    rainfall_m,
                );
            }
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
        let cumulative_before_m = cumulative_infiltration_m;
        cumulative_infiltration_m += interval_infiltration_m.min(rainfall_m);
        cumulative_infiltration_m = cumulative_infiltration_m
            .min(inputs.storage_capacity_m)
            .min(total_rainfall_m);
        validate_finite(
            "infiltration_depression.cumulative_infiltration_m",
            cumulative_infiltration_m,
        )?;
        let interval_excess_m =
            (rainfall_m - (cumulative_infiltration_m - cumulative_before_m)).max(0.0);
        let interval_infiltration_m = rainfall_m - interval_excess_m;
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct DirectWb14ContinuationIntervalInputs {
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub interval_supply_m: f64,
    pub interval_duration_s: f64,
    pub effective_conductivity_m_s: f64,
    pub matric_potential_m: f64,
    pub storage_capacity_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_field_names)]
pub(super) struct DirectWb14ContinuationIntervalOutcome {
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub interval_infiltration_m: f64,
    pub interval_excess_m: f64,
}

/// Advance the same production Green-Ampt interval transition used by WB14
/// without replaying or resetting the already accepted day continuation.
pub(super) fn advance_wb14_continuation_interval(
    inputs: DirectWb14ContinuationIntervalInputs,
) -> Result<DirectWb14ContinuationIntervalOutcome, DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "surface_liquid.cumulative_supply_m",
        inputs.cumulative_supply_m,
    )?;
    validate_nonnegative_direct_m(
        "surface_liquid.cumulative_infiltration_m",
        inputs.cumulative_infiltration_m,
    )?;
    validate_nonnegative_direct_m("surface_liquid.interval_supply_m", inputs.interval_supply_m)?;
    validate_positive_direct(
        "surface_liquid.interval_duration_s",
        inputs.interval_duration_s,
    )?;
    validate_positive_direct(
        "surface_liquid.effective_conductivity_m_s",
        inputs.effective_conductivity_m_s,
    )?;
    validate_nonnegative_direct_m(
        "surface_liquid.matric_potential_m",
        inputs.matric_potential_m,
    )?;
    validate_nonnegative_direct_m(
        "surface_liquid.storage_capacity_m",
        inputs.storage_capacity_m,
    )?;
    if inputs.cumulative_infiltration_m > inputs.cumulative_supply_m
        || inputs.cumulative_infiltration_m > inputs.storage_capacity_m
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "surface_liquid.continuation_bounds",
        });
    }
    let remaining_storage_m = inputs.storage_capacity_m - inputs.cumulative_infiltration_m;
    let interval_infiltration_m = if remaining_storage_m <= WB11_ZERO_THRESHOLD {
        0.0
    } else {
        compute_green_ampt_interval_infiltration(
            inputs.cumulative_infiltration_m,
            inputs.interval_supply_m.min(remaining_storage_m),
            inputs.interval_duration_s,
            inputs.interval_supply_m / inputs.interval_duration_s,
            inputs.effective_conductivity_m_s,
            inputs.matric_potential_m,
        )?
    };
    let cumulative_supply_m = inputs.cumulative_supply_m + inputs.interval_supply_m;
    let cumulative_infiltration_m = inputs.cumulative_infiltration_m + interval_infiltration_m;
    let interval_excess_m = inputs.interval_supply_m - interval_infiltration_m;
    validate_nonnegative_direct_m("surface_liquid.interval_excess_m", interval_excess_m)?;
    if cumulative_infiltration_m > cumulative_supply_m
        || cumulative_infiltration_m > inputs.storage_capacity_m
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "surface_liquid.ending_continuation_bounds",
        });
    }
    Ok(DirectWb14ContinuationIntervalOutcome {
        cumulative_supply_m,
        cumulative_infiltration_m,
        interval_infiltration_m,
        interval_excess_m,
    })
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
    pub interception_live_biomass_kg_m2: f64,
}

impl DirectCanopyInterceptionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            hyetograph_rainfall_m: 0.0,
            interception_rainfall_input_m: 0.0,
            canopy_cover_fraction: 0.0,
            leaf_area_index: 0.0,
            interception_live_biomass_kg_m2: 0.0,
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
    /// Producer-owned hourly liquid supplied in addition to direct rain.
    /// Routed melt is seeded by the runner before WB14; inter-OFE runon is
    /// added at R4K from the R4J-resolved totals. Both pass through the same
    /// infiltration/depression partition before any runoff timing is claimed.
    pub hourly_additional_supply_m: [f64; DC01_HOUR_BIN_COUNT],
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
        let biomass_kg_ha = inputs.interception_live_biomass_kg_m2 * WB15_BIOMASS_TO_KG_HA;
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
        "canopy_interception.interception_live_biomass_kg_m2",
        inputs.interception_live_biomass_kg_m2,
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
    pub frost_retained_local_liquid_m: f64,
    pub frost_preprojected_local_liquid_m: f64,
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
            frost_retained_local_liquid_m: 0.0,
            frost_preprojected_local_liquid_m: 0.0,
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
pub struct DirectPeakRunoffState {
    pub q_runoff_m: f64,
    pub peak_runoff_rate_m_s: f64,
    pub runoff_duration_s: f64,
    pub peak_hour_index: Option<usize>,
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
            peak_runoff_rate_m_s: 0.0,
            runoff_duration_s: 0.0,
            peak_hour_index: None,
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
    pub peak_runoff_rate_m_s: f64,
    pub runoff_duration_s: f64,
    pub peak_hour_index: Option<usize>,
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
            peak_runoff_rate_m_s: 0.0,
            runoff_duration_s: 0.0,
            peak_hour_index: None,
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
            peak_runoff_rate_m_s: state.peak_runoff_rate_m_s,
            runoff_duration_s: state.runoff_duration_s,
            peak_hour_index: state.peak_hour_index,
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
    pub peak_runoff_rate_m_s: f64,
    pub runoff_duration_s: f64,
    pub peak_hour_index: Option<usize>,
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

#[cfg(test)]
mod surface_liquid_continuation_tests {
    use super::*;

    #[test]
    fn forty_eight_stateful_intervals_match_the_existing_daily_wb14_wrapper() {
        let interval_supply_m = 0.000_45;
        let hyetograph = (0..48)
            .map(|index| {
                let start_s = f64::from(index) * 1_800.0;
                DirectWb14HyetographInterval {
                    start_s,
                    end_s: start_s + 1_800.0,
                    intensity_m_s: interval_supply_m / 1_800.0,
                }
            })
            .collect();
        let inputs = DirectWb14InfiltrationProducerInputs {
            hyetograph,
            hourly_additional_supply_m: [0.0; DC01_HOUR_BIN_COUNT],
            effective_conductivity_m_s: 1.1e-7,
            matric_potential_m: 0.12,
            storage_capacity_m: 0.015,
            depression_storage_capacity_m: 0.0,
        };
        let daily = compute_wb14_infiltration_depression_with_profile(&inputs)
            .expect("daily production wrapper");
        let mut cumulative_supply_m = 0.0;
        let mut cumulative_infiltration_m = 0.0;
        let mut interval_excess_m = 0.0;
        for _ in 0..48 {
            let outcome =
                advance_wb14_continuation_interval(DirectWb14ContinuationIntervalInputs {
                    cumulative_supply_m,
                    cumulative_infiltration_m,
                    interval_supply_m,
                    interval_duration_s: 1_800.0,
                    effective_conductivity_m_s: inputs.effective_conductivity_m_s,
                    matric_potential_m: inputs.matric_potential_m,
                    storage_capacity_m: inputs.storage_capacity_m,
                })
                .expect("stateful interval");
            cumulative_supply_m = outcome.cumulative_supply_m;
            cumulative_infiltration_m = outcome.cumulative_infiltration_m;
            interval_excess_m += outcome.interval_excess_m;
        }
        assert_eq!(
            cumulative_infiltration_m.to_bits(),
            daily.state.cumulative_infiltration_m.to_bits()
        );
        let daily_excess_m = daily.hourly_excess_m.iter().sum::<f64>();
        assert!(
            (interval_excess_m - daily_excess_m).abs()
                <= scale_aware_depth_closure_tolerance_m(&[interval_excess_m, daily_excess_m,])
        );
        assert_eq!(
            daily.state.depression_storage_delta_m.to_bits(),
            0.0_f64.to_bits()
        );
    }
}
