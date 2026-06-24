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
        let snow_lane_state = snow_frost_authority.current_snow_lane_state(lane);
        let frost_context = snow_frost_authority.frost_day_context(
            self.climate_request,
            day_index,
            day,
            lane_index,
            lane,
            &forcing,
            snow_lane_state,
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
            routed_melt_m: snow_liquid.routed_melt_m,
            post_winter_rain_m: snow_liquid.post_winter_rain_m,
            runtime_swe_after_m: snow_liquid.runtime_swe_after_m,
            runtime_depth_after_m: snow_liquid.runtime_depth_after_m,
            runtime_density_after_kg_m3: snow_liquid.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_liquid.runtime_settle_day_count_after,
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
        day_input.evapotranspiration_compute_inputs =
            Some(direct_publication_evapotranspiration_inputs(
                &seed_surface,
                day_index == 0,
            )?);
        day_input.hydrology_projection_inputs =
            Some(direct_publication_hydrology_projection_inputs(
                *self.profile_inputs(lane_index)?,
                &snow_liquid,
            ));
        if let Some(frost_context) = frost_context {
            day_input.winter_frost_compute_inputs = Some(frost_context.compute_inputs);
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
}

#[derive(Clone)]
struct DirectProductionLaneDayInputAuthority {
    canopy_cover_fraction: f64,
    leaf_area_index: f64,
    vegetative_dry_matter_kg_m2: f64,
    peak_runoff: DirectProductionPeakRunoffAuthority,
    percolation: DirectPercolationInputs,
    subsurface: DirectSubsurfaceComputeInputs,
    infiltration: DirectProductionInfiltrationAuthority,
    evapotranspiration: DirectProductionEvapotranspirationAuthority,
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
struct DirectProductionErosionAuthority {
    wave2_enabled: bool,
    erosion_inputs: DirectErosionInputs,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
struct DirectProductionSnowFrostAuthority {
    snow_runtime_swe_m: f64,
    snow_runtime_depth_m: f64,
    snow_runtime_density_kg_m3: f64,
    snow_runtime_settle_day_count: f64,
    snow_controls_projected: bool,
    snow_rst_c: f64,
    snow_newsnw_kg_m3: f64,
    snow_ssd_kg_m3: f64,
    avg_slope: f64,
    azimuth: f64,
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
    layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
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
        Ok(Self {
            climate_request,
            climate_span,
            lane_authority,
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
        Self::validate_active_snow_forcing(
            authority,
            lane_index,
            &forcing,
            rainfall_input_m,
            snow_lane_state.runtime_swe_m,
        )?;
        let snow_liquid = authority.snow_frost.snow_liquid_partition(
            self.climate_request,
            day_index,
            &forcing,
            rainfall_input_m,
            snow_lane_state,
            authority.canopy_cover_fraction,
        )?;
        let frost_context = authority.snow_frost.frost_day_context(
            self.climate_request,
            day_index,
            day,
            lane_index,
        lane,
        &forcing,
        snow_lane_state,
    )?;
        let interception_state = compute_direct_canopy_interception(
            DirectCanopyInterceptionInputs {
                hyetograph_rainfall_m: snow_liquid.post_winter_rain_m,
                interception_rainfall_input_m: snow_liquid.post_winter_rain_m,
                canopy_cover_fraction: authority.canopy_cover_fraction,
                leaf_area_index: authority.leaf_area_index,
                vegetative_dry_matter_kg_m2: authority.vegetative_dry_matter_kg_m2,
            },
        )
        .map_err(|source| direct_publication_runtime_error(&source))?;
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

        let mut day_input =
            DirectPublicationDayInput::calendar_only(direct_publication_calendar_day(day)?);
        day_input.precipitation_m = precipitation_m;
        day_input.effective_temperature_c = day.effective_temperature_c;
        day_input.interception_m = interception_state.interception_m;
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
            routed_melt_m: snow_liquid.routed_melt_m,
            post_winter_rain_m: snow_liquid.post_winter_rain_m,
            runtime_swe_after_m: snow_liquid.runtime_swe_after_m,
            runtime_depth_after_m: snow_liquid.runtime_depth_after_m,
            runtime_density_after_kg_m3: snow_liquid.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_liquid.runtime_settle_day_count_after,
        });
        day_input.peak_runoff_inputs = Some(authority.peak_runoff.inputs(hyetograph.clone()));
        day_input.infiltration_depression_inputs = Some(
            authority
                .infiltration
                .inputs(
                    lane_index,
                    &lane.subsurface_layers,
                    hyetograph,
                    frost_context
                        .as_ref()
                        .map(|context| context.frozen_infiltration_capacity_m_s),
                )?,
        );
        day_input.percolation_inputs =
            Some(authority.percolation_inputs(lane_index, lane)?);
        day_input.subsurface_compute_inputs =
            Some(authority.subsurface_inputs(lane_index, lane)?);
        day_input.evapotranspiration_compute_inputs =
            Some(authority.evapotranspiration.inputs(
                day,
                &forcing,
                lane.evapotranspiration_stage_state,
                &lane.subsurface_layers,
                self.climate_request,
            )?);
        let mut hydrology_projection_inputs =
            authority.hydrology_projection_inputs(&lane.subsurface_layers);
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
            canopy_cover_fraction: require_runtime_surface_scalar(seed_surface, "cancov")?,
            leaf_area_index: require_runtime_surface_scalar(seed_surface, "lai")?,
            vegetative_dry_matter_kg_m2: require_runtime_surface_scalar(seed_surface, "vdmt")?,
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

impl DirectProductionLaneDayInputAuthority {
    fn percolation_inputs(
        &self,
        lane_index: usize,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
    ) -> Result<DirectPercolationInputs, HillslopeCliError> {
        let mut inputs = self.percolation.clone();
        inputs.soil_water_initial_m = direct_production_lane_soil_water(lane, lane_index)?;
        inputs.layers.clone_from(&lane.subsurface_layers);
        Ok(inputs)
    }

    fn subsurface_inputs(
        &self,
        lane_index: usize,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
    ) -> Result<DirectSubsurfaceComputeInputs, HillslopeCliError> {
        direct_production_validate_layers(lane_index, &lane.subsurface_layers)?;
        let mut inputs = self.subsurface.clone();
        inputs.soil_depth_m = lane
            .subsurface_layers
            .iter()
            .map(|layer| layer.depth_m)
            .sum::<f64>();
        inputs.layers = lane.subsurface_layers.iter().cloned().map(Into::into).collect();
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
        })
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

impl DirectProductionSnowFrostAuthority {
    #[allow(clippy::too_many_lines)]
    fn from_seed(seed_surface: &HillslopeWritebackSurface) -> Result<Self, HillslopeCliError> {
        let _snow_file_present = direct_publication_optional_enabled_flag(
            seed_surface,
            "snow.options.snow_file_present",
        )?
        .unwrap_or(false);
        let snow_projection_present = [
            "snow.runtime_swe",
            "snow.runtime_depth_m",
            "snow.runtime_density_kg_m3",
            "snow.runtime_settle_day_count",
            "snow.options.snow_file_present",
            "snow.options.rst",
            "snow.options.newsnw",
            "snow.options.ssd",
        ]
        .iter()
        .any(|symbol| runtime_surface_symbol_value(seed_surface, symbol).is_some());
        let snow_runtime_swe_m = if snow_projection_present {
            let runtime_swe = direct_production_required_snow_state_scalar(
                seed_surface,
                "snow.runtime_swe",
                Some(0.0),
                None,
            )?;
            let _runtime_depth_m = direct_production_required_snow_state_scalar(
                seed_surface,
                "snow.runtime_depth_m",
                Some(0.0),
                None,
            )?;
            let _runtime_density_kg_m3 = direct_production_required_snow_state_scalar(
                seed_surface,
                "snow.runtime_density_kg_m3",
                Some(0.0),
                Some(522.0),
            )?;
            let _runtime_settle_day_count = direct_production_required_snow_state_scalar(
                seed_surface,
                "snow.runtime_settle_day_count",
                Some(0.0),
                None,
            )?;
            runtime_swe
        } else {
            0.0
        };
        let snow_runtime_depth_m = if snow_projection_present {
            direct_production_required_snow_state_scalar(
                seed_surface,
                "snow.runtime_depth_m",
                Some(0.0),
                None,
            )?
        } else {
            0.0
        };
        let snow_runtime_density_kg_m3 = if snow_projection_present {
            direct_production_required_snow_state_scalar(
                seed_surface,
                "snow.runtime_density_kg_m3",
                Some(0.0),
                Some(522.0),
            )?
        } else {
            0.0
        };
        let snow_runtime_settle_day_count = if snow_projection_present {
            direct_production_required_snow_state_scalar(
                seed_surface,
                "snow.runtime_settle_day_count",
                Some(0.0),
                None,
            )?
        } else {
            0.0
        };
        let snow_controls_projected = [
            "snow.options.rst",
            "snow.options.newsnw",
            "snow.options.ssd",
        ]
        .iter()
        .all(|symbol| runtime_surface_symbol_value(seed_surface, symbol).is_some());
        let (snow_rst_c, snow_newsnw_kg_m3, snow_ssd_kg_m3, avg_slope, azimuth) =
            if snow_controls_projected {
                (
                    require_runtime_surface_scalar(seed_surface, "snow.options.rst")?,
                    direct_production_required_snow_state_scalar(
                        seed_surface,
                        "snow.options.newsnw",
                        Some(0.0),
                        None,
                    )?,
                    direct_production_required_snow_state_scalar(
                        seed_surface,
                        "snow.options.ssd",
                        Some(0.0),
                        None,
                    )?,
                    direct_publication_required_positive_scalar(seed_surface, "avgslp")?,
                    require_runtime_surface_scalar(seed_surface, "azm")?,
                )
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0)
            };
        if snow_controls_projected && snow_newsnw_kg_m3 > snow_ssd_kg_m3 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} snow.options.newsnw must be <= snow.options.ssd for direct production snow state, observed {snow_newsnw_kg_m3} > {snow_ssd_kg_m3}"
                ),
            });
        }
        let frost_file_present = direct_publication_optional_enabled_flag(
            seed_surface,
            "frost.options.frost_file_present",
        )?
        .unwrap_or(false);
        let frost_wint_red_enabled =
            direct_publication_optional_enabled_flag(seed_surface, "frost.options.wintRed")?
                .unwrap_or(false);
        let frost_runtime_depth_m = direct_publication_optional_nonnegative_scalar(
            seed_surface,
            &["frost.runtime_dfrost", "frost.runtime_frdp_m"],
        )?
        .unwrap_or(0.0);
        let frost_runtime_frozen_water_m = direct_publication_optional_nonnegative_scalar(
            seed_surface,
            &[
                "frost.runtime_ws_frz",
                "frost.runtime_frwatc_frozen_water_after_m",
            ],
        )?
        .unwrap_or(0.0);
        let frost_projection_present = frost_wint_red_enabled
            || frost_file_present
            || frost_runtime_depth_m > 1.0e-12
            || frost_runtime_frozen_water_m > 1.0e-12
            || runtime_surface_symbol_value(seed_surface, "frost.options.fineTop").is_some()
            || runtime_surface_symbol_value(seed_surface, "frost.options.fineBot").is_some();
        let frost_typed_authority = if frost_projection_present {
            let layers = direct_publication_layer_states(seed_surface)?;
            direct_production_frost_typed_authority(
                seed_surface,
                &layers,
                frost_file_present,
                frost_wint_red_enabled,
                frost_projection_present,
            )?
        } else {
            None
        };
        let frost_layer_carry_projection = if frost_wint_red_enabled {
            direct_publication_frost_layer_carry_projection(seed_surface)?
        } else {
            None
        };
        Ok(Self {
            snow_runtime_swe_m,
            snow_runtime_depth_m,
            snow_runtime_density_kg_m3,
            snow_runtime_settle_day_count,
            snow_controls_projected,
            snow_rst_c,
            snow_newsnw_kg_m3,
            snow_ssd_kg_m3,
            avg_slope,
            azimuth,
            frost_typed_authority,
            frost_layer_carry_projection,
            frost_file_present,
            frost_wint_red_enabled,
            frost_runtime_depth_m,
            frost_runtime_frozen_water_m,
            frost_active: frost_runtime_depth_m > 1.0e-12
                || frost_runtime_frozen_water_m > 1.0e-12,
        })
    }

    fn initial_snow_lane_state(&self) -> DirectSnowLaneState {
        DirectSnowLaneState::from_runtime_values(
            self.snow_runtime_swe_m,
            self.snow_runtime_depth_m,
            self.snow_runtime_density_kg_m3,
            self.snow_runtime_settle_day_count,
        )
    }

    fn current_snow_lane_state(
        &self,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
    ) -> DirectSnowLaneState {
        let lane_state = lane.winter_column.snow;
        if lane_state.has_runtime_state() {
            lane_state
        } else {
            self.initial_snow_lane_state()
        }
    }

    fn snow_state_projected(&self, snow_lane_state: DirectSnowLaneState) -> bool {
        self.snow_controls_projected || snow_lane_state.has_runtime_state()
    }

    fn initial_frost_lane_state(&self) -> DirectFrostLaneState {
        let mut state = DirectFrostLaneState::zero();
        state.active_frost_coupling = self.frost_active;
        state.dfrost_m = self.frost_runtime_depth_m;
        state.frdp_m = self.frost_runtime_depth_m;
        state.ws_frz_m = self.frost_runtime_frozen_water_m;
        state.frwatc_frozen_water_after_m = self.frost_runtime_frozen_water_m;
        state
    }

    fn current_frost_lane_state(
        &self,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
    ) -> DirectFrostLaneState {
        let lane_state = lane.winter_column.frost.clone();
        if lane_state.has_runtime_state() {
            lane_state
        } else {
            self.initial_frost_lane_state()
        }
    }

    fn active_forcing(
        &self,
        forcing: &HillslopeDirectClimateDayForcing,
        hyetograph_rainfall_m: f64,
        runtime_swe_m: f64,
    ) -> Result<bool, HillslopeCliError> {
        if !hyetograph_rainfall_m.is_finite() || hyetograph_rainfall_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production active snow guard requires finite nonnegative rainfall, observed {hyetograph_rainfall_m}"
                ),
            });
        }
        if hyetograph_rainfall_m <= 1.0e-12 && runtime_swe_m <= 1.0e-12 {
            return Ok(false);
        }
        if runtime_swe_m > 1.0e-12 {
            return Ok(true);
        }
        let average_temperature_c = f64::midpoint(forcing.tmax_c, forcing.tmin_c);
        if !average_temperature_c.is_finite() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production active snow guard requires finite tmax/tmin, observed tmax={} tmin={}",
                    forcing.tmax_c, forcing.tmin_c
                ),
            });
        }
        Ok(self.snow_controls_projected && average_temperature_c < 0.0)
    }

    #[allow(clippy::too_many_arguments)]
    fn frost_day_context(
        &self,
        climate_request: &HillslopeClimateRuntimeRequest,
        day_index: usize,
        day: &ClimateDayProjection,
        lane_index: usize,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
        forcing: &HillslopeDirectClimateDayForcing,
        snow_lane_state: DirectSnowLaneState,
    ) -> Result<Option<DirectProductionFrostDayContext>, HillslopeCliError> {
        let frost_lane_state = self.current_frost_lane_state(lane);
        let frost_runtime_depth_m = frost_lane_state.dfrost_m;
        let frost_runtime_frozen_water_m = frost_lane_state.ws_frz_m;
        let should_project = frost_lane_state.has_runtime_state()
            || self.frost_active
            || self.active_frost_forcing(
                forcing,
                frost_runtime_depth_m,
                frost_runtime_frozen_water_m,
            )?;
        if !should_project {
            return Ok(None);
        }
        let Some(typed_authority) = self.frost_typed_authority.as_ref() else {
            return Err(direct_production_executor_blocked(format!(
                "direct production active frost requires frost controls for lane {}",
                lane_index + 1
            )));
        };
        let frost_hourly = self.frost_hourly_forcing(
            climate_request,
            day_index,
            snow_lane_state,
            frost_runtime_depth_m,
            frost_runtime_frozen_water_m,
        )?;
        let typed_context = DirectProductionFrostTypedComputeContext {
            lane_index,
            lane,
            day,
            forcing,
            snow_lane_state,
            frost_lane_state: &frost_lane_state,
            typed_authority,
            hourly: frost_hourly,
        };
        let compute_inputs = Self::typed_winter_frost_compute_inputs(&typed_context);
        let frost_outcome =
            Self::compute_typed_winter_frost_outcome(&typed_context, &compute_inputs)?;
        Ok(Some(DirectProductionFrostDayContext {
            compute_inputs,
            frozen_infiltration_capacity_m_s: frost_outcome.infcap_frz_m_s,
            layer_carry_projection: self.frost_layer_carry_projection.clone(),
        }))
    }

    fn frost_hourly_forcing(
        &self,
        climate_request: &HillslopeClimateRuntimeRequest,
        day_index: usize,
        snow_lane_state: DirectSnowLaneState,
        frost_runtime_depth_m: f64,
        frost_runtime_frozen_water_m: f64,
    ) -> Result<
        [DirectFrostHourlyForcing;
            openwepp_hillslope_orchestrator::DIRECT_WINTER_HOURLY_FORCING_COUNT],
        HillslopeCliError,
    > {
        let hourly = climate_request
            .direct_winter_hourly_forcing(
                day_index,
                DirectWinterHourlyContext {
                    snow_runtime_swe_m: snow_lane_state.runtime_swe_m,
                    frost_runtime_depth_m,
                    frost_runtime_frozen_water_m,
                    frost_file_present: self.frost_file_present,
                    frost_wint_red_enabled: self.frost_wint_red_enabled,
                    avg_slope: self.avg_slope,
                    azimuth: self.azimuth,
                    snow_rst_c: self.snow_rst_c,
                },
            )
            .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production frost hourly forcing failed: {source}"
                ),
            })?
            .ok_or_else(|| {
                direct_production_executor_blocked(format!(
                    "direct production active frost requires typed winter hourly forcing for day {}",
                    day_index + 1
                ))
            })?;
        let mut frost_hourly =
            [DirectFrostHourlyForcing::zero(); openwepp_hillslope_orchestrator::DIRECT_WINTER_HOURLY_FORCING_COUNT];
        for (index, hourly) in hourly.into_iter().enumerate() {
            frost_hourly[index] = DirectFrostHourlyForcing {
                radiation_mj_m2: hourly.radiation_mj_m2,
                air_temperature_c: hourly.air_temperature_c,
                cloud_fraction: hourly.cloud_fraction,
            };
        }
        Ok(frost_hourly)
    }

    fn typed_winter_frost_compute_inputs(
        context: &DirectProductionFrostTypedComputeContext<'_>,
    ) -> DirectWinterFrostComputeInputs {
        DirectWinterFrostComputeInputs {
            controls: context.typed_authority.controls,
            thermal: DirectFrostThermalInputs {
                snow_depth_m: context.snow_lane_state.runtime_depth_m,
                snow_density_kg_m3: context.snow_lane_state.runtime_density_kg_m3,
                residue_depth_m: context.typed_authority.residue_depth_m,
                wind_m_s: context.forcing.vwind_m_s,
                albedo: context.typed_authority.albedo,
                canopy_height_m: context.typed_authority.canopy_height_m,
                random_roughness_m: context.typed_authority.random_roughness_m,
                day_of_year: f64::from(context.day.julian_day),
                monthly_max_c: context.typed_authority.monthly_max_c,
                monthly_min_c: context.typed_authority.monthly_min_c,
            },
            theta_residual: context.typed_authority.theta_residual,
            theta_field_capacity: context.typed_authority.theta_field_capacity,
            soil_conductivity_m_s: context.typed_authority.soil_conductivity_m_s,
            layer_bulk_density_kg_m3: context.typed_authority.layer_bulk_density_kg_m3.clone(),
            hourly: context.hourly,
        }
    }

    fn compute_typed_winter_frost_outcome(
        context: &DirectProductionFrostTypedComputeContext<'_>,
        compute_inputs: &DirectWinterFrostComputeInputs,
    ) -> Result<DirectWinterFrostPartitionOutcome, HillslopeCliError> {
        let soil_conductivity_m_s =
            direct_production_typed_frost_soil_conductivity(
                context.typed_authority,
                &context.lane.subsurface_layers,
            )?;
        let layers = direct_production_frost_layer_inputs(
            context.lane_index,
            &context.lane.subsurface_layers,
            &context.typed_authority.layer_bulk_density_kg_m3,
        )?;
        let profile_depth_m = context
            .lane
            .subsurface_layers
            .iter()
            .map(|layer| layer.depth_m)
            .sum::<f64>();
        Wb11HydrologyKernel::compute_direct_winter_frost_partition(
            &DirectActiveFrostPartitionInputs {
                controls: compute_inputs.controls,
                thermal: compute_inputs.thermal,
                profile_depth_m,
                soil_water_m: context.lane.water.soil_water_m,
                theta_residual: compute_inputs.theta_residual,
                theta_field_capacity: compute_inputs.theta_field_capacity,
                soil_conductivity_m_s,
                prior_state: direct_production_frost_prior_state_input(context.frost_lane_state),
                layers,
                hourly: compute_inputs.hourly,
            },
        )
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct production typed active frost partition failed: {source}"
            ),
        })
    }

    fn active_frost_forcing(
        &self,
        forcing: &HillslopeDirectClimateDayForcing,
        frost_runtime_depth_m: f64,
        frost_runtime_frozen_water_m: f64,
    ) -> Result<bool, HillslopeCliError> {
        for (symbol, value) in [
            ("frost.runtime_dfrost", frost_runtime_depth_m),
            ("frost.runtime_ws_frz", frost_runtime_frozen_water_m),
            ("tmax", forcing.tmax_c),
            ("tmin", forcing.tmin_c),
        ] {
            if !value.is_finite() {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct production active frost guard requires finite {symbol}, observed {value}"
                    ),
                });
            }
        }
        if frost_runtime_depth_m > 1.0e-12 || frost_runtime_frozen_water_m > 1.0e-12 {
            return Ok(true);
        }
        Ok(self.frost_wint_red_enabled && forcing.tmin_c < 0.0)
    }

    fn snow_liquid_partition(
        &self,
        climate_request: &HillslopeClimateRuntimeRequest,
        day_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        hyetograph_rainfall_m: f64,
        snow_lane_state: DirectSnowLaneState,
        canopy_cover_fraction: f64,
    ) -> Result<openwepp_hillslope_orchestrator::DirectSnowLiquidPartition, HillslopeCliError> {
        if !self.active_forcing(forcing, hyetograph_rainfall_m, snow_lane_state.runtime_swe_m)? {
            return Ok(openwepp_hillslope_orchestrator::DirectSnowLiquidPartition {
                active_snow_coupling: false,
                snow_coupling_signed_s_m: 0.0,
                routed_melt_m: 0.0,
                post_winter_rain_m: hyetograph_rainfall_m,
                runtime_swe_after_m: snow_lane_state.runtime_swe_m,
                runtime_depth_after_m: snow_lane_state.runtime_depth_m,
                runtime_density_after_kg_m3: snow_lane_state.runtime_density_kg_m3,
                runtime_settle_day_count_after: snow_lane_state.runtime_settle_day_count,
            });
        }
        let hourly = climate_request
            .direct_winter_hourly_forcing(
                day_index,
                DirectWinterHourlyContext {
                    snow_runtime_swe_m: snow_lane_state.runtime_swe_m,
                    frost_runtime_depth_m: 0.0,
                    frost_runtime_frozen_water_m: 0.0,
                    frost_file_present: false,
                    frost_wint_red_enabled: false,
                    avg_slope: self.avg_slope,
                    azimuth: self.azimuth,
                    snow_rst_c: self.snow_rst_c,
                },
            )
            .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production typed winter hourly forcing failed: {source}"
                ),
            })?
            .ok_or_else(|| {
                direct_production_executor_blocked(format!(
                    "direct production active snow partition requires typed winter hourly forcing for day {}",
                    day_index + 1
                ))
            })?;
        let mut snow_hourly =
            [DirectSnowHourlyForcing::zero(); openwepp_hillslope_orchestrator::runtime_inputs::DIRECT_WINTER_HOURLY_FORCING_COUNT];
        for (index, hourly) in hourly.into_iter().enumerate() {
            snow_hourly[index] = DirectSnowHourlyForcing {
                rain_m: hourly.rain_m,
                snowfall_m: hourly.snowfall_m,
                radiation_mj_m2: hourly.radiation_mj_m2,
                air_temperature_c: hourly.air_temperature_c,
                cloud_fraction: hourly.cloud_fraction,
            };
        }
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
            DirectActiveSnowPartitionInputs {
                hyetograph_rainfall_m,
                rst_c: self.snow_rst_c,
                newsnw_kg_m3: self.snow_newsnw_kg_m3,
                ssd_kg_m3: self.snow_ssd_kg_m3,
                runtime_swe_m: snow_lane_state.runtime_swe_m,
                runtime_depth_m: snow_lane_state.runtime_depth_m,
                runtime_density_kg_m3: snow_lane_state.runtime_density_kg_m3,
                runtime_settle_day_count: snow_lane_state.runtime_settle_day_count,
                tmax_c: forcing.tmax_c,
                tmin_c: forcing.tmin_c,
                canopy_cover_fraction,
                wind_m_s: forcing.vwind_m_s,
                dewpoint_c: forcing.tdpt_c,
                hourly: snow_hourly,
            },
        )
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} direct production typed snow partition failed: {source}"),
        })
    }
}
