impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
    fn solve_ponded_cumulative_infiltration(
        phase_class: HillslopeKernelPhaseClass,
        conductivity: f64,
        matric_potential: f64,
        cumulative_start: f64,
        duration: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if duration <= WB11_ZERO_THRESHOLD {
            return Ok(cumulative_start);
        }
        if matric_potential <= WB11_ZERO_THRESHOLD {
            return Ok(cumulative_start + conductivity * duration);
        }

        let rhs = conductivity * duration;
        let start_plus_matric = cumulative_start + matric_potential;
        if start_plus_matric <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: cumulative_start,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let residual = |candidate: f64| {
            (candidate - cumulative_start)
                - matric_potential * ((candidate + matric_potential) / start_plus_matric).ln()
                - rhs
        };

        let mut lower = cumulative_start;
        let mut upper = cumulative_start + conductivity * duration + matric_potential;
        if upper <= lower {
            upper = lower + 1.0;
        }

        let mut upper_residual = residual(upper);
        if !upper_residual.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: upper,
                minimum: Some(cumulative_start),
                maximum: None,
            });
        }

        let mut expansion_steps = 0_usize;
        while upper_residual < 0.0 {
            upper = upper * 2.0 + 1.0;
            if !upper.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
            upper_residual = residual(upper);
            if !upper_residual.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
            expansion_steps += 1;
            if expansion_steps > 128 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
        }

        for _ in 0..128 {
            let midpoint = 0.5 * (lower + upper);
            let midpoint_residual = residual(midpoint);
            if !midpoint_residual.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: midpoint,
                    minimum: Some(cumulative_start),
                    maximum: Some(upper),
                });
            }
            if midpoint_residual > 0.0 {
                upper = midpoint;
            } else {
                lower = midpoint;
            }

            let tolerance = 1.0e-10 * upper.max(1.0);
            if (upper - lower) <= tolerance {
                break;
            }
        }

        let solution = 0.5 * (lower + upper);
        if !solution.is_finite() || solution < cumulative_start - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: solution,
                minimum: Some(cumulative_start),
                maximum: None,
            });
        }

        Ok(solution)
    }

    fn compute_interval_infiltration_depth(
        phase_class: HillslopeKernelPhaseClass,
        conductivity: f64,
        matric_potential: f64,
        cumulative_infiltration_start: f64,
        rainfall_rate: f64,
        interval_duration: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if interval_duration <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                value: interval_duration,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let interval_rainfall_depth = rainfall_rate * interval_duration;
        if !interval_rainfall_depth.is_finite() || interval_rainfall_depth < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: interval_rainfall_depth,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        if rainfall_rate <= conductivity + WB11_ZERO_THRESHOLD {
            return Ok(interval_rainfall_depth.max(0.0));
        }

        let interval_infiltration = if matric_potential <= WB11_ZERO_THRESHOLD {
            conductivity * interval_duration
        } else {
            let denominator = rainfall_rate - conductivity;
            if denominator <= WB11_ZERO_THRESHOLD {
                interval_rainfall_depth
            } else {
                let ponding_threshold = (conductivity * matric_potential) / denominator;
                if !ponding_threshold.is_finite() || ponding_threshold < 0.0 {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                        value: ponding_threshold,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }

                if cumulative_infiltration_start >= ponding_threshold - WB11_ZERO_THRESHOLD {
                    let cumulative_end = Self::solve_ponded_cumulative_infiltration(
                        phase_class,
                        conductivity,
                        matric_potential,
                        cumulative_infiltration_start,
                        interval_duration,
                    )?;
                    cumulative_end - cumulative_infiltration_start
                } else {
                    let infiltration_to_ponding =
                        (ponding_threshold - cumulative_infiltration_start).max(0.0);
                    let time_to_ponding = infiltration_to_ponding / rainfall_rate;

                    if time_to_ponding >= interval_duration - WB11_ZERO_THRESHOLD {
                        interval_rainfall_depth
                    } else {
                        let ponded_duration = interval_duration - time_to_ponding;
                        let cumulative_end = Self::solve_ponded_cumulative_infiltration(
                            phase_class,
                            conductivity,
                            matric_potential,
                            ponding_threshold,
                            ponded_duration,
                        )?;
                        infiltration_to_ponding + (cumulative_end - ponding_threshold)
                    }
                }
            }
        };

        if !interval_infiltration.is_finite()
            || interval_infiltration < -WB11_ZERO_THRESHOLD
            || interval_infiltration > interval_rainfall_depth + WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: interval_infiltration,
                minimum: Some(0.0),
                maximum: Some(interval_rainfall_depth),
            });
        }

        let non_negative_infiltration = if interval_infiltration < 0.0 {
            0.0
        } else {
            interval_infiltration
        };
        Ok(non_negative_infiltration.min(interval_rainfall_depth))
    }

    fn status_from_guard_error(error: &Wb11HydrologyKernelGuardError) -> SimulationStatus {
        let code = error.code();
        let status_result = match error.boundary_class() {
            BoundaryClass::NonFinite => {
                SimulationStatus::non_finite_failure(SimulationPhase::HillslopeKernel, code)
            }
            BoundaryClass::MissingRequiredInput | BoundaryClass::DomainViolation => {
                SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    error.boundary_class(),
                    code,
                )
            }
            _ => SimulationStatus::failure(
                SimulationPhase::HillslopeKernel,
                true,
                false,
                BoundaryClass::DomainViolation,
                "HKERNEL-WB11-GEN-E-003",
            ),
        };

        match status_result {
            Ok(status) => status,
            Err(_) => unreachable!("status message ids are non-empty WB11 constants"),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn run_evapotranspiration(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyEvapotranspiration;
        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water,
            Some(0.0),
            None,
        )?;

        let et_demand = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_ET_DEMAND)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_ET_DEMAND,
            et_demand,
            Some(0.0),
            None,
        )?;

        let lai = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_LAI)?;
        Self::require_state_range(phase_class, WB15_SYMBOL_PLANT_LAI, lai, Some(0.0), None)?;

        let residue_interception =
            Self::require_state_scalar(request, phase_class, WB17_SYMBOL_RESIDUE_INTERCEPTION)?;
        Self::require_state_range(
            phase_class,
            WB17_SYMBOL_RESIDUE_INTERCEPTION,
            residue_interception,
            Some(0.0),
            None,
        )?;

        let stage_s1_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_S1);
        let stage_s2_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_S2);
        let stage_threshold_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_TU);
        let stage_counter_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_TV);
        let stage_s1 =
            Self::optional_state_scalar_for_symbol(request, phase_class, &stage_s1_symbol)?;
        let stage_s2 =
            Self::optional_state_scalar_for_symbol(request, phase_class, &stage_s2_symbol)?;
        let stage_threshold =
            Self::optional_state_scalar_for_symbol(request, phase_class, &stage_threshold_symbol)?;
        let stage_counter =
            Self::optional_state_scalar_for_symbol(request, phase_class, &stage_counter_symbol)?;
        let stage_state = match (stage_s1, stage_s2, stage_threshold, stage_counter) {
            (None, None, None, None) => None,
            (Some(s1), Some(s2), Some(tu), Some(tv)) => {
                Self::require_state_range_for_symbol(
                    phase_class,
                    &stage_s1_symbol,
                    s1,
                    Some(0.0),
                    None,
                )?;
                Self::require_state_range_for_symbol(
                    phase_class,
                    &stage_s2_symbol,
                    s2,
                    Some(0.0),
                    None,
                )?;
                if tu <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: stage_threshold_symbol.clone(),
                        value: tu,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }
                Self::require_state_range_for_symbol(
                    phase_class,
                    &stage_counter_symbol,
                    tv,
                    Some(0.0),
                    None,
                )?;
                Some((s1, s2, tu, tv))
            }
            _ => {
                let missing_symbol = if stage_s1.is_none() {
                    stage_s1_symbol.clone()
                } else if stage_s2.is_none() {
                    stage_s2_symbol.clone()
                } else if stage_threshold.is_none() {
                    stage_threshold_symbol.clone()
                } else {
                    stage_counter_symbol.clone()
                };
                return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: missing_symbol,
                });
            }
        };

        let soil_evaporation_partition_potential =
            et_demand * (-WB17_LAI_PARTITION_COEFFICIENT * lai).exp();
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ES,
            soil_evaporation_partition_potential,
            Some(0.0),
            Some(et_demand),
        )?;

        let transpiration_partition_potential = et_demand - soil_evaporation_partition_potential;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_EP,
            transpiration_partition_potential,
            Some(0.0),
            Some(et_demand),
        )?;

        let residue_evaporation = residue_interception.min(soil_evaporation_partition_potential);
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ER,
            residue_evaporation,
            Some(0.0),
            Some(soil_evaporation_partition_potential),
        )?;

        let soil_evaporation_potential = soil_evaporation_partition_potential - residue_evaporation;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ES,
            soil_evaporation_potential,
            Some(0.0),
            Some(soil_evaporation_partition_potential),
        )?;

        let mut stage_state_updates = Vec::new();
        let soil_evaporation_demand = if let Some((mut s1, mut s2, tu, mut tv)) = stage_state {
            let infiltration =
                Self::optional_state_scalar(request, phase_class, WB12_SYMBOL_INFILTRATION)?
                    .unwrap_or(0.0);
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_INFILTRATION,
                infiltration,
                Some(0.0),
                None,
            )?;

            let mut es_stage = soil_evaporation_potential;
            if s1 < tu {
                s2 = 0.0;
                let sp = s1 - infiltration;
                s1 = if sp > 0.0 { sp } else { 0.0 };
                s1 += soil_evaporation_potential;
                let su = s1 - tu;
                if su > 0.0 {
                    es_stage = soil_evaporation_potential - WB17_STAGE_ONE_DEFICIT_SCALE * su;
                    s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
                    tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                }
            } else {
                let sb = infiltration - s2;
                if sb < 0.0 {
                    tv += 1.0;
                    es_stage = WB17_STAGE_TWO_DENOMINATOR * tv.sqrt() - s2;
                    if infiltration > 0.0 {
                        let mut esx = 0.8 * infiltration;
                        if es_stage > esx {
                            esx = es_stage + infiltration;
                        }
                        if esx > soil_evaporation_potential {
                            esx = soil_evaporation_potential;
                        }
                        es_stage = esx;
                    } else if es_stage > soil_evaporation_potential {
                        es_stage = soil_evaporation_potential;
                    }
                    s2 += es_stage - infiltration;
                    tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                } else {
                    s1 = tu - sb;
                    tv = 0.0;
                    s2 = 0.0;
                    if s1 < 0.0 {
                        s1 = 0.0;
                    }
                    s1 += soil_evaporation_potential;
                    let su = s1 - tu;
                    if su > 0.0 {
                        es_stage = soil_evaporation_potential - WB17_STAGE_ONE_DEFICIT_SCALE * su;
                        s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
                        tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                    }
                }
            }

            Self::require_state_range_for_symbol(
                phase_class,
                &stage_s1_symbol,
                s1,
                Some(0.0),
                None,
            )?;
            Self::require_state_range_for_symbol(
                phase_class,
                &stage_s2_symbol,
                s2,
                Some(0.0),
                None,
            )?;
            Self::require_state_range_for_symbol(
                phase_class,
                &stage_threshold_symbol,
                tu,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;
            Self::require_state_range_for_symbol(
                phase_class,
                &stage_counter_symbol,
                tv,
                Some(0.0),
                None,
            )?;
            Self::require_flux_range(
                phase_class,
                WB17_SYMBOL_ES,
                es_stage,
                Some(0.0),
                Some(soil_evaporation_potential),
            )?;

            stage_state_updates.extend([
                WritebackField::bounded(stage_s1_symbol.clone(), s1, Some(0.0), None),
                WritebackField::bounded(stage_s2_symbol.clone(), s2, Some(0.0), None),
                WritebackField::bounded(
                    stage_threshold_symbol.clone(),
                    tu,
                    Some(WB11_ZERO_THRESHOLD),
                    None,
                ),
                WritebackField::bounded(stage_counter_symbol.clone(), tv, Some(0.0), None),
            ]);
            es_stage
        } else {
            soil_evaporation_potential
        };

        let soil_evaporation_actual = soil_water.min(soil_evaporation_demand);
        let soil_after_evaporation = soil_water - soil_evaporation_actual;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_after_evaporation,
            Some(0.0),
            None,
        )?;

        let transpiration_actual = soil_after_evaporation.min(transpiration_partition_potential);
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_EP,
            transpiration_actual,
            Some(0.0),
            Some(transpiration_partition_potential),
        )?;

        let soil_water_after = soil_after_evaporation - transpiration_actual;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;

        let actual_et = residue_evaporation + soil_evaporation_actual + transpiration_actual;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_ET,
            actual_et,
            Some(0.0),
            Some(et_demand),
        )?;

        let etp = transpiration_partition_potential;
        let upi = etp;
        let ui = transpiration_actual;
        let etp_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_ETP);
        let uptake_potential_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UPI);
        let uptake_actual_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UI);
        Self::require_flux_range_for_symbol(
            phase_class,
            &etp_symbol,
            etp,
            Some(0.0),
            Some(et_demand),
        )?;
        Self::require_flux_range_for_symbol(
            phase_class,
            &uptake_potential_symbol,
            upi,
            Some(0.0),
            Some(et_demand),
        )?;
        Self::require_flux_range_for_symbol(
            phase_class,
            &uptake_actual_symbol,
            ui,
            Some(0.0),
            Some(upi),
        )?;

        let ws = if etp <= WB11_ZERO_THRESHOLD {
            1.0
        } else {
            ui / etp
        };
        Self::require_flux_range(phase_class, WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0))?;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ES,
            soil_evaporation_actual,
            Some(0.0),
            Some(soil_evaporation_demand),
        )?;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ER,
            residue_evaporation,
            Some(0.0),
            Some(residue_interception),
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-ET-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = vec![WritebackField::bounded(
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )];
        state_updates.extend(stage_state_updates);

        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![
                WritebackField::bounded(WB11_SYMBOL_ET, actual_et, Some(0.0), None),
                WritebackField::bounded(WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0)),
                WritebackField::bounded(WB17_SYMBOL_EP, transpiration_actual, Some(0.0), None),
                WritebackField::bounded(WB17_SYMBOL_ES, soil_evaporation_actual, Some(0.0), None),
                WritebackField::bounded(WB17_SYMBOL_ER, residue_evaporation, Some(0.0), None),
                WritebackField::bounded(etp_symbol, etp, Some(0.0), None),
                WritebackField::bounded(uptake_potential_symbol, upi, Some(0.0), None),
                WritebackField::bounded(uptake_actual_symbol, ui, Some(0.0), None),
            ],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_percolation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage;
        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water,
            Some(0.0),
            None,
        )?;

        // Keep legacy WB11 symbol validation to preserve mixed-lane seam guard
        // posture while WB18 per-layer symbols carry the execution authority.
        let field_capacity_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_FIELD_CAPACITY)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_FIELD_CAPACITY,
            field_capacity_legacy,
            Some(0.0),
            None,
        )?;
        let perc_fraction_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_PERC_FRACTION)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_PERC_FRACTION,
            perc_fraction_legacy,
            Some(0.0),
            Some(1.0),
        )?;

        let nsl_symbol = BoundarySymbol::from("nsl");
        let layer_count = Self::require_state_non_negative_integral_for_symbol(
            request,
            phase_class,
            &nsl_symbol,
        )?;
        if layer_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: nsl_symbol,
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }

        let mut theta = Vec::with_capacity(layer_count);
        let mut field_capacity = Vec::with_capacity(layer_count);
        let mut upper_limit = Vec::with_capacity(layer_count);
        let mut conductivity = Vec::with_capacity(layer_count);

        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index);

            let layer_theta =
                Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                layer_theta,
                Some(0.0),
                None,
            )?;

            let layer_fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &fc_symbol,
                layer_fc,
                Some(0.0),
                None,
            )?;

            let layer_ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
            if layer_ul <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ul_symbol,
                    value: layer_ul,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            if layer_fc > layer_ul + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fc_symbol,
                    value: layer_fc,
                    minimum: Some(0.0),
                    maximum: Some(layer_ul),
                });
            }

            let layer_ssc =
                Self::require_state_scalar_for_symbol(request, phase_class, &ssc_symbol)?;
            if layer_ssc <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ssc_symbol,
                    value: layer_ssc,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            theta.push(layer_theta);
            field_capacity.push(layer_fc);
            upper_limit.push(layer_ul);
            conductivity.push(layer_ssc);
        }

        let lane_substeps_symbol = BoundarySymbol::from("wb18_perc_lane_substeps");
        let lane_substeps_raw =
            Self::optional_state_scalar_for_symbol(request, phase_class, &lane_substeps_symbol)?
                .unwrap_or(1.0);
        if lane_substeps_raw < 1.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: lane_substeps_symbol.clone(),
                value: lane_substeps_raw,
                minimum: Some(1.0),
                maximum: None,
            });
        }
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
        let daily_lane = (lane_substeps - 1.0).abs() <= WB11_ZERO_THRESHOLD;

        let restrictive_layer_flag_symbol = BoundarySymbol::from("slflag");
        let restrictive_layer_flag_raw = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &restrictive_layer_flag_symbol,
        )?
        .unwrap_or(0.0);
        let restrictive_layer_enabled =
            if restrictive_layer_flag_raw.abs() <= WB11_ZERO_THRESHOLD {
                false
            } else if (restrictive_layer_flag_raw - 1.0).abs() <= WB11_ZERO_THRESHOLD {
                true
            } else {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: restrictive_layer_flag_symbol,
                    value: restrictive_layer_flag_raw,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            };
        let restrictive_layer_conductivity_symbol = BoundarySymbol::from("kslast");
        let restrictive_layer_conductivity = if restrictive_layer_enabled {
            let observed = Self::require_state_scalar_for_symbol(
                request,
                phase_class,
                &restrictive_layer_conductivity_symbol,
            )?;
            if observed <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: restrictive_layer_conductivity_symbol.clone(),
                    value: observed,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            Some(observed)
        } else {
            None
        };

        let mut per_layer_flux = vec![0.0_f64; layer_count];
        let mut percolation_loss = 0.0_f64;

        // Bottom-up routing mirrors legacy WEPP percolation ordering in PURK.
        let mut lane_substep_index = 0.0_f64;
        while lane_substep_index < lane_substeps {
            let mut substep_percolation_loss = 0.0_f64;
            for layer_index in (0..layer_count).rev() {
                let layer_theta = theta[layer_index];
                let layer_fc = field_capacity[layer_index];
                let layer_ul = upper_limit[layer_index];
                let layer_ssc = conductivity[layer_index];

                let excess = layer_theta - layer_fc;
                if excess <= WB11_ZERO_THRESHOLD {
                    continue;
                }

                let stz = layer_theta / layer_ul;
                if !stz.is_finite() || stz < 0.0 {
                    let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 1);
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: theta_symbol,
                        value: stz,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }

                let fx = if stz < WB18_PERC_SATURATION_THRESHOLD {
                    let fc_ul_ratio = layer_fc / layer_ul;
                    if !fc_ul_ratio.is_finite() || fc_ul_ratio >= 1.0 {
                        let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index + 1);
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: fc_symbol,
                            value: fc_ul_ratio,
                            minimum: Some(0.0),
                            maximum: Some(1.0),
                        });
                    }
                    // Legacy-authoritative fallback: watbal.for sets hk=0 when FC/UL <= 0.
                    let bi = if fc_ul_ratio <= 0.0 {
                        0.0
                    } else {
                        let derived = -WB18_PERC_BI_COEFFICIENT / fc_ul_ratio.log10();
                        if !derived.is_finite() || derived < 0.0 {
                            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index + 1);
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: fc_symbol,
                                value: derived,
                                minimum: Some(0.0),
                                maximum: None,
                            });
                        }
                        derived
                    };
                    stz.powf(bi).max(WB18_PERC_MIN_FX)
                } else {
                    1.0
                };
                if !fx.is_finite() || fx <= 0.0 {
                    let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index + 1);
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: ssc_symbol,
                        value: fx,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }

                let layer_ssc_effective =
                    if daily_lane && restrictive_layer_enabled && layer_index == layer_count - 1 {
                        let restrictive_layer_conductivity =
                            restrictive_layer_conductivity.unwrap_or(layer_ssc);
                        let denominator = layer_ssc + restrictive_layer_conductivity;
                        if denominator <= WB11_ZERO_THRESHOLD {
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: restrictive_layer_conductivity_symbol.clone(),
                                value: denominator,
                                minimum: Some(WB11_ZERO_THRESHOLD),
                                maximum: None,
                            });
                        }
                        let harmonic_mean =
                            (2.0 * layer_ssc * restrictive_layer_conductivity) / denominator;
                        if !harmonic_mean.is_finite() || harmonic_mean <= WB11_ZERO_THRESHOLD {
                            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                                phase_class,
                                symbol: restrictive_layer_conductivity_symbol.clone(),
                                value: harmonic_mean,
                                minimum: Some(WB11_ZERO_THRESHOLD),
                                maximum: None,
                            });
                        }
                        harmonic_mean
                    } else {
                        layer_ssc
                    };

                let ks_adjusted = layer_ssc_effective * fx;
                let pei_pre = (WB18_PERC_TIMESTEP_S * ks_adjusted).min(excess);
                let pei_unscaled = if layer_index < layer_count - 1 {
                    let lower_ratio = theta[layer_index + 1] / upper_limit[layer_index + 1];
                    let lower_radicand = 1.0 - lower_ratio;
                    if lower_radicand < -WB11_ZERO_THRESHOLD {
                        let lower_theta_symbol =
                            Self::wb18_perc_state_symbol("theta", layer_index + 2);
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: lower_theta_symbol,
                            value: lower_ratio,
                            minimum: Some(0.0),
                            maximum: Some(1.0),
                        });
                    }
                    let lower_factor = if lower_radicand <= 0.0 {
                        0.0
                    } else {
                        lower_radicand.sqrt()
                    };
                    pei_pre * lower_factor
                } else {
                    pei_pre
                };
                let pei = pei_unscaled / lane_substeps;

                let pei_symbol = Self::wb18_perc_flux_symbol(layer_index + 1);
                Self::require_flux_range_for_symbol(
                    phase_class,
                    &pei_symbol,
                    pei,
                    Some(0.0),
                    Some(excess),
                )?;

                theta[layer_index] -= pei;
                let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 1);
                Self::require_state_range_for_symbol(
                    phase_class,
                    &theta_symbol,
                    theta[layer_index],
                    Some(0.0),
                    None,
                )?;

                if layer_index < layer_count - 1 {
                    theta[layer_index + 1] += pei;
                } else {
                    substep_percolation_loss = pei;
                }

                per_layer_flux[layer_index] += pei;
            }
            percolation_loss += substep_percolation_loss;
            lane_substep_index += 1.0;
        }

        let soil_water_after: f64 = theta.iter().sum();
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_LOSS_D,
            percolation_loss,
            Some(0.0),
            None,
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-PERC-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = Vec::with_capacity(layer_count + 1);
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

        let mut flux_updates = Vec::with_capacity(layer_count + 2);
        for (index, value) in per_layer_flux.iter().enumerate() {
            flux_updates.push(WritebackField::bounded(
                Self::wb18_perc_flux_symbol(index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        flux_updates.push(WritebackField::bounded(
            WB11_SYMBOL_PERC_LOSS_D,
            percolation_loss,
            Some(0.0),
            None,
        ));
        flux_updates.push(WritebackField::bounded(
            WB11_SYMBOL_PERC_RECHARGE_PE,
            percolation_loss,
            Some(0.0),
            None,
        ));

        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_lateral_transfer(
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
        let solwpv_mode_is_2006 = solwpv_mode == 2006;
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

        let (mut theta, drain_threshold, conductivity, thickness, _upper_limit) =
            Self::wb19_load_layer_state(request, phase_class)?;
        let mut porosity = Vec::with_capacity(theta.len());
        let mut field_capacity_theta = Vec::with_capacity(theta.len());
        let mut coca = Vec::with_capacity(theta.len());
        for layer_index in 1..=theta.len() {
            let por_symbol = Self::wb19_por_symbol(layer_index);
            let por = Self::require_state_scalar_for_symbol(request, phase_class, &por_symbol)?;
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

            let thetfc_symbol = Self::wb19_thetfc_symbol(layer_index);
            let layer_thetfc =
                Self::require_state_scalar_for_symbol(request, phase_class, &thetfc_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &thetfc_symbol,
                layer_thetfc,
                Some(0.0),
                None,
            )?;

            let thetdr_symbol = Self::wb19_thetdr_symbol(layer_index);
            let layer_thetdr =
                Self::require_state_scalar_for_symbol(request, phase_class, &thetdr_symbol)?;
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
            field_capacity_theta.push(layer_thetfc);

            let coca_symbol = Self::wb19_coca_symbol(layer_index);
            let layer_coca =
                Self::require_state_scalar_for_symbol(request, phase_class, &coca_symbol)?;
            coca.push(layer_coca);
        }

        let mut q_lateral = 0.0_f64;
        let mut q_lateral_target_total = 0.0_f64;
        let mut watyld = 0.0_f64;
        let mut fcdep_after = 0.0_f64;
        let mut unsdep_after = soldep;
        let mut q_lateral_substeps = if mofe_hourly_carry_arrays_enabled {
            Vec::with_capacity(MOFE_HOURLY_CARRY_ARRAY_COUNT)
        } else {
            Vec::new()
        };
        for _ in 0..lane_substeps {
            let mut saturated_layer = vec![false; theta.len()];
            let mut encountered_unsaturated = false;
            for (index, (theta_i, threshold_i)) in
                theta.iter().zip(drain_threshold.iter()).enumerate()
            {
                let saturated = *theta_i + WB11_ZERO_THRESHOLD >= *threshold_i;
                if solwpv_mode_is_2006 {
                    saturated_layer[index] = saturated;
                } else if !encountered_unsaturated && saturated {
                    saturated_layer[index] = true;
                } else {
                    encountered_unsaturated = true;
                }
            }

            let mut fcdep_before = 0.0_f64;
            for (is_saturated, dg_i) in saturated_layer.iter().zip(thickness.iter()) {
                if *is_saturated {
                    fcdep_before += *dg_i;
                }
            }

            let mut conductivity_depth_sum = 0.0_f64;
            let mut saturated_depth_sum = 0.0_f64;
            let mut avpora = 0.0_f64;
            let mut avfca = 0.0_f64;
            let mut avcoca = 0.0_f64;
            if fcdep_before > WB11_ZERO_THRESHOLD {
                for layer_index in 0..theta.len() {
                    if !saturated_layer[layer_index] {
                        continue;
                    }
                    let layer_weight = thickness[layer_index] / fcdep_before;
                    saturated_depth_sum += thickness[layer_index];
                    conductivity_depth_sum += conductivity[layer_index] * thickness[layer_index];
                    avpora += porosity[layer_index] * layer_weight;
                    avfca += field_capacity_theta[layer_index] * layer_weight;
                    avcoca += coca[layer_index] * layer_weight;
                }
            }

            let q_lateral_potential = if fcdep_before <= WB11_ZERO_THRESHOLD
                || saturated_depth_sum <= WB11_ZERO_THRESHOLD
            {
                0.0
            } else {
                let ke = (86_400.0 / lane_substeps_f64) * (conductivity_depth_sum / saturated_depth_sum);
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

            let layer_pool = Self::wb19_drainable_storage(&theta, &drain_threshold);
            let available_pool = layer_pool;
            let q_lateral_target = q_lateral_potential.min(available_pool);
            let q_lateral_substep =
                Self::wb19_withdraw_top_down(&mut theta, &drain_threshold, q_lateral_target);
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
                if watyld <= WB11_ZERO_THRESHOLD {
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
        let drainable_after = Self::wb19_drainable_storage(&theta, &drain_threshold);
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
        for (index, value) in theta.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
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
        }
        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![WritebackField::bounded(
                WB11_SYMBOL_LATERAL_Q,
                q_lateral,
                Some(0.0),
                None,
            )],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_drainage(
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

        let q_lateral = Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_LATERAL_Q)?;
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
                        (conductivity_depth_sum / saturated_depth_sum) * 3600.0 * 100.0
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

                    let mut drain_depth_cm = (soldep - drain_depth) * 100.0;
                    if drain_depth_cm < 0.0 {
                        drain_depth_cm = 1.0;
                    }
                    let spacing_cm = drain_spacing * 100.0;
                    let radius_cm = (drain_diameter / 2.0) * 100.0;

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

                    let water_table_cm = (drain_depth - dep2watbl).max(0.0) * 100.0;
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

                    q_drainage_potential = (drainage_cm_h / 100.0) * lane_hour_fraction;
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

    fn wb14_ksatadj_flag(
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
    fn wb14_load_top_two_layer_ksatadj_metrics(
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
            let dg_symbol = Self::wb19_dg_symbol(layer_index);
            let thetdr_symbol = BoundarySymbol::from(format!("thetdr_{layer_index:04}"));

            let theta = Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            let fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
            let ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
            let dg = Self::require_state_scalar_for_symbol(request, phase_class, &dg_symbol)?;
            let thetdr_optional =
                Self::optional_state_scalar_for_symbol(request, phase_class, &thetdr_symbol)?;

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
                Some(thetdr_raw) if !use_legacy_ksatadj_theta_derivation => {
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
    fn resolve_wb14_effective_soil_conductivity(
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
        let upper_ks_mm_h = soil_conductivity * 3.6e6;

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

        Ok((if effective_ks_mm_h < 0.0 {
            0.0
        } else {
            effective_ks_mm_h
        }) / 3.6e6)
    }

    #[allow(clippy::too_many_lines)]
    fn run_runoff_reconciliation(
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

        let active_snow_coupling = Self::resolve_active_snow_coupling(request, phase_class)?;
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling(request, phase_class, hyetograph_rainfall)?
        } else {
            SnowCouplingOutcome {
                signed_s: 0.0,
                accumulation: 0.0,
                runtime_swe: 0.0,
                runtime_depth_m: 0.0,
                runtime_density_kg_m3: 0.0,
                runtime_settle_day_count: 0.0,
                hourly_state: Vec::new(),
            }
        };
        let hyetograph_liquid_input = hyetograph_rainfall - snow_coupling.accumulation;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_liquid_input,
            Some(0.0),
            None,
        )?;

        let interception =
            Self::compute_canopy_interception_depth(request, phase_class, hyetograph_liquid_input)?;
        let (hyetograph_liquid_after_interception, rainfall_scale) =
            Self::resolve_interception_rainfall_scale(
                phase_class,
                hyetograph_rainfall,
                hyetograph_liquid_input,
                interception,
            )?;
        let cumulative_infiltration = Self::compute_coupled_infiltration_depth(
            phase_class,
            infiltration_conductivity,
            matric_potential,
            &times,
            &intensities,
            rainfall_scale,
            irrigation_rate_m_per_s,
            irrigation_duration_s,
        )?;
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
            liquid_after_interception,
        )?;

        let runon_input = Self::resolve_runoff_carryover_input(request, phase_class)?;

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
        let runoff_snow_term = snow_coupling.signed_s + snow_coupling.accumulation;

        let q_runoff = Self::compute_runoff_after_interception(
            phase_class,
            liquid_after_interception,
            runoff_snow_term,
            runon_input,
            cumulative_infiltration,
            depression_storage_delta,
        )?;

        let closure_delta = if forward_solver_lane {
            let solver_closure = liquid_after_interception + runon_input + runoff_snow_term
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
        let mofe_hourly_lateral_carry = if mofe_hourly_carry_arrays_enabled {
            Some(Self::require_mofe_hourly_state_array(
                request,
                phase_class,
                MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
            )?)
        } else {
            None
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
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_SNOW_RUNTIME_SWE,
                snow_coupling.runtime_swe,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                SNOW_RUNTIME_DEPTH_M_SYMBOL,
                snow_coupling.runtime_depth_m,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL,
                snow_coupling.runtime_density_kg_m3,
                Some(0.0),
                Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
            ));
            state_updates.push(WritebackField::bounded(
                SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL,
                snow_coupling.runtime_settle_day_count,
                Some(0.0),
                None,
            ));
            for hourly in &snow_coupling.hourly_state {
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_BEFORE_ROOT, hourly.hour),
                    hourly.depth_before_m,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_AVAILABLE_ROOT, hourly.hour),
                    hourly.depth_available_m,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_DENSITY_BEFORE_ROOT, hourly.hour),
                    hourly.density_before_kg_m3,
                    Some(0.0),
                    Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_DEPTH_AFTER_ROOT, hourly.hour),
                    hourly.depth_after_m,
                    Some(0.0),
                    None,
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_DENSITY_AFTER_ROOT, hourly.hour),
                    hourly.density_after_kg_m3,
                    Some(0.0),
                    Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
                ));
                state_updates.push(WritebackField::bounded(
                    Self::hourly_symbol(SNOW_HOURLY_MELT_ROOT, hourly.hour),
                    hourly.melt_m,
                    Some(0.0),
                    None,
                ));
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
                Some(WB14_FROST_MAX_DEPTH_M),
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_DTHAW,
                frost_outcome.dthaw,
                Some(0.0),
                Some(WB14_FROST_MAX_DEPTH_M),
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
                BoundarySymbol::from(FROST_RUNTIME_FRDP_M_SYMBOL),
                frost_outcome.frdp_m,
                Some(0.0),
                Some(WB14_FROST_MAX_DEPTH_M),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_THDP_M_SYMBOL),
                frost_outcome.thdp_m,
                Some(0.0),
                Some(WB14_FROST_MAX_DEPTH_M),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_TFRDP_M_SYMBOL),
                frost_outcome.tfrdp_m,
                Some(0.0),
                Some(WB14_FROST_MAX_DEPTH_M),
            ));
            state_updates.push(WritebackField::bounded(
                BoundarySymbol::from(FROST_RUNTIME_TTHAWD_M_SYMBOL),
                frost_outcome.tthawd_m,
                Some(0.0),
                Some(WB14_FROST_MAX_DEPTH_M),
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

        let flux_updates = vec![
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

        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_storage_reconciliation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyStorageReconciliation;
        let storage_initial =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_INITIAL)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_INITIAL,
            storage_initial,
            Some(0.0),
            None,
        )?;

        let forward_solver_lane =
            Self::resolve_wb20_forward_solver_lane_enabled(request, phase_class)?;

        let closure_tolerance = Self::require_state_scalar(
            request,
            phase_class,
            WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE,
        )?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE,
            closure_tolerance,
            Some(0.0),
            None,
        )?;

        let precip_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_PRECIP_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_PRECIP_INPUT,
            precip_input,
            Some(0.0),
            None,
        )?;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;

        let snow_coupling_s =
            Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_SNOW_COUPLING_S)?;

        let interception_i =
            Self::require_flux_scalar(request, phase_class, WB15_SYMBOL_INTERCEPTION_I)?;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception_i,
            Some(0.0),
            None,
        )?;
        let irrigation_input =
            Self::optional_flux_scalar(request, phase_class, IRRIG_SYMBOL_DAILY_IRRIGATION)?
                .unwrap_or(0.0);
        Self::require_flux_range(
            phase_class,
            IRRIG_SYMBOL_DAILY_IRRIGATION,
            irrigation_input,
            Some(0.0),
            None,
        )?;

        let et = Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_ET)?;
        Self::require_flux_range(phase_class, WB11_SYMBOL_ET, et, Some(0.0), None)?;

        let percolation_loss =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_PERC_LOSS_D)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_LOSS_D,
            percolation_loss,
            Some(0.0),
            None,
        )?;

        let subsurface_loss =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_SUBHYD_QD)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_SUBHYD_QD,
            subsurface_loss,
            Some(0.0),
            None,
        )?;

        let storage_reconciled = Self::compute_storage_reconciled_with_interception(
            phase_class,
            storage_initial,
            precip_input,
            snow_coupling_s,
            irrigation_input,
            interception_i,
            q_runoff,
            et,
            percolation_loss,
            subsurface_loss,
        )?;

        let closure_delta = if forward_solver_lane {
            let solver_closure =
                storage_initial + precip_input + snow_coupling_s + irrigation_input
                    - interception_i
                    - q_runoff
                    - et
                    - percolation_loss
                    - subsurface_loss;
            solver_closure - storage_reconciled
        } else {
            let storage_observed =
                Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_OBSERVED)?;
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_STORAGE_OBSERVED,
                storage_observed,
                Some(0.0),
                None,
            )?;
            storage_reconciled - storage_observed
        };
        if closure_delta.abs() > closure_tolerance + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_STORAGE_CLOSURE_DELTA),
                value: closure_delta,
                minimum: Some(-closure_tolerance),
                maximum: Some(closure_tolerance),
            });
        }

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB12-STORAGE-OK-001",
        ) else {
            unreachable!("status message ids are non-empty WB12 constants")
        };
        let writeback = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded(
                WB12_SYMBOL_STORAGE_RECONCILED,
                storage_reconciled,
                Some(0.0),
                None,
            )],
            vec![WritebackField::unbounded(
                WB12_SYMBOL_STORAGE_CLOSURE_DELTA,
                closure_delta,
            )],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
    fn run_erod13_wave1_core(
        request: &HillslopeKernelRequest<'_>,
        q_runoff: f64,
        peakro: f64,
        watdur: f64,
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod13_core_enabled(request)? {
            return Ok(Vec::new());
        }

        let ie_symbol = BoundarySymbol::from(EROD13_SYMBOL_IE);
        let te_symbol = BoundarySymbol::from(EROD13_SYMBOL_TE);
        let fs_symbol = BoundarySymbol::from(EROD13_SYMBOL_FS);
        let ft_symbol = BoundarySymbol::from(EROD13_SYMBOL_FT);
        let taufe_symbol = BoundarySymbol::from(EROD13_SYMBOL_TAUFE);
        let q_symbol = BoundarySymbol::from(EROD13_SYMBOL_Q);
        let g_symbol = BoundarySymbol::from(EROD13_SYMBOL_G);
        let di_symbol = BoundarySymbol::from(EROD13_SYMBOL_DI);
        let beta_symbol = BoundarySymbol::from(EROD13_SYMBOL_BETA);
        let vf_symbol = BoundarySymbol::from(EROD13_SYMBOL_VF);
        let dgdx_symbol = BoundarySymbol::from(EROD13_SYMBOL_DGDX);
        let cntlen_symbol = BoundarySymbol::from(EROD13_SYMBOL_CNTLEN);
        let kr_symbol = BoundarySymbol::from(EROD13_SYMBOL_KR);
        let kradjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_KRADJF);
        let tcadjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCADJF);
        let shrsol_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHRSOL);
        let tcend_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCEND);
        let shcrit_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHCRIT);
        let detinr_symbol = BoundarySymbol::from(EROD13_SYMBOL_DETINR);
        let effdrr_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRR);
        let effdrn_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRN);
        let veleff_symbol = BoundarySymbol::from(EROD13_SYMBOL_VELEFF);
        let pkro_symbol = BoundarySymbol::from(EROD13_SYMBOL_PKRO);
        let tc_k_symbol = BoundarySymbol::from(EROD13_SYMBOL_TC_K);
        let tc_m_symbol = BoundarySymbol::from(EROD13_SYMBOL_TC_M);

        let ie = Self::require_erod13_state_scalar(request, &ie_symbol)?;
        Self::require_erod13_domain(&ie_symbol, ie, Some(0.0), None)?;
        let te = Self::require_erod13_state_scalar(request, &te_symbol)?;
        Self::require_erod13_domain(&te_symbol, te, Some(WB11_ZERO_THRESHOLD), None)?;
        let fs = Self::require_erod13_state_scalar(request, &fs_symbol)?;
        Self::require_erod13_domain(&fs_symbol, fs, Some(0.0), None)?;
        let ft = Self::require_erod13_state_scalar(request, &ft_symbol)?;
        Self::require_erod13_domain(&ft_symbol, ft, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod13_domain(&fs_symbol, fs, Some(0.0), Some(ft))?;
        let taufe = Self::require_erod13_state_scalar(request, &taufe_symbol)?;
        Self::require_erod13_domain(&taufe_symbol, taufe, Some(0.0), None)?;
        let q = Self::require_erod13_state_scalar(request, &q_symbol)?;
        Self::require_erod13_domain(&q_symbol, q, Some(0.0), None)?;
        let g = Self::require_erod13_state_scalar(request, &g_symbol)?;
        Self::require_erod13_domain(&g_symbol, g, Some(0.0), None)?;
        let di = Self::require_erod13_state_scalar(request, &di_symbol)?;
        Self::require_erod13_domain(&di_symbol, di, Some(0.0), None)?;
        let beta = Self::require_erod13_state_scalar(request, &beta_symbol)?;
        Self::require_erod13_domain(&beta_symbol, beta, Some(0.0), None)?;
        let vf = Self::require_erod13_state_scalar(request, &vf_symbol)?;
        Self::require_erod13_domain(&vf_symbol, vf, Some(0.0), None)?;
        let dgdx = Self::require_erod13_state_scalar(request, &dgdx_symbol)?;

        let cntlen = Self::require_erod13_state_scalar(request, &cntlen_symbol)?;
        Self::require_erod13_domain(&cntlen_symbol, cntlen, Some(WB11_ZERO_THRESHOLD), None)?;
        let kr = Self::require_erod13_state_scalar(request, &kr_symbol)?;
        Self::require_erod13_domain(&kr_symbol, kr, Some(WB11_ZERO_THRESHOLD), None)?;
        let kradjf = Self::require_erod13_state_scalar(request, &kradjf_symbol)?;
        Self::require_erod13_domain(&kradjf_symbol, kradjf, Some(WB11_ZERO_THRESHOLD), None)?;
        let tcadjf = Self::require_erod13_state_scalar(request, &tcadjf_symbol)?;
        Self::require_erod13_domain(&tcadjf_symbol, tcadjf, Some(EROD13_MIN_TCADJF), None)?;
        let shrsol = Self::require_erod13_state_scalar(request, &shrsol_symbol)?;
        Self::require_erod13_domain(&shrsol_symbol, shrsol, Some(WB11_ZERO_THRESHOLD), None)?;
        let tcend = Self::require_erod13_state_scalar(request, &tcend_symbol)?;
        Self::require_erod13_domain(&tcend_symbol, tcend, Some(WB11_ZERO_THRESHOLD), None)?;
        let shcrit = Self::require_erod13_state_scalar(request, &shcrit_symbol)?;
        Self::require_erod13_domain(&shcrit_symbol, shcrit, Some(0.0), None)?;
        let detinr = Self::require_erod13_state_scalar(request, &detinr_symbol)?;
        Self::require_erod13_domain(&detinr_symbol, detinr, Some(0.0), None)?;
        let effdrr = Self::require_erod13_state_scalar(request, &effdrr_symbol)?;
        Self::require_erod13_domain(&effdrr_symbol, effdrr, Some(WB11_ZERO_THRESHOLD), None)?;
        let effdrn = Self::require_erod13_state_scalar(request, &effdrn_symbol)?;
        Self::require_erod13_domain(&effdrn_symbol, effdrn, Some(WB11_ZERO_THRESHOLD), None)?;
        let veleff = Self::require_erod13_state_scalar(request, &veleff_symbol)?;
        Self::require_erod13_domain(&veleff_symbol, veleff, Some(0.0), None)?;
        let pkro = Self::require_erod13_state_scalar(request, &pkro_symbol)?;
        Self::require_erod13_domain(&pkro_symbol, pkro, Some(WB11_ZERO_THRESHOLD), None)?;
        let tc_k = Self::require_erod13_state_scalar(request, &tc_k_symbol)?;
        Self::require_erod13_domain(&tc_k_symbol, tc_k, Some(WB11_ZERO_THRESHOLD), None)?;
        let tc_m = Self::require_erod13_state_scalar(request, &tc_m_symbol)?;
        Self::require_erod13_domain(&tc_m_symbol, tc_m, Some(WB11_ZERO_THRESHOLD), None)?;

        Self::require_erod13_domain(
            &BoundarySymbol::from(WB12_SYMBOL_RUNOFF_Q),
            q_runoff,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod13_domain(
            &BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
            peakro,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod13_domain(
            &BoundarySymbol::from(WB16_SYMBOL_WATDUR),
            watdur,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let expected_watdur = q_runoff / peakro;
        let continuity_residual = (watdur - expected_watdur).abs();
        if continuity_residual > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(WB16_SYMBOL_WATDUR),
                value: watdur,
                minimum: Some(expected_watdur - EROD13_CONTINUITY_TOLERANCE),
                maximum: Some(expected_watdur + EROD13_CONTINUITY_TOLERANCE),
            });
        }

        let tau_f = taufe * (fs / ft);
        if !tau_f.is_finite() || tau_f < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: taufe_symbol.clone(),
                value: tau_f,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let eta = (cntlen * kr * kradjf * shrsol) / tcend;
        if !eta.is_finite() || eta < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_ETA),
                value: eta,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let taucn = (tcadjf * shcrit) / shrsol;
        if !taucn.is_finite() || taucn < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_TAUCN),
                value: taucn,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let theta = ((cntlen * detinr) / tcend) * (effdrr / effdrn);
        if !theta.is_finite() || theta < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_THETA),
                value: theta,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let phi = (beta * veleff) / pkro;
        if !phi.is_finite() || phi < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_PHI),
                value: phi,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let tc = tcadjf * tc_k * tau_f.powf(tc_m);
        if !tc.is_finite() || tc < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_TC),
                value: tc,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let (dc, df) = if tau_f > taucn && g < tc {
            if tc <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_TC),
                    value: tc,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            let dc = eta * (tau_f - taucn);
            if !dc.is_finite() || dc < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DC),
                    value: dc,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            let df = dc * ((tc - g) / tc);
            if !df.is_finite() || df < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DF),
                    value: df,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            (dc, df)
        } else if g > tc {
            Self::require_erod13_domain(&q_symbol, q, Some(WB11_ZERO_THRESHOLD), None)?;
            let df = -((beta * vf / q) * (g - tc));
            if !df.is_finite() || df > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DF),
                    value: df,
                    minimum: None,
                    maximum: Some(0.0),
                });
            }
            (0.0, df)
        } else {
            (0.0, 0.0)
        };

        let expected_dgdx = df + di;
        let dgdx_residual = (dgdx - expected_dgdx).abs();
        if dgdx_residual > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: dgdx_symbol,
                value: dgdx,
                minimum: Some(expected_dgdx - EROD13_CONTINUITY_TOLERANCE),
                maximum: Some(expected_dgdx + EROD13_CONTINUITY_TOLERANCE),
            });
        }

        Ok(vec![
            WritebackField::bounded(EROD13_SYMBOL_DC, dc, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_TC, tc, Some(0.0), None),
            WritebackField::unbounded(EROD13_SYMBOL_DF, df),
            WritebackField::bounded(EROD13_SYMBOL_ETA, eta, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_TAUCN, taucn, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_THETA, theta, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_PHI, phi, Some(0.0), None),
        ])
    }

    #[allow(clippy::too_many_lines)]
    fn run_erod14_wave2(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod14_wave2_enabled(request)? {
            return Ok(Vec::new());
        }

        let class_count_symbol = BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT);
        let class_count_value = Self::require_erod14_state_scalar(request, &class_count_symbol)?;
        if class_count_value < 1.0 - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: class_count_symbol,
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count_rounded = class_count_value.round();
        if (class_count_value - class_count_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count = format!("{class_count_rounded:.0}")
            .parse::<usize>()
            .map_err(|_| Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            })?;
        if class_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count_f64 = f64::from(u32::try_from(class_count).map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            }
        })?);

        let xtop_symbol = BoundarySymbol::from(EROD14_SYMBOL_XTOP);
        let xbot_symbol = BoundarySymbol::from(EROD14_SYMBOL_XBOT);
        let xdetst_symbol = BoundarySymbol::from(EROD14_SYMBOL_XDETST);
        let ldtop_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDTOP);
        let ldbot_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDBOT);
        let lddend_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDDEND);
        let qout_symbol = BoundarySymbol::from(EROD14_SYMBOL_QOUT);
        let qin_symbol = BoundarySymbol::from(EROD14_SYMBOL_QIN);
        let qostar_symbol = BoundarySymbol::from(EROD14_SYMBOL_QOSTAR);
        let slplen_symbol = BoundarySymbol::from(EROD14_SYMBOL_SLP_LEN);
        let ktrato_symbol = BoundarySymbol::from(EROD14_SYMBOL_KTRATO);
        let aintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_AINTC);
        let bintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_BINTC);
        let cintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_CINTC);
        let beta_symbol = BoundarySymbol::from(EROD14_SYMBOL_BETA);
        let qj_minus_1_symbol = BoundarySymbol::from(EROD14_SYMBOL_QJ_MINUS_1);
        let vj_symbol = BoundarySymbol::from(EROD14_SYMBOL_VJ);
        let qj_symbol = BoundarySymbol::from(EROD14_SYMBOL_QJ);
        let fh_runon_symbol = BoundarySymbol::from(EROD14_SYMBOL_FH);
        let fp_potential_symbol = BoundarySymbol::from(EROD14_SYMBOL_FP);
        let case_symbol = BoundarySymbol::from(EROD14_SYMBOL_CASE);
        let sumg_symbol = BoundarySymbol::from(EROD14_SYMBOL_SUMG);
        let er_symbol = BoundarySymbol::from(EROD14_SYMBOL_ER);
        let ssa_soil_symbol = BoundarySymbol::from(EROD14_SYMBOL_SSA_SOIL);

        let xtop = Self::require_erod14_state_scalar(request, &xtop_symbol)?;
        let xbot = Self::require_erod14_state_scalar(request, &xbot_symbol)?;
        let xdetst = Self::require_erod14_state_scalar(request, &xdetst_symbol)?;
        let ldtop = Self::require_erod14_state_scalar(request, &ldtop_symbol)?;
        let ldbot = Self::require_erod14_state_scalar(request, &ldbot_symbol)?;
        let lddend = Self::require_erod14_state_scalar(request, &lddend_symbol)?;
        let qout = Self::require_erod14_state_scalar(request, &qout_symbol)?;
        let qin = Self::require_erod14_state_scalar(request, &qin_symbol)?;
        let qostar = Self::require_erod14_state_scalar(request, &qostar_symbol)?;
        let slplen = Self::require_erod14_state_scalar(request, &slplen_symbol)?;
        let ktrato = Self::require_erod14_state_scalar(request, &ktrato_symbol)?;
        let aintc = Self::require_erod14_state_scalar(request, &aintc_symbol)?;
        let bintc = Self::require_erod14_state_scalar(request, &bintc_symbol)?;
        let cintc = Self::require_erod14_state_scalar(request, &cintc_symbol)?;
        let beta = Self::require_erod14_state_scalar(request, &beta_symbol)?;
        let qj_minus_1 = Self::require_erod14_state_scalar(request, &qj_minus_1_symbol)?;
        let vj = Self::require_erod14_state_scalar(request, &vj_symbol)?;
        let qj = Self::require_erod14_state_scalar(request, &qj_symbol)?;
        let fh = Self::require_erod14_state_scalar(request, &fh_runon_symbol)?;
        let fp = Self::require_erod14_state_scalar(request, &fp_potential_symbol)?;
        let case_value = Self::require_erod14_state_scalar(request, &case_symbol)?;
        let ssa_soil = Self::require_erod14_state_scalar(request, &ssa_soil_symbol)?;

        Self::require_erod14_domain(&xtop_symbol, xtop, Some(0.0), None)?;
        Self::require_erod14_domain(&xbot_symbol, xbot, Some(xtop), None)?;
        Self::require_erod14_domain(&xdetst_symbol, xdetst, Some(0.0), Some(xtop))?;
        Self::require_erod14_domain(&ldtop_symbol, ldtop, Some(0.0), None)?;
        Self::require_erod14_domain(&ldbot_symbol, ldbot, Some(0.0), None)?;
        Self::require_erod14_domain(&lddend_symbol, lddend, Some(0.0), Some(ldtop))?;
        Self::require_erod14_domain(&qout_symbol, qout, Some(0.0), None)?;
        Self::require_erod14_domain(&qin_symbol, qin, Some(0.0), None)?;
        Self::require_erod14_domain(&slplen_symbol, slplen, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod14_domain(&ktrato_symbol, ktrato, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod14_domain(&beta_symbol, beta, Some(0.0), None)?;
        Self::require_erod14_domain(&qj_minus_1_symbol, qj_minus_1, Some(0.0), None)?;
        Self::require_erod14_domain(&vj_symbol, vj, Some(0.0), None)?;
        Self::require_erod14_domain(&qj_symbol, qj, Some(0.0), None)?;
        Self::require_erod14_domain(&fh_runon_symbol, fh, Some(0.0), None)?;
        Self::require_erod14_domain(&fp_potential_symbol, fp, Some(0.0), None)?;
        Self::require_erod14_domain(&ssa_soil_symbol, ssa_soil, Some(WB11_ZERO_THRESHOLD), None)?;

        let case_rounded = case_value.round();
        if (case_value - case_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: case_symbol,
                value: case_value,
                minimum: Some(f64::from(EROD14_CASE_MIN)),
                maximum: Some(f64::from(EROD14_CASE_MAX)),
            });
        }
        let case_number = format!("{case_rounded:.0}").parse::<i32>().map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(f64::from(EROD14_CASE_MIN)),
                maximum: Some(f64::from(EROD14_CASE_MAX)),
            }
        })?;
        if !(EROD14_CASE_MIN..=EROD14_CASE_MAX).contains(&case_number) {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(f64::from(EROD14_CASE_MIN)),
                maximum: Some(f64::from(EROD14_CASE_MAX)),
            });
        }

        let case_is_zero = |value: f64| value.abs() <= EROD14_CASE_TOLERANCE;
        let case_matches = match case_number {
            1 => case_is_zero(qj_minus_1) && case_is_zero(vj) && case_is_zero(qj),
            2 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && vj > EROD14_CASE_TOLERANCE
                    && qj > EROD14_CASE_TOLERANCE
            }
            3 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && case_is_zero(vj)
                    && (fh - fp) > EROD14_CASE_TOLERANCE
                    && qj > EROD14_CASE_TOLERANCE
            }
            4 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && case_is_zero(vj)
                    && (fh - fp) <= EROD14_CASE_TOLERANCE
                    && case_is_zero(qj)
            }
            _ => false,
        };
        if !case_matches {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(f64::from(EROD14_CASE_MIN)),
                maximum: Some(f64::from(EROD14_CASE_MAX)),
            });
        }

        let theta_symbol = BoundarySymbol::from(EROD13_SYMBOL_THETA);
        let theta = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_THETA)
        {
            if !value.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                    symbol: theta_symbol,
                    value,
                });
            }
            value
        } else {
            Self::require_erod14_state_scalar(request, &theta_symbol)?
        };
        Self::require_erod14_domain(&theta_symbol, theta, Some(0.0), None)?;

        let mut fall = Vec::with_capacity(class_count);
        let mut frcflw = Vec::with_capacity(class_count);
        let mut fidel = Vec::with_capacity(class_count);
        let mut tcf1 = Vec::with_capacity(class_count);
        let mut ssa_class = Vec::with_capacity(class_count);
        let mut ftheta = Vec::with_capacity(class_count);
        let mut gu = Vec::with_capacity(class_count);
        let mut gend = vec![0.0_f64; class_count];
        let mut sedmax = vec![0.0_f64; class_count];
        let mut sed_frac = vec![0.0_f64; class_count];

        for class_index in 1..=class_count {
            let fall_symbol = Self::erod14_class_symbol(EROD14_ROOT_FALL, class_index);
            let frcflw_symbol = Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index);
            let frac_symbol = Self::erod14_class_symbol(EROD14_ROOT_FRAC, class_index);
            let fidel_symbol = Self::erod14_class_symbol(EROD14_ROOT_FIDEL, class_index);
            let tcf1_symbol = Self::erod14_class_symbol(EROD14_ROOT_TCF1, class_index);
            let ssa_class_symbol = Self::erod14_class_symbol(EROD14_ROOT_SSA_CLASS, class_index);

            let fall_value = Self::require_erod14_state_scalar(request, &fall_symbol)?;
            let frcflw_value = Self::require_erod14_state_scalar(request, &frcflw_symbol)?;
            let frac_value = Self::require_erod14_state_scalar(request, &frac_symbol)?;
            let fidel_value = Self::require_erod14_state_scalar(request, &fidel_symbol)?;
            let tcf1_value = Self::require_erod14_state_scalar(request, &tcf1_symbol)?;
            let ssa_class_value = Self::require_erod14_state_scalar(request, &ssa_class_symbol)?;

            Self::require_erod14_domain(&fall_symbol, fall_value, Some(0.0), None)?;
            Self::require_erod14_domain(&frcflw_symbol, frcflw_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&frac_symbol, frac_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&fidel_symbol, fidel_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&tcf1_symbol, tcf1_value, Some(0.0), None)?;
            Self::require_erod14_domain(
                &ssa_class_symbol,
                ssa_class_value,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            fall.push(fall_value);
            frcflw.push(frcflw_value);
            fidel.push(fidel_value);
            tcf1.push(tcf1_value);
            ssa_class.push(ssa_class_value);
            ftheta.push(fidel_value * theta);
            gu.push(frcflw_value * ldtop);
        }

        if qout <= WB11_ZERO_THRESHOLD {
            for i in 0..class_count {
                frcflw[i] = 0.0;
                sed_frac[i] = 0.0;
            }
            let mut updates = Vec::with_capacity(
                EROD14_BASE_UPDATE_FIELD_COUNT + (class_count * EROD14_CLASS_UPDATE_FIELD_COUNT),
            );
            updates.push(WritebackField::bounded(
                EROD14_SYMBOL_SUMG,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD14_SYMBOL_ER,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_TOTAL_DETACHMENT_KG,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_TOTAL_DEPOSITION_KG,
                lddend.max(0.0),
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_PARTICLE_CLASS_COUNT,
                class_count_f64,
                Some(1.0),
                None,
            ));
            for class_index in 1..=class_count {
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_GEND, class_index),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, class_index),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_SED_FRAC, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(
                        EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3,
                        class_index,
                    ),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD15_ROOT_PARTICLE_FLOW_FRACTION, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
            }
            return Ok(updates);
        }

        let pkro = (qout - qin) / slplen;
        if !pkro.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_QOUT),
                value: pkro,
                minimum: None,
                maximum: None,
            });
        }

        let tmpvr2 = xbot + qostar;
        let tmpvr3 = xtop + qostar;
        if tmpvr2.abs() <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: qostar_symbol,
                value: tmpvr2,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let tmpvr4 = tmpvr2 * tmpvr2;
        let tmpvr5 = tmpvr3 * tmpvr3;

        let mut sumg = 0.0_f64;
        for i in 0..class_count {
            let tmpvr1 = ktrato * tcf1[i];
            let aa = tmpvr1 * aintc;
            let bb = tmpvr1 * bintc;
            let cc = tmpvr1 * cintc;

            let mut phi = if pkro.abs() > EROD14_PKRO_ZERO_THRESHOLD {
                (beta * fall[i]) / pkro
            } else if qostar >= 0.0 {
                EROD14_MAX_PHI
            } else {
                -EROD14_MAX_PHI
            };
            phi = phi.clamp(-EROD14_MAX_PHI, EROD14_MAX_PHI);

            let mut ratio = tmpvr3 / tmpvr2;
            if qostar >= 0.0 && ratio > 1.0 {
                ratio = 1.0;
            }
            if ratio < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_SYMBOL_QOSTAR),
                    value: ratio,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let denom_coef1 = phi + 2.0;
            let denom_coef2 = phi + 1.0;
            if denom_coef1.abs() <= WB11_ZERO_THRESHOLD || denom_coef2.abs() <= WB11_ZERO_THRESHOLD
            {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_FALL, i + 1),
                    value: phi,
                    minimum: Some(-EROD14_MAX_PHI),
                    maximum: Some(EROD14_MAX_PHI),
                });
            }

            let mut attenuation_factor = ratio.powf(phi);
            if !attenuation_factor.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_FALL, i + 1),
                    value: attenuation_factor,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            if attenuation_factor < EROD14_ATTENUATION_FLOOR {
                attenuation_factor = 0.0;
            }

            let coef1 = phi * aa / denom_coef1;
            let coef2 = (phi * bb + ftheta[i] - 2.0 * aa * phi * qostar) / denom_coef2;
            let term1 = coef1 * tmpvr4;
            let term2 = coef2 * tmpvr2;
            let term3 = aa * qostar * qostar - bb * qostar + cc;
            let attenuation_tail = gu[i] - coef1 * tmpvr5 - coef2 * tmpvr3 - term3;
            let mut gend_i = term1 + term2 + term3 + attenuation_factor * attenuation_tail;
            if !gend_i.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                    value: gend_i,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            if gend_i < 0.0 {
                gend_i = 0.0;
            }
            gend[i] = gend_i;
            sumg += gend_i;
        }

        if sumg > 0.0 {
            for i in 0..class_count {
                gend[i] = gend[i] * ldbot / sumg;
                sedmax[i] = gu[i] + ftheta[i] * (xbot - xtop);
                Self::require_erod14_domain(
                    &Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, i + 1),
                    sedmax[i],
                    Some(0.0),
                    None,
                )?;
                if gend[i] < EROD14_CLASS_FLOOR {
                    gend[i] = EROD14_CLASS_FLOOR;
                }
            }

            let mut converged = false;
            for _ in 0..EROD14_MAX_REPROPORTION_ITERS {
                let mut ratbot = 0.0_f64;
                sumg = 0.0;
                let mut adjusted = false;

                for i in 0..class_count {
                    if gend[i] > sedmax[i] + WB11_ZERO_THRESHOLD {
                        gend[i] = sedmax[i];
                        adjusted = true;
                    } else if gend[i] < sedmax[i] - WB11_ZERO_THRESHOLD {
                        ratbot += gend[i];
                    }
                    sumg += gend[i];
                }

                if !adjusted {
                    converged = true;
                    break;
                }

                // Baseline enrich.for semantics: when clipping saturates every class
                // (`ratbot == 0`), re-enter the clipping loop instead of failing.
                if ratbot <= WB11_ZERO_THRESHOLD {
                    continue;
                }

                let gdeficit = ldbot - sumg;
                for i in 0..class_count {
                    if gend[i] < sedmax[i] - WB11_ZERO_THRESHOLD {
                        let gadd = gdeficit * gend[i] / ratbot;
                        let updated = gend[i] + gadd;
                        if !updated.is_finite() {
                            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                                symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                                value: updated,
                                minimum: Some(0.0),
                                maximum: None,
                            });
                        }
                        gend[i] = updated;
                    }
                }
            }

            if !converged {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_SYMBOL_LDBOT),
                    value: ldbot,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
        }

        sumg = gend.iter().sum();
        if !sumg.is_finite() || sumg < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: sumg_symbol,
                value: sumg,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        for i in 0..class_count {
            if gend[i] > sedmax[i] + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                    value: gend[i],
                    minimum: Some(0.0),
                    maximum: Some(sedmax[i]),
                });
            }
        }

        if sumg > 0.0 {
            for i in 0..class_count {
                frcflw[i] = gend[i] / sumg;
                sed_frac[i] = frcflw[i];
            }
            let sed_frac_sum: f64 = sed_frac.iter().sum();
            if (sed_frac_sum - 1.0).abs() > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_ROOT_SED_FRAC),
                    value: sed_frac_sum,
                    minimum: Some(1.0 - EROD13_CONTINUITY_TOLERANCE),
                    maximum: Some(1.0 + EROD13_CONTINUITY_TOLERANCE),
                });
            }
        } else {
            for i in 0..class_count {
                frcflw[i] = 0.0;
                sed_frac[i] = 0.0;
            }
        }

        let mut sumssa = 0.0_f64;
        for i in 0..class_count {
            sumssa += sed_frac[i] * ssa_class[i];
        }
        let er = if sumg > 0.0 {
            (sumssa / ssa_soil) + EROD14_ENRICHMENT_RATIO_OFFSET
        } else {
            0.0
        };
        if !er.is_finite() || er < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: er_symbol,
                value: er,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let mut updates = Vec::with_capacity(
            EROD14_BASE_UPDATE_FIELD_COUNT + (class_count * EROD14_CLASS_UPDATE_FIELD_COUNT),
        );
        updates.push(WritebackField::bounded(
            EROD14_SYMBOL_SUMG,
            sumg.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD14_SYMBOL_ER,
            er,
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_TOTAL_DETACHMENT_KG,
            sumg.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_TOTAL_DEPOSITION_KG,
            lddend.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_PARTICLE_CLASS_COUNT,
            class_count_f64,
            Some(1.0),
            None,
        ));

        for class_index in 1..=class_count {
            let i = class_index - 1;
            let concentration = if qout > WB11_ZERO_THRESHOLD {
                gend[i] / qout
            } else {
                0.0
            };
            if !concentration.is_finite() || concentration < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(
                        EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3,
                        class_index,
                    ),
                    value: concentration,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_GEND, class_index),
                gend[i],
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, class_index),
                sedmax[i],
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index),
                frcflw[i],
                Some(0.0),
                Some(1.0),
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_SED_FRAC, class_index),
                sed_frac[i],
                Some(0.0),
                Some(1.0),
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3, class_index),
                concentration,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD15_ROOT_PARTICLE_FLOW_FRACTION, class_index),
                sed_frac[i],
                Some(0.0),
                Some(1.0),
            ));
        }

        Ok(updates)
    }

    fn erod19_shear(a: f64, b: f64, c: f64, x: f64) -> f64 {
        let mut value = (a * x * x) + (b * x) + c;
        if value < 0.0 {
            value = 0.0;
        }
        let mut shear = value.powf(0.666_666_67);
        if shear <= 0.0 {
            shear = EROD19_SHEAR_FLOOR;
        }
        shear
    }

    fn erod19_root(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
        if a.abs() <= WB11_ZERO_THRESHOLD {
            return None;
        }
        let discriminant = (b * b) + (4.0 * a * c);
        if discriminant < 0.0 {
            return None;
        }
        let part = discriminant.sqrt();
        let two_a = 2.0 * a;
        if two_a.abs() <= WB11_ZERO_THRESHOLD {
            return None;
        }
        let mut x1 = (-b - part) / two_a;
        let mut x2 = (-b + part) / two_a;
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }
        Some((x1, x2))
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
    fn erod19_xcrit_classification(
        a: f64,
        b: f64,
        c: f64,
        tauc: f64,
        xb: f64,
        xe: f64,
    ) -> (f64, f64, f64) {
        let mut xc1 = xb;
        let mut xc2 = xe;
        let mut mshear = 1.0;

        let mut tauchk = tauc.powf(1.5) - c;
        if tauchk < 0.0 {
            tauchk = 0.0;
        }

        let taub = Self::erod19_shear(a, b, c, xb);
        let taue = Self::erod19_shear(a, b, c, xe);

        if a.abs() <= WB11_ZERO_THRESHOLD {
            if b.abs() > WB11_ZERO_THRESHOLD {
                xc1 = tauchk / b;
            } else {
                xc1 = EROD19_UNIFORM_XC_SENTINEL;
            }
            if taue > taub {
                mshear = 3.0;
                if xc1 <= xb {
                    mshear = 2.0;
                }
                if xc1 >= xe {
                    mshear = 1.0;
                }
            } else {
                mshear = 4.0;
                if xc1 >= xe {
                    mshear = 2.0;
                }
                if xc1 <= xb {
                    mshear = 1.0;
                }
            }
        } else if a > 0.0 && taue > taub {
            if taub >= tauc {
                mshear = 2.0;
            } else if taue <= tauc {
                mshear = 1.0;
            } else {
                mshear = 3.0;
                if let Some((x1, x2)) = Self::erod19_root(a, b, tauchk) {
                    if x1 >= xb && x1 <= xe {
                        xc1 = x1;
                    } else if x2 >= xb && x2 <= xe {
                        xc1 = x2;
                    }
                }
            }
        } else if taue >= tauc && taub >= tauc {
            mshear = 2.0;
        } else {
            let part = (b * b) + (4.0 * a * tauchk);
            if part <= 0.0 {
                mshear = 1.0;
            } else if let Some((x1, x2)) = Self::erod19_root(a, b, tauchk) {
                if taub <= tauc && taue >= tauc {
                    mshear = 3.0;
                    xc1 = if x1 <= xb || x1 >= xe { x2 } else { x1 };
                } else if taub >= tauc && taue <= tauc {
                    mshear = 4.0;
                    xc1 = if x1 <= xb || x1 >= xe { x2 } else { x1 };
                } else if taub <= tauc && taue <= tauc {
                    mshear = 5.0;
                    xc1 = x1;
                    xc2 = x2;
                    if x1 < xb
                        || x1 > xe
                        || x2 < xb
                        || x2 > xe
                        || (x1 - x2).abs() <= WB11_ZERO_THRESHOLD
                    {
                        mshear = 1.0;
                    }
                }
            }
        }

        (mshear, xc1.clamp(xb, xe), xc2.clamp(xb, xe))
    }

    #[allow(clippy::too_many_arguments)]
    fn erod19_depc(
        xu: f64,
        a: f64,
        b: f64,
        phi: f64,
        theta: f64,
        du: f64,
        ktrato: f64,
        qostar: f64,
    ) -> f64 {
        if (qostar + xu).abs() >= EROD19_DEPC_QOSTAR_XU_EPSILON {
            du - ((a * ktrato * phi * 2.0 * (qostar + xu)) / (phi + 2.0))
                - (((b * ktrato) - (2.0 * a * ktrato * qostar) - theta) * phi / (phi + 1.0))
        } else {
            0.0
        }
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn erod19_depend(
        xu: f64,
        xl: f64,
        a: f64,
        b: f64,
        cdep: f64,
        phi: f64,
        theta: f64,
        ktrato: f64,
        qostar: f64,
    ) -> f64 {
        let tmpvr1 = 2.0 * a * ktrato;
        let r1 = (phi / (1.0 + phi)) * ((b * ktrato) - theta - (tmpvr1 * qostar));
        let r2 = tmpvr1 * phi / (2.0 + phi);

        let mut xdend;
        if qostar >= 0.0 {
            xdend = xl;
            let denominator = xdend + qostar;
            let mut ratio = if denominator.abs() > WB11_ZERO_THRESHOLD {
                (xu + qostar) / denominator
            } else {
                1.0
            };
            if ratio <= 0.0 {
                ratio = 1.0;
            }
            let expon = 1.0 + phi;
            let f = r1 + (r2 * (xdend + qostar)) + (cdep * ratio.powf(expon));
            if f < 0.0 {
                return xdend;
            }
            xdend = xu + EROD19_DEPEND_INITIAL_STEP_POSITIVE;
            if xdend > xl {
                xdend = f64::midpoint(xu, xl);
            }
        } else {
            if (xu + qostar).abs() <= EROD19_DEPEND_XU_QOSTAR_NEAR_ZERO {
                return -qostar;
            }
            xdend = xu + EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
            if xdend > xl {
                xdend = f64::midpoint(xu, xl);
            }
            let denominator = xdend + qostar;
            let mut ratio = if denominator.abs() > WB11_ZERO_THRESHOLD {
                (xu + qostar) / denominator
            } else {
                1.0
            };
            if ratio <= 0.0 {
                ratio = 1.0;
            }
            let expon = 1.0 + phi;
            let f = r1 + (r2 * (xdend + qostar)) + (cdep * ratio.powf(expon));
            if f >= 0.0 {
                return xdend;
            }
        }

        let mut xmin = xl;
        let mut positive_f_count = 0_u32;
        let mut converged = false;
        for _ in 0..EROD19_DEPEND_NEWTON_MAX_ITERS {
            let tmp = xdend + qostar;
            let mut ratio = if tmp.abs() > WB11_ZERO_THRESHOLD {
                (xu + qostar) / tmp
            } else {
                1.0
            };
            if ratio < 0.0 {
                ratio = 1.0;
            }
            let expon = 1.0 + phi;
            let ratio_pow = ratio.powf(expon);
            let f = r1 + (r2 * (xdend + qostar)) + (cdep * ratio_pow);

            if f > 0.0 && qostar < 0.0 {
                positive_f_count += 1;
                if xdend < xmin {
                    xmin = xdend;
                }
            }

            if f.abs() <= EROD19_DEPEND_NEWTON_RESIDUAL_TOLERANCE {
                converged = true;
                break;
            }

            if tmp.abs() > WB11_ZERO_THRESHOLD {
                let df = r2 - (((1.0 + phi) * cdep * ratio_pow) / tmp);
                if df.abs() > WB11_ZERO_THRESHOLD {
                    xdend -= f / df;
                    if qostar < 0.0 {
                        if xdend < xu {
                            xdend = xu + EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
                        }
                        if xdend > -qostar {
                            xdend = -qostar - EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
                        }
                        if xdend > xl {
                            xdend = xl;
                        }
                    }
                } else {
                    xdend = xu + EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
                }
            }

            if xdend < xu {
                xdend = xu + EROD19_DEPEND_INITIAL_STEP_NEGATIVE;
            }
        }

        if !converged && qostar < 0.0 {
            if positive_f_count == 0 {
                xdend = xl;
            } else {
                xdend = xmin;
            }
        }

        xdend
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
    fn run_erod19_route_segment_migration(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod14_wave2_enabled(request)? {
            return Ok(Vec::new());
        }

        let nslpts_symbol = BoundarySymbol::from(EROD18_SYMBOL_NSLPTS);
        let nslpts_value = Self::require_erod18_state_scalar(request, &nslpts_symbol)?;

        let segment_index_u32 =
            u32::try_from(EROD18_ROUTE_SEGMENT_INDEX).map_err(|_| {
                Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: nslpts_symbol.clone(),
                    value: nslpts_value,
                    minimum: Some(2.0),
                    maximum: None,
                }
            })?;
        let min_segment_value = f64::from(segment_index_u32);
        Self::require_erod18_domain(&nslpts_symbol, nslpts_value, Some(min_segment_value), None)?;

        let nslpts_rounded = nslpts_value.round();
        if (nslpts_value - nslpts_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: nslpts_symbol.clone(),
                value: nslpts_value,
                minimum: Some(min_segment_value),
                maximum: None,
            });
        }
        let nslpts = format!("{nslpts_rounded:.0}").parse::<usize>().map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: nslpts_symbol.clone(),
                value: nslpts_value,
                minimum: Some(min_segment_value),
                maximum: None,
            }
        })?;
        if nslpts < EROD18_ROUTE_SEGMENT_INDEX {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: nslpts_symbol.clone(),
                value: nslpts_value,
                minimum: Some(min_segment_value),
                maximum: None,
            });
        }

        let segment_index = EROD18_ROUTE_SEGMENT_INDEX;
        let xu_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_XU, segment_index);
        let xl_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_XL, segment_index);
        let ainf_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_AINF, segment_index);
        let binf_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_BINF, segment_index);
        let cinf_symbol = Self::erod18_route_segment_symbol(EROD18_ROOT_CINF, segment_index);
        let ainftc_symbol =
            Self::erod18_route_segment_symbol(EROD18_ROOT_AINTC, segment_index);
        let binftc_symbol =
            Self::erod18_route_segment_symbol(EROD18_ROOT_BINTC, segment_index);
        let cinftc_symbol =
            Self::erod18_route_segment_symbol(EROD18_ROOT_CINTC, segment_index);

        let xu = Self::require_erod18_state_scalar(request, &xu_symbol)?;
        let xl = Self::require_erod18_state_scalar(request, &xl_symbol)?;
        let ainf = Self::require_erod18_state_scalar(request, &ainf_symbol)?;
        let binf = Self::require_erod18_state_scalar(request, &binf_symbol)?;
        let cinf = Self::require_erod18_state_scalar(request, &cinf_symbol)?;
        let ainftc = Self::require_erod18_state_scalar(request, &ainftc_symbol)?;
        let binftc = Self::require_erod18_state_scalar(request, &binftc_symbol)?;
        let cinftc = Self::require_erod18_state_scalar(request, &cinftc_symbol)?;

        Self::require_erod18_domain(&xu_symbol, xu, Some(0.0), None)?;
        Self::require_erod18_domain(&xl_symbol, xl, Some(xu), None)?;

        let qostar_symbol = BoundarySymbol::from(EROD18_SYMBOL_QOSTAR);
        let qostar = Self::require_erod18_state_scalar(request, &qostar_symbol)?;

        let xdetst_symbol = BoundarySymbol::from(EROD18_SYMBOL_XDETST);
        let xdetst = Self::require_erod18_state_scalar(request, &xdetst_symbol)?;
        Self::require_erod18_domain(&xdetst_symbol, xdetst, Some(0.0), Some(xl))?;

        let lddend_symbol = BoundarySymbol::from(EROD18_SYMBOL_LDDEND);
        let lddend = Self::require_erod18_state_scalar(request, &lddend_symbol)?;
        Self::require_erod18_domain(&lddend_symbol, lddend, Some(0.0), None)?;

        let ktrato_symbol = BoundarySymbol::from(EROD14_SYMBOL_KTRATO);
        let ktrato = Self::require_erod18_state_scalar(request, &ktrato_symbol)?;
        Self::require_erod18_domain(&ktrato_symbol, ktrato, Some(WB11_ZERO_THRESHOLD), None)?;

        let theta_symbol = BoundarySymbol::from(EROD13_SYMBOL_THETA);
        let theta = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_THETA)
        {
            value
        } else if request.state_surface.contains_key(&theta_symbol) {
            Self::require_erod18_state_scalar(request, &theta_symbol)?
        } else {
            let cntlen_symbol = BoundarySymbol::from(EROD13_SYMBOL_CNTLEN);
            let detinr_symbol = BoundarySymbol::from(EROD13_SYMBOL_DETINR);
            let tcend_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCEND);
            let effdrr_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRR);
            let effdrn_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRN);

            let cntlen = Self::require_erod18_state_scalar(request, &cntlen_symbol)?;
            let detinr = Self::require_erod18_state_scalar(request, &detinr_symbol)?;
            let tcend = Self::require_erod18_state_scalar(request, &tcend_symbol)?;
            let effdrr = Self::require_erod18_state_scalar(request, &effdrr_symbol)?;
            let effdrn = Self::require_erod18_state_scalar(request, &effdrn_symbol)?;

            Self::require_erod18_domain(&cntlen_symbol, cntlen, Some(WB11_ZERO_THRESHOLD), None)?;
            Self::require_erod18_domain(&detinr_symbol, detinr, Some(0.0), None)?;
            Self::require_erod18_domain(&tcend_symbol, tcend, Some(WB11_ZERO_THRESHOLD), None)?;
            Self::require_erod18_domain(&effdrr_symbol, effdrr, Some(WB11_ZERO_THRESHOLD), None)?;
            Self::require_erod18_domain(&effdrn_symbol, effdrn, Some(WB11_ZERO_THRESHOLD), None)?;

            ((cntlen * detinr) / tcend) * (effdrr / effdrn)
        };
        Self::require_erod18_domain(&theta_symbol, theta, Some(0.0), None)?;

        let phi_symbol = BoundarySymbol::from(EROD13_SYMBOL_PHI);
        let phi = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_PHI)
        {
            value
        } else if request.state_surface.contains_key(&phi_symbol) {
            Self::require_erod18_state_scalar(request, &phi_symbol)?
        } else if request
            .state_surface
            .contains_key(&BoundarySymbol::from(EROD14_SYMBOL_BETA))
        {
            let route_beta_symbol = BoundarySymbol::from(EROD14_SYMBOL_BETA);
            let route_beta = Self::require_erod18_state_scalar(request, &route_beta_symbol)?;
            Self::require_erod18_domain(&route_beta_symbol, route_beta, Some(0.0), None)?;
            route_beta
        } else {
            let beta_symbol = BoundarySymbol::from(EROD13_SYMBOL_BETA);
            let veleff_symbol = BoundarySymbol::from(EROD13_SYMBOL_VELEFF);
            let pkro_symbol = BoundarySymbol::from(EROD13_SYMBOL_PKRO);

            let beta = Self::require_erod18_state_scalar(request, &beta_symbol)?;
            let veleff = Self::require_erod18_state_scalar(request, &veleff_symbol)?;
            let pkro = Self::require_erod18_state_scalar(request, &pkro_symbol)?;

            Self::require_erod18_domain(&beta_symbol, beta, Some(0.0), None)?;
            Self::require_erod18_domain(&veleff_symbol, veleff, Some(0.0), None)?;
            Self::require_erod18_domain(&pkro_symbol, pkro, Some(WB11_ZERO_THRESHOLD), None)?;

            (beta * veleff) / pkro
        };
        Self::require_erod18_domain(&phi_symbol, phi, Some(0.0), None)?;
        Self::require_erod18_domain(
            &phi_symbol,
            phi,
            Some(WB11_ZERO_THRESHOLD),
            Some(EROD14_MAX_PHI),
        )?;

        let tauc_symbol = BoundarySymbol::from(EROD13_SYMBOL_TAUCN);
        let tauc = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_TAUCN)
        {
            value
        } else if request.state_surface.contains_key(&tauc_symbol) {
            Self::require_erod18_state_scalar(request, &tauc_symbol)?
        } else if request
            .state_surface
            .contains_key(&BoundarySymbol::from(EROD13_SYMBOL_SHRSOL))
        {
            let tcadjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCADJF);
            let shcrit_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHCRIT);
            let shrsol_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHRSOL);

            let tcadjf = Self::require_erod18_state_scalar(request, &tcadjf_symbol)?;
            let shcrit = Self::require_erod18_state_scalar(request, &shcrit_symbol)?;
            let shrsol = Self::require_erod18_state_scalar(request, &shrsol_symbol)?;

            Self::require_erod18_domain(&tcadjf_symbol, tcadjf, Some(EROD13_MIN_TCADJF), None)?;
            Self::require_erod18_domain(&shcrit_symbol, shcrit, Some(0.0), None)?;
            Self::require_erod18_domain(&shrsol_symbol, shrsol, Some(WB11_ZERO_THRESHOLD), None)?;

            (tcadjf * shcrit) / shrsol
        } else {
            theta * EROD19_TAUC_FALLBACK_SCALE
        };
        Self::require_erod18_domain(&tauc_symbol, tauc, Some(0.0), None)?;

        let g_symbol = BoundarySymbol::from(EROD13_SYMBOL_G);
        let ldlast = if let Some(value) = request.state_surface.get(&g_symbol) {
            let scalar = value.as_f64();
            if !scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod18NonFiniteSymbol {
                    symbol: g_symbol,
                    value: scalar,
                });
            }
            Self::require_erod18_domain(&g_symbol, scalar, Some(0.0), None)?;
            scalar
        } else {
            lddend
        };

        let mut dl = if qostar.abs() < EROD19_QOSTAR_NEAR_ZERO_THRESHOLD {
            (phi / (phi + 1.0)) * ((ktrato * binftc) - theta)
        } else {
            if qostar.abs() <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: qostar_symbol.clone(),
                    value: qostar,
                    minimum: Some(EROD19_QOSTAR_NEAR_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            (phi / qostar) * ((ktrato * cinftc) - ldlast)
        };
        if !dl.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                symbol: qostar_symbol,
                value: dl,
                minimum: None,
                maximum: None,
            });
        }
        let mut du = dl;

        let (mshear, xc1, xc2) = Self::erod19_xcrit_classification(ainf, binf, cinf, tauc, xu, xl);

        let (xdbeg, xdend, ndep, lddend_out, ldlast_out) = if du < 0.0 {
            let cdep = Self::erod19_depc(xu, ainftc, binftc, phi, theta, du, ktrato, qostar);
            if !cdep.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: BoundarySymbol::from(EROD18_SYMBOL_DL),
                    value: cdep,
                    minimum: None,
                    maximum: None,
                });
            }

            let mut xdend =
                Self::erod19_depend(xu, xl, ainftc, binftc, cdep, phi, theta, ktrato, qostar);
            if !xdend.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: BoundarySymbol::from(EROD18_SYMBOL_XDEND),
                    value: xdend,
                    minimum: Some(xu),
                    maximum: Some(xl),
                });
            }
            xdend = xdend.clamp(xu, xl);
            let mut xdbeg = 0.0;
            let mut ndep = 0.0;
            let mut ldlast_out = ldlast;
            let lddend_out;

            if xdend < xl - WB11_ZERO_THRESHOLD {
                let tc_xdend = (((ainftc * xdend * xdend) + (binftc * xdend) + cinftc).max(0.0))
                    * ktrato;
                let tc_xl = (((ainftc * xl * xl) + (binftc * xl) + cinftc).max(0.0)) * ktrato;
                if mshear > EROD18_MSHEAR_MIN + WB11_ZERO_THRESHOLD
                    && ldlast_out > tc_xdend + WB11_ZERO_THRESHOLD
                {
                    ndep = 1.0;
                    xdbeg = xdend;
                    ldlast_out = ldlast_out.min(tc_xl).max(0.0);
                    lddend_out = ldlast_out;
                } else {
                    lddend_out = ldlast_out.max(0.0);
                }
            } else {
                xdend = xl;
                lddend_out = ldlast_out.max(0.0);
            }
            (xdbeg, xdend, ndep, lddend_out, ldlast_out)
        } else {
            dl = 0.0;
            du = 0.0;
            let mut xdbeg = 0.0;
            let xdend = xl;
            let mut ndep = 0.0;
            let mut ldlast_out = ldlast;
            let lddend_out;

            let tc_upper = (ktrato * cinftc).max(0.0);
            let tc_xl = (((ainftc * xl * xl) + (binftc * xl) + cinftc).max(0.0)) * ktrato;
            if ldlast_out > tc_upper + WB11_ZERO_THRESHOLD {
                ndep = 1.0;
                xdbeg = xu;
                ldlast_out = ldlast_out.min(tc_xl).max(0.0);
                lddend_out = ldlast_out;
            } else {
                lddend_out = ldlast_out.max(0.0);
            }
            (xdbeg, xdend, ndep, lddend_out, ldlast_out)
        };

        Self::require_erod18_domain(
            &BoundarySymbol::from(EROD18_SYMBOL_MSHEAR),
            mshear,
            Some(EROD18_MSHEAR_MIN),
            Some(EROD18_MSHEAR_MAX),
        )?;

        let updates = vec![
            WritebackField::bounded(EROD18_SYMBOL_NSLPTS, nslpts_value, Some(min_segment_value), None),
            WritebackField::bounded(xu_symbol, xu, Some(0.0), None),
            WritebackField::bounded(xl_symbol, xl, Some(xu), None),
            WritebackField::unbounded(ainf_symbol, ainf),
            WritebackField::unbounded(binf_symbol, binf),
            WritebackField::unbounded(cinf_symbol, cinf),
            WritebackField::unbounded(ainftc_symbol, ainftc),
            WritebackField::unbounded(binftc_symbol, binftc),
            WritebackField::unbounded(cinftc_symbol, cinftc),
            WritebackField::unbounded(EROD18_SYMBOL_QOSTAR, qostar),
            WritebackField::bounded(EROD18_SYMBOL_XDBEG, xdbeg, Some(0.0), None),
            WritebackField::bounded(EROD18_SYMBOL_XDEND, xdend, Some(xu), Some(xl)),
            WritebackField::bounded(EROD18_SYMBOL_XDETST, xdetst, Some(0.0), Some(xl)),
            WritebackField::bounded(EROD18_SYMBOL_LDLAST, ldlast_out, Some(0.0), None),
            WritebackField::bounded(EROD18_SYMBOL_LDDEND, lddend_out, Some(0.0), None),
            WritebackField::unbounded(EROD18_SYMBOL_DU, du),
            WritebackField::unbounded(EROD18_SYMBOL_DL, dl),
            WritebackField::bounded(
                EROD18_SYMBOL_NDEP,
                ndep,
                Some(0.0),
                Some(1.0),
            ),
            WritebackField::bounded(
                EROD18_SYMBOL_MSHEAR,
                mshear,
                Some(EROD18_MSHEAR_MIN),
                Some(EROD18_MSHEAR_MAX),
            ),
            WritebackField::bounded(EROD18_SYMBOL_XC1, xc1, Some(xu), Some(xl)),
            WritebackField::bounded(EROD18_SYMBOL_XC2, xc2, Some(xu), Some(xl)),
        ];

        Ok(updates)
    }

    #[allow(clippy::too_many_lines)]
    fn run_peak_runoff(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPeakRunoff;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;
        if q_runoff < WB16_RUNOFF_NEAR_ZERO_THRESHOLD {
            let wb11_soil_water =
                Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
            Self::require_state_range(
                phase_class,
                WB11_SYMBOL_SOIL_WATER,
                wb11_soil_water,
                Some(0.0),
                None,
            )?;
            let watcon = wb11_soil_water;
            let total_soil = watcon * WB13_DEPTH_TO_MM;
            let soil_water_total = total_soil;

            let Ok(status) = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB16-PEAK-ZERO-001",
            ) else {
                unreachable!("status message ids are non-empty WB16 constants")
            };

            let writeback = KernelWritebackPayload::with_updates(
                vec![
                    WritebackField::bounded(
                        WB16_SYMBOL_PEAKRO,
                        WB16_PEAKRO_FLOOR,
                        Some(WB16_PEAKRO_FLOOR),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_WATDUR,
                        0.0,
                        Some(0.0),
                        Some(WB16_MAX_DURATION_S),
                    ),
                    WritebackField::bounded(WB16_SYMBOL_METHOD_BRANCH, 1.0, Some(1.0), Some(4.0)),
                    WritebackField::bounded(
                        WB16_SYMBOL_TSTAR,
                        0.0,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_QPSTAR,
                        0.0,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_VSTAR,
                        0.0,
                        Some(0.0),
                        Some(1.0),
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                        watcon,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                        total_soil,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                        soil_water_total,
                        Some(0.0),
                        None,
                    ),
                ],
                Vec::new(),
            );
            return Ok(KernelRunResponse::new(status, writeback));
        }

        let hyetograph_point_count = Self::resolve_hyetograph_point_count(request, phase_class)?;
        let (hyetograph_times, hyetograph_intensities) =
            Self::load_hyetograph_series(request, phase_class, hyetograph_point_count)?;
        let effdrr = if hyetograph_times.len() >= 2 {
            hyetograph_times[hyetograph_times.len() - 1] - hyetograph_times[0]
        } else {
            0.0
        };
        if !effdrr.is_finite() || effdrr <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("timem_0001"),
                value: effdrr,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let vave = q_runoff / effdrr;
        if !vave.is_finite() || vave <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_Q),
                value: vave,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let irrigation_rate_m_per_s =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_RUNTIME_RATE_MPS)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_RATE_MPS,
            irrigation_rate_m_per_s,
            Some(0.0),
            None,
        )?;

        let interception_i =
            Self::require_flux_scalar(request, phase_class, WB15_SYMBOL_INTERCEPTION_I)?;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception_i,
            Some(0.0),
            None,
        )?;

        let efflen = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EFFLEN)?;
        if efflen <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EFFLEN),
                value: efflen,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let ealpha = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EALPHA)?;
        if ealpha <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EALPHA),
                value: ealpha,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let exponent_m = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EXPONENT_M)?;
        if exponent_m <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EXPONENT_M),
                value: exponent_m,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let remax = hyetograph_intensities
            .iter()
            .copied()
            .fold(0.0_f64, f64::max)
            + irrigation_rate_m_per_s;
        if !remax.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("intsty_0001"),
                value: remax,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if remax <= WB11_ZERO_THRESHOLD {
            let wb11_soil_water =
                Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
            Self::require_state_range(
                phase_class,
                WB11_SYMBOL_SOIL_WATER,
                wb11_soil_water,
                Some(0.0),
                None,
            )?;
            let watcon = wb11_soil_water;
            let total_soil = watcon * WB13_DEPTH_TO_MM;
            let soil_water_total = total_soil;

            let Ok(status) = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB16-PEAK-ZERO-002",
            ) else {
                unreachable!("status message ids are non-empty WB16 constants")
            };

            let writeback = KernelWritebackPayload::with_updates(
                vec![
                    WritebackField::bounded(
                        WB16_SYMBOL_PEAKRO,
                        WB16_PEAKRO_FLOOR,
                        Some(WB16_PEAKRO_FLOOR),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_WATDUR,
                        0.0,
                        Some(0.0),
                        Some(WB16_MAX_DURATION_S),
                    ),
                    WritebackField::bounded(WB16_SYMBOL_METHOD_BRANCH, 1.0, Some(1.0), Some(4.0)),
                    WritebackField::bounded(
                        WB16_SYMBOL_TSTAR,
                        0.0,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_QPSTAR,
                        0.0,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_VSTAR,
                        0.0,
                        Some(0.0),
                        Some(1.0),
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                        watcon,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                        total_soil,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                        soil_water_total,
                        Some(0.0),
                        None,
                    ),
                ],
                Vec::new(),
            );
            return Ok(KernelRunResponse::new(status, writeback));
        }

        let vstar = vave / remax;
        if !vstar.is_finite() || vstar <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                value: vstar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let vave_power = vave.powf(exponent_m - 1.0);
        let te_base = efflen / (ealpha * vave_power);
        if !te_base.is_finite() || te_base <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EFFLEN),
                value: te_base,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let te = te_base.powf(1.0 / exponent_m);
        let tstar = te / effdrr;
        if !tstar.is_finite() || tstar <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_TSTAR),
                value: tstar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let (method_branch, qpstar) = if tstar >= 1.0 {
            (1.0, 1.0 / tstar.powf(exponent_m))
        } else if vstar < 1.0 {
            let tc_discriminant = 1.0 - (2.4 * (1.0 - vstar) * vstar);
            if !tc_discriminant.is_finite() || tc_discriminant < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                    value: tc_discriminant,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            let tc_denominator = 1.2 * (1.0 - vstar);
            if !tc_denominator.is_finite() || tc_denominator <= 0.0 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                    value: tc_denominator,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            let tc = (1.0 - tc_discriminant.sqrt()) / tc_denominator;
            if !tc.is_finite() || tc <= 0.0 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                    value: tc,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            if tstar > tc {
                (2.0, 1.0 / tstar)
            } else {
                (3.0, (1.0 / vstar) - 0.6 * (((1.0 - vstar) / vstar) * tstar))
            }
        } else {
            (4.0, 1.0)
        };
        if !qpstar.is_finite() || qpstar <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_QPSTAR),
                value: qpstar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let peakro_raw = vave * qpstar;
        if !peakro_raw.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
                value: peakro_raw,
                minimum: None,
                maximum: None,
            });
        }

        let peakro = peakro_raw.max(WB16_PEAKRO_FLOOR);
        if !peakro.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
                value: peakro,
                minimum: None,
                maximum: None,
            });
        }

        let watdur_raw = q_runoff / peakro;
        if !watdur_raw.is_finite() || watdur_raw < 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_WATDUR),
                value: watdur_raw,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let watdur = watdur_raw.min(WB16_MAX_DURATION_S);

        let wb11_soil_water =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            wb11_soil_water,
            Some(0.0),
            None,
        )?;
        let watcon = wb11_soil_water;
        let total_soil = watcon * WB13_DEPTH_TO_MM;
        let soil_water_total = total_soil;

        let erod13_state_updates = Self::run_erod13_wave1_core(request, q_runoff, peakro, watdur)?;
        let erod14_state_updates = Self::run_erod14_wave2(request, &erod13_state_updates)?;
        let erod19_state_updates = Self::run_erod19_route_segment_migration(request, &erod13_state_updates)?;
        let status_message_id = if !erod19_state_updates.is_empty() {
            "HKERNEL-EROD19-ROUTE-OK-001"
        } else if !erod14_state_updates.is_empty() {
            "HKERNEL-EROD14-WAVE2-OK-001"
        } else if !erod13_state_updates.is_empty() {
            "HKERNEL-EROD13-CORE-OK-001"
        } else {
            "HKERNEL-WB16-PEAK-OK-001"
        };

        let Ok(status) = SimulationStatus::ok(SimulationPhase::HillslopeKernel, status_message_id)
        else {
            unreachable!("status message ids are non-empty WB16 constants")
        };

        let mut state_updates = vec![
            WritebackField::bounded(WB16_SYMBOL_PEAKRO, peakro, Some(WB16_PEAKRO_FLOOR), None),
            WritebackField::bounded(
                WB16_SYMBOL_WATDUR,
                watdur,
                Some(0.0),
                Some(WB16_MAX_DURATION_S),
            ),
            WritebackField::bounded(
                WB16_SYMBOL_METHOD_BRANCH,
                method_branch,
                Some(1.0),
                Some(4.0),
            ),
            WritebackField::bounded(WB16_SYMBOL_TSTAR, tstar, Some(0.0), None),
            WritebackField::bounded(WB16_SYMBOL_QPSTAR, qpstar, Some(0.0), None),
            WritebackField::bounded(
                WB16_SYMBOL_VSTAR,
                vstar,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                watcon,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                total_soil,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                soil_water_total,
                Some(0.0),
                None,
            ),
        ];
        state_updates.extend(erod13_state_updates);
        state_updates.extend(erod14_state_updates);
        state_updates.extend(erod19_state_updates);

        let writeback = KernelWritebackPayload::with_updates(state_updates, Vec::new());
        Ok(KernelRunResponse::new(status, writeback))
    }
}
