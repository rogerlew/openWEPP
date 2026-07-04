const SNOWDENSITY09_DENSITY_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL";
const SNOWDENSITY1035_PHASE_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL";
const SNOWDENSITY1037_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1037_MELT_MODEL";
const SNOWDENSITY1038_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL";
const PARADIGM2_STAGE3_LIQUID_MODEL_ENV: &str =
    "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL";

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy)]
struct DirectProductionEvappmSeed {
    et_demand_m: f64,
    soil_evaporation_m: f64,
    plant_transpiration_m: f64,
    soil_evaporation_storage_return_m: f64,
}

#[allow(dead_code)]
impl<'a> DirectProductionDayInputBuilder<'a> {
    fn new(
        climate_request: &'a HillslopeClimateRuntimeRequest,
        climate_span: &'a ClimateRunSpanSummary,
        seed_authority: &DirectProductionSeedAuthority,
    ) -> Result<Self, HillslopeCliError> {
        if seed_authority.lanes.is_empty() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production requires at least one lane seed authority"
                ),
            });
        }
        let lane_authority = seed_authority
            .lanes
            .iter()
            .map(|lane| lane.day_input.clone())
            .collect::<Vec<_>>();
        let sturm_climate_class = direct_production_sturm_climate_class_for_density_candidate(
            climate_request,
            climate_span,
            &lane_authority,
        )?;
        let residue_cover_state = lane_authority
            .iter()
            .map(|authority| authority.residue_cover.initial_state())
            .collect::<Vec<_>>();
        Ok(Self {
            climate_request,
            climate_span,
            lane_authority,
            residue_cover_state: std::cell::RefCell::new(residue_cover_state),
            winter_hourly_geometry: seed_authority.winter_hourly_geometry,
            sturm_climate_class,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn build(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<DirectPublicationDayInput, HillslopeCliError> {
        let (day, simulation_year, forcing) = self.climate_day_for_build(day_index)?;
        let lane = Self::frame_lane_for_build(frame, lane_index)?;
        let authority = self.lane_authority(lane_index)?;
        let precipitation_m = forcing.prcp_m;
        let mut hyetograph = direct_production_hyetograph(&forcing)?;
        let rainfall_input_m = direct_publication_hyetograph_rainfall_m(&hyetograph)?;
        let snow_lane_state = authority.snow_frost.current_snow_lane_state(lane);
        let growth_state_before = *lane.plant_growth_state;
        let pre_growth_evapotranspiration_compute_inputs =
            authority.evapotranspiration.inputs_with_growth_surface(
                &day,
                &forcing,
                lane.evapotranspiration_stage_state.as_deref().copied(),
                &lane.subsurface_layers,
                self.climate_request,
                growth_state_before,
            )?;
        let (annual_growth_inputs, perennial_growth_inputs) = authority.growth.inputs(
            &day,
            simulation_year,
            lane_index + 1,
            &forcing,
            growth_state_before,
            lane.plant_water_stress,
            &pre_growth_evapotranspiration_compute_inputs,
        )?;
        let growth_state_for_publication = direct_production_growth_state_for_publication(
            &annual_growth_inputs,
            &perennial_growth_inputs,
            growth_state_before,
        )?;
        let residue_cover_projection = self.residue_cover_projection_for_build(
            authority,
            day,
            simulation_year,
            lane_index,
            &forcing,
            growth_state_before,
            growth_state_for_publication,
            lane.plant_water_stress,
        )?;
        maybe_write_frost_residue_cover_trace(day_index, lane_index, &residue_cover_projection)?;
        Self::validate_active_snow_forcing(
            authority,
            lane_index,
            &forcing,
            rainfall_input_m,
            snow_lane_state.runtime_swe_m,
        )?;
        let sturm_day_of_year = self.sturm_climate_class.map(|_| f64::from(day.julian_day));
        let snow_liquid = authority.snow_frost.snow_liquid_partition(
            self.climate_request,
            day_index,
            &forcing,
            rainfall_input_m,
            &snow_lane_state,
            growth_state_for_publication.canopy_cover_fraction,
            self.sturm_climate_class,
            sturm_day_of_year,
            self.winter_hourly_geometry,
        )?;
        maybe_write_r7h_direct_production_snow_trace(
            day_index,
            lane_index,
            rainfall_input_m,
            &snow_lane_state,
            authority.snow_frost.snow_melt_model,
            authority.snow_frost.snow_phase_model,
            &snow_liquid,
        )?;
        let frost_context = authority.snow_frost.frost_day_context(
            self.climate_request,
            day_index,
            &day,
            lane_index,
            lane,
            &forcing,
            &snow_lane_state,
            self.winter_hourly_geometry,
            rainfall_input_m > 1.0e-12 || snow_liquid.routed_melt_m > 1.0e-12,
            Some(residue_cover_projection.state_after.residue_depth_m),
        )?;
        let interception_state = compute_direct_canopy_interception(
            DirectCanopyInterceptionInputs {
                hyetograph_rainfall_m: snow_liquid.post_winter_rain_m,
                interception_rainfall_input_m: snow_liquid.post_winter_rain_m,
                canopy_cover_fraction: growth_state_for_publication.canopy_cover_fraction,
                leaf_area_index: growth_state_for_publication.leaf_area_index,
                interception_live_biomass_kg_m2: direct_growth_interception_live_biomass_from_state(
                    growth_state_for_publication,
                )?,
            },
        )
        .map_err(|source| direct_publication_runtime_error(&source))?;
        maybe_write_r7h_direct_production_wb15_trace(
            day_index,
            lane_index,
            growth_state_before,
            growth_state_for_publication,
            snow_liquid.post_winter_rain_m,
            interception_state,
        )?;
        let post_winter_hyetograph = direct_publication_scaled_hyetograph_to_rainfall(
            &hyetograph,
            snow_liquid.post_winter_rain_m,
        )?;
        let post_interception_hyetograph = direct_publication_scaled_hyetograph(
            &post_winter_hyetograph,
            interception_state.rainfall_scale,
        )?;
        hyetograph = direct_publication_hyetograph_with_added_daily_depth(
            &post_interception_hyetograph,
            snow_liquid.routed_melt_m,
        )?;
        let hydrology_layers = frost_context
            .as_ref()
            .map_or(lane.subsurface_layers.as_slice(), |context| {
                context.hydrology_layers.as_slice()
            });

        let mut day_input =
            DirectPublicationDayInput::calendar_only(direct_publication_calendar_day(&day)?);
        day_input.precipitation_m = precipitation_m;
        day_input.effective_temperature_c = day.effective_temperature_c;
        day_input.interception_m = interception_state.interception_m;
        day_input.canopy_cover_fraction = Some(growth_state_for_publication.canopy_cover_fraction);
        day_input.initial_soil_water_m = Some(direct_production_lane_soil_water(lane, lane_index)?);
        day_input.storage_input_inputs = Some(DirectStorageInputInputs {
            precip_input_handoff_m: Some(precipitation_m),
        });
        day_input.liquid_input_inputs =
            Some(direct_publication_liquid_input_inputs(
                interception_state.liquid_after_interception_m + snow_liquid.routed_melt_m,
            )?);
        day_input.snow_coupling_inputs = Some(DirectSnowCouplingInputs {
            snow_coupling_handoff_m: snow_liquid.snow_coupling_signed_s_m,
            snow_state_projected: authority.snow_frost.snow_state_projected(&snow_lane_state),
            active_snow_coupling: snow_liquid.active_snow_coupling,
            raw_melt_m: snow_liquid.raw_melt_m,
            redistributed_melt_m: snow_liquid.redistributed_melt_m,
            routed_melt_m: snow_liquid.routed_melt_m,
            snowpack_swe_loss_m: snow_liquid.snowpack_swe_loss_m,
            sublimation_m: snow_liquid.sublimation_m,
            post_winter_rain_m: snow_liquid.post_winter_rain_m,
            runtime_swe_after_m: snow_liquid.runtime_swe_after_m,
            runtime_depth_after_m: snow_liquid.runtime_depth_after_m,
            runtime_density_after_kg_m3: snow_liquid.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_liquid.runtime_settle_day_count_after,
            coe_boundary_depth_after_m: snow_liquid.coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: snow_liquid.coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: snow_liquid
                .coe_boundary_settle_day_count_after,
            liquid_holding_capacity_after_m: snow_liquid.liquid_holding_capacity_after_m,
            liquid_water_retained_after_m: snow_liquid.liquid_water_retained_after_m,
            liquid_water_released_m: snow_liquid.liquid_water_released_m,
            snow_albedo_state_after: snow_liquid.snow_albedo_state_after,
            snow_layers_after: snow_liquid.snow_layers_after.clone(),
            stage3_diagnostics: snow_liquid.stage3_diagnostics.boxed_when_enabled(),
        });
        day_input.peak_runoff_inputs = Some(authority.peak_runoff.inputs(hyetograph.clone()));
        day_input.infiltration_depression_inputs = Some(
            authority
                .infiltration
                .inputs(
                    lane_index,
                    hydrology_layers,
                    hyetograph,
                    frost_context
                        .as_ref()
                        .map(|context| context.frozen_infiltration_capacity_m_s),
                )?,
        );
        day_input.percolation_inputs =
            Some(authority.percolation_inputs(lane_index, lane, hydrology_layers)?);
        day_input.subsurface_compute_inputs =
            Some(authority.subsurface_inputs(lane_index, hydrology_layers)?);
        let evapotranspiration_compute_inputs = pre_growth_evapotranspiration_compute_inputs;
        day_input.evapotranspiration_compute_inputs = Some(evapotranspiration_compute_inputs);
        day_input.decomposition_inputs = Some(residue_cover_projection.decomposition_inputs);
        day_input.residue_partition_inputs = Some(residue_cover_projection.residue_partition_inputs);
        day_input.annual_growth_inputs = Some(annual_growth_inputs);
        day_input.perennial_growth_inputs = Some(perennial_growth_inputs);
        let mut hydrology_projection_inputs =
            authority.hydrology_projection_inputs(hydrology_layers);
        hydrology_projection_inputs.snow_water_m = snow_liquid.runtime_swe_after_m;
        day_input.hydrology_projection_inputs = Some(hydrology_projection_inputs);
        let erosion_active = direct_production_erosion_active(authority, &day_input)?;
        apply_direct_production_erosion_inputs(&mut day_input, authority, erosion_active);
        apply_direct_production_frost_context(&mut day_input, frost_context);
        day_input.frost_runtime_carry =
            direct_publication_frost_runtime_carry_from_lane_state(&lane.winter_column.frost);
        Ok(day_input)
    }

    fn climate_day_for_build(
        &self,
        day_index: usize,
    ) -> Result<(ClimateDayProjection, i32, HillslopeDirectClimateDayForcing), HillslopeCliError>
    {
        let day = *self.climate_span.days.get(day_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production day index {} exceeds climate span {}",
                    day_index + 1,
                    self.climate_span.days.len()
                ),
            }
        })?;
        direct_publication_validate_day(&day)?;
        let simulation_year =
            simulation_year_from_calendar_year(day.year, self.climate_span.first_day.year)?;
        let forcing =
            self.climate_request
                .direct_day_forcing(day_index)
                .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct production typed climate forcing failed: {source}"
                    ),
                })?;
        Ok((day, simulation_year, forcing))
    }

    fn frame_lane_for_build(
        frame: &DirectRunFrame,
        lane_index: usize,
    ) -> Result<&DirectLaneFrame, HillslopeCliError> {
        frame
            .lanes
            .get(lane_index)
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production lane index {} exceeds frame lane count {}",
                    lane_index + 1,
                    frame.lanes.len()
                ),
            })
    }

    #[allow(clippy::too_many_arguments)]
    fn residue_cover_projection_for_build(
        &self,
        authority: &DirectProductionLaneDayInputAuthority,
        day: ClimateDayProjection,
        simulation_year: i32,
        lane_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        growth_state_before: DirectGrowthStateSurface,
        growth_state_for_publication: DirectGrowthStateSurface,
        plant_water_stress: f64,
    ) -> Result<DirectProductionResidueCoverProjection, HillslopeCliError> {
        let mut states = self.residue_cover_state.borrow_mut();
        if lane_index >= states.len() {
            states.resize(lane_index + 1, authority.residue_cover.initial_state());
        }
        let projection = authority.residue_cover.project_day(
            &authority.growth,
            &day,
            simulation_year,
            lane_index + 1,
            forcing,
            states[lane_index],
            growth_state_before,
            growth_state_for_publication,
            plant_water_stress,
        )?;
        states[lane_index] = projection.state_after;
        Ok(projection)
    }

    fn lane_authority(
        &self,
        lane_index: usize,
    ) -> Result<&DirectProductionLaneDayInputAuthority, HillslopeCliError> {
        if self.lane_authority.len() == 1 {
            return Ok(&self.lane_authority[0]);
        }
        self.lane_authority.get(lane_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production lane index {} exceeds lane authority count {}",
                    lane_index + 1,
                    self.lane_authority.len()
                ),
            }
        })
    }

    fn validate_active_snow_forcing(
        authority: &DirectProductionLaneDayInputAuthority,
        lane_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        hyetograph_rainfall_m: f64,
        runtime_swe_m: f64,
    ) -> Result<(), HillslopeCliError> {
        let _active_snow = authority
            .snow_frost
            .active_forcing(forcing, hyetograph_rainfall_m, runtime_swe_m)?;
        let _ = lane_index;
        Ok(())
    }
}

fn maybe_write_r7h_direct_production_snow_trace(
    day_index: usize,
    lane_index: usize,
    hyetograph_rainfall_m: f64,
    snow_lane_state: &openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel,
    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> Result<(), HillslopeCliError> {
    let Some(path) = std::env::var_os("OPENWEPP_R7H_SNOW_TRACE_PATH") else {
        return Ok(());
    };
    if path.is_empty() {
        return Ok(());
    }
    if let Some(filter_day_index) =
        direct_production_trace_env_usize("OPENWEPP_R7H_SNOW_TRACE_DAY_INDEX")
        && filter_day_index != day_index
    {
        return Ok(());
    }
    if let Some(filter_lane_index) =
        direct_production_trace_env_usize("OPENWEPP_R7H_SNOW_TRACE_LANE_INDEX")
        && filter_lane_index != lane_index
    {
        return Ok(());
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed opening direct production snow trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        })?;
    let line = r7h_direct_production_snow_trace_line(
        day_index,
        lane_index,
        hyetograph_rainfall_m,
        snow_lane_state,
        snow_melt_model,
        snow_phase_model,
        snow_liquid,
    );
    std::io::Write::write_all(&mut file, line.as_bytes()).map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_snow_trace",
        detail: format!(
            "{SIMOUT_GUARD_ID} failed writing direct production snow trace {}: {error}",
            std::path::PathBuf::from(&path).display()
        ),
    })
}

fn r7h_direct_production_snow_trace_line(
    day_index: usize,
    lane_index: usize,
    hyetograph_rainfall_m: f64,
    snow_lane_state: &openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel,
    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> String {
    let layer = direct_snow_trace_layer_diagnostics(snow_lane_state, snow_liquid);
    let line = format!(
        "{{\"schema\":\"openwepp-r7h-direct-production-snow-trace-v1\",\
\"day_index\":{day_index},\
\"lane_index\":{lane_index},\
\"hyetograph_rainfall_m\":{},\
\"runtime_swe_before_m\":{},\
\"runtime_depth_before_m\":{},\
\"runtime_density_before_kg_m3\":{},\
\"runtime_settle_day_count_before\":{},\
\"liquid_water_retained_before_m\":{},\
\"snow_layer_count_before\":{},\
\"snow_layer_swe_sum_before_m\":{},\
\"snow_layer_depth_sum_before_m\":{},\
\"snow_layer_surface_density_before_kg_m3\":{},\
\"snow_layer_basal_density_before_kg_m3\":{},\
\"snow_layer_density_gradient_before_kg_m3\":{},\
\"snow_density_model\":\"{}\",\
\"snow_melt_model\":\"{}\",\
\"snow_phase_model\":\"{}\",\
\"active_snow_coupling\":{},\
\"snow_coupling_signed_s_m\":{},\
\"raw_melt_m\":{},\
\"snowpack_swe_loss_m\":{},\
\"accumulation_m\":{},\
\"sublimation_m\":{},\
\"routed_melt_m\":{},\
\"rain_retained_m\":{},\
\"rain_released_m\":{},\
\"liquid_holding_capacity_after_m\":{},\
\"liquid_water_retained_after_m\":{},\
\"liquid_water_released_m\":{},\
\"post_winter_rain_m\":{},\
\"runtime_swe_after_m\":{},\
\"runtime_depth_after_m\":{},\
\"runtime_density_after_kg_m3\":{},\
\"runtime_settle_day_count_after\":{},\
\"snow_layer_count_after\":{},\
\"snow_layer_swe_sum_after_m\":{},\
\"snow_layer_depth_sum_after_m\":{},\
\"snow_layer_surface_density_after_kg_m3\":{},\
\"snow_layer_basal_density_after_kg_m3\":{},\
\"snow_layer_density_gradient_after_kg_m3\":{}}}",
        direct_production_trace_number(hyetograph_rainfall_m),
        direct_production_trace_number(snow_lane_state.runtime_swe_m),
        direct_production_trace_number(snow_lane_state.runtime_depth_m),
        direct_production_trace_number(snow_lane_state.runtime_density_kg_m3),
        direct_production_trace_number(snow_lane_state.runtime_settle_day_count),
        direct_production_trace_number(snow_lane_state.liquid_water_retained_m),
        layer.count_before,
        direct_production_trace_number(layer.swe_sum_before_m),
        direct_production_trace_number(layer.depth_sum_before_m),
        direct_production_trace_number(layer.surface_density_before_kg_m3),
        direct_production_trace_number(layer.basal_density_before_kg_m3),
        direct_production_trace_number(layer.density_gradient_before_kg_m3),
        snow_liquid.snow_density_model.id(),
        snow_melt_model.id(),
        snow_phase_model.id(),
        snow_liquid.active_snow_coupling,
        direct_production_trace_number(snow_liquid.snow_coupling_signed_s_m),
        direct_production_trace_number(snow_liquid.raw_melt_m),
        direct_production_trace_number(snow_liquid.snowpack_swe_loss_m),
        direct_production_trace_number(snow_liquid.accumulation_m),
        direct_production_trace_number(snow_liquid.sublimation_m),
        direct_production_trace_number(snow_liquid.routed_melt_m),
        direct_production_trace_number(snow_liquid.rain_retained_m),
        direct_production_trace_number(snow_liquid.rain_released_m),
        direct_production_trace_number(snow_liquid.liquid_holding_capacity_after_m),
        direct_production_trace_number(snow_liquid.liquid_water_retained_after_m),
        direct_production_trace_number(snow_liquid.liquid_water_released_m),
        direct_production_trace_number(snow_liquid.post_winter_rain_m),
        direct_production_trace_number(snow_liquid.runtime_swe_after_m),
        direct_production_trace_number(snow_liquid.runtime_depth_after_m),
        direct_production_trace_number(snow_liquid.runtime_density_after_kg_m3),
        direct_production_trace_number(snow_liquid.runtime_settle_day_count_after),
        layer.count_after,
        direct_production_trace_number(layer.swe_sum_after_m),
        direct_production_trace_number(layer.depth_sum_after_m),
        direct_production_trace_number(layer.surface_density_after_kg_m3),
        direct_production_trace_number(layer.basal_density_after_kg_m3),
        direct_production_trace_number(layer.density_gradient_after_kg_m3),
    );
    format!("{line}\n")
}

struct DirectSnowTraceLayerDiagnostics {
    count_before: usize,
    count_after: usize,
    swe_sum_before_m: f64,
    swe_sum_after_m: f64,
    depth_sum_before_m: f64,
    depth_sum_after_m: f64,
    surface_density_before_kg_m3: f64,
    basal_density_before_kg_m3: f64,
    density_gradient_before_kg_m3: f64,
    surface_density_after_kg_m3: f64,
    basal_density_after_kg_m3: f64,
    density_gradient_after_kg_m3: f64,
}

fn direct_snow_trace_layer_diagnostics(
    snow_lane_state: &openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> DirectSnowTraceLayerDiagnostics {
    let (surface_before, basal_before, gradient_before) =
        snow_layer_density_profile(&snow_lane_state.layers);
    let (surface_after, basal_after, gradient_after) =
        snow_layer_density_profile(&snow_liquid.snow_layers_after);
    DirectSnowTraceLayerDiagnostics {
        count_before: snow_lane_state.layers.len(),
        count_after: snow_liquid.snow_layers_after.len(),
        swe_sum_before_m: snow_layer_swe_sum(&snow_lane_state.layers),
        swe_sum_after_m: snow_layer_swe_sum(&snow_liquid.snow_layers_after),
        depth_sum_before_m: snow_layer_depth_sum(&snow_lane_state.layers),
        depth_sum_after_m: snow_layer_depth_sum(&snow_liquid.snow_layers_after),
        surface_density_before_kg_m3: surface_before,
        basal_density_before_kg_m3: basal_before,
        density_gradient_before_kg_m3: gradient_before,
        surface_density_after_kg_m3: surface_after,
        basal_density_after_kg_m3: basal_after,
        density_gradient_after_kg_m3: gradient_after,
    }
}

fn snow_layer_swe_sum(
    layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState],
) -> f64 {
    layers.iter().map(|layer| layer.mass_swe_m).sum()
}

fn snow_layer_depth_sum(
    layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState],
) -> f64 {
    layers.iter().map(|layer| layer.thickness_m).sum()
}

fn snow_layer_density_profile(
    layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState],
) -> (f64, f64, f64) {
    let Some(surface) = layers.first() else {
        return (0.0, 0.0, 0.0);
    };
    let basal = layers.last().unwrap_or(surface);
    let surface_density = surface.density_kg_m3;
    let basal_density = basal.density_kg_m3;
    (surface_density, basal_density, basal_density - surface_density)
}

fn maybe_write_r7h_direct_production_wb15_trace(
    day_index: usize,
    lane_index: usize,
    growth_state_before: DirectGrowthStateSurface,
    growth_state_for_publication: DirectGrowthStateSurface,
    post_winter_rain_m: f64,
    interception_state: openwepp_hillslope_orchestrator::DirectCanopyInterceptionState,
) -> Result<(), HillslopeCliError> {
    let Some(path) = direct_production_trace_output_path("OPENWEPP_R7H_WB15_TRACE_PATH") else {
        return Ok(());
    };
    if !direct_production_trace_filters_allow(
        day_index,
        lane_index,
        "OPENWEPP_R7H_WB15_TRACE_DAY_INDEX",
        "OPENWEPP_R7H_WB15_TRACE_LANE_INDEX",
    ) {
        return Ok(());
    }
    let projected_interception_live_biomass_kg_m2 =
        direct_growth_interception_live_biomass_from_state(growth_state_for_publication)?;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_wb15_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed opening direct production WB15 trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        })?;
    let line = format!(
        "{{\"schema\":\"openwepp-r7h-direct-production-wb15-trace-v1\",\
\"day_index\":{day_index},\
\"lane_index\":{lane_index},\
\"growth_vdmt_before_kg_m2\":{},\
\"growth_tlive_before_kg_m2\":{},\
\"growth_projected_tlive_before_kg_m2\":{},\
\"growth_hia_before\":{},\
\"growth_cancov_before\":{},\
\"growth_lai_before\":{},\
\"publication_vdmt_kg_m2\":{},\
\"publication_hia\":{},\
\"publication_cancov\":{},\
\"publication_lai\":{},\
\"post_winter_rain_m\":{},\
\"interception_m\":{},\
\"liquid_after_interception_m\":{},\
\"rainfall_scale\":{}}}",
        direct_production_trace_number(growth_state_before.live_biomass_kg_m2),
        direct_production_trace_number(growth_state_before.interception_live_biomass_kg_m2),
        direct_production_trace_number(projected_interception_live_biomass_kg_m2),
        direct_production_trace_number(growth_state_before.harvest_index),
        direct_production_trace_number(growth_state_before.canopy_cover_fraction),
        direct_production_trace_number(growth_state_before.leaf_area_index),
        direct_production_trace_number(growth_state_for_publication.live_biomass_kg_m2),
        direct_production_trace_number(growth_state_for_publication.harvest_index),
        direct_production_trace_number(growth_state_for_publication.canopy_cover_fraction),
        direct_production_trace_number(growth_state_for_publication.leaf_area_index),
        direct_production_trace_number(post_winter_rain_m),
        direct_production_trace_number(interception_state.interception_m),
        direct_production_trace_number(interception_state.liquid_after_interception_m),
        direct_production_trace_number(interception_state.rainfall_scale),
    );
    let line = format!("{line}\n");
    std::io::Write::write_all(&mut file, line.as_bytes()).map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_wb15_trace",
        detail: format!(
            "{SIMOUT_GUARD_ID} failed writing direct production WB15 trace {}: {error}",
            std::path::PathBuf::from(&path).display()
        ),
    })
}

fn direct_production_trace_output_path(name: &str) -> Option<std::ffi::OsString> {
    std::env::var_os(name).filter(|path| !path.is_empty())
}

fn direct_production_trace_filters_allow(
    day_index: usize,
    lane_index: usize,
    day_filter_name: &str,
    lane_filter_name: &str,
) -> bool {
    direct_production_trace_index_filter_allows(day_filter_name, day_index)
        && direct_production_trace_index_filter_allows(lane_filter_name, lane_index)
}

fn direct_production_trace_index_filter_allows(name: &str, observed: usize) -> bool {
    match direct_production_trace_env_usize(name) {
        Some(filter) => filter == observed,
        None => true,
    }
}

fn direct_production_trace_env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.trim().parse::<usize>().ok()
}

fn parse_snowdensity1015_default_snow_density_model(
    value: Option<&str>,
) -> Result<openwepp_hillslope_orchestrator::SnowDensityModel, HillslopeCliError> {
    match value.map_or("", str::trim) {
        "" | "physics_bulk_density_compaction_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkDensityCompactionV1)
        }
        "legacy_wepp" => Ok(openwepp_hillslope_orchestrator::SnowDensityModel::LegacyWepp),
        "physics_bulk_shallow_guard_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkShallowGuardV1,
        ),
        "physics_bulk_climate_class_density_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkClimateClassDensityV1,
        ),
        "physics_bulk_multilayer_density_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkMultilayerDensityV1,
        ),
        observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_density_model",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOWDENSITY09_DENSITY_MODEL_ENV} must be legacy_wepp, physics_bulk_density_compaction_v1, physics_bulk_shallow_guard_v1, physics_bulk_climate_class_density_v1, or physics_bulk_multilayer_density_v1, observed {observed}"
            ),
        }),
    }
}

fn parse_snowdensity1037_diagnostic_snow_melt_model(
    value: Option<&str>,
) -> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match value.map_or("", str::trim) {
        "" | "legacy_coe" => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe)
        }
        "coe_winter_thaw_state_loss_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::CoeWinterThawStateLossV1)
        }
        observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOWDENSITY1037_MELT_MODEL_ENV} must be legacy_coe or coe_winter_thaw_state_loss_v1, observed {observed}"
            ),
        }),
    }
}

fn parse_snowdensity1015_default_snow_melt_model(
    value: Option<&str>,
) -> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match value.map_or("", str::trim) {
        "" | "coe_liquid_holding_capacity_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeLiquidHoldingCapacityV1,
        ),
        "coe_open_sublimation_stage_a_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeOpenSublimationStageAV1,
        ),
        "coe_open_sublimation_stage_b_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeOpenSublimationStageBV1,
        ),
        "legacy_coe" => Ok(openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe),
        observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOWDENSITY1038_MELT_MODEL_ENV} must be legacy_coe, coe_liquid_holding_capacity_v1, coe_open_sublimation_stage_a_v1, or coe_open_sublimation_stage_b_v1, observed {observed}"
            ),
        }),
    }
}

fn snowdensity1015_default_snow_density_model(
) -> Result<openwepp_hillslope_orchestrator::SnowDensityModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY09_DENSITY_MODEL_ENV) {
        Ok(value) => parse_snowdensity1015_default_snow_density_model(Some(&value)),
        Err(std::env::VarError::NotPresent) => {
            parse_snowdensity1015_default_snow_density_model(None)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_density_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY09_DENSITY_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn direct_production_sturm_climate_class_for_density_candidate(
    climate_request: &HillslopeClimateRuntimeRequest,
    climate_span: &ClimateRunSpanSummary,
    lane_authority: &[DirectProductionLaneDayInputAuthority],
) -> Result<Option<openwepp_hillslope_orchestrator::SnowClimateClass>, HillslopeCliError> {
    if !lane_authority.iter().any(|authority| {
        authority.snow_frost.snow_density_model
            == openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkClimateClassDensityV1
    }) {
        return Ok(None);
    }
    let normals = direct_production_sturm1995_climate_normals(climate_request, climate_span)?;
    openwepp_hillslope_orchestrator::sturm1995_climate_class_from_normals(normals)
        .map(Some)
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_sturm1995_climate_class",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed assigning Sturm 1995 climate class from run forcing normals: {source}"
            ),
        })
}

fn paradigm2_stage3_liquid_routing_model(
) -> Result<openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel, HillslopeCliError> {
    match std::env::var(PARADIGM2_STAGE3_LIQUID_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "disabled" => {
                Ok(openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled)
            }
            "layered_thermal_liquid_v1" => Ok(
                openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
            ),
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_stage3_liquid_routing_model",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {PARADIGM2_STAGE3_LIQUID_MODEL_ENV} must be disabled, layered_thermal_liquid_v1, or empty default, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => {
            Ok(openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_stage3_liquid_routing_model",
            detail: format!("{SIMOUT_GUARD_ID} {PARADIGM2_STAGE3_LIQUID_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn direct_production_sturm1995_climate_normals(
    climate_request: &HillslopeClimateRuntimeRequest,
    climate_span: &ClimateRunSpanSummary,
) -> Result<openwepp_hillslope_orchestrator::Sturm1995ClimateNormals, HillslopeCliError> {
    if climate_span.days.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_sturm1995_climate_class",
            detail: format!(
                "{SIMOUT_GUARD_ID} cannot assign Sturm 1995 climate class for empty climate span"
            ),
        });
    }
    let mut months = [DirectProductionSturm1995MonthlyAccumulator::default(); 12];
    for (day_index, day) in climate_span.days.iter().enumerate() {
        let forcing = climate_request.direct_day_forcing(day_index).map_err(|source| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_sturm1995_climate_class",
                detail: format!(
                    "{SIMOUT_GUARD_ID} failed reading daily forcing for Sturm 1995 climate normals: {source}"
                ),
            }
        })?;
        let month_index =
            usize::try_from(day.month - 1).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_sturm1995_climate_class",
                detail: format!(
                    "{SIMOUT_GUARD_ID} invalid climate month {} for Sturm 1995 climate normals",
                    day.month
                ),
            })?;
        let Some(month) = months.get_mut(month_index) else {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_sturm1995_climate_class",
                detail: format!(
                    "{SIMOUT_GUARD_ID} invalid climate month {} for Sturm 1995 climate normals",
                    day.month
                ),
            });
        };
        month.add(
            f64::midpoint(forcing.tmax_c, forcing.tmin_c),
            (forcing.prcp_m * 1_000.0).max(0.0),
            forcing.vwind_m_s,
        );
    }

    let mut cdm_c_month = 0.0;
    let mut spr_sum_mm_day = 0.0;
    let mut cold_month_count = 0u32;
    let mut winter_wind_sum_m_s = 0.0;
    let mut winter_wind_day_count = 0u32;
    for month in months.iter().filter(|month| month.day_count > 0) {
        let mean_temperature_c = month.mean_temperature_c();
        if mean_temperature_c < openwepp_hillslope_orchestrator::STURM1995_CDM_CRITICAL_TEMPERATURE_C
        {
            cdm_c_month += openwepp_hillslope_orchestrator::STURM1995_CDM_CRITICAL_TEMPERATURE_C
                - mean_temperature_c;
            spr_sum_mm_day += month.mean_precipitation_mm_day();
            cold_month_count += 1;
            winter_wind_sum_m_s += month.wind_m_s_sum;
            winter_wind_day_count += month.day_count;
        }
    }
    Ok(openwepp_hillslope_orchestrator::Sturm1995ClimateNormals {
        cooling_degree_month_c: cdm_c_month,
        snowfall_precipitation_rate_mm_day: if cold_month_count > 0 {
            spr_sum_mm_day / f64::from(cold_month_count)
        } else {
            0.0
        },
        winter_wind_m_s: if winter_wind_day_count > 0 {
            winter_wind_sum_m_s / f64::from(winter_wind_day_count)
        } else {
            0.0
        },
    })
}

#[derive(Clone, Copy, Default)]
struct DirectProductionSturm1995MonthlyAccumulator {
    temperature_c_sum: f64,
    precipitation_mm_sum: f64,
    wind_m_s_sum: f64,
    day_count: u32,
}

impl DirectProductionSturm1995MonthlyAccumulator {
    fn add(&mut self, temperature_c: f64, precipitation_mm: f64, wind_m_s: f64) {
        self.temperature_c_sum += temperature_c;
        self.precipitation_mm_sum += precipitation_mm;
        self.wind_m_s_sum += wind_m_s;
        self.day_count += 1;
    }

    fn mean_temperature_c(self) -> f64 {
        self.temperature_c_sum / f64::from(self.day_count)
    }

    fn mean_precipitation_mm_day(self) -> f64 {
        self.precipitation_mm_sum / f64::from(self.day_count)
    }
}

fn snowdensity1035_diagnostic_snow_phase_model(
) -> Result<openwepp_hillslope_orchestrator::SnowPhasePartitionModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY1035_PHASE_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "harder_pomeroy_hourly" => Ok(
                openwepp_hillslope_orchestrator::SnowPhasePartitionModel::HarderPomeroyHourly,
            ),
            "legacy_rst" => {
                Ok(openwepp_hillslope_orchestrator::SnowPhasePartitionModel::LegacyRst)
            }
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_phase_model",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {SNOWDENSITY1035_PHASE_MODEL_ENV} must be legacy_rst, harder_pomeroy_hourly, or empty default, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => {
            Ok(openwepp_hillslope_orchestrator::SnowPhasePartitionModel::HarderPomeroyHourly)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_phase_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY1035_PHASE_MODEL_ENV} must be UTF-8"),
        }),
    }
}

#[allow(dead_code)]
fn snowdensity1037_diagnostic_snow_melt_model(
) -> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY1037_MELT_MODEL_ENV) {
        Ok(value) => parse_snowdensity1037_diagnostic_snow_melt_model(Some(&value)),
        Err(std::env::VarError::NotPresent) => {
            parse_snowdensity1037_diagnostic_snow_melt_model(None)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY1037_MELT_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn snowdensity1015_default_snow_melt_model(
) -> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY1038_MELT_MODEL_ENV) {
        Ok(value) => parse_snowdensity1015_default_snow_melt_model(Some(&value)),
        Err(std::env::VarError::NotPresent) => {
            parse_snowdensity1015_default_snow_melt_model(None)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY1038_MELT_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn direct_production_erosion_active(
    authority: &DirectProductionLaneDayInputAuthority,
    day_input: &DirectPublicationDayInput,
) -> Result<bool, HillslopeCliError> {
    // SC-SED-001 1b-C: erosion is active when EITHER the multi-OFE Wave-2
    // router is on OR the single-OFE Wave-1 sediment-continuity seed is
    // enabled. The prior `wave2_enabled`-only gate suppressed the
    // single-OFE Wave-1 path entirely (`wave2_enabled` is false for one OFE),
    // so the seed never reached the day frame.
    if !authority.erosion.wave2_enabled
        && !authority.erosion.erosion_inputs.wave1_operand_seed.enabled
    {
        return Ok(false);
    }
    // SC-SED-001 1b-C: the single-OFE Wave-1 seed must attach EVERY day so
    // the persistent consolidation carry (`rfcum`/`daydis`) advances daily
    // per `soil.for` (aging on dry days after `rfcum > 0.01`), not only on
    // rainfall days. The solve still gates itself inactive on non-runoff
    // days, so this changes no sediment output — only the carry lineage.
    if authority.erosion.erosion_inputs.wave1_operand_seed.enabled {
        return Ok(true);
    }
    // Wave-2 (EROD14) multi-OFE path keeps the rainfall activation gate.
    let rainfall_m = direct_publication_hyetograph_rainfall_m(
        day_input
            .peak_runoff_inputs
            .as_ref()
            .map_or(&[][..], |inputs| inputs.hyetograph.as_slice()),
    )?;
    Ok(rainfall_m >= DIRECT_PUBLICATION_EROSION_MIN_POST_INTERCEPTION_RAINFALL_M)
}

fn apply_direct_production_erosion_inputs(
    day_input: &mut DirectPublicationDayInput,
    authority: &DirectProductionLaneDayInputAuthority,
    erosion_active: bool,
) {
    day_input.erosion_producer_required = erosion_active;
    if erosion_active {
        day_input.erosion_inputs = Some(authority.erosion.erosion_inputs.clone());
    }
}

fn apply_direct_production_frost_context(
    day_input: &mut DirectPublicationDayInput,
    frost_context: Option<DirectProductionFrostDayContext>,
) {
    if let Some(frost_context) = frost_context {
        day_input.winter_frost_compute_inputs = Some(frost_context.compute_inputs);
        day_input.winter_frost_outcome = Some(Box::new(frost_context.frost_outcome));
        day_input.frost_storage_liquid_delta_m = frost_context.storage_liquid_delta_m;
        day_input.frost_layer_carry_projection = frost_context.layer_carry_projection;
    }
}

fn direct_production_trace_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.17}")
    } else {
        "null".to_string()
    }
}

fn direct_production_growth_state_for_publication(
    annual_growth_inputs: &DirectGrowthInputs,
    perennial_growth_inputs: &DirectGrowthInputs,
    growth_state_before: DirectGrowthStateSurface,
) -> Result<DirectGrowthStateSurface, HillslopeCliError> {
    if perennial_growth_inputs.active_context.is_active() {
        return (*perennial_growth_inputs)
            .compute_perennial()
            .map(|growth| growth.state_after)
            .map_err(|source| direct_publication_runtime_error(&source));
    }
    if annual_growth_inputs.active_context.is_active() {
        return (*annual_growth_inputs)
            .compute_annual_or_fallow()
            .map(|growth| growth.state_after)
            .map_err(|source| direct_publication_runtime_error(&source));
    }
    Ok(growth_state_before)
}
