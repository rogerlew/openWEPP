#[allow(clippy::wildcard_imports)]
use super::super::*;

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_arguments)]
    fn update_hourly_opt_in_snow_albedo_state(
        phase_class: HillslopeKernelPhaseClass,
        melt_model: SnowMeltModel,
        albedo_model: Option<SnowAlbedoModel>,
        previous_state: Option<SnowAlbedoState>,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
        fresh_snow_depth_m: f64,
        fresh_snow_density_kg_m3: f64,
        positive_temperature_c_day_increment: f64,
        underlying_surface_albedo: f64,
    ) -> Result<Option<SnowAlbedoState>, Wb11HydrologyKernelGuardError> {
        if melt_model == SnowMeltModel::LegacyCoe {
            return Ok(previous_state);
        }

        let snow_water_equivalent_m = if snow_depth_m > WB11_ZERO_THRESHOLD
            && snow_density_kg_m3 > WB11_ZERO_THRESHOLD
        {
            openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                snow_depth_m,
                snow_density_kg_m3,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                    &error,
                )
            })?
        } else {
            0.0
        };
        let fresh_snow_water_equivalent_m = if fresh_snow_depth_m > WB11_ZERO_THRESHOLD
            && fresh_snow_density_kg_m3 > WB11_ZERO_THRESHOLD
        {
            openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                fresh_snow_depth_m,
                fresh_snow_density_kg_m3,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from(SNOW_HOURLY_SNOWFALL_ROOT),
                    &error,
                )
            })?
        } else {
            0.0
        };

        if snow_water_equivalent_m <= WB11_ZERO_THRESHOLD
            && fresh_snow_water_equivalent_m <= WB11_ZERO_THRESHOLD
        {
            return Ok(None);
        }

        update_snow_albedo_state(SnowAlbedoUpdateInputs {
            melt_model,
            albedo_model,
            previous_state,
            snow_water_equivalent_m,
            fresh_snow_water_equivalent_m,
            positive_temperature_c_day_increment,
            underlying_surface_albedo,
        })
        .map(|outcome| outcome.state)
        .map_err(|error| Self::snow_albedo_guard_error(phase_class, &error))
    }

    fn snow_albedo_guard_error(
        phase_class: HillslopeKernelPhaseClass,
        error: &SnowAlbedoError,
    ) -> Wb11HydrologyKernelGuardError {
        match error {
            SnowAlbedoError::MissingRequiredAlbedoModel => {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("snow_albedo_model_id"),
                }
            }
            SnowAlbedoError::MissingRequiredAlbedoState => {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("snow_albedo"),
                }
            }
            SnowAlbedoError::AlbedoModelMismatch { .. } => {
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from("snow_albedo_model_id"),
                    value: f64::NAN,
                    minimum: None,
                    maximum: None,
                }
            }
            SnowAlbedoError::NonFiniteInput { symbol, value } => {
                Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(*symbol),
                    value: *value,
                }
            }
            SnowAlbedoError::OutOfRangeInput {
                symbol,
                value,
                minimum,
                maximum,
            } => Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(*symbol),
                value: *value,
                minimum: Some(*minimum),
                maximum: Some(*maximum),
            },
        }
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn compute_simimpl29_melt_hour(
        phase_class: HillslopeKernelPhaseClass,
        cancov: f64,
        hrad_mj_m2: f64,
        cloud_fraction: f64,
        hrtemp_c: f64,
        tdpt_c: f64,
        vwind_m_s: f64,
        hrrain_m: f64,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
        shortwave_absorbed_fraction: f64,
    ) -> Result<SnowMeltComputation, Wb11HydrologyKernelGuardError> {
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("cancov"),
            cancov,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
            hrrain_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(WINTER_HOURLY_RAD_ROOT),
            hrad_mj_m2,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(WINTER_HOURLY_CLOUD_ROOT),
            cloud_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("vwind"),
            vwind_m_s,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            snow_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            snow_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow_melt_shortwave_absorbed_fraction"),
            shortwave_absorbed_fraction,
            Some(0.0),
            Some(1.0),
        )?;

        if snow_depth_m <= WB11_ZERO_THRESHOLD || snow_density_kg_m3 <= WB11_ZERO_THRESHOLD {
            return Ok(SnowMeltComputation {
                wmelt_m: 0.0,
                terms: SnowMeltTerms::default(),
            });
        }

        let hrtef =
            openwepp_unit_boundary::conversions::celsius_delta_to_fahrenheit_delta(hrtemp_c)
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from("winter.hourly.air_temp_c"),
                        &error,
                    )
                })?;
        let hrdtf =
            openwepp_unit_boundary::conversions::celsius_delta_to_fahrenheit_delta(tdpt_c)
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from("tdpt"),
                        &error,
                    )
                })?;

        let amelt = 0.0607
            * hrad_mj_m2
            * shortwave_absorbed_fraction
            * (1.0 - cancov * SIMIMPL29_CANOPY_FACTOR);
        let bmelt = 0.025 / 24.0 * hrtef
            - (0.84 * (1.0 - cloud_fraction)) * (1.0 - cancov * SIMIMPL29_CANOPY_FACTOR) / 24.0;

        let adj = 1.57 * SIMIMPL29_WIND_MEASUREMENT_HEIGHT_M.powf(-1.0 / 6.0);
        let vwmph =
            openwepp_unit_boundary::conversions::meters_per_second_to_legacy_miles_per_hour(
                vwind_m_s,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from("vwind"),
                    &error,
                )
            })?;
        let cmelt = if vwmph > 0.0 {
            (0.0084 / 24.0)
                * vwmph
                * (1.0 - 0.8 * cancov * SIMIMPL29_CANOPY_FACTOR)
                * ((0.22 * hrtef) + (0.78 * hrdtf))
                * adj
                + 0.8 * cancov * SIMIMPL29_CANOPY_FACTOR * 0.045 / 24.0 * hrtef
        } else {
            0.045 / 24.0 * hrtef
        };

        let rainin = openwepp_unit_boundary::conversions::meters_to_legacy_inches(hrrain_m)
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
                    &error,
                )
            })?;
        let dmelt = if hrdtf > 0.0 {
            0.007 * rainin * hrdtf
        } else {
            0.007 * rainin * hrtef
        };
        let terms = SnowMeltTerms {
            amelt_in: amelt,
            bmelt_in: bmelt,
            cmelt_in: cmelt,
            dmelt_in: dmelt,
            hrtef_f: hrtef,
            hrdtf_f: hrdtf,
            vwmph,
            rainin,
            wind_adjustment: adj,
        };

        let mut wmelt_m =
            openwepp_unit_boundary::conversions::legacy_inches_to_meters(
                amelt + bmelt + cmelt + dmelt,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                    &error,
                )
            })?;
        if !wmelt_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                value: wmelt_m,
                minimum: Some(0.0),
                maximum: Some(snow_depth_m),
            });
        }
        if wmelt_m >= 0.0 {
            let melt_depth_at_snow_density =
                openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                    wmelt_m,
                    snow_density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                        &error,
                    )
                })?;
            if melt_depth_at_snow_density >= snow_depth_m {
                wmelt_m =
                    openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                        snow_depth_m,
                        snow_density_kg_m3,
                    )
                    .map_err(|error| {
                        Self::unit_conversion_guard_error(
                            phase_class,
                            BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                            &error,
                        )
                    })?;
            }
        }

        let maximum_melt_m =
            openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                snow_depth_m,
                snow_density_kg_m3,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                    &error,
                )
            })?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
            wmelt_m,
            None,
            Some(maximum_melt_m),
        )?;
        Ok(SnowMeltComputation { wmelt_m, terms })
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn compute_active_snow_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
    ) -> Result<SnowCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_rainfall,
            Some(0.0),
            None,
        )?;

        let _rst = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_RST)?;
        let newsnw = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_NEWSNW)?;
        let ssd = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_SSD)?;
        let runtime_swe =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_RUNTIME_SWE)?;
        let tmax = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let tmin = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;
        let cancov = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_CANCOV)?;
        let vwind = Self::require_state_scalar_for_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("vwind"),
        )?;
        let tdpt = Self::require_state_scalar_for_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("tdpt"),
        )?;

        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_NEWSNW,
            newsnw,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(phase_class, WB14_SYMBOL_SNOW_SSD, ssd, Some(0.0), None)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_RUNTIME_SWE,
            runtime_swe,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_PLANT_CANCOV,
            cancov,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("vwind"),
            vwind,
            Some(0.0),
            None,
        )?;

        if newsnw > ssd + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
                value: newsnw,
                minimum: Some(0.0),
                maximum: Some(ssd),
            });
        }

        let depth_symbol = BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL);
        let density_symbol = BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL);
        let settle_day_count_symbol = BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL);

        let mut runtime_depth_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &depth_symbol,
        )?
        .unwrap_or(0.0);
        let mut runtime_density_kg_m3 = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &density_symbol,
        )?
        .unwrap_or(0.0);
        let mut settle_day_count = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &settle_day_count_symbol,
        )?
        .unwrap_or(0.0);

        Self::require_dynamic_state_range(
            phase_class,
            depth_symbol.clone(),
            runtime_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            density_symbol.clone(),
            runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            settle_day_count_symbol.clone(),
            settle_day_count,
            Some(0.0),
            None,
        )?;

        if runtime_depth_m <= WB11_ZERO_THRESHOLD && runtime_swe > WB11_ZERO_THRESHOLD {
            if runtime_density_kg_m3 <= WB11_ZERO_THRESHOLD {
                runtime_density_kg_m3 = newsnw;
            }
            runtime_depth_m =
                openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                    runtime_swe,
                    runtime_density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                        &error,
                    )
                })?;
        }
        if runtime_depth_m > WB11_ZERO_THRESHOLD && runtime_density_kg_m3 <= WB11_ZERO_THRESHOLD {
            runtime_density_kg_m3 = newsnw;
        }
        if runtime_depth_m <= WB11_ZERO_THRESHOLD {
            runtime_depth_m = 0.0;
            runtime_density_kg_m3 = 0.0;
        }

        let mut snodep = runtime_depth_m;
        let mut dens = runtime_density_kg_m3;
        let daily_mean_temp = f64::midpoint(tmax, tmin);

        let mut accumulation_water_m = 0.0;
        let mut total_rain_retained_m = 0.0;
        let mut total_rain_released_m = 0.0;
        let mut hourly_state = Vec::with_capacity(SIMIMPL29_HOURS_PER_DAY);

        for hour in 1..=SIMIMPL29_HOURS_PER_DAY {
            let hrrain = Self::require_hourly_state_scalar(
                request,
                phase_class,
                SNOW_HOURLY_RAIN_ROOT,
                hour,
            )?;
            let hrsnow = Self::require_hourly_state_scalar(
                request,
                phase_class,
                SNOW_HOURLY_SNOWFALL_ROOT,
                hour,
            )?;
            let hrad_mj_m2 = Self::require_hourly_state_scalar(
                request,
                phase_class,
                WINTER_HOURLY_RAD_ROOT,
                hour,
            )?;
            let hrtemp_c = Self::require_hourly_state_scalar(
                request,
                phase_class,
                WINTER_HOURLY_AIR_TEMP_ROOT,
                hour,
            )?;
            let cloud_fraction = Self::require_hourly_state_scalar(
                request,
                phase_class,
                WINTER_HOURLY_CLOUD_ROOT,
                hour,
            )?;

            Self::require_dynamic_state_range(
                phase_class,
                Self::hourly_symbol(SNOW_HOURLY_RAIN_ROOT, hour),
                hrrain,
                Some(0.0),
                None,
            )?;
            Self::require_dynamic_state_range(
                phase_class,
                Self::hourly_symbol(SNOW_HOURLY_SNOWFALL_ROOT, hour),
                hrsnow,
                Some(0.0),
                None,
            )?;

            if hour == 1 {
                settle_day_count += 1.0;
            }
            if hrsnow > WB11_ZERO_THRESHOLD {
                settle_day_count = 1.0;
            }

            let depth_before_m = snodep.max(0.0);
            let density_before_kg_m3 = dens.max(0.0);
            let depth_available_m = depth_before_m;
            let mut rain_retained_m = 0.0;
            let mut rain_released_m = 0.0;
            let mut melt_raw_m = 0.0;
            let mut melt_m = 0.0;
            let mut melt_terms = SnowMeltTerms::default();
            let mut melt_branch_active = 0.0;

            if snodep <= WB11_ZERO_THRESHOLD {
                if hrsnow <= WB11_ZERO_THRESHOLD {
                    snodep = 0.0;
                    dens = 0.0;
                } else {
                    snodep = hrsnow;
                    dens = newsnw;
                }
            } else if daily_mean_temp < 0.0 {
                let mut snodpt = snodep;
                let mut densgt;

                let mut setf = ((-(settle_day_count * 2.0)).exp() * SIMIMPL29_SNOWPACK_SETTLE_BASE)
                    + 1.0;
                if dens > ssd {
                    setf = 1.0;
                }
                densgt = dens * setf;
                if densgt > SIMIMPL29_SNOW_DENSITY_CAP_KG_M3 {
                    densgt = SIMIMPL29_SNOW_DENSITY_CAP_KG_M3;
                }
                if densgt > WB11_ZERO_THRESHOLD {
                    snodpt = snodpt * dens / densgt;
                }

                if hrsnow <= WB11_ZERO_THRESHOLD {
                    snodep = snodpt;
                    dens = densgt;
                } else {
                    snodep = snodpt + hrsnow;
                    if snodep > WB11_ZERO_THRESHOLD {
                        dens = ((densgt * snodpt) + (newsnw * hrsnow)) / snodep;
                    } else {
                        dens = 0.0;
                    }
                }
            } else {
                let snodpt = snodep;
                if hrsnow > WB11_ZERO_THRESHOLD {
                    snodep += hrsnow;
                    if snodep > WB11_ZERO_THRESHOLD {
                        dens = ((dens * snodpt) + (newsnw * hrsnow)) / snodep;
                    } else {
                        dens = 0.0;
                    }
                }

                if snodep > WB11_ZERO_THRESHOLD {
                    let melt_computation = Self::compute_simimpl29_melt_hour(
                        phase_class,
                        cancov,
                        hrad_mj_m2,
                        cloud_fraction,
                        hrtemp_c,
                        tdpt,
                        vwind,
                        hrrain,
                        snodep,
                        dens,
                        1.0,
                    )?;
                    melt_branch_active = 1.0;
                    melt_terms = melt_computation.terms;
                    let wmelt = melt_computation.wmelt_m;
                    melt_raw_m = wmelt;
                    let smelt = if wmelt > WB11_ZERO_THRESHOLD {
                        openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                            wmelt,
                            dens,
                        )
                        .map_err(|error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                                &error,
                            )
                        })?
                    } else {
                        0.0
                    };
                    let snodpt_after_inputs = snodep;
                    snodep = snodpt_after_inputs - smelt;
                    if snodep <= WB11_ZERO_THRESHOLD {
                        if smelt > 0.0 {
                            melt_m =
                                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                    snodpt_after_inputs,
                                    dens,
                                )
                                .map_err(|error| {
                                    Self::unit_conversion_guard_error(
                                        phase_class,
                                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                                        &error,
                                    )
                                })?;
                        }
                        snodep = 0.0;
                        dens = 0.0;
                    } else if dens >= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
                        if smelt > 0.0 {
                            melt_m =
                                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                    smelt,
                                    dens,
                                )
                                .map_err(|error| {
                                    Self::unit_conversion_guard_error(
                                        phase_class,
                                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                                        &error,
                                    )
                                })?;
                        } else {
                            melt_m = wmelt.min(0.0);
                        }
                    } else {
                        let mut densgt = dens * (snodpt_after_inputs / snodep);
                        if densgt <= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
                            melt_m = wmelt.min(0.0);
                            if hrrain > WB11_ZERO_THRESHOLD {
                                let densic = openwepp_unit_boundary::conversions::water_depth_meters_to_snow_density_increment(
                                    hrrain,
                                    snodep,
                                )
                                .map_err(|error| {
                                    Self::unit_conversion_guard_error(
                                        phase_class,
                                        BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
                                        &error,
                                    )
                                })?;
                                if densic
                                    <= (SIMIMPL29_DENSITY_MELT_GATE_KG_M3 - densgt)
                                        + WB11_ZERO_THRESHOLD
                                {
                                    rain_retained_m = hrrain;
                                    densgt += densic;
                                } else {
                                    rain_retained_m =
                                        openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                            snodep,
                                            SIMIMPL29_DENSITY_MELT_GATE_KG_M3 - densgt,
                                        )
                                        .map_err(|error| {
                                            Self::unit_conversion_guard_error(
                                                phase_class,
                                                BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
                                                &error,
                                            )
                                        })?;
                                    densgt = SIMIMPL29_DENSITY_MELT_GATE_KG_M3;
                                }
                            }
                        } else {
                            melt_m =
                                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                    snodep,
                                    densgt - SIMIMPL29_DENSITY_MELT_GATE_KG_M3,
                                )
                                .map_err(|error| {
                                    Self::unit_conversion_guard_error(
                                        phase_class,
                                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                                        &error,
                                    )
                                })?;
                            densgt = SIMIMPL29_DENSITY_MELT_GATE_KG_M3;
                        }
                        dens = densgt;
                    }
                }
            }

            if dens > SIMIMPL29_SNOW_DENSITY_CAP_KG_M3 {
                dens = SIMIMPL29_SNOW_DENSITY_CAP_KG_M3;
            }
            if snodep <= WB11_ZERO_THRESHOLD {
                snodep = 0.0;
                dens = 0.0;
            }
            if depth_before_m > WB11_ZERO_THRESHOLD
                && hrrain > rain_retained_m + WB11_ZERO_THRESHOLD
            {
                rain_released_m = hrrain - rain_retained_m;
            }

            accumulation_water_m += hrsnow * 0.1;
            total_rain_retained_m += rain_retained_m;
            total_rain_released_m += rain_released_m;

            hourly_state.push(SnowHourlyState {
                hour,
                depth_before_m,
                depth_available_m,
                density_before_kg_m3,
                depth_after_m: snodep,
                density_after_kg_m3: dens,
                rain_retained_m,
                rain_released_m,
                melt_raw_m,
                melt_m,
                melt_amelt_in: melt_terms.amelt_in,
                melt_bmelt_in: melt_terms.bmelt_in,
                melt_cmelt_in: melt_terms.cmelt_in,
                melt_dmelt_in: melt_terms.dmelt_in,
                melt_hrtef_f: melt_terms.hrtef_f,
                melt_hrdtf_f: melt_terms.hrdtf_f,
                melt_vwmph: melt_terms.vwmph,
                melt_rainin: melt_terms.rainin,
                melt_wind_adjustment: melt_terms.wind_adjustment,
                melt_branch_active,
                dewpoint_c: tdpt,
                wind_m_s: vwind,
            });
        }

        let raw_melt_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_raw_m)
            .sum::<f64>();
        let melt_redistribution = Self::redistribute_daily_signed_snowmelt(&mut hourly_state);
        for hourly in &mut hourly_state {
            if hourly.rain_released_m > WB11_ZERO_THRESHOLD {
                hourly.melt_m += hourly.rain_released_m;
            }
        }
        let available_runtime_swe_for_state_loss =
            runtime_swe + accumulation_water_m + total_rain_retained_m;
        if !available_runtime_swe_for_state_loss.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: available_runtime_swe_for_state_loss,
            });
        }
        if !melt_redistribution.snowpack_state_loss_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: melt_redistribution.snowpack_state_loss_m,
            });
        }
        let bounded_state_loss_m = if melt_redistribution.snowpack_state_loss_m
            > available_runtime_swe_for_state_loss
                + SIMIMPL29_SNOWPACK_STATE_LOSS_OVERDRAW_TOLERANCE_M
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: available_runtime_swe_for_state_loss
                    - melt_redistribution.snowpack_state_loss_m,
                minimum: Some(0.0),
                maximum: None,
            });
        } else if melt_redistribution.snowpack_state_loss_m > available_runtime_swe_for_state_loss
        {
            available_runtime_swe_for_state_loss
        } else {
            melt_redistribution.snowpack_state_loss_m
        };
        let runtime_swe_after_raw = available_runtime_swe_for_state_loss - bounded_state_loss_m;
        if !runtime_swe_after_raw.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: runtime_swe_after_raw,
            });
        }
        let runtime_swe_after = if runtime_swe_after_raw <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            runtime_swe_after_raw
        };
        if runtime_swe_after <= WB11_ZERO_THRESHOLD {
            snodep = 0.0;
            dens = 0.0;
        } else if dens > WB11_ZERO_THRESHOLD {
            snodep =
                openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                    runtime_swe_after,
                    dens,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                        &error,
                    )
                })?;
        }
        let signed_s =
            melt_redistribution.routed_melt_total_m - accumulation_water_m - total_rain_retained_m;
        let routed_melt_total_m =
            melt_redistribution.routed_melt_total_m + total_rain_released_m;
        if !signed_s.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_SNOW_COUPLING_S),
                value: signed_s,
                minimum: None,
                maximum: None,
            });
        }
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow.routed_melt_m"),
            routed_melt_total_m,
            Some(0.0),
            None,
        )?;

        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            runtime_swe_after,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            snodep,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            dens,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        Ok(SnowCouplingOutcome {
            signed_s,
            accumulation: accumulation_water_m,
            rain_retained: total_rain_retained_m,
            rain_released: total_rain_released_m,
            raw_melt: raw_melt_total_m,
            redistributed_melt: melt_redistribution.routed_melt_total_m,
            snowpack_state_loss: bounded_state_loss_m,
            runtime_swe: runtime_swe_after,
            runtime_depth_m: snodep,
            runtime_density_kg_m3: dens,
            runtime_settle_day_count: settle_day_count,
            snow_albedo_state_after: None,
            hourly_state,
        })
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn compute_active_snow_coupling_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<SnowCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
            inputs.hyetograph_rainfall_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SNOW_RST),
            inputs.rst_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
            inputs.newsnw_kg_m3,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SNOW_SSD),
            inputs.ssd_kg_m3,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            inputs.runtime_swe_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_TMAX),
            inputs.tmax_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_TMIN),
            inputs.tmin_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB15_SYMBOL_PLANT_CANCOV),
            inputs.canopy_cover_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("vwind"),
            inputs.wind_m_s,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("tdpt"),
            inputs.dewpoint_c,
            None,
            None,
        )?;
        if inputs.newsnw_kg_m3 > inputs.ssd_kg_m3 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
                value: inputs.newsnw_kg_m3,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: Some(inputs.ssd_kg_m3),
            });
        }

        let mut runtime_depth_m = inputs.runtime_depth_m;
        let mut runtime_density_kg_m3 = inputs.runtime_density_kg_m3;
        let mut settle_day_count = inputs.runtime_settle_day_count;

        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            runtime_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL),
            settle_day_count,
            Some(0.0),
            None,
        )?;

        if runtime_depth_m <= WB11_ZERO_THRESHOLD && inputs.runtime_swe_m > WB11_ZERO_THRESHOLD {
            if runtime_density_kg_m3 <= WB11_ZERO_THRESHOLD {
                runtime_density_kg_m3 = inputs.newsnw_kg_m3;
            }
            runtime_depth_m =
                openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                    inputs.runtime_swe_m,
                    runtime_density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                        &error,
                    )
                })?;
        }
        if runtime_depth_m > WB11_ZERO_THRESHOLD
            && runtime_density_kg_m3 <= WB11_ZERO_THRESHOLD
        {
            runtime_density_kg_m3 = inputs.newsnw_kg_m3;
        }
        if runtime_depth_m <= WB11_ZERO_THRESHOLD {
            runtime_depth_m = 0.0;
            runtime_density_kg_m3 = 0.0;
        }

        let mut snodep = runtime_depth_m;
        let mut dens = runtime_density_kg_m3;
        let daily_mean_temp = f64::midpoint(inputs.tmax_c, inputs.tmin_c);

        let mut accumulation_water_m = 0.0;
        let mut total_rain_retained_m = 0.0;
        let mut total_rain_released_m = 0.0;
        let mut snow_albedo_state_after = inputs.snow_albedo_state;
        let mut hourly_state = Vec::with_capacity(SIMIMPL29_HOURS_PER_DAY);

        for hour in 1..=SIMIMPL29_HOURS_PER_DAY {
            let hourly = inputs.hourly[hour - 1];
            let hrrain = hourly.rain_m;
            let hrsnow = hourly.snowfall_m;
            let hrad_mj_m2 = hourly.radiation_mj_m2;
            let hrtemp_c = hourly.air_temperature_c;
            let cloud_fraction = hourly.cloud_fraction;

            Self::require_direct_typed_snow_value(
                phase_class,
                Self::hourly_symbol(SNOW_HOURLY_RAIN_ROOT, hour),
                hrrain,
                Some(0.0),
                None,
            )?;
            Self::require_direct_typed_snow_value(
                phase_class,
                Self::hourly_symbol(SNOW_HOURLY_SNOWFALL_ROOT, hour),
                hrsnow,
                Some(0.0),
                None,
            )?;
            Self::require_direct_typed_snow_value(
                phase_class,
                Self::hourly_symbol(WINTER_HOURLY_RAD_ROOT, hour),
                hrad_mj_m2,
                Some(0.0),
                None,
            )?;
            Self::require_direct_typed_snow_value(
                phase_class,
                Self::hourly_symbol(WINTER_HOURLY_AIR_TEMP_ROOT, hour),
                hrtemp_c,
                None,
                None,
            )?;
            Self::require_direct_typed_snow_value(
                phase_class,
                Self::hourly_symbol(WINTER_HOURLY_CLOUD_ROOT, hour),
                cloud_fraction,
                Some(0.0),
                Some(1.0),
            )?;

            if hour == 1 {
                settle_day_count += 1.0;
            }
            if hrsnow > WB11_ZERO_THRESHOLD {
                settle_day_count = 1.0;
            }

            let depth_before_m = snodep.max(0.0);
            let density_before_kg_m3 = dens.max(0.0);
            let depth_available_m = depth_before_m;
            let mut rain_retained_m = 0.0;
            let mut rain_released_m = 0.0;
            let mut melt_raw_m = 0.0;
            let mut melt_m = 0.0;
            let mut melt_terms = SnowMeltTerms::default();
            let mut melt_branch_active = 0.0;
            let mut albedo_updated_this_hour = false;

            if snodep <= WB11_ZERO_THRESHOLD {
                if hrsnow <= WB11_ZERO_THRESHOLD {
                    snodep = 0.0;
                    dens = 0.0;
                } else {
                    snodep = hrsnow;
                    dens = inputs.newsnw_kg_m3;
                }
            } else if daily_mean_temp < 0.0 {
                let mut snodpt = snodep;
                let mut densgt;

                let mut setf = ((-(settle_day_count * 2.0)).exp()
                    * SIMIMPL29_SNOWPACK_SETTLE_BASE)
                    + 1.0;
                if dens > inputs.ssd_kg_m3 {
                    setf = 1.0;
                }
                densgt = dens * setf;
                if densgt > SIMIMPL29_SNOW_DENSITY_CAP_KG_M3 {
                    densgt = SIMIMPL29_SNOW_DENSITY_CAP_KG_M3;
                }
                if densgt > WB11_ZERO_THRESHOLD {
                    snodpt = snodpt * dens / densgt;
                }

                if hrsnow <= WB11_ZERO_THRESHOLD {
                    snodep = snodpt;
                    dens = densgt;
                } else {
                    snodep = snodpt + hrsnow;
                    if snodep > WB11_ZERO_THRESHOLD {
                        dens = ((densgt * snodpt) + (inputs.newsnw_kg_m3 * hrsnow)) / snodep;
                    } else {
                        dens = 0.0;
                    }
                }
            } else {
                let snodpt = snodep;
                if hrsnow > WB11_ZERO_THRESHOLD {
                    snodep += hrsnow;
                    if snodep > WB11_ZERO_THRESHOLD {
                        dens = ((dens * snodpt) + (inputs.newsnw_kg_m3 * hrsnow)) / snodep;
                    } else {
                        dens = 0.0;
                    }
                }

                if snodep > WB11_ZERO_THRESHOLD {
                    let positive_temperature_c_day_increment = hrtemp_c.max(0.0) / 24.0;
                    snow_albedo_state_after = Self::update_hourly_opt_in_snow_albedo_state(
                        phase_class,
                        inputs.snow_melt_model,
                        inputs.snow_albedo_model,
                        snow_albedo_state_after,
                        snodep,
                        dens,
                        hrsnow,
                        inputs.newsnw_kg_m3,
                        positive_temperature_c_day_increment,
                        inputs.underlying_surface_albedo,
                    )?;
                    albedo_updated_this_hour = true;
                    let shortwave_absorbed_fraction = match inputs.snow_melt_model {
                        SnowMeltModel::LegacyCoe => 1.0,
                        SnowMeltModel::CoeShortwaveAlbedoV1 => snow_albedo_state_after
                            .ok_or(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                                phase_class,
                                symbol: BoundarySymbol::from("snow_albedo"),
                            })?
                            .shortwave_absorbed_fraction(),
                    };
                    let melt_computation = Self::compute_simimpl29_melt_hour(
                        phase_class,
                        inputs.canopy_cover_fraction,
                        hrad_mj_m2,
                        cloud_fraction,
                        hrtemp_c,
                        inputs.dewpoint_c,
                        inputs.wind_m_s,
                        hrrain,
                        snodep,
                        dens,
                        shortwave_absorbed_fraction,
                    )?;
                    melt_branch_active = 1.0;
                    melt_terms = melt_computation.terms;
                    let wmelt = melt_computation.wmelt_m;
                    melt_raw_m = wmelt;
                    let smelt = if wmelt > WB11_ZERO_THRESHOLD {
                        openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                            wmelt,
                            dens,
                        )
                        .map_err(|error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                                &error,
                            )
                        })?
                    } else {
                        0.0
                    };
                    let snodpt_after_inputs = snodep;
                    snodep = snodpt_after_inputs - smelt;
                    if snodep <= WB11_ZERO_THRESHOLD {
                        if smelt > 0.0 {
                            melt_m =
                                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                    snodpt_after_inputs,
                                    dens,
                                )
                                .map_err(|error| {
                                    Self::unit_conversion_guard_error(
                                        phase_class,
                                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                                        &error,
                                    )
                                })?;
                        }
                        snodep = 0.0;
                        dens = 0.0;
                    } else if dens >= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
                        if smelt > 0.0 {
                            melt_m =
                                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                    smelt,
                                    dens,
                                )
                                .map_err(|error| {
                                    Self::unit_conversion_guard_error(
                                        phase_class,
                                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                                        &error,
                                    )
                                })?;
                        } else {
                            melt_m = wmelt.min(0.0);
                        }
                    } else {
                        let mut densgt = dens * (snodpt_after_inputs / snodep);
                        if densgt <= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
                            melt_m = wmelt.min(0.0);
                            if hrrain > WB11_ZERO_THRESHOLD {
                                let densic = openwepp_unit_boundary::conversions::water_depth_meters_to_snow_density_increment(
                                    hrrain,
                                    snodep,
                                )
                                .map_err(|error| {
                                    Self::unit_conversion_guard_error(
                                        phase_class,
                                        BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
                                        &error,
                                    )
                                })?;
                                if densic
                                    <= (SIMIMPL29_DENSITY_MELT_GATE_KG_M3 - densgt)
                                        + WB11_ZERO_THRESHOLD
                                {
                                    rain_retained_m = hrrain;
                                    densgt += densic;
                                } else {
                                    rain_retained_m =
                                        openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                            snodep,
                                            SIMIMPL29_DENSITY_MELT_GATE_KG_M3 - densgt,
                                        )
                                        .map_err(|error| {
                                            Self::unit_conversion_guard_error(
                                                phase_class,
                                                BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
                                                &error,
                                            )
                                        })?;
                                    densgt = SIMIMPL29_DENSITY_MELT_GATE_KG_M3;
                                }
                            }
                        } else {
                            melt_m =
                                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                    snodep,
                                    densgt - SIMIMPL29_DENSITY_MELT_GATE_KG_M3,
                                )
                                .map_err(|error| {
                                    Self::unit_conversion_guard_error(
                                        phase_class,
                                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                                        &error,
                                    )
                                })?;
                            densgt = SIMIMPL29_DENSITY_MELT_GATE_KG_M3;
                        }
                        dens = densgt;
                    }
                }
            }

            if !albedo_updated_this_hour {
                let positive_temperature_c_day_increment = hrtemp_c.max(0.0) / 24.0;
                snow_albedo_state_after = Self::update_hourly_opt_in_snow_albedo_state(
                    phase_class,
                    inputs.snow_melt_model,
                    inputs.snow_albedo_model,
                    snow_albedo_state_after,
                    snodep,
                    dens,
                    hrsnow,
                    inputs.newsnw_kg_m3,
                    positive_temperature_c_day_increment,
                    inputs.underlying_surface_albedo,
                )?;
            }

            if dens > SIMIMPL29_SNOW_DENSITY_CAP_KG_M3 {
                dens = SIMIMPL29_SNOW_DENSITY_CAP_KG_M3;
            }
            if snodep <= WB11_ZERO_THRESHOLD {
                snodep = 0.0;
                dens = 0.0;
            }
            if depth_before_m > WB11_ZERO_THRESHOLD
                && hrrain > rain_retained_m + WB11_ZERO_THRESHOLD
            {
                rain_released_m = hrrain - rain_retained_m;
            }

            accumulation_water_m += hrsnow * 0.1;
            total_rain_retained_m += rain_retained_m;
            total_rain_released_m += rain_released_m;

            hourly_state.push(SnowHourlyState {
                hour,
                depth_before_m,
                depth_available_m,
                density_before_kg_m3,
                depth_after_m: snodep,
                density_after_kg_m3: dens,
                rain_retained_m,
                rain_released_m,
                melt_raw_m,
                melt_m,
                melt_amelt_in: melt_terms.amelt_in,
                melt_bmelt_in: melt_terms.bmelt_in,
                melt_cmelt_in: melt_terms.cmelt_in,
                melt_dmelt_in: melt_terms.dmelt_in,
                melt_hrtef_f: melt_terms.hrtef_f,
                melt_hrdtf_f: melt_terms.hrdtf_f,
                melt_vwmph: melt_terms.vwmph,
                melt_rainin: melt_terms.rainin,
                melt_wind_adjustment: melt_terms.wind_adjustment,
                melt_branch_active,
                dewpoint_c: inputs.dewpoint_c,
                wind_m_s: inputs.wind_m_s,
            });
        }

        let raw_melt_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_raw_m)
            .sum::<f64>();
        let melt_redistribution = Self::redistribute_daily_signed_snowmelt(&mut hourly_state);
        for hourly in &mut hourly_state {
            if hourly.rain_released_m > WB11_ZERO_THRESHOLD {
                hourly.melt_m += hourly.rain_released_m;
            }
        }
        let available_runtime_swe_for_state_loss =
            inputs.runtime_swe_m + accumulation_water_m + total_rain_retained_m;
        if !available_runtime_swe_for_state_loss.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: available_runtime_swe_for_state_loss,
            });
        }
        if !melt_redistribution.snowpack_state_loss_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: melt_redistribution.snowpack_state_loss_m,
            });
        }
        let bounded_state_loss_m = if melt_redistribution.snowpack_state_loss_m
            > available_runtime_swe_for_state_loss
                + SIMIMPL29_SNOWPACK_STATE_LOSS_OVERDRAW_TOLERANCE_M
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: available_runtime_swe_for_state_loss
                    - melt_redistribution.snowpack_state_loss_m,
                minimum: Some(0.0),
                maximum: None,
            });
        } else if melt_redistribution.snowpack_state_loss_m > available_runtime_swe_for_state_loss {
            available_runtime_swe_for_state_loss
        } else {
            melt_redistribution.snowpack_state_loss_m
        };
        let runtime_swe_after_raw = available_runtime_swe_for_state_loss - bounded_state_loss_m;
        if !runtime_swe_after_raw.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: runtime_swe_after_raw,
            });
        }
        let runtime_swe_after = if runtime_swe_after_raw <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            runtime_swe_after_raw
        };
        if runtime_swe_after <= WB11_ZERO_THRESHOLD {
            snodep = 0.0;
            dens = 0.0;
        } else if dens > WB11_ZERO_THRESHOLD {
            snodep =
                openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                    runtime_swe_after,
                    dens,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                        &error,
                    )
                })?;
        }
        let signed_s = melt_redistribution.routed_melt_total_m
            - accumulation_water_m
            - total_rain_retained_m;
        let routed_melt_total_m =
            melt_redistribution.routed_melt_total_m + total_rain_released_m;
        if !signed_s.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_SNOW_COUPLING_S),
                value: signed_s,
                minimum: None,
                maximum: None,
            });
        }
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.routed_melt_m"),
            routed_melt_total_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            runtime_swe_after,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            snodep,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            dens,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        Ok(SnowCouplingOutcome {
            signed_s,
            accumulation: accumulation_water_m,
            rain_retained: total_rain_retained_m,
            rain_released: total_rain_released_m,
            raw_melt: raw_melt_total_m,
            redistributed_melt: melt_redistribution.routed_melt_total_m,
            snowpack_state_loss: bounded_state_loss_m,
            runtime_swe: runtime_swe_after,
            runtime_depth_m: snodep,
            runtime_density_kg_m3: dens,
            runtime_settle_day_count: settle_day_count,
            snow_albedo_state_after,
            hourly_state,
        })
    }
}
