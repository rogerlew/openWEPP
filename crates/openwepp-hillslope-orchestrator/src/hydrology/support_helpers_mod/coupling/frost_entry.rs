#[allow(clippy::wildcard_imports)]
use super::super::super::*;

#[allow(clippy::wildcard_imports)]
use super::*;

#[derive(Debug, Clone, Copy)]
struct ActiveFrostControls {
    fine_top_count: usize,
    fine_bot_count: usize,
    ksnowf: f64,
    kresf: f64,
    ksoilf: f64,
    kfactor1: f64,
    kfactor2: f64,
    kfactor3: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostPriorContext {
    profile_depth_m: f64,
    prior_depth_summary: FrostDepthSummary,
    effective_prior_frdp_m: f64,
    prior_nft: f64,
    soil_water: f64,
    prior_ws_frz: f64,
    fgthwd_flag: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostProfileShadowContext {
    profile_depth_m: f64,
    prior_depth_summary: FrostDepthSummary,
    prior_layer_frozen_depth_m: f64,
    prior_layer_frozen_store_m: f64,
    prior_fine_frozen_store_m: f64,
    prior_layer_state_active: bool,
    prior_fine_state_active: bool,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostSurfaceInputs {
    snow_depth_m: f64,
    residue_depth_m: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostThermalContext {
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    ksnowf: f64,
    residue_depth_m: f64,
    conductivity_residue_w_m_k: f64,
    snow_conductivity_w_m_k: f64,
    seasonal_temperature_curve: FrostSeasonalTemperatureCurve,
    sdate: f64,
    kfactor_selected: f64,
}

struct ActiveFrostHourlyContext<'request, 'surface> {
    request: &'request HillslopeKernelRequest<'surface>,
    phase_class: HillslopeKernelPhaseClass,
    layer_water_state: &'request [FrostLayerWaterState],
    profile_depth_m: f64,
    effective_prior_frdp_m: f64,
    thermal: ActiveFrostThermalContext,
    ksoilf: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostFinalScalars {
    dfrost: f64,
    dthaw: f64,
    nft: f64,
    ws_frz: f64,
    infcap_frz: f64,
    soil_water_after_frwatc: Option<f64>,
    frwatc_soil_water_after: f64,
    frwatc_freeze_exchange: f64,
    frwatc_thaw_release: f64,
    frwatc_net_liquid_delta: f64,
    tfrdp_m: f64,
    tthawd_m: f64,
    fgthwd_flag: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostCompletionContext {
    phase_class: HillslopeKernelPhaseClass,
    prior: ActiveFrostPriorContext,
    thermal: ActiveFrostThermalContext,
    soil_conductivity: f64,
    freeze_started: bool,
    fgthwd_flag: f64,
}

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

    fn require_integral_unit_interval_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
        let value = Self::require_state_scalar(request, phase_class, symbol)?;
        Self::require_state_range(phase_class, symbol, value, Some(0.0), Some(1.0))?;
        let rounded = value.round();
        if (value - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        Ok((value, rounded))
    }

    fn require_frost_fine_count_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
        value: f64,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        let rounded = value.round();
        let parsed = format!("{rounded:.0}").parse::<usize>().map_err(|_| {
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value,
                minimum: Some(1.0),
                maximum: Some(10.0),
            }
        })?;
        if !(1..=10).contains(&parsed) {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value,
                minimum: Some(1.0),
                maximum: Some(10.0),
            });
        }
        Ok(parsed)
    }

    fn require_active_frost_fine_counts(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(usize, usize), Wb11HydrologyKernelGuardError> {
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

        Ok((
            Self::require_frost_fine_count_value(
                phase_class,
                WB14_SYMBOL_FROST_FINE_TOP,
                fine_top,
            )?,
            Self::require_frost_fine_count_value(
                phase_class,
                WB14_SYMBOL_FROST_FINE_BOT,
                fine_bot,
            )?,
        ))
    }

    fn require_active_frost_conductivity_controls(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(f64, f64, f64), Wb11HydrologyKernelGuardError> {
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
        Ok((ksnowf, kresf, ksoilf))
    }

    fn require_active_frost_kfactors(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(f64, f64, f64), Wb11HydrologyKernelGuardError> {
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
        Ok((kfactor1, kfactor2, kfactor3))
    }

    fn require_active_frost_controls(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<ActiveFrostControls, Wb11HydrologyKernelGuardError> {
        let (wint_red, wint_rounded) =
            Self::require_integral_unit_interval_state(request, phase_class, WB14_SYMBOL_FROST_WINT_RED)?;
        if wint_rounded < 1.0 - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(1.0),
                maximum: Some(1.0),
            });
        }

        let (fine_top_count, fine_bot_count) =
            Self::require_active_frost_fine_counts(request, phase_class)?;
        let (ksnowf, kresf, ksoilf) =
            Self::require_active_frost_conductivity_controls(request, phase_class)?;
        let (kfactor1, kfactor2, kfactor3) =
            Self::require_active_frost_kfactors(request, phase_class)?;

        Ok(ActiveFrostControls {
            fine_top_count,
            fine_bot_count,
            ksnowf,
            kresf,
            ksoilf,
            kfactor1,
            kfactor2,
            kfactor3,
        })
    }

    fn frost_fine_layer_count_for_layer(
        phase_class: HillslopeKernelPhaseClass,
        dg_symbol: &BoundarySymbol,
        dg_m: f64,
        layer_index: usize,
        layer_count: usize,
        controls: ActiveFrostControls,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        if layer_index != layer_count {
            return Ok(if layer_index < 3 {
                controls.fine_top_count
            } else {
                controls.fine_bot_count
            });
        }

        let spacing_mm = if layer_index > 2 {
            200.0 / Self::diagnostic_count_to_f64(controls.fine_bot_count)
        } else {
            // UNIT-CONVERSION-ALLOW: cm_m_scale percentage allocation, not dimensional conversion.
            100.0 / Self::diagnostic_count_to_f64(controls.fine_top_count)
        };
        let dg_mm =
            openwepp_unit_boundary::conversions::meters_to_millimeters(dg_m).map_err(|error| {
                Self::unit_conversion_guard_error(phase_class, dg_symbol.clone(), &error)
            })?;
        let dg_mm_trunc = dg_mm.trunc();
        let ratio_trunc = (dg_mm / spacing_mm).trunc();
        let mut count = format!("{ratio_trunc:.0}").parse::<usize>().map_err(|_| {
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: dg_symbol.clone(),
                value: ratio_trunc,
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
            }
        })?;
        let count_trunc_mm = (Self::diagnostic_count_to_f64(count) * spacing_mm).trunc();
        if (count_trunc_mm - dg_mm_trunc).abs() > WB11_ZERO_THRESHOLD {
            count += 1;
        }
        Ok(count.max(1))
    }

    fn require_frost_layer_water_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
        layer_count: usize,
        controls: ActiveFrostControls,
    ) -> Result<FrostLayerWaterState, Wb11HydrologyKernelGuardError> {
        let (dg_symbol, dg_m) = Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
        Self::require_state_range_for_symbol(
            phase_class,
            &dg_symbol,
            dg_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;

        let fine_layer_count = Self::frost_fine_layer_count_for_layer(
            phase_class,
            &dg_symbol,
            dg_m,
            layer_index,
            layer_count,
            controls,
        )?;

        let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
        let theta_m = Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
        Self::require_state_range_for_symbol(phase_class, &theta_symbol, theta_m, Some(0.0), None)?;

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
        let frozen_depth_m =
            Self::optional_state_scalar_for_symbol(request, phase_class, &frozen_depth_symbol)?
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

        Ok(FrostLayerWaterState {
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
        })
    }

    fn require_active_frost_layer_water_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        controls: ActiveFrostControls,
    ) -> Result<(usize, Vec<FrostLayerWaterState>), Wb11HydrologyKernelGuardError> {
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
            let layer_state = Self::require_frost_layer_water_state(
                request,
                phase_class,
                layer_index,
                layer_count,
                controls,
            )?;
            total_fine_layer_count += layer_state.fine_layer_count;
            layer_water_state.push(layer_state);
        }

        Ok((total_fine_layer_count, layer_water_state))
    }

    fn require_frost_profile_shadow_context(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_water_state: &[FrostLayerWaterState],
    ) -> Result<(FrostFineShadowState, ActiveFrostProfileShadowContext), Wb11HydrologyKernelGuardError>
    {
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
        let prior_layer_frozen_store_m = Self::frost_layer_soilf_sum(layer_water_state);
        let prior_layer_state_active = prior_layer_frozen_depth_m > WB11_ZERO_THRESHOLD
            || prior_layer_frozen_store_m > WB11_ZERO_THRESHOLD;
        let shadow_fine_state =
            Self::compute_shadow_fine_state(request, phase_class, layer_water_state)?;
        let prior_depth_summary =
            Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let prior_fine_frozen_store_m = shadow_fine_state
            .layer_state
            .iter()
            .map(|layer| layer.soilf_m)
            .sum::<f64>();
        let prior_fine_state_active = prior_depth_summary.frdp > WB11_ZERO_THRESHOLD
            || prior_fine_frozen_store_m > WB11_ZERO_THRESHOLD;

        Ok((
            shadow_fine_state,
            ActiveFrostProfileShadowContext {
                profile_depth_m,
                prior_depth_summary,
                prior_layer_frozen_depth_m,
                prior_layer_frozen_store_m,
                prior_fine_frozen_store_m,
                prior_layer_state_active,
                prior_fine_state_active,
            },
        ))
    }

    fn require_active_frost_surface_inputs(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<ActiveFrostSurfaceInputs, Wb11HydrologyKernelGuardError> {
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

        Ok(ActiveFrostSurfaceInputs {
            snow_depth_m,
            residue_depth_m,
        })
    }

    fn require_optional_profile_bounded_frost_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol_name: &'static str,
        profile_depth_m: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from(symbol_name);
        let value = Self::optional_state_scalar_for_symbol(request, phase_class, &symbol)?
            .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            symbol,
            value,
            Some(0.0),
            Some(profile_depth_m),
        )?;
        Ok(value)
    }

    fn require_effective_prior_frdp_m(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        profile: ActiveFrostProfileShadowContext,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let prior_frdp_m = Self::require_optional_profile_bounded_frost_state(
            request,
            phase_class,
            FROST_RUNTIME_FRDP_M_SYMBOL,
            profile.profile_depth_m,
        )?;
        let effective_prior_frdp_m = if profile.prior_fine_state_active {
            profile.prior_depth_summary.frdp
        } else if profile.prior_layer_state_active {
            profile.prior_layer_frozen_depth_m
        } else {
            prior_frdp_m
        };
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(FROST_RUNTIME_FRDP_M_SYMBOL),
            effective_prior_frdp_m,
            Some(0.0),
            Some(profile.profile_depth_m),
        )?;
        Ok(effective_prior_frdp_m)
    }

    fn require_prior_fgthwd_flag(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let fgthwd_symbol = BoundarySymbol::from(FROST_RUNTIME_FGTHWD_FLAG_SYMBOL);
        let fgthwd_flag =
            Self::optional_state_scalar_for_symbol(request, phase_class, &fgthwd_symbol)?
                .unwrap_or(0.0);
        Self::require_dynamic_state_range(
            phase_class,
            fgthwd_symbol,
            fgthwd_flag,
            Some(0.0),
            Some(1.0),
        )?;
        Ok(fgthwd_flag)
    }

    fn require_prior_frost_nft(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let prior_nft =
            Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_RUNTIME_NFT)?
                .unwrap_or(0.0);
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            prior_nft,
            Some(0.0),
            None,
        )?;
        Ok(prior_nft)
    }

    fn require_active_frost_soil_water(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::require_active_frost_theta_bounds(request, phase_class)?;
        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water,
            Some(0.0),
            None,
        )?;
        Ok(soil_water)
    }

    fn require_prior_frost_ws_frz(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        profile: ActiveFrostProfileShadowContext,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let prior_runtime_ws_frz =
            Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_RUNTIME_WS_FRZ)?
                .unwrap_or(0.0);
        let prior_ws_frz = if profile.prior_layer_state_active {
            profile.prior_layer_frozen_store_m
        } else if profile.prior_fine_state_active {
            profile.prior_fine_frozen_store_m
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
        Ok(prior_ws_frz)
    }

    fn require_active_frost_storage_inputs(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        profile: ActiveFrostProfileShadowContext,
    ) -> Result<ActiveFrostPriorContext, Wb11HydrologyKernelGuardError> {
        let effective_prior_frdp_m =
            Self::require_effective_prior_frdp_m(request, phase_class, profile)?;
        let _prior_thdp_m = Self::require_optional_profile_bounded_frost_state(
            request,
            phase_class,
            FROST_RUNTIME_THDP_M_SYMBOL,
            profile.profile_depth_m,
        )?;
        let _prior_top_frost_depth_m = Self::require_optional_profile_bounded_frost_state(
            request,
            phase_class,
            FROST_RUNTIME_TFRDP_M_SYMBOL,
            profile.profile_depth_m,
        )?;
        let _prior_tthawd_m = Self::require_optional_profile_bounded_frost_state(
            request,
            phase_class,
            FROST_RUNTIME_TTHAWD_M_SYMBOL,
            profile.profile_depth_m,
        )?;

        let fgthwd_flag = Self::require_prior_fgthwd_flag(request, phase_class)?;
        let prior_nft = Self::require_prior_frost_nft(request, phase_class)?;
        let soil_water = Self::require_active_frost_soil_water(request, phase_class)?;
        let prior_ws_frz = Self::require_prior_frost_ws_frz(request, phase_class, profile)?;

        Ok(ActiveFrostPriorContext {
            profile_depth_m: profile.profile_depth_m,
            prior_depth_summary: profile.prior_depth_summary,
            effective_prior_frdp_m,
            prior_nft,
            soil_water,
            prior_ws_frz,
            fgthwd_flag,
        })
    }

    fn require_active_frost_theta_bounds(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
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
        Ok(())
    }

    fn require_active_frost_snow_conductivity(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        controls: ActiveFrostControls,
        surface_inputs: ActiveFrostSurfaceInputs,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
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
        if surface_inputs.snow_depth_m <= SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
            || snow_density_kg_m3 <= 0.0
        {
            return Ok((snow_density_kg_m3, 0.0));
        }

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
        Ok((
            snow_density_kg_m3,
            (base * controls.ksnowf).max(WB11_ZERO_THRESHOLD),
        ))
    }

    fn require_active_frost_thermal_context(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        controls: ActiveFrostControls,
        surface_inputs: ActiveFrostSurfaceInputs,
    ) -> Result<ActiveFrostThermalContext, Wb11HydrologyKernelGuardError> {
        let kfactor_selected = Self::resolve_frozen_soil_kfactor(
            request,
            phase_class,
            controls.kfactor1,
            controls.kfactor2,
            controls.kfactor3,
        )?;
        let conductivity_residue_w_m_k = FROST_RUNTIME_KRES_BASE_W_M_K * controls.kresf;
        let (snow_density_kg_m3, snow_conductivity_w_m_k) =
            Self::require_active_frost_snow_conductivity(request, phase_class, controls, surface_inputs)?;
        let seasonal_temperature_curve =
            Self::require_frost_seasonal_temperature_curve(request, phase_class)?;
        let sdate = Self::require_integral_state_day(request, phase_class)?;

        Ok(ActiveFrostThermalContext {
            snow_depth_m: surface_inputs.snow_depth_m,
            snow_density_kg_m3,
            ksnowf: controls.ksnowf,
            residue_depth_m: surface_inputs.residue_depth_m,
            conductivity_residue_w_m_k,
            snow_conductivity_w_m_k,
            seasonal_temperature_curve,
            sdate,
            kfactor_selected,
        })
    }

    fn frost_branch_matches(actual: f64, expected: f64) -> bool {
        (actual - expected).abs() <= WB11_ZERO_THRESHOLD
    }

    fn apply_active_frost_freeze_step(
        context: &ActiveFrostHourlyContext<'_, '_>,
        shadow_fine_state: &mut FrostFineShadowState,
        hourly: &mut FrostHourlyState,
        surface_temp_c: f64,
        lower_front_heat_w_m2: f64,
        fgthwd_flag: &mut f64,
        freeze_started: &mut bool,
    ) {
        if !Self::frost_branch_matches(hourly.frzflg, 1.0)
            && !Self::frost_branch_matches(hourly.frzflg, 2.0)
        {
            return;
        }

        Self::freeze_fine_front_with_resistance_feedback(
            &mut shadow_fine_state.fine_layers,
            &mut shadow_fine_state.layer_state,
            context.layer_water_state,
            hourly.frzflg,
            surface_temp_c,
            lower_front_heat_w_m2,
            context.thermal.snow_depth_m,
            context.thermal.snow_conductivity_w_m_k,
            context.thermal.residue_depth_m,
            context.thermal.conductivity_residue_w_m_k,
            &mut shadow_fine_state.watbtm_m,
        );
        let depth_after = Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let hourly_frdp_m = depth_after.frdp.min(context.profile_depth_m);
        if hourly_frdp_m > WB11_ZERO_THRESHOLD {
            *fgthwd_flag = 0.0;
            if context.effective_prior_frdp_m <= WB11_ZERO_THRESHOLD {
                *freeze_started = true;
            }
        }
        hourly.tilled_frozen_depth_m = hourly_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
        hourly.untilled_frozen_depth_m =
            (hourly_frdp_m - hourly.tilled_frozen_depth_m).max(0.0);
    }

    fn apply_active_frost_thaw_step(
        context: &ActiveFrostHourlyContext<'_, '_>,
        shadow_fine_state: &mut FrostFineShadowState,
        hourly: &mut FrostHourlyState,
        depth_before: FrostDepthSummary,
        surface_temp_c: f64,
        lower_front_heat_w_m2: f64,
        fgthwd_flag: &mut f64,
    ) {
        if depth_before.frdp <= WB11_ZERO_THRESHOLD {
            return;
        }

        if Self::frost_branch_matches(hourly.frzflg, 2.0)
            && lower_front_heat_w_m2 > WB11_ZERO_THRESHOLD
        {
            Self::thaw_fine_bottom_with_resistance_feedback(
                &mut shadow_fine_state.fine_layers,
                &mut shadow_fine_state.layer_state,
                context.layer_water_state,
                context.thermal.seasonal_temperature_curve,
                context.thermal.sdate,
                context.ksoilf,
                &mut shadow_fine_state.watbtm_m,
            );
        } else if Self::frost_branch_matches(hourly.frzflg, 3.0) {
            let watpdg_m = Self::thaw_fine_top_with_resistance_feedback(
                &mut shadow_fine_state.fine_layers,
                &mut shadow_fine_state.layer_state,
                context.layer_water_state,
                surface_temp_c.max(0.0),
                context.thermal.snow_depth_m,
                context.thermal.snow_conductivity_w_m_k,
                context.thermal.residue_depth_m,
                context.thermal.conductivity_residue_w_m_k,
            );
            shadow_fine_state.watpdg_m += watpdg_m;
            if lower_front_heat_w_m2 > WB11_ZERO_THRESHOLD {
                Self::thaw_fine_bottom_with_resistance_feedback(
                    &mut shadow_fine_state.fine_layers,
                    &mut shadow_fine_state.layer_state,
                    context.layer_water_state,
                    context.thermal.seasonal_temperature_curve,
                    context.thermal.sdate,
                    context.ksoilf,
                    &mut shadow_fine_state.watbtm_m,
                );
            }
        } else if Self::frost_branch_matches(hourly.frzflg, 4.0) {
            Self::thaw_fine_bottom_with_resistance_feedback(
                &mut shadow_fine_state.fine_layers,
                &mut shadow_fine_state.layer_state,
                context.layer_water_state,
                context.thermal.seasonal_temperature_curve,
                context.thermal.sdate,
                context.ksoilf,
                &mut shadow_fine_state.watbtm_m,
            );
        }

        let depth_after = Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let mut hourly_frdp_m = depth_after.frdp.min(context.profile_depth_m);
        *fgthwd_flag = if hourly_frdp_m <= WB11_ZERO_THRESHOLD {
            1.0
        } else {
            0.0
        };
        if *fgthwd_flag > 0.0 {
            hourly_frdp_m = 0.0;
        }
        hourly.tilled_frozen_depth_m = hourly_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
        hourly.untilled_frozen_depth_m =
            (hourly_frdp_m - hourly.tilled_frozen_depth_m).max(0.0);
    }

    fn canonicalize_active_frost_fine_layers(
        shadow_fine_state: &mut FrostFineShadowState,
        layer_water_state: &[FrostLayerWaterState],
    ) {
        for fine in &mut shadow_fine_state.fine_layers {
            let Some(water_layer) = layer_water_state
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            Self::canonicalize_fine_layer_liquid_theta(fine, water_layer);
        }
    }

    fn advance_active_frost_hour(
        context: &ActiveFrostHourlyContext<'_, '_>,
        shadow_fine_state: &mut FrostFineShadowState,
        hourly: &mut FrostHourlyState,
        fgthwd_flag: &mut f64,
        freeze_started: &mut bool,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::reset_fine_layer_hour_timers(&mut shadow_fine_state.fine_layers);
        let depth_before = Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let hourly_frdp_m = depth_before.frdp.min(context.profile_depth_m);
        let surface_temp_c = Self::legacy_tmpadj_surface_temperature_c(
            context.request,
            context.phase_class,
            hourly.hour,
            context.thermal.snow_depth_m,
            context.thermal.snow_density_kg_m3,
            context.thermal.ksnowf,
            context.thermal.residue_depth_m,
            context.thermal.conductivity_residue_w_m_k,
            depth_before,
        )?;

        let (resistance_m2_c_w, _, ksrf_w_m_k) = Self::frost_surface_heat_path(
            depth_before.frdp,
            context.thermal.snow_depth_m,
            context.thermal.snow_conductivity_w_m_k,
            context.thermal.residue_depth_m,
            context.thermal.conductivity_residue_w_m_k,
            surface_temp_c < 0.0,
            Self::shallow_front_minimum_conduction_path_m(&shadow_fine_state.fine_layers),
        );
        let signed_surface_flux_w_m2 = surface_temp_c / resistance_m2_c_w;
        hourly.surface_temp_c = surface_temp_c;
        let lower_front_heat_w_m2 = Self::lower_front_heat_w_m2(
            context.thermal.seasonal_temperature_curve,
            context.thermal.sdate,
            depth_before.frdp,
            &shadow_fine_state.fine_layers,
            context.layer_water_state,
            context.ksoilf,
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

        Self::apply_active_frost_freeze_step(
            context,
            shadow_fine_state,
            hourly,
            surface_temp_c,
            lower_front_heat_w_m2,
            fgthwd_flag,
            freeze_started,
        );
        Self::apply_active_frost_thaw_step(
            context,
            shadow_fine_state,
            hourly,
            depth_before,
            surface_temp_c,
            lower_front_heat_w_m2,
            fgthwd_flag,
        );
        Self::canonicalize_active_frost_fine_layers(shadow_fine_state, context.layer_water_state);
        hourly.ksrf_w_m_k = ksrf_w_m_k.max(WB11_ZERO_THRESHOLD);
        if !Self::frost_branch_matches(hourly.frzflg, 1.0)
            && !Self::frost_branch_matches(hourly.frzflg, 2.0)
            && depth_before.frdp <= WB11_ZERO_THRESHOLD
        {
            hourly.tilled_frozen_depth_m = hourly_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
            hourly.untilled_frozen_depth_m =
                (hourly_frdp_m - hourly.tilled_frozen_depth_m).max(0.0);
        }
        Ok(())
    }

    fn compute_active_frost_hourly_state(
        context: &ActiveFrostHourlyContext<'_, '_>,
        shadow_fine_state: &mut FrostFineShadowState,
        prior_fgthwd_flag: f64,
    ) -> Result<([FrostHourlyState; SIMIMPL29_HOURS_PER_DAY], bool, f64), Wb11HydrologyKernelGuardError>
    {
        let mut freeze_started = false;
        let mut fgthwd_flag = prior_fgthwd_flag;
        let mut hourly_state = std::array::from_fn(|hour_index| FrostHourlyState {
            hour: hour_index + 1,
            frzflg: 0.0,
            qsrf_w_m2: 0.0,
            quf_w_m2: 0.0,
            ksrf_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            surface_temp_c: 0.0,
            snow_depth_m: context.thermal.snow_depth_m,
            residue_depth_m: context.thermal.residue_depth_m,
            tilled_frozen_depth_m: 0.0,
            untilled_frozen_depth_m: 0.0,
        });
        for hourly in &mut hourly_state {
            Self::advance_active_frost_hour(
                context,
                shadow_fine_state,
                hourly,
                &mut fgthwd_flag,
                &mut freeze_started,
            )?;
        }
        Ok((hourly_state, freeze_started, fgthwd_flag))
    }

    fn validate_aggregated_active_frost_layers(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        shadow_fine_state: &mut FrostFineShadowState,
        layer_water_state: &mut [FrostLayerWaterState],
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::aggregate_active_layers_from_fine_state(
            &mut shadow_fine_state.fine_layers,
            &mut shadow_fine_state.layer_state,
            layer_water_state,
        );
        for fine in &shadow_fine_state.fine_layers {
            let Some(water_layer) = layer_water_state
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            Self::require_shadow_fine_state_domains(Some(request), phase_class, fine, water_layer)?;
        }
        Ok(())
    }

    fn resolve_active_frost_soil_water_after(
        context: ActiveFrostCompletionContext,
        shadow_fine_state: &FrostFineShadowState,
        frwatc_freeze_exchange: f64,
        frwatc_thaw_release: f64,
    ) -> Result<(Option<f64>, f64, f64), Wb11HydrologyKernelGuardError> {
        let raw_frwatc_soil_water_after = shadow_fine_state
            .layer_state
            .iter()
            .map(|layer| layer.soil_water_m)
            .sum::<f64>();
        let material_frwatc_exchange = frwatc_freeze_exchange > WB11_ZERO_THRESHOLD
            || frwatc_thaw_release > WB11_ZERO_THRESHOLD
            || shadow_fine_state.watpdg_m > WB11_ZERO_THRESHOLD
            || shadow_fine_state.watbtm_m > WB11_ZERO_THRESHOLD;
        let mut frwatc_net_liquid_delta =
            raw_frwatc_soil_water_after - context.prior.soil_water;
        if !material_frwatc_exchange && frwatc_net_liquid_delta.abs() <= WB11_ZERO_THRESHOLD {
            frwatc_net_liquid_delta = 0.0;
        }
        let raw_frwatc_soil_water_after = if material_frwatc_exchange {
            raw_frwatc_soil_water_after
        } else {
            context.prior.soil_water + frwatc_net_liquid_delta
        };
        if raw_frwatc_soil_water_after < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class: context.phase_class,
                symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                value: raw_frwatc_soil_water_after,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let frwatc_soil_water_after = raw_frwatc_soil_water_after.max(0.0);
        let soil_water_after_frwatc = if material_frwatc_exchange {
            Some(frwatc_soil_water_after)
        } else {
            None
        };
        Ok((
            soil_water_after_frwatc,
            frwatc_soil_water_after,
            frwatc_net_liquid_delta,
        ))
    }

    fn require_active_frost_final_ranges(
        context: ActiveFrostCompletionContext,
        scalars: &ActiveFrostFinalScalars,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DFROST,
            scalars.dfrost,
            Some(0.0),
            Some(context.prior.profile_depth_m),
        )?;
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DTHAW,
            scalars.dthaw,
            Some(0.0),
            Some(context.prior.profile_depth_m),
        )?;
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            scalars.nft,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            scalars.ws_frz,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ,
            scalars.infcap_frz,
            Some(0.0),
            Some(context.soil_conductivity),
        )?;
        Ok(())
    }

    fn compute_active_frost_final_scalars(
        context: ActiveFrostCompletionContext,
        shadow_fine_state: &FrostFineShadowState,
        layer_water_state: &[FrostLayerWaterState],
    ) -> Result<ActiveFrostFinalScalars, Wb11HydrologyKernelGuardError> {
        let final_depth =
            Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let dfrost = final_depth.frdp.min(context.prior.profile_depth_m);
        let thdp_m = final_depth.thdp;
        let tfrdp_m = final_depth.tfrdp;
        let tthawd_m = final_depth.tthawd;
        let bottom_retreat_m = (context.prior.prior_depth_summary.frdp - dfrost).max(0.0);
        let dthaw = thdp_m.max(bottom_retreat_m);
        let nft = if context.freeze_started {
            context.prior.prior_nft + 1.0
        } else {
            context.prior.prior_nft
        };
        let ws_frz = Self::frost_layer_soilf_sum(layer_water_state);
        let raw_frwatc_freeze_exchange =
            if ws_frz > context.prior.prior_ws_frz + WB11_ZERO_THRESHOLD {
                ws_frz - context.prior.prior_ws_frz
            } else {
                0.0
            };
        let frwatc_freeze_exchange =
            if raw_frwatc_freeze_exchange > context.prior.soil_water
                && raw_frwatc_freeze_exchange <= context.prior.soil_water + WB11_ZERO_THRESHOLD
            {
                context.prior.soil_water
            } else {
                raw_frwatc_freeze_exchange
            };
        let frwatc_thaw_release =
            if context.prior.prior_ws_frz > ws_frz + WB11_ZERO_THRESHOLD {
                context.prior.prior_ws_frz - ws_frz
            } else {
                0.0
            };
        if frwatc_freeze_exchange > context.prior.soil_water + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class: context.phase_class,
                symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                value: context.prior.soil_water,
                minimum: Some(frwatc_freeze_exchange),
                maximum: None,
            });
        }

        let (soil_water_after_frwatc, frwatc_soil_water_after, frwatc_net_liquid_delta) =
            Self::resolve_active_frost_soil_water_after(
                context,
                shadow_fine_state,
                frwatc_freeze_exchange,
                frwatc_thaw_release,
            )?;
        let freeze_fraction = (dfrost / FROST_RUNTIME_TILLAGE_DEPTH_M).clamp(0.0, 1.0);
        let infcap_frz = context.soil_conductivity
            * (1.0 - freeze_fraction + freeze_fraction * context.thermal.kfactor_selected);

        let scalars = ActiveFrostFinalScalars {
            dfrost,
            dthaw,
            nft,
            ws_frz,
            infcap_frz,
            soil_water_after_frwatc,
            frwatc_soil_water_after,
            frwatc_freeze_exchange,
            frwatc_thaw_release,
            frwatc_net_liquid_delta,
            tfrdp_m,
            tthawd_m,
            fgthwd_flag: context.fgthwd_flag,
        };
        Self::require_active_frost_final_ranges(context, &scalars)?;
        Ok(scalars)
    }

    fn build_frost_fine_layer_diagnostic_state(
        phase_class: HillslopeKernelPhaseClass,
        layer_count: usize,
        shadow_fine_state: &FrostFineShadowState,
        layer_water_state: &[FrostLayerWaterState],
    ) -> Result<Vec<FrostFineLayerDiagnosticState>, Wb11HydrologyKernelGuardError> {
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
                slsic_m: Self::canonicalize_near_upper_bound(fine.slsic_m, slsic_capacity_m),
                slsw_theta,
                sltime_s: fine.sltime_s,
                slsic_capacity_m,
                slsw_theta_capacity,
            });
        }
        Ok(fine_layer_diagnostic_state)
    }

    fn assemble_active_frost_outcome(
        context: ActiveFrostCompletionContext,
        scalars: ActiveFrostFinalScalars,
        total_fine_layer_count: usize,
        shadow_fine_state: FrostFineShadowState,
        hourly_state: &[FrostHourlyState; SIMIMPL29_HOURS_PER_DAY],
        layer_water_state: Vec<FrostLayerWaterState>,
        fine_layer_diagnostic_state: Vec<FrostFineLayerDiagnosticState>,
    ) -> FrostCouplingOutcome {
        FrostCouplingOutcome {
            dfrost: scalars.dfrost,
            dthaw: scalars.dthaw,
            nft: scalars.nft,
            ws_frz: scalars.ws_frz,
            infcap_frz: scalars.infcap_frz,
            soil_water_after_frwatc: scalars.soil_water_after_frwatc,
            frwatc_soil_water_before: context.prior.soil_water,
            frwatc_soil_water_after: scalars.frwatc_soil_water_after,
            frwatc_frozen_water_before: context.prior.prior_ws_frz,
            frwatc_frozen_water_after: scalars.ws_frz,
            frwatc_freeze_debit: scalars.frwatc_freeze_exchange,
            frwatc_thaw_credit: scalars.frwatc_thaw_release,
            frwatc_net_liquid_delta: scalars.frwatc_net_liquid_delta,
            frdp_m: scalars.dfrost,
            thdp_m: scalars.dthaw,
            tfrdp_m: scalars.tfrdp_m,
            tthawd_m: scalars.tthawd_m,
            profile_depth_m: context.prior.profile_depth_m,
            fgthwd_flag: scalars.fgthwd_flag,
            total_fine_layer_count: Self::diagnostic_count_to_f64(total_fine_layer_count),
            conductivity_tilled_w_m_k: FROST_RUNTIME_KFTILL_W_M_K,
            conductivity_untilled_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            conductivity_residue_w_m_k: context.thermal.conductivity_residue_w_m_k,
            shadow_total_water_before_m: shadow_fine_state.total_water_before_m,
            shadow_total_water_after_m: shadow_fine_state.total_water_after_m,
            shadow_wb_delta_m: shadow_fine_state.wb_delta_m,
            shadow_frwatc_residual_m: shadow_fine_state.residual_m,
            watpdg_m: shadow_fine_state.watpdg_m,
            watbtm_m: shadow_fine_state.watbtm_m,
            hourly_state: *hourly_state,
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
        }
    }

    fn finalize_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        context: ActiveFrostCompletionContext,
        mut shadow_fine_state: FrostFineShadowState,
        hourly_state: &[FrostHourlyState; SIMIMPL29_HOURS_PER_DAY],
        mut layer_water_state: Vec<FrostLayerWaterState>,
        total_fine_layer_count: usize,
    ) -> Result<FrostCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::validate_aggregated_active_frost_layers(
            request,
            context.phase_class,
            &mut shadow_fine_state,
            &mut layer_water_state,
        )?;
        let scalars =
            Self::compute_active_frost_final_scalars(context, &shadow_fine_state, &layer_water_state)?;
        let fine_layer_diagnostic_state = Self::build_frost_fine_layer_diagnostic_state(
            context.phase_class,
            layer_water_state.len(),
            &shadow_fine_state,
            &layer_water_state,
        )?;
        Ok(Self::assemble_active_frost_outcome(
            context,
            scalars,
            total_fine_layer_count,
            shadow_fine_state,
            hourly_state,
            layer_water_state,
            fine_layer_diagnostic_state,
        ))
    }

    pub(crate) fn compute_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<FrostCouplingOutcome, Wb11HydrologyKernelGuardError> {
        let controls = Self::require_active_frost_controls(request, phase_class)?;
        let (total_fine_layer_count, layer_water_state) =
            Self::require_active_frost_layer_water_state(request, phase_class, controls)?;
        let (mut shadow_fine_state, profile_shadow_context) =
            Self::require_frost_profile_shadow_context(request, phase_class, &layer_water_state)?;
        let surface_inputs = Self::require_active_frost_surface_inputs(request, phase_class)?;
        let prior_context = Self::require_active_frost_storage_inputs(
            request,
            phase_class,
            profile_shadow_context,
        )?;
        let thermal_context = Self::require_active_frost_thermal_context(
            request,
            phase_class,
            controls,
            surface_inputs,
        )?;
        let hourly_context = ActiveFrostHourlyContext {
            request,
            phase_class,
            layer_water_state: &layer_water_state,
            profile_depth_m: prior_context.profile_depth_m,
            effective_prior_frdp_m: prior_context.effective_prior_frdp_m,
            thermal: thermal_context,
            ksoilf: controls.ksoilf,
        };
        let (hourly_state, freeze_started, fgthwd_flag) = Self::compute_active_frost_hourly_state(
            &hourly_context,
            &mut shadow_fine_state,
            prior_context.fgthwd_flag,
        )?;
        let completion_context = ActiveFrostCompletionContext {
            phase_class,
            prior: prior_context,
            thermal: thermal_context,
            soil_conductivity,
            freeze_started,
            fgthwd_flag,
        };
        let outcome = Self::finalize_active_frost_coupling(
            request,
            completion_context,
            shadow_fine_state,
            &hourly_state,
            layer_water_state,
            total_fine_layer_count,
        )?;
        Ok(outcome)
    }

}
