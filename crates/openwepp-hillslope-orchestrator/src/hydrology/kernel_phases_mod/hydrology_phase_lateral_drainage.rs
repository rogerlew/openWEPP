#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
pub(crate) fn run_lateral_transfer(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyLateralTransfer;
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

        let (mut theta, drain_threshold, conductivity, thickness, upper_limit) =
            Self::wb19_load_layer_state(request, phase_class)?;
        let lateral_conductivity = if !daily_lateral_lane && solwpv_mode >= 7778 {
            Self::wb19_load_hourly_lateral_conductivity(request, phase_class, theta.len())?
        } else {
            conductivity.clone()
        };
        let lateral_withdrawal_threshold =
            Self::wb19_frozen_adjusted_lateral_thresholds(request, phase_class, &drain_threshold)?;
        let frozen_water = Self::wb19_frozen_water_by_layer(request, phase_class, theta.len())?;
        let top_effective_upper_limit = if mofe_hourly_carry_arrays_enabled {
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
        };
        let mut field_capacity_store = Vec::with_capacity(theta.len());
        let mut porosity = Vec::with_capacity(theta.len());
        let mut field_capacity_theta = Vec::with_capacity(theta.len());
        let mut coca = Vec::with_capacity(theta.len());
        for layer_index in 1..=theta.len() {
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

        let mut q_lateral = 0.0_f64;
        let mut q_lateral_potential_total = 0.0_f64;
        let mut q_lateral_target_total = 0.0_f64;
        let mut lateral_capacity_tdv_total = 0.0_f64;
        let mut watyld = 0.0_f64;
        let mut fcdep_after = 0.0_f64;
        let mut unsdep_after = soldep;
        let mut lateral_layer_withdrawal = vec![0.0_f64; theta.len()];
        let mut lateral_capacity_active_count = vec![0.0_f64; theta.len()];
        let mut lateral_conductivity_active_count = vec![0.0_f64; theta.len()];
        let mut q_lateral_substeps = if mofe_hourly_carry_arrays_enabled {
            Vec::with_capacity(MOFE_HOURLY_CARRY_ARRAY_COUNT)
        } else {
            Vec::new()
        };
        let mut surface_saturation_substeps = if mofe_hourly_carry_arrays_enabled {
            Vec::with_capacity(MOFE_HOURLY_CARRY_ARRAY_COUNT)
        } else {
            Vec::new()
        };
        for substep_index in 0..lane_substeps {
            let mut capacity_active_layer = vec![false; theta.len()];
            let mut conductivity_active_layer = vec![false; theta.len()];
            if daily_lateral_lane {
                let mut daily_top_contiguous_block_open = true;
                for (index, theta_i) in theta.iter().enumerate() {
                    let daily_layer_active =
                        *theta_i + WB11_ZERO_THRESHOLD >= lateral_withdrawal_threshold[index];
                    let active = if solwpv_mode_lt_2006 {
                        let top_contiguous_active =
                            daily_top_contiguous_block_open && daily_layer_active;
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
            } else {
                for (index, theta_i) in theta.iter().enumerate() {
                    let meblfc = if index + 1 == theta.len() {
                        true
                    } else {
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
                        theta[index + 1] / lower_upper_limit >= 1.0 - WB11_ZERO_THRESHOLD
                    };
                    capacity_active_layer[index] = *theta_i + WB11_ZERO_THRESHOLD
                        >= lateral_withdrawal_threshold[index]
                        && meblfc;
                    conductivity_active_layer[index] =
                        *theta_i + WB11_ZERO_THRESHOLD >= drain_threshold[index] && meblfc;
                }
            }
            for (index, is_capacity_active) in capacity_active_layer.iter().enumerate() {
                if *is_capacity_active {
                    lateral_capacity_active_count[index] += 1.0;
                }
            }
            for (index, is_conductivity_active) in conductivity_active_layer.iter().enumerate() {
                if *is_conductivity_active {
                    lateral_conductivity_active_count[index] += 1.0;
                }
            }

            let mut fcdep_before = 0.0_f64;
            for (is_capacity_active, dg_i) in capacity_active_layer.iter().zip(thickness.iter()) {
                if *is_capacity_active {
                    fcdep_before += *dg_i;
                }
            }

            let mut conductivity_depth_sum = 0.0_f64;
            let mut saturated_depth_sum = 0.0_f64;
            let mut avpora = 0.0_f64;
            let mut avfca = 0.0_f64;
            let mut avcoca = 0.0_f64;
            let mut lateral_capacity_tdv = 0.0_f64;
            let mut legacy_saturation_fraction = 1.0_f64;
            if fcdep_before > WB11_ZERO_THRESHOLD {
                if daily_lateral_lane {
                    let mut daily_average_storage = 0.0_f64;
                    let mut daily_average_upper_limit = 0.0_f64;
                    let mut daily_average_hk = 0.0_f64;
                    for layer_index in 0..theta.len() {
                        if capacity_active_layer[layer_index] {
                            lateral_capacity_tdv += (theta[layer_index]
                                - lateral_withdrawal_threshold[layer_index])
                                .max(0.0);
                        }
                        if !conductivity_active_layer[layer_index] {
                            continue;
                        }
                        let fc_upper_ratio =
                            field_capacity_store[layer_index] / upper_limit[layer_index];
                        let layer_hk = if fc_upper_ratio > 0.0 {
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
                            computed_hk
                        } else {
                            0.0
                        };
                        let layer_weight = thickness[layer_index] / fcdep_before;
                        saturated_depth_sum += thickness[layer_index];
                        avpora += porosity[layer_index] * layer_weight;
                        avfca += field_capacity_theta[layer_index] * layer_weight;
                        avcoca += coca[layer_index] * layer_weight;

                        if solwpv_mode_lt_2006 {
                            conductivity_depth_sum +=
                                conductivity[layer_index] * thickness[layer_index];
                            let effective_upper_limit =
                                (upper_limit[layer_index] - frozen_water[layer_index]).max(0.0);
                            daily_average_storage += theta[layer_index] * layer_weight;
                            daily_average_upper_limit += effective_upper_limit * layer_weight;
                            daily_average_hk += layer_hk * layer_weight;
                        } else {
                            let effective_upper_limit =
                                upper_limit[layer_index] - frozen_water[layer_index];
                            let saturation_fraction = if effective_upper_limit > 0.0 {
                                theta[layer_index] / effective_upper_limit
                            } else {
                                1.0
                            };
                            if !saturation_fraction.is_finite()
                                || saturation_fraction < -WB11_ZERO_THRESHOLD
                            {
                                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                    phase_class,
                                    symbol: Self::wb18_perc_state_symbol("theta", layer_index + 1),
                                    value: saturation_fraction,
                                    minimum: Some(0.0),
                                    maximum: None,
                                });
                            }
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
                            conductivity_depth_sum += conductivity[layer_index]
                                * conductivity_fraction
                                * thickness[layer_index];
                        }
                    }
                    // UNIT-CONVERSION-ALLOW: mm_m_scale legacy soil-water averaging threshold in meters, not conversion.
                    if solwpv_mode_lt_2006 && daily_average_upper_limit > 0.001 {
                        let saturation_fraction =
                            daily_average_storage / daily_average_upper_limit;
                        legacy_saturation_fraction = if saturation_fraction < 0.95 {
                            saturation_fraction.powf(daily_average_hk).max(0.002)
                        } else {
                            1.0
                        };
                        if !legacy_saturation_fraction.is_finite() {
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: Self::wb18_perc_state_symbol("theta", 1),
                                value: legacy_saturation_fraction,
                                minimum: Some(0.0),
                                maximum: None,
                            });
                        }
                    }
                } else {
                    for layer_index in 0..theta.len() {
                        if capacity_active_layer[layer_index] {
                            lateral_capacity_tdv += (theta[layer_index]
                                - lateral_withdrawal_threshold[layer_index])
                                .max(0.0);
                        }
                        if !conductivity_active_layer[layer_index] {
                            continue;
                        }
                        let storage_excess =
                            (theta[layer_index] - drain_threshold[layer_index]).max(0.0);
                        let saturation_denominator =
                            upper_limit[layer_index] - drain_threshold[layer_index];
                        if saturation_denominator <= WB11_ZERO_THRESHOLD {
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: Self::wb18_perc_state_symbol("ul", layer_index + 1),
                                value: upper_limit[layer_index],
                                minimum: Some(
                                    drain_threshold[layer_index] + WB11_ZERO_THRESHOLD,
                                ),
                                maximum: None,
                            });
                        }
                        let saturation_fraction =
                            (storage_excess / saturation_denominator).clamp(0.0, 1.0);
                        if !saturation_fraction.is_finite() {
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: Self::wb18_perc_state_symbol("theta", layer_index + 1),
                                value: saturation_fraction,
                                minimum: Some(0.0),
                                maximum: Some(1.0),
                            });
                        }
                        legacy_saturation_fraction = saturation_fraction;
                        let layer_weight = thickness[layer_index] / fcdep_before;
                        saturated_depth_sum += thickness[layer_index];
                        conductivity_depth_sum += lateral_conductivity[layer_index]
                            * saturation_fraction
                            * thickness[layer_index];
                        avpora += porosity[layer_index] * layer_weight;
                        avfca += field_capacity_theta[layer_index] * layer_weight;
                        avcoca += coca[layer_index] * layer_weight;
                    }
                }
            }

            let q_lateral_potential = if fcdep_before <= WB11_ZERO_THRESHOLD
                || saturated_depth_sum <= WB11_ZERO_THRESHOLD
            {
                0.0
            } else {
                let mut ke = (86_400.0 / lane_substeps_f64)
                    * (conductivity_depth_sum / saturated_depth_sum);
                if solwpv_mode_lt_2006 {
                    ke *= legacy_saturation_fraction;
                }
                if !ke.is_finite() || ke < 0.0 {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: avgslp_symbol.clone(),
                        value: ke,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }

                let slope_angle = avgslp.atan();
                let slope_factor = slope_angle.sin();
                if !slope_factor.is_finite() || slope_factor < -WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: avgslp_symbol.clone(),
                        value: slope_factor,
                        minimum: Some(0.0),
                        maximum: Some(1.0),
                    });
                }

                (fcdep_before * anisotropy * ke * slope_factor.max(0.0)) / slplen
            };

            Self::require_flux_range(
                phase_class,
                WB11_SYMBOL_LATERAL_Q,
                q_lateral_potential,
                Some(0.0),
                None,
            )?;
            q_lateral_potential_total += q_lateral_potential;
            lateral_capacity_tdv_total += lateral_capacity_tdv;

            let available_pool =
                Self::wb19_drainable_storage(&theta, &lateral_withdrawal_threshold);
            let q_lateral_target = q_lateral_potential
                .min(available_pool)
                .min(lateral_capacity_tdv);
            let q_lateral_substep = Self::wb19_withdraw_top_down(
                &mut theta,
                &lateral_withdrawal_threshold,
                q_lateral_target,
                &mut lateral_layer_withdrawal,
            );
            q_lateral_target_total += q_lateral_target;
            q_lateral += q_lateral_substep;
            if mofe_hourly_carry_arrays_enabled {
                q_lateral_substeps.push(q_lateral_substep);
            }
            Self::require_flux_range(
                phase_class,
                WB11_SYMBOL_LATERAL_Q,
                q_lateral_substep,
                Some(0.0),
                Some(q_lateral_target),
            )?;
            if let Some(top_limit) = top_effective_upper_limit {
                let saturation_excess = theta[0] - top_limit;
                let current_saturation_runoff = if saturation_excess > WB11_ZERO_THRESHOLD {
                    theta[0] = top_limit;
                    saturation_excess
                } else {
                    0.0
                };
                Self::require_state_range_for_symbol(
                    phase_class,
                    &Self::hourly_symbol(
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

            watyld = 0.0;
            if fcdep_before > WB11_ZERO_THRESHOLD {
                watyld = avpora - (avfca + (1.0 - avcoca));
                if !watyld.is_finite() {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: BoundarySymbol::from(WB19_SYMBOL_WATER_YIELD_WATYLD),
                        value: watyld,
                        minimum: None,
                        maximum: None,
                    });
                }
            }

            fcdep_after = fcdep_before;
            if solwpv_mode_lt_2006 && fcdep_before > WB11_ZERO_THRESHOLD {
                if q_lateral_substep > WB11_ZERO_THRESHOLD && watyld <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: BoundarySymbol::from(WB19_SYMBOL_WATER_YIELD_WATYLD),
                        value: watyld,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }
                fcdep_after = (fcdep_before - (q_lateral_substep / watyld)).max(0.0);
            }
            unsdep_after = (soldep - fcdep_after).max(0.0);
        }

        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral,
            Some(0.0),
            Some(q_lateral_target_total),
        )?;
        let drainable_after = Self::wb19_drainable_storage(&theta, &lateral_withdrawal_threshold);
        let soil_water_after = Self::wb19_apply_soil_water_withdrawal(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            soil_water_before,
            q_lateral,
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
        let mut state_updates = Vec::with_capacity(theta.len() + 5);
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
            fcdep_after,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_UNSATURATED_DEPTH_UNSDEP,
            unsdep_after,
            Some(0.0),
            Some(soldep),
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_WATER_YIELD_WATYLD,
            watyld,
            None,
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_POTENTIAL,
            q_lateral_potential_total,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_TARGET,
            q_lateral_target_total,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_CAPACITY_TDV,
            lateral_capacity_tdv_total,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_TDVV,
            lateral_capacity_tdv_total,
            Some(0.0),
            None,
        ));
        state_updates.push(WritebackField::bounded(
            WB19_SYMBOL_LATERAL_UNREALIZED,
            (q_lateral_target_total - q_lateral).max(0.0),
            Some(0.0),
            Some(q_lateral_target_total),
        ));
        for (index, value) in theta.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        for (index, value) in lateral_layer_withdrawal.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                format!("{}_{:04}", WB19_SYMBOL_LATERAL_WITHDRAWAL_ROOT, index + 1),
                *value,
                Some(0.0),
                Some(q_lateral),
            ));
        }
        for (index, value) in lateral_capacity_active_count.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                format!(
                    "{}_{:04}",
                    WB19_SYMBOL_LATERAL_CAPACITY_ACTIVE_COUNT_ROOT,
                    index + 1
                ),
                *value,
                Some(0.0),
                Some(lane_substeps_f64),
            ));
        }
        for (index, value) in lateral_conductivity_active_count.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                format!(
                    "{}_{:04}",
                    WB19_SYMBOL_LATERAL_CONDUCTIVITY_ACTIVE_COUNT_ROOT,
                    index + 1
                ),
                *value,
                Some(0.0),
                Some(lane_substeps_f64),
            ));
        }
        if mofe_hourly_carry_arrays_enabled {
            for (index, value) in q_lateral_substeps.iter().enumerate() {
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT, index + 1),
                    Self::normalize_non_negative_within_tolerance(*value),
                    Some(0.0),
                    None,
                ));
            }
            for (index, value) in surface_saturation_substeps.iter().enumerate() {
                let symbol = Self::hourly_symbol(
                    MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
                    index + 1,
                );
                let previous_value = Self::optional_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &symbol,
                )?
                .unwrap_or(0.0);
                Self::require_state_range_for_symbol(
                    phase_class,
                    &symbol,
                    previous_value,
                    Some(0.0),
                    None,
                )?;
                let exported_value = Self::normalize_non_negative_within_tolerance(
                    previous_value + *value,
                );
                state_updates.push(WritebackField::bounded(
                    symbol,
                    exported_value,
                    Some(0.0),
                    None,
                ));
            }
        }
        let mut flux_updates = vec![WritebackField::bounded(
            WB11_SYMBOL_LATERAL_Q,
            q_lateral,
            Some(0.0),
            None,
        )];
        if let Some(q_drainage) = q_drainage {
            let q_subhyd = q_drainage + q_lateral;
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
        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
pub(crate) fn run_drainage(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyDrainage;
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

        let (mut theta, drain_threshold, conductivity, thickness, _upper_limit) =
            Self::wb19_load_layer_state(request, phase_class)?;
        let mut q_drainage = 0.0_f64;
        let mut q_drainage_target_total = 0.0_f64;
        for _ in 0..lane_substeps {
            let layer_pool = Self::wb19_drainable_storage(&theta, &drain_threshold);
            let remaining_capacity = (drainage_capacity - q_drainage).max(0.0);
            let mut q_drainage_potential = 0.0_f64;
            let mut tile_layer_index = theta.len().saturating_sub(1);

            if drain_enabled && remaining_capacity > WB11_ZERO_THRESHOLD {
                let drain_depth_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_DEPTH);
                let drain_depth = Self::require_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &drain_depth_symbol,
                )?;
                if drain_depth <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: drain_depth_symbol,
                        value: drain_depth,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }

                let drain_spacing_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_SPACING);
                let drain_spacing = Self::require_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &drain_spacing_symbol,
                )?;
                if drain_spacing <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: drain_spacing_symbol,
                        value: drain_spacing,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }

                let drain_diameter_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_DIAMETER);
                let drain_diameter = Self::require_state_scalar_for_symbol(
                    request,
                    phase_class,
                    &drain_diameter_symbol,
                )?;
                if drain_diameter <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: drain_diameter_symbol,
                        value: drain_diameter,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }

                let soldep_symbol = BoundarySymbol::from("solthk");
                let soldep =
                    Self::require_state_scalar_for_symbol(request, phase_class, &soldep_symbol)?;
                if soldep <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: soldep_symbol,
                        value: soldep,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }

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

                let dep2watbl = soldep - watbl;
                if !dep2watbl.is_finite() {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: soldep_symbol,
                        value: dep2watbl,
                        minimum: Some(0.0),
                        maximum: Some(soldep),
                    });
                }

                if dep2watbl <= drain_depth + WB11_ZERO_THRESHOLD {
                    let mut cumulative_depth = 0.0_f64;
                    let mut tile_layer = 0usize;
                    for (idx, dg) in thickness.iter().enumerate() {
                        cumulative_depth += *dg;
                        if cumulative_depth <= drain_depth + WB11_ZERO_THRESHOLD {
                            tile_layer = idx;
                        }
                    }
                    tile_layer_index = (tile_layer + 1).min(theta.len().saturating_sub(1));

                    let mut cumulative_layer_depth = 0.0_f64;
                    let mut conductivity_depth_sum = 0.0_f64;
                    let mut saturated_depth_sum = 0.0_f64;
                    for idx in 0..theta.len() {
                        cumulative_layer_depth += thickness[idx];
                        if cumulative_layer_depth + WB11_ZERO_THRESHOLD >= dep2watbl {
                            conductivity_depth_sum += conductivity[idx] * thickness[idx];
                            saturated_depth_sum += thickness[idx];
                        }
                    }

                    let dranks = if saturated_depth_sum > WB11_ZERO_THRESHOLD {
                        let saturated_conductivity_m_s =
                            conductivity_depth_sum / saturated_depth_sum;
                        openwepp_unit_boundary::conversions::meters_per_second_to_centimeters_per_hour(
                            saturated_conductivity_m_s,
                        )
                        .map_err(|error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                drain_spacing_symbol.clone(),
                                &error,
                            )
                        })?
                    } else {
                        0.0
                    };
                    if !dranks.is_finite() || dranks < 0.0 {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: drain_spacing_symbol.clone(),
                            value: dranks,
                            minimum: Some(0.0),
                            maximum: None,
                        });
                    }

                    let drain_depth_delta_m = soldep - drain_depth;
                    let drain_depth_cm = if drain_depth_delta_m < 0.0 {
                        1.0
                    } else {
                        openwepp_unit_boundary::conversions::meters_to_centimeters(
                            drain_depth_delta_m,
                        )
                        .map_err(|error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                drain_depth_symbol.clone(),
                                &error,
                            )
                        })?
                    };
                    let spacing_cm =
                        openwepp_unit_boundary::conversions::meters_to_centimeters(drain_spacing)
                            .map_err(|error| {
                                Self::unit_conversion_guard_error(
                                    phase_class,
                                    drain_spacing_symbol.clone(),
                                    &error,
                                )
                            })?;
                    let radius_cm =
                        openwepp_unit_boundary::conversions::meters_to_centimeters(
                            drain_diameter / 2.0,
                        )
                        .map_err(|error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                drain_diameter_symbol.clone(),
                                &error,
                            )
                        })?;

                    let spacing_ratio = drain_depth_cm / spacing_cm;
                    let equivalent_depth_cm = if spacing_ratio <= 0.3 && spacing_ratio > 0.0 {
                        let radius_ratio = drain_depth_cm / radius_cm;
                        if radius_ratio <= WB11_ZERO_THRESHOLD {
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: drain_diameter_symbol.clone(),
                                value: radius_ratio,
                                minimum: Some(WB11_ZERO_THRESHOLD),
                                maximum: None,
                            });
                        }
                        let denominator = 1.0
                            + spacing_ratio
                                * ((8.0 / std::f64::consts::PI) * radius_ratio.ln()
                                    - WB19_DRAIN_ALPHA);
                        if denominator <= WB11_ZERO_THRESHOLD {
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: drain_spacing_symbol.clone(),
                                value: denominator,
                                minimum: Some(WB11_ZERO_THRESHOLD),
                                maximum: None,
                            });
                        }
                        drain_depth_cm / denominator
                    } else {
                        let log_term = (spacing_cm / radius_cm).ln() - 1.15;
                        if log_term <= WB11_ZERO_THRESHOLD {
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: drain_spacing_symbol.clone(),
                                value: log_term,
                                minimum: Some(WB11_ZERO_THRESHOLD),
                                maximum: None,
                            });
                        }
                        (spacing_cm * std::f64::consts::PI) / (8.0 * log_term)
                    };
                    if !equivalent_depth_cm.is_finite() || equivalent_depth_cm < 0.0 {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: drain_spacing_symbol.clone(),
                            value: equivalent_depth_cm,
                            minimum: Some(0.0),
                            maximum: None,
                        });
                    }

                    let water_table_cm =
                        openwepp_unit_boundary::conversions::meters_to_centimeters(
                            (drain_depth - dep2watbl).max(0.0),
                        )
                        .map_err(|error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                drain_depth_symbol.clone(),
                                &error,
                            )
                        })?;
                    let drainage_cm_h = (8.0 * dranks * equivalent_depth_cm * water_table_cm
                        + 4.0 * dranks * water_table_cm.powi(2))
                        / spacing_cm.powi(2);
                    if !drainage_cm_h.is_finite() || drainage_cm_h < -WB11_ZERO_THRESHOLD {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: drain_depth_symbol.clone(),
                            value: drainage_cm_h,
                            minimum: Some(0.0),
                            maximum: None,
                        });
                    }

                    q_drainage_potential =
                        openwepp_unit_boundary::conversions::centimeters_to_meters(
                            drainage_cm_h,
                        )
                        .map_err(|error| {
                            Self::unit_conversion_guard_error(
                                phase_class,
                                drain_depth_symbol.clone(),
                                &error,
                            )
                        })?
                            * lane_hour_fraction;
                    Self::require_flux_range(
                        phase_class,
                        WB11_SYMBOL_DRAINAGE_QDD,
                        q_drainage_potential,
                        Some(0.0),
                        None,
                    )?;
                }
            }

            let available_pool = layer_pool;
            let q_drainage_target = q_drainage_potential
                .min(remaining_capacity)
                .min(available_pool);
            let q_drainage_substep = Self::wb19_withdraw_tile_to_surface(
                &mut theta,
                &drain_threshold,
                tile_layer_index,
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

        let drainable_after = Self::wb19_drainable_storage(&theta, &drain_threshold);
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_DRAINAGE_QDD,
            q_drainage,
            Some(0.0),
            Some(q_drainage_target_total.min(drainage_capacity)),
        )?;
        let soil_water_after = Self::wb19_apply_soil_water_withdrawal(
            phase_class,
            WB11_SYMBOL_DRAINAGE_QDD,
            soil_water_before,
            q_drainage,
        )?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        )?;

        let q_subhyd = q_lateral + q_drainage;
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
        let mut state_updates = Vec::with_capacity(theta.len() + 1);
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
        for (index, value) in theta.iter().enumerate() {
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
                    q_drainage,
                    Some(0.0),
                    Some(drainage_capacity),
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

    #[allow(clippy::too_many_lines)]
pub(crate) fn wb14_load_top_two_layer_ksatadj_metrics(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(f64, f64, f64), Wb11HydrologyKernelGuardError> {
        let mut theta_sum = 0.0_f64;
        let mut ul_sum = 0.0_f64;
        let mut fc_sum = 0.0_f64;
        let mut thetfc_sum = 0.0_f64;
        let mut thetdr_sum = 0.0_f64;
        let mut dg_sum = 0.0_f64;
        let mut use_legacy_ksatadj_theta_derivation = false;

        for layer_index in 1..=2 {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let (dg_symbol, dg) =
                Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;

            let theta = Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            let fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
            let ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
            let thetdr_optional =
                Self::optional_wb19_thetdr_scalar(request, phase_class, layer_index)?;

            if theta < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: theta_symbol,
                    value: theta,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            if fc < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fc_symbol,
                    value: fc,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            if ul <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ul_symbol,
                    value: ul,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            if dg <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: dg_symbol,
                    value: dg,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            if fc > ul + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fc_symbol,
                    value: fc,
                    minimum: Some(0.0),
                    maximum: Some(ul),
                });
            }
            if theta > ul + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: theta_symbol,
                    value: theta,
                    minimum: Some(0.0),
                    maximum: Some(ul),
                });
            }
            let legacy_wp_store = ul - fc;
            theta_sum += theta.max(0.0);
            ul_sum += ul;
            fc_sum += fc.max(0.0);
            dg_sum += dg;

            match thetdr_optional {
                Some((thetdr_symbol, thetdr_raw)) if !use_legacy_ksatadj_theta_derivation => {
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
                    let expected_wp_store = thetdr * dg;
                    let uses_legacy_fcwp_layout = (legacy_wp_store - expected_wp_store).abs() <= 1.0e-9;
                    let layer_thetfc = if uses_legacy_fcwp_layout {
                        fc / dg
                    } else {
                        (fc / dg) + thetdr
                    };
                    if !layer_thetfc.is_finite()
                        || layer_thetfc < thetdr - WB11_ZERO_THRESHOLD
                        || layer_thetfc > 1.0 + WB11_ZERO_THRESHOLD
                    {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: fc_symbol,
                            value: layer_thetfc,
                            minimum: Some(thetdr),
                            maximum: Some(1.0),
                        });
                    }
                    thetdr_sum += thetdr * dg;
                    thetfc_sum += layer_thetfc.max(0.0) * dg;
                }
                None => {
                    use_legacy_ksatadj_theta_derivation = true;
                }
                Some(_) => {}
            }
        }

        if ul_sum <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("wb18_perc_ul_agg_0001_0002"),
                value: ul_sum,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if dg_sum <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("dg_agg_0001_0002"),
                value: dg_sum,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let mut sat_frac = theta_sum / ul_sum;
        if !sat_frac.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("sat_frac"),
                value: sat_frac,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        if sat_frac < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("sat_frac"),
                value: sat_frac,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        sat_frac = sat_frac.clamp(0.0, 1.0);

        let (avthetafc, avthetadr) = if use_legacy_ksatadj_theta_derivation {
            (fc_sum / dg_sum, (ul_sum - fc_sum) / dg_sum)
        } else {
            (thetfc_sum / dg_sum, thetdr_sum / dg_sum)
        };

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

        Ok((sat_frac, avthetafc, avthetadr))
    }

    #[allow(clippy::too_many_lines)]
pub(crate) fn resolve_wb14_effective_soil_conductivity(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if !Self::wb14_ksatadj_flag(request, phase_class)? {
            return Ok(soil_conductivity);
        }

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
                value: solwpv,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let (sat_frac, avthetafc, avthetadr) =
            Self::wb14_load_top_two_layer_ksatadj_metrics(request, phase_class)?;
        let upper_ks_mm_h =
            openwepp_unit_boundary::ProcessRateMillimetersPerHour::from_meters_per_second(
                soil_conductivity,
            )
            .map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    BoundarySymbol::from("keff"),
                    &error,
                )
            })?
            .as_millimeters_per_hour();

        let effective_ks_mm_h = if (solwpv_rounded - 9001.0).abs() <= WB11_ZERO_THRESHOLD {
            let ksatfac_symbol = BoundarySymbol::from("ksatfac");
            let ksatfac =
                Self::require_state_scalar_for_symbol(request, phase_class, &ksatfac_symbol)?;
            if ksatfac <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ksatfac_symbol,
                    value: ksatfac,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            let ksatrec_symbol = BoundarySymbol::from("ksatrec");
            let ksatrec =
                Self::require_state_scalar_for_symbol(request, phase_class, &ksatrec_symbol)?;
            if ksatrec <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ksatrec_symbol,
                    value: ksatrec,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

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
            ((upper_ks_mm_h - lower_ks_mm_h) / denominator) * ((sat_frac / ksatrec).exp() - 1.0)
                + lower_ks_mm_h
        } else if solwpv_rounded >= 9002.0 - WB11_ZERO_THRESHOLD {
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

            let mut effective_ks = upper_ks_mm_h * sat_frac.powf(exponent);
            if (solwpv_rounded - 9003.0).abs() <= WB11_ZERO_THRESHOLD {
                let lkeff_symbol = BoundarySymbol::from("lkeff");
                let lkeff =
                    Self::require_state_scalar_for_symbol(request, phase_class, &lkeff_symbol)?;
                if lkeff > 0.0 && effective_ks < lkeff {
                    effective_ks = lkeff;
                }
            }
            effective_ks
        } else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("solwpv"),
                value: solwpv,
                minimum: Some(9001.0),
                maximum: None,
            });
        };

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
