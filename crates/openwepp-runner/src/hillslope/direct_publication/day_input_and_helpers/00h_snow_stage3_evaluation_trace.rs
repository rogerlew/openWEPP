#[allow(clippy::too_many_lines)]
fn direct_snow_trace_stage3_evaluation_fields(
    evaluation: &openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationDiagnostics,
) -> String {
    let pairing_id = evaluation
        .pairing_id
        .map_or_else(|| "null".to_string(), |value| format!("\"{value}\""));
    let hourly_values = |value: fn(
        &openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationHourDiagnostics,
    ) -> f64| {
        let values = evaluation
            .hourly
            .iter()
            .map(|hour| direct_production_trace_number(value(hour)))
            .collect::<Vec<_>>()
            .join(",");
        format!("[{values}]")
    };
    let hourly_shortwave = hourly_values(|hour| {
        hour.shortwave_energy_j_m2
    });
    let hourly_longwave = hourly_values(|hour| {
        hour.longwave_energy_j_m2
    });
    let hourly_sensible = hourly_values(|hour| {
        hour.sensible_flux_w_m2 * 3_600.0
    });
    let hourly_latent = hourly_values(|hour| {
        hour.latent_flux_w_m2 * 3_600.0
    });
    let hourly_advected = hourly_values(|hour| {
        hour.advected_flux_w_m2 * 3_600.0
    });
    let hourly_internal_conduction = hourly_values(|hour| {
        hour.internal_active_lower_conduction_j_m2
    });
    let hourly_cold_content_export = hourly_values(|hour| {
        hour.cold_content_export_j_m2
    });
    let hourly_requested = hourly_values(|hour| {
        hour.requested_seconds
    });
    let hourly_evaluated = hourly_values(|hour| {
        hour.evaluated_seconds
    });
    let hourly_vapor = hourly_values(|hour| {
        hour.vapor_mass_exchange_kg_m2
    });
    let hourly_complete = hourly_values(|hour| {
        hour.complete_energy_j_m2
    });
    let hourly_cold_required = hourly_values(|hour| {
        hour.cold_required_j_m2
    });
    let hourly_cold_change = hourly_values(|hour| {
        hour.cold_energy_change_j_m2
    });
    let hourly_excess = hourly_values(|hour| {
        hour.excess_energy_j_m2
    });
    let hourly_available_ice = hourly_values(|hour| {
        hour.ice_available_kg_m2
    });
    let hourly_sublimation = hourly_values(|hour| {
        hour.sublimation_kg_m2
    });
    let hourly_melt = hourly_values(|hour| hour.melt_kg_m2);
    let hourly_terminal = hourly_values(|hour| {
        hour.unallocated_after_exhaustion_j_m2
    });
    let hourly_residual = hourly_values(|hour| {
        hour.energy_closure_residual_j_m2
    });
    let hourly_carrier_evaluated = evaluation
        .hourly
        .iter()
        .map(|hour| hour.complete_carrier_evaluated.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "\"stage3_evaluation_operator_id\":\"{}\",\
\"stage3_evaluation_source_snapshot_id\":\"{}\",\
\"stage3_evaluation_support_id\":\"{}\",\
\"stage3_evaluation_cadence_id\":\"{}\",\
\"stage3_evaluation_carrier_id\":\"{}\",\
\"stage3_evaluation_coverage_id\":\"{}\",\
\"stage3_evaluation_claim_class\":\"{}\",\
\"stage3_evaluation_unresolved_boundaries_id\":\"{}\",\
\"stage3_evaluation_pairing_id\":{},\
\"stage3_evaluation_arm_ids\":[\"{}\",\"{}\"],\
\"stage3_evaluation_arm_count\":{},\
\"stage3_evaluation_source_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_forcing_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_geometry_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_non_formulation_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_surface_arm_non_formulation_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_complete_arm_non_formulation_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_requested_seconds\":{},\
\"stage3_evaluation_evaluated_seconds\":{},\
\"stage3_evaluation_coverage_fraction\":{},\
\"stage3_evaluation_surface_arm_applicable\":{},\
\"stage3_evaluation_surface_arm_shortwave_j_m2\":{},\
\"stage3_evaluation_surface_arm_longwave_j_m2\":{},\
\"stage3_evaluation_surface_arm_latent_j_m2\":{},\
\"stage3_evaluation_surface_arm_sensible_applicable\":{},\
\"stage3_evaluation_surface_arm_advected_applicable\":{},\
\"stage3_evaluation_surface_arm_internal_conduction_applicable\":{},\
\"stage3_evaluation_surface_arm_total_j_m2\":{},\
\"stage3_evaluation_complete_arm_shortwave_j_m2\":{},\
\"stage3_evaluation_complete_arm_longwave_j_m2\":{},\
\"stage3_evaluation_complete_arm_sensible_j_m2\":{},\
\"stage3_evaluation_complete_arm_latent_j_m2\":{},\
\"stage3_evaluation_complete_arm_advected_j_m2\":{},\
\"stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2\":{},\
\"stage3_evaluation_complete_arm_applicable\":{},\
\"stage3_evaluation_complete_arm_internal_conduction_applicable\":{},\
\"stage3_evaluation_complete_arm_vapor_mass_exchange_kg_m2\":{},\
\"stage3_evaluation_complete_arm_cold_content_export_j_m2\":{},\
\"stage3_evaluation_complete_arm_cold_content_export_applicable\":{},\
\"stage3_evaluation_complete_arm_available_ice_kg_m2\":{},\
\"stage3_evaluation_complete_arm_available_ice_applicable\":{},\
\"stage3_evaluation_complete_arm_total_j_m2\":{},\
\"stage3_evaluation_complete_arm_sequential_ledger_applicable\":{},\
\"stage3_evaluation_complete_arm_cold_energy_change_j_m2\":{},\
\"stage3_evaluation_complete_arm_excess_energy_j_m2\":{},\
\"stage3_evaluation_complete_arm_sublimation_kg_m2\":{},\
\"stage3_evaluation_complete_arm_melt_kg_m2\":{},\
\"stage3_evaluation_complete_arm_terminal_unallocated_j_m2\":{},\
\"stage3_evaluation_complete_arm_terminal_unallocated_applicable\":{},\
\"stage3_evaluation_complete_arm_component_residual_j_m2\":{},\
\"stage3_evaluation_complete_arm_maximum_thermodynamic_residual_j_m2\":{},\
\"stage3_evaluation_hourly_shortwave_j_m2\":{},\
\"stage3_evaluation_hourly_longwave_j_m2\":{},\
\"stage3_evaluation_hourly_sensible_j_m2\":{},\
\"stage3_evaluation_hourly_latent_j_m2\":{},\
\"stage3_evaluation_hourly_advected_j_m2\":{},\
\"stage3_evaluation_hourly_internal_active_lower_conduction_j_m2\":{},\
\"stage3_evaluation_hourly_cold_content_export_j_m2\":{},\
\"stage3_evaluation_hourly_vapor_mass_exchange_kg_m2\":{},\
\"stage3_evaluation_hourly_complete_energy_j_m2\":{},\
\"stage3_evaluation_hourly_cold_required_j_m2\":{},\
\"stage3_evaluation_hourly_cold_energy_change_j_m2\":{},\
\"stage3_evaluation_hourly_excess_energy_j_m2\":{},\
\"stage3_evaluation_hourly_available_ice_kg_m2\":{},\
\"stage3_evaluation_hourly_sublimation_kg_m2\":{},\
\"stage3_evaluation_hourly_melt_kg_m2\":{},\
\"stage3_evaluation_hourly_terminal_unallocated_j_m2\":{},\
\"stage3_evaluation_hourly_energy_closure_residual_j_m2\":{},\
\"stage3_evaluation_hourly_complete_carrier_evaluated\":[{}],\
\"stage3_evaluation_hourly_requested_seconds\":{},\
\"stage3_evaluation_hourly_evaluated_seconds\":{}",
        evaluation.operator.id(),
        evaluation.source_snapshot_id,
        evaluation.support_id,
        evaluation.cadence_id,
        evaluation.carrier_id,
        evaluation.coverage_id,
        evaluation.claim_class,
        evaluation.unresolved_boundaries_id,
        pairing_id,
        evaluation.arm_ids[0],
        evaluation.arm_ids[1],
        evaluation.arm_count,
        evaluation.source_fingerprint,
        evaluation.forcing_fingerprint,
        evaluation.geometry_fingerprint,
        evaluation.non_formulation_fingerprint,
        evaluation.surface_arm_non_formulation_fingerprint,
        evaluation.complete_arm_non_formulation_fingerprint,
        direct_production_trace_number(evaluation.requested_seconds),
        direct_production_trace_number(evaluation.evaluated_seconds),
        direct_production_trace_number(evaluation.coverage_fraction),
        evaluation.surface_arm_applicable,
        direct_production_trace_number(evaluation.surface_arm_shortwave_j_m2),
        direct_production_trace_number(evaluation.surface_arm_longwave_j_m2),
        direct_production_trace_number(evaluation.surface_arm_latent_j_m2),
        evaluation.surface_arm_sensible_applicable,
        evaluation.surface_arm_advected_applicable,
        evaluation.surface_arm_internal_conduction_applicable,
        direct_production_trace_number(evaluation.surface_arm_total_j_m2),
        direct_production_trace_number(evaluation.complete_arm_shortwave_j_m2),
        direct_production_trace_number(evaluation.complete_arm_longwave_j_m2),
        direct_production_trace_number(evaluation.complete_arm_sensible_j_m2),
        direct_production_trace_number(evaluation.complete_arm_latent_j_m2),
        direct_production_trace_number(evaluation.complete_arm_advected_j_m2),
        direct_production_trace_number(
            evaluation.complete_arm_internal_active_lower_conduction_j_m2
        ),
        evaluation.complete_arm_applicable,
        evaluation.complete_arm_internal_conduction_applicable,
        direct_production_trace_number(
            evaluation.complete_arm_vapor_mass_exchange_kg_m2
        ),
        direct_production_trace_number(
            evaluation.complete_arm_cold_content_export_j_m2
        ),
        evaluation.complete_arm_cold_content_export_applicable,
        direct_production_trace_number(evaluation.complete_arm_available_ice_kg_m2),
        evaluation.complete_arm_available_ice_applicable,
        direct_production_trace_number(evaluation.complete_arm_total_j_m2),
        evaluation.complete_arm_sequential_ledger_applicable,
        direct_production_trace_number(evaluation.complete_arm_cold_energy_change_j_m2),
        direct_production_trace_number(evaluation.complete_arm_excess_energy_j_m2),
        direct_production_trace_number(evaluation.complete_arm_sublimation_kg_m2),
        direct_production_trace_number(evaluation.complete_arm_melt_kg_m2),
        direct_production_trace_number(
            evaluation.complete_arm_terminal_unallocated_j_m2
        ),
        evaluation.complete_arm_terminal_unallocated_applicable,
        direct_production_trace_number(evaluation.complete_arm_component_residual_j_m2),
        direct_production_trace_number(
            evaluation.complete_arm_maximum_thermodynamic_residual_j_m2
        ),
        hourly_shortwave,
        hourly_longwave,
        hourly_sensible,
        hourly_latent,
        hourly_advected,
        hourly_internal_conduction,
        hourly_cold_content_export,
        hourly_vapor,
        hourly_complete,
        hourly_cold_required,
        hourly_cold_change,
        hourly_excess,
        hourly_available_ice,
        hourly_sublimation,
        hourly_melt,
        hourly_terminal,
        hourly_residual,
        hourly_carrier_evaluated,
        hourly_requested,
        hourly_evaluated,
    )
}

#[cfg(test)]
mod stage3_v11_trace_tests {
    use super::*;
    use openwepp_hillslope_orchestrator::{
        DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowLayerState,
        DirectSnowSurfaceEnergyOptions, DirectSnowTurbulentGeometry, SnowDensityModel,
        SnowMeltModel, SnowStage3LiquidRoutingModel, SnowSurfaceLongwaveModel,
        SnowSurfaceSublimationModel, Wb11HydrologyKernel,
    };

    fn retired_day_entry_input() -> DirectActiveSnowPartitionInputs {
        let mut layer = DirectSnowLayerState::new(0.18, 0.40, 450.0, 12.0);
        layer.temperature_c = -8.0;
        layer.cold_content_j_m2 = 0.18 * 1_000.0 * 2_100.0 * 8.0;
        let hourly = [DirectSnowHourlyForcing {
            radiation_mj_m2: 0.2,
            air_temperature_c: -5.0,
            ..DirectSnowHourlyForcing::zero()
        }; 24];
        DirectActiveSnowPartitionInputs {
            hyetograph_rainfall_m: 0.0,
            rst_c: 0.0,
            newsnw_kg_m3: 100.0,
            ssd_kg_m3: 522.0,
            runtime_swe_m: 0.18,
            runtime_depth_m: 0.40,
            runtime_density_kg_m3: 450.0,
            runtime_settle_day_count: 12.0,
            liquid_water_retained_m: 0.0,
            tmax_c: -3.0,
            tmin_c: -7.0,
            canopy_cover_fraction: 0.45,
            wind_m_s: 3.0,
            dewpoint_c: -15.0,
            snow_melt_model: SnowMeltModel::AdaptiveCompositionalStage3V1,
            snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
            stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
            surface_energy_options: DirectSnowSurfaceEnergyOptions {
                longwave_model: SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
                sublimation_model: SnowSurfaceSublimationModel::NeutralBulkStage3V1,
                daily_solar_radiation_mj_m2: 5.0,
                daily_extraterrestrial_radiation_mj_m2: 10.0,
                daylight: true,
                atmospheric_pressure_pa: 101_324.6,
                turbulent_geometry: DirectSnowTurbulentGeometry::CLIGEN_V1,
                complete_carrier_shadow: false,
            },
            sturm_climate_class: None,
            sturm_day_of_year: None,
            coe_boundary_depth_m: 0.40,
            coe_boundary_density_kg_m3: 450.0,
            coe_boundary_settle_day_count: 12.0,
            snow_albedo_model: None,
            snow_albedo_state: None,
            snow_layers: vec![layer],
            underlying_surface_albedo: 0.2,
            hourly,
        }
    }

    #[test]
    fn retired_day_oriented_stage3_entry_fails_closed() {
        let error = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
            &retired_day_entry_input(),
        )
        .expect_err("retired day-oriented Stage3 entry must fail closed");
        assert!(
            format!("{error:?}").contains("snow.adaptive_stage3_legacy_sublimation_entry"),
            "retired entry must identify the authoritative V11 cutover guard: {error:?}"
        );
    }

    #[test]
    fn trace_without_retired_day_payloads_stays_on_the_base_schema() {
        assert_eq!(
            direct_snow_trace_schema(None, None, None),
            "openwepp-r7h-direct-production-snow-trace-v4"
        );
    }
}
