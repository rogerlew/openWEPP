#[allow(clippy::wildcard_imports)]
use super::super::*;
use openwepp_unit_boundary::TemperatureCelsius;

#[derive(Debug, Clone, Copy)]
struct ActiveSnowBoundaryState {
    depth_m: f64,
    density_kg_m3: f64,
    settle_day_count: f64,
}

#[cfg(test)]
mod cqr_row5_tests {
    use super::*;

    const PHASE: HillslopeKernelPhaseClass =
        HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;

    fn simimpl29_melt(
        vwind_m_s: f64,
        hrrain_m: f64,
        tdpt_c: f64,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
        hrad_mj_m2: f64,
        hrtemp_c: f64,
    ) -> Result<SnowMeltComputation, Wb11HydrologyKernelGuardError> {
        Wb11HydrologyKernel::compute_simimpl29_melt_hour(
            PHASE,
            0.25,
            hrad_mj_m2,
            0.4,
            hrtemp_c,
            tdpt_c,
            vwind_m_s,
            hrrain_m,
            snow_depth_m,
            snow_density_kg_m3,
            0.7,
        )
    }

    #[test]
    fn simimpl29_melt_hour_covers_zero_wind_rain_and_cap_paths() {
        let no_pack = simimpl29_melt(0.0, 0.0, -4.0, 0.0, 120.0, 0.0, -2.0).unwrap();
        assert!(no_pack.wmelt_m.abs() <= 1.0e-12);

        let zero_wind = simimpl29_melt(0.0, 0.0, -2.0, 0.4, 240.0, 0.2, 1.0).unwrap();
        assert!(zero_wind.wmelt_m.is_finite());
        let zero_wind_diagnostics = zero_wind.diagnostics.expect("verbose melt diagnostics");
        assert!(
            (zero_wind_diagnostics.coe_melt_applied_m
                - zero_wind_diagnostics.coe_melt_uncapped_m
                - zero_wind_diagnostics.coe_melt_cap_adjustment_m)
                .abs()
                <= 1.0e-12
        );

        let windy_rain = simimpl29_melt(4.0, 0.004, 1.5, 0.4, 240.0, 0.2, 3.0).unwrap();
        assert!(windy_rain.wmelt_m.is_finite());
        let windy_diagnostics = windy_rain.diagnostics.expect("verbose melt diagnostics");
        assert!(
            (windy_diagnostics.coe_melt_amelt_m - 0.000_161_886_9).abs() <= 1.0e-12
        );
        assert!(
            (windy_diagnostics.coe_melt_bmelt_m - (-0.000_257_175)).abs() <= 1.0e-12
        );
        assert!(
            (windy_diagnostics.coe_melt_cmelt_m - 0.000_275_696_527_821_433_83).abs()
                <= 1.0e-12
        );
        assert!(
            (windy_diagnostics.coe_melt_dmelt_m - 0.000_075_599_848_8).abs()
                <= 1.0e-12
        );
        let windy_component_sum = windy_diagnostics.coe_melt_amelt_m
            + windy_diagnostics.coe_melt_bmelt_m
            + windy_diagnostics.coe_melt_cmelt_m
            + windy_diagnostics.coe_melt_dmelt_m;
        assert!(
            (windy_diagnostics.coe_melt_uncapped_m - windy_component_sum).abs()
                <= 1.0e-12
        );

        let capped = simimpl29_melt(8.0, 0.03, 6.0, 0.01, 100.0, 80.0, 12.0).unwrap();
        let maximum_melt_m =
            openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                0.01, 100.0,
            )
            .unwrap();
        assert!(capped.wmelt_m <= maximum_melt_m + 1.0e-12);
        let capped_diagnostics = capped.diagnostics.expect("verbose melt diagnostics");
        assert!(capped_diagnostics.coe_melt_cap_adjustment_m < 0.0);
        let component_sum = capped_diagnostics.coe_melt_amelt_m
            + capped_diagnostics.coe_melt_bmelt_m
            + capped_diagnostics.coe_melt_cmelt_m
            + capped_diagnostics.coe_melt_dmelt_m;
        assert!(
            (capped_diagnostics.coe_melt_applied_m
                - component_sum
                - capped_diagnostics.coe_melt_cap_adjustment_m)
                .abs()
                <= 1.0e-12
        );

        let error = Wb11HydrologyKernel::compute_simimpl29_melt_hour(
            PHASE, -0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 100.0, 1.0,
        )
        .unwrap_err();
        assert!(matches!(
            error,
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
        ));

        let corrupted = DirectSnowMeltHourDiagnostics {
            coe_melt_amelt_m: 0.001,
            coe_melt_uncapped_m: 0.001,
            coe_melt_applied_m: 0.002,
            ..DirectSnowMeltHourDiagnostics::default()
        };
        let error = Wb11HydrologyKernel::validate_coe_melt_diagnostic_closure(PHASE, corrupted)
            .expect_err("a corrupted component ledger must fail before publication");
        match error {
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { symbol, .. } => {
                assert_eq!(
                    symbol.as_str(),
                    "snow.hourly.coe_melt_component_closure_residual_m"
                );
            }
            other => panic!("unexpected component-closure error: {other}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ActiveSnowPackState {
    depth_m: f64,
    density_kg_m3: f64,
    settle_day_count: f64,
    liquid_water_retained_m: f64,
    snow_albedo_state_after: Option<SnowAlbedoState>,
}

#[derive(Debug, Clone, Copy)]
struct ActiveSnowHourPolicy {
    daily_mean_temp_c: f64,
    capacity_drainage_opt_in: bool,
    capture: DirectSnowDiagnosticCapture,
}

#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::struct_field_names)]
struct ActiveSnowHourlyFluxes {
    rain_retained_m: f64,
    rain_released_m: f64,
    liquid_holding_capacity_m: f64,
    liquid_water_released_m: f64,
    sublimation_m: f64,
    melt_raw_m: f64,
    melt_m: f64,
    melt_diagnostics: Option<DirectSnowMeltHourDiagnostics>,
}

impl ActiveSnowHourlyFluxes {
    fn into_hourly_state(
        self,
        state_before: ActiveSnowPackState,
        state_after: ActiveSnowPackState,
    ) -> SnowHourlyState {
        SnowHourlyState {
            rain_released_m: self.rain_released_m,
            liquid_holding_capacity_m: self.liquid_holding_capacity_m,
            liquid_water_retained_before_m: state_before.liquid_water_retained_m,
            liquid_water_retained_after_m: state_after.liquid_water_retained_m,
            liquid_water_released_m: self.liquid_water_released_m,
            sublimation_m: self.sublimation_m,
            melt_raw_m: self.melt_raw_m,
            melt_m: self.melt_m,
            melt_diagnostics: self.melt_diagnostics,
            pack_depth_before_m: state_before.depth_m,
            pack_depth_after_m: state_after.depth_m,
            pack_density_before_kg_m3: state_before.density_kg_m3,
            pack_density_after_kg_m3: state_after.density_kg_m3,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::struct_field_names)]
struct ActiveSnowDailyTotals {
    accumulation_water_m: f64,
    rain_retained_m: f64,
    rain_released_m: f64,
    liquid_water_released_m: f64,
    sublimation_m: f64,
    final_liquid_holding_capacity_m: f64,
}

impl ActiveSnowDailyTotals {
    fn add_fluxes(&mut self, snowfall_m: f64, fluxes: ActiveSnowHourlyFluxes) {
        self.accumulation_water_m += snowfall_m * 0.1;
        self.rain_retained_m += fluxes.rain_retained_m;
        self.rain_released_m += fluxes.rain_released_m;
        self.liquid_water_released_m += fluxes.liquid_water_released_m;
        self.sublimation_m += fluxes.sublimation_m;
        self.final_liquid_holding_capacity_m = fluxes.liquid_holding_capacity_m;
    }
}

#[cfg(test)]
mod wet_compaction_operand_tests {
    use super::*;

    const PHASE: HillslopeKernelPhaseClass =
        HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;

    fn hourly_melt(melt_raw_m: f64) -> SnowHourlyState {
        SnowHourlyState {
            rain_released_m: 0.0,
            liquid_holding_capacity_m: 0.0,
            liquid_water_retained_before_m: 0.0,
            liquid_water_retained_after_m: 0.0,
            liquid_water_released_m: 0.0,
            sublimation_m: 0.0,
            melt_raw_m,
            melt_m: melt_raw_m.max(0.0),
            melt_diagnostics: None,
            pack_depth_before_m: 1.0,
            pack_depth_after_m: 1.0,
            pack_density_before_kg_m3: 200.0,
            pack_density_after_kg_m3: 200.0,
        }
    }

    #[test]
    fn helper_sums_positive_melt_and_contact_rain_and_fails_closed() {
        let hourly = [hourly_melt(0.011), hourly_melt(-0.007)];
        let totals = ActiveSnowDailyTotals {
            rain_retained_m: 0.003,
            rain_released_m: 0.005,
            ..ActiveSnowDailyTotals::default()
        };
        let actual = Wb11HydrologyKernel::wet_compaction_liquid_input_m(
            PHASE, &hourly, totals,
        )
        .expect("valid wet-compaction source operands must compute");
        assert!((actual - 0.019).abs() <= 1.0e-12);

        let nonfinite = ActiveSnowDailyTotals {
            rain_retained_m: f64::NAN,
            ..ActiveSnowDailyTotals::default()
        };
        let error = Wb11HydrologyKernel::wet_compaction_liquid_input_m(
            PHASE, &hourly, nonfinite,
        )
        .expect_err("non-finite wet-compaction input must fail");
        assert!(matches!(
            error,
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol { ref symbol, .. }
                if symbol.as_str() == "snow.wet_compaction_liquid_input_m"
        ));

        let negative = ActiveSnowDailyTotals {
            rain_released_m: -0.020,
            ..ActiveSnowDailyTotals::default()
        };
        let error = Wb11HydrologyKernel::wet_compaction_liquid_input_m(
            PHASE, &hourly, negative,
        )
        .expect_err("negative wet-compaction input must fail");
        assert!(matches!(
            error,
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { ref symbol, .. }
                if symbol.as_str() == "snow.wet_compaction_liquid_input_m"
        ));
    }
}

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
            false,
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            snow_density_kg_m3,
            Some(0.0),
            None,
        )?;
        let surface_layer_depth_m =
            snow_depth_m.min(SNOW_SUBLIMATION_STAGE_B_ACTIVE_LAYER_DEPTH_M);
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation.surface_layer_depth_m"),
            surface_layer_depth_m,
            Some(0.0),
            Some(SNOW_SUBLIMATION_STAGE_B_ACTIVE_LAYER_DEPTH_M),
        )?;
        let surface_temperature_c = air_temperature_c.min(0.0);
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation.surface_temperature_c"),
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation.surface_layer_cold_content_j_m2"),
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
            false,
        )
    }

    // This boundary mirrors the contract's complete hourly forcing tuple. Keeping
    // those operands explicit makes the legacy-water and EB-03 ice-saturation
    // branches visible at each call site.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn coe_open_sublimation_hour_m(
        phase_class: HillslopeKernelPhaseClass,
        canopy_cover_fraction: f64,
        wind_m_s: f64,
        air_temperature_c: f64,
        dewpoint_c: f64,
        snow_depth_m: f64,
        surface_temperature_c: f64,
        surface_uses_ice_saturation: bool,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB15_SYMBOL_PLANT_CANCOV),
            canopy_cover_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation.wind_m_s"),
            wind_m_s,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation.air_temperature_c"),
            air_temperature_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation.dewpoint_c"),
            dewpoint_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            snow_depth_m,
            Some(0.0),
            None,
        )?;

        if snow_depth_m <= WB11_ZERO_THRESHOLD || wind_m_s <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation.surface_temperature_c"),
            surface_temperature_c,
            None,
            Some(0.0),
        )?;

        let roughness_ratio =
            SIMIMPL29_WIND_MEASUREMENT_HEIGHT_M / SNOW_SUBLIMATION_ROUGHNESS_LENGTH_M;
        let neutral_transfer_coefficient =
            (SNOW_SUBLIMATION_VON_KARMAN / roughness_ratio.ln()).powi(2);
        let surface_vapor_pressure_pa = Self::surface_vapor_pressure_pa(
            phase_class,
            surface_temperature_c,
            surface_uses_ice_saturation,
        )?;
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation"),
            sublimation_m,
            Some(0.0),
            None,
        )?;
        Ok(sublimation_m)
    }

    fn surface_vapor_pressure_pa(
        phase_class: HillslopeKernelPhaseClass,
        surface_temperature_c: f64,
        surface_uses_ice_saturation: bool,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if !surface_uses_ice_saturation {
            return Ok(
                Self::saturation_vapor_pressure_water_kpa(surface_temperature_c)
                    * SNOW_SUBLIMATION_KPA_TO_PA,
            );
        }
        openwepp_meteorology::surface_energy::saturation_vapor_pressure_snobal_pa(
            TemperatureCelsius::try_new(surface_temperature_c).map_err(|_| {
                Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("snow_sublimation.surface_temperature_c"),
                    value: surface_temperature_c,
                }
            })?,
        )
        .map_err(
            |_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("snow_sublimation.surface_temperature_k"),
                value: surface_temperature_c + SNOW_SUBLIMATION_SURFACE_TEMP_K,
                minimum: Some(f64::MIN_POSITIVE),
                maximum: Some(SNOW_SUBLIMATION_SURFACE_TEMP_K),
            },
        )
        .map(openwepp_meteorology::surface_energy::PressurePascals::as_pascals)
    }

    fn saturation_vapor_pressure_water_kpa(temperature_c: f64) -> f64 {
        0.6108 * ((17.27 * temperature_c) / (temperature_c + 237.3)).exp()
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    #[cfg(test)]
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
        Self::compute_simimpl29_melt_hour_with_capture(
            phase_class,
            cancov,
            hrad_mj_m2,
            cloud_fraction,
            hrtemp_c,
            tdpt_c,
            vwind_m_s,
            hrrain_m,
            snow_depth_m,
            snow_density_kg_m3,
            shortwave_absorbed_fraction,
            DirectSnowDiagnosticCapture::Verbose,
        )
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn compute_simimpl29_melt_hour_with_capture(
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
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<SnowMeltComputation, Wb11HydrologyKernelGuardError> {
        Self::validate_simimpl29_melt_inputs(
            phase_class,
            cancov,
            hrad_mj_m2,
            cloud_fraction,
            vwind_m_s,
            hrrain_m,
            snow_depth_m,
            snow_density_kg_m3,
            shortwave_absorbed_fraction,
        )?;

        if snow_depth_m <= WB11_ZERO_THRESHOLD || snow_density_kg_m3 <= WB11_ZERO_THRESHOLD {
            return Ok(SnowMeltComputation {
                wmelt_m: 0.0,
                diagnostics: capture
                    .is_verbose()
                    .then(DirectSnowMeltHourDiagnostics::default),
            });
        }

        let [amelt_inches, bmelt_inches, cmelt_inches, dmelt_inches] =
            Self::simimpl29_hourly_melt_inches(
            phase_class,
            cancov,
            hrad_mj_m2,
            cloud_fraction,
            hrtemp_c,
            tdpt_c,
            vwind_m_s,
            hrrain_m,
            shortwave_absorbed_fraction,
        )?;
        let melt_inches = amelt_inches + bmelt_inches + cmelt_inches + dmelt_inches;
        let wmelt_m = openwepp_unit_boundary::conversions::legacy_inches_to_meters(melt_inches).map_err(
                |error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                        &error,
                    )
                },
            )?;
        let [amelt_m, bmelt_m, cmelt_m, dmelt_m] =
            [amelt_inches, bmelt_inches, cmelt_inches, dmelt_inches].map(|term| {
            openwepp_unit_boundary::conversions::legacy_inches_to_meters(term)
        });
        let amelt_m = amelt_m.map_err(|error| {
            Self::unit_conversion_guard_error(
                phase_class,
                BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                &error,
            )
        })?;
        let bmelt_m = bmelt_m.map_err(|error| {
            Self::unit_conversion_guard_error(
                phase_class,
                BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                &error,
            )
        })?;
        let cmelt_m = cmelt_m.map_err(|error| {
            Self::unit_conversion_guard_error(
                phase_class,
                BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                &error,
            )
        })?;
        let dmelt_m = dmelt_m.map_err(|error| {
            Self::unit_conversion_guard_error(
                phase_class,
                BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                &error,
            )
        })?;
        Self::cap_simimpl29_melt_to_snowpack(
            phase_class,
            wmelt_m,
            snow_depth_m,
            snow_density_kg_m3,
            [amelt_m, bmelt_m, cmelt_m, dmelt_m],
            capture,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_simimpl29_melt_inputs(
        phase_class: HillslopeKernelPhaseClass,
        cancov: f64,
        hrad_mj_m2: f64,
        cloud_fraction: f64,
        vwind_m_s: f64,
        hrrain_m: f64,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
        shortwave_absorbed_fraction: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from("cancov"),
            cancov,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
            hrrain_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(WINTER_HOURLY_RAD_ROOT),
            hrad_mj_m2,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(WINTER_HOURLY_CLOUD_ROOT),
            cloud_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from("vwind"),
            vwind_m_s,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            snow_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            snow_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from("snow_melt_shortwave_absorbed_fraction"),
            shortwave_absorbed_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn simimpl29_hourly_melt_inches(
        phase_class: HillslopeKernelPhaseClass,
        cancov: f64,
        hrad_mj_m2: f64,
        cloud_fraction: f64,
        hrtemp_c: f64,
        tdpt_c: f64,
        vwind_m_s: f64,
        hrrain_m: f64,
        shortwave_absorbed_fraction: f64,
    ) -> Result<[f64; 4], Wb11HydrologyKernelGuardError> {
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
        Ok([amelt, bmelt, cmelt, dmelt])
    }

    fn cap_simimpl29_melt_to_snowpack(
        phase_class: HillslopeKernelPhaseClass,
        mut wmelt_m: f64,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
        components_m: [f64; 4],
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<SnowMeltComputation, Wb11HydrologyKernelGuardError> {
        if !wmelt_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                value: wmelt_m,
                minimum: Some(0.0),
                maximum: Some(snow_depth_m),
            });
        }
        let uncapped_m = wmelt_m;
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
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
            wmelt_m,
            None,
            Some(maximum_melt_m),
        )?;
        let cap_adjustment_m = wmelt_m - uncapped_m;
        Self::validate_coe_melt_component_closure(
            phase_class,
            components_m,
            cap_adjustment_m,
            wmelt_m,
        )?;
        let diagnostics = capture.is_verbose().then(|| DirectSnowMeltHourDiagnostics {
            coe_melt_amelt_m: components_m[0],
            coe_melt_bmelt_m: components_m[1],
            coe_melt_cmelt_m: components_m[2],
            coe_melt_dmelt_m: components_m[3],
            coe_melt_uncapped_m: uncapped_m,
            coe_melt_cap_adjustment_m: cap_adjustment_m,
            coe_melt_applied_m: wmelt_m,
        });
        Ok(SnowMeltComputation { wmelt_m, diagnostics })
    }

    fn validate_coe_melt_component_closure(
        phase_class: HillslopeKernelPhaseClass,
        components_m: [f64; 4],
        cap_adjustment_m: f64,
        applied_m: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let component_sum_m = components_m.iter().sum::<f64>();
        let residual_m = applied_m - component_sum_m - cap_adjustment_m;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.hourly.coe_melt_component_closure_residual_m"),
            residual_m,
            Some(-1.0e-12),
            Some(1.0e-12),
        )
    }

    #[cfg(test)]
    fn validate_coe_melt_diagnostic_closure(
        phase_class: HillslopeKernelPhaseClass,
        diagnostics: DirectSnowMeltHourDiagnostics,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::validate_coe_melt_component_closure(
            phase_class,
            [
                diagnostics.coe_melt_amelt_m,
                diagnostics.coe_melt_bmelt_m,
                diagnostics.coe_melt_cmelt_m,
                diagnostics.coe_melt_dmelt_m,
            ],
            diagnostics.coe_melt_cap_adjustment_m,
            diagnostics.coe_melt_applied_m,
        )
    }

    pub(crate) fn compute_active_snow_coupling_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<SnowCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::validate_active_snow_coupling_inputs(phase_class, inputs)?;
        let boundary = Self::resolve_active_snow_boundary_state(phase_class, inputs)?;
        let capacity_drainage_opt_in = Self::snow_capacity_drainage_opt_in(inputs.snow_melt_model);
        let mut state = ActiveSnowPackState {
            depth_m: boundary.depth_m,
            density_kg_m3: boundary.density_kg_m3,
            settle_day_count: boundary.settle_day_count,
            liquid_water_retained_m: if capacity_drainage_opt_in {
                inputs.liquid_water_retained_m
            } else {
                0.0
            },
            snow_albedo_state_after: inputs.snow_albedo_state,
        };
        let daily_mean_temp = f64::midpoint(inputs.tmax_c, inputs.tmin_c);
        let hour_policy = ActiveSnowHourPolicy {
            daily_mean_temp_c: daily_mean_temp,
            capacity_drainage_opt_in,
            capture,
        };
        let mut totals = ActiveSnowDailyTotals::default();
        let mut hourly_state = Vec::with_capacity(SIMIMPL29_HOURS_PER_DAY);

        for hour in 1..=SIMIMPL29_HOURS_PER_DAY {
            let (hour_state, fluxes) = Self::compute_active_snow_coupling_hour(
                phase_class,
                inputs,
                hour,
                hour_policy,
                &mut state,
            )?;
            totals.add_fluxes(inputs.hourly[hour - 1].snowfall_m, fluxes);
            hourly_state.push(hour_state);
        }

        Self::finalize_active_snow_coupling(
            phase_class,
            inputs,
            capacity_drainage_opt_in,
            state,
            totals,
            hourly_state,
            capture,
        )
    }

    fn validate_active_snow_coupling_inputs(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
            inputs.hyetograph_rainfall_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_SNOW_RST),
            inputs.rst_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
            inputs.newsnw_kg_m3,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_SNOW_SSD),
            inputs.ssd_kg_m3,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            inputs.runtime_swe_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_TMAX),
            inputs.tmax_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_TMIN),
            inputs.tmin_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB15_SYMBOL_PLANT_CANCOV),
            inputs.canopy_cover_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("vwind"),
            inputs.wind_m_s,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("tdpt"),
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
        Ok(())
    }

    fn resolve_active_snow_boundary_state(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<ActiveSnowBoundaryState, Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            inputs.runtime_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            inputs.runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL),
            inputs.runtime_settle_day_count,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.coe_boundary_depth_m"),
            inputs.coe_boundary_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.coe_boundary_density_kg_m3"),
            inputs.coe_boundary_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.coe_boundary_settle_day_count"),
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
        let settle_day_count = if inputs.snow_density_model == SnowDensityModel::LegacyWepp {
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
        Ok(ActiveSnowBoundaryState {
            depth_m: boundary_depth_m,
            density_kg_m3: boundary_density_kg_m3,
            settle_day_count,
        })
    }

    fn snow_capacity_drainage_opt_in(snow_melt_model: SnowMeltModel) -> bool {
        matches!(
            snow_melt_model,
            SnowMeltModel::CoeLiquidHoldingCapacityV1
                | SnowMeltModel::CoeOpenSublimationStageAV1
                | SnowMeltModel::CoeOpenSublimationStageBV1
        )
    }

    fn compute_active_snow_coupling_hour(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hour: usize,
        policy: ActiveSnowHourPolicy,
        state: &mut ActiveSnowPackState,
    ) -> Result<(SnowHourlyState, ActiveSnowHourlyFluxes), Wb11HydrologyKernelGuardError> {
        let hourly = inputs.hourly[hour - 1];
        Self::validate_active_snow_hour(phase_class, hour, hourly)?;
        let future_snowfall_this_day = inputs.hourly[hour..]
            .iter()
            .any(|future| future.snowfall_m > WB11_ZERO_THRESHOLD);

        Self::advance_active_snow_settle_clock(state, hour, hourly.snowfall_m);
        let state_before = *state;
        let depth_before_m = state_before.depth_m.max(0.0);
        let mut fluxes = ActiveSnowHourlyFluxes {
            melt_diagnostics: policy
                .capture
                .is_verbose()
                .then(DirectSnowMeltHourDiagnostics::default),
            ..ActiveSnowHourlyFluxes::default()
        };
        let albedo_updated_this_hour = Self::advance_active_snowpack_for_hour(
            phase_class,
            inputs,
            hourly,
            policy,
            state,
            &mut fluxes,
        )?;

        Self::maybe_update_idle_hourly_snow_albedo(
            phase_class,
            inputs,
            hourly,
            future_snowfall_this_day,
            albedo_updated_this_hour,
            state,
        )?;
        Self::apply_active_snow_sublimation_and_liquid_limits(
            phase_class,
            inputs,
            hourly,
            policy.capacity_drainage_opt_in,
            state,
            &mut fluxes,
        )?;
        if depth_before_m > WB11_ZERO_THRESHOLD
            && hourly.rain_m > fluxes.rain_retained_m + WB11_ZERO_THRESHOLD
        {
            fluxes.rain_released_m = hourly.rain_m - fluxes.rain_retained_m;
        }

        Ok((fluxes.into_hourly_state(state_before, *state), fluxes))
    }

    fn validate_active_snow_hour(
        phase_class: HillslopeKernelPhaseClass,
        hour: usize,
        hourly: DirectSnowHourlyForcing,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || Self::hourly_symbol(SNOW_HOURLY_RAIN_ROOT, hour),
            hourly.rain_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || Self::hourly_symbol(SNOW_HOURLY_SNOWFALL_ROOT, hour),
            hourly.snowfall_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || Self::hourly_symbol(WINTER_HOURLY_RAD_ROOT, hour),
            hourly.radiation_mj_m2,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || Self::hourly_symbol(WINTER_HOURLY_AIR_TEMP_ROOT, hour),
            hourly.air_temperature_c,
            None,
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || Self::hourly_symbol(WINTER_HOURLY_CLOUD_ROOT, hour),
            hourly.cloud_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::validate_active_snow_phase_diagnostics(phase_class, hour, hourly)
    }

    fn validate_active_snow_phase_diagnostics(
        phase_class: HillslopeKernelPhaseClass,
        hour: usize,
        hourly: DirectSnowHourlyForcing,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || Self::hourly_symbol("snow.hourly.rain_fraction", hour),
            hourly.rain_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || Self::hourly_symbol("snow.hourly.snow_fraction", hour),
            hourly.snow_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        if let Some(temperature_c) = hourly.hydrometeor_temperature_c {
            Self::require_direct_typed_snow_value_with(
                phase_class,
                || Self::hourly_symbol("snow.hourly.hydrometeor_temperature_c", hour),
                temperature_c,
                None,
                None,
            )?;
        }
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || Self::hourly_symbol("snow.hourly.active_precipitation_m", hour),
            hourly.active_precipitation_m,
            Some(0.0),
            None,
        )?;
        let precipitation_m = hourly.active_precipitation_m;
        let phase_fraction_sum = hourly.rain_fraction + hourly.snow_fraction;
        let expected_phase_fraction_sum = if precipitation_m > 0.0 { 1.0 } else { 0.0 };
        if (phase_fraction_sum - expected_phase_fraction_sum).abs() > 1.0e-12 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::hourly_symbol("snow.hourly.phase_fraction_sum", hour),
                value: phase_fraction_sum,
                minimum: Some(expected_phase_fraction_sum),
                maximum: Some(expected_phase_fraction_sum),
            });
        }
        let precipitation_amount_residual_m =
            (precipitation_m - hourly.rain_m - hourly.snowfall_m * 0.1).abs();
        if precipitation_amount_residual_m > 1.0e-12 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::hourly_symbol("snow.hourly.phase_amount_residual_m", hour),
                value: precipitation_amount_residual_m,
                minimum: Some(0.0),
                maximum: Some(1.0e-12),
            });
        }
        if precipitation_m > 0.0 {
            let maximum_component_residual_m = (hourly.rain_m
                - precipitation_m * hourly.rain_fraction)
                .abs()
                .max(
                    (hourly.snowfall_m * 0.1 - precipitation_m * hourly.snow_fraction).abs(),
                );
            if maximum_component_residual_m > 1.0e-12 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: Self::hourly_symbol("snow.hourly.phase_component_residual_m", hour),
                    value: maximum_component_residual_m,
                    minimum: Some(0.0),
                    maximum: Some(1.0e-12),
                });
            }
        }
        Ok(())
    }

    fn advance_active_snow_settle_clock(
        state: &mut ActiveSnowPackState,
        hour: usize,
        snowfall_m: f64,
    ) {
        if hour == 1 {
            state.settle_day_count += 1.0;
        }
        if snowfall_m > WB11_ZERO_THRESHOLD {
            state.settle_day_count = 1.0;
        }
    }

    fn advance_active_snowpack_for_hour(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        policy: ActiveSnowHourPolicy,
        state: &mut ActiveSnowPackState,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        if state.depth_m <= WB11_ZERO_THRESHOLD {
            Self::reset_or_start_snowpack_from_hourly_snow(state, hourly.snowfall_m, inputs);
            return Ok(false);
        }
        if policy.daily_mean_temp_c < 0.0 {
            Self::advance_cold_snowpack_density(state, hourly.snowfall_m, inputs);
            return Ok(false);
        }

        Self::advance_warm_snowpack_new_snow(state, hourly.snowfall_m, inputs);
        if state.depth_m > WB11_ZERO_THRESHOLD {
            Self::apply_active_snowpack_melt_for_hour(
                phase_class,
                inputs,
                hourly,
                policy.capacity_drainage_opt_in,
                policy.capture,
                state,
                fluxes,
            )?;
            return Ok(true);
        }
        Ok(false)
    }

    fn reset_or_start_snowpack_from_hourly_snow(
        state: &mut ActiveSnowPackState,
        snowfall_m: f64,
        inputs: &DirectActiveSnowPartitionInputs,
    ) {
        if snowfall_m <= WB11_ZERO_THRESHOLD {
            state.depth_m = 0.0;
            state.density_kg_m3 = 0.0;
        } else {
            state.depth_m = snowfall_m;
            state.density_kg_m3 = inputs.newsnw_kg_m3;
        }
    }

    fn advance_cold_snowpack_density(
        state: &mut ActiveSnowPackState,
        snowfall_m: f64,
        inputs: &DirectActiveSnowPartitionInputs,
    ) {
        let mut compacted_depth_m = state.depth_m;
        let mut setf =
            ((-(state.settle_day_count * 2.0)).exp() * SIMIMPL29_SNOWPACK_SETTLE_BASE) + 1.0;
        if state.density_kg_m3 > inputs.ssd_kg_m3 {
            setf = 1.0;
        }
        let mut compacted_density_kg_m3 = state.density_kg_m3 * setf;
        if compacted_density_kg_m3 > SIMIMPL29_SNOW_DENSITY_CAP_KG_M3 {
            compacted_density_kg_m3 = SIMIMPL29_SNOW_DENSITY_CAP_KG_M3;
        }
        if compacted_density_kg_m3 > WB11_ZERO_THRESHOLD {
            compacted_depth_m = compacted_depth_m * state.density_kg_m3 / compacted_density_kg_m3;
        }

        if snowfall_m <= WB11_ZERO_THRESHOLD {
            state.depth_m = compacted_depth_m;
            state.density_kg_m3 = compacted_density_kg_m3;
        } else {
            state.depth_m = compacted_depth_m + snowfall_m;
            state.density_kg_m3 = if state.depth_m > WB11_ZERO_THRESHOLD {
                ((compacted_density_kg_m3 * compacted_depth_m)
                    + (inputs.newsnw_kg_m3 * snowfall_m))
                    / state.depth_m
            } else {
                0.0
            };
        }
    }

    fn advance_warm_snowpack_new_snow(
        state: &mut ActiveSnowPackState,
        snowfall_m: f64,
        inputs: &DirectActiveSnowPartitionInputs,
    ) {
        let depth_before_snowfall_m = state.depth_m;
        if snowfall_m > WB11_ZERO_THRESHOLD {
            state.depth_m += snowfall_m;
            state.density_kg_m3 = if state.depth_m > WB11_ZERO_THRESHOLD {
                ((state.density_kg_m3 * depth_before_snowfall_m)
                    + (inputs.newsnw_kg_m3 * snowfall_m))
                    / state.depth_m
            } else {
                0.0
            };
        }
    }

    fn apply_active_snowpack_melt_for_hour(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        capacity_drainage_opt_in: bool,
        capture: DirectSnowDiagnosticCapture,
        state: &mut ActiveSnowPackState,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let positive_temperature_c_day_increment = hourly.air_temperature_c.max(0.0) / 24.0;
        state.snow_albedo_state_after = Self::update_hourly_opt_in_snow_albedo_state(
            phase_class,
            inputs.snow_melt_model,
            inputs.snow_albedo_model,
            state.snow_albedo_state_after,
            state.depth_m,
            state.density_kg_m3,
            hourly.snowfall_m,
            inputs.newsnw_kg_m3,
            positive_temperature_c_day_increment,
            inputs.underlying_surface_albedo,
        )?;
        let shortwave_absorbed_fraction =
            Self::active_snow_shortwave_absorbed_fraction(phase_class, inputs, state)?;
        let melt = Self::compute_simimpl29_melt_hour_with_capture(
            phase_class,
            inputs.canopy_cover_fraction,
            hourly.radiation_mj_m2,
            hourly.cloud_fraction,
            hourly.air_temperature_c,
            inputs.dewpoint_c,
            inputs.wind_m_s,
            hourly.rain_m,
            state.depth_m,
            state.density_kg_m3,
            shortwave_absorbed_fraction,
            capture,
        )?;
        let wmelt = melt.wmelt_m;
        fluxes.melt_raw_m = wmelt;
        fluxes.melt_diagnostics = melt.diagnostics;
        let smelt = Self::active_snow_melt_depth_m(phase_class, wmelt, state.density_kg_m3)?;
        let depth_after_inputs_m = state.depth_m;
        state.depth_m = depth_after_inputs_m - smelt;
        Self::apply_active_snow_melt_to_pack(
            phase_class,
            inputs,
            hourly.rain_m,
            wmelt,
            smelt,
            depth_after_inputs_m,
            capacity_drainage_opt_in,
            state,
            fluxes,
        )
    }

    fn active_snow_shortwave_absorbed_fraction(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        state: &ActiveSnowPackState,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        match inputs.snow_melt_model {
            SnowMeltModel::LegacyCoe
            | SnowMeltModel::CoeWinterThawStateLossV1
            | SnowMeltModel::CoeLiquidHoldingCapacityV1
            | SnowMeltModel::CoeOpenSublimationStageAV1
            | SnowMeltModel::CoeOpenSublimationStageBV1 => Ok(1.0),
            SnowMeltModel::CoeShortwaveAlbedoV1 => state
                .snow_albedo_state_after
                .ok_or(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("snow_albedo"),
                })
                .map(SnowAlbedoState::shortwave_absorbed_fraction),
        }
    }

    fn active_snow_melt_depth_m(
        phase_class: HillslopeKernelPhaseClass,
        wmelt_m: f64,
        snow_density_kg_m3: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if wmelt_m <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }
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
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_active_snow_melt_to_pack(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly_rain_m: f64,
        wmelt_m: f64,
        smelt_m: f64,
        depth_after_inputs_m: f64,
        capacity_drainage_opt_in: bool,
        state: &mut ActiveSnowPackState,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if state.depth_m <= WB11_ZERO_THRESHOLD {
            return Self::apply_pack_exhausting_melt(
                phase_class,
                depth_after_inputs_m,
                smelt_m,
                capacity_drainage_opt_in,
                state,
                fluxes,
            );
        }
        if state.density_kg_m3 >= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
            return Self::apply_high_density_pack_melt(
                phase_class,
                smelt_m,
                wmelt_m,
                state.density_kg_m3,
                fluxes,
            );
        }
        Self::apply_low_density_pack_melt(
            phase_class,
            inputs,
            hourly_rain_m,
            wmelt_m,
            depth_after_inputs_m,
            capacity_drainage_opt_in,
            state,
            fluxes,
        )
    }

    fn apply_pack_exhausting_melt(
        phase_class: HillslopeKernelPhaseClass,
        depth_after_inputs_m: f64,
        smelt_m: f64,
        capacity_drainage_opt_in: bool,
        state: &mut ActiveSnowPackState,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if smelt_m > 0.0 {
            fluxes.melt_m =
                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                    depth_after_inputs_m,
                    state.density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                        &error,
                    )
                })?;
        }
        if capacity_drainage_opt_in && state.liquid_water_retained_m > WB11_ZERO_THRESHOLD {
            fluxes.melt_m += state.liquid_water_retained_m;
            fluxes.liquid_water_released_m += state.liquid_water_retained_m;
            state.liquid_water_retained_m = 0.0;
        }
        state.depth_m = 0.0;
        state.density_kg_m3 = 0.0;
        Ok(())
    }

    fn apply_high_density_pack_melt(
        phase_class: HillslopeKernelPhaseClass,
        smelt_m: f64,
        wmelt_m: f64,
        snow_density_kg_m3: f64,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if smelt_m > 0.0 {
            fluxes.melt_m =
                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                    smelt_m,
                    snow_density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                        &error,
                    )
                })?;
        } else {
            fluxes.melt_m = wmelt_m.min(0.0);
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_low_density_pack_melt(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly_rain_m: f64,
        wmelt_m: f64,
        depth_after_inputs_m: f64,
        capacity_drainage_opt_in: bool,
        state: &mut ActiveSnowPackState,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let thaw_state_loss_opt_in =
            inputs.snow_melt_model == SnowMeltModel::CoeWinterThawStateLossV1
                && wmelt_m > WB11_ZERO_THRESHOLD;
        let capacity_drainage_melt_opt_in =
            capacity_drainage_opt_in && wmelt_m > WB11_ZERO_THRESHOLD;
        let mut density_after_melt_kg_m3 =
            if thaw_state_loss_opt_in || capacity_drainage_melt_opt_in {
                state.density_kg_m3
            } else {
                state.density_kg_m3 * (depth_after_inputs_m / state.depth_m)
            };

        if density_after_melt_kg_m3 <= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
            if capacity_drainage_melt_opt_in {
                density_after_melt_kg_m3 = Self::apply_capacity_drainage_melt(
                    phase_class,
                    hourly_rain_m,
                    wmelt_m,
                    depth_after_inputs_m,
                    state,
                    fluxes,
                )?;
            } else {
                fluxes.melt_m = if thaw_state_loss_opt_in {
                    wmelt_m
                } else {
                    wmelt_m.min(0.0)
                };
                if hourly_rain_m > WB11_ZERO_THRESHOLD {
                    density_after_melt_kg_m3 = Self::apply_low_density_rain_retention(
                        phase_class,
                        hourly_rain_m,
                        density_after_melt_kg_m3,
                        state,
                        fluxes,
                    )?;
                }
            }
        } else {
            fluxes.melt_m =
                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                    state.depth_m,
                    density_after_melt_kg_m3 - SIMIMPL29_DENSITY_MELT_GATE_KG_M3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                        &error,
                    )
                })?;
            density_after_melt_kg_m3 = SIMIMPL29_DENSITY_MELT_GATE_KG_M3;
        }
        state.density_kg_m3 = density_after_melt_kg_m3;
        Ok(())
    }

    fn apply_capacity_drainage_melt(
        phase_class: HillslopeKernelPhaseClass,
        hourly_rain_m: f64,
        wmelt_m: f64,
        depth_after_inputs_m: f64,
        state: &mut ActiveSnowPackState,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        fluxes.liquid_holding_capacity_m =
            Self::snow_liquid_holding_capacity_m(state.depth_m, state.density_kg_m3);
        let mut available_capacity_m =
            (fluxes.liquid_holding_capacity_m - state.liquid_water_retained_m).max(0.0);
        let retained_melt_m = wmelt_m.min(available_capacity_m);
        let released_melt_m = (wmelt_m - retained_melt_m).max(0.0);
        available_capacity_m -= retained_melt_m;
        fluxes.rain_retained_m = hourly_rain_m.min(available_capacity_m);
        fluxes.rain_released_m = (hourly_rain_m - fluxes.rain_retained_m).max(0.0);
        state.liquid_water_retained_m += retained_melt_m + fluxes.rain_retained_m;
        fluxes.liquid_water_released_m += released_melt_m;
        fluxes.melt_m = released_melt_m;
        let pack_water_after_m =
            openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                depth_after_inputs_m,
                state.density_kg_m3,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                    &error,
                )
            })?
                + fluxes.rain_retained_m
                - released_melt_m;
        openwepp_unit_boundary::conversions::water_depth_meters_to_snow_density_increment(
            pack_water_after_m.max(0.0),
            state.depth_m,
        )
        .map_err(|error| {
            Self::unit_conversion_guard_error(
                phase_class,
                BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
                &error,
            )
        })
    }

    fn apply_low_density_rain_retention(
        phase_class: HillslopeKernelPhaseClass,
        hourly_rain_m: f64,
        mut density_after_melt_kg_m3: f64,
        state: &ActiveSnowPackState,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let density_increment_kg_m3 =
            openwepp_unit_boundary::conversions::water_depth_meters_to_snow_density_increment(
                hourly_rain_m,
                state.depth_m,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
                    &error,
                )
            })?;
        if density_increment_kg_m3
            <= (SIMIMPL29_DENSITY_MELT_GATE_KG_M3 - density_after_melt_kg_m3)
                + WB11_ZERO_THRESHOLD
        {
            fluxes.rain_retained_m = hourly_rain_m;
            density_after_melt_kg_m3 += density_increment_kg_m3;
        } else {
            fluxes.rain_retained_m =
                openwepp_unit_boundary::conversions::snow_depth_meters_to_water_equivalent_meters(
                    state.depth_m,
                    SIMIMPL29_DENSITY_MELT_GATE_KG_M3 - density_after_melt_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
                        &error,
                    )
                })?;
            density_after_melt_kg_m3 = SIMIMPL29_DENSITY_MELT_GATE_KG_M3;
        }
        Ok(density_after_melt_kg_m3)
    }

    fn maybe_update_idle_hourly_snow_albedo(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        future_snowfall_this_day: bool,
        albedo_updated_this_hour: bool,
        state: &mut ActiveSnowPackState,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if albedo_updated_this_hour
            || (state.depth_m <= WB11_ZERO_THRESHOLD
                && hourly.snowfall_m <= WB11_ZERO_THRESHOLD
                && future_snowfall_this_day)
        {
            return Ok(());
        }
        let positive_temperature_c_day_increment = hourly.air_temperature_c.max(0.0) / 24.0;
        state.snow_albedo_state_after = Self::update_hourly_opt_in_snow_albedo_state(
            phase_class,
            inputs.snow_melt_model,
            inputs.snow_albedo_model,
            state.snow_albedo_state_after,
            state.depth_m,
            state.density_kg_m3,
            hourly.snowfall_m,
            inputs.newsnw_kg_m3,
            positive_temperature_c_day_increment,
            inputs.underlying_surface_albedo,
        )?;
        Ok(())
    }

    fn apply_active_snow_sublimation_and_liquid_limits(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        capacity_drainage_opt_in: bool,
        state: &mut ActiveSnowPackState,
        fluxes: &mut ActiveSnowHourlyFluxes,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if state.density_kg_m3 > SIMIMPL29_SNOW_DENSITY_CAP_KG_M3 {
            state.density_kg_m3 = SIMIMPL29_SNOW_DENSITY_CAP_KG_M3;
        }
        if state.depth_m > WB11_ZERO_THRESHOLD {
            fluxes.sublimation_m = Self::active_snow_sublimation_for_hour(
                phase_class,
                inputs,
                hourly,
                state.depth_m,
                state.density_kg_m3,
            )?;
        }
        if state.depth_m <= WB11_ZERO_THRESHOLD {
            state.depth_m = 0.0;
            state.density_kg_m3 = 0.0;
            if capacity_drainage_opt_in && state.liquid_water_retained_m > WB11_ZERO_THRESHOLD {
                fluxes.melt_m += state.liquid_water_retained_m;
                fluxes.liquid_water_released_m += state.liquid_water_retained_m;
                state.liquid_water_retained_m = 0.0;
            }
        } else if capacity_drainage_opt_in {
            fluxes.liquid_holding_capacity_m =
                Self::snow_liquid_holding_capacity_m(state.depth_m, state.density_kg_m3);
            if state.liquid_water_retained_m
                > fluxes.liquid_holding_capacity_m + WB11_ZERO_THRESHOLD
            {
                let excess_m = state.liquid_water_retained_m - fluxes.liquid_holding_capacity_m;
                fluxes.melt_m += excess_m;
                fluxes.liquid_water_released_m += excess_m;
                state.liquid_water_retained_m = fluxes.liquid_holding_capacity_m;
            }
        }
        Ok(())
    }

    fn active_snow_sublimation_for_hour(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        match inputs.snow_melt_model {
            SnowMeltModel::CoeOpenSublimationStageAV1 => Self::coe_open_sublimation_stage_a_hour_m(
                phase_class,
                inputs.canopy_cover_fraction,
                inputs.wind_m_s,
                hourly.air_temperature_c,
                inputs.dewpoint_c,
                snow_depth_m,
            ),
            SnowMeltModel::CoeOpenSublimationStageBV1 => Self::coe_open_sublimation_stage_b_hour_m(
                phase_class,
                inputs.canopy_cover_fraction,
                inputs.wind_m_s,
                hourly.air_temperature_c,
                inputs.dewpoint_c,
                snow_depth_m,
                snow_density_kg_m3,
            ),
            SnowMeltModel::LegacyCoe
            | SnowMeltModel::CoeShortwaveAlbedoV1
            | SnowMeltModel::CoeWinterThawStateLossV1
            | SnowMeltModel::CoeLiquidHoldingCapacityV1 => Ok(0.0),
        }
    }

    fn finalize_active_snow_coupling(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        capacity_drainage_opt_in: bool,
        mut state: ActiveSnowPackState,
        totals: ActiveSnowDailyTotals,
        mut hourly_state: Vec<SnowHourlyState>,
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<SnowCouplingOutcome, Wb11HydrologyKernelGuardError> {
        let mut final_liquid_holding_capacity_m = totals.final_liquid_holding_capacity_m;
        let mut liquid_water_retained_m = state.liquid_water_retained_m;
        if let Some(last_hour) = hourly_state.last() {
            final_liquid_holding_capacity_m = last_hour.liquid_holding_capacity_m;
            liquid_water_retained_m = last_hour.liquid_water_retained_after_m;
        }
        Self::validate_active_snow_hourly_totals(phase_class, &hourly_state, totals)?;

        let raw_melt_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_raw_m)
            .sum::<f64>();
        let wet_compaction_liquid_input_m =
            Self::wet_compaction_liquid_input_m(phase_class, &hourly_state, totals)?;
        let melt_redistribution = Self::redistribute_daily_signed_snowmelt(&mut hourly_state);
        Self::add_released_rain_to_hourly_melt(&mut hourly_state);
        let available_runtime_swe_for_state_loss =
            inputs.runtime_swe_m + totals.accumulation_water_m + totals.rain_retained_m;
        let bounded_state_loss_m = Self::bounded_active_snow_state_loss(
            phase_class,
            available_runtime_swe_for_state_loss,
            melt_redistribution.snowpack_state_loss_m,
        )?;
        let bounded_sublimation_m = Self::bounded_active_snow_sublimation(
            phase_class,
            available_runtime_swe_for_state_loss,
            bounded_state_loss_m,
            totals.sublimation_m,
        )?;
        let runtime_swe_after = Self::active_snow_runtime_swe_after(
            phase_class,
            available_runtime_swe_for_state_loss,
            bounded_state_loss_m,
            bounded_sublimation_m,
        )?;
        if Self::update_active_snow_runtime_geometry(phase_class, runtime_swe_after, &mut state)? {
            liquid_water_retained_m = 0.0;
            final_liquid_holding_capacity_m = 0.0;
        }
        let routed_snowpack_m = if capacity_drainage_opt_in {
            bounded_state_loss_m
        } else {
            melt_redistribution.routed_melt_total_m
        };
        let signed_s = routed_snowpack_m - totals.accumulation_water_m - totals.rain_retained_m;
        let routed_melt_total_m = routed_snowpack_m + totals.rain_released_m;
        Self::validate_active_snow_final_outputs(
            phase_class,
            signed_s,
            routed_melt_total_m,
            runtime_swe_after,
            state.depth_m,
            state.density_kg_m3,
        )?;
        let hourly_routed_melt = Self::build_active_snow_hourly_routed_melt_from_shape(
            phase_class,
            &hourly_state,
            routed_melt_total_m,
        )?;
        let verbose_diagnostics = if capture.is_verbose() {
            let mut hourly_melt = [DirectSnowMeltHourDiagnostics::default(); 24];
            for (index, state) in hourly_state.iter().enumerate() {
                hourly_melt[index] = state.melt_diagnostics.ok_or_else(|| {
                    Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: BoundarySymbol::from("snow.verbose_hourly_melt_diagnostics"),
                    }
                })?;
            }
            Some(Box::new(SnowCouplingVerboseDiagnostics {
                hourly_melt,
                hourly_trace: Self::active_snow_hourly_trace(&hourly_state),
            }))
        } else {
            None
        };

        Ok(SnowCouplingOutcome {
            signed_s,
            accumulation: totals.accumulation_water_m,
            rain_retained: totals.rain_retained_m,
            rain_released: totals.rain_released_m,
            liquid_holding_capacity: final_liquid_holding_capacity_m,
            liquid_water_retained: liquid_water_retained_m,
            liquid_water_released: totals.liquid_water_released_m,
            sublimation: bounded_sublimation_m,
            raw_melt: raw_melt_total_m,
            redistributed_melt: melt_redistribution.routed_melt_total_m,
            wet_compaction_liquid_input_m,
            hourly_routed_melt,
            verbose_diagnostics,
            snowpack_state_loss: bounded_state_loss_m,
            runtime_swe: runtime_swe_after,
            runtime_depth_m: state.depth_m,
            runtime_density_kg_m3: state.density_kg_m3,
            runtime_settle_day_count: state.settle_day_count,
            snow_albedo_state_after: state.snow_albedo_state_after,
        })
    }

    fn wet_compaction_liquid_input_m(
        phase_class: HillslopeKernelPhaseClass,
        hourly_state: &[SnowHourlyState],
        totals: ActiveSnowDailyTotals,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let gross_positive_generated_melt_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_raw_m.max(0.0))
            .sum::<f64>();
        let liquid_input_m = gross_positive_generated_melt_m
            + totals.rain_retained_m
            + totals.rain_released_m;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.wet_compaction_liquid_input_m"),
            liquid_input_m,
            Some(0.0),
            None,
        )?;
        Ok(liquid_input_m)
    }

    fn active_snow_hourly_trace(hourly_state: &[SnowHourlyState]) -> SnowHourlyTrace {
        SnowHourlyTrace {
            liquid_holding_capacity: std::array::from_fn(|index| {
                hourly_state[index].liquid_holding_capacity_m
            }),
            liquid_water_retained_before: std::array::from_fn(|index| {
                hourly_state[index].liquid_water_retained_before_m
            }),
            liquid_water_retained_after: std::array::from_fn(|index| {
                hourly_state[index].liquid_water_retained_after_m
            }),
            liquid_water_released: std::array::from_fn(|index| {
                hourly_state[index].liquid_water_released_m
            }),
            rain_released: std::array::from_fn(|index| hourly_state[index].rain_released_m),
            sublimation: std::array::from_fn(|index| hourly_state[index].sublimation_m),
            pack_depth_before: std::array::from_fn(|index| {
                hourly_state[index].pack_depth_before_m
            }),
            pack_depth_after: std::array::from_fn(|index| {
                hourly_state[index].pack_depth_after_m
            }),
            pack_density_before: std::array::from_fn(|index| {
                hourly_state[index].pack_density_before_kg_m3
            }),
            pack_density_after: std::array::from_fn(|index| {
                hourly_state[index].pack_density_after_kg_m3
            }),
        }
    }

    fn build_active_snow_hourly_routed_melt_from_shape(
        phase_class: HillslopeKernelPhaseClass,
        hourly_state: &[SnowHourlyState],
        routed_melt_total_m: f64,
    ) -> Result<[f64; SIMIMPL29_HOURS_PER_DAY], Wb11HydrologyKernelGuardError> {
        if hourly_state.len() != SIMIMPL29_HOURS_PER_DAY {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("snow.hourly.melt_m.count"),
                value: f64::from(u32::try_from(hourly_state.len()).unwrap_or(u32::MAX)),
                minimum: Some(24.0),
                maximum: Some(24.0),
            });
        }

        let mut hourly_routed_melt = [0.0_f64; SIMIMPL29_HOURS_PER_DAY];
        let mut hourly_total_m = 0.0;
        for (hour, hourly) in hourly_state.iter().enumerate() {
            Self::require_direct_typed_snow_value_with(
                phase_class,
                || BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                hourly.melt_m,
                Some(0.0),
                None,
            )?;
            hourly_routed_melt[hour] = hourly.melt_m;
            hourly_total_m += hourly.melt_m;
            Self::require_direct_typed_snow_value_with(
                phase_class,
                || BoundarySymbol::from("snow.hourly.melt_m.total"),
                hourly_total_m,
                Some(0.0),
                None,
            )?;
        }

        if routed_melt_total_m <= WB11_ZERO_THRESHOLD {
            return Ok([0.0; SIMIMPL29_HOURS_PER_DAY]);
        }
        if hourly_total_m <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                value: hourly_total_m,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        // SC-RUNOFFPART-001#INV-RUNOFFPART-022 owns this allocation:
        // producer hourly melt shape supplies timing, while daily
        // snow.routed_melt_m remains the magnitude authority.
        if (hourly_total_m - routed_melt_total_m).abs() > WB11_ZERO_THRESHOLD {
            let scale = routed_melt_total_m / hourly_total_m;
            Self::require_direct_typed_snow_value_with(
                phase_class,
                || BoundarySymbol::from("snow.hourly_routed_melt_m.scale"),
                scale,
                Some(0.0),
                None,
            )?;
            for hourly_melt_m in &mut hourly_routed_melt {
                *hourly_melt_m *= scale;
            }
        }

        let closed_total_m = hourly_routed_melt.iter().sum::<f64>();
        Self::require_active_snow_total_closure(
            phase_class,
            SNOW_HOURLY_MELT_ROOT,
            closed_total_m,
            routed_melt_total_m,
        )?;
        Ok(hourly_routed_melt)
    }

    fn validate_active_snow_hourly_totals(
        phase_class: HillslopeKernelPhaseClass,
        hourly_state: &[SnowHourlyState],
        totals: ActiveSnowDailyTotals,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let hourly_liquid_released_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.liquid_water_released_m)
            .sum::<f64>();
        let _max_retained_liquid_before_hour_m = hourly_state
            .iter()
            .map(|hourly| hourly.liquid_water_retained_before_m)
            .fold(0.0, f64::max);
        Self::require_active_snow_total_closure(
            phase_class,
            "snow_liquid_water_released_m",
            hourly_liquid_released_total_m,
            totals.liquid_water_released_m,
        )?;
        let hourly_sublimation_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.sublimation_m)
            .sum::<f64>();
        Self::require_active_snow_total_closure(
            phase_class,
            SNOW_HOURLY_SUBLIMATION_ROOT,
            hourly_sublimation_total_m,
            totals.sublimation_m,
        )
    }

    fn require_active_snow_total_closure(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &'static str,
        observed_m: f64,
        expected_m: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if (observed_m - expected_m).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(());
        }
        Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
            phase_class,
            symbol: BoundarySymbol::from(symbol),
            value: observed_m - expected_m,
            minimum: Some(-WB11_ZERO_THRESHOLD),
            maximum: Some(WB11_ZERO_THRESHOLD),
        })
    }

    fn add_released_rain_to_hourly_melt(hourly_state: &mut [SnowHourlyState]) {
        for hourly in hourly_state {
            if hourly.rain_released_m > WB11_ZERO_THRESHOLD {
                hourly.melt_m += hourly.rain_released_m;
            }
        }
    }

    fn bounded_active_snow_state_loss(
        phase_class: HillslopeKernelPhaseClass,
        available_runtime_swe_for_state_loss: f64,
        snowpack_state_loss_m: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if !available_runtime_swe_for_state_loss.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: available_runtime_swe_for_state_loss,
            });
        }
        if !snowpack_state_loss_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: snowpack_state_loss_m,
            });
        }
        if snowpack_state_loss_m
            > available_runtime_swe_for_state_loss
                + SIMIMPL29_SNOWPACK_STATE_LOSS_OVERDRAW_TOLERANCE_M
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: available_runtime_swe_for_state_loss - snowpack_state_loss_m,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(snowpack_state_loss_m.min(available_runtime_swe_for_state_loss))
    }

    fn bounded_active_snow_sublimation(
        phase_class: HillslopeKernelPhaseClass,
        available_runtime_swe_for_state_loss: f64,
        bounded_state_loss_m: f64,
        total_sublimation_m: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow_sublimation"),
            bounded_sublimation_m,
            Some(0.0),
            Some(available_swe_after_state_loss_m),
        )?;
        Ok(bounded_sublimation_m)
    }

    fn active_snow_runtime_swe_after(
        phase_class: HillslopeKernelPhaseClass,
        available_runtime_swe_for_state_loss: f64,
        bounded_state_loss_m: f64,
        bounded_sublimation_m: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let runtime_swe_after_raw =
            available_runtime_swe_for_state_loss - bounded_state_loss_m - bounded_sublimation_m;
        if !runtime_swe_after_raw.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                value: runtime_swe_after_raw,
            });
        }
        Ok(if runtime_swe_after_raw <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            runtime_swe_after_raw
        })
    }

    fn update_active_snow_runtime_geometry(
        phase_class: HillslopeKernelPhaseClass,
        runtime_swe_after: f64,
        state: &mut ActiveSnowPackState,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        if runtime_swe_after <= WB11_ZERO_THRESHOLD {
            state.depth_m = 0.0;
            state.density_kg_m3 = 0.0;
            return Ok(true);
        }
        if state.density_kg_m3 > WB11_ZERO_THRESHOLD {
            state.depth_m =
                openwepp_unit_boundary::conversions::water_equivalent_meters_to_snow_depth_meters(
                    runtime_swe_after,
                    state.density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                        &error,
                    )
                })?;
        }
        Ok(false)
    }

    fn validate_active_snow_final_outputs(
        phase_class: HillslopeKernelPhaseClass,
        signed_s: f64,
        routed_melt_total_m: f64,
        runtime_swe_after: f64,
        runtime_depth_after_m: f64,
        runtime_density_after_kg_m3: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if !signed_s.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_SNOW_COUPLING_S),
                value: signed_s,
                minimum: None,
                maximum: None,
            });
        }
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.routed_melt_m"),
            routed_melt_total_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            runtime_swe_after,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            runtime_depth_after_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            runtime_density_after_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hourly_state(melt_m: f64) -> SnowHourlyState {
        SnowHourlyState {
            rain_released_m: 0.0,
            liquid_holding_capacity_m: 0.0,
            liquid_water_retained_before_m: 0.0,
            liquid_water_retained_after_m: 0.0,
            liquid_water_released_m: 0.0,
            sublimation_m: 0.0,
            melt_raw_m: melt_m,
            melt_m,
            melt_diagnostics: Some(DirectSnowMeltHourDiagnostics {
                coe_melt_applied_m: melt_m,
                ..DirectSnowMeltHourDiagnostics::default()
            }),
            pack_depth_before_m: 0.0,
            pack_depth_after_m: 0.0,
            pack_density_before_kg_m3: 0.0,
            pack_density_after_kg_m3: 0.0,
        }
    }

    #[test]
    fn active_snow_hourly_routed_melt_preserves_shape_and_closes_daily_scalar() {
        let mut hourly = [hourly_state(0.0); SIMIMPL29_HOURS_PER_DAY];
        hourly[5] = hourly_state(0.006);
        hourly[6] = hourly_state(0.002);

        let routed = Wb11HydrologyKernel::build_active_snow_hourly_routed_melt_from_shape(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            &hourly,
            0.004,
        )
        .expect("positive producer melt shape should allocate daily routed scalar");

        assert!((routed[5] - 0.003).abs() <= 1.0e-15);
        assert!((routed[6] - 0.001).abs() <= 1.0e-15);
        assert!((routed.iter().sum::<f64>() - 0.004).abs() <= 1.0e-15);
    }

    #[test]
    fn phase_fraction_closure_distinguishes_dry_and_active_hours() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Wb11HydrologyKernel::validate_active_snow_hour(
            phase_class,
            0,
            DirectSnowHourlyForcing::zero(),
        )
        .expect("a dry hour has zero rain and snow fractions");

        let invalid_dry = DirectSnowHourlyForcing {
            rain_fraction: 1.0,
            ..DirectSnowHourlyForcing::zero()
        };
        assert!(
            Wb11HydrologyKernel::validate_active_snow_hour(phase_class, 1, invalid_dry).is_err(),
            "a dry hour cannot carry an active phase fraction"
        );

        let mismatched_dry_amount = DirectSnowHourlyForcing {
            rain_m: 0.001,
            ..DirectSnowHourlyForcing::zero()
        };
        assert!(
            Wb11HydrologyKernel::validate_active_snow_hour(
                phase_class,
                2,
                mismatched_dry_amount,
            )
            .is_err(),
            "dry phase metadata cannot hide a positive precipitation operand"
        );

        let invalid_active = DirectSnowHourlyForcing {
            active_precipitation_m: 0.001,
            rain_m: 0.001,
            rain_fraction: 0.0,
            snow_fraction: 0.0,
            ..DirectSnowHourlyForcing::zero()
        };
        assert!(
            Wb11HydrologyKernel::validate_active_snow_hour(phase_class, 3, invalid_active)
                .is_err(),
            "active precipitation requires complementary phase fractions"
        );

        let mismatched_active = DirectSnowHourlyForcing {
            active_precipitation_m: 0.001,
            rain_m: 0.001,
            rain_fraction: 0.5,
            snow_fraction: 0.5,
            ..DirectSnowHourlyForcing::zero()
        };
        assert!(
            Wb11HydrologyKernel::validate_active_snow_hour(phase_class, 4, mismatched_active)
                .is_err(),
            "phase fractions must reconstruct the independently carried amounts"
        );
    }
}
