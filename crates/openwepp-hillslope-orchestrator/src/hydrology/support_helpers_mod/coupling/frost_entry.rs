#[allow(clippy::wildcard_imports)]
use super::super::super::*;

#[allow(clippy::wildcard_imports)]
use super::*;

impl Wb11HydrologyKernel {
    pub(crate) fn resolve_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        if let Some(value) = request
            .state_surface
            .get(&BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT))
        {
            let scalar = value.as_f64();
            if !scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT),
                    value: scalar,
                });
            }
            if !(-WB11_ZERO_THRESHOLD..=1.0 + WB11_ZERO_THRESHOLD).contains(&scalar) {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT),
                    value: scalar,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            }

            let rounded = scalar.round();
            if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT),
                    value: scalar,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            }
        }

        let Some(wint_red) =
            Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_WINT_RED)?
        else {
            return Ok(false);
        };
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_WINT_RED,
            wint_red,
            Some(0.0),
            Some(1.0),
        )?;
        let wint_rounded = wint_red.round();
        if (wint_red - wint_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        Ok(wint_rounded >= 1.0 - WB11_ZERO_THRESHOLD)
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn compute_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<FrostCouplingOutcome, Wb11HydrologyKernelGuardError> {
        let wint_red =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_WINT_RED)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_WINT_RED,
            wint_red,
            Some(0.0),
            Some(1.0),
        )?;
        let wint_rounded = wint_red.round();
        if (wint_red - wint_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        if wint_rounded < 1.0 - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(1.0),
                maximum: Some(1.0),
            });
        }

        let fine_top =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_FINE_TOP)?;
        let fine_bot =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_FINE_BOT)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_FINE_TOP, fine_top),
            (WB14_SYMBOL_FROST_FINE_BOT, fine_bot),
        ] {
            Self::require_state_range(phase_class, symbol, value, Some(1.0), Some(10.0))?;
            let rounded = value.round();
            if (value - rounded).abs() > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                });
            }
        }

        let ksnowf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KSNOWF)?;
        let kresf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KRESF)?;
        let ksoilf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KSOILF)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_KSNOWF, ksnowf),
            (WB14_SYMBOL_FROST_KRESF, kresf),
            (WB14_SYMBOL_FROST_KSOILF, ksoilf),
        ] {
            Self::require_state_range(phase_class, symbol, value, Some(0.1), Some(10.0))?;
        }

        let kfactor1 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR1)?;
        let kfactor2 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR2)?;
        let kfactor3 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR3)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_KFACTOR1, kfactor1),
            (WB14_SYMBOL_FROST_KFACTOR2, kfactor2),
            (WB14_SYMBOL_FROST_KFACTOR3, kfactor3),
        ] {
            if value <= 0.0 + WB11_ZERO_THRESHOLD || value > 1.0 + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: Some(1.0),
                });
            }
        }

        let fine_top_count = {
            let rounded = fine_top.round();
            let parsed = format!("{rounded:.0}")
                .parse::<usize>()
                .map_err(|_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FINE_TOP),
                    value: fine_top,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                })?;
            if !(1..=10).contains(&parsed) {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FINE_TOP),
                    value: fine_top,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                });
            }
            parsed
        };
        let fine_bot_count = {
            let rounded = fine_bot.round();
            let parsed = format!("{rounded:.0}")
                .parse::<usize>()
                .map_err(|_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FINE_BOT),
                    value: fine_bot,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                })?;
            if !(1..=10).contains(&parsed) {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FINE_BOT),
                    value: fine_bot,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                });
            }
            parsed
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

        let mut layer_water_state = Vec::with_capacity(layer_count);
        let mut total_fine_layer_count = 0usize;
        for layer_index in 1..=layer_count {
            let (dg_symbol, dg_m) =
                Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &dg_symbol,
                dg_m,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            let fine_layer_count = if layer_index == layer_count {
                let spacing_mm = if layer_index > 2 {
                    200.0 / Self::diagnostic_count_to_f64(fine_bot_count)
                } else {
                    // UNIT-CONVERSION-ALLOW: cm_m_scale percentage allocation, not dimensional conversion.
                    100.0 / Self::diagnostic_count_to_f64(fine_top_count)
                };
                let dg_mm =
                    openwepp_unit_boundary::conversions::meters_to_millimeters(dg_m).map_err(
                        |error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                dg_symbol.clone(),
                                &error,
                            )
                        },
                    )?;
                let dg_mm_trunc = dg_mm.trunc();
                let ratio_trunc = (dg_mm / spacing_mm).trunc();
                let mut count = format!("{ratio_trunc:.0}")
                    .parse::<usize>()
                    .map_err(|_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: dg_symbol.clone(),
                        value: ratio_trunc,
                        minimum: Some(0.0),
                        maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
                    })?;
                let count_trunc_mm = (Self::diagnostic_count_to_f64(count) * spacing_mm).trunc();
                if (count_trunc_mm - dg_mm_trunc).abs() > WB11_ZERO_THRESHOLD {
                    count += 1;
                }
                count.max(1)
            } else if layer_index < 3 {
                fine_top_count
            } else {
                fine_bot_count
            };

            total_fine_layer_count += fine_layer_count;
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let theta_m =
                Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                theta_m,
                Some(0.0),
                None,
            )?;

            let upper_limit_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let upper_limit_m =
                Self::require_state_scalar_for_symbol(request, phase_class, &upper_limit_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &upper_limit_symbol,
                upper_limit_m,
                Some(0.0),
                None,
            )?;

            let (thetdr_symbol, thetdr) =
                Self::require_wb19_thetdr_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &thetdr_symbol,
                thetdr,
                Some(0.0),
                Some(1.0),
            )?;

            let (bulk_density_symbol, bulk_density_kg_m3) =
                Self::require_wb19_bulk_density_kg_m3_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &bulk_density_symbol,
                bulk_density_kg_m3,
                Some(WB11_ZERO_THRESHOLD),
                Some(2_650.0),
            )?;
            let frozen_depth_symbol = Self::wb18_perc_state_symbol("frozen_depth", layer_index);
            let frozen_depth_m = Self::optional_state_scalar_for_symbol(
                request,
                phase_class,
                &frozen_depth_symbol,
            )?
            .unwrap_or(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &frozen_depth_symbol,
                frozen_depth_m,
                Some(0.0),
                Some(dg_m),
            )?;

            let frzw_symbol = Self::wb18_perc_state_symbol("frzw", layer_index);
            let frzw_m =
                Self::optional_state_scalar_for_symbol(request, phase_class, &frzw_symbol)?
                    .unwrap_or(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &frzw_symbol,
                frzw_m,
                Some(0.0),
                Some(upper_limit_m),
            )?;

            layer_water_state.push(FrostLayerWaterState {
                layer_index,
                fine_layer_count,
                fine_layer_thickness_m: dg_m / Self::diagnostic_count_to_f64(fine_layer_count),
                dg_m,
                bulk_density_kg_m3,
                thetdr,
                theta_m,
                upper_limit_m,
                frozen_depth_m,
                frzw_m,
            });
        }

        let profile_depth_symbol = BoundarySymbol::from(PL_GROWTH_SOIL_DEPTH_SYMBOL);
        let profile_depth_m =
            Self::require_state_scalar_for_symbol(request, phase_class, &profile_depth_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            profile_depth_symbol,
            profile_depth_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let prior_layer_frozen_depth_m = layer_water_state
            .iter()
            .map(|layer| layer.frozen_depth_m)
            .sum::<f64>();
        let prior_layer_frozen_store_m = Self::frost_layer_soilf_sum(&layer_water_state);
        let prior_layer_state_active = prior_layer_frozen_depth_m > WB11_ZERO_THRESHOLD
            || prior_layer_frozen_store_m > WB11_ZERO_THRESHOLD;
        let mut shadow_fine_state =
            Self::compute_shadow_fine_state(request, phase_class, &layer_water_state)?;
        let prior_depth_summary =
            Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let prior_fine_frozen_store_m = shadow_fine_state
            .layer_state
            .iter()
            .map(|layer| layer.soilf_m)
            .sum::<f64>();
        let prior_fine_state_active = prior_depth_summary.frdp > WB11_ZERO_THRESHOLD
            || prior_fine_frozen_store_m > WB11_ZERO_THRESHOLD;

        let snow_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_SNOW_DEPTH_SYMBOL);
        let snow_depth_m =
            Self::require_state_scalar_for_symbol(request, phase_class, &snow_depth_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            snow_depth_symbol,
            snow_depth_m,
            Some(0.0),
            None,
        )?;

        let residue_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_RESIDUE_DEPTH_SYMBOL);
        let residue_depth_m =
            Self::require_state_scalar_for_symbol(request, phase_class, &residue_depth_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            residue_depth_symbol,
            residue_depth_m,
            Some(0.0),
            None,
        )?;

        let _tmax = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let _tmin = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;

        let frost_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_FRDP_M_SYMBOL);
        let prior_frdp_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &frost_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            frost_depth_symbol.clone(),
            prior_frdp_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;
        let effective_prior_frdp_m = if prior_fine_state_active {
            prior_depth_summary.frdp
        } else if prior_layer_state_active {
            prior_layer_frozen_depth_m
        } else {
            prior_frdp_m
        };
        Self::require_dynamic_state_range(
            phase_class,
            frost_depth_symbol,
            effective_prior_frdp_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;

        let thaw_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_THDP_M_SYMBOL);
        let prior_thdp_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &thaw_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            thaw_depth_symbol.clone(),
            prior_thdp_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;

        let top_frost_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_TFRDP_M_SYMBOL);
        let prior_top_frost_depth_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &top_frost_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            top_frost_depth_symbol.clone(),
            prior_top_frost_depth_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;

        let top_thaw_depth_symbol = BoundarySymbol::from(FROST_RUNTIME_TTHAWD_M_SYMBOL);
        let prior_tthawd_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &top_thaw_depth_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            top_thaw_depth_symbol.clone(),
            prior_tthawd_m,
            Some(0.0),
            Some(profile_depth_m),
        )?;

        let fgthwd_symbol = BoundarySymbol::from(FROST_RUNTIME_FGTHWD_FLAG_SYMBOL);
        let prior_fgthwd_flag = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &fgthwd_symbol,
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            fgthwd_symbol,
            prior_fgthwd_flag,
            Some(0.0),
            Some(1.0),
        )?;

        let prior_nft = Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_RUNTIME_NFT)?
            .unwrap_or(0.0);
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            prior_nft,
            Some(0.0),
            None,
        )?;

        let theta_residual =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_THETA_RESIDUAL)?;
        let theta_field_capacity =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_RESIDUAL,
            theta_residual,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY,
            theta_field_capacity,
            Some(0.0),
            None,
        )?;
        if theta_field_capacity < theta_residual - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY),
                value: theta_field_capacity,
                minimum: Some(theta_residual),
                maximum: None,
            });
        }

        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water,
            Some(0.0),
            None,
        )?;

        let mut fgthwd_flag = prior_fgthwd_flag;

        let prior_runtime_ws_frz =
            Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_RUNTIME_WS_FRZ)?
                .unwrap_or(0.0);
        let prior_ws_frz = if prior_layer_state_active {
            prior_layer_frozen_store_m
        } else if prior_fine_state_active {
            prior_fine_frozen_store_m
        } else {
            prior_runtime_ws_frz
        };
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            prior_ws_frz,
            Some(0.0),
            None,
        )?;

        let kfactor_selected = Self::resolve_frozen_soil_kfactor(
            request,
            phase_class,
            kfactor1,
            kfactor2,
            kfactor3,
        )?;

        let conductivity_residue_w_m_k = FROST_RUNTIME_KRES_BASE_W_M_K * kresf;

        let snow_density_kg_m3 = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
        )?
        .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            snow_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        let snow_conductivity_w_m_k = if snow_depth_m > SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
            && snow_density_kg_m3 > 0.0
        {
            let density_g_cm3 =
                openwepp_unit_boundary::conversions::kilograms_per_cubic_meter_to_grams_per_cubic_centimeter(
                    snow_density_kg_m3,
                )
                .map_err(|error| {
                    Self::unit_conversion_guard_error(
                        phase_class,
                        BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
                        &error,
                    )
                })?;
            let base = if snow_density_kg_m3 < 156.0 {
                0.023 + (0.234 * density_g_cm3)
            } else {
                0.138 - 1.01 * density_g_cm3 + 3.233 * density_g_cm3.powi(2)
            };
            (base * ksnowf).max(WB11_ZERO_THRESHOLD)
        } else {
            0.0
        };
        let seasonal_temperature_curve =
            Self::require_frost_seasonal_temperature_curve(request, phase_class)?;
        let sdate = Self::require_integral_state_day(request, phase_class)?;

        let mut freeze_started = false;
        let mut hourly_state = std::array::from_fn(|hour_index| FrostHourlyState {
            hour: hour_index + 1,
            frzflg: 0.0,
            qsrf_w_m2: 0.0,
            quf_w_m2: 0.0,
            ksrf_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            surface_temp_c: 0.0,
            snow_depth_m,
            residue_depth_m,
            tilled_frozen_depth_m: 0.0,
            untilled_frozen_depth_m: 0.0,
        });
        for hourly in &mut hourly_state {
            Self::reset_fine_layer_hour_timers(&mut shadow_fine_state.fine_layers);
            let depth_before =
                Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
            let mut hourly_frdp_m = depth_before.frdp.min(profile_depth_m);
            let surface_temp_c = Self::legacy_tmpadj_surface_temperature_c(
                request,
                phase_class,
                hourly.hour,
                snow_depth_m,
                snow_density_kg_m3,
                ksnowf,
                residue_depth_m,
                conductivity_residue_w_m_k,
                depth_before,
            )?;

            let (resistance_m2_c_w, _, ksrf_w_m_k) = Self::frost_surface_heat_path(
                depth_before.frdp,
                snow_depth_m,
                snow_conductivity_w_m_k,
                residue_depth_m,
                conductivity_residue_w_m_k,
                surface_temp_c < 0.0,
                Self::shallow_front_minimum_conduction_path_m(&shadow_fine_state.fine_layers),
            );
            let signed_surface_flux_w_m2 = surface_temp_c / resistance_m2_c_w;
            hourly.surface_temp_c = surface_temp_c;
            let lower_front_heat_w_m2 =
                Self::lower_front_heat_w_m2(
                    seasonal_temperature_curve,
                    sdate,
                    depth_before.frdp,
                    &shadow_fine_state.fine_layers,
                    &layer_water_state,
                    ksoilf,
                );
            let signed_net_flux_w_m2 = signed_surface_flux_w_m2 + lower_front_heat_w_m2;
            hourly.qsrf_w_m2 = (-signed_surface_flux_w_m2).max(0.0);
            hourly.quf_w_m2 = lower_front_heat_w_m2;
            hourly.frzflg = Self::select_frost_branch(
                signed_surface_flux_w_m2,
                lower_front_heat_w_m2,
                signed_net_flux_w_m2,
                depth_before,
            );

            if (hourly.frzflg - 1.0).abs() <= WB11_ZERO_THRESHOLD
                || (hourly.frzflg - 2.0).abs() <= WB11_ZERO_THRESHOLD
            {
                Self::freeze_fine_front_with_resistance_feedback(
                    &mut shadow_fine_state.fine_layers,
                    &mut shadow_fine_state.layer_state,
                    &layer_water_state,
                    hourly.frzflg,
                    surface_temp_c,
                    lower_front_heat_w_m2,
                    snow_depth_m,
                    snow_conductivity_w_m_k,
                    residue_depth_m,
                    conductivity_residue_w_m_k,
                    &mut shadow_fine_state.watbtm_m,
                );
                let depth_after =
                    Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
                hourly_frdp_m = depth_after.frdp.min(profile_depth_m);
                if hourly_frdp_m > WB11_ZERO_THRESHOLD {
                    fgthwd_flag = 0.0;
                    if effective_prior_frdp_m <= WB11_ZERO_THRESHOLD {
                        freeze_started = true;
                    }
                }
            }
            if depth_before.frdp > WB11_ZERO_THRESHOLD {
                if (hourly.frzflg - 2.0).abs() <= WB11_ZERO_THRESHOLD
                    && lower_front_heat_w_m2 > WB11_ZERO_THRESHOLD
                {
                    Self::thaw_fine_bottom_with_resistance_feedback(
                        &mut shadow_fine_state.fine_layers,
                        &mut shadow_fine_state.layer_state,
                        &layer_water_state,
                        seasonal_temperature_curve,
                        sdate,
                        ksoilf,
                        &mut shadow_fine_state.watbtm_m,
                    );
                } else if (hourly.frzflg - 3.0).abs() <= WB11_ZERO_THRESHOLD {
                    let watpdg_m = Self::thaw_fine_top_with_resistance_feedback(
                        &mut shadow_fine_state.fine_layers,
                        &mut shadow_fine_state.layer_state,
                        &layer_water_state,
                        surface_temp_c.max(0.0),
                        snow_depth_m,
                        snow_conductivity_w_m_k,
                        residue_depth_m,
                        conductivity_residue_w_m_k,
                    );
                    shadow_fine_state.watpdg_m += watpdg_m;
                    if lower_front_heat_w_m2 > WB11_ZERO_THRESHOLD {
                        Self::thaw_fine_bottom_with_resistance_feedback(
                            &mut shadow_fine_state.fine_layers,
                            &mut shadow_fine_state.layer_state,
                            &layer_water_state,
                            seasonal_temperature_curve,
                            sdate,
                            ksoilf,
                            &mut shadow_fine_state.watbtm_m,
                        );
                    }
                } else if (hourly.frzflg - 4.0).abs() <= WB11_ZERO_THRESHOLD {
                    Self::thaw_fine_bottom_with_resistance_feedback(
                        &mut shadow_fine_state.fine_layers,
                        &mut shadow_fine_state.layer_state,
                        &layer_water_state,
                        seasonal_temperature_curve,
                        sdate,
                        ksoilf,
                        &mut shadow_fine_state.watbtm_m,
                    );
                }

                let depth_after =
                    Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
                hourly_frdp_m = depth_after.frdp.min(profile_depth_m);
                fgthwd_flag = if hourly_frdp_m <= WB11_ZERO_THRESHOLD {
                    1.0
                } else {
                    0.0
                };
                if fgthwd_flag > 0.0 {
                    hourly_frdp_m = 0.0;
                }
            }
            for fine in &mut shadow_fine_state.fine_layers {
                let Some(water_layer) = layer_water_state
                    .iter()
                    .find(|layer| layer.layer_index == fine.layer_index)
                else {
                    continue;
                };
                Self::canonicalize_fine_layer_liquid_theta(fine, water_layer);
            }
            hourly.ksrf_w_m_k = ksrf_w_m_k.max(WB11_ZERO_THRESHOLD);
            hourly.tilled_frozen_depth_m = hourly_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
            hourly.untilled_frozen_depth_m =
                (hourly_frdp_m - hourly.tilled_frozen_depth_m).max(0.0);
        }

        Self::aggregate_active_layers_from_fine_state(
            &mut shadow_fine_state.fine_layers,
            &mut shadow_fine_state.layer_state,
            &mut layer_water_state,
        );
        for fine in &shadow_fine_state.fine_layers {
            let Some(water_layer) = layer_water_state
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            Self::require_shadow_fine_state_domains(phase_class, fine, water_layer)?;
        }
        let final_depth =
            Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let dfrost = final_depth.frdp.min(profile_depth_m);
        let thdp_m = final_depth.thdp;
        let tfrdp_m = final_depth.tfrdp;
        let tthawd_m = final_depth.tthawd;
        let bottom_retreat_m = (prior_depth_summary.frdp - dfrost).max(0.0);
        let dthaw = thdp_m.max(bottom_retreat_m);
        let nft = if freeze_started { prior_nft + 1.0 } else { prior_nft };
        let ws_frz = Self::frost_layer_soilf_sum(&layer_water_state);
        let raw_frwatc_freeze_exchange = if ws_frz > prior_ws_frz + WB11_ZERO_THRESHOLD {
            ws_frz - prior_ws_frz
        } else {
            0.0
        };
        let frwatc_freeze_exchange =
            if raw_frwatc_freeze_exchange > soil_water
                && raw_frwatc_freeze_exchange <= soil_water + WB11_ZERO_THRESHOLD
            {
                soil_water
            } else {
                raw_frwatc_freeze_exchange
            };
        let frwatc_thaw_release = if prior_ws_frz > ws_frz + WB11_ZERO_THRESHOLD {
            prior_ws_frz - ws_frz
        } else {
            0.0
        };
        if frwatc_freeze_exchange > soil_water + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                value: soil_water,
                minimum: Some(frwatc_freeze_exchange),
                maximum: None,
            });
        }
        let raw_frwatc_soil_water_after = shadow_fine_state
            .layer_state
            .iter()
            .map(|layer| layer.soil_water_m)
            .sum::<f64>();
        let frwatc_net_liquid_delta = raw_frwatc_soil_water_after - soil_water;
        let raw_frwatc_soil_water_after = if frwatc_freeze_exchange > WB11_ZERO_THRESHOLD
            || frwatc_thaw_release > WB11_ZERO_THRESHOLD
            || shadow_fine_state.watpdg_m > WB11_ZERO_THRESHOLD
            || shadow_fine_state.watbtm_m > WB11_ZERO_THRESHOLD
        {
            raw_frwatc_soil_water_after
        } else {
            soil_water + frwatc_net_liquid_delta
        };
        if raw_frwatc_soil_water_after < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                value: raw_frwatc_soil_water_after,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let frwatc_soil_water_after = raw_frwatc_soil_water_after.max(0.0);
        let soil_water_after_frwatc = if frwatc_freeze_exchange > WB11_ZERO_THRESHOLD
            || frwatc_thaw_release > WB11_ZERO_THRESHOLD
            || shadow_fine_state.watpdg_m > WB11_ZERO_THRESHOLD
            || shadow_fine_state.watbtm_m > WB11_ZERO_THRESHOLD
        {
            Some(frwatc_soil_water_after)
        } else {
            None
        };
        let freeze_fraction = (dfrost / FROST_RUNTIME_TILLAGE_DEPTH_M).clamp(0.0, 1.0);
        let infcap_frz =
            soil_conductivity * (1.0 - freeze_fraction + freeze_fraction * kfactor_selected);

        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DFROST,
            dfrost,
            Some(0.0),
            Some(profile_depth_m),
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DTHAW,
            dthaw,
            Some(0.0),
            Some(profile_depth_m),
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            nft,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            ws_frz,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ,
            infcap_frz,
            Some(0.0),
            Some(soil_conductivity),
        )?;

        let mut fine_layer_diagnostic_state =
            Vec::with_capacity(shadow_fine_state.fine_layers.len());
        for fine in &shadow_fine_state.fine_layers {
            let Some(water_layer) = layer_water_state
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: Self::frost_fine_layer_symbol(
                        FROST_RUNTIME_FINE_FGFRST_ROOT,
                        fine.layer_index,
                        fine.fine_index,
                    ),
                    value: Self::diagnostic_count_to_f64(fine.layer_index),
                    minimum: Some(1.0),
                    maximum: Some(Self::diagnostic_count_to_f64(layer_count)),
                });
            };
            let slsic_capacity_m = Self::fine_layer_ice_capacity_m(water_layer, fine);
            let slsw_theta_capacity = Self::fine_layer_liquid_theta_capacity(water_layer);
            let slsw_theta = Self::canonicalize_near_upper_bound(
                Self::canonicalize_near_lower_bound(fine.slsw_theta, water_layer.thetdr),
                slsw_theta_capacity,
            );
            fine_layer_diagnostic_state.push(FrostFineLayerDiagnosticState {
                layer_index: fine.layer_index,
                fine_index: fine.fine_index,
                fgfrst: fine.fgfrst,
                slfsd_m: fine.slfsd_m,
                slsic_m: Self::canonicalize_near_upper_bound(
                    fine.slsic_m,
                    slsic_capacity_m,
                ),
                slsw_theta,
                sltime_s: fine.sltime_s,
                slsic_capacity_m,
                slsw_theta_capacity,
            });
        }

        Ok(FrostCouplingOutcome {
            dfrost,
            dthaw,
            nft,
            ws_frz,
            infcap_frz,
            soil_water_after_frwatc,
            frwatc_soil_water_before: soil_water,
            frwatc_soil_water_after,
            frwatc_frozen_water_before: prior_ws_frz,
            frwatc_frozen_water_after: ws_frz,
            frwatc_freeze_debit: frwatc_freeze_exchange,
            frwatc_thaw_credit: frwatc_thaw_release,
            frwatc_net_liquid_delta,
            frdp_m: dfrost,
            thdp_m: dthaw,
            tfrdp_m,
            tthawd_m,
            profile_depth_m,
            fgthwd_flag,
            total_fine_layer_count: Self::diagnostic_count_to_f64(total_fine_layer_count),
            conductivity_tilled_w_m_k: FROST_RUNTIME_KFTILL_W_M_K,
            conductivity_untilled_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            conductivity_residue_w_m_k,
            shadow_total_water_before_m: shadow_fine_state.total_water_before_m,
            shadow_total_water_after_m: shadow_fine_state.total_water_after_m,
            shadow_wb_delta_m: shadow_fine_state.wb_delta_m,
            shadow_frwatc_residual_m: shadow_fine_state.residual_m,
            watpdg_m: shadow_fine_state.watpdg_m,
            watbtm_m: shadow_fine_state.watbtm_m,
            hourly_state,
            layer_topology_state: layer_water_state
                .into_iter()
                .map(|layer| FrostLayerTopologyState {
                    layer_index: layer.layer_index,
                    fine_layer_count: layer.fine_layer_count,
                    fine_layer_thickness_m: layer.fine_layer_thickness_m,
                    dg_m: layer.dg_m,
                    upper_limit_m: layer.upper_limit_m,
                    theta_after_m: layer.theta_m,
                    frozen_depth_m: layer.frozen_depth_m,
                    frzw_m: layer.frzw_m,
                })
                .collect(),
            shadow_layer_state: shadow_fine_state
                .layer_state
                .into_iter()
                .map(|layer| FrostLayerShadowState {
                    layer_index: layer.layer_index,
                    st_m: layer.st_m,
                    soil_water_m: layer.soil_water_m,
                    frozen_depth_m: layer.frozen_m,
                    frzw_m: layer.frzw_m,
                    soilf_m: layer.soilf_m,
                    yst_m: layer.yst_m,
                    nwfrzz_m: layer.nwfrzz_m,
                })
                .collect(),
            fine_layer_state: fine_layer_diagnostic_state,
        })
    }
}
