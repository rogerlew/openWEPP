#[allow(clippy::wildcard_imports)]
use super::super::*;

impl Wb11HydrologyKernel {
    pub(crate) fn require_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                phase_class,
                symbol: key,
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        Ok(scalar)
    }

    pub(crate) fn require_flux_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionFluxSymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.flux_surface.get(&key) else {
            return Err(Wb11HydrologyKernelGuardError::MissingRequiredFluxSymbol {
                phase_class,
                symbol: key,
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteFluxSymbol {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    pub(crate) fn optional_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.state_surface.get(&key) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    pub(crate) fn optional_flux_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionFluxSymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.flux_surface.get(&key) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteFluxSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    pub(crate) fn optional_flux_scalar_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.flux_surface.get(symbol) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteFluxSymbol {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    pub(crate) fn require_flux_scalar_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.flux_surface.get(symbol) else {
            return Err(Wb11HydrologyKernelGuardError::MissingRequiredFluxSymbol {
                phase_class,
                symbol: symbol.clone(),
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteFluxSymbol {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    pub(crate) fn optional_state_scalar_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    pub(crate) fn require_state_scalar_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                phase_class,
                symbol: symbol.clone(),
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    pub(crate) fn require_state_scalar_for_preferred_or_legacy_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        preferred_symbol: &BoundarySymbol,
        legacy_symbol: &BoundarySymbol,
    ) -> Result<(BoundarySymbol, f64), Wb11HydrologyKernelGuardError> {
        if request.state_surface.contains_key(preferred_symbol) {
            return Self::require_state_scalar_for_symbol(request, phase_class, preferred_symbol)
                .map(|value| (preferred_symbol.clone(), value));
        }
        Self::require_state_scalar_for_symbol(request, phase_class, legacy_symbol)
            .map(|value| (legacy_symbol.clone(), value))
    }

    pub(crate) fn optional_state_scalar_for_preferred_or_legacy_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        preferred_symbol: &BoundarySymbol,
        legacy_symbol: &BoundarySymbol,
    ) -> Result<Option<(BoundarySymbol, f64)>, Wb11HydrologyKernelGuardError> {
        if request.state_surface.contains_key(preferred_symbol) {
            return Self::optional_state_scalar_for_symbol(request, phase_class, preferred_symbol)
                .map(|value| value.map(|scalar| (preferred_symbol.clone(), scalar)));
        }
        Self::optional_state_scalar_for_symbol(request, phase_class, legacy_symbol)
            .map(|value| value.map(|scalar| (legacy_symbol.clone(), scalar)))
    }

    pub(crate) fn hourly_symbol(root: &str, hour: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{hour:04}"))
    }

    pub(crate) fn unit_conversion_guard_error(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        error: &openwepp_unit_boundary::BoundaryError,
    ) -> Wb11HydrologyKernelGuardError {
        match error {
            openwepp_unit_boundary::BoundaryError::NonFinite { value, .. } => {
                Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol,
                    value: *value,
                }
            }
            openwepp_unit_boundary::BoundaryError::BelowMinimum { value, minimum, .. } => {
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol,
                    value: *value,
                    minimum: Some(*minimum),
                    maximum: None,
                }
            }
            openwepp_unit_boundary::BoundaryError::AboveMaximum { value, maximum, .. } => {
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol,
                    value: *value,
                    minimum: None,
                    maximum: Some(*maximum),
                }
            }
        }
    }

    pub(crate) fn typed_water_depth_writeback_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
    ) -> Result<BoundaryValue, Wb11HydrologyKernelGuardError> {
        BoundaryValue::water_depth_meters(value)
            .map_err(|error| Self::unit_conversion_guard_error(phase_class, symbol.clone(), &error))
    }

    pub(crate) fn typed_density_writeback_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
    ) -> Result<BoundaryValue, Wb11HydrologyKernelGuardError> {
        BoundaryValue::density_kilograms_per_cubic_meter(value)
            .map_err(|error| Self::unit_conversion_guard_error(phase_class, symbol.clone(), &error))
    }

    pub(crate) fn typed_temperature_writeback_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
    ) -> Result<BoundaryValue, Wb11HydrologyKernelGuardError> {
        BoundaryValue::temperature_celsius(value)
            .map_err(|error| Self::unit_conversion_guard_error(phase_class, symbol.clone(), &error))
    }

    pub(crate) fn typed_linear_rate_writeback_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
    ) -> Result<BoundaryValue, Wb11HydrologyKernelGuardError> {
        BoundaryValue::linear_rate_meters_per_second(value)
            .map_err(|error| Self::unit_conversion_guard_error(phase_class, symbol.clone(), &error))
    }

    pub(crate) fn typed_fraction_writeback_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
    ) -> Result<BoundaryValue, Wb11HydrologyKernelGuardError> {
        BoundaryValue::fraction_unit_interval(value)
            .map_err(|error| Self::unit_conversion_guard_error(phase_class, symbol.clone(), &error))
    }

    pub(crate) fn typed_water_depth_writeback_field(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<WritebackField, Wb11HydrologyKernelGuardError> {
        let symbol = symbol.into();
        Ok(WritebackField::bounded(
            symbol.clone(),
            Self::typed_water_depth_writeback_value(phase_class, &symbol, value)?,
            minimum,
            maximum,
        ))
    }

    pub(crate) fn typed_density_writeback_field(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<WritebackField, Wb11HydrologyKernelGuardError> {
        let symbol = symbol.into();
        Ok(WritebackField::bounded(
            symbol.clone(),
            Self::typed_density_writeback_value(phase_class, &symbol, value)?,
            minimum,
            maximum,
        ))
    }

    pub(crate) fn typed_temperature_writeback_field(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<WritebackField, Wb11HydrologyKernelGuardError> {
        let symbol = symbol.into();
        Ok(WritebackField::bounded(
            symbol.clone(),
            Self::typed_temperature_writeback_value(phase_class, &symbol, value)?,
            minimum,
            maximum,
        ))
    }

    pub(crate) fn typed_linear_rate_writeback_field(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<WritebackField, Wb11HydrologyKernelGuardError> {
        let symbol = symbol.into();
        Ok(WritebackField::bounded(
            symbol.clone(),
            Self::typed_linear_rate_writeback_value(phase_class, &symbol, value)?,
            minimum,
            maximum,
        ))
    }

    pub(crate) fn typed_fraction_writeback_field(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Into<BoundarySymbol>,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<WritebackField, Wb11HydrologyKernelGuardError> {
        let symbol = symbol.into();
        Ok(WritebackField::bounded(
            symbol.clone(),
            Self::typed_fraction_writeback_value(phase_class, &symbol, value)?,
            minimum,
            maximum,
        ))
    }

    pub(crate) fn require_hourly_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        root: &str,
        hour: usize,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let symbol = Self::hourly_symbol(root, hour);
        Self::require_state_scalar_for_symbol(request, phase_class, &symbol)
    }

    pub(crate) fn require_dynamic_state_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol,
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol,
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn resolve_wb20_forward_solver_lane_enabled(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from(WB20_SYMBOL_FORWARD_SOLVER_LANE_ENABLED);
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

    pub(crate) fn resolve_mofe_hourly_carry_arrays_enabled(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from(MOFE_HOURLY_CARRY_ARRAYS_ENABLED_SYMBOL);
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

    pub(crate) fn require_mofe_hourly_state_array(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        root: &str,
    ) -> Result<[f64; MOFE_HOURLY_CARRY_ARRAY_COUNT], Wb11HydrologyKernelGuardError> {
        let mut values = [0.0_f64; MOFE_HOURLY_CARRY_ARRAY_COUNT];
        for hour in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
            let symbol = Self::hourly_symbol(root, hour);
            let value = Self::require_state_scalar_for_symbol(request, phase_class, &symbol)?;
            Self::require_state_range_for_symbol(phase_class, &symbol, value, Some(0.0), None)?;
            values[hour - 1] = Self::normalize_non_negative_within_tolerance(value);
        }
        Ok(values)
    }

    pub(crate) fn resolve_mofe_hourly_upstream_carryover(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Option<MofeHourlyUpstreamCarryover>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_mofe_hourly_carry_arrays_enabled(request, phase_class)? {
            return Ok(None);
        }

        let upstream_saturation = Self::require_mofe_hourly_state_array(
            request,
            phase_class,
            MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT,
        )?;
        let upstream_lateral = Self::require_mofe_hourly_state_array(
            request,
            phase_class,
            MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
        )?;
        let upstream_saturation_total: f64 = upstream_saturation.iter().copied().sum();
        let upstream_lateral_total: f64 = upstream_lateral.iter().copied().sum();
        let upstream_total = upstream_saturation_total + upstream_lateral_total;
        let upstream_total = Self::normalize_non_negative_within_tolerance(upstream_total);

        let area_ratio_symbol = BoundarySymbol::from(MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL);
        let area_ratio = if upstream_total > WB11_ZERO_THRESHOLD {
            let value = Self::require_state_scalar_for_symbol(request, phase_class, &area_ratio_symbol)?;
            if value <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: area_ratio_symbol,
                    value,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            value
        } else {
            let value =
                Self::optional_state_scalar_for_symbol(request, phase_class, &area_ratio_symbol)?
                    .unwrap_or(1.0);
            if value <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: area_ratio_symbol,
                    value,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            value
        };

        let surface_runoff = upstream_saturation_total * area_ratio;
        let lateral_runon = upstream_lateral_total * area_ratio;
        for (symbol, value) in [
            (BoundarySymbol::from("UpStrmQ"), surface_runoff),
            (BoundarySymbol::from("SubRIn"), lateral_runon),
        ] {
            if !value.is_finite() || value < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol,
                    value,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
        }
        let carryover = surface_runoff + lateral_runon;
        if !carryover.is_finite() || carryover < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CARRYOVER),
                value: carryover,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let carryover = MofeHourlyUpstreamCarryover {
            surface_runoff: Self::normalize_non_negative_within_tolerance(surface_runoff),
            lateral_runon: Self::normalize_non_negative_within_tolerance(lateral_runon),
        };
        let carryover_total = Self::normalize_non_negative_within_tolerance(carryover.total());

        let carryover_symbol = BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CARRYOVER);
        if let Some(aggregate_carryover) =
            Self::optional_flux_scalar_for_symbol(request, phase_class, &carryover_symbol)?
        {
            Self::require_flux_range_for_symbol(
                phase_class,
                &carryover_symbol,
                aggregate_carryover,
                Some(0.0),
                None,
            )?;
            if (aggregate_carryover - carryover_total).abs() > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: carryover_symbol,
                    value: aggregate_carryover,
                    minimum: Some(carryover_total),
                    maximum: Some(carryover_total),
                });
            }
        }

        Ok(Some(carryover))
    }

    pub(crate) fn resolve_mofe_hourly_current_saturation_carry(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        frost_coupling: Option<&FrostCouplingOutcome>,
    ) -> Result<MofeHourlyCurrentSaturationCarry, Wb11HydrologyKernelGuardError> {
        let mut carry = Self::require_mofe_hourly_state_array(
            request,
            phase_class,
            MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
        )?;

        let theta_symbol = Self::wb18_perc_state_symbol("theta", 1);
        let theta = Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
        Self::require_state_range_for_symbol(phase_class, &theta_symbol, theta, Some(0.0), None)?;

        let upper_limit_symbol = Self::wb18_perc_state_symbol("ul", 1);
        let upper_limit =
            Self::require_state_scalar_for_symbol(request, phase_class, &upper_limit_symbol)?;
        Self::require_state_range_for_symbol(
            phase_class,
            &upper_limit_symbol,
            upper_limit,
            Some(0.0),
            None,
        )?;

        let frozen_water_symbol = Self::wb18_perc_state_symbol("frzw", 1);
        let frozen_water = if let Some(outcome) =
            frost_coupling.filter(|outcome| outcome.ws_frz > WB11_ZERO_THRESHOLD)
        {
            let value = outcome
                .layer_topology_state
                .iter()
                .find(|layer| layer.layer_index == 1)
                .map(|layer| layer.frzw_m)
                .ok_or_else(|| Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: frozen_water_symbol.clone(),
                })?;
            Self::require_state_range_for_symbol(
                phase_class,
                &frozen_water_symbol,
                value,
                Some(0.0),
                None,
            )?;
            value
        } else {
            Self::optional_state_scalar_for_symbol(request, phase_class, &frozen_water_symbol)?
                .unwrap_or(0.0)
        };
        Self::require_state_range_for_symbol(
            phase_class,
            &frozen_water_symbol,
            frozen_water,
            Some(0.0),
            Some(upper_limit),
        )?;

        let effective_upper_limit = (upper_limit - frozen_water).max(0.0);
        let saturation_excess = theta - effective_upper_limit;
        let clipped_top_layer_theta = if saturation_excess > WB11_ZERO_THRESHOLD {
            carry[0] = Self::normalize_non_negative_within_tolerance(carry[0] + saturation_excess);
            if !carry[0].is_finite() || carry[0] < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: Self::hourly_symbol(MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT, 1),
                    value: carry[0],
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            Some(effective_upper_limit)
        } else {
            None
        };

        Ok(MofeHourlyCurrentSaturationCarry {
            values: carry,
            clipped_top_layer_theta,
        })
    }

    pub(crate) fn resolve_runoff_carryover_input(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if let Some(carryover) = Self::resolve_mofe_hourly_upstream_carryover(request, phase_class)?
        {
            return Ok(Self::normalize_non_negative_within_tolerance(
                carryover.total(),
            ));
        }

        let carryover_symbol = BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CARRYOVER);
        if let Some(carryover) =
            Self::optional_flux_scalar_for_symbol(request, phase_class, &carryover_symbol)?
        {
            Self::require_flux_range_for_symbol(
                phase_class,
                &carryover_symbol,
                carryover,
                Some(0.0),
                None,
            )?;
            return Ok(Self::normalize_non_negative_within_tolerance(carryover));
        }

        let runon_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RUNON_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RUNON_INPUT,
            runon_input,
            Some(0.0),
            None,
        )?;
        Ok(Self::normalize_non_negative_within_tolerance(runon_input))
    }

    pub(crate) fn require_state_non_negative_integral_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        let scalar = Self::require_state_scalar_for_symbol(request, phase_class, symbol)?;
        if scalar < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let rounded = scalar.round();
        if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let rounded_text = format!("{rounded:.0}");
        let Ok(parsed_count) = rounded_text.parse::<usize>() else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
            });
        };
        Ok(parsed_count)
    }

    pub(crate) fn require_state_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn require_state_range_for_symbol(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn require_flux_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionFluxSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn require_flux_range_for_symbol(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn optional_erod13_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod13NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    pub(crate) fn require_erod13_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Err(Wb11HydrologyKernelGuardError::Erod13MissingRequiredSymbol {
                symbol: symbol.clone(),
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod13NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    pub(crate) fn require_erod13_domain(
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn resolve_erod13_core_enabled(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let enabled_symbol = BoundarySymbol::from(EROD13_SYMBOL_CORE_ENABLED);
        let Some(value) = Self::optional_erod13_state_scalar(request, &enabled_symbol)? else {
            return Ok(false);
        };
        if value.abs() <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }
        if (value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(true);
        }
        Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
            symbol: enabled_symbol,
            value,
            minimum: Some(0.0),
            maximum: Some(1.0),
        })
    }

    pub(crate) fn optional_erod14_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    pub(crate) fn require_erod14_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Err(Wb11HydrologyKernelGuardError::Erod14MissingRequiredSymbol {
                symbol: symbol.clone(),
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    pub(crate) fn require_erod14_domain(
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn resolve_erod14_wave2_enabled(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let enabled_symbol = BoundarySymbol::from(EROD14_SYMBOL_WAVE2_ENABLED);
        let Some(value) = Self::optional_erod14_state_scalar(request, &enabled_symbol)? else {
            return Ok(false);
        };
        if value.abs() <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }
        if (value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(true);
        }
        Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
            symbol: enabled_symbol,
            value,
            minimum: Some(0.0),
            maximum: Some(1.0),
        })
    }

    pub(crate) fn require_erod18_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Err(Wb11HydrologyKernelGuardError::Erod18MissingRequiredSymbol {
                symbol: symbol.clone(),
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod18NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    pub(crate) fn require_erod18_domain(
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn erod14_class_symbol(root: &str, class_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{class_index:04}"))
    }

    pub(crate) fn erod18_route_segment_symbol(root: &str, segment_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{segment_index:04}"))
    }

    pub(crate) fn extract_state_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        let target = BoundarySymbol::from(symbol);
        fields.iter().find_map(|field| {
            if field.symbol == target {
                Some(field.value.as_f64())
            } else {
                None
            }
        })
    }

    pub(crate) fn wb18_perc_state_symbol(field: &str, layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb18_perc_{field}_{layer_index:04}"))
    }

    pub(crate) fn wb18_perc_flux_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb18_perc_pei_{layer_index:04}"))
    }

    pub(crate) fn wb19_dg_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_dg_{layer_index:04}"))
    }

    pub(crate) fn wb19_legacy_dg_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("dg_{layer_index:04}"))
    }

    pub(crate) fn wb19_coca_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_coca_{layer_index:04}"))
    }

    pub(crate) fn wb19_legacy_coca_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("coca_{layer_index:04}"))
    }

    pub(crate) fn wb19_por_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_por_{layer_index:04}"))
    }

    pub(crate) fn wb19_legacy_por_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("por_{layer_index:04}"))
    }

    pub(crate) fn wb19_thetfc_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_thetfc_{layer_index:04}"))
    }

    pub(crate) fn wb19_legacy_thetfc_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("thetfc_{layer_index:04}"))
    }

    pub(crate) fn wb19_thetdr_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_thetdr_{layer_index:04}"))
    }

    pub(crate) fn wb19_legacy_thetdr_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("thetdr_{layer_index:04}"))
    }

    pub(crate) fn wb19_bulk_density_kg_m3_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb19_bulk_density_kg_m3_{layer_index:04}"))
    }

    pub(crate) fn require_wb19_dg_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<(BoundarySymbol, f64), Wb11HydrologyKernelGuardError> {
        Self::require_state_scalar_for_preferred_or_legacy_symbol(
            request,
            phase_class,
            &Self::wb19_dg_symbol(layer_index),
            &Self::wb19_legacy_dg_symbol(layer_index),
        )
    }

    pub(crate) fn require_wb19_coca_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<(BoundarySymbol, f64), Wb11HydrologyKernelGuardError> {
        Self::require_state_scalar_for_preferred_or_legacy_symbol(
            request,
            phase_class,
            &Self::wb19_coca_symbol(layer_index),
            &Self::wb19_legacy_coca_symbol(layer_index),
        )
    }

    pub(crate) fn require_wb19_por_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<(BoundarySymbol, f64), Wb11HydrologyKernelGuardError> {
        Self::require_state_scalar_for_preferred_or_legacy_symbol(
            request,
            phase_class,
            &Self::wb19_por_symbol(layer_index),
            &Self::wb19_legacy_por_symbol(layer_index),
        )
    }

    pub(crate) fn require_wb19_thetfc_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<(BoundarySymbol, f64), Wb11HydrologyKernelGuardError> {
        Self::require_state_scalar_for_preferred_or_legacy_symbol(
            request,
            phase_class,
            &Self::wb19_thetfc_symbol(layer_index),
            &Self::wb19_legacy_thetfc_symbol(layer_index),
        )
    }

    pub(crate) fn require_wb19_thetdr_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<(BoundarySymbol, f64), Wb11HydrologyKernelGuardError> {
        Self::require_state_scalar_for_preferred_or_legacy_symbol(
            request,
            phase_class,
            &Self::wb19_thetdr_symbol(layer_index),
            &Self::wb19_legacy_thetdr_symbol(layer_index),
        )
    }

    pub(crate) fn require_wb19_bulk_density_kg_m3_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<(BoundarySymbol, f64), Wb11HydrologyKernelGuardError> {
        let symbol = Self::wb19_bulk_density_kg_m3_symbol(layer_index);
        let value = Self::require_state_scalar_for_symbol(request, phase_class, &symbol)?;
        Ok((symbol, value))
    }

    pub(crate) fn optional_wb19_thetdr_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_index: usize,
    ) -> Result<Option<(BoundarySymbol, f64)>, Wb11HydrologyKernelGuardError> {
        Self::optional_state_scalar_for_preferred_or_legacy_symbol(
            request,
            phase_class,
            &Self::wb19_thetdr_symbol(layer_index),
            &Self::wb19_legacy_thetdr_symbol(layer_index),
        )
    }

    pub(crate) fn require_wb11_layer_count(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(BoundarySymbol, usize), Wb11HydrologyKernelGuardError> {
        let preferred_symbol = BoundarySymbol::from("wb11_nsl");
        let legacy_symbol = BoundarySymbol::from("nsl");
        let symbol = if request.state_surface.contains_key(&preferred_symbol) {
            preferred_symbol
        } else {
            legacy_symbol
        };
        let layer_count =
            Self::require_state_non_negative_integral_for_symbol(request, phase_class, &symbol)?;
        Ok((symbol, layer_count))
    }

    pub(crate) fn wb17_layer_flux_symbol(root: &str, layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{layer_index:04}"))
    }

    pub(crate) fn frost_layer_symbol(root: &str, layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{layer_index:04}"))
    }

    pub(crate) fn frost_fine_layer_symbol(
        root: &str,
        layer_index: usize,
        fine_index: usize,
    ) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{layer_index:04}_{fine_index:04}"))
    }

    pub(crate) fn resolve_frozen_soil_kfactor(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        kfactor1: f64,
        kfactor2: f64,
        kfactor3: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from(FROST_LANDUSE_CLASS_PROXY_SYMBOL);
        let Some(class_proxy) =
            Self::optional_state_scalar_for_symbol(request, phase_class, &symbol)?
        else {
            return Ok(kfactor1.min(kfactor2.min(kfactor3)));
        };

        let rounded = class_proxy.round();
        if (class_proxy - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value: class_proxy,
                minimum: Some(1.0),
                maximum: Some(3.0),
            });
        }

        let class_text = format!("{rounded:.0}");
        let class_code =
            class_text
                .parse::<i32>()
                .map_err(|_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(FROST_LANDUSE_CLASS_PROXY_SYMBOL),
                    value: class_proxy,
                    minimum: Some(1.0),
                    maximum: Some(3.0),
                })?;

        match class_code {
            1 => Ok(kfactor1),
            2 => Ok(kfactor2),
            3 => Ok(kfactor3),
            _ => Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(FROST_LANDUSE_CLASS_PROXY_SYMBOL),
                value: class_proxy,
                minimum: Some(1.0),
                maximum: Some(3.0),
            }),
        }
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::type_complexity)]
    pub(crate) fn wb19_load_layer_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), Wb11HydrologyKernelGuardError>
    {
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

        let mut theta = Vec::with_capacity(layer_count);
        let mut drain_threshold = Vec::with_capacity(layer_count);
        let mut conductivity = Vec::with_capacity(layer_count);
        let mut thickness = Vec::with_capacity(layer_count);
        let mut upper_limit = Vec::with_capacity(layer_count);

        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index);
            let (dg_symbol, layer_dg) =
                Self::require_wb19_dg_scalar(request, phase_class, layer_index)?;
            let (coca_symbol, coca) =
                Self::require_wb19_coca_scalar(request, phase_class, layer_index)?;

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
            Self::require_state_range_for_symbol(
                phase_class,
                &ul_symbol,
                layer_ul,
                Some(0.0),
                None,
            )?;
            if layer_ul < layer_fc - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ul_symbol,
                    value: layer_ul,
                    minimum: Some(layer_fc),
                    maximum: None,
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

            if layer_dg <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: dg_symbol,
                    value: layer_dg,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            if coca <= WB11_ZERO_THRESHOLD || coca > 1.0 + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: coca_symbol,
                    value: coca,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: Some(1.0),
                });
            }

            let layer_drain_threshold = layer_fc + ((1.0 - coca) * layer_dg);
            Self::require_state_range_for_symbol(
                phase_class,
                &fc_symbol,
                layer_drain_threshold,
                Some(0.0),
                None,
            )?;

            theta.push(layer_theta);
            drain_threshold.push(layer_drain_threshold);
            conductivity.push(layer_ssc);
            thickness.push(layer_dg);
            upper_limit.push(layer_ul);
        }

        Ok((theta, drain_threshold, conductivity, thickness, upper_limit))
    }

    pub(crate) fn wb19_lateral_ssh_state_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{WB19_SYMBOL_LATERAL_SSH_ROOT}_{layer_index:04}"))
    }

    pub(crate) fn wb19_load_hourly_lateral_conductivity(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_count: usize,
    ) -> Result<Vec<f64>, Wb11HydrologyKernelGuardError> {
        let mut lateral_conductivity = Vec::with_capacity(layer_count);
        for layer_index in 1..=layer_count {
            let symbol = Self::wb19_lateral_ssh_state_symbol(layer_index);
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
            lateral_conductivity.push(value);
        }
        Ok(lateral_conductivity)
    }

    pub(crate) fn wb19_solwpv_mode(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<i32, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from("solwpv");
        let solwpv = Self::require_state_scalar_for_symbol(request, phase_class, &symbol)?;
        if solwpv < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value: solwpv,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let rounded = solwpv.round();
        if (solwpv - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("solwpv"),
                value: solwpv,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let mode_text = format!("{rounded:.0}");
        let mode =
            mode_text
                .parse::<i32>()
                .map_err(|_| Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from("solwpv"),
                    value: solwpv,
                    minimum: Some(0.0),
                    maximum: None,
                })?;
        Ok(mode)
    }

    pub(crate) fn wb19_lateral_drain_lane_substeps(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from(WB19_SYMBOL_LATERAL_DRAIN_LANE_SUBSTEPS);
        let default_substeps =
            if Self::resolve_mofe_hourly_carry_arrays_enabled(request, phase_class)? {
                MOFE_HOURLY_CARRY_ARRAY_COUNT
            } else {
                1
            };
        let lane_substeps_raw = Self::optional_state_scalar_for_symbol(request, phase_class, &symbol)?
            .unwrap_or_else(|| Self::diagnostic_count_to_f64(default_substeps));
        if lane_substeps_raw < 1.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: lane_substeps_raw,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let lane_substeps = lane_substeps_raw.round();
        if (lane_substeps_raw - lane_substeps).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: lane_substeps_raw,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let lane_substeps_text = format!("{lane_substeps:.0}");
        lane_substeps_text.parse::<usize>().map_err(|_| {
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value: lane_substeps_raw,
                minimum: Some(1.0),
                maximum: None,
            }
        })
    }

    pub(crate) fn wb19_drainable_storage(theta: &[f64], drain_threshold: &[f64]) -> f64 {
        theta
            .iter()
            .zip(drain_threshold.iter())
            .map(|(theta_i, threshold_i)| (theta_i - threshold_i).max(0.0))
            .sum()
    }

    pub(crate) fn wb19_frozen_adjusted_lateral_thresholds(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        drain_threshold: &[f64],
    ) -> Result<Vec<f64>, Wb11HydrologyKernelGuardError> {
        let frozen_water =
            Self::wb19_frozen_water_by_layer(request, phase_class, drain_threshold.len())?;
        let mut adjusted_threshold = Vec::with_capacity(drain_threshold.len());
        for (index, threshold_i) in drain_threshold.iter().enumerate() {
            let layer_threshold = (threshold_i - frozen_water[index]).max(0.0);
            if !layer_threshold.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: Self::wb18_perc_state_symbol("frzw", index + 1),
                    value: layer_threshold,
                    minimum: Some(0.0),
                    maximum: Some(*threshold_i),
                });
            }
            adjusted_threshold.push(layer_threshold);
        }
        Ok(adjusted_threshold)
    }

    pub(crate) fn wb19_frozen_water_by_layer(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        layer_count: usize,
    ) -> Result<Vec<f64>, Wb11HydrologyKernelGuardError> {
        let mut frozen_water_by_layer = Vec::with_capacity(layer_count);
        for layer_index in 1..=layer_count {
            let frozen_water_symbol = Self::wb18_perc_state_symbol("frzw", layer_index);
            let frozen_water =
                Self::optional_state_scalar_for_symbol(request, phase_class, &frozen_water_symbol)?
                    .unwrap_or(0.0);
            Self::require_state_range_for_symbol(
                phase_class,
                &frozen_water_symbol,
                frozen_water,
                Some(0.0),
                None,
            )?;
            frozen_water_by_layer.push(frozen_water);
        }
        Ok(frozen_water_by_layer)
    }

    pub(crate) fn wb19_withdraw_top_down(
        theta: &mut [f64],
        drain_threshold: &[f64],
        amount: f64,
        layer_withdrawal: &mut [f64],
    ) -> f64 {
        let mut remaining = amount.max(0.0);
        for (layer_index, (theta_i, threshold_i)) in
            theta.iter_mut().zip(drain_threshold.iter()).enumerate()
        {
            if remaining <= WB11_ZERO_THRESHOLD {
                break;
            }
            let available = (*theta_i - *threshold_i).max(0.0);
            if available <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let withdrawn = available.min(remaining);
            *theta_i -= withdrawn;
            if let Some(layer_withdrawal_i) = layer_withdrawal.get_mut(layer_index) {
                *layer_withdrawal_i += withdrawn;
            }
            remaining -= withdrawn;
        }
        amount.max(0.0) - remaining.max(0.0)
    }

    pub(crate) fn wb19_withdraw_tile_to_surface(
        theta: &mut [f64],
        drain_threshold: &[f64],
        tile_layer_index: usize,
        amount: f64,
    ) -> f64 {
        let mut remaining = amount.max(0.0);
        if theta.is_empty() {
            return 0.0;
        }
        let upper_layer = tile_layer_index.min(theta.len() - 1);
        for layer in (0..=upper_layer).rev() {
            if remaining <= WB11_ZERO_THRESHOLD {
                break;
            }
            let available = (theta[layer] - drain_threshold[layer]).max(0.0);
            if available > WB11_ZERO_THRESHOLD {
                let withdrawn = available.min(remaining);
                theta[layer] -= withdrawn;
                remaining -= withdrawn;
            }
        }
        amount.max(0.0) - remaining.max(0.0)
    }

    pub(crate) fn wb19_apply_soil_water_withdrawal(
        phase_class: HillslopeKernelPhaseClass,
        withdrawal_symbol: HillslopeProductionFluxSymbol,
        soil_water_before: f64,
        realized_withdrawal: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::require_flux_range(
            phase_class,
            withdrawal_symbol,
            realized_withdrawal,
            Some(0.0),
            Some(soil_water_before),
        )?;
        let soil_water_after = soil_water_before - realized_withdrawal;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;
        Ok(soil_water_after)
    }

    pub(crate) fn diagnostic_count_to_f64(value: usize) -> f64 {
        value.to_string().parse::<f64>().unwrap_or(f64::INFINITY)
    }

    pub(crate) fn diagnostic_i64_to_f64(value: i64) -> f64 {
        value.to_string().parse::<f64>().unwrap_or_else(|_| {
            if value.is_negative() {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            }
        })
    }

    pub(crate) fn optional_state_non_negative_integral(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
    ) -> Result<Option<usize>, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.state_surface.get(&key) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        if scalar < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: key,
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let rounded = scalar.round();
        if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: key,
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let rounded_text = format!("{rounded:.0}");
        let Ok(parsed_count) = rounded_text.parse::<usize>() else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: key,
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
            });
        };

        Ok(Some(parsed_count))
    }

    pub(crate) fn resolve_hyetograph_point_count(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        let ninten = Self::optional_state_non_negative_integral(
            request,
            phase_class,
            WB14_SYMBOL_HYETOGRAPH_NINTEN,
        )?;
        let nbrkpt = Self::optional_state_non_negative_integral(
            request,
            phase_class,
            WB14_SYMBOL_HYETOGRAPH_NBRKPT,
        )?;

        let point_count = match (ninten, nbrkpt) {
            (None, None) => {
                return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                });
            }
            (Some(ninten_points), Some(nbrkpt_points)) => {
                if ninten_points > 0 && nbrkpt_points > 0 && ninten_points != nbrkpt_points {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                        value: Self::diagnostic_count_to_f64(ninten_points),
                        minimum: Some(Self::diagnostic_count_to_f64(nbrkpt_points)),
                        maximum: Some(Self::diagnostic_count_to_f64(nbrkpt_points)),
                    });
                }
                ninten_points.max(nbrkpt_points)
            }
            (Some(ninten_points), None) => ninten_points,
            (None, Some(nbrkpt_points)) => nbrkpt_points,
        };

        if point_count > MAX_CLIMATE_FORCING_SERIES_POINTS {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                value: Self::diagnostic_count_to_f64(point_count),
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(
                    MAX_CLIMATE_FORCING_SERIES_POINTS,
                )),
            });
        }

        Ok(point_count)
    }

    pub(crate) fn load_hyetograph_series(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        point_count: usize,
    ) -> Result<(Vec<f64>, Vec<f64>), Wb11HydrologyKernelGuardError> {
        if point_count == 0 {
            return Ok((Vec::new(), Vec::new()));
        }

        let mut times = Vec::with_capacity(point_count);
        let mut intensities = Vec::with_capacity(point_count);

        for index in 1..=point_count {
            let time_symbol = format!("timem_{index:04}");
            let intensity_symbol = format!("intsty_{index:04}");

            let time_key = BoundarySymbol::from(time_symbol.clone());
            let Some(time_value) = request.state_surface.get(&time_key) else {
                return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: time_key,
                });
            };
            let time_scalar = time_value.as_f64();
            if !time_scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(time_symbol.as_str()),
                    value: time_scalar,
                });
            }
            if time_scalar < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(time_symbol.as_str()),
                    value: time_scalar,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            times.push(if time_scalar < 0.0 { 0.0 } else { time_scalar });

            let intensity_key = BoundarySymbol::from(intensity_symbol.clone());
            let Some(intensity_value) = request.state_surface.get(&intensity_key) else {
                return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: intensity_key,
                });
            };
            let intensity_scalar = intensity_value.as_f64();
            if !intensity_scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(intensity_symbol.as_str()),
                    value: intensity_scalar,
                });
            }
            if intensity_scalar < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(intensity_symbol.as_str()),
                    value: intensity_scalar,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            intensities.push(if intensity_scalar < 0.0 {
                0.0
            } else {
                intensity_scalar
            });
        }

        if point_count == 1 && intensities[0] > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("intsty_0001"),
                value: intensities[0],
                minimum: Some(0.0),
                maximum: Some(0.0),
            });
        }

        for index in 1..point_count {
            let previous = times[index - 1];
            let current = times[index];
            if current <= previous + WB11_ZERO_THRESHOLD {
                let symbol = format!("timem_{:04}", index + 1);
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value: current,
                    minimum: Some(previous + WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
        }

        Ok((times, intensities))
    }

}
