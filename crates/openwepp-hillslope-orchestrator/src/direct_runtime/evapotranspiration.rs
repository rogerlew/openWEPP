use crate::constants::{
    WB11_ZERO_THRESHOLD, WB15_CANCOV_MAX, WB17_CANOPY_BARE_SOIL_OFFSET,
    WB17_CANOPY_EAJ_COEFFICIENT, WB17_PLTOL_MAX, WB17_PLTOL_MIN, WB17_SOIL_EVAPORATION_DEPTH_M,
    WB17_STAGE_ONE_DEFICIT_SCALE, WB17_STAGE_TWO_DEFICIT_SCALE, WB17_STAGE_TWO_DENOMINATOR,
    WB17_SWU_UB, WB17_SWU_UOB, WB17_TRANSPIRATION_LAI_FULL_COVER,
};

use super::{
    DIRECT_AUDIT, DIRECT_R4N_ROOT_PHASE_SPAN_COUNT, DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT,
    DirectDayFrame, DirectEvapotranspirationTraceEvent, DirectRuntimeError,
    DirectSubsurfaceLayerState, validate_finite, validate_nonnegative_direct_m,
};

#[derive(Debug, Clone)]
struct R7hEtTraceConfig {
    path: std::path::PathBuf,
    exact_day_index: Option<usize>,
    exact_lane_index: Option<usize>,
}

static R7H_ET_TRACE_CONFIG: std::sync::OnceLock<Option<R7hEtTraceConfig>> =
    std::sync::OnceLock::new();

fn r7h_et_trace_config() -> Option<&'static R7hEtTraceConfig> {
    R7H_ET_TRACE_CONFIG
        .get_or_init(|| {
            let path = std::env::var_os("OPENWEPP_R7H_ET_TRACE_PATH")?;
            if path.is_empty() {
                return None;
            }
            Some(R7hEtTraceConfig {
                path: std::path::PathBuf::from(path),
                exact_day_index: r7h_et_trace_env_usize("OPENWEPP_R7H_ET_TRACE_DAY_INDEX"),
                exact_lane_index: r7h_et_trace_env_usize("OPENWEPP_R7H_ET_TRACE_LANE_INDEX"),
            })
        })
        .as_ref()
}

fn r7h_et_trace_env_usize(name: &str) -> Option<usize> {
    let value = std::env::var(name).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<usize>().ok()
}

fn r7h_et_trace_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.17}")
    } else {
        "null".to_string()
    }
}

fn r7h_et_trace_f64_array(values: impl IntoIterator<Item = f64>) -> String {
    let mut output = String::from("[");
    let mut first = true;
    for value in values {
        if !first {
            output.push(',');
        }
        first = false;
        output.push_str(&r7h_et_trace_number(value));
    }
    output.push(']');
    output
}

#[allow(clippy::too_many_lines)]
fn maybe_write_r7h_et_trace(event: &DirectEvapotranspirationTraceEvent) {
    let Some(config) = r7h_et_trace_config() else {
        return;
    };
    if !r7h_et_trace_allows(config, event) {
        return;
    }

    let mut line = String::new();
    line.push('{');
    line.push_str("\"schema\":\"openwepp-r7h-et-trace-v1\"");
    line.push_str(",\"day_index\":");
    line.push_str(&event.day_index.to_string());
    line.push_str(",\"lane_index\":");
    line.push_str(&event.lane_index.to_string());
    line.push_str(",\"et_demand_m\":");
    line.push_str(&r7h_et_trace_number(event.et_demand_m));
    line.push_str(",\"root_depth_m\":");
    line.push_str(&r7h_et_trace_number(event.root_depth_m));
    line.push_str(",\"leaf_area_index\":");
    line.push_str(&r7h_et_trace_number(event.leaf_area_index));
    line.push_str(",\"canopy_cover_fraction\":");
    line.push_str(&r7h_et_trace_number(event.canopy_cover_fraction));
    line.push_str(",\"residue_interception_m\":");
    line.push_str(&r7h_et_trace_number(event.residue_interception_m));
    line.push_str(",\"plant_tolerance\":");
    line.push_str(&r7h_et_trace_number(event.plant_tolerance));
    line.push_str(",\"growth_input_vdmt_before_kg_m2\":");
    line.push_str(&r7h_et_trace_number(event.growth_input_vdmt_before_kg_m2));
    line.push_str(",\"growth_input_tlive_before_kg_m2\":");
    line.push_str(&r7h_et_trace_number(event.growth_input_tlive_before_kg_m2));
    line.push_str(",\"growth_input_hia_before\":");
    line.push_str(&r7h_et_trace_number(event.growth_input_hia_before));
    line.push_str(",\"growth_output_vdmt_after_kg_m2\":");
    line.push_str(&r7h_et_trace_number(event.growth_output_vdmt_after_kg_m2));
    line.push_str(",\"growth_output_tlive_after_kg_m2\":");
    line.push_str(&r7h_et_trace_number(event.growth_output_tlive_after_kg_m2));
    line.push_str(",\"growth_output_hia_after\":");
    line.push_str(&r7h_et_trace_number(event.growth_output_hia_after));
    if let Some(pmet_soil_evaporation_m) = event.pmet_soil_evaporation_m {
        line.push_str(",\"pmet_soil_evaporation_m\":");
        line.push_str(&r7h_et_trace_number(pmet_soil_evaporation_m));
        line.push_str(",\"pmet_plant_transpiration_m\":");
        line.push_str(&r7h_et_trace_number(
            event.pmet_plant_transpiration_m.unwrap_or(0.0),
        ));
        line.push_str(",\"pmet_soil_evaporation_storage_return_m\":");
        line.push_str(&r7h_et_trace_number(
            event.pmet_soil_evaporation_storage_return_m.unwrap_or(0.0),
        ));
    } else {
        line.push_str(",\"pmet_soil_evaporation_m\":null");
        line.push_str(",\"pmet_plant_transpiration_m\":null");
        line.push_str(",\"pmet_soil_evaporation_storage_return_m\":null");
    }
    line.push_str(",\"surface_soil_water_before_m\":");
    line.push_str(&r7h_et_trace_number(event.surface_soil_water_before_m));
    line.push_str(",\"surface_soil_water_after_m\":");
    line.push_str(&r7h_et_trace_number(event.surface_soil_water_after_m));
    line.push_str(",\"root_soil_water_before_m\":");
    line.push_str(&r7h_et_trace_number(event.root_soil_water_before_m));
    line.push_str(",\"root_soil_water_after_m\":");
    line.push_str(&r7h_et_trace_number(event.root_soil_water_after_m));
    line.push_str(",\"soil_evaporation_m\":");
    line.push_str(&r7h_et_trace_number(event.soil_evaporation_m));
    line.push_str(",\"residue_evaporation_m\":");
    line.push_str(&r7h_et_trace_number(event.residue_evaporation_m));
    line.push_str(",\"plant_transpiration_m\":");
    line.push_str(&r7h_et_trace_number(event.plant_transpiration_m));
    line.push_str(",\"water_stress\":");
    line.push_str(&r7h_et_trace_number(event.water_stress));
    line.push_str(",\"uptake_potential_m\":");
    line.push_str(&r7h_et_trace_number(event.uptake_potential_m));
    line.push_str(",\"uptake_actual_m\":");
    line.push_str(&r7h_et_trace_number(event.uptake_actual_m));
    line.push_str(",\"surface_layer_theta_m\":");
    line.push_str(&r7h_et_trace_f64_array(
        event.surface_layer_theta_m.iter().copied(),
    ));
    line.push_str(",\"root_layer_theta_m\":");
    line.push_str(&r7h_et_trace_f64_array(
        event.root_layer_theta_m.iter().copied(),
    ));
    line.push_str(",\"root_layer_upper_limit_m\":");
    line.push_str(&r7h_et_trace_f64_array(
        event.root_layer_upper_limit_m.iter().copied(),
    ));
    line.push_str(",\"root_layer_depth_m\":");
    line.push_str(&r7h_et_trace_f64_array(
        event.root_layer_depth_m.iter().copied(),
    ));
    line.push_str(",\"root_layer_uptake_potential_m\":");
    line.push_str(&r7h_et_trace_f64_array(
        event.root_layer_uptake_potential_m.iter().copied(),
    ));
    line.push_str(",\"root_layer_uptake_actual_m\":");
    line.push_str(&r7h_et_trace_f64_array(
        event.root_layer_uptake_actual_m.iter().copied(),
    ));
    line.push('}');
    line.push('\n');

    r7h_append_trace_line(&config.path, &line);
}

fn r7h_et_trace_allows(
    config: &R7hEtTraceConfig,
    event: &DirectEvapotranspirationTraceEvent,
) -> bool {
    if let Some(exact_day_index) = config.exact_day_index
        && event.day_index != exact_day_index
    {
        return false;
    }
    if let Some(exact_lane_index) = config.exact_lane_index
        && event.lane_index != exact_lane_index
    {
        return false;
    }
    true
}

fn r7h_append_trace_line(path: &std::path::Path, line: &str) {
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
    pub fn run_r4n_surface_et_span(
        &mut self,
    ) -> Result<DirectEvapotranspirationSurfaceSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT;

        DIRECT_AUDIT.record_direct_phase_entry();
        let surface = self.compute_r4n_surface_et()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        self.water.soil_water_m = surface.soil_water_after_soil_evap_m;
        self.evapotranspiration_surface = surface.clone();
        DIRECT_AUDIT.record_direct_state_mutation();

        self.evapotranspiration_surface_downstream_operands =
            DirectEvapotranspirationSurfaceDownstreamOperands::from(surface.clone());
        DIRECT_AUDIT.record_downstream_operand_production();

        let evapotranspiration_surface_shadow_projection =
            DirectEvapotranspirationSurfaceShadowProjection {
                lane_index: self.lane_index,
                day_index: self.day_index,
                soil_water_before_m: self
                    .evapotranspiration_surface_downstream_operands
                    .soil_water_before_m,
                soil_water_after_soil_evap_m: self
                    .evapotranspiration_surface_downstream_operands
                    .soil_water_after_soil_evap_m,
                evapotranspiration_seed_m: self
                    .evapotranspiration_surface_downstream_operands
                    .evapotranspiration_seed_m,
                transpiration_demand_m: self
                    .evapotranspiration_surface_downstream_operands
                    .transpiration_demand_m,
                soil_evaporation_m: self
                    .evapotranspiration_surface_downstream_operands
                    .soil_evaporation_m,
                residue_evaporation_m: self
                    .evapotranspiration_surface_downstream_operands
                    .residue_evaporation_m,
                soil_evaporation_storage_return_m: self
                    .evapotranspiration_surface_downstream_operands
                    .soil_evaporation_storage_return_m,
                residue_interception_after_m: self
                    .evapotranspiration_surface_downstream_operands
                    .residue_interception_after_m,
                layer_soil_evaporation_withdrawal_m: self
                    .evapotranspiration_surface_downstream_operands
                    .layer_soil_evaporation_withdrawal_m
                    .clone(),
                layer_state_after_soil_evap: self
                    .evapotranspiration_surface_downstream_operands
                    .layer_state_after_soil_evap
                    .clone(),
            };
        self.evapotranspiration_surface_shadow_projection =
            Some(evapotranspiration_surface_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectEvapotranspirationSurfaceSpanReport {
            phase_count,
            phase_entry_count: phase_count as u64,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            evapotranspiration_surface_shadow_projection,
        })
    }

    pub fn run_r4n_root_uptake_span(
        &mut self,
    ) -> Result<DirectEvapotranspirationComputeSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4N_ROOT_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let evapotranspiration_compute = self.compute_r4n_root_uptake()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.water.soil_water_m = evapotranspiration_compute.soil_water_after_m;
        self.water.evapotranspiration_m = evapotranspiration_compute.evapotranspiration_m;
        self.evapotranspiration_compute = evapotranspiration_compute.clone();
        self.storage_reconciliation_inputs.evapotranspiration_m =
            evapotranspiration_compute.evapotranspiration_m;
        self.storage_reconciliation_inputs
            .evapotranspiration_storage_return_m =
            evapotranspiration_compute.soil_evaporation_storage_return_m;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.evapotranspiration_compute_downstream_operands =
            DirectEvapotranspirationComputeDownstreamOperands::from(
                evapotranspiration_compute.clone(),
            );
        DIRECT_AUDIT.record_downstream_operand_production();

        let evapotranspiration_compute_shadow_projection =
            DirectEvapotranspirationComputeShadowProjection {
                lane_index: self.lane_index,
                day_index: self.day_index,
                soil_water_before_root_uptake_m: self
                    .evapotranspiration_compute_downstream_operands
                    .soil_water_before_root_uptake_m,
                soil_water_after_m: self
                    .evapotranspiration_compute_downstream_operands
                    .soil_water_after_m,
                evapotranspiration_m: self
                    .evapotranspiration_compute_downstream_operands
                    .evapotranspiration_m,
                soil_evaporation_m: self
                    .evapotranspiration_compute_downstream_operands
                    .soil_evaporation_m,
                residue_evaporation_m: self
                    .evapotranspiration_compute_downstream_operands
                    .residue_evaporation_m,
                soil_evaporation_storage_return_m: self
                    .evapotranspiration_compute_downstream_operands
                    .soil_evaporation_storage_return_m,
                plant_transpiration_m: self
                    .evapotranspiration_compute_downstream_operands
                    .plant_transpiration_m,
                transpiration_demand_m: self
                    .evapotranspiration_compute_downstream_operands
                    .transpiration_demand_m,
                water_stress: self
                    .evapotranspiration_compute_downstream_operands
                    .water_stress,
                uptake_potential_m: self
                    .evapotranspiration_compute_downstream_operands
                    .uptake_potential_m,
                uptake_actual_m: self
                    .evapotranspiration_compute_downstream_operands
                    .uptake_actual_m,
                effective_plant_tolerance: self
                    .evapotranspiration_compute_downstream_operands
                    .effective_plant_tolerance,
                layer_uptake_potential_m: self
                    .evapotranspiration_compute_downstream_operands
                    .layer_uptake_potential_m
                    .clone(),
                layer_uptake_actual_m: self
                    .evapotranspiration_compute_downstream_operands
                    .layer_uptake_actual_m
                    .clone(),
                layer_state_after_root_uptake: self
                    .evapotranspiration_compute_downstream_operands
                    .layer_state_after_root_uptake
                    .clone(),
            };
        self.evapotranspiration_compute_shadow_projection =
            Some(evapotranspiration_compute_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();
        // Event construction clones six layer Vecs; build it only when the
        // trace sink is configured.
        if r7h_et_trace_config().is_some() {
            maybe_write_r7h_et_trace(&DirectEvapotranspirationTraceEvent::from_day_frame(self));
        }

        Ok(DirectEvapotranspirationComputeSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            evapotranspiration_compute_shadow_projection,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn compute_r4n_surface_et(
        &self,
    ) -> Result<DirectEvapotranspirationSurfaceState, DirectRuntimeError> {
        let percolation = self.percolation_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4M percolation producer",
            },
        )?;
        let inputs = &self.evapotranspiration_compute_inputs;
        self.validate_required_growth_context_for_r4n(inputs)?;
        validate_surface_inputs(inputs)?;

        let mut layers = percolation.layer_state_after.clone();
        validate_et_layers("evapotranspiration.layers", &layers)?;
        let soil_water_before_m = percolation.soil_water_after_m;
        let surface_demand = compute_surface_et_demand_components(&mut layers, inputs)?;
        let SurfaceEtDemandComponents {
            soil_evaporation_with_residue_m,
            transpiration_demand_m,
            pmet_component_mode,
            soil_evaporation_storage_return_m,
            stage_state_after,
        } = surface_demand;

        validate_nonnegative_direct_m(
            "evapotranspiration.soil_evaporation_with_residue_m",
            soil_evaporation_with_residue_m,
        )?;
        validate_nonnegative_direct_m(
            "evapotranspiration.transpiration_demand_m",
            transpiration_demand_m,
        )?;

        let mut residue_evaporation_m = inputs.residue_interception_m;
        let soil_evaporation_extraction_demand_m =
            if soil_evaporation_with_residue_m < inputs.residue_interception_m {
                residue_evaporation_m = if pmet_component_mode {
                    soil_evaporation_with_residue_m
                } else {
                    soil_evaporation_with_residue_m.max(0.0)
                };
                if let Some(top_layer) = layers.first_mut() {
                    top_layer.theta_m += inputs.residue_interception_m - residue_evaporation_m;
                    validate_finite("evapotranspiration.top_layer_storage_m", top_layer.theta_m)?;
                }
                0.0
            } else {
                soil_evaporation_with_residue_m - inputs.residue_interception_m
            };

        let (soil_evaporation_m, layer_soil_evaporation_withdrawal_m) =
            withdraw_soil_evaporation(&mut layers, soil_evaporation_extraction_demand_m)?;
        apply_post_et_upper_limit_redistribution(
            &mut layers,
            inputs.outside_water_depth_m > 1.0e-6,
        )?;
        validate_et_layers("evapotranspiration.layers_after_soil_evap", &layers)?;
        let soil_water_after_soil_evap_m = aggregate_soil_water(&layers)?;
        let evapotranspiration_seed_m = residue_evaporation_m + soil_evaporation_m;
        validate_nonnegative_direct_m(
            "evapotranspiration.evapotranspiration_seed_m",
            evapotranspiration_seed_m,
        )?;

        Ok(DirectEvapotranspirationSurfaceState {
            soil_water_before_m,
            soil_water_after_soil_evap_m,
            evapotranspiration_seed_m,
            transpiration_demand_m,
            soil_evaporation_m,
            residue_evaporation_m,
            soil_evaporation_storage_return_m,
            residue_interception_after_m: 0.0,
            stage_state_after,
            layer_soil_evaporation_withdrawal_m,
            layer_state_after_soil_evap: layers,
        })
    }

    fn compute_r4n_root_uptake(
        &self,
    ) -> Result<DirectEvapotranspirationComputeState, DirectRuntimeError> {
        let surface = self
            .evapotranspiration_surface_shadow_projection
            .as_ref()
            .ok_or(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4N surface evapotranspiration producer",
            })?;
        let subsurface = self.subsurface_compute_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4O subsurface compute producer",
            },
        )?;
        let inputs = &self.evapotranspiration_compute_inputs;
        self.validate_required_growth_context_for_r4n(inputs)?;
        validate_root_inputs(inputs)?;

        let mut layers = subsurface.layer_state_after.clone();
        validate_et_layers("root_uptake.layers", &layers)?;
        if layers.len() != surface.layer_state_after_soil_evap.len() {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "root_uptake.layer_count",
            });
        }
        let soil_water_before_root_uptake_m = subsurface.soil_water_after_m;
        let computed_soil_water_before_m = aggregate_soil_water(&layers)?;
        let effective_plant_tolerance = effective_swu_plant_tolerance(inputs.plant_tolerance);
        validate_between(
            "root_uptake.effective_plant_tolerance",
            effective_plant_tolerance,
            WB17_PLTOL_MIN,
            WB17_PLTOL_MAX,
        )?;

        let mut layer_uptake_potential_m = vec![0.0_f64; layers.len()];
        let mut layer_uptake_actual_m = vec![0.0_f64; layers.len()];
        let mut plant_transpiration_m = 0.0;
        let profile_depth_m = layers.iter().map(|layer| layer.depth_m).sum::<f64>();
        validate_nonnegative_direct_m("root_uptake.profile_depth_m", profile_depth_m)?;
        let effective_root_depth_m = inputs.root_depth_m.min(profile_depth_m);
        if surface.transpiration_demand_m > WB11_ZERO_THRESHOLD
            && effective_root_depth_m > WB11_ZERO_THRESHOLD
        {
            run_swu_root_uptake(
                &mut layers,
                surface.transpiration_demand_m,
                effective_root_depth_m,
                effective_plant_tolerance,
                &mut layer_uptake_potential_m,
                &mut layer_uptake_actual_m,
                &mut plant_transpiration_m,
            )?;
        }

        let uptake_potential_m = layer_uptake_potential_m.iter().sum::<f64>();
        validate_nonnegative_direct_m("root_uptake.uptake_potential_m", uptake_potential_m)?;
        let uptake_actual_m = layer_uptake_actual_m.iter().sum::<f64>();
        validate_nonnegative_direct_m("root_uptake.uptake_actual_m", uptake_actual_m)?;

        let mut soil_water_after_m = soil_water_before_root_uptake_m;
        if uptake_actual_m > WB11_ZERO_THRESHOLD {
            soil_water_after_m = aggregate_soil_water(&layers)?;
            let storage_uptake_m = computed_soil_water_before_m - soil_water_after_m;
            let storage_correction_m = storage_uptake_m - uptake_actual_m;
            if storage_correction_m.abs() > f64::EPSILON
                && let Some(index) = layer_uptake_actual_m.iter().rposition(|value| *value > 0.0)
            {
                layers[index].theta_m += storage_correction_m;
                validate_nonnegative_direct_m(
                    "root_uptake.corrected_layer_theta_m",
                    layers[index].theta_m,
                )?;
                soil_water_after_m = aggregate_soil_water(&layers)?;
            }
        }
        validate_nonnegative_direct_m("root_uptake.soil_water_after_m", soil_water_after_m)?;

        let evapotranspiration_m = surface.evapotranspiration_seed_m + uptake_actual_m;
        validate_nonnegative_direct_m("root_uptake.evapotranspiration_m", evapotranspiration_m)?;
        validate_surface_shadow_storage_return(surface)?;
        let water_stress = if surface.transpiration_demand_m <= WB11_ZERO_THRESHOLD
            || effective_root_depth_m <= WB11_ZERO_THRESHOLD
        {
            1.0
        } else {
            (uptake_actual_m / surface.transpiration_demand_m).min(1.0)
        };
        validate_between("root_uptake.water_stress", water_stress, 0.0, 1.0)?;

        Ok(DirectEvapotranspirationComputeState {
            soil_water_before_root_uptake_m,
            soil_water_after_m,
            evapotranspiration_m,
            soil_evaporation_m: surface.soil_evaporation_m,
            residue_evaporation_m: surface.residue_evaporation_m,
            soil_evaporation_storage_return_m: surface.soil_evaporation_storage_return_m,
            plant_transpiration_m,
            transpiration_demand_m: surface.transpiration_demand_m,
            water_stress,
            uptake_potential_m,
            uptake_actual_m,
            effective_plant_tolerance,
            layer_uptake_potential_m,
            layer_uptake_actual_m,
            layer_state_after_root_uptake: layers,
        })
    }

    fn validate_required_growth_context_for_r4n(
        &self,
        inputs: &DirectEvapotranspirationComputeInputs,
    ) -> Result<(), DirectRuntimeError> {
        if inputs.growth_context_required
            && self.annual_growth_shadow_projection.is_none()
            && self.perennial_growth_shadow_projection.is_none()
        {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R5D growth transition",
            });
        }
        Ok(())
    }
}

fn validate_surface_shadow_storage_return(
    surface: &DirectEvapotranspirationSurfaceShadowProjection,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "root_uptake.soil_evaporation_storage_return_m",
        surface.soil_evaporation_storage_return_m,
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationStageState {
    pub s1_m: f64,
    pub s2_m: f64,
    pub threshold_m: f64,
    pub counter: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationPmetInputs {
    pub soil_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub soil_evaporation_storage_return_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationPmetComputeInputs {
    pub runtime_day_of_year: u16,
    pub radiation_ly: f64,
    pub wind_m_s: f64,
    pub dew_point_c: f64,
    pub temperature_max_c: f64,
    pub temperature_min_c: f64,
    pub latitude_degrees: f64,
    pub elevation_m: f64,
    pub kcb: f64,
    pub rawp: f64,
    pub canopy_height_m: f64,
    pub radpot_ly: Option<f64>,
    pub solthk_m: Vec<Option<f64>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeInputs {
    pub et_demand_m: f64,
    pub leaf_area_index: f64,
    pub canopy_height_m: f64,
    pub canopy_cover_fraction: f64,
    pub residue_interception_m: f64,
    pub same_pass_infiltration_m: f64,
    pub outside_water_depth_m: f64,
    pub root_depth_m: f64,
    pub plant_tolerance: f64,
    pub growth_context_required: bool,
    pub stage_state: Option<DirectEvapotranspirationStageState>,
    pub pmet: Option<DirectEvapotranspirationPmetInputs>,
    pub pmet_compute: Option<DirectEvapotranspirationPmetComputeInputs>,
}

impl DirectEvapotranspirationComputeInputs {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            et_demand_m: 0.0,
            leaf_area_index: 0.0,
            canopy_height_m: 0.0,
            canopy_cover_fraction: 0.0,
            residue_interception_m: 0.0,
            same_pass_infiltration_m: 0.0,
            outside_water_depth_m: 0.0,
            root_depth_m: 0.0,
            plant_tolerance: 0.0,
            growth_context_required: false,
            stage_state: None,
            pmet: None,
            pmet_compute: None,
        }
    }
}

impl DirectEvapotranspirationPmetComputeInputs {
    #[allow(clippy::manual_midpoint, clippy::similar_names, clippy::too_many_lines)]
    pub(super) fn compute(
        &self,
        layers: &[DirectSubsurfaceLayerState],
        et: &DirectEvapotranspirationComputeInputs,
    ) -> Result<DirectEvapotranspirationPmetInputs, DirectRuntimeError> {
        validate_et_layers("pmet.layers", layers)?;
        validate_pmet_runtime_day(self.runtime_day_of_year)?;
        validate_nonnegative_direct_m("pmet.radiation_ly", self.radiation_ly)?;
        validate_nonnegative_direct_m("pmet.wind_m_s", self.wind_m_s)?;
        validate_finite("pmet.dew_point_c", self.dew_point_c)?;
        validate_finite("pmet.temperature_max_c", self.temperature_max_c)?;
        validate_finite("pmet.temperature_min_c", self.temperature_min_c)?;
        validate_finite("pmet.latitude_degrees", self.latitude_degrees)?;
        validate_finite("pmet.elevation_m", self.elevation_m)?;
        validate_finite("pmet.kcb", self.kcb)?;
        validate_finite("pmet.rawp", self.rawp)?;
        validate_nonnegative_direct_m("pmet.canopy_height_m", self.canopy_height_m)?;
        if let Some(radpot_ly) = self.radpot_ly {
            validate_positive("pmet.radpot_ly", radpot_ly)?;
        }
        validate_nonnegative_direct_m("pmet.leaf_area_index", et.leaf_area_index)?;
        validate_nonnegative_direct_m("pmet.root_depth_m", et.root_depth_m)?;

        let tave = f64::midpoint(self.temperature_max_c, self.temperature_min_c);
        let ed = pmet_saturation_vapor_pressure_kpa(self.dew_point_c);
        let emaxt = pmet_saturation_vapor_pressure_kpa(self.temperature_max_c);
        let emint = pmet_saturation_vapor_pressure_kpa(self.temperature_min_c);
        let ee = f64::midpoint(emaxt, emint);
        validate_positive("pmet.emaxt", emaxt)?;
        let radpot = self.radpot_or_legacy();
        validate_positive("pmet.radpot", radpot)?;

        let ra = self.radiation_ly / 23.9;
        let rso = radpot / 23.9;
        let rbo = (0.34 - 0.14 * ed.sqrt())
            * 4.9e-9
            * (((self.temperature_max_c + 273.2).powi(4)
                + (self.temperature_min_c + 273.2).powi(4))
                / 2.0)
            * (1.35 * (ra / rso) - 0.35);
        let rn_mj_m2 = ra.mul_add(0.77, -rbo);
        let fwv_m_s = self.wind_m_s * 4.87 / (67.8_f64.mul_add(10.0, -5.42)).ln();
        let dlt =
            4098.0 / ((tave + 237.3) * (tave + 237.3)) * pmet_saturation_vapor_pressure_kpa(tave);
        let pressure_base = 1.0 - 0.0065 * self.elevation_m / 293.0;
        validate_positive("pmet.pressure_base", pressure_base)?;
        let pb = 101.3 * pressure_base.powf(5.26);
        let gma = 0.000_665 * pb;
        let denominator = dlt + gma * self.wind_m_s.mul_add(0.34, 1.0);
        validate_positive("pmet.etorc_denominator", denominator)?;
        let etorc_mm = (0.408 * dlt * rn_mj_m2
            + gma * (900.0 / (tave + 273.0)) * (ee - ed) * fwv_m_s)
            / denominator;
        let rhd_pct = ed / emaxt * 100.0;
        let height_factor = (self.canopy_height_m / 3.0).powf(0.3);
        let kcbadj = pmet_adjusted_crop_coefficient(et, self.kcb, fwv_m_s, rhd_pct, height_factor);
        let kcbcon = kcbadj * (1.0 - (-0.45 * et.leaf_area_index).exp());
        let etke = pmet_soil_evaporation_coefficient(kcbadj, et.leaf_area_index);

        let profile_depth_m = pmet_profile_depth_m(layers)?;
        let epdp_m = 0.1_f64.min(profile_depth_m);
        let (tew_mm, rew_mm, wfevp_base_mm) = self.evaporation_storage_terms(layers, epdp_m)?;
        let wfevp_mm = wfevp_base_mm + et.residue_interception_m * 1_000.0;
        let etkr = pmet_evaporation_reduction_coefficient(tew_mm, rew_mm, wfevp_mm);
        let tpdp_m = et.root_depth_m.min(profile_depth_m);
        let (taw_mm, wftrp_mm) = self.transpiration_storage_terms(layers, tpdp_m, wfevp_mm)?;
        let etcsc = kcbadj * etorc_mm;
        let rawpaj = self.rawp + 0.04 * (5.0 - etcsc);
        let raw_mm = rawpaj * taw_mm;
        let etks = pmet_transpiration_stress_coefficient(taw_mm, wftrp_mm, raw_mm);
        let potes_m = etorc_mm * etke * 0.001;
        let es_raw_m =
            pmet_raw_soil_evaporation_m(et, potes_m, etke, etkr, fwv_m_s, rhd_pct, height_factor);
        let soil_evaporation_storage_return_m = if es_raw_m < 0.0 { -es_raw_m } else { 0.0 };
        let soil_evaporation_m = es_raw_m.max(0.0);
        let ep_raw_m = etorc_mm * etks * kcbcon * 0.001;
        let plant_transpiration_m = ep_raw_m.max(0.0);
        for (name, value) in [
            ("pmet.etorc_mm", etorc_mm),
            ("pmet.rn_mj_m2", rn_mj_m2),
            ("pmet.fwv_m_s", fwv_m_s),
            ("pmet.rhd_pct", rhd_pct),
            ("pmet.kcbadj", kcbadj),
            ("pmet.kcbcon", kcbcon),
            ("pmet.etke", etke),
            ("pmet.etkr", etkr),
            ("pmet.etks", etks),
            ("pmet.tew_mm", tew_mm),
            ("pmet.rew_mm", rew_mm),
            ("pmet.wfevp_mm", wfevp_mm),
            ("pmet.taw_mm", taw_mm),
            ("pmet.raw_mm", raw_mm),
            ("pmet.wftrp_mm", wftrp_mm),
            ("pmet.es_m", soil_evaporation_m),
            (
                "pmet.es_storage_return_m",
                soil_evaporation_storage_return_m,
            ),
            ("pmet.ep_m", plant_transpiration_m),
        ] {
            validate_finite(name, value)?;
        }
        Ok(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m,
            plant_transpiration_m,
            soil_evaporation_storage_return_m,
        })
    }

    fn radpot_or_legacy(&self) -> f64 {
        self.radpot_ly.unwrap_or_else(|| {
            pmet_legacy_sunmap_horizontal_radpot_ly(
                self.latitude_degrees,
                f64::from(self.runtime_day_of_year),
            )
        })
    }

    pub(super) fn evaporation_storage_terms(
        &self,
        layers: &[DirectSubsurfaceLayerState],
        epdp_m: f64,
    ) -> Result<(f64, f64, f64), DirectRuntimeError> {
        let mut tew_mm = 0.0_f64;
        let mut rew_mm = 0.0_f64;
        let mut wfevp_mm = 0.0_f64;
        let mut cumulative_depth_m = 0.0_f64;
        for (offset, layer) in layers.iter().enumerate() {
            let layer_index = offset + 1;
            let solthk = self.solthk(layer_index, cumulative_depth_m, layer.depth_m)?;
            let layer_fraction = if solthk <= epdp_m {
                1.0
            } else if cumulative_depth_m < epdp_m {
                (epdp_m - cumulative_depth_m) / (solthk - cumulative_depth_m)
            } else {
                0.0
            };
            if layer.residual_theta > layer.field_capacity_theta {
                return Err(DirectRuntimeError::DirectDomainViolation {
                    field: "pmet.layer_residual_theta",
                });
            }
            if layer_fraction > 0.0 {
                tew_mm += (layer.field_capacity_theta - 0.5 * layer.residual_theta)
                    * layer.depth_m
                    * 1_000.0
                    * layer_fraction;
                rew_mm +=
                    (layer.field_capacity_theta - layer.residual_theta) * layer.depth_m * 1_000.0
                        / 3.0
                        * layer_fraction;
                wfevp_mm += layer.theta_m * 1_000.0 * layer_fraction;
            }
            cumulative_depth_m = solthk;
            if cumulative_depth_m >= epdp_m {
                break;
            }
        }
        Ok((tew_mm, rew_mm, wfevp_mm))
    }

    pub(super) fn transpiration_storage_terms(
        &self,
        layers: &[DirectSubsurfaceLayerState],
        tpdp_m: f64,
        wfevp_mm: f64,
    ) -> Result<(f64, f64), DirectRuntimeError> {
        let mut taw_mm = 0.0_f64;
        let mut wftrp_mm = 0.0_f64;
        let mut cumulative_depth_m = 0.0_f64;
        for (offset, layer) in layers.iter().enumerate() {
            let layer_index = offset + 1;
            let solthk = self.solthk(layer_index, cumulative_depth_m, layer.depth_m)?;
            if tpdp_m <= 0.0 {
                break;
            }
            if solthk <= tpdp_m {
                taw_mm +=
                    (layer.field_capacity_theta - layer.residual_theta) * layer.depth_m * 1_000.0;
                wftrp_mm += layer.theta_m * 1_000.0;
            } else if cumulative_depth_m < tpdp_m {
                let layer_span_m = solthk - cumulative_depth_m;
                validate_positive("pmet.layer_span_m", layer_span_m)?;
                let fraction = (tpdp_m - cumulative_depth_m) / layer_span_m;
                taw_mm += (layer.field_capacity_theta - layer.residual_theta)
                    * layer.depth_m
                    * 1_000.0
                    * fraction;
                wftrp_mm = wfevp_mm + layer.theta_m * 1_000.0 * fraction;
                break;
            }
            cumulative_depth_m = solthk;
            if cumulative_depth_m >= tpdp_m {
                break;
            }
        }
        Ok((taw_mm, wftrp_mm))
    }

    fn solthk(
        &self,
        layer_index: usize,
        cumulative_depth_m: f64,
        depth_m: f64,
    ) -> Result<f64, DirectRuntimeError> {
        let solthk = self
            .solthk_m
            .get(layer_index - 1)
            .and_then(|value| *value)
            .unwrap_or(cumulative_depth_m + depth_m);
        if solthk <= cumulative_depth_m {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "pmet.solthk_m",
            });
        }
        Ok(solthk)
    }
}

pub(super) fn pmet_adjusted_crop_coefficient(
    et: &DirectEvapotranspirationComputeInputs,
    kcb: f64,
    fwv_m_s: f64,
    rhd_pct: f64,
    height_factor: f64,
) -> f64 {
    if et.leaf_area_index > 0.0 && et.root_depth_m > 0.0 {
        kcb + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor
    } else {
        0.0
    }
}

pub(super) fn pmet_soil_evaporation_coefficient(kcbadj: f64, leaf_area_index: f64) -> f64 {
    if kcbadj > 0.0 {
        kcbadj * (-0.45 * leaf_area_index).exp()
    } else {
        1.2
    }
}

pub(super) fn pmet_evaporation_reduction_coefficient(
    tew_mm: f64,
    rew_mm: f64,
    wfevp_mm: f64,
) -> f64 {
    if (tew_mm - wfevp_mm) <= rew_mm {
        1.0
    } else {
        let denominator = tew_mm - rew_mm;
        if denominator <= 0.0 {
            1.0
        } else {
            (wfevp_mm / denominator).powi(2)
        }
    }
}

pub(super) fn pmet_transpiration_stress_coefficient(
    taw_mm: f64,
    wftrp_mm: f64,
    raw_mm: f64,
) -> f64 {
    let etksden = taw_mm - raw_mm;
    if etksden <= 0.0 || (taw_mm - wftrp_mm) <= raw_mm {
        1.0
    } else {
        wftrp_mm / etksden
    }
}

pub(super) fn pmet_raw_soil_evaporation_m(
    et: &DirectEvapotranspirationComputeInputs,
    potes_m: f64,
    soil_evaporation_coefficient: f64,
    evaporation_reduction_coefficient: f64,
    fwv_m_s: f64,
    rhd_pct: f64,
    height_factor: f64,
) -> f64 {
    if potes_m > et.residue_interception_m {
        let bpotes_m = potes_m - et.residue_interception_m;
        let eaj = (-0.5 * (et.canopy_cover_fraction + 0.1)).exp();
        let kcmax = 1.2 + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor;
        let kecon =
            (soil_evaporation_coefficient * evaporation_reduction_coefficient).min(eaj * kcmax);
        kecon * bpotes_m / soil_evaporation_coefficient + et.residue_interception_m
    } else {
        potes_m
    }
}

fn pmet_profile_depth_m(layers: &[DirectSubsurfaceLayerState]) -> Result<f64, DirectRuntimeError> {
    let profile_depth_m = layers.iter().map(|layer| layer.depth_m).sum::<f64>();
    validate_positive("pmet.profile_depth_m", profile_depth_m)?;
    Ok(profile_depth_m)
}

fn validate_pmet_runtime_day(day: u16) -> Result<(), DirectRuntimeError> {
    if (1..=366).contains(&day) {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "pmet.runtime_day_of_year",
        })
    }
}

fn pmet_saturation_vapor_pressure_kpa(temperature_c: f64) -> f64 {
    0.6108 * (17.27 * temperature_c / (temperature_c + 237.3)).exp()
}

fn pmet_legacy_sunmap_horizontal_radpot_ly(latitude_degrees: f64, runtime_day: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let radlat = latitude_degrees * pi / 180.0;
    let declination = 0.00698 - 0.4067 * ((runtime_day + 10.0) * 0.0172).cos();
    let earth_sun_distance_factor = 1.0 - 0.0167 * ((runtime_day - 3.0) * 0.0172).cos();
    let radiation_factor = (60.0 * 1.94) / (earth_sun_distance_factor * earth_sun_distance_factor);
    let sunset_argument = -(radlat.tan() * declination.tan()).clamp(-1.0, 1.0);
    let sunset_angle = sunset_argument.acos();
    radiation_factor
        * ((declination.sin() * radlat.sin() * (sunset_angle - -sunset_angle) * 12.0 / pi)
            + (declination.cos()
                * radlat.cos()
                * (sunset_angle.sin() - (-sunset_angle).sin())
                * 12.0
                / pi))
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationSurfaceState {
    pub soil_water_before_m: f64,
    pub soil_water_after_soil_evap_m: f64,
    pub evapotranspiration_seed_m: f64,
    pub transpiration_demand_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub soil_evaporation_storage_return_m: f64,
    pub residue_interception_after_m: f64,
    pub stage_state_after: Option<DirectEvapotranspirationStageState>,
    pub layer_soil_evaporation_withdrawal_m: Vec<f64>,
    pub layer_state_after_soil_evap: Vec<DirectSubsurfaceLayerState>,
}

impl DirectEvapotranspirationSurfaceState {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_m: 0.0,
            soil_water_after_soil_evap_m: 0.0,
            evapotranspiration_seed_m: 0.0,
            transpiration_demand_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            soil_evaporation_storage_return_m: 0.0,
            residue_interception_after_m: 0.0,
            stage_state_after: None,
            layer_soil_evaporation_withdrawal_m: Vec::new(),
            layer_state_after_soil_evap: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationSurfaceDownstreamOperands {
    pub soil_water_before_m: f64,
    pub soil_water_after_soil_evap_m: f64,
    pub evapotranspiration_seed_m: f64,
    pub transpiration_demand_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub soil_evaporation_storage_return_m: f64,
    pub residue_interception_after_m: f64,
    pub layer_soil_evaporation_withdrawal_m: Vec<f64>,
    pub layer_state_after_soil_evap: Vec<DirectSubsurfaceLayerState>,
}

impl DirectEvapotranspirationSurfaceDownstreamOperands {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_m: 0.0,
            soil_water_after_soil_evap_m: 0.0,
            evapotranspiration_seed_m: 0.0,
            transpiration_demand_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            soil_evaporation_storage_return_m: 0.0,
            residue_interception_after_m: 0.0,
            layer_soil_evaporation_withdrawal_m: Vec::new(),
            layer_state_after_soil_evap: Vec::new(),
        }
    }
}

impl From<DirectEvapotranspirationSurfaceState>
    for DirectEvapotranspirationSurfaceDownstreamOperands
{
    fn from(state: DirectEvapotranspirationSurfaceState) -> Self {
        Self {
            soil_water_before_m: state.soil_water_before_m,
            soil_water_after_soil_evap_m: state.soil_water_after_soil_evap_m,
            evapotranspiration_seed_m: state.evapotranspiration_seed_m,
            transpiration_demand_m: state.transpiration_demand_m,
            soil_evaporation_m: state.soil_evaporation_m,
            residue_evaporation_m: state.residue_evaporation_m,
            soil_evaporation_storage_return_m: state.soil_evaporation_storage_return_m,
            residue_interception_after_m: state.residue_interception_after_m,
            layer_soil_evaporation_withdrawal_m: state.layer_soil_evaporation_withdrawal_m,
            layer_state_after_soil_evap: state.layer_state_after_soil_evap,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationSurfaceShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub soil_water_before_m: f64,
    pub soil_water_after_soil_evap_m: f64,
    pub evapotranspiration_seed_m: f64,
    pub transpiration_demand_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub soil_evaporation_storage_return_m: f64,
    pub residue_interception_after_m: f64,
    pub layer_soil_evaporation_withdrawal_m: Vec<f64>,
    pub layer_state_after_soil_evap: Vec<DirectSubsurfaceLayerState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeState {
    pub soil_water_before_root_uptake_m: f64,
    pub soil_water_after_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub soil_evaporation_storage_return_m: f64,
    pub plant_transpiration_m: f64,
    pub transpiration_demand_m: f64,
    pub water_stress: f64,
    pub uptake_potential_m: f64,
    pub uptake_actual_m: f64,
    pub effective_plant_tolerance: f64,
    pub layer_uptake_potential_m: Vec<f64>,
    pub layer_uptake_actual_m: Vec<f64>,
    pub layer_state_after_root_uptake: Vec<DirectSubsurfaceLayerState>,
}

impl DirectEvapotranspirationComputeState {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_root_uptake_m: 0.0,
            soil_water_after_m: 0.0,
            evapotranspiration_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            soil_evaporation_storage_return_m: 0.0,
            plant_transpiration_m: 0.0,
            transpiration_demand_m: 0.0,
            water_stress: 1.0,
            uptake_potential_m: 0.0,
            uptake_actual_m: 0.0,
            effective_plant_tolerance: 0.25,
            layer_uptake_potential_m: Vec::new(),
            layer_uptake_actual_m: Vec::new(),
            layer_state_after_root_uptake: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeDownstreamOperands {
    pub soil_water_before_root_uptake_m: f64,
    pub soil_water_after_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub soil_evaporation_storage_return_m: f64,
    pub plant_transpiration_m: f64,
    pub transpiration_demand_m: f64,
    pub water_stress: f64,
    pub uptake_potential_m: f64,
    pub uptake_actual_m: f64,
    pub effective_plant_tolerance: f64,
    pub layer_uptake_potential_m: Vec<f64>,
    pub layer_uptake_actual_m: Vec<f64>,
    pub layer_state_after_root_uptake: Vec<DirectSubsurfaceLayerState>,
}

impl DirectEvapotranspirationComputeDownstreamOperands {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_root_uptake_m: 0.0,
            soil_water_after_m: 0.0,
            evapotranspiration_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            soil_evaporation_storage_return_m: 0.0,
            plant_transpiration_m: 0.0,
            transpiration_demand_m: 0.0,
            water_stress: 1.0,
            uptake_potential_m: 0.0,
            uptake_actual_m: 0.0,
            effective_plant_tolerance: 0.25,
            layer_uptake_potential_m: Vec::new(),
            layer_uptake_actual_m: Vec::new(),
            layer_state_after_root_uptake: Vec::new(),
        }
    }
}

impl From<DirectEvapotranspirationComputeState>
    for DirectEvapotranspirationComputeDownstreamOperands
{
    fn from(state: DirectEvapotranspirationComputeState) -> Self {
        Self {
            soil_water_before_root_uptake_m: state.soil_water_before_root_uptake_m,
            soil_water_after_m: state.soil_water_after_m,
            evapotranspiration_m: state.evapotranspiration_m,
            soil_evaporation_m: state.soil_evaporation_m,
            residue_evaporation_m: state.residue_evaporation_m,
            soil_evaporation_storage_return_m: state.soil_evaporation_storage_return_m,
            plant_transpiration_m: state.plant_transpiration_m,
            transpiration_demand_m: state.transpiration_demand_m,
            water_stress: state.water_stress,
            uptake_potential_m: state.uptake_potential_m,
            uptake_actual_m: state.uptake_actual_m,
            effective_plant_tolerance: state.effective_plant_tolerance,
            layer_uptake_potential_m: state.layer_uptake_potential_m,
            layer_uptake_actual_m: state.layer_uptake_actual_m,
            layer_state_after_root_uptake: state.layer_state_after_root_uptake,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub soil_water_before_root_uptake_m: f64,
    pub soil_water_after_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub soil_evaporation_storage_return_m: f64,
    pub plant_transpiration_m: f64,
    pub transpiration_demand_m: f64,
    pub water_stress: f64,
    pub uptake_potential_m: f64,
    pub uptake_actual_m: f64,
    pub effective_plant_tolerance: f64,
    pub layer_uptake_potential_m: Vec<f64>,
    pub layer_uptake_actual_m: Vec<f64>,
    pub layer_state_after_root_uptake: Vec<DirectSubsurfaceLayerState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationSurfaceSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub evapotranspiration_surface_shadow_projection:
        DirectEvapotranspirationSurfaceShadowProjection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub evapotranspiration_compute_shadow_projection:
        DirectEvapotranspirationComputeShadowProjection,
}

fn validate_surface_inputs(
    inputs: &DirectEvapotranspirationComputeInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("evapotranspiration.et_demand_m", inputs.et_demand_m)?;
    validate_nonnegative_direct_m("evapotranspiration.leaf_area_index", inputs.leaf_area_index)?;
    validate_between(
        "evapotranspiration.canopy_cover_fraction",
        inputs.canopy_cover_fraction,
        0.0,
        WB15_CANCOV_MAX,
    )?;
    validate_nonnegative_direct_m(
        "evapotranspiration.residue_interception_m",
        inputs.residue_interception_m,
    )?;
    validate_nonnegative_direct_m(
        "evapotranspiration.same_pass_infiltration_m",
        inputs.same_pass_infiltration_m,
    )?;
    validate_nonnegative_direct_m(
        "evapotranspiration.outside_water_depth_m",
        inputs.outside_water_depth_m,
    )?;
    if let Some(stage) = inputs.stage_state {
        validate_stage_state(stage)?;
    }
    Ok(())
}

fn validate_root_inputs(
    inputs: &DirectEvapotranspirationComputeInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("root_uptake.root_depth_m", inputs.root_depth_m)?;
    validate_finite("root_uptake.plant_tolerance", inputs.plant_tolerance)?;
    Ok(())
}

fn validate_stage_state(
    stage: DirectEvapotranspirationStageState,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("evapotranspiration.stage_s1_m", stage.s1_m)?;
    validate_nonnegative_direct_m("evapotranspiration.stage_s2_m", stage.s2_m)?;
    validate_positive("evapotranspiration.stage_threshold_m", stage.threshold_m)?;
    validate_nonnegative_direct_m("evapotranspiration.stage_counter", stage.counter)?;
    Ok(())
}

fn validate_pmet_inputs(
    pmet: DirectEvapotranspirationPmetInputs,
) -> Result<(), DirectRuntimeError> {
    validate_finite(
        "evapotranspiration.pmet_soil_evaporation_m",
        pmet.soil_evaporation_m,
    )?;
    if pmet.soil_evaporation_m < -WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "evapotranspiration.pmet_soil_evaporation_m",
        });
    }
    validate_nonnegative_direct_m(
        "evapotranspiration.pmet_plant_transpiration_m",
        pmet.plant_transpiration_m,
    )?;
    validate_nonnegative_direct_m(
        "evapotranspiration.pmet_soil_evaporation_storage_return_m",
        pmet.soil_evaporation_storage_return_m,
    )?;
    Ok(())
}

struct SurfaceEtDemandComponents {
    soil_evaporation_with_residue_m: f64,
    transpiration_demand_m: f64,
    pmet_component_mode: bool,
    soil_evaporation_storage_return_m: f64,
    stage_state_after: Option<DirectEvapotranspirationStageState>,
}

fn compute_surface_et_demand_components(
    layers: &mut [DirectSubsurfaceLayerState],
    inputs: &DirectEvapotranspirationComputeInputs,
) -> Result<SurfaceEtDemandComponents, DirectRuntimeError> {
    let computed_pmet = if let Some(pmet_compute) = &inputs.pmet_compute {
        Some(pmet_compute.compute(layers, inputs)?)
    } else {
        inputs.pmet
    };
    if let Some(pmet) = computed_pmet {
        return compute_pmet_surface_et_demand(layers, pmet, inputs.stage_state);
    }
    compute_manual_surface_et_demand(inputs)
}

fn compute_pmet_surface_et_demand(
    layers: &mut [DirectSubsurfaceLayerState],
    pmet: DirectEvapotranspirationPmetInputs,
    stage_state_after: Option<DirectEvapotranspirationStageState>,
) -> Result<SurfaceEtDemandComponents, DirectRuntimeError> {
    validate_pmet_inputs(pmet)?;
    if let Some(top_layer) = layers.first_mut() {
        top_layer.theta_m += pmet.soil_evaporation_storage_return_m;
        validate_finite("evapotranspiration.top_layer_storage_m", top_layer.theta_m)?;
    }
    Ok(SurfaceEtDemandComponents {
        soil_evaporation_with_residue_m: normalize_within_zero_tolerance(pmet.soil_evaporation_m)?,
        transpiration_demand_m: pmet.plant_transpiration_m,
        pmet_component_mode: true,
        soil_evaporation_storage_return_m: pmet.soil_evaporation_storage_return_m,
        stage_state_after,
    })
}

fn compute_manual_surface_et_demand(
    inputs: &DirectEvapotranspirationComputeInputs,
) -> Result<SurfaceEtDemandComponents, DirectRuntimeError> {
    let soil_evaporation_partition_potential_m = inputs.et_demand_m
        * (-WB17_CANOPY_EAJ_COEFFICIENT
            * (inputs.canopy_cover_fraction + WB17_CANOPY_BARE_SOIL_OFFSET))
            .exp();
    validate_nonnegative_direct_m(
        "evapotranspiration.soil_evaporation_partition_potential_m",
        soil_evaporation_partition_potential_m,
    )?;
    if soil_evaporation_partition_potential_m > inputs.et_demand_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "evapotranspiration.soil_evaporation_partition_potential_m",
        });
    }

    let transpiration_partition_potential_m =
        if inputs.leaf_area_index > WB17_TRANSPIRATION_LAI_FULL_COVER {
            inputs.et_demand_m
        } else {
            inputs.leaf_area_index * inputs.et_demand_m / WB17_TRANSPIRATION_LAI_FULL_COVER
        };
    validate_nonnegative_direct_m(
        "evapotranspiration.transpiration_partition_potential_m",
        transpiration_partition_potential_m,
    )?;

    let residue_evaporation_m = inputs
        .residue_interception_m
        .min(soil_evaporation_partition_potential_m);
    let soil_evaporation_potential_m =
        soil_evaporation_partition_potential_m - residue_evaporation_m;
    let (soil_evaporation_demand_m, stage_state_after) =
        compute_manual_soil_evaporation_demand(inputs, soil_evaporation_potential_m)?;

    let mut soil_evaporation_with_residue_m =
        soil_evaporation_demand_m + inputs.residue_interception_m;
    let potential_et_before_layer_m =
        soil_evaporation_with_residue_m + transpiration_partition_potential_m;
    if inputs.et_demand_m < potential_et_before_layer_m {
        soil_evaporation_with_residue_m =
            (inputs.et_demand_m - transpiration_partition_potential_m).max(0.0);
    }
    Ok(SurfaceEtDemandComponents {
        soil_evaporation_with_residue_m,
        transpiration_demand_m: transpiration_partition_potential_m,
        pmet_component_mode: false,
        soil_evaporation_storage_return_m: 0.0,
        stage_state_after,
    })
}

fn compute_manual_soil_evaporation_demand(
    inputs: &DirectEvapotranspirationComputeInputs,
    soil_evaporation_potential_m: f64,
) -> Result<(f64, Option<DirectEvapotranspirationStageState>), DirectRuntimeError> {
    if let Some(stage_state) = inputs.stage_state {
        let (stage_soil_evaporation_m, next_stage) = compute_stage_soil_evaporation(
            stage_state,
            inputs.same_pass_infiltration_m,
            soil_evaporation_potential_m,
        )?;
        Ok((stage_soil_evaporation_m, Some(next_stage)))
    } else {
        Ok((soil_evaporation_potential_m, None))
    }
}

fn validate_et_layers(
    field: &'static str,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<(), DirectRuntimeError> {
    if layers.is_empty() {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    for layer in layers {
        validate_nonnegative_direct_m("evapotranspiration.layer_theta_m", layer.theta_m)?;
        validate_positive(
            "evapotranspiration.layer_upper_limit_m",
            layer.upper_limit_m,
        )?;
        validate_positive("evapotranspiration.layer_depth_m", layer.depth_m)?;
        validate_nonnegative_direct_m(
            "evapotranspiration.layer_residual_theta",
            layer.residual_theta,
        )?;
        validate_nonnegative_direct_m(
            "evapotranspiration.layer_frozen_depth_m",
            layer.frozen_depth_m,
        )?;
        validate_nonnegative_direct_m(
            "evapotranspiration.layer_frozen_water_m",
            layer.frozen_water_m,
        )?;
        if layer.frozen_depth_m > layer.depth_m + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "evapotranspiration.layer_frozen_depth_m",
            });
        }
    }
    Ok(())
}

pub(super) fn compute_stage_soil_evaporation(
    stage: DirectEvapotranspirationStageState,
    infiltration_m: f64,
    soil_evaporation_potential_m: f64,
) -> Result<(f64, DirectEvapotranspirationStageState), DirectRuntimeError> {
    let mut s1 = stage.s1_m;
    let mut s2 = stage.s2_m;
    let tu = stage.threshold_m;
    let mut tv = stage.counter;
    let mut es_stage_m = soil_evaporation_potential_m;

    if s1 < tu {
        s2 = 0.0;
        let sp = s1 - infiltration_m;
        s1 = if sp > 0.0 { sp } else { 0.0 };
        s1 += soil_evaporation_potential_m;
        let su = s1 - tu;
        if su > 0.0 {
            es_stage_m = soil_evaporation_potential_m - WB17_STAGE_ONE_DEFICIT_SCALE * su;
            s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
            tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
        }
    } else {
        let sb = infiltration_m - s2;
        if sb < 0.0 {
            tv += 1.0;
            es_stage_m = WB17_STAGE_TWO_DENOMINATOR * tv.sqrt() - s2;
            if infiltration_m > 0.0 {
                let mut esx_m = 0.8 * infiltration_m;
                if es_stage_m > esx_m {
                    esx_m = es_stage_m + infiltration_m;
                }
                if esx_m > soil_evaporation_potential_m {
                    esx_m = soil_evaporation_potential_m;
                }
                es_stage_m = esx_m;
            } else if es_stage_m > soil_evaporation_potential_m {
                es_stage_m = soil_evaporation_potential_m;
            }
            s2 += es_stage_m - infiltration_m;
            tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
        } else {
            s1 = tu - sb;
            tv = 0.0;
            s2 = 0.0;
            if s1 < 0.0 {
                s1 = 0.0;
            }
            s1 += soil_evaporation_potential_m;
            let su = s1 - tu;
            if su > 0.0 {
                es_stage_m = soil_evaporation_potential_m - WB17_STAGE_ONE_DEFICIT_SCALE * su;
                s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
                tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
            }
        }
    }

    validate_nonnegative_direct_m("evapotranspiration.stage_es_m", es_stage_m)?;
    if es_stage_m > soil_evaporation_potential_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "evapotranspiration.stage_es_m",
        });
    }
    let next_stage = DirectEvapotranspirationStageState {
        s1_m: s1,
        s2_m: s2,
        threshold_m: tu,
        counter: tv,
    };
    validate_stage_state(next_stage)?;
    Ok((es_stage_m, next_stage))
}

fn withdraw_soil_evaporation(
    layers: &mut [DirectSubsurfaceLayerState],
    demand_m: f64,
) -> Result<(f64, Vec<f64>), DirectRuntimeError> {
    validate_nonnegative_direct_m("evapotranspiration.soil_evaporation_demand_m", demand_m)?;
    let mut layer_withdrawal_m = vec![0.0_f64; layers.len()];
    let mut remaining_m = demand_m;
    let mut cumulative_depth_m = 0.0;
    for (index, layer) in layers.iter_mut().enumerate() {
        if remaining_m <= WB11_ZERO_THRESHOLD {
            break;
        }
        let previous_depth_m = cumulative_depth_m;
        cumulative_depth_m += layer.depth_m;
        if previous_depth_m >= WB17_SOIL_EVAPORATION_DEPTH_M {
            break;
        }

        let withdrawable_m = if cumulative_depth_m > WB17_SOIL_EVAPORATION_DEPTH_M {
            let layer_interval_m = cumulative_depth_m - previous_depth_m;
            validate_positive(
                "evapotranspiration.soil_evaporation_layer_interval_m",
                layer_interval_m,
            )?;
            let evaporation_interval_m =
                (WB17_SOIL_EVAPORATION_DEPTH_M - previous_depth_m).max(0.0);
            layer.theta_m * evaporation_interval_m / layer_interval_m
        } else {
            layer.theta_m
        };
        validate_nonnegative_direct_m(
            "evapotranspiration.soil_evaporation_withdrawable_m",
            withdrawable_m,
        )?;
        if withdrawable_m > 0.0 {
            let withdrawn_m = remaining_m.min(withdrawable_m);
            layer.theta_m -= withdrawn_m;
            remaining_m -= withdrawn_m;
            layer_withdrawal_m[index] = withdrawn_m;
            if layer.theta_m < 1.0e-10 {
                layer.theta_m = 0.0;
            }
        }
        if cumulative_depth_m > WB17_SOIL_EVAPORATION_DEPTH_M {
            break;
        }
    }
    let soil_evaporation_actual_m = demand_m - remaining_m;
    validate_nonnegative_direct_m(
        "evapotranspiration.soil_evaporation_actual_m",
        soil_evaporation_actual_m,
    )?;
    Ok((soil_evaporation_actual_m, layer_withdrawal_m))
}

fn apply_post_et_upper_limit_redistribution(
    layers: &mut [DirectSubsurfaceLayerState],
    outside_water_active: bool,
) -> Result<(), DirectRuntimeError> {
    for index in (1..layers.len()).rev() {
        let active_cap_m = if outside_water_active {
            (layers[index].upper_limit_m - layers[index].frozen_water_m).max(0.0)
        } else {
            layers[index].upper_limit_m
        };
        validate_nonnegative_direct_m("evapotranspiration.active_cap_m", active_cap_m)?;
        if layers[index].theta_m > active_cap_m + WB11_ZERO_THRESHOLD {
            let excess_m = layers[index].theta_m - active_cap_m;
            layers[index].theta_m = active_cap_m;
            layers[index - 1].theta_m += excess_m;
            validate_nonnegative_direct_m(
                "evapotranspiration.redistributed_layer_theta_m",
                layers[index].theta_m,
            )?;
            validate_nonnegative_direct_m(
                "evapotranspiration.redistributed_layer_theta_m",
                layers[index - 1].theta_m,
            )?;
        }
    }
    Ok(())
}

fn aggregate_soil_water(layers: &[DirectSubsurfaceLayerState]) -> Result<f64, DirectRuntimeError> {
    let mut soil_water_m = 0.0;
    for layer in layers {
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        soil_water_m += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        validate_finite("evapotranspiration.aggregate_soil_water_m", soil_water_m)?;
    }
    Ok(soil_water_m.max(0.0))
}

fn effective_swu_plant_tolerance(raw_plant_tolerance: f64) -> f64 {
    if raw_plant_tolerance <= 0.0 {
        0.25
    } else {
        raw_plant_tolerance.clamp(WB17_PLTOL_MIN, WB17_PLTOL_MAX)
    }
}

fn run_swu_root_uptake(
    layers: &mut [DirectSubsurfaceLayerState],
    transpiration_demand_m: f64,
    effective_root_depth_m: f64,
    plant_tolerance: f64,
    layer_uptake_potential_m: &mut [f64],
    layer_uptake_actual_m: &mut [f64],
    plant_transpiration_m: &mut f64,
) -> Result<(), DirectRuntimeError> {
    let mut rooted_layer_count = layers.len();
    let mut root_cumulative_depth_m = 0.0;
    for (index, layer) in layers.iter().enumerate() {
        root_cumulative_depth_m += layer.depth_m;
        if effective_root_depth_m <= root_cumulative_depth_m + WB11_ZERO_THRESHOLD {
            rooted_layer_count = index + 1;
            break;
        }
    }

    let mut previous_cumulative_potential_m = 0.0;
    let mut layer_cumulative_depth_m = 0.0;
    for index in 0..rooted_layer_count {
        layer_cumulative_depth_m += layers[index].depth_m;
        let gx_m = if index + 1 < rooted_layer_count {
            layer_cumulative_depth_m
        } else {
            effective_root_depth_m
        };
        let cumulative_potential_m = (1.0 - (-WB17_SWU_UB * gx_m / effective_root_depth_m).exp())
            * transpiration_demand_m
            / WB17_SWU_UOB;
        let mut potential_uptake_m = cumulative_potential_m - previous_cumulative_potential_m;
        if potential_uptake_m < 0.0 && potential_uptake_m.abs() <= WB11_ZERO_THRESHOLD {
            potential_uptake_m = 0.0;
        }
        validate_nonnegative_direct_m("root_uptake.potential_uptake_m", potential_uptake_m)?;
        layer_uptake_potential_m[index] = potential_uptake_m;

        let stress_threshold_m = plant_tolerance * layers[index].upper_limit_m;
        validate_nonnegative_direct_m("root_uptake.stress_threshold_m", stress_threshold_m)?;
        let mut layer_uptake_m = potential_uptake_m;
        if stress_threshold_m > 0.0 && layers[index].theta_m < stress_threshold_m {
            layer_uptake_m *= layers[index].theta_m / stress_threshold_m;
        }
        if layers[index].theta_m < layer_uptake_m {
            layer_uptake_m = layers[index].theta_m;
        }
        let remaining_transpiration_m = (transpiration_demand_m - *plant_transpiration_m).max(0.0);
        if layer_uptake_m > remaining_transpiration_m {
            layer_uptake_m = remaining_transpiration_m;
        }
        if layer_uptake_m < 1.0e-10 {
            layer_uptake_m = 0.0;
        }
        layer_uptake_actual_m[index] = layer_uptake_m;
        layers[index].theta_m -= layer_uptake_m;
        if layers[index].theta_m < 1.0e-10 {
            layers[index].theta_m = 0.0;
        }
        *plant_transpiration_m += layer_uptake_m;
        validate_nonnegative_direct_m("root_uptake.layer_theta_after_m", layers[index].theta_m)?;
        validate_nonnegative_direct_m("root_uptake.layer_uptake_m", layer_uptake_m)?;
        previous_cumulative_potential_m = cumulative_potential_m;
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

fn validate_between(
    field: &'static str,
    value: f64,
    minimum: f64,
    maximum: f64,
) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value >= minimum - WB11_ZERO_THRESHOLD && value <= maximum + WB11_ZERO_THRESHOLD {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

fn normalize_within_zero_tolerance(value: f64) -> Result<f64, DirectRuntimeError> {
    validate_finite("evapotranspiration.zero_tolerant_value_m", value)?;
    if value < 0.0 && value.abs() <= WB11_ZERO_THRESHOLD {
        Ok(0.0)
    } else {
        Ok(value)
    }
}
