fn direct_snow_trace_accumulation_melt_fields(
    diagnostics: &openwepp_hillslope_orchestrator::DirectSnowAccumulationMeltDiagnostics,
    applicable: bool,
) -> String {
    let rows = (0..24)
        .map(|index| {
            let melt = diagnostics.hourly_melt[index];
            let hydrometeor_temperature = diagnostics.hourly_hydrometeor_temperature_c[index]
                .map_or_else(|| "null".to_string(), direct_production_trace_number);
            format!(
                "{{\"hour\":{},\"active_precipitation_m\":{},\"rain_m\":{},\"snowfall_depth_m\":{},\"snowfall_swe_m\":{},\"air_temperature_c\":{},\"radiation_mj_m2\":{},\"cloud_fraction\":{},\"rain_fraction\":{},\"snow_fraction\":{},\"phase_model\":\"{}\",\"hydrometeor_temperature_c\":{},\"coe_melt_amelt_m\":{},\"coe_melt_bmelt_m\":{},\"coe_melt_cmelt_m\":{},\"coe_melt_dmelt_m\":{},\"coe_melt_uncapped_m\":{},\"coe_melt_cap_adjustment_m\":{},\"coe_melt_applied_m\":{},\"routed_melt_m\":{},\"liquid_holding_capacity_m\":{},\"liquid_water_retained_before_m\":{},\"liquid_water_retained_after_m\":{},\"liquid_water_released_m\":{},\"rain_released_m\":{},\"sublimation_m\":{},\"pack_depth_before_m\":{},\"pack_depth_after_m\":{},\"pack_density_before_kg_m3\":{},\"pack_density_after_kg_m3\":{},\"modeled_wind_redistribution_m\":{}}}",
                index + 1,
                direct_production_trace_number(
                    diagnostics.hourly_active_precipitation_m[index],
                ),
                direct_production_trace_number(diagnostics.hourly_rain_m[index]),
                direct_production_trace_number(diagnostics.hourly_snowfall_depth_m[index]),
                direct_production_trace_number(diagnostics.hourly_snowfall_swe_m[index]),
                direct_production_trace_number(diagnostics.hourly_air_temperature_c[index]),
                direct_production_trace_number(diagnostics.hourly_radiation_mj_m2[index]),
                direct_production_trace_number(diagnostics.hourly_cloud_fraction[index]),
                direct_production_trace_number(diagnostics.hourly_rain_fraction[index]),
                direct_production_trace_number(diagnostics.hourly_snow_fraction[index]),
                diagnostics.hourly_phase_model[index].id(),
                hydrometeor_temperature,
                direct_production_trace_number(melt.coe_melt_amelt_m),
                direct_production_trace_number(melt.coe_melt_bmelt_m),
                direct_production_trace_number(melt.coe_melt_cmelt_m),
                direct_production_trace_number(melt.coe_melt_dmelt_m),
                direct_production_trace_number(melt.coe_melt_uncapped_m),
                direct_production_trace_number(melt.coe_melt_cap_adjustment_m),
                direct_production_trace_number(melt.coe_melt_applied_m),
                direct_production_trace_number(diagnostics.hourly_routed_melt_m[index]),
                direct_production_trace_number(
                    diagnostics.hourly_liquid_holding_capacity_m[index],
                ),
                direct_production_trace_number(
                    diagnostics.hourly_liquid_water_retained_before_m[index],
                ),
                direct_production_trace_number(
                    diagnostics.hourly_liquid_water_retained_after_m[index],
                ),
                direct_production_trace_number(
                    diagnostics.hourly_liquid_water_released_m[index],
                ),
                direct_production_trace_number(diagnostics.hourly_rain_released_m[index]),
                direct_production_trace_number(diagnostics.hourly_sublimation_m[index]),
                direct_production_trace_number(diagnostics.hourly_pack_depth_before_m[index]),
                direct_production_trace_number(diagnostics.hourly_pack_depth_after_m[index]),
                direct_production_trace_number(
                    diagnostics.hourly_pack_density_before_kg_m3[index],
                ),
                direct_production_trace_number(
                    diagnostics.hourly_pack_density_after_kg_m3[index],
                ),
                direct_production_trace_number(diagnostics.modeled_wind_redistribution_m[index]),
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "\"accumulation_melt_diagnostic_applicable\":{applicable},\"wind_m_s\":{},\"dewpoint_c\":{},\"canopy_cover_fraction\":{},\"accumulation_melt_hourly\":[{rows}]",
        direct_production_trace_number(diagnostics.wind_m_s),
        direct_production_trace_number(diagnostics.dewpoint_c),
        direct_production_trace_number(diagnostics.canopy_cover_fraction),
    )
}

fn direct_snow_trace_persistent_state(
    state: &openwepp_hillslope_orchestrator::DirectSnowStage3PersistentState,
) -> String {
    let layers = state.layers.iter().map(|layer| format!(
        "{{\"mass_swe_m\":{},\"thickness_m\":{},\"density_kg_m3\":{},\"settle_day_count\":{},\"temperature_c\":{},\"liquid_water_m\":{},\"cold_content_j_m2\":{},\"refrozen_liquid_m\":{}}}",
        direct_production_trace_number(layer.mass_swe_m),
        direct_production_trace_number(layer.thickness_m),
        direct_production_trace_number(layer.density_kg_m3),
        direct_production_trace_number(layer.settle_day_count),
        direct_production_trace_number(layer.temperature_c),
        direct_production_trace_number(layer.liquid_water_m),
        direct_production_trace_number(layer.cold_content_j_m2),
        direct_production_trace_number(layer.refrozen_liquid_m),
    )).collect::<Vec<_>>().join(",");
    format!(
        "{{\"schema_version\":{},\"fingerprint\":\"{:016x}\",\"lane_id\":{},\"next_interval_index\":{},\"layers\":[{}],\"detached_retained_liquid_kg_m2\":{},\"initial_ice_kg_m2\":{},\"initial_retained_liquid_kg_m2\":{},\"cumulative_snowfall_kg_m2\":{},\"cumulative_external_liquid_kg_m2\":{},\"cumulative_deposition_kg_m2\":{},\"cumulative_sublimation_kg_m2\":{},\"cumulative_melt_kg_m2\":{},\"cumulative_unresolved_liquid_kg_m2\":{},\"cumulative_complete_energy_j_m2\":{},\"cumulative_cold_energy_change_j_m2\":{},\"cumulative_terminal_unallocated_energy_j_m2\":{}}}",
        state.schema_version, state.fingerprint, state.lane_id, state.next_interval_index, layers,
        direct_production_trace_number(state.detached_retained_liquid_kg_m2),
        direct_production_trace_number(state.initial_ice_kg_m2),
        direct_production_trace_number(state.initial_retained_liquid_kg_m2),
        direct_production_trace_number(state.cumulative_snowfall_kg_m2),
        direct_production_trace_number(state.cumulative_external_liquid_kg_m2),
        direct_production_trace_number(state.cumulative_deposition_kg_m2),
        direct_production_trace_number(state.cumulative_sublimation_kg_m2),
        direct_production_trace_number(state.cumulative_melt_kg_m2),
        direct_production_trace_number(state.cumulative_unresolved_liquid_kg_m2),
        direct_production_trace_number(state.cumulative_complete_energy_j_m2),
        direct_production_trace_number(state.cumulative_cold_energy_change_j_m2),
        direct_production_trace_number(state.cumulative_terminal_unallocated_energy_j_m2),
    )
}

#[allow(clippy::format_in_format_args)]
fn direct_snow_trace_diagnostic_suffix(
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
    verbose_diagnostics: &openwepp_hillslope_orchestrator::DirectSnowVerboseDiagnostics,
    thermal: &DirectSnowTraceThermalDiagnostics,
    evaluation: Option<&openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationDiagnostics>,
    reconciliation: Option<
        &openwepp_hillslope_orchestrator::DirectSnowStage3OperatorReconciliation,
    >,
    persistent: Option<
        &openwepp_hillslope_orchestrator::DirectSnowStage3PersistentDayResult,
    >,
) -> String {
    let base = format!(
        "{},{},{},{}",
        direct_snow_trace_density_process_fields(&snow_liquid.density_process_diagnostics),
        direct_snow_trace_accumulation_melt_fields(
            &verbose_diagnostics.accumulation_melt,
            snow_liquid.active_snow_coupling,
        ),
        direct_snow_trace_stage3_fields(
            &snow_liquid.stage3_outcome(),
            &snow_liquid.liquid_disposition_ledger(),
            &verbose_diagnostics.stage3,
        ),
        direct_snow_trace_thermal_fields(thermal)
    );
    let persistent = persistent.map(|value| {
        let start_state = direct_snow_trace_persistent_state(&value.start_state);
        let end_state = direct_snow_trace_persistent_state(&value.state);
        let layers = value
            .state
            .layers
            .iter()
            .map(|layer| format!(
                "{{\"mass_swe_m\":{},\"thickness_m\":{},\"density_kg_m3\":{},\"settle_day_count\":{},\"temperature_c\":{},\"liquid_water_m\":{},\"cold_content_j_m2\":{},\"refrozen_liquid_m\":{}}}",
                direct_production_trace_number(layer.mass_swe_m),
                direct_production_trace_number(layer.thickness_m),
                direct_production_trace_number(layer.density_kg_m3),
                direct_production_trace_number(layer.settle_day_count),
                direct_production_trace_number(layer.temperature_c),
                direct_production_trace_number(layer.liquid_water_m),
                direct_production_trace_number(layer.cold_content_j_m2),
                direct_production_trace_number(layer.refrozen_liquid_m),
            ))
            .collect::<Vec<_>>()
            .join(",");
        format!(
        "\"stage3_persistent_start_state\":{},\"stage3_persistent_end_state\":{},{}",
        start_state,
        end_state,
        format!(
        "\"stage3_persistent_state_schema_version\":{},\"stage3_persistent_lane_id\":{},\"stage3_persistent_next_interval_index\":{},\"stage3_persistent_start_state_fingerprint\":\"{:016x}\",\"stage3_persistent_end_state_fingerprint\":\"{:016x}\",\"stage3_persistent_state_layers\":[{}],\"stage3_persistent_initial_ice_kg_m2\":{},\"stage3_persistent_initial_retained_liquid_kg_m2\":{},\"stage3_persistent_cumulative_snowfall_kg_m2\":{},\"stage3_persistent_cumulative_external_liquid_kg_m2\":{},\"stage3_persistent_cumulative_deposition_kg_m2\":{},\"stage3_persistent_cumulative_sublimation_kg_m2\":{},\"stage3_persistent_cumulative_melt_kg_m2\":{},\"stage3_persistent_cumulative_unresolved_liquid_kg_m2\":{},\"stage3_persistent_cumulative_complete_energy_j_m2\":{},\"stage3_persistent_cumulative_terminal_unallocated_energy_j_m2\":{},\"stage3_persistent_lifecycle\":\"{}\",\"stage3_persistent_start_ice_kg_m2\":{},\"stage3_persistent_start_retained_liquid_kg_m2\":{},\"stage3_persistent_snowfall_kg_m2\":{},\"stage3_persistent_external_liquid_kg_m2\":{},\"stage3_persistent_deposition_kg_m2\":{},\"stage3_persistent_sublimation_kg_m2\":{},\"stage3_persistent_melt_kg_m2\":{},\"stage3_persistent_end_ice_kg_m2\":{},\"stage3_persistent_end_retained_liquid_kg_m2\":{},\"stage3_persistent_retained_liquid_censored_loss_kg_m2\":{},\"stage3_persistent_ice_mass_closure_residual_kg_m2\":{},\"stage3_persistent_total_water_closure_residual_kg_m2\":{},\"stage3_persistent_unresolved_liquid_kg_m2\":{},\"stage3_persistent_terminal_unallocated_energy_j_m2\":{}",
        value.state.schema_version,
        value.state.lane_id,
        value.state.next_interval_index,
        value.start_state_fingerprint,
        value.end_state_fingerprint,
        layers,
        direct_production_trace_number(value.state.initial_ice_kg_m2),
        direct_production_trace_number(value.state.initial_retained_liquid_kg_m2),
        direct_production_trace_number(value.state.cumulative_snowfall_kg_m2),
        direct_production_trace_number(value.state.cumulative_external_liquid_kg_m2),
        direct_production_trace_number(value.state.cumulative_deposition_kg_m2),
        direct_production_trace_number(value.state.cumulative_sublimation_kg_m2),
        direct_production_trace_number(value.state.cumulative_melt_kg_m2),
        direct_production_trace_number(value.state.cumulative_unresolved_liquid_kg_m2),
        direct_production_trace_number(value.state.cumulative_complete_energy_j_m2),
        direct_production_trace_number(
            value.state.cumulative_terminal_unallocated_energy_j_m2,
        ),
        value.lifecycle,
        direct_production_trace_number(value.start_ice_kg_m2),
        direct_production_trace_number(value.start_retained_liquid_kg_m2),
        direct_production_trace_number(value.snowfall_kg_m2),
        direct_production_trace_number(value.external_liquid_kg_m2),
        direct_production_trace_number(value.deposition_kg_m2),
        direct_production_trace_number(value.sublimation_kg_m2),
        direct_production_trace_number(value.melt_kg_m2),
        direct_production_trace_number(value.end_ice_kg_m2),
        direct_production_trace_number(value.end_retained_liquid_kg_m2),
        direct_production_trace_number(value.retained_liquid_censored_loss_kg_m2),
        direct_production_trace_number(value.ice_mass_closure_residual_kg_m2),
        direct_production_trace_number(value.total_water_closure_residual_kg_m2),
        direct_production_trace_number(value.unresolved_liquid_kg_m2),
        direct_production_trace_number(value.terminal_unallocated_energy_j_m2),
    ))
    });
    match (
        evaluation.map(direct_snow_trace_stage3_evaluation_fields),
        reconciliation.map(direct_snow_trace_stage3_reconciliation_fields),
        persistent,
    ) {
        (Some(evaluation), Some(reconciliation), Some(persistent)) => {
            format!("{base},{evaluation},{reconciliation},{persistent}}}")
        }
        (Some(evaluation), None, Some(persistent)) => {
            format!("{base},{evaluation},{persistent}}}")
        }
        (None, Some(reconciliation), Some(persistent)) => {
            format!("{base},{reconciliation},{persistent}}}")
        }
        (None, None, Some(persistent)) => format!("{base},{persistent}}}"),
        (Some(evaluation), Some(reconciliation), None) => {
            format!("{base},{evaluation},{reconciliation}}}")
        }
        (Some(evaluation), None, None) => format!("{base},{evaluation}}}"),
        (None, None, None) => format!("{base}}}"),
        (None, Some(reconciliation), None) => format!("{base},{reconciliation}}}"),
    }
}

#[cfg(test)]
mod accumulation_melt_trace_tests {
    use super::*;
    use openwepp_hillslope_orchestrator::{
        DirectSnowAccumulationMeltDiagnostics, DirectSnowMeltHourDiagnostics,
        SnowPhasePartitionModel,
    };

    #[test]
    fn real_trace_formatter_preserves_phase_depth_swe_and_distinct_melt_operands() {
        let mut diagnostics = DirectSnowAccumulationMeltDiagnostics {
            wind_m_s: 2.5,
            dewpoint_c: -1.25,
            canopy_cover_fraction: 0.35,
            ..DirectSnowAccumulationMeltDiagnostics::default()
        };
        diagnostics.hourly_active_precipitation_m[0] = 0.005;
        diagnostics.hourly_rain_m[0] = 0.003;
        diagnostics.hourly_snowfall_depth_m[0] = 0.02;
        diagnostics.hourly_snowfall_swe_m[0] = 0.002;
        diagnostics.hourly_air_temperature_c[0] = -2.5;
        diagnostics.hourly_radiation_mj_m2[0] = 0.75;
        diagnostics.hourly_cloud_fraction[0] = 0.65;
        diagnostics.hourly_rain_fraction[0] = 0.6;
        diagnostics.hourly_snow_fraction[0] = 0.4;
        diagnostics.hourly_phase_model[0] = SnowPhasePartitionModel::HarderPomeroyHourly;
        diagnostics.hourly_hydrometeor_temperature_c[0] = Some(-0.75);
        diagnostics.hourly_melt[0] = DirectSnowMeltHourDiagnostics {
            coe_melt_amelt_m: 0.001,
            coe_melt_bmelt_m: 0.002,
            coe_melt_cmelt_m: 0.003,
            coe_melt_dmelt_m: 0.004,
            coe_melt_uncapped_m: 0.01,
            coe_melt_cap_adjustment_m: -0.004,
            coe_melt_applied_m: 0.006,
        };
        diagnostics.hourly_routed_melt_m[0] = 0.0055;
        diagnostics.hourly_liquid_holding_capacity_m[0] = 0.0045;
        diagnostics.hourly_liquid_water_retained_before_m[0] = 0.0015;
        diagnostics.hourly_liquid_water_retained_after_m[0] = 0.0025;
        diagnostics.hourly_liquid_water_released_m[0] = 0.0005;
        diagnostics.hourly_rain_released_m[0] = 0.0004;
        diagnostics.hourly_sublimation_m[0] = 0.0003;
        diagnostics.hourly_pack_depth_before_m[0] = 0.42;
        diagnostics.hourly_pack_depth_after_m[0] = 0.39;
        diagnostics.hourly_pack_density_before_kg_m3[0] = 210.0;
        diagnostics.hourly_pack_density_after_kg_m3[0] = 225.0;

        let json = format!(
            "{{{}}}",
            direct_snow_trace_accumulation_melt_fields(&diagnostics, true)
        );
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("diagnostic suffix must be valid JSON");
        assert_eq!(value["accumulation_melt_diagnostic_applicable"], true);
        assert_eq!(value["wind_m_s"], 2.5);
        assert_eq!(value["dewpoint_c"], -1.25);
        assert_eq!(value["canopy_cover_fraction"], 0.35);
        let hour = &value["accumulation_melt_hourly"][0];
        assert_eq!(hour["phase_model"], "harder_pomeroy_hourly");
        assert_eq!(hour["hydrometeor_temperature_c"], -0.75);
        assert_eq!(hour["active_precipitation_m"], 0.005);
        assert_eq!(hour["rain_m"], 0.003);
        assert_eq!(hour["snowfall_depth_m"], 0.02);
        assert_eq!(hour["snowfall_swe_m"], 0.002);
        assert_ne!(hour["snowfall_depth_m"], hour["snowfall_swe_m"]);
        assert_eq!(hour["air_temperature_c"], -2.5);
        assert_eq!(hour["radiation_mj_m2"], 0.75);
        assert_eq!(hour["cloud_fraction"], 0.65);
        assert_eq!(hour["rain_fraction"], 0.6);
        assert_eq!(hour["snow_fraction"], 0.4);
        assert_eq!(hour["coe_melt_amelt_m"], 0.001);
        assert_eq!(hour["coe_melt_bmelt_m"], 0.002);
        assert_eq!(hour["coe_melt_cmelt_m"], 0.003);
        assert_eq!(hour["coe_melt_dmelt_m"], 0.004);
        assert_eq!(hour["coe_melt_uncapped_m"], 0.01);
        assert_eq!(hour["coe_melt_cap_adjustment_m"], -0.004);
        assert_eq!(hour["coe_melt_applied_m"], 0.006);
        assert_eq!(hour["routed_melt_m"], 0.0055);
        assert_eq!(hour["liquid_holding_capacity_m"], 0.0045);
        assert_eq!(hour["liquid_water_retained_before_m"], 0.0015);
        assert_eq!(hour["liquid_water_retained_after_m"], 0.0025);
        assert_eq!(hour["liquid_water_released_m"], 0.0005);
        assert_eq!(hour["rain_released_m"], 0.0004);
        assert_eq!(hour["sublimation_m"], 0.0003);
        assert_eq!(hour["pack_depth_before_m"], 0.42);
        assert_eq!(hour["pack_depth_after_m"], 0.39);
        assert_eq!(hour["pack_density_before_kg_m3"], 210.0);
        assert_eq!(hour["pack_density_after_kg_m3"], 225.0);
        assert_eq!(hour["modeled_wind_redistribution_m"], 0.0);
    }
}
