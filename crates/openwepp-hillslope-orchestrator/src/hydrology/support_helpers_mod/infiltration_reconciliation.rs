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
        if !melt_model.requires_snow_albedo_state() {
            return Ok(None);
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

    fn snow_liquid_holding_capacity_m(snow_depth_m: f64, snow_density_kg_m3: f64) -> f64 {
        if snow_depth_m <= WB11_ZERO_THRESHOLD
            || snow_density_kg_m3 <= WB11_ZERO_THRESHOLD
            || snow_density_kg_m3 >= SIMIMPL29_RHO_ICE_KG_M3
        {
            return 0.0;
        }
        let pore_fraction = 1.0 - (snow_density_kg_m3 / SIMIMPL29_RHO_ICE_KG_M3);
        (SIMIMPL29_LIQUID_HOLDING_CAPACITY_VOLUME_FRACTION * pore_fraction * snow_depth_m)
            .max(0.0)
    }

    fn coe_open_sublimation_stage_a_hour_m(
        phase_class: HillslopeKernelPhaseClass,
        canopy_cover_fraction: f64,
        wind_m_s: f64,
        air_temperature_c: f64,
        dewpoint_c: f64,
        snow_depth_m: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::coe_open_sublimation_hour_m(
            phase_class,
            canopy_cover_fraction,
            wind_m_s,
            air_temperature_c,
            dewpoint_c,
            snow_depth_m,
            0.0,
        )
    }

    fn coe_open_sublimation_stage_b_hour_m(
        phase_class: HillslopeKernelPhaseClass,
        canopy_cover_fraction: f64,
        wind_m_s: f64,
        air_temperature_c: f64,
        dewpoint_c: f64,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            snow_density_kg_m3,
            Some(0.0),
            None,
        )?;
        let surface_layer_depth_m =
            snow_depth_m.min(SNOW_SUBLIMATION_STAGE_B_ACTIVE_LAYER_DEPTH_M);
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation.surface_layer_depth_m"),
            surface_layer_depth_m,
            Some(0.0),
            Some(SNOW_SUBLIMATION_STAGE_B_ACTIVE_LAYER_DEPTH_M),
        )?;
        let surface_temperature_c = air_temperature_c.min(0.0);
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation.surface_temperature_c"),
            surface_temperature_c,
            None,
            Some(0.0),
        )?;
        let surface_layer_mass_kg_m2 = surface_layer_depth_m * snow_density_kg_m3;
        let surface_layer_cold_content_j_m2 = if surface_temperature_c < 0.0 {
            SNOW_SUBLIMATION_STAGE_B_ICE_HEAT_CAPACITY_J_KG_K
                * surface_layer_mass_kg_m2
                * surface_temperature_c
        } else {
            0.0
        };
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation.surface_layer_cold_content_j_m2"),
            surface_layer_cold_content_j_m2,
            None,
            Some(0.0),
        )?;

        Self::coe_open_sublimation_hour_m(
            phase_class,
            canopy_cover_fraction,
            wind_m_s,
            air_temperature_c,
            dewpoint_c,
            snow_depth_m,
            surface_temperature_c,
        )
    }

    fn coe_open_sublimation_hour_m(
        phase_class: HillslopeKernelPhaseClass,
        canopy_cover_fraction: f64,
        wind_m_s: f64,
        air_temperature_c: f64,
        dewpoint_c: f64,
        snow_depth_m: f64,
        surface_temperature_c: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB15_SYMBOL_PLANT_CANCOV),
            canopy_cover_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation.wind_m_s"),
            wind_m_s,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation.air_temperature_c"),
            air_temperature_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation.dewpoint_c"),
            dewpoint_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            snow_depth_m,
            Some(0.0),
            None,
        )?;

        if snow_depth_m <= WB11_ZERO_THRESHOLD || wind_m_s <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation.surface_temperature_c"),
            surface_temperature_c,
            None,
            Some(0.0),
        )?;

        let roughness_ratio =
            SIMIMPL29_WIND_MEASUREMENT_HEIGHT_M / SNOW_SUBLIMATION_ROUGHNESS_LENGTH_M;
        let neutral_transfer_coefficient =
            (SNOW_SUBLIMATION_VON_KARMAN / roughness_ratio.ln()).powi(2);
        let surface_vapor_pressure_pa = Self::saturation_vapor_pressure_water_kpa(
            surface_temperature_c,
        ) * SNOW_SUBLIMATION_KPA_TO_PA;
        let air_vapor_pressure_pa =
            Self::saturation_vapor_pressure_water_kpa(dewpoint_c) * SNOW_SUBLIMATION_KPA_TO_PA;
        let vapor_pressure_deficit_pa = (surface_vapor_pressure_pa - air_vapor_pressure_pa).max(0.0);
        if vapor_pressure_deficit_pa <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }

        let air_temperature_k =
            (air_temperature_c + SNOW_SUBLIMATION_SURFACE_TEMP_K)
                .max(SNOW_SUBLIMATION_MIN_AIR_TEMP_K);
        let vapor_density_deficit_kg_m3 = SNOW_SUBLIMATION_WATER_MOLECULAR_WEIGHT_KG_MOL
            * vapor_pressure_deficit_pa
            / (SNOW_SUBLIMATION_UNIVERSAL_GAS_CONSTANT_J_MOL_K * air_temperature_k);
        let open_canopy_fraction = (1.0 - canopy_cover_fraction).clamp(0.0, 1.0);
        let sublimation_kg_m2 = neutral_transfer_coefficient
            * wind_m_s
            * vapor_density_deficit_kg_m3
            * FROST_RUNTIME_SECONDS_PER_HOUR
            * open_canopy_fraction;
        let sublimation_m = sublimation_kg_m2 / SNOW_SUBLIMATION_RHO_WATER_KG_M3;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation"),
            sublimation_m,
            Some(0.0),
            None,
        )?;
        Ok(sublimation_m)
    }

    fn saturation_vapor_pressure_water_kpa(temperature_c: f64) -> f64 {
        0.6108 * ((17.27 * temperature_c) / (temperature_c + 237.3)).exp()
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
            return Ok(SnowMeltComputation { wmelt_m: 0.0 });
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
        Ok(SnowMeltComputation { wmelt_m })
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

        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            inputs.runtime_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            inputs.runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL),
            inputs.runtime_settle_day_count,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.coe_boundary_depth_m"),
            inputs.coe_boundary_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.coe_boundary_density_kg_m3"),
            inputs.coe_boundary_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.coe_boundary_settle_day_count"),
            inputs.coe_boundary_settle_day_count,
            Some(0.0),
            None,
        )?;

        let mut boundary_depth_m = if inputs.snow_density_model == SnowDensityModel::LegacyWepp {
            inputs.runtime_depth_m
        } else {
            inputs.coe_boundary_depth_m
        };
        let mut boundary_density_kg_m3 =
            if inputs.snow_density_model == SnowDensityModel::LegacyWepp {
                inputs.runtime_density_kg_m3
            } else {
                inputs.coe_boundary_density_kg_m3
            };
        let mut settle_day_count = if inputs.snow_density_model == SnowDensityModel::LegacyWepp {
            inputs.runtime_settle_day_count
        } else {
            inputs.coe_boundary_settle_day_count
        };

        if boundary_depth_m <= WB11_ZERO_THRESHOLD && inputs.runtime_swe_m > WB11_ZERO_THRESHOLD {
            if boundary_density_kg_m3 <= WB11_ZERO_THRESHOLD {
                boundary_density_kg_m3 = inputs.newsnw_kg_m3;
            }
            boundary_depth_m =
                openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                    inputs.runtime_swe_m,
                    boundary_density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                        &error,
                    )
                })?;
        }
        if boundary_depth_m > WB11_ZERO_THRESHOLD
            && boundary_density_kg_m3 <= WB11_ZERO_THRESHOLD
        {
            boundary_density_kg_m3 = inputs.newsnw_kg_m3;
        }
        if boundary_depth_m <= WB11_ZERO_THRESHOLD {
            boundary_depth_m = 0.0;
            boundary_density_kg_m3 = 0.0;
        }

        let mut snodep = boundary_depth_m;
        let mut dens = boundary_density_kg_m3;
        let daily_mean_temp = f64::midpoint(inputs.tmax_c, inputs.tmin_c);

        let mut accumulation_water_m = 0.0;
        let mut total_rain_retained_m = 0.0;
        let mut total_rain_released_m = 0.0;
        let capacity_drainage_opt_in = matches!(
            inputs.snow_melt_model,
            SnowMeltModel::CoeLiquidHoldingCapacityV1
                | SnowMeltModel::CoeOpenSublimationStageAV1
                | SnowMeltModel::CoeOpenSublimationStageBV1
        );
        let mut liquid_water_retained_m = if capacity_drainage_opt_in {
            inputs.liquid_water_retained_m
        } else {
            0.0
        };
        let mut total_liquid_water_released_m = 0.0;
        let mut total_sublimation_m = 0.0;
        let mut final_liquid_holding_capacity_m = 0.0;
        let mut snow_albedo_state_after = inputs.snow_albedo_state;
        let mut hourly_state = Vec::with_capacity(SIMIMPL29_HOURS_PER_DAY);

        for hour in 1..=SIMIMPL29_HOURS_PER_DAY {
            let hourly = inputs.hourly[hour - 1];
            let hrrain = hourly.rain_m;
            let hrsnow = hourly.snowfall_m;
            let hrad_mj_m2 = hourly.radiation_mj_m2;
            let hrtemp_c = hourly.air_temperature_c;
            let cloud_fraction = hourly.cloud_fraction;
            let future_snowfall_this_day = inputs.hourly[hour..]
                .iter()
                .any(|future| future.snowfall_m > WB11_ZERO_THRESHOLD);

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
            let mut rain_retained_m = 0.0;
            let mut rain_released_m = 0.0;
            let liquid_water_retained_before_m = liquid_water_retained_m;
            let mut liquid_holding_capacity_m = 0.0;
            let mut liquid_water_released_m = 0.0;
            let mut sublimation_m = 0.0;
            let mut melt_raw_m = 0.0;
            let mut melt_m = 0.0;
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
                        SnowMeltModel::LegacyCoe
                        | SnowMeltModel::CoeWinterThawStateLossV1
                        | SnowMeltModel::CoeLiquidHoldingCapacityV1
                        | SnowMeltModel::CoeOpenSublimationStageAV1
                        | SnowMeltModel::CoeOpenSublimationStageBV1 => 1.0,
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
                        if capacity_drainage_opt_in
                            && liquid_water_retained_m > WB11_ZERO_THRESHOLD
                        {
                            melt_m += liquid_water_retained_m;
                            liquid_water_released_m += liquid_water_retained_m;
                            liquid_water_retained_m = 0.0;
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
                        let thaw_state_loss_opt_in =
                            inputs.snow_melt_model == SnowMeltModel::CoeWinterThawStateLossV1
                                && wmelt > WB11_ZERO_THRESHOLD;
                        let capacity_drainage_melt_opt_in =
                            capacity_drainage_opt_in && wmelt > WB11_ZERO_THRESHOLD;
                        let mut densgt = if thaw_state_loss_opt_in
                            || capacity_drainage_melt_opt_in
                        {
                            dens
                        } else {
                            dens * (snodpt_after_inputs / snodep)
                        };
                        if densgt <= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
                            if capacity_drainage_melt_opt_in {
                                liquid_holding_capacity_m =
                                    Self::snow_liquid_holding_capacity_m(snodep, densgt);
                                let mut available_capacity_m = (liquid_holding_capacity_m
                                    - liquid_water_retained_m)
                                    .max(0.0);
                                let retained_melt_m = wmelt.min(available_capacity_m);
                                let released_melt_m = (wmelt - retained_melt_m).max(0.0);
                                available_capacity_m -= retained_melt_m;
                                rain_retained_m = hrrain.min(available_capacity_m);
                                rain_released_m = (hrrain - rain_retained_m).max(0.0);
                                liquid_water_retained_m += retained_melt_m + rain_retained_m;
                                liquid_water_released_m += released_melt_m;
                                melt_m = released_melt_m;
                                let pack_water_after_m =
                                    openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                                        snodpt_after_inputs,
                                        dens,
                                    )
                                    .map_err(|error| {
                                        Self::unit_conversion_guard_error(
                                            phase_class,
                                            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                                            &error,
                                        )
                                    })?
                                        + rain_retained_m
                                        - released_melt_m;
                                densgt =
                                    openwepp_unit_boundary::conversions::water_depth_meters_to_snow_density_increment(
                                        pack_water_after_m.max(0.0),
                                        snodep,
                                    )
                                    .map_err(|error| {
                                        Self::unit_conversion_guard_error(
                                            phase_class,
                                            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
                                            &error,
                                        )
                                    })?;
                            } else {
                                melt_m = if thaw_state_loss_opt_in {
                                wmelt
                            } else {
                                wmelt.min(0.0)
                            };
                            }
                            if hrrain > WB11_ZERO_THRESHOLD && !capacity_drainage_melt_opt_in {
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

            if !albedo_updated_this_hour
                && (snodep > WB11_ZERO_THRESHOLD
                    || hrsnow > WB11_ZERO_THRESHOLD
                    || !future_snowfall_this_day)
            {
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
            if snodep > WB11_ZERO_THRESHOLD {
                sublimation_m = match inputs.snow_melt_model {
                    SnowMeltModel::CoeOpenSublimationStageAV1 => {
                        Self::coe_open_sublimation_stage_a_hour_m(
                            phase_class,
                            inputs.canopy_cover_fraction,
                            inputs.wind_m_s,
                            hrtemp_c,
                            inputs.dewpoint_c,
                            snodep,
                        )?
                    }
                    SnowMeltModel::CoeOpenSublimationStageBV1 => {
                        Self::coe_open_sublimation_stage_b_hour_m(
                            phase_class,
                            inputs.canopy_cover_fraction,
                            inputs.wind_m_s,
                            hrtemp_c,
                            inputs.dewpoint_c,
                            snodep,
                            dens,
                        )?
                    }
                    SnowMeltModel::LegacyCoe
                    | SnowMeltModel::CoeShortwaveAlbedoV1
                    | SnowMeltModel::CoeWinterThawStateLossV1
                    | SnowMeltModel::CoeLiquidHoldingCapacityV1 => 0.0,
                };
            }
            if snodep <= WB11_ZERO_THRESHOLD {
                snodep = 0.0;
                dens = 0.0;
                if capacity_drainage_opt_in && liquid_water_retained_m > WB11_ZERO_THRESHOLD {
                    melt_m += liquid_water_retained_m;
                    liquid_water_released_m += liquid_water_retained_m;
                    liquid_water_retained_m = 0.0;
                }
            } else if capacity_drainage_opt_in {
                liquid_holding_capacity_m = Self::snow_liquid_holding_capacity_m(snodep, dens);
                if liquid_water_retained_m
                    > liquid_holding_capacity_m + WB11_ZERO_THRESHOLD
                {
                    let excess_m = liquid_water_retained_m - liquid_holding_capacity_m;
                    melt_m += excess_m;
                    liquid_water_released_m += excess_m;
                    liquid_water_retained_m = liquid_holding_capacity_m;
                }
            }
            if depth_before_m > WB11_ZERO_THRESHOLD
                && hrrain > rain_retained_m + WB11_ZERO_THRESHOLD
            {
                rain_released_m = hrrain - rain_retained_m;
            }

            accumulation_water_m += hrsnow * 0.1;
            total_rain_retained_m += rain_retained_m;
            total_rain_released_m += rain_released_m;
            total_liquid_water_released_m += liquid_water_released_m;
            total_sublimation_m += sublimation_m;
            final_liquid_holding_capacity_m = liquid_holding_capacity_m;

            hourly_state.push(SnowHourlyState {
                rain_released_m,
                liquid_holding_capacity_m,
                liquid_water_retained_before_m,
                liquid_water_retained_after_m: liquid_water_retained_m,
                liquid_water_released_m,
                sublimation_m,
                melt_raw_m,
                melt_m,
            });
        }

        if let Some(last_hour) = hourly_state.last() {
            final_liquid_holding_capacity_m = last_hour.liquid_holding_capacity_m;
            liquid_water_retained_m = last_hour.liquid_water_retained_after_m;
        }
        let hourly_liquid_released_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.liquid_water_released_m)
            .sum::<f64>();
        let _max_retained_liquid_before_hour_m = hourly_state
            .iter()
            .map(|hourly| hourly.liquid_water_retained_before_m)
            .fold(0.0, f64::max);
        if (hourly_liquid_released_total_m - total_liquid_water_released_m).abs()
            > WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("snow_liquid_water_released_m"),
                value: hourly_liquid_released_total_m - total_liquid_water_released_m,
                minimum: Some(-WB11_ZERO_THRESHOLD),
                maximum: Some(WB11_ZERO_THRESHOLD),
            });
        }
        let hourly_sublimation_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.sublimation_m)
            .sum::<f64>();
        if (hourly_sublimation_total_m - total_sublimation_m).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(SNOW_HOURLY_SUBLIMATION_ROOT),
                value: hourly_sublimation_total_m - total_sublimation_m,
                minimum: Some(-WB11_ZERO_THRESHOLD),
                maximum: Some(WB11_ZERO_THRESHOLD),
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
        if !total_sublimation_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(SNOW_HOURLY_SUBLIMATION_ROOT),
                value: total_sublimation_m,
            });
        }
        let available_swe_after_state_loss_m =
            (available_runtime_swe_for_state_loss - bounded_state_loss_m).max(0.0);
        let bounded_sublimation_m = total_sublimation_m.min(available_swe_after_state_loss_m);
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow_sublimation"),
            bounded_sublimation_m,
            Some(0.0),
            Some(available_swe_after_state_loss_m),
        )?;
        let runtime_swe_after_raw =
            available_runtime_swe_for_state_loss - bounded_state_loss_m - bounded_sublimation_m;
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
            liquid_water_retained_m = 0.0;
            final_liquid_holding_capacity_m = 0.0;
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
        let routed_snowpack_m = if capacity_drainage_opt_in {
            bounded_state_loss_m
        } else {
            melt_redistribution.routed_melt_total_m
        };
        let signed_s = routed_snowpack_m - accumulation_water_m - total_rain_retained_m;
        let routed_melt_total_m = routed_snowpack_m + total_rain_released_m;
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
            liquid_holding_capacity: final_liquid_holding_capacity_m,
            liquid_water_retained: liquid_water_retained_m,
            liquid_water_released: total_liquid_water_released_m,
            sublimation: bounded_sublimation_m,
            raw_melt: raw_melt_total_m,
            redistributed_melt: melt_redistribution.routed_melt_total_m,
            snowpack_state_loss: bounded_state_loss_m,
            runtime_swe: runtime_swe_after,
            runtime_depth_m: snodep,
            runtime_density_kg_m3: dens,
            runtime_settle_day_count: settle_day_count,
            snow_albedo_state_after,
        })
    }
}
