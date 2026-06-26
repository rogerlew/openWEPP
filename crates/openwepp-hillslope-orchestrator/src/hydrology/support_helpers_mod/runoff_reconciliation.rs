#[allow(clippy::wildcard_imports)]
use super::super::*;

fn inactive_direct_winter_frost_partition() -> DirectWinterFrostPartitionOutcome {
    DirectWinterFrostPartitionOutcome {
        active_frost_coupling: false,
        dthaw_after_m: 0.0,
        nft_after: 0.0,
        infcap_frz_m_s: 0.0,
        soil_water_after_frwatc_m: None,
        frwatc_soil_water_before_m: 0.0,
        frwatc_soil_water_after_m: 0.0,
        frwatc_frozen_water_before_m: 0.0,
        frwatc_frozen_water_after_m: 0.0,
        frwatc_freeze_debit_m: 0.0,
        frwatc_thaw_credit_m: 0.0,
        frwatc_net_liquid_delta_m: 0.0,
        frozen_water_after_m: 0.0,
        frost_depth_after_m: 0.0,
        thdp_after_m: 0.0,
        tfrdp_after_m: 0.0,
        tthawd_after_m: 0.0,
        fgthwd_flag_after: 0.0,
        total_fine_layer_count: 0.0,
        conductivity_tilled_w_m_k: 0.0,
        conductivity_untilled_w_m_k: 0.0,
        conductivity_residue_w_m_k: 0.0,
        shadow_total_water_before_m: 0.0,
        shadow_total_water_after_m: 0.0,
        shadow_wb_delta_m: 0.0,
        shadow_frwatc_residual_m: 0.0,
        watpdg_m: 0.0,
        watbtm_m: 0.0,
        layer_projection: Vec::new(),
        layer_shadow_projection: Vec::new(),
        fine_layer_projection: Vec::new(),
    }
}

fn active_direct_winter_frost_partition(
    frost_coupling: &FrostCouplingOutcome,
) -> DirectWinterFrostPartitionOutcome {
    DirectWinterFrostPartitionOutcome {
        active_frost_coupling: true,
        dthaw_after_m: frost_coupling.dthaw,
        nft_after: frost_coupling.nft,
        infcap_frz_m_s: frost_coupling.infcap_frz,
        soil_water_after_frwatc_m: frost_coupling.soil_water_after_frwatc,
        frwatc_soil_water_before_m: frost_coupling.frwatc_soil_water_before,
        frwatc_soil_water_after_m: frost_coupling.frwatc_soil_water_after,
        frwatc_frozen_water_before_m: frost_coupling.frwatc_frozen_water_before,
        frwatc_frozen_water_after_m: frost_coupling.frwatc_frozen_water_after,
        frwatc_freeze_debit_m: frost_coupling.frwatc_freeze_debit,
        frwatc_thaw_credit_m: frost_coupling.frwatc_thaw_credit,
        frwatc_net_liquid_delta_m: frost_coupling.frwatc_net_liquid_delta,
        frozen_water_after_m: frost_coupling.frwatc_frozen_water_after,
        frost_depth_after_m: frost_coupling.frdp_m,
        thdp_after_m: frost_coupling.thdp_m,
        tfrdp_after_m: frost_coupling.tfrdp_m,
        tthawd_after_m: frost_coupling.tthawd_m,
        fgthwd_flag_after: frost_coupling.fgthwd_flag,
        total_fine_layer_count: frost_coupling.total_fine_layer_count,
        conductivity_tilled_w_m_k: frost_coupling.conductivity_tilled_w_m_k,
        conductivity_untilled_w_m_k: frost_coupling.conductivity_untilled_w_m_k,
        conductivity_residue_w_m_k: frost_coupling.conductivity_residue_w_m_k,
        shadow_total_water_before_m: frost_coupling.shadow_total_water_before_m,
        shadow_total_water_after_m: frost_coupling.shadow_total_water_after_m,
        shadow_wb_delta_m: frost_coupling.shadow_wb_delta_m,
        shadow_frwatc_residual_m: frost_coupling.shadow_frwatc_residual_m,
        watpdg_m: frost_coupling.watpdg_m,
        watbtm_m: frost_coupling.watbtm_m,
        layer_projection: frost_coupling
            .layer_topology_state
            .iter()
            .map(|layer| DirectFrostLayerProjection {
                layer_index: layer.layer_index,
                theta_after_m: layer.theta_after_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frzw_m,
            })
            .collect(),
        layer_shadow_projection: frost_coupling
            .shadow_layer_state
            .iter()
            .map(|layer| DirectFrostLayerShadowProjection {
                layer_index: layer.layer_index,
                st_m: layer.st_m,
                soil_water_m: layer.soil_water_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frzw_m,
                soilf_m: layer.soilf_m,
                yst_m: layer.yst_m,
                nwfrzz_m: layer.nwfrzz_m,
            })
            .collect(),
        fine_layer_projection: frost_coupling
            .fine_layer_state
            .iter()
            .map(|fine| DirectFrostFineLayerProjection {
                layer_index: fine.layer_index,
                fine_index: fine.fine_index,
                fgfrst: fine.fgfrst,
                slfsd_m: fine.slfsd_m,
                slsic_m: fine.slsic_m,
                slsw_theta: fine.slsw_theta,
                sltime_s: fine.sltime_s,
            })
            .collect(),
    }
}

impl Wb11HydrologyKernel {
    pub fn compute_direct_winter_frost_partition(
        inputs: &DirectActiveFrostPartitionInputs,
    ) -> Result<DirectWinterFrostPartitionOutcome, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_CONDUCTIVITY,
            inputs.soil_conductivity_m_s,
            Some(0.0),
            None,
        )?;
        if !inputs.controls.wint_red_enabled {
            return Ok(inactive_direct_winter_frost_partition());
        }
        let frost_coupling = Self::compute_active_frost_coupling_from_typed(phase_class, inputs)?;
        Ok(active_direct_winter_frost_partition(&frost_coupling))
    }

    pub fn compute_direct_snow_liquid_partition(
        state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        hyetograph_rainfall_m: f64,
    ) -> Result<DirectSnowLiquidPartition, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_rainfall_m,
            Some(0.0),
            None,
        )?;
        let request = HillslopeKernelRequest::new(
            "direct_snow_liquid_partition",
            HillslopeConsumerAdapter::Runoff,
            state_surface,
            flux_surface,
        );
        let runtime_swe = Self::validate_runtime_snow_state_domains(&request, phase_class)?;
        let active_snow_coupling = if hyetograph_rainfall_m <= WB11_ZERO_THRESHOLD
            && runtime_swe <= WB11_ZERO_THRESHOLD
        {
            false
        } else {
            Self::resolve_active_snow_coupling(&request, phase_class)?
        };
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling(&request, phase_class, hyetograph_rainfall_m)?
        } else {
            SnowCouplingOutcome {
                signed_s: 0.0,
                accumulation: 0.0,
                rain_retained: 0.0,
                rain_released: 0.0,
                raw_melt: 0.0,
                redistributed_melt: 0.0,
                snowpack_state_loss: 0.0,
                runtime_swe,
                runtime_depth_m: Self::optional_state_scalar_for_symbol(
                    &request,
                    phase_class,
                    &BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
                )?
                .unwrap_or(0.0),
                runtime_density_kg_m3: Self::optional_state_scalar_for_symbol(
                    &request,
                    phase_class,
                    &BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
                )?
                .unwrap_or(0.0),
                runtime_settle_day_count: Self::optional_state_scalar_for_symbol(
                    &request,
                    phase_class,
                    &BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL),
                )?
                .unwrap_or(0.0),
                snow_albedo_state_after: None,
                hourly_state: Vec::new(),
            }
        };
        let (routed_melt_m, post_winter_rain_m) =
            Self::resolve_snow_partition_terms(phase_class, hyetograph_rainfall_m, &snow_coupling)?;

        Ok(DirectSnowLiquidPartition {
            active_snow_coupling,
            snow_density_model: SnowDensityModel::LegacyWepp,
            snow_coupling_signed_s_m: snow_coupling.signed_s,
            raw_melt_m: snow_coupling.raw_melt,
            redistributed_melt_m: snow_coupling.redistributed_melt,
            routed_melt_m,
            snowpack_swe_loss_m: snow_coupling.snowpack_state_loss,
            post_winter_rain_m,
            runtime_swe_after_m: snow_coupling.runtime_swe,
            runtime_depth_after_m: snow_coupling.runtime_depth_m,
            runtime_density_after_kg_m3: snow_coupling.runtime_density_kg_m3,
            runtime_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            coe_boundary_depth_after_m: snow_coupling.runtime_depth_m,
            coe_boundary_density_after_kg_m3: snow_coupling.runtime_density_kg_m3,
            coe_boundary_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            density_swe_identity_residual_m: 0.0,
            density_unbounded_swe_residual_m: 0.0,
            snow_albedo_state_after: snow_coupling.snow_albedo_state_after,
        })
    }

    pub fn compute_direct_snow_liquid_partition_from_typed(
        inputs: DirectActiveSnowPartitionInputs,
    ) -> Result<DirectSnowLiquidPartition, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
            inputs.hyetograph_rainfall_m,
            Some(0.0),
            None,
        )?;
        let active_snow_coupling =
            if inputs.hyetograph_rainfall_m <= WB11_ZERO_THRESHOLD
                && inputs.runtime_swe_m <= WB11_ZERO_THRESHOLD
            {
                false
            } else {
                inputs.runtime_swe_m > WB11_ZERO_THRESHOLD
                    || f64::midpoint(inputs.tmax_c, inputs.tmin_c) < 0.0
            };
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling_from_typed(phase_class, &inputs)?
        } else {
            Self::require_direct_typed_snow_value(
                phase_class,
                BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
                inputs.runtime_swe_m,
                Some(0.0),
                None,
            )?;
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
            SnowCouplingOutcome {
                signed_s: 0.0,
                accumulation: 0.0,
                rain_retained: 0.0,
                rain_released: 0.0,
                raw_melt: 0.0,
                redistributed_melt: 0.0,
                snowpack_state_loss: 0.0,
                runtime_swe: inputs.runtime_swe_m,
                runtime_depth_m: inputs.runtime_depth_m,
                runtime_density_kg_m3: inputs.runtime_density_kg_m3,
                runtime_settle_day_count: inputs.runtime_settle_day_count,
                snow_albedo_state_after: inputs.snow_albedo_state,
                hourly_state: Vec::new(),
            }
        };
        let (routed_melt_m, post_winter_rain_m) =
            Self::resolve_snow_partition_terms(phase_class, inputs.hyetograph_rainfall_m, &snow_coupling)?;
        let density_outcome = Self::resolve_typed_snow_density_outcome(
            phase_class,
            &inputs,
            &snow_coupling,
            routed_melt_m,
        )?;

        Ok(DirectSnowLiquidPartition {
            active_snow_coupling,
            snow_density_model: inputs.snow_density_model,
            snow_coupling_signed_s_m: snow_coupling.signed_s,
            raw_melt_m: snow_coupling.raw_melt,
            redistributed_melt_m: snow_coupling.redistributed_melt,
            routed_melt_m,
            snowpack_swe_loss_m: snow_coupling.snowpack_state_loss,
            post_winter_rain_m,
            runtime_swe_after_m: density_outcome.runtime_swe_after_m,
            runtime_depth_after_m: density_outcome.runtime_depth_after_m,
            runtime_density_after_kg_m3: density_outcome.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            coe_boundary_depth_after_m: density_outcome.coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: density_outcome.coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            density_swe_identity_residual_m: density_outcome.max_abs_swe_identity_residual_m,
            density_unbounded_swe_residual_m: density_outcome.max_abs_unbounded_swe_residual_m,
            snow_albedo_state_after: snow_coupling.snow_albedo_state_after,
        })
    }

    fn resolve_typed_snow_density_outcome(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        snow_coupling: &SnowCouplingOutcome,
        routed_melt_m: f64,
    ) -> Result<SnowDensityRuntimeOutcome, Wb11HydrologyKernelGuardError> {
        let mean_air_temperature_c = inputs
            .hourly
            .iter()
            .map(|hour| hour.air_temperature_c)
            .sum::<f64>()
            / 24.0;
        update_snow_density_runtime_state(SnowDensityRuntimeInputs {
            model: inputs.snow_density_model,
            prior_swe_m: inputs.runtime_swe_m,
            prior_depth_m: inputs.runtime_depth_m,
            prior_density_kg_m3: inputs.runtime_density_kg_m3,
            boundary_swe_after_m: snow_coupling.runtime_swe,
            boundary_depth_after_m: snow_coupling.runtime_depth_m,
            boundary_density_after_kg_m3: snow_coupling.runtime_density_kg_m3,
            snow_input_m: snow_coupling.accumulation,
            liquid_for_compaction_m: snow_coupling.snowpack_state_loss + routed_melt_m,
            mean_air_temperature_c,
            runtime_density_cap_kg_m3: SIMIMPL29_SNOW_DENSITY_CAP_KG_M3,
        })
        .map_err(|error| Self::snow_density_guard_error(phase_class, &error))
    }

    fn snow_density_guard_error(
        phase_class: HillslopeKernelPhaseClass,
        error: &SnowDensityError,
    ) -> Wb11HydrologyKernelGuardError {
        match error {
            SnowDensityError::NonFiniteInput { symbol, value } => {
                Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(*symbol),
                    value: *value,
                }
            }
            SnowDensityError::OutOfRangeInput {
                symbol,
                value,
                minimum,
                maximum,
            } => Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(*symbol),
                value: *value,
                minimum: *minimum,
                maximum: *maximum,
            },
        }
    }

    pub(crate) fn require_direct_typed_snow_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if !value.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol,
                value,
            });
        }
        Self::require_dynamic_state_range(phase_class, symbol, value, minimum, maximum)
    }

    pub(crate) fn redistribute_daily_signed_snowmelt(
        hourly_state: &mut [SnowHourlyState],
    ) -> SnowMeltRedistributionOutcome {
        // SNOWSCI-S1: runtime snow storage is single-sourced from the depth/density
        // store, so routed snowpack melt must match the positive water-equivalent
        // loss already applied to that store. Negative raw melt remains available
        // through `melt_raw_m` diagnostics, but it cannot create a second SWE debit.
        let positive_melt_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_m.max(0.0))
            .sum::<f64>();

        if positive_melt_total_m <= WB11_ZERO_THRESHOLD {
            for hourly in hourly_state {
                hourly.melt_m = hourly.melt_m.max(0.0);
            }
            return SnowMeltRedistributionOutcome {
                routed_melt_total_m: positive_melt_total_m,
                snowpack_state_loss_m: positive_melt_total_m,
            };
        }

        for hourly in hourly_state {
            hourly.melt_m = hourly.melt_m.max(0.0);
        }
        SnowMeltRedistributionOutcome {
            routed_melt_total_m: positive_melt_total_m,
            snowpack_state_loss_m: positive_melt_total_m,
        }
    }

    pub(crate) fn compute_canopy_interception_depth(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let cancov = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_CANCOV)?;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_PLANT_CANCOV,
            cancov,
            Some(0.0),
            Some(WB15_CANCOV_MAX),
        )?;

        let lai = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_LAI)?;
        Self::require_state_range(phase_class, WB15_SYMBOL_PLANT_LAI, lai, Some(0.0), None)?;

        let vdmt = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_VDMT)?;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_PLANT_VDMT,
            vdmt,
            Some(0.0),
            None,
        )?;

        if cancov <= WB11_ZERO_THRESHOLD || lai <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }

        let biomass_kg_ha = vdmt * WB15_BIOMASS_TO_KG_HA;
        if !biomass_kg_ha.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB15_SYMBOL_PLANT_VDMT),
                value: biomass_kg_ha,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let interception_biomass_kg_ha = biomass_kg_ha.min(WB15_INTERCEPT_BIOMASS_MAX_KG_HA);
        let potential_interception = cancov
            * ((WB15_INTERCEPT_LINEAR_COEFF * interception_biomass_kg_ha
                - WB15_INTERCEPT_QUADRATIC_COEFF * interception_biomass_kg_ha.powi(2))
                / WB15_INTERCEPT_MM_TO_M);
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            potential_interception,
            Some(0.0),
            None,
        )?;

        let interception =
            Self::normalize_non_negative_within_tolerance(potential_interception.min(hyetograph_rainfall));
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception,
            Some(0.0),
            Some(hyetograph_rainfall),
        )?;
        Ok(interception)
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(crate) fn compute_coupled_infiltration_depth(
        phase_class: HillslopeKernelPhaseClass,
        infiltration_conductivity: f64,
        matric_potential: f64,
        times: &[f64],
        intensities: &[f64],
        rainfall_scale: f64,
        snowmelt_depth_m: f64,
        snowmelt_hourly_state: &[SnowHourlyState],
        irrigation_rate_m_per_s: f64,
        irrigation_duration_s: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow.routed_melt_m"),
            snowmelt_depth_m,
            Some(0.0),
            None,
        )?;

        let mut snowmelt_shape_scale = 0.0_f64;
        if snowmelt_depth_m > WB11_ZERO_THRESHOLD {
            let hourly_melt_total = snowmelt_hourly_state
                .iter()
                .try_fold(0.0_f64, |total, hourly| {
                    if !hourly.melt_m.is_finite() {
                        return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                            phase_class,
                            symbol: Self::hourly_symbol(SNOW_HOURLY_MELT_ROOT, hourly.hour),
                            value: hourly.melt_m,
                        });
                    }
                    if hourly.melt_m < -WB11_ZERO_THRESHOLD {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: Self::hourly_symbol(SNOW_HOURLY_MELT_ROOT, hourly.hour),
                            value: hourly.melt_m,
                            minimum: Some(0.0),
                            maximum: None,
                        });
                    }
                    Ok(total + hourly.melt_m.max(0.0))
                })?;
            if hourly_melt_total <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from("snow.routed_melt_m"),
                    value: hourly_melt_total,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            snowmelt_shape_scale = snowmelt_depth_m / hourly_melt_total;
        }

        let mut breakpoints = Vec::new();
        breakpoints.extend(times.iter().copied().filter(|time| time.is_finite()));
        if snowmelt_depth_m > WB11_ZERO_THRESHOLD {
            for hour in 0..=SIMIMPL29_HOURS_PER_DAY {
                breakpoints.push(Self::diagnostic_count_to_f64(hour) * 3_600.0);
            }
        }
        if irrigation_rate_m_per_s > WB11_ZERO_THRESHOLD {
            breakpoints.push(0.0);
            breakpoints.push(irrigation_duration_s.max(0.0));
        }
        breakpoints.sort_by(f64::total_cmp);
        breakpoints.dedup_by(|left, right| (*left - *right).abs() <= WB11_ZERO_THRESHOLD);

        let mut cumulative_infiltration = 0.0_f64;
        for segment_index in 0..breakpoints.len().saturating_sub(1) {
            let segment_start = breakpoints[segment_index];
            let segment_end = breakpoints[segment_index + 1];
            let interval_duration = segment_end - segment_start;
            if interval_duration <= WB11_ZERO_THRESHOLD {
                continue;
            }

            let mut scaled_rainfall_rate = 0.0_f64;
            for index in 0..times.len().saturating_sub(1) {
                if segment_start >= times[index] - WB11_ZERO_THRESHOLD
                    && segment_end <= times[index + 1] + WB11_ZERO_THRESHOLD
                {
                    scaled_rainfall_rate = intensities[index] * rainfall_scale;
                    break;
                }
            }
            let interval_rainfall = scaled_rainfall_rate * interval_duration;
            if !interval_rainfall.is_finite() || interval_rainfall < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                    value: interval_rainfall,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let mut interval_snowmelt_depth = 0.0_f64;
            if snowmelt_depth_m > WB11_ZERO_THRESHOLD {
                for hourly in snowmelt_hourly_state {
                    let hour_start =
                        Self::diagnostic_count_to_f64(hourly.hour.saturating_sub(1)) * 3_600.0;
                    let hour_end = Self::diagnostic_count_to_f64(hourly.hour) * 3_600.0;
                    let overlap = Self::bounded_interval_overlap_duration(
                        segment_start,
                        segment_end,
                        hour_start,
                        hour_end,
                    );
                    if overlap > WB11_ZERO_THRESHOLD {
                        interval_snowmelt_depth +=
                            hourly.melt_m.max(0.0) * snowmelt_shape_scale * overlap / 3_600.0;
                    }
                }
            }
            if !interval_snowmelt_depth.is_finite()
                || interval_snowmelt_depth < -WB11_ZERO_THRESHOLD
            {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from("snow.routed_melt_m"),
                    value: interval_snowmelt_depth,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let interval_irrigation_duration = Self::interval_overlap_duration(
                segment_start,
                segment_end,
                irrigation_duration_s,
            );
            let interval_irrigation_depth = irrigation_rate_m_per_s * interval_irrigation_duration;
            if !interval_irrigation_depth.is_finite()
                || interval_irrigation_depth < -WB11_ZERO_THRESHOLD
            {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(IRRIG_SYMBOL_DAILY_IRRIGATION),
                    value: interval_irrigation_depth,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let interval_liquid_depth = interval_rainfall
                + interval_snowmelt_depth.max(0.0)
                + interval_irrigation_depth.max(0.0);
            if interval_duration <= WB11_ZERO_THRESHOLD {
                continue;
            }

            let rainfall_rate = interval_liquid_depth / interval_duration;
            if !rainfall_rate.is_finite() || rainfall_rate < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                    value: rainfall_rate,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let interval_infiltration = Self::compute_interval_infiltration_depth(
                phase_class,
                infiltration_conductivity,
                matric_potential,
                cumulative_infiltration,
                rainfall_rate,
                interval_duration,
            )?;
            cumulative_infiltration += interval_infiltration;
        }

        if !cumulative_infiltration.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: cumulative_infiltration,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(cumulative_infiltration)
    }

    pub(crate) fn resolve_wb14_top_two_layer_storage_available(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let nsl_symbol = if request
            .state_surface
            .contains_key(&BoundarySymbol::from("wb11_nsl"))
        {
            BoundarySymbol::from("wb11_nsl")
        } else {
            BoundarySymbol::from("nsl")
        };
        let nsl = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &nsl_symbol,
        )?
        .unwrap_or(2.0);
        Self::require_state_range_for_symbol(
            phase_class,
            &nsl_symbol,
            nsl,
            Some(1.0),
            None,
        )?;

        let inspected_layers = if nsl < 1.5 { 1 } else { 2 };
        let mut available = 0.0_f64;
        let mut saw_layer = false;
        for layer_index in 1..=inspected_layers {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let upper_limit_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let theta = Self::optional_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            let upper_limit =
                Self::optional_state_scalar_for_symbol(request, phase_class, &upper_limit_symbol)?;
            match (theta, upper_limit) {
                (Some(theta), Some(upper_limit)) => {
                    Self::require_state_range_for_symbol(
                        phase_class,
                        &theta_symbol,
                        theta,
                        Some(0.0),
                        None,
                    )?;
                    Self::require_state_range_for_symbol(
                        phase_class,
                        &upper_limit_symbol,
                        upper_limit,
                        Some(0.0),
                        None,
                    )?;
                    saw_layer = true;
                    available += (upper_limit - theta).max(0.0);
                }
                (None, None) if !saw_layer => return Ok(None),
                (None, None) => {}
                (Some(_), None) => {
                    return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: upper_limit_symbol,
                    });
                }
                (None, Some(_)) => {
                    return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: theta_symbol,
                    });
                }
            }
        }

        if !available.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("wb18_perc_ul_agg_0001_0002"),
                value: available,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(Some(available))
    }

    pub(crate) fn apply_wb14_storage_limit_to_infiltration(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        cumulative_infiltration: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(available_storage) =
            Self::resolve_wb14_top_two_layer_storage_available(request, phase_class)?
        else {
            return Ok(cumulative_infiltration);
        };
        Ok(cumulative_infiltration.min(available_storage))
    }

    pub(crate) fn resolve_wb14_producer_published_infiltration(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let lane_substeps_symbol = BoundarySymbol::from("wb18_perc_lane_substeps");
        if let Some(lane_substeps_raw) =
            Self::optional_state_scalar_for_symbol(request, phase_class, &lane_substeps_symbol)?
        {
            Self::require_state_range_for_symbol(
                phase_class,
                &lane_substeps_symbol,
                lane_substeps_raw,
                Some(1.0),
                None,
            )?;
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
            if (lane_substeps - 1.0).abs() > WB11_ZERO_THRESHOLD {
                let same_pass_lineage_symbol =
                    BoundarySymbol::from(WB12_SYMBOL_INFILTRATION_SAME_PASS_LINEAGE);
                let same_pass_lineage = Self::optional_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &same_pass_lineage_symbol,
                )?
                .unwrap_or(0.0);
                Self::require_state_range_for_symbol(
                    phase_class,
                    &same_pass_lineage_symbol,
                    same_pass_lineage,
                    Some(0.0),
                    Some(1.0),
                )?;
                if same_pass_lineage < 0.5 {
                    return Ok(None);
                }
            }
        }

        if !request
            .state_surface
            .contains_key(&BoundarySymbol::from("management.initial.params.tillay2_m"))
        {
            return Ok(None);
        }
        if !request
            .flux_surface
            .contains_key(&BoundarySymbol::from(WB11_SYMBOL_PERC_LOSS_D))
        {
            return Ok(None);
        }

        let infiltration =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_INFILTRATION)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_INFILTRATION,
            infiltration,
            Some(0.0),
            None,
        )?;
        Ok(Some(infiltration))
    }

    pub(crate) fn resolve_interception_rainfall_scale(
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
        interception_rainfall_input: f64,
        interception: f64,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
        let liquid_after_interception_raw = interception_rainfall_input - interception;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            liquid_after_interception_raw,
            Some(0.0),
            Some(interception_rainfall_input),
        )?;
        let liquid_after_interception =
            Self::normalize_non_negative_within_tolerance(liquid_after_interception_raw);

        if hyetograph_rainfall <= WB11_ZERO_THRESHOLD {
            return Ok((liquid_after_interception, 0.0));
        }

        let rainfall_scale = liquid_after_interception / hyetograph_rainfall;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            rainfall_scale,
            Some(0.0),
            None,
        )?;
        Ok((liquid_after_interception, rainfall_scale))
    }

    pub(crate) fn require_infiltration_liquid_closure(
        phase_class: HillslopeKernelPhaseClass,
        cumulative_infiltration: f64,
        liquid_after_interception: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if cumulative_infiltration > liquid_after_interception + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: cumulative_infiltration,
                minimum: Some(0.0),
                maximum: Some(liquid_after_interception),
            });
        }

        Ok(())
    }

    pub(crate) fn require_non_negative_liquid_input(
        phase_class: HillslopeKernelPhaseClass,
        liquid_input: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            liquid_input,
            Some(0.0),
            None,
        )?;
        Ok(())
    }

    pub(crate) fn normalize_non_negative_within_tolerance(value: f64) -> f64 {
        if (-WB11_ZERO_THRESHOLD..0.0).contains(&value) {
            return 0.0;
        }
        value
    }

    pub(crate) fn compute_runoff_after_interception(
        phase_class: HillslopeKernelPhaseClass,
        liquid_after_interception: f64,
        signed_s: f64,
        runon_input: f64,
        cumulative_infiltration: f64,
        depression_storage_delta: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let liquid_input = liquid_after_interception + signed_s;
        Self::require_non_negative_liquid_input(phase_class, liquid_input)?;

        let q_runoff =
            liquid_input + runon_input - cumulative_infiltration - depression_storage_delta;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;
        Ok(Self::normalize_non_negative_within_tolerance(q_runoff))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_storage_reconciled_with_interception(
        phase_class: HillslopeKernelPhaseClass,
        storage_initial: f64,
        precip_input: f64,
        snow_coupling_s: f64,
        irrigation_input: f64,
        runon_input: f64,
        interception: f64,
        q_runoff: f64,
        et: f64,
        percolation_loss: f64,
        subsurface_loss: f64,
        frost_liquid_exchange: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let storage_reconciled =
            storage_initial + precip_input + snow_coupling_s + irrigation_input
                + runon_input
                + frost_liquid_exchange
                - interception
                - q_runoff
                - et
                - percolation_loss
                - subsurface_loss;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_RECONCILED,
            storage_reconciled,
            Some(0.0),
            None,
        )?;
        Ok(storage_reconciled)
    }
}
