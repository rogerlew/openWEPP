#[allow(clippy::wildcard_imports)]
use super::super::super::*;

use std::fmt::Write as _;

#[allow(clippy::wildcard_imports)]
use super::*;

#[derive(Debug, Clone, Copy)]
struct ActiveFrostControls {
    fine_top_count: usize,
    fine_bot_count: usize,
    ksnowf: f64,
    kresf: f64,
    ksoilf: f64,
}

#[cfg(test)]
mod cqr_row5_tests {
    use super::*;

    fn controls(landuse_class_proxy: Option<f64>) -> DirectFrostControlInputs {
        DirectFrostControlInputs {
            frost_file_present: true,
            wint_red_enabled: true,
            fine_top_count: 1,
            fine_bot_count: 1,
            ksnowf: 1.0,
            kresf: 2.0,
            ksoilf: 3.0,
            kfactor1: 0.31,
            kfactor2: 0.42,
            kfactor3: 0.53,
            landuse_class_proxy,
        }
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1.0e-12,
            "actual={actual} expected={expected}"
        );
    }

    #[test]
    fn r7g_json_string_escapes_special_and_control_characters() {
        assert_eq!(r7g_json_string("plain"), "\"plain\"");
        assert_eq!(
            r7g_json_string("quote\" slash\\ newline\n carriage\r tab\t control\u{001f}"),
            "\"quote\\\" slash\\\\ newline\\n carriage\\r tab\\t control\\u001f\""
        );
    }

    #[test]
    fn frozen_soil_kfactor_resolves_proxy_classes_and_errors() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        assert_close(
            Wb11HydrologyKernel::resolve_frozen_soil_kfactor_from_typed(
                phase_class,
                controls(None)
            )
            .unwrap(),
            0.31
        );
        assert_close(
            Wb11HydrologyKernel::resolve_frozen_soil_kfactor_from_typed(
                phase_class,
                controls(Some(1.0))
            )
            .unwrap(),
            0.31
        );
        assert_close(
            Wb11HydrologyKernel::resolve_frozen_soil_kfactor_from_typed(
                phase_class,
                controls(Some(2.0))
            )
            .unwrap(),
            0.42
        );
        assert_close(
            Wb11HydrologyKernel::resolve_frozen_soil_kfactor_from_typed(
                phase_class,
                controls(Some(3.0))
            )
            .unwrap(),
            0.53
        );

        for proxy in [1.25, 4.0] {
            let error = Wb11HydrologyKernel::resolve_frozen_soil_kfactor_from_typed(
                phase_class,
                controls(Some(proxy)),
            )
            .unwrap_err();
            assert!(matches!(
                error,
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
            ));
            assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostPriorContext {
    profile_depth_m: f64,
    prior_depth_summary: FrostDepthSummary,
    effective_prior_frdp_m: f64,
    prior_nft: f64,
    soil_water: f64,
    prior_ws_frz: f64,
    fgthwd_flag: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostProfileShadowContext {
    profile_depth_m: f64,
    prior_depth_summary: FrostDepthSummary,
    prior_layer_frozen_depth_m: f64,
    prior_layer_frozen_store_m: f64,
    prior_fine_frozen_store_m: f64,
    prior_layer_state_active: bool,
    prior_fine_state_active: bool,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostThermalContext {
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    ksnowf: f64,
    residue_depth_m: f64,
    conductivity_residue_w_m_k: f64,
    snow_conductivity_w_m_k: f64,
    seasonal_temperature_curve: FrostSeasonalTemperatureCurve,
    sdate: f64,
    kfactor_selected: f64,
}

struct ActiveFrostHourlyContext<'forcing> {
    phase_class: HillslopeKernelPhaseClass,
    hourly_forcing: &'forcing [DirectFrostHourlyForcing; SIMIMPL29_HOURS_PER_DAY],
    tmpadj: ActiveFrostTmpadjContext,
    layer_water_state: &'forcing [FrostLayerWaterState],
    profile_depth_m: f64,
    effective_prior_frdp_m: f64,
    thermal: ActiveFrostThermalContext,
    ksoilf: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostFinalScalars {
    dfrost: f64,
    dthaw: f64,
    nft: f64,
    ws_frz: f64,
    infcap_frz: f64,
    soil_water_after_frwatc: Option<f64>,
    frwatc_soil_water_after: f64,
    frwatc_freeze_exchange: f64,
    frwatc_thaw_release: f64,
    frwatc_net_liquid_delta: f64,
    tfrdp_m: f64,
    tthawd_m: f64,
    fgthwd_flag: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostCompletionContext {
    phase_class: HillslopeKernelPhaseClass,
    prior: ActiveFrostPriorContext,
    thermal: ActiveFrostThermalContext,
    soil_conductivity: f64,
    freeze_started: bool,
    fgthwd_flag: f64,
}

#[derive(Debug, Clone)]
struct R7gFrostTraceConfig {
    path: std::path::PathBuf,
    exact_day: Option<f64>,
    exact_year: Option<f64>,
}

static R7G_FROST_TRACE_CONFIG: std::sync::OnceLock<Option<R7gFrostTraceConfig>> =
    std::sync::OnceLock::new();

fn r7g_frost_trace_config() -> Option<&'static R7gFrostTraceConfig> {
    R7G_FROST_TRACE_CONFIG
        .get_or_init(|| {
            let path = std::env::var_os("OPENWEPP_R7G_FROST_TRACE_PATH")?;
            if path.is_empty() {
                return None;
            }
            Some(R7gFrostTraceConfig {
                path: std::path::PathBuf::from(path),
                exact_day: r7g_frost_trace_env_f64("OPENWEPP_R7G_FROST_TRACE_DAY"),
                exact_year: r7g_frost_trace_env_f64("OPENWEPP_R7G_FROST_TRACE_YEAR"),
            })
        })
        .as_ref()
}

fn r7g_frost_trace_env_f64(name: &str) -> Option<f64> {
    let value = std::env::var(name).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<f64>().ok()
}

fn r7g_frost_trace_matches_filter(
    config: &R7gFrostTraceConfig,
    day: Option<f64>,
    year: Option<f64>,
) -> bool {
    if let Some(exact_day) = config.exact_day
        && day.is_none_or(|day| (day - exact_day).abs() > WB11_ZERO_THRESHOLD)
    {
        return false;
    }
    if let Some(exact_year) = config.exact_year
        && year.is_none_or(|year| (year - exact_year).abs() > WB11_ZERO_THRESHOLD)
    {
        return false;
    }
    true
}

fn r7g_json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            control if control.is_control() => {
                let _ = write!(&mut escaped, "\\u{:04x}", u32::from(control));
            }
            other => escaped.push(other),
        }
    }
    escaped.push('"');
    escaped
}

fn r7g_frost_trace_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.17}")
    } else {
        "null".to_string()
    }
}

fn r7g_frost_trace_optional_number(value: Option<f64>) -> String {
    value.map_or_else(|| "null".to_string(), r7g_frost_trace_number)
}

fn r7g_frost_trace_array(values: impl IntoIterator<Item = f64>) -> String {
    let mut out = String::from("[");
    let mut first = true;
    for value in values {
        if first {
            first = false;
        } else {
            out.push(',');
        }
        out.push_str(&r7g_frost_trace_number(value));
    }
    out.push(']');
    out
}

fn r7g_frost_trace_usize_array(values: impl IntoIterator<Item = usize>) -> String {
    let mut out = String::from("[");
    let mut first = true;
    for value in values {
        if first {
            first = false;
        } else {
            out.push(',');
        }
        out.push_str(&value.to_string());
    }
    out.push(']');
    out
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn maybe_write_r7g_frost_trace(
    phase_name: &str,
    day: Option<f64>,
    year: Option<f64>,
    prior: ActiveFrostPriorContext,
    profile: ActiveFrostProfileShadowContext,
    thermal: ActiveFrostThermalContext,
    tmpadj: ActiveFrostTmpadjContext,
    hourly_forcing: &[DirectFrostHourlyForcing; SIMIMPL29_HOURS_PER_DAY],
    prior_shadow: &FrostFineShadowState,
    hourly_state: &[FrostHourlyState; SIMIMPL29_HOURS_PER_DAY],
    outcome: &FrostCouplingOutcome,
    fast_path: bool,
) {
    let Some(config) = r7g_frost_trace_config() else {
        return;
    };
    if !r7g_frost_trace_matches_filter(config, day, year) {
        return;
    }

    let final_depth = FrostDepthSummary {
        frdp: outcome.frdp_m,
        thdp: outcome.thdp_m,
        tfrdp: outcome.tfrdp_m,
        tthawd: outcome.tthawd_m,
    };
    let mut line = String::new();
    line.push('{');
    line.push_str("\"schema\":\"openwepp-r7g-frost-trace-v1\"");
    line.push_str(",\"phase\":");
    line.push_str(&r7g_json_string(phase_name));
    line.push_str(",\"day\":");
    line.push_str(&r7g_frost_trace_optional_number(day));
    line.push_str(",\"year\":");
    line.push_str(&r7g_frost_trace_optional_number(year));
    line.push_str(",\"fast_path\":");
    line.push_str(if fast_path { "true" } else { "false" });
    line.push_str(",\"prior_frdp_m\":");
    line.push_str(&r7g_frost_trace_number(prior.effective_prior_frdp_m));
    line.push_str(",\"prior_ws_frz_m\":");
    line.push_str(&r7g_frost_trace_number(prior.prior_ws_frz));
    line.push_str(",\"prior_soil_water_m\":");
    line.push_str(&r7g_frost_trace_number(prior.soil_water));
    line.push_str(",\"profile_prior_depth_frdp_m\":");
    line.push_str(&r7g_frost_trace_number(profile.prior_depth_summary.frdp));
    line.push_str(",\"profile_prior_layer_frozen_depth_m\":");
    line.push_str(&r7g_frost_trace_number(profile.prior_layer_frozen_depth_m));
    line.push_str(",\"profile_prior_layer_frozen_store_m\":");
    line.push_str(&r7g_frost_trace_number(profile.prior_layer_frozen_store_m));
    line.push_str(",\"profile_prior_fine_frozen_store_m\":");
    line.push_str(&r7g_frost_trace_number(profile.prior_fine_frozen_store_m));
    line.push_str(",\"snow_depth_m\":");
    line.push_str(&r7g_frost_trace_number(thermal.snow_depth_m));
    line.push_str(",\"snow_density_kg_m3\":");
    line.push_str(&r7g_frost_trace_number(thermal.snow_density_kg_m3));
    line.push_str(",\"snow_conductivity_w_m_k\":");
    line.push_str(&r7g_frost_trace_number(thermal.snow_conductivity_w_m_k));
    line.push_str(",\"residue_depth_m\":");
    line.push_str(&r7g_frost_trace_number(thermal.residue_depth_m));
    line.push_str(",\"wind_m_s\":");
    line.push_str(&r7g_frost_trace_number(tmpadj.wind_m_s));
    line.push_str(",\"albedo\":");
    line.push_str(&r7g_frost_trace_number(tmpadj.albedo));
    line.push_str(",\"canopy_height_m\":");
    line.push_str(&r7g_frost_trace_number(tmpadj.canopy_height_m));
    line.push_str(",\"random_roughness_m\":");
    line.push_str(&r7g_frost_trace_number(tmpadj.random_roughness_m));
    line.push_str(",\"kfactor_selected\":");
    line.push_str(&r7g_frost_trace_number(thermal.kfactor_selected));
    line.push_str(",\"final_frdp_m\":");
    line.push_str(&r7g_frost_trace_number(final_depth.frdp));
    line.push_str(",\"final_thdp_m\":");
    line.push_str(&r7g_frost_trace_number(final_depth.thdp));
    line.push_str(",\"final_ws_frz_m\":");
    line.push_str(&r7g_frost_trace_number(outcome.ws_frz));
    line.push_str(",\"final_soil_water_after_m\":");
    line.push_str(&r7g_frost_trace_optional_number(outcome.soil_water_after_frwatc));
    line.push_str(",\"frwatc_soil_water_after_m\":");
    line.push_str(&r7g_frost_trace_number(outcome.frwatc_soil_water_after));
    line.push_str(",\"frwatc_freeze_debit_m\":");
    line.push_str(&r7g_frost_trace_number(outcome.frwatc_freeze_debit));
    line.push_str(",\"frwatc_net_liquid_delta_m\":");
    line.push_str(&r7g_frost_trace_number(outcome.frwatc_net_liquid_delta));
    line.push_str(",\"final_layer_theta_m\":");
    line.push_str(&r7g_frost_trace_array(
        outcome
            .layer_topology_state
            .iter()
            .map(|layer| layer.theta_after_m),
    ));
    line.push_str(",\"final_layer_frozen_depth_m\":");
    line.push_str(&r7g_frost_trace_array(
        outcome
            .layer_topology_state
            .iter()
            .map(|layer| layer.frozen_depth_m),
    ));
    line.push_str(",\"final_layer_frozen_water_m\":");
    line.push_str(&r7g_frost_trace_array(
        outcome.layer_topology_state.iter().map(|layer| layer.frzw_m),
    ));
    line.push_str(",\"shadow_layer_soil_water_m\":");
    line.push_str(&r7g_frost_trace_array(
        outcome
            .shadow_layer_state
            .iter()
            .map(|layer| layer.soil_water_m),
    ));
    line.push_str(",\"shadow_layer_frozen_depth_m\":");
    line.push_str(&r7g_frost_trace_array(
        outcome
            .shadow_layer_state
            .iter()
            .map(|layer| layer.frozen_depth_m),
    ));
    line.push_str(",\"shadow_layer_frozen_water_m\":");
    line.push_str(&r7g_frost_trace_array(
        outcome.shadow_layer_state.iter().map(|layer| layer.frzw_m),
    ));
    line.push_str(",\"hour_radiation_mj_m2\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_forcing.iter().map(|hourly| hourly.radiation_mj_m2),
    ));
    line.push_str(",\"hour_air_temperature_c\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_forcing.iter().map(|hourly| hourly.air_temperature_c),
    ));
    line.push_str(",\"hour_cloud_fraction\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_forcing.iter().map(|hourly| hourly.cloud_fraction),
    ));
    line.push_str(",\"hour_frzflg\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_state.iter().map(|hourly| hourly.frzflg),
    ));
    line.push_str(",\"hour_surface_temp_c\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_state.iter().map(|hourly| hourly.surface_temp_c),
    ));
    line.push_str(",\"hour_qsrf_w_m2\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_state.iter().map(|hourly| hourly.qsrf_w_m2),
    ));
    line.push_str(",\"hour_quf_w_m2\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_state.iter().map(|hourly| hourly.quf_w_m2),
    ));
    line.push_str(",\"hour_tilled_frozen_depth_m\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_state
            .iter()
            .map(|hourly| hourly.tilled_frozen_depth_m),
    ));
    line.push_str(",\"hour_untilled_frozen_depth_m\":");
    line.push_str(&r7g_frost_trace_array(
        hourly_state
            .iter()
            .map(|hourly| hourly.untilled_frozen_depth_m),
    ));
    line.push_str(",\"prior_fine_layer_index\":");
    line.push_str(&r7g_frost_trace_usize_array(
        prior_shadow.fine_layers.iter().map(|fine| fine.layer_index),
    ));
    line.push_str(",\"prior_fine_index\":");
    line.push_str(&r7g_frost_trace_usize_array(
        prior_shadow.fine_layers.iter().map(|fine| fine.fine_index),
    ));
    line.push_str(",\"prior_fine_slfsd_m\":");
    line.push_str(&r7g_frost_trace_array(
        prior_shadow.fine_layers.iter().map(|fine| fine.slfsd_m),
    ));
    line.push_str(",\"prior_fine_slsic_m\":");
    line.push_str(&r7g_frost_trace_array(
        prior_shadow.fine_layers.iter().map(|fine| fine.slsic_m),
    ));
    line.push_str(",\"prior_fine_slsw_theta\":");
    line.push_str(&r7g_frost_trace_array(
        prior_shadow.fine_layers.iter().map(|fine| fine.slsw_theta),
    ));
    line.push_str(",\"final_fine_slfsd_m\":");
    line.push_str(&r7g_frost_trace_array(
        outcome.fine_layer_state.iter().map(|fine| fine.slfsd_m),
    ));
    line.push_str(",\"final_fine_slsic_m\":");
    line.push_str(&r7g_frost_trace_array(
        outcome.fine_layer_state.iter().map(|fine| fine.slsic_m),
    ));
    line.push_str(",\"final_fine_slsw_theta\":");
    line.push_str(&r7g_frost_trace_array(
        outcome.fine_layer_state.iter().map(|fine| fine.slsw_theta),
    ));
    line.push('}');
    line.push('\n');

    if let Some(parent) = config.path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.path)
    {
        let _ = std::io::Write::write_all(&mut file, line.as_bytes());
    }
}

impl Wb11HydrologyKernel {
    fn require_frost_fine_count_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
        value: f64,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        let rounded = value.round();
        let parsed = format!("{rounded:.0}").parse::<usize>().map_err(|_| {
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value,
                minimum: Some(1.0),
                maximum: Some(10.0),
            }
        })?;
        if !(1..=10).contains(&parsed) {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value,
                minimum: Some(1.0),
                maximum: Some(10.0),
            });
        }
        Ok(parsed)
    }

    fn frost_fine_layer_count_for_layer(
        phase_class: HillslopeKernelPhaseClass,
        dg_m: f64,
        layer_index: usize,
        layer_count: usize,
        controls: ActiveFrostControls,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        if layer_index != layer_count {
            return Ok(if layer_index < 3 {
                controls.fine_top_count
            } else {
                controls.fine_bot_count
            });
        }

        let spacing_mm = if layer_index > 2 {
            200.0 / Self::diagnostic_count_to_f64(controls.fine_bot_count)
        } else {
            // UNIT-CONVERSION-ALLOW: cm_m_scale percentage allocation, not dimensional conversion.
            100.0 / Self::diagnostic_count_to_f64(controls.fine_top_count)
        };
        let dg_mm =
            openwepp_unit_boundary::conversions::meters_to_millimeters(dg_m).map_err(|error| {
                Self::unit_conversion_guard_error(
                    phase_class,
                    Self::wb19_dg_symbol(layer_index),
                    &error,
                )
            })?;
        let dg_mm_trunc = dg_mm.trunc();
        let ratio_trunc = (dg_mm / spacing_mm).trunc();
        let mut count = format!("{ratio_trunc:.0}").parse::<usize>().map_err(|_| {
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: Self::wb19_dg_symbol(layer_index),
                value: ratio_trunc,
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
            }
        })?;
        let count_trunc_mm = (Self::diagnostic_count_to_f64(count) * spacing_mm).trunc();
        if (count_trunc_mm - dg_mm_trunc).abs() > WB11_ZERO_THRESHOLD {
            count += 1;
        }
        Ok(count.max(1))
    }

    fn require_active_frost_controls_from_typed(
        inputs: DirectFrostControlInputs,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<ActiveFrostControls, Wb11HydrologyKernelGuardError> {
        if !inputs.wint_red_enabled {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: 0.0,
                minimum: Some(1.0),
                maximum: Some(1.0),
            });
        }
        let fine_top_count = Self::require_frost_fine_count_value(
            phase_class,
            WB14_SYMBOL_FROST_FINE_TOP,
            Self::diagnostic_count_to_f64(inputs.fine_top_count),
        )?;
        let fine_bot_count = Self::require_frost_fine_count_value(
            phase_class,
            WB14_SYMBOL_FROST_FINE_BOT,
            Self::diagnostic_count_to_f64(inputs.fine_bot_count),
        )?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_KSNOWF, inputs.ksnowf),
            (WB14_SYMBOL_FROST_KRESF, inputs.kresf),
            (WB14_SYMBOL_FROST_KSOILF, inputs.ksoilf),
        ] {
            Self::require_state_range(phase_class, symbol, value, Some(0.1), Some(10.0))?;
        }
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_KFACTOR1, inputs.kfactor1),
            (WB14_SYMBOL_FROST_KFACTOR2, inputs.kfactor2),
            (WB14_SYMBOL_FROST_KFACTOR3, inputs.kfactor3),
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
        Ok(ActiveFrostControls {
            fine_top_count,
            fine_bot_count,
            ksnowf: inputs.ksnowf,
            kresf: inputs.kresf,
            ksoilf: inputs.ksoilf,
        })
    }

    fn require_frost_layer_water_state_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        layer: DirectFrostLayerInput,
        layer_count: usize,
        controls: ActiveFrostControls,
    ) -> Result<FrostLayerWaterState, Wb11HydrologyKernelGuardError> {
        if layer.layer_index == 0 || layer.layer_index > layer_count {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("wb11_nsl"),
                value: Self::diagnostic_count_to_f64(layer.layer_index),
                minimum: Some(1.0),
                maximum: Some(Self::diagnostic_count_to_f64(layer_count)),
            });
        }
        Self::require_state_range_with(
            phase_class,
            || Self::wb19_dg_symbol(layer.layer_index),
            layer.depth_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let fine_layer_count = Self::frost_fine_layer_count_for_layer(
            phase_class,
            layer.depth_m,
            layer.layer_index,
            layer_count,
            controls,
        )?;
        Self::require_state_range_with(
            phase_class,
            || Self::wb18_perc_state_symbol("theta", layer.layer_index),
            layer.theta_m,
            Some(0.0),
            None,
        )?;
        Self::require_state_range_with(
            phase_class,
            || Self::wb18_perc_state_symbol("ul", layer.layer_index),
            layer.upper_limit_m,
            Some(0.0),
            None,
        )?;
        Self::require_state_range_with(
            phase_class,
            || Self::wb19_thetdr_symbol(layer.layer_index),
            layer.residual_theta,
            Some(0.0),
            Some(1.0),
        )?;
        Self::require_state_range_with(
            phase_class,
            || Self::wb19_bulk_density_kg_m3_symbol(layer.layer_index),
            layer.bulk_density_kg_m3,
            Some(WB11_ZERO_THRESHOLD),
            Some(2_650.0),
        )?;
        Self::require_state_range_with(
            phase_class,
            || Self::wb18_perc_state_symbol("frozen_depth", layer.layer_index),
            layer.frozen_depth_m,
            Some(0.0),
            Some(layer.depth_m),
        )?;
        Self::require_state_range_with(
            phase_class,
            || Self::wb18_perc_state_symbol("frzw", layer.layer_index),
            layer.frozen_water_m,
            Some(0.0),
            Some(layer.upper_limit_m),
        )?;
        Ok(FrostLayerWaterState {
            layer_index: layer.layer_index,
            fine_layer_count,
            fine_layer_thickness_m: layer.depth_m / Self::diagnostic_count_to_f64(fine_layer_count),
            dg_m: layer.depth_m,
            bulk_density_kg_m3: layer.bulk_density_kg_m3,
            thetdr: layer.residual_theta,
            theta_m: layer.theta_m,
            upper_limit_m: layer.upper_limit_m,
            frozen_depth_m: layer.frozen_depth_m,
            frzw_m: layer.frozen_water_m,
        })
    }

    fn require_active_frost_layer_water_state_from_typed(
        inputs: &[DirectFrostLayerInput],
        phase_class: HillslopeKernelPhaseClass,
        controls: ActiveFrostControls,
    ) -> Result<(usize, Vec<FrostLayerWaterState>), Wb11HydrologyKernelGuardError> {
        if inputs.is_empty() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("wb11_nsl"),
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let mut layer_water_state = Vec::with_capacity(inputs.len());
        let mut total_fine_layer_count = 0usize;
        for layer in inputs {
            let layer_state = Self::require_frost_layer_water_state_from_typed(
                phase_class,
                *layer,
                inputs.len(),
                controls,
            )?;
            total_fine_layer_count += layer_state.fine_layer_count;
            layer_water_state.push(layer_state);
        }
        Ok((total_fine_layer_count, layer_water_state))
    }

    fn require_frost_profile_shadow_context_from_typed(
        inputs: &DirectActiveFrostPartitionInputs,
        phase_class: HillslopeKernelPhaseClass,
        layer_water_state: &[FrostLayerWaterState],
    ) -> Result<(FrostFineShadowState, ActiveFrostProfileShadowContext), Wb11HydrologyKernelGuardError>
    {
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(PL_GROWTH_SOIL_DEPTH_SYMBOL),
            inputs.profile_depth_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let prior_layer_frozen_depth_m = layer_water_state
            .iter()
            .map(|layer| layer.frozen_depth_m)
            .sum::<f64>();
        let prior_layer_frozen_store_m = Self::frost_layer_soilf_sum(layer_water_state);
        let prior_layer_state_active = prior_layer_frozen_depth_m > WB11_ZERO_THRESHOLD
            || prior_layer_frozen_store_m > WB11_ZERO_THRESHOLD;
        let shadow_fine_state = Self::compute_shadow_fine_state_from_typed(
            &inputs.prior_state,
            phase_class,
            layer_water_state,
        )?;
        let prior_depth_summary =
            Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let prior_fine_frozen_store_m = shadow_fine_state
            .layer_state
            .iter()
            .map(|layer| layer.soilf_m)
            .sum::<f64>();
        let prior_fine_state_active = prior_depth_summary.frdp > WB11_ZERO_THRESHOLD
            || prior_fine_frozen_store_m > WB11_ZERO_THRESHOLD;
        Ok((
            shadow_fine_state,
            ActiveFrostProfileShadowContext {
                profile_depth_m: inputs.profile_depth_m,
                prior_depth_summary,
                prior_layer_frozen_depth_m,
                prior_layer_frozen_store_m,
                prior_fine_frozen_store_m,
                prior_layer_state_active,
                prior_fine_state_active,
            },
        ))
    }

    fn require_profile_bounded_typed_frost_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol_name: &'static str,
        value: f64,
        profile_depth_m: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(symbol_name),
            value,
            Some(0.0),
            Some(profile_depth_m),
        )?;
        Ok(value)
    }

    fn require_typed_active_frost_storage_inputs(
        inputs: &DirectActiveFrostPartitionInputs,
        phase_class: HillslopeKernelPhaseClass,
        profile: ActiveFrostProfileShadowContext,
    ) -> Result<ActiveFrostPriorContext, Wb11HydrologyKernelGuardError> {
        let effective_prior_frdp_m =
            Self::resolve_typed_effective_prior_frdp_m(inputs, phase_class, profile)?;
        Self::require_typed_active_frost_storage_scalars(inputs, phase_class)?;
        let prior_ws_frz = Self::resolve_typed_prior_ws_frz(inputs, phase_class, profile)?;
        Ok(ActiveFrostPriorContext {
            profile_depth_m: profile.profile_depth_m,
            prior_depth_summary: profile.prior_depth_summary,
            effective_prior_frdp_m,
            prior_nft: inputs.prior_state.nft,
            soil_water: inputs.soil_water_m,
            prior_ws_frz,
            fgthwd_flag: inputs.prior_state.fgthwd_flag,
        })
    }

    fn resolve_typed_effective_prior_frdp_m(
        inputs: &DirectActiveFrostPartitionInputs,
        phase_class: HillslopeKernelPhaseClass,
        profile: ActiveFrostProfileShadowContext,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let prior_frdp_m = Self::require_profile_bounded_typed_frost_value(
            phase_class,
            FROST_RUNTIME_FRDP_M_SYMBOL,
            inputs.prior_state.frdp_m,
            profile.profile_depth_m,
        )?;
        let _prior_thdp_m = Self::require_profile_bounded_typed_frost_value(
            phase_class,
            FROST_RUNTIME_THDP_M_SYMBOL,
            inputs.prior_state.thdp_m,
            profile.profile_depth_m,
        )?;
        let _prior_top_frost_depth_m = Self::require_profile_bounded_typed_frost_value(
            phase_class,
            FROST_RUNTIME_TFRDP_M_SYMBOL,
            inputs.prior_state.tfrdp_m,
            profile.profile_depth_m,
        )?;
        let _prior_tthawd_m = Self::require_profile_bounded_typed_frost_value(
            phase_class,
            FROST_RUNTIME_TTHAWD_M_SYMBOL,
            inputs.prior_state.tthawd_m,
            profile.profile_depth_m,
        )?;
        let effective_prior_frdp_m = if profile.prior_fine_state_active {
            profile.prior_depth_summary.frdp
        } else if profile.prior_layer_state_active {
            profile.prior_layer_frozen_depth_m
        } else {
            prior_frdp_m
        };
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(FROST_RUNTIME_FRDP_M_SYMBOL),
            effective_prior_frdp_m,
            Some(0.0),
            Some(profile.profile_depth_m),
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(FROST_RUNTIME_FGTHWD_FLAG_SYMBOL),
            inputs.prior_state.fgthwd_flag,
            Some(0.0),
            Some(1.0),
        )?;
        Ok(effective_prior_frdp_m)
    }

    fn require_typed_active_frost_storage_scalars(
        inputs: &DirectActiveFrostPartitionInputs,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            inputs.prior_state.nft,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_RESIDUAL,
            inputs.theta_residual,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY,
            inputs.theta_field_capacity,
            Some(0.0),
            None,
        )?;
        if inputs.theta_field_capacity < inputs.theta_residual - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY),
                value: inputs.theta_field_capacity,
                minimum: Some(inputs.theta_residual),
                maximum: None,
            });
        }
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            inputs.soil_water_m,
            Some(0.0),
            None,
        )?;
        Ok(())
    }

    fn resolve_typed_prior_ws_frz(
        inputs: &DirectActiveFrostPartitionInputs,
        phase_class: HillslopeKernelPhaseClass,
        profile: ActiveFrostProfileShadowContext,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let prior_runtime_ws_frz = inputs.prior_state.ws_frz_m;
        let prior_ws_frz = if profile.prior_layer_state_active {
            profile.prior_layer_frozen_store_m
        } else if profile.prior_fine_state_active {
            profile.prior_fine_frozen_store_m
        } else {
            prior_runtime_ws_frz
        };
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            prior_ws_frz,
            Some(0.0),
            None,
        )?;
        Ok(prior_ws_frz)
    }

    fn resolve_frozen_soil_kfactor_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        controls: DirectFrostControlInputs,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(class_proxy) = controls.landuse_class_proxy else {
            return Ok(controls.kfactor1.min(controls.kfactor2.min(controls.kfactor3)));
        };
        let rounded = class_proxy.round();
        let symbol = BoundarySymbol::from(FROST_LANDUSE_CLASS_PROXY_SYMBOL);
        if (class_proxy - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value: class_proxy,
                minimum: Some(1.0),
                maximum: Some(3.0),
            });
        }
        match format!("{rounded:.0}").parse::<i32>().map_err(|_| {
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(FROST_LANDUSE_CLASS_PROXY_SYMBOL),
                value: class_proxy,
                minimum: Some(1.0),
                maximum: Some(3.0),
            }
        })? {
            1 => Ok(controls.kfactor1),
            2 => Ok(controls.kfactor2),
            3 => Ok(controls.kfactor3),
            _ => Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(FROST_LANDUSE_CLASS_PROXY_SYMBOL),
                value: class_proxy,
                minimum: Some(1.0),
                maximum: Some(3.0),
            }),
        }
    }

    fn require_active_frost_thermal_context_from_typed(
        inputs: &DirectActiveFrostPartitionInputs,
        phase_class: HillslopeKernelPhaseClass,
        controls: ActiveFrostControls,
    ) -> Result<ActiveFrostThermalContext, Wb11HydrologyKernelGuardError> {
        let kfactor_selected =
            Self::resolve_frozen_soil_kfactor_from_typed(phase_class, inputs.controls)?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(FROST_RUNTIME_SNOW_DEPTH_SYMBOL),
            inputs.thermal.snow_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            inputs.thermal.snow_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(FROST_RUNTIME_RESIDUE_DEPTH_SYMBOL),
            inputs.thermal.residue_depth_m,
            Some(0.0),
            None,
        )?;
        let conductivity_residue_w_m_k = FROST_RUNTIME_KRES_BASE_W_M_K * controls.kresf;
        let snow_conductivity_w_m_k =
            if inputs.thermal.snow_depth_m <= SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M
                || inputs.thermal.snow_density_kg_m3 <= 0.0
            {
                0.0
            } else {
                Self::tmpadj_snow_conductivity_w_m_k(
                    phase_class,
                    inputs.thermal.snow_density_kg_m3,
                    controls.ksnowf,
                )?
            };
        let sdate = inputs.thermal.day_of_year.round();
        if (inputs.thermal.day_of_year - sdate).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(PL_RUNTIME_DAY_SYMBOL),
                value: inputs.thermal.day_of_year,
                minimum: Some(1.0),
                maximum: Some(366.0),
            });
        }
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from(PL_RUNTIME_DAY_SYMBOL),
            sdate,
            Some(1.0),
            Some(366.0),
        )?;
        let seasonal_temperature_curve =
            inputs.thermal.seasonal_temperature_curve;
        Ok(ActiveFrostThermalContext {
            snow_depth_m: inputs.thermal.snow_depth_m,
            snow_density_kg_m3: inputs.thermal.snow_density_kg_m3,
            ksnowf: controls.ksnowf,
            residue_depth_m: inputs.thermal.residue_depth_m,
            conductivity_residue_w_m_k,
            snow_conductivity_w_m_k,
            seasonal_temperature_curve,
            sdate,
            kfactor_selected,
        })
    }

    fn frost_branch_matches(actual: f64, expected: f64) -> bool {
        (actual - expected).abs() <= WB11_ZERO_THRESHOLD
    }

    fn apply_active_frost_freeze_step(
        context: &ActiveFrostHourlyContext<'_>,
        shadow_fine_state: &mut FrostFineShadowState,
        hourly: &mut FrostHourlyState,
        surface_temp_c: f64,
        lower_front_heat_w_m2: f64,
        fgthwd_flag: &mut f64,
        freeze_started: &mut bool,
    ) {
        if !Self::frost_branch_matches(hourly.frzflg, 1.0)
            && !Self::frost_branch_matches(hourly.frzflg, 2.0)
        {
            return;
        }

        Self::freeze_fine_front_with_resistance_feedback(
            &mut shadow_fine_state.fine_layers,
            &mut shadow_fine_state.layer_state,
            context.layer_water_state,
            hourly.frzflg,
            surface_temp_c,
            lower_front_heat_w_m2,
            context.thermal.snow_depth_m,
            context.thermal.snow_conductivity_w_m_k,
            context.thermal.residue_depth_m,
            context.thermal.conductivity_residue_w_m_k,
            &mut shadow_fine_state.watbtm_m,
        );
        let depth_after = Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let hourly_frdp_m = depth_after.frdp.min(context.profile_depth_m);
        if hourly_frdp_m > WB11_ZERO_THRESHOLD {
            *fgthwd_flag = 0.0;
            if context.effective_prior_frdp_m <= WB11_ZERO_THRESHOLD {
                *freeze_started = true;
            }
        }
        hourly.tilled_frozen_depth_m = hourly_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
        hourly.untilled_frozen_depth_m =
            (hourly_frdp_m - hourly.tilled_frozen_depth_m).max(0.0);
    }

    fn apply_active_frost_thaw_step(
        context: &ActiveFrostHourlyContext<'_>,
        shadow_fine_state: &mut FrostFineShadowState,
        hourly: &mut FrostHourlyState,
        depth_before: FrostDepthSummary,
        surface_temp_c: f64,
        lower_front_heat_w_m2: f64,
        fgthwd_flag: &mut f64,
    ) {
        if depth_before.frdp <= WB11_ZERO_THRESHOLD {
            return;
        }

        if Self::frost_branch_matches(hourly.frzflg, 2.0)
            && lower_front_heat_w_m2 > WB11_ZERO_THRESHOLD
        {
            Self::thaw_fine_bottom_with_resistance_feedback(
                &mut shadow_fine_state.fine_layers,
                &mut shadow_fine_state.layer_state,
                context.layer_water_state,
                context.thermal.seasonal_temperature_curve,
                context.thermal.sdate,
                context.ksoilf,
                &mut shadow_fine_state.watbtm_m,
            );
        } else if Self::frost_branch_matches(hourly.frzflg, 3.0) {
            let watpdg_m = Self::thaw_fine_top_with_resistance_feedback(
                &mut shadow_fine_state.fine_layers,
                &mut shadow_fine_state.layer_state,
                context.layer_water_state,
                surface_temp_c.max(0.0),
                context.thermal.snow_depth_m,
                context.thermal.snow_conductivity_w_m_k,
                context.thermal.residue_depth_m,
                context.thermal.conductivity_residue_w_m_k,
            );
            shadow_fine_state.watpdg_m += watpdg_m;
            if lower_front_heat_w_m2 > WB11_ZERO_THRESHOLD {
                Self::thaw_fine_bottom_with_resistance_feedback(
                    &mut shadow_fine_state.fine_layers,
                    &mut shadow_fine_state.layer_state,
                    context.layer_water_state,
                    context.thermal.seasonal_temperature_curve,
                    context.thermal.sdate,
                    context.ksoilf,
                    &mut shadow_fine_state.watbtm_m,
                );
            }
        } else if Self::frost_branch_matches(hourly.frzflg, 4.0) {
            Self::thaw_fine_bottom_with_resistance_feedback(
                &mut shadow_fine_state.fine_layers,
                &mut shadow_fine_state.layer_state,
                context.layer_water_state,
                context.thermal.seasonal_temperature_curve,
                context.thermal.sdate,
                context.ksoilf,
                &mut shadow_fine_state.watbtm_m,
            );
        }

        let depth_after = Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let mut hourly_frdp_m = depth_after.frdp.min(context.profile_depth_m);
        *fgthwd_flag = if hourly_frdp_m <= WB11_ZERO_THRESHOLD {
            1.0
        } else {
            0.0
        };
        if *fgthwd_flag > 0.0 {
            hourly_frdp_m = 0.0;
        }
        hourly.tilled_frozen_depth_m = hourly_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
        hourly.untilled_frozen_depth_m =
            (hourly_frdp_m - hourly.tilled_frozen_depth_m).max(0.0);
    }

    fn canonicalize_active_frost_fine_layers(
        shadow_fine_state: &mut FrostFineShadowState,
        layer_water_state: &[FrostLayerWaterState],
    ) {
        for fine in &mut shadow_fine_state.fine_layers {
            let Some(water_layer) = layer_water_state
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            Self::canonicalize_fine_layer_liquid_theta(fine, water_layer);
        }
    }

    fn advance_active_frost_hour(
        context: &ActiveFrostHourlyContext<'_>,
        shadow_fine_state: &mut FrostFineShadowState,
        hourly: &mut FrostHourlyState,
        fgthwd_flag: &mut f64,
        freeze_started: &mut bool,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::reset_fine_layer_hour_timers(&mut shadow_fine_state.fine_layers);
        let depth_before = Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let hourly_frdp_m = depth_before.frdp.min(context.profile_depth_m);
        let surface_temp_c = Self::legacy_tmpadj_surface_temperature_from_typed(
            context.phase_class,
            hourly.hour,
            context.hourly_forcing[hourly.hour - 1],
            context.tmpadj,
            context.thermal.snow_depth_m,
            context.thermal.snow_density_kg_m3,
            context.thermal.ksnowf,
            context.thermal.residue_depth_m,
            context.thermal.conductivity_residue_w_m_k,
            depth_before,
        )?;

        let (resistance_m2_c_w, _, ksrf_w_m_k) = Self::frost_surface_heat_path(
            depth_before.frdp,
            context.thermal.snow_depth_m,
            context.thermal.snow_conductivity_w_m_k,
            context.thermal.residue_depth_m,
            context.thermal.conductivity_residue_w_m_k,
            surface_temp_c < 0.0,
            Self::shallow_front_minimum_conduction_path_m(&shadow_fine_state.fine_layers),
        );
        let signed_surface_flux_w_m2 = surface_temp_c / resistance_m2_c_w;
        hourly.surface_temp_c = surface_temp_c;
        let lower_front_heat_w_m2 = Self::lower_front_heat_w_m2(
            context.thermal.seasonal_temperature_curve,
            context.thermal.sdate,
            depth_before.frdp,
            &shadow_fine_state.fine_layers,
            context.layer_water_state,
            context.ksoilf,
        );
        let signed_net_flux_w_m2 = signed_surface_flux_w_m2 + lower_front_heat_w_m2;
        hourly.qsrf_w_m2 = (-signed_surface_flux_w_m2).max(0.0);
        hourly.quf_w_m2 = lower_front_heat_w_m2;
        hourly.frzflg = Self::select_frost_branch(
            signed_surface_flux_w_m2,
            lower_front_heat_w_m2,
            signed_net_flux_w_m2,
            depth_before,
        );

        Self::apply_active_frost_freeze_step(
            context,
            shadow_fine_state,
            hourly,
            surface_temp_c,
            lower_front_heat_w_m2,
            fgthwd_flag,
            freeze_started,
        );
        Self::apply_active_frost_thaw_step(
            context,
            shadow_fine_state,
            hourly,
            depth_before,
            surface_temp_c,
            lower_front_heat_w_m2,
            fgthwd_flag,
        );
        Self::canonicalize_active_frost_fine_layers(shadow_fine_state, context.layer_water_state);
        hourly.ksrf_w_m_k = ksrf_w_m_k.max(WB11_ZERO_THRESHOLD);
        if !Self::frost_branch_matches(hourly.frzflg, 1.0)
            && !Self::frost_branch_matches(hourly.frzflg, 2.0)
            && depth_before.frdp <= WB11_ZERO_THRESHOLD
        {
            hourly.tilled_frozen_depth_m = hourly_frdp_m.min(FROST_RUNTIME_TILLAGE_DEPTH_M);
            hourly.untilled_frozen_depth_m =
                (hourly_frdp_m - hourly.tilled_frozen_depth_m).max(0.0);
        }
        Ok(())
    }

    fn compute_active_frost_hourly_state(
        context: &ActiveFrostHourlyContext<'_>,
        shadow_fine_state: &mut FrostFineShadowState,
        prior_fgthwd_flag: f64,
    ) -> Result<([FrostHourlyState; SIMIMPL29_HOURS_PER_DAY], bool, f64), Wb11HydrologyKernelGuardError>
    {
        let mut freeze_started = false;
        let mut fgthwd_flag = prior_fgthwd_flag;
        let mut hourly_state = std::array::from_fn(|hour_index| FrostHourlyState {
            hour: hour_index + 1,
            frzflg: 0.0,
            qsrf_w_m2: 0.0,
            quf_w_m2: 0.0,
            ksrf_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            surface_temp_c: 0.0,
            tilled_frozen_depth_m: 0.0,
            untilled_frozen_depth_m: 0.0,
        });
        for hourly in &mut hourly_state {
            Self::advance_active_frost_hour(
                context,
                shadow_fine_state,
                hourly,
                &mut fgthwd_flag,
                &mut freeze_started,
            )?;
        }
        Ok((hourly_state, freeze_started, fgthwd_flag))
    }

    fn active_frost_prior_is_zero(
        prior_context: ActiveFrostPriorContext,
        profile_context: ActiveFrostProfileShadowContext,
    ) -> bool {
        prior_context.effective_prior_frdp_m <= WB11_ZERO_THRESHOLD
            && prior_context.prior_ws_frz <= WB11_ZERO_THRESHOLD
            && profile_context.prior_layer_frozen_depth_m <= WB11_ZERO_THRESHOLD
            && profile_context.prior_layer_frozen_store_m <= WB11_ZERO_THRESHOLD
            && profile_context.prior_fine_frozen_store_m <= WB11_ZERO_THRESHOLD
            && profile_context.prior_depth_summary.frdp <= WB11_ZERO_THRESHOLD
    }

    fn active_frost_zero_prior_can_start_freeze(
        phase_class: HillslopeKernelPhaseClass,
        hourly_forcing: &[DirectFrostHourlyForcing; SIMIMPL29_HOURS_PER_DAY],
        tmpadj: ActiveFrostTmpadjContext,
        layer_water_state: &[FrostLayerWaterState],
        shadow_fine_state: &FrostFineShadowState,
        thermal_context: ActiveFrostThermalContext,
        ksoilf: f64,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let depth_summary = FrostDepthSummary {
            frdp: 0.0,
            thdp: 0.0,
            tfrdp: 0.0,
            tthawd: 0.0,
        };
        let shallow_minimum_path_m =
            Self::shallow_front_minimum_conduction_path_m(&shadow_fine_state.fine_layers);
        let lower_front_heat_w_m2 = Self::lower_front_heat_w_m2(
            thermal_context.seasonal_temperature_curve,
            thermal_context.sdate,
            0.0,
            &shadow_fine_state.fine_layers,
            layer_water_state,
            ksoilf,
        );
        for hour in 1..=SIMIMPL29_HOURS_PER_DAY {
            let surface_temp_c = Self::legacy_tmpadj_surface_temperature_from_typed(
                phase_class,
                hour,
                hourly_forcing[hour - 1],
                tmpadj,
                thermal_context.snow_depth_m,
                thermal_context.snow_density_kg_m3,
                thermal_context.ksnowf,
                thermal_context.residue_depth_m,
                thermal_context.conductivity_residue_w_m_k,
                depth_summary,
            )?;
            let (resistance_m2_c_w, _, _) = Self::frost_surface_heat_path(
                0.0,
                thermal_context.snow_depth_m,
                thermal_context.snow_conductivity_w_m_k,
                thermal_context.residue_depth_m,
                thermal_context.conductivity_residue_w_m_k,
                surface_temp_c < 0.0,
                shallow_minimum_path_m,
            );
            let signed_surface_flux_w_m2 = surface_temp_c / resistance_m2_c_w;
            let signed_net_flux_w_m2 = signed_surface_flux_w_m2 + lower_front_heat_w_m2;
            let branch = Self::select_frost_branch(
                signed_surface_flux_w_m2,
                lower_front_heat_w_m2,
                signed_net_flux_w_m2,
                depth_summary,
            );
            if Self::frost_branch_matches(branch, 1.0)
                || Self::frost_branch_matches(branch, 2.0)
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn no_freeze_active_frost_outcome(
        phase_class: HillslopeKernelPhaseClass,
        hourly_forcing: &[DirectFrostHourlyForcing; SIMIMPL29_HOURS_PER_DAY],
        tmpadj: ActiveFrostTmpadjContext,
        prior_context: ActiveFrostPriorContext,
        thermal_context: ActiveFrostThermalContext,
        soil_conductivity: f64,
        ksoilf: f64,
        total_fine_layer_count: usize,
        layer_water_state: &[FrostLayerWaterState],
        shadow_fine_state: &FrostFineShadowState,
    ) -> Result<FrostCouplingOutcome, Wb11HydrologyKernelGuardError> {
        let fine_layer_diagnostic_state = Self::build_frost_fine_layer_diagnostic_state(
            phase_class,
            layer_water_state.len(),
            shadow_fine_state,
            layer_water_state,
        )?;
        let depth_summary = FrostDepthSummary {
            frdp: 0.0,
            thdp: 0.0,
            tfrdp: 0.0,
            tthawd: 0.0,
        };
        let shallow_minimum_path_m =
            Self::shallow_front_minimum_conduction_path_m(&shadow_fine_state.fine_layers);
        let lower_front_heat_w_m2 = Self::lower_front_heat_w_m2(
            thermal_context.seasonal_temperature_curve,
            thermal_context.sdate,
            0.0,
            &shadow_fine_state.fine_layers,
            layer_water_state,
            ksoilf,
        );
        let mut hourly_state = std::array::from_fn(|hour_index| FrostHourlyState {
            hour: hour_index + 1,
            frzflg: 0.0,
            surface_temp_c: 0.0,
            qsrf_w_m2: 0.0,
            quf_w_m2: 0.0,
            ksrf_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            tilled_frozen_depth_m: 0.0,
            untilled_frozen_depth_m: 0.0,
        });
        for hourly in &mut hourly_state {
            let surface_temp_c = Self::legacy_tmpadj_surface_temperature_from_typed(
                phase_class,
                hourly.hour,
                hourly_forcing[hourly.hour - 1],
                tmpadj,
                thermal_context.snow_depth_m,
                thermal_context.snow_density_kg_m3,
                thermal_context.ksnowf,
                thermal_context.residue_depth_m,
                thermal_context.conductivity_residue_w_m_k,
                depth_summary,
            )?;
            let (resistance_m2_c_w, _, ksrf_w_m_k) = Self::frost_surface_heat_path(
                0.0,
                thermal_context.snow_depth_m,
                thermal_context.snow_conductivity_w_m_k,
                thermal_context.residue_depth_m,
                thermal_context.conductivity_residue_w_m_k,
                surface_temp_c < 0.0,
                shallow_minimum_path_m,
            );
            let signed_surface_flux_w_m2 = surface_temp_c / resistance_m2_c_w;
            let signed_net_flux_w_m2 = signed_surface_flux_w_m2 + lower_front_heat_w_m2;
            hourly.surface_temp_c = surface_temp_c;
            hourly.qsrf_w_m2 = (-signed_surface_flux_w_m2).max(0.0);
            hourly.quf_w_m2 = lower_front_heat_w_m2;
            hourly.frzflg = Self::select_frost_branch(
                signed_surface_flux_w_m2,
                lower_front_heat_w_m2,
                signed_net_flux_w_m2,
                depth_summary,
            );
            hourly.ksrf_w_m_k = ksrf_w_m_k.max(WB11_ZERO_THRESHOLD);
        }
        Ok(FrostCouplingOutcome {
            dthaw: 0.0,
            nft: prior_context.prior_nft,
            ws_frz: 0.0,
            infcap_frz: soil_conductivity,
            soil_water_after_frwatc: None,
            frwatc_soil_water_before: prior_context.soil_water,
            frwatc_soil_water_after: prior_context.soil_water,
            frwatc_frozen_water_before: prior_context.prior_ws_frz,
            frwatc_frozen_water_after: 0.0,
            frwatc_freeze_debit: 0.0,
            frwatc_thaw_credit: 0.0,
            frwatc_net_liquid_delta: 0.0,
            frdp_m: 0.0,
            thdp_m: 0.0,
            tfrdp_m: 0.0,
            tthawd_m: 0.0,
            fgthwd_flag: prior_context.fgthwd_flag,
            total_fine_layer_count: Self::diagnostic_count_to_f64(total_fine_layer_count),
            conductivity_tilled_w_m_k: FROST_RUNTIME_KFTILL_W_M_K,
            conductivity_untilled_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            conductivity_residue_w_m_k: thermal_context.conductivity_residue_w_m_k,
            shadow_total_water_before_m: shadow_fine_state.total_water_before_m,
            shadow_total_water_after_m: shadow_fine_state.total_water_after_m,
            shadow_wb_delta_m: 0.0,
            shadow_frwatc_residual_m: 0.0,
            watpdg_m: 0.0,
            watbtm_m: 0.0,
            hourly_state,
            layer_topology_state: Vec::new(),
            shadow_layer_state: shadow_fine_state
                .layer_state
                .iter()
                .map(|layer| FrostLayerShadowState {
                    layer_index: layer.layer_index,
                    st_m: layer.st_m,
                    soil_water_m: layer.soil_water_m,
                    frozen_depth_m: layer.frozen_m,
                    frzw_m: layer.frzw_m,
                    soilf_m: layer.soilf_m,
                    yst_m: layer.yst_m,
                    nwfrzz_m: layer.nwfrzz_m,
                })
                .collect(),
            fine_layer_state: fine_layer_diagnostic_state,
        })
    }

    fn validate_aggregated_active_frost_layers(
        phase_class: HillslopeKernelPhaseClass,
        shadow_fine_state: &mut FrostFineShadowState,
        layer_water_state: &mut [FrostLayerWaterState],
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::aggregate_active_layers_from_fine_state(
            &mut shadow_fine_state.fine_layers,
            &mut shadow_fine_state.layer_state,
            layer_water_state,
        );
        for fine in &shadow_fine_state.fine_layers {
            let Some(water_layer) = layer_water_state
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                continue;
            };
            Self::require_shadow_fine_state_domains(phase_class, fine, water_layer)?;
        }
        Ok(())
    }

    fn resolve_active_frost_soil_water_after(
        context: ActiveFrostCompletionContext,
        shadow_fine_state: &FrostFineShadowState,
        frwatc_freeze_exchange: f64,
        frwatc_thaw_release: f64,
    ) -> Result<(Option<f64>, f64, f64), Wb11HydrologyKernelGuardError> {
        let raw_frwatc_soil_water_after = shadow_fine_state
            .layer_state
            .iter()
            .map(|layer| layer.soil_water_m)
            .sum::<f64>();
        let material_frwatc_exchange = frwatc_freeze_exchange > WB11_ZERO_THRESHOLD
            || frwatc_thaw_release > WB11_ZERO_THRESHOLD
            || shadow_fine_state.watpdg_m > WB11_ZERO_THRESHOLD
            || shadow_fine_state.watbtm_m > WB11_ZERO_THRESHOLD;
        let mut frwatc_net_liquid_delta =
            raw_frwatc_soil_water_after - context.prior.soil_water;
        if !material_frwatc_exchange && frwatc_net_liquid_delta.abs() <= WB11_ZERO_THRESHOLD {
            frwatc_net_liquid_delta = 0.0;
        }
        let raw_frwatc_soil_water_after = if material_frwatc_exchange {
            raw_frwatc_soil_water_after
        } else {
            context.prior.soil_water + frwatc_net_liquid_delta
        };
        if raw_frwatc_soil_water_after < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class: context.phase_class,
                symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                value: raw_frwatc_soil_water_after,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let frwatc_soil_water_after = raw_frwatc_soil_water_after.max(0.0);
        let soil_water_after_frwatc = if material_frwatc_exchange {
            Some(frwatc_soil_water_after)
        } else {
            None
        };
        Ok((
            soil_water_after_frwatc,
            frwatc_soil_water_after,
            frwatc_net_liquid_delta,
        ))
    }

    fn require_active_frost_final_ranges(
        context: ActiveFrostCompletionContext,
        scalars: &ActiveFrostFinalScalars,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DFROST,
            scalars.dfrost,
            Some(0.0),
            Some(context.prior.profile_depth_m),
        )?;
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DTHAW,
            scalars.dthaw,
            Some(0.0),
            Some(context.prior.profile_depth_m),
        )?;
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            scalars.nft,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            scalars.ws_frz,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            context.phase_class,
            WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ,
            scalars.infcap_frz,
            Some(0.0),
            Some(context.soil_conductivity),
        )?;
        Ok(())
    }

    fn compute_active_frost_final_scalars(
        context: ActiveFrostCompletionContext,
        shadow_fine_state: &FrostFineShadowState,
        layer_water_state: &[FrostLayerWaterState],
    ) -> Result<ActiveFrostFinalScalars, Wb11HydrologyKernelGuardError> {
        let final_depth =
            Self::derived_frost_depths_from_fine_state(&shadow_fine_state.fine_layers);
        let dfrost = final_depth.frdp.min(context.prior.profile_depth_m);
        let thdp_m = final_depth.thdp;
        let tfrdp_m = final_depth.tfrdp;
        let tthawd_m = final_depth.tthawd;
        let bottom_retreat_m = (context.prior.prior_depth_summary.frdp - dfrost).max(0.0);
        let dthaw = thdp_m.max(bottom_retreat_m);
        let nft = if context.freeze_started {
            context.prior.prior_nft + 1.0
        } else {
            context.prior.prior_nft
        };
        let ws_frz = Self::frost_layer_soilf_sum(layer_water_state);
        let raw_frwatc_freeze_exchange =
            if ws_frz > context.prior.prior_ws_frz + WB11_ZERO_THRESHOLD {
                ws_frz - context.prior.prior_ws_frz
            } else {
                0.0
            };
        let frwatc_freeze_exchange =
            if raw_frwatc_freeze_exchange > context.prior.soil_water
                && raw_frwatc_freeze_exchange <= context.prior.soil_water + WB11_ZERO_THRESHOLD
            {
                context.prior.soil_water
            } else {
                raw_frwatc_freeze_exchange
            };
        let frwatc_thaw_release =
            if context.prior.prior_ws_frz > ws_frz + WB11_ZERO_THRESHOLD {
                context.prior.prior_ws_frz - ws_frz
            } else {
                0.0
            };
        if frwatc_freeze_exchange > context.prior.soil_water + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class: context.phase_class,
                symbol: BoundarySymbol::from(WB11_SYMBOL_SOIL_WATER),
                value: context.prior.soil_water,
                minimum: Some(frwatc_freeze_exchange),
                maximum: None,
            });
        }

        let (soil_water_after_frwatc, frwatc_soil_water_after, frwatc_net_liquid_delta) =
            Self::resolve_active_frost_soil_water_after(
                context,
                shadow_fine_state,
                frwatc_freeze_exchange,
                frwatc_thaw_release,
            )?;
        let freeze_fraction = (dfrost / FROST_RUNTIME_TILLAGE_DEPTH_M).clamp(0.0, 1.0);
        let infcap_frz = context.soil_conductivity
            * (1.0 - freeze_fraction + freeze_fraction * context.thermal.kfactor_selected);

        let scalars = ActiveFrostFinalScalars {
            dfrost,
            dthaw,
            nft,
            ws_frz,
            infcap_frz,
            soil_water_after_frwatc,
            frwatc_soil_water_after,
            frwatc_freeze_exchange,
            frwatc_thaw_release,
            frwatc_net_liquid_delta,
            tfrdp_m,
            tthawd_m,
            fgthwd_flag: context.fgthwd_flag,
        };
        Self::require_active_frost_final_ranges(context, &scalars)?;
        Ok(scalars)
    }

    fn build_frost_fine_layer_diagnostic_state(
        phase_class: HillslopeKernelPhaseClass,
        layer_count: usize,
        shadow_fine_state: &FrostFineShadowState,
        layer_water_state: &[FrostLayerWaterState],
    ) -> Result<Vec<FrostFineLayerDiagnosticState>, Wb11HydrologyKernelGuardError> {
        let mut fine_layer_diagnostic_state =
            Vec::with_capacity(shadow_fine_state.fine_layers.len());
        for fine in &shadow_fine_state.fine_layers {
            let Some(water_layer) = layer_water_state
                .iter()
                .find(|layer| layer.layer_index == fine.layer_index)
            else {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: Self::frost_fine_layer_symbol(
                        FROST_RUNTIME_FINE_FGFRST_ROOT,
                        fine.layer_index,
                        fine.fine_index,
                    ),
                    value: Self::diagnostic_count_to_f64(fine.layer_index),
                    minimum: Some(1.0),
                    maximum: Some(Self::diagnostic_count_to_f64(layer_count)),
                });
            };
            let slsic_capacity_m = Self::fine_layer_ice_capacity_m(water_layer, fine);
            let slsw_theta_capacity = Self::fine_layer_liquid_theta_capacity(water_layer);
            let slsw_theta = Self::canonicalize_near_upper_bound(
                Self::canonicalize_near_lower_bound(fine.slsw_theta, water_layer.thetdr),
                slsw_theta_capacity,
            );
            fine_layer_diagnostic_state.push(FrostFineLayerDiagnosticState {
                layer_index: fine.layer_index,
                fine_index: fine.fine_index,
                fgfrst: fine.fgfrst,
                slfsd_m: fine.slfsd_m,
                slsic_m: Self::canonicalize_near_upper_bound(fine.slsic_m, slsic_capacity_m),
                slsw_theta,
                sltime_s: fine.sltime_s,
            });
        }
        Ok(fine_layer_diagnostic_state)
    }

    fn assemble_active_frost_outcome(
        context: ActiveFrostCompletionContext,
        scalars: ActiveFrostFinalScalars,
        total_fine_layer_count: usize,
        shadow_fine_state: FrostFineShadowState,
        hourly_state: &[FrostHourlyState; SIMIMPL29_HOURS_PER_DAY],
        layer_water_state: Vec<FrostLayerWaterState>,
        fine_layer_diagnostic_state: Vec<FrostFineLayerDiagnosticState>,
    ) -> FrostCouplingOutcome {
        FrostCouplingOutcome {
            dthaw: scalars.dthaw,
            nft: scalars.nft,
            ws_frz: scalars.ws_frz,
            infcap_frz: scalars.infcap_frz,
            soil_water_after_frwatc: scalars.soil_water_after_frwatc,
            frwatc_soil_water_before: context.prior.soil_water,
            frwatc_soil_water_after: scalars.frwatc_soil_water_after,
            frwatc_frozen_water_before: context.prior.prior_ws_frz,
            frwatc_frozen_water_after: scalars.ws_frz,
            frwatc_freeze_debit: scalars.frwatc_freeze_exchange,
            frwatc_thaw_credit: scalars.frwatc_thaw_release,
            frwatc_net_liquid_delta: scalars.frwatc_net_liquid_delta,
            frdp_m: scalars.dfrost,
            thdp_m: scalars.dthaw,
            tfrdp_m: scalars.tfrdp_m,
            tthawd_m: scalars.tthawd_m,
            fgthwd_flag: scalars.fgthwd_flag,
            total_fine_layer_count: Self::diagnostic_count_to_f64(total_fine_layer_count),
            conductivity_tilled_w_m_k: FROST_RUNTIME_KFTILL_W_M_K,
            conductivity_untilled_w_m_k: FROST_RUNTIME_KFUTIL_W_M_K,
            conductivity_residue_w_m_k: context.thermal.conductivity_residue_w_m_k,
            shadow_total_water_before_m: shadow_fine_state.total_water_before_m,
            shadow_total_water_after_m: shadow_fine_state.total_water_after_m,
            shadow_wb_delta_m: shadow_fine_state.wb_delta_m,
            shadow_frwatc_residual_m: shadow_fine_state.residual_m,
            watpdg_m: shadow_fine_state.watpdg_m,
            watbtm_m: shadow_fine_state.watbtm_m,
            hourly_state: *hourly_state,
            layer_topology_state: layer_water_state
                .into_iter()
                .map(|layer| FrostLayerTopologyState {
                    layer_index: layer.layer_index,
                    theta_after_m: layer.theta_m,
                    frozen_depth_m: layer.frozen_depth_m,
                    frzw_m: layer.frzw_m,
                })
                .collect(),
            shadow_layer_state: shadow_fine_state
                .layer_state
                .into_iter()
                .map(|layer| FrostLayerShadowState {
                    layer_index: layer.layer_index,
                    st_m: layer.st_m,
                    soil_water_m: layer.soil_water_m,
                    frozen_depth_m: layer.frozen_m,
                    frzw_m: layer.frzw_m,
                    soilf_m: layer.soilf_m,
                    yst_m: layer.yst_m,
                    nwfrzz_m: layer.nwfrzz_m,
                })
                .collect(),
            fine_layer_state: fine_layer_diagnostic_state,
        }
    }

    fn finalize_active_frost_coupling(
        context: ActiveFrostCompletionContext,
        mut shadow_fine_state: FrostFineShadowState,
        hourly_state: &[FrostHourlyState; SIMIMPL29_HOURS_PER_DAY],
        mut layer_water_state: Vec<FrostLayerWaterState>,
        total_fine_layer_count: usize,
    ) -> Result<FrostCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::validate_aggregated_active_frost_layers(
            context.phase_class,
            &mut shadow_fine_state,
            &mut layer_water_state,
        )?;
        let scalars =
            Self::compute_active_frost_final_scalars(context, &shadow_fine_state, &layer_water_state)?;
        let fine_layer_diagnostic_state = Self::build_frost_fine_layer_diagnostic_state(
            context.phase_class,
            layer_water_state.len(),
            &shadow_fine_state,
            &layer_water_state,
        )?;
        Ok(Self::assemble_active_frost_outcome(
            context,
            scalars,
            total_fine_layer_count,
            shadow_fine_state,
            hourly_state,
            layer_water_state,
            fine_layer_diagnostic_state,
        ))
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn compute_active_frost_coupling_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveFrostPartitionInputs,
    ) -> Result<FrostCouplingOutcome, Wb11HydrologyKernelGuardError> {
        let controls = Self::require_active_frost_controls_from_typed(inputs.controls, phase_class)?;
        let (total_fine_layer_count, layer_water_state) =
            Self::require_active_frost_layer_water_state_from_typed(
                &inputs.layers,
                phase_class,
                controls,
            )?;
        let (mut shadow_fine_state, profile_shadow_context) =
            Self::require_frost_profile_shadow_context_from_typed(
                inputs,
                phase_class,
                &layer_water_state,
            )?;
        let prior_shadow_fine_state = shadow_fine_state.clone();
        let prior_context =
            Self::require_typed_active_frost_storage_inputs(inputs, phase_class, profile_shadow_context)?;
        let thermal_context =
            Self::require_active_frost_thermal_context_from_typed(inputs, phase_class, controls)?;
        let tmpadj = ActiveFrostTmpadjContext {
            wind_m_s: inputs.thermal.wind_m_s,
            albedo: inputs.thermal.albedo,
            canopy_height_m: inputs.thermal.canopy_height_m,
            random_roughness_m: inputs.thermal.random_roughness_m,
        };
        if Self::active_frost_prior_is_zero(prior_context, profile_shadow_context)
            && !Self::active_frost_zero_prior_can_start_freeze(
                phase_class,
                &inputs.hourly,
                tmpadj,
                &layer_water_state,
                &prior_shadow_fine_state,
                thermal_context,
                controls.ksoilf,
            )?
        {
            let outcome = Self::no_freeze_active_frost_outcome(
                phase_class,
                &inputs.hourly,
                tmpadj,
                prior_context,
                thermal_context,
                inputs.soil_conductivity_m_s,
                controls.ksoilf,
                total_fine_layer_count,
                &layer_water_state,
                &prior_shadow_fine_state,
            )?;
            let hourly_state = outcome.hourly_state;
            maybe_write_r7g_frost_trace(
                "direct-typed-active-frost",
                Some(thermal_context.sdate),
                None,
                prior_context,
                profile_shadow_context,
                thermal_context,
                tmpadj,
                &inputs.hourly,
                &prior_shadow_fine_state,
                &hourly_state,
                &outcome,
                true,
            );
            return Ok(outcome);
        }
        let hourly_context = ActiveFrostHourlyContext {
            phase_class,
            hourly_forcing: &inputs.hourly,
            tmpadj,
            layer_water_state: &layer_water_state,
            profile_depth_m: prior_context.profile_depth_m,
            effective_prior_frdp_m: prior_context.effective_prior_frdp_m,
            thermal: thermal_context,
            ksoilf: controls.ksoilf,
        };
        let (hourly_state, freeze_started, fgthwd_flag) = Self::compute_active_frost_hourly_state(
            &hourly_context,
            &mut shadow_fine_state,
            prior_context.fgthwd_flag,
        )?;
        let completion_context = ActiveFrostCompletionContext {
            phase_class,
            prior: prior_context,
            thermal: thermal_context,
            soil_conductivity: inputs.soil_conductivity_m_s,
            freeze_started,
            fgthwd_flag,
        };
        let outcome = Self::finalize_active_frost_coupling(
            completion_context,
            shadow_fine_state,
            &hourly_state,
            layer_water_state,
            total_fine_layer_count,
        )?;
        maybe_write_r7g_frost_trace(
            "direct-typed-active-frost",
            Some(thermal_context.sdate),
            None,
            prior_context,
            profile_shadow_context,
            thermal_context,
            tmpadj,
            &inputs.hourly,
            &prior_shadow_fine_state,
            &hourly_state,
            &outcome,
            false,
        );
        Ok(outcome)
    }

}
