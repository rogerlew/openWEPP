const SNOWDENSITY09_DENSITY_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL";
const SNOWDENSITY1035_PHASE_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL";
const SNOWDENSITY1037_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1037_MELT_MODEL";
const SNOWDENSITY1038_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL";
const PARADIGM2_STAGE3_LIQUID_MODEL_ENV: &str = "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL";
const SNOW_SURFACE_LONGWAVE_MODEL_ENV: &str = "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL";
const SNOW_SURFACE_SUBLIMATION_MODEL_ENV: &str = "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL";
const SNOW_STAGE3_COMPLETE_CARRIER_SHADOW_ENV: &str =
    "OPENWEPP_SNOW_STAGE3_COMPLETE_CARRIER_SHADOW";
const SNOW_STAGE3_EVALUATION_OPERATOR_ENV: &str =
    "OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR";
#[derive(Clone, Debug)]
struct CanopyResearchTraceConfig {
    path: std::ffi::OsString,
    site_id: String,
    arm_id: String,
}

#[cfg(test)]
fn canopy_research_trace_test_config(
) -> &'static std::sync::Mutex<Option<CanopyResearchTraceConfig>> {
    static CONFIG: std::sync::OnceLock<std::sync::Mutex<Option<CanopyResearchTraceConfig>>> =
        std::sync::OnceLock::new();
    CONFIG.get_or_init(|| std::sync::Mutex::new(None))
}

#[cfg(test)]
fn set_canopy_research_trace_test_config(config: Option<CanopyResearchTraceConfig>) {
    *canopy_research_trace_test_config()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = config;
}

fn canopy_research_trace_config_from_values(
    path: Option<std::ffi::OsString>,
    site_id: Option<String>,
    arm_id: Option<String>,
) -> Result<Option<CanopyResearchTraceConfig>, HillslopeCliError> {
    let Some(path) = path.filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    let required_identity = |name: &'static str, value: Option<String>| {
        value
            .filter(|identity| !identity.trim().is_empty())
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "canopy_research_trace",
                detail: format!(
                    "{SIMOUT_GUARD_ID} enabled canopy research trace requires nonempty {name}"
                ),
            })
    };
    Ok(Some(CanopyResearchTraceConfig {
        path,
        site_id: required_identity("OPENWEPP_CANOPY_RESEARCH_SITE_ID", site_id)?,
        arm_id: required_identity("OPENWEPP_CANOPY_RESEARCH_ARM_ID", arm_id)?,
    }))
}

fn canopy_research_trace_config(
) -> Result<Option<CanopyResearchTraceConfig>, HillslopeCliError> {
    #[cfg(test)]
    if let Some(config) = canopy_research_trace_test_config()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
    {
        return Ok(Some(config));
    }
    canopy_research_trace_config_from_values(
        std::env::var_os("OPENWEPP_CANOPY_RESEARCH_TRACE_PATH"),
        std::env::var("OPENWEPP_CANOPY_RESEARCH_SITE_ID").ok(),
        std::env::var("OPENWEPP_CANOPY_RESEARCH_ARM_ID").ok(),
    )
}

fn direct_native_forest_vpd_pa(
    forcing: &HillslopeDirectClimateDayForcing,
) -> Result<f64, HillslopeCliError> {
    let mean_saturation_kpa = 0.5
        * (saturation_vapor_pressure_kpa(forcing.tmax_c)
            + saturation_vapor_pressure_kpa(forcing.tmin_c));
    let actual_vapor_pressure_kpa = saturation_vapor_pressure_kpa(forcing.tdpt_c);
    let vapor_pressure_deficit_kpa = mean_saturation_kpa - actual_vapor_pressure_kpa;
    if !vapor_pressure_deficit_kpa.is_finite() || vapor_pressure_deficit_kpa < 0.0 {
        return Err(direct_growth_failure(format!(
            "native forest GSI derived VPD must be finite and nonnegative, observed {vapor_pressure_deficit_kpa} kPa"
        )));
    }
    Ok(vapor_pressure_deficit_kpa * 1_000.0)
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy)]
struct DirectProductionEvappmSeed {
    et_demand_m: f64,
    soil_evaporation_m: f64,
    plant_transpiration_m: f64,
    soil_evaporation_storage_return_m: f64,
}

struct DirectProductionGrowthBuildState {
    pre_growth_evapotranspiration_compute_inputs: DirectEvapotranspirationComputeInputs,
    annual_growth_inputs: DirectGrowthInputs,
    perennial_growth_inputs: DirectGrowthInputs,
    growth_state_before: DirectGrowthStateSurface,
    growth_state_for_publication: DirectGrowthStateSurface,
    native_canopy: Option<openwepp_plant_phenology::ForestCanopyRealization>,
    native_canopy_daily: Option<openwepp_plant_phenology::ForestCanopyDailyResult>,
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
        let forest_canopy_state = vec![None; lane_authority.len()];
        let canopy_research_pending = vec![None; lane_authority.len()];
        let persistent_enabled = lane_authority.iter().any(|authority| {
            authority.snow_frost.snow_stage3_evaluation_operator
                == Some(openwepp_hillslope_orchestrator::SnowStage3EvaluationOperator::PersistentAccumulationShadowV1)
        });
        let persistent_lane_count = lane_authority.len();
        Ok(Self {
            climate_request,
            climate_span,
            lane_authority,
            residue_cover_state: std::cell::RefCell::new(residue_cover_state),
            forest_canopy_state: std::cell::RefCell::new(forest_canopy_state),
            canopy_research_pending: std::cell::RefCell::new(canopy_research_pending),
            snow_stage3_historical_evaluation_state: persistent_enabled
                .then(|| std::cell::RefCell::new(vec![None; persistent_lane_count])),
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
        let DirectProductionGrowthBuildState {
            pre_growth_evapotranspiration_compute_inputs,
            annual_growth_inputs,
            perennial_growth_inputs,
            growth_state_before,
            growth_state_for_publication,
            native_canopy,
            native_canopy_daily,
        } = self.growth_state_for_build(
            authority,
            &day,
            simulation_year,
            lane_index,
            &forcing,
            lane,
        )?;
        let residue_cover_projection = self.residue_cover_projection_for_build(
            authority,
            day,
            simulation_year,
            lane_index,
            &forcing,
            growth_state_before,
            growth_state_for_publication,
            native_canopy.map(|canopy| canopy.leaf_off_litter_kg_m2),
            lane.plant_water_stress,
        )?;
        maybe_write_frost_residue_cover_trace(day_index, lane_index, &residue_cover_projection)?;
        Self::validate_active_snow_forcing(
            lane_index,
            rainfall_input_m,
            snow_lane_state.runtime_swe_m,
        )?;
        let sturm_day_of_year = self.sturm_climate_class.map(|_| f64::from(day.julian_day));
        let snow_diagnostic_capture =
            DirectSnowDiagnosticCaptureRequest::resolve(day_index, lane_index);
        let persistent_state = self
            .snow_stage3_historical_evaluation_state
            .as_ref()
            .and_then(|states| {
            states.borrow().get(lane_index).cloned().flatten()
        });
        let snow_result = authority.snow_frost.snow_liquid_partition(
            self.climate_request,
            day_index,
            &forcing,
            rainfall_input_m,
            &snow_lane_state,
            growth_state_for_publication.canopy_cover_fraction,
            self.sturm_climate_class,
            sturm_day_of_year,
            self.winter_hourly_geometry,
            snow_diagnostic_capture.capture,
            lane_index,
            u64::try_from(day_index).map_err(|_| direct_production_executor_blocked(
                "day index cannot be represented by persistent snow shadow",
            ))?,
            persistent_state.as_ref(),
        )?;
        let persistent_day = snow_result.persistent;
        let persistent_next_state = persistent_day
            .as_ref()
            .map(|persistent| persistent.state.clone());
        let mut snow_reconciliation = snow_result.standard.reconciliation;
        let mut snow_evaluation = snow_result.standard.result;
        if let Some(persistent) = persistent_day.as_ref() {
            snow_evaluation.evaluation = Some(persistent.evaluation);
            snow_reconciliation = Some(persistent.reconciliation.clone());
        }
        let snow_liquid = snow_evaluation.authoritative;
        let snow_trace_row = DirectSnowTraceRowContext {
            day_index,
            lane_index,
            hyetograph_rainfall_m: rainfall_input_m,
            snow_lane_state: &snow_lane_state,
            snow_melt_model: authority.snow_frost.snow_melt_model,
            snow_phase_model: authority.snow_frost.snow_phase_model,
            snow_liquid: &snow_liquid,
            stage3_evaluation: snow_evaluation.evaluation.as_ref(),
            stage3_reconciliation: snow_reconciliation.as_deref(),
            stage3_persistent: persistent_day.as_ref(),
        };
        let frost_context = authority.snow_frost.frost_day_context(
            self.climate_request,
            day_index,
            &day,
            lane_index,
            lane,
            &forcing,
            &snow_lane_state,
            self.winter_hourly_geometry,
            rainfall_input_m > 1.0e-12
                || snow_liquid
                    .solid_to_liquid_ledger()
                    .liquid_handoff_m
                    > 1.0e-12,
            Some(residue_cover_projection.state_after.residue_depth_m),
            Some(growth_state_for_publication.canopy_height_m),
        )?;
        let interception_inputs = DirectCanopyInterceptionInputs {
            hyetograph_rainfall_m: snow_liquid.post_winter_rain_m,
            interception_rainfall_input_m: snow_liquid.post_winter_rain_m,
            canopy_cover_fraction: growth_state_for_publication.canopy_cover_fraction,
            leaf_area_index: growth_state_for_publication.leaf_area_index,
            interception_live_biomass_kg_m2: direct_growth_interception_live_biomass_from_state(
                growth_state_for_publication,
            )?,
        };
        let interception_state = compute_direct_canopy_interception(interception_inputs)
            .map_err(|source| direct_publication_runtime_error(&source))?;
        if let Some(daily) = native_canopy_daily {
            let trace = NativeCanopyBuilderTrace {
                day_index,
                lane_index,
                year: day.year,
                month: day.month,
                day_of_month: day.day_of_month,
                daily,
                leaf_litter_input_kg_m2: residue_cover_projection.leaf_litter_input_kg_m2,
                needle_litter_input_kg_m2: (residue_cover_projection.needle_litter_status
                    == "complete")
                    .then_some(residue_cover_projection.needle_litter_input_kg_m2),
                fine_woody_litter_input_kg_m2: (residue_cover_projection
                    .fine_woody_litter_status
                    == "complete")
                    .then_some(residue_cover_projection.fine_woody_litter_input_kg_m2),
                needle_litter_status: residue_cover_projection.needle_litter_status,
                needle_litter_source_mode: residue_cover_projection.needle_litter_source_mode,
                fine_woody_litter_status: residue_cover_projection.fine_woody_litter_status,
                fine_woody_litter_source_mode: residue_cover_projection
                    .fine_woody_litter_source_mode,
                litter_source_completeness: if residue_cover_projection.needle_litter_status
                    == "not_represented"
                    || residue_cover_projection.fine_woody_litter_status == "not_represented"
                {
                    "incomplete"
                } else {
                    "complete"
                },
                #[cfg(test)]
                canopy: daily.canopy,
                snow_canopy_cover_fraction: growth_state_for_publication.canopy_cover_fraction,
                interception_inputs,
                #[cfg(test)]
                interception_state,
                #[cfg(test)]
                projected_surface_residue_kg_m2: residue_cover_projection
                    .state_after
                    .surface_residue_kg_m2,
                #[cfg(test)]
                projected_residue_depth_m: residue_cover_projection.state_after.residue_depth_m,
                #[cfg(test)]
                frost_residue_depth_m: frost_context
                    .as_ref()
                    .map(|context| context.compute_inputs.thermal.residue_depth_m),
                #[cfg(test)]
                frost_canopy_height_m: frost_context
                    .as_ref()
                    .map(|context| context.compute_inputs.thermal.canopy_height_m),
            };
            let mut pending = self.canopy_research_pending.borrow_mut();
            if lane_index >= pending.len() {
                pending.resize(lane_index + 1, None);
            }
            pending[lane_index] = Some(trace);
            #[cfg(test)]
            record_native_canopy_builder_trace(&trace);
        }
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
        hyetograph = post_interception_hyetograph;
        let hourly_routed_melt_m = snow_liquid.hourly_routed_melt_m;
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
        day_input.liquid_input_inputs = Some(direct_publication_liquid_input_inputs(
            interception_state.liquid_after_interception_m
                + snow_liquid.solid_to_liquid_ledger().liquid_handoff_m,
        )?);
        day_input.snow_coupling_inputs = Some(DirectSnowCouplingInputs {
            snow_coupling_handoff_m: snow_liquid.snow_coupling_signed_s_m,
            snow_state_projected: authority.snow_frost.snow_state_projected(&snow_lane_state),
            active_snow_coupling: snow_liquid.active_snow_coupling,
            mass_transition_ledgers: Box::new(snow_liquid.mass_transition_ledgers),
            hourly_routed_melt_m: snow_liquid.hourly_routed_melt_m,
            sublimation_m: snow_liquid.sublimation_m,
            post_winter_rain_m: snow_liquid.post_winter_rain_m,
            runtime_swe_after_m: snow_liquid.runtime_swe_after_m,
            runtime_depth_after_m: snow_liquid.runtime_depth_after_m,
            runtime_density_after_kg_m3: snow_liquid.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_liquid.runtime_settle_day_count_after,
            coe_boundary_depth_after_m: snow_liquid.coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: snow_liquid.coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: snow_liquid.coe_boundary_settle_day_count_after,
            liquid_holding_capacity_after_m: snow_liquid.liquid_holding_capacity_after_m,
            liquid_water_retained_after_m: snow_liquid.liquid_water_retained_after_m,
            liquid_water_released_m: snow_liquid.liquid_water_released_m,
            snow_albedo_state_after: snow_liquid.snow_albedo_state_after,
            snow_layers_after: snow_liquid.snow_layers_after.clone(),
        });
        day_input.infiltration_depression_inputs = Some(
            authority.infiltration.inputs(
                lane_index,
                hydrology_layers,
                hyetograph,
                hourly_routed_melt_m,
                frost_context
                    .as_ref()
                    .map(|context| context.frozen_infiltration_capacity_m_s),
            )?,
        );
        day_input.percolation_inputs =
            Some(authority.percolation_inputs(lane_index, lane, hydrology_layers)?);
        day_input.subsurface_compute_inputs =
            Some(authority.subsurface_inputs(lane_index, hydrology_layers)?);
        let evapotranspiration_compute_inputs = if native_canopy.is_some() {
            authority.evapotranspiration.inputs_with_growth_surface(
                &day,
                &forcing,
                lane.evapotranspiration_stage_state.as_deref().copied(),
                &lane.subsurface_layers,
                self.climate_request,
                growth_state_for_publication,
            )?
        } else {
            pre_growth_evapotranspiration_compute_inputs
        };
        day_input.evapotranspiration_compute_inputs = Some(evapotranspiration_compute_inputs);
        day_input.decomposition_inputs = Some(residue_cover_projection.decomposition_inputs);
        day_input.residue_partition_inputs =
            Some(residue_cover_projection.residue_partition_inputs);
        day_input.annual_growth_inputs = Some(annual_growth_inputs);
        day_input.perennial_growth_inputs = Some(perennial_growth_inputs);
        let mut hydrology_projection_inputs =
            authority.hydrology_projection_inputs(hydrology_layers);
        hydrology_projection_inputs.snow_water_m = snow_liquid.runtime_swe_after_m;
        day_input.hydrology_projection_inputs = Some(hydrology_projection_inputs);
        let erosion_active = direct_production_erosion_active(authority);
        apply_direct_production_erosion_inputs(&mut day_input, authority, erosion_active);
        apply_direct_production_frost_context(&mut day_input, frost_context);
        day_input.frost_runtime_carry =
            direct_publication_frost_runtime_carry_from_lane_state(&lane.winter_column.frost);
        maybe_write_r7h_direct_production_snow_trace(
            &snow_diagnostic_capture,
            &snow_trace_row,
        )?;
        if let (Some(states), Some(next_state)) =
            (
                &self.snow_stage3_historical_evaluation_state,
                persistent_next_state,
            )
        {
            states.borrow_mut()[lane_index] = Some(next_state);
        }
        Ok(day_input)
    }

    #[allow(clippy::too_many_arguments)]
    fn growth_state_for_build(
        &self,
        authority: &DirectProductionLaneDayInputAuthority,
        day: &ClimateDayProjection,
        simulation_year: i32,
        lane_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        lane: &DirectLaneFrame,
    ) -> Result<DirectProductionGrowthBuildState, HillslopeCliError> {
        let growth_state_before = *lane.plant_growth_state;
        let pre_growth_evapotranspiration_compute_inputs =
            authority.evapotranspiration.inputs_with_growth_surface(
                day,
                forcing,
                lane.evapotranspiration_stage_state.as_deref().copied(),
                &lane.subsurface_layers,
                self.climate_request,
                growth_state_before,
            )?;
        let (mut annual_growth_inputs, mut perennial_growth_inputs) = authority.growth.inputs(
            day,
            simulation_year,
            lane_index + 1,
            forcing,
            growth_state_before,
            lane.plant_water_stress,
            &pre_growth_evapotranspiration_compute_inputs,
        )?;
        let baseline_growth_state_for_publication = direct_production_growth_state_for_publication(
            &annual_growth_inputs,
            &perennial_growth_inputs,
            growth_state_before,
        )?;
        let (growth_state_for_publication, native_canopy_daily) = self
            .native_forest_growth_state_for_build(
                authority,
                *day,
                simulation_year,
                lane_index,
                forcing,
                baseline_growth_state_for_publication,
            )?;
        let native_canopy = native_canopy_daily.map(|daily| daily.canopy);
        if native_canopy.is_some() {
            if perennial_growth_inputs.active_context.is_active() {
                perennial_growth_inputs.state_before = growth_state_for_publication;
                perennial_growth_inputs.active_action = DirectGrowthAction::TypedStateOverride;
            } else if annual_growth_inputs.active_context.is_active() {
                annual_growth_inputs.state_before = growth_state_for_publication;
                annual_growth_inputs.active_action = DirectGrowthAction::TypedStateOverride;
            } else {
                return Err(direct_growth_failure(
                    "native forest GSI produced state without an active growth consumer",
                ));
            }
        }
        Ok(DirectProductionGrowthBuildState {
            pre_growth_evapotranspiration_compute_inputs,
            annual_growth_inputs,
            perennial_growth_inputs,
            growth_state_before,
            growth_state_for_publication,
            native_canopy,
            native_canopy_daily,
        })
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
        let forcing = self
            .climate_request
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
        native_leaf_off_litter_kg_m2: Option<f64>,
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
            native_leaf_off_litter_kg_m2,
            plant_water_stress,
        )?;
        states[lane_index] = projection.state_after;
        Ok(projection)
    }

    fn native_forest_growth_state_for_build(
        &self,
        authority: &DirectProductionLaneDayInputAuthority,
        day: ClimateDayProjection,
        simulation_year: i32,
        lane_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        mut growth_state: DirectGrowthStateSurface,
    ) -> Result<
        (
            DirectGrowthStateSurface,
            Option<openwepp_plant_phenology::ForestCanopyDailyResult>,
        ),
        HillslopeCliError,
    > {
        let runtime_year =
            direct_growth_i32_to_usize("simulation_year", simulation_year, 1, usize::MAX)?;
        let runtime_day = direct_growth_u16_to_usize("day", day.julian_day, 1, 366)?;
        let Some(selection) =
            authority
                .growth
                .active_crop(runtime_year, runtime_day, lane_index + 1)?
        else {
            return Ok((growth_state, None));
        };
        let Some(phenology) = selection.crop.forest_phenology else {
            return Ok((growth_state, None));
        };
        openwepp_hillslope_orchestrator::validate_direct_native_canopy_height_parameters(
            selection.crop.bbb,
            selection.crop.hmax,
        )
        .map_err(|source| direct_growth_failure(source.to_string()))?;
        let parameters = openwepp_plant_phenology::ForestCanopyParameters {
            gsi: openwepp_plant_phenology::GsiParameters {
                minimum_temperature_inactive_c: phenology.minimum_temperature_inactive_c,
                minimum_temperature_unconstrained_c: phenology.minimum_temperature_unconstrained_c,
                vapor_pressure_deficit_unconstrained_pa: phenology
                    .vapor_pressure_deficit_unconstrained_pa,
                vapor_pressure_deficit_inactive_pa: phenology.vapor_pressure_deficit_inactive_pa,
                photoperiod_inactive_hours: phenology.photoperiod_inactive_hours,
                photoperiod_unconstrained_hours: phenology.photoperiod_unconstrained_hours,
            },
            summer_foliar_biomass_kg_m2: phenology.summer_foliar_biomass_kg_m2,
            maximum_leaf_area_index: selection.crop.xmxlai,
            evergreen_fraction: phenology.evergreen_fraction,
            structural_canopy_cover_fraction: phenology.structural_canopy_cover_fraction,
            structural_biomass_kg_m2: phenology.structural_biomass_kg_m2,
            canopy_cover_coefficient_m2_kg: selection.crop.bb,
        };
        let gsi_forcing = openwepp_plant_phenology::GsiDailyForcing {
            minimum_temperature_c: forcing.tmin_c,
            vapor_pressure_deficit_pa: direct_native_forest_vpd_pa(forcing)?,
            latitude_degrees: self.climate_request.direct_latitude_degrees(),
            date: openwepp_plant_phenology::GsiDate {
                year: day.year,
                ordinal_day: day.julian_day,
            },
        };
        let mut states = self.forest_canopy_state.borrow_mut();
        if lane_index >= states.len() {
            states.resize(lane_index + 1, None);
        }
        let (daily, canopy_height_m) = advance_native_canopy_with_checked_height(
            &mut states[lane_index],
            parameters,
            gsi_forcing,
            selection.crop.bbb,
            selection.crop.hmax,
        )?;
        let canopy = daily.canopy;
        growth_state.live_biomass_kg_m2 = canopy.live_foliar_biomass_kg_m2;
        growth_state.interception_live_biomass_kg_m2 = canopy.live_foliar_biomass_kg_m2;
        growth_state.leaf_area_index = canopy.leaf_area_index;
        growth_state.canopy_cover_fraction = canopy.canopy_cover_fraction;
        growth_state.canopy_height_m = canopy_height_m;
        Ok((growth_state, Some(daily)))
    }

    pub(crate) fn canopy_research_trace_for(
        &self,
        day_index: usize,
        lane_index: usize,
    ) -> Option<NativeCanopyBuilderTrace> {
        self.canopy_research_pending
            .borrow()
            .get(lane_index)
            .and_then(|trace| *trace)
            .filter(|trace| trace.day_index == day_index)
    }

    pub(crate) fn write_canopy_research_trace(
        &self,
        day_frame: &openwepp_hillslope_orchestrator::DirectDayFrame,
        builder: Option<NativeCanopyBuilderTrace>,
    ) -> Result<(), HillslopeCliError> {
        let Some(config) = canopy_research_trace_config()? else {
            return Ok(());
        };
        let Some(builder) = builder else {
            return Ok(());
        };
        let growth_state_after = if day_frame.perennial_growth_inputs.active_context.is_active() {
            day_frame.perennial_growth.state_after
        } else {
            day_frame.annual_growth.state_after
        };
        let surface_before_decay_kg_m2 =
            day_frame.decomposition.surface_residue_seed_kg_m2
                + day_frame.decomposition.surface_litter_input_kg_m2;
        let decomposition_loss_kg_m2 =
            surface_before_decay_kg_m2 * (1.0 - day_frame.decomposition.surface_decay_factor);
        let gsi = builder.daily.gsi;
        let canopy = builder.daily.canopy;
        let value = serde_json::json!({
            "schema": "openwepp-canopy-research-daily-v1",
            "date": format!("{:04}-{:02}-{:02}", builder.year, builder.month, builder.day_of_month),
            "year": builder.year,
            "day_of_year": self.climate_span.days[builder.day_index].julian_day,
            "day_index": builder.day_index,
            "lane_index": builder.lane_index,
            "site_id": config.site_id,
            "arm_id": config.arm_id,
            "gsi": {
                "minimum_temperature_indicator": gsi.indicators.minimum_temperature,
                "vapor_pressure_deficit_indicator": gsi.indicators.vapor_pressure_deficit,
                "photoperiod_indicator": gsi.indicators.photoperiod,
                "photoperiod_hours": gsi.indicators.photoperiod_hours,
                "instantaneous": gsi.indicators.instantaneous_gsi,
                "gsi21": gsi.growing_season_index,
                "sample_count": gsi.sample_count
            },
            "canopy": {
                "structural_biomass_kg_m2": canopy.structural_biomass_kg_m2,
                "evergreen_foliar_biomass_kg_m2": canopy.evergreen_foliar_biomass_kg_m2,
                "deciduous_foliar_biomass_kg_m2": canopy.deciduous_foliar_biomass_kg_m2,
                "total_foliar_biomass_kg_m2": canopy.live_foliar_biomass_kg_m2,
                "total_aboveground_live_biomass_kg_m2": canopy.live_foliar_biomass_kg_m2 + canopy.structural_biomass_kg_m2,
                "leaf_area_index_m2_m2": canopy.leaf_area_index,
                "cover_fraction": canopy.canopy_cover_fraction,
                "leaf_on_allocation_kg_m2": canopy.leaf_on_allocation_kg_m2,
                "leaf_off_transfer_kg_m2": canopy.leaf_off_litter_kg_m2
            },
            "consumers": {
                "growth_live_foliar_biomass_kg_m2": growth_state_after.live_biomass_kg_m2,
                "snow_canopy_cover_fraction": builder.snow_canopy_cover_fraction,
                "interception_leaf_area_index_m2_m2": builder.interception_inputs.leaf_area_index,
                "interception_canopy_cover_fraction": builder.interception_inputs.canopy_cover_fraction,
                "interception_live_biomass_kg_m2": builder.interception_inputs.interception_live_biomass_kg_m2,
                "interception_m": day_frame.interception_m,
                "et_leaf_area_index_m2_m2": day_frame.evapotranspiration_compute_inputs.leaf_area_index,
                "et_canopy_cover_fraction": day_frame.evapotranspiration_compute_inputs.canopy_cover_fraction,
                "runoff_m": day_frame.water.runoff_m,
                "erosion_canopy_cover_fraction": day_frame.erosion_daily_consumers.map(|consumers| consumers.canopy_cover_fraction),
                "erosion_interrill_cover_fraction": day_frame.erosion_daily_consumers.map(|consumers| consumers.interrill_cover_fraction),
                "erosion_rill_cover_fraction": day_frame.erosion_daily_consumers.map(|consumers| consumers.rill_cover_fraction),
                "frost_residue_depth_m": day_frame.frost_daily_consumers.map(|consumers| consumers.residue_depth_m)
            },
            "residue": canopy_research_residue_value(day_frame, &builder, decomposition_loss_kg_m2)
        });
        validate_canopy_research_trace_value(&value)?;
        let mut line = serde_json::to_string(&value).map_err(|error| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "canopy_research_trace",
                detail: format!("{SIMOUT_GUARD_ID} failed serializing canopy research trace: {error}"),
            }
        })?;
        line.push('\n');
        write_canopy_research_trace_line(&config.path, line.as_bytes())
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

    pub(crate) fn laned_shadow_lane_day_operands(
        &self,
        day_frame: &openwepp_hillslope_orchestrator::DirectDayFrame,
    ) -> Result<crate::hillslope::laned_shadow::LanedShadowLaneDayOperands, HillslopeCliError> {
        let _ = self.lane_authority(day_frame.lane_index)?;
        build_laned_shadow_lane_day_operands(
            day_frame.lane_index,
            day_frame.day_index,
            day_frame.wb14_hourly_rainfall_m,
            *day_frame
                .snow_coupling_downstream_operands
                .hourly_routed_melt_m,
            day_frame.evapotranspiration_compute_inputs.leaf_area_index,
            Some(day_frame.evapotranspiration_compute_inputs.canopy_height_m),
        )
    }

    fn validate_active_snow_forcing(
        lane_index: usize,
        hyetograph_rainfall_m: f64,
        runtime_swe_m: f64,
    ) -> Result<(), HillslopeCliError> {
        let _active_snow = DirectProductionSnowFrostAuthority::active_forcing(
            hyetograph_rainfall_m,
            runtime_swe_m,
        )?;
        let _ = lane_index;
        Ok(())
    }
}

fn canopy_research_residue_value(
    day_frame: &openwepp_hillslope_orchestrator::DirectDayFrame,
    builder: &NativeCanopyBuilderTrace,
    decomposition_loss_kg_m2: f64,
) -> serde_json::Value {
    let weight = day_frame
        .residue_partition_inputs
        .rescov_interrill_weight;
    serde_json::json!({
        "leaf_litter_input_kg_m2": builder.leaf_litter_input_kg_m2,
        "needle_litter_input_kg_m2": builder.needle_litter_input_kg_m2,
        "fine_woody_litter_input_kg_m2": builder.fine_woody_litter_input_kg_m2,
        "needle_litter_status": builder.needle_litter_status,
        "needle_litter_source_mode": builder.needle_litter_source_mode,
        "fine_woody_litter_status": builder.fine_woody_litter_status,
        "fine_woody_litter_source_mode": builder.fine_woody_litter_source_mode,
        "source_completeness": builder.litter_source_completeness,
        "total_litter_input_kg_m2": day_frame.decomposition.surface_litter_input_kg_m2,
        "surface_residue_before_kg_m2": day_frame.decomposition.surface_residue_seed_kg_m2,
        "surface_residue_after_kg_m2": day_frame.decomposition.surface_residue_kg_m2,
        "interrill_ground_residue_before_kg_m2":
            day_frame.decomposition_inputs.interrill_ground_seed_kg_m2,
        "interrill_ground_residue_after_kg_m2":
            day_frame.decomposition.interrill_ground_residue_kg_m2,
        "rill_ground_residue_before_kg_m2":
            day_frame.decomposition_inputs.rill_ground_seed_kg_m2,
        "rill_ground_residue_after_kg_m2":
            day_frame.decomposition.rill_ground_residue_kg_m2,
        "weighted_ground_residue_after_kg_m2":
            weight * day_frame.decomposition.interrill_ground_residue_kg_m2
                + (1.0 - weight) * day_frame.decomposition.rill_ground_residue_kg_m2,
        "residue_cover_factor_m2_kg": day_frame.decomposition.residue_cover_factor,
        "rescov_interrill_weight": weight,
        "interrill_cover_fraction": day_frame.residue_partition.interrill_cover_fraction,
        "rill_cover_fraction": day_frame.residue_partition.rill_cover_fraction,
        "composite_cover_fraction": day_frame.residue_partition.cover_fraction,
        "residue_depth_conversion_m_per_kg_m2":
            day_frame.decomposition.residue_depth_conversion_m_per_kg_m2,
        "decomposition_loss_kg_m2": decomposition_loss_kg_m2,
        "surface_decay_factor": day_frame.decomposition.surface_decay_factor,
        "residue_depth_m": day_frame.decomposition.residue_depth_m
    })
}

#[allow(clippy::items_after_statements, clippy::too_many_lines)]
fn validate_canopy_research_trace_value(
    value: &serde_json::Value,
) -> Result<(), HillslopeCliError> {
    const REQUIRED_STRINGS: &[&str] = &[
        "/schema",
        "/date",
        "/site_id",
        "/arm_id",
        "/residue/needle_litter_status",
        "/residue/needle_litter_source_mode",
        "/residue/fine_woody_litter_status",
        "/residue/fine_woody_litter_source_mode",
        "/residue/source_completeness",
    ];
    const REQUIRED_NULLABLE_NUMBERS: &[&str] = &[
        "/consumers/erosion_canopy_cover_fraction",
        "/consumers/frost_residue_depth_m",
        "/consumers/erosion_interrill_cover_fraction",
        "/consumers/erosion_rill_cover_fraction",
        "/residue/needle_litter_input_kg_m2",
        "/residue/fine_woody_litter_input_kg_m2",
    ];
    const REQUIRED_NUMBERS: &[&str] = &[
        "/year",
        "/day_of_year",
        "/day_index",
        "/lane_index",
        "/gsi/minimum_temperature_indicator",
        "/gsi/vapor_pressure_deficit_indicator",
        "/gsi/photoperiod_indicator",
        "/gsi/photoperiod_hours",
        "/gsi/instantaneous",
        "/gsi/gsi21",
        "/gsi/sample_count",
        "/canopy/structural_biomass_kg_m2",
        "/canopy/evergreen_foliar_biomass_kg_m2",
        "/canopy/deciduous_foliar_biomass_kg_m2",
        "/canopy/total_foliar_biomass_kg_m2",
        "/canopy/total_aboveground_live_biomass_kg_m2",
        "/canopy/leaf_area_index_m2_m2",
        "/canopy/cover_fraction",
        "/canopy/leaf_on_allocation_kg_m2",
        "/canopy/leaf_off_transfer_kg_m2",
        "/consumers/growth_live_foliar_biomass_kg_m2",
        "/consumers/snow_canopy_cover_fraction",
        "/consumers/interception_leaf_area_index_m2_m2",
        "/consumers/interception_canopy_cover_fraction",
        "/consumers/interception_live_biomass_kg_m2",
        "/consumers/interception_m",
        "/consumers/et_leaf_area_index_m2_m2",
        "/consumers/et_canopy_cover_fraction",
        "/consumers/runoff_m",
        "/residue/leaf_litter_input_kg_m2",
        "/residue/total_litter_input_kg_m2",
        "/residue/surface_residue_before_kg_m2",
        "/residue/surface_residue_after_kg_m2",
        "/residue/interrill_ground_residue_before_kg_m2",
        "/residue/interrill_ground_residue_after_kg_m2",
        "/residue/rill_ground_residue_before_kg_m2",
        "/residue/rill_ground_residue_after_kg_m2",
        "/residue/weighted_ground_residue_after_kg_m2",
        "/residue/residue_cover_factor_m2_kg",
        "/residue/rescov_interrill_weight",
        "/residue/interrill_cover_fraction",
        "/residue/rill_cover_fraction",
        "/residue/composite_cover_fraction",
        "/residue/residue_depth_conversion_m_per_kg_m2",
        "/residue/decomposition_loss_kg_m2",
        "/residue/surface_decay_factor",
        "/residue/residue_depth_m",
    ];
    for path in REQUIRED_STRINGS {
        if value
            .pointer(path)
            .and_then(serde_json::Value::as_str)
            .is_none_or(str::is_empty)
        {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "canopy_research_trace",
                detail: format!(
                    "{SIMOUT_GUARD_ID} canopy research trace required string {path} is missing or empty"
                ),
            });
        }
    }
    for path in REQUIRED_NUMBERS {
        if !value
            .pointer(path)
            .and_then(serde_json::Value::as_f64)
            .is_some_and(f64::is_finite)
        {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "canopy_research_trace",
                detail: format!(
                    "{SIMOUT_GUARD_ID} canopy research trace required number {path} is missing or nonfinite"
                ),
            });
        }
    }
    for (status_path, mode_path) in [
        (
            "/residue/needle_litter_status",
            "/residue/needle_litter_source_mode",
        ),
        (
            "/residue/fine_woody_litter_status",
            "/residue/fine_woody_litter_source_mode",
        ),
    ] {
        let status = value
            .pointer(status_path)
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        let mode = value
            .pointer(mode_path)
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        let valid = matches!(
            (status, mode),
            ("complete", "prescribed_scenario" | "measured_daily")
                | ("not_represented" | "not_applicable", "none")
        );
        if !valid {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "canopy_research_trace",
                detail: format!(
                    "{SIMOUT_GUARD_ID} invalid litter authority disclosure: {status_path}={status}, {mode_path}={mode}"
                ),
            });
        }
    }
    let expected_completeness = if [
        "/residue/needle_litter_status",
        "/residue/fine_woody_litter_status",
    ]
    .iter()
    .any(|path| value.pointer(path).and_then(serde_json::Value::as_str) == Some("not_represented"))
    {
        "incomplete"
    } else {
        "complete"
    };
    if value
        .pointer("/residue/source_completeness")
        .and_then(serde_json::Value::as_str)
        != Some(expected_completeness)
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "canopy_research_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} litter source completeness does not match tissue status"
            ),
        });
    }
    for path in REQUIRED_NULLABLE_NUMBERS {
        match value.pointer(path) {
            Some(serde_json::Value::Null) => {}
            Some(serde_json::Value::Number(number))
                if number.as_f64().is_some_and(f64::is_finite) => {}
            _ => {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "canopy_research_trace",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} canopy research trace nullable number {path} is missing or invalid"
                    ),
                });
            }
        }
    }
    let leaf = value["residue"]["leaf_litter_input_kg_m2"]
        .as_f64()
        .unwrap_or(f64::NAN);
    let tissue_mass = |name: &str| {
        let status = value["residue"][format!("{name}_litter_status")]
            .as_str()
            .unwrap_or_default();
        match (
            status,
            &value["residue"][format!("{name}_litter_input_kg_m2")],
        ) {
            ("complete", serde_json::Value::Number(number)) => number.as_f64(),
            ("not_represented" | "not_applicable", serde_json::Value::Null) => Some(0.0),
            _ => None,
        }
    };
    let needle = tissue_mass("needle").unwrap_or(f64::NAN);
    let fine_woody = tissue_mass("fine_woody").unwrap_or(f64::NAN);
    let total = value["residue"]["total_litter_input_kg_m2"]
        .as_f64()
        .unwrap_or(f64::NAN);
    let reconstructed = leaf + needle + fine_woody;
    let tolerance = 32.0 * f64::EPSILON * total.abs().max(1.0);
    if leaf < 0.0
        || needle < 0.0
        || fine_woody < 0.0
        || total < 0.0
        || (reconstructed - total).abs() > tolerance
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "canopy_research_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} litter source closure failed: leaf={leaf} needle={needle} fine_woody={fine_woody} total={total}"
            ),
        });
    }
    fn visit(value: &serde_json::Value, path: &str) -> Result<(), HillslopeCliError> {
        match value {
            serde_json::Value::Null
                if matches!(
                    path,
                    "/consumers/erosion_canopy_cover_fraction"
                        | "/consumers/erosion_interrill_cover_fraction"
                        | "/consumers/erosion_rill_cover_fraction"
                        | "/consumers/frost_residue_depth_m"
                        | "/residue/needle_litter_input_kg_m2"
                        | "/residue/fine_woody_litter_input_kg_m2"
                ) =>
            {
                Ok(())
            }
            serde_json::Value::Null => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "canopy_research_trace",
                detail: format!(
                    "{SIMOUT_GUARD_ID} canopy research trace required value {path} is null or nonfinite"
                ),
            }),
            serde_json::Value::Number(number)
                if number.as_f64().is_some_and(f64::is_finite) =>
            {
                Ok(())
            }
            serde_json::Value::Number(_) => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "canopy_research_trace",
                detail: format!(
                    "{SIMOUT_GUARD_ID} canopy research trace numeric value {path} is nonfinite"
                ),
            }),
            serde_json::Value::Array(values) => values
                .iter()
                .enumerate()
                .try_for_each(|(index, child)| visit(child, &format!("{path}/{index}"))),
            serde_json::Value::Object(values) => values
                .iter()
                .try_for_each(|(key, child)| visit(child, &format!("{path}/{key}"))),
            serde_json::Value::String(_) | serde_json::Value::Bool(_) => Ok(()),
        }
    }
    visit(value, "")
}

fn write_canopy_research_trace_line(
    path: &std::ffi::OsStr,
    line: &[u8],
) -> Result<(), HillslopeCliError> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "canopy_research_trace",
                detail: format!(
                    "{SIMOUT_GUARD_ID} failed opening canopy research trace {}: {error}",
                    std::path::PathBuf::from(path).display()
                ),
            })?;
    std::io::Write::write_all(&mut file, line).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "canopy_research_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed writing canopy research trace {}: {error}",
                std::path::PathBuf::from(path).display()
            ),
        }
    })
}

fn maybe_write_r7h_direct_production_snow_trace(
    request: &DirectSnowDiagnosticCaptureRequest,
    context: &DirectSnowTraceRowContext<'_>,
) -> Result<(), HillslopeCliError> {
    let Some(verbose_diagnostics) =
        selected_snow_verbose_diagnostics(request, context.snow_liquid)?
    else {
        return Ok(());
    };
    let path = request.selected_path.as_ref().ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_trace",
            detail: format!("{SIMOUT_GUARD_ID} selected snow trace path was lost"),
        }
    })?;
    if let Some(persistent) = context.stage3_persistent {
        if persistent.state.schema_version == 2 {
            validate_snow_terminal_event_trace_consumer(persistent).map_err(|detail| {
                HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_production_snow_terminal_event_trace",
                    detail: format!("{SIMOUT_GUARD_ID} {detail}"),
                }
            })?;
        }
    }

    let line = r7h_direct_production_snow_trace_line(context, verbose_diagnostics);
    if context
        .stage3_persistent
        .is_some_and(|persistent| persistent.state.schema_version == 2)
    {
        validate_snow_terminal_event_trace_row(&line).map_err(|detail| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_terminal_event_trace",
                detail: format!("{SIMOUT_GUARD_ID} {detail}"),
            }
        })?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed opening direct production snow trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        })?;
    std::io::Write::write_all(&mut file, line.as_bytes()).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed writing direct production snow trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        }
    })
}

#[allow(clippy::too_many_lines)]
fn r7h_direct_production_snow_trace_line(
    context: &DirectSnowTraceRowContext<'_>,
    verbose_diagnostics: &openwepp_hillslope_orchestrator::DirectSnowVerboseDiagnostics,
) -> String {
    let DirectSnowTraceRowContext {
        day_index,
        lane_index,
        hyetograph_rainfall_m,
        snow_lane_state,
        snow_melt_model,
        snow_phase_model,
        snow_liquid,
        stage3_evaluation,
        stage3_reconciliation,
        stage3_persistent,
    } = *context;
    let layer = direct_snow_trace_layer_diagnostics(snow_lane_state, snow_liquid);
    let thermal = direct_snow_trace_thermal_diagnostics(&verbose_diagnostics.stage3);
    let layers_before = direct_snow_trace_layers(&snow_lane_state.layers);
    let layers_after = direct_snow_trace_layers(&snow_liquid.snow_layers_after);
    let schema = direct_snow_trace_schema(
        stage3_evaluation,
        stage3_reconciliation,
        stage3_persistent,
    );
    let line = format!(
        "{{\"schema\":\"{schema}\",\
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
\"snow_layer_density_gradient_after_kg_m3\":{},\
\"snow_layer_minimum_temperature_after_c\":{},\
\"snow_layer_maximum_temperature_after_c\":{},\
\"snow_layers_before\":{},\
\"snow_layers_after\":{}",
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
        direct_production_trace_number(snow_liquid.solid_to_liquid_ledger().raw_signed_melt_m),
        direct_production_trace_number(snow_liquid.solid_to_liquid_ledger().snowpack_swe_loss_m),
        direct_production_trace_number(snow_liquid.accumulation_m),
        direct_production_trace_number(snow_liquid.sublimation_m),
        direct_production_trace_number(snow_liquid.solid_to_liquid_ledger().liquid_handoff_m),
        direct_production_trace_number(snow_liquid.rain_retained_m),
        direct_production_trace_number(snow_liquid.solid_to_liquid_ledger().rain_released_m),
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
        direct_production_trace_number(layer.minimum_temperature_after_c),
        direct_production_trace_number(layer.maximum_temperature_after_c),
        layers_before,
        layers_after,
    );
    format!(
        "{line},{}\n",
        direct_snow_trace_diagnostic_suffix(
            snow_liquid,
            verbose_diagnostics,
            &thermal,
            stage3_evaluation,
            stage3_reconciliation,
            stage3_persistent,
        )
    )
}

fn direct_snow_trace_density_process_fields(
    diagnostics: &openwepp_hillslope_orchestrator::SnowDensityProcessDiagnostics,
) -> String {
    format!(
        "\"density_process_applicable\":{},\
\"density_process_fresh_snow_density_available\":{},\
\"density_process_initial_density_kg_m3\":{},\
\"density_process_initial_snow_mass_kg_m2\":{},\
\"density_process_liquid_for_compaction_mass_kg_m2\":{},\
\"density_process_compaction_temperature_c\":{},\
\"density_process_snow_input_mass_kg_m2\":{},\
\"density_process_snow_input_depth_m\":{},\
\"density_process_fresh_snow_density_kg_m3\":{},\
\"density_process_fresh_snow_mixing_delta_kg_m3\":{},\
\"density_process_wet_compaction_delta_kg_m3\":{},\
\"density_process_destructive_metamorphism_delta_kg_m3\":{},\
\"density_process_overburden_compaction_delta_kg_m3\":{},\
\"density_process_structural_projection_delta_kg_m3\":{},\
\"density_process_climate_fallback_used\":{},\
\"density_process_climate_fallback_delta_kg_m3\":{},\
\"density_process_internal_cap_delta_kg_m3\":{},\
\"density_process_runtime_cap_delta_kg_m3\":{},\
\"density_process_downstream_stage3_delta_kg_m3\":{},\
\"density_process_final_density_kg_m3\":{},\
\"density_process_closure_residual_kg_m3\":{}",
        diagnostics.applicable,
        diagnostics.fresh_snow_density_available,
        direct_production_trace_number(diagnostics.initial_density_kg_m3),
        direct_production_trace_number(diagnostics.initial_snow_mass_kg_m2),
        direct_production_trace_number(diagnostics.liquid_for_compaction_mass_kg_m2),
        direct_production_trace_number(diagnostics.compaction_temperature_c),
        direct_production_trace_number(diagnostics.snow_input_mass_kg_m2),
        direct_production_trace_number(diagnostics.snow_input_depth_m),
        direct_production_trace_number(diagnostics.fresh_snow_density_kg_m3),
        direct_production_trace_number(diagnostics.fresh_snow_mixing_delta_kg_m3),
        direct_production_trace_number(diagnostics.wet_compaction_delta_kg_m3),
        direct_production_trace_number(diagnostics.destructive_metamorphism_delta_kg_m3),
        direct_production_trace_number(diagnostics.overburden_compaction_delta_kg_m3),
        direct_production_trace_number(diagnostics.structural_projection_delta_kg_m3),
        diagnostics.climate_fallback_used,
        direct_production_trace_number(diagnostics.climate_fallback_delta_kg_m3),
        direct_production_trace_number(diagnostics.internal_cap_delta_kg_m3),
        direct_production_trace_number(diagnostics.runtime_cap_delta_kg_m3),
        direct_production_trace_number(diagnostics.downstream_stage3_delta_kg_m3),
        direct_production_trace_number(diagnostics.final_density_kg_m3),
        direct_production_trace_number(diagnostics.closure_residual_kg_m3),
    )
}

fn direct_snow_trace_layers(
    layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState],
) -> String {
    let rows = layers
        .iter()
        .map(|layer| {
            format!(
                "{{\"mass_swe_m\":{},\"thickness_m\":{},\"density_kg_m3\":{},\"settle_day_count\":{},\"temperature_c\":{},\"liquid_water_m\":{},\"cold_content_j_m2\":{},\"refrozen_liquid_m\":{}}}",
                direct_production_trace_number(layer.mass_swe_m),
                direct_production_trace_number(layer.thickness_m),
                direct_production_trace_number(layer.density_kg_m3),
                direct_production_trace_number(layer.settle_day_count),
                direct_production_trace_number(layer.temperature_c),
                direct_production_trace_number(layer.liquid_water_m),
                direct_production_trace_number(layer.cold_content_j_m2),
                direct_production_trace_number(layer.refrozen_liquid_m),
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{rows}]")
}

struct DirectSnowStage3HourlyTraceFields {
    shortwave: String,
    longwave: String,
    vapor_mass: String,
    latent_heat: String,
    latent_flux: String,
    active_mass: String,
    active_depth: String,
    active_temperature: String,
    active_cold_content: String,
    lower_present_fraction: String,
    lower_mass: String,
    lower_depth: String,
    lower_temperature: String,
    lower_cold_content: String,
    shadow_complete_energy: String,
    shadow_cold_energy_change: String,
    shadow_melt: String,
    shadow_terminal_energy: String,
    shadow_energy_residual: String,
}

fn direct_snow_trace_stage3_hourly_fields(
    diagnostics: &openwepp_hillslope_orchestrator::DirectSnowStage3Diagnostics,
) -> DirectSnowStage3HourlyTraceFields {
    DirectSnowStage3HourlyTraceFields {
        shortwave: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.net_shortwave_w_m2
        }),
        longwave: direct_snow_trace_hourly_values(diagnostics, |hour| hour.net_longwave_w_m2),
        vapor_mass: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.vapor_mass_exchange_kg_m2
        }),
        latent_heat: direct_snow_trace_hourly_values(diagnostics, |hour| hour.latent_heat_j_kg),
        latent_flux: direct_snow_trace_hourly_values(diagnostics, |hour| hour.latent_flux_w_m2),
        active_mass: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.active_layer_mass_kg_m2
        }),
        active_depth: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.active_layer_depth_m
        }),
        active_temperature: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.active_layer_temperature_c
        }),
        active_cold_content: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.active_layer_cold_content_j_m2
        }),
        lower_present_fraction: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.lower_layer_present_fraction
        }),
        lower_mass: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.lower_layer_mass_kg_m2
        }),
        lower_depth: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.lower_layer_depth_m
        }),
        lower_temperature: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.lower_layer_temperature_c
        }),
        lower_cold_content: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.lower_layer_cold_content_j_m2
        }),
        shadow_complete_energy: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.shadow_complete_energy_j_m2
        }),
        shadow_cold_energy_change: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.shadow_cold_energy_change_j_m2
        }),
        shadow_melt: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.shadow_melt_kg_m2
        }),
        shadow_terminal_energy: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.shadow_unallocated_after_exhaustion_j_m2
        }),
        shadow_energy_residual: direct_snow_trace_hourly_values(diagnostics, |hour| {
            hour.shadow_energy_closure_residual_j_m2
        }),
    }
}

#[allow(clippy::too_many_lines)]
fn direct_snow_trace_stage3_fields(
    outcome: &openwepp_hillslope_orchestrator::DirectSnowStage3Outcome,
    ledger: &openwepp_hillslope_orchestrator::DirectSnowLiquidDispositionLedger,
    diagnostics: &openwepp_hillslope_orchestrator::DirectSnowStage3Diagnostics,
) -> String {
    let hourly = direct_snow_trace_stage3_hourly_fields(diagnostics);
    format!(
        "\
\"stage3_energy_enabled\":{},\
\"stage3_incoming_liquid_m\":{},\
\"stage3_routed_liquid_m\":{},\
\"stage3_retained_liquid_delta_m\":{},\
\"stage3_liquid_closure_residual_m\":{},\
\"stage3_cold_content_before_j_m2\":{},\
\"stage3_cold_content_after_j_m2\":{},\
\"stage3_energy_closure_residual_j_m2\":{},\
\"stage3_surface_energy_j_m2\":{},\
\"stage3_conduction_energy_j_m2\":{},\
\"stage3_shortwave_energy_j_m2\":{},\
\"stage3_longwave_energy_j_m2\":{},\
\"stage3_latent_energy_j_m2\":{},\
\"stage3_vapor_mass_exchange_kg_m2\":{},\
\"stage3_latent_mass_energy_j_m2\":{},\
\"stage3_hourly_net_shortwave_w_m2\":{},\
\"stage3_hourly_net_longwave_w_m2\":{},\
\"stage3_hourly_vapor_mass_exchange_kg_m2\":{},\
\"stage3_hourly_latent_heat_j_kg\":{},\
\"stage3_hourly_latent_flux_w_m2\":{},\
\"stage3_hourly_active_mass_kg_m2\":{},\
\"stage3_hourly_active_depth_m\":{},\
\"stage3_hourly_active_temperature_c\":{},\
\"stage3_hourly_active_cold_content_j_m2\":{},\
\"stage3_hourly_lower_present_fraction\":{},\
\"stage3_hourly_lower_mass_kg_m2\":{},\
\"stage3_hourly_lower_depth_m\":{},\
\"stage3_hourly_lower_temperature_c\":{},\
\"stage3_hourly_lower_cold_content_j_m2\":{},\
\"stage3_shadow_hourly_complete_energy_j_m2\":{},\
\"stage3_shadow_hourly_cold_energy_change_j_m2\":{},\
\"stage3_shadow_hourly_melt_kg_m2\":{},\
\"stage3_shadow_hourly_terminal_energy_j_m2\":{},\
\"stage3_shadow_hourly_energy_residual_j_m2\":{},\
\"stage3_shadow_complete_energy_j_m2\":{},\
\"stage3_shadow_cold_energy_change_j_m2\":{},\
\"stage3_shadow_excess_energy_j_m2\":{},\
\"stage3_shadow_sublimation_kg_m2\":{},\
\"stage3_shadow_melt_kg_m2\":{},\
\"stage3_shadow_unallocated_after_exhaustion_j_m2\":{},\
\"stage3_shadow_maximum_energy_closure_residual_j_m2\":{},\
\"stage3_latent_refreeze_energy_j_m2\":{},\
\"stage3_cold_content_export_j_m2\":{},\
\"stage3_mass_latent_identity_residual_j_m2\":{},\
\"stage3_unused_positive_energy_j_m2\":{},\
\"stage3_thermal_domain_suspended_seconds\":{},\
\"stage3_minimum_unresolved_thermal_mass_kg_m2\":{},\
\"stage3_lower_thermal_volume_collapsed_seconds\":{},\
\"stage3_minimum_collapsed_lower_mass_kg_m2\":{},\
\"stage3_refrozen_liquid_m\":{}",
        outcome.enabled,
        direct_production_trace_number(ledger.incoming_liquid_m),
        direct_production_trace_number(ledger.routed_liquid_m),
        direct_production_trace_number(ledger.retained_liquid_delta_m),
        direct_production_trace_number(ledger.liquid_closure_residual_m),
        direct_production_trace_number(diagnostics.cold_content_before_j_m2),
        direct_production_trace_number(diagnostics.cold_content_after_j_m2),
        direct_production_trace_number(diagnostics.energy_closure_residual_j_m2),
        direct_production_trace_number(diagnostics.surface_energy_j_m2),
        direct_production_trace_number(diagnostics.conduction_energy_j_m2),
        direct_production_trace_number(diagnostics.shortwave_energy_j_m2),
        direct_production_trace_number(diagnostics.longwave_energy_j_m2),
        direct_production_trace_number(diagnostics.latent_energy_j_m2),
        direct_production_trace_number(diagnostics.vapor_mass_exchange_kg_m2),
        direct_production_trace_number(diagnostics.latent_mass_energy_j_m2),
        hourly.shortwave,
        hourly.longwave,
        hourly.vapor_mass,
        hourly.latent_heat,
        hourly.latent_flux,
        hourly.active_mass,
        hourly.active_depth,
        hourly.active_temperature,
        hourly.active_cold_content,
        hourly.lower_present_fraction,
        hourly.lower_mass,
        hourly.lower_depth,
        hourly.lower_temperature,
        hourly.lower_cold_content,
        hourly.shadow_complete_energy,
        hourly.shadow_cold_energy_change,
        hourly.shadow_melt,
        hourly.shadow_terminal_energy,
        hourly.shadow_energy_residual,
        direct_production_trace_number(diagnostics.shadow_complete_energy_j_m2),
        direct_production_trace_number(diagnostics.shadow_cold_energy_change_j_m2),
        direct_production_trace_number(diagnostics.shadow_excess_energy_j_m2),
        direct_production_trace_number(diagnostics.shadow_sublimation_kg_m2),
        direct_production_trace_number(diagnostics.shadow_melt_kg_m2),
        direct_production_trace_number(
            diagnostics.shadow_unallocated_after_exhaustion_j_m2
        ),
        direct_production_trace_number(
            diagnostics.shadow_maximum_energy_closure_residual_j_m2
        ),
        direct_production_trace_number(diagnostics.latent_refreeze_energy_j_m2),
        direct_production_trace_number(diagnostics.cold_content_export_j_m2),
        direct_production_trace_number(
            diagnostics.mass_latent_identity_residual_j_m2
        ),
        direct_production_trace_number(diagnostics.unused_positive_energy_j_m2),
        direct_production_trace_number(diagnostics.thermal_domain_suspended_seconds),
        direct_production_trace_number(diagnostics.minimum_unresolved_thermal_mass_kg_m2),
        direct_production_trace_number(diagnostics.lower_thermal_volume_collapsed_seconds),
        direct_production_trace_number(diagnostics.minimum_collapsed_lower_mass_kg_m2),
        direct_production_trace_number(ledger.refrozen_liquid_m),
    )
}

fn direct_snow_trace_hourly_values(
    diagnostics: &openwepp_hillslope_orchestrator::DirectSnowStage3Diagnostics,
    value: impl Fn(
        &openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyHourDiagnostics,
    ) -> f64,
) -> String {
    let values = diagnostics
        .hourly_surface_energy
        .iter()
        .map(|hour| direct_production_trace_number(value(hour)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{values}]")
}

#[cfg(test)]
mod stage3_trace_field_tests {
    use super::*;

    #[test]
    fn formatter_preserves_exact_liquid_and_hourly_thermal_operands() {
        let mut diagnostics =
            openwepp_hillslope_orchestrator::DirectSnowStage3Diagnostics::disabled();
        let outcome = openwepp_hillslope_orchestrator::DirectSnowStage3Outcome {
            enabled: true,
            ..openwepp_hillslope_orchestrator::DirectSnowStage3Outcome::default()
        };
        let ledger = openwepp_hillslope_orchestrator::DirectSnowLiquidDispositionLedger {
            incoming_liquid_m: 0.021,
            routed_liquid_m: 0.009,
            retained_liquid_delta_m: 0.004,
            refrozen_liquid_m: 0.006,
            liquid_closure_residual_m: 0.002,
        };
        diagnostics.hourly_surface_energy[0].active_layer_mass_kg_m2 = 41.0;
        diagnostics.hourly_surface_energy[0].active_layer_depth_m = 0.22;
        diagnostics.hourly_surface_energy[0].active_layer_temperature_c = -1.5;
        diagnostics.hourly_surface_energy[0].active_layer_cold_content_j_m2 = 12_500.0;
        diagnostics.hourly_surface_energy[0].lower_layer_present_fraction = 0.75;
        diagnostics.hourly_surface_energy[0].lower_layer_mass_kg_m2 = 64.0;
        diagnostics.hourly_surface_energy[0].lower_layer_depth_m = 0.31;
        diagnostics.hourly_surface_energy[0].lower_layer_temperature_c = -2.25;
        diagnostics.hourly_surface_energy[0].lower_layer_cold_content_j_m2 = 23_500.0;

        let json = format!(
            "{{{}}}",
            direct_snow_trace_stage3_fields(&outcome, &ledger, &diagnostics)
        );
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("Stage-3 suffix must be valid JSON");
        assert_eq!(value["stage3_incoming_liquid_m"], 0.021);
        assert_eq!(value["stage3_routed_liquid_m"], 0.009);
        assert_eq!(value["stage3_retained_liquid_delta_m"], 0.004);
        assert_eq!(value["stage3_refrozen_liquid_m"], 0.006);
        assert_eq!(value["stage3_liquid_closure_residual_m"], 0.002);
        assert_eq!(value["stage3_hourly_active_mass_kg_m2"][0], 41.0);
        assert_eq!(value["stage3_hourly_active_depth_m"][0], 0.22);
        assert_eq!(value["stage3_hourly_active_temperature_c"][0], -1.5);
        assert_eq!(
            value["stage3_hourly_active_cold_content_j_m2"][0],
            12_500.0
        );
        assert_eq!(value["stage3_hourly_lower_present_fraction"][0], 0.75);
        assert_eq!(value["stage3_hourly_lower_mass_kg_m2"][0], 64.0);
        assert_eq!(value["stage3_hourly_lower_depth_m"][0], 0.31);
        assert_eq!(value["stage3_hourly_lower_temperature_c"][0], -2.25);
        assert_eq!(
            value["stage3_hourly_lower_cold_content_j_m2"][0],
            23_500.0
        );
    }

    #[test]
    #[allow(clippy::float_cmp, clippy::too_many_lines)]
    fn schema_v5_consumer_reconstructs_shadow_operands_and_rejects_production_aliases() {
        use openwepp_hillslope_orchestrator::{
            DirectSnowStage3EvaluationDiagnostics, DirectSnowStage3EvaluationHourDiagnostics,
            SnowStage3EvaluationOperator,
        };

        let mut hourly = [DirectSnowStage3EvaluationHourDiagnostics::zero(); 24];
        hourly[0] = DirectSnowStage3EvaluationHourDiagnostics {
            shortwave_energy_j_m2: 11.0,
            longwave_energy_j_m2: 12.0,
            sensible_flux_w_m2: 0.013,
            latent_flux_w_m2: 0.014,
            advected_flux_w_m2: 0.015,
            internal_active_lower_conduction_j_m2: 16.0,
            cold_content_export_j_m2: 17.0,
            requested_seconds: 100.0,
            evaluated_seconds: 80.0,
            ..DirectSnowStage3EvaluationHourDiagnostics::zero()
        };
        let evaluation = DirectSnowStage3EvaluationDiagnostics {
            operator: SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
            source_snapshot_id: "post_coe_daily_initial_snapshot_v1",
            support_id: "stage3_daily_24_hour_support_v1",
            cadence_id: "stage3_fixed_hourly_immutable_snapshot_v1",
            carrier_id: "stage3_carrier_pair_v1",
            coverage_id: "evaluated_seconds_over_requested_seconds_v1",
            claim_class: "carrier_component_comparison",
            unresolved_boundaries_id: "snow_ground_cross_day_terminal_recipient_unresolved_v1",
            pairing_id: Some("stage3_carrier_pair_v1"),
            arm_ids: ["stage3_surface_energy_v1", "stage3_complete_carrier_v1"],
            arm_count: 2,
            source_fingerprint: 0x11,
            forcing_fingerprint: 0x22,
            geometry_fingerprint: 0x33,
            non_formulation_fingerprint: 0x44,
            surface_arm_non_formulation_fingerprint: 0x44,
            complete_arm_non_formulation_fingerprint: 0x44,
            requested_seconds: 100.0,
            evaluated_seconds: 80.0,
            coverage_fraction: 0.8,
            surface_arm_applicable: true,
            surface_arm_shortwave_j_m2: 1.0,
            surface_arm_longwave_j_m2: 2.0,
            surface_arm_latent_j_m2: 3.0,
            surface_arm_sensible_applicable: false,
            surface_arm_advected_applicable: false,
            surface_arm_internal_conduction_applicable: false,
            surface_arm_total_j_m2: 6.0,
            complete_arm_shortwave_j_m2: 1.0,
            complete_arm_longwave_j_m2: 2.0,
            complete_arm_sensible_j_m2: 4.0,
            complete_arm_latent_j_m2: 5.0,
            complete_arm_advected_j_m2: 6.0,
            complete_arm_internal_active_lower_conduction_j_m2: 0.0,
            complete_arm_applicable: true,
            complete_arm_internal_conduction_applicable: false,
            complete_arm_vapor_mass_exchange_kg_m2: -0.25,
            complete_arm_cold_content_export_j_m2: 0.0,
            complete_arm_cold_content_export_applicable: false,
            complete_arm_available_ice_kg_m2: 0.0,
            complete_arm_available_ice_applicable: false,
            complete_arm_total_j_m2: 18.0,
            complete_arm_sequential_ledger_applicable: false,
            complete_arm_cold_energy_change_j_m2: 0.0,
            complete_arm_excess_energy_j_m2: 0.0,
            complete_arm_sublimation_kg_m2: 0.0,
            complete_arm_melt_kg_m2: 0.0,
            complete_arm_terminal_unallocated_j_m2: 0.0,
            complete_arm_terminal_unallocated_applicable: false,
            complete_arm_component_residual_j_m2: 0.0,
            complete_arm_maximum_thermodynamic_residual_j_m2: 0.0,
            hourly,
        };

        let fields = direct_snow_trace_stage3_evaluation_fields(&evaluation);
        let row = format!("{{\"schema\":\"openwepp-r7h-direct-production-snow-trace-v5\",{fields}}}\n");
        let path = std::env::temp_dir().join(format!(
            "openwepp-stage3-v5-consumer-{}.jsonl",
            std::process::id()
        ));
        std::fs::write(&path, row).expect("write schema-v5 JSONL row");
        let observed = std::fs::read_to_string(&path).expect("read schema-v5 JSONL row");
        std::fs::remove_file(&path).expect("remove schema-v5 JSONL test row");
        let value: serde_json::Value =
            serde_json::from_str(observed.trim()).expect("schema-v5 row is valid JSON");
        assert_eq!(
            value["schema"],
            "openwepp-r7h-direct-production-snow-trace-v5"
        );
        let surface = value["stage3_evaluation_surface_arm_shortwave_j_m2"]
            .as_f64()
            .expect("surface shortwave")
            + value["stage3_evaluation_surface_arm_longwave_j_m2"]
                .as_f64()
                .expect("surface longwave")
            + value["stage3_evaluation_surface_arm_latent_j_m2"]
                .as_f64()
                .expect("surface latent");
        assert_eq!(surface, 6.0);
        let complete = [
            "stage3_evaluation_complete_arm_shortwave_j_m2",
            "stage3_evaluation_complete_arm_longwave_j_m2",
            "stage3_evaluation_complete_arm_sensible_j_m2",
            "stage3_evaluation_complete_arm_latent_j_m2",
            "stage3_evaluation_complete_arm_advected_j_m2",
            "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2",
        ]
        .into_iter()
        .map(|field| value[field].as_f64().expect("complete component"))
        .sum::<f64>();
        assert_eq!(complete, 18.0);
        assert_eq!(value["stage3_evaluation_hourly_shortwave_j_m2"][0], 11.0);
        assert_eq!(
            value["stage3_evaluation_hourly_cold_content_export_j_m2"][0],
            17.0
        );
        assert_ne!(
            value["stage3_evaluation_surface_arm_shortwave_j_m2"],
            serde_json::Value::from(999.0)
        );
        assert_ne!(
            value["stage3_evaluation_hourly_shortwave_j_m2"][0],
            serde_json::Value::from(777.0)
        );
        assert_eq!(
            value["stage3_evaluation_non_formulation_fingerprint_fnv1a64"],
            "0000000000000044"
        );
        assert_eq!(
            direct_snow_trace_schema(None, None, None),
            "openwepp-r7h-direct-production-snow-trace-v4"
        );
        assert_eq!(
            direct_snow_trace_schema(Some(&evaluation), None, None),
            "openwepp-r7h-direct-production-snow-trace-v5"
        );
    }
}

#[derive(Default)]
struct DirectSnowTraceThermalDiagnostics {
    maximum_active_depth_m: f64,
    maximum_lower_depth_m: f64,
    maximum_active_mass_kg_m2: f64,
    maximum_lower_mass_kg_m2: f64,
    maximum_abs_g0_w_m2: f64,
    peak_g0_w_m2: f64,
    peak_g0_requested_w_m2: f64,
    peak_g0_rejected_w_m2: f64,
    peak_g0_pressure_pa: f64,
    peak_g0_active_temperature_c: f64,
    peak_g0_lower_temperature_c: f64,
    peak_g0_active_depth_m: f64,
    peak_g0_lower_depth_m: f64,
    peak_g0_active_conductivity_w_m_k: f64,
    peak_g0_lower_conductivity_w_m_k: f64,
    peak_g0_active_resistance_m2_k_w: f64,
    peak_g0_lower_resistance_m2_k_w: f64,
    minimum_substep_seconds: f64,
    maximum_active_energy_residual_j_m2: f64,
    maximum_lower_energy_residual_j_m2: f64,
    maximum_conduction_cancellation_residual_j_m2: f64,
}

fn direct_snow_trace_thermal_fields(thermal: &DirectSnowTraceThermalDiagnostics) -> String {
    format!(
        "\"stage3_maximum_active_depth_m\":{},\
\"stage3_maximum_lower_depth_m\":{},\
\"stage3_maximum_active_mass_kg_m2\":{},\
\"stage3_maximum_lower_mass_kg_m2\":{},\
\"stage3_maximum_abs_g0_w_m2\":{},\
\"stage3_peak_g0_w_m2\":{},\
\"stage3_peak_g0_requested_w_m2\":{},\
\"stage3_peak_g0_rejected_w_m2\":{},\
\"stage3_peak_g0_pressure_pa\":{},\
\"stage3_peak_g0_active_temperature_c\":{},\
\"stage3_peak_g0_lower_temperature_c\":{},\
\"stage3_peak_g0_active_depth_m\":{},\
\"stage3_peak_g0_lower_depth_m\":{},\
\"stage3_peak_g0_active_conductivity_w_m_k\":{},\
\"stage3_peak_g0_lower_conductivity_w_m_k\":{},\
\"stage3_peak_g0_active_resistance_m2_k_w\":{},\
\"stage3_peak_g0_lower_resistance_m2_k_w\":{},\
\"stage3_minimum_substep_seconds\":{},\
\"stage3_maximum_active_energy_residual_j_m2\":{},\
\"stage3_maximum_lower_energy_residual_j_m2\":{},\
\"stage3_maximum_conduction_cancellation_residual_j_m2\":{}",
        direct_production_trace_number(thermal.maximum_active_depth_m),
        direct_production_trace_number(thermal.maximum_lower_depth_m),
        direct_production_trace_number(thermal.maximum_active_mass_kg_m2),
        direct_production_trace_number(thermal.maximum_lower_mass_kg_m2),
        direct_production_trace_number(thermal.maximum_abs_g0_w_m2),
        direct_production_trace_number(thermal.peak_g0_w_m2),
        direct_production_trace_number(thermal.peak_g0_requested_w_m2),
        direct_production_trace_number(thermal.peak_g0_rejected_w_m2),
        direct_production_trace_number(thermal.peak_g0_pressure_pa),
        direct_production_trace_number(thermal.peak_g0_active_temperature_c),
        direct_production_trace_number(thermal.peak_g0_lower_temperature_c),
        direct_production_trace_number(thermal.peak_g0_active_depth_m),
        direct_production_trace_number(thermal.peak_g0_lower_depth_m),
        direct_production_trace_number(thermal.peak_g0_active_conductivity_w_m_k),
        direct_production_trace_number(thermal.peak_g0_lower_conductivity_w_m_k),
        direct_production_trace_number(thermal.peak_g0_active_resistance_m2_k_w),
        direct_production_trace_number(thermal.peak_g0_lower_resistance_m2_k_w),
        direct_production_trace_number(thermal.minimum_substep_seconds),
        direct_production_trace_number(thermal.maximum_active_energy_residual_j_m2),
        direct_production_trace_number(thermal.maximum_lower_energy_residual_j_m2),
        direct_production_trace_number(
            thermal.maximum_conduction_cancellation_residual_j_m2
        ),
    )
}

fn direct_snow_trace_thermal_diagnostics(
    stage3_diagnostics: &openwepp_hillslope_orchestrator::DirectSnowStage3Diagnostics,
) -> DirectSnowTraceThermalDiagnostics {
    let mut diagnostics = DirectSnowTraceThermalDiagnostics::default();
    for hour in stage3_diagnostics.hourly_surface_energy {
        diagnostics.maximum_active_depth_m =
            diagnostics.maximum_active_depth_m.max(hour.active_layer_depth_m);
        diagnostics.maximum_lower_depth_m =
            diagnostics.maximum_lower_depth_m.max(hour.lower_layer_depth_m);
        diagnostics.maximum_active_mass_kg_m2 = diagnostics
            .maximum_active_mass_kg_m2
            .max(hour.active_layer_mass_kg_m2);
        diagnostics.maximum_lower_mass_kg_m2 = diagnostics
            .maximum_lower_mass_kg_m2
            .max(hour.lower_layer_mass_kg_m2);
        diagnostics.maximum_abs_g0_w_m2 = diagnostics
            .maximum_abs_g0_w_m2
            .max(hour.peak_substep_applied_g0_w_m2.abs());
        if hour.peak_substep_requested_g0_w_m2.abs()
            > diagnostics.peak_g0_requested_w_m2.abs()
        {
            diagnostics.peak_g0_w_m2 = hour.peak_substep_applied_g0_w_m2;
            diagnostics.peak_g0_requested_w_m2 =
                hour.peak_substep_requested_g0_w_m2;
            diagnostics.peak_g0_rejected_w_m2 =
                hour.peak_substep_rejected_g0_w_m2;
            diagnostics.peak_g0_pressure_pa = hour.peak_substep_pressure_pa;
            diagnostics.peak_g0_active_temperature_c =
                hour.peak_substep_active_temperature_c;
            diagnostics.peak_g0_lower_temperature_c =
                hour.peak_substep_lower_temperature_c;
            diagnostics.peak_g0_active_depth_m =
                hour.peak_substep_active_depth_m;
            diagnostics.peak_g0_lower_depth_m =
                hour.peak_substep_lower_depth_m;
            diagnostics.peak_g0_active_conductivity_w_m_k =
                hour.peak_substep_active_conductivity_w_m_k;
            diagnostics.peak_g0_lower_conductivity_w_m_k =
                hour.peak_substep_lower_conductivity_w_m_k;
            diagnostics.peak_g0_active_resistance_m2_k_w =
                hour.peak_substep_active_resistance_m2_k_w;
            diagnostics.peak_g0_lower_resistance_m2_k_w =
                hour.peak_substep_lower_resistance_m2_k_w;
        }
        if hour.minimum_substep_seconds > 0.0
            && (diagnostics.minimum_substep_seconds == 0.0
                || hour.minimum_substep_seconds < diagnostics.minimum_substep_seconds)
        {
            diagnostics.minimum_substep_seconds = hour.minimum_substep_seconds;
        }
        diagnostics.maximum_active_energy_residual_j_m2 = diagnostics
            .maximum_active_energy_residual_j_m2
            .max(hour.maximum_active_energy_closure_residual_j_m2);
        diagnostics.maximum_lower_energy_residual_j_m2 = diagnostics
            .maximum_lower_energy_residual_j_m2
            .max(hour.maximum_lower_energy_closure_residual_j_m2);
        diagnostics.maximum_conduction_cancellation_residual_j_m2 = diagnostics
            .maximum_conduction_cancellation_residual_j_m2
            .max(hour.maximum_conduction_cancellation_residual_j_m2);
    }
    diagnostics
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
    minimum_temperature_after_c: f64,
    maximum_temperature_after_c: f64,
}

fn direct_snow_trace_layer_diagnostics(
    snow_lane_state: &openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> DirectSnowTraceLayerDiagnostics {
    let (surface_before, basal_before, gradient_before) =
        snow_layer_density_profile(&snow_lane_state.layers);
    let (surface_after, basal_after, gradient_after) =
        snow_layer_density_profile(&snow_liquid.snow_layers_after);
    let (minimum_temperature_after_c, maximum_temperature_after_c) =
        snow_layer_temperature_range(&snow_liquid.snow_layers_after);
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
        minimum_temperature_after_c,
        maximum_temperature_after_c,
    }
}

fn snow_layer_swe_sum(layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState]) -> f64 {
    layers.iter().map(|layer| layer.mass_swe_m).sum()
}

fn snow_layer_depth_sum(layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState]) -> f64 {
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
    (
        surface_density,
        basal_density,
        basal_density - surface_density,
    )
}

fn snow_layer_temperature_range(
    layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState],
) -> (f64, f64) {
    if layers.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    layers.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(minimum, maximum), layer| {
            (
                minimum.min(layer.temperature_c),
                maximum.max(layer.temperature_c),
            )
        },
    )
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
    std::io::Write::write_all(&mut file, line.as_bytes()).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_wb15_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed writing direct production WB15 trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        }
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
        "physics_bulk_shallow_guard_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkShallowGuardV1)
        }
        "physics_bulk_climate_class_density_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkClimateClassDensityV1)
        }
        "physics_bulk_multilayer_density_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkMultilayerDensityV1)
        }
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
        "" | "legacy_coe" => Ok(openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe),
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
        "" | "coe_liquid_holding_capacity_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::CoeLiquidHoldingCapacityV1)
        }
        "coe_open_sublimation_stage_a_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::CoeOpenSublimationStageAV1)
        }
        "coe_open_sublimation_stage_b_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::CoeOpenSublimationStageBV1)
        }
        "legacy_coe" => Ok(openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe),
        observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOWDENSITY1038_MELT_MODEL_ENV} must be legacy_coe, coe_liquid_holding_capacity_v1, coe_open_sublimation_stage_a_v1, or coe_open_sublimation_stage_b_v1, observed {observed}"
            ),
        }),
    }
}

fn snowdensity1015_default_snow_density_model()
-> Result<openwepp_hillslope_orchestrator::SnowDensityModel, HillslopeCliError> {
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

fn paradigm2_stage3_liquid_routing_model()
-> Result<openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel, HillslopeCliError> {
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

fn snow_surface_longwave_model()
-> Result<openwepp_hillslope_orchestrator::SnowSurfaceLongwaveModel, HillslopeCliError> {
    match std::env::var(SNOW_SURFACE_LONGWAVE_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "disabled" => Ok(openwepp_hillslope_orchestrator::SnowSurfaceLongwaveModel::Disabled),
            "dilley_unsworth_subcanopy_v1" => Ok(
                openwepp_hillslope_orchestrator::SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
            ),
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_surface_longwave_model",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {SNOW_SURFACE_LONGWAVE_MODEL_ENV} must be disabled, dilley_unsworth_subcanopy_v1, or empty default, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => Ok(
            openwepp_hillslope_orchestrator::SnowSurfaceLongwaveModel::Disabled,
        ),
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_surface_longwave_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOW_SURFACE_LONGWAVE_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn snow_surface_sublimation_model()
-> Result<openwepp_hillslope_orchestrator::SnowSurfaceSublimationModel, HillslopeCliError> {
    match std::env::var(SNOW_SURFACE_SUBLIMATION_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "disabled" => Ok(
                openwepp_hillslope_orchestrator::SnowSurfaceSublimationModel::Disabled,
            ),
            "neutral_bulk_stage3_v1" => Ok(
                openwepp_hillslope_orchestrator::SnowSurfaceSublimationModel::NeutralBulkStage3V1,
            ),
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_surface_sublimation_model",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {SNOW_SURFACE_SUBLIMATION_MODEL_ENV} must be disabled, neutral_bulk_stage3_v1, or empty default, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => Ok(
            openwepp_hillslope_orchestrator::SnowSurfaceSublimationModel::Disabled,
        ),
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_surface_sublimation_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOW_SURFACE_SUBLIMATION_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn snow_stage3_evaluation_operator() -> Result<
    Option<openwepp_hillslope_orchestrator::SnowStage3EvaluationOperator>,
    HillslopeCliError,
> {
    let explicit = std::env::var(SNOW_STAGE3_EVALUATION_OPERATOR_ENV).map_err(|error| match error {
        std::env::VarError::NotPresent => None,
        std::env::VarError::NotUnicode(_) => Some(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_stage3_evaluation_operator",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOW_STAGE3_EVALUATION_OPERATOR_ENV} must be UTF-8"
            ),
        }),
    });
    let legacy = std::env::var(SNOW_STAGE3_COMPLETE_CARRIER_SHADOW_ENV).map_err(|error| match error {
        std::env::VarError::NotPresent => None,
        std::env::VarError::NotUnicode(_) => Some(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_stage3_complete_carrier_shadow",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOW_STAGE3_COMPLETE_CARRIER_SHADOW_ENV} must be UTF-8"
            ),
        }),
    });
    match (explicit, legacy) {
        (Err(Some(error)), _) | (_, Err(Some(error))) => Err(error),
        (explicit, legacy) => snow_stage3_evaluation_operator_from_values(
            explicit.ok().as_deref(),
            legacy.ok().as_deref(),
        ),
    }
}

fn snow_stage3_evaluation_operator_from_values(
    explicit_value: Option<&str>,
    legacy_value: Option<&str>,
) -> Result<
    Option<openwepp_hillslope_orchestrator::SnowStage3EvaluationOperator>,
    HillslopeCliError,
> {
    use openwepp_hillslope_orchestrator::SnowStage3EvaluationOperator;

    let explicit = match explicit_value {
        None => None,
        Some(value) => Some(match value.trim() {
            "" | "disabled" => None,
            "same_state_paired_carrier_v1" => {
                Some(SnowStage3EvaluationOperator::SameStatePairedCarrierV1)
            }
            "sequential_resolved_shadow_v1" => {
                Some(SnowStage3EvaluationOperator::SequentialResolvedShadowV1)
            }
            "persistent_accumulation_shadow_v1" => {
                Some(SnowStage3EvaluationOperator::PersistentAccumulationShadowV1)
            }
            observed => {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_production_snow_stage3_evaluation_operator",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} {SNOW_STAGE3_EVALUATION_OPERATOR_ENV} must be disabled, same_state_paired_carrier_v1, sequential_resolved_shadow_v1, persistent_accumulation_shadow_v1, or empty default, observed {observed}"
                    ),
                });
            }
        }),
    };
    let legacy = match legacy_value {
        None => None,
        Some(value) => Some(match value.trim() {
            "" | "0" | "false" | "disabled" => None,
            "1" | "true" | "enabled" => {
                Some(SnowStage3EvaluationOperator::SequentialResolvedShadowV1)
            }
            observed => {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_production_snow_stage3_complete_carrier_shadow",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} {SNOW_STAGE3_COMPLETE_CARRIER_SHADOW_ENV} must be enabled, disabled, true, false, 1, 0, or empty default, observed {observed}"
                    ),
                });
            }
        }),
    };
    match (explicit, legacy) {
        (Some(explicit_value), Some(legacy_value)) if explicit_value != legacy_value => {
            Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_stage3_evaluation_operator",
                detail: format!(
                    "{SIMOUT_GUARD_ID} conflicting {SNOW_STAGE3_EVALUATION_OPERATOR_ENV} and {SNOW_STAGE3_COMPLETE_CARRIER_SHADOW_ENV} requests"
                ),
            })
        }
        (Some(value), _) | (_, Some(value)) => Ok(value),
        (None, None) => Ok(None),
    }
}

#[cfg(test)]
mod stage3_evaluation_operator_tests {
    use super::*;
    use openwepp_hillslope_orchestrator::SnowStage3EvaluationOperator;

    #[test]
    fn default_and_legacy_compatibility_are_typed() {
        assert_eq!(
            snow_stage3_evaluation_operator_from_values(None, None)
                .expect("absent default"),
            None
        );
        assert_eq!(
            snow_stage3_evaluation_operator_from_values(None, Some("enabled"))
                .expect("legacy compatibility spelling"),
            Some(SnowStage3EvaluationOperator::SequentialResolvedShadowV1)
        );
        assert_eq!(
            snow_stage3_evaluation_operator_from_values(
                Some("same_state_paired_carrier_v1"),
                None,
            )
            .expect("typed paired request"),
            Some(SnowStage3EvaluationOperator::SameStatePairedCarrierV1)
        );
        assert_eq!(
            snow_stage3_evaluation_operator_from_values(
                Some("persistent_accumulation_shadow_v1"),
                None,
            )
            .expect("typed persistent request"),
            Some(SnowStage3EvaluationOperator::PersistentAccumulationShadowV1)
        );
    }

    #[test]
    fn conflicting_or_unknown_requests_fail_closed() {
        let conflict = snow_stage3_evaluation_operator_from_values(
            Some("same_state_paired_carrier_v1"),
            Some("enabled"),
        )
        .expect_err("legacy sequential request conflicts with paired request");
        assert!(conflict.to_string().contains("conflicting"));
        let unsupported = snow_stage3_evaluation_operator_from_values(Some("generic"), None)
            .expect_err("generic operator is not admitted");
        assert!(unsupported.to_string().contains("generic"));
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
        let month_index = usize::try_from(day.month - 1).map_err(|_| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_sturm1995_climate_class",
                detail: format!(
                    "{SIMOUT_GUARD_ID} invalid climate month {} for Sturm 1995 climate normals",
                    day.month
                ),
            }
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
        if mean_temperature_c
            < openwepp_hillslope_orchestrator::STURM1995_CDM_CRITICAL_TEMPERATURE_C
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

fn snowdensity1035_diagnostic_snow_phase_model()
-> Result<openwepp_hillslope_orchestrator::SnowPhasePartitionModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY1035_PHASE_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "harder_pomeroy_hourly" => {
                Ok(openwepp_hillslope_orchestrator::SnowPhasePartitionModel::HarderPomeroyHourly)
            }
            "legacy_rst" => Ok(openwepp_hillslope_orchestrator::SnowPhasePartitionModel::LegacyRst),
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
fn snowdensity1037_diagnostic_snow_melt_model()
-> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
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

fn snowdensity1015_default_snow_melt_model()
-> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY1038_MELT_MODEL_ENV) {
        Ok(value) => parse_snowdensity1015_default_snow_melt_model(Some(&value)),
        Err(std::env::VarError::NotPresent) => parse_snowdensity1015_default_snow_melt_model(None),
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY1038_MELT_MODEL_ENV} must be UTF-8"),
        }),
    }
}

// E.3 stage 2e (Wave-2 deleted): erosion activation is the Wave-1 seed
// alone. SC-SED-001 1b-C: the seed must attach EVERY day so the
// persistent consolidation carry (`rfcum`/`daydis`) advances daily per
// `soil.for` (aging on dry days after `rfcum > 0.01`), not only on
// rainfall days. The solve still gates itself inactive on non-runoff
// days. A disabled seed (active-tillage scope) has no erosion producer.
fn direct_production_erosion_active(authority: &DirectProductionLaneDayInputAuthority) -> bool {
    authority.erosion.erosion_inputs.wave1_operand_seed.enabled
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

fn advance_native_canopy_with_checked_height(
    committed_state: &mut Option<openwepp_plant_phenology::ForestCanopyState>,
    parameters: openwepp_plant_phenology::ForestCanopyParameters,
    forcing: openwepp_plant_phenology::GsiDailyForcing,
    canopy_height_coefficient_m2_kg: f64,
    maximum_canopy_height_m: f64,
) -> Result<
    (
        openwepp_plant_phenology::ForestCanopyDailyResult,
        f64,
    ),
    HillslopeCliError,
> {
    let mut candidate_state = committed_state
        .clone()
        .unwrap_or_else(openwepp_plant_phenology::ForestCanopyState::new_uninitialized);
    let daily = candidate_state
        .advance(parameters, forcing)
        .map_err(|source| direct_growth_failure(source.to_string()))?;
    let canopy_height_m = openwepp_hillslope_orchestrator::direct_native_canopy_height_m(
        daily.canopy.live_foliar_biomass_kg_m2,
        daily.canopy.structural_biomass_kg_m2,
        canopy_height_coefficient_m2_kg,
        maximum_canopy_height_m,
    )
    .map_err(|source| direct_growth_failure(source.to_string()))?;
    *committed_state = Some(candidate_state);
    Ok((daily, canopy_height_m))
}

fn build_laned_shadow_lane_day_operands(
    lane_index: usize,
    day_index: usize,
    hourly_rainfall_m: [f64; openwepp_hillslope_orchestrator::ofe_routing::seam::SEAM_HOUR_BINS],
    hourly_routed_melt_m: [f64; openwepp_hillslope_orchestrator::ofe_routing::seam::SEAM_HOUR_BINS],
    leaf_area_index: f64,
    canopy_height_m: Option<f64>,
) -> Result<crate::hillslope::laned_shadow::LanedShadowLaneDayOperands, HillslopeCliError> {
    if !leaf_area_index.is_finite() || leaf_area_index < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "laned_shadow_dynamic_operands",
            detail: format!(
                "{SIMOUT_GUARD_ID} Lane D shadow requires finite nonnegative post-growth LAI for lane {} day {}, observed {}",
                lane_index + 1,
                day_index + 1,
                leaf_area_index
            ),
        });
    }
    for (hour_index, rainfall_m) in hourly_rainfall_m.iter().enumerate() {
        if !rainfall_m.is_finite() || *rainfall_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "laned_shadow_dynamic_operands",
                detail: format!(
                    "{SIMOUT_GUARD_ID} Lane D shadow requires finite nonnegative WB14 hourly rainfall for lane {} day {} hour {}, observed {}",
                    lane_index + 1,
                    day_index + 1,
                    hour_index + 1,
                    rainfall_m
                ),
            });
        }
    }
    for (hour_index, routed_melt_m) in hourly_routed_melt_m.iter().enumerate() {
        if !routed_melt_m.is_finite() || *routed_melt_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "laned_shadow_dynamic_operands",
                detail: format!(
                    "{SIMOUT_GUARD_ID} Lane D shadow requires finite nonnegative hourly routed melt for lane {} day {} hour {}, observed {}",
                    lane_index + 1,
                    day_index + 1,
                    hour_index + 1,
                    routed_melt_m
                ),
            });
        }
    }
    let canopy_height_m = match canopy_height_m {
        Some(height_m) if height_m.is_finite() && height_m >= 0.0 => height_m,
        Some(height_m) => {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "laned_shadow_dynamic_operands",
                detail: format!(
                    "{SIMOUT_GUARD_ID} Lane D shadow post-growth canhgt must be finite and nonnegative for lane {}, observed {}",
                    lane_index + 1,
                    height_m
                ),
            });
        }
        None if leaf_area_index <= 0.0 => 0.0,
        None => {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "laned_shadow_dynamic_operands",
                detail: format!(
                    "{SIMOUT_GUARD_ID} Lane D shadow requires post-growth canhgt when post-growth LAI is positive for lane {} day {} (LAI={})",
                    lane_index + 1,
                    day_index + 1,
                    leaf_area_index
                ),
            });
        }
    };
    if leaf_area_index > 0.0 && canopy_height_m <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "laned_shadow_dynamic_operands",
            detail: format!(
                "{SIMOUT_GUARD_ID} Lane D shadow requires canhgt > 0 when post-growth LAI is positive for lane {} day {} (LAI={}, canhgt={})",
                lane_index + 1,
                day_index + 1,
                leaf_area_index,
                canopy_height_m
            ),
        });
    }
    Ok(crate::hillslope::laned_shadow::LanedShadowLaneDayOperands {
        hourly_rainfall_m,
        hourly_routed_melt_m,
        leaf_area_index,
        canopy_height_m,
    })
}

#[cfg(test)]
mod laned_shadow_dynamic_operand_tests {
    use super::*;

    fn dynamic_operand_error_detail(
        result: Result<
            crate::hillslope::laned_shadow::LanedShadowLaneDayOperands,
            HillslopeCliError,
        >,
    ) -> String {
        match result {
            Err(HillslopeCliError::RuntimeSurfaceFailure { surface, detail }) => {
                assert_eq!(surface, "laned_shadow_dynamic_operands");
                detail
            }
            Err(error) => panic!("unexpected error: {error}"),
            Ok(_) => panic!("expected dynamic operand validation failure"),
        }
    }

    #[test]
    fn laned_shadow_dynamic_operands_reject_missing_canhgt_when_lai_positive() {
        let detail = dynamic_operand_error_detail(build_laned_shadow_lane_day_operands(
            0, 0, [0.0; 24], [0.0; 24], 1.25, None,
        ));

        assert!(detail.contains("requires post-growth canhgt"));
    }

    #[test]
    fn laned_shadow_dynamic_operands_reject_zero_canhgt_when_lai_positive() {
        let detail = dynamic_operand_error_detail(build_laned_shadow_lane_day_operands(
            0,
            0,
            [0.0; 24],
            [0.0; 24],
            1.25,
            Some(0.0),
        ));

        assert!(detail.contains("requires canhgt > 0"));
    }

    #[test]
    fn laned_shadow_dynamic_operands_preserve_hourly_rainfall_when_valid() {
        let mut hourly_rainfall_m = [0.0; 24];
        hourly_rainfall_m[3] = 0.0125;

        let mut hourly_routed_melt_m = [0.0; 24];
        hourly_routed_melt_m[7] = 0.0025;

        let operands = build_laned_shadow_lane_day_operands(
            0,
            0,
            hourly_rainfall_m,
            hourly_routed_melt_m,
            0.0,
            None,
        )
        .expect("bare day operands should accept absent canopy height");

        assert_eq!(
            operands.hourly_rainfall_m[3].to_bits(),
            0.0125_f64.to_bits()
        );
        assert_eq!(
            operands.hourly_routed_melt_m[7].to_bits(),
            0.0025_f64.to_bits()
        );
        assert_eq!(operands.leaf_area_index.to_bits(), 0.0_f64.to_bits());
        assert_eq!(operands.canopy_height_m.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn m08_dynamic_operand_guards_preserve_index_and_priority() {
        let mut invalid_rain = [0.0; 24];
        invalid_rain[4] = f64::NAN;
        let mut invalid_melt = [0.0; 24];
        invalid_melt[6] = -0.25;

        let detail = dynamic_operand_error_detail(build_laned_shadow_lane_day_operands(
            1,
            2,
            invalid_rain,
            invalid_melt,
            -0.5,
            Some(f64::NAN),
        ));
        assert_eq!(
            detail,
            format!(
                "{SIMOUT_GUARD_ID} Lane D shadow requires finite nonnegative post-growth LAI for lane 2 day 3, observed -0.5"
            )
        );

        let detail = dynamic_operand_error_detail(build_laned_shadow_lane_day_operands(
            1,
            2,
            invalid_rain,
            invalid_melt,
            0.0,
            None,
        ));
        assert_eq!(
            detail,
            format!(
                "{SIMOUT_GUARD_ID} Lane D shadow requires finite nonnegative WB14 hourly rainfall for lane 2 day 3 hour 5, observed NaN"
            )
        );

        for invalid_lai in [f64::NAN, -0.25, f64::INFINITY, f64::NEG_INFINITY] {
            let detail = dynamic_operand_error_detail(build_laned_shadow_lane_day_operands(
                1,
                2,
                [0.0; 24],
                [0.0; 24],
                invalid_lai,
                Some(1.0),
            ));
            assert_eq!(
                detail,
                format!(
                    "{SIMOUT_GUARD_ID} Lane D shadow requires finite nonnegative post-growth LAI for lane 2 day 3, observed {invalid_lai}"
                )
            );
        }

        for invalid_rainfall in [f64::NAN, -0.25, f64::INFINITY, f64::NEG_INFINITY] {
            let mut rainfall = [0.0; 24];
            rainfall[4] = invalid_rainfall;
            let detail = dynamic_operand_error_detail(build_laned_shadow_lane_day_operands(
                1, 2, rainfall, [0.0; 24], 0.0, None,
            ));
            assert_eq!(
                detail,
                format!(
                    "{SIMOUT_GUARD_ID} Lane D shadow requires finite nonnegative WB14 hourly rainfall for lane 2 day 3 hour 5, observed {invalid_rainfall}"
                )
            );
        }

        for invalid_routed_melt in [f64::NAN, -0.25, f64::INFINITY, f64::NEG_INFINITY] {
            let mut routed_melt = [0.0; 24];
            routed_melt[6] = invalid_routed_melt;
            let detail = dynamic_operand_error_detail(build_laned_shadow_lane_day_operands(
                1,
                2,
                [0.0; 24],
                routed_melt,
                0.0,
                Some(f64::NAN),
            ));
            assert_eq!(
                detail,
                format!(
                    "{SIMOUT_GUARD_ID} Lane D shadow requires finite nonnegative hourly routed melt for lane 2 day 3 hour 7, observed {invalid_routed_melt}"
                )
            );
        }

        for invalid_height in [f64::NAN, -0.25, f64::INFINITY, f64::NEG_INFINITY] {
            let detail = dynamic_operand_error_detail(build_laned_shadow_lane_day_operands(
                1,
                2,
                [0.0; 24],
                [0.0; 24],
                1.0,
                Some(invalid_height),
            ));
            assert_eq!(
                detail,
                format!(
                    "{SIMOUT_GUARD_ID} Lane D shadow post-growth canhgt must be finite and nonnegative for lane 2, observed {invalid_height}"
                )
            );
        }

        let operands =
            build_laned_shadow_lane_day_operands(1, 2, [0.0; 24], [0.0; 24], 1.25, Some(0.75))
                .expect("finite positive vegetation operands must pass");
        assert_eq!(operands.leaf_area_index.to_bits(), 1.25_f64.to_bits());
        assert_eq!(operands.canopy_height_m.to_bits(), 0.75_f64.to_bits());
    }
}
