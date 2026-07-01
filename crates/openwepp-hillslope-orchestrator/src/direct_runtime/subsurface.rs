use crate::constants::{
    MOFE_HOURLY_CARRY_ARRAY_COUNT, WB11_ZERO_THRESHOLD, WB18_DEEP_PERCOLATION_ROUNDOFF_TOLERANCE_M,
    WB18_PERC_BI_COEFFICIENT, WB18_PERC_MIN_FX, WB18_PERC_SATURATION_THRESHOLD,
    WB18_PERC_TIMESTEP_S, WB18_STORAGE_ROUNDOFF_TOLERANCE_M, WB19_DRAIN_ALPHA,
    WB19_DRAIN_HOURS_PER_DAY,
};

use super::{
    DIRECT_AUDIT, DIRECT_R4M_PHASE_SPAN_COUNT, DIRECT_R4O_PHASE_SPAN_COUNT, DirectDayFrame,
    DirectDeepSeepageDownstreamOperands, DirectDeepSeepageState, DirectPercolationTraceEvent,
    DirectRuntimeError, DirectSubsurfaceLossDownstreamOperands, DirectSubsurfaceLossState,
    DirectSubsurfaceSaturationTraceEvent, validate_finite, validate_nonnegative_direct_m,
};

#[derive(Debug, Clone)]
struct R7hSubsurfaceSaturationTraceConfig {
    path: std::path::PathBuf,
    exact_day_index: Option<usize>,
    exact_lane_index: Option<usize>,
}

static R7H_SUBSURFACE_SATURATION_TRACE_CONFIG: std::sync::OnceLock<
    Option<R7hSubsurfaceSaturationTraceConfig>,
> = std::sync::OnceLock::new();

#[derive(Debug, Clone)]
struct R7hPercolationTraceConfig {
    path: std::path::PathBuf,
    exact_day_index: Option<usize>,
    exact_lane_index: Option<usize>,
}

static R7H_PERCOLATION_TRACE_CONFIG: std::sync::OnceLock<Option<R7hPercolationTraceConfig>> =
    std::sync::OnceLock::new();

fn r7h_subsurface_saturation_trace_config() -> Option<&'static R7hSubsurfaceSaturationTraceConfig> {
    R7H_SUBSURFACE_SATURATION_TRACE_CONFIG
        .get_or_init(|| {
            let path = std::env::var_os("OPENWEPP_R7H_SUBSURFACE_SATURATION_TRACE_PATH")?;
            if path.is_empty() {
                return None;
            }
            Some(R7hSubsurfaceSaturationTraceConfig {
                path: std::path::PathBuf::from(path),
                exact_day_index: r7h_subsurface_trace_env_usize(
                    "OPENWEPP_R7H_SUBSURFACE_SATURATION_TRACE_DAY_INDEX",
                ),
                exact_lane_index: r7h_subsurface_trace_env_usize(
                    "OPENWEPP_R7H_SUBSURFACE_SATURATION_TRACE_LANE_INDEX",
                ),
            })
        })
        .as_ref()
}

fn r7h_percolation_trace_config() -> Option<&'static R7hPercolationTraceConfig> {
    R7H_PERCOLATION_TRACE_CONFIG
        .get_or_init(|| {
            let path = std::env::var_os("OPENWEPP_R7H_PERCOLATION_TRACE_PATH")?;
            if path.is_empty() {
                return None;
            }
            Some(R7hPercolationTraceConfig {
                path: std::path::PathBuf::from(path),
                exact_day_index: r7h_subsurface_trace_env_usize(
                    "OPENWEPP_R7H_PERCOLATION_TRACE_DAY_INDEX",
                ),
                exact_lane_index: r7h_subsurface_trace_env_usize(
                    "OPENWEPP_R7H_PERCOLATION_TRACE_LANE_INDEX",
                ),
            })
        })
        .as_ref()
}

fn r7h_subsurface_trace_env_usize(name: &str) -> Option<usize> {
    let value = std::env::var(name).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<usize>().ok()
}

fn r7h_subsurface_trace_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.17}")
    } else {
        "null".to_string()
    }
}

fn r7h_subsurface_trace_f64_array(values: impl IntoIterator<Item = f64>) -> String {
    let mut output = String::from("[");
    let mut first = true;
    for value in values {
        if !first {
            output.push(',');
        }
        first = false;
        output.push_str(&r7h_subsurface_trace_number(value));
    }
    output.push(']');
    output
}

fn maybe_write_r7h_percolation_trace(event: &DirectPercolationTraceEvent) {
    let Some(config) = r7h_percolation_trace_config() else {
        return;
    };
    if let Some(exact_day_index) = config.exact_day_index
        && event.day_index != exact_day_index
    {
        return;
    }
    if let Some(exact_lane_index) = config.exact_lane_index
        && event.lane_index != exact_lane_index
    {
        return;
    }

    let mut line = String::new();
    line.push('{');
    line.push_str("\"schema\":\"openwepp-r7h-percolation-trace-v1\"");
    line.push_str(",\"day_index\":");
    line.push_str(&event.day_index.to_string());
    line.push_str(",\"lane_index\":");
    line.push_str(&event.lane_index.to_string());
    line.push_str(",\"lane_substeps\":");
    line.push_str(&event.lane_substeps.to_string());
    line.push_str(",\"same_pass_infiltration_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.same_pass_infiltration_m));
    line.push_str(",\"same_pass_infiltration_lineage\":");
    line.push_str(if event.same_pass_infiltration_lineage {
        "true"
    } else {
        "false"
    });
    line.push_str(",\"tillage_depth_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.tillage_depth_m));
    line.push_str(",\"soil_water_before_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.soil_water_before_m));
    line.push_str(",\"computed_soil_water_before_m\":");
    line.push_str(&r7h_subsurface_trace_number(
        event.computed_soil_water_before_m,
    ));
    line.push_str(",\"soil_water_after_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.soil_water_after_m));
    line.push_str(",\"deep_seepage_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.deep_seepage_m));
    line.push_str(",\"per_layer_flux_m\":");
    line.push_str(&r7h_subsurface_trace_f64_array(
        event.per_layer_flux_m.iter().copied(),
    ));
    line.push_str(",\"layer_theta_after_m\":");
    line.push_str(&r7h_subsurface_trace_f64_array(
        event.layer_state_after.iter().map(|layer| layer.theta_m),
    ));
    line.push_str(",\"layer_upper_limit_m\":");
    line.push_str(&r7h_subsurface_trace_f64_array(
        event
            .layer_state_after
            .iter()
            .map(|layer| layer.upper_limit_m),
    ));
    line.push_str(",\"layer_frozen_water_m\":");
    line.push_str(&r7h_subsurface_trace_f64_array(
        event
            .layer_state_after
            .iter()
            .map(|layer| layer.frozen_water_m),
    ));
    line.push('}');
    line.push('\n');

    if let Some(parent) = config.path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.path)
    {
        let _ = std::io::Write::write_all(&mut file, line.as_bytes());
    }
}

fn maybe_write_r7h_subsurface_saturation_trace(event: &DirectSubsurfaceSaturationTraceEvent) {
    let Some(config) = r7h_subsurface_saturation_trace_config() else {
        return;
    };
    if let Some(exact_day_index) = config.exact_day_index
        && event.day_index != exact_day_index
    {
        return;
    }
    if let Some(exact_lane_index) = config.exact_lane_index
        && event.lane_index != exact_lane_index
    {
        return;
    }

    let mut line = String::new();
    line.push('{');
    line.push_str("\"schema\":\"openwepp-r7h-subsurface-saturation-trace-v1\"");
    line.push_str(",\"day_index\":");
    line.push_str(&event.day_index.to_string());
    line.push_str(",\"lane_index\":");
    line.push_str(&event.lane_index.to_string());
    line.push_str(",\"substep_index\":");
    line.push_str(&event.substep_index.to_string());
    line.push_str(",\"lane_substeps\":");
    line.push_str(&event.lane_substeps.to_string());
    line.push_str(",\"mofe_hourly_carry_arrays_enabled\":");
    line.push_str(if event.mofe_hourly_carry_arrays_enabled {
        "true"
    } else {
        "false"
    });
    line.push_str(",\"solwpv_mode\":");
    line.push_str(&event.solwpv_mode.to_string());
    line.push_str(",\"theta_before_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.theta_before_m));
    line.push_str(",\"upper_limit_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.upper_limit_m));
    line.push_str(",\"frozen_water_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.frozen_water_m));
    line.push_str(",\"effective_upper_limit_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.effective_upper_limit_m));
    line.push_str(",\"saturation_excess_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.saturation_excess_m));
    line.push_str(",\"current_saturation_runoff_m\":");
    line.push_str(&r7h_subsurface_trace_number(
        event.current_saturation_runoff_m,
    ));
    line.push_str(",\"theta_after_m\":");
    line.push_str(&r7h_subsurface_trace_number(event.theta_after_m));
    line.push('}');
    line.push('\n');

    if let Some(parent) = config.path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.path)
    {
        let _ = std::io::Write::write_all(&mut file, line.as_bytes());
    }
}

impl DirectDayFrame {
    pub fn run_r4m_percolation_span(
        &mut self,
    ) -> Result<DirectPercolationSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4M_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let percolation = self.compute_r4m_percolation()?;
        maybe_write_r7h_percolation_trace(&DirectPercolationTraceEvent::from_state(
            self.day_index,
            self.lane_index,
            &percolation,
            &self.percolation_inputs,
        ));
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.water.soil_water_m = percolation.soil_water_after_m;
        self.percolation = percolation.clone();
        self.deep_seepage = DirectDeepSeepageState {
            deep_seepage_m: percolation.deep_seepage_m,
        };
        self.deep_seepage_downstream_operands =
            DirectDeepSeepageDownstreamOperands::from(self.deep_seepage);
        self.storage_reconciliation_inputs.deep_seepage_m = percolation.deep_seepage_m;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.percolation_downstream_operands =
            DirectPercolationDownstreamOperands::from(percolation.clone());
        DIRECT_AUDIT.record_downstream_operand_production();

        let percolation_shadow_projection = DirectPercolationShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            soil_water_before_m: self.percolation_downstream_operands.soil_water_before_m,
            soil_water_after_m: self.percolation_downstream_operands.soil_water_after_m,
            deep_seepage_m: self.percolation_downstream_operands.deep_seepage_m,
            recharge_m: self.percolation_downstream_operands.recharge_m,
            per_layer_flux_m: self
                .percolation_downstream_operands
                .per_layer_flux_m
                .clone(),
            layer_state_after: self
                .percolation_downstream_operands
                .layer_state_after
                .clone(),
        };
        self.percolation_shadow_projection = Some(percolation_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectPercolationSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            percolation_shadow_projection,
        })
    }

    pub fn run_r4o_subsurface_compute_span(
        &mut self,
    ) -> Result<DirectSubsurfaceComputeSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4O_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let subsurface_compute = self.compute_r4o_subsurface()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.water.soil_water_m = subsurface_compute.soil_water_after_m;
        self.water.drainage_m = subsurface_compute.tile_drainage_m;
        self.water.lateral_flow_m = subsurface_compute.lateral_flow_m;
        self.subsurface_compute = subsurface_compute.clone();
        self.subsurface_loss = DirectSubsurfaceLossState {
            subsurface_loss_m: subsurface_compute.subsurface_loss_m,
        };
        self.subsurface_loss_downstream_operands =
            DirectSubsurfaceLossDownstreamOperands::from(self.subsurface_loss);
        self.storage_reconciliation_inputs.subsurface_loss_m = subsurface_compute.subsurface_loss_m;
        DIRECT_AUDIT.record_direct_state_mutation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.subsurface_compute_downstream_operands =
            DirectSubsurfaceComputeDownstreamOperands::from(subsurface_compute.clone());
        DIRECT_AUDIT.record_downstream_operand_production();

        let subsurface_compute_shadow_projection = DirectSubsurfaceComputeShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            soil_water_before_m: self
                .subsurface_compute_downstream_operands
                .soil_water_before_m,
            soil_water_after_m: self
                .subsurface_compute_downstream_operands
                .soil_water_after_m,
            lateral_flow_m: self.subsurface_compute_downstream_operands.lateral_flow_m,
            tile_drainage_m: self.subsurface_compute_downstream_operands.tile_drainage_m,
            subsurface_loss_m: self
                .subsurface_compute_downstream_operands
                .subsurface_loss_m,
            lateral_target_m: self.subsurface_compute_downstream_operands.lateral_target_m,
            drainage_target_m: self
                .subsurface_compute_downstream_operands
                .drainage_target_m,
            lateral_capacity_m: self
                .subsurface_compute_downstream_operands
                .lateral_capacity_m,
            hourly_lateral_carry_m: self
                .subsurface_compute_downstream_operands
                .hourly_lateral_carry_m,
            hourly_saturation_carry_m: self
                .subsurface_compute_downstream_operands
                .hourly_saturation_carry_m,
            layer_state_after: self
                .subsurface_compute_downstream_operands
                .layer_state_after
                .clone(),
            lateral_layer_withdrawal_m: self
                .subsurface_compute_downstream_operands
                .lateral_layer_withdrawal_m
                .clone(),
        };
        self.subsurface_compute_shadow_projection =
            Some(subsurface_compute_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectSubsurfaceComputeSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            subsurface_compute_shadow_projection,
        })
    }

    fn compute_r4m_percolation(&self) -> Result<DirectPercolationState, DirectRuntimeError> {
        let mut layers = self.percolation_inputs.layers.clone();
        validate_layers("percolation.layers", &layers)?;
        validate_percolation_inputs(&self.percolation_inputs)?;

        let computed_soil_water_before = aggregate_soil_water(&layers)?;
        let mut per_layer_flux_m = vec![0.0; layers.len()];
        let mut deep_seepage_m = 0.0;
        let lane_substeps_f64 = count_to_f64(
            "percolation.lane_substeps",
            self.percolation_inputs.lane_substeps,
        )?;

        for _ in 0..self.percolation_inputs.lane_substeps {
            apply_same_pass_infiltration(
                &mut layers,
                self.percolation_inputs.same_pass_infiltration_m / lane_substeps_f64,
                self.percolation_inputs.tillage_depth_m,
            )?;
            deep_seepage_m += run_percolation_substep(
                &mut layers,
                &self.percolation_inputs,
                &mut per_layer_flux_m,
                lane_substeps_f64,
            )?;
        }

        canonicalize_deep_percolation_roundoff(
            &mut layers,
            &mut per_layer_flux_m,
            &mut deep_seepage_m,
        );
        let soil_water_after_m = reconcile_percolation_soil_water(
            &mut layers,
            self.percolation_inputs.soil_water_initial_m,
            computed_soil_water_before,
            self.percolation_inputs
                .reconcile_legacy_soil_water_from_layers,
            self.percolation_inputs.same_pass_infiltration_m,
            deep_seepage_m,
        )?;

        Ok(DirectPercolationState {
            soil_water_before_m: self.percolation_inputs.soil_water_initial_m,
            computed_soil_water_before_m: computed_soil_water_before,
            soil_water_after_m,
            deep_seepage_m,
            recharge_m: deep_seepage_m,
            per_layer_flux_m,
            layer_state_after: layers,
        })
    }

    fn compute_r4o_subsurface(&self) -> Result<DirectSubsurfaceComputeState, DirectRuntimeError> {
        let percolation = self.percolation_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4M percolation",
            },
        )?;
        validate_subsurface_inputs(&self.subsurface_compute_inputs)?;
        let (mut layers, soil_water_before_m) =
            if let Some(surface_et) = self.evapotranspiration_surface_shadow_projection.as_ref() {
                (
                    surface_et.layer_state_after_soil_evap.clone(),
                    surface_et.soil_water_after_soil_evap_m,
                )
            } else {
                (
                    percolation.layer_state_after.clone(),
                    percolation.soil_water_after_m,
                )
            };
        validate_layers("subsurface.layers", &layers)?;
        if layers.len() != self.subsurface_compute_inputs.layers.len() {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "subsurface.layer_count",
            });
        }
        layers
            .iter_mut()
            .zip(self.subsurface_compute_inputs.layers.iter())
            .for_each(|(layer, inputs)| layer.copy_subsurface_parameters_from(inputs));

        let lane_substeps = self.subsurface_compute_inputs.lane_substeps;
        let lane_substeps_f64 = count_to_f64("subsurface.lane_substeps", lane_substeps)?;
        let drainage = run_drainage(
            &mut layers,
            &self.subsurface_compute_inputs,
            lane_substeps_f64,
        )?;
        let soil_water_after_drainage =
            apply_soil_water_withdrawal(soil_water_before_m, drainage.tile_drainage_m)?;
        let lateral = run_lateral(
            &mut layers,
            &self.subsurface_compute_inputs,
            lane_substeps_f64,
            self.day_index,
            self.lane_index,
        )?;
        let subsurface_loss_m = lateral.flow_m + drainage.tile_drainage_m;
        validate_finite("subsurface.subsurface_loss_m", subsurface_loss_m)?;
        let soil_water_after_m =
            apply_soil_water_withdrawal(soil_water_after_drainage, lateral.flow_m)?;

        Ok(DirectSubsurfaceComputeState {
            soil_water_before_m,
            soil_water_after_m,
            lateral_flow_m: lateral.flow_m,
            tile_drainage_m: drainage.tile_drainage_m,
            subsurface_loss_m,
            lateral_potential_m: lateral.potential_m,
            lateral_target_m: lateral.target_m,
            drainage_target_m: drainage.drainage_target_m,
            lateral_capacity_m: lateral.capacity_m,
            water_yield_m: lateral.water_yield_m,
            saturated_depth_m: lateral.saturated_depth_m,
            unsaturated_depth_m: lateral.unsaturated_depth_m,
            hourly_lateral_carry_m: lateral.hourly_lateral_carry,
            hourly_saturation_carry_m: lateral.hourly_saturation_carry,
            lateral_layer_withdrawal_m: lateral.layer_withdrawal_m,
            layer_state_after: layers,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSubsurfaceLayerInputs {
    pub theta_m: f64,
    pub field_capacity_m: f64,
    pub upper_limit_m: f64,
    pub conductivity_m_s: f64,
    pub depth_m: f64,
    pub residual_theta: f64,
    pub frozen_depth_m: f64,
    pub frozen_water_m: f64,
    pub porosity: f64,
    pub field_capacity_theta: f64,
    pub coca: f64,
    pub lateral_conductivity_m_s: f64,
}

impl DirectSubsurfaceLayerInputs {
    #[must_use]
    pub const fn neutral() -> Self {
        Self {
            theta_m: 0.0,
            field_capacity_m: 0.0,
            upper_limit_m: 1.0,
            conductivity_m_s: 1.0e-6,
            depth_m: 1.0,
            residual_theta: 0.0,
            frozen_depth_m: 0.0,
            frozen_water_m: 0.0,
            porosity: 1.0,
            field_capacity_theta: 0.5,
            coca: 1.0,
            lateral_conductivity_m_s: 1.0e-6,
        }
    }
}

impl From<DirectSubsurfaceLayerState> for DirectSubsurfaceLayerInputs {
    fn from(state: DirectSubsurfaceLayerState) -> Self {
        Self {
            theta_m: state.theta_m,
            field_capacity_m: state.field_capacity_m,
            upper_limit_m: state.upper_limit_m,
            conductivity_m_s: state.conductivity_m_s,
            depth_m: state.depth_m,
            residual_theta: state.residual_theta,
            frozen_depth_m: state.frozen_depth_m,
            frozen_water_m: state.frozen_water_m,
            porosity: state.porosity,
            field_capacity_theta: state.field_capacity_theta,
            coca: state.coca,
            lateral_conductivity_m_s: state.lateral_conductivity_m_s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSubsurfaceLayerState {
    pub theta_m: f64,
    pub field_capacity_m: f64,
    pub upper_limit_m: f64,
    pub conductivity_m_s: f64,
    pub depth_m: f64,
    pub residual_theta: f64,
    pub frozen_depth_m: f64,
    pub frozen_water_m: f64,
    pub porosity: f64,
    pub field_capacity_theta: f64,
    pub coca: f64,
    pub lateral_conductivity_m_s: f64,
}

impl DirectSubsurfaceLayerState {
    #[must_use]
    pub fn neutral() -> Self {
        Self::from(DirectSubsurfaceLayerInputs::neutral())
    }

    fn drain_threshold_m(&self) -> f64 {
        self.field_capacity_m + ((1.0 - self.coca) * self.depth_m)
    }

    fn lateral_withdrawal_threshold_m(&self) -> f64 {
        (self.drain_threshold_m() - self.frozen_water_m).max(0.0)
    }

    fn copy_subsurface_parameters_from(&mut self, inputs: &DirectSubsurfaceLayerInputs) {
        self.field_capacity_m = inputs.field_capacity_m;
        self.upper_limit_m = inputs.upper_limit_m;
        self.conductivity_m_s = inputs.conductivity_m_s;
        self.depth_m = inputs.depth_m;
        self.residual_theta = inputs.residual_theta;
        self.frozen_depth_m = inputs.frozen_depth_m;
        self.frozen_water_m = inputs.frozen_water_m;
        self.porosity = inputs.porosity;
        self.field_capacity_theta = inputs.field_capacity_theta;
        self.coca = inputs.coca;
        self.lateral_conductivity_m_s = inputs.lateral_conductivity_m_s;
    }
}

impl From<DirectSubsurfaceLayerInputs> for DirectSubsurfaceLayerState {
    fn from(inputs: DirectSubsurfaceLayerInputs) -> Self {
        Self {
            theta_m: inputs.theta_m,
            field_capacity_m: inputs.field_capacity_m,
            upper_limit_m: inputs.upper_limit_m,
            conductivity_m_s: inputs.conductivity_m_s,
            depth_m: inputs.depth_m,
            residual_theta: inputs.residual_theta,
            frozen_depth_m: inputs.frozen_depth_m,
            frozen_water_m: inputs.frozen_water_m,
            porosity: inputs.porosity,
            field_capacity_theta: inputs.field_capacity_theta,
            coca: inputs.coca,
            lateral_conductivity_m_s: inputs.lateral_conductivity_m_s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPercolationInputs {
    pub soil_water_initial_m: f64,
    pub reconcile_legacy_soil_water_from_layers: bool,
    pub same_pass_infiltration_m: f64,
    pub same_pass_infiltration_lineage: bool,
    pub tillage_depth_m: f64,
    pub lane_substeps: usize,
    pub restrictive_layer_enabled: bool,
    pub restrictive_layer_conductivity_m_s: f64,
    pub restrictive_layer_thickness_m: f64,
    pub layers: Vec<DirectSubsurfaceLayerState>,
}

impl DirectPercolationInputs {
    #[must_use]
    pub fn neutral() -> Self {
        Self {
            soil_water_initial_m: 0.0,
            reconcile_legacy_soil_water_from_layers: false,
            same_pass_infiltration_m: 0.0,
            same_pass_infiltration_lineage: false,
            tillage_depth_m: 0.0,
            lane_substeps: 1,
            restrictive_layer_enabled: false,
            restrictive_layer_conductivity_m_s: 0.0,
            restrictive_layer_thickness_m: 0.0,
            layers: vec![DirectSubsurfaceLayerState::neutral()],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPercolationState {
    pub soil_water_before_m: f64,
    pub computed_soil_water_before_m: f64,
    pub soil_water_after_m: f64,
    pub deep_seepage_m: f64,
    pub recharge_m: f64,
    pub per_layer_flux_m: Vec<f64>,
    pub layer_state_after: Vec<DirectSubsurfaceLayerState>,
}

impl DirectPercolationState {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_m: 0.0,
            computed_soil_water_before_m: 0.0,
            soil_water_after_m: 0.0,
            deep_seepage_m: 0.0,
            recharge_m: 0.0,
            per_layer_flux_m: Vec::new(),
            layer_state_after: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPercolationDownstreamOperands {
    pub soil_water_before_m: f64,
    pub soil_water_after_m: f64,
    pub deep_seepage_m: f64,
    pub recharge_m: f64,
    pub per_layer_flux_m: Vec<f64>,
    pub layer_state_after: Vec<DirectSubsurfaceLayerState>,
}

impl DirectPercolationDownstreamOperands {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_m: 0.0,
            soil_water_after_m: 0.0,
            deep_seepage_m: 0.0,
            recharge_m: 0.0,
            per_layer_flux_m: Vec::new(),
            layer_state_after: Vec::new(),
        }
    }
}

impl From<DirectPercolationState> for DirectPercolationDownstreamOperands {
    fn from(state: DirectPercolationState) -> Self {
        Self {
            soil_water_before_m: state.soil_water_before_m,
            soil_water_after_m: state.soil_water_after_m,
            deep_seepage_m: state.deep_seepage_m,
            recharge_m: state.recharge_m,
            per_layer_flux_m: state.per_layer_flux_m,
            layer_state_after: state.layer_state_after,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPercolationShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub soil_water_before_m: f64,
    pub soil_water_after_m: f64,
    pub deep_seepage_m: f64,
    pub recharge_m: f64,
    pub per_layer_flux_m: Vec<f64>,
    pub layer_state_after: Vec<DirectSubsurfaceLayerState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSubsurfaceComputeInputs {
    pub avg_slope: f64,
    pub slope_length_m: f64,
    pub lateral_anisotropy_ratio: f64,
    pub soil_depth_m: f64,
    pub solwpv_mode: i32,
    pub mofe_hourly_carry_arrays_enabled: bool,
    pub lane_substeps: usize,
    pub drainage_capacity_m: f64,
    pub drain_enabled: bool,
    pub drain_depth_m: f64,
    pub drain_spacing_m: f64,
    pub drain_diameter_m: f64,
    pub layers: Vec<DirectSubsurfaceLayerInputs>,
}

impl DirectSubsurfaceComputeInputs {
    #[must_use]
    pub fn neutral() -> Self {
        Self {
            avg_slope: 0.0,
            slope_length_m: 1.0,
            lateral_anisotropy_ratio: 1.0,
            soil_depth_m: 1.0,
            solwpv_mode: 2006,
            mofe_hourly_carry_arrays_enabled: false,
            lane_substeps: 1,
            drainage_capacity_m: 0.0,
            drain_enabled: false,
            drain_depth_m: 0.5,
            drain_spacing_m: 1.0,
            drain_diameter_m: 0.1,
            layers: vec![DirectSubsurfaceLayerInputs::neutral()],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSubsurfaceComputeState {
    pub soil_water_before_m: f64,
    pub soil_water_after_m: f64,
    pub lateral_flow_m: f64,
    pub tile_drainage_m: f64,
    pub subsurface_loss_m: f64,
    pub lateral_potential_m: f64,
    pub lateral_target_m: f64,
    pub drainage_target_m: f64,
    pub lateral_capacity_m: f64,
    pub water_yield_m: f64,
    pub saturated_depth_m: f64,
    pub unsaturated_depth_m: f64,
    pub hourly_lateral_carry_m: [f64; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub hourly_saturation_carry_m: [f64; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub lateral_layer_withdrawal_m: Vec<f64>,
    pub layer_state_after: Vec<DirectSubsurfaceLayerState>,
}

impl DirectSubsurfaceComputeState {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_m: 0.0,
            soil_water_after_m: 0.0,
            lateral_flow_m: 0.0,
            tile_drainage_m: 0.0,
            subsurface_loss_m: 0.0,
            lateral_potential_m: 0.0,
            lateral_target_m: 0.0,
            drainage_target_m: 0.0,
            lateral_capacity_m: 0.0,
            water_yield_m: 0.0,
            saturated_depth_m: 0.0,
            unsaturated_depth_m: 0.0,
            hourly_lateral_carry_m: [0.0; MOFE_HOURLY_CARRY_ARRAY_COUNT],
            hourly_saturation_carry_m: [0.0; MOFE_HOURLY_CARRY_ARRAY_COUNT],
            lateral_layer_withdrawal_m: Vec::new(),
            layer_state_after: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSubsurfaceComputeDownstreamOperands {
    pub soil_water_before_m: f64,
    pub soil_water_after_m: f64,
    pub lateral_flow_m: f64,
    pub tile_drainage_m: f64,
    pub subsurface_loss_m: f64,
    pub lateral_target_m: f64,
    pub drainage_target_m: f64,
    pub lateral_capacity_m: f64,
    pub hourly_lateral_carry_m: [f64; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub hourly_saturation_carry_m: [f64; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub lateral_layer_withdrawal_m: Vec<f64>,
    pub layer_state_after: Vec<DirectSubsurfaceLayerState>,
}

impl DirectSubsurfaceComputeDownstreamOperands {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_m: 0.0,
            soil_water_after_m: 0.0,
            lateral_flow_m: 0.0,
            tile_drainage_m: 0.0,
            subsurface_loss_m: 0.0,
            lateral_target_m: 0.0,
            drainage_target_m: 0.0,
            lateral_capacity_m: 0.0,
            hourly_lateral_carry_m: [0.0; MOFE_HOURLY_CARRY_ARRAY_COUNT],
            hourly_saturation_carry_m: [0.0; MOFE_HOURLY_CARRY_ARRAY_COUNT],
            lateral_layer_withdrawal_m: Vec::new(),
            layer_state_after: Vec::new(),
        }
    }
}

impl From<DirectSubsurfaceComputeState> for DirectSubsurfaceComputeDownstreamOperands {
    fn from(state: DirectSubsurfaceComputeState) -> Self {
        Self {
            soil_water_before_m: state.soil_water_before_m,
            soil_water_after_m: state.soil_water_after_m,
            lateral_flow_m: state.lateral_flow_m,
            tile_drainage_m: state.tile_drainage_m,
            subsurface_loss_m: state.subsurface_loss_m,
            lateral_target_m: state.lateral_target_m,
            drainage_target_m: state.drainage_target_m,
            lateral_capacity_m: state.lateral_capacity_m,
            hourly_lateral_carry_m: state.hourly_lateral_carry_m,
            hourly_saturation_carry_m: state.hourly_saturation_carry_m,
            lateral_layer_withdrawal_m: state.lateral_layer_withdrawal_m,
            layer_state_after: state.layer_state_after,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSubsurfaceComputeShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub soil_water_before_m: f64,
    pub soil_water_after_m: f64,
    pub lateral_flow_m: f64,
    pub tile_drainage_m: f64,
    pub subsurface_loss_m: f64,
    pub lateral_target_m: f64,
    pub drainage_target_m: f64,
    pub lateral_capacity_m: f64,
    pub hourly_lateral_carry_m: [f64; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub hourly_saturation_carry_m: [f64; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub layer_state_after: Vec<DirectSubsurfaceLayerState>,
    pub lateral_layer_withdrawal_m: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPercolationSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub percolation_shadow_projection: DirectPercolationShadowProjection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSubsurfaceComputeSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub subsurface_compute_shadow_projection: DirectSubsurfaceComputeShadowProjection,
}

#[derive(Default)]
struct DrainageRun {
    tile_drainage_m: f64,
    drainage_target_m: f64,
}

#[derive(Default)]
struct LateralRun {
    flow_m: f64,
    potential_m: f64,
    target_m: f64,
    capacity_m: f64,
    water_yield_m: f64,
    saturated_depth_m: f64,
    unsaturated_depth_m: f64,
    hourly_lateral_carry: [f64; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    hourly_saturation_carry: [f64; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    layer_withdrawal_m: Vec<f64>,
}

#[derive(Default)]
struct LateralMetrics {
    saturated_depth_m: f64,
    conductivity_depth_sum: f64,
    conductivity_active_depth_sum: f64,
    avpora: f64,
    avfca: f64,
    avcoca: f64,
    capacity_m: f64,
    legacy_saturation_fraction: f64,
}

fn validate_percolation_inputs(inputs: &DirectPercolationInputs) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "percolation.soil_water_initial_m",
        inputs.soil_water_initial_m,
    )?;
    validate_nonnegative_direct_m(
        "percolation.same_pass_infiltration_m",
        inputs.same_pass_infiltration_m,
    )?;
    validate_nonnegative_direct_m("percolation.tillage_depth_m", inputs.tillage_depth_m)?;
    if inputs.lane_substeps == 0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "percolation.lane_substeps",
        });
    }
    if inputs.restrictive_layer_enabled {
        validate_positive(
            "percolation.restrictive_layer_conductivity_m_s",
            inputs.restrictive_layer_conductivity_m_s,
        )?;
        if inputs.lane_substeps > 1 {
            validate_positive(
                "percolation.restrictive_layer_thickness_m",
                inputs.restrictive_layer_thickness_m,
            )?;
        }
    }
    Ok(())
}

fn validate_subsurface_inputs(
    inputs: &DirectSubsurfaceComputeInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("subsurface.avg_slope", inputs.avg_slope)?;
    validate_positive("subsurface.slope_length_m", inputs.slope_length_m)?;
    validate_positive(
        "subsurface.lateral_anisotropy_ratio",
        inputs.lateral_anisotropy_ratio,
    )?;
    validate_positive("subsurface.soil_depth_m", inputs.soil_depth_m)?;
    if inputs.solwpv_mode < 0 || inputs.lane_substeps == 0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "subsurface.branch_config",
        });
    }
    if inputs.mofe_hourly_carry_arrays_enabled
        && inputs.lane_substeps != MOFE_HOURLY_CARRY_ARRAY_COUNT
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "subsurface.lane_substeps",
        });
    }
    validate_nonnegative_direct_m("subsurface.drainage_capacity_m", inputs.drainage_capacity_m)?;
    validate_positive("subsurface.drain_depth_m", inputs.drain_depth_m)?;
    validate_positive("subsurface.drain_spacing_m", inputs.drain_spacing_m)?;
    validate_positive("subsurface.drain_diameter_m", inputs.drain_diameter_m)?;
    Ok(())
}

fn validate_layers(
    field: &'static str,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<(), DirectRuntimeError> {
    if layers.is_empty() {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    for layer in layers {
        validate_nonnegative_direct_m("layer.theta_m", layer.theta_m)?;
        validate_nonnegative_direct_m("layer.field_capacity_m", layer.field_capacity_m)?;
        validate_positive("layer.upper_limit_m", layer.upper_limit_m)?;
        validate_positive("layer.conductivity_m_s", layer.conductivity_m_s)?;
        validate_positive("layer.depth_m", layer.depth_m)?;
        validate_nonnegative_direct_m("layer.residual_theta", layer.residual_theta)?;
        validate_nonnegative_direct_m("layer.frozen_depth_m", layer.frozen_depth_m)?;
        validate_nonnegative_direct_m("layer.frozen_water_m", layer.frozen_water_m)?;
        validate_positive_fraction("layer.porosity", layer.porosity)?;
        validate_positive_fraction("layer.field_capacity_theta", layer.field_capacity_theta)?;
        validate_positive_fraction("layer.coca", layer.coca)?;
        validate_positive(
            "layer.lateral_conductivity_m_s",
            layer.lateral_conductivity_m_s,
        )?;
        if layer.field_capacity_m > layer.upper_limit_m + WB11_ZERO_THRESHOLD
            || layer.frozen_depth_m > layer.depth_m + WB11_ZERO_THRESHOLD
        {
            return Err(DirectRuntimeError::DirectDomainViolation { field: "layer" });
        }
    }
    Ok(())
}

fn validate_positive(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value > WB11_ZERO_THRESHOLD {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

fn validate_positive_fraction(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_positive(field, value)?;
    if value <= 1.0 + WB11_ZERO_THRESHOLD {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

fn count_to_f64(field: &'static str, count: usize) -> Result<f64, DirectRuntimeError> {
    let value = count
        .to_string()
        .parse::<f64>()
        .map_err(|_| DirectRuntimeError::DirectDomainViolation { field })?;
    validate_positive(field, value)?;
    Ok(value)
}

fn aggregate_soil_water(layers: &[DirectSubsurfaceLayerState]) -> Result<f64, DirectRuntimeError> {
    let mut soil_water_m = 0.0;
    for layer in layers {
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        soil_water_m += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        validate_finite("percolation.aggregate_soil_water_m", soil_water_m)?;
    }
    Ok(soil_water_m.max(0.0))
}

fn apply_same_pass_infiltration(
    layers: &mut [DirectSubsurfaceLayerState],
    infiltration_m: f64,
    tillage_depth_m: f64,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("percolation.same_pass_infiltration_m", infiltration_m)?;
    if infiltration_m <= WB11_ZERO_THRESHOLD {
        return Ok(());
    }
    let first_depth = layers
        .first()
        .ok_or(DirectRuntimeError::DirectDomainViolation {
            field: "percolation.layers",
        })?
        .depth_m;
    let resolved_tillage_depth_m = if tillage_depth_m > WB11_ZERO_THRESHOLD {
        tillage_depth_m
    } else {
        first_depth
    };
    validate_positive(
        "percolation.resolved_tillage_depth_m",
        resolved_tillage_depth_m,
    )?;

    let mut remaining_infiltration_m = infiltration_m;
    let mut cumulative_depth_m = 0.0;
    for layer in layers.iter_mut() {
        if remaining_infiltration_m <= WB11_ZERO_THRESHOLD {
            break;
        }
        cumulative_depth_m += layer.depth_m;
        let add_to_layer = if cumulative_depth_m < resolved_tillage_depth_m - WB11_ZERO_THRESHOLD {
            remaining_infiltration_m * layer.depth_m / resolved_tillage_depth_m
        } else {
            remaining_infiltration_m
        };
        validate_nonnegative_direct_m("percolation.layer_infiltration_m", add_to_layer)?;
        layer.theta_m += add_to_layer.max(0.0);
        validate_finite("percolation.layer_theta_m", layer.theta_m)?;
        remaining_infiltration_m -= add_to_layer;
    }

    if remaining_infiltration_m > WB11_ZERO_THRESHOLD {
        let last = layers
            .last_mut()
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "percolation.layers",
            })?;
        last.theta_m += remaining_infiltration_m;
        validate_finite("percolation.layer_theta_m", last.theta_m)?;
    }
    Ok(())
}

fn run_percolation_substep(
    layers: &mut [DirectSubsurfaceLayerState],
    inputs: &DirectPercolationInputs,
    per_layer_flux_m: &mut [f64],
    lane_substeps_f64: f64,
) -> Result<f64, DirectRuntimeError> {
    let mut substep_deep_seepage_m = 0.0;
    for layer_index in (0..layers.len()).rev() {
        substep_deep_seepage_m += route_percolation_layer(
            layers,
            inputs,
            per_layer_flux_m,
            lane_substeps_f64,
            layer_index,
        )?;
    }
    Ok(substep_deep_seepage_m)
}

fn route_percolation_layer(
    layers: &mut [DirectSubsurfaceLayerState],
    inputs: &DirectPercolationInputs,
    per_layer_flux_m: &mut [f64],
    lane_substeps_f64: f64,
    layer_index: usize,
) -> Result<f64, DirectRuntimeError> {
    let layer_theta_m = layers[layer_index].theta_m;
    let layer_field_capacity_m = layers[layer_index].field_capacity_m;
    let layer_upper_limit_m = layers[layer_index].upper_limit_m;
    let layer_frozen_water_m = layers[layer_index].frozen_water_m;
    let layer_count = layers.len();
    let effective_field_capacity_m = (layer_field_capacity_m - layer_frozen_water_m).max(0.0);
    let excess_m = layer_theta_m - effective_field_capacity_m;
    if excess_m <= WB11_ZERO_THRESHOLD {
        return Ok(0.0);
    }
    let saturation_ratio = (layer_theta_m + layer_frozen_water_m) / layer_upper_limit_m;
    validate_nonnegative_direct_m("percolation.saturation_ratio", saturation_ratio)?;
    let is_bottom_layer = layer_index == layer_count - 1;
    let lower_ratio = if is_bottom_layer {
        0.0
    } else {
        let lower_layer = &layers[layer_index + 1];
        (lower_layer.theta_m + lower_layer.frozen_water_m) / lower_layer.upper_limit_m
    };
    validate_nonnegative_direct_m("percolation.lower_saturation_ratio", lower_ratio)?;
    let saturated_lower_boundary = is_bottom_layer || lower_ratio >= WB18_PERC_SATURATION_THRESHOLD;
    let fx = percolation_layer_fx(
        layer_field_capacity_m,
        layer_upper_limit_m,
        saturation_ratio,
        inputs.lane_substeps == 1,
        saturated_lower_boundary,
    )?;
    let effective_conductivity_m_s =
        effective_percolation_conductivity(layers, inputs, layer_index, is_bottom_layer)?;
    let ks_adjusted_m_s = effective_conductivity_m_s * fx;
    validate_nonnegative_direct_m("percolation.ks_adjusted_m_s", ks_adjusted_m_s)?;
    let pei_pre_m = (WB18_PERC_TIMESTEP_S * ks_adjusted_m_s).min(excess_m);
    validate_nonnegative_direct_m("percolation.pei_pre_m", pei_pre_m)?;
    let pei_unscaled_m = if is_bottom_layer {
        pei_pre_m
    } else {
        pei_pre_m * (1.0 - lower_ratio.min(WB18_PERC_SATURATION_THRESHOLD)).sqrt()
    };
    let pei_m = pei_unscaled_m / lane_substeps_f64;
    validate_nonnegative_direct_m("percolation.per_layer_flux_m", pei_m)?;

    layers[layer_index].theta_m -= pei_m;
    validate_nonnegative_direct_m(
        "percolation.layer_theta_after_m",
        layers[layer_index].theta_m,
    )?;
    if is_bottom_layer {
        per_layer_flux_m[layer_index] += pei_m;
        validate_finite(
            "percolation.per_layer_flux_m",
            per_layer_flux_m[layer_index],
        )?;
        Ok(pei_m)
    } else {
        layers[layer_index + 1].theta_m += pei_m;
        validate_finite(
            "percolation.lower_layer_theta_after_m",
            layers[layer_index + 1].theta_m,
        )?;
        per_layer_flux_m[layer_index] += pei_m;
        validate_finite(
            "percolation.per_layer_flux_m",
            per_layer_flux_m[layer_index],
        )?;
        Ok(0.0)
    }
}

fn percolation_layer_fx(
    field_capacity_m: f64,
    upper_limit_m: f64,
    saturation_ratio: f64,
    daily_lane: bool,
    saturated_lower_boundary: bool,
) -> Result<f64, DirectRuntimeError> {
    let mut fx = if saturation_ratio < WB18_PERC_SATURATION_THRESHOLD {
        let fc_ul_ratio = field_capacity_m / upper_limit_m;
        validate_finite("percolation.fc_ul_ratio", fc_ul_ratio)?;
        if fc_ul_ratio >= 1.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "percolation.fc_ul_ratio",
            });
        }
        let bi = if fc_ul_ratio <= 0.0 {
            0.0
        } else {
            let derived = -WB18_PERC_BI_COEFFICIENT / fc_ul_ratio.log10();
            validate_nonnegative_direct_m("percolation.bi", derived)?;
            derived
        };
        saturation_ratio.powf(bi).max(WB18_PERC_MIN_FX)
    } else {
        1.0
    };
    if !daily_lane && saturated_lower_boundary {
        fx = 1.0;
    }
    validate_positive("percolation.fx", fx)?;
    Ok(fx)
}

fn effective_percolation_conductivity(
    layers: &[DirectSubsurfaceLayerState],
    inputs: &DirectPercolationInputs,
    layer_index: usize,
    is_bottom_layer: bool,
) -> Result<f64, DirectRuntimeError> {
    let layer_conductivity_m_s = layers[layer_index].conductivity_m_s;
    if !(inputs.restrictive_layer_enabled && is_bottom_layer) {
        return Ok(layer_conductivity_m_s);
    }
    if inputs.lane_substeps == 1 {
        let denominator = layer_conductivity_m_s + inputs.restrictive_layer_conductivity_m_s;
        validate_positive("percolation.restrictive_denominator", denominator)?;
        let harmonic_mean =
            (2.0 * layer_conductivity_m_s * inputs.restrictive_layer_conductivity_m_s)
                / denominator;
        validate_positive("percolation.restrictive_harmonic_m_s", harmonic_mean)?;
        Ok(harmonic_mean)
    } else {
        let denominator = (layers[layer_index].depth_m / layer_conductivity_m_s)
            + (inputs.restrictive_layer_thickness_m / inputs.restrictive_layer_conductivity_m_s);
        validate_positive("percolation.restrictive_weighted_denominator", denominator)?;
        let thickness_weighted =
            (layers[layer_index].depth_m + inputs.restrictive_layer_thickness_m) / denominator;
        validate_positive("percolation.restrictive_weighted_m_s", thickness_weighted)?;
        Ok(thickness_weighted)
    }
}

fn canonicalize_deep_percolation_roundoff(
    layers: &mut [DirectSubsurfaceLayerState],
    per_layer_flux_m: &mut [f64],
    deep_seepage_m: &mut f64,
) {
    if (0.0..=WB18_DEEP_PERCOLATION_ROUNDOFF_TOLERANCE_M).contains(deep_seepage_m) {
        if *deep_seepage_m > 0.0 {
            let bottom_index = layers.len() - 1;
            layers[bottom_index].theta_m += *deep_seepage_m;
            per_layer_flux_m[bottom_index] =
                (per_layer_flux_m[bottom_index] - *deep_seepage_m).max(0.0);
        }
        *deep_seepage_m = 0.0;
    }
}

fn reconcile_percolation_soil_water(
    layers: &mut [DirectSubsurfaceLayerState],
    soil_water_initial_m: f64,
    computed_soil_water_before_m: f64,
    reconcile_legacy_soil_water_from_layers: bool,
    same_pass_infiltration_m: f64,
    deep_seepage_m: f64,
) -> Result<f64, DirectRuntimeError> {
    let mut computed_soil_water_after_m = aggregate_soil_water(layers)?;
    let preserve_scalar_ledger = !reconcile_legacy_soil_water_from_layers
        && (soil_water_initial_m.max(0.0) - computed_soil_water_before_m).abs()
            <= WB18_STORAGE_ROUNDOFF_TOLERANCE_M;
    let soil_water_after_m = if preserve_scalar_ledger {
        let ledger_after_m =
            soil_water_initial_m.max(0.0) + same_pass_infiltration_m - deep_seepage_m;
        validate_nonnegative_direct_m("percolation.ledger_soil_water_after_m", ledger_after_m)?;
        ledger_after_m.max(0.0)
    } else {
        computed_soil_water_after_m
    };

    if preserve_scalar_ledger {
        let storage_roundoff_delta_m = soil_water_after_m - computed_soil_water_after_m;
        if storage_roundoff_delta_m.abs() <= WB18_STORAGE_ROUNDOFF_TOLERANCE_M {
            apply_storage_roundoff_delta(layers, storage_roundoff_delta_m)?;
            computed_soil_water_after_m = aggregate_soil_water(layers)?;
        }
        if (computed_soil_water_after_m - soil_water_after_m).abs()
            > WB18_STORAGE_ROUNDOFF_TOLERANCE_M
        {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "percolation.soil_water_after_m",
            });
        }
    }
    Ok(soil_water_after_m)
}

fn apply_storage_roundoff_delta(
    layers: &mut [DirectSubsurfaceLayerState],
    delta_m: f64,
) -> Result<(), DirectRuntimeError> {
    if delta_m.abs() <= f64::EPSILON {
        return Ok(());
    }
    if delta_m > 0.0 {
        let bottom = layers
            .last_mut()
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "percolation.layers",
            })?;
        bottom.theta_m += delta_m;
        validate_nonnegative_direct_m("percolation.roundoff_theta_m", bottom.theta_m)?;
        return Ok(());
    }
    let debit_m = -delta_m;
    if let Some(index) = layers
        .iter()
        .rposition(|layer| layer.theta_m + WB11_ZERO_THRESHOLD >= debit_m)
    {
        layers[index].theta_m = (layers[index].theta_m - debit_m).max(0.0);
        validate_nonnegative_direct_m("percolation.roundoff_theta_m", layers[index].theta_m)?;
        return Ok(());
    }
    Err(DirectRuntimeError::DirectDomainViolation {
        field: "percolation.roundoff_delta_m",
    })
}

fn run_drainage(
    layers: &mut [DirectSubsurfaceLayerState],
    inputs: &DirectSubsurfaceComputeInputs,
    lane_substeps_f64: f64,
) -> Result<DrainageRun, DirectRuntimeError> {
    let mut run = DrainageRun::default();
    let lane_hour_fraction = WB19_DRAIN_HOURS_PER_DAY / lane_substeps_f64;
    validate_positive("subsurface.lane_hour_fraction", lane_hour_fraction)?;
    let drain_thresholds = drain_thresholds(layers)?;
    for _ in 0..inputs.lane_substeps {
        let pool_m = drainable_storage(layers, &drain_thresholds);
        let remaining_capacity_m = (inputs.drainage_capacity_m - run.tile_drainage_m).max(0.0);
        let potential =
            drainage_potential(layers, inputs, remaining_capacity_m, lane_hour_fraction)?;
        let target_m = potential.0.min(remaining_capacity_m).min(pool_m);
        let withdrawn_m =
            withdraw_tile_to_surface(layers, &drain_thresholds, potential.1, target_m);
        validate_nonnegative_direct_m("subsurface.tile_drainage_substep_m", withdrawn_m)?;
        run.drainage_target_m += target_m;
        run.tile_drainage_m += withdrawn_m;
        validate_finite("subsurface.drainage_target_m", run.drainage_target_m)?;
        validate_finite("subsurface.tile_drainage_m", run.tile_drainage_m)?;
    }
    Ok(run)
}

fn drainage_potential(
    layers: &[DirectSubsurfaceLayerState],
    inputs: &DirectSubsurfaceComputeInputs,
    remaining_capacity_m: f64,
    lane_hour_fraction: f64,
) -> Result<(f64, usize), DirectRuntimeError> {
    let mut tile_layer_index = layers.len().saturating_sub(1);
    if !inputs.drain_enabled || remaining_capacity_m <= WB11_ZERO_THRESHOLD {
        return Ok((0.0, tile_layer_index));
    }
    let drain_thresholds = drain_thresholds(layers)?;
    let dep2watbl_m = drainage_depth_to_water_table(inputs, layers, &drain_thresholds)?;
    if dep2watbl_m <= inputs.drain_depth_m + WB11_ZERO_THRESHOLD {
        tile_layer_index = drainage_tile_layer_index(layers, inputs.drain_depth_m);
        let potential_m = drainage_potential_flux(inputs, layers, dep2watbl_m, lane_hour_fraction)?;
        validate_nonnegative_direct_m("subsurface.drainage_potential_m", potential_m)?;
        Ok((potential_m, tile_layer_index))
    } else {
        Ok((0.0, tile_layer_index))
    }
}

fn drainage_depth_to_water_table(
    inputs: &DirectSubsurfaceComputeInputs,
    layers: &[DirectSubsurfaceLayerState],
    drain_thresholds: &[f64],
) -> Result<f64, DirectRuntimeError> {
    let mut watbl_m = 0.0;
    let mut hit_unsaturated_zone = false;
    for index in (0..layers.len()).rev() {
        if layers[index].theta_m + WB11_ZERO_THRESHOLD >= drain_thresholds[index] {
            if !hit_unsaturated_zone {
                watbl_m += layers[index].depth_m;
            }
        } else {
            hit_unsaturated_zone = true;
        }
    }
    let dep2watbl_m = inputs.soil_depth_m - watbl_m;
    validate_finite("subsurface.dep2watbl_m", dep2watbl_m)?;
    Ok(dep2watbl_m)
}

fn drainage_tile_layer_index(layers: &[DirectSubsurfaceLayerState], drain_depth_m: f64) -> usize {
    let mut cumulative_depth_m = 0.0;
    let mut tile_layer = 0_usize;
    for (index, layer) in layers.iter().enumerate() {
        cumulative_depth_m += layer.depth_m;
        if cumulative_depth_m <= drain_depth_m + WB11_ZERO_THRESHOLD {
            tile_layer = index;
        }
    }
    (tile_layer + 1).min(layers.len().saturating_sub(1))
}

fn drainage_potential_flux(
    inputs: &DirectSubsurfaceComputeInputs,
    layers: &[DirectSubsurfaceLayerState],
    dep2watbl_m: f64,
    lane_hour_fraction: f64,
) -> Result<f64, DirectRuntimeError> {
    let dranks_cm_h = drainage_saturated_conductivity_cm_h(inputs, layers, dep2watbl_m)?;
    let drain_depth_cm = ((inputs.soil_depth_m - inputs.drain_depth_m).max(0.0) * 100.0).max(1.0);
    let spacing_cm = inputs.drain_spacing_m * 100.0;
    let radius_cm = inputs.drain_diameter_m * 50.0;
    validate_positive("subsurface.drain_spacing_cm", spacing_cm)?;
    validate_positive("subsurface.drain_radius_cm", radius_cm)?;
    let equivalent_depth_cm = drainage_equivalent_depth_cm(drain_depth_cm, spacing_cm, radius_cm)?;
    let water_table_cm = (inputs.drain_depth_m - dep2watbl_m).max(0.0) * 100.0;
    let drainage_cm_h = (8.0 * dranks_cm_h * equivalent_depth_cm * water_table_cm
        + 4.0 * dranks_cm_h * water_table_cm.powi(2))
        / spacing_cm.powi(2);
    validate_nonnegative_direct_m("subsurface.drainage_cm_h", drainage_cm_h)?;
    Ok((drainage_cm_h / 100.0) * lane_hour_fraction)
}

fn drainage_saturated_conductivity_cm_h(
    inputs: &DirectSubsurfaceComputeInputs,
    layers: &[DirectSubsurfaceLayerState],
    dep2watbl_m: f64,
) -> Result<f64, DirectRuntimeError> {
    let mut cumulative_depth_m = 0.0;
    let mut conductivity_depth_sum = 0.0;
    let mut saturated_depth_sum = 0.0;
    for layer in layers {
        cumulative_depth_m += layer.depth_m;
        if cumulative_depth_m + WB11_ZERO_THRESHOLD >= dep2watbl_m {
            conductivity_depth_sum += layer.conductivity_m_s * layer.depth_m;
            saturated_depth_sum += layer.depth_m;
        }
    }
    let dranks_cm_h = if saturated_depth_sum > WB11_ZERO_THRESHOLD {
        let saturated_conductivity_m_s = conductivity_depth_sum / saturated_depth_sum;
        saturated_conductivity_m_s * 100.0 * 3600.0
    } else {
        0.0
    };
    validate_nonnegative_direct_m("subsurface.dranks_cm_h", dranks_cm_h)?;
    if dep2watbl_m > inputs.soil_depth_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "subsurface.dep2watbl_m",
        });
    }
    Ok(dranks_cm_h)
}

fn drainage_equivalent_depth_cm(
    drain_depth_cm: f64,
    spacing_cm: f64,
    radius_cm: f64,
) -> Result<f64, DirectRuntimeError> {
    let spacing_ratio = drain_depth_cm / spacing_cm;
    let equivalent_depth_cm = if spacing_ratio <= 0.3 && spacing_ratio > 0.0 {
        let radius_ratio = drain_depth_cm / radius_cm;
        validate_positive("subsurface.drain_radius_ratio", radius_ratio)?;
        let denominator = 1.0
            + spacing_ratio * ((8.0 / std::f64::consts::PI) * radius_ratio.ln() - WB19_DRAIN_ALPHA);
        validate_positive("subsurface.drain_shallow_denominator", denominator)?;
        drain_depth_cm / denominator
    } else {
        let log_term = (spacing_cm / radius_cm).ln() - 1.15;
        validate_positive("subsurface.drain_deep_log_term", log_term)?;
        (spacing_cm * std::f64::consts::PI) / (8.0 * log_term)
    };
    validate_nonnegative_direct_m("subsurface.equivalent_depth_cm", equivalent_depth_cm)?;
    Ok(equivalent_depth_cm)
}

fn run_lateral(
    layers: &mut [DirectSubsurfaceLayerState],
    inputs: &DirectSubsurfaceComputeInputs,
    lane_substeps_f64: f64,
    day_index: usize,
    lane_index: usize,
) -> Result<LateralRun, DirectRuntimeError> {
    let mut run = LateralRun {
        layer_withdrawal_m: vec![0.0; layers.len()],
        ..LateralRun::default()
    };
    for substep_index in 0..inputs.lane_substeps {
        let active_layers = lateral_active_layers(layers, inputs)?;
        let metrics = lateral_metrics(layers, inputs, &active_layers, lane_substeps_f64)?;
        let potential_m = lateral_potential(inputs, &metrics, lane_substeps_f64)?;
        run.potential_m += potential_m;
        run.capacity_m += metrics.capacity_m;
        let withdrawal_thresholds = lateral_withdrawal_thresholds(layers)?;
        let available_pool_m = drainable_storage(layers, &withdrawal_thresholds);
        let target_m = potential_m.min(available_pool_m).min(metrics.capacity_m);
        let substep_m = withdraw_top_down(
            layers,
            &withdrawal_thresholds,
            target_m,
            &mut run.layer_withdrawal_m,
        );
        run.target_m += target_m;
        run.flow_m += substep_m;
        if inputs.mofe_hourly_carry_arrays_enabled {
            run.hourly_lateral_carry[substep_index] = substep_m;
        }
        record_surface_saturation_carry(
            layers,
            inputs,
            substep_index,
            &mut run,
            day_index,
            lane_index,
        )?;
        let depths = lateral_depths(inputs, &metrics, substep_m)?;
        run.water_yield_m = depths.0;
        run.saturated_depth_m = depths.1;
        run.unsaturated_depth_m = depths.2;
        validate_finite("subsurface.lateral_flow_m", run.flow_m)?;
    }
    Ok(run)
}

fn lateral_active_layers(
    layers: &[DirectSubsurfaceLayerState],
    inputs: &DirectSubsurfaceComputeInputs,
) -> Result<(Vec<bool>, Vec<bool>), DirectRuntimeError> {
    if inputs.lane_substeps == 1 && !inputs.mofe_hourly_carry_arrays_enabled {
        let mut capacity = vec![false; layers.len()];
        let mut conductivity = vec![false; layers.len()];
        let mut top_contiguous_block_open = true;
        for (index, layer) in layers.iter().enumerate() {
            let active =
                layer.theta_m + WB11_ZERO_THRESHOLD >= layer.lateral_withdrawal_threshold_m();
            let accepted = if inputs.solwpv_mode < 2006 {
                let contiguous = top_contiguous_block_open && active;
                if !active {
                    top_contiguous_block_open = false;
                }
                contiguous
            } else {
                active
            };
            capacity[index] = accepted;
            conductivity[index] = accepted;
        }
        return Ok((capacity, conductivity));
    }

    let mut capacity = vec![false; layers.len()];
    let mut conductivity = vec![false; layers.len()];
    for index in 0..layers.len() {
        let lower_saturated = lateral_lower_layer_saturated(layers, index)?;
        capacity[index] = layers[index].theta_m + WB11_ZERO_THRESHOLD
            >= layers[index].lateral_withdrawal_threshold_m()
            && lower_saturated;
        conductivity[index] = layers[index].theta_m + WB11_ZERO_THRESHOLD
            >= layers[index].drain_threshold_m()
            && lower_saturated;
    }
    Ok((capacity, conductivity))
}

fn lateral_lower_layer_saturated(
    layers: &[DirectSubsurfaceLayerState],
    index: usize,
) -> Result<bool, DirectRuntimeError> {
    if index + 1 == layers.len() {
        return Ok(true);
    }
    validate_positive(
        "subsurface.lower_upper_limit_m",
        layers[index + 1].upper_limit_m,
    )?;
    Ok(layers[index + 1].theta_m / layers[index + 1].upper_limit_m >= 1.0 - WB11_ZERO_THRESHOLD)
}

fn lateral_metrics(
    layers: &[DirectSubsurfaceLayerState],
    inputs: &DirectSubsurfaceComputeInputs,
    active_layers: &(Vec<bool>, Vec<bool>),
    lane_substeps_f64: f64,
) -> Result<LateralMetrics, DirectRuntimeError> {
    let saturated_depth_m = active_layers
        .0
        .iter()
        .zip(layers.iter())
        .filter_map(|(active, layer)| active.then_some(layer.depth_m))
        .sum::<f64>();
    let mut metrics = LateralMetrics {
        saturated_depth_m,
        legacy_saturation_fraction: 1.0,
        ..LateralMetrics::default()
    };
    if saturated_depth_m <= WB11_ZERO_THRESHOLD {
        return Ok(metrics);
    }
    let legacy_daily_lateral = inputs.lane_substeps == 1
        && !inputs.mofe_hourly_carry_arrays_enabled
        && inputs.solwpv_mode < 2006;
    let mut daily_average_storage = 0.0_f64;
    let mut daily_average_upper_limit = 0.0_f64;
    let mut daily_average_hk = 0.0_f64;
    for (index, layer) in layers.iter().enumerate() {
        if active_layers.0[index] {
            metrics.capacity_m += (layer.theta_m - layer.lateral_withdrawal_threshold_m()).max(0.0);
        }
        if !active_layers.1[index] {
            continue;
        }
        let layer_weight = layer.depth_m / saturated_depth_m;
        metrics.conductivity_active_depth_sum += layer.depth_m;
        metrics.avpora += layer.porosity * layer_weight;
        metrics.avfca += layer.field_capacity_theta * layer_weight;
        metrics.avcoca += layer.coca * layer_weight;
        if legacy_daily_lateral {
            let layer_hk = lateral_layer_hk(layer)?;
            let effective_upper_limit_m = (layer.upper_limit_m - layer.frozen_water_m).max(0.0);
            metrics.conductivity_depth_sum += layer.conductivity_m_s * layer.depth_m;
            daily_average_storage += layer.theta_m * layer_weight;
            daily_average_upper_limit += effective_upper_limit_m * layer_weight;
            daily_average_hk += layer_hk * layer_weight;
        } else if inputs.lane_substeps == 1 && !inputs.mofe_hourly_carry_arrays_enabled {
            add_daily_lateral_conductivity(layer, inputs, &mut metrics)?;
        } else {
            let saturation_fraction = hourly_lateral_saturation_fraction(layer)?;
            metrics.legacy_saturation_fraction = saturation_fraction;
            metrics.conductivity_depth_sum +=
                layer.lateral_conductivity_m_s * saturation_fraction * layer.depth_m;
        }
    }
    if legacy_daily_lateral && daily_average_upper_limit > 0.001 {
        let saturation_fraction = daily_average_storage / daily_average_upper_limit;
        metrics.legacy_saturation_fraction = if saturation_fraction < 0.95 {
            saturation_fraction.powf(daily_average_hk).max(0.002)
        } else {
            1.0
        };
        validate_nonnegative_direct_m(
            "subsurface.legacy_saturation_fraction",
            metrics.legacy_saturation_fraction,
        )?;
    }
    validate_finite("subsurface.lane_substeps_f64", lane_substeps_f64)?;
    Ok(metrics)
}

fn add_daily_lateral_conductivity(
    layer: &DirectSubsurfaceLayerState,
    inputs: &DirectSubsurfaceComputeInputs,
    metrics: &mut LateralMetrics,
) -> Result<(), DirectRuntimeError> {
    if inputs.solwpv_mode < 2006 {
        let layer_hk = lateral_layer_hk(layer)?;
        let layer_weight = layer.depth_m / metrics.saturated_depth_m;
        let effective_upper_limit_m = (layer.upper_limit_m - layer.frozen_water_m).max(0.0);
        metrics.conductivity_depth_sum += layer.conductivity_m_s * layer.depth_m;
        let daily_average_storage = layer.theta_m * layer_weight;
        let daily_average_upper_limit = effective_upper_limit_m * layer_weight;
        let daily_average_hk = layer_hk * layer_weight;
        if daily_average_upper_limit > 0.001 {
            let saturation_fraction = daily_average_storage / daily_average_upper_limit;
            metrics.legacy_saturation_fraction = if saturation_fraction < 0.95 {
                saturation_fraction.powf(daily_average_hk).max(0.002)
            } else {
                1.0
            };
            validate_nonnegative_direct_m(
                "subsurface.legacy_saturation_fraction",
                metrics.legacy_saturation_fraction,
            )?;
        }
    } else {
        let layer_hk = lateral_layer_hk(layer)?;
        let effective_upper_limit_m = layer.upper_limit_m - layer.frozen_water_m;
        let saturation_fraction = if effective_upper_limit_m > 0.0 {
            layer.theta_m / effective_upper_limit_m
        } else {
            1.0
        };
        validate_nonnegative_direct_m("subsurface.daily_saturation_fraction", saturation_fraction)?;
        let conductivity_fraction = if saturation_fraction < 0.95 {
            saturation_fraction.powf(layer_hk).max(0.002)
        } else {
            1.0
        };
        validate_nonnegative_direct_m("subsurface.conductivity_fraction", conductivity_fraction)?;
        metrics.conductivity_depth_sum +=
            layer.conductivity_m_s * conductivity_fraction * layer.depth_m;
    }
    Ok(())
}

fn lateral_layer_hk(layer: &DirectSubsurfaceLayerState) -> Result<f64, DirectRuntimeError> {
    let fc_upper_ratio = layer.field_capacity_m / layer.upper_limit_m;
    if fc_upper_ratio <= 0.0 {
        return Ok(0.0);
    }
    let computed_hk = -2.655 / fc_upper_ratio.log10();
    validate_nonnegative_direct_m("subsurface.lateral_hk", computed_hk)?;
    Ok(computed_hk)
}

fn hourly_lateral_saturation_fraction(
    layer: &DirectSubsurfaceLayerState,
) -> Result<f64, DirectRuntimeError> {
    let drain_threshold_m = layer.drain_threshold_m();
    let storage_excess_m = (layer.theta_m - drain_threshold_m).max(0.0);
    let saturation_denominator_m = layer.upper_limit_m - drain_threshold_m;
    validate_positive(
        "subsurface.hourly_saturation_denominator_m",
        saturation_denominator_m,
    )?;
    let saturation_fraction = (storage_excess_m / saturation_denominator_m).clamp(0.0, 1.0);
    validate_nonnegative_direct_m("subsurface.hourly_saturation_fraction", saturation_fraction)?;
    Ok(saturation_fraction)
}

fn lateral_potential(
    inputs: &DirectSubsurfaceComputeInputs,
    metrics: &LateralMetrics,
    lane_substeps_f64: f64,
) -> Result<f64, DirectRuntimeError> {
    if metrics.saturated_depth_m <= WB11_ZERO_THRESHOLD
        || metrics.conductivity_active_depth_sum <= WB11_ZERO_THRESHOLD
    {
        return Ok(0.0);
    }
    let mut ke = (86_400.0 / lane_substeps_f64)
        * (metrics.conductivity_depth_sum / metrics.conductivity_active_depth_sum);
    if inputs.solwpv_mode < 2006 {
        ke *= metrics.legacy_saturation_fraction;
    }
    validate_nonnegative_direct_m("subsurface.ke", ke)?;
    let slope_factor = inputs.avg_slope.atan().sin();
    validate_nonnegative_direct_m("subsurface.slope_factor", slope_factor)?;
    let potential_m =
        (metrics.saturated_depth_m * inputs.lateral_anisotropy_ratio * ke * slope_factor)
            / inputs.slope_length_m;
    validate_nonnegative_direct_m("subsurface.lateral_potential_m", potential_m)?;
    Ok(potential_m)
}

fn lateral_depths(
    inputs: &DirectSubsurfaceComputeInputs,
    metrics: &LateralMetrics,
    lateral_substep_m: f64,
) -> Result<(f64, f64, f64), DirectRuntimeError> {
    if metrics.saturated_depth_m <= WB11_ZERO_THRESHOLD {
        return Ok((0.0, 0.0, inputs.soil_depth_m));
    }
    let water_yield_m = metrics.avpora - (metrics.avfca + (1.0 - metrics.avcoca));
    validate_finite("subsurface.water_yield_m", water_yield_m)?;
    let mut saturated_depth_after_m = metrics.saturated_depth_m;
    if inputs.solwpv_mode < 2006 {
        if lateral_substep_m > WB11_ZERO_THRESHOLD && water_yield_m <= WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "subsurface.water_yield_m",
            });
        }
        saturated_depth_after_m =
            (metrics.saturated_depth_m - (lateral_substep_m / water_yield_m)).max(0.0);
    }
    let unsaturated_depth_after_m = (inputs.soil_depth_m - saturated_depth_after_m).max(0.0);
    Ok((
        water_yield_m,
        saturated_depth_after_m,
        unsaturated_depth_after_m,
    ))
}

fn record_surface_saturation_carry(
    layers: &mut [DirectSubsurfaceLayerState],
    inputs: &DirectSubsurfaceComputeInputs,
    substep_index: usize,
    run: &mut LateralRun,
    day_index: usize,
    lane_index: usize,
) -> Result<(), DirectRuntimeError> {
    if !inputs.mofe_hourly_carry_arrays_enabled {
        return Ok(());
    }
    let theta_before_m = layers[0].theta_m;
    let top_effective_upper_limit_m = (layers[0].upper_limit_m - layers[0].frozen_water_m).max(0.0);
    let saturation_excess_m = theta_before_m - top_effective_upper_limit_m;
    let current_saturation_runoff_m = if saturation_excess_m > WB11_ZERO_THRESHOLD {
        layers[0].theta_m = top_effective_upper_limit_m;
        saturation_excess_m
    } else {
        0.0
    };
    validate_nonnegative_direct_m(
        "subsurface.hourly_saturation_carry_m",
        current_saturation_runoff_m,
    )?;
    run.hourly_saturation_carry[substep_index] = current_saturation_runoff_m;
    maybe_write_r7h_subsurface_saturation_trace(
        &DirectSubsurfaceSaturationTraceEvent::from_substep(
            day_index,
            lane_index,
            substep_index,
            inputs,
            theta_before_m,
            layers[0].upper_limit_m,
            layers[0].frozen_water_m,
            top_effective_upper_limit_m,
            saturation_excess_m,
            current_saturation_runoff_m,
            layers[0].theta_m,
        ),
    );
    Ok(())
}

fn drain_thresholds(layers: &[DirectSubsurfaceLayerState]) -> Result<Vec<f64>, DirectRuntimeError> {
    layers
        .iter()
        .map(|layer| {
            let threshold = layer.drain_threshold_m();
            validate_nonnegative_direct_m("subsurface.drain_threshold_m", threshold)?;
            Ok(threshold)
        })
        .collect()
}

fn lateral_withdrawal_thresholds(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<Vec<f64>, DirectRuntimeError> {
    layers
        .iter()
        .map(|layer| {
            let threshold = layer.lateral_withdrawal_threshold_m();
            validate_nonnegative_direct_m("subsurface.lateral_withdrawal_threshold_m", threshold)?;
            Ok(threshold)
        })
        .collect()
}

fn drainable_storage(layers: &[DirectSubsurfaceLayerState], thresholds: &[f64]) -> f64 {
    layers
        .iter()
        .zip(thresholds.iter())
        .map(|(layer, threshold)| (layer.theta_m - *threshold).max(0.0))
        .sum()
}

fn withdraw_tile_to_surface(
    layers: &mut [DirectSubsurfaceLayerState],
    thresholds: &[f64],
    tile_layer_index: usize,
    amount_m: f64,
) -> f64 {
    let mut remaining_m = amount_m.max(0.0);
    if layers.is_empty() {
        return 0.0;
    }
    let upper_layer = tile_layer_index.min(layers.len() - 1);
    for layer_index in (0..=upper_layer).rev() {
        if remaining_m <= WB11_ZERO_THRESHOLD {
            break;
        }
        let available_m = (layers[layer_index].theta_m - thresholds[layer_index]).max(0.0);
        if available_m > WB11_ZERO_THRESHOLD {
            let withdrawn_m = available_m.min(remaining_m);
            layers[layer_index].theta_m -= withdrawn_m;
            remaining_m -= withdrawn_m;
        }
    }
    amount_m.max(0.0) - remaining_m.max(0.0)
}

fn withdraw_top_down(
    layers: &mut [DirectSubsurfaceLayerState],
    thresholds: &[f64],
    amount_m: f64,
    layer_withdrawal_m: &mut [f64],
) -> f64 {
    let mut remaining_m = amount_m.max(0.0);
    for (index, layer) in layers.iter_mut().enumerate() {
        if remaining_m <= WB11_ZERO_THRESHOLD {
            break;
        }
        let available_m = (layer.theta_m - thresholds[index]).max(0.0);
        if available_m <= WB11_ZERO_THRESHOLD {
            continue;
        }
        let withdrawn_m = available_m.min(remaining_m);
        layer.theta_m -= withdrawn_m;
        layer_withdrawal_m[index] += withdrawn_m;
        remaining_m -= withdrawn_m;
    }
    amount_m.max(0.0) - remaining_m.max(0.0)
}

fn apply_soil_water_withdrawal(
    soil_water_before_m: f64,
    withdrawal_m: f64,
) -> Result<f64, DirectRuntimeError> {
    validate_nonnegative_direct_m("subsurface.withdrawal_m", withdrawal_m)?;
    if withdrawal_m > soil_water_before_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "subsurface.withdrawal_m",
        });
    }
    let soil_water_after_m = (soil_water_before_m - withdrawal_m).max(0.0);
    validate_nonnegative_direct_m("subsurface.soil_water_after_m", soil_water_after_m)?;
    Ok(soil_water_after_m)
}
