const SNOWDENSITY09_DENSITY_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL";
const SNOWDENSITY1035_PHASE_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL";
const SNOWDENSITY1037_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1037_MELT_MODEL";
const SNOWDENSITY1038_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL";

struct DirectPublicationDayInputBuilder<'a> {
    climate_request: &'a HillslopeClimateRuntimeRequest,
    climate_span: &'a ClimateRunSpanSummary,
    climate_context_state: std::cell::RefCell<DirectPublicationClimateContextState>,
    seed_surfaces: Vec<HillslopeWritebackSurface>,
    execution_lane: ExecutionLane,
    profile_inputs: Vec<DirectHydrologyProjectionInputs>,
    erosion_guard_active: bool,
    record_compatibility_edge_invocations: bool,
}
struct DirectPublicationClimateContextState {
    rolling_surface: HillslopeWritebackSurface,
    current_day_index: Option<usize>,
    current_day_context_surface: Option<HillslopeWritebackSurface>,
}
impl<'a> DirectPublicationDayInputBuilder<'a> {
    fn new(
        climate_request: &'a HillslopeClimateRuntimeRequest,
        climate_span: &'a ClimateRunSpanSummary,
        static_runtime_surface: &'a HillslopeWritebackSurface,
        execution_lane: ExecutionLane,
    ) -> Result<Self, HillslopeCliError> {
        Self::new_with_seed_surfaces(
            climate_request,
            climate_span,
            vec![static_runtime_surface.clone()],
            static_runtime_surface,
            execution_lane,
        )
    }
    fn new_with_seed_surfaces(
        climate_request: &'a HillslopeClimateRuntimeRequest,
        climate_span: &'a ClimateRunSpanSummary,
        seed_surfaces: Vec<HillslopeWritebackSurface>,
        climate_context_surface: &HillslopeWritebackSurface,
        execution_lane: ExecutionLane,
    ) -> Result<Self, HillslopeCliError> {
        Self::new_with_seed_surfaces_and_erosion_guard(
            climate_request,
            climate_span,
            seed_surfaces,
            climate_context_surface,
            execution_lane,
            false,
        )
    }
    fn new_with_seed_surfaces_and_erosion_guard(
        climate_request: &'a HillslopeClimateRuntimeRequest,
        climate_span: &'a ClimateRunSpanSummary,
        seed_surfaces: Vec<HillslopeWritebackSurface>,
        climate_context_surface: &HillslopeWritebackSurface,
        execution_lane: ExecutionLane,
        erosion_guard_active: bool,
    ) -> Result<Self, HillslopeCliError> {
        if seed_surfaces.is_empty() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication requires at least one seed surface"
                ),
            });
        }
        let profile_inputs = seed_surfaces
            .iter()
            .map(direct_publication_profile_inputs)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            climate_request,
            climate_span,
            climate_context_state: std::cell::RefCell::new(
                DirectPublicationClimateContextState {
                    rolling_surface: climate_context_surface.clone(),
                    current_day_index: None,
                    current_day_context_surface: None,
                },
            ),
            seed_surfaces,
            execution_lane,
            profile_inputs,
            erosion_guard_active,
            record_compatibility_edge_invocations: erosion_guard_active,
        })
    }
    fn build(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<DirectPublicationDayInput, HillslopeCliError> {
        self.build_with_seed_surface(frame, day_index, lane_index)
            .map(|(day_input, _seed_surface)| day_input)
    }

    #[allow(clippy::too_many_lines)]
    fn build_with_seed_surface(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<(DirectPublicationDayInput, HillslopeWritebackSurface), HillslopeCliError> {
        let lane = frame.lanes.get(lane_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication lane index {} exceeds frame lane count {}",
                    lane_index + 1,
                    frame.lanes.len()
                ),
            }
        })?;
        let frost_lane_state = lane.winter_column.frost.clone();
        let forcing =
            self.climate_request
                .direct_day_forcing(day_index)
                .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct publication typed climate forcing failed: {source}"
                    ),
                })?;
        if self.record_compatibility_edge_invocations {
            record_direct_runtime_compatibility_edge_invocation();
        }
        let (mut seed_surface, day) = self.seed_surface(frame, day_index, lane_index)?;
        let simulation_year =
            simulation_year_from_calendar_year(day.year, self.climate_span.first_day.year)?;

        let precipitation_m = day.precipitation_mm / 1_000.0;
        let mut day_input =
            DirectPublicationDayInput::calendar_only(direct_publication_calendar_day(day)?);
        day_input.frost_runtime_carry =
            direct_publication_frost_runtime_carry_from_lane_state(&frost_lane_state);
        let hyetograph = direct_publication_hyetograph(&seed_surface)?;
        let frost_layers = direct_publication_layer_states(&seed_surface)?;
        if !direct_publication_has_frost_runtime_carry(&seed_surface)? {
            overlay_direct_publication_frost_fine_state(
                &mut seed_surface,
                lane_index,
                &frost_layers,
            )?;
        }
        let snow_frost_authority = DirectProductionSnowFrostAuthority::from_seed(&seed_surface)?;
        let winter_hourly_geometry =
            DirectProductionWinterHourlyGeometry::from_climate_context_surface(&seed_surface)?;
        let growth_state_before = direct_growth_state_for_lane(&seed_surface, day_index, lane)?;
        overlay_direct_publication_growth_state(&mut seed_surface, lane_index, growth_state_before)?;
        let snow_lane_state = snow_frost_authority.current_snow_lane_state(lane);
        let frost_context = snow_frost_authority.frost_day_context(
            self.climate_request,
            day_index,
            day,
            lane_index,
            lane,
            &forcing,
            snow_lane_state,
            winter_hourly_geometry,
            precipitation_m > 1.0e-12 || snow_lane_state.runtime_swe_m > 1.0e-12,
        )?;
        if let Some(frost_context) = &frost_context {
            insert_direct_seed_scalar(
                &mut seed_surface,
                "frost.runtime_infcap_frz",
                frost_context.frozen_infiltration_capacity_m_s,
                lane_index,
            )?;
        }
        let snow_liquid =
            direct_publication_snow_liquid_partition(&seed_surface, &hyetograph)?;
        let interception_state =
            direct_publication_interception_state(
                &seed_surface,
                snow_liquid.post_winter_rain_m,
                &hyetograph,
            )?;
        let post_winter_hyetograph = direct_publication_scaled_hyetograph_to_rainfall(
            &hyetograph,
            snow_liquid.post_winter_rain_m,
        )?;
        let post_interception_hyetograph = direct_publication_scaled_hyetograph(
            &post_winter_hyetograph,
            interception_state.rainfall_scale,
        )?;
        let liquid_hyetograph = direct_publication_hyetograph_with_added_daily_depth(
            &post_interception_hyetograph,
            snow_liquid.routed_melt_m,
        )?;
        day_input.precipitation_m = precipitation_m;
        day_input.effective_temperature_c = day.effective_temperature_c;
        day_input.interception_m = interception_state.interception_m;
        day_input.peak_runoff_inputs = Some(direct_publication_peak_runoff_inputs(
            &seed_surface,
            liquid_hyetograph.clone(),
        )?);
        let erosion_wave2_active = self.erosion_guard_active
            && direct_publication_erosion_wave2_active(
                &seed_surface,
                &liquid_hyetograph,
            )?;
        day_input.erosion_producer_required = erosion_wave2_active;
        if erosion_wave2_active {
            day_input.erosion_inputs = Some(direct_publication_erosion_inputs(&seed_surface)?);
        }
        day_input.liquid_input_inputs =
            Some(direct_publication_liquid_input_inputs(
                interception_state.liquid_after_interception_m + snow_liquid.routed_melt_m,
            )?);
        day_input.storage_input_inputs =
            Some(direct_publication_storage_input_inputs(&seed_surface)?);
        day_input.snow_coupling_inputs = Some(DirectSnowCouplingInputs {
            snow_coupling_handoff_m: snow_liquid.snow_coupling_signed_s_m,
            snow_state_projected: true,
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
        });
        let percolation_inputs =
            direct_publication_percolation_inputs(&seed_surface, precipitation_m)?;
        let subsurface_inputs = direct_publication_subsurface_inputs(&seed_surface)?;
        day_input.infiltration_depression_inputs = Some(
            direct_publication_infiltration_depression_inputs(
                &seed_surface,
                liquid_hyetograph,
            )?,
        );
        day_input.initial_soil_water_m =
            Some(require_runtime_surface_scalar(&seed_surface, "wb11_soil_water")?);
        day_input.percolation_inputs = Some(percolation_inputs);
        day_input.subsurface_compute_inputs = Some(subsurface_inputs);
        let growth_authority = DirectProductionGrowthAuthority::from_seed(&seed_surface)?;
        let mut dynamic_evapotranspiration =
            DirectProductionEvapotranspirationAuthority::from_seed(&seed_surface, frost_layers.len())?;
        dynamic_evapotranspiration.apply_growth_surface(growth_state_before);
        let pre_growth_evapotranspiration_compute_inputs = dynamic_evapotranspiration.inputs(
            day,
            &forcing,
            None,
            &frost_layers,
            self.climate_request,
        )?;
        let (annual_growth_inputs, perennial_growth_inputs) = growth_authority.inputs(
            day,
            simulation_year,
            lane_index + 1,
            &forcing,
            growth_state_before,
            direct_growth_water_stress_for_lane(&seed_surface, day_index, lane)?,
            &pre_growth_evapotranspiration_compute_inputs,
        )?;
        let evapotranspiration_compute_inputs = pre_growth_evapotranspiration_compute_inputs;
        day_input.evapotranspiration_compute_inputs = Some(evapotranspiration_compute_inputs);
        day_input.annual_growth_inputs = Some(annual_growth_inputs);
        day_input.perennial_growth_inputs = Some(perennial_growth_inputs);
        day_input.hydrology_projection_inputs =
            Some(direct_publication_hydrology_projection_inputs(
                *self.profile_inputs(lane_index)?,
                &snow_liquid,
        ));
        if let Some(frost_context) = frost_context {
            day_input.winter_frost_compute_inputs = Some(frost_context.compute_inputs);
            day_input.frost_storage_liquid_delta_m = frost_context.storage_liquid_delta_m;
            day_input.frost_layer_carry_projection = frost_context.layer_carry_projection;
        }
        Ok((day_input, seed_surface))
    }
    fn seed_surface(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<(HillslopeWritebackSurface, &ClimateDayProjection), HillslopeCliError> {
        let day = self.climate_span.days.get(day_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication day index {} exceeds climate span {}",
                    day_index + 1,
                    self.climate_span.days.len()
                ),
            }
        })?;
        direct_publication_validate_day(day)?;
        let lane = frame
            .lanes
            .get(lane_index)
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication lane index {} exceeds frame lane count {}",
                    lane_index + 1,
                    frame.lanes.len()
                ),
            })?;

        let seed_authority = self.seed_surface_authority(lane_index)?;
        let mut seed_surface = seed_authority.clone();
        let climate_context_surface = self.climate_context_surface(frame, day_index)?;
        let mut climate_surface = build_day_climate_surface(
            self.climate_request,
            day_index,
            &climate_context_surface,
            day,
        )?;
        self.advance_climate_context(day_index, &climate_context_surface, &climate_surface);
        seed_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
            seed_surface,
            std::mem::take(&mut climate_surface),
        );
        overlay_direct_publication_lane_state(&mut seed_surface, day_index, lane_index, lane)?;
        seed_wb11_runtime_surface_inputs(&mut seed_surface, self.execution_lane)?;
        Ok((seed_surface, day))
    }

    fn seed_surface_authority(
        &self,
        lane_index: usize,
    ) -> Result<&HillslopeWritebackSurface, HillslopeCliError> {
        if self.seed_surfaces.len() == 1 {
            return Ok(&self.seed_surfaces[0]);
        }
        self.seed_surfaces.get(lane_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication lane index {} exceeds lane seed authority count {}",
                    lane_index + 1,
                    self.seed_surfaces.len()
                ),
            }
        })
    }

    fn climate_context_surface(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
    ) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
        {
            let state = self.climate_context_state.borrow();
            if state.current_day_index == Some(day_index) {
                if let Some(context_surface) = state.current_day_context_surface.as_ref() {
                    return Ok(context_surface.clone());
                }
            }
        }

        let mut context_surface = self.climate_context_state.borrow().rolling_surface.clone();
        if day_index > 0 {
            let outlet_lane_index = frame.lanes.len().checked_sub(1).ok_or_else(|| {
                HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct publication climate context requires at least one lane"
                    ),
                }
            })?;
            let outlet_lane =
                frame
                    .lanes
                    .get(outlet_lane_index)
                    .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct publication outlet lane index {} exceeds frame lane count {}",
                        outlet_lane_index + 1,
                        frame.lanes.len()
                    ),
                })?;
            let outlet_seed_surface = self.seed_surface_authority(outlet_lane_index)?.clone();
            context_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
                context_surface,
                outlet_seed_surface,
            );
            overlay_direct_publication_lane_state(
                &mut context_surface,
                day_index,
                outlet_lane_index,
                outlet_lane,
            )?;
            seed_wb11_runtime_surface_inputs(&mut context_surface, self.execution_lane)?;
        }

        let mut state = self.climate_context_state.borrow_mut();
        state.current_day_index = Some(day_index);
        state.current_day_context_surface = Some(context_surface.clone());
        Ok(context_surface)
    }

    fn advance_climate_context(
        &self,
        day_index: usize,
        context_surface: &HillslopeWritebackSurface,
        climate_surface: &HillslopeWritebackSurface,
    ) {
        let mut state = self.climate_context_state.borrow_mut();
        if state.current_day_index == Some(day_index) {
            state.rolling_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
                context_surface.clone(),
                climate_surface.clone(),
            );
        }
    }

    fn profile_inputs(
        &self,
        lane_index: usize,
    ) -> Result<&DirectHydrologyProjectionInputs, HillslopeCliError> {
        if self.profile_inputs.len() == 1 {
            return Ok(&self.profile_inputs[0]);
        }
        self.profile_inputs.get(lane_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication lane index {} exceeds profile seed count {}",
                    lane_index + 1,
                    self.profile_inputs.len()
                ),
            }
        })
    }
}
struct DirectProductionDayInputBuilder<'a> {
    climate_request: &'a HillslopeClimateRuntimeRequest,
    climate_span: &'a ClimateRunSpanSummary,
    lane_authority: Vec<DirectProductionLaneDayInputAuthority>,
    winter_hourly_geometry: DirectProductionWinterHourlyGeometry,
    sturm_climate_class: Option<openwepp_hillslope_orchestrator::SnowClimateClass>,
}

#[derive(Clone)]
struct DirectProductionLaneDayInputAuthority {
    peak_runoff: DirectProductionPeakRunoffAuthority,
    percolation: DirectPercolationInputs,
    subsurface: DirectSubsurfaceComputeInputs,
    infiltration: DirectProductionInfiltrationAuthority,
    evapotranspiration: DirectProductionEvapotranspirationAuthority,
    growth: DirectProductionGrowthAuthority,
    hydrology_projection: DirectHydrologyProjectionInputs,
    erosion: DirectProductionErosionAuthority,
    snow_frost: DirectProductionSnowFrostAuthority,
}

#[derive(Clone)]
struct DirectProductionPeakRunoffAuthority {
    irrigation_rate_m_s: f64,
    efflen_m: f64,
    ealpha: f64,
    exponent_m: f64,
}

#[derive(Clone)]
struct DirectProductionInfiltrationAuthority {
    effective_conductivity_m_s: Option<f64>,
    matric_potential_m: Option<f64>,
    depression_storage_capacity_m: f64,
}

#[derive(Clone)]
struct DirectProductionEvapotranspirationAuthority {
    leaf_area_index: f64,
    canopy_cover_fraction: f64,
    residue_interception_m: f64,
    root_depth_m: f64,
    plant_tolerance: f64,
    priestley_taylor: DirectProductionPriestleyTaylorAuthority,
    pmet: Option<DirectProductionPmetAuthority>,
}

#[derive(Clone)]
struct DirectProductionPriestleyTaylorAuthority {
    salb: f64,
}

#[derive(Clone)]
struct DirectProductionPmetAuthority {
    kcb: f64,
    rawp: f64,
    canhgt: f64,
    radpot_ly: Option<f64>,
    solthk_m: Vec<Option<f64>>,
}

#[derive(Clone)]
struct DirectProductionGrowthAuthority {
    active: bool,
    rotation_years: usize,
    rotation_repeats: usize,
    slots: Vec<DirectProductionGrowthSlotAuthority>,
    monthly_temperature_max_c: [f64; 12],
    monthly_temperature_min_c: [f64; 12],
    soil_depth_m: f64,
}

#[derive(Clone)]
struct DirectProductionGrowthSlotAuthority {
    ofe_index: usize,
    year_in_rotation: usize,
    rotation_index: usize,
    crops: Vec<DirectProductionGrowthCropAuthority>,
}

#[derive(Clone, Copy)]
struct DirectProductionGrowthCropAuthority {
    schedule_imngmt: u8,
    imngmt: u8,
    jdharv: u16,
    jdplt: u16,
    jdstop: u16,
    btemp: f64,
    otemp: f64,
    gddmax: f64,
    dlai: f64,
    dropfc: f64,
    decfct: f64,
    spriod: f64,
    bb: f64,
    beinp: f64,
    extnct: f64,
    hi: f64,
    xmxlai: f64,
    rsr: f64,
    rtmmax: f64,
    rdmax: f64,
}

#[derive(Clone)]
struct DirectProductionErosionAuthority {
    wave2_enabled: bool,
    erosion_inputs: DirectErosionInputs,
}

#[derive(Clone, Copy)]
struct DirectProductionWinterHourlyGeometry {
    avg_slope: f64,
    azimuth: f64,
}

impl DirectProductionWinterHourlyGeometry {
    fn from_climate_context_surface(
        climate_context_surface: &HillslopeWritebackSurface,
    ) -> Result<Self, HillslopeCliError> {
        Ok(Self {
            avg_slope: direct_publication_required_positive_scalar(
                climate_context_surface,
                "avgslp",
            )?,
            azimuth: require_runtime_surface_scalar(climate_context_surface, "azm")?,
        })
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
struct DirectProductionSnowFrostAuthority {
    snow_runtime_swe_m: f64,
    snow_runtime_depth_m: f64,
    snow_runtime_density_kg_m3: f64,
    snow_runtime_settle_day_count: f64,
    snow_controls_projected: bool,
    snow_density_model: openwepp_hillslope_orchestrator::SnowDensityModel,
    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel,
    snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel,
    snow_rst_c: f64,
    snow_newsnw_kg_m3: f64,
    snow_ssd_kg_m3: f64,
    frost_typed_authority: Option<DirectProductionFrostTypedAuthority>,
    frost_layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
    frost_file_present: bool,
    frost_wint_red_enabled: bool,
    frost_runtime_depth_m: f64,
    frost_runtime_frozen_water_m: f64,
    frost_active: bool,
}

#[derive(Clone)]
struct DirectProductionFrostTypedAuthority {
    controls: DirectFrostControlInputs,
    layer_bulk_density_kg_m3: Vec<f64>,
    soil_conductivity_m_s: Option<f64>,
    residue_depth_m: f64,
    theta_residual: f64,
    theta_field_capacity: f64,
    albedo: f64,
    canopy_height_m: f64,
    random_roughness_m: f64,
    monthly_max_c: [f64; 12],
    monthly_min_c: [f64; 12],
}

struct DirectProductionFrostDayContext {
    compute_inputs: DirectWinterFrostComputeInputs,
    frozen_infiltration_capacity_m_s: f64,
    storage_liquid_delta_m: Option<f64>,
    layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
    hydrology_layers: Vec<DirectSubsurfaceLayerState>,
}

struct DirectProductionFrostTypedComputeContext<'a> {
    lane_index: usize,
    lane: &'a openwepp_hillslope_orchestrator::DirectLaneFrame,
    day: &'a ClimateDayProjection,
    forcing: &'a HillslopeDirectClimateDayForcing,
    snow_lane_state: DirectSnowLaneState,
    frost_lane_state: &'a DirectFrostLaneState,
    typed_authority: &'a DirectProductionFrostTypedAuthority,
    hourly: [DirectFrostHourlyForcing;
        openwepp_hillslope_orchestrator::DIRECT_WINTER_HOURLY_FORCING_COUNT],
}

fn direct_production_frost_storage_liquid_delta(
    frost_outcome: &DirectWinterFrostPartitionOutcome,
) -> Option<f64> {
    const MATERIAL_FROST_THRESHOLD_M: f64 = 1.0e-12;
    if frost_outcome.frwatc_net_liquid_delta_m <= MATERIAL_FROST_THRESHOLD_M {
        return None;
    }
    if frost_outcome.frost_depth_after_m > MATERIAL_FROST_THRESHOLD_M
        || frost_outcome.frozen_water_after_m > MATERIAL_FROST_THRESHOLD_M
    {
        return None;
    }
    Some(frost_outcome.frwatc_net_liquid_delta_m)
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy)]
struct DirectProductionEvappmSeed {
    et_demand_m: f64,
    soil_evaporation_m: f64,
    plant_transpiration_m: f64,
    soil_evaporation_storage_return_m: f64,
}

impl<'a> DirectProductionDayInputBuilder<'a> {
    fn new(
        climate_request: &'a HillslopeClimateRuntimeRequest,
        climate_span: &'a ClimateRunSpanSummary,
        seed_surfaces: &[HillslopeWritebackSurface],
        climate_context_surface: &HillslopeWritebackSurface,
        execution_lane: ExecutionLane,
    ) -> Result<Self, HillslopeCliError> {
        if seed_surfaces.is_empty() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production requires at least one lane seed authority"
                ),
            });
        }
        let lane_authority = seed_surfaces
            .iter()
            .map(|seed_surface| {
                let day_zero_seed_surface = direct_publication_day_zero_seed_surface(
                    climate_request,
                    climate_span,
                    seed_surface,
                    climate_context_surface,
                    execution_lane,
                )?;
                Self::build_lane_authority(&day_zero_seed_surface)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let sturm_climate_class = direct_production_sturm_climate_class_for_density_candidate(
            climate_request,
            climate_span,
            &lane_authority,
        )?;
        let winter_hourly_geometry =
            DirectProductionWinterHourlyGeometry::from_climate_context_surface(
                seed_surfaces.last().ok_or_else(|| {
                    HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "direct_publication_frame",
                        detail: format!(
                            "{SIMOUT_GUARD_ID} direct production requires outlet seed authority for winter hourly geometry"
                        ),
                    }
                })?,
            )?;
        Ok(Self {
            climate_request,
            climate_span,
            lane_authority,
            winter_hourly_geometry,
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
        let day = self.climate_span.days.get(day_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production day index {} exceeds climate span {}",
                    day_index + 1,
                    self.climate_span.days.len()
                ),
            }
        })?;
        direct_publication_validate_day(day)?;
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
        let lane = frame
            .lanes
            .get(lane_index)
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production lane index {} exceeds frame lane count {}",
                    lane_index + 1,
                    frame.lanes.len()
                ),
        })?;
        let authority = self.lane_authority(lane_index)?;
        let precipitation_m = forcing.prcp_m;
        let mut hyetograph = direct_production_hyetograph(&forcing)?;
        let rainfall_input_m = direct_publication_hyetograph_rainfall_m(&hyetograph)?;
        let snow_lane_state = authority.snow_frost.current_snow_lane_state(lane);
        let growth_state_before = *lane.plant_growth_state;
        let pre_growth_evapotranspiration_compute_inputs =
            authority.evapotranspiration.inputs_with_growth_surface(
                day,
                &forcing,
                lane.evapotranspiration_stage_state.as_deref().copied(),
                &lane.subsurface_layers,
                self.climate_request,
                growth_state_before,
            )?;
        let (annual_growth_inputs, perennial_growth_inputs) = authority.growth.inputs(
            day,
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
            snow_lane_state,
            growth_state_for_publication.canopy_cover_fraction,
            self.sturm_climate_class,
            sturm_day_of_year,
            self.winter_hourly_geometry,
        )?;
        maybe_write_r7h_direct_production_snow_trace(
            day_index,
            lane_index,
            rainfall_input_m,
            snow_lane_state,
            authority.snow_frost.snow_melt_model,
            authority.snow_frost.snow_phase_model,
            snow_liquid,
        )?;
        let frost_context = authority.snow_frost.frost_day_context(
            self.climate_request,
            day_index,
            day,
            lane_index,
            lane,
            &forcing,
            snow_lane_state,
            self.winter_hourly_geometry,
            rainfall_input_m > 1.0e-12 || snow_liquid.routed_melt_m > 1.0e-12,
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
            DirectPublicationDayInput::calendar_only(direct_publication_calendar_day(day)?);
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
            snow_state_projected: authority.snow_frost.snow_state_projected(snow_lane_state),
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
        day_input.annual_growth_inputs = Some(annual_growth_inputs);
        day_input.perennial_growth_inputs = Some(perennial_growth_inputs);
        let mut hydrology_projection_inputs =
            authority.hydrology_projection_inputs(hydrology_layers);
        hydrology_projection_inputs.snow_water_m = snow_liquid.runtime_swe_after_m;
        day_input.hydrology_projection_inputs = Some(hydrology_projection_inputs);
        let erosion_active = authority.erosion.wave2_enabled
            && direct_publication_hyetograph_rainfall_m(
                day_input
                    .peak_runoff_inputs
                    .as_ref()
                    .map_or(&[][..], |inputs| inputs.hyetograph.as_slice()),
            )? >= DIRECT_PUBLICATION_EROSION_MIN_POST_INTERCEPTION_RAINFALL_M;
        day_input.erosion_producer_required = erosion_active;
        if erosion_active {
            day_input.erosion_inputs = Some(authority.erosion.erosion_inputs.clone());
        }
        if let Some(frost_context) = frost_context {
            day_input.winter_frost_compute_inputs = Some(frost_context.compute_inputs);
            day_input.frost_storage_liquid_delta_m = frost_context.storage_liquid_delta_m;
            day_input.frost_layer_carry_projection = frost_context.layer_carry_projection;
        }
        day_input.frost_runtime_carry =
            direct_publication_frost_runtime_carry_from_lane_state(&lane.winter_column.frost);
        Ok(day_input)
    }

    fn build_lane_authority(
        seed_surface: &HillslopeWritebackSurface,
    ) -> Result<DirectProductionLaneDayInputAuthority, HillslopeCliError> {
        let layers = direct_publication_layer_states(seed_surface)?;
        let percolation = direct_publication_percolation_inputs(seed_surface, 0.0)?;
        let subsurface = direct_publication_subsurface_inputs(seed_surface)?;
        Ok(DirectProductionLaneDayInputAuthority {
            peak_runoff: DirectProductionPeakRunoffAuthority {
                irrigation_rate_m_s: direct_publication_optional_nonnegative_scalar(
                    seed_surface,
                    &["irrigation.runtime_rate_m_per_s"],
                )?
                .unwrap_or(0.0),
                efflen_m: require_runtime_surface_scalar(seed_surface, "efflen")?,
                ealpha: require_runtime_surface_scalar(seed_surface, "ealpha")?,
                exponent_m: require_runtime_surface_scalar(seed_surface, "m")?,
            },
            percolation,
            subsurface,
            infiltration: DirectProductionInfiltrationAuthority {
                effective_conductivity_m_s: direct_publication_optional_nonnegative_scalar(
                    seed_surface,
                    &[
                        "wb14_effective_conductivity_m_s",
                        "frost.runtime_infcap_frz",
                        "wb14_soil_conductivity_m_s",
                    ],
                )?,
                matric_potential_m: direct_publication_optional_nonnegative_scalar(
                    seed_surface,
                    &["wb14_matric_potential_m"],
                )?,
                depression_storage_capacity_m: direct_publication_optional_nonnegative_scalar(
                    seed_surface,
                    &[
                        "wb14_depression_storage_capacity_m",
                        "wb12_depression_storage_capacity_m",
                    ],
                )?
                .unwrap_or(0.0),
            },
            evapotranspiration: DirectProductionEvapotranspirationAuthority::from_seed(
                seed_surface,
                layers.len(),
            )?,
            growth: DirectProductionGrowthAuthority::from_seed(seed_surface)?,
            hydrology_projection: direct_publication_profile_inputs(seed_surface)?,
            erosion: DirectProductionErosionAuthority::from_seed(seed_surface)?,
            snow_frost: DirectProductionSnowFrostAuthority::from_seed(seed_surface)?,
        })
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
    snow_lane_state: openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel,
    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel,
    snow_liquid: openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
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
\"runtime_settle_day_count_after\":{}}}",
        direct_production_trace_number(hyetograph_rainfall_m),
        direct_production_trace_number(snow_lane_state.runtime_swe_m),
        direct_production_trace_number(snow_lane_state.runtime_depth_m),
        direct_production_trace_number(snow_lane_state.runtime_density_kg_m3),
        direct_production_trace_number(snow_lane_state.runtime_settle_day_count),
        direct_production_trace_number(snow_lane_state.liquid_water_retained_m),
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
    );
    let line = format!("{line}\n");
    std::io::Write::write_all(&mut file, line.as_bytes()).map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_snow_trace",
        detail: format!(
            "{SIMOUT_GUARD_ID} failed writing direct production snow trace {}: {error}",
            std::path::PathBuf::from(&path).display()
        ),
    })
}

fn maybe_write_r7h_direct_production_wb15_trace(
    day_index: usize,
    lane_index: usize,
    growth_state_before: DirectGrowthStateSurface,
    growth_state_for_publication: DirectGrowthStateSurface,
    post_winter_rain_m: f64,
    interception_state: openwepp_hillslope_orchestrator::DirectCanopyInterceptionState,
) -> Result<(), HillslopeCliError> {
    let Some(path) = std::env::var_os("OPENWEPP_R7H_WB15_TRACE_PATH") else {
        return Ok(());
    };
    if path.is_empty() {
        return Ok(());
    }
    if let Some(filter_day_index) = direct_production_trace_env_usize(
        "OPENWEPP_R7H_WB15_TRACE_DAY_INDEX",
    ) && filter_day_index != day_index
    {
        return Ok(());
    }
    if let Some(filter_lane_index) = direct_production_trace_env_usize(
        "OPENWEPP_R7H_WB15_TRACE_LANE_INDEX",
    ) && filter_lane_index != lane_index
    {
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

fn direct_production_trace_env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.trim().parse::<usize>().ok()
}

fn snowdensity1015_default_snow_density_model(
) -> Result<openwepp_hillslope_orchestrator::SnowDensityModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY09_DENSITY_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "physics_bulk_density_compaction_v1" => Ok(
                openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkDensityCompactionV1,
            ),
            "legacy_wepp" => Ok(openwepp_hillslope_orchestrator::SnowDensityModel::LegacyWepp),
            "physics_bulk_shallow_guard_v1" => Ok(
                openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkShallowGuardV1,
            ),
            "physics_bulk_climate_class_density_v1" => Ok(
                openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkClimateClassDensityV1,
            ),
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_density_model",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {SNOWDENSITY09_DENSITY_MODEL_ENV} must be legacy_wepp, physics_bulk_density_compaction_v1, physics_bulk_shallow_guard_v1, or physics_bulk_climate_class_density_v1, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => {
            Ok(openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkDensityCompactionV1)
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
        Ok(value) => match value.trim() {
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
        },
        Err(std::env::VarError::NotPresent) => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe)
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
        Ok(value) => match value.trim() {
            "" | "coe_liquid_holding_capacity_v1" => {
                Ok(openwepp_hillslope_orchestrator::SnowMeltModel::CoeLiquidHoldingCapacityV1)
            }
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
        },
        Err(std::env::VarError::NotPresent) => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::CoeLiquidHoldingCapacityV1)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY1038_MELT_MODEL_ENV} must be UTF-8"),
        }),
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

impl DirectProductionLaneDayInputAuthority {
    fn percolation_inputs(
        &self,
        lane_index: usize,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
        layers: &[DirectSubsurfaceLayerState],
    ) -> Result<DirectPercolationInputs, HillslopeCliError> {
        direct_production_validate_layers(lane_index, layers)?;
        let mut inputs = self.percolation.clone();
        inputs.soil_water_initial_m = direct_production_lane_soil_water(lane, lane_index)?;
        inputs.layers.clear();
        inputs.layers.extend_from_slice(layers);
        Ok(inputs)
    }

    fn subsurface_inputs(
        &self,
        lane_index: usize,
        layers: &[DirectSubsurfaceLayerState],
    ) -> Result<DirectSubsurfaceComputeInputs, HillslopeCliError> {
        direct_production_validate_layers(lane_index, layers)?;
        let mut inputs = self.subsurface.clone();
        inputs.soil_depth_m = layers.iter().map(|layer| layer.depth_m).sum::<f64>();
        inputs.layers = layers.iter().cloned().map(Into::into).collect();
        Ok(inputs)
    }

    fn hydrology_projection_inputs(
        &self,
        layers: &[DirectSubsurfaceLayerState],
    ) -> DirectHydrologyProjectionInputs {
        let mut inputs = self.hydrology_projection;
        inputs.frozen_soil_water_m = layers.iter().map(|layer| layer.frozen_water_m).sum();
        inputs.frost_depth_m = direct_production_frost_depth_m(layers);
        inputs
    }
}

impl DirectProductionPeakRunoffAuthority {
    fn inputs(&self, hyetograph: Vec<DirectWb14HyetographInterval>) -> DirectPeakRunoffInputs {
        DirectPeakRunoffInputs {
            hyetograph,
            irrigation_rate_m_s: self.irrigation_rate_m_s,
            efflen_m: self.efflen_m,
            ealpha: self.ealpha,
            exponent_m: self.exponent_m,
        }
    }
}

impl DirectProductionInfiltrationAuthority {
    #[allow(clippy::too_many_arguments)]
    fn inputs(
        &self,
        lane_index: usize,
        layers: &[DirectSubsurfaceLayerState],
        hyetograph: Vec<DirectWb14HyetographInterval>,
        frost_infcap_m_s: Option<f64>,
    ) -> Result<DirectInfiltrationDepressionInputs, HillslopeCliError> {
        direct_production_validate_layers(lane_index, layers)?;
        let effective_conductivity_m_s = frost_infcap_m_s
            .filter(|value| *value > 0.0)
            .or(self.effective_conductivity_m_s)
            .filter(|value| *value > 0.0)
            .or_else(|| layers.first().map(|layer| layer.conductivity_m_s))
            .ok_or_else(|| {
                direct_production_executor_blocked(
                    "direct production WB14 infiltration requires layer conductivity",
                )
            })?;
        let matric_potential_m = self.matric_potential_m.unwrap_or_else(|| {
            let first_layer = &layers[0];
            first_layer.depth_m * (first_layer.field_capacity_theta - first_layer.residual_theta).max(0.0)
        });
        let storage_capacity_m = direct_publication_wb14_top_storage_capacity(layers)?;
        Ok(DirectInfiltrationDepressionInputs {
            cumulative_infiltration_handoff_m: 0.0,
            depression_storage_delta_handoff_m: 0.0,
            producer_inputs: Some(DirectWb14InfiltrationProducerInputs {
                hyetograph,
                effective_conductivity_m_s,
                matric_potential_m,
                storage_capacity_m,
                depression_storage_capacity_m: self.depression_storage_capacity_m,
            }),
        })
    }
}

impl DirectProductionEvapotranspirationAuthority {
    fn from_seed(
        seed_surface: &HillslopeWritebackSurface,
        layer_count: usize,
    ) -> Result<Self, HillslopeCliError> {
        let iflget =
            runtime_surface_symbol_value(seed_surface, "pmetpara.mode.iflget").unwrap_or(1.0);
        if !iflget.is_finite() {
            return Err(direct_production_executor_blocked(format!(
                "pmetpara.mode.iflget must be finite when present, observed {iflget}"
            )));
        }
        let pmet = if (iflget - 1.0).abs() <= 1.0e-12 {
            None
        } else {
            Some(DirectProductionPmetAuthority {
                kcb: require_runtime_surface_scalar(seed_surface, "pmetpara.selected.kcb")?,
                rawp: require_runtime_surface_scalar(seed_surface, "pmetpara.selected.rawp")?,
                canhgt: require_runtime_surface_scalar(seed_surface, "canhgt")?,
                radpot_ly: runtime_surface_symbol_value(seed_surface, "radpot"),
                solthk_m: (1..=layer_count)
                    .map(|layer_index| {
                        runtime_surface_symbol_value(
                            seed_surface,
                            format!("wb19_solthk_{layer_index:04}").as_str(),
                        )
                    })
                    .collect(),
            })
        };
        Ok(Self {
            leaf_area_index: require_runtime_surface_scalar(seed_surface, "lai")?,
            canopy_cover_fraction: require_runtime_surface_scalar(seed_surface, "cancov")?,
            residue_interception_m: require_runtime_surface_scalar(
                seed_surface,
                "wb17_residue_interception",
            )?,
            root_depth_m: require_runtime_surface_scalar(seed_surface, "rtd")?,
            plant_tolerance: require_preferred_or_legacy_runtime_surface_scalar(
                seed_surface,
                "swu_effective_pltol",
                "pltol",
            )?,
            priestley_taylor: DirectProductionPriestleyTaylorAuthority {
                salb: require_runtime_surface_scalar(seed_surface, "salb")?,
            },
            pmet,
        })
    }
    #[allow(clippy::too_many_arguments)]
    fn inputs(
        &self,
        day: &ClimateDayProjection,
        forcing: &HillslopeDirectClimateDayForcing,
        stage_state: Option<DirectEvapotranspirationStageState>,
        layers: &[DirectSubsurfaceLayerState],
        climate_request: &HillslopeClimateRuntimeRequest,
    ) -> Result<DirectEvapotranspirationComputeInputs, HillslopeCliError> {
        let (et_demand_m, pmet) = if let Some(pmet_authority) = &self.pmet {
            let seed = pmet_authority.compute_seed(day, forcing, layers, self, climate_request)?;
            (
                seed.et_demand_m,
                Some(DirectEvapotranspirationPmetInputs {
                    soil_evaporation_m: seed.soil_evaporation_m,
                    plant_transpiration_m: seed.plant_transpiration_m,
                    soil_evaporation_storage_return_m: seed.soil_evaporation_storage_return_m,
                }),
            )
        } else {
            (
                self.priestley_taylor
                    .compute_demand(forcing, self.leaf_area_index, self.canopy_cover_fraction)?,
                None,
            )
        };
        Ok(DirectEvapotranspirationComputeInputs {
            et_demand_m,
            leaf_area_index: self.leaf_area_index,
            canopy_cover_fraction: self.canopy_cover_fraction,
            residue_interception_m: self.residue_interception_m,
            same_pass_infiltration_m: 0.0,
            outside_water_depth_m: 0.0,
            root_depth_m: self.root_depth_m,
            plant_tolerance: self.plant_tolerance,
            growth_context_required: false,
            stage_state: if pmet.is_some() { None } else { stage_state },
            pmet,
            pmet_compute: None,
        })
    }

    fn inputs_with_growth_surface(
        &self,
        day: &ClimateDayProjection,
        forcing: &HillslopeDirectClimateDayForcing,
        stage_state: Option<DirectEvapotranspirationStageState>,
        layers: &[DirectSubsurfaceLayerState],
        climate_request: &HillslopeClimateRuntimeRequest,
        growth_surface: DirectGrowthStateSurface,
    ) -> Result<DirectEvapotranspirationComputeInputs, HillslopeCliError> {
        let mut dynamic = self.clone();
        dynamic.apply_growth_surface(growth_surface);
        dynamic.inputs(day, forcing, stage_state, layers, climate_request)
    }

    fn apply_growth_surface(&mut self, growth_surface: DirectGrowthStateSurface) {
        self.leaf_area_index = growth_surface.leaf_area_index;
        self.canopy_cover_fraction = growth_surface.canopy_cover_fraction;
        self.root_depth_m = growth_surface.root_depth_m;
    }
}

impl DirectProductionPriestleyTaylorAuthority {
    fn compute_demand(
        &self,
        forcing: &HillslopeDirectClimateDayForcing,
        leaf_area_index: f64,
        canopy_cover_fraction: f64,
    ) -> Result<f64, HillslopeCliError> {
        if forcing.rad_ly < 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "rad must be >= 0.0 for direct production ET demand, observed {}",
                forcing.rad_ly
            )));
        }
        if !(0.0..=1.0).contains(&self.salb) {
            return Err(direct_production_executor_blocked(format!(
                "salb must be within [0,1] for direct production ET demand, observed {}",
                self.salb
            )));
        }
        let tave = 0.5 * (forcing.tmax_c + forcing.tmin_c);
        let tk = tave + 273.0;
        if tk <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "derived tk must be > 0.0 for direct production ET demand, observed {tk}"
            )));
        }
        let delta = (21.255 - 5304.0 / tk).exp() * 5304.0 / (tk * tk);
        let gamma = delta / (delta + 0.68);
        let eaj = (-0.5 * (canopy_cover_fraction + 0.1)).exp();
        let alb = if leaf_area_index > 0.0 {
            0.23 * (1.0 - eaj) + self.salb * eaj
        } else {
            self.salb
        };
        let demand_m = (0.00128 * ((forcing.rad_ly * (1.0 - alb)) / 58.3) * gamma).max(0.0);
        if !demand_m.is_finite() {
            return Err(direct_production_executor_blocked(format!(
                "derived direct production ET demand is non-finite ({demand_m})"
            )));
        }
        Ok(demand_m)
    }
}

impl DirectProductionPmetAuthority {
    #[allow(clippy::manual_midpoint, clippy::similar_names, clippy::too_many_lines)]
    fn compute_seed(
        &self,
        day: &ClimateDayProjection,
        forcing: &HillslopeDirectClimateDayForcing,
        layers: &[DirectSubsurfaceLayerState],
        et: &DirectProductionEvapotranspirationAuthority,
        climate_request: &HillslopeClimateRuntimeRequest,
    ) -> Result<DirectProductionEvappmSeed, HillslopeCliError> {
        direct_production_validate_layers(0, layers)?;
        if forcing.rad_ly < 0.0 || forcing.vwind_m_s < 0.0 {
            return Err(direct_production_executor_blocked(
                "direct production PMET requires nonnegative rad and vwind",
            ));
        }
        if self.canhgt < 0.0 || et.leaf_area_index < 0.0 || et.root_depth_m < 0.0 {
            return Err(direct_production_executor_blocked(
                "direct production PMET canopy and root controls must be nonnegative",
            ));
        }
        let tave = 0.5 * (forcing.tmax_c + forcing.tmin_c);
        let ed = saturation_vapor_pressure_kpa(forcing.tdpt_c);
        let emaxt = saturation_vapor_pressure_kpa(forcing.tmax_c);
        let emint = saturation_vapor_pressure_kpa(forcing.tmin_c);
        let ee = 0.5 * (emaxt + emint);
        if emaxt <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "derived emaxt must be > 0.0 for direct production PMET, observed {emaxt}"
            )));
        }
        let radpot = self.radpot_ly.unwrap_or_else(|| {
            legacy_sunmap_horizontal_radpot_ly(
                climate_request.direct_latitude_degrees(),
                f64::from(day.julian_day),
            )
        });
        if radpot <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "radpot must be > 0.0 for direct production PMET, observed {radpot}"
            )));
        }
        let ra = forcing.rad_ly / 23.9;
        let rso = radpot / 23.9;
        let rbo = (0.34 - 0.14 * ed.sqrt())
            * 4.9e-9
            * (((forcing.tmax_c + 273.2).powi(4) + (forcing.tmin_c + 273.2).powi(4)) / 2.0)
            * (1.35 * (ra / rso) - 0.35);
        let rn_mj_m2 = ra * 0.77 - rbo;
        let fwv_m_s = forcing.vwind_m_s * 4.87 / (67.8_f64.mul_add(10.0, -5.42)).ln();
        let dlt = 4098.0 / ((tave + 237.3) * (tave + 237.3))
            * saturation_vapor_pressure_kpa(tave);
        let pressure_base = 1.0 - 0.0065 * climate_request.direct_elevation_m() / 293.0;
        if pressure_base <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "legacy pressure base must be > 0.0 for direct production PMET, observed {pressure_base}"
            )));
        }
        let pb = 101.3 * pressure_base.powf(5.26);
        let gma = 0.000_665 * pb;
        let denominator = dlt + gma * (1.0 + 0.34 * fwv_m_s);
        if denominator <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "direct production PMET etorc denominator must be > 0.0, observed {denominator}"
            )));
        }
        let etorc_mm =
            (0.408 * dlt * rn_mj_m2 + gma * (900.0 / (tave + 273.0)) * (ee - ed) * fwv_m_s)
                / denominator;
        let rhd_pct = ed / emaxt * 100.0;
        let height_factor = (self.canhgt / 3.0).powf(0.3);
        let kcbadj = if et.leaf_area_index > 0.0 && et.root_depth_m > 0.0 {
            self.kcb + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor
        } else {
            0.0
        };
        let kcbcon = kcbadj * (1.0 - (-0.45 * et.leaf_area_index).exp());
        let etke = if kcbadj > 0.0 {
            kcbadj * (-0.45 * et.leaf_area_index).exp()
        } else {
            1.2
        };

        let profile_depth_m = direct_production_profile_depth_m(layers)?;
        let epdp_m = 0.1_f64.min(profile_depth_m);
        let (tew_mm, rew_mm, wfevp_base_mm) =
            self.evaporation_storage_terms(layers, epdp_m)?;
        let wfevp_mm = wfevp_base_mm + et.residue_interception_m * 1_000.0;
        let etkr = if (tew_mm - wfevp_mm) <= rew_mm {
            1.0
        } else {
            let denominator = tew_mm - rew_mm;
            if denominator <= 0.0 {
                1.0
            } else {
                (wfevp_mm / denominator).powi(2)
            }
        };
        let tpdp_m = et.root_depth_m.min(profile_depth_m);
        let (taw_mm, wftrp_mm) =
            self.transpiration_storage_terms(layers, tpdp_m, wfevp_mm)?;
        let etcsc = kcbadj * etorc_mm;
        let rawpaj = self.rawp + 0.04 * (5.0 - etcsc);
        let raw_mm = rawpaj * taw_mm;
        let etksden = taw_mm - raw_mm;
        let etks = if etksden <= 0.0 || (taw_mm - wftrp_mm) <= raw_mm {
            1.0
        } else {
            wftrp_mm / etksden
        };
        let potes_m = etorc_mm * etke * 0.001;
        let es_raw_m = if potes_m > et.residue_interception_m {
            let bpotes_m = potes_m - et.residue_interception_m;
            let eaj = (-0.5 * (et.canopy_cover_fraction + 0.1)).exp();
            let kcmax = 1.2 + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor;
            let kecon = (etke * etkr).min(eaj * kcmax);
            kecon * bpotes_m / etke + et.residue_interception_m
        } else {
            potes_m
        };
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
            if !value.is_finite() {
                return Err(direct_production_executor_blocked(format!(
                    "derived {name} must be finite, observed {value}"
                )));
            }
        }
        Ok(DirectProductionEvappmSeed {
            et_demand_m: plant_transpiration_m,
            soil_evaporation_m,
            plant_transpiration_m,
            soil_evaporation_storage_return_m,
        })
    }

    fn evaporation_storage_terms(
        &self,
        layers: &[DirectSubsurfaceLayerState],
        epdp_m: f64,
    ) -> Result<(f64, f64, f64), HillslopeCliError> {
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
                return Err(direct_production_executor_blocked(format!(
                    "wb19_thetdr_{layer_index:04} must be <= wb19_thetfc_{layer_index:04}"
                )));
            }
            if layer_fraction > 0.0 {
                tew_mm +=
                    (layer.field_capacity_theta - 0.5 * layer.residual_theta)
                        * layer.depth_m
                        * 1_000.0
                        * layer_fraction;
                rew_mm +=
                    (layer.field_capacity_theta - layer.residual_theta)
                        * layer.depth_m
                        * 1_000.0
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

    fn transpiration_storage_terms(
        &self,
        layers: &[DirectSubsurfaceLayerState],
        tpdp_m: f64,
        wfevp_mm: f64,
    ) -> Result<(f64, f64), HillslopeCliError> {
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
                taw_mm += (layer.field_capacity_theta - layer.residual_theta)
                    * layer.depth_m
                    * 1_000.0;
                wftrp_mm += layer.theta_m * 1_000.0;
            } else if cumulative_depth_m < tpdp_m {
                let layer_span_m = solthk - cumulative_depth_m;
                if layer_span_m <= 0.0 {
                    return Err(direct_production_executor_blocked(format!(
                        "wb19_solthk_{layer_index:04} must increase with depth for direct production PMET"
                    )));
                }
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
    ) -> Result<f64, HillslopeCliError> {
        let solthk = self
            .solthk_m
            .get(layer_index - 1)
            .and_then(|value| *value)
            .unwrap_or(cumulative_depth_m + depth_m);
        if solthk <= cumulative_depth_m {
            return Err(direct_production_executor_blocked(format!(
                "wb19_solthk_{layer_index:04} must increase with depth for direct production PMET"
            )));
        }
        Ok(solthk)
    }
}

impl DirectProductionGrowthAuthority {
    fn from_seed(seed_surface: &HillslopeWritebackSurface) -> Result<Self, HillslopeCliError> {
        let Some(slot_count_value) =
            runtime_surface_symbol_value(seed_surface, "pl_schedule_slot_count")
        else {
            return Ok(Self::inactive());
        };
        let slot_count = direct_growth_integral_usize(
            "pl_schedule_slot_count",
            slot_count_value,
            1,
            usize::MAX,
        )?;
        let rotation_years = direct_growth_required_integral_usize(
            seed_surface,
            "pl_schedule_rotation_years",
            1,
            usize::MAX,
        )?;
        let rotation_repeats = direct_growth_required_integral_usize(
            seed_surface,
            "pl_schedule_rotation_repeats",
            1,
            usize::MAX,
        )?;
        let mut slots = Vec::with_capacity(slot_count);
        for slot_index in 1..=slot_count {
            slots.push(DirectProductionGrowthSlotAuthority::from_seed(
                seed_surface,
                slot_index,
            )?);
        }
        Ok(Self {
            active: true,
            rotation_years,
            rotation_repeats,
            slots,
            monthly_temperature_max_c: direct_production_monthly_temperature(
                seed_surface,
                "obmaxt",
            )?,
            monthly_temperature_min_c: direct_production_monthly_temperature(
                seed_surface,
                "obmint",
            )?,
            soil_depth_m: direct_publication_required_positive_scalar(seed_surface, "solthk")?,
        })
    }

    fn inactive() -> Self {
        Self {
            active: false,
            rotation_years: 1,
            rotation_repeats: 1,
            slots: Vec::new(),
            monthly_temperature_max_c: [0.0; 12],
            monthly_temperature_min_c: [0.0; 12],
            soil_depth_m: 0.0,
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn inputs(
        &self,
        day: &ClimateDayProjection,
        simulation_year: i32,
        ofe_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        state_before: DirectGrowthStateSurface,
        water_stress: f64,
        et_inputs: &DirectEvapotranspirationComputeInputs,
    ) -> Result<(DirectGrowthInputs, DirectGrowthInputs), HillslopeCliError> {
        if !self.active {
            return Ok((DirectGrowthInputs::zero(), DirectGrowthInputs::zero()));
        }
        let runtime_year =
            direct_growth_i32_to_usize("simulation_year", simulation_year, 1, usize::MAX)?;
        let ofe_index = direct_growth_validate_usize("ofe_index", ofe_index, 1, usize::MAX)?;
        let runtime_day = direct_growth_u16_to_usize("day", day.julian_day, 1, 366)?;
        let Some(selection) = self.active_crop(runtime_year, runtime_day, ofe_index)? else {
            return Ok((DirectGrowthInputs::zero(), DirectGrowthInputs::zero()));
        };
        let runtime_day = direct_growth_usize_to_u16("day", runtime_day)?;
        let slot_index = direct_growth_usize_to_u16("slot_index", selection.slot_index)?;
        let crop_slot_index =
            direct_growth_usize_to_u16("crop_slot_index", selection.crop_slot_index)?;

        match selection.crop.imngmt {
            1 | 3 => {
                let active_action = if runtime_day == selection.crop.jdplt {
                    DirectGrowthAction::PlantingReset
                } else if runtime_day == selection.crop.jdharv {
                    DirectGrowthAction::HarvestReset
                } else {
                    DirectGrowthAction::None
                };
                Ok((
                    self.crop_inputs(
                        selection.crop,
                        DirectGrowthActiveContext::AnnualOrFallow {
                            active_slot_index: slot_index,
                            active_crop_slot_index: crop_slot_index,
                            runtime_day_of_year: runtime_day,
                        },
                        active_action,
                        forcing,
                        state_before,
                        water_stress,
                        et_inputs,
                    ),
                    DirectGrowthInputs::zero(),
                ))
            }
            2 => {
                let active_action = if selection.crop.jdplt != 0
                    && runtime_day == selection.crop.jdplt
                {
                    DirectGrowthAction::PlantingReset
                } else if selection.crop.jdstop != 0 && runtime_day == selection.crop.jdstop {
                    DirectGrowthAction::StopReset
                } else {
                    DirectGrowthAction::None
                };
                Ok((
                    DirectGrowthInputs::zero(),
                    self.crop_inputs(
                        selection.crop,
                        DirectGrowthActiveContext::Perennial {
                            active_slot_index: slot_index,
                            active_crop_slot_index: crop_slot_index,
                            runtime_day_of_year: runtime_day,
                        },
                        active_action,
                        forcing,
                        state_before,
                        water_stress,
                        et_inputs,
                    ),
                ))
            }
            _ => Err(direct_growth_failure(format!(
                "unsupported direct production growth management class {}",
                selection.crop.imngmt
            ))),
        }
    }

    fn active_crop(
        &self,
        runtime_year: usize,
        runtime_day: usize,
        ofe_index: usize,
    ) -> Result<Option<DirectGrowthActiveCropSelection<'_>>, HillslopeCliError> {
        let max_runtime_year = self.rotation_repeats.saturating_mul(self.rotation_years);
        if runtime_year > max_runtime_year {
            return Err(direct_growth_failure(format!(
                "year {runtime_year} exceeds direct growth rotation span {max_runtime_year}"
            )));
        }
        let rotation_index = ((runtime_year - 1) / self.rotation_years) + 1;
        let year_in_rotation = ((runtime_year - 1) % self.rotation_years) + 1;
        let year_slot_candidates = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| {
                slot.year_in_rotation == year_in_rotation && slot.rotation_index == rotation_index
            })
            .collect::<Vec<_>>();
        let mut slot_candidates = year_slot_candidates
            .iter()
            .copied()
            .filter(|(_, slot)| slot.ofe_index == ofe_index)
            .collect::<Vec<_>>();
        let (slot_offset, slot) = match slot_candidates.as_mut_slice() {
            [(slot_offset, slot)] => (*slot_offset, *slot),
            [] if year_slot_candidates.len() == 1 && year_slot_candidates[0].1.ofe_index == 1 => {
                year_slot_candidates[0]
            }
            [] => {
                return Err(direct_growth_failure(format!(
                    "missing direct growth PL slot for OFE {ofe_index} year_in_rotation={year_in_rotation}"
                )));
            }
            _ => {
                return Err(direct_growth_failure(format!(
                    "ambiguous direct growth PL slots for primary OFE year_in_rotation={year_in_rotation}"
                )));
            }
        };
        let mut crop_candidates = slot
            .crops
            .iter()
            .enumerate()
            .filter(|(_, crop)| crop.active_on_day(runtime_day))
            .collect::<Vec<_>>();
        let (crop_offset, crop) = match crop_candidates.as_mut_slice() {
            [(crop_offset, crop)] => (*crop_offset, *crop),
            [] => return Ok(None),
            _ => {
                return Err(direct_growth_failure(format!(
                    "ambiguous active direct growth crops for slot {} day {runtime_day}",
                    slot_offset + 1
                )));
            }
        };
        Ok(Some(DirectGrowthActiveCropSelection {
            slot_index: slot_offset + 1,
            crop_slot_index: crop_offset + 1,
            crop,
        }))
    }
    #[allow(clippy::too_many_arguments)]
    fn crop_inputs(
        &self,
        crop: &DirectProductionGrowthCropAuthority,
        active_context: DirectGrowthActiveContext,
        active_action: DirectGrowthAction,
        forcing: &HillslopeDirectClimateDayForcing,
        state_before: DirectGrowthStateSurface,
        water_stress: f64,
        et_inputs: &DirectEvapotranspirationComputeInputs,
    ) -> DirectGrowthInputs {
        DirectGrowthInputs {
            active_context,
            active_action,
            state_before,
            planting_day: crop.jdplt,
            harvest_day: crop.jdharv,
            stop_day: crop.jdstop,
            water_stress,
            temperature_max_c: forcing.tmax_c,
            temperature_min_c: forcing.tmin_c,
            radiation_mj_m2: forcing.rad_ly,
            monthly_temperature_max_c: self.monthly_temperature_max_c,
            monthly_temperature_min_c: self.monthly_temperature_min_c,
            soil_depth_m: self.soil_depth_m,
            btemp: crop.btemp,
            otemp: crop.otemp,
            gddmax: crop.gddmax,
            dlai: crop.dlai,
            dropfc: crop.dropfc,
            decfct: crop.decfct,
            spriod: crop.spriod,
            bb: crop.bb,
            beinp: crop.beinp,
            extnct: crop.extnct,
            hi: crop.hi,
            xmxlai: crop.xmxlai,
            rsr: crop.rsr,
            rtmmax: crop.rtmmax,
            rdmax: crop.rdmax,
            et_demand_m: et_inputs.et_demand_m,
            residue_interception_m: et_inputs.residue_interception_m,
            plant_tolerance: et_inputs.plant_tolerance,
        }
    }
}

impl DirectProductionGrowthSlotAuthority {
    fn from_seed(
        seed_surface: &HillslopeWritebackSurface,
        slot_index: usize,
    ) -> Result<Self, HillslopeCliError> {
        let crop_slots = direct_growth_required_integral_usize(
            seed_surface,
            &direct_growth_schedule_slot_symbol(slot_index, "crop_slots"),
            1,
            usize::MAX,
        )?;
        let mut crops = Vec::with_capacity(crop_slots);
        for crop_slot_index in 1..=crop_slots {
            crops.push(DirectProductionGrowthCropAuthority::from_seed(
                seed_surface,
                slot_index,
                crop_slot_index,
            )?);
        }
        Ok(Self {
            ofe_index: direct_growth_required_integral_usize(
                seed_surface,
                &direct_growth_schedule_slot_symbol(slot_index, "ofe_index"),
                1,
                usize::MAX,
            )?,
            year_in_rotation: direct_growth_required_integral_usize(
                seed_surface,
                &direct_growth_schedule_slot_symbol(slot_index, "year_in_rotation"),
                1,
                usize::MAX,
            )?,
            rotation_index: direct_growth_required_integral_usize(
                seed_surface,
                &direct_growth_schedule_slot_symbol(slot_index, "rotation_index"),
                1,
                usize::MAX,
            )?,
            crops,
        })
    }
}
impl DirectProductionGrowthCropAuthority {
    #[allow(clippy::too_many_lines)]
    fn from_seed(
        seed_surface: &HillslopeWritebackSurface,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> Result<Self, HillslopeCliError> {
        let schedule_imngmt = direct_growth_required_integral_u8(
            seed_surface,
            &direct_growth_schedule_slot_crop_symbol(slot_index, crop_slot_index, "imngmt"),
            1,
            3,
        )?;
        let imngmt = direct_growth_required_integral_u8(
            seed_surface,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "imngmt"),
            1,
            3,
        )?;
        let jdplt_min = usize::from(schedule_imngmt != 2);
        let jdplt = direct_growth_required_integral_u16(
            seed_surface,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "jdplt"),
            jdplt_min,
            366,
        )?;
        let jdharv = direct_growth_required_integral_u16(
            seed_surface,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "jdharv"),
            0,
            366,
        )?;
        let (jdstop, _mgtopt) = if schedule_imngmt == 2 {
            (
                direct_growth_required_integral_u16(
                    seed_surface,
                    &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "jdstop"),
                    0,
                    366,
                )?,
                direct_growth_required_integral_u8(
                    seed_surface,
                    &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "mgtopt"),
                    1,
                    3,
                )?,
            )
        } else {
            (0, 1)
        };
        Ok(Self {
            schedule_imngmt,
            imngmt,
            jdharv,
            jdplt,
            jdstop,
            btemp: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "btemp"),
            )?,
            otemp: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "otemp"),
            )?,
            gddmax: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "gddmax"),
            )?,
            dlai: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "dlai"),
            )?,
            dropfc: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "dropfc"),
            )?,
            decfct: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "decfct"),
            )?,
            spriod: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "spriod"),
            )?,
            bb: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "bb"),
            )?,
            beinp: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "beinp"),
            )?,
            extnct: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "extnct"),
            )?,
            hi: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "hi"),
            )?,
            xmxlai: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "xmxlai"),
            )?,
            rsr: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "rsr"),
            )?,
            rtmmax: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "rtmmax"),
            )?,
            rdmax: direct_growth_required_scalar(
                seed_surface,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "rdmax"),
            )?,
        })
    }

    fn active_on_day(self, runtime_day: usize) -> bool {
        if self.schedule_imngmt == 2 {
            if self.jdplt == 0 {
                self.jdstop == 0 || runtime_day <= usize::from(self.jdstop)
            } else if self.jdstop == 0 {
                direct_growth_day_is_within_window(
                    runtime_day,
                    usize::from(self.jdplt),
                    usize::from(self.jdharv.max(1)),
                )
            } else {
                direct_growth_day_is_within_window(
                    runtime_day,
                    usize::from(self.jdplt),
                    usize::from(self.jdstop),
                )
            }
        } else {
            direct_growth_day_is_within_window(
                runtime_day,
                usize::from(self.jdplt),
                usize::from(self.jdharv.max(1)),
            )
        }
    }
}

struct DirectGrowthActiveCropSelection<'a> {
    slot_index: usize,
    crop_slot_index: usize,
    crop: &'a DirectProductionGrowthCropAuthority,
}

fn direct_growth_state_surface_from_seed(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<DirectGrowthStateSurface, HillslopeCliError> {
    Ok(DirectGrowthStateSurface {
        sumgdd: require_runtime_surface_scalar(seed_surface, "sumgdd")?,
        live_biomass_kg_m2: require_runtime_surface_scalar(seed_surface, "vdmt")?,
        interception_live_biomass_kg_m2: direct_growth_interception_live_biomass_from_seed(
            seed_surface,
        )?,
        canopy_cover_fraction: require_runtime_surface_scalar(seed_surface, "cancov")?,
        leaf_area_index: require_runtime_surface_scalar(seed_surface, "lai")?,
        root_mass_kg_m2: require_runtime_surface_scalar(seed_surface, "rtmass")?,
        root_depth_m: require_runtime_surface_scalar(seed_surface, "rtd")?,
        harvest_index: require_runtime_surface_scalar(seed_surface, "hia")?,
    })
}

fn direct_growth_interception_live_biomass_from_seed(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    let vdmt = require_runtime_surface_scalar(seed_surface, "vdmt")?;
    direct_growth_nonnegative_scalar("vdmt", vdmt)?;
    let hia = require_runtime_surface_scalar(seed_surface, "hia")?;
    direct_growth_validate_harvest_index(hia)?;
    if let Some(tlive) = runtime_surface_symbol_value(seed_surface, "tlive") {
        direct_growth_nonnegative_scalar("tlive", tlive)?;
        return Ok(tlive);
    }
    Ok(vdmt)
}

fn direct_growth_interception_live_biomass_from_state(
    growth_state: DirectGrowthStateSurface,
) -> Result<f64, HillslopeCliError> {
    direct_growth_nonnegative_scalar("growth.vdmt", growth_state.live_biomass_kg_m2)?;
    direct_growth_validate_harvest_index(growth_state.harvest_index)?;
    if growth_state.interception_live_biomass_kg_m2 > 0.0 || growth_state.live_biomass_kg_m2 == 0.0
    {
        direct_growth_nonnegative_scalar(
            "growth.tlive",
            growth_state.interception_live_biomass_kg_m2,
        )?;
        Ok(growth_state.interception_live_biomass_kg_m2)
    } else {
        Ok(growth_state.live_biomass_kg_m2)
    }
}

fn direct_growth_validate_harvest_index(hia: f64) -> Result<(), HillslopeCliError> {
    if hia.is_finite() && (0.0..=1.0).contains(&hia) {
        Ok(())
    } else {
        Err(direct_production_executor_blocked(format!(
            "{SIMOUT_GUARD_ID} hia must be finite and within [0, 1] to construct direct WB15 tlive bridge, observed {hia}"
        )))
    }
}

fn direct_growth_nonnegative_scalar(symbol: &str, value: f64) -> Result<(), HillslopeCliError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(direct_production_executor_blocked(format!(
            "{SIMOUT_GUARD_ID} {symbol} must be finite and >= 0.0 for direct growth state, observed {value}"
        )))
    }
}

fn direct_growth_state_for_lane(
    seed_surface: &HillslopeWritebackSurface,
    day_index: usize,
    lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
) -> Result<DirectGrowthStateSurface, HillslopeCliError> {
    if day_index == 0 {
        direct_growth_state_surface_from_seed(seed_surface)
    } else {
        Ok(*lane.plant_growth_state)
    }
}

fn direct_growth_water_stress_for_lane(
    seed_surface: &HillslopeWritebackSurface,
    day_index: usize,
    lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
) -> Result<f64, HillslopeCliError> {
    if day_index == 0 {
        require_runtime_surface_scalar(seed_surface, "Ws")
    } else {
        Ok(lane.plant_water_stress)
    }
}

fn direct_growth_schedule_slot_symbol(slot_index: usize, root: &str) -> String {
    format!("pl_schedule_slot_{slot_index:04}_{root}")
}

fn direct_growth_schedule_slot_crop_symbol(
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> String {
    format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn direct_growth_slot_crop_symbol(
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> String {
    format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn direct_growth_day_is_within_window(
    runtime_day: usize,
    start_day: usize,
    end_day: usize,
) -> bool {
    if start_day <= end_day {
        runtime_day >= start_day && runtime_day <= end_day
    } else {
        runtime_day >= start_day || runtime_day <= end_day
    }
}

fn direct_growth_required_scalar(
    seed_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    require_runtime_surface_scalar(seed_surface, symbol)
}

fn direct_growth_required_integral_usize(
    seed_surface: &HillslopeWritebackSurface,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    let value = require_runtime_surface_scalar(seed_surface, symbol)?;
    direct_growth_integral_usize(symbol, value, min_allowed, max_allowed)
}

fn direct_growth_required_integral_u16(
    seed_surface: &HillslopeWritebackSurface,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<u16, HillslopeCliError> {
    let value = direct_growth_required_integral_usize(
        seed_surface,
        symbol,
        min_allowed,
        max_allowed,
    )?;
    direct_growth_usize_to_u16(symbol, value)
}

fn direct_growth_required_integral_u8(
    seed_surface: &HillslopeWritebackSurface,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<u8, HillslopeCliError> {
    let value = direct_growth_required_integral_usize(
        seed_surface,
        symbol,
        min_allowed,
        max_allowed,
    )?;
    u8::try_from(value).map_err(|_| {
        direct_growth_failure(format!("{symbol} value {value} exceeds u8 range"))
    })
}

fn direct_growth_integral_usize(
    symbol: &str,
    value: f64,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    if !value.is_finite() {
        return Err(direct_growth_failure(format!(
            "{symbol} must be finite for direct growth, observed {value}"
        )));
    }
    let rounded = value.round();
    if (value - rounded).abs() > 1.0e-12 || rounded < 0.0 {
        return Err(direct_growth_failure(format!(
            "{symbol} must be integral for direct growth, observed {value}"
        )));
    }
    let parsed = direct_growth_rounded_to_usize(symbol, rounded)?;
    if parsed < min_allowed || parsed > max_allowed {
        return Err(direct_growth_failure(format!(
            "{symbol} value {parsed} outside [{min_allowed}, {max_allowed}] for direct growth"
        )));
    }
    Ok(parsed)
}
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
fn direct_growth_rounded_to_usize(symbol: &str, value: f64) -> Result<usize, HillslopeCliError> {
    if value > usize::MAX as f64 {
        return Err(direct_growth_failure(format!(
            "{symbol} value {value} exceeds usize range"
        )));
    }
    Ok(value as usize)
}

fn direct_growth_i32_to_usize(
    symbol: &str,
    value: i32,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    if value < 0 {
        return Err(direct_growth_failure(format!(
            "{symbol} must be non-negative for direct growth, observed {value}"
        )));
    }
    let parsed = usize::try_from(value).map_err(|_| {
        direct_growth_failure(format!("{symbol} value {value} exceeds usize range"))
    })?;
    if parsed < min_allowed || parsed > max_allowed {
        return Err(direct_growth_failure(format!(
            "{symbol} value {parsed} outside [{min_allowed}, {max_allowed}] for direct growth"
        )));
    }
    Ok(parsed)
}

fn direct_growth_u16_to_usize(
    symbol: &str,
    value: u16,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    let parsed = usize::from(value);
    if parsed < min_allowed || parsed > max_allowed {
        return Err(direct_growth_failure(format!(
            "{symbol} value {parsed} outside [{min_allowed}, {max_allowed}] for direct growth"
        )));
    }
    Ok(parsed)
}

fn direct_growth_validate_usize(
    symbol: &str,
    value: usize,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    if value < min_allowed || value > max_allowed {
        return Err(direct_growth_failure(format!(
            "{symbol} value {value} outside [{min_allowed}, {max_allowed}] for direct growth"
        )));
    }
    Ok(value)
}

fn direct_growth_usize_to_u16(symbol: &str, value: usize) -> Result<u16, HillslopeCliError> {
    u16::try_from(value).map_err(|_| {
        direct_growth_failure(format!("{symbol} value {value} exceeds u16 range"))
    })
}

fn direct_growth_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!("{SIMOUT_GUARD_ID} {}", detail.into()),
    }
}

impl DirectProductionErosionAuthority {
    fn from_seed(
        seed_surface: &HillslopeWritebackSurface,
    ) -> Result<Self, HillslopeCliError> {
        let wave1_enabled =
            direct_publication_optional_enabled_flag(seed_surface, "erod13_core_enabled")?
                .unwrap_or(false);
        let wave2_enabled = parse_mofe03_binary_flag(
            "erod14_wave2_enabled",
            runtime_surface_symbol_value(seed_surface, "erod14_wave2_enabled").unwrap_or(0.0),
        )?;
        Ok(Self {
            wave2_enabled,
            erosion_inputs: DirectErosionInputs {
                wave1_enabled,
                wave2_enabled,
                wave1: if wave1_enabled {
                    direct_publication_erod13_inputs(seed_surface)?
                } else {
                    DirectErod13Inputs::zero()
                },
                wave2: if wave2_enabled {
                    direct_publication_erod14_inputs(seed_surface)?
                } else {
                    DirectErod14Inputs::zero()
                },
            },
        })
    }
}
