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
                "{{\"hour\":{},\"active_precipitation_m\":{},\"rain_m\":{},\"snowfall_depth_m\":{},\"snowfall_swe_m\":{},\"rain_fraction\":{},\"snow_fraction\":{},\"phase_model\":\"{}\",\"hydrometeor_temperature_c\":{},\"coe_melt_amelt_m\":{},\"coe_melt_bmelt_m\":{},\"coe_melt_cmelt_m\":{},\"coe_melt_dmelt_m\":{},\"coe_melt_uncapped_m\":{},\"coe_melt_cap_adjustment_m\":{},\"coe_melt_applied_m\":{},\"modeled_wind_redistribution_m\":{}}}",
                index + 1,
                direct_production_trace_number(
                    diagnostics.hourly_active_precipitation_m[index],
                ),
                direct_production_trace_number(diagnostics.hourly_rain_m[index]),
                direct_production_trace_number(diagnostics.hourly_snowfall_depth_m[index]),
                direct_production_trace_number(diagnostics.hourly_snowfall_swe_m[index]),
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
                direct_production_trace_number(diagnostics.modeled_wind_redistribution_m[index]),
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "\"accumulation_melt_diagnostic_applicable\":{applicable},\"accumulation_melt_hourly\":[{rows}]"
    )
}

fn direct_snow_trace_diagnostic_suffix(
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
    thermal: &DirectSnowTraceThermalDiagnostics,
) -> String {
    format!(
        "{},{},{},{}",
        direct_snow_trace_density_process_fields(&snow_liquid.density_process_diagnostics),
        direct_snow_trace_accumulation_melt_fields(
            &snow_liquid.accumulation_melt_diagnostics,
            snow_liquid.active_snow_coupling,
        ),
        direct_snow_trace_stage3_fields(&snow_liquid.stage3_diagnostics),
        direct_snow_trace_thermal_fields(thermal)
    )
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
        let mut diagnostics = DirectSnowAccumulationMeltDiagnostics::default();
        diagnostics.hourly_active_precipitation_m[0] = 0.005;
        diagnostics.hourly_rain_m[0] = 0.003;
        diagnostics.hourly_snowfall_depth_m[0] = 0.02;
        diagnostics.hourly_snowfall_swe_m[0] = 0.002;
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

        let json = format!(
            "{{{}}}",
            direct_snow_trace_accumulation_melt_fields(&diagnostics, true)
        );
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("diagnostic suffix must be valid JSON");
        assert_eq!(value["accumulation_melt_diagnostic_applicable"], true);
        let hour = &value["accumulation_melt_hourly"][0];
        assert_eq!(hour["phase_model"], "harder_pomeroy_hourly");
        assert_eq!(hour["hydrometeor_temperature_c"], -0.75);
        assert_eq!(hour["active_precipitation_m"], 0.005);
        assert_eq!(hour["rain_m"], 0.003);
        assert_eq!(hour["snowfall_depth_m"], 0.02);
        assert_eq!(hour["snowfall_swe_m"], 0.002);
        assert_ne!(hour["snowfall_depth_m"], hour["snowfall_swe_m"]);
        assert_eq!(hour["rain_fraction"], 0.6);
        assert_eq!(hour["snow_fraction"], 0.4);
        assert_eq!(hour["coe_melt_amelt_m"], 0.001);
        assert_eq!(hour["coe_melt_bmelt_m"], 0.002);
        assert_eq!(hour["coe_melt_cmelt_m"], 0.003);
        assert_eq!(hour["coe_melt_dmelt_m"], 0.004);
        assert_eq!(hour["coe_melt_uncapped_m"], 0.01);
        assert_eq!(hour["coe_melt_cap_adjustment_m"], -0.004);
        assert_eq!(hour["coe_melt_applied_m"], 0.006);
        assert_eq!(hour["modeled_wind_redistribution_m"], 0.0);
    }
}
