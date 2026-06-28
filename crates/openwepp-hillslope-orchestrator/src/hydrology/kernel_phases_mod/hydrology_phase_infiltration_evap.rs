#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

impl Wb11HydrologyKernel {
pub(crate) fn resolve_snow_partition_terms(
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
        snow_coupling: &SnowCouplingOutcome,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
        let runoff_snow_term = snow_coupling.signed_s
            + snow_coupling.accumulation
            + snow_coupling.rain_retained
            + snow_coupling.rain_released;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow.routed_melt_m"),
            runoff_snow_term,
            Some(0.0),
            None,
        )?;
        let runoff_snow_term =
            Self::normalize_non_negative_within_tolerance(runoff_snow_term);
        let hyetograph_liquid_input_raw = hyetograph_rainfall
            - snow_coupling.accumulation
            - snow_coupling.rain_retained
            - snow_coupling.rain_released;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_liquid_input_raw,
            Some(0.0),
            None,
        )?;
        let hyetograph_liquid_input =
            Self::normalize_non_negative_within_tolerance(hyetograph_liquid_input_raw);
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow.post_winter_rain_m"),
            hyetograph_liquid_input,
            Some(0.0),
            None,
        )?;

        Ok((runoff_snow_term, hyetograph_liquid_input))
    }

    #[allow(clippy::too_many_lines)]
pub(crate) fn solve_ponded_cumulative_infiltration(
        phase_class: HillslopeKernelPhaseClass,
        conductivity: f64,
        matric_potential: f64,
        cumulative_start: f64,
        duration: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if duration <= WB11_ZERO_THRESHOLD {
            return Ok(cumulative_start);
        }
        if matric_potential <= WB11_ZERO_THRESHOLD {
            return Ok(cumulative_start + conductivity * duration);
        }

        let rhs = conductivity * duration;
        let start_plus_matric = cumulative_start + matric_potential;
        if start_plus_matric <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: cumulative_start,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let residual = |candidate: f64| {
            (candidate - cumulative_start)
                - matric_potential * ((candidate + matric_potential) / start_plus_matric).ln()
                - rhs
        };

        let mut lower = cumulative_start;
        let mut upper = cumulative_start + conductivity * duration + matric_potential;
        if upper <= lower {
            upper = lower + 1.0;
        }

        let mut upper_residual = residual(upper);
        if !upper_residual.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: upper,
                minimum: Some(cumulative_start),
                maximum: None,
            });
        }

        let mut expansion_steps = 0_usize;
        while upper_residual < 0.0 {
            upper = upper * 2.0 + 1.0;
            if !upper.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
            upper_residual = residual(upper);
            if !upper_residual.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
            expansion_steps += 1;
            if expansion_steps > 128 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
        }

        for _ in 0..128 {
            let midpoint = 0.5 * (lower + upper);
            let midpoint_residual = residual(midpoint);
            if !midpoint_residual.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: midpoint,
                    minimum: Some(cumulative_start),
                    maximum: Some(upper),
                });
            }
            if midpoint_residual > 0.0 {
                upper = midpoint;
            } else {
                lower = midpoint;
            }

            let tolerance = 1.0e-10 * upper.max(1.0);
            if (upper - lower) <= tolerance {
                break;
            }
        }

        let solution = 0.5 * (lower + upper);
        if !solution.is_finite() || solution < cumulative_start - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: solution,
                minimum: Some(cumulative_start),
                maximum: None,
            });
        }

        Ok(solution)
    }
pub(crate) fn compute_interval_infiltration_depth(
        phase_class: HillslopeKernelPhaseClass,
        conductivity: f64,
        matric_potential: f64,
        cumulative_infiltration_start: f64,
        rainfall_rate: f64,
        interval_duration: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if interval_duration <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                value: interval_duration,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let interval_rainfall_depth = rainfall_rate * interval_duration;
        if !interval_rainfall_depth.is_finite() || interval_rainfall_depth < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: interval_rainfall_depth,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        if rainfall_rate <= conductivity + WB11_ZERO_THRESHOLD {
            return Ok(interval_rainfall_depth.max(0.0));
        }

        let interval_infiltration = if matric_potential <= WB11_ZERO_THRESHOLD {
            conductivity * interval_duration
        } else {
            let denominator = rainfall_rate - conductivity;
            if denominator <= WB11_ZERO_THRESHOLD {
                interval_rainfall_depth
            } else {
                let ponding_threshold = (conductivity * matric_potential) / denominator;
                if !ponding_threshold.is_finite() || ponding_threshold < 0.0 {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                        value: ponding_threshold,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }

                if cumulative_infiltration_start >= ponding_threshold - WB11_ZERO_THRESHOLD {
                    let cumulative_end = Self::solve_ponded_cumulative_infiltration(
                        phase_class,
                        conductivity,
                        matric_potential,
                        cumulative_infiltration_start,
                        interval_duration,
                    )?;
                    cumulative_end - cumulative_infiltration_start
                } else {
                    let infiltration_to_ponding =
                        (ponding_threshold - cumulative_infiltration_start).max(0.0);
                    let time_to_ponding = infiltration_to_ponding / rainfall_rate;

                    if time_to_ponding >= interval_duration - WB11_ZERO_THRESHOLD {
                        interval_rainfall_depth
                    } else {
                        let ponded_duration = interval_duration - time_to_ponding;
                        let cumulative_end = Self::solve_ponded_cumulative_infiltration(
                            phase_class,
                            conductivity,
                            matric_potential,
                            ponding_threshold,
                            ponded_duration,
                        )?;
                        infiltration_to_ponding + (cumulative_end - ponding_threshold)
                    }
                }
            }
        };

        if !interval_infiltration.is_finite()
            || interval_infiltration < -WB11_ZERO_THRESHOLD
            || interval_infiltration
                > interval_rainfall_depth + WB14_INTERVAL_INFILTRATION_ROUNDOFF_TOLERANCE_M
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: interval_infiltration,
                minimum: Some(0.0),
                maximum: Some(interval_rainfall_depth),
            });
        }

        let non_negative_infiltration = if interval_infiltration < 0.0 {
            0.0
        } else {
            interval_infiltration
        };
        Ok(non_negative_infiltration.min(interval_rainfall_depth))
    }
pub(crate) fn status_from_guard_error(error: &Wb11HydrologyKernelGuardError) -> SimulationStatus {
        let code = error.code();
        let status_result = match error.boundary_class() {
            BoundaryClass::NonFinite => {
                SimulationStatus::non_finite_failure(SimulationPhase::HillslopeKernel, code)
            }
            BoundaryClass::MissingRequiredInput | BoundaryClass::DomainViolation => {
                SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    error.boundary_class(),
                    code,
                )
            }
            _ => SimulationStatus::failure(
                SimulationPhase::HillslopeKernel,
                true,
                false,
                BoundaryClass::DomainViolation,
                "HKERNEL-WB11-GEN-E-003",
            ),
        };

        match status_result {
            Ok(status) => status,
            Err(_) => unreachable!("status message ids are non-empty WB11 constants"),
        }
    }

    #[allow(clippy::too_many_lines)]
pub(crate) fn compute_same_pass_wb14_infiltration_lineage(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let Some(rainfall_input) =
            Self::optional_state_scalar(request, phase_class, WB12_SYMBOL_RAINFALL_INPUT)?
        else {
            return Ok(None);
        };
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            rainfall_input,
            Some(0.0),
            None,
        )?;

        let soil_conductivity =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_CONDUCTIVITY)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_CONDUCTIVITY,
            soil_conductivity,
            Some(0.0),
            None,
        )?;
        let soil_conductivity = Self::resolve_wb14_effective_soil_conductivity(
            request,
            phase_class,
            soil_conductivity,
        )?;

        let active_frost_coupling = Self::resolve_active_frost_coupling(request, phase_class)?;
        let frost_coupling = if active_frost_coupling {
            Some(Self::compute_active_frost_coupling(
                request,
                phase_class,
                soil_conductivity,
            )?)
        } else {
            None
        };
        let infiltration_conductivity = frost_coupling
            .as_ref()
            .map_or(soil_conductivity, |outcome| outcome.infcap_frz);

        let soil_layer_depth =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_LAYER_DEPTH)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_LAYER_DEPTH,
            soil_layer_depth,
            Some(0.0),
            None,
        )?;
        let theta_residual =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_THETA_RESIDUAL)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_RESIDUAL,
            theta_residual,
            Some(0.0),
            None,
        )?;
        let theta_field_capacity = Self::require_state_scalar(
            request,
            phase_class,
            WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY,
            theta_field_capacity,
            Some(0.0),
            None,
        )?;
        let moisture_deficit = theta_field_capacity - theta_residual;
        if moisture_deficit < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY),
                value: theta_field_capacity,
                minimum: Some(theta_residual),
                maximum: None,
            });
        }
        let matric_potential = soil_layer_depth * moisture_deficit.max(0.0);
        if !matric_potential.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: matric_potential,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let hyetograph_point_count = Self::resolve_hyetograph_point_count(request, phase_class)?;
        let (times, intensities) =
            Self::load_hyetograph_series(request, phase_class, hyetograph_point_count)?;

        let mut hyetograph_rainfall = 0.0_f64;
        for index in 0..times.len().saturating_sub(1) {
            let interval_rainfall = intensities[index] * (times[index + 1] - times[index]);
            if !interval_rainfall.is_finite() || interval_rainfall < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                    value: interval_rainfall,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            hyetograph_rainfall += interval_rainfall.max(0.0);
        }
        if !hyetograph_rainfall.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: hyetograph_rainfall,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let hyetograph_duration_s = if times.len() >= 2 {
            times[times.len() - 1] - times[0]
        } else {
            0.0
        };
        let active_irrigation_event =
            Self::resolve_active_irrigation_event(request, phase_class, hyetograph_duration_s)?;
        let irrigation_depth_m = active_irrigation_event.map_or(0.0, |event| event.depth_m);
        let irrigation_duration_s = active_irrigation_event.map_or(0.0, |event| event.duration_s);
        let irrigation_rate_m_per_s =
            active_irrigation_event.map_or(0.0, |event| event.rate_m_per_s);
        let coupled_rainfall_input = hyetograph_rainfall + irrigation_depth_m;
        if (rainfall_input - coupled_rainfall_input).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: rainfall_input - coupled_rainfall_input,
                minimum: Some(-WB11_ZERO_THRESHOLD),
                maximum: Some(WB11_ZERO_THRESHOLD),
            });
        }

        let runtime_swe = Self::validate_runtime_snow_state_domains(request, phase_class)?;
        if hyetograph_rainfall <= WB11_ZERO_THRESHOLD
            && irrigation_depth_m <= WB11_ZERO_THRESHOLD
            && runtime_swe <= WB11_ZERO_THRESHOLD
        {
            return Ok(None);
        }

        let active_snow_coupling = Self::resolve_active_snow_coupling(request, phase_class)?;
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling(request, phase_class, hyetograph_rainfall)?
        } else {
            SnowCouplingOutcome {
                signed_s: 0.0,
                accumulation: 0.0,
                rain_retained: 0.0,
                rain_released: 0.0,
                liquid_holding_capacity: 0.0,
                liquid_water_retained: 0.0,
                liquid_water_released: 0.0,
                sublimation: 0.0,
                raw_melt: 0.0,
                redistributed_melt: 0.0,
                snowpack_state_loss: 0.0,
                runtime_swe: 0.0,
                runtime_depth_m: 0.0,
                runtime_density_kg_m3: 0.0,
                runtime_settle_day_count: 0.0,
                snow_albedo_state_after: None,
                hourly_state: Vec::new(),
            }
        };
        let (runoff_snow_term, hyetograph_liquid_input) =
            Self::resolve_snow_partition_terms(phase_class, hyetograph_rainfall, &snow_coupling)?;

        let interception =
            Self::compute_canopy_interception_depth(request, phase_class, hyetograph_liquid_input)?;
        let (_liquid_after_interception, rainfall_scale) = Self::resolve_interception_rainfall_scale(
            phase_class,
            hyetograph_rainfall,
            hyetograph_liquid_input,
            interception,
        )?;

        let cumulative_infiltration = Self::compute_coupled_infiltration_depth(
            phase_class,
            infiltration_conductivity,
            matric_potential,
            &times,
            &intensities,
            rainfall_scale,
            runoff_snow_term,
            &snow_coupling.hourly_state,
            irrigation_rate_m_per_s,
            irrigation_duration_s,
        )?;
        let cumulative_infiltration = Self::apply_wb14_storage_limit_to_infiltration(
            request,
            phase_class,
            cumulative_infiltration,
        )?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_INFILTRATION,
            cumulative_infiltration,
            Some(0.0),
            None,
        )?;
        Ok(Some(cumulative_infiltration))
    }
    pub(crate) fn resolve_wb18_same_pass_infiltration_lineage(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let should_reconstruct =
            Self::wb18_should_reconstruct_same_pass_infiltration_lineage(request, phase_class)?;
        let reconstruct_zero_infiltration = should_reconstruct
            && (Self::wb18_same_pass_reconstruction_daily_lane(request, phase_class)?
                || Self::wb18_storage_target_includes_same_pass_ingress(request, phase_class)?
                || Self::resolve_mofe_hourly_carry_arrays_enabled(request, phase_class)?);
        let infiltration_symbol = BoundarySymbol::from(WB12_SYMBOL_INFILTRATION);
        if let Some(infiltration) =
            Self::optional_state_scalar_for_symbol(request, phase_class, &infiltration_symbol)?
        {
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_INFILTRATION,
                infiltration,
                Some(0.0),
                None,
            )?;
            if infiltration > WB11_ZERO_THRESHOLD || !reconstruct_zero_infiltration {
                Self::validate_runtime_snow_state_domains(request, phase_class)?;
                return Ok(Some(infiltration));
            }
        }

        if reconstruct_zero_infiltration {
            return Self::compute_same_pass_wb14_infiltration_lineage(request, phase_class);
        }

        Self::validate_runtime_snow_state_domains(request, phase_class)?;

        Ok(None)
    }
    fn wb18_same_pass_reconstruction_daily_lane(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let lane_substeps_symbol = BoundarySymbol::from("wb18_perc_lane_substeps");
        let lane_substeps_raw =
            Self::optional_state_scalar_for_symbol(request, phase_class, &lane_substeps_symbol)?
                .unwrap_or(1.0);
        if lane_substeps_raw < 1.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: lane_substeps_symbol.clone(),
                value: lane_substeps_raw,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let lane_substeps = lane_substeps_raw.round();
        if (lane_substeps_raw - lane_substeps).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: lane_substeps_symbol,
                value: lane_substeps_raw,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        Ok((lane_substeps - 1.0).abs() <= WB11_ZERO_THRESHOLD)
    }

    fn wb18_storage_target_includes_same_pass_ingress(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let Some(initial) = Self::optional_state_scalar(
            request,
            phase_class,
            WB12_SYMBOL_STORAGE_INITIAL,
        )?
        else {
            return Ok(false);
        };
        let Some(observed) = Self::optional_state_scalar(
            request,
            phase_class,
            WB12_SYMBOL_STORAGE_OBSERVED,
        )?
        else {
            return Ok(false);
        };
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_INITIAL,
            initial,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_OBSERVED,
            observed,
            Some(0.0),
            None,
        )?;
        Ok(observed > initial + WB11_ZERO_THRESHOLD)
    }
pub(crate) fn wb18_should_reconstruct_same_pass_infiltration_lineage(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        if let Some(rainfall_input) =
            Self::optional_state_scalar(request, phase_class, WB12_SYMBOL_RAINFALL_INPUT)?
        {
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_RAINFALL_INPUT,
                rainfall_input,
                Some(0.0),
                None,
            )?;
            if rainfall_input > WB11_ZERO_THRESHOLD {
                return Ok(true);
            }
        }

        let runtime_swe_symbol = BoundarySymbol::from("snow.runtime_swe");
        if let Some(runtime_swe) =
            Self::optional_state_scalar_for_symbol(request, phase_class, &runtime_swe_symbol)?
        {
            if runtime_swe > WB11_ZERO_THRESHOLD {
                return Ok(true);
            }
        }

        let irrigation_input =
            Self::optional_flux_scalar(request, phase_class, IRRIG_SYMBOL_DAILY_IRRIGATION)?
                .unwrap_or(0.0);
        Self::require_flux_range(
            phase_class,
            IRRIG_SYMBOL_DAILY_IRRIGATION,
            irrigation_input,
            Some(0.0),
            None,
        )?;

        Ok(irrigation_input > WB11_ZERO_THRESHOLD)
    }

    #[allow(clippy::too_many_lines)]
pub(crate) fn run_evapotranspiration(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyEvapotranspiration;
        let soil_water_initial =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_initial,
            Some(0.0),
            None,
        )?;

        let et_demand = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_ET_DEMAND)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_ET_DEMAND,
            et_demand,
            Some(0.0),
            None,
        )?;

        let lai = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_LAI)?;
        Self::require_state_range(phase_class, WB15_SYMBOL_PLANT_LAI, lai, Some(0.0), None)?;

        let canopy_cover =
            Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_CANCOV)?;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_PLANT_CANCOV,
            canopy_cover,
            Some(0.0),
            Some(WB15_CANCOV_MAX),
        )?;

        let residue_interception =
            Self::require_state_scalar(request, phase_class, WB17_SYMBOL_RESIDUE_INTERCEPTION)?;
        Self::require_state_range(
            phase_class,
            WB17_SYMBOL_RESIDUE_INTERCEPTION,
            residue_interception,
            Some(0.0),
            None,
        )?;

        let evappm_branch_symbol = BoundarySymbol::from("wb11_et_seed_branch_evappm");
        let evappm_branch = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &evappm_branch_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_state_range_for_symbol(
            phase_class,
            &evappm_branch_symbol,
            evappm_branch,
            Some(0.0),
            Some(1.0),
        )?;
        let evappm_pmet_components = if evappm_branch >= 0.5 {
            let pmet_soil_evaporation_symbol = BoundarySymbol::from("pmet.es_m");
            let pmet_transpiration_symbol = BoundarySymbol::from("pmet.ep_m");
            let pmet_storage_return_symbol = BoundarySymbol::from("pmet.es_storage_return_m");
            let pmet_soil_evaporation = Self::require_state_scalar_for_symbol(
                request,
                phase_class,
                &pmet_soil_evaporation_symbol,
            )?;
            let pmet_transpiration = Self::require_state_scalar_for_symbol(
                request,
                phase_class,
                &pmet_transpiration_symbol,
            )?;
            Self::require_state_range_for_symbol(
                phase_class,
                &pmet_soil_evaporation_symbol,
                pmet_soil_evaporation,
                Some(-WB11_ZERO_THRESHOLD),
                None,
            )?;
            let pmet_soil_evaporation =
                Self::normalize_non_negative_within_tolerance(pmet_soil_evaporation);
            Self::require_state_range_for_symbol(
                phase_class,
                &pmet_transpiration_symbol,
                pmet_transpiration,
                Some(0.0),
                None,
            )?;
            let pmet_storage_return = Self::optional_state_scalar_for_symbol(
                request,
                phase_class,
                &pmet_storage_return_symbol,
            )?
            .unwrap_or(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &pmet_storage_return_symbol,
                pmet_storage_return,
                Some(0.0),
                None,
            )?;
            Some((
                pmet_soil_evaporation,
                pmet_transpiration,
                pmet_storage_return,
            ))
        } else {
            None
        };

        let stage_s1_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_S1);
        let stage_s2_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_S2);
        let stage_threshold_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_TU);
        let stage_counter_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_TV);
        let stage_state = if evappm_pmet_components.is_some() {
            None
        } else {
            let stage_s1 =
                Self::optional_state_scalar_for_symbol(request, phase_class, &stage_s1_symbol)?;
            let stage_s2 =
                Self::optional_state_scalar_for_symbol(request, phase_class, &stage_s2_symbol)?;
            let stage_threshold = Self::optional_state_scalar_for_symbol(
                request,
                phase_class,
                &stage_threshold_symbol,
            )?;
            let stage_counter =
                Self::optional_state_scalar_for_symbol(request, phase_class, &stage_counter_symbol)?;
            match (stage_s1, stage_s2, stage_threshold, stage_counter) {
                (None, None, None, None) => None,
                (Some(s1), Some(s2), Some(tu), Some(tv)) => {
                    Self::require_state_range_for_symbol(
                        phase_class,
                        &stage_s1_symbol,
                        s1,
                        Some(0.0),
                        None,
                    )?;
                    Self::require_state_range_for_symbol(
                        phase_class,
                        &stage_s2_symbol,
                        s2,
                        Some(0.0),
                        None,
                    )?;
                    if tu <= WB11_ZERO_THRESHOLD {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: stage_threshold_symbol.clone(),
                            value: tu,
                            minimum: Some(WB11_ZERO_THRESHOLD),
                            maximum: None,
                        });
                    }
                    Self::require_state_range_for_symbol(
                        phase_class,
                        &stage_counter_symbol,
                        tv,
                        Some(0.0),
                        None,
                    )?;
                    Some((s1, s2, tu, tv))
                }
                _ => {
                    let missing_symbol = if stage_s1.is_none() {
                        stage_s1_symbol.clone()
                    } else if stage_s2.is_none() {
                        stage_s2_symbol.clone()
                    } else if stage_threshold.is_none() {
                        stage_threshold_symbol.clone()
                    } else {
                        stage_counter_symbol.clone()
                    };
                    return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: missing_symbol,
                    });
                }
            }
        };

        let (nsl_symbol, layer_count) = Self::require_wb11_layer_count(request, phase_class)?;
        if layer_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: nsl_symbol,
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }

        let mut layer_storage = Vec::with_capacity(layer_count);
        let mut layer_depth = Vec::with_capacity(layer_count);
        let mut layer_upper_limit = Vec::with_capacity(layer_count);
        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let theta =
                Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                theta,
                Some(0.0),
                None,
            )?;

            let (dg_symbol, dg) =
                Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &dg_symbol,
                dg,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let layer_ul =
                Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
            if layer_ul <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ul_symbol,
                    value: layer_ul,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            layer_storage.push(theta);
            layer_depth.push(dg);
            layer_upper_limit.push(layer_ul);
        }

        let mut stage_state_updates = Vec::new();
        let same_pass_infiltration = if stage_state.is_some() {
            Self::compute_same_pass_wb14_infiltration_lineage(request, phase_class)?
        } else {
            None
        };
        let post_et_outside_water_depth = if let Some(infiltration) = same_pass_infiltration {
            infiltration
        } else {
            let infiltration =
                Self::optional_state_scalar(request, phase_class, WB12_SYMBOL_INFILTRATION)?
                    .unwrap_or(0.0);
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_INFILTRATION,
                infiltration,
                Some(0.0),
                None,
            )?;
            infiltration
        };
        let pmet_component_mode = evappm_pmet_components.is_some();
        let (soil_evaporation_with_residue, transpiration_partition_potential) =
            if let Some((pmet_es_m, pmet_ep_m, pmet_es_storage_return_m)) = evappm_pmet_components {
                Self::require_flux_range(
                    phase_class,
                    WB17_SYMBOL_ES,
                    pmet_es_m,
                    Some(0.0),
                    None,
                )?;
                Self::require_flux_range(
                    phase_class,
                    WB17_SYMBOL_EP,
                    pmet_ep_m,
                    Some(0.0),
                    None,
                )?;
                if let Some(top_layer_storage) = layer_storage.first_mut() {
                    *top_layer_storage += pmet_es_storage_return_m;
                }
                (pmet_es_m, pmet_ep_m)
            } else {
                let soil_evaporation_partition_potential = et_demand
                    * (-WB17_CANOPY_EAJ_COEFFICIENT
                        * (canopy_cover + WB17_CANOPY_BARE_SOIL_OFFSET))
                        .exp();
                Self::require_flux_range(
                    phase_class,
                    WB17_SYMBOL_ES,
                    soil_evaporation_partition_potential,
                    Some(0.0),
                    Some(et_demand),
                )?;

                let transpiration_partition_potential =
                    if lai > WB17_TRANSPIRATION_LAI_FULL_COVER {
                        et_demand
                    } else {
                        lai * et_demand / WB17_TRANSPIRATION_LAI_FULL_COVER
                    };
                Self::require_flux_range(
                    phase_class,
                    WB17_SYMBOL_EP,
                    transpiration_partition_potential,
                    Some(0.0),
                    Some(et_demand),
                )?;

                let residue_evaporation =
                    residue_interception.min(soil_evaporation_partition_potential);
                Self::require_flux_range(
                    phase_class,
                    WB17_SYMBOL_ER,
                    residue_evaporation,
                    Some(0.0),
                    Some(soil_evaporation_partition_potential),
                )?;

                let soil_evaporation_potential =
                    soil_evaporation_partition_potential - residue_evaporation;
                Self::require_flux_range(
                    phase_class,
                    WB17_SYMBOL_ES,
                    soil_evaporation_potential,
                    Some(0.0),
                    Some(soil_evaporation_partition_potential),
                )?;

                let soil_evaporation_demand =
                    if let Some((mut s1, mut s2, tu, mut tv)) = stage_state {
                        let infiltration = if let Some(value) = same_pass_infiltration {
                            value
                        } else {
                            Self::optional_state_scalar(
                                request,
                                phase_class,
                                WB12_SYMBOL_INFILTRATION,
                            )?
                            .unwrap_or(0.0)
                        };
                        Self::require_state_range(
                            phase_class,
                            WB12_SYMBOL_INFILTRATION,
                            infiltration,
                            Some(0.0),
                            None,
                        )?;

                        let mut es_stage = soil_evaporation_potential;
                        if s1 < tu {
                            s2 = 0.0;
                            let sp = s1 - infiltration;
                            s1 = if sp > 0.0 { sp } else { 0.0 };
                            s1 += soil_evaporation_potential;
                            let su = s1 - tu;
                            if su > 0.0 {
                                es_stage =
                                    soil_evaporation_potential - WB17_STAGE_ONE_DEFICIT_SCALE * su;
                                s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
                                tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                            }
                        } else {
                            let sb = infiltration - s2;
                            if sb < 0.0 {
                                tv += 1.0;
                                es_stage = WB17_STAGE_TWO_DENOMINATOR * tv.sqrt() - s2;
                                if infiltration > 0.0 {
                                    let mut esx = 0.8 * infiltration;
                                    if es_stage > esx {
                                        esx = es_stage + infiltration;
                                    }
                                    if esx > soil_evaporation_potential {
                                        esx = soil_evaporation_potential;
                                    }
                                    es_stage = esx;
                                } else if es_stage > soil_evaporation_potential {
                                    es_stage = soil_evaporation_potential;
                                }
                                s2 += es_stage - infiltration;
                                tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                            } else {
                                s1 = tu - sb;
                                tv = 0.0;
                                s2 = 0.0;
                                if s1 < 0.0 {
                                    s1 = 0.0;
                                }
                                s1 += soil_evaporation_potential;
                                let su = s1 - tu;
                                if su > 0.0 {
                                    es_stage = soil_evaporation_potential
                                        - WB17_STAGE_ONE_DEFICIT_SCALE * su;
                                    s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
                                    tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                                }
                            }
                        }

                        Self::require_state_range_for_symbol(
                            phase_class,
                            &stage_s1_symbol,
                            s1,
                            Some(0.0),
                            None,
                        )?;
                        Self::require_state_range_for_symbol(
                            phase_class,
                            &stage_s2_symbol,
                            s2,
                            Some(0.0),
                            None,
                        )?;
                        Self::require_state_range_for_symbol(
                            phase_class,
                            &stage_threshold_symbol,
                            tu,
                            Some(WB11_ZERO_THRESHOLD),
                            None,
                        )?;
                        Self::require_state_range_for_symbol(
                            phase_class,
                            &stage_counter_symbol,
                            tv,
                            Some(0.0),
                            None,
                        )?;
                        Self::require_flux_range(
                            phase_class,
                            WB17_SYMBOL_ES,
                            es_stage,
                            Some(0.0),
                            Some(soil_evaporation_potential),
                        )?;

                        stage_state_updates.extend([
                            WritebackField::bounded(
                                stage_s1_symbol.clone(),
                                s1,
                                Some(0.0),
                                None,
                            ),
                            WritebackField::bounded(
                                stage_s2_symbol.clone(),
                                s2,
                                Some(0.0),
                                None,
                            ),
                            WritebackField::bounded(
                                stage_threshold_symbol.clone(),
                                tu,
                                Some(WB11_ZERO_THRESHOLD),
                                None,
                            ),
                            WritebackField::bounded(
                                stage_counter_symbol.clone(),
                                tv,
                                Some(0.0),
                                None,
                            ),
                        ]);
                        es_stage
                    } else {
                        soil_evaporation_potential
                    };

                let mut soil_evaporation_with_residue =
                    soil_evaporation_demand + residue_interception;
                let potential_et_before_layer =
                    soil_evaporation_with_residue + transpiration_partition_potential;
                if et_demand < potential_et_before_layer {
                    soil_evaporation_with_residue =
                        (et_demand - transpiration_partition_potential).max(0.0);
                }
                (
                    soil_evaporation_with_residue,
                    transpiration_partition_potential,
                )
            };
        let mut residue_evaporation = residue_interception;
        let soil_evaporation_extraction_demand =
            if soil_evaporation_with_residue < residue_interception {
                residue_evaporation = if pmet_component_mode {
                    soil_evaporation_with_residue
                } else {
                    soil_evaporation_with_residue.max(0.0)
                };
                if let Some(top_layer_storage) = layer_storage.first_mut() {
                    *top_layer_storage += residue_interception - residue_evaporation;
                }
                0.0
            } else {
                soil_evaporation_with_residue - residue_interception
            };

        let mut remaining_soil_evaporation = soil_evaporation_extraction_demand;
        let mut cumulative_depth = 0.0;
        for (index, storage) in layer_storage.iter_mut().enumerate() {
            if remaining_soil_evaporation <= WB11_ZERO_THRESHOLD {
                break;
            }
            let previous_depth = cumulative_depth;
            cumulative_depth += layer_depth[index];
            if previous_depth >= WB17_SOIL_EVAPORATION_DEPTH_M {
                break;
            }

            let withdrawable = if cumulative_depth > WB17_SOIL_EVAPORATION_DEPTH_M {
                let layer_interval = cumulative_depth - previous_depth;
                if layer_interval <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: Self::wb19_dg_symbol(index + 1),
                        value: layer_interval,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }
                let evaporation_interval =
                    (WB17_SOIL_EVAPORATION_DEPTH_M - previous_depth).max(0.0);
                *storage * evaporation_interval / layer_interval
            } else {
                *storage
            };

            if withdrawable > 0.0 {
                let withdrawn = remaining_soil_evaporation.min(withdrawable);
                *storage -= withdrawn;
                remaining_soil_evaporation -= withdrawn;
                if *storage < 1.0e-10 {
                    *storage = 0.0;
                }
            }

            if cumulative_depth > WB17_SOIL_EVAPORATION_DEPTH_M {
                break;
            }
        }
        let soil_evaporation_actual =
            soil_evaporation_extraction_demand - remaining_soil_evaporation;

        Self::apply_post_et_upper_limit_redistribution(
            request,
            phase_class,
            &mut layer_storage,
            &layer_upper_limit,
            post_et_outside_water_depth > 1.0e-6,
        )?;

        let soil_water_after =
            Self::wb18_aggregate_soil_water_after_percolation(request, phase_class, &layer_storage)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;

        let actual_et = residue_evaporation + soil_evaporation_actual;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_ET,
            actual_et,
            Some(0.0),
            None,
        )?;

        let etp = transpiration_partition_potential;
        let upi = etp;
        let ui = 0.0;
        let etp_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_ETP);
        let uptake_potential_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UPI);
        let uptake_actual_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UI);
        Self::require_flux_range_for_symbol(
            phase_class,
            &etp_symbol,
            etp,
            Some(0.0),
            Some(et_demand),
        )?;
        Self::require_flux_range_for_symbol(
            phase_class,
            &uptake_potential_symbol,
            upi,
            Some(0.0),
            Some(et_demand),
        )?;
        Self::require_flux_range_for_symbol(
            phase_class,
            &uptake_actual_symbol,
            ui,
            Some(0.0),
            None,
        )?;

        let ws = 1.0;
        Self::require_flux_range(phase_class, WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0))?;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ES,
            soil_evaporation_actual,
            Some(0.0),
            Some(soil_evaporation_extraction_demand),
        )?;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ER,
            residue_evaporation,
            Some(0.0),
            Some(residue_interception),
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-ET-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = vec![WritebackField::bounded(
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )];
        state_updates.push(WritebackField::bounded(
            WB17_SYMBOL_RESIDUE_INTERCEPTION,
            0.0,
            Some(0.0),
            None,
        ));
        if let Some(infiltration) = same_pass_infiltration {
            state_updates.push(WritebackField::bounded(
                WB12_SYMBOL_INFILTRATION,
                infiltration,
                Some(0.0),
                None,
            ));
        }
        for (index, value) in layer_storage.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        state_updates.extend(stage_state_updates);

        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![
                WritebackField::bounded(
                    WB11_SYMBOL_ET,
                    actual_et,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0)),
                WritebackField::bounded(WB17_SYMBOL_EP, 0.0, Some(0.0), None),
                WritebackField::bounded(
                    WB17_SYMBOL_ES,
                    soil_evaporation_actual,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(
                    WB17_SYMBOL_ER,
                    residue_evaporation,
                    Some(0.0),
                    None,
                ),
                WritebackField::bounded(etp_symbol, etp, Some(0.0), None),
                WritebackField::bounded(uptake_potential_symbol, upi, Some(0.0), None),
                WritebackField::bounded(uptake_actual_symbol, ui, Some(0.0), None),
            ],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }


}
