impl Wb11HydrologyKernel {
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
}
