use super::{
    DIRECT_AUDIT, DIRECT_R4B_PHASE_SPAN_COUNT, DIRECT_R4C_PHASE_SPAN_COUNT,
    DIRECT_R4D_PHASE_SPAN_COUNT, DIRECT_R4E_PHASE_SPAN_COUNT, DIRECT_R4F_PHASE_SPAN_COUNT,
    DIRECT_R4G_PHASE_SPAN_COUNT, DirectDayFrame, DirectRuntimeError, DirectSnowLaneState,
    DirectSnowRuntimeCarry, DirectSnowStage3Diagnostics, DirectSubsurfaceLayerState,
    SnowAlbedoState, WB11_ZERO_THRESHOLD, validate_finite, validate_nonnegative_direct_m,
};
use crate::winter_column::DirectSnowLayerState;

#[derive(Debug, Clone)]
struct R7hStorageTraceConfig {
    path: std::path::PathBuf,
    exact_day_index: Option<usize>,
    exact_lane_index: Option<usize>,
}

static R7H_STORAGE_TRACE_CONFIG: std::sync::OnceLock<Option<R7hStorageTraceConfig>> =
    std::sync::OnceLock::new();

fn r7h_storage_trace_config() -> Option<&'static R7hStorageTraceConfig> {
    R7H_STORAGE_TRACE_CONFIG
        .get_or_init(|| {
            let path = std::env::var_os("OPENWEPP_R7H_STORAGE_TRACE_PATH")?;
            if path.is_empty() {
                return None;
            }
            Some(R7hStorageTraceConfig {
                path: std::path::PathBuf::from(path),
                exact_day_index: r7h_storage_trace_env_usize(
                    "OPENWEPP_R7H_STORAGE_TRACE_DAY_INDEX",
                ),
                exact_lane_index: r7h_storage_trace_env_usize(
                    "OPENWEPP_R7H_STORAGE_TRACE_LANE_INDEX",
                ),
            })
        })
        .as_ref()
}

fn r7h_storage_trace_env_usize(name: &str) -> Option<usize> {
    let value = std::env::var(name).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<usize>().ok()
}

fn r7h_storage_trace_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.17}")
    } else {
        "null".to_string()
    }
}

fn r7h_storage_trace_f64_array(values: impl IntoIterator<Item = f64>) -> String {
    let mut output = String::from("[");
    let mut first = true;
    for value in values {
        if !first {
            output.push(',');
        }
        first = false;
        output.push_str(&r7h_storage_trace_number(value));
    }
    output.push(']');
    output
}

fn r7h_storage_trace_layer_aggregate_m(day_frame: &DirectDayFrame) -> f64 {
    let mut aggregate_m = 0.0;
    for layer in &day_frame
        .evapotranspiration_compute
        .layer_state_after_root_uptake
    {
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        aggregate_m += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
    }
    aggregate_m
}

fn r4b_aggregate_liquid_soil_water(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, DirectRuntimeError> {
    let mut aggregate_m = 0.0;
    for layer in layers {
        validate_nonnegative_direct_m(
            "storage_reconciliation.frost_storage_projection_theta_m",
            layer.theta_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.frost_storage_projection_depth_m",
            layer.depth_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.frost_storage_projection_frozen_depth_m",
            layer.frozen_depth_m,
        )?;
        if layer.frozen_depth_m > layer.depth_m + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "storage_reconciliation.frost_storage_projection_frozen_depth_m",
            });
        }
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        aggregate_m += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        validate_finite(
            "storage_reconciliation.frost_storage_projection_aggregate_m",
            aggregate_m,
        )?;
    }
    Ok(aggregate_m)
}

fn r4b_available_active_theta_m(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, DirectRuntimeError> {
    let mut available_m = 0.0;
    for layer in layers {
        validate_nonnegative_direct_m(
            "storage_reconciliation.frost_storage_projection_theta_m",
            layer.theta_m,
        )?;
        available_m += layer.theta_m;
        validate_finite(
            "storage_reconciliation.frost_storage_projection_available_theta_m",
            available_m,
        )?;
    }
    Ok(available_m)
}

fn r4b_apply_explicit_frost_storage_projection_delta(
    layers: &mut [DirectSubsurfaceLayerState],
    delta_m: f64,
) -> Result<(), DirectRuntimeError> {
    if layers.is_empty() {
        return Err(DirectRuntimeError::MissingDirectUpstream {
            upstream: "R4N evapotranspiration/root-uptake producer",
        });
    }
    if delta_m > 0.0 {
        layers[0].theta_m += delta_m;
        validate_nonnegative_direct_m(
            "storage_reconciliation.frost_storage_projection_theta_m",
            layers[0].theta_m,
        )?;
        validate_finite(
            "storage_reconciliation.frost_storage_projection_theta_m",
            layers[0].theta_m,
        )?;
        return Ok(());
    }

    let mut remaining_debit_m = -delta_m;
    let available_m = r4b_available_active_theta_m(layers)?;
    if available_m + WB11_ZERO_THRESHOLD < remaining_debit_m {
        return Err(DirectRuntimeError::NegativeDirectValue {
            field: "storage_reconciliation.frost_storage_projection_theta_m",
        });
    }

    for layer in layers {
        if remaining_debit_m <= WB11_ZERO_THRESHOLD {
            break;
        }
        let debit_m = layer.theta_m.min(remaining_debit_m);
        layer.theta_m -= debit_m;
        if layer.theta_m < 0.0 && layer.theta_m.abs() <= WB11_ZERO_THRESHOLD {
            layer.theta_m = 0.0;
        }
        validate_nonnegative_direct_m(
            "storage_reconciliation.frost_storage_projection_theta_m",
            layer.theta_m,
        )?;
        validate_finite(
            "storage_reconciliation.frost_storage_projection_theta_m",
            layer.theta_m,
        )?;
        remaining_debit_m -= debit_m;
        validate_finite(
            "storage_reconciliation.frost_storage_projection_remaining_debit_m",
            remaining_debit_m,
        )?;
    }
    if remaining_debit_m > WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::NegativeDirectValue {
            field: "storage_reconciliation.frost_storage_projection_theta_m",
        });
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn maybe_write_r7h_storage_trace(day_frame: &DirectDayFrame) {
    let Some(config) = r7h_storage_trace_config() else {
        return;
    };
    if !r7h_storage_trace_allows(config, day_frame) {
        return;
    }

    let storage = day_frame.storage_reconciliation;
    let mut line = String::new();
    line.push('{');
    line.push_str("\"schema\":\"openwepp-r7h-storage-trace-v1\"");
    line.push_str(",\"day_index\":");
    line.push_str(&day_frame.day_index.to_string());
    line.push_str(",\"lane_index\":");
    line.push_str(&day_frame.lane_index.to_string());
    line.push_str(",\"storage_initial_m\":");
    line.push_str(&r7h_storage_trace_number(storage.storage_initial_m));
    line.push_str(",\"precip_input_m\":");
    line.push_str(&r7h_storage_trace_number(storage.precip_input_m));
    line.push_str(",\"snow_coupling_m\":");
    line.push_str(&r7h_storage_trace_number(storage.snow_coupling_m));
    line.push_str(",\"frost_liquid_delta_m\":");
    line.push_str(&r7h_storage_trace_number(storage.frost_liquid_delta_m));
    line.push_str(",\"runon_input_m\":");
    line.push_str(&r7h_storage_trace_number(storage.runon_input_m));
    line.push_str(",\"runoff_liquid_input_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame.runoff_partition.liquid_input_m,
    ));
    line.push_str(",\"runoff_runon_input_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame.runoff_partition.runon_input_m,
    ));
    line.push_str(",\"runoff_cumulative_infiltration_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame.runoff_partition.cumulative_infiltration_m,
    ));
    line.push_str(",\"runoff_depression_storage_delta_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame.runoff_partition.depression_storage_delta_m,
    ));
    line.push_str(",\"runoff_frost_retained_local_liquid_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame
            .runoff_partition_inputs
            .frost_retained_local_liquid_m,
    ));
    line.push_str(",\"runoff_frost_preprojected_local_liquid_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame
            .runoff_partition_inputs
            .frost_preprojected_local_liquid_m,
    ));
    line.push_str(",\"runoff_partition_runoff_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame.runoff_partition.partition_runoff_m,
    ));
    line.push_str(",\"surface_saturation_runoff_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame.saturation_addback.surface_saturation_runoff_m,
    ));
    line.push_str(",\"interception_m\":");
    line.push_str(&r7h_storage_trace_number(storage.interception_m));
    line.push_str(",\"q_runoff_m\":");
    line.push_str(&r7h_storage_trace_number(storage.q_runoff_m));
    line.push_str(",\"evapotranspiration_m\":");
    line.push_str(&r7h_storage_trace_number(storage.evapotranspiration_m));
    if let Some(pmet) = day_frame.evapotranspiration_compute_inputs.pmet {
        line.push_str(",\"pmet_soil_evaporation_m\":");
        line.push_str(&r7h_storage_trace_number(pmet.soil_evaporation_m));
        line.push_str(",\"pmet_plant_transpiration_m\":");
        line.push_str(&r7h_storage_trace_number(pmet.plant_transpiration_m));
        line.push_str(",\"pmet_soil_evaporation_storage_return_m\":");
        line.push_str(&r7h_storage_trace_number(
            pmet.soil_evaporation_storage_return_m,
        ));
    } else {
        line.push_str(",\"pmet_soil_evaporation_m\":null");
        line.push_str(",\"pmet_plant_transpiration_m\":null");
        line.push_str(",\"pmet_soil_evaporation_storage_return_m\":null");
    }
    line.push_str(",\"deep_seepage_m\":");
    line.push_str(&r7h_storage_trace_number(storage.deep_seepage_m));
    line.push_str(",\"subsurface_loss_m\":");
    line.push_str(&r7h_storage_trace_number(storage.subsurface_loss_m));
    line.push_str(",\"storage_reconciled_m\":");
    line.push_str(&r7h_storage_trace_number(storage.storage_reconciled_m));
    line.push_str(",\"root_uptake_layer_aggregate_m\":");
    line.push_str(&r7h_storage_trace_number(
        r7h_storage_trace_layer_aggregate_m(day_frame),
    ));
    line.push_str(",\"root_uptake_soil_water_after_m\":");
    line.push_str(&r7h_storage_trace_number(
        day_frame.evapotranspiration_compute.soil_water_after_m,
    ));
    line.push_str(",\"root_layer_theta_m\":");
    line.push_str(&r7h_storage_trace_f64_array(
        day_frame
            .evapotranspiration_compute
            .layer_state_after_root_uptake
            .iter()
            .map(|layer| layer.theta_m),
    ));
    line.push_str(",\"root_layer_upper_limit_m\":");
    line.push_str(&r7h_storage_trace_f64_array(
        day_frame
            .evapotranspiration_compute
            .layer_state_after_root_uptake
            .iter()
            .map(|layer| layer.upper_limit_m),
    ));
    line.push_str(",\"root_layer_frozen_water_m\":");
    line.push_str(&r7h_storage_trace_f64_array(
        day_frame
            .evapotranspiration_compute
            .layer_state_after_root_uptake
            .iter()
            .map(|layer| layer.frozen_water_m),
    ));
    line.push_str(",\"closure_residual_m\":");
    line.push_str(&r7h_storage_trace_number(storage.closure_residual_m));
    line.push('}');
    line.push('\n');

    r7h_storage_append_trace_line(&config.path, &line);
}

fn r7h_storage_trace_allows(config: &R7hStorageTraceConfig, day_frame: &DirectDayFrame) -> bool {
    if let Some(exact_day_index) = config.exact_day_index
        && day_frame.day_index != exact_day_index
    {
        return false;
    }
    if let Some(exact_lane_index) = config.exact_lane_index
        && day_frame.lane_index != exact_lane_index
    {
        return false;
    }
    true
}

fn r7h_storage_append_trace_line(path: &std::path::Path, line: &str) {
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

    pub fn run_r4d_deep_seepage_span(
        &mut self,
    ) -> Result<DirectDeepSeepageSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4D_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let deep_seepage = self.compute_r4d_deep_seepage()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.deep_seepage = deep_seepage;
        self.storage_reconciliation_inputs.deep_seepage_m = deep_seepage.deep_seepage_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.deep_seepage_downstream_operands =
            DirectDeepSeepageDownstreamOperands::from(deep_seepage);
        DIRECT_AUDIT.record_downstream_operand_production();

        let deep_seepage_shadow_projection = DirectDeepSeepageShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            deep_seepage_m: self.deep_seepage_downstream_operands.deep_seepage_m,
        };
        self.deep_seepage_shadow_projection = Some(deep_seepage_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectDeepSeepageSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            deep_seepage_shadow_projection,
        })
    }

    pub fn run_r4e_subsurface_loss_span(
        &mut self,
    ) -> Result<DirectSubsurfaceLossSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4E_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let subsurface_loss = self.compute_r4e_subsurface_loss()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.subsurface_loss = subsurface_loss;
        self.storage_reconciliation_inputs.subsurface_loss_m = subsurface_loss.subsurface_loss_m;
        DIRECT_AUDIT.record_direct_state_mutation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.subsurface_loss_downstream_operands =
            DirectSubsurfaceLossDownstreamOperands::from(subsurface_loss);
        DIRECT_AUDIT.record_downstream_operand_production();

        let subsurface_loss_shadow_projection = DirectSubsurfaceLossShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            subsurface_loss_m: self.subsurface_loss_downstream_operands.subsurface_loss_m,
        };
        self.subsurface_loss_shadow_projection = Some(subsurface_loss_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectSubsurfaceLossSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            subsurface_loss_shadow_projection,
        })
    }

    pub fn run_r4f_evapotranspiration_span(
        &mut self,
    ) -> Result<DirectEvapotranspirationSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4F_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let evapotranspiration = self.compute_r4f_evapotranspiration()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.evapotranspiration = evapotranspiration;
        self.water.evapotranspiration_m = evapotranspiration.evapotranspiration_m;
        self.storage_reconciliation_inputs.evapotranspiration_m =
            evapotranspiration.evapotranspiration_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.evapotranspiration_downstream_operands =
            DirectEvapotranspirationDownstreamOperands::from(evapotranspiration);
        DIRECT_AUDIT.record_downstream_operand_production();

        let evapotranspiration_shadow_projection = DirectEvapotranspirationShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            evapotranspiration_m: self
                .evapotranspiration_downstream_operands
                .evapotranspiration_m,
        };
        self.evapotranspiration_shadow_projection = Some(evapotranspiration_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectEvapotranspirationSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            evapotranspiration_shadow_projection,
        })
    }

    pub fn run_r4g_snow_coupling_span(
        &mut self,
    ) -> Result<DirectSnowCouplingSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4G_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let snow_coupling = self.compute_r4g_snow_coupling()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.snow_coupling = snow_coupling.clone();
        if snow_coupling.snow_state_projected {
            self.winter_column.snow =
                DirectSnowLaneState::from_runtime_values_boundary_liquid_albedo_and_layers(
                    snow_coupling.runtime_swe_after_m,
                    snow_coupling.runtime_depth_after_m,
                    snow_coupling.runtime_density_after_kg_m3,
                    snow_coupling.runtime_settle_day_count_after,
                    snow_coupling.coe_boundary_depth_after_m,
                    snow_coupling.coe_boundary_density_after_kg_m3,
                    snow_coupling.coe_boundary_settle_day_count_after,
                    snow_coupling.liquid_water_retained_after_m,
                    snow_coupling.snow_albedo_state_after,
                    snow_coupling.snow_layers_after.clone(),
                );
            self.snow_runtime_carry = Some(DirectSnowRuntimeCarry::from(&self.winter_column.snow));
        }
        self.storage_reconciliation_inputs.snow_coupling_m = snow_coupling.snow_coupling_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.snow_coupling_downstream_operands =
            DirectSnowCouplingDownstreamOperands::from(snow_coupling);
        DIRECT_AUDIT.record_downstream_operand_production();

        let snow_coupling_shadow_projection = DirectSnowCouplingShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            snow_coupling_m: self.snow_coupling_downstream_operands.snow_coupling_m,
            active_snow_coupling: self.snow_coupling_downstream_operands.active_snow_coupling,
            raw_melt_m: self.snow_coupling_downstream_operands.raw_melt_m,
            redistributed_melt_m: self.snow_coupling_downstream_operands.redistributed_melt_m,
            routed_melt_m: self.snow_coupling_downstream_operands.routed_melt_m,
            snowpack_swe_loss_m: self.snow_coupling_downstream_operands.snowpack_swe_loss_m,
            sublimation_m: self.snow_coupling_downstream_operands.sublimation_m,
            post_winter_rain_m: self.snow_coupling_downstream_operands.post_winter_rain_m,
            runtime_swe_after_m: self.snow_coupling_downstream_operands.runtime_swe_after_m,
            runtime_depth_after_m: self.snow_coupling_downstream_operands.runtime_depth_after_m,
            runtime_density_after_kg_m3: self
                .snow_coupling_downstream_operands
                .runtime_density_after_kg_m3,
            runtime_settle_day_count_after: self
                .snow_coupling_downstream_operands
                .runtime_settle_day_count_after,
            coe_boundary_depth_after_m: self
                .snow_coupling_downstream_operands
                .coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: self
                .snow_coupling_downstream_operands
                .coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: self
                .snow_coupling_downstream_operands
                .coe_boundary_settle_day_count_after,
            snow_albedo_state_after: self
                .snow_coupling_downstream_operands
                .snow_albedo_state_after,
            stage3_diagnostics: self
                .snow_coupling_downstream_operands
                .stage3_diagnostics
                .clone(),
        };
        self.snow_coupling_shadow_projection = Some(snow_coupling_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectSnowCouplingSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            snow_coupling_shadow_projection,
        })
    }

    pub fn run_r4b_storage_reconciliation_span(
        &mut self,
    ) -> Result<DirectStorageReconciliationSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4B_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;
        let mut state_mutation_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        if let Some(frost_storage_liquid_delta_m) = self.frost_storage_liquid_delta_m {
            validate_finite(
                "storage_reconciliation.frost_storage_liquid_delta_m",
                frost_storage_liquid_delta_m,
            )?;
            self.storage_reconciliation_inputs.frost_liquid_delta_m = frost_storage_liquid_delta_m;
            DIRECT_AUDIT.record_direct_state_mutation();
            state_mutation_count += 1;
        }
        let storage_reconciliation = self.compute_r4b_storage_reconciliation()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.storage_reconciliation = storage_reconciliation;
        self.water.soil_water_m = storage_reconciliation.storage_reconciled_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        state_mutation_count += 1;
        if self.rebalance_r4b_explicit_frost_storage_projection(
            storage_reconciliation.storage_reconciled_m,
        )? {
            DIRECT_AUDIT.record_direct_state_mutation();
            state_mutation_count += 1;
        }
        self.storage_downstream_operands =
            DirectStorageDownstreamOperands::from(storage_reconciliation);
        DIRECT_AUDIT.record_downstream_operand_production();

        let storage_shadow_projection = DirectStorageShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            storage_initial_m: self.storage_downstream_operands.storage_initial_m,
            precip_input_m: self.storage_downstream_operands.precip_input_m,
            snow_coupling_m: self.storage_downstream_operands.snow_coupling_m,
            frost_liquid_delta_m: self.storage_downstream_operands.frost_liquid_delta_m,
            runon_input_m: self.storage_downstream_operands.runon_input_m,
            interception_m: self.storage_downstream_operands.interception_m,
            q_runoff_m: self.storage_downstream_operands.q_runoff_m,
            evapotranspiration_m: self.storage_downstream_operands.evapotranspiration_m,
            evapotranspiration_storage_return_m: self
                .storage_downstream_operands
                .evapotranspiration_storage_return_m,
            deep_seepage_m: self.storage_downstream_operands.deep_seepage_m,
            subsurface_loss_m: self.storage_downstream_operands.subsurface_loss_m,
            storage_reconciled_m: self.storage_downstream_operands.storage_reconciled_m,
            closure_residual_m: self.storage_downstream_operands.closure_residual_m,
        };
        self.storage_shadow_projection = Some(storage_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();
        maybe_write_r7h_storage_trace(self);

        Ok(DirectStorageReconciliationSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            storage_shadow_projection,
        })
    }

    fn rebalance_r4b_explicit_frost_storage_projection(
        &mut self,
        storage_reconciled_m: f64,
    ) -> Result<bool, DirectRuntimeError> {
        if self.frost_storage_liquid_delta_m.is_none() {
            return Ok(false);
        }
        // The explicit WB12 frost storage source owns only the aggregate
        // storage delta here. Layer-depth fidelity is deferred to the
        // observation-anchored frost-depth work, so this path must not infer a
        // compatibility layer distribution.
        let aggregate_m = r4b_aggregate_liquid_soil_water(
            &self
                .evapotranspiration_compute
                .layer_state_after_root_uptake,
        )?;
        let delta_m = storage_reconciled_m - aggregate_m;
        validate_finite(
            "storage_reconciliation.frost_storage_projection_delta_m",
            delta_m,
        )?;
        if delta_m.abs() <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }
        r4b_apply_explicit_frost_storage_projection_delta(
            &mut self
                .evapotranspiration_compute
                .layer_state_after_root_uptake,
            delta_m,
        )?;
        self.evapotranspiration_compute.soil_water_after_m = storage_reconciled_m;
        if let Some(shadow) = &mut self.evapotranspiration_compute_shadow_projection {
            shadow.soil_water_after_m = storage_reconciled_m;
            shadow.layer_state_after_root_uptake.clone_from(
                &self
                    .evapotranspiration_compute
                    .layer_state_after_root_uptake,
            );
        }
        Ok(true)
    }

    fn compute_r4c_storage_input(&self) -> Result<DirectStorageInputState, DirectRuntimeError> {
        self.validate_r4c_storage_input_domain()?;
        let precip_input_m = self
            .storage_input_inputs
            .precip_input_handoff_m
            .unwrap_or(self.downstream_operands.precipitation_m);
        Ok(DirectStorageInputState {
            storage_initial_m: self.water.soil_water_m,
            precip_input_m,
        })
    }

    fn compute_r4d_deep_seepage(&self) -> Result<DirectDeepSeepageState, DirectRuntimeError> {
        self.validate_r4d_deep_seepage_domain()?;
        Ok(DirectDeepSeepageState {
            deep_seepage_m: self.deep_seepage_inputs.deep_seepage_handoff_m,
        })
    }

    fn compute_r4e_subsurface_loss(&self) -> Result<DirectSubsurfaceLossState, DirectRuntimeError> {
        self.validate_r4e_subsurface_loss_domain()?;
        Ok(DirectSubsurfaceLossState {
            subsurface_loss_m: self.subsurface_loss_inputs.subsurface_loss_handoff_m,
        })
    }

    fn compute_r4f_evapotranspiration(
        &self,
    ) -> Result<DirectEvapotranspirationState, DirectRuntimeError> {
        self.validate_r4f_evapotranspiration_domain()?;
        Ok(DirectEvapotranspirationState {
            evapotranspiration_m: self.evapotranspiration_inputs.evapotranspiration_handoff_m,
        })
    }

    fn compute_r4g_snow_coupling(&self) -> Result<DirectSnowCouplingState, DirectRuntimeError> {
        self.validate_r4g_snow_coupling_domain()?;
        Ok(DirectSnowCouplingState {
            snow_coupling_m: self.snow_coupling_inputs.snow_coupling_handoff_m,
            snow_state_projected: self.snow_coupling_inputs.snow_state_projected,
            active_snow_coupling: self.snow_coupling_inputs.active_snow_coupling,
            raw_melt_m: self.snow_coupling_inputs.raw_melt_m,
            redistributed_melt_m: self.snow_coupling_inputs.redistributed_melt_m,
            routed_melt_m: self.snow_coupling_inputs.routed_melt_m,
            snowpack_swe_loss_m: self.snow_coupling_inputs.snowpack_swe_loss_m,
            sublimation_m: self.snow_coupling_inputs.sublimation_m,
            post_winter_rain_m: self.snow_coupling_inputs.post_winter_rain_m,
            runtime_swe_after_m: self.snow_coupling_inputs.runtime_swe_after_m,
            runtime_depth_after_m: self.snow_coupling_inputs.runtime_depth_after_m,
            runtime_density_after_kg_m3: self.snow_coupling_inputs.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: self
                .snow_coupling_inputs
                .runtime_settle_day_count_after,
            coe_boundary_depth_after_m: self.snow_coupling_inputs.coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: self
                .snow_coupling_inputs
                .coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: self
                .snow_coupling_inputs
                .coe_boundary_settle_day_count_after,
            liquid_holding_capacity_after_m: self
                .snow_coupling_inputs
                .liquid_holding_capacity_after_m,
            liquid_water_retained_after_m: self.snow_coupling_inputs.liquid_water_retained_after_m,
            liquid_water_released_m: self.snow_coupling_inputs.liquid_water_released_m,
            snow_albedo_state_after: self.snow_coupling_inputs.snow_albedo_state_after,
            snow_layers_after: self.snow_coupling_inputs.snow_layers_after.clone(),
            stage3_diagnostics: self.snow_coupling_inputs.stage3_diagnostics.clone(),
        })
    }

    fn compute_r4b_storage_reconciliation(
        &self,
    ) -> Result<DirectStorageReconciliationState, DirectRuntimeError> {
        self.validate_r4b_storage_reconciliation_domain()?;
        let inputs = self.storage_reconciliation_inputs;
        let q_runoff_m = self.runoff_downstream_operands.q_runoff_m;
        let storage_reconciled_m = inputs.storage_initial_m
            + inputs.precip_input_m
            + inputs.snow_coupling_m
            + inputs.runon_input_m
            + inputs.frost_liquid_delta_m
            + inputs.evapotranspiration_storage_return_m
            - inputs.interception_m
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
        let closure_residual_m = inputs.storage_initial_m
            + inputs.precip_input_m
            + inputs.snow_coupling_m
            + inputs.runon_input_m
            + inputs.frost_liquid_delta_m
            + inputs.evapotranspiration_storage_return_m
            - inputs.interception_m
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
            frost_liquid_delta_m: inputs.frost_liquid_delta_m,
            runon_input_m: inputs.runon_input_m,
            interception_m: inputs.interception_m,
            q_runoff_m,
            evapotranspiration_m: inputs.evapotranspiration_m,
            evapotranspiration_storage_return_m: inputs.evapotranspiration_storage_return_m,
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
        if let Some(precip_input_handoff_m) = self.storage_input_inputs.precip_input_handoff_m {
            validate_nonnegative_direct_m(
                "storage_input.precip_input_handoff_m",
                precip_input_handoff_m,
            )?;
        } else {
            validate_nonnegative_direct_m(
                "storage_input.precip_input_m",
                self.downstream_operands.precipitation_m,
            )?;
        }
        Ok(())
    }

    fn validate_r4d_deep_seepage_domain(&self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "deep_seepage.deep_seepage_handoff_m",
            self.deep_seepage_inputs.deep_seepage_handoff_m,
        )?;
        Ok(())
    }

    fn validate_r4e_subsurface_loss_domain(&self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "subsurface_loss.subsurface_loss_handoff_m",
            self.subsurface_loss_inputs.subsurface_loss_handoff_m,
        )?;
        Ok(())
    }

    fn validate_r4f_evapotranspiration_domain(&self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "evapotranspiration.evapotranspiration_handoff_m",
            self.evapotranspiration_inputs.evapotranspiration_handoff_m,
        )?;
        Ok(())
    }

    fn validate_r4g_snow_coupling_domain(&self) -> Result<(), DirectRuntimeError> {
        validate_finite(
            "snow_coupling.snow_coupling_handoff_m",
            self.snow_coupling_inputs.snow_coupling_handoff_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.routed_melt_m",
            self.snow_coupling_inputs.routed_melt_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.post_winter_rain_m",
            self.snow_coupling_inputs.post_winter_rain_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.runtime_swe_after_m",
            self.snow_coupling_inputs.runtime_swe_after_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.runtime_depth_after_m",
            self.snow_coupling_inputs.runtime_depth_after_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.runtime_density_after_kg_m3",
            self.snow_coupling_inputs.runtime_density_after_kg_m3,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.runtime_settle_day_count_after",
            self.snow_coupling_inputs.runtime_settle_day_count_after,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.coe_boundary_depth_after_m",
            self.snow_coupling_inputs.coe_boundary_depth_after_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.coe_boundary_density_after_kg_m3",
            self.snow_coupling_inputs.coe_boundary_density_after_kg_m3,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.coe_boundary_settle_day_count_after",
            self.snow_coupling_inputs
                .coe_boundary_settle_day_count_after,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.liquid_holding_capacity_after_m",
            self.snow_coupling_inputs.liquid_holding_capacity_after_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.liquid_water_retained_after_m",
            self.snow_coupling_inputs.liquid_water_retained_after_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.liquid_water_released_m",
            self.snow_coupling_inputs.liquid_water_released_m,
        )?;
        validate_nonnegative_direct_m(
            "snow_coupling.sublimation_m",
            self.snow_coupling_inputs.sublimation_m,
        )?;
        if self.snow_coupling_inputs.runtime_density_after_kg_m3 > 522.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "snow_coupling.runtime_density_after_kg_m3",
            });
        }
        if self.snow_coupling_inputs.coe_boundary_density_after_kg_m3 > 522.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "snow_coupling.coe_boundary_density_after_kg_m3",
            });
        }
        Ok(())
    }

    fn validate_r4b_storage_reconciliation_domain(&self) -> Result<(), DirectRuntimeError> {
        if self.storage_input_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4C storage input producer",
            });
        }
        if self.percolation_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4M percolation producer",
            });
        }
        if self.subsurface_compute_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4O subsurface compute producer",
            });
        }
        if self.evapotranspiration_compute_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4N evapotranspiration/root-uptake producer",
            });
        }
        if self.snow_coupling_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4G snow-coupling producer",
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
        validate_finite(
            "storage_reconciliation.frost_liquid_delta_m",
            self.storage_reconciliation_inputs.frost_liquid_delta_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.runon_input_m",
            self.storage_reconciliation_inputs.runon_input_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.interception_m",
            self.storage_reconciliation_inputs.interception_m,
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
            "storage_reconciliation.evapotranspiration_storage_return_m",
            self.storage_reconciliation_inputs
                .evapotranspiration_storage_return_m,
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
pub struct DirectStorageInputInputs {
    pub precip_input_handoff_m: Option<f64>,
}

impl DirectStorageInputInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            precip_input_handoff_m: None,
        }
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
pub struct DirectDeepSeepageInputs {
    pub deep_seepage_handoff_m: f64,
}

impl DirectDeepSeepageInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            deep_seepage_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectDeepSeepageState {
    pub deep_seepage_m: f64,
}

impl DirectDeepSeepageState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            deep_seepage_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectDeepSeepageDownstreamOperands {
    pub deep_seepage_m: f64,
}

impl DirectDeepSeepageDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            deep_seepage_m: 0.0,
        }
    }
}

impl From<DirectDeepSeepageState> for DirectDeepSeepageDownstreamOperands {
    fn from(state: DirectDeepSeepageState) -> Self {
        Self {
            deep_seepage_m: state.deep_seepage_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectDeepSeepageShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub deep_seepage_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSubsurfaceLossInputs {
    pub subsurface_loss_handoff_m: f64,
}

impl DirectSubsurfaceLossInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            subsurface_loss_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSubsurfaceLossState {
    pub subsurface_loss_m: f64,
}

impl DirectSubsurfaceLossState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            subsurface_loss_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSubsurfaceLossDownstreamOperands {
    pub subsurface_loss_m: f64,
}

impl DirectSubsurfaceLossDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            subsurface_loss_m: 0.0,
        }
    }
}

impl From<DirectSubsurfaceLossState> for DirectSubsurfaceLossDownstreamOperands {
    fn from(state: DirectSubsurfaceLossState) -> Self {
        Self {
            subsurface_loss_m: state.subsurface_loss_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSubsurfaceLossShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub subsurface_loss_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationInputs {
    pub evapotranspiration_handoff_m: f64,
}

impl DirectEvapotranspirationInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            evapotranspiration_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationState {
    pub evapotranspiration_m: f64,
}

impl DirectEvapotranspirationState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            evapotranspiration_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationDownstreamOperands {
    pub evapotranspiration_m: f64,
}

impl DirectEvapotranspirationDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            evapotranspiration_m: 0.0,
        }
    }
}

impl From<DirectEvapotranspirationState> for DirectEvapotranspirationDownstreamOperands {
    fn from(state: DirectEvapotranspirationState) -> Self {
        Self {
            evapotranspiration_m: state.evapotranspiration_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub evapotranspiration_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowCouplingInputs {
    pub snow_coupling_handoff_m: f64,
    pub snow_state_projected: bool,
    pub active_snow_coupling: bool,
    pub raw_melt_m: f64,
    pub redistributed_melt_m: f64,
    pub routed_melt_m: f64,
    pub snowpack_swe_loss_m: f64,
    pub sublimation_m: f64,
    pub post_winter_rain_m: f64,
    pub runtime_swe_after_m: f64,
    pub runtime_depth_after_m: f64,
    pub runtime_density_after_kg_m3: f64,
    pub runtime_settle_day_count_after: f64,
    pub coe_boundary_depth_after_m: f64,
    pub coe_boundary_density_after_kg_m3: f64,
    pub coe_boundary_settle_day_count_after: f64,
    pub liquid_holding_capacity_after_m: f64,
    pub liquid_water_retained_after_m: f64,
    pub liquid_water_released_m: f64,
    pub snow_albedo_state_after: Option<SnowAlbedoState>,
    pub snow_layers_after: Vec<DirectSnowLayerState>,
    pub stage3_diagnostics: Option<Box<DirectSnowStage3Diagnostics>>,
}

impl DirectSnowCouplingInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            snow_coupling_handoff_m: 0.0,
            snow_state_projected: false,
            active_snow_coupling: false,
            raw_melt_m: 0.0,
            redistributed_melt_m: 0.0,
            routed_melt_m: 0.0,
            snowpack_swe_loss_m: 0.0,
            sublimation_m: 0.0,
            post_winter_rain_m: 0.0,
            runtime_swe_after_m: 0.0,
            runtime_depth_after_m: 0.0,
            runtime_density_after_kg_m3: 0.0,
            runtime_settle_day_count_after: 0.0,
            coe_boundary_depth_after_m: 0.0,
            coe_boundary_density_after_kg_m3: 0.0,
            coe_boundary_settle_day_count_after: 0.0,
            liquid_holding_capacity_after_m: 0.0,
            liquid_water_retained_after_m: 0.0,
            liquid_water_released_m: 0.0,
            snow_albedo_state_after: None,
            snow_layers_after: Vec::new(),
            stage3_diagnostics: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowCouplingState {
    pub snow_coupling_m: f64,
    pub snow_state_projected: bool,
    pub active_snow_coupling: bool,
    pub raw_melt_m: f64,
    pub redistributed_melt_m: f64,
    pub routed_melt_m: f64,
    pub snowpack_swe_loss_m: f64,
    pub sublimation_m: f64,
    pub post_winter_rain_m: f64,
    pub runtime_swe_after_m: f64,
    pub runtime_depth_after_m: f64,
    pub runtime_density_after_kg_m3: f64,
    pub runtime_settle_day_count_after: f64,
    pub coe_boundary_depth_after_m: f64,
    pub coe_boundary_density_after_kg_m3: f64,
    pub coe_boundary_settle_day_count_after: f64,
    pub liquid_holding_capacity_after_m: f64,
    pub liquid_water_retained_after_m: f64,
    pub liquid_water_released_m: f64,
    pub snow_albedo_state_after: Option<SnowAlbedoState>,
    pub snow_layers_after: Vec<DirectSnowLayerState>,
    pub stage3_diagnostics: Option<Box<DirectSnowStage3Diagnostics>>,
}

impl DirectSnowCouplingState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            snow_coupling_m: 0.0,
            snow_state_projected: false,
            active_snow_coupling: false,
            raw_melt_m: 0.0,
            redistributed_melt_m: 0.0,
            routed_melt_m: 0.0,
            snowpack_swe_loss_m: 0.0,
            sublimation_m: 0.0,
            post_winter_rain_m: 0.0,
            runtime_swe_after_m: 0.0,
            runtime_depth_after_m: 0.0,
            runtime_density_after_kg_m3: 0.0,
            runtime_settle_day_count_after: 0.0,
            coe_boundary_depth_after_m: 0.0,
            coe_boundary_density_after_kg_m3: 0.0,
            coe_boundary_settle_day_count_after: 0.0,
            liquid_holding_capacity_after_m: 0.0,
            liquid_water_retained_after_m: 0.0,
            liquid_water_released_m: 0.0,
            snow_albedo_state_after: None,
            snow_layers_after: Vec::new(),
            stage3_diagnostics: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowCouplingDownstreamOperands {
    pub snow_coupling_m: f64,
    pub active_snow_coupling: bool,
    pub raw_melt_m: f64,
    pub redistributed_melt_m: f64,
    pub routed_melt_m: f64,
    pub snowpack_swe_loss_m: f64,
    pub sublimation_m: f64,
    pub post_winter_rain_m: f64,
    pub runtime_swe_after_m: f64,
    pub runtime_depth_after_m: f64,
    pub runtime_density_after_kg_m3: f64,
    pub runtime_settle_day_count_after: f64,
    pub coe_boundary_depth_after_m: f64,
    pub coe_boundary_density_after_kg_m3: f64,
    pub coe_boundary_settle_day_count_after: f64,
    pub liquid_holding_capacity_after_m: f64,
    pub liquid_water_retained_after_m: f64,
    pub liquid_water_released_m: f64,
    pub snow_albedo_state_after: Option<SnowAlbedoState>,
    pub stage3_diagnostics: Option<Box<DirectSnowStage3Diagnostics>>,
}

impl DirectSnowCouplingDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            snow_coupling_m: 0.0,
            active_snow_coupling: false,
            raw_melt_m: 0.0,
            redistributed_melt_m: 0.0,
            routed_melt_m: 0.0,
            snowpack_swe_loss_m: 0.0,
            sublimation_m: 0.0,
            post_winter_rain_m: 0.0,
            runtime_swe_after_m: 0.0,
            runtime_depth_after_m: 0.0,
            runtime_density_after_kg_m3: 0.0,
            runtime_settle_day_count_after: 0.0,
            coe_boundary_depth_after_m: 0.0,
            coe_boundary_density_after_kg_m3: 0.0,
            coe_boundary_settle_day_count_after: 0.0,
            liquid_holding_capacity_after_m: 0.0,
            liquid_water_retained_after_m: 0.0,
            liquid_water_released_m: 0.0,
            snow_albedo_state_after: None,
            stage3_diagnostics: None,
        }
    }
}

impl From<DirectSnowCouplingState> for DirectSnowCouplingDownstreamOperands {
    fn from(state: DirectSnowCouplingState) -> Self {
        Self {
            snow_coupling_m: state.snow_coupling_m,
            active_snow_coupling: state.active_snow_coupling,
            raw_melt_m: state.raw_melt_m,
            redistributed_melt_m: state.redistributed_melt_m,
            routed_melt_m: state.routed_melt_m,
            snowpack_swe_loss_m: state.snowpack_swe_loss_m,
            sublimation_m: state.sublimation_m,
            post_winter_rain_m: state.post_winter_rain_m,
            runtime_swe_after_m: state.runtime_swe_after_m,
            runtime_depth_after_m: state.runtime_depth_after_m,
            runtime_density_after_kg_m3: state.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: state.runtime_settle_day_count_after,
            coe_boundary_depth_after_m: state.coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: state.coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: state.coe_boundary_settle_day_count_after,
            liquid_holding_capacity_after_m: state.liquid_holding_capacity_after_m,
            liquid_water_retained_after_m: state.liquid_water_retained_after_m,
            liquid_water_released_m: state.liquid_water_released_m,
            snow_albedo_state_after: state.snow_albedo_state_after,
            stage3_diagnostics: state.stage3_diagnostics,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowCouplingShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub snow_coupling_m: f64,
    pub active_snow_coupling: bool,
    pub raw_melt_m: f64,
    pub redistributed_melt_m: f64,
    pub routed_melt_m: f64,
    pub snowpack_swe_loss_m: f64,
    pub sublimation_m: f64,
    pub post_winter_rain_m: f64,
    pub runtime_swe_after_m: f64,
    pub runtime_depth_after_m: f64,
    pub runtime_density_after_kg_m3: f64,
    pub runtime_settle_day_count_after: f64,
    pub coe_boundary_depth_after_m: f64,
    pub coe_boundary_density_after_kg_m3: f64,
    pub coe_boundary_settle_day_count_after: f64,
    pub snow_albedo_state_after: Option<SnowAlbedoState>,
    pub stage3_diagnostics: Option<Box<DirectSnowStage3Diagnostics>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageReconciliationInputs {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub frost_liquid_delta_m: f64,
    pub runon_input_m: f64,
    pub interception_m: f64,
    pub evapotranspiration_m: f64,
    pub evapotranspiration_storage_return_m: f64,
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
            frost_liquid_delta_m: 0.0,
            runon_input_m: 0.0,
            interception_m: 0.0,
            evapotranspiration_m: 0.0,
            evapotranspiration_storage_return_m: 0.0,
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
    pub frost_liquid_delta_m: f64,
    pub runon_input_m: f64,
    pub interception_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub evapotranspiration_storage_return_m: f64,
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
            frost_liquid_delta_m: 0.0,
            runon_input_m: 0.0,
            interception_m: 0.0,
            q_runoff_m: 0.0,
            evapotranspiration_m: 0.0,
            evapotranspiration_storage_return_m: 0.0,
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
    pub frost_liquid_delta_m: f64,
    pub runon_input_m: f64,
    pub interception_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub evapotranspiration_storage_return_m: f64,
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
            frost_liquid_delta_m: 0.0,
            runon_input_m: 0.0,
            interception_m: 0.0,
            q_runoff_m: 0.0,
            evapotranspiration_m: 0.0,
            evapotranspiration_storage_return_m: 0.0,
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
            frost_liquid_delta_m: state.frost_liquid_delta_m,
            runon_input_m: state.runon_input_m,
            interception_m: state.interception_m,
            q_runoff_m: state.q_runoff_m,
            evapotranspiration_m: state.evapotranspiration_m,
            evapotranspiration_storage_return_m: state.evapotranspiration_storage_return_m,
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
    pub frost_liquid_delta_m: f64,
    pub runon_input_m: f64,
    pub interception_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub evapotranspiration_storage_return_m: f64,
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
pub struct DirectDeepSeepageSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub deep_seepage_shadow_projection: DirectDeepSeepageShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSubsurfaceLossSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub subsurface_loss_shadow_projection: DirectSubsurfaceLossShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub evapotranspiration_shadow_projection: DirectEvapotranspirationShadowProjection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowCouplingSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub snow_coupling_shadow_projection: DirectSnowCouplingShadowProjection,
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
