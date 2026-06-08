pub(super) fn pl_schedule_slot_symbol(root: &str, slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_{root}")
}

pub(super) fn pl_schedule_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

pub(super) fn pl_growth_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

pub(super) fn hphys0245_trace_config_from_env() -> Result<Option<Hphys0245TraceConfig>, HillslopeCliError> {
    let Some(path_value) = std::env::var_os(HPHYS0245_TRACE_PATH_ENV) else {
        return Ok(None);
    };
    if path_value.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "hphys0245_trace",
            detail: format!("{HPHYS0245_TRACE_PATH_ENV} cannot be empty when set"),
        });
    }

    let max_days = match std::env::var(HPHYS0245_TRACE_MAX_DAYS_ENV) {
        Ok(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                let parsed = trimmed.parse::<usize>().map_err(|error| {
                    HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "hphys0245_trace",
                        detail: format!(
                            "{HPHYS0245_TRACE_MAX_DAYS_ENV} must be a positive integer, observed {trimmed}: {error}"
                        ),
                    }
                })?;
                if parsed == 0 {
                    return Err(HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "hphys0245_trace",
                        detail: format!("{HPHYS0245_TRACE_MAX_DAYS_ENV} must be >= 1"),
                    });
                }
                Some(parsed)
            }
        }
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "hphys0245_trace",
                detail: format!("{HPHYS0245_TRACE_MAX_DAYS_ENV} must be valid UTF-8"),
            });
        }
    };

    Ok(Some(Hphys0245TraceConfig {
        path: PathBuf::from(path_value),
        max_days,
    }))
}

pub(super) fn write_hphys0245_trace_jsonl(
    config: &Hphys0245TraceConfig,
    rows: &[Hphys0245TraceRow],
) -> Result<(), HillslopeCliError> {
    crate::hillslope::intake_lane_setup::ensure_output_parent_directory(&config.path)?;
    let mut payload = String::new();
    for row in rows {
        let line = serde_json::to_string(row).map_err(|source| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "hphys0245_trace",
                detail: format!("failed serializing trace row: {source}"),
            }
        })?;
        payload.push_str(&line);
        payload.push('\n');
    }
    fs::write(&config.path, payload).map_err(|source| HillslopeCliError::OutputWrite {
        path: config.path.clone(),
        source,
    })
}

pub(super) fn hphys0245_surface_after_writeback(
    request: &HillslopeKernelRequest<'_>,
    payload: &KernelWritebackPayload,
) -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface {
        state_surface: request.state_surface.clone(),
        flux_surface: request.flux_surface.clone(),
    };
    for field in &payload.state_updates {
        surface
            .state_surface
            .insert(field.symbol.clone(), field.value);
    }
    for field in &payload.flux_updates {
        surface
            .flux_surface
            .insert(field.symbol.clone(), field.value);
    }
    surface
}

pub(super) fn hphys0245_et_seed_branch(runtime_surface: &HillslopeWritebackSurface) -> Option<String> {
    if runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_evappm")
        .is_some_and(|value| value >= 0.5)
    {
        return Some("evappm_pmet".to_string());
    }
    if runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_priestley_taylor")
        .is_some_and(|value| value >= 0.5)
    {
        return Some("evap_priestley_taylor".to_string());
    }
    None
}

pub(super) fn hphys0245_optional_delta(after: Option<f64>, before: Option<f64>) -> Option<f64> {
    match (after, before) {
        (Some(after), Some(before)) => Some(after - before),
        _ => None,
    }
}

#[allow(
    clippy::similar_names,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]
pub(super) fn build_hphys0245_trace_row(
    run_name: &str,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_year: i32,
    julian_day: u16,
    boundary: &str,
    phase: Option<&str>,
    runtime_surface: &HillslopeWritebackSurface,
    wb13_row: Option<&SimulationOwnedWb13Row>,
    snow_runtime_before: Option<Hphys0245SnowRuntimeBeforeState>,
) -> Hphys0245TraceRow {
    let theta_layers =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_theta_");
    let wb18_ul_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_ul_");
    let wb18_thetdr_layers = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_thetdr_",
        "thetdr_",
    );
    let wb18_dg_layers_m = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_dg_",
        "dg_",
    );
    let wb18_fc_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_fc_");
    let wb19_coca_layers = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_coca_",
        "coca_",
    );
    let wb19_frzw_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_frzw_");
    let wb19_drfc_layers_m =
        hphys0245_wb19_drfc_layers(&wb18_fc_layers_m, &wb18_dg_layers_m, &wb19_coca_layers);
    let wb19_fzdrfc_layers_m =
        hphys0245_wb19_fzdrfc_layers(&wb19_drfc_layers_m, &wb19_frzw_layers_m);
    let wb18_frozen_depth_layers_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb18_perc_frozen_depth_",
    );
    let pei_layers =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "wb18_perc_pei_");
    let potential_uptake_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "UPi_");
    let actual_uptake_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "Ui_");
    let wb19_lateral_withdrawal_layers_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_withdrawal_",
    );
    let wb19_lateral_capacity_active_count_layers = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_capacity_active_count_",
    );
    let wb19_lateral_conductivity_active_count_layers = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_conductivity_active_count_",
    );
    let theta_sum = hphys0245_sum_or_none(&theta_layers);
    let pei_sum = hphys0245_sum_or_none(&pei_layers);
    let wb11_soil_water = runtime_surface_symbol_value(runtime_surface, "wb11_soil_water");
    let wb12_infiltration_m = runtime_surface_symbol_value(runtime_surface, "wb12_infiltration");
    let wb12_rainfall_input_m =
        runtime_surface_symbol_value(runtime_surface, "wb12_rainfall_input");
    let wb12_runon_input_m = runtime_surface_symbol_value(runtime_surface, "wb12_runon_input");
    let wb12_depression_storage_delta_m =
        runtime_surface_symbol_value(runtime_surface, "wb12_depression_storage_delta");
    let wb14_soil_conductivity_m_s =
        runtime_surface_symbol_value(runtime_surface, "wb14_soil_conductivity_m_s")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "ssc"));
    let wb14_frost_infcap_m_s =
        runtime_surface_symbol_value(runtime_surface, "frost.runtime_infcap_frz");
    let wb14_effective_conductivity_m_s =
        runtime_surface_symbol_value(runtime_surface, "wb14_effective_conductivity_m_s")
            .or_else(|| wb14_frost_infcap_m_s.or(wb14_soil_conductivity_m_s));
    let wb14_soil_layer_depth_m = runtime_surface_symbol_value(runtime_surface, "dg");
    let wb14_theta_residual = runtime_surface_symbol_value(runtime_surface, "thetdr");
    let wb14_theta_field_capacity = runtime_surface_symbol_value(runtime_surface, "thetfc");
    let wb14_matric_potential_m =
        runtime_surface_symbol_value(runtime_surface, "wb14_matric_potential_m").or_else(
            || match (
                wb14_soil_layer_depth_m,
                wb14_theta_residual,
                wb14_theta_field_capacity,
            ) {
                (Some(depth), Some(theta_residual), Some(theta_field_capacity)) => {
                    Some(depth * (theta_field_capacity - theta_residual).max(0.0))
                }
                _ => None,
            },
        );
    let wb18_recomputed_soil_water_m = hphys0245_recompute_wb18_soil_water(
        &theta_layers,
        &wb18_thetdr_layers,
        &wb18_dg_layers_m,
        &wb18_frozen_depth_layers_m,
    );
    let wb18_recomputed_minus_wb11_m = match (wb18_recomputed_soil_water_m, wb11_soil_water) {
        (Some(recomputed), Some(wb11)) => Some(recomputed - wb11),
        _ => None,
    };
    let wb11_minus_theta_sum_m = match (wb11_soil_water, theta_sum) {
        (Some(wb11), Some(theta)) => Some(wb11 - theta),
        _ => None,
    };
    let wb13_wat = wb13_row.map(|row| &row.wb13_row);
    let effective_pltol = runtime_surface_symbol_value(runtime_surface, "swu_effective_pltol");
    let wb17_swu_stress_threshold_layers_m =
        hphys0245_swu_stress_threshold_layers(&wb18_ul_layers_m, effective_pltol);
    let wb17_swu_storage_to_threshold_layers = hphys0245_swu_storage_to_threshold_layers(
        &theta_layers,
        &wb17_swu_stress_threshold_layers_m,
    );
    let snow_hourly_rain_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_m_",
    ));
    let snow_hourly_snowfall_depth_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.snowfall_m_",
    ));
    let snow_hourly_melt_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.melt_m_",
    ));
    let snow_hourly_melt_raw_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.melt_raw_m_",
    ));
    let snow_hourly_rain_retained_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_retained_m_",
    ));
    let snow_hourly_rain_released_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_released_m_",
    ));
    let snow_hourly_rain_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "snow.hourly.rain_m_");
    let snow_hourly_snowfall_depth_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.snowfall_m_",
    );
    let snow_hourly_stmtim_rain_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.rain_m_",
    );
    let snow_hourly_stmtim_stmdur_s = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.stmdur_s_",
    );
    let snow_hourly_stmtim_wntdur_h = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.wntdur_h_",
    );
    let snow_hourly_stmtim_wnttim_h = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.wnttim_h_",
    );
    let snow_hourly_stmtim_hrtemp_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.hrtemp_c_",
    );
    let snow_hourly_stmtim_rst_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.rst_c_",
    );
    let snow_hourly_stmtim_hrrain_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.hrrain_m_",
    );
    let snow_hourly_stmtim_hrsnow_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.hrsnow_m_",
    );
    let snow_hourly_stmtim_active_interval = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.active_interval_",
    );
    let snow_hourly_stmtim_rain_branch = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.rain_branch_",
    );
    let snow_hourly_stmtim_snow_branch = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.stmtim.snow_branch_",
    );
    let snow_hourly_depth_before_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.depth_before_m_",
    );
    let snow_hourly_depth_available_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.depth_available_m_",
    );
    let snow_hourly_depth_after_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.depth_after_m_",
    );
    let snow_hourly_density_before_kg_m3 = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.density_before_kg_m3_",
    );
    let snow_hourly_density_after_kg_m3 = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.density_after_kg_m3_",
    );
    let snow_hourly_melt_raw_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_raw_m_",
    );
    let snow_hourly_melt_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "snow.hourly.melt_m_");
    let snow_hourly_melt_amelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_amelt_in_",
    );
    let snow_hourly_melt_bmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_bmelt_in_",
    );
    let snow_hourly_melt_cmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_cmelt_in_",
    );
    let snow_hourly_melt_dmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_dmelt_in_",
    );
    let snow_hourly_melt_hrtef_f = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_hrtef_f_",
    );
    let snow_hourly_melt_hrdtf_f = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_hrdtf_f_",
    );
    let snow_hourly_melt_vwmph = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_vwmph_",
    );
    let snow_hourly_melt_rainin = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_rainin_",
    );
    let snow_hourly_melt_wind_adjustment = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_wind_adjustment_",
    );
    let snow_hourly_melt_branch_active = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_branch_active_",
    );
    let winter_hourly_air_temp_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.air_temp_c_",
    );
    let winter_hourly_rad_mj_m2 = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.rad_mj_m2_",
    );
    let winter_hourly_cloud_fraction = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.cloud_fraction_",
    );
    let winter_hourly_dewpoint_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.dewpoint_c_",
    );
    let winter_hourly_wind_m_s = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.wind_m_s_",
    );
    let snow_hourly_snowfall_water_equiv_sum_m = match (
        snow_hourly_snowfall_depth_sum_m,
        runtime_surface_symbol_value(runtime_surface, "snow.options.newsnw"),
    ) {
        (Some(depth_sum_m), Some(new_snow_density_kg_m3)) => {
            Some(depth_sum_m * new_snow_density_kg_m3 / 1_000.0)
        }
        _ => None,
    };
    let snow_s_m = runtime_surface_symbol_value_prefer_flux(runtime_surface, "S");
    let snow_routed_melt_m =
        runtime_surface_flux_symbol_value(runtime_surface, "snow.routed_melt_m");
    let snow_post_winter_rain_m =
        runtime_surface_flux_symbol_value(runtime_surface, "snow.post_winter_rain_m");
    let wb12_partition_liquid_supply_m = match (
        snow_post_winter_rain_m,
        snow_routed_melt_m,
        wb12_runon_input_m,
    ) {
        (Some(post_winter_rain), Some(routed_melt), Some(runon)) => {
            Some(post_winter_rain + routed_melt + runon)
        }
        _ => None,
    };
    let wb12_partition_residual_before_q_m = match (
        wb12_partition_liquid_supply_m,
        wb12_infiltration_m,
        wb12_depression_storage_delta_m,
    ) {
        (Some(supply), Some(infiltration), Some(depression_storage_delta)) => {
            Some(supply - infiltration - depression_storage_delta)
        }
        _ => None,
    };
    let snow_runtime_swe_m = runtime_surface_symbol_value(runtime_surface, "snow.runtime_swe");
    let snow_runtime_depth_m =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_depth_m");
    let snow_runtime_density_kg_m3 =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_density_kg_m3");
    let snow_runtime_settle_day_count =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_settle_day_count");
    let snow_runtime_swe_before_m = snow_runtime_before.and_then(|state| state.swe_m);
    let snow_runtime_depth_before_m = snow_runtime_before.and_then(|state| state.depth_m);
    let snow_runtime_density_before_kg_m3 =
        snow_runtime_before.and_then(|state| state.density_kg_m3);
    let snow_runtime_settle_day_count_before =
        snow_runtime_before.and_then(|state| state.settle_day_count);
    let snow_runtime_swe_delta_m =
        hphys0245_optional_delta(snow_runtime_swe_m, snow_runtime_swe_before_m);
    let snow_runtime_depth_delta_m =
        hphys0245_optional_delta(snow_runtime_depth_m, snow_runtime_depth_before_m);
    let snow_runtime_density_delta_kg_m3 = hphys0245_optional_delta(
        snow_runtime_density_kg_m3,
        snow_runtime_density_before_kg_m3,
    );
    let snow_runtime_settle_day_count_delta = hphys0245_optional_delta(
        snow_runtime_settle_day_count,
        snow_runtime_settle_day_count_before,
    );
    let snow_runtime_swe_closure_error_m = match (
        snow_s_m,
        snow_hourly_melt_sum_m,
        snow_hourly_snowfall_water_equiv_sum_m,
        snow_hourly_rain_retained_sum_m,
        snow_hourly_rain_released_sum_m,
    ) {
        (
            Some(snow_s_m),
            Some(melt_sum_m),
            Some(snowfall_water_equiv_sum_m),
            Some(rain_retained_sum_m),
            Some(rain_released_sum_m),
        ) => Some(
            snow_s_m
                - ((melt_sum_m - rain_released_sum_m)
                    - snowfall_water_equiv_sum_m
                    - rain_retained_sum_m),
        ),
        _ => None,
    };

    Hphys0245TraceRow {
        schema: HPHYS0245_TRACE_SCHEMA,
        run_name: run_name.to_string(),
        sim_day_index,
        simulation_year,
        calendar_year,
        julian_day,
        boundary: boundary.to_string(),
        phase: phase.map(ToString::to_string),
        wb11_soil_water_m: wb11_soil_water,
        wb11_soil_water_mm: wb11_soil_water.map(|value| value * 1_000.0),
        wb12_infiltration_m,
        wb12_rainfall_input_m,
        wb12_runon_input_m,
        wb12_depression_storage_delta_m,
        wb12_partition_liquid_supply_m,
        wb12_partition_residual_before_q_m,
        wb14_soil_conductivity_m_s,
        wb14_frost_infcap_m_s,
        wb14_effective_conductivity_m_s,
        wb14_soil_layer_depth_m,
        wb14_theta_residual,
        wb14_theta_field_capacity,
        wb14_matric_potential_m,
        wb18_theta_sum_m: theta_sum,
        wb18_theta_layers_m: theta_layers,
        wb18_thetdr_layers,
        wb18_dg_layers_m,
        wb18_fc_layers_m,
        wb19_coca_layers,
        wb19_frzw_layers_m,
        wb19_drfc_layers_m,
        wb19_fzdrfc_layers_m,
        wb18_frozen_depth_layers_m,
        wb18_recomputed_soil_water_m,
        wb18_recomputed_minus_wb11_m,
        wb18_pei_sum_m: pei_sum,
        wb18_pei_layers_m: pei_layers,
        d_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "D"),
        pe_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Pe"),
        wb13_dp_mm: wb13_wat.map(|row| row.dp),
        wb13_total_soil_mm: wb13_wat.map(|row| row.total_soil),
        wb13_soil_water_total_mm: wb13_wat.map(|row| row.soil_water_total),
        snow_runtime_swe_m,
        snow_runtime_depth_m,
        snow_runtime_density_kg_m3,
        snow_runtime_settle_day_count,
        snow_runtime_swe_before_m,
        snow_runtime_depth_before_m,
        snow_runtime_density_before_kg_m3,
        snow_runtime_settle_day_count_before,
        snow_runtime_swe_delta_m,
        snow_runtime_depth_delta_m,
        snow_runtime_density_delta_kg_m3,
        snow_runtime_settle_day_count_delta,
        snow_s_m,
        snow_routed_melt_m,
        snow_post_winter_rain_m,
        snow_hourly_rain_sum_m,
        snow_hourly_rain_retained_sum_m,
        snow_hourly_rain_released_sum_m,
        snow_hourly_snowfall_depth_sum_m,
        snow_hourly_snowfall_water_equiv_sum_m,
        snow_hourly_melt_raw_sum_m,
        snow_hourly_melt_sum_m,
        snow_hourly_rain_m,
        snow_hourly_snowfall_depth_m,
        snow_hourly_stmtim_rain_m,
        snow_hourly_stmtim_stmdur_s,
        snow_hourly_stmtim_wntdur_h,
        snow_hourly_stmtim_wnttim_h,
        snow_hourly_stmtim_hrtemp_c,
        snow_hourly_stmtim_rst_c,
        snow_hourly_stmtim_hrrain_m,
        snow_hourly_stmtim_hrsnow_m,
        snow_hourly_stmtim_active_interval,
        snow_hourly_stmtim_rain_branch,
        snow_hourly_stmtim_snow_branch,
        snow_hourly_depth_before_m,
        snow_hourly_depth_available_m,
        snow_hourly_depth_after_m,
        snow_hourly_density_before_kg_m3,
        snow_hourly_density_after_kg_m3,
        snow_hourly_melt_raw_m,
        snow_hourly_melt_m,
        snow_hourly_melt_amelt_in,
        snow_hourly_melt_bmelt_in,
        snow_hourly_melt_cmelt_in,
        snow_hourly_melt_dmelt_in,
        snow_hourly_melt_hrtef_f,
        snow_hourly_melt_hrdtf_f,
        snow_hourly_melt_vwmph,
        snow_hourly_melt_rainin,
        snow_hourly_melt_wind_adjustment,
        snow_hourly_melt_branch_active,
        winter_hourly_air_temp_c,
        winter_hourly_rad_mj_m2,
        winter_hourly_cloud_fraction,
        winter_hourly_dewpoint_c,
        winter_hourly_wind_m_s,
        snow_runtime_swe_closure_error_m,
        wb13_p_mm: wb13_wat.map(|row| row.p),
        wb13_rm_mm: wb13_wat.map(|row| row.rm),
        wb13_q_mm: wb13_wat.map(|row| row.q),
        wb13_snow_water_mm: wb13_wat.map(|row| row.snow_water),
        wb11_minus_theta_sum_m,
        pl_sumgdd: runtime_surface_symbol_value(runtime_surface, "sumgdd"),
        pl_vdmt: runtime_surface_symbol_value(runtime_surface, "vdmt"),
        pl_cancov: runtime_surface_symbol_value(runtime_surface, "cancov"),
        pl_lai: runtime_surface_symbol_value(runtime_surface, "lai"),
        pl_rtmass: runtime_surface_symbol_value(runtime_surface, "rtmass"),
        pl_rtd: runtime_surface_symbol_value(runtime_surface, "rtd"),
        pl_hia: runtime_surface_symbol_value(runtime_surface, "hia"),
        pl_pltol: runtime_surface_symbol_value(runtime_surface, "pltol"),
        pl_swu_effective_pltol: effective_pltol,
        pmet_sidecar_present: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.mode.sidecar_present",
        ),
        pmet_iflget: runtime_surface_symbol_value(runtime_surface, "pmetpara.mode.iflget"),
        pmet_selected_kcb: runtime_surface_symbol_value(runtime_surface, "pmetpara.selected.kcb"),
        pmet_selected_rawp: runtime_surface_symbol_value(runtime_surface, "pmetpara.selected.rawp"),
        pmet_selected_line_index: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.selected.line_index",
        ),
        pmet_lookup_fallback_first_row_used: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.lookup.fallback_first_row_used",
        ),
        wb11_et_demand_m: runtime_surface_symbol_value(runtime_surface, "wb11_et_demand"),
        wb11_et_seed_branch: hphys0245_et_seed_branch(runtime_surface),
        pmet_etorc_mm: runtime_surface_symbol_value(runtime_surface, "pmet.etorc_mm"),
        pmet_rn_mj_m2: runtime_surface_symbol_value(runtime_surface, "pmet.rn_mj_m2"),
        pmet_fwv_m_s: runtime_surface_symbol_value(runtime_surface, "pmet.fwv_m_s"),
        pmet_rhd_pct: runtime_surface_symbol_value(runtime_surface, "pmet.rhd_pct"),
        pmet_kcbadj: runtime_surface_symbol_value(runtime_surface, "pmet.kcbadj"),
        pmet_kcbcon: runtime_surface_symbol_value(runtime_surface, "pmet.kcbcon"),
        pmet_etke: runtime_surface_symbol_value(runtime_surface, "pmet.etke"),
        pmet_etkr: runtime_surface_symbol_value(runtime_surface, "pmet.etkr"),
        pmet_etks: runtime_surface_symbol_value(runtime_surface, "pmet.etks"),
        pmet_tew_mm: runtime_surface_symbol_value(runtime_surface, "pmet.tew_mm"),
        pmet_rew_mm: runtime_surface_symbol_value(runtime_surface, "pmet.rew_mm"),
        pmet_wfevp_mm: runtime_surface_symbol_value(runtime_surface, "pmet.wfevp_mm"),
        pmet_taw_mm: runtime_surface_symbol_value(runtime_surface, "pmet.taw_mm"),
        pmet_raw_mm: runtime_surface_symbol_value(runtime_surface, "pmet.raw_mm"),
        pmet_wftrp_mm: runtime_surface_symbol_value(runtime_surface, "pmet.wftrp_mm"),
        pmet_es_m: runtime_surface_symbol_value(runtime_surface, "pmet.es_m"),
        pmet_ep_m: runtime_surface_symbol_value(runtime_surface, "pmet.ep_m"),
        etp_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Etp"),
        upi_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "UPi"),
        ui_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ui"),
        wb18_ul_layers_m,
        wb17_swu_stress_threshold_layers_m,
        wb17_swu_storage_to_threshold_layers,
        wb17_upi_layers_m: potential_uptake_layers_m,
        wb17_ui_layers_m: actual_uptake_layers_m,
        ep_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ep"),
        ws: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ws"),
        wb19_q_lateral_potential_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_potential",
        ),
        wb19_q_lateral_target_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_target",
        ),
        wb19_lateral_capacity_tdv_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_lateral_capacity_tdv",
        ),
        wb19_tdvv_m: runtime_surface_symbol_value(runtime_surface, "wb19_tdvv"),
        wb19_q_lateral_unrealized_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_unrealized",
        ),
        wb19_lateral_withdrawal_layers_m,
        wb19_lateral_capacity_active_count_layers,
        wb19_lateral_conductivity_active_count_layers,
        q_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "q"),
        qdd_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Qdd"),
        qd_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Qd"),
    }
}

pub(super) fn hphys0245_prefixed_surface_values(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    prefix: &str,
) -> BTreeMap<String, f64> {
    surface
        .iter()
        .filter_map(|(symbol, value)| {
            let symbol = symbol.as_str();
            symbol
                .strip_prefix(prefix)
                .map(|suffix| (suffix.to_string(), value.as_f64()))
        })
        .collect()
}

pub(super) fn hphys0245_prefixed_surface_values_with_fallback(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    preferred_prefix: &str,
    fallback_prefix: &str,
) -> BTreeMap<String, f64> {
    let mut values = hphys0245_prefixed_surface_values(surface, fallback_prefix);
    values.extend(hphys0245_prefixed_surface_values(surface, preferred_prefix));
    values
}

pub(super) fn hphys0245_prefixed_runtime_values(
    runtime_surface: &HillslopeWritebackSurface,
    prefix: &str,
) -> BTreeMap<String, f64> {
    let mut values = hphys0245_prefixed_surface_values(&runtime_surface.state_surface, prefix);
    values.extend(hphys0245_prefixed_surface_values(
        &runtime_surface.flux_surface,
        prefix,
    ));
    values
}

pub(super) fn hphys0245_sum_runtime_prefix(runtime_surface: &HillslopeWritebackSurface, prefix: &str) -> f64 {
    hphys0245_prefixed_runtime_values(runtime_surface, prefix)
        .values()
        .copied()
        .sum()
}

pub(super) fn hphys0245_sum_or_none(values: &BTreeMap<String, f64>) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.values().copied().sum())
    }
}

pub(super) fn hphys0245_swu_stress_threshold_layers(
    ul_layers: &BTreeMap<String, f64>,
    effective_pltol: Option<f64>,
) -> BTreeMap<String, f64> {
    let Some(effective_pltol) = effective_pltol else {
        return BTreeMap::new();
    };
    if !effective_pltol.is_finite() || effective_pltol < 0.0 {
        return BTreeMap::new();
    }
    ul_layers
        .iter()
        .filter_map(|(suffix, ul)| {
            if ul.is_finite() && *ul >= 0.0 {
                let threshold = effective_pltol * *ul;
                if threshold.is_finite() {
                    return Some((suffix.clone(), threshold));
                }
            }
            None
        })
        .collect()
}

pub(super) fn hphys0245_swu_storage_to_threshold_layers(
    theta_layers: &BTreeMap<String, f64>,
    threshold_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    threshold_layers
        .iter()
        .filter_map(|(suffix, threshold)| {
            if !threshold.is_finite() || *threshold <= 0.0 {
                return None;
            }
            let theta = theta_layers.get(suffix)?;
            let ratio = *theta / *threshold;
            if ratio.is_finite() {
                Some((suffix.clone(), ratio))
            } else {
                None
            }
        })
        .collect()
}

pub(super) fn hphys0245_wb19_drfc_layers(
    fc_layers: &BTreeMap<String, f64>,
    dg_layers: &BTreeMap<String, f64>,
    coca_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    fc_layers
        .iter()
        .filter_map(|(suffix, fc)| {
            let dg = dg_layers.get(suffix)?;
            let coca = coca_layers.get(suffix)?;
            let drfc = *fc + ((1.0 - *coca) * *dg);
            if fc.is_finite() && dg.is_finite() && coca.is_finite() && drfc.is_finite() {
                Some((suffix.clone(), drfc))
            } else {
                None
            }
        })
        .collect()
}

pub(super) fn hphys0245_wb19_fzdrfc_layers(
    drfc_layers: &BTreeMap<String, f64>,
    frzw_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    drfc_layers
        .iter()
        .filter_map(|(suffix, drfc)| {
            let frzw = frzw_layers.get(suffix)?;
            let fzdrfc = (*drfc - frzw).max(0.0);
            if drfc.is_finite() && frzw.is_finite() && fzdrfc.is_finite() {
                Some((suffix.clone(), fzdrfc))
            } else {
                None
            }
        })
        .collect()
}

pub(super) fn hphys0245_recompute_wb18_soil_water(
    theta_layers: &BTreeMap<String, f64>,
    thetdr_layers: &BTreeMap<String, f64>,
    dg_layers: &BTreeMap<String, f64>,
    frozen_depth_layers: &BTreeMap<String, f64>,
) -> Option<f64> {
    if theta_layers.is_empty() {
        return None;
    }
    let mut soil_water = 0.0;
    for (suffix, theta) in theta_layers {
        let thetdr = thetdr_layers.get(suffix)?;
        let dg = dg_layers.get(suffix)?;
        let frozen_depth = frozen_depth_layers.get(suffix).copied().unwrap_or(0.0);
        if !theta.is_finite()
            || !thetdr.is_finite()
            || !dg.is_finite()
            || !frozen_depth.is_finite()
            || *thetdr < 0.0
            || *dg <= 0.0
            || frozen_depth < 0.0
            || frozen_depth > *dg
        {
            return None;
        }
        let layer_soil_water = *theta + (*thetdr * (*dg - frozen_depth));
        if !layer_soil_water.is_finite() {
            return None;
        }
        soil_water += layer_soil_water;
    }
    if soil_water.is_finite() {
        Some(soil_water)
    } else {
        None
    }
}

pub(super) fn format_wb12_storage_terms(runtime_surface: &HillslopeWritebackSurface) -> String {
    fn get(runtime_surface: &HillslopeWritebackSurface, symbol: &str) -> String {
        runtime_surface_symbol_value(runtime_surface, symbol)
            .map_or_else(|| "NA".to_string(), |value| format!("{value:.10}"))
    }

    let storage_initial = runtime_surface_symbol_value(runtime_surface, "wb12_storage_initial");
    let precip_input = runtime_surface_symbol_value(runtime_surface, "wb12_precip_input");
    let snow_coupling_s = runtime_surface_symbol_value(runtime_surface, "S");
    let irrigation_input = runtime_surface_symbol_value(runtime_surface, "Irr");
    let interception_i = runtime_surface_symbol_value(runtime_surface, "I");
    let q_runoff = runtime_surface_symbol_value(runtime_surface, "Q");
    let et = runtime_surface_symbol_value(runtime_surface, "ET");
    let percolation_loss = runtime_surface_symbol_value(runtime_surface, "D");
    let subsurface_loss = runtime_surface_symbol_value(runtime_surface, "Qd");
    let reconciled_est = match (
        storage_initial,
        precip_input,
        snow_coupling_s,
        irrigation_input,
        interception_i,
        q_runoff,
        et,
        percolation_loss,
        subsurface_loss,
    ) {
        (
            Some(storage_initial),
            Some(precip_input),
            Some(snow_coupling_s),
            Some(irrigation_input),
            Some(interception_i),
            Some(q_runoff),
            Some(et),
            Some(percolation_loss),
            Some(subsurface_loss),
        ) => format!(
            "{:.10}",
            storage_initial + precip_input + snow_coupling_s + irrigation_input
                - interception_i
                - q_runoff
                - et
                - percolation_loss
                - subsurface_loss
        ),
        _ => "NA".to_string(),
    };

    format!(
        "{{storage_initial={},precip_input={},S={},Irr={},I={},Q={},ET={},D={},Qd={},reconciled_est={}}}",
        get(runtime_surface, "wb12_storage_initial"),
        get(runtime_surface, "wb12_precip_input"),
        get(runtime_surface, "S"),
        get(runtime_surface, "Irr"),
        get(runtime_surface, "I"),
        get(runtime_surface, "Q"),
        get(runtime_surface, "ET"),
        get(runtime_surface, "D"),
        get(runtime_surface, "Qd"),
        reconciled_est
    )
}

#[allow(clippy::too_many_lines)]
pub(super) fn format_wb18_perc_guard_terms(runtime_surface: &HillslopeWritebackSurface) -> String {
    let mut layer_suffixes = runtime_surface
        .state_surface
        .keys()
        .filter_map(|symbol| symbol.as_str().strip_prefix("wb18_perc_fc_"))
        .filter(|suffix| suffix.len() == 4 && suffix.chars().all(|ch| ch.is_ascii_digit()))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    layer_suffixes.sort_unstable();
    layer_suffixes.dedup();

    if layer_suffixes.is_empty() {
        return "{layers=none}".to_string();
    }

    let fmt_opt = |value: Option<f64>| {
        value.map_or_else(|| "NA".to_string(), |observed| format!("{observed:.10}"))
    };
    let fmt_state = |symbol: &str| fmt_opt(runtime_surface_symbol_value(runtime_surface, symbol));

    let invalid_layers = layer_suffixes
        .iter()
        .filter_map(|suffix| {
            let fc =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_fc_{suffix}"))?;
            let ul =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_ul_{suffix}"))?;
            let theta =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_theta_{suffix}"))?;
            let ssc =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_ssc_{suffix}"));
            let thetfc = runtime_surface_symbol_value(runtime_surface, &format!("thetfc_{suffix}"));
            let thetdr = runtime_surface_symbol_value(runtime_surface, &format!("thetdr_{suffix}"));
            let dg = runtime_surface_symbol_value(runtime_surface, &format!("dg_{suffix}"));
            let por = runtime_surface_symbol_value(runtime_surface, &format!("por_{suffix}"));
            let cpm = runtime_surface_symbol_value(runtime_surface, &format!("cpm_{suffix}"));
            let frozen_depth = runtime_surface_symbol_value(
                runtime_surface,
                &format!("wb18_perc_frozen_depth_{suffix}"),
            );
            let ratio = fc / ul;
            let stz = theta / ul;
            let dynamic_branch_active = stz.is_finite() && stz < 0.95;
            let ratio_domain_invalid = !ratio.is_finite() || ratio >= 1.0;
            let legacy_bi_zero_candidate = ratio.is_finite() && ratio <= 0.0;

            let mut flags = Vec::new();
            if !ul.is_finite() || ul <= 0.0 {
                flags.push("ul_nonpositive");
            }
            if !theta.is_finite() || theta < 0.0 {
                flags.push("theta_invalid");
            }
            if ratio_domain_invalid {
                flags.push("fc_ul_ratio_invalid");
            }
            if legacy_bi_zero_candidate {
                flags.push("legacy_bi_zero_candidate");
            }
            if let Some(ssc_value) = ssc
                && (!ssc_value.is_finite() || ssc_value <= 0.0)
            {
                flags.push("ssc_nonpositive");
            }
            if let Some(thetdr_value) = thetdr
                && (!thetdr_value.is_finite() || !(0.0..=1.0).contains(&thetdr_value))
            {
                flags.push("thetdr_out_of_range");
            }
            if let Some(dg_value) = dg
                && (!dg_value.is_finite() || dg_value <= 0.0)
            {
                flags.push("dg_nonpositive");
            }
            if let (Some(frozen_depth_value), Some(dg_value)) = (frozen_depth, dg)
                && (!frozen_depth_value.is_finite()
                    || frozen_depth_value < 0.0
                    || frozen_depth_value > dg_value)
            {
                flags.push("frozen_depth_out_of_range");
            }

            let lower_ratio_summary = suffix
                .parse::<usize>()
                .ok()
                .and_then(|index| {
                    let lower_suffix = format!("{:04}", index + 1);
                    let lower_theta = runtime_surface_symbol_value(
                        runtime_surface,
                        &format!("wb18_perc_theta_{lower_suffix}"),
                    )?;
                    let lower_ul = runtime_surface_symbol_value(
                        runtime_surface,
                        &format!("wb18_perc_ul_{lower_suffix}"),
                    )?;
                    Some(lower_theta / lower_ul)
                })
                .filter(|ratio| !ratio.is_finite() || *ratio < 0.0)
                .map(|ratio| format!("lower_ratio={ratio:.10}"));
            if lower_ratio_summary.is_some() {
                flags.push("lower_ratio_invalid");
            }

            if flags.is_empty() {
                return None;
            }
            Some(format!(
                "L{}(flags={},fc={:.10},ul={:.10},theta={:.10},ratio={:.10},stz={:.10},dynamic_branch_active={},ssc={},thetfc={},thetdr={},dg={},frozen_depth={},por={},cpm={}{})",
                suffix,
                flags.join("+"),
                fc,
                ul,
                theta,
                ratio,
                stz,
                dynamic_branch_active,
                fmt_opt(ssc),
                fmt_opt(thetfc),
                fmt_opt(thetdr),
                fmt_opt(dg),
                fmt_opt(frozen_depth),
                fmt_opt(por),
                fmt_opt(cpm),
                lower_ratio_summary.map_or_else(String::new, |summary| format!(",{summary}")),
            ))
        })
        .collect::<Vec<_>>();

    let invalid_summary = if invalid_layers.is_empty() {
        "none".to_string()
    } else {
        invalid_layers.join("|")
    };

    format!(
        "{{layer_count={},lane_substeps={},infiltration={},tillay2={},slflag={},kslast={},ui_bdrkth={},invalid_layers={}}}",
        layer_suffixes.len(),
        fmt_state("wb18_perc_lane_substeps"),
        fmt_state("wb12_infiltration"),
        fmt_state("management.initial.params.tillay2_m"),
        fmt_state("slflag"),
        fmt_state("kslast"),
        fmt_state("ui_bdrkth"),
        invalid_summary
    )
}

