#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
pub(crate) fn run_runoff_reconciliation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let rainfall_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RAINFALL_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            rainfall_input,
            Some(0.0),
            None,
        )?;
        let closure_tolerance =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_CLOSURE_TOLERANCE)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RUNOFF_CLOSURE_TOLERANCE,
            closure_tolerance,
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
        let effective_moisture_deficit = if moisture_deficit < 0.0 {
            0.0
        } else {
            moisture_deficit
        };
        let matric_potential = soil_layer_depth * effective_moisture_deficit;
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
            let interval_duration = times[index + 1] - times[index];
            let rainfall_rate = intensities[index];
            let interval_rainfall = rainfall_rate * interval_duration;
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

        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_DEPTH_M,
            irrigation_depth_m,
            Some(0.0),
            None,
        )?;

        let coupled_rainfall_input = hyetograph_rainfall + irrigation_depth_m;
        if !coupled_rainfall_input.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: coupled_rainfall_input,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        if (rainfall_input - coupled_rainfall_input).abs() > closure_tolerance + WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: rainfall_input - coupled_rainfall_input,
                minimum: Some(-closure_tolerance),
                maximum: Some(closure_tolerance),
            });
        }

        let runtime_swe = Self::validate_runtime_snow_state_domains(request, phase_class)?;
        let active_snow_coupling = if hyetograph_rainfall <= WB11_ZERO_THRESHOLD
            && irrigation_depth_m <= WB11_ZERO_THRESHOLD
            && runtime_swe <= WB11_ZERO_THRESHOLD
        {
            false
        } else {
            Self::resolve_active_snow_coupling(request, phase_class)?
        };
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling(request, phase_class, hyetograph_rainfall)?
        } else {
            SnowCouplingOutcome {
                signed_s: 0.0,
                accumulation: 0.0,
                rain_retained: 0.0,
                rain_released: 0.0,
                runtime_swe: 0.0,
                runtime_depth_m: 0.0,
                runtime_density_kg_m3: 0.0,
                runtime_settle_day_count: 0.0,
                hourly_state: Vec::new(),
            }
        };
        let (runoff_snow_term, hyetograph_liquid_input) =
            Self::resolve_snow_partition_terms(phase_class, hyetograph_rainfall, &snow_coupling)?;

        let interception =
            Self::compute_canopy_interception_depth(request, phase_class, hyetograph_liquid_input)?;
        let (hyetograph_liquid_after_interception, rainfall_scale) =
            Self::resolve_interception_rainfall_scale(
                phase_class,
                hyetograph_rainfall,
                hyetograph_liquid_input,
                interception,
            )?;
        let cumulative_infiltration =
            if let Some(producer_published_infiltration) =
                Self::resolve_wb14_producer_published_infiltration(request, phase_class)?
            {
                producer_published_infiltration
            } else {
                let computed_infiltration = Self::compute_coupled_infiltration_depth(
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
                Self::apply_wb14_storage_limit_to_infiltration(
                    request,
                    phase_class,
                    computed_infiltration,
                )?
            };
        let liquid_after_interception = hyetograph_liquid_after_interception + irrigation_depth_m;
        if !liquid_after_interception.is_finite()
            || liquid_after_interception < -WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: liquid_after_interception,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Self::require_infiltration_liquid_closure(
            phase_class,
            cumulative_infiltration,
            liquid_after_interception + runoff_snow_term,
        )?;

        let runon_input = Self::resolve_runoff_carryover_input(request, phase_class)?;
        let mofe_hourly_carry_arrays_enabled =
            Self::resolve_mofe_hourly_carry_arrays_enabled(request, phase_class)?;
        let mofe_hourly_saturation_carry = if mofe_hourly_carry_arrays_enabled {
            Some(Self::resolve_mofe_hourly_current_saturation_carry(
                request,
                phase_class,
                frost_coupling.as_ref(),
            )?)
        } else {
            None
        };
        let surface_saturation_runoff = mofe_hourly_saturation_carry
            .as_ref()
            .map_or(0.0, |carry| carry.iter().copied().sum::<f64>());
        Self::require_flux_range(
            phase_class,
            WB12_SYMBOL_RUNOFF_Q,
            surface_saturation_runoff,
            Some(0.0),
            None,
        )?;
        let mofe_hourly_lateral_carry = if mofe_hourly_carry_arrays_enabled {
            Some(Self::require_mofe_hourly_state_array(
                request,
                phase_class,
                MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
            )?)
        } else {
            None
        };

        let depression_storage_delta =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_DEPRESSION_STORAGE_DELTA)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_DEPRESSION_STORAGE_DELTA,
            depression_storage_delta,
            Some(0.0),
            None,
        )?;

        let forward_solver_lane =
            Self::resolve_wb20_forward_solver_lane_enabled(request, phase_class)?;

        let partition_runoff = Self::compute_runoff_after_interception(
            phase_class,
            liquid_after_interception,
            runoff_snow_term,
            runon_input,
            cumulative_infiltration,
            depression_storage_delta,
        )?;
        let q_runoff = partition_runoff + surface_saturation_runoff;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;

        let closure_delta = if forward_solver_lane {
            let solver_closure = liquid_after_interception
                + runon_input
                + runoff_snow_term
                + surface_saturation_runoff
                - cumulative_infiltration
                - depression_storage_delta;
            solver_closure - q_runoff
        } else {
            let runoff_observed =
                Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_OBSERVED)?;
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_RUNOFF_OBSERVED,
                runoff_observed,
                Some(0.0),
                None,
            )?;
            q_runoff - runoff_observed
        };
        if closure_delta.abs() > closure_tolerance + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CLOSURE_DELTA),
                value: closure_delta,
                minimum: Some(-closure_tolerance),
                maximum: Some(closure_tolerance),
            });
        }

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB14-RUNOFF-OK-001",
        ) else {
            unreachable!("status message ids are non-empty WB14 constants")
        };

        let mut state_updates = vec![
            WritebackField::bounded(
                WB12_SYMBOL_INFILTRATION,
                cumulative_infiltration,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(WB12_SYMBOL_RUNOFF_RECONCILED, q_runoff, Some(0.0), None),
            WritebackField::bounded(
                BoundarySymbol::from("wb14_soil_conductivity_m_s"),
                soil_conductivity,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from("wb14_effective_conductivity_m_s"),
                infiltration_conductivity,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from("wb14_matric_potential_m"),
                matric_potential,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_SOURCE,
                active_irrigation_event.map_or(0.0, |event| event.source.as_scalar()),
                Some(0.0),
                Some(2.0),
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_DEPTH_M,
                irrigation_depth_m,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_DURATION_S,
                irrigation_duration_s,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_RATE_MPS,
                irrigation_rate_m_per_s,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_EVENT_INDEX,
                active_irrigation_event.map_or(0.0, |event| {
                    Self::diagnostic_count_to_f64(event.event_index)
                }),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_SYSTEM_TYPE,
                active_irrigation_event.map_or(0.0, |event| event.system_type),
                Some(0.0),
                Some(2.0),
            ),
        ];
        if active_snow_coupling {
            state_updates.push(Self::typed_water_depth_writeback_field(
                phase_class,
                WB14_SYMBOL_SNOW_RUNTIME_SWE,
                snow_coupling.runtime_swe,
                Some(0.0),
                None,
            )?);
            state_updates.push(Self::typed_water_depth_writeback_field(
                phase_class,
                SNOW_RUNTIME_DEPTH_M_SYMBOL,
                snow_coupling.runtime_depth_m,
                Some(0.0),
                None,
            )?);
            state_updates.push(Self::typed_density_writeback_field(
                phase_class,
                SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL,
                snow_coupling.runtime_density_kg_m3,
                Some(0.0),
                Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
            )?);
            state_updates.push(WritebackField::bounded(
                SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL,
                snow_coupling.runtime_settle_day_count,
                Some(0.0),
                None,
            ));
            for hourly in &snow_coupling.hourly_state {
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_BEFORE_ROOT, hourly.hour),
                    hourly.depth_before_m,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_AVAILABLE_ROOT, hourly.hour),
                    hourly.depth_available_m,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_density_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DENSITY_BEFORE_ROOT, hourly.hour),
                    hourly.density_before_kg_m3,
                    Some(0.0),
                    Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_AFTER_ROOT, hourly.hour),
                    hourly.depth_after_m,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_density_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DENSITY_AFTER_ROOT, hourly.hour),
                    hourly.density_after_kg_m3,
                    Some(0.0),
                    Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_RAIN_RETAINED_ROOT, hourly.hour),
                    hourly.rain_retained_m,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_RAIN_RELEASED_ROOT, hourly.hour),
                    hourly.rain_released_m,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_RAW_ROOT, hourly.hour),
                    hourly.melt_raw_m,
                    None,
                    None,
                ));
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_MELT_ROOT, hourly.hour),
                    hourly.melt_m,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_AMELT_ROOT, hourly.hour),
                    hourly.melt_amelt_in,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_BMELT_ROOT, hourly.hour),
                    hourly.melt_bmelt_in,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_CMELT_ROOT, hourly.hour),
                    hourly.melt_cmelt_in,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_DMELT_ROOT, hourly.hour),
                    hourly.melt_dmelt_in,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_HRTEF_ROOT, hourly.hour),
                    hourly.melt_hrtef_f,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_HRDTF_ROOT, hourly.hour),
                    hourly.melt_hrdtf_f,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_VWMPH_ROOT, hourly.hour),
                    hourly.melt_vwmph,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_RAININ_ROOT, hourly.hour),
                    hourly.melt_rainin,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_WIND_ADJUSTMENT_ROOT, hourly.hour),
                    hourly.melt_wind_adjustment,
                    None,
                    None,
                ));
                state_updates.push(Self::typed_fraction_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_MELT_BRANCH_ACTIVE_ROOT, hourly.hour),
                    hourly.melt_branch_active,
                    Some(0.0),
                    Some(1.0),
                )?);
                state_updates.push(Self::typed_temperature_writeback_field(
                    phase_class,
                    Self::hourly_symbol(WINTER_HOURLY_DEWPOINT_ROOT, hourly.hour),
                    hourly.dewpoint_c,
                    None,
                    None,
                )?);
                state_updates.push(Self::typed_linear_rate_writeback_field(
                    phase_class,
                    Self::hourly_symbol(WINTER_HOURLY_WIND_ROOT, hourly.hour),
                    hourly.wind_m_s,
                    Some(0.0),
                    None,
                )?);
            }
        } else {
            state_updates.push(Self::typed_water_depth_writeback_field(
                phase_class,
                WB14_SYMBOL_SNOW_RUNTIME_SWE,
                0.0,
                Some(0.0),
                None,
            )?);
            state_updates.push(Self::typed_water_depth_writeback_field(
                phase_class,
                SNOW_RUNTIME_DEPTH_M_SYMBOL,
                0.0,
                Some(0.0),
                None,
            )?);
            state_updates.push(Self::typed_density_writeback_field(
                phase_class,
                SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL,
                0.0,
                Some(0.0),
                Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
            )?);
            state_updates.push(WritebackField::bounded(
                SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL,
                0.0,
                Some(0.0),
                None,
            ));
            for hour in 1..=SIMIMPL29_HOURS_PER_DAY {
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_BEFORE_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_AVAILABLE_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_density_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DENSITY_BEFORE_ROOT, hour),
                    0.0,
                    Some(0.0),
                    Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_AFTER_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_density_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_DENSITY_AFTER_ROOT, hour),
                    0.0,
                    Some(0.0),
                    Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_RAIN_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_SNOWFALL_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_MELT_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_RAW_ROOT, hour),
                    0.0,
                    None,
                    None,
                ));
                state_updates.push(Self::typed_water_depth_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_RAIN_RETAINED_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                )?);
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_AMELT_ROOT, hour),
                    0.0,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_BMELT_ROOT, hour),
                    0.0,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_CMELT_ROOT, hour),
                    0.0,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_DMELT_ROOT, hour),
                    0.0,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_HRTEF_ROOT, hour),
                    0.0,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_HRDTF_ROOT, hour),
                    0.0,
                    None,
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_VWMPH_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_RAININ_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_WIND_ADJUSTMENT_ROOT, hour),
                    0.0,
                    None,
                    None,
                ));
                state_updates.push(Self::typed_fraction_writeback_field(
                    phase_class,
                    Self::hourly_symbol(SNOW_HOURLY_MELT_BRANCH_ACTIVE_ROOT, hour),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                )?);
                state_updates.push(Self::typed_temperature_writeback_field(
                    phase_class,
                    Self::hourly_symbol(WINTER_HOURLY_DEWPOINT_ROOT, hour),
                    0.0,
                    None,
                    None,
                )?);
                state_updates.push(Self::typed_linear_rate_writeback_field(
                    phase_class,
                    Self::hourly_symbol(WINTER_HOURLY_WIND_ROOT, hour),
                    0.0,
                    Some(0.0),
                    None,
                )?);
            }
        }
        if let Some(frost_outcome) = frost_coupling {
            if let Some(soil_water_after_frwatc) = frost_outcome.soil_water_after_frwatc {
                state_updates.push(WritebackField::bounded(
                    WB11_SYMBOL_SOIL_WATER,
                    soil_water_after_frwatc,
                    Some(0.0),
                    None,
                ));
            }
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_DFROST,
                frost_outcome.dfrost,
                Some(0.0),
                Some(frost_outcome.profile_depth_m),
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_DTHAW,
                frost_outcome.dthaw,
                Some(0.0),
                Some(frost_outcome.profile_depth_m),
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_NFT,
                frost_outcome.nft,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
                frost_outcome.ws_frz,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ,
                frost_outcome.infcap_frz,
                Some(0.0),
                Some(soil_conductivity),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_FRWATC_SOIL_WATER_BEFORE_SYMBOL),
                frost_outcome.frwatc_soil_water_before,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_FRWATC_SOIL_WATER_AFTER_SYMBOL),
                frost_outcome.frwatc_soil_water_after,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_FRWATC_FROZEN_WATER_BEFORE_SYMBOL),
                frost_outcome.frwatc_frozen_water_before,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_FRWATC_FROZEN_WATER_AFTER_SYMBOL),
                frost_outcome.frwatc_frozen_water_after,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_FRWATC_FREEZE_DEBIT_SYMBOL),
                frost_outcome.frwatc_freeze_debit,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_FRWATC_THAW_CREDIT_SYMBOL),
                frost_outcome.frwatc_thaw_credit,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::unbounded(
                BoundarySymbol::from(FROST_RUNTIME_FRWATC_NET_LIQUID_DELTA_SYMBOL),
                frost_outcome.frwatc_net_liquid_delta,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_FRDP_M_SYMBOL),
                frost_outcome.frdp_m,
                Some(0.0),
                Some(frost_outcome.profile_depth_m),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_THDP_M_SYMBOL),
                frost_outcome.thdp_m,
                Some(0.0),
                Some(frost_outcome.profile_depth_m),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_TFRDP_M_SYMBOL),
                frost_outcome.tfrdp_m,
                Some(0.0),
                Some(frost_outcome.profile_depth_m),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_TTHAWD_M_SYMBOL),
                frost_outcome.tthawd_m,
                Some(0.0),
                Some(frost_outcome.profile_depth_m),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_FGTHWD_FLAG_SYMBOL),
                frost_outcome.fgthwd_flag,
                Some(0.0),
                Some(1.0),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_TOTAL_FINE_LAYER_COUNT_SYMBOL),
                frost_outcome.total_fine_layer_count,
                Some(1.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_CONDUCTIVITY_TILLED_SYMBOL),
                frost_outcome.conductivity_tilled_w_m_k,
                Some(WB11_ZERO_THRESHOLD),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_CONDUCTIVITY_UNTILLED_SYMBOL),
                frost_outcome.conductivity_untilled_w_m_k,
                Some(WB11_ZERO_THRESHOLD),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_CONDUCTIVITY_RESIDUE_SYMBOL),
                frost_outcome.conductivity_residue_w_m_k,
                Some(WB11_ZERO_THRESHOLD),
                None,
            ));
            for layer in &frost_outcome.layer_topology_state {
                state_updates.push(WritebackField::bounded(
                    Self::frost_layer_symbol(FROST_RUNTIME_LAYER_FINE_COUNT_ROOT, layer.layer_index),
                    Self::diagnostic_count_to_f64(layer.fine_layer_count),
                    Some(1.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::frost_layer_symbol(
                        FROST_RUNTIME_LAYER_FINE_THICKNESS_ROOT,
                        layer.layer_index,
                    ),
                    layer.fine_layer_thickness_m,
                    Some(WB11_ZERO_THRESHOLD),
                    None,
                ));
            }
            for hourly in &frost_outcome.hourly_state {
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(FROST_HOURLY_QSRF_ROOT, hourly.hour),
                    hourly.qsrf_w_m2,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(FROST_HOURLY_QUF_ROOT, hourly.hour),
                    hourly.quf_w_m2,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(FROST_HOURLY_KSRF_ROOT, hourly.hour),
                    hourly.ksrf_w_m_k,
                    Some(WB11_ZERO_THRESHOLD),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(FROST_HOURLY_SNOW_DEPTH_ROOT, hourly.hour),
                    hourly.snow_depth_m,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(FROST_HOURLY_RESIDUE_DEPTH_ROOT, hourly.hour),
                    hourly.residue_depth_m,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(FROST_HOURLY_TILLED_FROZEN_DEPTH_ROOT, hourly.hour),
                    hourly.tilled_frozen_depth_m,
                    Some(0.0),
                    Some(FROST_RUNTIME_TILLAGE_DEPTH_M),
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(FROST_HOURLY_UNTILLED_FROZEN_DEPTH_ROOT, hourly.hour),
                    hourly.untilled_frozen_depth_m,
                    Some(0.0),
                    None,
                ));
            }
        }
        if let (Some(saturation_carry), Some(lateral_carry)) =
            (mofe_hourly_saturation_carry, mofe_hourly_lateral_carry)
        {
            for hour in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
                let saturation_value = saturation_carry[hour - 1];
                let lateral_value = lateral_carry[hour - 1];
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT, hour),
                    saturation_value,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT, hour),
                    saturation_value,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT, hour),
                    lateral_value,
                    Some(0.0),
                    None,
                ));
            }
        }

        let mut flux_updates = vec![
            WritebackField::bounded(
                WB15_SYMBOL_INTERCEPTION_I,
                interception,
                Some(0.0),
                Some(hyetograph_rainfall),
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_DAILY_IRRIGATION,
                irrigation_depth_m,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None),
            WritebackField::bounded(
                BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CARRYOVER),
                runon_input,
                Some(0.0),
                None,
            ),
            WritebackField::unbounded(WB12_SYMBOL_RUNOFF_CLOSURE_DELTA, closure_delta),
            WritebackField::unbounded(WB12_SYMBOL_SNOW_COUPLING_S, snow_coupling.signed_s),
        ];
        flux_updates.extend(Self::publish_same_day_snow_publication_fluxes(
            phase_class,
            runoff_snow_term,
            hyetograph_liquid_input,
        )?);

        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        Ok(KernelRunResponse::new(status, writeback))
    }
pub(crate) fn publish_same_day_snow_publication_fluxes(
        phase_class: HillslopeKernelPhaseClass,
        routed_melt_m: f64,
        post_winter_rain_m: f64,
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        // HPHYS0291 same-day snow publication lifecycle: WB13 consumes producer-owned
        // fluxes, not state defaults or downstream reconstructions.
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow.routed_melt_m"),
            routed_melt_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow.post_winter_rain_m"),
            post_winter_rain_m,
            Some(0.0),
            None,
        )?;
        Ok(vec![
            WritebackField::bounded(
                BoundarySymbol::from("snow.routed_melt_m"),
                routed_melt_m,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from("snow.post_winter_rain_m"),
                post_winter_rain_m,
                Some(0.0),
                None,
            ),
        ])
    }


}
