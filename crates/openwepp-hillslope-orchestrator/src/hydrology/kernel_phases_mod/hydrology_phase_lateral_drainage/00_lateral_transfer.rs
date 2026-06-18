impl Wb11HydrologyKernel {
    pub(crate) fn run_lateral_transfer(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyLateralTransfer;
        let inputs = Self::wb19_lateral_transfer_inputs(request, phase_class)?;
        let lane_config = Self::wb19_lateral_lane_config(request, phase_class)?;
        let layer_state = Self::wb19_lateral_layer_state(request, phase_class, &lane_config)?;
        let result = Self::wb19_run_lateral_substeps(
            request,
            phase_class,
            &inputs,
            &lane_config,
            layer_state,
        )?;
        Self::wb19_lateral_response(request, phase_class, &inputs, &lane_config, &result)
    }

    fn wb19_lateral_transfer_inputs(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Wb19LateralInputs, Wb11HydrologyKernelGuardError> {
        let drainable_storage_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_DRAINABLE_STORAGE)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_storage_legacy,
            Some(0.0),
            None,
        )?;
        let soil_water_before =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_before,
            Some(0.0),
            None,
        )?;

        let recharge_pe =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_PERC_RECHARGE_PE)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_RECHARGE_PE,
            recharge_pe,
            Some(0.0),
            None,
        )?;
        let q_drainage = Self::optional_flux_scalar(request, phase_class, WB11_SYMBOL_DRAINAGE_QDD)?;
        if let Some(value) = q_drainage {
            Self::require_flux_range(
                phase_class,
                WB11_SYMBOL_DRAINAGE_QDD,
                value,
                Some(0.0),
                None,
            )?;
        }

        let avgslp_symbol = BoundarySymbol::from(WB19_SYMBOL_AVG_SLOPE);
        let avgslp = Self::require_state_scalar_for_symbol(request, phase_class, &avgslp_symbol)?;
        Self::require_state_range_for_symbol(phase_class, &avgslp_symbol, avgslp, Some(0.0), None)?;

        let slplen_symbol = BoundarySymbol::from(WB19_SYMBOL_SLOPE_LENGTH);
        let slplen = Self::require_state_scalar_for_symbol(request, phase_class, &slplen_symbol)?;
        if slplen <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: slplen_symbol,
                value: slplen,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let anisotropy_symbol = BoundarySymbol::from(WB19_SYMBOL_LATERAL_ANISOTROPY_RATIO);
        let anisotropy =
            Self::require_state_scalar_for_symbol(request, phase_class, &anisotropy_symbol)?;
        if anisotropy <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: anisotropy_symbol,
                value: anisotropy,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let soldep_symbol = BoundarySymbol::from("solthk");
        let soldep = Self::require_state_scalar_for_symbol(request, phase_class, &soldep_symbol)?;
        if soldep <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: soldep_symbol,
                value: soldep,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        Ok(Wb19LateralInputs {
            soil_water_before,
            q_drainage,
            avgslp_symbol,
            avgslp,
            slplen,
            anisotropy,
            soldep,
        })
    }

    fn wb19_lateral_lane_config(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Wb19LaneConfig, Wb11HydrologyKernelGuardError> {
        let solwpv_mode = Self::wb19_solwpv_mode(request, phase_class)?;
        let solwpv_mode_lt_2006 = solwpv_mode < 2006;
        let mofe_hourly_carry_arrays_enabled =
            Self::resolve_mofe_hourly_carry_arrays_enabled(request, phase_class)?;
        let lane_substeps = Self::wb19_lateral_drain_lane_substeps(request, phase_class)?;
        if mofe_hourly_carry_arrays_enabled && lane_substeps != MOFE_HOURLY_CARRY_ARRAY_COUNT {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB19_SYMBOL_LATERAL_DRAIN_LANE_SUBSTEPS),
                value: Self::diagnostic_count_to_f64(lane_substeps),
                minimum: Some(Self::diagnostic_count_to_f64(
                    MOFE_HOURLY_CARRY_ARRAY_COUNT,
                )),
                maximum: Some(Self::diagnostic_count_to_f64(
                    MOFE_HOURLY_CARRY_ARRAY_COUNT,
                )),
            });
        }
        let lane_substeps_f64 = lane_substeps
            .to_string()
            .parse::<f64>()
            .unwrap_or(f64::INFINITY);
        if !lane_substeps_f64.is_finite() || lane_substeps_f64 <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB19_SYMBOL_LATERAL_DRAIN_LANE_SUBSTEPS),
                value: lane_substeps_f64,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let daily_lateral_lane = lane_substeps == 1 && !mofe_hourly_carry_arrays_enabled;

        Ok(Wb19LaneConfig {
            solwpv_mode,
            solwpv_mode_lt_2006,
            mofe_hourly_carry_arrays_enabled,
            lane_substeps,
            lane_substeps_f64,
            daily_lateral_lane,
        })
    }

    fn wb19_lateral_layer_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        lane_config: &Wb19LaneConfig,
    ) -> Result<Wb19LateralLayerState, Wb11HydrologyKernelGuardError> {
        let (theta, drain_threshold, conductivity, thickness, upper_limit) =
            Self::wb19_load_layer_state(request, phase_class)?;
        let lateral_conductivity = if !lane_config.daily_lateral_lane && lane_config.solwpv_mode >= 7778
        {
            Self::wb19_load_hourly_lateral_conductivity(request, phase_class, theta.len())?
        } else {
            conductivity.clone()
        };
        let lateral_withdrawal_threshold =
            Self::wb19_frozen_adjusted_lateral_thresholds(request, phase_class, &drain_threshold)?;
        let frozen_water = Self::wb19_frozen_water_by_layer(request, phase_class, theta.len())?;
        let top_effective_upper_limit = Self::wb19_lateral_top_effective_upper_limit(
            request,
            phase_class,
            lane_config,
            &upper_limit,
        )?;
        let parameters =
            Self::wb19_lateral_layer_parameters(request, phase_class, theta.len(), &thickness)?;

        Ok(Wb19LateralLayerState {
            theta,
            drain_threshold,
            conductivity,
            thickness,
            upper_limit,
            lateral_conductivity,
            lateral_withdrawal_threshold,
            frozen_water,
            top_effective_upper_limit,
            parameters,
        })
    }

    fn wb19_lateral_top_effective_upper_limit(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        lane_config: &Wb19LaneConfig,
        upper_limit: &[f64],
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        Ok(if lane_config.mofe_hourly_carry_arrays_enabled {
            let top_upper_limit = upper_limit[0];
            let frozen_water_symbol = Self::wb18_perc_state_symbol("frzw", 1);
            let frozen_water =
                Self::optional_state_scalar_for_symbol(request, phase_class, &frozen_water_symbol)?
                    .unwrap_or(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &frozen_water_symbol,
                frozen_water,
                Some(0.0),
                Some(top_upper_limit),
            )?;
            Some(top_upper_limit - frozen_water)
        } else {
            None
        })
    }

    fn wb19_lateral_layer_parameters(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_count: usize,
        thickness: &[f64],
    ) -> Result<Wb19LateralLayerParameters, Wb11HydrologyKernelGuardError> {
        let mut field_capacity_store = Vec::with_capacity(layer_count);
        let mut porosity = Vec::with_capacity(layer_count);
        let mut field_capacity_theta = Vec::with_capacity(layer_count);
        let mut coca = Vec::with_capacity(layer_count);
        for layer_index in 1..=layer_count {
            let (por_symbol, por) =
                Self::require_wb19_por_scalar(request, phase_class, layer_index)?;
            if por <= WB11_ZERO_THRESHOLD || por > 1.0 + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: por_symbol,
                    value: por,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: Some(1.0),
                });
            }
            porosity.push(por);

            let fc_store_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let layer_fc_store =
                Self::require_state_scalar_for_symbol(request, phase_class, &fc_store_symbol)?;

            let (thetfc_symbol, layer_thetfc) =
                Self::require_wb19_thetfc_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &thetfc_symbol,
                layer_thetfc,
                Some(0.0),
                None,
            )?;

            let (thetdr_symbol, layer_thetdr) =
                Self::require_wb19_thetdr_scalar(request, phase_class, layer_index)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &thetdr_symbol,
                layer_thetdr,
                Some(0.0),
                None,
            )?;
            if layer_thetdr > layer_thetfc + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: thetdr_symbol,
                    value: layer_thetdr,
                    minimum: None,
                    maximum: Some(layer_thetfc),
                });
            }

            let layer_dg = thickness[layer_index - 1];
            let expected_fc_store = (layer_thetfc - layer_thetdr) * layer_dg;
            if !expected_fc_store.is_finite()
                || (layer_fc_store - expected_fc_store).abs() > WB11_ZERO_THRESHOLD
            {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fc_store_symbol,
                    value: layer_fc_store,
                    minimum: Some(expected_fc_store),
                    maximum: Some(expected_fc_store),
                });
            }
            field_capacity_store.push(layer_fc_store);
            field_capacity_theta.push(layer_thetfc);

            let (_coca_symbol, layer_coca) =
                Self::require_wb19_coca_scalar(request, phase_class, layer_index)?;
            coca.push(layer_coca);
        }

        Ok(Wb19LateralLayerParameters {
            field_capacity_store,
            porosity,
            field_capacity_theta,
            coca,
        })
    }

    fn wb19_run_lateral_substeps(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19LateralInputs,
        lane_config: &Wb19LaneConfig,
        mut state: Wb19LateralLayerState,
    ) -> Result<Wb19LateralRunResult, Wb11HydrologyKernelGuardError> {
        let mut accumulator =
            Self::wb19_lateral_run_accumulator(lane_config, state.theta.len(), inputs.soldep);
        for substep_index in 0..lane_config.lane_substeps {
            Self::wb19_run_lateral_substep(
                request,
                phase_class,
                inputs,
                lane_config,
                substep_index,
                &mut state,
                &mut accumulator,
            )?;
        }

        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            accumulator.q_lateral,
            Some(0.0),
            Some(accumulator.q_lateral_target_total),
        )?;
        Ok(Self::wb19_finish_lateral_run_result(state, accumulator))
    }

    fn wb19_lateral_run_accumulator(
        lane_config: &Wb19LaneConfig,
        layer_count: usize,
        soldep: f64,
    ) -> Wb19LateralRunAccumulator {
        Wb19LateralRunAccumulator {
            q_lateral: 0.0,
            q_lateral_potential_total: 0.0,
            q_lateral_target_total: 0.0,
            lateral_capacity_tdv_total: 0.0,
            watyld: 0.0,
            fcdep_after: 0.0,
            unsdep_after: soldep,
            lateral_layer_withdrawal: vec![0.0; layer_count],
            lateral_capacity_active_count: vec![0.0; layer_count],
            lateral_conductivity_active_count: vec![0.0; layer_count],
            q_lateral_substeps: Self::wb19_hourly_accumulator_vec(lane_config),
            surface_saturation_substeps: Self::wb19_hourly_accumulator_vec(lane_config),
        }
    }

    fn wb19_hourly_accumulator_vec(lane_config: &Wb19LaneConfig) -> Vec<f64> {
        if lane_config.mofe_hourly_carry_arrays_enabled {
            Vec::with_capacity(MOFE_HOURLY_CARRY_ARRAY_COUNT)
        } else {
            Vec::new()
        }
    }

    fn wb19_run_lateral_substep(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19LateralInputs,
        lane_config: &Wb19LaneConfig,
        substep_index: usize,
        state: &mut Wb19LateralLayerState,
        accumulator: &mut Wb19LateralRunAccumulator,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let active_layers = Self::wb19_lateral_active_layers(phase_class, lane_config, state)?;
        Self::wb19_increment_lateral_active_counts(
            &active_layers,
            &mut accumulator.lateral_capacity_active_count,
            &mut accumulator.lateral_conductivity_active_count,
        );
        let metrics =
            Self::wb19_lateral_substep_metrics(phase_class, lane_config, state, &active_layers)?;
        let q_lateral_potential =
            Self::wb19_lateral_potential(phase_class, inputs, lane_config, &metrics)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral_potential,
            Some(0.0),
            None,
        )?;
        accumulator.q_lateral_potential_total += q_lateral_potential;
        accumulator.lateral_capacity_tdv_total += metrics.lateral_capacity_tdv;

        let available_pool =
            Self::wb19_drainable_storage(&state.theta, &state.lateral_withdrawal_threshold);
        let q_lateral_target = q_lateral_potential
            .min(available_pool)
            .min(metrics.lateral_capacity_tdv);
        let q_lateral_substep = Self::wb19_withdraw_top_down(
            &mut state.theta,
            &state.lateral_withdrawal_threshold,
            q_lateral_target,
            &mut accumulator.lateral_layer_withdrawal,
        );
        accumulator.q_lateral_target_total += q_lateral_target;
        accumulator.q_lateral += q_lateral_substep;
        if lane_config.mofe_hourly_carry_arrays_enabled {
            accumulator.q_lateral_substeps.push(q_lateral_substep);
        }
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral_substep,
            Some(0.0),
            Some(q_lateral_target),
        )?;
        Self::wb19_record_lateral_surface_saturation(
            request,
            phase_class,
            substep_index,
            state,
            &mut accumulator.surface_saturation_substeps,
        )?;
        let depths = Self::wb19_lateral_water_yield_and_depths(
            phase_class,
            inputs.soldep,
            lane_config,
            &metrics,
            q_lateral_substep,
        )?;
        accumulator.watyld = depths.watyld;
        accumulator.fcdep_after = depths.fcdep_after;
        accumulator.unsdep_after = depths.unsdep_after;
        Ok(())
    }

    fn wb19_finish_lateral_run_result(
        state: Wb19LateralLayerState,
        accumulator: Wb19LateralRunAccumulator,
    ) -> Wb19LateralRunResult {
        Wb19LateralRunResult {
            theta: state.theta,
            lateral_withdrawal_threshold: state.lateral_withdrawal_threshold,
            q_lateral: accumulator.q_lateral,
            q_lateral_potential_total: accumulator.q_lateral_potential_total,
            q_lateral_target_total: accumulator.q_lateral_target_total,
            lateral_capacity_tdv_total: accumulator.lateral_capacity_tdv_total,
            watyld: accumulator.watyld,
            fcdep_after: accumulator.fcdep_after,
            unsdep_after: accumulator.unsdep_after,
            lateral_layer_withdrawal: accumulator.lateral_layer_withdrawal,
            lateral_capacity_active_count: accumulator.lateral_capacity_active_count,
            lateral_conductivity_active_count: accumulator.lateral_conductivity_active_count,
            q_lateral_substeps: accumulator.q_lateral_substeps,
            surface_saturation_substeps: accumulator.surface_saturation_substeps,
        }
    }

    fn wb19_lateral_active_layers(
        phase_class: HillslopeKernelPhaseClass,
        lane_config: &Wb19LaneConfig,
        state: &Wb19LateralLayerState,
    ) -> Result<Wb19LateralActiveLayers, Wb11HydrologyKernelGuardError> {
        if lane_config.daily_lateral_lane {
            return Ok(Self::wb19_daily_lateral_active_layers(
                lane_config,
                &state.theta,
                &state.lateral_withdrawal_threshold,
            ));
        }
        let mut capacity_active_layer = vec![false; state.theta.len()];
        let mut conductivity_active_layer = vec![false; state.theta.len()];
        for (index, theta_i) in state.theta.iter().enumerate() {
            let meblfc = Self::wb19_lateral_lower_layer_saturated(
                phase_class,
                index,
                &state.theta,
                &state.upper_limit,
            )?;
            capacity_active_layer[index] = *theta_i + WB11_ZERO_THRESHOLD
                >= state.lateral_withdrawal_threshold[index]
                && meblfc;
            conductivity_active_layer[index] =
                *theta_i + WB11_ZERO_THRESHOLD >= state.drain_threshold[index] && meblfc;
        }
        Ok(Wb19LateralActiveLayers {
            capacity_active_layer,
            conductivity_active_layer,
        })
    }

    fn wb19_daily_lateral_active_layers(
        lane_config: &Wb19LaneConfig,
        theta: &[f64],
        lateral_withdrawal_threshold: &[f64],
    ) -> Wb19LateralActiveLayers {
        let mut capacity_active_layer = vec![false; theta.len()];
        let mut conductivity_active_layer = vec![false; theta.len()];
        let mut daily_top_contiguous_block_open = true;
        for (index, theta_i) in theta.iter().enumerate() {
            let daily_layer_active =
                *theta_i + WB11_ZERO_THRESHOLD >= lateral_withdrawal_threshold[index];
            let active = if lane_config.solwpv_mode_lt_2006 {
                let top_contiguous_active = daily_top_contiguous_block_open && daily_layer_active;
                if !daily_layer_active {
                    daily_top_contiguous_block_open = false;
                }
                top_contiguous_active
            } else {
                daily_layer_active
            };
            capacity_active_layer[index] = active;
            conductivity_active_layer[index] = active;
        }
        Wb19LateralActiveLayers {
            capacity_active_layer,
            conductivity_active_layer,
        }
    }

    fn wb19_lateral_lower_layer_saturated(
        phase_class: HillslopeKernelPhaseClass,
        index: usize,
        theta: &[f64],
        upper_limit: &[f64],
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        if index + 1 == theta.len() {
            return Ok(true);
        }
        let lower_upper_limit = upper_limit[index + 1];
        if lower_upper_limit <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::wb18_perc_state_symbol("ul", index + 2),
                value: lower_upper_limit,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(theta[index + 1] / lower_upper_limit >= 1.0 - WB11_ZERO_THRESHOLD)
    }

    fn wb19_increment_lateral_active_counts(
        active_layers: &Wb19LateralActiveLayers,
        lateral_capacity_active_count: &mut [f64],
        lateral_conductivity_active_count: &mut [f64],
    ) {
        for (index, is_capacity_active) in active_layers.capacity_active_layer.iter().enumerate() {
            if *is_capacity_active {
                lateral_capacity_active_count[index] += 1.0;
            }
        }
        for (index, is_conductivity_active) in active_layers
            .conductivity_active_layer
            .iter()
            .enumerate()
        {
            if *is_conductivity_active {
                lateral_conductivity_active_count[index] += 1.0;
            }
        }
    }

    fn wb19_lateral_substep_metrics(
        phase_class: HillslopeKernelPhaseClass,
        lane_config: &Wb19LaneConfig,
        state: &Wb19LateralLayerState,
        active_layers: &Wb19LateralActiveLayers,
    ) -> Result<Wb19LateralSubstepMetrics, Wb11HydrologyKernelGuardError> {
        let fcdep_before = Self::wb19_lateral_fcdep_before(
            &active_layers.capacity_active_layer,
            &state.thickness,
        );
        let metrics = Wb19LateralSubstepMetrics {
            fcdep_before,
            legacy_saturation_fraction: 1.0,
            ..Wb19LateralSubstepMetrics::default()
        };
        if fcdep_before <= WB11_ZERO_THRESHOLD {
            return Ok(metrics);
        }
        if lane_config.daily_lateral_lane {
            Self::wb19_daily_lateral_substep_metrics(
                phase_class,
                lane_config,
                state,
                active_layers,
                metrics,
            )
        } else {
            Self::wb19_hourly_lateral_substep_metrics(
                phase_class,
                state,
                active_layers,
                metrics,
            )
        }
    }

    fn wb19_lateral_fcdep_before(capacity_active_layer: &[bool], thickness: &[f64]) -> f64 {
        capacity_active_layer
            .iter()
            .zip(thickness.iter())
            .filter_map(|(is_capacity_active, dg_i)| is_capacity_active.then_some(*dg_i))
            .sum()
    }

    fn wb19_daily_lateral_substep_metrics(
        phase_class: HillslopeKernelPhaseClass,
        lane_config: &Wb19LaneConfig,
        state: &Wb19LateralLayerState,
        active_layers: &Wb19LateralActiveLayers,
        metrics: Wb19LateralSubstepMetrics,
    ) -> Result<Wb19LateralSubstepMetrics, Wb11HydrologyKernelGuardError> {
        if lane_config.solwpv_mode_lt_2006 {
            Self::wb19_legacy_daily_lateral_metrics(phase_class, state, active_layers, metrics)
        } else {
            Self::wb19_modern_daily_lateral_metrics(phase_class, state, active_layers, metrics)
        }
    }

    fn wb19_legacy_daily_lateral_metrics(
        phase_class: HillslopeKernelPhaseClass,
        state: &Wb19LateralLayerState,
        active_layers: &Wb19LateralActiveLayers,
        mut metrics: Wb19LateralSubstepMetrics,
    ) -> Result<Wb19LateralSubstepMetrics, Wb11HydrologyKernelGuardError> {
        let mut daily_average_storage = 0.0_f64;
        let mut daily_average_upper_limit = 0.0_f64;
        let mut daily_average_hk = 0.0_f64;
        for layer_index in 0..state.theta.len() {
            Self::wb19_add_lateral_capacity_tdv(state, active_layers, &mut metrics, layer_index);
            if !active_layers.conductivity_active_layer[layer_index] {
                continue;
            }
            let layer_hk = Self::wb19_lateral_layer_hk(phase_class, state, layer_index)?;
            let layer_weight = state.thickness[layer_index] / metrics.fcdep_before;
            Self::wb19_add_lateral_average_terms(state, &mut metrics, layer_index, layer_weight);
            metrics.conductivity_depth_sum +=
                state.conductivity[layer_index] * state.thickness[layer_index];
            let effective_upper_limit =
                (state.upper_limit[layer_index] - state.frozen_water[layer_index]).max(0.0);
            daily_average_storage += state.theta[layer_index] * layer_weight;
            daily_average_upper_limit += effective_upper_limit * layer_weight;
            daily_average_hk += layer_hk * layer_weight;
        }
        Self::wb19_legacy_daily_saturation_fraction(
            phase_class,
            &mut metrics,
            daily_average_storage,
            daily_average_upper_limit,
            daily_average_hk,
        )?;
        Ok(metrics)
    }

    fn wb19_modern_daily_lateral_metrics(
        phase_class: HillslopeKernelPhaseClass,
        state: &Wb19LateralLayerState,
        active_layers: &Wb19LateralActiveLayers,
        mut metrics: Wb19LateralSubstepMetrics,
    ) -> Result<Wb19LateralSubstepMetrics, Wb11HydrologyKernelGuardError> {
        for layer_index in 0..state.theta.len() {
            Self::wb19_add_lateral_capacity_tdv(state, active_layers, &mut metrics, layer_index);
            if !active_layers.conductivity_active_layer[layer_index] {
                continue;
            }
            let layer_hk = Self::wb19_lateral_layer_hk(phase_class, state, layer_index)?;
            let layer_weight = state.thickness[layer_index] / metrics.fcdep_before;
            Self::wb19_add_lateral_average_terms(state, &mut metrics, layer_index, layer_weight);
            let effective_upper_limit =
                state.upper_limit[layer_index] - state.frozen_water[layer_index];
            let saturation_fraction =
                Self::wb19_modern_daily_saturation_fraction(phase_class, state, layer_index, effective_upper_limit)?;
            let conductivity_fraction =
                Self::wb19_daily_conductivity_fraction(phase_class, layer_index, saturation_fraction, layer_hk)?;
            metrics.conductivity_depth_sum += state.conductivity[layer_index]
                * conductivity_fraction
                * state.thickness[layer_index];
        }
        Ok(metrics)
    }

    fn wb19_hourly_lateral_substep_metrics(
        phase_class: HillslopeKernelPhaseClass,
        state: &Wb19LateralLayerState,
        active_layers: &Wb19LateralActiveLayers,
        mut metrics: Wb19LateralSubstepMetrics,
    ) -> Result<Wb19LateralSubstepMetrics, Wb11HydrologyKernelGuardError> {
        for layer_index in 0..state.theta.len() {
            Self::wb19_add_lateral_capacity_tdv(state, active_layers, &mut metrics, layer_index);
            if !active_layers.conductivity_active_layer[layer_index] {
                continue;
            }
            let saturation_fraction =
                Self::wb19_hourly_lateral_saturation_fraction(phase_class, state, layer_index)?;
            metrics.legacy_saturation_fraction = saturation_fraction;
            let layer_weight = state.thickness[layer_index] / metrics.fcdep_before;
            Self::wb19_add_lateral_average_terms(state, &mut metrics, layer_index, layer_weight);
            metrics.conductivity_depth_sum += state.lateral_conductivity[layer_index]
                * saturation_fraction
                * state.thickness[layer_index];
        }
        Ok(metrics)
    }

    fn wb19_add_lateral_capacity_tdv(
        state: &Wb19LateralLayerState,
        active_layers: &Wb19LateralActiveLayers,
        metrics: &mut Wb19LateralSubstepMetrics,
        layer_index: usize,
    ) {
        if active_layers.capacity_active_layer[layer_index] {
            metrics.lateral_capacity_tdv += (state.theta[layer_index]
                - state.lateral_withdrawal_threshold[layer_index])
                .max(0.0);
        }
    }

    fn wb19_add_lateral_average_terms(
        state: &Wb19LateralLayerState,
        metrics: &mut Wb19LateralSubstepMetrics,
        layer_index: usize,
        layer_weight: f64,
    ) {
        metrics.saturated_depth_sum += state.thickness[layer_index];
        metrics.avpora += state.parameters.porosity[layer_index] * layer_weight;
        metrics.avfca += state.parameters.field_capacity_theta[layer_index] * layer_weight;
        metrics.avcoca += state.parameters.coca[layer_index] * layer_weight;
    }

    fn wb19_lateral_layer_hk(
        phase_class: HillslopeKernelPhaseClass,
        state: &Wb19LateralLayerState,
        layer_index: usize,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let fc_upper_ratio =
            state.parameters.field_capacity_store[layer_index] / state.upper_limit[layer_index];
        if fc_upper_ratio <= 0.0 {
            return Ok(0.0);
        }
        let computed_hk = -2.655 / fc_upper_ratio.log10();
        if !computed_hk.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::wb18_perc_state_symbol("fc", layer_index + 1),
                value: fc_upper_ratio,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(computed_hk)
    }

    fn wb19_legacy_daily_saturation_fraction(
        phase_class: HillslopeKernelPhaseClass,
        metrics: &mut Wb19LateralSubstepMetrics,
        daily_average_storage: f64,
        daily_average_upper_limit: f64,
        daily_average_hk: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        // UNIT-CONVERSION-ALLOW: mm_m_scale legacy soil-water averaging threshold in meters, not conversion.
        if daily_average_upper_limit > 0.001 {
            let saturation_fraction = daily_average_storage / daily_average_upper_limit;
            metrics.legacy_saturation_fraction = if saturation_fraction < 0.95 {
                saturation_fraction.powf(daily_average_hk).max(0.002)
            } else {
                1.0
            };
            if !metrics.legacy_saturation_fraction.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: Self::wb18_perc_state_symbol("theta", 1),
                    value: metrics.legacy_saturation_fraction,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
        }
        Ok(())
    }

    fn wb19_modern_daily_saturation_fraction(
        phase_class: HillslopeKernelPhaseClass,
        state: &Wb19LateralLayerState,
        layer_index: usize,
        effective_upper_limit: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let saturation_fraction = if effective_upper_limit > 0.0 {
            state.theta[layer_index] / effective_upper_limit
        } else {
            1.0
        };
        if !saturation_fraction.is_finite() || saturation_fraction < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::wb18_perc_state_symbol("theta", layer_index + 1),
                value: saturation_fraction,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(saturation_fraction)
    }

    fn wb19_daily_conductivity_fraction(
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
        saturation_fraction: f64,
        layer_hk: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let conductivity_fraction = if saturation_fraction < 0.95 {
            saturation_fraction.powf(layer_hk).max(0.002)
        } else {
            1.0
        };
        if !conductivity_fraction.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::wb18_perc_state_symbol("theta", layer_index + 1),
                value: conductivity_fraction,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(conductivity_fraction)
    }

    fn wb19_hourly_lateral_saturation_fraction(
        phase_class: HillslopeKernelPhaseClass,
        state: &Wb19LateralLayerState,
        layer_index: usize,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let storage_excess = (state.theta[layer_index] - state.drain_threshold[layer_index]).max(0.0);
        let saturation_denominator =
            state.upper_limit[layer_index] - state.drain_threshold[layer_index];
        if saturation_denominator <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::wb18_perc_state_symbol("ul", layer_index + 1),
                value: state.upper_limit[layer_index],
                minimum: Some(state.drain_threshold[layer_index] + WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let saturation_fraction = (storage_excess / saturation_denominator).clamp(0.0, 1.0);
        if !saturation_fraction.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::wb18_perc_state_symbol("theta", layer_index + 1),
                value: saturation_fraction,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        Ok(saturation_fraction)
    }

    fn wb19_lateral_potential(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19LateralInputs,
        lane_config: &Wb19LaneConfig,
        metrics: &Wb19LateralSubstepMetrics,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if metrics.fcdep_before <= WB11_ZERO_THRESHOLD
            || metrics.saturated_depth_sum <= WB11_ZERO_THRESHOLD
        {
            return Ok(0.0);
        }
        let mut ke = (86_400.0 / lane_config.lane_substeps_f64)
            * (metrics.conductivity_depth_sum / metrics.saturated_depth_sum);
        if lane_config.solwpv_mode_lt_2006 {
            ke *= metrics.legacy_saturation_fraction;
        }
        if !ke.is_finite() || ke < 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: inputs.avgslp_symbol.clone(),
                value: ke,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let slope_angle = inputs.avgslp.atan();
        let slope_factor = slope_angle.sin();
        if !slope_factor.is_finite() || slope_factor < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: inputs.avgslp_symbol.clone(),
                value: slope_factor,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        Ok((metrics.fcdep_before * inputs.anisotropy * ke * slope_factor.max(0.0)) / inputs.slplen)
    }

    fn wb19_record_lateral_surface_saturation(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        substep_index: usize,
        state: &mut Wb19LateralLayerState,
        surface_saturation_substeps: &mut Vec<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(top_limit) = state.top_effective_upper_limit {
            let saturation_excess = state.theta[0] - top_limit;
            let current_saturation_runoff = if saturation_excess > WB11_ZERO_THRESHOLD {
                state.theta[0] = top_limit;
                saturation_excess
            } else {
                0.0
            };
            Self::require_state_range_for_symbol(
                phase_class,
                &Self::hourly_symbol_for_request(
                    request,
                    MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
                    substep_index + 1,
                ),
                current_saturation_runoff,
                Some(0.0),
                None,
            )?;
            surface_saturation_substeps.push(Self::normalize_non_negative_within_tolerance(
                current_saturation_runoff,
            ));
        }
        Ok(())
    }

    fn wb19_lateral_water_yield_and_depths(
        phase_class: HillslopeKernelPhaseClass,
        soldep: f64,
        lane_config: &Wb19LaneConfig,
        metrics: &Wb19LateralSubstepMetrics,
        q_lateral_substep: f64,
    ) -> Result<Wb19LateralDepths, Wb11HydrologyKernelGuardError> {
        let watyld = Self::wb19_lateral_watyld(phase_class, metrics)?;
        let mut fcdep_after = metrics.fcdep_before;
        if lane_config.solwpv_mode_lt_2006 && metrics.fcdep_before > WB11_ZERO_THRESHOLD {
            if q_lateral_substep > WB11_ZERO_THRESHOLD && watyld <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB19_SYMBOL_WATER_YIELD_WATYLD),
                    value: watyld,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            fcdep_after = (metrics.fcdep_before - (q_lateral_substep / watyld)).max(0.0);
        }
        Ok(Wb19LateralDepths {
            watyld,
            fcdep_after,
            unsdep_after: (soldep - fcdep_after).max(0.0),
        })
    }

    fn wb19_lateral_watyld(
        phase_class: HillslopeKernelPhaseClass,
        metrics: &Wb19LateralSubstepMetrics,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if metrics.fcdep_before <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }
        let watyld = metrics.avpora - (metrics.avfca + (1.0 - metrics.avcoca));
        if !watyld.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB19_SYMBOL_WATER_YIELD_WATYLD),
                value: watyld,
                minimum: None,
                maximum: None,
            });
        }
        Ok(watyld)
    }

    fn wb19_lateral_response(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19LateralInputs,
        lane_config: &Wb19LaneConfig,
        result: &Wb19LateralRunResult,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let drainable_after =
            Self::wb19_drainable_storage(&result.theta, &result.lateral_withdrawal_threshold);
        let soil_water_after = Self::wb19_apply_soil_water_withdrawal(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            inputs.soil_water_before,
            result.q_lateral,
        )?;

        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-LAT-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let state_updates = Self::wb19_lateral_state_updates(
            request,
            phase_class,
            inputs.soldep,
            lane_config,
            result,
            drainable_after,
            soil_water_after,
        )?;
        let flux_updates = Self::wb19_lateral_flux_updates(phase_class, inputs, result)?;
        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        Ok(KernelRunResponse::new(status, writeback))
    }

    fn wb19_lateral_state_updates(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        soldep: f64,
        lane_config: &Wb19LaneConfig,
        result: &Wb19LateralRunResult,
        drainable_after: f64,
        soil_water_after: f64,
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        let mut state_updates = Self::wb19_lateral_summary_state_updates(
            soldep,
            result,
            drainable_after,
            soil_water_after,
        );
        Self::wb19_append_lateral_layer_state_updates(lane_config, result, &mut state_updates);
        if lane_config.mofe_hourly_carry_arrays_enabled {
            Self::wb19_lateral_hourly_state_updates(
                request,
                phase_class,
                result,
                &mut state_updates,
            )?;
        }
        Ok(state_updates)
    }

    fn wb19_lateral_summary_state_updates(
        soldep: f64,
        result: &Wb19LateralRunResult,
        drainable_after: f64,
        soil_water_after: f64,
    ) -> Vec<WritebackField> {
        let mut state_updates = Vec::with_capacity(result.theta.len() + 5);
        state_updates.push(WritebackField::bounded(
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_SATURATED_DEPTH_FCDEP,
            result.fcdep_after,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_UNSATURATED_DEPTH_UNSDEP,
            result.unsdep_after,
            Some(0.0),
            Some(soldep),
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_WATER_YIELD_WATYLD,
            result.watyld,
            None,
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_POTENTIAL,
            result.q_lateral_potential_total,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_TARGET,
            result.q_lateral_target_total,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_CAPACITY_TDV,
            result.lateral_capacity_tdv_total,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_TDVV,
            result.lateral_capacity_tdv_total,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_UNREALIZED,
            (result.q_lateral_target_total - result.q_lateral).max(0.0),
            Some(0.0),
            Some(result.q_lateral_target_total),
        ));
        state_updates
    }

    fn wb19_append_lateral_layer_state_updates(
        lane_config: &Wb19LaneConfig,
        result: &Wb19LateralRunResult,
        state_updates: &mut Vec<WritebackField>,
    ) {
        for (index, value) in result.theta.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        for (index, value) in result.lateral_layer_withdrawal.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                format!("{}_{:04}", WB19_SYMBOL_LATERAL_WITHDRAWAL_ROOT, index + 1),
                *value,
                Some(0.0),
                Some(result.q_lateral),
            ));
        }
        for (index, value) in result.lateral_capacity_active_count.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                format!(
                    "{}_{:04}",
                    WB19_SYMBOL_LATERAL_CAPACITY_ACTIVE_COUNT_ROOT,
                    index + 1
                ),
                *value,
                Some(0.0),
                Some(lane_config.lane_substeps_f64),
            ));
        }
        for (index, value) in result
            .lateral_conductivity_active_count
            .iter()
            .enumerate()
        {
            state_updates.push(WritebackField::bounded(
                format!(
                    "{}_{:04}",
                    WB19_SYMBOL_LATERAL_CONDUCTIVITY_ACTIVE_COUNT_ROOT,
                    index + 1
                ),
                *value,
                Some(0.0),
                Some(lane_config.lane_substeps_f64),
            ));
        }
    }

    fn wb19_lateral_hourly_state_updates(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        result: &Wb19LateralRunResult,
        state_updates: &mut Vec<WritebackField>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        for (index, value) in result.q_lateral_substeps.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::hourly_symbol_for_request(
                    request,
                    MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
                    index + 1,
                ),
                Self::normalize_non_negative_within_tolerance(*value),
                Some(0.0),
                None,
            ));
        }
        for (index, value) in result.surface_saturation_substeps.iter().enumerate() {
            let symbol = Self::hourly_symbol_for_request(
                request,
                MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
                index + 1,
            );
            let previous_value =
                Self::optional_state_scalar_for_symbol(request, phase_class, &symbol)?
                    .unwrap_or(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &symbol,
                previous_value,
                Some(0.0),
                None,
            )?;
            let exported_value =
                Self::normalize_non_negative_within_tolerance(previous_value + *value);
            state_updates.push(WritebackField::bounded(
                symbol,
                exported_value,
                Some(0.0),
                None,
            ));
        }
        Ok(())
    }

    fn wb19_lateral_flux_updates(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19LateralInputs,
        result: &Wb19LateralRunResult,
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        let mut flux_updates = vec![WritebackField::bounded(
            WB11_SYMBOL_LATERAL_Q,
            result.q_lateral,
            Some(0.0),
            None,
        )];
        if let Some(q_drainage) = inputs.q_drainage {
            let q_subhyd = q_drainage + result.q_lateral;
            Self::require_flux_range(
                phase_class,
                WB11_SYMBOL_SUBHYD_QD,
                q_subhyd,
                Some(0.0),
                None,
            )?;
            flux_updates.push(WritebackField::bounded(
                WB11_SYMBOL_SUBHYD_QD,
                q_subhyd,
                Some(0.0),
                None,
            ));
        }
        Ok(flux_updates)
    }
}
