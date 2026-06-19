#[allow(clippy::wildcard_imports)]
use super::super::*;

mod frost;
mod frost_entry;

#[derive(Debug, Clone)]
struct FrostLayerWaterState {
    layer_index: usize,
    fine_layer_count: usize,
    fine_layer_thickness_m: f64,
    dg_m: f64,
    bulk_density_kg_m3: f64,
    thetdr: f64,
    theta_m: f64,
    upper_limit_m: f64,
    frozen_depth_m: f64,
    frzw_m: f64,
}

#[derive(Debug, Clone)]
struct FrostFineLayerState {
    layer_index: usize,
    fine_index: usize,
    fine_layer_thickness_m: f64,
    fgfrst: f64,
    slfsd_m: f64,
    slsic_m: f64,
    slsw_theta: f64,
    sltime_s: f64,
}

#[derive(Debug, Clone)]
struct FrostLayerExchangeState {
    layer_index: usize,
    thetdr: f64,
    st_m: f64,
    yst_m: f64,
    nwfrzz_m: f64,
    frozen_m: f64,
    frzw_m: f64,
    soilf_m: f64,
    soil_water_m: f64,
}

#[derive(Debug, Clone)]
struct FrostFineShadowState {
    fine_layers: Vec<FrostFineLayerState>,
    layer_state: Vec<FrostLayerExchangeState>,
    total_water_before_m: f64,
    total_water_after_m: f64,
    wb_delta_m: f64,
    residual_m: f64,
    watpdg_m: f64,
    watbtm_m: f64,
}

#[derive(Debug, Clone, Copy)]
struct FrostDepthSummary {
    frdp: f64,
    thdp: f64,
    tfrdp: f64,
    tthawd: f64,
}

#[derive(Debug, Clone, Copy)]
struct FrostSeasonalTemperatureCurve {
    annual_mean_c: f64,
    amplitude_c: f64,
    phase_shift_days: f64,
}

impl Wb11HydrologyKernel {
    pub(crate) fn interval_overlap_duration(
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

    pub(crate) fn bounded_interval_overlap_duration(
        interval_start: f64,
        interval_end: f64,
        active_start: f64,
        active_end: f64,
    ) -> f64 {
        if active_end <= active_start {
            return 0.0;
        }
        let overlap_start = interval_start.max(active_start);
        let overlap_end = interval_end.min(active_end);
        (overlap_end - overlap_start).max(0.0)
    }

    pub(crate) fn resolve_active_snow_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT);
        if let Some(value) = Self::state_value_for_symbol(request, &key) {
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
        }

        let runtime_swe = Self::validate_runtime_snow_state_domains(request, phase_class)?;

        let tmax = Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let tmin = Self::optional_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;
        let cold_day_active = match (tmax, tmin) {
            (Some(tmax), Some(tmin)) => f64::midpoint(tmax, tmin) < 0.0,
            _ => false,
        };
        let snow_controls_projected =
            Self::state_value_for_symbol(request, &BoundarySymbol::from(WB14_SYMBOL_SNOW_RST))
                .is_some()
                && Self::state_value_for_symbol(
                    request,
                    &BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
                )
                .is_some()
                && Self::state_value_for_symbol(request, &BoundarySymbol::from(WB14_SYMBOL_SNOW_SSD))
                    .is_some();

        let active_snow_coupling =
            runtime_swe > WB11_ZERO_THRESHOLD || (cold_day_active && snow_controls_projected);
        Ok(active_snow_coupling)
    }

    pub(crate) fn validate_runtime_snow_state_domains(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let runtime_swe_symbol = BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE);
        let depth_symbol = BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL);
        let density_symbol = BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL);
        let settle_day_count_symbol = BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL);
        let snow_option_symbols = [
            BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT),
            BoundarySymbol::from(WB14_SYMBOL_SNOW_RST),
            BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
            BoundarySymbol::from(WB14_SYMBOL_SNOW_SSD),
        ];

        let snow_projection_present = [
            &runtime_swe_symbol,
            &depth_symbol,
            &density_symbol,
            &settle_day_count_symbol,
        ]
        .into_iter()
        .chain(snow_option_symbols.iter())
        .any(|symbol| Self::state_value_for_symbol(request, symbol).is_some());
        if !snow_projection_present {
            return Ok(0.0);
        }

        let runtime_swe =
            Self::require_state_scalar_for_symbol(request, phase_class, &runtime_swe_symbol)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_RUNTIME_SWE,
            runtime_swe,
            Some(0.0),
            None,
        )?;

        let runtime_depth_m =
            Self::require_state_scalar_for_symbol(request, phase_class, &depth_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            depth_symbol,
            runtime_depth_m,
            Some(0.0),
            None,
        )?;

        let runtime_density_kg_m3 =
            Self::require_state_scalar_for_symbol(request, phase_class, &density_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            density_symbol,
            runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        let runtime_settle_day_count =
            Self::require_state_scalar_for_symbol(request, phase_class, &settle_day_count_symbol)?;
        Self::require_dynamic_state_range(
            phase_class,
            settle_day_count_symbol,
            runtime_settle_day_count,
            Some(0.0),
            None,
        )?;

        Ok(runtime_swe)
    }
}
