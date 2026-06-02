
/// WB11 hydrology production kernel for ET/perc/lateral/drain lanes.
#[derive(Debug, Clone, Default)]
pub struct Wb11HydrologyKernel;

#[derive(Debug, Clone, Copy)]
struct SnowHourlyState {
    hour: usize,
    depth_before_m: f64,
    depth_available_m: f64,
    density_before_kg_m3: f64,
    depth_after_m: f64,
    density_after_kg_m3: f64,
    melt_m: f64,
}

#[derive(Debug, Clone)]
struct SnowCouplingOutcome {
    signed_s: f64,
    accumulation: f64,
    runtime_swe: f64,
    runtime_depth_m: f64,
    runtime_density_kg_m3: f64,
    runtime_settle_day_count: f64,
    hourly_state: Vec<SnowHourlyState>,
}

#[derive(Debug, Clone)]
struct FrostCouplingOutcome {
    dfrost: f64,
    dthaw: f64,
    nft: f64,
    ws_frz: f64,
    infcap_frz: f64,
    soil_water_after_frwatc: Option<f64>,
    frdp_m: f64,
    thdp_m: f64,
    tfrdp_m: f64,
    tthawd_m: f64,
    fgthwd_flag: f64,
    total_fine_layer_count: f64,
    conductivity_tilled_w_m_k: f64,
    conductivity_untilled_w_m_k: f64,
    conductivity_residue_w_m_k: f64,
    hourly_state: [FrostHourlyState; SIMIMPL29_HOURS_PER_DAY],
    layer_topology_state: Vec<FrostLayerTopologyState>,
}

#[derive(Debug, Clone, Copy)]
struct FrostHourlyState {
    hour: usize,
    qsrf_w_m2: f64,
    quf_w_m2: f64,
    ksrf_w_m_k: f64,
    snow_depth_m: f64,
    residue_depth_m: f64,
    tilled_frozen_depth_m: f64,
    untilled_frozen_depth_m: f64,
}

#[derive(Debug, Clone, Copy)]
struct FrostLayerTopologyState {
    layer_index: usize,
    fine_layer_count: usize,
    fine_layer_thickness_m: f64,
}

#[derive(Debug, Clone, Copy)]
enum IrrigationScheduleSource {
    Depletion,
    FixedDate,
}

impl IrrigationScheduleSource {
    const fn as_scalar(self) -> f64 {
        match self {
            Self::Depletion => 1.0,
            Self::FixedDate => 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ActiveIrrigationEvent {
    source: IrrigationScheduleSource,
    event_index: usize,
    system_type: f64,
    depth_m: f64,
    duration_s: f64,
    rate_m_per_s: f64,
}

const SNOW_RUNTIME_DEPTH_M_SYMBOL: &str = "snow.runtime_depth_m";
const SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL: &str = "snow.runtime_density_kg_m3";
const SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL: &str = "snow.runtime_settle_day_count";

const SNOW_HOURLY_DEPTH_BEFORE_ROOT: &str = "snow.hourly.depth_before_m";
const SNOW_HOURLY_DEPTH_AVAILABLE_ROOT: &str = "snow.hourly.depth_available_m";
const SNOW_HOURLY_DENSITY_BEFORE_ROOT: &str = "snow.hourly.density_before_kg_m3";
const SNOW_HOURLY_DEPTH_AFTER_ROOT: &str = "snow.hourly.depth_after_m";
const SNOW_HOURLY_DENSITY_AFTER_ROOT: &str = "snow.hourly.density_after_kg_m3";
const SNOW_HOURLY_MELT_ROOT: &str = "snow.hourly.melt_m";
const SNOW_HOURLY_RAIN_ROOT: &str = "snow.hourly.rain_m";
const SNOW_HOURLY_SNOWFALL_ROOT: &str = "snow.hourly.snowfall_m";

const WINTER_HOURLY_RAD_ROOT: &str = "winter.hourly.rad_mj_m2";
const WINTER_HOURLY_AIR_TEMP_ROOT: &str = "winter.hourly.air_temp_c";
const WINTER_HOURLY_CLOUD_ROOT: &str = "winter.hourly.cloud_fraction";
const FROST_HOURLY_QSRF_ROOT: &str = "frost.hourly.qsrf_w_m2";
const FROST_HOURLY_QUF_ROOT: &str = "frost.hourly.quf_w_m2";
const FROST_HOURLY_KSRF_ROOT: &str = "frost.hourly.ksrf_w_m_k";
const FROST_HOURLY_SNOW_DEPTH_ROOT: &str = "frost.hourly.snow_depth_m";
const FROST_HOURLY_RESIDUE_DEPTH_ROOT: &str = "frost.hourly.residue_depth_m";
const FROST_HOURLY_TILLED_FROZEN_DEPTH_ROOT: &str = "frost.hourly.tilled_frozen_depth_m";
const FROST_HOURLY_UNTILLED_FROZEN_DEPTH_ROOT: &str = "frost.hourly.untilled_frozen_depth_m";
const FROST_RUNTIME_FRDP_M_SYMBOL: &str = "frost.runtime_frdp_m";
const FROST_RUNTIME_THDP_M_SYMBOL: &str = "frost.runtime_thdp_m";
const FROST_RUNTIME_TFRDP_M_SYMBOL: &str = "frost.runtime_tfrdp_m";
const FROST_RUNTIME_TTHAWD_M_SYMBOL: &str = "frost.runtime_tthawd_m";
const FROST_RUNTIME_FGTHWD_FLAG_SYMBOL: &str = "frost.runtime_fgthwd_flag";
const FROST_RUNTIME_TOTAL_FINE_LAYER_COUNT_SYMBOL: &str = "frost.runtime_total_fine_layer_count";
const FROST_RUNTIME_LAYER_FINE_COUNT_ROOT: &str = "frost.runtime_nfine";
const FROST_RUNTIME_LAYER_FINE_THICKNESS_ROOT: &str = "frost.runtime_fine_thickness_m";
const FROST_RUNTIME_CONDUCTIVITY_TILLED_SYMBOL: &str = "frost.runtime_kftill_w_m_k";
const FROST_RUNTIME_CONDUCTIVITY_UNTILLED_SYMBOL: &str = "frost.runtime_kfutil_w_m_k";
const FROST_RUNTIME_CONDUCTIVITY_RESIDUE_SYMBOL: &str = "frost.runtime_kres_w_m_k";
const FROST_RUNTIME_SNOW_DEPTH_SYMBOL: &str = "snow.runtime_depth_m";
const FROST_RUNTIME_RESIDUE_DEPTH_SYMBOL: &str = "frost.runtime_residue_depth_m";
const FROST_LANDUSE_CLASS_PROXY_SYMBOL: &str = "landuse.class_proxy";
const FROST_RUNTIME_TILLAGE_DEPTH_M: f64 = 0.20;
const FROST_RUNTIME_KFTILL_W_M_K: f64 = 1.75;
const FROST_RUNTIME_KFUTIL_W_M_K: f64 = 2.1;
const FROST_RUNTIME_KRES_BASE_W_M_K: f64 = 0.05;
const FROST_RUNTIME_FREEZE_INDEX_SCALE_C: f64 = 6.0;

const SIMIMPL29_HOURS_PER_DAY: usize = 24;
const SIMIMPL29_SNOW_DENSITY_CAP_KG_M3: f64 = 522.0;
const SIMIMPL29_DENSITY_MELT_GATE_KG_M3: f64 = 350.0;
const SIMIMPL29_SNOWPACK_SETTLE_BASE: f64 = 0.041_666_7;
const SIMIMPL29_CANOPY_FACTOR: f64 = 1.0;
const SIMIMPL29_WIND_MEASUREMENT_HEIGHT_M: f64 = 10.0;

impl Wb11HydrologyKernel {
    fn require_state_scalar(
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

    fn require_flux_scalar(
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

    fn optional_state_scalar(
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

    fn optional_flux_scalar(
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

    fn optional_flux_scalar_for_symbol(
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

    fn optional_state_scalar_for_symbol(
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

    fn require_state_scalar_for_symbol(
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

    fn hourly_symbol(root: &str, hour: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{hour:04}"))
    }

    fn require_hourly_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        root: &str,
        hour: usize,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let symbol = Self::hourly_symbol(root, hour);
        Self::require_state_scalar_for_symbol(request, phase_class, &symbol)
    }

    fn require_dynamic_state_range(
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

    fn resolve_wb20_forward_solver_lane_enabled(
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

    fn resolve_mofe_hourly_carry_arrays_enabled(
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

    fn require_mofe_hourly_state_array(
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

    fn resolve_mofe_hourly_upstream_carryover(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
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
        let upstream_total: f64 = upstream_saturation
            .iter()
            .chain(upstream_lateral.iter())
            .copied()
            .sum();
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

        let carryover = upstream_total * area_ratio;
        if !carryover.is_finite() || carryover < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CARRYOVER),
                value: carryover,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let carryover = Self::normalize_non_negative_within_tolerance(carryover);

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
            if (aggregate_carryover - carryover).abs() > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: carryover_symbol,
                    value: aggregate_carryover,
                    minimum: Some(carryover),
                    maximum: Some(carryover),
                });
            }
        }

        Ok(Some(carryover))
    }

    fn resolve_mofe_hourly_current_saturation_carry(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        frost_coupling: Option<&FrostCouplingOutcome>,
    ) -> Result<[f64; MOFE_HOURLY_CARRY_ARRAY_COUNT], Wb11HydrologyKernelGuardError> {
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
        let frozen_water = if frost_coupling.is_some_and(|outcome| {
            outcome.ws_frz > WB11_ZERO_THRESHOLD
        }) {
            let value =
                Self::require_state_scalar_for_symbol(request, phase_class, &frozen_water_symbol)?;
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
        if saturation_excess > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::hourly_symbol(MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT, 1),
                value: saturation_excess,
                minimum: Some(0.0),
                maximum: Some(0.0),
            });
        }

        Ok([0.0_f64; MOFE_HOURLY_CARRY_ARRAY_COUNT])
    }

    fn resolve_runoff_carryover_input(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if let Some(carryover) = Self::resolve_mofe_hourly_upstream_carryover(request, phase_class)?
        {
            return Ok(carryover);
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

    fn require_state_non_negative_integral_for_symbol(
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

    fn require_state_range(
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

    fn require_state_range_for_symbol(
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

    fn require_flux_range(
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

    fn require_flux_range_for_symbol(
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

    fn optional_erod13_state_scalar(
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

    fn require_erod13_state_scalar(
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

    fn require_erod13_domain(
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

    fn resolve_erod13_core_enabled(
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

    fn optional_erod14_state_scalar(
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

    fn require_erod14_state_scalar(
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

    fn require_erod14_domain(
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

    fn resolve_erod14_wave2_enabled(
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

    fn require_erod18_state_scalar(
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

    fn require_erod18_domain(
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

    fn erod14_class_symbol(root: &str, class_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{class_index:04}"))
    }

    fn erod18_route_segment_symbol(root: &str, segment_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{segment_index:04}"))
    }

    fn extract_state_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        let target = BoundarySymbol::from(symbol);
        fields.iter().find_map(|field| {
            if field.symbol == target {
                Some(field.value.as_f64())
            } else {
                None
            }
        })
    }

    fn wb18_perc_state_symbol(field: &str, layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb18_perc_{field}_{layer_index:04}"))
    }

    fn wb18_perc_flux_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb18_perc_pei_{layer_index:04}"))
    }

    fn wb19_dg_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("dg_{layer_index:04}"))
    }

    fn wb19_coca_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("coca_{layer_index:04}"))
    }

    fn wb19_por_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("por_{layer_index:04}"))
    }

    fn wb19_thetfc_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("thetfc_{layer_index:04}"))
    }

    fn wb19_thetdr_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("thetdr_{layer_index:04}"))
    }

    fn frost_layer_symbol(root: &str, layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{layer_index:04}"))
    }

    fn resolve_frozen_soil_kfactor(
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

    fn fallback_hourly_air_temperature_c(tmax: f64, tmin: f64, hour: usize) -> f64 {
        let daily_mean = f64::midpoint(tmax, tmin);
        let daily_amp = (tmax - tmin) / 2.0;
        let phase = (std::f64::consts::TAU / 24.0) * (Self::diagnostic_count_to_f64(hour) - 8.0);
        daily_mean + (daily_amp * phase.sin())
    }

    fn resolve_frost_hourly_air_temperature_c(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        tmax: f64,
        tmin: f64,
        hour: usize,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let symbol = Self::hourly_symbol(WINTER_HOURLY_AIR_TEMP_ROOT, hour);
        let Some(air_temp_c) = Self::optional_state_scalar_for_symbol(request, phase_class, &symbol)?
        else {
            return Ok(Self::fallback_hourly_air_temperature_c(tmax, tmin, hour));
        };
        Ok(air_temp_c)
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::type_complexity)]
    fn wb19_load_layer_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), Wb11HydrologyKernelGuardError>
    {
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
        let mut drain_threshold = Vec::with_capacity(layer_count);
        let mut conductivity = Vec::with_capacity(layer_count);
        let mut thickness = Vec::with_capacity(layer_count);
        let mut upper_limit = Vec::with_capacity(layer_count);

        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index);
            let dg_symbol = Self::wb19_dg_symbol(layer_index);
            let coca_symbol = Self::wb19_coca_symbol(layer_index);

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

            let layer_dg = Self::require_state_scalar_for_symbol(request, phase_class, &dg_symbol)?;
            if layer_dg <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: dg_symbol,
                    value: layer_dg,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            let coca =
                Self::require_state_scalar_for_symbol(request, phase_class, &coca_symbol)?;
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

    fn wb19_solwpv_mode(
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

    fn wb19_lateral_drain_lane_substeps(
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

    fn wb19_drainable_storage(theta: &[f64], drain_threshold: &[f64]) -> f64 {
        theta
            .iter()
            .zip(drain_threshold.iter())
            .map(|(theta_i, threshold_i)| (theta_i - threshold_i).max(0.0))
            .sum()
    }

    fn wb19_withdraw_top_down(theta: &mut [f64], drain_threshold: &[f64], amount: f64) -> f64 {
        let mut remaining = amount.max(0.0);
        for (theta_i, threshold_i) in theta.iter_mut().zip(drain_threshold.iter()) {
            if remaining <= WB11_ZERO_THRESHOLD {
                break;
            }
            let available = (*theta_i - *threshold_i).max(0.0);
            if available <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let withdrawn = available.min(remaining);
            *theta_i -= withdrawn;
            remaining -= withdrawn;
        }
        amount.max(0.0) - remaining.max(0.0)
    }

    fn wb19_withdraw_tile_to_surface(
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

    fn wb19_apply_soil_water_withdrawal(
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

    fn diagnostic_count_to_f64(value: usize) -> f64 {
        value.to_string().parse::<f64>().unwrap_or(f64::INFINITY)
    }

    fn diagnostic_i64_to_f64(value: i64) -> f64 {
        value.to_string().parse::<f64>().unwrap_or_else(|_| {
            if value.is_negative() {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            }
        })
    }

    fn optional_state_non_negative_integral(
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

    fn resolve_hyetograph_point_count(
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

    fn load_hyetograph_series(
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

    fn irrigation_depletion_period_symbol(
        period_index: usize,
        field: HillslopeIrrigationDepletionPeriodField,
    ) -> BoundarySymbol {
        BoundarySymbol::from(format!(
            "irrigation.depletion.period_{period_index:04}.{}",
            field.as_str()
        ))
    }

    fn irrigation_fixeddate_event_symbol(
        event_index: usize,
        field: HillslopeIrrigationFixedDateEventField,
    ) -> BoundarySymbol {
        BoundarySymbol::from(format!(
            "irrigation.fixeddate.event_{event_index:04}.{}",
            field.as_str()
        ))
    }

    fn require_non_negative_integral_state_symbol(
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
        let Ok(parsed) = rounded_text.parse::<usize>() else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
            });
        };
        Ok(parsed)
    }

    fn normalize_irrigation_event(
        phase_class: HillslopeKernelPhaseClass,
        source: IrrigationScheduleSource,
        event_index: usize,
        system_type: f64,
        depth_m: f64,
        rate_m_per_s: f64,
        hyetograph_duration_s: f64,
    ) -> Result<ActiveIrrigationEvent, Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_DEPTH_M,
            depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_RATE_MPS,
            rate_m_per_s,
            Some(0.0),
            None,
        )?;
        if depth_m <= WB11_ZERO_THRESHOLD {
            return Ok(ActiveIrrigationEvent {
                source,
                event_index,
                system_type,
                depth_m: 0.0,
                duration_s: 0.0,
                rate_m_per_s: 0.0,
            });
        }
        if rate_m_per_s <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(IRRIG_SYMBOL_RUNTIME_RATE_MPS),
                value: rate_m_per_s,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let raw_duration = depth_m / rate_m_per_s;
        if !raw_duration.is_finite() || raw_duration <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(IRRIG_SYMBOL_RUNTIME_DURATION_S),
                value: raw_duration,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        if hyetograph_duration_s <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(IRRIG_SYMBOL_RUNTIME_DURATION_S),
                value: hyetograph_duration_s,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let (duration_s, adjusted_rate) = if raw_duration > hyetograph_duration_s {
            (hyetograph_duration_s, depth_m / hyetograph_duration_s)
        } else {
            (raw_duration, rate_m_per_s)
        };

        Ok(ActiveIrrigationEvent {
            source,
            event_index,
            system_type,
            depth_m,
            duration_s,
            rate_m_per_s: adjusted_rate,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn resolve_fixeddate_irrigation_event(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        runtime_day: usize,
        runtime_year: usize,
        hyetograph_duration_s: f64,
    ) -> Result<Option<ActiveIrrigationEvent>, Wb11HydrologyKernelGuardError> {
        let event_count = Self::require_non_negative_integral_state_symbol(
            request,
            phase_class,
            &BoundarySymbol::from(IRRIG_SYMBOL_FIXEDDATE_EVENT_COUNT),
        )?;
        if event_count == 0 {
            return Ok(None);
        }

        let system_type =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_FIXEDDATE_SYSTEM_TYPE)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_FIXEDDATE_SYSTEM_TYPE,
            system_type,
            Some(1.0),
            Some(2.0),
        )?;

        for event_index in 1..=event_count {
            let ofe_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::OfeId,
            );
            let event_ofe = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &ofe_symbol,
            )?;
            if event_ofe != 1 {
                continue;
            }

            let day_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::Day,
            );
            let event_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &day_symbol,
            )?;
            let year_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::Year,
            );
            let event_year = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &year_symbol,
            )?;

            let termination_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::ScheduleTerminationFlag,
            );
            let termination_flag =
                Self::require_state_scalar_for_symbol(request, phase_class, &termination_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_SOURCE,
                termination_flag,
                Some(0.0),
                Some(1.0),
            )?;
            if termination_flag >= 1.0 - WB11_ZERO_THRESHOLD {
                continue;
            }

            if event_day != runtime_day || event_year != runtime_year {
                continue;
            }

            if system_type >= 2.0 - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(IRRIG_SYMBOL_FIXEDDATE_SYSTEM_TYPE),
                    value: system_type,
                    minimum: Some(1.0),
                    maximum: Some(1.0),
                });
            }

            let depth_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::SprinklerDepthMeters,
            );
            let depth_m =
                Self::require_state_scalar_for_symbol(request, phase_class, &depth_symbol)?;
            let rate_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::SprinklerRateMetersPerSecond,
            );
            let base_rate =
                Self::require_state_scalar_for_symbol(request, phase_class, &rate_symbol)?;
            let nozzle_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::SprinklerNozzleFactor,
            );
            let nozzle =
                Self::require_state_scalar_for_symbol(request, phase_class, &nozzle_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_RATE_MPS,
                nozzle,
                Some(0.0),
                None,
            )?;
            let rate_m_per_s = base_rate * nozzle;
            return Ok(Some(Self::normalize_irrigation_event(
                phase_class,
                IrrigationScheduleSource::FixedDate,
                event_index,
                system_type,
                depth_m,
                rate_m_per_s,
                hyetograph_duration_s,
            )?));
        }

        Ok(None)
    }

    #[allow(clippy::too_many_lines)]
    fn resolve_depletion_irrigation_event(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        runtime_day: usize,
        runtime_year: usize,
        hyetograph_duration_s: f64,
    ) -> Result<Option<ActiveIrrigationEvent>, Wb11HydrologyKernelGuardError> {
        let period_count = Self::require_non_negative_integral_state_symbol(
            request,
            phase_class,
            &BoundarySymbol::from(IRRIG_SYMBOL_DEPLETION_PERIOD_COUNT),
        )?;
        if period_count == 0 {
            return Ok(None);
        }

        let system_type =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_DEPLETION_SYSTEM_TYPE)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_DEPLETION_SYSTEM_TYPE,
            system_type,
            Some(1.0),
            Some(2.0),
        )?;

        let min_depth =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_DEPLETION_MIN_DEPTH_M)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_DEPLETION_MIN_DEPTH_M,
            min_depth,
            Some(0.0),
            None,
        )?;
        let max_depth =
            Self::optional_state_scalar(request, phase_class, IRRIG_SYMBOL_DEPLETION_MAX_DEPTH_M)?;
        if let Some(value) = max_depth {
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_DEPLETION_MAX_DEPTH_M,
                value,
                Some(min_depth),
                None,
            )?;
        }

        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        let field_capacity =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_FIELD_CAPACITY)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_FIELD_CAPACITY,
            field_capacity,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let depletion_ratio = soil_water / field_capacity;
        if !depletion_ratio.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("irrigation.depletion.trigger_ratio"),
                value: depletion_ratio,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let runtime_date_key = i64::try_from(runtime_year)
            .unwrap_or(i64::MAX)
            .saturating_mul(1000)
            .saturating_add(i64::try_from(runtime_day).unwrap_or(i64::MAX));

        for period_index in 1..=period_count {
            let element_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::ElementId,
            );
            let element_id = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &element_symbol,
            )?;
            if element_id != 1 {
                continue;
            }

            let start_day_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::StartDoy,
            );
            let start_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &start_day_symbol,
            )?;
            let start_year_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::StartYear,
            );
            let start_year = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &start_year_symbol,
            )?;
            let end_day_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::EndDoy,
            );
            let end_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &end_day_symbol,
            )?;
            let end_year_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::EndYear,
            );
            let end_year = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &end_year_symbol,
            )?;

            let start_key = i64::try_from(start_year)
                .unwrap_or(i64::MAX)
                .saturating_mul(1000)
                .saturating_add(i64::try_from(start_day).unwrap_or(i64::MAX));
            let end_key = i64::try_from(end_year)
                .unwrap_or(i64::MAX)
                .saturating_mul(1000)
                .saturating_add(i64::try_from(end_day).unwrap_or(i64::MAX));
            if end_key < start_key {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from("irrigation.depletion.period_window"),
                    value: Self::diagnostic_i64_to_f64(end_key),
                    minimum: Some(Self::diagnostic_i64_to_f64(start_key)),
                    maximum: None,
                });
            }
            if runtime_date_key < start_key || runtime_date_key > end_key {
                continue;
            }

            let threshold_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::DepletionTriggerRatio,
            );
            let threshold =
                Self::require_state_scalar_for_symbol(request, phase_class, &threshold_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_SOURCE,
                threshold,
                Some(0.0),
                Some(1.0),
            )?;
            if depletion_ratio > threshold + WB11_ZERO_THRESHOLD {
                continue;
            }

            if system_type >= 2.0 - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(IRRIG_SYMBOL_DEPLETION_SYSTEM_TYPE),
                    value: system_type,
                    minimum: Some(1.0),
                    maximum: Some(1.0),
                });
            }

            let depth_ratio_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::SprinklerDepthRatio,
            );
            let depth_ratio =
                Self::require_state_scalar_for_symbol(request, phase_class, &depth_ratio_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_DEPTH_M,
                depth_ratio,
                Some(0.0),
                None,
            )?;
            let depth_cap = max_depth.unwrap_or(min_depth);
            let depth_from_ratio = depth_ratio * depth_cap;
            let depth_m = depth_from_ratio.max(min_depth);

            let rate_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::SprinklerRateMetersPerSecond,
            );
            let base_rate =
                Self::require_state_scalar_for_symbol(request, phase_class, &rate_symbol)?;
            let nozzle_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::SprinklerNozzleFactor,
            );
            let nozzle =
                Self::require_state_scalar_for_symbol(request, phase_class, &nozzle_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_RATE_MPS,
                nozzle,
                Some(0.0),
                None,
            )?;
            let rate_m_per_s = base_rate * nozzle;
            return Ok(Some(Self::normalize_irrigation_event(
                phase_class,
                IrrigationScheduleSource::Depletion,
                period_index,
                system_type,
                depth_m,
                rate_m_per_s,
                hyetograph_duration_s,
            )?));
        }

        Ok(None)
    }

    fn resolve_active_irrigation_event(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_duration_s: f64,
    ) -> Result<Option<ActiveIrrigationEvent>, Wb11HydrologyKernelGuardError> {
        let fixeddate_enabled =
            Self::optional_state_scalar(request, phase_class, IRRIG_SYMBOL_FIXEDDATE_ENABLED)?;
        let depletion_enabled =
            Self::optional_state_scalar(request, phase_class, IRRIG_SYMBOL_DEPLETION_ENABLED)?;

        if fixeddate_enabled.is_none() && depletion_enabled.is_none() {
            return Ok(None);
        }

        let runtime_day = Self::require_non_negative_integral_state_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("day"),
        )?;
        if !(1..=366).contains(&runtime_day) {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("day"),
                value: Self::diagnostic_count_to_f64(runtime_day),
                minimum: Some(1.0),
                maximum: Some(366.0),
            });
        }
        let runtime_year = Self::require_non_negative_integral_state_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("year"),
        )?;

        if fixeddate_enabled.unwrap_or(0.0) >= 1.0 - WB11_ZERO_THRESHOLD {
            if let Some(event) = Self::resolve_fixeddate_irrigation_event(
                request,
                phase_class,
                runtime_day,
                runtime_year,
                hyetograph_duration_s,
            )? {
                return Ok(Some(event));
            }
        }

        if depletion_enabled.unwrap_or(0.0) >= 1.0 - WB11_ZERO_THRESHOLD {
            if let Some(event) = Self::resolve_depletion_irrigation_event(
                request,
                phase_class,
                runtime_day,
                runtime_year,
                hyetograph_duration_s,
            )? {
                return Ok(Some(event));
            }
        }

        Ok(None)
    }

    fn interval_overlap_duration(
        interval_start: f64,
        interval_end: f64,
        active_duration: f64,
    ) -> f64 {
        if active_duration <= 0.0 {
            return 0.0;
        }
        let overlap_start = interval_start.max(0.0);
        let overlap_end = interval_end.min(active_duration);
        (overlap_end - overlap_start).max(0.0)
    }

    fn resolve_active_snow_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT);
        let Some(value) = request.state_surface.get(&key) else {
            return Ok(false);
        };

        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        if !(-WB11_ZERO_THRESHOLD..=1.0 + WB11_ZERO_THRESHOLD).contains(&scalar) {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        let rounded = scalar.round();
        if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        Ok(rounded >= 1.0 - WB11_ZERO_THRESHOLD)
    }

    fn resolve_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT);
        let Some(value) = request.state_surface.get(&key) else {
            return Ok(false);
        };

        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
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
        if rounded < 1.0 - WB11_ZERO_THRESHOLD {
            return Ok(false);
        }

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

        Ok(wint_rounded >= 1.0 - WB11_ZERO_THRESHOLD)
    }

    #[allow(clippy::too_many_lines)]
    fn compute_active_frost_coupling(
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

        let nsl_symbol = BoundarySymbol::from("nsl");
        let layer_count =
            Self::require_state_non_negative_integral_for_symbol(request, phase_class, &nsl_symbol)?;
        if layer_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: nsl_symbol,
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }

        let mut layer_topology_state = Vec::with_capacity(layer_count);
        let mut total_fine_layer_count = 0usize;
        for layer_index in 1..=layer_count {
            let dg_symbol = Self::wb19_dg_symbol(layer_index);
            let dg_m = Self::require_state_scalar_for_symbol(request, phase_class, &dg_symbol)?;
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
                    100.0 / Self::diagnostic_count_to_f64(fine_top_count)
                };
                let dg_mm = dg_m * 1_000.0;
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
            layer_topology_state.push(FrostLayerTopologyState {
                layer_index,
                fine_layer_count,
                fine_layer_thickness_m: dg_m / Self::diagnostic_count_to_f64(fine_layer_count),
            });
        }

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

        let tmax = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let tmin = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;

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
            Some(WB14_FROST_MAX_DEPTH_M),
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
            Some(WB14_FROST_MAX_DEPTH_M),
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
            Some(WB14_FROST_MAX_DEPTH_M),
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
            Some(WB14_FROST_MAX_DEPTH_M),
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

        let freeze_active = tmin <= 0.0 + WB11_ZERO_THRESHOLD;
        let daily_mean_temp_c = f64::midpoint(tmax, tmin);
        let freeze_index = ((0.0 - daily_mean_temp_c) / FROST_RUNTIME_FREEZE_INDEX_SCALE_C)
            .clamp(0.0, 1.0);
        let thaw_index = (daily_mean_temp_c / FROST_RUNTIME_FREEZE_INDEX_SCALE_C).clamp(0.0, 1.0);

        let mut frdp_m = prior_frdp_m;
        let mut thdp_m = prior_thdp_m;
        let mut tfrdp_m = prior_top_frost_depth_m;
        let mut tthawd_m = prior_tthawd_m;
        let mut fgthwd_flag = prior_fgthwd_flag;
        if freeze_active {
            frdp_m = frdp_m.max(WB14_FROST_MAX_DEPTH_M * freeze_index);
            if frdp_m > WB11_ZERO_THRESHOLD {
                thdp_m = 0.0;
                tthawd_m = 0.0;
                tfrdp_m = 0.0;
                fgthwd_flag = 0.0;
            }
        } else if frdp_m > WB11_ZERO_THRESHOLD {
            let thaw_amount = WB14_FROST_MAX_DEPTH_M * thaw_index;
            frdp_m = (frdp_m - thaw_amount).max(0.0);
            thdp_m = (thdp_m + thaw_amount).min(WB14_FROST_MAX_DEPTH_M);
            fgthwd_flag = if frdp_m <= WB11_ZERO_THRESHOLD { 1.0 } else { 0.0 };
            if fgthwd_flag > 0.0 {
                tfrdp_m = 0.0;
                tthawd_m = 0.0;
            }
        }

        let dfrost = frdp_m;
        let dthaw = thdp_m;
        let was_frozen = prior_frdp_m > WB11_ZERO_THRESHOLD;
        let is_frozen = frdp_m > WB11_ZERO_THRESHOLD;
        let nft = if freeze_active && is_frozen && !was_frozen {
            prior_nft + 1.0
        } else {
            prior_nft
        };

        let theta_active = (theta_field_capacity - theta_residual).max(WB11_ZERO_THRESHOLD);
        let ws_frz = dfrost * theta_active;
        let prior_ws_frz = Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_FROST_RUNTIME_WS_FRZ)?
            .unwrap_or(0.0);
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            prior_ws_frz,
            Some(0.0),
            None,
        )?;

        let frwatc_exchange = if ws_frz > prior_ws_frz + WB11_ZERO_THRESHOLD {
            ws_frz - prior_ws_frz
        } else {
            0.0
        };
        let soil_water_after_frwatc = if frwatc_exchange > WB11_ZERO_THRESHOLD {
            Some((soil_water - frwatc_exchange).max(0.0))
        } else {
            None
        };

        let kfactor_selected = Self::resolve_frozen_soil_kfactor(
            request,
            phase_class,
            kfactor1,
            kfactor2,
            kfactor3,
        )?;
        let freeze_fraction = (dfrost / WB14_FROST_MAX_DEPTH_M).clamp(0.0, 1.0);
        let infcap_frz =
            soil_conductivity * (1.0 - freeze_fraction + freeze_fraction * kfactor_selected);

        let tilled_frozen_depth_m = frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
        let untilled_frozen_depth_m = (frdp_m - tilled_frozen_depth_m).max(0.0);
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

        let snow_conductivity_w_m_k = if snow_depth_m > 0.001 && snow_density_kg_m3 > 0.0 {
            let density_kg_m3 = snow_density_kg_m3;
            let base = if density_kg_m3 < 156.0 {
                0.023 + (0.234 * (density_kg_m3 / 1_000.0))
            } else {
                0.138 - 1.01 * (density_kg_m3 / 1_000.0)
                    + 3.233 * (density_kg_m3 / 1_000.0).powi(2)
            };
            (base * ksnowf).max(WB11_ZERO_THRESHOLD)
        } else {
            0.0
        };

        let mut hourly_state = std::array::from_fn(|hour_index| FrostHourlyState {
            hour: hour_index + 1,
            qsrf_w_m2: 0.0,
            quf_w_m2: 0.0,
            ksrf_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            snow_depth_m,
            residue_depth_m,
            tilled_frozen_depth_m,
            untilled_frozen_depth_m,
        });
        for hourly in &mut hourly_state {
            let hourly_air_temp_c = Self::resolve_frost_hourly_air_temperature_c(
                request,
                phase_class,
                tmax,
                tmin,
                hourly.hour,
            )?;
            let surface_temp_c = if snow_depth_m > 0.001 && hourly_air_temp_c > 0.0 {
                0.0
            } else {
                hourly_air_temp_c
            };

            let mut resistance_m2_c_w = 0.0;
            if snow_depth_m > 0.001 && snow_conductivity_w_m_k > WB11_ZERO_THRESHOLD {
                resistance_m2_c_w += snow_depth_m / snow_conductivity_w_m_k;
            }
            if residue_depth_m > 0.001 && conductivity_residue_w_m_k > WB11_ZERO_THRESHOLD {
                resistance_m2_c_w += residue_depth_m / conductivity_residue_w_m_k;
            }
            if tilled_frozen_depth_m > WB11_ZERO_THRESHOLD {
                resistance_m2_c_w += tilled_frozen_depth_m / FROST_RUNTIME_KFTILL_W_M_K;
            }
            if untilled_frozen_depth_m > WB11_ZERO_THRESHOLD {
                resistance_m2_c_w += untilled_frozen_depth_m / FROST_RUNTIME_KFUTIL_W_M_K;
            }

            if resistance_m2_c_w <= WB11_ZERO_THRESHOLD {
                resistance_m2_c_w = 0.5 / FROST_RUNTIME_KFTILL_W_M_K;
            }

            let total_frozen_path_m =
                snow_depth_m + residue_depth_m + tilled_frozen_depth_m + untilled_frozen_depth_m;
            let ksrf_w_m_k = if resistance_m2_c_w > WB11_ZERO_THRESHOLD {
                let path_m = total_frozen_path_m.max(0.005);
                path_m / resistance_m2_c_w
            } else {
                FROST_RUNTIME_KFUTIL_W_M_K
            };
            let flux_w_m2 = surface_temp_c.abs() / resistance_m2_c_w;
            if surface_temp_c <= 0.0 {
                hourly.qsrf_w_m2 = flux_w_m2;
                hourly.quf_w_m2 = 0.0;
            } else {
                hourly.qsrf_w_m2 = 0.0;
                hourly.quf_w_m2 = flux_w_m2;
            }
            hourly.ksrf_w_m_k = ksrf_w_m_k.max(WB11_ZERO_THRESHOLD);
        }

        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DFROST,
            dfrost,
            Some(0.0),
            Some(WB14_FROST_MAX_DEPTH_M),
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DTHAW,
            dthaw,
            Some(0.0),
            Some(WB14_FROST_MAX_DEPTH_M),
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

        Ok(FrostCouplingOutcome {
            dfrost,
            dthaw,
            nft,
            ws_frz,
            infcap_frz,
            soil_water_after_frwatc,
            frdp_m: dfrost,
            thdp_m: dthaw,
            tfrdp_m,
            tthawd_m,
            fgthwd_flag,
            total_fine_layer_count: Self::diagnostic_count_to_f64(total_fine_layer_count),
            conductivity_tilled_w_m_k: FROST_RUNTIME_KFTILL_W_M_K,
            conductivity_untilled_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            conductivity_residue_w_m_k,
            hourly_state,
            layer_topology_state,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_simimpl29_melt_hour(
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
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("cancov"),
            cancov,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_HOURLY_RAIN_ROOT),
            hrrain_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(WINTER_HOURLY_RAD_ROOT),
            hrad_mj_m2,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(WINTER_HOURLY_CLOUD_ROOT),
            cloud_fraction,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("vwind"),
            vwind_m_s,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            snow_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            snow_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        if snow_depth_m <= WB11_ZERO_THRESHOLD || snow_density_kg_m3 <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }

        let hrtef = hrtemp_c * (9.0 / 5.0);
        let hrdtf = tdpt_c * (9.0 / 5.0);

        let amelt = 0.0607 * hrad_mj_m2 * (1.0 - cancov * SIMIMPL29_CANOPY_FACTOR);
        let bmelt = 0.025 / 24.0 * hrtef
            - (0.84 * (1.0 - cloud_fraction)) * (1.0 - cancov * SIMIMPL29_CANOPY_FACTOR) / 24.0;

        let adj = 1.57 * SIMIMPL29_WIND_MEASUREMENT_HEIGHT_M.powf(-1.0 / 6.0);
        let vwmph = (vwind_m_s * 3600.0) / 1609.0;
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

        let rainin = hrrain_m * 39.37;
        let dmelt = if hrdtf > 0.0 {
            0.007 * rainin * hrdtf
        } else {
            0.007 * rainin * hrtef
        };

        let mut wmelt_m = 0.0254 * (amelt + bmelt + cmelt + dmelt);
        if !wmelt_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
                value: wmelt_m,
                minimum: Some(0.0),
                maximum: Some(snow_depth_m),
            });
        }
        if wmelt_m < 0.0 {
            wmelt_m = 0.0;
        }

        let melt_depth_at_snow_density = wmelt_m * 1000.0 / snow_density_kg_m3;
        if melt_depth_at_snow_density >= snow_depth_m {
            wmelt_m = snow_depth_m * (snow_density_kg_m3 / 1000.0);
        }

        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_HOURLY_MELT_ROOT),
            wmelt_m,
            Some(0.0),
            Some(snow_depth_m * (snow_density_kg_m3 / 1000.0)),
        )?;
        Ok(wmelt_m)
    }

    #[allow(clippy::too_many_lines)]
    fn compute_active_snow_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
    ) -> Result<SnowCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_rainfall,
            Some(0.0),
            None,
        )?;

        let _rst = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_RST)?;
        let newsnw = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_NEWSNW)?;
        let ssd = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_SSD)?;
        let runtime_swe =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_RUNTIME_SWE)?;
        let tmax = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let tmin = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;
        let cancov = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_CANCOV)?;
        let vwind = Self::require_state_scalar_for_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("vwind"),
        )?;
        let tdpt = Self::require_state_scalar_for_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("tdpt"),
        )?;

        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_NEWSNW,
            newsnw,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(phase_class, WB14_SYMBOL_SNOW_SSD, ssd, Some(0.0), None)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_RUNTIME_SWE,
            runtime_swe,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_PLANT_CANCOV,
            cancov,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("vwind"),
            vwind,
            Some(0.0),
            None,
        )?;

        if newsnw > ssd + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
                value: newsnw,
                minimum: Some(0.0),
                maximum: Some(ssd),
            });
        }

        let depth_symbol = BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL);
        let density_symbol = BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL);
        let settle_day_count_symbol = BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL);

        let mut runtime_depth_m = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &depth_symbol,
        )?
        .unwrap_or(0.0);
        let mut runtime_density_kg_m3 = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &density_symbol,
        )?
        .unwrap_or(0.0);
        let mut settle_day_count = Self::optional_state_scalar_for_symbol(
            request,
            phase_class,
            &settle_day_count_symbol,
        )?
        .unwrap_or(0.0);

        Self::require_dynamic_state_range(
            phase_class,
            depth_symbol.clone(),
            runtime_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            density_symbol.clone(),
            runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            settle_day_count_symbol.clone(),
            settle_day_count,
            Some(0.0),
            None,
        )?;

        if runtime_depth_m <= WB11_ZERO_THRESHOLD && runtime_swe > WB11_ZERO_THRESHOLD {
            if runtime_density_kg_m3 <= WB11_ZERO_THRESHOLD {
                runtime_density_kg_m3 = newsnw;
            }
            runtime_depth_m = runtime_swe * 1000.0 / runtime_density_kg_m3;
        }
        if runtime_depth_m > WB11_ZERO_THRESHOLD && runtime_density_kg_m3 <= WB11_ZERO_THRESHOLD {
            runtime_density_kg_m3 = newsnw;
        }
        if runtime_depth_m <= WB11_ZERO_THRESHOLD {
            runtime_depth_m = 0.0;
            runtime_density_kg_m3 = 0.0;
        }

        let mut snodep = runtime_depth_m;
        let mut dens = runtime_density_kg_m3;
        let daily_mean_temp = f64::midpoint(tmax, tmin);

        let mut accumulation_water_m = 0.0;
        let mut total_melt_m = 0.0;
        let mut hourly_state = Vec::with_capacity(SIMIMPL29_HOURS_PER_DAY);

        for hour in 1..=SIMIMPL29_HOURS_PER_DAY {
            let hrrain = Self::require_hourly_state_scalar(
                request,
                phase_class,
                SNOW_HOURLY_RAIN_ROOT,
                hour,
            )?;
            let hrsnow = Self::require_hourly_state_scalar(
                request,
                phase_class,
                SNOW_HOURLY_SNOWFALL_ROOT,
                hour,
            )?;
            let hrad_mj_m2 = Self::require_hourly_state_scalar(
                request,
                phase_class,
                WINTER_HOURLY_RAD_ROOT,
                hour,
            )?;
            let hrtemp_c = Self::require_hourly_state_scalar(
                request,
                phase_class,
                WINTER_HOURLY_AIR_TEMP_ROOT,
                hour,
            )?;
            let cloud_fraction = Self::require_hourly_state_scalar(
                request,
                phase_class,
                WINTER_HOURLY_CLOUD_ROOT,
                hour,
            )?;

            Self::require_dynamic_state_range(
                phase_class,
                Self::hourly_symbol(SNOW_HOURLY_RAIN_ROOT, hour),
                hrrain,
                Some(0.0),
                None,
            )?;
            Self::require_dynamic_state_range(
                phase_class,
                Self::hourly_symbol(SNOW_HOURLY_SNOWFALL_ROOT, hour),
                hrsnow,
                Some(0.0),
                None,
            )?;

            if hour == 1 {
                settle_day_count += 1.0;
            }
            if hrsnow > WB11_ZERO_THRESHOLD {
                settle_day_count = 1.0;
            }

            let depth_before_m = snodep.max(0.0);
            let density_before_kg_m3 = dens.max(0.0);
            let depth_available_m = depth_before_m;
            let mut melt_m = 0.0;

            if snodep <= WB11_ZERO_THRESHOLD {
                if hrsnow <= WB11_ZERO_THRESHOLD {
                    snodep = 0.0;
                    dens = 0.0;
                } else {
                    snodep = hrsnow;
                    dens = newsnw;
                }
            } else if daily_mean_temp < 0.0 {
                let mut snodpt = snodep;
                let mut densgt;

                let mut setf = ((-(settle_day_count * 2.0)).exp() * SIMIMPL29_SNOWPACK_SETTLE_BASE)
                    + 1.0;
                if dens > ssd {
                    setf = 1.0;
                }
                densgt = dens * setf;
                if densgt > SIMIMPL29_SNOW_DENSITY_CAP_KG_M3 {
                    densgt = SIMIMPL29_SNOW_DENSITY_CAP_KG_M3;
                }
                if densgt > WB11_ZERO_THRESHOLD {
                    snodpt = snodpt * dens / densgt;
                }

                if hrsnow <= WB11_ZERO_THRESHOLD {
                    snodep = snodpt;
                    dens = densgt;
                } else {
                    snodep = snodpt + hrsnow;
                    if snodep > WB11_ZERO_THRESHOLD {
                        dens = ((densgt * snodpt) + (newsnw * hrsnow)) / snodep;
                    } else {
                        dens = 0.0;
                    }
                }
            } else {
                let snodpt = snodep;
                if hrsnow > WB11_ZERO_THRESHOLD {
                    snodep += hrsnow;
                    if snodep > WB11_ZERO_THRESHOLD {
                        dens = ((dens * snodpt) + (newsnw * hrsnow)) / snodep;
                    } else {
                        dens = 0.0;
                    }
                }

                if snodep > WB11_ZERO_THRESHOLD {
                    let wmelt = Self::compute_simimpl29_melt_hour(
                        phase_class,
                        cancov,
                        hrad_mj_m2,
                        cloud_fraction,
                        hrtemp_c,
                        tdpt,
                        vwind,
                        hrrain,
                        snodep,
                        dens,
                    )?;
                    if wmelt > WB11_ZERO_THRESHOLD {
                        let smelt = (wmelt * 1000.0) / dens;
                        let snodpt_after_inputs = snodep;
                        snodep = snodpt_after_inputs - smelt;
                        if snodep <= WB11_ZERO_THRESHOLD {
                            melt_m = snodpt_after_inputs * dens * 0.001;
                            snodep = 0.0;
                            dens = 0.0;
                        } else if dens >= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
                            melt_m = smelt * dens * 0.001;
                        } else {
                            let mut densgt = dens * (snodpt_after_inputs / snodep);
                            if densgt <= SIMIMPL29_DENSITY_MELT_GATE_KG_M3 {
                                melt_m = 0.0;
                                if hrrain > WB11_ZERO_THRESHOLD {
                                    let densic = 1000.0 * hrrain / snodep;
                                    if densic
                                        <= (SIMIMPL29_DENSITY_MELT_GATE_KG_M3 - densgt)
                                            + WB11_ZERO_THRESHOLD
                                    {
                                        densgt += densic;
                                    } else {
                                        densgt = SIMIMPL29_DENSITY_MELT_GATE_KG_M3;
                                    }
                                }
                            } else {
                                melt_m =
                                    ((densgt - SIMIMPL29_DENSITY_MELT_GATE_KG_M3) * snodep) * 0.001;
                                densgt = SIMIMPL29_DENSITY_MELT_GATE_KG_M3;
                            }
                            dens = densgt;
                        }
                    }
                }
            }

            if dens > SIMIMPL29_SNOW_DENSITY_CAP_KG_M3 {
                dens = SIMIMPL29_SNOW_DENSITY_CAP_KG_M3;
            }
            if snodep <= WB11_ZERO_THRESHOLD {
                snodep = 0.0;
                dens = 0.0;
            }

            accumulation_water_m += hrsnow * 0.1;
            total_melt_m += melt_m.max(0.0);

            hourly_state.push(SnowHourlyState {
                hour,
                depth_before_m,
                depth_available_m,
                density_before_kg_m3,
                depth_after_m: snodep,
                density_after_kg_m3: dens,
                melt_m: melt_m.max(0.0),
            });
        }

        let runtime_swe_after = (snodep * dens) * 0.001;
        let signed_s = total_melt_m - accumulation_water_m;
        if !signed_s.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_SNOW_COUPLING_S),
                value: signed_s,
                minimum: None,
                maximum: None,
            });
        }

        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            runtime_swe_after,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            snodep,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            dens,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        Ok(SnowCouplingOutcome {
            signed_s,
            accumulation: accumulation_water_m,
            runtime_swe: runtime_swe_after,
            runtime_depth_m: snodep,
            runtime_density_kg_m3: dens,
            runtime_settle_day_count: settle_day_count,
            hourly_state,
        })
    }

    fn compute_canopy_interception_depth(
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
            Some(WB15_VDMT_MAX),
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
                maximum: Some(WB15_VDMT_MAX * WB15_BIOMASS_TO_KG_HA),
            });
        }

        let potential_interception = cancov
            * ((WB15_INTERCEPT_LINEAR_COEFF * biomass_kg_ha
                - WB15_INTERCEPT_QUADRATIC_COEFF * biomass_kg_ha.powi(2))
                / WB15_INTERCEPT_MM_TO_M);
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            potential_interception,
            Some(0.0),
            None,
        )?;

        let interception = potential_interception.min(hyetograph_rainfall);
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception,
            Some(0.0),
            Some(hyetograph_rainfall),
        )?;
        Ok(interception)
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_coupled_infiltration_depth(
        phase_class: HillslopeKernelPhaseClass,
        infiltration_conductivity: f64,
        matric_potential: f64,
        times: &[f64],
        intensities: &[f64],
        rainfall_scale: f64,
        irrigation_rate_m_per_s: f64,
        irrigation_duration_s: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut cumulative_infiltration = 0.0_f64;
        for index in 0..times.len().saturating_sub(1) {
            let interval_duration = times[index + 1] - times[index];
            let scaled_rainfall_rate = intensities[index] * rainfall_scale;
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

            let interval_irrigation_duration = Self::interval_overlap_duration(
                times[index],
                times[index + 1],
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

            let interval_liquid_depth = interval_rainfall + interval_irrigation_depth.max(0.0);
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

    fn resolve_interception_rainfall_scale(
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
        interception_rainfall_input: f64,
        interception: f64,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
        let liquid_after_interception = interception_rainfall_input - interception;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            liquid_after_interception,
            Some(0.0),
            Some(interception_rainfall_input),
        )?;

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

    fn require_infiltration_liquid_closure(
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

    fn require_non_negative_liquid_input(
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

    fn normalize_non_negative_within_tolerance(value: f64) -> f64 {
        if (-WB11_ZERO_THRESHOLD..0.0).contains(&value) {
            return 0.0;
        }
        value
    }

    fn compute_runoff_after_interception(
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
    fn compute_storage_reconciled_with_interception(
        phase_class: HillslopeKernelPhaseClass,
        storage_initial: f64,
        precip_input: f64,
        snow_coupling_s: f64,
        irrigation_input: f64,
        interception: f64,
        q_runoff: f64,
        et: f64,
        percolation_loss: f64,
        subsurface_loss: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let storage_reconciled =
            storage_initial + precip_input + snow_coupling_s + irrigation_input
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
