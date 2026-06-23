struct DirectPublicationDayInputBuilder<'a> {
    climate_request: &'a HillslopeClimateRuntimeRequest,
    climate_span: &'a ClimateRunSpanSummary,
    climate_context_state: std::cell::RefCell<DirectPublicationClimateContextState>,
    seed_surfaces: Vec<HillslopeWritebackSurface>,
    execution_lane: ExecutionLane,
    profile_inputs: Vec<DirectHydrologyProjectionInputs>,
    erosion_guard_active: bool,
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

    fn build_with_seed_surface(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<(DirectPublicationDayInput, HillslopeWritebackSurface), HillslopeCliError> {
        let frost_runtime_carry = frame
            .lanes
            .get(lane_index)
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication lane index {} exceeds frame lane count {}",
                    lane_index + 1,
                    frame.lanes.len()
                ),
            })?
            .frost_runtime_carry
            .clone();
        let (mut seed_surface, day) = self.seed_surface(frame, day_index, lane_index)?;

        let precipitation_m = day.precipitation_mm / 1_000.0;
        let mut day_input =
            DirectPublicationDayInput::calendar_only(direct_publication_calendar_day(day)?);
        day_input.frost_runtime_carry = frost_runtime_carry;
        let hyetograph = direct_publication_hyetograph(&seed_surface)?;
        let frost_layers = direct_publication_layer_states(&seed_surface)?;
        if !direct_publication_has_frost_runtime_carry(&seed_surface)? {
            overlay_direct_publication_frost_fine_state(
                &mut seed_surface,
                lane_index,
                &frost_layers,
            )?;
        }
        let frost_runoff_surface = DirectFrostRunoffSurface::from_surface_maps(
            seed_surface.state_surface.clone(),
            seed_surface.flux_surface.clone(),
        );
        let frost_partition =
            direct_publication_frost_liquid_partition(&seed_surface, &frost_layers)?;
        apply_direct_publication_frost_infiltration_cap(
            &mut seed_surface,
            &frost_partition,
            lane_index,
        )?;
        let snow_liquid =
            direct_publication_snow_liquid_partition(&seed_surface, &hyetograph)?;
        let interception_state =
            direct_publication_interception_state(
                &seed_surface,
                snow_liquid.post_winter_rain_m,
                &hyetograph,
            )?;
        let post_interception_hyetograph =
            direct_publication_scaled_hyetograph(&hyetograph, interception_state.rainfall_scale)?;
        day_input.precipitation_m = precipitation_m;
        day_input.effective_temperature_c = day.effective_temperature_c;
        day_input.interception_m = interception_state.interception_m;
        day_input.peak_runoff_inputs = Some(direct_publication_peak_runoff_inputs(
            &seed_surface,
            hyetograph.clone(),
        )?);
        let erosion_wave2_active = self.erosion_guard_active
            && direct_publication_erosion_wave2_active(
                &seed_surface,
                &post_interception_hyetograph,
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
        });
        let percolation_inputs =
            direct_publication_percolation_inputs(&seed_surface, precipitation_m)?;
        let subsurface_inputs = direct_publication_subsurface_inputs(&seed_surface)?;
        day_input.infiltration_depression_inputs = Some(
            direct_publication_infiltration_depression_inputs(
                &seed_surface,
                post_interception_hyetograph,
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
        day_input.frost_runoff_surface = Some(frost_runoff_surface);
        day_input.frost_layer_carry_projection =
            direct_publication_frost_layer_carry_projection(&seed_surface)?;
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

fn direct_publication_erosion_wave2_active(
    seed_surface: &HillslopeWritebackSurface,
    hyetograph: &[DirectWb14HyetographInterval],
) -> Result<bool, HillslopeCliError> {
    let wave2_enabled = parse_mofe03_binary_flag(
        "erod14_wave2_enabled",
        runtime_surface_symbol_value(seed_surface, "erod14_wave2_enabled").unwrap_or(0.0),
    )?;
    let rainfall_m = direct_publication_hyetograph_rainfall_m(hyetograph)?;
    Ok(
        wave2_enabled
            && rainfall_m >= DIRECT_PUBLICATION_EROSION_MIN_POST_INTERCEPTION_RAINFALL_M,
    )
}

fn direct_publication_erosion_inputs(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<DirectErosionInputs, HillslopeCliError> {
    let wave1_enabled =
        direct_publication_optional_enabled_flag(seed_surface, "erod13_core_enabled")?
            .unwrap_or(false);
    Ok(DirectErosionInputs {
        wave1_enabled,
        wave2_enabled: true,
        wave1: if wave1_enabled {
            direct_publication_erod13_inputs(seed_surface)?
        } else {
            DirectErod13Inputs::zero()
        },
        wave2: direct_publication_erod14_inputs(seed_surface)?,
    })
}

fn direct_publication_peak_runoff_inputs(
    seed_surface: &HillslopeWritebackSurface,
    hyetograph: Vec<DirectWb14HyetographInterval>,
) -> Result<DirectPeakRunoffInputs, HillslopeCliError> {
    Ok(DirectPeakRunoffInputs {
        hyetograph,
        irrigation_rate_m_s: direct_publication_optional_nonnegative_scalar(
            seed_surface,
            &["irrigation.runtime_rate_m_per_s"],
        )?
        .unwrap_or(0.0),
        efflen_m: require_runtime_surface_scalar(seed_surface, "efflen")?,
        ealpha: require_runtime_surface_scalar(seed_surface, "ealpha")?,
        exponent_m: require_runtime_surface_scalar(seed_surface, "m")?,
    })
}

fn direct_publication_erod13_inputs(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<DirectErod13Inputs, HillslopeCliError> {
    Ok(DirectErod13Inputs {
        ie_m_s: require_runtime_surface_scalar(seed_surface, "Ie")?,
        te_s: require_runtime_surface_scalar(seed_surface, "te")?,
        fs: require_runtime_surface_scalar(seed_surface, "fs")?,
        ft: require_runtime_surface_scalar(seed_surface, "ft")?,
        taufe_pa: require_runtime_surface_scalar(seed_surface, "taufe")?,
        q_m2_s: require_runtime_surface_scalar(seed_surface, "q")?,
        g_kg_s_m: require_runtime_surface_scalar(seed_surface, "G")?,
        di_kg_s_m2: require_runtime_surface_scalar(seed_surface, "Di")?,
        beta: require_runtime_surface_scalar(seed_surface, "beta")?,
        vf_m_s: require_runtime_surface_scalar(seed_surface, "vf")?,
        dgdx_kg_s_m2: require_runtime_surface_scalar(seed_surface, "dGdx")?,
        cntlen_m: require_runtime_surface_scalar(seed_surface, "cntlen")?,
        kr_s_m: require_runtime_surface_scalar(seed_surface, "kr")?,
        kradjf: require_runtime_surface_scalar(seed_surface, "kradjf")?,
        tcadjf: require_runtime_surface_scalar(seed_surface, "tcadjf")?,
        shrsol_pa: require_runtime_surface_scalar(seed_surface, "shrsol")?,
        tcend_kg_s_m: require_runtime_surface_scalar(seed_surface, "tcend")?,
        shcrit_pa: require_runtime_surface_scalar(seed_surface, "shcrit")?,
        detinr_kg_s_m2: require_runtime_surface_scalar(seed_surface, "detinr")?,
        effdrr_m: require_runtime_surface_scalar(seed_surface, "effdrr")?,
        effdrn_m: require_runtime_surface_scalar(seed_surface, "effdrn")?,
        veleff_m_s: require_runtime_surface_scalar(seed_surface, "veleff")?,
        pkro_m3_s: require_runtime_surface_scalar(seed_surface, "pkro")?,
        tc_k: require_runtime_surface_scalar(seed_surface, "erod13_tc_k")?,
        tc_m: require_runtime_surface_scalar(seed_surface, "erod13_tc_m")?,
        q_runoff_m: 0.0,
        peakro_m3_s: 0.0,
        watdur_s: 0.0,
    })
}

fn direct_publication_erod14_inputs(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<DirectErod14Inputs, HillslopeCliError> {
    let slplen_m = require_runtime_surface_scalar(seed_surface, "erod14_slplen")?;
    let hbp_sediment_concentration_scale =
        require_runtime_surface_scalar(seed_surface, "efflen")? / slplen_m;
    let class_count = scalar_to_usize(
        "erod14_class_count",
        require_runtime_surface_scalar(seed_surface, "erod14_class_count")?,
    )?;
    let mut classes = Vec::with_capacity(class_count);
    for class_index in 1..=class_count {
        classes.push(DirectErod14ClassInputs {
            fall_m_s: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_fall", class_index).as_str(),
            )?,
            frcflw: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_frcflw", class_index).as_str(),
            )?,
            frac: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_frac", class_index).as_str(),
            )?,
            fidel: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_fidel", class_index).as_str(),
            )?,
            tcf1: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_tcf1", class_index).as_str(),
            )?,
            ssa_class: require_runtime_surface_scalar(
                seed_surface,
                direct_publication_erod14_class_symbol("erod14_ssa_class", class_index).as_str(),
            )?,
        });
    }
    Ok(DirectErod14Inputs {
        xtop_m: require_runtime_surface_scalar(seed_surface, "erod14_xtop")?,
        xbot_m: require_runtime_surface_scalar(seed_surface, "erod14_xbot")?,
        xdetst_m: require_runtime_surface_scalar(seed_surface, "erod14_xdetst")?,
        ldtop_kg_s_m: require_runtime_surface_scalar(seed_surface, "erod14_ldtop")?,
        ldbot_kg_s_m: require_runtime_surface_scalar(seed_surface, "erod14_ldbot")?,
        lddend_kg: require_runtime_surface_scalar(seed_surface, "erod14_lddend")?,
        qout_m3_s: require_runtime_surface_scalar(seed_surface, "erod14_qout")?,
        qin_m3_s: require_runtime_surface_scalar(seed_surface, "erod14_qin")?,
        qostar_m: require_runtime_surface_scalar(seed_surface, "erod14_qostar")?,
        hbp_sediment_concentration_scale,
        slplen_m,
        ktrato: require_runtime_surface_scalar(seed_surface, "erod14_ktrato")?,
        aintc: require_runtime_surface_scalar(seed_surface, "erod14_ainftc")?,
        bintc: require_runtime_surface_scalar(seed_surface, "erod14_binftc")?,
        cintc: require_runtime_surface_scalar(seed_surface, "erod14_cinftc")?,
        beta: require_runtime_surface_scalar(seed_surface, "erod14_beta")?,
        qj_minus_1_m3_s: require_runtime_surface_scalar(seed_surface, "erod14_Qj_minus_1")?,
        vj_m: require_runtime_surface_scalar(seed_surface, "erod14_Vj")?,
        qj_m3_s: require_runtime_surface_scalar(seed_surface, "erod14_Qj")?,
        fh_m: require_runtime_surface_scalar(seed_surface, "erod14_Fh")?,
        fp_m: require_runtime_surface_scalar(seed_surface, "erod14_Fp")?,
        case_value: require_runtime_surface_scalar(seed_surface, "erod14_case")?,
        peak_runoff_m3_s: 0.0,
        runoff_duration_s: 0.0,
        ssa_soil: require_runtime_surface_scalar(seed_surface, "erod14_ssa_soil")?,
        theta: require_runtime_surface_scalar(seed_surface, "theta")?,
        classes,
    })
}

fn direct_publication_erod14_class_symbol(root: &str, class_index: usize) -> String {
    format!("{root}_{class_index:04}")
}

fn direct_publication_day_zero_seed_surface(
    climate_request: &HillslopeClimateRuntimeRequest,
    climate_span: &ClimateRunSpanSummary,
    seed_authority: &HillslopeWritebackSurface,
    climate_context_surface: &HillslopeWritebackSurface,
    execution_lane: ExecutionLane,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    let day = climate_span.days.first().ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} direct publication requires at least one climate day"),
        }
    })?;
    direct_publication_validate_day(day)?;
    let mut seed_surface = seed_authority.clone();
    let mut climate_surface = build_day_climate_surface(climate_request, 0, climate_context_surface, day)?;
    seed_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
        seed_surface,
        std::mem::take(&mut climate_surface),
    );
    seed_wb11_runtime_surface_inputs(&mut seed_surface, execution_lane)?;
    Ok(seed_surface)
}

fn direct_publication_validate_day(day: &ClimateDayProjection) -> Result<(), HillslopeCliError> {
    if !day.precipitation_mm.is_finite() || day.precipitation_mm < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication precipitation must be finite and >= 0.0, observed {}",
                day.precipitation_mm
            ),
        });
    }
    if !day.effective_temperature_c.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication effective temperature must be finite, observed {}",
                day.effective_temperature_c
            ),
        });
    }
    Ok(())
}

fn overlay_direct_publication_lane_state(
    seed_surface: &mut HillslopeWritebackSurface,
    day_index: usize,
    lane_index: usize,
    lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
) -> Result<(), HillslopeCliError> {
    if lane.subsurface_layers.is_empty() {
        if day_index == 0 {
            return Ok(());
        }
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication day {} lane {} requires committed direct-carried layers before PMET construction",
                day_index + 1,
                lane_index + 1
            ),
        });
    }
    let nsl = lane.subsurface_layers.len();
    let nsl_u32 = u32::try_from(nsl).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!(
            "{SIMOUT_GUARD_ID} direct publication lane {} layer count {nsl} exceeds u32 range",
            lane_index + 1
        ),
    })?;
    let nsl_value = f64::from(nsl_u32);
    insert_direct_seed_scalar(seed_surface, "wb11_nsl", nsl_value, lane_index)?;
    insert_direct_seed_scalar(seed_surface, "nsl", nsl_value, lane_index)?;
    let layer_summary =
        overlay_direct_publication_layer_state(seed_surface, lane_index, &lane.subsurface_layers)?;
    if !lane.water.soil_water_m.is_finite() || lane.water.soil_water_m < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane {} soil water carry must be finite and non-negative, observed {}",
                lane_index + 1,
                lane.water.soil_water_m
            ),
        });
    }
    let soil_water_m = lane.water.soil_water_m;
    if (layer_summary.aggregate_soil_water - soil_water_m).abs() > 1.0e-9 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane {} layer aggregate {} diverges from storage carry {}",
                lane_index + 1,
                layer_summary.aggregate_soil_water,
                soil_water_m
            ),
        });
    }
    if let Some(carry) = &lane.frost_runtime_carry {
        insert_direct_seed_scalar(seed_surface, "wb11_soil_water", soil_water_m, lane_index)?;
        overlay_direct_publication_frost_runtime_carry(seed_surface, lane_index, carry)?;
        return Ok(());
    }
    overlay_direct_publication_frost_fine_state(
        seed_surface,
        lane_index,
        &lane.subsurface_layers,
    )?;
    for (symbol, value) in [
        ("wb11_soil_water", soil_water_m),
        ("frost.runtime_ws_frz", layer_summary.frozen_water),
        (
            "frost.runtime_frwatc_frozen_water_after_m",
            layer_summary.frozen_water,
        ),
        ("frost.runtime_frdp_m", layer_summary.frost_depth),
        ("frost.runtime_dfrost", layer_summary.frost_depth),
    ] {
        insert_direct_seed_scalar(seed_surface, symbol, value, lane_index)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct DirectPublicationLayerOverlaySummary {
    aggregate_soil_water: f64,
    frozen_water: f64,
    frost_depth: f64,
}

fn overlay_direct_publication_layer_state(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<DirectPublicationLayerOverlaySummary, HillslopeCliError> {
    let mut aggregate_soil_water = 0.0_f64;
    let mut frozen_water = 0.0_f64;
    let mut frost_depth = 0.0_f64;
    let mut layer_top_m = 0.0_f64;
    for (layer_offset, layer) in layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        aggregate_soil_water += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        frozen_water += layer.frozen_water_m;
        if layer.frozen_depth_m > 1.0e-12 {
            frost_depth = layer_top_m + layer.frozen_depth_m;
        }
        for (symbol, value) in direct_publication_layer_seed_scalars(layer_index, layer) {
            insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
        }
        layer_top_m += layer.depth_m;
    }
    Ok(DirectPublicationLayerOverlaySummary {
        aggregate_soil_water,
        frozen_water,
        frost_depth,
    })
}

fn direct_publication_layer_seed_scalars(
    layer_index: usize,
    layer: &DirectSubsurfaceLayerState,
) -> [(String, f64); 12] {
    [
        (format!("wb18_perc_theta_{layer_index:04}"), layer.theta_m),
        (
            format!("wb18_perc_fc_{layer_index:04}"),
            layer.field_capacity_m,
        ),
        (
            format!("wb18_perc_ul_{layer_index:04}"),
            layer.upper_limit_m,
        ),
        (
            format!("wb18_perc_ssc_{layer_index:04}"),
            layer.conductivity_m_s,
        ),
        (format!("wb19_dg_{layer_index:04}"), layer.depth_m),
        (
            format!("wb19_thetdr_{layer_index:04}"),
            layer.residual_theta,
        ),
        (
            format!("wb18_perc_frozen_depth_{layer_index:04}"),
            layer.frozen_depth_m,
        ),
        (
            format!("wb18_perc_frzw_{layer_index:04}"),
            layer.frozen_water_m,
        ),
        (format!("wb19_por_{layer_index:04}"), layer.porosity),
        (
            format!("wb19_thetfc_{layer_index:04}"),
            layer.field_capacity_theta,
        ),
        (format!("wb19_coca_{layer_index:04}"), layer.coca),
        (format!("coca_{layer_index:04}"), layer.coca),
    ]
}

fn overlay_direct_publication_frost_fine_state(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<(), HillslopeCliError> {
    if runtime_surface_symbol_value(seed_surface, "frost.options.fineTop").is_none()
        || runtime_surface_symbol_value(seed_surface, "frost.options.fineBot").is_none()
    {
        return Ok(());
    }
    let layer_count = layers.len();
    let fine_top_count =
        direct_publication_frost_fine_count(seed_surface, "frost.options.fineTop")?;
    let fine_bot_count =
        direct_publication_frost_fine_count(seed_surface, "frost.options.fineBot")?;
    for (layer_offset, layer) in layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        let fine_layer_count = direct_publication_frost_fine_layer_count(
            layer_index,
            layer_count,
            layer.depth_m,
            fine_top_count,
            fine_bot_count,
        )?;
        let fine_layer_thickness_m =
            layer.depth_m / usize_to_scalar("frost.runtime_nfine", fine_layer_count)?;
        let mut remaining_frozen_depth_m = layer.frozen_depth_m;
        let soilf_m = layer.frozen_water_m + layer.residual_theta * layer.frozen_depth_m;
        let ice_per_frozen_m = if layer.frozen_depth_m > 1.0e-12 {
            soilf_m / layer.frozen_depth_m
        } else {
            0.0
        };
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
        for fine_index in 1..=fine_layer_count {
            let slfsd_m = remaining_frozen_depth_m
                .min(fine_layer_thickness_m)
                .max(0.0);
            remaining_frozen_depth_m = (remaining_frozen_depth_m - slfsd_m).max(0.0);
            let fgfrst = if slfsd_m >= fine_layer_thickness_m - 1.0e-12 {
                1.0
            } else if slfsd_m > 1.0e-12 {
                2.0
            } else {
                0.0
            };
            for (symbol, value) in [
                (
                    format!("frost.runtime_fgfrst_{layer_index:04}_{fine_index:04}"),
                    fgfrst,
                ),
                (
                    format!("frost.runtime_slfsd_m_{layer_index:04}_{fine_index:04}"),
                    slfsd_m,
                ),
                (
                    format!("frost.runtime_slsic_m_{layer_index:04}_{fine_index:04}"),
                    ice_per_frozen_m * slfsd_m,
                ),
                (
                    format!("frost.runtime_slsw_theta_{layer_index:04}_{fine_index:04}"),
                    slsw_theta,
                ),
                (
                    format!("frost.runtime_sltime_s_{layer_index:04}_{fine_index:04}"),
                    0.0,
                ),
            ] {
                insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
            }
        }
    }
    Ok(())
}

fn direct_publication_has_frost_runtime_carry(
    seed_surface: &HillslopeWritebackSurface,
) -> Result<bool, HillslopeCliError> {
    direct_publication_optional_enabled_flag(
        seed_surface,
        "frost.direct_runtime_carry_present",
    )
    .map(|value| value.unwrap_or(false))
}

fn direct_publication_frost_runtime_carry_has_fine_projection(
    carry: &DirectFrostRuntimeCarry,
) -> bool {
    !carry.fine_layers.is_empty()
}

fn overlay_direct_publication_frost_runtime_carry(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    carry: &DirectFrostRuntimeCarry,
) -> Result<(), HillslopeCliError> {
    insert_direct_frost_runtime_scalars(
        seed_surface,
        lane_index,
        DirectFrostRuntimeScalarSeed {
            dfrost_m: carry.dfrost_m,
            dthaw_m: carry.dthaw_m,
            nft: carry.nft,
            ws_frz_m: carry.ws_frz_m,
            infcap_frz_m_s: carry.infcap_frz_m_s,
            frwatc_soil_water_before_m: carry.frwatc_soil_water_before_m,
            frwatc_soil_water_after_m: carry.frwatc_soil_water_after_m,
            frwatc_frozen_water_before_m: carry.frwatc_frozen_water_before_m,
            frwatc_frozen_water_after_m: carry.frwatc_frozen_water_after_m,
            frwatc_freeze_debit_m: carry.frwatc_freeze_debit_m,
            frwatc_thaw_credit_m: carry.frwatc_thaw_credit_m,
            frwatc_net_liquid_delta_m: carry.frwatc_net_liquid_delta_m,
            frdp_m: carry.frdp_m,
            thdp_m: carry.thdp_m,
            tfrdp_m: carry.tfrdp_m,
            tthawd_m: carry.tthawd_m,
            fgthwd_flag: carry.fgthwd_flag,
            total_fine_layer_count: carry.total_fine_layer_count,
            conductivity_tilled_w_m_k: carry.conductivity_tilled_w_m_k,
            conductivity_untilled_w_m_k: carry.conductivity_untilled_w_m_k,
            conductivity_residue_w_m_k: carry.conductivity_residue_w_m_k,
            shadow_total_water_before_m: carry.shadow_total_water_before_m,
            shadow_total_water_after_m: carry.shadow_total_water_after_m,
            shadow_wb_delta_m: carry.shadow_wb_delta_m,
            shadow_frwatc_residual_m: carry.shadow_frwatc_residual_m,
            watpdg_m: carry.watpdg_m,
            watbtm_m: carry.watbtm_m,
            fine_projection: direct_publication_frost_runtime_carry_has_fine_projection(carry),
        },
    )?;
    for layer in &carry.layer_shadows {
        insert_direct_frost_layer_shadow(seed_surface, lane_index, *layer)?;
    }
    for fine in &carry.fine_layers {
        insert_direct_frost_fine_layer(seed_surface, lane_index, *fine)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct DirectFrostRuntimeScalarSeed {
    dfrost_m: f64,
    dthaw_m: f64,
    nft: f64,
    ws_frz_m: f64,
    infcap_frz_m_s: f64,
    frwatc_soil_water_before_m: f64,
    frwatc_soil_water_after_m: f64,
    frwatc_frozen_water_before_m: f64,
    frwatc_frozen_water_after_m: f64,
    frwatc_freeze_debit_m: f64,
    frwatc_thaw_credit_m: f64,
    frwatc_net_liquid_delta_m: f64,
    frdp_m: f64,
    thdp_m: f64,
    tfrdp_m: f64,
    tthawd_m: f64,
    fgthwd_flag: f64,
    total_fine_layer_count: f64,
    conductivity_tilled_w_m_k: f64,
    conductivity_untilled_w_m_k: f64,
    conductivity_residue_w_m_k: f64,
    shadow_total_water_before_m: f64,
    shadow_total_water_after_m: f64,
    shadow_wb_delta_m: f64,
    shadow_frwatc_residual_m: f64,
    watpdg_m: f64,
    watbtm_m: f64,
    fine_projection: bool,
}

fn insert_direct_frost_runtime_scalars(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    seed: DirectFrostRuntimeScalarSeed,
) -> Result<(), HillslopeCliError> {
    for (symbol, value) in [
        (
            "frost.direct_runtime_carry_present",
            if seed.fine_projection { 1.0 } else { 0.0 },
        ),
        ("frost.runtime_dfrost", seed.dfrost_m),
        ("frost.runtime_dthaw", seed.dthaw_m),
        ("frost.runtime_nft", seed.nft),
        ("frost.runtime_ws_frz", seed.ws_frz_m),
        ("frost.runtime_infcap_frz", seed.infcap_frz_m_s),
        (
            "frost.runtime_frwatc_soil_water_before_m",
            seed.frwatc_soil_water_before_m,
        ),
        (
            "frost.runtime_frwatc_soil_water_after_m",
            seed.frwatc_soil_water_after_m,
        ),
        (
            "frost.runtime_frwatc_frozen_water_before_m",
            seed.frwatc_frozen_water_before_m,
        ),
        (
            "frost.runtime_frwatc_frozen_water_after_m",
            seed.frwatc_frozen_water_after_m,
        ),
        (
            "frost.runtime_frwatc_freeze_debit_m",
            seed.frwatc_freeze_debit_m,
        ),
        (
            "frost.runtime_frwatc_thaw_credit_m",
            seed.frwatc_thaw_credit_m,
        ),
        (
            "frost.runtime_frwatc_net_liquid_delta_m",
            seed.frwatc_net_liquid_delta_m,
        ),
        ("frost.runtime_frdp_m", seed.frdp_m),
        ("frost.runtime_thdp_m", seed.thdp_m),
        ("frost.runtime_tfrdp_m", seed.tfrdp_m),
        ("frost.runtime_tthawd_m", seed.tthawd_m),
        ("frost.runtime_fgthwd_flag", seed.fgthwd_flag),
        (
            "frost.runtime_total_fine_layer_count",
            seed.total_fine_layer_count,
        ),
        ("frost.runtime_kftill_w_m_k", seed.conductivity_tilled_w_m_k),
        (
            "frost.runtime_kfutil_w_m_k",
            seed.conductivity_untilled_w_m_k,
        ),
        ("frost.runtime_kres_w_m_k", seed.conductivity_residue_w_m_k),
        (
            "frost.runtime_shadow_total_water_before_m",
            seed.shadow_total_water_before_m,
        ),
        (
            "frost.runtime_shadow_total_water_after_m",
            seed.shadow_total_water_after_m,
        ),
        ("frost.runtime_shadow_wb_delta_m", seed.shadow_wb_delta_m),
        (
            "frost.runtime_shadow_frwatc_residual_m",
            seed.shadow_frwatc_residual_m,
        ),
        ("frost.runtime_watpdg_m", seed.watpdg_m),
        ("frost.runtime_watbtm_m", seed.watbtm_m),
    ] {
        insert_direct_seed_scalar(seed_surface, symbol, value, lane_index)?;
    }
    Ok(())
}

fn insert_direct_frost_layer_shadow(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    layer: DirectFrostLayerShadowCarry,
) -> Result<(), HillslopeCliError> {
    let layer_symbols: [(String, f64); 7] = [
        (
            format!("frost.runtime_shadow_st_m_{:04}", layer.layer_index),
            layer.st_m,
        ),
        (
            format!(
                "frost.runtime_shadow_soil_water_m_{:04}",
                layer.layer_index
            ),
            layer.soil_water_m,
        ),
        (
            format!(
                "frost.runtime_shadow_frozen_depth_m_{:04}",
                layer.layer_index
            ),
            layer.frozen_depth_m,
        ),
        (
            format!("frost.runtime_shadow_frzw_m_{:04}", layer.layer_index),
            layer.frozen_water_m,
        ),
        (
            format!("frost.runtime_shadow_soilf_m_{:04}", layer.layer_index),
            layer.soilf_m,
        ),
        (
            format!("frost.runtime_yst_m_{:04}", layer.layer_index),
            layer.yst_m,
        ),
        (
            format!("frost.runtime_nwfrzz_m_{:04}", layer.layer_index),
            layer.nwfrzz_m,
        ),
    ];
    for (symbol, value) in layer_symbols {
        insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
    }
    Ok(())
}

fn insert_direct_frost_fine_layer(
    seed_surface: &mut HillslopeWritebackSurface,
    lane_index: usize,
    fine: DirectFrostFineLayerCarry,
) -> Result<(), HillslopeCliError> {
    for (symbol, value) in [
        (
            format!(
                "frost.runtime_fgfrst_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.fgfrst,
        ),
        (
            format!(
                "frost.runtime_slfsd_m_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slfsd_m,
        ),
        (
            format!(
                "frost.runtime_slsic_m_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slsic_m,
        ),
        (
            format!(
                "frost.runtime_slsw_theta_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.slsw_theta,
        ),
        (
            format!(
                "frost.runtime_sltime_s_{:04}_{:04}",
                fine.layer_index, fine.fine_index
            ),
            fine.sltime_s,
        ),
    ] {
        insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
    }
    Ok(())
}

fn insert_direct_seed_scalar(
    seed_surface: &mut HillslopeWritebackSurface,
    symbol: &str,
    value: f64,
    lane_index: usize,
) -> Result<(), HillslopeCliError> {
    if !value.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane {} carried symbol {symbol} is non-finite ({value})",
                lane_index + 1
            ),
        });
    }
    seed_surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    Ok(())
}

fn direct_publication_percolation_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    _precipitation_m: f64,
) -> Result<DirectPercolationInputs, HillslopeCliError> {
    let layers = direct_publication_layer_states(runtime_surface)?;
    let soil_water_initial_m = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    let lane_substeps = scalar_to_usize(
        "wb18_perc_lane_substeps",
        require_runtime_surface_scalar(runtime_surface, "wb18_perc_lane_substeps")?,
    )?;
    let restrictive_layer_enabled =
        direct_publication_optional_enabled_flag(runtime_surface, "slflag")?.unwrap_or(false);
    let restrictive_layer_conductivity_m_s = if restrictive_layer_enabled {
        direct_publication_required_positive_scalar(runtime_surface, "kslast")?
    } else {
        0.0
    };
    let restrictive_layer_thickness_m = if restrictive_layer_enabled && lane_substeps > 1 {
        direct_publication_required_positive_scalar(runtime_surface, "ui_bdrkth")?
    } else {
        0.0
    };
    Ok(DirectPercolationInputs {
        soil_water_initial_m,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps,
        restrictive_layer_enabled,
        restrictive_layer_conductivity_m_s,
        restrictive_layer_thickness_m,
        layers,
    })
}

fn direct_publication_liquid_input_inputs(
    liquid_input_handoff_m: f64,
) -> Result<DirectLiquidInputInputs, HillslopeCliError> {
    if liquid_input_handoff_m < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} wb12_rainfall_input must be >= 0.0 for direct R4I liquid input, observed {liquid_input_handoff_m}"
            ),
        });
    }
    Ok(DirectLiquidInputInputs {
        liquid_input_handoff_m,
    })
}

fn direct_publication_storage_input_inputs(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<DirectStorageInputInputs, HillslopeCliError> {
    let precip_input_handoff_m = require_runtime_surface_scalar(runtime_surface, "wb12_precip_input")?;
    if precip_input_handoff_m < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} wb12_precip_input must be >= 0.0 for direct R4C storage input, observed {precip_input_handoff_m}"
            ),
        });
    }
    Ok(DirectStorageInputInputs {
        precip_input_handoff_m: Some(precip_input_handoff_m),
    })
}

fn direct_publication_interception_state(
    runtime_surface: &HillslopeWritebackSurface,
    rainfall_input_m: f64,
    hyetograph: &[DirectWb14HyetographInterval],
) -> Result<openwepp_hillslope_orchestrator::DirectCanopyInterceptionState, HillslopeCliError> {
    let hyetograph_rainfall_m = direct_publication_hyetograph_rainfall_m(hyetograph)?;
    compute_direct_canopy_interception(DirectCanopyInterceptionInputs {
        hyetograph_rainfall_m,
        interception_rainfall_input_m: rainfall_input_m,
        canopy_cover_fraction: require_runtime_surface_scalar(runtime_surface, "cancov")?,
        leaf_area_index: require_runtime_surface_scalar(runtime_surface, "lai")?,
        vegetative_dry_matter_kg_m2: require_runtime_surface_scalar(runtime_surface, "vdmt")?,
    })
    .map_err(|source| direct_publication_runtime_error(&source))
}

fn direct_publication_snow_liquid_partition(
    runtime_surface: &HillslopeWritebackSurface,
    hyetograph: &[DirectWb14HyetographInterval],
) -> Result<openwepp_hillslope_orchestrator::DirectSnowLiquidPartition, HillslopeCliError> {
    let hyetograph_rainfall_m = direct_publication_hyetograph_rainfall_m(hyetograph)?;
    Wb11HydrologyKernel::compute_direct_snow_liquid_partition(
        &runtime_surface.state_surface,
        &runtime_surface.flux_surface,
        hyetograph_rainfall_m,
    )
    .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!("{SIMOUT_GUARD_ID} direct R4G snow/liquid partition failed: {source}"),
    })
}

fn direct_publication_frost_liquid_partition(
    runtime_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<openwepp_hillslope_orchestrator::DirectFrostLiquidPartition, HillslopeCliError> {
    let soil_conductivity_m_s = direct_publication_wb14_base_conductivity(runtime_surface, layers)?;
    Wb11HydrologyKernel::compute_direct_frost_liquid_partition(
        &runtime_surface.state_surface,
        &runtime_surface.flux_surface,
        soil_conductivity_m_s,
    )
    .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!("{SIMOUT_GUARD_ID} direct R4G frost/liquid partition failed: {source}"),
    })
}

fn apply_direct_publication_frost_infiltration_cap(
    runtime_surface: &mut HillslopeWritebackSurface,
    frost_partition: &openwepp_hillslope_orchestrator::DirectFrostLiquidPartition,
    lane_index: usize,
) -> Result<(), HillslopeCliError> {
    insert_direct_seed_scalar(
        runtime_surface,
        "frost.runtime_infcap_frz",
        frost_partition.infcap_frz_m_s,
        lane_index,
    )
}

fn direct_publication_hydrology_projection_inputs(
    mut profile_inputs: DirectHydrologyProjectionInputs,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> DirectHydrologyProjectionInputs {
    profile_inputs.snow_water_m = snow_liquid.runtime_swe_after_m;
    profile_inputs
}

fn direct_publication_infiltration_depression_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    hyetograph: Vec<DirectWb14HyetographInterval>,
) -> Result<DirectInfiltrationDepressionInputs, HillslopeCliError> {
    let layers = direct_publication_layer_states(runtime_surface)?;
    let effective_conductivity_m_s =
        direct_publication_wb14_effective_conductivity(runtime_surface, &layers)?;
    let matric_potential_m = direct_publication_wb14_matric_potential(runtime_surface, &layers)?;
    let storage_capacity_m = direct_publication_wb14_top_storage_capacity(&layers)?;
    let depression_storage_capacity_m = direct_publication_optional_nonnegative_scalar(
        runtime_surface,
        &[
            "wb14_depression_storage_capacity_m",
            "wb12_depression_storage_capacity_m",
        ],
    )?
    .unwrap_or(0.0);

    Ok(DirectInfiltrationDepressionInputs {
        cumulative_infiltration_handoff_m: 0.0,
        depression_storage_delta_handoff_m: 0.0,
        producer_inputs: Some(DirectWb14InfiltrationProducerInputs {
            hyetograph,
            effective_conductivity_m_s,
            matric_potential_m,
            storage_capacity_m,
            depression_storage_capacity_m,
        }),
    })
}

fn direct_publication_hyetograph_rainfall_m(
    hyetograph: &[DirectWb14HyetographInterval],
) -> Result<f64, HillslopeCliError> {
    let mut total_m = 0.0_f64;
    for interval in hyetograph {
        let duration_s = interval.end_s - interval.start_s;
        if duration_s <= 0.0 || interval.intensity_m_s <= 0.0 {
            continue;
        }
        let rainfall_m = duration_s * interval.intensity_m_s;
        if !rainfall_m.is_finite() || rainfall_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct WB15 hyetograph rainfall must be finite and >= 0.0, observed {rainfall_m}"
                ),
            });
        }
        total_m += rainfall_m;
        if !total_m.is_finite() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct WB15 hyetograph rainfall total is non-finite"
                ),
            });
        }
    }
    Ok(total_m)
}

fn direct_publication_scaled_hyetograph(
    hyetograph: &[DirectWb14HyetographInterval],
    rainfall_scale: f64,
) -> Result<Vec<DirectWb14HyetographInterval>, HillslopeCliError> {
    if !rainfall_scale.is_finite() || rainfall_scale < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct WB15 rainfall scale must be finite and >= 0.0, observed {rainfall_scale}"
            ),
        });
    }
    hyetograph
        .iter()
        .map(|interval| {
            let intensity_m_s = interval.intensity_m_s * rainfall_scale;
            if !intensity_m_s.is_finite() || intensity_m_s < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct WB15 scaled hyetograph intensity must be finite and >= 0.0, observed {intensity_m_s}"
                    ),
                });
            }
            Ok(DirectWb14HyetographInterval {
                start_s: interval.start_s,
                end_s: interval.end_s,
                intensity_m_s,
            })
        })
        .collect()
}

fn direct_publication_hyetograph(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Vec<DirectWb14HyetographInterval>, HillslopeCliError> {
    let point_symbol = if runtime_surface_symbol_value(runtime_surface, "ninten").is_some() {
        "ninten"
    } else {
        "nbrkpt"
    };
    let point_count = scalar_to_usize(
        point_symbol,
        require_runtime_surface_scalar(runtime_surface, point_symbol)?,
    )?;
    if point_count < 2 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct hyetograph requires at least two time points, observed {point_count}"
            ),
        });
    }
    let mut intervals = Vec::with_capacity(point_count - 1);
    for point_index in 1..point_count {
        let start_symbol = wb13_primary_layer_symbol("timem", point_index);
        let end_symbol = wb13_primary_layer_symbol("timem", point_index + 1);
        let intensity_symbol = wb13_primary_layer_symbol("intsty", point_index);
        let start_s = require_runtime_surface_scalar(runtime_surface, start_symbol.as_str())?;
        let end_s = require_runtime_surface_scalar(runtime_surface, end_symbol.as_str())?;
        let intensity_m_s =
            require_runtime_surface_scalar(runtime_surface, intensity_symbol.as_str())?;
        intervals.push(DirectWb14HyetographInterval {
            start_s,
            end_s,
            intensity_m_s,
        });
    }
    Ok(intervals)
}

fn direct_publication_wb14_effective_conductivity(
    runtime_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if let Some(value) = direct_publication_optional_nonnegative_scalar(
        runtime_surface,
        &[
            "wb14_effective_conductivity_m_s",
            "frost.runtime_infcap_frz",
            "wb14_soil_conductivity_m_s",
        ],
    )? {
        if value > 0.0 {
            return Ok(value);
        }
    }
    layers
        .first()
        .map(|layer| layer.conductivity_m_s)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct infiltration requires at least one layer conductivity"
            ),
        })
}

fn direct_publication_wb14_base_conductivity(
    runtime_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if let Some(value) =
        direct_publication_optional_nonnegative_scalar(runtime_surface, &["wb14_soil_conductivity_m_s"])?
    {
        if value > 0.0 {
            return Ok(value);
        }
    }
    layers
        .first()
        .map(|layer| layer.conductivity_m_s)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct frost partition requires at least one layer conductivity"
            ),
        })
}

fn direct_publication_wb14_matric_potential(
    runtime_surface: &HillslopeWritebackSurface,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if let Some(value) = direct_publication_optional_nonnegative_scalar(
        runtime_surface,
        &["wb14_matric_potential_m"],
    )? {
        return Ok(value);
    }
    let first_layer = layers
        .first()
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct infiltration requires at least one layer for matric potential"
            ),
        })?;
    Ok(first_layer.depth_m * (first_layer.field_capacity_theta - first_layer.residual_theta).max(0.0))
}

fn direct_publication_wb14_top_storage_capacity(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    if layers.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} WB14 direct infiltration requires layer storage capacity"
            ),
        });
    }
    Ok(layers
        .iter()
        .take(2)
        .map(|layer| (layer.upper_limit_m - layer.frozen_water_m - layer.theta_m).max(0.0))
        .sum())
}

fn direct_publication_optional_nonnegative_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbols: &[&str],
) -> Result<Option<f64>, HillslopeCliError> {
    for symbol in symbols {
        if let Some(value) = runtime_surface_symbol_value(runtime_surface, symbol) {
            if !value.is_finite() || value < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} {symbol} must be finite and >= 0.0 for WB14 direct infiltration, observed {value}"
                    ),
                });
            }
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn direct_publication_subsurface_inputs(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<DirectSubsurfaceComputeInputs, HillslopeCliError> {
    let layer_states = direct_publication_layer_states(runtime_surface)?;
    let soil_depth_m = layer_states.iter().map(|layer| layer.depth_m).sum::<f64>();
    let lane_substeps = scalar_to_usize(
        "wb19_lateral_drain_lane_substeps",
        require_runtime_surface_scalar(runtime_surface, "wb19_lateral_drain_lane_substeps")?,
    )?;
    let drain_enabled = direct_publication_enabled_flag(runtime_surface, "wb19_drain_enabled")?;
    let drain_depth_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_depth")?
    } else {
        0.5
    };
    let drain_spacing_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_spacing")?
    } else {
        1.0
    };
    let drain_diameter_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_diameter")?
    } else {
        0.1
    };
    Ok(DirectSubsurfaceComputeInputs {
        avg_slope: require_runtime_surface_scalar(runtime_surface, "avgslp")?,
        slope_length_m: require_runtime_surface_scalar(runtime_surface, "slplen")?,
        lateral_anisotropy_ratio: require_runtime_surface_scalar(
            runtime_surface,
            "wb19_lateral_anisotropy_ratio",
        )?,
        soil_depth_m,
        solwpv_mode: scalar_to_i32(
            "solwpv",
            require_runtime_surface_scalar(runtime_surface, "solwpv")?,
        )?,
        mofe_hourly_carry_arrays_enabled: lane_substeps == 24,
        lane_substeps,
        drainage_capacity_m: 0.0,
        drain_enabled,
        drain_depth_m,
        drain_spacing_m,
        drain_diameter_m,
        layers: layer_states.into_iter().map(Into::into).collect(),
    })
}

fn direct_publication_layer_states(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Vec<DirectSubsurfaceLayerState>, HillslopeCliError> {
    let nsl = direct_publication_layer_count(runtime_surface)?;
    let mut layers = Vec::with_capacity(nsl);
    for layer_index in 1..=nsl {
        layers.push(direct_publication_layer_state(
            runtime_surface,
            layer_index,
        )?);
    }
    Ok(layers)
}

fn direct_publication_layer_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    let nsl_symbol = if runtime_surface_symbol_value(runtime_surface, "wb11_nsl").is_some() {
        "wb11_nsl"
    } else {
        "nsl"
    };
    scalar_to_usize(
        nsl_symbol,
        require_runtime_surface_scalar(runtime_surface, nsl_symbol)?,
    )
}

fn direct_publication_frost_layer_carry_projection(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Option<Vec<DirectFrostLayerCarryProjection>>, HillslopeCliError> {
    let Some(wint_red) = runtime_surface_symbol_value(runtime_surface, "frost.options.wintRed")
    else {
        return Ok(None);
    };
    if wint_red.abs() <= 1.0e-12 {
        return Ok(None);
    }
    if (wint_red - 1.0).abs() > 1.0e-12 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} frost.options.wintRed must be 0 or 1, observed {wint_red}"
            ),
        });
    }
    let layer_count = direct_publication_layer_count(runtime_surface)?;
    let fine_top_count =
        direct_publication_frost_fine_count(runtime_surface, "frost.options.fineTop")?;
    let fine_bot_count =
        direct_publication_frost_fine_count(runtime_surface, "frost.options.fineBot")?;
    let mut projection = Vec::with_capacity(layer_count);
    for layer_index in 1..=layer_count {
        let depth_m = require_runtime_surface_scalar(
            runtime_surface,
            format!("wb19_dg_{layer_index:04}").as_str(),
        )?;
        if !depth_m.is_finite() || depth_m <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} wb19_dg_{layer_index:04} must be finite and > 0.0, observed {depth_m}"
                ),
            });
        }
        let fine_layer_count = direct_publication_frost_fine_layer_count(
            layer_index,
            layer_count,
            depth_m,
            fine_top_count,
            fine_bot_count,
        )?;
        let fine_layer_thickness_m =
            depth_m / usize_to_scalar("frost.runtime_nfine", fine_layer_count)?;
        projection.push(DirectFrostLayerCarryProjection {
            layer_index,
            fine_layer_count,
            fine_layer_thickness_m,
        });
    }
    Ok(Some(projection))
}

fn direct_publication_frost_fine_count(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<usize, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    let parsed = scalar_to_usize(symbol, value)?;
    if !(1..=10).contains(&parsed) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} {symbol} must be an integer in [1,10], observed {value}"
            ),
        });
    }
    Ok(parsed)
}

fn direct_publication_frost_fine_layer_count(
    layer_index: usize,
    layer_count: usize,
    depth_m: f64,
    fine_top_count: usize,
    fine_bot_count: usize,
) -> Result<usize, HillslopeCliError> {
    if layer_index != layer_count {
        return Ok(if layer_index < 3 {
            fine_top_count
        } else {
            fine_bot_count
        });
    }
    let spacing_mm = if layer_index > 2 {
        200.0 / usize_to_scalar("frost.options.fineBot", fine_bot_count)?
    } else {
        100.0 / usize_to_scalar("frost.options.fineTop", fine_top_count)?
    };
    let depth_mm = depth_m * 1_000.0;
    let depth_mm_trunc = depth_mm.trunc();
    let ratio_trunc = (depth_mm / spacing_mm).trunc();
    let mut count = format!("{ratio_trunc:.0}")
        .parse::<usize>()
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed converting frost fine layer ratio {ratio_trunc} to usize: {error}"
            ),
        })?;
    let count_trunc_mm =
        (usize_to_scalar("frost.runtime_nfine", count)? * spacing_mm).trunc();
    if (count_trunc_mm - depth_mm_trunc).abs() > 1.0e-12 {
        count += 1;
    }
    Ok(count.max(1))
}

fn direct_publication_layer_state(
    runtime_surface: &HillslopeWritebackSurface,
    layer_index: usize,
) -> Result<DirectSubsurfaceLayerState, HillslopeCliError> {
    let theta_m = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb18_perc_theta_{layer_index:04}").as_str(),
    )?;
    let field_capacity_m = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb18_perc_fc_{layer_index:04}").as_str(),
    )?;
    let upper_limit_m = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb18_perc_ul_{layer_index:04}").as_str(),
    )?;
    let conductivity_m_s = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb18_perc_ssc_{layer_index:04}").as_str(),
    )?;
    let lateral_conductivity_m_s =
        direct_publication_lateral_conductivity_m_s(runtime_surface, layer_index, conductivity_m_s)?;
    let depth_m = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_dg_{layer_index:04}").as_str(),
    )?;
    let residual_theta = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_thetdr_{layer_index:04}").as_str(),
    )?;
    let frozen_depth_m = runtime_surface_symbol_value(
        runtime_surface,
        format!("wb18_perc_frozen_depth_{layer_index:04}").as_str(),
    )
    .unwrap_or(0.0);
    let frozen_water_m = runtime_surface_symbol_value(
        runtime_surface,
        format!("wb18_perc_frzw_{layer_index:04}").as_str(),
    )
    .unwrap_or(0.0);
    let porosity = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_por_{layer_index:04}").as_str(),
    )?;
    let field_capacity_theta = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_thetfc_{layer_index:04}").as_str(),
    )?;
    let coca = require_preferred_or_legacy_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_coca_{layer_index:04}").as_str(),
        format!("coca_{layer_index:04}").as_str(),
    )?;
    Ok(DirectSubsurfaceLayerState::from(
        DirectSubsurfaceLayerInputs {
            theta_m,
            field_capacity_m,
            upper_limit_m,
            conductivity_m_s,
            depth_m,
            residual_theta,
            frozen_depth_m,
            frozen_water_m,
            porosity,
            field_capacity_theta,
            coca,
            lateral_conductivity_m_s,
        },
    ))
}

fn direct_publication_lateral_conductivity_m_s(
    runtime_surface: &HillslopeWritebackSurface,
    layer_index: usize,
    vertical_conductivity_m_s: f64,
) -> Result<f64, HillslopeCliError> {
    let lateral_symbol = format!("wb19_lateral_ssh_{layer_index:04}");
    if let Some(value) = runtime_surface_symbol_value(runtime_surface, lateral_symbol.as_str()) {
        if value.is_finite() && value > 0.0 {
            return Ok(value);
        }
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} {lateral_symbol} must be finite and > 0.0 for direct hourly WB19 lateral conductivity, observed {value}"
            ),
        });
    }

    let lane_substeps =
        runtime_surface_symbol_value(runtime_surface, "wb19_lateral_drain_lane_substeps")
            .unwrap_or(1.0);
    let solwpv = runtime_surface_symbol_value(runtime_surface, "solwpv").unwrap_or(0.0);
    if lane_substeps > 1.0 && solwpv >= 7778.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct hourly WB19 lateral conductivity requires {lateral_symbol}; substituting wb18_perc_ssc_{layer_index:04} violates INV-SUBHYD-027"
            ),
        });
    }
    Ok(vertical_conductivity_m_s)
}

fn direct_publication_evapotranspiration_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    include_stage_state: bool,
) -> Result<DirectEvapotranspirationComputeInputs, HillslopeCliError> {
    let pmet = if runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_evappm")
        .is_some_and(|value| value >= 0.5)
    {
        Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m: require_runtime_surface_scalar(runtime_surface, "pmet.es_m")?,
            plant_transpiration_m: require_runtime_surface_scalar(runtime_surface, "pmet.ep_m")?,
            soil_evaporation_storage_return_m: runtime_surface_symbol_value(
                runtime_surface,
                "pmet.es_storage_return_m",
            )
            .unwrap_or(0.0),
        })
    } else {
        None
    };
    let stage_state = if pmet.is_some() || !include_stage_state {
        None
    } else {
        direct_publication_stage_state(runtime_surface)?
    };
    Ok(DirectEvapotranspirationComputeInputs {
        et_demand_m: require_runtime_surface_scalar(runtime_surface, "wb11_et_demand")?,
        leaf_area_index: require_runtime_surface_scalar(runtime_surface, "lai")?,
        canopy_cover_fraction: require_runtime_surface_scalar(runtime_surface, "cancov")?,
        residue_interception_m: require_runtime_surface_scalar(
            runtime_surface,
            "wb17_residue_interception",
        )?,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: require_runtime_surface_scalar(runtime_surface, "rtd")?,
        plant_tolerance: require_preferred_or_legacy_runtime_surface_scalar(
            runtime_surface,
            "swu_effective_pltol",
            "pltol",
        )?,
        growth_context_required: false,
        stage_state,
        pmet,
    })
}

fn direct_publication_stage_state(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Option<DirectEvapotranspirationStageState>, HillslopeCliError> {
    let s1 = runtime_surface_symbol_value(runtime_surface, "s1");
    let s2 = runtime_surface_symbol_value(runtime_surface, "s2");
    let tu = runtime_surface_symbol_value(runtime_surface, "tu");
    let tv = runtime_surface_symbol_value(runtime_surface, "tv");
    match (s1, s2, tu, tv) {
        (None, None, None, None) => Ok(None),
        (Some(s1_m), Some(s2_m), Some(threshold_m), Some(counter)) => {
            Ok(Some(DirectEvapotranspirationStageState {
                s1_m,
                s2_m,
                threshold_m,
                counter,
            }))
        }
        _ => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication WB17 stage state requires complete s1/s2/tu/tv symbols"
            ),
        }),
    }
}

fn direct_publication_enabled_flag(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
) -> Result<bool, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    direct_publication_parse_enabled_flag(symbol, value)
}

fn direct_publication_optional_enabled_flag(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
) -> Result<Option<bool>, HillslopeCliError> {
    runtime_surface_symbol_value(runtime_surface, symbol)
        .map(|value| direct_publication_parse_enabled_flag(symbol, value))
        .transpose()
}

fn direct_publication_parse_enabled_flag(
    symbol: &'static str,
    value: f64,
) -> Result<bool, HillslopeCliError> {
    if value.abs() <= 1.0e-12 {
        Ok(false)
    } else if (value - 1.0).abs() <= 1.0e-12 {
        Ok(true)
    } else {
        Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} {symbol} must be 0 or 1, observed {value}"),
        })
    }
}

fn direct_publication_required_positive_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
) -> Result<f64, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    if value.is_finite() && value > 0.0 {
        return Ok(value);
    }
    Err(HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!(
            "{SIMOUT_GUARD_ID} {symbol} must be finite and > 0.0 for direct publication, observed {value}"
        ),
    })
}

fn require_preferred_or_legacy_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    preferred_symbol: &str,
    legacy_symbol: &str,
) -> Result<f64, HillslopeCliError> {
    if let Some(value) = runtime_surface_symbol_value(runtime_surface, preferred_symbol) {
        return Ok(value);
    }
    require_runtime_surface_scalar(runtime_surface, legacy_symbol)
}

fn direct_publication_profile_inputs(
    static_runtime_surface: &HillslopeWritebackSurface,
) -> Result<DirectHydrologyProjectionInputs, HillslopeCliError> {
    let profile_depth_m = direct_publication_static_mm_to_m(
        static_runtime_surface,
        "wb13_profile_depth_mm",
        true,
    )?;
    let profile_porosity_cap_m = direct_publication_static_mm_to_m(
        static_runtime_surface,
        "wb13_profile_porosity_cap_mm",
        false,
    )?;
    let profile_field_capacity_m =
        derive_profile_fc_store_from_authoritative_layers(static_runtime_surface)? / 1_000.0;
    let profile_wilting_point_m = direct_publication_static_mm_to_m(
        static_runtime_surface,
        "wb13_profile_wp_store_mm",
        false,
    )?;
    if profile_porosity_cap_m < profile_field_capacity_m {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} parsed profile porosity cap must be >= field capacity store"
            ),
        });
    }
    if profile_field_capacity_m < profile_wilting_point_m {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} parsed profile field capacity store must be >= wilting point store"
            ),
        });
    }
    Ok(DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 1.0e-9,
        profile_depth_m: Some(profile_depth_m),
        profile_porosity_cap_m: Some(profile_porosity_cap_m),
        profile_field_capacity_m: Some(profile_field_capacity_m),
        profile_wilting_point_m: Some(profile_wilting_point_m),
        ..DirectHydrologyProjectionInputs::zero()
    })
}

fn direct_publication_static_mm_to_m(
    static_runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
    require_positive: bool,
) -> Result<f64, HillslopeCliError> {
    let value_mm = require_runtime_surface_scalar(static_runtime_surface, symbol)?;
    if !value_mm.is_finite()
        || if require_positive {
            value_mm <= 0.0
        } else {
            value_mm < 0.0
        }
    {
        let comparator = if require_positive { "> 0.0" } else { ">= 0.0" };
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} parsed direct publication profile symbol {symbol} must be finite and {comparator}, observed {value_mm}"
            ),
        });
    }
    Ok(value_mm / 1_000.0)
}

fn direct_publication_calendar_days(
    climate_span: &ClimateRunSpanSummary,
) -> Result<Vec<DirectPublicationCalendarDay>, HillslopeCliError> {
    let mut calendar_days = Vec::with_capacity(climate_span.days.len());
    for day in &climate_span.days {
        calendar_days.push(direct_publication_calendar_day(day)?);
    }
    Ok(calendar_days)
}

fn direct_publication_calendar_day(
    day: &ClimateDayProjection,
) -> Result<DirectPublicationCalendarDay, HillslopeCliError> {
    let month = i8::try_from(day.month).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!(
            "{SIMOUT_GUARD_ID} direct publication month out of i8 range: {}",
            day.month
        ),
    })?;
    let day_of_month =
        i8::try_from(day.day_of_month).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication day-of-month out of i8 range: {}",
                day.day_of_month
            ),
        })?;
    let water_year = if day.month >= 10 {
        day.year + 1
    } else {
        day.year
    };
    let water_year =
        i16::try_from(water_year).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} direct publication water-year out of i16 range"),
        })?;
    Ok(DirectPublicationCalendarDay {
        year: day.year,
        julian_day: day.julian_day,
        month,
        day_of_month,
        water_year,
    })
}

fn validate_direct_publication_artifacts(
    artifacts: &DirectPublicationArtifacts,
) -> Result<(), HillslopeCliError> {
    let frame = &artifacts.execution.publication_frame;
    let row_count = frame.rows().len();
    let pass_row_count = frame.identity.day_count;
    if row_count == 0
        || artifacts.hbp_bytes.is_empty()
        || artifacts.wat_rows.len() != row_count
        || artifacts.pass_projection_rows.len() != pass_row_count
        || artifacts.loss_text.is_empty()
        || artifacts.manifest_text.is_empty()
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication consumers failed frame row-count validation"
            ),
        });
    }
    Ok(())
}

fn build_direct_publication_manifest_provenance(
    publication: &DirectRunPublicationFrame,
) -> Result<
    (
        HillslopeWb13PublicationProvenance,
        HillslopeMofeHourlyCarryProvenance,
    ),
    HillslopeCliError,
> {
    let facts = direct_publication_manifest_facts(publication)?;
    Ok((
        build_direct_publication_wb13_manifest_provenance(&facts)?,
        build_direct_publication_mofe_hourly_carry_provenance(&facts)?,
    ))
}

struct DirectPublicationManifestFacts<'a> {
    rows: &'a [openwepp_hillslope_orchestrator::DirectPublicationDayRow],
    first_row: &'a openwepp_hillslope_orchestrator::DirectPublicationDayRow,
    last_row: &'a openwepp_hillslope_orchestrator::DirectPublicationDayRow,
    contributor_ofe_count: usize,
    expected_row_count: usize,
    publishes_per_ofe_records: bool,
    sim_day_index_monotonic: bool,
    publication_area_m2: f64,
}

fn direct_publication_manifest_facts(
    publication: &DirectRunPublicationFrame,
) -> Result<DirectPublicationManifestFacts<'_>, HillslopeCliError> {
    let rows = publication.rows();
    let first_row = rows.first().ok_or_else(|| {
        direct_publication_cutover_blocked(
            "direct publication manifest provenance requires at least one row",
        )
    })?;
    let last_row = rows.last().ok_or_else(|| {
        direct_publication_cutover_blocked(
            "direct publication manifest provenance requires at least one row",
        )
    })?;
    let contributor_ofe_count = publication.identity.lane_count;
    if contributor_ofe_count == 0 {
        return Err(direct_publication_cutover_blocked(
            "direct publication manifest provenance requires at least one lane",
        ));
    }
    let expected_row_count = publication
        .identity
        .lane_count
        .checked_mul(publication.identity.day_count)
        .ok_or_else(|| {
            direct_publication_cutover_blocked(
                "direct publication manifest expected row count overflowed",
            )
        })?;
    if rows.len() != expected_row_count {
        return Err(direct_publication_cutover_blocked(format!(
            "direct publication manifest row count mismatch: expected {expected_row_count}, actual {}",
            rows.len()
        )));
    }
    let publishes_per_ofe_records = contributor_ofe_count > 1;
    let sim_day_index_monotonic = rows
        .windows(2)
        .all(|pair| pair[0].sim_day_index <= pair[1].sim_day_index);
    let mut area_by_ofe = BTreeMap::new();
    for row in rows {
        if !row.area_m2.is_finite() || row.area_m2 <= 0.0 {
            return Err(direct_publication_cutover_blocked(format!(
                "direct publication manifest row area must be finite and > 0.0, observed {}",
                row.area_m2
            )));
        }
        if let Some(existing) = area_by_ofe.insert(row.ofe_id, row.area_m2) {
            if existing.to_bits() != row.area_m2.to_bits() {
                return Err(direct_publication_cutover_blocked(format!(
                    "direct publication manifest area changed for OFE {}: first={}, observed={}",
                    row.ofe_id, existing, row.area_m2
                )));
            }
        }
    }
    if area_by_ofe.len() != contributor_ofe_count {
        return Err(direct_publication_cutover_blocked(format!(
            "direct publication manifest area lane count mismatch: expected {contributor_ofe_count}, observed {}",
            area_by_ofe.len()
        )));
    }
    let publication_area_m2 = area_by_ofe.values().sum();
    Ok(DirectPublicationManifestFacts {
        rows,
        first_row,
        last_row,
        contributor_ofe_count,
        expected_row_count,
        publishes_per_ofe_records,
        sim_day_index_monotonic,
        publication_area_m2,
    })
}

fn build_direct_publication_wb13_manifest_provenance(
    facts: &DirectPublicationManifestFacts<'_>,
) -> Result<HillslopeWb13PublicationProvenance, HillslopeCliError> {
    let publishes_per_ofe_records = facts.publishes_per_ofe_records;
    let identity_status = if publishes_per_ofe_records {
        MF_IDENTITY_STATUS
    } else {
        "pass-direct-publication-frame"
    };
    Ok(HillslopeWb13PublicationProvenance {
        source: WB13_PUBLICATION_SOURCE_DIRECT_PUBLICATION_FRAME.to_string(),
        projection_fallback_used: false,
        guard_id: SIMOUT_GUARD_ID.to_string(),
        replay_candidate_surfaces: Vec::new(),
        publication_ofe_policy: if publishes_per_ofe_records {
            MF_PUBLICATION_OFE_POLICY
        } else {
            MOFE04_PUBLICATION_OFE_POLICY
        }
        .to_string(),
        contributor_ofe_count: facts.contributor_ofe_count,
        static_per_ofe_slice_count: facts.contributor_ofe_count,
        per_ofe_state_policy: if publishes_per_ofe_records {
            MF_PER_OFE_STATE_POLICY
        } else {
            "direct-publication-frame-state"
        }
        .to_string(),
        per_ofe_dynamic_water_balance_state: true,
        per_ofe_dynamic_wb_state: true,
        per_ofe_record_count: direct_manifest_per_ofe_value(publishes_per_ofe_records, facts.rows.len()),
        transfer_identity_status: identity_status.to_string(),
        per_element_identity_status: identity_status.to_string(),
        aggregate_identity_status: identity_status.to_string(),
        area_policy: MOFE04_PUBLICATION_AREA_POLICY.to_string(),
        storage_lineage_policy: if publishes_per_ofe_records {
            MF_STORAGE_LINEAGE_POLICY
        } else {
            "direct-publication-frame-state"
        }
        .to_string(),
        per_ofe_internal_day_count: direct_manifest_per_ofe_value(
            publishes_per_ofe_records,
            facts.expected_row_count / facts.contributor_ofe_count,
        ),
        per_ofe_expected_record_count: direct_manifest_per_ofe_value(
            publishes_per_ofe_records,
            facts.expected_row_count,
        ),
        transfer_identity_max_abs_mm: 0.0,
        per_element_identity_max_abs_mm: 0.0,
        aggregate_transfer_cancellation_max_abs_mm: 0.0,
        hillslope_total_identity_max_abs_mm: 0.0,
        publication_area_m2: facts.publication_area_m2,
        row_count: facts.rows.len(),
        sim_day_index_monotonic: facts.sim_day_index_monotonic,
        first_row_key: direct_publication_row_key_provenance(facts.first_row)?,
        last_row_key: direct_publication_row_key_provenance(facts.last_row)?,
    })
}

fn direct_manifest_per_ofe_value(active: bool, value: usize) -> usize {
    if active {
        value
    } else {
        0
    }
}

fn build_direct_publication_mofe_hourly_carry_provenance(
    facts: &DirectPublicationManifestFacts<'_>,
) -> Result<HillslopeMofeHourlyCarryProvenance, HillslopeCliError> {
    let upstream_carry_total_m = if facts.publishes_per_ofe_records {
        sum_direct_publication_upstream_carry_m(facts.rows)?
    } else {
        0.0
    };
    let current_carry_total_m = upstream_carry_total_m;
    Ok(HillslopeMofeHourlyCarryProvenance {
        policy: if facts.publishes_per_ofe_records {
            MOFE_HOURLY_CARRY_POLICY
        } else {
            "single-ofe-direct-publication-no-carry"
        }
        .to_string(),
        active: facts.publishes_per_ofe_records,
        substep_count: if facts.publishes_per_ofe_records {
            MOFE_HOURLY_CARRY_ARRAY_COUNT
        } else {
            0
        },
        required_arrays: if facts.publishes_per_ofe_records {
            MOFE_HOURLY_REQUIRED_ARRAYS
                .iter()
                .map(|root| (*root).to_string())
                .collect()
        } else {
            Vec::new()
        },
        upstream_carry_total_m,
        current_carry_total_m,
    })
}

fn sum_direct_publication_upstream_carry_m(
    rows: &[openwepp_hillslope_orchestrator::DirectPublicationDayRow],
) -> Result<f64, HillslopeCliError> {
    let mut total_mm = 0.0_f64;
    for row in rows {
        let upstream_surface_mm = row.transfer.upstream_surface_mm;
        let upstream_lateral_mm = row.transfer.upstream_lateral_mm;
        if !upstream_surface_mm.is_finite()
            || upstream_surface_mm < 0.0
            || !upstream_lateral_mm.is_finite()
            || upstream_lateral_mm < 0.0
        {
            return Err(direct_publication_cutover_blocked(format!(
                "direct publication manifest carry totals require finite nonnegative transfer operands, observed upstream_surface_mm={} upstream_lateral_mm={} for OFE {} sim day {}",
                upstream_surface_mm, upstream_lateral_mm, row.ofe_id, row.sim_day_index
            )));
        }
        total_mm += upstream_surface_mm + upstream_lateral_mm;
    }
    let total_m = total_mm / 1000.0;
    if !total_m.is_finite() || total_m < 0.0 {
        return Err(direct_publication_cutover_blocked(format!(
            "direct publication manifest carry total is invalid: {total_m}"
        )));
    }
    Ok(total_m)
}

fn direct_publication_row_key_provenance(
    row: &openwepp_hillslope_orchestrator::DirectPublicationDayRow,
) -> Result<HillslopeWb13RowKeyProvenance, HillslopeCliError> {
    Ok(HillslopeWb13RowKeyProvenance {
        year: row.calendar.year,
        julian_day: row.calendar.julian_day,
        ofe: u16::try_from(row.ofe_id).map_err(|_| {
            direct_publication_cutover_blocked(format!(
                "direct publication manifest OFE id {} exceeds u16 range",
                row.ofe_id
            ))
        })?,
        sim_day_index: row.sim_day_index,
    })
}

#[cfg(test)]
fn reduced_pass_mismatch_fields(
    direct_rows: &[HillslopePassRow],
    compatibility_rows: &[HillslopePassRow],
) -> Vec<&'static str> {
    let mut mismatches = BTreeSet::new();
    if direct_rows.len() != compatibility_rows.len() {
        mismatches.insert("row_count");
    }
    for (direct, compatibility) in direct_rows.iter().zip(compatibility_rows) {
        insert_mismatch_if(
            &mut mismatches,
            "wepp_id",
            direct.wepp_id != compatibility.wepp_id,
        );
        insert_mismatch_if(&mut mismatches, "year", direct.year != compatibility.year);
        insert_mismatch_if(
            &mut mismatches,
            "sim_day_index",
            direct.sim_day_index != compatibility.sim_day_index,
        );
        insert_mismatch_if(
            &mut mismatches,
            "julian",
            direct.julian != compatibility.julian,
        );
        insert_mismatch_if(&mut mismatches, "month", direct.month != compatibility.month);
        insert_mismatch_if(
            &mut mismatches,
            "day_of_month",
            direct.day_of_month != compatibility.day_of_month,
        );
        insert_mismatch_if(
            &mut mismatches,
            "water_year",
            direct.water_year != compatibility.water_year,
        );
        insert_float_mismatch(
            &mut mismatches,
            "runvol",
            direct.runvol_m3,
            compatibility.runvol_m3,
        );
        insert_float_mismatch(
            &mut mismatches,
            "sbrunv",
            direct.sbrunv_m3,
            compatibility.sbrunv_m3,
        );
        insert_float_mismatch(
            &mut mismatches,
            "peakro",
            direct.peakro_m3_s,
            compatibility.peakro_m3_s,
        );
        insert_float_mismatch(
            &mut mismatches,
            "total_detachment",
            direct.total_detachment_kg,
            compatibility.total_detachment_kg,
        );
        insert_float_mismatch(
            &mut mismatches,
            "total_deposition",
            direct.total_deposition_kg,
            compatibility.total_deposition_kg,
        );
        for (index, (direct_fraction, compatibility_fraction)) in direct
            .sediment_concentration_kg_m3
            .iter()
            .zip(compatibility.sediment_concentration_kg_m3)
            .enumerate()
        {
            insert_float_mismatch(
                &mut mismatches,
                match index {
                    0 => "sediment_concentration_1",
                    1 => "sediment_concentration_2",
                    2 => "sediment_concentration_3",
                    3 => "sediment_concentration_4",
                    _ => "sediment_concentration_5",
                },
                *direct_fraction,
                compatibility_fraction,
            );
        }
    }
    pass_mismatch_field_order()
        .iter()
        .copied()
        .filter(|field| mismatches.contains(field))
        .collect()
}

#[cfg(test)]
fn pass_mismatch_field_order() -> &'static [&'static str] {
    &[
        "row_count",
        "wepp_id",
        "year",
        "sim_day_index",
        "julian",
        "month",
        "day_of_month",
        "water_year",
        "runvol",
        "sbrunv",
        "peakro",
        "total_detachment",
        "total_deposition",
        "sediment_concentration_1",
        "sediment_concentration_2",
        "sediment_concentration_3",
        "sediment_concentration_4",
        "sediment_concentration_5",
    ]
}

fn direct_publication_runtime_error(
    source: &openwepp_hillslope_orchestrator::DirectRuntimeError,
) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: source.to_string(),
    }
}

fn direct_publication_day_input_build_error(error: &HillslopeCliError) -> DirectRuntimeError {
    DirectRuntimeError::PublicationDayInputBuildFailure {
        detail: error.to_string(),
    }
}
