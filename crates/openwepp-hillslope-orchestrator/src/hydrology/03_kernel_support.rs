
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

    fn erod14_class_symbol(root: &str, class_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{class_index:04}"))
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

    #[allow(clippy::type_complexity)]
    fn wb19_load_layer_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), Wb11HydrologyKernelGuardError> {
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
        let mut conductivity = Vec::with_capacity(layer_count);
        let mut thickness = Vec::with_capacity(layer_count);

        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index);
            let dg_symbol = Self::wb19_dg_symbol(layer_index);

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

            theta.push(layer_theta);
            field_capacity.push(layer_fc);
            conductivity.push(layer_ssc);
            thickness.push(layer_dg);
        }

        Ok((theta, field_capacity, conductivity, thickness))
    }

    fn wb19_drainable_storage(theta: &[f64], field_capacity: &[f64]) -> f64 {
        theta
            .iter()
            .zip(field_capacity.iter())
            .map(|(theta_i, fc_i)| (theta_i - fc_i).max(0.0))
            .sum()
    }

    fn wb19_withdraw_top_down(theta: &mut [f64], field_capacity: &[f64], amount: f64) -> f64 {
        let mut remaining = amount.max(0.0);
        for (theta_i, fc_i) in theta.iter_mut().zip(field_capacity.iter()) {
            if remaining <= WB11_ZERO_THRESHOLD {
                break;
            }
            let available = (*theta_i - *fc_i).max(0.0);
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
        field_capacity: &[f64],
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
            let available = (theta[layer] - field_capacity[layer]).max(0.0);
            if available > WB11_ZERO_THRESHOLD {
                let withdrawn = available.min(remaining);
                theta[layer] -= withdrawn;
                remaining -= withdrawn;
            }
        }
        amount.max(0.0) - remaining.max(0.0)
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
        if tmax < tmin - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_TMAX),
                value: tmax,
                minimum: Some(tmin),
                maximum: None,
            });
        }

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

        if tmax < tmin - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_TMAX),
                value: tmax,
                minimum: Some(tmin),
                maximum: None,
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

        let mut per_layer_flux = vec![0.0_f64; layer_count];
        let mut percolation_loss = 0.0_f64;

        // Bottom-up routing mirrors legacy WEPP percolation ordering in PURK.
        for layer_index in (0..layer_count).rev() {
            let layer_theta = theta[layer_index];
            let layer_fc = field_capacity[layer_index];
            let layer_ul = upper_limit[layer_index];
            let layer_ssc = conductivity[layer_index];

            let excess = layer_theta - layer_fc;
            if excess <= WB11_ZERO_THRESHOLD {
                per_layer_flux[layer_index] = 0.0;
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
                stz.powf(WB18_PERC_SHAPE_EXPONENT).max(WB18_PERC_MIN_FX)
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

            let ks_adjusted = layer_ssc * fx;
            let pei_pre = (WB18_PERC_TIMESTEP_S * ks_adjusted).min(excess);
            let pei = if layer_index < layer_count - 1 {
                let lower_ratio = theta[layer_index + 1] / upper_limit[layer_index + 1];
                let lower_radicand = 1.0 - lower_ratio;
                if lower_radicand < -WB11_ZERO_THRESHOLD {
                    let lower_theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 2);
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
                percolation_loss = pei;
            }

            per_layer_flux[layer_index] = pei;
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

        let (mut theta, field_capacity, conductivity, thickness) =
            Self::wb19_load_layer_state(request, phase_class)?;

        let mut saturated_thickness = 0.0_f64;
        let mut conductivity_depth_sum = 0.0_f64;
        let mut saturated_depth_sum = 0.0_f64;
        for (((theta_i, fc_i), ssc_i), dg_i) in theta
            .iter()
            .zip(field_capacity.iter())
            .zip(conductivity.iter())
            .zip(thickness.iter())
        {
            if *theta_i + WB11_ZERO_THRESHOLD >= *fc_i {
                saturated_thickness += *dg_i;
                saturated_depth_sum += *dg_i;
                conductivity_depth_sum += *ssc_i * *dg_i;
            }
        }

        let q_lateral_potential = if saturated_thickness <= WB11_ZERO_THRESHOLD
            || saturated_depth_sum <= WB11_ZERO_THRESHOLD
        {
            0.0
        } else {
            let ke = 86_400.0 * (conductivity_depth_sum / saturated_depth_sum);
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

            (saturated_thickness * anisotropy * ke * slope_factor.max(0.0)) / slplen
        };

        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral_potential,
            Some(0.0),
            None,
        )?;

        let layer_pool = Self::wb19_drainable_storage(&theta, &field_capacity);
        let available_pool = layer_pool.max(drainable_storage_legacy + recharge_pe);
        let q_lateral = q_lateral_potential.min(available_pool);

        let _withdrawn = Self::wb19_withdraw_top_down(&mut theta, &field_capacity, q_lateral);

        let drainable_after = (available_pool - q_lateral).max(0.0);
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        )?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral,
            Some(0.0),
            Some(available_pool),
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-LAT-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = Vec::with_capacity(theta.len() + 1);
        state_updates.push(WritebackField::bounded(
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
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

        let drain_depth_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_DEPTH);
        let drain_depth =
            Self::require_state_scalar_for_symbol(request, phase_class, &drain_depth_symbol)?;
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
        let drain_spacing =
            Self::require_state_scalar_for_symbol(request, phase_class, &drain_spacing_symbol)?;
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
        let drain_diameter =
            Self::require_state_scalar_for_symbol(request, phase_class, &drain_diameter_symbol)?;
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

        let (mut theta, field_capacity, conductivity, thickness) =
            Self::wb19_load_layer_state(request, phase_class)?;
        let layer_pool = Self::wb19_drainable_storage(&theta, &field_capacity);
        let available_pool = layer_pool.max(drainable_storage_legacy);

        let mut q_drainage_potential = 0.0_f64;
        let mut tile_layer_index = theta.len().saturating_sub(1);

        if drain_enabled {
            let mut watbl = 0.0_f64;
            let mut hit_unsat_zone = false;
            for idx in (0..theta.len()).rev() {
                if theta[idx] + WB11_ZERO_THRESHOLD >= field_capacity[idx] {
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
                            * ((8.0 / std::f64::consts::PI) * radius_ratio.ln() - WB19_DRAIN_ALPHA);
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

                q_drainage_potential = (drainage_cm_h / 100.0) * WB19_DRAIN_HOURS_PER_DAY;
                Self::require_flux_range(
                    phase_class,
                    WB11_SYMBOL_DRAINAGE_QDD,
                    q_drainage_potential,
                    Some(0.0),
                    None,
                )?;
            }
        }

        let q_drainage = q_drainage_potential
            .min(drainage_capacity)
            .min(available_pool);
        let _withdrawn = Self::wb19_withdraw_tile_to_surface(
            &mut theta,
            &field_capacity,
            tile_layer_index,
            q_drainage,
        );

        let drainable_after = (available_pool - q_drainage).max(0.0);
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        )?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_DRAINAGE_QDD,
            q_drainage,
            Some(0.0),
            Some(drainage_capacity),
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
        let mut dg_sum = 0.0_f64;

        for layer_index in 1..=2 {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let dg_symbol = Self::wb19_dg_symbol(layer_index);

            let theta = Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            let fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
            let ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
            let dg = Self::require_state_scalar_for_symbol(request, phase_class, &dg_symbol)?;

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

            theta_sum += theta.max(0.0);
            ul_sum += ul;
            fc_sum += fc.max(0.0);
            dg_sum += dg;
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

        let avthetafc = fc_sum / dg_sum;
        let avthetadr = (ul_sum - fc_sum) / dg_sum;

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

        let runon_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RUNON_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RUNON_INPUT,
            runon_input,
            Some(0.0),
            None,
        )?;

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
                minimum: Some(1.0),
                maximum: Some(4.0),
            });
        }
        let case_number = format!("{case_rounded:.0}").parse::<i32>().map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(1.0),
                maximum: Some(4.0),
            }
        })?;
        if !(1..=4).contains(&case_number) {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(1.0),
                maximum: Some(4.0),
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
                minimum: Some(1.0),
                maximum: Some(4.0),
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
            let mut updates = Vec::with_capacity(5 + (class_count * 6));
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
            if attenuation_factor < 1.0e-8 {
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

                if ratbot <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                        symbol: BoundarySymbol::from(EROD14_SYMBOL_LDBOT),
                        value: ratbot,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
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
            (sumssa / ssa_soil) + 0.005
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

        let mut updates = Vec::with_capacity(5 + (class_count * 6));
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

    #[allow(clippy::too_many_lines)]
    fn run_peak_runoff(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPeakRunoff;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;
        if q_runoff <= WB11_ZERO_THRESHOLD {
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
                    WritebackField::bounded(WB16_SYMBOL_METHOD_BRANCH, 1.0, Some(1.0), Some(3.0)),
                    WritebackField::bounded(
                        WB16_SYMBOL_TSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_QPSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_VSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
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
        if !vave.is_finite() || vave <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_Q),
                value: vave,
                minimum: Some(WB11_ZERO_THRESHOLD),
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

        let timep = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_TIMEP)?;
        Self::require_state_range(phase_class, WB16_SYMBOL_TIMEP, timep, Some(0.0), Some(1.0))?;

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
        if exponent_m <= 1.0 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EXPONENT_M),
                value: exponent_m,
                minimum: Some(1.0 + WB11_ZERO_THRESHOLD),
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
                    WritebackField::bounded(WB16_SYMBOL_METHOD_BRANCH, 1.0, Some(1.0), Some(3.0)),
                    WritebackField::bounded(
                        WB16_SYMBOL_TSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_QPSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_VSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
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
        if !vstar.is_finite() || vstar <= WB11_ZERO_THRESHOLD || vstar > 1.0 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                value: vstar,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }

        let vave_power = vave.powf(exponent_m - 1.0);
        let te_base = efflen / (ealpha * vave_power);
        if !te_base.is_finite() || te_base <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EFFLEN),
                value: te_base,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let te = te_base.powf(1.0 / exponent_m);
        let tstar = te / effdrr;
        if !tstar.is_finite() || tstar <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_TSTAR),
                value: tstar,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let (method_branch, qpstar) = if tstar >= 1.0 {
            (1.0, 1.0 / tstar.powf(exponent_m))
        } else if tstar > timep {
            (2.0, 1.0 / tstar)
        } else {
            (3.0, (1.0 / vstar) - 0.6 * (((1.0 - vstar) / vstar) * tstar))
        };
        if !qpstar.is_finite() || qpstar <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_QPSTAR),
                value: qpstar,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let peakro_raw = vave * qpstar;
        if !peakro_raw.is_finite() || peakro_raw <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
                value: peakro_raw,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let peakro = peakro_raw.max(WB16_PEAKRO_FLOOR);
        if !peakro.is_finite() || peakro <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
                value: peakro,
                minimum: Some(WB11_ZERO_THRESHOLD),
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
        let status_message_id = if !erod14_state_updates.is_empty() {
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
                Some(3.0),
            ),
            WritebackField::bounded(WB16_SYMBOL_TSTAR, tstar, Some(WB11_ZERO_THRESHOLD), None),
            WritebackField::bounded(WB16_SYMBOL_QPSTAR, qpstar, Some(WB11_ZERO_THRESHOLD), None),
            WritebackField::bounded(
                WB16_SYMBOL_VSTAR,
                vstar,
                Some(WB11_ZERO_THRESHOLD),
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
        ];
        state_updates.extend(erod13_state_updates);
        state_updates.extend(erod14_state_updates);

        let writeback = KernelWritebackPayload::with_updates(state_updates, Vec::new());
        Ok(KernelRunResponse::new(status, writeback))
    }
}
