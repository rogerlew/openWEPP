macro_rules! direct_trace_json_object {
    ({$($key:literal: $value:expr),* $(,)?}) => {{
        let mut object = serde_json::Map::new();
        $(object.insert($key.to_string(), serde_json::json!($value));)*
        serde_json::Value::Object(object)
    }};
}

fn direct_snow_trace_schema(
    evaluation: Option<&openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationDiagnostics>,
    reconciliation: Option<
        &openwepp_hillslope_orchestrator::DirectSnowStage3OperatorReconciliation,
    >,
) -> &'static str {
    if reconciliation.is_some() {
        "openwepp-r7h-direct-production-snow-trace-v6"
    } else if evaluation.is_some() {
        "openwepp-r7h-direct-production-snow-trace-v5"
    } else {
        "openwepp-r7h-direct-production-snow-trace-v4"
    }
}

#[allow(clippy::too_many_lines)]
fn direct_snow_trace_stage3_reconciliation_fields(
    reconciliation: &openwepp_hillslope_orchestrator::DirectSnowStage3OperatorReconciliation,
) -> String {
    let hourly_status = reconciliation
        .hourly_status
        .iter()
        .map(|status| {
            direct_trace_json_object!({
                "evaluated": status.evaluated,
                "reason": status.reason,
            })
        })
        .collect::<Vec<_>>();
    let tuples = reconciliation
        .tuples
        .iter()
        .map(|tuple| {
            direct_trace_json_object!({
                "operator_id": tuple.operator.id(),
                "hour_index": tuple.hour_index,
                "substep_index": tuple.substep_index,
                "elapsed_start_seconds": tuple.elapsed_start_seconds,
                "requested_seconds": tuple.requested_seconds,
                "evaluated_seconds": tuple.evaluated_seconds,
                "duration_seconds": tuple.duration_seconds,
                "applicable": tuple.applicable,
                "applicability_reason": tuple.applicability_reason,
                "source_fingerprint_fnv1a64": format!("{:016x}", tuple.source_fingerprint_fnv1a64),
                "forcing_fingerprint_fnv1a64": format!("{:016x}", tuple.forcing_fingerprint_fnv1a64),
                "geometry_fingerprint_fnv1a64": format!("{:016x}", tuple.geometry_fingerprint_fnv1a64),
                "effective_input_fingerprint_fnv1a64": format!("{:016x}", tuple.effective_input_fingerprint_fnv1a64),
                "projection_id": tuple.projection_id,
                "active_layer_prefix_count_before": tuple.active_layer_prefix_count_before,
                "total_layer_count_before": tuple.total_layer_count_before,
                "active_layer_state_fingerprint_before_fnv1a64": format!("{:016x}", tuple.active_layer_state_fingerprint_before_fnv1a64),
                "total_layer_state_fingerprint_before_fnv1a64": format!("{:016x}", tuple.total_layer_state_fingerprint_before_fnv1a64),
                "active_layer_prefix_count_after": tuple.active_layer_prefix_count_after,
                "total_layer_count_after": tuple.total_layer_count_after,
                "active_layer_state_fingerprint_after_fnv1a64": tuple.active_layer_state_fingerprint_after_fnv1a64.map(|value| format!("{value:016x}")),
                "total_layer_state_fingerprint_after_fnv1a64": format!("{:016x}", tuple.total_layer_state_fingerprint_after_fnv1a64),
                "after_surface_applicable": tuple.after_surface_applicable,
                "after_surface_applicability_reason": tuple.after_surface_applicability_reason,
                "active_ice_mass_before_kg_m2": tuple.active_ice_mass_before_kg_m2,
                "active_ice_mass_after_kg_m2": tuple.active_ice_mass_after_kg_m2,
                "total_ice_mass_before_kg_m2": tuple.total_ice_mass_before_kg_m2,
                "total_ice_mass_after_kg_m2": tuple.total_ice_mass_after_kg_m2,
                "active_depth_before_m": tuple.active_depth_before_m,
                "active_depth_after_m": tuple.active_depth_after_m,
                "active_density_before_kg_m3": tuple.active_density_before_kg_m3,
                "active_density_after_kg_m3": tuple.active_density_after_kg_m3,
                "active_cold_before_j_m2": tuple.active_cold_before_j_m2,
                "active_cold_after_j_m2": tuple.active_cold_after_j_m2,
                "total_cold_before_j_m2": tuple.total_cold_before_j_m2,
                "total_cold_after_j_m2": tuple.total_cold_after_j_m2,
                "surface_temperature_before_c": tuple.surface_temperature_before_c,
                "surface_temperature_after_c": tuple.surface_temperature_after_c,
                "air_temperature_c": tuple.air_temperature_c,
                "dewpoint_c": tuple.dewpoint_c,
                "wind_speed_m_s": tuple.wind_speed_m_s,
                "air_pressure_pa": tuple.air_pressure_pa,
                "hourly_radiation_mj_m2": tuple.hourly_radiation_mj_m2,
                "daily_solar_radiation_mj_m2": tuple.daily_solar_radiation_mj_m2,
                "daily_extraterrestrial_radiation_mj_m2": tuple.daily_extraterrestrial_radiation_mj_m2,
                "daylight": tuple.daylight,
                "canopy_cover_fraction": tuple.canopy_cover_fraction,
                "rain_m": tuple.rain_m,
                "snowfall_geometric_m": tuple.snowfall_geometric_m,
                "rain_mass_flux_kg_m2_s": tuple.rain_mass_flux_kg_m2_s,
                "snow_mass_flux_kg_m2_s": tuple.snow_mass_flux_kg_m2_s,
                "rain_temperature_c": tuple.rain_temperature_c,
                "snow_temperature_c": tuple.snow_temperature_c,
                "rain_specific_heat_j_kg_k": tuple.rain_specific_heat_j_kg_k,
                "snow_specific_heat_j_kg_k": tuple.snow_specific_heat_j_kg_k,
                "incoming_shortwave_w_m2": tuple.incoming_shortwave_w_m2,
                "snow_albedo_fraction": tuple.snow_albedo_fraction,
                "snow_albedo_source_id": tuple.snow_albedo_source_id,
                "snow_albedo_model_id": tuple.snow_albedo_model_id,
                "snow_albedo_accumulated_positive_temperature_c_day": tuple.snow_albedo_accumulated_positive_temperature_c_day,
                "net_shortwave_w_m2": tuple.net_shortwave_w_m2,
                "actual_vapor_pressure_pa": tuple.actual_vapor_pressure_pa,
                "longwave_cloud_fraction": tuple.longwave_cloud_fraction,
                "sky_view_fraction": tuple.sky_view_fraction,
                "atmospheric_longwave_w_m2": tuple.atmospheric_longwave_w_m2,
                "canopy_longwave_w_m2": tuple.canopy_longwave_w_m2,
                "subcanopy_longwave_w_m2": tuple.subcanopy_longwave_w_m2,
                "outgoing_longwave_w_m2": tuple.outgoing_longwave_w_m2,
                "net_longwave_w_m2": tuple.net_longwave_w_m2,
                "air_temperature_height_m": tuple.air_temperature_height_m,
                "vapor_pressure_height_m": tuple.vapor_pressure_height_m,
                "wind_speed_height_m": tuple.wind_speed_height_m,
                "aerodynamic_roughness_length_m": tuple.aerodynamic_roughness_length_m,
                "turbulent_max_iterations": tuple.turbulent_max_iterations,
                "turbulent_convergence_tolerance": tuple.turbulent_convergence_tolerance,
                "surface_vapor_pressure_pa": tuple.surface_vapor_pressure_pa,
                "air_potential_temperature_k": tuple.air_potential_temperature_k,
                "surface_temperature_k": tuple.surface_temperature_k,
                "specific_humidity_air_kg_kg": tuple.specific_humidity_air_kg_kg,
                "specific_humidity_surface_kg_kg": tuple.specific_humidity_surface_kg_kg,
                "air_density_kg_m3": tuple.air_density_kg_m3,
                "displacement_height_m": tuple.displacement_height_m,
                "log_momentum": tuple.log_momentum,
                "log_sensible": tuple.log_sensible,
                "log_latent": tuple.log_latent,
                "turbulent_termination_status": tuple.turbulent_termination_status,
                "stability_class": tuple.stability_class,
                "obukhov_length_m": tuple.obukhov_length_m,
                "psi_momentum": tuple.psi_momentum,
                "psi_sensible": tuple.psi_sensible,
                "psi_latent": tuple.psi_latent,
                "turbulent_iterations": tuple.turbulent_iterations,
                "friction_velocity_m_s": tuple.friction_velocity_m_s,
                "sensible_exchange_velocity_m_s": tuple.sensible_exchange_velocity_m_s,
                "latent_exchange_velocity_m_s": tuple.latent_exchange_velocity_m_s,
                "surface_latent_heat_j_kg": tuple.surface_latent_heat_j_kg,
                "vapor_mass_flux_kg_m2_s": tuple.vapor_mass_flux_kg_m2_s,
                "sensible_flux_w_m2": tuple.sensible_flux_w_m2,
                "latent_flux_w_m2": tuple.latent_flux_w_m2,
                "precipitation_advected_flux_w_m2": tuple.precipitation_advected_flux_w_m2,
                "complete_external_flux_w_m2": tuple.complete_external_flux_w_m2,
                "vapor_mass_exchange_kg_m2": tuple.vapor_mass_exchange_kg_m2,
                "sublimation_kg_m2": tuple.sublimation_kg_m2,
                "deposition_kg_m2": tuple.deposition_kg_m2,
                "melt_kg_m2": tuple.melt_kg_m2,
                "active_cold_energy_change_j_m2": tuple.active_cold_energy_change_j_m2,
                "lower_cold_energy_change_j_m2": tuple.lower_cold_energy_change_j_m2,
                "cold_content_export_j_m2": tuple.cold_content_export_j_m2,
                "internal_active_lower_conduction_j_m2": tuple.internal_active_lower_conduction_j_m2,
                "legacy_sequential_complete_j_m2": tuple.legacy_sequential_complete_j_m2,
                "energy_closure_residual_j_m2": tuple.energy_closure_residual_j_m2,
            })
        })
        .collect::<Vec<_>>();
    let value = serde_json::json!({
        "schema_version": reconciliation.schema_version,
        "hourly_status": hourly_status,
        "tuples": tuples,
    });
    format!("\"stage3_operator_reconciliation\":{value}")
}
