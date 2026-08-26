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
mod stage3_evaluation_real_consumer_tests {
    use super::*;
    use openwepp_hillslope_orchestrator::{
        DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowLaneState,
        DirectSnowLayerState, DirectSnowLiquidPartition, DirectSnowSurfaceEnergyOptions,
        DirectSnowTurbulentGeometry,
        SnowDensityModel, SnowMeltModel, SnowPhasePartitionModel, SnowStage3EvaluationOperator,
        SnowStage3LiquidRoutingModel, SnowSurfaceLongwaveModel, SnowSurfaceSublimationModel,
        Wb11HydrologyKernel,
    };

    const LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;

    fn poison_production_diagnostics(partition: &mut DirectSnowLiquidPartition) {
        let verbose = partition
            .verbose_diagnostics
            .as_mut()
            .expect("verbose solver diagnostics");
        verbose.stage3.shortwave_energy_j_m2 = 9.91e12;
        verbose.stage3.surface_energy_j_m2 = 9.92e12;
        verbose.stage3.energy_closure_residual_j_m2 = 9.93e12;
        verbose.stage3.hourly_surface_energy[0].net_shortwave_w_m2 = 9.94e12;
    }

    fn evaluated_partition(
        inputs: &DirectActiveSnowPartitionInputs,
        operator: Option<SnowStage3EvaluationOperator>,
        schema6: bool,
    ) -> (
        DirectSnowLiquidPartition,
        Option<openwepp_hillslope_orchestrator::DirectSnowStage3EvaluationDiagnostics>,
        Option<Box<openwepp_hillslope_orchestrator::DirectSnowStage3OperatorReconciliation>>,
    ) {
        match operator {
            Some(operator) if schema6 => {
                let result = Wb11HydrologyKernel::
                    compute_direct_snow_liquid_partition_with_capture_and_reconciliation(
                        inputs,
                        openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture::Verbose,
                        Some(operator),
                    )
                    .expect("solver-produced schema-v6 evaluation partition");
                (
                    result.result.authoritative,
                    result.result.evaluation,
                    result.reconciliation,
                )
            }
            Some(operator) => {
                let result =
                    Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
                        inputs, operator,
                    )
                    .expect("solver-produced evaluation partition");
                (result.authoritative, result.evaluation, None)
            }
            None => (
                Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(inputs)
                    .expect("solver-produced production partition"),
                None,
                None,
            ),
        }
    }

    fn solver_row(
        operator: Option<SnowStage3EvaluationOperator>,
        terminal: bool,
        schema6: bool,
    ) -> (String, serde_json::Value) {
        solver_row_with_nonterminal_state(operator, terminal, schema6, None)
    }

    fn solver_row_with_nonterminal_state(
        operator: Option<SnowStage3EvaluationOperator>,
        terminal: bool,
        schema6: bool,
        nonterminal_state: Option<(f64, f64, f64, f64)>,
    ) -> (String, serde_json::Value) {
        let (mass_swe_m, depth_m, density_kg_m3, temperature_c) = if terminal {
            (0.001_1, 0.002_2, 500.0, 0.0)
        } else {
            nonterminal_state.unwrap_or((0.18, 0.40, 450.0, -8.0))
        };
        let mut layer = DirectSnowLayerState::new(mass_swe_m, depth_m, density_kg_m3, 12.0);
        layer.temperature_c = temperature_c;
        layer.cold_content_j_m2 =
            mass_swe_m * 1_000.0 * 2_100.0 * (-temperature_c).max(0.0);
        let hourly = [DirectSnowHourlyForcing {
            radiation_mj_m2: if terminal { 1_000.0 } else { 0.0 },
            air_temperature_c: if terminal { 0.0 } else { -5.0 },
            ..DirectSnowHourlyForcing::zero()
        }; 24];
        let inputs = DirectActiveSnowPartitionInputs {
            hyetograph_rainfall_m: 0.0,
            rst_c: 0.0,
            newsnw_kg_m3: 100.0,
            ssd_kg_m3: 522.0,
            runtime_swe_m: mass_swe_m,
            runtime_depth_m: depth_m,
            runtime_density_kg_m3: density_kg_m3,
            runtime_settle_day_count: 12.0,
            liquid_water_retained_m: 0.0,
            tmax_c: -3.0,
            tmin_c: -7.0,
            canopy_cover_fraction: 0.45,
            wind_m_s: 3.0,
            dewpoint_c: -15.0,
            snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
            snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
            stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
            surface_energy_options: DirectSnowSurfaceEnergyOptions {
                longwave_model: SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
                sublimation_model: SnowSurfaceSublimationModel::Disabled,
                daily_solar_radiation_mj_m2: if terminal { 48.0 } else { 5.0 },
                daily_extraterrestrial_radiation_mj_m2: 10.0,
                daylight: true,
                atmospheric_pressure_pa: 101_324.6,
                turbulent_geometry: DirectSnowTurbulentGeometry::CLIGEN_V1,
                complete_carrier_shadow: false,
            },
            sturm_climate_class: None,
            sturm_day_of_year: None,
            coe_boundary_depth_m: depth_m,
            coe_boundary_density_kg_m3: density_kg_m3,
            coe_boundary_settle_day_count: 12.0,
            snow_albedo_model: None,
            snow_albedo_state: None,
            snow_layers: vec![layer],
            underlying_surface_albedo: 0.2,
            hourly,
        };
        let (mut partition, evaluation, reconciliation) =
            evaluated_partition(&inputs, operator, schema6);
        poison_production_diagnostics(&mut partition);

        let mut lane = DirectSnowLaneState::from_runtime_values(
            mass_swe_m,
            depth_m,
            density_kg_m3,
            12.0,
        );
        lane.layers = vec![layer];
        let context = DirectSnowTraceRowContext {
            day_index: 17,
            lane_index: 3,
            hyetograph_rainfall_m: 0.0,
            snow_lane_state: &lane,
            snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
            snow_phase_model: SnowPhasePartitionModel::LegacyRst,
            snow_liquid: &partition,
            stage3_evaluation: evaluation.as_ref(),
            stage3_reconciliation: reconciliation.as_deref(),
            stage3_persistent: None,
        };
        let verbose = partition
            .verbose_diagnostics
            .as_deref()
            .expect("verbose solver diagnostics");
        let row = r7h_direct_production_snow_trace_line(&context, verbose);
        let path = std::env::temp_dir().join(format!(
            "openwepp-stage3-real-{}-{}-{}.jsonl",
            std::process::id(),
            if schema6 { "v6" } else { "v5" },
            operator.map_or("disabled", SnowStage3EvaluationOperator::id)
        ));
        std::fs::write(&path, &row).expect("write full schema-v5 trace row");
        let observed = std::fs::read_to_string(&path).expect("reread full schema-v5 trace row");
        std::fs::remove_file(path).expect("remove schema-v5 trace row");
        let value = serde_json::from_str(observed.trim()).expect("full snow trace row is valid JSON");
        (observed, value)
    }

    fn numbers(value: &serde_json::Value, field: &str) -> Vec<f64> {
        value[field]
            .as_array()
            .unwrap_or_else(|| panic!("{field} must be an array"))
            .iter()
            .map(|item| item.as_f64().unwrap_or_else(|| panic!("{field} item must be numeric")))
            .collect()
    }

    fn number(value: &serde_json::Value, field: &str) -> f64 {
        value[field]
            .as_f64()
            .unwrap_or_else(|| panic!("{field} must be numeric"))
    }

    fn assert_close(left: f64, right: f64, context: &str) {
        assert!((left - right).abs() <= 1.0e-6, "{context}: {left} != {right}");
    }

    fn dormant_persistent_day_result(
    ) -> openwepp_hillslope_orchestrator::DirectSnowStage3PersistentDayResult {
        let inputs = DirectActiveSnowPartitionInputs {
            hyetograph_rainfall_m: 0.0,
            rst_c: 0.0,
            newsnw_kg_m3: 100.0,
            ssd_kg_m3: 522.0,
            runtime_swe_m: 0.0,
            runtime_depth_m: 0.0,
            runtime_density_kg_m3: 0.0,
            runtime_settle_day_count: 0.0,
            liquid_water_retained_m: 0.0,
            tmax_c: -3.0,
            tmin_c: -7.0,
            canopy_cover_fraction: 0.45,
            wind_m_s: 3.0,
            dewpoint_c: -15.0,
            snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
            snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
            stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
            surface_energy_options: DirectSnowSurfaceEnergyOptions {
                longwave_model: SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
                sublimation_model: SnowSurfaceSublimationModel::Disabled,
                daily_solar_radiation_mj_m2: 5.0,
                daily_extraterrestrial_radiation_mj_m2: 10.0,
                daylight: true,
                atmospheric_pressure_pa: 101_324.6,
                turbulent_geometry: DirectSnowTurbulentGeometry::CLIGEN_V1,
                complete_carrier_shadow: false,
            },
            sturm_climate_class: None,
            sturm_day_of_year: None,
            coe_boundary_depth_m: 0.0,
            coe_boundary_density_kg_m3: 0.0,
            coe_boundary_settle_day_count: 0.0,
            snow_albedo_model: None,
            snow_albedo_state: None,
            snow_layers: Vec::new(),
            underlying_surface_albedo: 0.2,
            hourly: [DirectSnowHourlyForcing {
                air_temperature_c: -5.0,
                ..DirectSnowHourlyForcing::zero()
            }; 24],
        };
        let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(0, Vec::new())
            .expect("valid dormant trace state");
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &state, 0, 0)
            .expect("dormant trace day")
    }

    fn fnv1a64(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf2_9ce4_8422_2325, |mut hash, byte| {
            hash ^= u64::from(*byte);
            hash.wrapping_mul(0x0000_0100_0000_01b3)
        })
    }

    fn persistent_state_fingerprint(value: &serde_json::Value) -> u64 {
        fn add(mut hash: u64, value: u64) -> u64 {
            for byte in value.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
            hash
        }
        let mut hash = 0xcbf2_9ce4_8422_2325;
        for integer in ["schema_version", "lane_id", "next_interval_index"] {
            hash = add(hash, value[integer].as_u64().expect("integer state field"));
        }
        for field in [
            "cumulative_snowfall_kg_m2",
            "cumulative_external_liquid_kg_m2",
            "cumulative_deposition_kg_m2",
            "cumulative_sublimation_kg_m2",
            "cumulative_melt_kg_m2",
            "cumulative_unresolved_liquid_kg_m2",
            "initial_ice_kg_m2",
            "initial_retained_liquid_kg_m2",
            "detached_retained_liquid_kg_m2",
            "cumulative_complete_energy_j_m2",
            "cumulative_cold_energy_change_j_m2",
            "cumulative_terminal_unallocated_energy_j_m2",
        ] {
            let number = number(value, field);
            hash = add(hash, if number == 0.0 { 0 } else { number.to_bits() });
        }
        for layer in value["layers"].as_array().expect("state layers") {
            for field in [
                "mass_swe_m", "thickness_m", "density_kg_m3", "settle_day_count",
                "temperature_c", "liquid_water_m", "cold_content_j_m2", "refrozen_liquid_m",
            ] {
                let number = number(layer, field);
                hash = add(hash, if number == 0.0 { 0 } else { number.to_bits() });
            }
        }
        hash
    }

    #[allow(clippy::too_many_lines)]
    fn consume_persistent_v7(value: &serde_json::Value) -> Result<(), &'static str> {
        if value["schema"] != "openwepp-r7h-direct-production-snow-trace-v7" {
            return Err("unknown trace schema");
        }
        if value["stage3_evaluation_operator_id"] != "persistent_accumulation_shadow_v1" {
            return Err("adjacent evaluation operator");
        }
        for (field, expected) in [
            ("stage3_evaluation_source_snapshot_id", "pre_interval_authoritative_initial_snapshot_v1"),
            ("stage3_evaluation_support_id", "stage3_persistent_daily_24_hour_support_v1"),
            ("stage3_evaluation_cadence_id", "stage3_dynamic_substep_with_hourly_forcing_v1"),
            ("stage3_evaluation_carrier_id", "stage3_complete_carrier_v1"),
            ("stage3_evaluation_claim_class", "persistent_state_continuity_experiment"),
            ("stage3_evaluation_unresolved_boundaries_id", "snow_ground_cross_day_terminal_recipient_unresolved_v1"),
        ] {
            if value[field] != expected {
                return Err("persistent evaluation tag mismatch");
            }
        }
        if value["stage3_evaluation_arm_ids"]
            != serde_json::json!(["stage3_complete_carrier_v1", "not_applicable"])
            || value["stage3_evaluation_arm_count"] != 1
            || !value["stage3_evaluation_pairing_id"].is_null()
        {
            return Err("persistent evaluation arm mismatch");
        }
        let start = &value["stage3_persistent_start_state"];
        let end = &value["stage3_persistent_end_state"];
        if start["schema_version"] != 1 || end["schema_version"] != 1 {
            return Err("unknown state schema");
        }
        if end["next_interval_index"].as_u64()
            != start["next_interval_index"]
                .as_u64()
                .and_then(|value| value.checked_add(1))
        {
            return Err("out-of-order state");
        }
        if value["stage3_persistent_lane_id"] != start["lane_id"]
            || start["lane_id"] != end["lane_id"]
            || value["stage3_persistent_start_state_fingerprint"] != start["fingerprint"]
            || value["stage3_persistent_end_state_fingerprint"] != end["fingerprint"]
        {
            return Err("top-level state alias mismatch");
        }
        if value["stage3_persistent_next_interval_index"] != end["next_interval_index"] {
            return Err("top-level interval alias mismatch");
        }
        for field in [
            "snowfall_kg_m2",
            "external_liquid_kg_m2",
            "deposition_kg_m2",
            "sublimation_kg_m2",
            "melt_kg_m2",
            "unresolved_liquid_kg_m2",
        ] {
            let top = format!("stage3_persistent_cumulative_{field}");
            let nested = format!("cumulative_{field}");
            if value[&top] != end[&nested] {
                return Err("top-level cumulative alias mismatch");
            }
        }
        for (cumulative, daily) in [
            ("cumulative_snowfall_kg_m2", "stage3_persistent_snowfall_kg_m2"),
            ("cumulative_external_liquid_kg_m2", "stage3_persistent_external_liquid_kg_m2"),
            ("cumulative_deposition_kg_m2", "stage3_persistent_deposition_kg_m2"),
            ("cumulative_sublimation_kg_m2", "stage3_persistent_sublimation_kg_m2"),
            ("cumulative_melt_kg_m2", "stage3_persistent_melt_kg_m2"),
            ("cumulative_unresolved_liquid_kg_m2", "stage3_persistent_unresolved_liquid_kg_m2"),
        ] {
            let end_value = number(end, cumulative);
            let start_value = number(start, cumulative);
            let daily_value = number(value, daily);
            let tolerance =
                1.0e-12_f64.max(1.0e-12 * (end_value.abs() + start_value.abs() + daily_value.abs()));
            if (end_value - start_value - daily_value).abs() > tolerance
            {
                return Err("daily cumulative delta mismatch");
            }
        }
        let unresolved = number(value, "stage3_persistent_unresolved_liquid_kg_m2");
        let external = number(value, "stage3_persistent_external_liquid_kg_m2");
        let melt_liquid = number(value, "stage3_persistent_melt_kg_m2");
        let retained_loss =
            number(value, "stage3_persistent_retained_liquid_censored_loss_kg_m2");
        if (unresolved - external - melt_liquid - retained_loss).abs()
            > 1.0e-12_f64.max(
                1.0e-12
                    * (unresolved.abs()
                        + external.abs()
                        + melt_liquid.abs()
                        + retained_loss.abs()),
            )
        {
            return Err("unresolved liquid decomposition mismatch");
        }
        let lifecycle = match (
            number(value, "stage3_persistent_start_ice_kg_m2") > 0.0,
            number(value, "stage3_persistent_end_ice_kg_m2") > 0.0,
        ) {
            (false, false) => "dormant",
            (false, true) => "reappeared",
            (true, false) => "disappeared",
            (true, true) => "active",
        };
        if value["stage3_persistent_lifecycle"] != lifecycle {
            return Err("lifecycle mismatch");
        }
        for (state, ice_field, liquid_field) in [
            (start, "stage3_persistent_start_ice_kg_m2", "stage3_persistent_start_retained_liquid_kg_m2"),
            (end, "stage3_persistent_end_ice_kg_m2", "stage3_persistent_end_retained_liquid_kg_m2"),
        ] {
            let layers = state["layers"].as_array().ok_or("missing state layers")?;
            let ice = layers.iter().map(|layer| number(layer, "mass_swe_m") * 1_000.0).sum::<f64>();
            let liquid = layers.iter().map(|layer| number(layer, "liquid_water_m") * 1_000.0).sum::<f64>()
                + number(state, "detached_retained_liquid_kg_m2");
            if (ice - number(value, ice_field)).abs() > 1.0e-12_f64.max(1.0e-12 * ice.abs())
                || (liquid - number(value, liquid_field)).abs()
                    > 1.0e-12_f64.max(1.0e-12 * liquid.abs())
            {
                return Err("state endpoint alias mismatch");
            }
        }
        for state in [start, end] {
            let expected = format!("{:016x}", persistent_state_fingerprint(state));
            if state["fingerprint"].as_str() != Some(expected.as_str()) {
                return Err("state fingerprint mismatch");
            }
        }
        let mass_scale = [
            "stage3_persistent_start_ice_kg_m2",
            "stage3_persistent_start_retained_liquid_kg_m2",
            "stage3_persistent_snowfall_kg_m2",
            "stage3_persistent_external_liquid_kg_m2",
            "stage3_persistent_deposition_kg_m2",
            "stage3_persistent_sublimation_kg_m2",
            "stage3_persistent_melt_kg_m2",
            "stage3_persistent_end_ice_kg_m2",
            "stage3_persistent_end_retained_liquid_kg_m2",
            "stage3_persistent_unresolved_liquid_kg_m2",
        ]
        .iter()
        .map(|field| number(value, field).abs())
        .sum::<f64>();
        let mass_tolerance = 1.0e-12_f64.max(1.0e-12 * mass_scale);
        let ice_residual = number(value, "stage3_persistent_start_ice_kg_m2")
            + number(value, "stage3_persistent_snowfall_kg_m2")
            + number(value, "stage3_persistent_deposition_kg_m2")
            - number(value, "stage3_persistent_sublimation_kg_m2")
            - number(value, "stage3_persistent_melt_kg_m2")
            - number(value, "stage3_persistent_end_ice_kg_m2");
        let water_residual = number(value, "stage3_persistent_start_ice_kg_m2")
            + number(value, "stage3_persistent_start_retained_liquid_kg_m2")
            + number(value, "stage3_persistent_snowfall_kg_m2")
            + number(value, "stage3_persistent_external_liquid_kg_m2")
            + number(value, "stage3_persistent_deposition_kg_m2")
            - number(value, "stage3_persistent_sublimation_kg_m2")
            - number(value, "stage3_persistent_unresolved_liquid_kg_m2")
            - number(value, "stage3_persistent_end_ice_kg_m2")
            - number(value, "stage3_persistent_end_retained_liquid_kg_m2");
        if ice_residual.abs() > mass_tolerance
            || water_residual.abs() > mass_tolerance
            || number(value, "stage3_persistent_ice_mass_closure_residual_kg_m2").abs()
                > mass_tolerance
            || number(value, "stage3_persistent_total_water_closure_residual_kg_m2").abs()
                > mass_tolerance
        {
            return Err("producer residual rejected");
        }
        let end_ice = end["layers"]
            .as_array()
            .ok_or("missing end layers")?
            .iter()
            .map(|layer| number(layer, "mass_swe_m") * 1_000.0)
            .sum::<f64>();
        let end_liquid = end["layers"]
            .as_array()
            .ok_or("missing end layers")?
            .iter()
            .map(|layer| number(layer, "liquid_water_m") * 1_000.0)
            .sum::<f64>()
            + number(end, "detached_retained_liquid_kg_m2");
        let cumulative_water_residual = number(end, "initial_ice_kg_m2")
            + number(end, "initial_retained_liquid_kg_m2")
            + number(end, "cumulative_snowfall_kg_m2")
            + number(end, "cumulative_external_liquid_kg_m2")
            + number(end, "cumulative_deposition_kg_m2")
            - number(end, "cumulative_sublimation_kg_m2")
            - number(end, "cumulative_unresolved_liquid_kg_m2")
            - end_ice
            - end_liquid;
        let cumulative_mass_scale = [
            "initial_ice_kg_m2",
            "initial_retained_liquid_kg_m2",
            "cumulative_snowfall_kg_m2",
            "cumulative_external_liquid_kg_m2",
            "cumulative_deposition_kg_m2",
            "cumulative_sublimation_kg_m2",
            "cumulative_unresolved_liquid_kg_m2",
        ]
        .iter()
        .map(|field| number(end, field).abs())
        .sum::<f64>()
            + end_ice.abs()
            + end_liquid.abs();
        if cumulative_water_residual.abs()
            > 1.0e-12_f64.max(1.0e-12 * cumulative_mass_scale)
        {
            return Err("cumulative water residual rejected");
        }
        let complete = number(end, "cumulative_complete_energy_j_m2")
            - number(start, "cumulative_complete_energy_j_m2");
        let cold = number(end, "cumulative_cold_energy_change_j_m2")
            - number(start, "cumulative_cold_energy_change_j_m2");
        let melt = number(end, "cumulative_melt_kg_m2")
            - number(start, "cumulative_melt_kg_m2");
        let terminal = number(end, "cumulative_terminal_unallocated_energy_j_m2")
            - number(start, "cumulative_terminal_unallocated_energy_j_m2");
        let energy_residual = complete - cold - 333_600.0 * melt - terminal;
        let energy_scale = complete.abs() + cold.abs() + (333_600.0 * melt).abs() + terminal.abs();
        if energy_residual.abs() > 1.0e-6_f64.max(1.0e-12 * energy_scale) {
            return Err("daily energy residual rejected");
        }
        for (hourly, expected, absolute_floor) in [
            ("stage3_evaluation_hourly_complete_energy_j_m2", complete, 1.0e-6_f64),
            ("stage3_evaluation_hourly_cold_energy_change_j_m2", cold, 1.0e-6_f64),
            ("stage3_evaluation_hourly_melt_kg_m2", melt, 1.0e-12_f64),
            ("stage3_evaluation_hourly_terminal_unallocated_j_m2", terminal, 1.0e-6_f64),
        ] {
            let hourly_values = numbers(value, hourly);
            let sum = hourly_values.iter().sum::<f64>();
            let sum_abs = hourly_values.iter().map(|operand| operand.abs()).sum::<f64>();
            if (sum - expected).abs()
                > absolute_floor.max(1.0e-12 * (sum_abs + expected.abs()))
            {
                return Err("hourly primitive reconstruction mismatch");
            }
        }
        let evaluated_seconds = numbers(value, "stage3_evaluation_hourly_evaluated_seconds")
            .iter()
            .sum::<f64>();
        if (evaluated_seconds - number(value, "stage3_evaluation_evaluated_seconds")).abs()
            > 1.0e-9
        {
            return Err("support reconstruction mismatch");
        }
        let hourly_requested = numbers(value, "stage3_evaluation_hourly_requested_seconds");
        let requested_seconds = hourly_requested.iter().sum::<f64>();
        let requested_total = number(value, "stage3_evaluation_requested_seconds");
        let coverage = number(value, "stage3_evaluation_coverage_fraction");
        let hourly_evaluated = numbers(value, "stage3_evaluation_hourly_evaluated_seconds");
        if (requested_total - 86_400.0).abs() > 1.0e-9
            || hourly_requested.iter().any(|seconds| (*seconds - 3_600.0).abs() > 1.0e-9)
            || hourly_evaluated
                .iter()
                .zip(&hourly_requested)
                .any(|(evaluated, requested)| evaluated < &0.0 || evaluated > requested)
            || (requested_seconds - requested_total).abs() > 1.0e-9
            || (coverage - evaluated_seconds / requested_total).abs() > 1.0e-12
        {
            return Err("requested support reconstruction mismatch");
        }
        let evaluated_flags = value["stage3_evaluation_hourly_complete_carrier_evaluated"]
            .as_array()
            .ok_or("missing evaluated flags")?;
        if evaluated_flags.iter().zip(hourly_evaluated).any(|(flag, seconds)| {
            flag.as_bool() != Some(seconds > 0.0)
        }) {
            return Err("hourly support flag mismatch");
        }
        Ok(())
    }

    fn consume_persistent_v7_sequence(rows: &[serde_json::Value]) -> Result<(), &'static str> {
        let mut prior_end = std::collections::BTreeMap::<u64, (String, u64)>::new();
        for row in rows {
            consume_persistent_v7(row)?;
            let start = &row["stage3_persistent_start_state"];
            let lane = start["lane_id"].as_u64().ok_or("missing start lane")?;
            if let Some((fingerprint, interval)) = prior_end.get(&lane)
                && (start["fingerprint"].as_str() != Some(fingerprint.as_str())
                    || start["next_interval_index"].as_u64() != Some(*interval))
            {
                return Err("cross-row continuity mismatch");
            }
            let end = &row["stage3_persistent_end_state"];
            prior_end.insert(
                end["lane_id"].as_u64().ok_or("missing end lane")?,
                (
                end["fingerprint"].as_str().ok_or("missing end fingerprint")?.to_owned(),
                end["next_interval_index"].as_u64().ok_or("missing end interval")?,
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn disabled_full_row_retains_the_frozen_schema_v4_bytes() {
        let (row, value) = solver_row(None, false, false);
        assert_eq!(value["schema"], "openwepp-r7h-direct-production-snow-trace-v4");
        assert!(
            value
                .as_object()
                .expect("v4 row object")
                .keys()
                .all(|field| !field.starts_with("stage3_evaluation_"))
        );
        assert_eq!(fnv1a64(row.as_bytes()), 0xa398_0f35_4195_8836);
    }

    #[test]
    fn schema_v6_real_consumer_reads_complete_tuples_and_reconstructs_primitives() {
        let (_, paired) = solver_row(
            Some(SnowStage3EvaluationOperator::SameStatePairedCarrierV1),
            false,
            true,
        );
        assert_eq!(
            paired["schema"],
            "openwepp-r7h-direct-production-snow-trace-v6"
        );
        let reconciliation = &paired["stage3_operator_reconciliation"];
        assert_eq!(reconciliation["schema_version"], 6);
        let tuples = reconciliation["tuples"]
            .as_array()
            .expect("schema-v6 tuples");
        assert_eq!(tuples.len(), 24);
        let tuple = &tuples[0];
        for field in [
            "effective_input_fingerprint_fnv1a64",
            "active_layer_state_fingerprint_before_fnv1a64",
            "total_layer_state_fingerprint_after_fnv1a64",
            "snow_albedo_source_id",
            "longwave_model_id",
            "sublimation_model_id",
            "turbulent_termination_status",
            "stability_class",
            "sensible_exchange_velocity_m_s",
            "precipitation_advected_flux_w_m2",
            "complete_external_flux_w_m2",
            "energy_closure_residual_j_m2",
        ] {
            assert!(tuple.get(field).is_some(), "schema-v6 tuple missing {field}");
        }
        let incoming = tuple["hourly_radiation_mj_m2"]
            .as_f64()
            .expect("hourly radiation")
            * 1_000_000.0
            / 3_600.0;
        assert_close(
            tuple["incoming_shortwave_w_m2"]
                .as_f64()
                .expect("incoming shortwave"),
            incoming,
            "incoming shortwave reconstruction",
        );
        let external = [
            "net_shortwave_w_m2",
            "net_longwave_w_m2",
            "sensible_flux_w_m2",
            "latent_flux_w_m2",
            "precipitation_advected_flux_w_m2",
        ]
        .iter()
        .map(|field| tuple[*field].as_f64().expect("external operand"))
        .sum::<f64>();
        assert_close(
            tuple["complete_external_flux_w_m2"]
                .as_f64()
                .expect("complete external"),
            external,
            "complete external reconstruction",
        );
        assert_eq!(
            tuple["active_ice_mass_before_kg_m2"],
            tuple["active_ice_mass_after_kg_m2"]
        );
        assert_ne!(tuple["net_shortwave_w_m2"], 9.94e12);
    }

    #[test]
    fn schema_v6_terminal_tuple_uses_null_after_surface_fields() {
        let (_, sequential) = solver_row(
            Some(SnowStage3EvaluationOperator::SequentialResolvedShadowV1),
            true,
            true,
        );
        let tuples = sequential["stage3_operator_reconciliation"]["tuples"]
            .as_array()
            .expect("sequential schema-v6 tuples");
        let terminal = tuples
            .iter()
            .find(|tuple| tuple["after_surface_applicable"] == false)
            .expect("terminal tuple");
        assert_eq!(
            terminal["after_surface_applicability_reason"],
            "post_substep_no_resolved_surface"
        );
        for field in [
            "active_layer_prefix_count_after",
            "active_layer_state_fingerprint_after_fnv1a64",
            "active_ice_mass_after_kg_m2",
            "active_depth_after_m",
            "active_density_after_kg_m3",
            "active_cold_after_j_m2",
            "surface_temperature_after_c",
        ] {
            assert!(terminal[field].is_null(), "{field} must be null after meltout");
        }
        assert!(terminal["total_ice_mass_after_kg_m2"].is_number());
        assert!(terminal["total_cold_after_j_m2"].is_number());
    }

    #[test]
    fn schema_v6_serialized_dynamic_substeps_preserve_exact_transition_continuity() {
        let (_, value) = solver_row_with_nonterminal_state(
            Some(SnowStage3EvaluationOperator::SequentialResolvedShadowV1),
            false,
            true,
            Some((0.02, 0.04, 500.0, -8.0)),
        );
        let tuples = value["stage3_operator_reconciliation"]["tuples"]
            .as_array()
            .expect("serialized sequential tuples");
        assert!(tuples.len() > 1);
        assert_eq!(tuples[0]["hour_index"], 0);
        assert_eq!(tuples[0]["substep_index"], 0);
        assert_eq!(tuples[1]["hour_index"], 0);
        assert_eq!(tuples[1]["substep_index"], 1);
        for pair in tuples.windows(2) {
            let (previous, next) = (&pair[0], &pair[1]);
            assert_eq!(previous["after_surface_applicable"], true);
            for (after, before) in [
                ("active_layer_prefix_count_after", "active_layer_prefix_count_before"),
                ("total_layer_count_after", "total_layer_count_before"),
                (
                    "active_layer_state_fingerprint_after_fnv1a64",
                    "active_layer_state_fingerprint_before_fnv1a64",
                ),
                (
                    "total_layer_state_fingerprint_after_fnv1a64",
                    "total_layer_state_fingerprint_before_fnv1a64",
                ),
            ] {
                assert_eq!(previous[after], next[before], "{after} -> {before}");
            }
            for (after, before) in [
                ("active_ice_mass_after_kg_m2", "active_ice_mass_before_kg_m2"),
                ("total_ice_mass_after_kg_m2", "total_ice_mass_before_kg_m2"),
                ("active_depth_after_m", "active_depth_before_m"),
                ("active_density_after_kg_m3", "active_density_before_kg_m3"),
                ("active_cold_after_j_m2", "active_cold_before_j_m2"),
                ("total_cold_after_j_m2", "total_cold_before_j_m2"),
                ("surface_temperature_after_c", "surface_temperature_before_c"),
            ] {
                assert_eq!(
                    previous[after]
                        .as_f64()
                        .unwrap_or_else(|| panic!("{after} numeric"))
                        .to_bits(),
                    next[before]
                        .as_f64()
                        .unwrap_or_else(|| panic!("{before} numeric"))
                        .to_bits(),
                    "{after} -> {before}"
                );
            }
        }
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        clippy::float_cmp,
        clippy::cast_precision_loss
    )]
    fn full_solver_rows_reconstruct_all_v5_operands_and_reject_adjacent_aliases() {
        let (_, paired) = solver_row(
            Some(SnowStage3EvaluationOperator::SameStatePairedCarrierV1),
            false,
            false,
        );
        let (_, sequential) = solver_row(
            Some(SnowStage3EvaluationOperator::SequentialResolvedShadowV1),
            true,
            false,
        );
        let required = [
            "operator_id", "source_snapshot_id", "support_id", "cadence_id", "carrier_id",
            "coverage_id", "claim_class", "unresolved_boundaries_id", "pairing_id", "arm_ids",
            "arm_count", "source_fingerprint_fnv1a64", "forcing_fingerprint_fnv1a64",
            "geometry_fingerprint_fnv1a64", "non_formulation_fingerprint_fnv1a64",
            "surface_arm_non_formulation_fingerprint_fnv1a64",
            "complete_arm_non_formulation_fingerprint_fnv1a64", "requested_seconds",
            "evaluated_seconds", "coverage_fraction", "surface_arm_applicable",
            "surface_arm_shortwave_j_m2", "surface_arm_longwave_j_m2",
            "surface_arm_latent_j_m2", "surface_arm_sensible_applicable",
            "surface_arm_advected_applicable", "surface_arm_internal_conduction_applicable",
            "surface_arm_total_j_m2", "complete_arm_shortwave_j_m2",
            "complete_arm_longwave_j_m2", "complete_arm_sensible_j_m2",
            "complete_arm_latent_j_m2", "complete_arm_advected_j_m2",
            "complete_arm_internal_active_lower_conduction_j_m2", "complete_arm_applicable",
            "complete_arm_internal_conduction_applicable", "complete_arm_vapor_mass_exchange_kg_m2",
            "complete_arm_cold_content_export_j_m2", "complete_arm_cold_content_export_applicable",
            "complete_arm_available_ice_kg_m2", "complete_arm_available_ice_applicable",
            "complete_arm_total_j_m2", "complete_arm_sequential_ledger_applicable",
            "complete_arm_cold_energy_change_j_m2", "complete_arm_excess_energy_j_m2",
            "complete_arm_sublimation_kg_m2", "complete_arm_melt_kg_m2",
            "complete_arm_terminal_unallocated_j_m2",
            "complete_arm_terminal_unallocated_applicable", "complete_arm_component_residual_j_m2",
            "complete_arm_maximum_thermodynamic_residual_j_m2", "hourly_shortwave_j_m2",
            "hourly_longwave_j_m2", "hourly_sensible_j_m2", "hourly_latent_j_m2",
            "hourly_advected_j_m2", "hourly_internal_active_lower_conduction_j_m2",
            "hourly_cold_content_export_j_m2", "hourly_vapor_mass_exchange_kg_m2",
            "hourly_complete_energy_j_m2", "hourly_cold_required_j_m2",
            "hourly_cold_energy_change_j_m2", "hourly_excess_energy_j_m2",
            "hourly_available_ice_kg_m2", "hourly_sublimation_kg_m2", "hourly_melt_kg_m2",
            "hourly_terminal_unallocated_j_m2", "hourly_energy_closure_residual_j_m2",
            "hourly_complete_carrier_evaluated", "hourly_requested_seconds",
            "hourly_evaluated_seconds",
        ];
        for value in [&paired, &sequential] {
            assert_eq!(value["schema"], "openwepp-r7h-direct-production-snow-trace-v5");
            let object = value.as_object().expect("trace row object");
            for suffix in required {
                let field = format!("stage3_evaluation_{suffix}");
                assert!(object.contains_key(&field), "consumer omitted {field}");
            }
            assert!(object.keys().all(|field| {
                !field.starts_with("stage3_evaluation_") || !field.contains("ground")
            }));
            let requested = number(value, "stage3_evaluation_requested_seconds");
            let evaluated = number(value, "stage3_evaluation_evaluated_seconds");
            assert_close(
                number(value, "stage3_evaluation_coverage_fraction"),
                evaluated / requested,
                "coverage must use seconds, not row/hour counts",
            );
            assert_close(
                numbers(value, "stage3_evaluation_hourly_requested_seconds")
                    .iter()
                    .sum(),
                requested,
                "hourly requested support",
            );
            assert_close(
                numbers(value, "stage3_evaluation_hourly_evaluated_seconds")
                    .iter()
                    .sum(),
                evaluated,
                "hourly evaluated support",
            );
            assert_ne!(
                number(value, "stage3_evaluation_complete_arm_shortwave_j_m2"),
                number(value, "stage3_shortwave_energy_j_m2"),
                "production shortwave alias must remain distinct"
            );
            assert_ne!(
                number(value, "stage3_evaluation_complete_arm_component_residual_j_m2"),
                number(value, "stage3_energy_closure_residual_j_m2"),
                "producer residual must not substitute for evaluation reconstruction"
            );
        }

        assert_eq!(paired["stage3_evaluation_surface_arm_applicable"], true);
        assert_eq!(
            paired["stage3_evaluation_surface_arm_non_formulation_fingerprint_fnv1a64"],
            paired["stage3_evaluation_complete_arm_non_formulation_fingerprint_fnv1a64"]
        );
        let paired_surface = ["shortwave", "longwave", "latent"]
            .into_iter()
            .map(|term| number(&paired, &format!("stage3_evaluation_surface_arm_{term}_j_m2")))
            .sum::<f64>();
        assert_close(
            paired_surface,
            number(&paired, "stage3_evaluation_surface_arm_total_j_m2"),
            "paired surface total",
        );

        let components = ["shortwave", "longwave", "sensible", "latent", "advected"];
        for value in [&paired, &sequential] {
            let mut total = components
                .iter()
                .map(|term| number(value, &format!("stage3_evaluation_complete_arm_{term}_j_m2")))
                .sum::<f64>();
            if value["stage3_evaluation_complete_arm_internal_conduction_applicable"] == true {
                total += number(
                    value,
                    "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2",
                );
            }
            assert_close(
                total,
                number(value, "stage3_evaluation_complete_arm_total_j_m2"),
                "complete component total",
            );
        }

        assert_eq!(sequential["stage3_evaluation_surface_arm_applicable"], false);
        assert!(number(&sequential, "stage3_evaluation_evaluated_seconds")
            < number(&sequential, "stage3_evaluation_requested_seconds"));
        assert_ne!(
            number(&sequential, "stage3_evaluation_requested_seconds"),
            1.0,
            "one JSONL row is not requested support"
        );
        let evaluated_hours = numbers(
            &sequential,
            "stage3_evaluation_hourly_evaluated_seconds",
        );
        let nonzero_hour_fraction = evaluated_hours
            .iter()
            .filter(|seconds| **seconds > 0.0)
            .count() as f64
            / 24.0;
        assert_ne!(
            number(&sequential, "stage3_evaluation_coverage_fraction"),
            nonzero_hour_fraction,
            "nonzero-hour count is not substep coverage"
        );
        assert_ne!(
            number(&sequential, "stage3_evaluation_complete_arm_melt_kg_m2"),
            number(&sequential, "raw_melt_m") * 1_000.0,
            "CoE melt must not substitute for evaluation melt"
        );
        assert_close(
            number(&sequential, "stage3_evaluation_complete_arm_total_j_m2"),
            number(
                &sequential,
                "stage3_evaluation_complete_arm_cold_energy_change_j_m2",
            ) + LATENT_HEAT_FUSION_J_KG
                * number(&sequential, "stage3_evaluation_complete_arm_melt_kg_m2")
                + number(
                    &sequential,
                    "stage3_evaluation_complete_arm_terminal_unallocated_j_m2",
                ),
            "sequential cold/fusion/terminal identity",
        );
        for (hour, (((complete, cold), melt), terminal)) in numbers(
            &sequential,
            "stage3_evaluation_hourly_complete_energy_j_m2",
        )
        .into_iter()
        .zip(numbers(
            &sequential,
            "stage3_evaluation_hourly_cold_energy_change_j_m2",
        ))
        .zip(numbers(&sequential, "stage3_evaluation_hourly_melt_kg_m2"))
        .zip(numbers(
            &sequential,
            "stage3_evaluation_hourly_terminal_unallocated_j_m2",
        ))
        .enumerate()
        {
            let residual = numbers(
                &sequential,
                "stage3_evaluation_hourly_energy_closure_residual_j_m2",
            )[hour];
            assert_close(
                complete,
                cold + LATENT_HEAT_FUSION_J_KG * melt + terminal + residual,
                "hourly sequential identity",
            );
        }
        for (term, daily_field) in [
            ("shortwave", "complete_arm_shortwave_j_m2"),
            ("longwave", "complete_arm_longwave_j_m2"),
            ("sensible", "complete_arm_sensible_j_m2"),
            ("latent", "complete_arm_latent_j_m2"),
            ("advected", "complete_arm_advected_j_m2"),
            (
                "internal_active_lower_conduction",
                "complete_arm_internal_active_lower_conduction_j_m2",
            ),
        ] {
            assert_close(
                numbers(
                    &sequential,
                    &format!("stage3_evaluation_hourly_{term}_j_m2"),
                )
                .iter()
                .sum(),
                number(&sequential, &format!("stage3_evaluation_{daily_field}")),
                "hourly component reconstruction",
            );
        }
        for (hourly_field, daily_field) in [
            (
                "hourly_complete_energy_j_m2",
                "complete_arm_total_j_m2",
            ),
            (
                "hourly_vapor_mass_exchange_kg_m2",
                "complete_arm_vapor_mass_exchange_kg_m2",
            ),
            (
                "hourly_cold_content_export_j_m2",
                "complete_arm_cold_content_export_j_m2",
            ),
            (
                "hourly_cold_energy_change_j_m2",
                "complete_arm_cold_energy_change_j_m2",
            ),
            (
                "hourly_excess_energy_j_m2",
                "complete_arm_excess_energy_j_m2",
            ),
            (
                "hourly_sublimation_kg_m2",
                "complete_arm_sublimation_kg_m2",
            ),
            ("hourly_melt_kg_m2", "complete_arm_melt_kg_m2"),
            (
                "hourly_terminal_unallocated_j_m2",
                "complete_arm_terminal_unallocated_j_m2",
            ),
        ] {
            assert_close(
                numbers(
                    &sequential,
                    &format!("stage3_evaluation_{hourly_field}"),
                )
                .iter()
                .sum(),
                number(
                    &sequential,
                    &format!("stage3_evaluation_{daily_field}"),
                ),
                "hourly sequential operand reconstruction",
            );
        }
        assert_close(
            numbers(
                &sequential,
                "stage3_evaluation_hourly_available_ice_kg_m2",
            )
            .into_iter()
            .fold(0.0_f64, f64::max),
            number(
                &sequential,
                "stage3_evaluation_complete_arm_available_ice_kg_m2",
            ),
            "available ice uses maximum pre-debit support",
        );
        let maximum_hourly_residual = numbers(
            &sequential,
            "stage3_evaluation_hourly_energy_closure_residual_j_m2",
        )
        .into_iter()
        .map(f64::abs)
        .fold(0.0_f64, f64::max);
        assert_close(
            maximum_hourly_residual,
            number(
                &sequential,
                "stage3_evaluation_complete_arm_maximum_thermodynamic_residual_j_m2",
            ),
            "maximum thermodynamic residual",
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn inactive_day_emits_declared_v6_evaluation_without_mutating_authority() {
        let lane = DirectSnowLaneState::zero();
        let disabled = inactive_direct_snow_evaluation_result(
            SnowDensityModel::LegacyWepp,
            0.0,
            &lane,
            openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture::Verbose,
            None,
        );
        let enabled = inactive_direct_snow_evaluation_result(
            SnowDensityModel::LegacyWepp,
            0.0,
            &lane,
            openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture::Verbose,
            Some(SnowStage3EvaluationOperator::SameStatePairedCarrierV1),
        );

        assert_eq!(enabled.result.authoritative, disabled.result.authoritative);
        assert!(disabled.result.evaluation.is_none());
        assert!(disabled.reconciliation.is_none());
        assert_eq!(
            direct_snow_trace_schema(None, None, None),
            "openwepp-r7h-direct-production-snow-trace-v4"
        );

        let evaluation = enabled
            .result
            .evaluation
            .as_ref()
            .expect("enabled inactive evaluation");
        let reconciliation = enabled
            .reconciliation
            .as_deref()
            .expect("enabled inactive reconciliation");
        assert_eq!(
            direct_snow_trace_schema(Some(evaluation), Some(reconciliation), None),
            "openwepp-r7h-direct-production-snow-trace-v6"
        );
        let mut persistent_evaluation = *evaluation;
        persistent_evaluation.operator = SnowStage3EvaluationOperator::PersistentAccumulationShadowV1;
        persistent_evaluation.source_snapshot_id = "pre_interval_authoritative_initial_snapshot_v1";
        persistent_evaluation.support_id = "stage3_persistent_daily_24_hour_support_v1";
        persistent_evaluation.cadence_id = "stage3_dynamic_substep_with_hourly_forcing_v1";
        persistent_evaluation.carrier_id = "stage3_complete_carrier_v1";
        persistent_evaluation.claim_class = "persistent_state_continuity_experiment";
        persistent_evaluation.pairing_id = None;
        persistent_evaluation.arm_ids = ["stage3_complete_carrier_v1", "not_applicable"];
        persistent_evaluation.arm_count = 1;
        persistent_evaluation.requested_seconds = 86_400.0;
        persistent_evaluation.evaluated_seconds = 3_600.0;
        persistent_evaluation.coverage_fraction = 1.0 / 24.0;
        persistent_evaluation.complete_arm_applicable = true;
        persistent_evaluation.complete_arm_total_j_m2 = 583_819.0;
        persistent_evaluation.complete_arm_cold_energy_change_j_m2 = 2.0;
        persistent_evaluation.complete_arm_melt_kg_m2 = 1.75;
        persistent_evaluation.complete_arm_terminal_unallocated_j_m2 = 17.0;
        persistent_evaluation.complete_arm_terminal_unallocated_applicable = true;
        persistent_evaluation.hourly[0].complete_energy_j_m2 = 583_819.0;
        persistent_evaluation.hourly[0].cold_energy_change_j_m2 = 2.0;
        persistent_evaluation.hourly[0].melt_kg_m2 = 1.75;
        persistent_evaluation.hourly[0].unallocated_after_exhaustion_j_m2 = 17.0;
        persistent_evaluation.hourly[0].requested_seconds = 3_600.0;
        persistent_evaluation.hourly[0].evaluated_seconds = 3_600.0;
        persistent_evaluation.hourly[0].complete_carrier_evaluated = true;
        let mut persistent = dormant_persistent_day_result();
        persistent.start_state = Box::new(
            openwepp_hillslope_orchestrator::DirectSnowStage3PersistentState {
                schema_version: 1,
                terminal_event_model: None,
                fingerprint: 0x5bcd_7547_4b65_2ea4,
                lane_id: 0,
                next_interval_index: 0,
                layers: Vec::new(),
                detached_retained_liquid_kg_m2: 0.0,
                initial_ice_kg_m2: 0.0,
                initial_retained_liquid_kg_m2: 0.0,
                cumulative_snowfall_kg_m2: 0.0,
                cumulative_external_liquid_kg_m2: 0.0,
                cumulative_deposition_kg_m2: 0.0,
                cumulative_sublimation_kg_m2: 0.0,
                cumulative_melt_kg_m2: 0.0,
                cumulative_unresolved_liquid_kg_m2: 0.0,
                cumulative_complete_energy_j_m2: 0.0,
                cumulative_cold_energy_change_j_m2: 0.0,
                cumulative_terminal_unallocated_energy_j_m2: 0.0,
            },
        );
        persistent.state = openwepp_hillslope_orchestrator::DirectSnowStage3PersistentState {
            schema_version: 1,
            terminal_event_model: None,
            fingerprint: 0x7428_f289_6003_8068,
            lane_id: 0,
            next_interval_index: 1,
            layers: Vec::new(),
            detached_retained_liquid_kg_m2: 0.0,
            initial_ice_kg_m2: 0.0,
            initial_retained_liquid_kg_m2: 0.0,
            cumulative_snowfall_kg_m2: 2.0,
            cumulative_external_liquid_kg_m2: 3.0,
            cumulative_deposition_kg_m2: 0.0,
            cumulative_sublimation_kg_m2: 0.25,
            cumulative_melt_kg_m2: 1.75,
            cumulative_unresolved_liquid_kg_m2: 4.75,
            cumulative_complete_energy_j_m2: 583_819.0,
            cumulative_cold_energy_change_j_m2: 2.0,
            cumulative_terminal_unallocated_energy_j_m2: 17.0,
        };
        persistent.evaluation = persistent_evaluation;
        persistent.reconciliation = Box::new(reconciliation.clone());
        persistent.lifecycle = "dormant";
        persistent.start_state_fingerprint = 0x5bcd_7547_4b65_2ea4;
        persistent.end_state_fingerprint = 0x7428_f289_6003_8068;
        persistent.start_ice_kg_m2 = 0.0;
        persistent.start_retained_liquid_kg_m2 = 0.0;
        persistent.snowfall_kg_m2 = 2.0;
        persistent.external_liquid_kg_m2 = 3.0;
        persistent.deposition_kg_m2 = 0.0;
        persistent.refrozen_kg_m2 = 0.0;
        persistent.sublimation_kg_m2 = 0.25;
        persistent.melt_kg_m2 = 1.75;
        persistent.end_ice_kg_m2 = 0.0;
        persistent.end_retained_liquid_kg_m2 = 0.0;
        persistent.retained_liquid_censored_loss_kg_m2 = 0.0;
        persistent.ice_mass_closure_residual_kg_m2 = 0.0;
        persistent.total_water_closure_residual_kg_m2 = 0.0;
        persistent.unresolved_liquid_kg_m2 = 4.75;
        persistent.terminal_unallocated_energy_j_m2 = 17.0;
        persistent.terminal_event = None;
        persistent.terminal_intervals = Vec::new();
        assert_eq!(
            direct_snow_trace_schema(
                Some(&persistent.evaluation),
                Some(reconciliation),
                Some(&persistent),
            ),
            "openwepp-r7h-direct-production-snow-trace-v7"
        );
        let context = DirectSnowTraceRowContext {
            day_index: 0,
            lane_index: 0,
            hyetograph_rainfall_m: 0.003,
            snow_lane_state: &lane,
            snow_melt_model: SnowMeltModel::LegacyCoe,
            snow_phase_model: SnowPhasePartitionModel::LegacyRst,
            snow_liquid: &enabled.result.authoritative,
            stage3_evaluation: Some(&persistent.evaluation),
            stage3_reconciliation: Some(reconciliation),
            stage3_persistent: Some(&persistent),
        };
        let verbose = enabled
            .result
            .authoritative
            .verbose_diagnostics
            .as_deref()
            .expect("verbose inactive diagnostics");
        let row = r7h_direct_production_snow_trace_line(&context, verbose);
        let consumed: serde_json::Value =
            serde_json::from_str(row.trim()).expect("schema-v7 row is valid JSON");
        assert_eq!(
            consumed["schema"],
            "openwepp-r7h-direct-production-snow-trace-v7"
        );
        assert!(consumed.get("stage3_terminal_event_model").is_none());
        assert!(consumed.get("stage3_terminal_event").is_none());
        assert_eq!(
            format!(
                "{:016x}",
                persistent_state_fingerprint(&consumed["stage3_persistent_end_state"])
            ),
            consumed["stage3_persistent_end_state"]["fingerprint"]
                .as_str()
                .expect("end fingerprint"),
        );
        let mut unknown = consumed.clone();
        unknown["schema"] = serde_json::json!("openwepp-r7h-direct-production-snow-trace-v8");
        assert_eq!(consume_persistent_v7(&unknown), Err("unknown trace schema"));
        let mut poisoned = consumed.clone();
        poisoned["stage3_persistent_total_water_closure_residual_kg_m2"] =
            serde_json::json!(1.0);
        assert_eq!(consume_persistent_v7(&poisoned), Err("producer residual rejected"));
        assert_eq!(
            consume_persistent_v7_sequence(&[consumed.clone(), consumed.clone()]),
            Err("cross-row continuity mismatch")
        );
        let mut continuation = consumed.clone();
        continuation["stage3_persistent_start_state"] =
            consumed["stage3_persistent_end_state"].clone();
        continuation["stage3_persistent_end_state"] =
            consumed["stage3_persistent_end_state"].clone();
        continuation["stage3_persistent_end_state"]["next_interval_index"] =
            serde_json::json!(2);
        let continuation_end_fingerprint = format!(
            "{:016x}",
            persistent_state_fingerprint(&continuation["stage3_persistent_end_state"])
        );
        continuation["stage3_persistent_end_state"]["fingerprint"] =
            serde_json::json!(continuation_end_fingerprint);
        continuation["stage3_persistent_start_state_fingerprint"] =
            consumed["stage3_persistent_end_state_fingerprint"].clone();
        continuation["stage3_persistent_end_state_fingerprint"] =
            continuation["stage3_persistent_end_state"]["fingerprint"].clone();
        continuation["stage3_persistent_next_interval_index"] = serde_json::json!(2);
        for field in [
            "stage3_persistent_snowfall_kg_m2",
            "stage3_persistent_external_liquid_kg_m2",
            "stage3_persistent_deposition_kg_m2",
            "stage3_persistent_sublimation_kg_m2",
            "stage3_persistent_melt_kg_m2",
            "stage3_persistent_unresolved_liquid_kg_m2",
            "stage3_persistent_terminal_unallocated_energy_j_m2",
        ] {
            continuation[field] = serde_json::json!(0.0);
        }
        for field in [
            "stage3_evaluation_complete_arm_total_j_m2",
            "stage3_evaluation_complete_arm_cold_energy_change_j_m2",
            "stage3_evaluation_complete_arm_melt_kg_m2",
            "stage3_evaluation_complete_arm_terminal_unallocated_j_m2",
        ] {
            continuation[field] = serde_json::json!(0.0);
        }
        for field in [
            "stage3_evaluation_hourly_complete_energy_j_m2",
            "stage3_evaluation_hourly_cold_energy_change_j_m2",
            "stage3_evaluation_hourly_melt_kg_m2",
            "stage3_evaluation_hourly_terminal_unallocated_j_m2",
        ] {
            continuation[field] =
                serde_json::Value::Array(vec![serde_json::json!(0.0); 24]);
        }
        let mut lane_one = consumed.clone();
        lane_one["stage3_persistent_lane_id"] = serde_json::json!(1);
        for state_field in ["stage3_persistent_start_state", "stage3_persistent_end_state"] {
            lane_one[state_field]["lane_id"] = serde_json::json!(1);
            let fingerprint = format!(
                "{:016x}",
                persistent_state_fingerprint(&lane_one[state_field])
            );
            lane_one[state_field]["fingerprint"] = serde_json::json!(fingerprint);
        }
        lane_one["stage3_persistent_start_state_fingerprint"] =
            lane_one["stage3_persistent_start_state"]["fingerprint"].clone();
        lane_one["stage3_persistent_end_state_fingerprint"] =
            lane_one["stage3_persistent_end_state"]["fingerprint"].clone();
        consume_persistent_v7_sequence(&[consumed.clone(), lane_one, continuation])
            .expect("interleaved per-lane continuity");
        assert_eq!(consumed["stage3_persistent_start_state_fingerprint"], "5bcd75474b652ea4");
        assert_eq!(consumed["stage3_persistent_end_state_fingerprint"], "7428f28960038068");
        assert!(consumed["stage3_persistent_state_layers"].as_array().is_some());
        assert_eq!(consumed["stage3_persistent_start_state"]["next_interval_index"], 0);
        assert_eq!(consumed["stage3_persistent_end_state"]["next_interval_index"], 1);
        let end_state = &consumed["stage3_persistent_end_state"];
        let start_state = &consumed["stage3_persistent_start_state"];
        assert_eq!(start_state["schema_version"], 1);
        assert_eq!(end_state["schema_version"], 1);
        assert_eq!(
            format!("{:016x}", persistent_state_fingerprint(start_state)),
            start_state["fingerprint"].as_str().expect("start fingerprint"),
        );
        assert_eq!(
            format!("{:016x}", persistent_state_fingerprint(end_state)),
            end_state["fingerprint"].as_str().expect("end fingerprint"),
        );
        consume_persistent_v7(&consumed).expect("valid schema-v7 persistent row");
        let cumulative_end = number(end_state, "initial_ice_kg_m2")
            + number(end_state, "initial_retained_liquid_kg_m2")
            + number(end_state, "cumulative_snowfall_kg_m2")
            + number(end_state, "cumulative_external_liquid_kg_m2")
            + number(end_state, "cumulative_deposition_kg_m2")
            - number(end_state, "cumulative_sublimation_kg_m2")
            - number(end_state, "cumulative_unresolved_liquid_kg_m2");
        assert_close(cumulative_end, 0.0, "independent cumulative water closure");
        assert_ne!(
            number(end_state, "cumulative_complete_energy_j_m2").to_bits(),
            number(end_state, "cumulative_terminal_unallocated_energy_j_m2").to_bits(),
            "terminal energy must not alias complete carrier energy",
        );
        assert_close(
            number(end_state, "cumulative_complete_energy_j_m2")
                - number(end_state, "cumulative_cold_energy_change_j_m2")
                - 333_600.0 * number(end_state, "cumulative_melt_kg_m2")
                - number(end_state, "cumulative_terminal_unallocated_energy_j_m2"),
            0.0,
            "independent cumulative energy closure",
        );
        let reconstructed_end = number(&consumed, "stage3_persistent_start_ice_kg_m2")
            + number(&consumed, "stage3_persistent_snowfall_kg_m2")
            + number(&consumed, "stage3_persistent_deposition_kg_m2")
            - number(&consumed, "stage3_persistent_sublimation_kg_m2")
            - number(&consumed, "stage3_persistent_melt_kg_m2");
        assert_close(
            reconstructed_end,
            number(&consumed, "stage3_persistent_end_ice_kg_m2"),
            "independent schema-v7 daily ice reconstruction",
        );
        assert_close(
            number(&consumed, "stage3_persistent_external_liquid_kg_m2")
                + number(&consumed, "stage3_persistent_melt_kg_m2")
                + number(
                    &consumed,
                    "stage3_persistent_retained_liquid_censored_loss_kg_m2",
                ),
            number(&consumed, "stage3_persistent_unresolved_liquid_kg_m2"),
            "all liquid without a recipient is censored",
        );
        assert_ne!(
            number(&consumed, "stage3_persistent_external_liquid_kg_m2").to_bits(),
            number(&consumed, "stage3_persistent_snowfall_kg_m2").to_bits(),
            "liquid and snowfall aliases must remain distinguishable",
        );
        assert_eq!(
            number(&consumed, "stage3_persistent_cumulative_external_liquid_kg_m2")
                .to_bits(),
            3.0_f64.to_bits(),
        );
        assert_eq!(evaluation.requested_seconds.to_bits(), 86_400.0_f64.to_bits());
        assert_eq!(evaluation.evaluated_seconds.to_bits(), 0.0_f64.to_bits());
        assert_eq!(evaluation.coverage_fraction.to_bits(), 0.0_f64.to_bits());
        assert_eq!(evaluation.source_fingerprint, 0);
        assert_eq!(evaluation.forcing_fingerprint, 0);
        assert_eq!(evaluation.geometry_fingerprint, 0);
        assert_eq!(evaluation.non_formulation_fingerprint, 0);
        assert!(evaluation.hourly.iter().all(|hour| {
            hour.requested_seconds.to_bits() == 3_600.0_f64.to_bits()
                && hour.evaluated_seconds.to_bits() == 0.0_f64.to_bits()
                && !hour.complete_carrier_evaluated
        }));
        assert_eq!(reconciliation.schema_version, 6);
        assert!(reconciliation.tuples.is_empty());
        assert!(reconciliation
            .hourly_status
            .iter()
            .all(|status| !status.evaluated && status.reason == "operator_not_selected"));
    }
}
