#[allow(clippy::too_many_lines)]
fn direct_snow_trace_stage3_evaluation_fields(
    diagnostics: &openwepp_hillslope_orchestrator::DirectSnowStage3Diagnostics,
) -> Option<String> {
    let evaluation = diagnostics.evaluation?;
    let pairing_id = evaluation
        .pairing_id
        .map_or_else(|| "null".to_string(), |value| format!("\"{value}\""));
    let hourly_shortwave = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_shortwave_energy_j_m2
    });
    let hourly_longwave = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_longwave_energy_j_m2
    });
    let hourly_sensible = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_sensible_flux_w_m2 * hour.shadow_evaluated_seconds
    });
    let hourly_latent = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_latent_flux_w_m2 * hour.shadow_evaluated_seconds
    });
    let hourly_advected = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_advected_flux_w_m2 * hour.shadow_evaluated_seconds
    });
    let hourly_internal_conduction = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_internal_active_lower_conduction_j_m2
    });
    let hourly_cold_content_export = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_cold_content_export_j_m2
    });
    let hourly_requested = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_requested_seconds
    });
    let hourly_evaluated = direct_snow_trace_hourly_values(diagnostics, |hour| {
        hour.shadow_evaluated_seconds
    });
    Some(format!(
        "\"stage3_evaluation_operator_id\":\"{}\",\
\"stage3_evaluation_source_snapshot_id\":\"{}\",\
\"stage3_evaluation_support_id\":\"{}\",\
\"stage3_evaluation_cadence_id\":\"{}\",\
\"stage3_evaluation_carrier_id\":\"{}\",\
\"stage3_evaluation_claim_class\":\"{}\",\
\"stage3_evaluation_unresolved_boundaries_id\":\"{}\",\
\"stage3_evaluation_pairing_id\":{},\
\"stage3_evaluation_arm_ids\":[\"{}\",\"{}\"],\
\"stage3_evaluation_arm_count\":{},\
\"stage3_evaluation_source_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_forcing_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_geometry_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_non_formulation_fingerprint_fnv1a64\":\"{:016x}\",\
\"stage3_evaluation_requested_seconds\":{},\
\"stage3_evaluation_evaluated_seconds\":{},\
\"stage3_evaluation_coverage_fraction\":{},\
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
\"stage3_evaluation_complete_arm_internal_conduction_applicable\":{},\
\"stage3_evaluation_complete_arm_vapor_mass_exchange_kg_m2\":{},\
\"stage3_evaluation_complete_arm_cold_content_export_j_m2\":{},\
\"stage3_evaluation_complete_arm_cold_content_export_applicable\":{},\
\"stage3_evaluation_complete_arm_available_ice_kg_m2\":{},\
\"stage3_evaluation_complete_arm_available_ice_applicable\":{},\
\"stage3_evaluation_complete_arm_total_j_m2\":{},\
\"stage3_evaluation_complete_arm_terminal_unallocated_j_m2\":{},\
\"stage3_evaluation_complete_arm_residual_j_m2\":{},\
\"stage3_evaluation_hourly_shortwave_j_m2\":{},\
\"stage3_evaluation_hourly_longwave_j_m2\":{},\
\"stage3_evaluation_hourly_sensible_j_m2\":{},\
\"stage3_evaluation_hourly_latent_j_m2\":{},\
\"stage3_evaluation_hourly_advected_j_m2\":{},\
\"stage3_evaluation_hourly_internal_active_lower_conduction_j_m2\":{},\
\"stage3_evaluation_hourly_cold_content_export_j_m2\":{},\
\"stage3_evaluation_hourly_requested_seconds\":{},\
\"stage3_evaluation_hourly_evaluated_seconds\":{}",
        evaluation.operator.id(),
        evaluation.source_snapshot_id,
        evaluation.support_id,
        evaluation.cadence_id,
        evaluation.carrier_id,
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
        direct_production_trace_number(evaluation.requested_seconds),
        direct_production_trace_number(evaluation.evaluated_seconds),
        direct_production_trace_number(evaluation.coverage_fraction),
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
        direct_production_trace_number(
            evaluation.complete_arm_terminal_unallocated_j_m2
        ),
        direct_production_trace_number(evaluation.complete_arm_residual_j_m2),
        hourly_shortwave,
        hourly_longwave,
        hourly_sensible,
        hourly_latent,
        hourly_advected,
        hourly_internal_conduction,
        hourly_cold_content_export,
        hourly_requested,
        hourly_evaluated,
    ))
}
