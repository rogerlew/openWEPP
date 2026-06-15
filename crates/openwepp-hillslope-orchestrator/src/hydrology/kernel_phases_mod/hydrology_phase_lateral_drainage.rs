#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

struct Wb19LateralInputs {
    soil_water_before: f64,
    q_drainage: Option<f64>,
    avgslp_symbol: BoundarySymbol,
    avgslp: f64,
    slplen: f64,
    anisotropy: f64,
    soldep: f64,
}

struct Wb19LaneConfig {
    solwpv_mode: i32,
    solwpv_mode_lt_2006: bool,
    mofe_hourly_carry_arrays_enabled: bool,
    lane_substeps: usize,
    lane_substeps_f64: f64,
    daily_lateral_lane: bool,
}

struct Wb19LateralLayerParameters {
    field_capacity_store: Vec<f64>,
    porosity: Vec<f64>,
    field_capacity_theta: Vec<f64>,
    coca: Vec<f64>,
}

struct Wb19LateralLayerState {
    theta: Vec<f64>,
    drain_threshold: Vec<f64>,
    conductivity: Vec<f64>,
    thickness: Vec<f64>,
    upper_limit: Vec<f64>,
    lateral_conductivity: Vec<f64>,
    lateral_withdrawal_threshold: Vec<f64>,
    frozen_water: Vec<f64>,
    top_effective_upper_limit: Option<f64>,
    parameters: Wb19LateralLayerParameters,
}

struct Wb19LateralActiveLayers {
    capacity_active_layer: Vec<bool>,
    conductivity_active_layer: Vec<bool>,
}

#[derive(Default)]
struct Wb19LateralSubstepMetrics {
    fcdep_before: f64,
    conductivity_depth_sum: f64,
    saturated_depth_sum: f64,
    avpora: f64,
    avfca: f64,
    avcoca: f64,
    lateral_capacity_tdv: f64,
    legacy_saturation_fraction: f64,
}

struct Wb19LateralDepths {
    watyld: f64,
    fcdep_after: f64,
    unsdep_after: f64,
}

struct Wb19LateralRunResult {
    theta: Vec<f64>,
    lateral_withdrawal_threshold: Vec<f64>,
    q_lateral: f64,
    q_lateral_potential_total: f64,
    q_lateral_target_total: f64,
    lateral_capacity_tdv_total: f64,
    watyld: f64,
    fcdep_after: f64,
    unsdep_after: f64,
    lateral_layer_withdrawal: Vec<f64>,
    lateral_capacity_active_count: Vec<f64>,
    lateral_conductivity_active_count: Vec<f64>,
    q_lateral_substeps: Vec<f64>,
    surface_saturation_substeps: Vec<f64>,
}

struct Wb19LateralRunAccumulator {
    q_lateral: f64,
    q_lateral_potential_total: f64,
    q_lateral_target_total: f64,
    lateral_capacity_tdv_total: f64,
    watyld: f64,
    fcdep_after: f64,
    unsdep_after: f64,
    lateral_layer_withdrawal: Vec<f64>,
    lateral_capacity_active_count: Vec<f64>,
    lateral_conductivity_active_count: Vec<f64>,
    q_lateral_substeps: Vec<f64>,
    surface_saturation_substeps: Vec<f64>,
}

struct Wb19DrainageInputs {
    soil_water_before: f64,
    drainage_capacity: f64,
    q_lateral: f64,
    drain_enabled: bool,
    lane_substeps: usize,
    lane_hour_fraction: f64,
}

struct Wb19DrainageGeometry {
    drain_depth_symbol: BoundarySymbol,
    drain_depth: f64,
    drain_spacing_symbol: BoundarySymbol,
    drain_spacing: f64,
    drain_diameter_symbol: BoundarySymbol,
    drain_diameter: f64,
    soldep_symbol: BoundarySymbol,
    soldep: f64,
}

struct Wb19DrainagePotential {
    q_drainage_potential: f64,
    tile_layer_index: usize,
}

struct Wb19DrainageRunResult {
    theta: Vec<f64>,
    q_drainage: f64,
    q_drainage_target_total: f64,
}

struct Wb19DrainageLayerSlices<'a> {
    theta: &'a [f64],
    drain_threshold: &'a [f64],
    conductivity: &'a [f64],
    thickness: &'a [f64],
}

#[derive(Default)]
struct Wb14KsatadjMetricSums {
    theta_sum: f64,
    ul_sum: f64,
    fc_sum: f64,
    thetfc_sum: f64,
    thetdr_sum: f64,
    dg_sum: f64,
    use_legacy_ksatadj_theta_derivation: bool,
}

struct Wb14KsatadjLayerMetrics {
    theta_symbol: BoundarySymbol,
    fc_symbol: BoundarySymbol,
    ul_symbol: BoundarySymbol,
    dg_symbol: BoundarySymbol,
    theta: f64,
    fc: f64,
    ul: f64,
    dg: f64,
    thetdr_optional: Option<(BoundarySymbol, f64)>,
}

impl Wb11HydrologyKernel {
    pub(crate) fn run_lateral_transfer(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyLateralTransfer;
        let inputs = Self::wb19_lateral_transfer_inputs(request, phase_class)?;
        let lane_config = Self::wb19_lateral_lane_config(request, phase_class)?;
        let layer_state = Self::wb19_lateral_layer_state(request, phase_class, &lane_config)?;
        let result =
            Self::wb19_run_lateral_substeps(phase_class, &inputs, &lane_config, layer_state)?;
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
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19LateralInputs,
        lane_config: &Wb19LaneConfig,
        mut state: Wb19LateralLayerState,
    ) -> Result<Wb19LateralRunResult, Wb11HydrologyKernelGuardError> {
        let mut accumulator =
            Self::wb19_lateral_run_accumulator(lane_config, state.theta.len(), inputs.soldep);
        for substep_index in 0..lane_config.lane_substeps {
            Self::wb19_run_lateral_substep(
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
                &Self::hourly_symbol(MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT, substep_index + 1),
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
                Self::hourly_symbol(MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT, index + 1),
                Self::normalize_non_negative_within_tolerance(*value),
                Some(0.0),
                None,
            ));
        }
        for (index, value) in result.surface_saturation_substeps.iter().enumerate() {
            let symbol = Self::hourly_symbol(MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT, index + 1);
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

    pub(crate) fn run_drainage(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyDrainage;
        let inputs = Self::wb19_drainage_inputs(request, phase_class)?;
        let (theta, drain_threshold, conductivity, thickness, _upper_limit) =
            Self::wb19_load_layer_state(request, phase_class)?;
        let result = Self::wb19_run_drainage_substeps(
            request,
            phase_class,
            &inputs,
            theta,
            &drain_threshold,
            &conductivity,
            &thickness,
        )?;
        Self::wb19_drainage_response(phase_class, &inputs, &result, &drain_threshold)
    }

    fn wb19_drainage_inputs(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Wb19DrainageInputs, Wb11HydrologyKernelGuardError> {
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

        let drainage_capacity =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_DRAINAGE_COEFFICIENT)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINAGE_COEFFICIENT,
            drainage_capacity,
            Some(0.0),
            None,
        )?;

        let q_lateral = Self::optional_flux_scalar(request, phase_class, WB11_SYMBOL_LATERAL_Q)?
            .unwrap_or(0.0);
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral,
            Some(0.0),
            None,
        )?;

        let drain_enabled_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_ENABLED);
        let drain_enabled_value =
            Self::require_state_scalar_for_symbol(request, phase_class, &drain_enabled_symbol)?;
        let drain_enabled = if (drain_enabled_value - 0.0).abs() <= WB11_ZERO_THRESHOLD {
            false
        } else if (drain_enabled_value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            true
        } else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: drain_enabled_symbol,
                value: drain_enabled_value,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        };
        let lane_substeps = Self::wb19_lateral_drain_lane_substeps(request, phase_class)?;
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
        let lane_hour_fraction = WB19_DRAIN_HOURS_PER_DAY / lane_substeps_f64;
        if !lane_hour_fraction.is_finite() || lane_hour_fraction <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB19_SYMBOL_LATERAL_DRAIN_LANE_SUBSTEPS),
                value: lane_hour_fraction,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        Ok(Wb19DrainageInputs {
            soil_water_before,
            drainage_capacity,
            q_lateral,
            drain_enabled,
            lane_substeps,
            lane_hour_fraction,
        })
    }

    fn wb19_run_drainage_substeps(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19DrainageInputs,
        mut theta: Vec<f64>,
        drain_threshold: &[f64],
        conductivity: &[f64],
        thickness: &[f64],
    ) -> Result<Wb19DrainageRunResult, Wb11HydrologyKernelGuardError> {
        let mut q_drainage = 0.0_f64;
        let mut q_drainage_target_total = 0.0_f64;
        for _ in 0..inputs.lane_substeps {
            let layer_pool = Self::wb19_drainable_storage(&theta, drain_threshold);
            let remaining_capacity = (inputs.drainage_capacity - q_drainage).max(0.0);
            let layer_slices = Wb19DrainageLayerSlices {
                theta: &theta,
                drain_threshold,
                conductivity,
                thickness,
            };
            let potential = Self::wb19_drainage_substep_potential(
                request,
                phase_class,
                inputs,
                remaining_capacity,
                &layer_slices,
            )?;

            let available_pool = layer_pool;
            let q_drainage_target = potential
                .q_drainage_potential
                .min(remaining_capacity)
                .min(available_pool);
            let q_drainage_substep = Self::wb19_withdraw_tile_to_surface(
                &mut theta,
                drain_threshold,
                potential.tile_layer_index,
                q_drainage_target,
            );
            q_drainage_target_total += q_drainage_target;
            q_drainage += q_drainage_substep;
            Self::require_flux_range(
                phase_class,
                WB11_SYMBOL_DRAINAGE_QDD,
                q_drainage_substep,
                Some(0.0),
                Some(q_drainage_target),
            )?;
        }

        Ok(Wb19DrainageRunResult {
            theta,
            q_drainage,
            q_drainage_target_total,
        })
    }

    fn wb19_drainage_substep_potential(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19DrainageInputs,
        remaining_capacity: f64,
        layer_slices: &Wb19DrainageLayerSlices<'_>,
    ) -> Result<Wb19DrainagePotential, Wb11HydrologyKernelGuardError> {
        let mut potential = Wb19DrainagePotential {
            q_drainage_potential: 0.0,
            tile_layer_index: layer_slices.theta.len().saturating_sub(1),
        };
        if !inputs.drain_enabled || remaining_capacity <= WB11_ZERO_THRESHOLD {
            return Ok(potential);
        }
        let geometry = Self::wb19_drainage_geometry_inputs(request, phase_class)?;
        let dep2watbl = Self::wb19_drainage_depth_to_water_table(
            phase_class,
            &geometry,
            layer_slices.theta,
            layer_slices.drain_threshold,
            layer_slices.thickness,
        )?;
        if dep2watbl <= geometry.drain_depth + WB11_ZERO_THRESHOLD {
            potential.tile_layer_index = Self::wb19_drainage_tile_layer_index(
                layer_slices.thickness,
                geometry.drain_depth,
                layer_slices.theta.len(),
            );
            potential.q_drainage_potential = Self::wb19_drainage_potential_flux(
                phase_class,
                inputs,
                &geometry,
                dep2watbl,
                layer_slices.conductivity,
                layer_slices.thickness,
            )?;
            Self::require_flux_range(
                phase_class,
                WB11_SYMBOL_DRAINAGE_QDD,
                potential.q_drainage_potential,
                Some(0.0),
                None,
            )?;
        }
        Ok(potential)
    }

    fn wb19_drainage_geometry_inputs(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Wb19DrainageGeometry, Wb11HydrologyKernelGuardError> {
        let drain_depth_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_DEPTH);
        let drain_depth =
            Self::require_positive_state_for_symbol(request, phase_class, drain_depth_symbol.clone())?;
        let drain_spacing_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_SPACING);
        let drain_spacing =
            Self::require_positive_state_for_symbol(request, phase_class, drain_spacing_symbol.clone())?;
        let drain_diameter_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_DIAMETER);
        let drain_diameter =
            Self::require_positive_state_for_symbol(request, phase_class, drain_diameter_symbol.clone())?;
        let soldep_symbol = BoundarySymbol::from("solthk");
        let soldep = Self::require_positive_state_for_symbol(request, phase_class, soldep_symbol.clone())?;

        Ok(Wb19DrainageGeometry {
            drain_depth_symbol,
            drain_depth,
            drain_spacing_symbol,
            drain_spacing,
            drain_diameter_symbol,
            drain_diameter,
            soldep_symbol,
            soldep,
        })
    }

    fn require_positive_state_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let value = Self::require_state_scalar_for_symbol(request, phase_class, &symbol)?;
        if value <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(value)
    }

    fn wb19_drainage_depth_to_water_table(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
        theta: &[f64],
        drain_threshold: &[f64],
        thickness: &[f64],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut watbl = 0.0_f64;
        let mut hit_unsat_zone = false;
        for idx in (0..theta.len()).rev() {
            if theta[idx] + WB11_ZERO_THRESHOLD >= drain_threshold[idx] {
                if !hit_unsat_zone {
                    watbl += thickness[idx];
                }
            } else {
                hit_unsat_zone = true;
            }
        }

        let dep2watbl = geometry.soldep - watbl;
        if !dep2watbl.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: geometry.soldep_symbol.clone(),
                value: dep2watbl,
                minimum: Some(0.0),
                maximum: Some(geometry.soldep),
            });
        }
        Ok(dep2watbl)
    }

    fn wb19_drainage_tile_layer_index(
        thickness: &[f64],
        drain_depth: f64,
        layer_count: usize,
    ) -> usize {
        let mut cumulative_depth = 0.0_f64;
        let mut tile_layer = 0usize;
        for (idx, dg) in thickness.iter().enumerate() {
            cumulative_depth += *dg;
            if cumulative_depth <= drain_depth + WB11_ZERO_THRESHOLD {
                tile_layer = idx;
            }
        }
        (tile_layer + 1).min(layer_count.saturating_sub(1))
    }

    fn wb19_drainage_potential_flux(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19DrainageInputs,
        geometry: &Wb19DrainageGeometry,
        dep2watbl: f64,
        conductivity: &[f64],
        thickness: &[f64],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let dranks =
            Self::wb19_drainage_saturated_conductivity_cm_h(phase_class, geometry, dep2watbl, conductivity, thickness)?;
        let drain_depth_cm = Self::wb19_drain_depth_cm(phase_class, geometry)?;
        let spacing_cm = Self::wb19_drain_spacing_cm(phase_class, geometry)?;
        let radius_cm = Self::wb19_drain_radius_cm(phase_class, geometry)?;
        let equivalent_depth_cm =
            Self::wb19_drainage_equivalent_depth_cm(phase_class, geometry, drain_depth_cm, spacing_cm, radius_cm)?;
        let water_table_cm = Self::wb19_drainage_water_table_cm(phase_class, geometry, dep2watbl)?;
        let drainage_cm_h = (8.0 * dranks * equivalent_depth_cm * water_table_cm
            + 4.0 * dranks * water_table_cm.powi(2))
            / spacing_cm.powi(2);
        if !drainage_cm_h.is_finite() || drainage_cm_h < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: geometry.drain_depth_symbol.clone(),
                value: drainage_cm_h,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        openwepp_unit_boundary::conversions::centimeters_to_meters(drainage_cm_h)
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    geometry.drain_depth_symbol.clone(),
                    &error,
                )
            })
            .map(|value| value * inputs.lane_hour_fraction)
    }

    fn wb19_drainage_saturated_conductivity_cm_h(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
        dep2watbl: f64,
        conductivity: &[f64],
        thickness: &[f64],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut cumulative_layer_depth = 0.0_f64;
        let mut conductivity_depth_sum = 0.0_f64;
        let mut saturated_depth_sum = 0.0_f64;
        for idx in 0..conductivity.len() {
            cumulative_layer_depth += thickness[idx];
            if cumulative_layer_depth + WB11_ZERO_THRESHOLD >= dep2watbl {
                conductivity_depth_sum += conductivity[idx] * thickness[idx];
                saturated_depth_sum += thickness[idx];
            }
        }

        let dranks = if saturated_depth_sum > WB11_ZERO_THRESHOLD {
            let saturated_conductivity_m_s = conductivity_depth_sum / saturated_depth_sum;
            openwepp_unit_boundary::conversions::meters_per_second_to_centimeters_per_hour(
                saturated_conductivity_m_s,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    geometry.drain_spacing_symbol.clone(),
                    &error,
                )
            })?
        } else {
            0.0
        };
        if !dranks.is_finite() || dranks < 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: geometry.drain_spacing_symbol.clone(),
                value: dranks,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(dranks)
    }

    fn wb19_drain_depth_cm(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let drain_depth_delta_m = geometry.soldep - geometry.drain_depth;
        if drain_depth_delta_m < 0.0 {
            return Ok(1.0);
        }
        openwepp_unit_boundary::conversions::meters_to_centimeters(drain_depth_delta_m).map_err(
            |error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    geometry.drain_depth_symbol.clone(),
                    &error,
                )
            },
        )
    }

    fn wb19_drain_spacing_cm(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        openwepp_unit_boundary::conversions::meters_to_centimeters(geometry.drain_spacing)
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    geometry.drain_spacing_symbol.clone(),
                    &error,
                )
            })
    }

    fn wb19_drain_radius_cm(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        openwepp_unit_boundary::conversions::meters_to_centimeters(geometry.drain_diameter / 2.0)
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    geometry.drain_diameter_symbol.clone(),
                    &error,
                )
            })
    }

    fn wb19_drainage_equivalent_depth_cm(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
        drain_depth_cm: f64,
        spacing_cm: f64,
        radius_cm: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let spacing_ratio = drain_depth_cm / spacing_cm;
        let equivalent_depth_cm = if spacing_ratio <= 0.3 && spacing_ratio > 0.0 {
            Self::wb19_drainage_shallow_equivalent_depth_cm(
                phase_class,
                geometry,
                drain_depth_cm,
                radius_cm,
                spacing_ratio,
            )?
        } else {
            Self::wb19_drainage_deep_equivalent_depth_cm(
                phase_class,
                geometry,
                spacing_cm,
                radius_cm,
            )?
        };
        if !equivalent_depth_cm.is_finite() || equivalent_depth_cm < 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: geometry.drain_spacing_symbol.clone(),
                value: equivalent_depth_cm,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(equivalent_depth_cm)
    }

    fn wb19_drainage_shallow_equivalent_depth_cm(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
        drain_depth_cm: f64,
        radius_cm: f64,
        spacing_ratio: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let radius_ratio = drain_depth_cm / radius_cm;
        if radius_ratio <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: geometry.drain_diameter_symbol.clone(),
                value: radius_ratio,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let denominator = 1.0
            + spacing_ratio
                * ((8.0 / std::f64::consts::PI) * radius_ratio.ln() - WB19_DRAIN_ALPHA);
        if denominator <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: geometry.drain_spacing_symbol.clone(),
                value: denominator,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(drain_depth_cm / denominator)
    }

    fn wb19_drainage_deep_equivalent_depth_cm(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
        spacing_cm: f64,
        radius_cm: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let log_term = (spacing_cm / radius_cm).ln() - 1.15;
        if log_term <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: geometry.drain_spacing_symbol.clone(),
                value: log_term,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok((spacing_cm * std::f64::consts::PI) / (8.0 * log_term))
    }

    fn wb19_drainage_water_table_cm(
        phase_class: HillslopeKernelPhaseClass,
        geometry: &Wb19DrainageGeometry,
        dep2watbl: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        openwepp_unit_boundary::conversions::meters_to_centimeters(
            (geometry.drain_depth - dep2watbl).max(0.0),
        )
        .map_err(|error| {
            Self::unit_conversion_guard_error(
                phase_class,
                geometry.drain_depth_symbol.clone(),
                &error,
            )
        })
    }

    fn wb19_drainage_response(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &Wb19DrainageInputs,
        result: &Wb19DrainageRunResult,
        drain_threshold: &[f64],
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let drainable_after = Self::wb19_drainable_storage(&result.theta, drain_threshold);
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_DRAINAGE_QDD,
            result.q_drainage,
            Some(0.0),
            Some(result.q_drainage_target_total.min(inputs.drainage_capacity)),
        )?;
        let soil_water_after = Self::wb19_apply_soil_water_withdrawal(
            phase_class,
            WB11_SYMBOL_DRAINAGE_QDD,
            inputs.soil_water_before,
            result.q_drainage,
        )?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        )?;

        let q_subhyd = inputs.q_lateral + result.q_drainage;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_SUBHYD_QD,
            q_subhyd,
            Some(0.0),
            None,
        )?;

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB11-DRAIN-OK-001",
        ) else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = Vec::with_capacity(result.theta.len() + 1);
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
        for (index, value) in result.theta.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![
                WritebackField::bounded(
                    WB11_SYMBOL_DRAINAGE_QDD,
                    result.q_drainage,
                    Some(0.0),
                    Some(inputs.drainage_capacity),
                ),
                WritebackField::bounded(WB11_SYMBOL_SUBHYD_QD, q_subhyd, Some(0.0), None),
            ],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }
    pub(crate) fn wb14_ksatadj_flag(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from("ksatadj");
        let Some(value) = Self::optional_state_scalar_for_symbol(request, phase_class, &symbol)?
        else {
            return Ok(false);
        };
        if value.abs() <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }
        if (value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(true);
        }
        Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
            phase_class,
            symbol,
            value,
            minimum: Some(0.0),
            maximum: Some(1.0),
        })
    }

    pub(crate) fn wb14_load_top_two_layer_ksatadj_metrics(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(f64, f64, f64), Wb11HydrologyKernelGuardError> {
        let mut sums = Wb14KsatadjMetricSums::default();
        for layer_index in 1..=2 {
            let layer = Self::wb14_load_ksatadj_layer(request, phase_class, layer_index)?;
            Self::wb14_accumulate_ksatadj_layer(phase_class, &mut sums, &layer)?;
        }
        Self::wb14_finalize_ksatadj_metrics(phase_class, &sums)
    }

    fn wb14_load_ksatadj_layer(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<Wb14KsatadjLayerMetrics, Wb11HydrologyKernelGuardError> {
        let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
        let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
        let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
        let (dg_symbol, dg) = Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
        let theta = Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
        let fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
        let ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
        let thetdr_optional = Self::optional_wb19_thetdr_scalar(request, phase_class, layer_index)?;

        Ok(Wb14KsatadjLayerMetrics {
            theta_symbol,
            fc_symbol,
            ul_symbol,
            dg_symbol,
            theta,
            fc,
            ul,
            dg,
            thetdr_optional,
        })
    }

    fn wb14_accumulate_ksatadj_layer(
        phase_class: HillslopeKernelPhaseClass,
        sums: &mut Wb14KsatadjMetricSums,
        layer: &Wb14KsatadjLayerMetrics,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::wb14_validate_ksatadj_layer(phase_class, layer)?;
        let legacy_wp_store = layer.ul - layer.fc;
        sums.theta_sum += layer.theta.max(0.0);
        sums.ul_sum += layer.ul;
        sums.fc_sum += layer.fc.max(0.0);
        sums.dg_sum += layer.dg;
        Self::wb14_accumulate_ksatadj_theta_terms(phase_class, sums, layer, legacy_wp_store)
    }

    fn wb14_validate_ksatadj_layer(
        phase_class: HillslopeKernelPhaseClass,
        layer: &Wb14KsatadjLayerMetrics,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_wb14_ksatadj_minimum(phase_class, layer.theta_symbol.clone(), layer.theta, 0.0)?;
        Self::require_wb14_ksatadj_minimum(phase_class, layer.fc_symbol.clone(), layer.fc, 0.0)?;
        Self::require_wb14_ksatadj_minimum(
            phase_class,
            layer.ul_symbol.clone(),
            layer.ul,
            WB11_ZERO_THRESHOLD,
        )?;
        Self::require_wb14_ksatadj_minimum(
            phase_class,
            layer.dg_symbol.clone(),
            layer.dg,
            WB11_ZERO_THRESHOLD,
        )?;
        if layer.fc > layer.ul + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.fc_symbol.clone(),
                value: layer.fc,
                minimum: Some(0.0),
                maximum: Some(layer.ul),
            });
        }
        if layer.theta > layer.ul + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.theta_symbol.clone(),
                value: layer.theta,
                minimum: Some(0.0),
                maximum: Some(layer.ul),
            });
        }
        Ok(())
    }

    fn require_wb14_ksatadj_minimum(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let violated = if minimum <= WB11_ZERO_THRESHOLD {
            value < -WB11_ZERO_THRESHOLD
        } else {
            value <= WB11_ZERO_THRESHOLD
        };
        if violated {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value,
                minimum: Some(minimum),
                maximum: None,
            });
        }
        Ok(())
    }

    fn wb14_accumulate_ksatadj_theta_terms(
        phase_class: HillslopeKernelPhaseClass,
        sums: &mut Wb14KsatadjMetricSums,
        layer: &Wb14KsatadjLayerMetrics,
        legacy_wp_store: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        match &layer.thetdr_optional {
            Some((thetdr_symbol, thetdr_raw)) if !sums.use_legacy_ksatadj_theta_derivation => {
                Self::wb14_accumulate_explicit_theta_terms(
                    phase_class,
                    sums,
                    layer,
                    thetdr_symbol.clone(),
                    *thetdr_raw,
                    legacy_wp_store,
                )
            }
            None => {
                sums.use_legacy_ksatadj_theta_derivation = true;
                Ok(())
            }
            Some(_) => Ok(()),
        }
    }

    fn wb14_accumulate_explicit_theta_terms(
        phase_class: HillslopeKernelPhaseClass,
        sums: &mut Wb14KsatadjMetricSums,
        layer: &Wb14KsatadjLayerMetrics,
        thetdr_symbol: BoundarySymbol,
        thetdr_raw: f64,
        legacy_wp_store: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if !(-WB11_ZERO_THRESHOLD..=1.0 + WB11_ZERO_THRESHOLD).contains(&thetdr_raw) {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: thetdr_symbol,
                value: thetdr_raw,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        let thetdr = thetdr_raw.max(0.0);
        let expected_wp_store = thetdr * layer.dg;
        let uses_legacy_fcwp_layout = (legacy_wp_store - expected_wp_store).abs() <= 1.0e-9;
        let layer_thetfc = if uses_legacy_fcwp_layout {
            layer.fc / layer.dg
        } else {
            (layer.fc / layer.dg) + thetdr
        };
        if !layer_thetfc.is_finite()
            || layer_thetfc < thetdr - WB11_ZERO_THRESHOLD
            || layer_thetfc > 1.0 + WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: layer.fc_symbol.clone(),
                value: layer_thetfc,
                minimum: Some(thetdr),
                maximum: Some(1.0),
            });
        }
        sums.thetdr_sum += thetdr * layer.dg;
        sums.thetfc_sum += layer_thetfc.max(0.0) * layer.dg;
        Ok(())
    }

    fn wb14_finalize_ksatadj_metrics(
        phase_class: HillslopeKernelPhaseClass,
        sums: &Wb14KsatadjMetricSums,
    ) -> Result<(f64, f64, f64), Wb11HydrologyKernelGuardError> {
        Self::wb14_validate_ksatadj_sums(phase_class, sums)?;
        let sat_frac = Self::wb14_ksatadj_saturation_fraction(phase_class, sums)?;
        let (avthetafc, avthetadr) = Self::wb14_ksatadj_theta_averages(sums);
        Self::wb14_validate_ksatadj_theta_averages(phase_class, avthetafc, avthetadr)?;

        Ok((sat_frac, avthetafc, avthetadr))
    }

    fn wb14_validate_ksatadj_sums(
        phase_class: HillslopeKernelPhaseClass,
        sums: &Wb14KsatadjMetricSums,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if sums.ul_sum <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("wb18_perc_ul_agg_0001_0002"),
                value: sums.ul_sum,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if sums.dg_sum <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("dg_agg_0001_0002"),
                value: sums.dg_sum,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(())
    }

    fn wb14_ksatadj_saturation_fraction(
        phase_class: HillslopeKernelPhaseClass,
        sums: &Wb14KsatadjMetricSums,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let sat_frac = sums.theta_sum / sums.ul_sum;
        if !sat_frac.is_finite() || sat_frac < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("sat_frac"),
                value: sat_frac,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        Ok(sat_frac.clamp(0.0, 1.0))
    }

    fn wb14_ksatadj_theta_averages(sums: &Wb14KsatadjMetricSums) -> (f64, f64) {
        if sums.use_legacy_ksatadj_theta_derivation {
            (sums.fc_sum / sums.dg_sum, (sums.ul_sum - sums.fc_sum) / sums.dg_sum)
        } else {
            (sums.thetfc_sum / sums.dg_sum, sums.thetdr_sum / sums.dg_sum)
        }
    }

    fn wb14_validate_ksatadj_theta_averages(
        phase_class: HillslopeKernelPhaseClass,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if avthetafc <= WB11_ZERO_THRESHOLD || avthetadr <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avthetafc_avthetadr"),
                value: avthetafc.min(avthetadr),
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if avthetafc <= avthetadr + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avthetafc"),
                value: avthetafc,
                minimum: Some(avthetadr + WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(())
    }

    pub(crate) fn resolve_wb14_effective_soil_conductivity(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if !Self::wb14_ksatadj_flag(request, phase_class)? {
            return Ok(soil_conductivity);
        }

        let solwpv_rounded = Self::wb14_ksatadj_solwpv_mode(request, phase_class)?;
        let (sat_frac, avthetafc, avthetadr) =
            Self::wb14_load_top_two_layer_ksatadj_metrics(request, phase_class)?;
        let upper_ks_mm_h =
            Self::wb14_soil_conductivity_to_mm_h(phase_class, soil_conductivity)?;
        let effective_ks_mm_h = Self::wb14_effective_ks_mm_h(
            request,
            phase_class,
            solwpv_rounded,
            upper_ks_mm_h,
            sat_frac,
            avthetafc,
            avthetadr,
        )?;
        Self::wb14_effective_ks_to_mps(phase_class, effective_ks_mm_h)
    }

    fn wb14_ksatadj_solwpv_mode(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let solwpv_symbol = BoundarySymbol::from("solwpv");
        let solwpv =
            Self::require_state_scalar_for_symbol(request, phase_class, &solwpv_symbol)?;
        if solwpv < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: solwpv_symbol,
                value: solwpv,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let solwpv_rounded = solwpv.round();
        if (solwpv - solwpv_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("solwpv"),
                value: solwpv_rounded,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(solwpv_rounded)
    }

    fn wb14_soil_conductivity_to_mm_h(
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        openwepp_unit_boundary::ProcessRateMillimetersPerHour::from_meters_per_second(
            soil_conductivity,
        )
        .map_err(|error| {
            Self::unit_conversion_guard_error(phase_class, BoundarySymbol::from("keff"), &error)
        })
        .map(openwepp_unit_boundary::ProcessRateMillimetersPerHour::as_millimeters_per_hour)
    }

    fn wb14_effective_ks_mm_h(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        solwpv_rounded: f64,
        upper_ks_mm_h: f64,
        sat_frac: f64,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if (solwpv_rounded - 9001.0).abs() <= WB11_ZERO_THRESHOLD {
            Self::wb14_effective_ks_9001(request, phase_class, upper_ks_mm_h, sat_frac)
        } else if solwpv_rounded >= 9002.0 - WB11_ZERO_THRESHOLD {
            Self::wb14_effective_ks_9002_plus(
                request,
                phase_class,
                solwpv_rounded,
                upper_ks_mm_h,
                sat_frac,
                avthetafc,
                avthetadr,
            )
        } else {
            Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("solwpv"),
                value: solwpv_rounded,
                minimum: Some(9001.0),
                maximum: None,
            })
        }
    }

    fn wb14_effective_ks_9001(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        upper_ks_mm_h: f64,
        sat_frac: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let ksatfac_symbol = BoundarySymbol::from("ksatfac");
        let ksatfac =
            Self::require_positive_state_for_symbol(request, phase_class, ksatfac_symbol.clone())?;
        let ksatrec_symbol = BoundarySymbol::from("ksatrec");
        let ksatrec =
            Self::require_positive_state_for_symbol(request, phase_class, ksatrec_symbol.clone())?;
        let lower_ks_mm_h = upper_ks_mm_h / ksatfac;
        let denominator = (1.0 / ksatrec).exp() - 1.0;
        if denominator <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("ksatrec"),
                value: denominator,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        Ok(((upper_ks_mm_h - lower_ks_mm_h) / denominator) * ((sat_frac / ksatrec).exp() - 1.0)
            + lower_ks_mm_h)
    }

    fn wb14_effective_ks_9002_plus(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        solwpv_rounded: f64,
        upper_ks_mm_h: f64,
        sat_frac: f64,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let exponent = Self::wb14_effective_ks_9002_exponent(phase_class, avthetafc, avthetadr)?;
        let mut effective_ks = upper_ks_mm_h * sat_frac.powf(exponent);
        if (solwpv_rounded - 9003.0).abs() <= WB11_ZERO_THRESHOLD {
            let lkeff_symbol = BoundarySymbol::from("lkeff");
            let lkeff = Self::require_state_scalar_for_symbol(request, phase_class, &lkeff_symbol)?;
            if lkeff > 0.0 && effective_ks < lkeff {
                effective_ks = lkeff;
            }
        }
        Ok(effective_ks)
    }

    fn wb14_effective_ks_9002_exponent(
        phase_class: HillslopeKernelPhaseClass,
        avthetafc: f64,
        avthetadr: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let psi_denominator = avthetafc.ln() - avthetadr.ln();
        if psi_denominator <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("avthetafc_avthetadr"),
                value: psi_denominator,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let psi = (1500.0_f64.ln() - 33.0_f64.ln()) / psi_denominator;
        if psi <= WB11_ZERO_THRESHOLD || !psi.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("psi"),
                value: psi,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let lambda = 1.0 / psi;
        let exponent = (2.0 * lambda) + 3.0;
        if !lambda.is_finite() || !exponent.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("lambda"),
                value: lambda,
                minimum: None,
                maximum: None,
            });
        }
        Ok(exponent)
    }

    fn wb14_effective_ks_to_mps(
        phase_class: HillslopeKernelPhaseClass,
        effective_ks_mm_h: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if !effective_ks_mm_h.is_finite() || effective_ks_mm_h < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("keff"),
                value: effective_ks_mm_h,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let effective_ks_mm_h = if effective_ks_mm_h < 0.0 {
            0.0
        } else {
            effective_ks_mm_h
        };
        openwepp_unit_boundary::ProcessRateMillimetersPerHour::try_new(effective_ks_mm_h)
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from("keff"),
                    &error,
                )
            })
            .map(openwepp_unit_boundary::ProcessRateMillimetersPerHour::as_meters_per_second)
    }


}
