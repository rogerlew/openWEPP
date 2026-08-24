#[allow(clippy::wildcard_imports)]
use super::*;

mod evaluation;
mod persistent_state;
mod support;
mod terminal_event;

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
    fn evaluate_stage3_persistent_day_internal(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        supports: &[DirectSnowStage3SupportInput],
        terminal_request: Option<DirectSnowTerminalEventRequest>,
        boundary: Option<Stage3SnowSurfaceBoundaryReceiptV1>,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        Self::validate_stage3_persistent_state(state)?;
        if state.lane_id != lane_id || state.next_interval_index != interval_index {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_identity_or_order",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        let tag = Stage3EvaluationTag::new(
            SnowStage3EvaluationOperator::PersistentAccumulationShadowV1,
        );
        let start_ice_kg_m2 = Self::stage3_total_ice_mass_swe_m(&state.layers)
            * STAGE3_RHO_WATER_KG_M3;
        let start_retained_liquid_kg_m2 = state
            .layers
            .iter()
            .map(|layer| layer.liquid_water_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>()
            + state.detached_retained_liquid_kg_m2;
        let cold_content = state
            .layers
            .iter()
            .map(Self::stage3_layer_cold_content_j_m2)
            .collect();
        let summary = Self::evaluate_stage3_sequential_melt_shadow(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            tag,
            inputs,
            supports,
            state.layers.clone(),
            cold_content,
            terminal_request,
            state.detached_retained_liquid_kg_m2,
            boundary,
        )?;
        Self::validate_stage3_shadow_summary(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            &summary,
        )?;
        let snowfall_kg_m2 = supports
            .iter()
            .map(|support| support.forcing.snowfall_m * 0.1 * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>();
        let external_liquid_kg_m2 = supports
            .iter()
            .map(|support| support.forcing.rain_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>();
        let deposition_kg_m2 = summary
            .reconciliation
            .tuples
            .iter()
            .filter_map(|tuple| tuple.deposition_kg_m2)
            .sum::<f64>()
            + summary.terminal_deposition_kg_m2;
        let refrozen_kg_m2 = summary.terminal_refrozen_kg_m2;
        let end_ice_kg_m2 = Self::stage3_total_ice_mass_swe_m(&summary.final_layers)
            * STAGE3_RHO_WATER_KG_M3;
        let end_retained_liquid_kg_m2 = summary
            .final_layers
            .iter()
            .map(|layer| layer.liquid_water_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>();
        let retained_liquid_censored_loss_kg_m2 =
            (start_retained_liquid_kg_m2 - end_retained_liquid_kg_m2).max(0.0);
        let unresolved_liquid_kg_m2 = if terminal_request.is_some() {
            (external_liquid_kg_m2 + summary.melt_kg_m2 + start_retained_liquid_kg_m2
                - refrozen_kg_m2
                - end_retained_liquid_kg_m2)
                .max(0.0)
        } else {
            external_liquid_kg_m2
                + summary.melt_kg_m2
                + retained_liquid_censored_loss_kg_m2
        };
        let residual = start_ice_kg_m2 + snowfall_kg_m2 + deposition_kg_m2 + refrozen_kg_m2
            - summary.sublimation_kg_m2
            - summary.melt_kg_m2
            - end_ice_kg_m2;
        let ice_tolerance = 1.0e-12_f64.max(
            1.0e-12
                * (start_ice_kg_m2
                    + snowfall_kg_m2
                    + deposition_kg_m2
                    + summary.sublimation_kg_m2
                    + summary.melt_kg_m2
                    + end_ice_kg_m2),
        );
        Self::require_direct_typed_snow_value_with(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            || BoundarySymbol::from("snow.stage3_persistent_mass_residual_kg_m2"),
            residual.abs(),
            Some(0.0),
            Some(ice_tolerance),
        )?;
        let total_water_residual = start_ice_kg_m2
            + start_retained_liquid_kg_m2
            + snowfall_kg_m2
            + external_liquid_kg_m2
            + deposition_kg_m2
            - summary.sublimation_kg_m2
            - unresolved_liquid_kg_m2
            - end_ice_kg_m2
            - end_retained_liquid_kg_m2;
        let water_tolerance = 1.0e-12_f64.max(
            1.0e-12
                * (start_ice_kg_m2
                    + start_retained_liquid_kg_m2
                    + snowfall_kg_m2
                    + external_liquid_kg_m2
                    + deposition_kg_m2
                    + summary.sublimation_kg_m2
                    + unresolved_liquid_kg_m2
                    + end_ice_kg_m2
                    + end_retained_liquid_kg_m2),
        );
        Self::require_direct_typed_snow_value_with(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            || BoundarySymbol::from("snow.stage3_persistent_total_water_residual_kg_m2"),
            total_water_residual.abs(),
            Some(0.0),
            Some(water_tolerance),
        )?;
        let lifecycle = match (start_ice_kg_m2 > 0.0, end_ice_kg_m2 > 0.0) {
            (false, false) => "dormant",
            (false, true) => "reappeared",
            (true, false) => "disappeared",
            (true, true) => "active",
        };
        let mut next = state.clone();
        next.next_interval_index = next.next_interval_index.checked_add(1).ok_or_else(|| {
            Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_interval_overflow",
                f64::MAX,
                Some(0.0),
                None,
            )
        })?;
        next.layers.clone_from(&summary.final_layers);
        next.detached_retained_liquid_kg_m2 = 0.0;
        next.cumulative_snowfall_kg_m2 += snowfall_kg_m2;
        next.cumulative_external_liquid_kg_m2 += external_liquid_kg_m2;
        next.cumulative_deposition_kg_m2 += deposition_kg_m2;
        next.cumulative_sublimation_kg_m2 += summary.sublimation_kg_m2;
        next.cumulative_melt_kg_m2 += summary.melt_kg_m2;
        next.cumulative_unresolved_liquid_kg_m2 += unresolved_liquid_kg_m2;
        next.cumulative_complete_energy_j_m2 += summary.complete_energy_j_m2;
        next.cumulative_cold_energy_change_j_m2 += summary.cold_energy_change_j_m2;
        next.cumulative_terminal_unallocated_energy_j_m2 +=
            summary.unallocated_after_exhaustion_j_m2;
        next.fingerprint = Self::stage3_persistent_state_fingerprint(&next);
        Self::validate_stage3_persistent_state(&next)?;
        let cumulative_energy_residual = next.cumulative_complete_energy_j_m2
            - next.cumulative_cold_energy_change_j_m2
            - STAGE3_LATENT_HEAT_FUSION_J_KG * next.cumulative_melt_kg_m2
            - next.cumulative_terminal_unallocated_energy_j_m2;
        let cumulative_energy_scale = next.cumulative_complete_energy_j_m2.abs()
            + next.cumulative_cold_energy_change_j_m2.abs()
            + (STAGE3_LATENT_HEAT_FUSION_J_KG * next.cumulative_melt_kg_m2).abs()
            + next.cumulative_terminal_unallocated_energy_j_m2.abs();
        Self::require_direct_typed_snow_value_with(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            || BoundarySymbol::from("snow.stage3_persistent_cumulative_energy_residual_j_m2"),
            cumulative_energy_residual.abs(),
            Some(0.0),
            Some(1.0e-6_f64.max(1.0e-12 * cumulative_energy_scale)),
        )?;
        let cumulative_residual = next.initial_ice_kg_m2
            + next.initial_retained_liquid_kg_m2
            + next.cumulative_snowfall_kg_m2
            + next.cumulative_external_liquid_kg_m2
            + next.cumulative_deposition_kg_m2
            - next.cumulative_sublimation_kg_m2
            - next.cumulative_unresolved_liquid_kg_m2
            - end_ice_kg_m2
            - end_retained_liquid_kg_m2;
        let cumulative_scale = next.initial_ice_kg_m2
            + next.initial_retained_liquid_kg_m2
            + next.cumulative_snowfall_kg_m2
            + next.cumulative_external_liquid_kg_m2
            + next.cumulative_deposition_kg_m2
            + next.cumulative_sublimation_kg_m2
            + next.cumulative_unresolved_liquid_kg_m2
            + end_ice_kg_m2
            + end_retained_liquid_kg_m2;
        Self::require_direct_typed_snow_value_with(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            || BoundarySymbol::from("snow.stage3_persistent_cumulative_water_residual_kg_m2"),
            cumulative_residual.abs(),
            Some(0.0),
            Some(1.0e-12_f64.max(1.0e-12 * cumulative_scale)),
        )?;
        let end_state_fingerprint = next.fingerprint;
        Ok(DirectSnowStage3PersistentDayResult {
            start_state: Box::new(state.clone()),
            state: next,
            evaluation: Self::stage3_evaluation_diagnostics(&summary),
            reconciliation: Box::new(summary.reconciliation),
            lifecycle,
            start_state_fingerprint: state.fingerprint,
            end_state_fingerprint,
            start_ice_kg_m2,
            start_retained_liquid_kg_m2,
            snowfall_kg_m2,
            external_liquid_kg_m2,
            deposition_kg_m2,
            refrozen_kg_m2,
            sublimation_kg_m2: summary.sublimation_kg_m2,
            melt_kg_m2: summary.melt_kg_m2,
            end_ice_kg_m2,
            end_retained_liquid_kg_m2,
            retained_liquid_censored_loss_kg_m2,
            ice_mass_closure_residual_kg_m2: residual,
            total_water_closure_residual_kg_m2: total_water_residual,
            unresolved_liquid_kg_m2,
            terminal_unallocated_energy_j_m2: summary.unallocated_after_exhaustion_j_m2,
            terminal_event: summary.terminal_event,
            terminal_intervals: summary.terminal_intervals,
        })
    }

    pub(crate) fn validate_stage3_persistent_state(
        state: &DirectSnowStage3PersistentState,
    ) -> Result<(), DirectSnowStage3EvaluationError> {
        let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        if !matches!(state.schema_version, 1 | 2) {
            return Err(Self::stage3_domain_error(
                phase,
                "snow.stage3_persistent_snapshot_version",
                f64::from(state.schema_version),
                Some(1.0),
                Some(2.0),
            )
            .into());
        }
        if (state.schema_version == 1 && state.terminal_event_model.is_some())
            || (state.schema_version == 2
                && state.terminal_event_model
                    != Some(DirectSnowTerminalEventModel::EnthalpyEventV1))
        {
            return Err(Self::stage3_domain_error(
                phase,
                "snow.stage3_persistent_terminal_model_binding",
                f64::from(state.schema_version),
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        if state.fingerprint != Self::stage3_persistent_state_fingerprint(state) {
            return Err(Self::stage3_domain_error(
                phase,
                "snow.stage3_persistent_snapshot_fingerprint",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        for layer in &state.layers {
            Self::validate_stage3_layer(phase, layer)?;
        }
        for (symbol, value) in [
            (
                "snow.stage3_persistent_detached_retained_liquid",
                state.detached_retained_liquid_kg_m2,
            ),
            ("snow.stage3_persistent_initial_ice", state.initial_ice_kg_m2),
            (
                "snow.stage3_persistent_initial_retained_liquid",
                state.initial_retained_liquid_kg_m2,
            ),
            ("snow.stage3_persistent_cumulative_snowfall", state.cumulative_snowfall_kg_m2),
            ("snow.stage3_persistent_cumulative_external_liquid", state.cumulative_external_liquid_kg_m2),
            ("snow.stage3_persistent_cumulative_deposition", state.cumulative_deposition_kg_m2),
            ("snow.stage3_persistent_cumulative_sublimation", state.cumulative_sublimation_kg_m2),
            ("snow.stage3_persistent_cumulative_melt", state.cumulative_melt_kg_m2),
            ("snow.stage3_persistent_cumulative_unresolved_liquid", state.cumulative_unresolved_liquid_kg_m2),
        ] {
            Self::require_direct_typed_snow_value_with(
                phase,
                || BoundarySymbol::from(symbol),
                value,
                Some(0.0),
                None,
            )?;
        }
        for (symbol, value) in [
            (
                "snow.stage3_persistent_cumulative_complete_energy",
                state.cumulative_complete_energy_j_m2,
            ),
            (
                "snow.stage3_persistent_cumulative_cold_energy_change",
                state.cumulative_cold_energy_change_j_m2,
            ),
        ] {
            Self::require_direct_typed_snow_value_with(
                phase,
                || BoundarySymbol::from(symbol),
                value,
                None,
                None,
            )?;
        }
        Self::require_direct_typed_snow_value_with(
            phase,
            || BoundarySymbol::from("snow.stage3_persistent_cumulative_terminal_unallocated_energy"),
            state.cumulative_terminal_unallocated_energy_j_m2,
            Some(0.0),
            None,
        )?;
        Ok(())
    }

    pub(crate) fn stage3_persistent_state_fingerprint(
        state: &DirectSnowStage3PersistentState,
    ) -> u64 {
        let mut fingerprint = Self::stage3_fnv1a_start();
        fingerprint = Self::stage3_fnv1a_u64(fingerprint, u64::from(state.schema_version));
        fingerprint = Self::stage3_fnv1a_u64(fingerprint, u64::from(state.lane_id));
        fingerprint = Self::stage3_fnv1a_u64(fingerprint, state.next_interval_index);
        if let Some(model) = state.terminal_event_model {
            fingerprint = Self::stage3_fnv1a_u64(
                fingerprint,
                match model {
                    DirectSnowTerminalEventModel::EnthalpyEventV1 => 1,
                },
            );
        }
        for value in [
            state.cumulative_snowfall_kg_m2,
            state.cumulative_external_liquid_kg_m2,
            state.cumulative_deposition_kg_m2,
            state.cumulative_sublimation_kg_m2,
            state.cumulative_melt_kg_m2,
            state.cumulative_unresolved_liquid_kg_m2,
            state.initial_ice_kg_m2,
            state.initial_retained_liquid_kg_m2,
            state.detached_retained_liquid_kg_m2,
            state.cumulative_complete_energy_j_m2,
            state.cumulative_cold_energy_change_j_m2,
            state.cumulative_terminal_unallocated_energy_j_m2,
        ] {
            fingerprint = Self::stage3_fnv1a_u64(
                fingerprint,
                if value == 0.0 { 0 } else { value.to_bits() },
            );
        }
        for layer in &state.layers {
            for value in [
                layer.mass_swe_m,
                layer.thickness_m,
                layer.density_kg_m3,
                layer.settle_day_count,
                layer.temperature_c,
                layer.liquid_water_m,
                layer.cold_content_j_m2,
                layer.refrozen_liquid_m,
            ] {
                fingerprint = Self::stage3_fnv1a_u64(
                    fingerprint,
                    if value == 0.0 { 0 } else { value.to_bits() },
                );
            }
        }
        fingerprint
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_stage3_liquid_routing(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        incoming_liquid_m: f64,
        aggregate: Stage3AggregateState,
        layers: &mut Vec<DirectSnowLayerState>,
        capture: DirectSnowDiagnosticCapture,
        evaluation_operator: Option<SnowStage3EvaluationOperator>,
    ) -> Result<DirectSnowStage3Resolution, DirectSnowStage3EvaluationError> {
        if !Self::stage3_liquid_routing_enabled(phase_class, inputs, incoming_liquid_m)? {
            return Ok(DirectSnowStage3Resolution::disabled(capture));
        }
        let evaluation_tag = if capture.is_verbose() {
            Self::stage3_evaluation_operator(
                phase_class,
                inputs.surface_energy_options.complete_carrier_shadow,
                evaluation_operator,
            )?
                .map(|operator| {
                    let tag = Stage3EvaluationTag::new(operator);
                    Self::validate_stage3_evaluation_tag(phase_class, tag)?;
                    Ok::<Stage3EvaluationTag, Wb11HydrologyKernelGuardError>(tag)
                })
                .transpose()?
        } else {
            None
        };
        let project_legacy_shadow = evaluation_operator.is_none()
            && inputs.surface_energy_options.complete_carrier_shadow;
        Self::prepare_stage3_layer_stack(phase_class, inputs, aggregate, layers)?;
        if layers.is_empty() {
            let meltwater_temperature_c = if incoming_liquid_m > WB11_ZERO_THRESHOLD {
                Some(Self::stage3_temperature(phase_class, 0.0)?)
            } else {
                None
            };
            let (evaluation, reconciliation) = if let Some(tag) = evaluation_tag {
                let mut summary = Stage3ShadowSummary::new(tag);
                Self::stage3_shadow_fingerprints(inputs, layers, &[], &mut summary);
                summary.complete_arm_non_formulation_fingerprint =
                    summary.non_formulation_fingerprint;
                if tag.operator == SnowStage3EvaluationOperator::SameStatePairedCarrierV1 {
                    summary.surface_arm_non_formulation_fingerprint =
                        summary.non_formulation_fingerprint;
                }
                for hour in &mut summary.hourly {
                    hour.requested_seconds = STAGE3_SECONDS_PER_HOUR;
                }
                for hour in &mut summary.reconciliation.hourly_status {
                    *hour = DirectSnowStage3ReconciliationHourStatus {
                        evaluated: false,
                        reason: "no_resolved_snow_at_day_start",
                    };
                }
                Self::validate_stage3_shadow_summary(phase_class, &summary)?;
                (
                    Some(Self::stage3_evaluation_diagnostics(&summary)),
                    Some(Box::new(summary.reconciliation)),
                )
            } else {
                (None, None)
            };
            let diagnostics = capture
                .is_verbose()
                .then(DirectSnowStage3Diagnostics::disabled);
            return Ok(DirectSnowStage3Resolution {
                outcome: DirectSnowStage3Outcome {
                    enabled: true,
                    meltwater_temperature_c,
                    sublimation_m: 0.0,
                },
                liquid_disposition_ledger: DirectSnowLiquidDispositionLedger {
                    incoming_liquid_m,
                    routed_liquid_m: incoming_liquid_m,
                    ..DirectSnowLiquidDispositionLedger::default()
                },
                diagnostics,
                evaluation,
                reconciliation,
            });
        }

        let mut cold_content_by_layer = Vec::with_capacity(layers.len());
        let mut cold_content_before_j_m2 = 0.0;
        let initially_unresolved = Self::stage3_total_ice_mass_swe_m(layers)
            <= STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
        for layer in layers.iter() {
            Self::validate_stage3_layer(phase_class, layer)?;
            let cold_content = if initially_unresolved {
                layer.cold_content_j_m2
            } else {
                Self::stage3_layer_cold_content_j_m2(layer)
            };
            cold_content_by_layer.push(cold_content);
            cold_content_before_j_m2 += cold_content;
        }
        let shadow_summary = match evaluation_tag {
                Some(tag) if tag.operator == SnowStage3EvaluationOperator::SameStatePairedCarrierV1 => {
                    let summary = Self::evaluate_stage3_same_state_paired_carrier(
                        phase_class,
                        tag,
                        inputs,
                        layers,
                        &cold_content_by_layer,
                    )?;
                    Self::validate_stage3_shadow_summary(phase_class, &summary)?;
                    Some(summary)
                }
                Some(tag) => {
                    let supports = inputs
                        .hourly
                        .iter()
                        .copied()
                        .map(|forcing| DirectSnowStage3SupportInput {
                            forcing,
                            duration_seconds: STAGE3_SECONDS_PER_HOUR,
                        })
                        .collect::<Vec<_>>();
                    let summary = Self::evaluate_stage3_sequential_melt_shadow(
                        phase_class,
                        tag,
                        inputs,
                        &supports,
                        layers.clone(),
                        cold_content_by_layer.clone(),
                        None,
                        0.0,
                        None,
                    )?;
                    Self::validate_stage3_shadow_summary(phase_class, &summary)?;
                    Some(summary)
                }
                None => None,
        };
        let mut active_layer_count: usize;

        let mut surface_energy_j_m2 = 0.0;
        let mut conduction_energy_j_m2 = 0.0;
        let mut shortwave_energy_j_m2 = 0.0;
        let mut longwave_energy_j_m2 = 0.0;
        let mut latent_energy_j_m2 = 0.0;
        let mut vapor_mass_exchange_kg_m2 = 0.0;
        let mut latent_mass_energy_j_m2 = 0.0;
        let mut sublimation_m = 0.0;
        let mut cold_content_export_j_m2 = 0.0;
        let mut mass_latent_identity_residual_j_m2 = 0.0;
        let mut unused_positive_energy_j_m2 = 0.0;
        let mut thermal_domain_suspended_seconds = 0.0;
        let mut minimum_unresolved_thermal_mass_kg_m2: f64 = 0.0;
        let mut lower_thermal_volume_collapsed_seconds = 0.0;
        let mut minimum_collapsed_lower_mass_kg_m2: f64 = 0.0;
        let mut hourly_surface_energy = capture
            .is_verbose()
            .then(|| Box::new([DirectSnowSurfaceEnergyHourDiagnostics::zero(); 24]));
        for (hour_index, hourly) in inputs.hourly.iter().enumerate() {
            if layers.is_empty() {
                break;
            }
            let mut elapsed_seconds = 0.0;
            let mut hour_diagnostics = capture
                .is_verbose()
                .then(DirectSnowSurfaceEnergyHourDiagnostics::zero);
            let mut hour_latent_energy_j_m2 = 0.0;
            let mut hour_latent_mass_energy_j_m2 = 0.0;
            while elapsed_seconds < STAGE3_SECONDS_PER_HOUR && !layers.is_empty() {
                let total_mass_swe_m = Self::stage3_total_ice_mass_swe_m(layers);
                if total_mass_swe_m <= STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M {
                    let total_mass_kg_m2 = total_mass_swe_m * STAGE3_RHO_WATER_KG_M3;
                    thermal_domain_suspended_seconds +=
                        STAGE3_SECONDS_PER_HOUR - elapsed_seconds;
                    minimum_unresolved_thermal_mass_kg_m2 =
                        if minimum_unresolved_thermal_mass_kg_m2 > 0.0 {
                            minimum_unresolved_thermal_mass_kg_m2.min(total_mass_kg_m2)
                        } else {
                            total_mass_kg_m2
                        };
                    break;
                }
                active_layer_count =
                    Self::align_stage3_active_layer_boundary(layers, &mut cold_content_by_layer);
                let (_, lower_mass_swe_m) =
                    Self::stage3_control_volume_masses_swe_m(layers, active_layer_count);
                let collapsed_lower_mass_kg_m2 = if Self::
                    stage3_lower_volume_is_subresolution_swe_m(lower_mass_swe_m)
                {
                    active_layer_count = layers.len();
                    Some(lower_mass_swe_m * STAGE3_RHO_WATER_KG_M3)
                } else {
                    None
                };
                Self::normalize_stage3_control_volume_temperature(
                    &mut layers[..active_layer_count],
                    &mut cold_content_by_layer[..active_layer_count],
                );
                Self::normalize_stage3_control_volume_temperature(
                    &mut layers[active_layer_count..],
                    &mut cold_content_by_layer[active_layer_count..],
                );
                active_layer_count = Self::coalesce_stage3_thermal_fragments(
                    layers,
                    &mut cold_content_by_layer,
                    active_layer_count,
                );
                let requested_substep_seconds =
                    Self::stage3_substep_seconds(layers, active_layer_count);
                let substep_seconds =
                    requested_substep_seconds.min(STAGE3_SECONDS_PER_HOUR - elapsed_seconds);
                if let Some(collapsed_mass_kg_m2) = collapsed_lower_mass_kg_m2 {
                    lower_thermal_volume_collapsed_seconds += substep_seconds;
                    minimum_collapsed_lower_mass_kg_m2 =
                        if minimum_collapsed_lower_mass_kg_m2 > 0.0 {
                            minimum_collapsed_lower_mass_kg_m2.min(collapsed_mass_kg_m2)
                        } else {
                            collapsed_mass_kg_m2
                        };
                }
                let active_state = Self::stage3_control_volume_state(
                    phase_class,
                    &layers[..active_layer_count],
                    &cold_content_by_layer[..active_layer_count],
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?;
                let lower_state = if active_layer_count < layers.len() {
                    Some(Self::stage3_control_volume_state(
                        phase_class,
                        &layers[active_layer_count..],
                        &cold_content_by_layer[active_layer_count..],
                        inputs.surface_energy_options.atmospheric_pressure_pa,
                    )?)
                } else {
                    None
                };
                let surface_temperature_c =
                    Self::stage3_temperature_from_cold_content_values(
                        active_state.mass_swe_m,
                        active_state.cold_content_j_m2,
                    );
                let carrier = Self::stage3_hourly_surface_energy(
                    phase_class,
                    inputs,
                    *hourly,
                    Stage3SurfaceInterval {
                        surface_temperature_c,
                        snow_depth_m: active_state.depth_m,
                        snow_density_kg_m3: active_state.density_kg_m3,
                        duration_seconds: substep_seconds,
                        forcing_duration_seconds: STAGE3_SECONDS_PER_HOUR,
                        boundary: None,
                    },
                    None,
                    capture,
                )?;
                shortwave_energy_j_m2 += carrier.shortwave_j_m2;
                longwave_energy_j_m2 += carrier.longwave_j_m2;
                latent_energy_j_m2 += carrier.latent_j_m2;
                vapor_mass_exchange_kg_m2 += carrier.vapor_mass_exchange_kg_m2;
                latent_mass_energy_j_m2 += carrier.latent_mass_energy_j_m2;
                hour_latent_energy_j_m2 += carrier.latent_j_m2;
                hour_latent_mass_energy_j_m2 += carrier.latent_mass_energy_j_m2;
                mass_latent_identity_residual_j_m2 +=
                    carrier.mass_latent_identity_residual_j_m2;
                let conduction = Self::apply_stage3_active_lower_conduction(
                    phase_class,
                    layers,
                    &mut cold_content_by_layer,
                    active_layer_count,
                    substep_seconds,
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?;
                conduction_energy_j_m2 +=
                    conduction.active_energy + conduction.lower_energy;
                let applied = Self::apply_stage3_control_volume_energy(
                    carrier.total_j_m2,
                    layers,
                    &mut cold_content_by_layer,
                    0,
                    active_layer_count,
                );
                let active_cold_content_after_fluxes_j_m2 =
                    cold_content_by_layer[..active_layer_count]
                        .iter()
                        .sum::<f64>();
                let lower_cold_content_after_fluxes_j_m2 =
                    cold_content_by_layer[active_layer_count..]
                        .iter()
                        .sum::<f64>();
                let active_energy_closure_residual_j_m2 = applied
                    + conduction.active_energy
                    - (active_state.cold_content_j_m2
                        - active_cold_content_after_fluxes_j_m2);
                let lower_energy_closure_residual_j_m2 = conduction.lower_energy
                    - (lower_state.map_or(0.0, |state| state.cold_content_j_m2)
                        - lower_cold_content_after_fluxes_j_m2);
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from("snow.stage3_active_energy_residual_j_m2"),
                    active_energy_closure_residual_j_m2.abs(),
                    None,
                    Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
                )?;
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from("snow.stage3_lower_energy_residual_j_m2"),
                    lower_energy_closure_residual_j_m2.abs(),
                    None,
                    Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
                )?;
                surface_energy_j_m2 += applied;
                let unused = (carrier.total_j_m2 - applied).max(0.0);
                unused_positive_energy_j_m2 += unused;
                if let (Some(hour_diagnostics), Some(surface)) =
                    (hour_diagnostics.as_mut(), carrier.diagnostics)
                {
                    Self::accumulate_stage3_hour_diagnostics(
                        hour_diagnostics,
                        &Stage3SubstepDiagnostics {
                            surface,
                            duration_seconds: substep_seconds,
                            applied_j_m2: applied,
                            unused_j_m2: unused,
                            active: active_state,
                            lower: lower_state,
                            conduction,
                            active_energy_closure_residual_j_m2,
                            lower_energy_closure_residual_j_m2,
                            atmospheric_pressure_pa: inputs
                                .surface_energy_options
                                .atmospheric_pressure_pa,
                        },
                    );
                }
                if carrier.sublimation_m > 0.0 {
                    let (removed_m, exported_j_m2, _) =
                        Self::remove_stage3_active_sublimation(
                            carrier.sublimation_m,
                            layers,
                            &mut cold_content_by_layer,
                            &mut active_layer_count,
                        );
                    cold_content_export_j_m2 += exported_j_m2;
                    sublimation_m += removed_m;
                }
                elapsed_seconds += substep_seconds;
            }
            if let (Some(hourly_surface_energy), Some(mut hour_diagnostics)) =
                (hourly_surface_energy.as_deref_mut(), hour_diagnostics)
            {
                Self::finish_stage3_hour_diagnostics(
                    &mut hour_diagnostics,
                    hour_latent_energy_j_m2,
                    hour_latent_mass_energy_j_m2,
                );
                if project_legacy_shadow && let Some(ref shadow) = shadow_summary {
                    let sequential = shadow.hourly[hour_index];
                    hour_diagnostics.shadow_sensible_flux_w_m2 = sequential.sensible_flux_w_m2;
                    hour_diagnostics.shadow_latent_flux_w_m2 = sequential.latent_flux_w_m2;
                    hour_diagnostics.shadow_advected_flux_w_m2 = sequential.advected_flux_w_m2;
                    hour_diagnostics.shadow_complete_energy_j_m2 = sequential.complete_energy_j_m2;
                    hour_diagnostics.shadow_vapor_mass_exchange_kg_m2 =
                        sequential.vapor_mass_exchange_kg_m2;
                    hour_diagnostics.shadow_cold_required_j_m2 =
                        sequential.cold_required_j_m2;
                    hour_diagnostics.shadow_cold_energy_change_j_m2 =
                        sequential.cold_energy_change_j_m2;
                    hour_diagnostics.shadow_excess_energy_j_m2 =
                        sequential.excess_energy_j_m2;
                    hour_diagnostics.shadow_ice_available_kg_m2 =
                        sequential.ice_available_kg_m2;
                    hour_diagnostics.shadow_sublimation_kg_m2 =
                        sequential.sublimation_kg_m2;
                    hour_diagnostics.shadow_melt_kg_m2 = sequential.melt_kg_m2;
                    hour_diagnostics.shadow_unallocated_after_exhaustion_j_m2 =
                        sequential.unallocated_after_exhaustion_j_m2;
                    hour_diagnostics.shadow_energy_closure_residual_j_m2 =
                        sequential.energy_closure_residual_j_m2;
                    hour_diagnostics.shadow_complete_carrier_evaluated =
                        sequential.complete_carrier_evaluated;
                }
                hourly_surface_energy[hour_index] = hour_diagnostics;
            }
        }
        let reconstruct_liquid_temperature = thermal_domain_suspended_seconds == 0.0;
        let (routed_liquid_m, retained_delta_m, refrozen_liquid_m) =
            Self::route_stage3_liquid_through_layers(
                incoming_liquid_m,
                layers,
                &mut cold_content_by_layer,
                reconstruct_liquid_temperature,
            );

        let cold_content_after_j_m2 = cold_content_by_layer.iter().sum::<f64>();
        let latent_refreeze_energy_j_m2 =
            refrozen_liquid_m * STAGE3_LATENT_HEAT_FUSION_J_KG * STAGE3_RHO_WATER_KG_M3;
        let liquid_closure_residual_m =
            incoming_liquid_m - routed_liquid_m - retained_delta_m - refrozen_liquid_m;
        let energy_closure_residual_j_m2 =
            surface_energy_j_m2
                + conduction_energy_j_m2
                + latent_refreeze_energy_j_m2
                + cold_content_export_j_m2
                - (cold_content_before_j_m2 - cold_content_after_j_m2);

        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_liquid_closure_residual_m"),
            liquid_closure_residual_m.abs(),
            None,
            Some(SNOW_STAGE3_LIQUID_CLOSURE_TOLERANCE_M),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_energy_residual_j_m2"),
            energy_closure_residual_j_m2.abs(),
            None,
            Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.mass_latent_identity_residual_j_m2"),
            mass_latent_identity_residual_j_m2.abs(),
            None,
            Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
        )?;

        let meltwater_temperature_c = if routed_liquid_m > WB11_ZERO_THRESHOLD {
            Some(Self::stage3_temperature(phase_class, 0.0)?)
        } else {
            None
        };

        let legacy_shadow_summary = if project_legacy_shadow {
            shadow_summary.as_ref()
        } else {
            None
        };
        Ok(DirectSnowStage3Resolution {
            outcome: DirectSnowStage3Outcome {
                enabled: true,
                meltwater_temperature_c,
                sublimation_m,
            },
            liquid_disposition_ledger: DirectSnowLiquidDispositionLedger {
                incoming_liquid_m,
                routed_liquid_m,
                retained_liquid_delta_m: retained_delta_m,
                refrozen_liquid_m,
                liquid_closure_residual_m,
            },
            diagnostics: hourly_surface_energy.map(|hourly_surface_energy| {
                DirectSnowStage3Diagnostics {
                    cold_content_before_j_m2,
                    cold_content_after_j_m2,
                    surface_energy_j_m2,
                    conduction_energy_j_m2,
                    latent_refreeze_energy_j_m2,
                    energy_closure_residual_j_m2,
                    shortwave_energy_j_m2,
                    longwave_energy_j_m2,
                    latent_energy_j_m2,
                    vapor_mass_exchange_kg_m2,
                    latent_mass_energy_j_m2,
                    cold_content_export_j_m2,
                    mass_latent_identity_residual_j_m2,
                    unused_positive_energy_j_m2,
                    shadow_complete_energy_j_m2: legacy_shadow_summary
                        .map_or(0.0, |shadow| shadow.complete_energy_j_m2),
                    shadow_cold_energy_change_j_m2: legacy_shadow_summary
                        .map_or(0.0, |shadow| shadow.cold_energy_change_j_m2),
                    shadow_excess_energy_j_m2: legacy_shadow_summary
                        .map_or(0.0, |shadow| shadow.excess_energy_j_m2),
                    shadow_sublimation_kg_m2: legacy_shadow_summary
                        .map_or(0.0, |shadow| shadow.sublimation_kg_m2),
                    shadow_melt_kg_m2: legacy_shadow_summary
                        .map_or(0.0, |shadow| shadow.melt_kg_m2),
                    shadow_unallocated_after_exhaustion_j_m2: legacy_shadow_summary.map_or(
                        0.0,
                        |shadow| shadow.unallocated_after_exhaustion_j_m2,
                    ),
                    shadow_maximum_energy_closure_residual_j_m2: legacy_shadow_summary.map_or(
                        0.0,
                        |shadow| shadow.maximum_energy_closure_residual_j_m2,
                    ),
                    thermal_domain_suspended_seconds,
                    minimum_unresolved_thermal_mass_kg_m2,
                    lower_thermal_volume_collapsed_seconds,
                    minimum_collapsed_lower_mass_kg_m2,
                    hourly_surface_energy: *hourly_surface_energy,
                }
            }),
            evaluation: shadow_summary.as_ref().map(Self::stage3_evaluation_diagnostics),
            reconciliation: shadow_summary
                .as_ref()
                .map(|summary| Box::new(summary.reconciliation.clone())),
        })
    }

    pub(super) fn stage3_evaluation_diagnostics(
        shadow: &Stage3ShadowSummary,
    ) -> DirectSnowStage3EvaluationDiagnostics {
        let tag = shadow.tag;
        let paired = tag.operator == SnowStage3EvaluationOperator::SameStatePairedCarrierV1;
        let complete_component_total_j_m2 = shadow.complete_shortwave_j_m2
            + shadow.complete_longwave_j_m2
            + shadow.complete_sensible_j_m2
            + shadow.complete_latent_j_m2
            + shadow.complete_advected_j_m2
            + shadow.internal_active_lower_conduction_j_m2;
        DirectSnowStage3EvaluationDiagnostics {
            operator: tag.operator,
            source_snapshot_id: tag.source_snapshot_id,
            support_id: tag.support_id,
            cadence_id: tag.cadence_id,
            carrier_id: tag.carrier_id,
            coverage_id: tag.coverage_id,
            claim_class: tag.claim_class,
            unresolved_boundaries_id: tag.unresolved_boundaries_id,
            pairing_id: tag.pairing_id,
            arm_ids: tag.arm_ids,
            arm_count: tag.arm_count,
            source_fingerprint: shadow.source_fingerprint,
            forcing_fingerprint: shadow.forcing_fingerprint,
            geometry_fingerprint: shadow.geometry_fingerprint,
            non_formulation_fingerprint: shadow.non_formulation_fingerprint,
            surface_arm_non_formulation_fingerprint: shadow
                .surface_arm_non_formulation_fingerprint,
            complete_arm_non_formulation_fingerprint: shadow
                .complete_arm_non_formulation_fingerprint,
            requested_seconds: shadow.requested_seconds,
            evaluated_seconds: shadow.evaluated_seconds,
            coverage_fraction: shadow.evaluated_seconds / shadow.requested_seconds,
            surface_arm_applicable: paired,
            surface_arm_shortwave_j_m2: shadow.surface_arm_shortwave_j_m2,
            surface_arm_longwave_j_m2: shadow.surface_arm_longwave_j_m2,
            surface_arm_latent_j_m2: shadow.surface_arm_latent_j_m2,
            surface_arm_sensible_applicable: false,
            surface_arm_advected_applicable: false,
            surface_arm_internal_conduction_applicable: false,
            surface_arm_total_j_m2: shadow.surface_arm_total_j_m2,
            complete_arm_shortwave_j_m2: shadow.complete_shortwave_j_m2,
            complete_arm_longwave_j_m2: shadow.complete_longwave_j_m2,
            complete_arm_sensible_j_m2: shadow.complete_sensible_j_m2,
            complete_arm_latent_j_m2: shadow.complete_latent_j_m2,
            complete_arm_advected_j_m2: shadow.complete_advected_j_m2,
            complete_arm_internal_active_lower_conduction_j_m2: shadow
                .internal_active_lower_conduction_j_m2,
            complete_arm_applicable: true,
            complete_arm_internal_conduction_applicable: !paired,
            complete_arm_vapor_mass_exchange_kg_m2: shadow.complete_vapor_mass_exchange_kg_m2,
            complete_arm_cold_content_export_j_m2: shadow.cold_content_export_j_m2,
            complete_arm_cold_content_export_applicable: !paired,
            complete_arm_available_ice_kg_m2: shadow.available_ice_kg_m2,
            complete_arm_available_ice_applicable: !paired,
            complete_arm_total_j_m2: shadow.complete_energy_j_m2,
            complete_arm_sequential_ledger_applicable: !paired,
            complete_arm_cold_energy_change_j_m2: shadow.cold_energy_change_j_m2,
            complete_arm_excess_energy_j_m2: shadow.excess_energy_j_m2,
            complete_arm_sublimation_kg_m2: shadow.sublimation_kg_m2,
            complete_arm_melt_kg_m2: shadow.melt_kg_m2,
            complete_arm_terminal_unallocated_j_m2: shadow.unallocated_after_exhaustion_j_m2,
            complete_arm_terminal_unallocated_applicable: !paired,
            complete_arm_component_residual_j_m2: shadow.complete_energy_j_m2
                - complete_component_total_j_m2,
            complete_arm_maximum_thermodynamic_residual_j_m2: shadow
                .maximum_energy_closure_residual_j_m2,
            hourly: shadow.hourly,
        }
    }

    fn stage3_liquid_routing_enabled(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        incoming_liquid_m: f64,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let surface_energy_enabled = inputs.surface_energy_options.longwave_model
            != SnowSurfaceLongwaveModel::Disabled
            || inputs.surface_energy_options.sublimation_model
                != SnowSurfaceSublimationModel::Disabled;
        if inputs.stage3_liquid_routing_model == SnowStage3LiquidRoutingModel::Disabled {
            if surface_energy_enabled {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.surface_energy_requires_stage3_provider",
                    1.0,
                    Some(0.0),
                    Some(0.0),
                ));
            }
            return Ok(false);
        }
        if inputs.stage3_liquid_routing_model
            != SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1
        {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_liquid_routing_model",
                1.0,
                Some(0.0),
                Some(0.0),
            ));
        }
        if !matches!(
            inputs.snow_density_model,
            SnowDensityModel::PhysicsBulkDensityCompactionV1
                | SnowDensityModel::PhysicsBulkMultilayerDensityV1
        ) {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_requires_bulk_or_multilayer_density_model",
                1.0,
                Some(0.0),
                Some(0.0),
            ));
        }
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_incoming_liquid_m"),
            incoming_liquid_m,
            Some(0.0),
            None,
        )?;
        if inputs.surface_energy_options.sublimation_model
            != SnowSurfaceSublimationModel::Disabled
            && matches!(
                inputs.snow_melt_model,
                SnowMeltModel::CoeOpenSublimationStageAV1
                    | SnowMeltModel::CoeOpenSublimationStageBV1
            )
        {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.incompatible_sublimation_selectors",
                1.0,
                Some(0.0),
                Some(0.0),
            ));
        }
        Ok(true)
    }

    fn stage3_evaluation_operator(
        phase_class: HillslopeKernelPhaseClass,
        complete_carrier_shadow: bool,
        evaluation_operator: Option<SnowStage3EvaluationOperator>,
    ) -> Result<Option<SnowStage3EvaluationOperator>, Wb11HydrologyKernelGuardError> {
        match (complete_carrier_shadow, evaluation_operator) {
            (false, Some(SnowStage3EvaluationOperator::PersistentAccumulationShadowV1)) => {
                Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_persistent_requires_stateful_api",
                    1.0,
                    Some(0.0),
                    Some(0.0),
                ))
            }
            (false, operator) => Ok(operator),
            (true, None | Some(SnowStage3EvaluationOperator::SequentialResolvedShadowV1)) => {
                Ok(Some(SnowStage3EvaluationOperator::SequentialResolvedShadowV1))
            }
            (
                true,
                Some(
                    SnowStage3EvaluationOperator::SameStatePairedCarrierV1
                    | SnowStage3EvaluationOperator::PersistentAccumulationShadowV1,
                ),
            ) => {
                Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_evaluation_request_conflict",
                    2.0,
                    Some(0.0),
                    Some(1.0),
                ))
            }
        }
    }

    fn validate_stage3_evaluation_tag(
        phase_class: HillslopeKernelPhaseClass,
        tag: Stage3EvaluationTag,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if tag != Stage3EvaluationTag::new(tag.operator) {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_evaluation_tag",
                0.0,
                Some(1.0),
                Some(1.0),
            ));
        }
        Ok(())
    }

    fn validate_stage3_shadow_summary(
        phase_class: HillslopeKernelPhaseClass,
        summary: &Stage3ShadowSummary,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_evaluation_requested_seconds"),
            summary.requested_seconds,
            Some(f64::EPSILON),
            Some(24.0 * STAGE3_SECONDS_PER_HOUR),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_evaluation_evaluated_seconds"),
            summary.evaluated_seconds,
            Some(0.0),
            Some(summary.requested_seconds),
        )?;
        let hourly_requested_seconds = summary
            .hourly
            .iter()
            .map(|hour| hour.requested_seconds)
            .sum::<f64>();
        let hourly_evaluated_seconds = summary
            .hourly
            .iter()
            .map(|hour| hour.evaluated_seconds)
            .sum::<f64>();
        for (symbol, residual) in [
            (
                "snow.stage3_evaluation_requested_support_residual_seconds",
                summary.requested_seconds - hourly_requested_seconds,
            ),
            (
                "snow.stage3_evaluation_evaluated_support_residual_seconds",
                summary.evaluated_seconds - hourly_evaluated_seconds,
            ),
            (
                "snow.stage3_evaluation_surface_component_residual_j_m2",
                summary.surface_arm_total_j_m2
                    - summary.surface_arm_shortwave_j_m2
                    - summary.surface_arm_longwave_j_m2
                    - summary.surface_arm_latent_j_m2,
            ),
            (
                "snow.stage3_evaluation_complete_component_residual_j_m2",
                summary.complete_energy_j_m2
                    - summary.complete_shortwave_j_m2
                    - summary.complete_longwave_j_m2
                    - summary.complete_sensible_j_m2
                    - summary.complete_latent_j_m2
                    - summary.complete_advected_j_m2
                    - summary.internal_active_lower_conduction_j_m2,
            ),
        ] {
            Self::require_direct_typed_snow_value_with(
                phase_class,
                || BoundarySymbol::from(symbol),
                residual.abs(),
                None,
                Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
            )?;
        }
        if summary.non_formulation_fingerprint == 0
            || summary.complete_arm_non_formulation_fingerprint == 0
        {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_evaluation_fingerprint",
                0.0,
                Some(1.0),
                None,
            ));
        }
        if summary.tag.operator == SnowStage3EvaluationOperator::SameStatePairedCarrierV1 {
            if summary.surface_arm_non_formulation_fingerprint
                != summary.complete_arm_non_formulation_fingerprint
            {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_evaluation_paired_fingerprint_equality",
                    0.0,
                    Some(1.0),
                    Some(1.0),
                ));
            }
        } else {
            let sequential_residual = summary.complete_energy_j_m2
                - summary.cold_energy_change_j_m2
                - STAGE3_LATENT_HEAT_FUSION_J_KG * summary.melt_kg_m2
                - summary.unallocated_after_exhaustion_j_m2;
            Self::require_direct_typed_snow_value_with(
                phase_class,
                || BoundarySymbol::from("snow.stage3_evaluation_sequential_residual_j_m2"),
                sequential_residual.abs(),
                None,
                Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
            )?;
        }
        Self::validate_stage3_reconciliation(phase_class, summary)?;
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn validate_stage3_reconciliation(
        phase_class: HillslopeKernelPhaseClass,
        summary: &Stage3ShadowSummary,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let reconciliation = &summary.reconciliation;
        if reconciliation.schema_version != 6 || reconciliation.tuples.len() > 1_440 {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_reconciliation_schema_or_tuple_count",
                1_441.0,
                Some(0.0),
                Some(1_440.0),
            ));
        }
        let mut elapsed_by_hour = [0.0_f64; 24];
        let mut count_by_hour = [0_usize; 24];
        let mut last_hour_index = None;
        let mut previous_sequential: Option<&DirectSnowStage3ReconciliationTuple> = None;
        for tuple in &reconciliation.tuples {
            if tuple.operator != summary.tag.operator
                || tuple.hour_index >= 24
                || last_hour_index.is_some_and(|last| tuple.hour_index < last)
                || tuple.source_fingerprint_fnv1a64 != summary.source_fingerprint
                || tuple.forcing_fingerprint_fnv1a64 != summary.forcing_fingerprint
                || tuple.geometry_fingerprint_fnv1a64 != summary.geometry_fingerprint
                || tuple.effective_input_fingerprint_fnv1a64 == 0
                || tuple.substep_index != count_by_hour[tuple.hour_index]
                || tuple.elapsed_start_seconds.to_bits()
                    != elapsed_by_hour[tuple.hour_index].to_bits()
                || !tuple.requested_seconds.is_finite()
                || tuple.requested_seconds <= 0.0
                || tuple.evaluated_seconds.to_bits() != tuple.duration_seconds.to_bits()
                || !tuple.duration_seconds.is_finite()
                || tuple.duration_seconds <= 0.0
                || !tuple.applicable
                || tuple.applicability_reason != "evaluated"
                || tuple.longwave_model_id != "dilley_unsworth_subcanopy_v1"
                || tuple.sublimation_model_id != "disabled"
                || tuple.elapsed_start_seconds + tuple.duration_seconds
                    > tuple.requested_seconds
            {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_reconciliation_identity_or_order",
                    24.0,
                    Some(0.0),
                    Some(23.0),
                ));
            }
            last_hour_index = Some(tuple.hour_index);
            if tuple.operator == SnowStage3EvaluationOperator::SequentialResolvedShadowV1 {
                if let Some(previous) = previous_sequential {
                    let continuous = previous.after_surface_applicable
                        && previous.active_layer_prefix_count_after
                            == Some(tuple.active_layer_prefix_count_before)
                        && previous.total_layer_count_after == tuple.total_layer_count_before
                        && previous.active_layer_state_fingerprint_after_fnv1a64
                            == Some(tuple.active_layer_state_fingerprint_before_fnv1a64)
                        && previous.total_layer_state_fingerprint_after_fnv1a64
                            == tuple.total_layer_state_fingerprint_before_fnv1a64
                        && previous.active_ice_mass_after_kg_m2.map(f64::to_bits)
                            == Some(tuple.active_ice_mass_before_kg_m2.to_bits())
                        && previous.active_depth_after_m.map(f64::to_bits)
                            == Some(tuple.active_depth_before_m.to_bits())
                        && previous.active_density_after_kg_m3.map(f64::to_bits)
                            == Some(tuple.active_density_before_kg_m3.to_bits())
                        && previous.active_cold_after_j_m2.map(f64::to_bits)
                            == Some(tuple.active_cold_before_j_m2.to_bits())
                        && previous.total_ice_mass_after_kg_m2.to_bits()
                            == tuple.total_ice_mass_before_kg_m2.to_bits()
                        && previous.total_cold_after_j_m2.to_bits()
                            == tuple.total_cold_before_j_m2.to_bits()
                        && previous.surface_temperature_after_c.map(f64::to_bits)
                            == Some(tuple.surface_temperature_before_c.to_bits());
                    if !continuous {
                        return Err(Self::stage3_domain_error(
                            phase_class,
                            "snow.stage3_reconciliation_sequential_continuity",
                            1.0,
                            Some(0.0),
                            Some(0.0),
                        ));
                    }
                }
                previous_sequential = Some(tuple);
            }
            let duration_seconds = tuple.duration_seconds;
            let tolerance = |floor: f64, operands: &[f64]| {
                floor.max(1.0e-12 * operands.iter().map(|value| value.abs()).sum::<f64>())
            };
            let incoming_reconstructed =
                tuple.hourly_radiation_mj_m2 * 1_000_000.0 / STAGE3_SECONDS_PER_HOUR;
            let shortwave_reconstructed =
                tuple.incoming_shortwave_w_m2 * (1.0 - tuple.snow_albedo_fraction);
            let rain_flux_reconstructed =
                tuple.rain_m * STAGE3_RHO_WATER_KG_M3 / STAGE3_SECONDS_PER_HOUR;
            let snow_flux_reconstructed = tuple.snowfall_geometric_m
                * 0.1
                * STAGE3_RHO_WATER_KG_M3
                / STAGE3_SECONDS_PER_HOUR;
            let external_reconstructed = tuple.net_shortwave_w_m2
                + tuple.net_longwave_w_m2
                + tuple.sensible_flux_w_m2
                + tuple.latent_flux_w_m2
                + tuple.precipitation_advected_flux_w_m2;
            let vapor_mass_reconstructed = tuple.vapor_mass_flux_kg_m2_s * duration_seconds;
            let reconstruction = [
                (
                    "snow.stage3_reconciliation_incoming_shortwave",
                    tuple.incoming_shortwave_w_m2 - incoming_reconstructed,
                    tolerance(
                        1.0e-10,
                        &[tuple.incoming_shortwave_w_m2, incoming_reconstructed],
                    ),
                ),
                (
                    "snow.stage3_reconciliation_net_shortwave",
                    tuple.net_shortwave_w_m2 - shortwave_reconstructed,
                    tolerance(
                        1.0e-10,
                        &[tuple.net_shortwave_w_m2, shortwave_reconstructed],
                    ),
                ),
                (
                    "snow.stage3_reconciliation_rain_mass_flux",
                    tuple.rain_mass_flux_kg_m2_s - rain_flux_reconstructed,
                    tolerance(
                        1.0e-12,
                        &[tuple.rain_mass_flux_kg_m2_s, rain_flux_reconstructed],
                    ),
                ),
                (
                    "snow.stage3_reconciliation_snow_mass_flux",
                    tuple.snow_mass_flux_kg_m2_s - snow_flux_reconstructed,
                    tolerance(
                        1.0e-12,
                        &[tuple.snow_mass_flux_kg_m2_s, snow_flux_reconstructed],
                    ),
                ),
                (
                    "snow.stage3_reconciliation_external_flux",
                    tuple.complete_external_flux_w_m2 - external_reconstructed,
                    tolerance(
                        1.0e-10,
                        &[
                            tuple.complete_external_flux_w_m2,
                            tuple.net_shortwave_w_m2,
                            tuple.net_longwave_w_m2,
                            tuple.sensible_flux_w_m2,
                            tuple.latent_flux_w_m2,
                            tuple.precipitation_advected_flux_w_m2,
                        ],
                    ),
                ),
                (
                    "snow.stage3_reconciliation_vapor_mass",
                    tuple.vapor_mass_exchange_kg_m2 - vapor_mass_reconstructed,
                    tolerance(
                        1.0e-12,
                        &[tuple.vapor_mass_exchange_kg_m2, vapor_mass_reconstructed],
                    ),
                ),
            ];
            for (symbol, residual, allowed) in reconstruction {
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from(symbol),
                    residual.abs(),
                    None,
                    Some(allowed),
                )?;
            }
            if let Some(latent_heat_j_kg) = tuple.surface_latent_heat_j_kg {
                let latent_residual = tuple.latent_flux_w_m2
                    - tuple.vapor_mass_flux_kg_m2_s * latent_heat_j_kg;
                let latent_reconstructed = tuple.vapor_mass_flux_kg_m2_s * latent_heat_j_kg;
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from("snow.stage3_reconciliation_latent_flux"),
                    latent_residual.abs(),
                    None,
                    Some(tolerance(
                        1.0e-10,
                        &[tuple.latent_flux_w_m2, latent_reconstructed],
                    )),
                )?;
            } else if tuple.turbulent_termination_status != "zero_wind"
                || tuple.latent_flux_w_m2 != 0.0
                || tuple.vapor_mass_flux_kg_m2_s != 0.0
            {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_reconciliation_latent_applicability",
                    tuple.latent_flux_w_m2,
                    Some(0.0),
                    Some(0.0),
                ));
            }
            if tuple.operator == SnowStage3EvaluationOperator::SameStatePairedCarrierV1 {
                let same_state = tuple.projection_id == "whole_column_immutable"
                    && tuple.after_surface_applicable
                    && tuple.after_surface_applicability_reason == "resolved_surface"
                    && tuple.active_ice_mass_after_kg_m2.is_some_and(|value| {
                    value.to_bits() == tuple.active_ice_mass_before_kg_m2.to_bits()
                }) && tuple.active_depth_after_m.is_some_and(|value| {
                    value.to_bits() == tuple.active_depth_before_m.to_bits()
                }) && tuple.active_density_after_kg_m3.is_some_and(|value| {
                    value.to_bits() == tuple.active_density_before_kg_m3.to_bits()
                }) && tuple.active_cold_after_j_m2.is_some_and(|value| {
                    value.to_bits() == tuple.active_cold_before_j_m2.to_bits()
                }) && tuple.surface_temperature_after_c.is_some_and(|value| {
                    value.to_bits() == tuple.surface_temperature_before_c.to_bits()
                }) && tuple.total_ice_mass_after_kg_m2.to_bits()
                    == tuple.total_ice_mass_before_kg_m2.to_bits()
                    && tuple.active_layer_prefix_count_after
                        == Some(tuple.active_layer_prefix_count_before)
                    && tuple.total_layer_count_after == tuple.total_layer_count_before
                    && tuple.total_cold_after_j_m2.to_bits()
                        == tuple.total_cold_before_j_m2.to_bits()
                    && tuple.active_layer_state_fingerprint_after_fnv1a64
                        == Some(tuple.active_layer_state_fingerprint_before_fnv1a64)
                    && tuple.total_layer_state_fingerprint_after_fnv1a64
                        == tuple.total_layer_state_fingerprint_before_fnv1a64
                    && tuple.sublimation_kg_m2.is_none()
                    && tuple.deposition_kg_m2.is_none()
                    && tuple.melt_kg_m2.is_none()
                    && tuple.active_cold_energy_change_j_m2.is_none()
                    && tuple.lower_cold_energy_change_j_m2.is_none()
                    && tuple.cold_content_export_j_m2.is_none()
                    && tuple.internal_active_lower_conduction_j_m2.is_none()
                    && tuple.legacy_sequential_complete_j_m2.is_none()
                    && tuple.energy_closure_residual_j_m2.is_none();
                if !same_state {
                    return Err(Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_reconciliation_same_state_endpoint",
                        1.0,
                        Some(0.0),
                        Some(0.0),
                    ));
                }
            } else if let (
                Some(melt_kg_m2),
                Some(sublimation_kg_m2),
                Some(deposition_kg_m2),
                Some(active_cold_change_j_m2),
                Some(lower_cold_change_j_m2),
                Some(cold_export_j_m2),
                Some(conduction_j_m2),
                Some(legacy_j_m2),
                Some(energy_closure_residual_j_m2),
            ) = (
                tuple.melt_kg_m2,
                tuple.sublimation_kg_m2,
                tuple.deposition_kg_m2,
                tuple.active_cold_energy_change_j_m2,
                tuple.lower_cold_energy_change_j_m2,
                tuple.cold_content_export_j_m2,
                tuple.internal_active_lower_conduction_j_m2,
                tuple.legacy_sequential_complete_j_m2,
                tuple.energy_closure_residual_j_m2,
            ) {
                let after_surface_valid = if tuple.after_surface_applicable {
                    tuple.after_surface_applicability_reason == "resolved_surface"
                        && tuple.active_layer_prefix_count_after.is_some()
                        && tuple.active_layer_state_fingerprint_after_fnv1a64.is_some()
                        && tuple.active_ice_mass_after_kg_m2.is_some()
                        && tuple.active_depth_after_m.is_some()
                        && tuple.active_density_after_kg_m3.is_some()
                        && tuple.active_cold_after_j_m2.is_some()
                        && tuple.surface_temperature_after_c.is_some()
                } else {
                    tuple.after_surface_applicability_reason
                        == "post_substep_no_resolved_surface"
                        && tuple.active_layer_prefix_count_after.is_none()
                        && tuple.active_layer_state_fingerprint_after_fnv1a64.is_none()
                        && tuple.active_ice_mass_after_kg_m2.is_none()
                        && tuple.active_depth_after_m.is_none()
                        && tuple.active_density_after_kg_m3.is_none()
                        && tuple.active_cold_after_j_m2.is_none()
                        && tuple.surface_temperature_after_c.is_none()
                };
                if tuple.projection_id != "aligned_active_dynamic" || !after_surface_valid {
                    return Err(Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_reconciliation_sequential_after_surface",
                        1.0,
                        Some(0.0),
                        Some(0.0),
                    ));
                }
                for (index, (symbol, residual)) in [
                    (
                        "snow.stage3_reconciliation_mass_endpoint",
                        tuple.total_ice_mass_after_kg_m2
                            - tuple.total_ice_mass_before_kg_m2
                            + melt_kg_m2
                            + sublimation_kg_m2
                            - deposition_kg_m2,
                    ),
                    (
                        "snow.stage3_reconciliation_cold_endpoint",
                        tuple.total_cold_after_j_m2
                            - tuple.total_cold_before_j_m2
                            + active_cold_change_j_m2
                            + lower_cold_change_j_m2
                            + cold_export_j_m2,
                    ),
                    (
                        "snow.stage3_reconciliation_legacy_bridge",
                        legacy_j_m2
                            - tuple.complete_external_flux_w_m2 * duration_seconds
                            - conduction_j_m2,
                    ),
                    (
                        "snow.stage3_reconciliation_energy_closure",
                        energy_closure_residual_j_m2,
                    ),
                ]
                .into_iter()
                .enumerate()
                {
                    Self::require_direct_typed_snow_value_with(
                        phase_class,
                        || BoundarySymbol::from(symbol),
                        residual.abs(),
                        None,
                        Some(if index == 0 {
                            tolerance(
                                1.0e-12,
                                &[
                                    tuple.total_ice_mass_after_kg_m2,
                                    tuple.total_ice_mass_before_kg_m2,
                                    melt_kg_m2,
                                    sublimation_kg_m2,
                                    deposition_kg_m2,
                                ],
                            )
                        } else {
                            let operands = if index == 1 {
                                vec![
                                    tuple.total_cold_after_j_m2,
                                    tuple.total_cold_before_j_m2,
                                    active_cold_change_j_m2,
                                    lower_cold_change_j_m2,
                                    cold_export_j_m2,
                                ]
                            } else if index == 2 {
                                vec![
                                    legacy_j_m2,
                                    tuple.complete_external_flux_w_m2 * duration_seconds,
                                    conduction_j_m2,
                                ]
                            } else {
                                vec![energy_closure_residual_j_m2]
                            };
                            tolerance(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2, &operands)
                        }),
                    )?;
                }
            } else {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_reconciliation_sequential_applicability",
                    1.0,
                    Some(0.0),
                    Some(0.0),
                ));
            }
            elapsed_by_hour[tuple.hour_index] += duration_seconds;
            count_by_hour[tuple.hour_index] += 1;
        }
        for hour_index in 0..24 {
            let status = reconciliation.hourly_status[hour_index];
            let terminal_status = summary.terminal_event.is_some_and(|event| {
                (hour_index == event.hour_index
                    && status.evaluated
                    && status.reason == "terminal_enthalpy_event_v1")
                    || (hour_index > event.hour_index
                        && !status.evaluated
                        && status.reason == "post_terminal_event_censored")
            });
            if (!terminal_status && status.evaluated != (count_by_hour[hour_index] > 0))
                || elapsed_by_hour[hour_index] > STAGE3_SECONDS_PER_HOUR
                || (!terminal_status && status.evaluated && status.reason != "evaluated")
                || (!status.evaluated
                    && !terminal_status
                    && !matches!(
                        status.reason,
                        "no_resolved_snow_at_day_start"
                            | "thin_pack_boundary_reached"
                            | "operator_not_selected"
                    ))
            {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_reconciliation_hourly_status",
                    24.0,
                    Some(0.0),
                    Some(23.0),
                ));
            }
            if summary.tag.operator == SnowStage3EvaluationOperator::SameStatePairedCarrierV1
                && status.evaluated
                && (count_by_hour[hour_index] != 1
                    || elapsed_by_hour[hour_index].to_bits()
                        != STAGE3_SECONDS_PER_HOUR.to_bits())
            {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_reconciliation_same_state_cadence",
                    elapsed_by_hour[hour_index],
                    Some(STAGE3_SECONDS_PER_HOUR),
                    Some(STAGE3_SECONDS_PER_HOUR),
                ));
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn prepare_stage3_layer_stack(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        aggregate: Stage3AggregateState,
        layers: &mut Vec<DirectSnowLayerState>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if inputs.snow_density_model == SnowDensityModel::PhysicsBulkMultilayerDensityV1 {
            if aggregate.swe_after_m > WB11_ZERO_THRESHOLD && layers.is_empty() {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_missing_layers_with_snow",
                    aggregate.swe_after_m,
                    None,
                    Some(0.0),
                ));
            }
            return Ok(());
        }

        if aggregate.swe_after_m <= WB11_ZERO_THRESHOLD {
            layers.clear();
            return Ok(());
        }
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_bulk_equivalent_runtime_depth_m"),
            aggregate.depth_after_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_bulk_equivalent_density_kg_m3"),
            aggregate.density_after_kg_m3,
            Some(WB11_ZERO_THRESHOLD),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        if layers.is_empty() {
            layers.extend(inputs.snow_layers.iter().copied());
        }
        Self::adjust_stage3_layer_swe_to_target(
            layers,
            aggregate.swe_after_m,
            aggregate.depth_after_m,
            aggregate.density_after_kg_m3,
            aggregate.settle_day_count_after,
        );
        Self::merge_stage3_bulk_equivalent_bottom_layers(layers);
        Self::apply_stage3_bulk_equivalent_density(phase_class, layers, aggregate)
    }

    pub(super) fn adjust_stage3_layer_swe_to_target(
        layers: &mut Vec<DirectSnowLayerState>,
        target_swe_m: f64,
        target_depth_m: f64,
        target_density_kg_m3: f64,
        settle_day_count: f64,
    ) {
        layers.retain(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m));
        let mut current_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        if current_swe_m <= WB11_ZERO_THRESHOLD {
            layers.push(DirectSnowLayerState::new(
                target_swe_m,
                target_depth_m,
                target_density_kg_m3,
                settle_day_count,
            ));
            return;
        }

        if current_swe_m > target_swe_m + STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M {
            let mut remaining_removal_m = current_swe_m - target_swe_m;
            while remaining_removal_m > 0.0 && !layers.is_empty() {
                let residual_mass_swe_m = layers[0].mass_swe_m - remaining_removal_m;
                if remaining_removal_m >= layers[0].mass_swe_m
                    || !snow_density_layer_has_resolved_mass(residual_mass_swe_m)
                {
                    remaining_removal_m =
                        (remaining_removal_m - layers[0].mass_swe_m).max(0.0);
                    layers.remove(0);
                } else {
                    let original_mass_m = layers[0].mass_swe_m;
                    let retained_fraction =
                        ((original_mass_m - remaining_removal_m) / original_mass_m).max(0.0);
                    layers[0].mass_swe_m -= remaining_removal_m;
                    layers[0].liquid_water_m *= retained_fraction;
                    layers[0].cold_content_j_m2 *= retained_fraction;
                    layers[0].refrozen_liquid_m *= retained_fraction;
                    remaining_removal_m = 0.0;
                }
            }
        } else if target_swe_m
            > current_swe_m + STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M
        {
            let added_swe_m = target_swe_m - current_swe_m;
            layers.insert(
                0,
                DirectSnowLayerState::new(
                    added_swe_m,
                    added_swe_m * STAGE3_RHO_WATER_KG_M3 / target_density_kg_m3,
                    target_density_kg_m3,
                    settle_day_count,
                ),
            );
        }

        current_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        if let Some(surface) = layers.first_mut() {
            let correction_m = target_swe_m - current_swe_m;
            if correction_m < 0.0 {
                let original_mass_m = surface.mass_swe_m;
                surface.mass_swe_m = (surface.mass_swe_m + correction_m).max(0.0);
                let retained_fraction = if original_mass_m > 0.0 {
                    surface.mass_swe_m / original_mass_m
                } else {
                    0.0
                };
                surface.liquid_water_m *= retained_fraction;
                surface.cold_content_j_m2 *= retained_fraction;
                surface.refrozen_liquid_m *= retained_fraction;
            } else {
                surface.mass_swe_m += correction_m;
            }
        }
    }

    fn merge_stage3_bulk_equivalent_bottom_layers(layers: &mut Vec<DirectSnowLayerState>) {
        while layers.len() > STAGE3_BULK_EQUIVALENT_MAX_LAYERS {
            let Some(bottom) = layers.pop() else {
                break;
            };
            let Some(previous_bottom) = layers.last_mut() else {
                layers.push(bottom);
                break;
            };
            let combined_mass_m = previous_bottom.mass_swe_m + bottom.mass_swe_m;
            if combined_mass_m > WB11_ZERO_THRESHOLD {
                previous_bottom.settle_day_count = (previous_bottom.settle_day_count
                    * previous_bottom.mass_swe_m
                    + bottom.settle_day_count * bottom.mass_swe_m)
                    / combined_mass_m;
            }
            previous_bottom.mass_swe_m = combined_mass_m;
            previous_bottom.liquid_water_m += bottom.liquid_water_m;
            previous_bottom.cold_content_j_m2 += bottom.cold_content_j_m2;
            previous_bottom.refrozen_liquid_m += bottom.refrozen_liquid_m;
        }
    }

    fn apply_stage3_bulk_equivalent_density(
        phase_class: HillslopeKernelPhaseClass,
        layers: &mut [DirectSnowLayerState],
        aggregate: Stage3AggregateState,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if layers.is_empty() && aggregate.swe_after_m > WB11_ZERO_THRESHOLD {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_missing_layers_with_snow",
                aggregate.swe_after_m,
                None,
                Some(0.0),
            ));
        }
        for layer in layers.iter_mut() {
            if layer.settle_day_count <= WB11_ZERO_THRESHOLD {
                layer.settle_day_count = aggregate.settle_day_count_after;
            }
            layer.density_kg_m3 = aggregate.density_after_kg_m3;
            layer.thickness_m = layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3
                / aggregate.density_after_kg_m3;
            if aggregate.swe_after_m > STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M {
                layer.cold_content_j_m2 = Self::stage3_layer_cold_content_j_m2(layer);
            }
            layer.refrozen_liquid_m = layer.refrozen_liquid_m.max(0.0);
            layer.liquid_water_m = layer.liquid_water_m.max(0.0);
        }

        let layer_swe_sum_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        let layer_depth_sum_m = layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_bulk_equivalent_layer_swe_residual_m"),
            (layer_swe_sum_m - aggregate.swe_after_m).abs(),
            None,
            Some(STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_bulk_equivalent_layer_depth_residual_m"),
            (layer_depth_sum_m - aggregate.depth_after_m).abs(),
            None,
            Some(STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M),
        )
    }

    fn route_stage3_liquid_through_layers(
        incoming_liquid_m: f64,
        layers: &mut [DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
        reconstruct_temperature: bool,
    ) -> (f64, f64, f64) {
        let mut liquid_to_route_m = incoming_liquid_m;
        let mut retained_delta_m = 0.0;
        let mut refrozen_liquid_m = 0.0;
        for (layer, cold_content) in layers.iter_mut().zip(cold_content_by_layer.iter_mut()) {
            let refreeze_capacity_m =
                (*cold_content / (STAGE3_LATENT_HEAT_FUSION_J_KG * STAGE3_RHO_WATER_KG_M3))
                    .max(0.0);
            let refrozen_here_m = liquid_to_route_m.min(refreeze_capacity_m);
            liquid_to_route_m -= refrozen_here_m;
            *cold_content -=
                refrozen_here_m * STAGE3_LATENT_HEAT_FUSION_J_KG * STAGE3_RHO_WATER_KG_M3;
            refrozen_liquid_m += refrozen_here_m;

            let capacity_m =
                Self::stage3_layer_liquid_holding_capacity_m(layer.thickness_m, layer.density_kg_m3);
            let available_capacity_m = (capacity_m - layer.liquid_water_m).max(0.0);
            let retained_here_m = liquid_to_route_m.min(available_capacity_m);
            liquid_to_route_m -= retained_here_m;
            retained_delta_m += retained_here_m;

            layer.liquid_water_m += retained_here_m;
            layer.refrozen_liquid_m += refrozen_here_m;
            layer.cold_content_j_m2 = (*cold_content).max(0.0);
            if reconstruct_temperature {
                layer.temperature_c = Self::stage3_temperature_from_cold_content(layer);
            }
        }
        (liquid_to_route_m.max(0.0), retained_delta_m, refrozen_liquid_m)
    }

    fn validate_stage3_layer(
        phase_class: HillslopeKernelPhaseClass,
        layer: &DirectSnowLayerState,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_mass_swe_m"),
            layer.mass_swe_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_thickness_m"),
            layer.thickness_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_density_kg_m3"),
            layer.density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_settle_day_count"),
            layer.settle_day_count,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_temperature_c"),
            layer.temperature_c,
            None,
            Some(0.0),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_liquid_water_m"),
            layer.liquid_water_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_cold_content_j_m2"),
            layer.cold_content_j_m2,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_refrozen_liquid_m"),
            layer.refrozen_liquid_m,
            Some(0.0),
            None,
        )
    }

    fn stage3_total_ice_mass_swe_m(layers: &[DirectSnowLayerState]) -> f64 {
        layers.iter().map(|layer| layer.mass_swe_m).sum()
    }

    fn stage3_control_volume_masses_swe_m(
        layers: &[DirectSnowLayerState],
        active_layer_count: usize,
    ) -> (f64, f64) {
        let active_mass_swe_m = Self::stage3_total_ice_mass_swe_m(&layers[..active_layer_count]);
        let lower_mass_swe_m = if active_layer_count < layers.len() {
            Self::stage3_total_ice_mass_swe_m(&layers[active_layer_count..])
        } else {
            0.0
        };
        (active_mass_swe_m, lower_mass_swe_m)
    }

    pub(super) fn stage3_lower_volume_is_subresolution_swe_m(lower_mass_swe_m: f64) -> bool {
        lower_mass_swe_m > 0.0
            && lower_mass_swe_m < STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M
    }

    fn stage3_layer_cold_content_j_m2(layer: &DirectSnowLayerState) -> f64 {
        let cold_content = if layer.cold_content_j_m2 > WB11_ZERO_THRESHOLD {
            layer.cold_content_j_m2
        } else if layer.temperature_c >= 0.0 || layer.mass_swe_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            layer.mass_swe_m
                * STAGE3_RHO_WATER_KG_M3
                * STAGE3_SPECIFIC_HEAT_ICE_J_KG_K
                * (-layer.temperature_c)
        };
        cold_content.max(0.0)
    }

    // This contract adapter keeps the independently reconstructable longwave,
    // vapor-mass, latent-energy, and total-energy operands adjacent.
    #[allow(clippy::too_many_lines)]
    fn align_stage3_active_layer_boundary(
        layers: &mut Vec<DirectSnowLayerState>,
        cold_content_by_layer: &mut Vec<f64>,
    ) -> usize {
        let total_depth_m = layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
        let target_depth_m = total_depth_m.min(STAGE3_ACTIVE_LAYER_MAX_DEPTH_M);
        let mut depth_above_m = 0.0;
        for index in 0..layers.len() {
            let layer = layers[index];
            let depth_below_m = depth_above_m + layer.thickness_m;
            if depth_below_m + WB11_ZERO_THRESHOLD < target_depth_m {
                depth_above_m = depth_below_m;
                continue;
            }
            let active_depth_in_layer_m = target_depth_m - depth_above_m;
            if active_depth_in_layer_m > WB11_ZERO_THRESHOLD
                && active_depth_in_layer_m
                    < layer.thickness_m - WB11_ZERO_THRESHOLD
            {
                let active_fraction = active_depth_in_layer_m / layer.thickness_m;
                let lower_fraction = 1.0 - active_fraction;
                let original_cold_content = cold_content_by_layer[index];
                let mut active_part = layer;
                active_part.mass_swe_m *= active_fraction;
                active_part.thickness_m = active_depth_in_layer_m;
                active_part.liquid_water_m *= active_fraction;
                active_part.cold_content_j_m2 = original_cold_content * active_fraction;
                active_part.refrozen_liquid_m *= active_fraction;
                let mut lower_part = layer;
                lower_part.mass_swe_m *= lower_fraction;
                lower_part.thickness_m -= active_depth_in_layer_m;
                lower_part.liquid_water_m *= lower_fraction;
                lower_part.cold_content_j_m2 = original_cold_content * lower_fraction;
                lower_part.refrozen_liquid_m *= lower_fraction;
                layers[index] = active_part;
                layers.insert(index + 1, lower_part);
                cold_content_by_layer[index] = active_part.cold_content_j_m2;
                cold_content_by_layer.insert(index + 1, lower_part.cold_content_j_m2);
            }
            return index + 1;
        }
        layers.len()
    }

    fn normalize_stage3_control_volume_temperature(
        layers: &mut [DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
    ) {
        if layers.is_empty() {
            return;
        }
        let total_mass_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        let total_cold_content_j_m2 = cold_content_by_layer.iter().sum::<f64>();
        let temperature_c = Self::stage3_temperature_from_cold_content_values(
            total_mass_swe_m,
            total_cold_content_j_m2,
        );
        for (layer, cold_content) in layers.iter_mut().zip(cold_content_by_layer.iter_mut()) {
            *cold_content =
                total_cold_content_j_m2 * layer.mass_swe_m / total_mass_swe_m;
            layer.cold_content_j_m2 = *cold_content;
            layer.temperature_c = temperature_c;
        }
    }

    fn coalesce_stage3_thermal_fragments(
        layers: &mut Vec<DirectSnowLayerState>,
        cold_content_by_layer: &mut Vec<f64>,
        mut active_layer_count: usize,
    ) -> usize {
        let mut index = 0;
        while index + 1 < layers.len() {
            if index + 1 == active_layer_count
                || (layers[index].density_kg_m3 - layers[index + 1].density_kg_m3).abs()
                    > WB11_ZERO_THRESHOLD
                || (layers[index].settle_day_count - layers[index + 1].settle_day_count).abs()
                    > WB11_ZERO_THRESHOLD
                || (layers[index].temperature_c - layers[index + 1].temperature_c).abs()
                    > WB11_ZERO_THRESHOLD
            {
                index += 1;
                continue;
            }
            let upper = layers[index];
            let lower = layers[index + 1];
            let merged = Self::merge_stage3_thermal_fragments(
                upper,
                lower,
                cold_content_by_layer[index] + cold_content_by_layer[index + 1],
            );
            layers[index] = merged;
            layers.remove(index + 1);
            cold_content_by_layer[index] = merged.cold_content_j_m2;
            cold_content_by_layer.remove(index + 1);
            if index + 1 < active_layer_count {
                active_layer_count -= 1;
            }
        }
        active_layer_count
    }

    fn merge_stage3_thermal_fragments(
        upper: DirectSnowLayerState,
        lower: DirectSnowLayerState,
        cold_content_j_m2: f64,
    ) -> DirectSnowLayerState {
        let mass_swe_m = upper.mass_swe_m + lower.mass_swe_m;
        let thickness_m = upper.thickness_m + lower.thickness_m;
        let mut projected = upper;
        projected.mass_swe_m = mass_swe_m;
        projected.thickness_m = thickness_m;
        projected.density_kg_m3 = mass_swe_m * STAGE3_RHO_WATER_KG_M3 / thickness_m;
        projected.settle_day_count = if mass_swe_m > WB11_ZERO_THRESHOLD {
            (upper.settle_day_count * upper.mass_swe_m
                + lower.settle_day_count * lower.mass_swe_m)
                / mass_swe_m
        } else {
            0.0
        };
        projected.liquid_water_m = upper.liquid_water_m + lower.liquid_water_m;
        projected.cold_content_j_m2 = cold_content_j_m2;
        projected.temperature_c =
            Self::stage3_temperature_from_cold_content_values(mass_swe_m, cold_content_j_m2);
        projected.refrozen_liquid_m = upper.refrozen_liquid_m + lower.refrozen_liquid_m;
        projected
    }

    fn stage3_control_volume_state(
        phase_class: HillslopeKernelPhaseClass,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &[f64],
        atmospheric_pressure_pa: f64,
    ) -> Result<Stage3ThermalControlVolume, Wb11HydrologyKernelGuardError> {
        let mass_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        let depth_m = layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
        let cold_content_j_m2 = cold_content_by_layer.iter().sum::<f64>();
        let temperature_c =
            Self::stage3_temperature_from_cold_content_values(mass_swe_m, cold_content_j_m2);
        let temperature = Self::stage3_temperature(phase_class, temperature_c)?;
        let pressure = PressurePascals::try_new(atmospheric_pressure_pa).map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_atmospheric_pressure_pa",
                atmospheric_pressure_pa,
                Some(0.0),
                None,
            )
        })?;
        let mut resistance_m2_k_w = 0.0;
        for (layer_index, layer) in layers.iter().enumerate() {
            let conductivity = snow_effective_thermal_conductivity_snobal(
                layer.density_kg_m3,
                temperature,
                pressure,
            )
            .map_err(|source| {
                Wb11HydrologyKernelGuardError::SnowStage3Conductivity(Box::new(
                    SnowStage3ConductivityError {
                    phase_class,
                    source,
                    layer_index,
                    layer: *layer,
                    control_volume_layers: layers.to_vec(),
                    control_volume_temperature: temperature,
                    atmospheric_pressure_pa,
                    },
                ))
            })?;
            resistance_m2_k_w +=
                layer.thickness_m / conductivity.as_watts_per_meter_kelvin();
        }
        Ok(Stage3ThermalControlVolume {
            mass_swe_m,
            depth_m,
            density_kg_m3: mass_swe_m * STAGE3_RHO_WATER_KG_M3 / depth_m,
            cold_content_j_m2,
            conductivity_w_m_k: depth_m / resistance_m2_k_w,
        })
    }

    fn stage3_substep_seconds(
        layers: &[DirectSnowLayerState],
        active_layer_count: usize,
    ) -> f64 {
        let active_mass_kg_m2 = layers[..active_layer_count]
            .iter()
            .map(|layer| layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>();
        let lower_mass_kg_m2 = layers[active_layer_count..]
            .iter()
            .map(|layer| layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>();
        let minimum_mass_kg_m2 = if lower_mass_kg_m2 > WB11_ZERO_THRESHOLD {
            active_mass_kg_m2.min(lower_mass_kg_m2)
        } else {
            active_mass_kg_m2
        };
        if minimum_mass_kg_m2 >= STAGE3_NORMAL_TIMESTEP_MASS_KG_M2 {
            STAGE3_SECONDS_PER_HOUR
        } else if minimum_mass_kg_m2 >= STAGE3_MEDIUM_TIMESTEP_MASS_KG_M2 {
            STAGE3_MEDIUM_TIMESTEP_SECONDS
        } else {
            STAGE3_SMALL_TIMESTEP_SECONDS
        }
    }

    fn apply_stage3_control_volume_energy(
        energy_j_m2: f64,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
        start: usize,
        end: usize,
    ) -> f64 {
        if start >= end {
            return 0.0;
        }
        let cold_content_j_m2 = cold_content_by_layer[start..end].iter().sum::<f64>();
        if energy_j_m2 >= 0.0 {
            let used_j_m2 = energy_j_m2.min(cold_content_j_m2);
            if cold_content_j_m2 > WB11_ZERO_THRESHOLD {
                let retained_fraction = 1.0 - used_j_m2 / cold_content_j_m2;
                for cold_content in &mut cold_content_by_layer[start..end] {
                    *cold_content *= retained_fraction;
                }
            }
            used_j_m2
        } else {
            let total_mass_swe_m = layers[start..end]
                .iter()
                .map(|layer| layer.mass_swe_m)
                .sum::<f64>();
            for (layer, cold_content) in layers[start..end]
                .iter()
                .zip(&mut cold_content_by_layer[start..end])
            {
                *cold_content +=
                    -energy_j_m2 * layer.mass_swe_m / total_mass_swe_m;
            }
            energy_j_m2
        }
    }

    fn apply_stage3_active_lower_conduction(
        phase_class: HillslopeKernelPhaseClass,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
        active_layer_count: usize,
        duration_seconds: f64,
        atmospheric_pressure_pa: f64,
    ) -> Result<Stage3ConductionExchange, Wb11HydrologyKernelGuardError> {
        if active_layer_count >= layers.len() {
            return Ok(Stage3ConductionExchange::ZERO);
        }
        let active = Self::stage3_control_volume_state(
            phase_class,
            &layers[..active_layer_count],
            &cold_content_by_layer[..active_layer_count],
            atmospheric_pressure_pa,
        )?;
        let lower = Self::stage3_control_volume_state(
            phase_class,
            &layers[active_layer_count..],
            &cold_content_by_layer[active_layer_count..],
            atmospheric_pressure_pa,
        )?;
        let requested_transfer_j_m2 =
            Self::stage3_active_lower_conduction_energy(
                phase_class,
                active,
                lower,
                duration_seconds,
            )?;
        let exchange = if requested_transfer_j_m2 > 0.0 {
            let transfer_j_m2 = requested_transfer_j_m2.min(
                cold_content_by_layer[..active_layer_count]
                    .iter()
                    .sum::<f64>(),
            );
            Self::apply_stage3_control_volume_energy(
                transfer_j_m2,
                layers,
                cold_content_by_layer,
                0,
                active_layer_count,
            );
            Self::apply_stage3_control_volume_energy(
                -transfer_j_m2,
                layers,
                cold_content_by_layer,
                active_layer_count,
                layers.len(),
            );
            Stage3ConductionExchange {
                requested_active_energy: requested_transfer_j_m2,
                flux: transfer_j_m2 / duration_seconds,
                active_energy: transfer_j_m2,
                lower_energy: -transfer_j_m2,
                rejected_active_energy: requested_transfer_j_m2 - transfer_j_m2,
            }
        } else if requested_transfer_j_m2 < 0.0 {
            let transfer_j_m2 = (-requested_transfer_j_m2).min(
                cold_content_by_layer[active_layer_count..]
                    .iter()
                    .sum::<f64>(),
            );
            Self::apply_stage3_control_volume_energy(
                -transfer_j_m2,
                layers,
                cold_content_by_layer,
                0,
                active_layer_count,
            );
            Self::apply_stage3_control_volume_energy(
                transfer_j_m2,
                layers,
                cold_content_by_layer,
                active_layer_count,
                layers.len(),
            );
            Stage3ConductionExchange {
                requested_active_energy: requested_transfer_j_m2,
                flux: -transfer_j_m2 / duration_seconds,
                active_energy: -transfer_j_m2,
                lower_energy: transfer_j_m2,
                rejected_active_energy: requested_transfer_j_m2 + transfer_j_m2,
            }
        } else {
            Stage3ConductionExchange::ZERO
        };
        Ok(exchange)
    }

    fn stage3_active_lower_conduction_energy(
        phase_class: HillslopeKernelPhaseClass,
        active: Stage3ThermalControlVolume,
        lower: Stage3ThermalControlVolume,
        duration_seconds: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let active_temperature_c = Self::stage3_temperature_from_cold_content_values(
            active.mass_swe_m,
            active.cold_content_j_m2,
        );
        let lower_temperature_c = Self::stage3_temperature_from_cold_content_values(
            lower.mass_swe_m,
            lower.cold_content_j_m2,
        );
        let flux = conductive_heat_flux(
            ThermalConductivityWattsPerMeterKelvin::try_new(active.conductivity_w_m_k)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_active_conductivity_w_m_k",
                        active.conductivity_w_m_k,
                        Some(0.0),
                        None,
                    )
                })?,
            ThermalConductivityWattsPerMeterKelvin::try_new(lower.conductivity_w_m_k)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_lower_conductivity_w_m_k",
                        lower.conductivity_w_m_k,
                        Some(0.0),
                        None,
                    )
                })?,
            Self::stage3_temperature(phase_class, active_temperature_c)?,
            Self::stage3_temperature(phase_class, lower_temperature_c)?,
            PositiveLengthMeters::try_new(active.depth_m).map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_active_depth_m",
                    active.depth_m,
                    Some(0.0),
                    None,
                )
            })?,
            PositiveLengthMeters::try_new(lower.depth_m).map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_lower_depth_m",
                    lower.depth_m,
                    Some(0.0),
                    None,
                )
            })?,
        )
        .map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_active_lower_conduction_w_m2",
                active_temperature_c - lower_temperature_c,
                None,
                None,
            )
        })?;
        Ok(flux.as_watts_per_square_meter() * duration_seconds)
    }

    pub(super) fn remove_stage3_active_sublimation(
        requested_m: f64,
        layers: &mut Vec<DirectSnowLayerState>,
        cold_content_by_layer: &mut Vec<f64>,
        active_layer_count: &mut usize,
    ) -> (f64, f64, usize) {
        let mut remaining_m = requested_m.min(
            layers[..*active_layer_count]
                .iter()
                .map(|layer| layer.mass_swe_m)
                .sum::<f64>(),
        );
        let mut removed_m = 0.0;
        let mut exported_j_m2 = 0.0;
        let mut removed_layer_count = 0;
        while remaining_m > WB11_ZERO_THRESHOLD && *active_layer_count > 0 {
            let layer_mass_m = layers[0].mass_swe_m;
            let removal_m = remaining_m.min(layer_mass_m);
            let fraction_removed = removal_m / layer_mass_m;
            let exported = cold_content_by_layer[0] * fraction_removed;
            cold_content_by_layer[0] -= exported;
            exported_j_m2 += exported;
            removed_m += removal_m;
            remaining_m -= removal_m;
            layers[0].mass_swe_m -= removal_m;
            layers[0].liquid_water_m *= 1.0 - fraction_removed;
            layers[0].refrozen_liquid_m *= 1.0 - fraction_removed;
            layers[0].thickness_m =
                layers[0].mass_swe_m * STAGE3_RHO_WATER_KG_M3 / layers[0].density_kg_m3;
            if !snow_density_layer_has_resolved_mass(layers[0].mass_swe_m) {
                layers.remove(0);
                cold_content_by_layer.remove(0);
                *active_layer_count -= 1;
                removed_layer_count += 1;
            }
        }
        (removed_m, exported_j_m2, removed_layer_count)
    }

    fn accumulate_stage3_hour_diagnostics(
        hourly: &mut DirectSnowSurfaceEnergyHourDiagnostics,
        substep: &Stage3SubstepDiagnostics,
    ) {
        let Stage3SubstepDiagnostics {
            surface,
            duration_seconds,
            applied_j_m2,
            unused_j_m2,
            active,
            lower,
            conduction,
            active_energy_closure_residual_j_m2,
            lower_energy_closure_residual_j_m2,
            atmospheric_pressure_pa,
        } = *substep;
        let weight = duration_seconds / STAGE3_SECONDS_PER_HOUR;
        hourly.surface_temperature_c += surface.surface_temperature_c * weight;
        hourly.canopy_temperature_equals_air &= surface.canopy_temperature_equals_air;
        hourly.atmospheric_longwave_w_m2 += surface.atmospheric_longwave_w_m2 * weight;
        hourly.canopy_longwave_w_m2 += surface.canopy_longwave_w_m2 * weight;
        hourly.sky_view_fraction += surface.sky_view_fraction * weight;
        hourly.subcanopy_longwave_w_m2 += surface.subcanopy_longwave_w_m2 * weight;
        hourly.outgoing_longwave_w_m2 += surface.outgoing_longwave_w_m2 * weight;
        hourly.net_longwave_w_m2 += surface.net_longwave_w_m2 * weight;
        hourly.net_shortwave_w_m2 += surface.net_shortwave_w_m2 * weight;
        hourly.vapor_mass_exchange_kg_m2 += surface.vapor_mass_exchange_kg_m2;
        hourly.shadow_sensible_flux_w_m2 += surface.shadow_sensible_flux_w_m2 * weight;
        hourly.shadow_latent_flux_w_m2 += surface.shadow_latent_flux_w_m2 * weight;
        hourly.shadow_advected_flux_w_m2 += surface.shadow_advected_flux_w_m2 * weight;
        if surface.shadow_complete_carrier_evaluated {
            hourly.shadow_complete_energy_j_m2 +=
                surface.shadow_complete_energy_j_m2 + conduction.active_energy;
        }
        hourly.shadow_vapor_mass_exchange_kg_m2 +=
            surface.shadow_vapor_mass_exchange_kg_m2;
        hourly.shadow_complete_carrier_evaluated |= surface.shadow_complete_carrier_evaluated;
        hourly.potential_surface_energy_j_m2 += surface.potential_surface_energy_j_m2;
        hourly.applied_surface_energy_j_m2 += applied_j_m2;
        hourly.unused_positive_energy_j_m2 += unused_j_m2;
        hourly.active_layer_mass_kg_m2 +=
            active.mass_swe_m * STAGE3_RHO_WATER_KG_M3 * weight;
        hourly.active_layer_depth_m += active.depth_m * weight;
        hourly.active_layer_temperature_c +=
            Self::stage3_temperature_from_cold_content_values(
                active.mass_swe_m,
                active.cold_content_j_m2,
            ) * weight;
        hourly.active_layer_cold_content_j_m2 += active.cold_content_j_m2 * weight;
        hourly.active_layer_effective_conductivity_w_m_k +=
            active.conductivity_w_m_k * weight;
        hourly.active_layer_thermal_resistance_m2_k_w +=
            active.depth_m / active.conductivity_w_m_k * weight;
        hourly.atmospheric_pressure_pa += atmospheric_pressure_pa * weight;
        if let Some(lower) = lower {
            hourly.lower_layer_present_fraction += weight;
            hourly.lower_layer_mass_kg_m2 +=
                lower.mass_swe_m * STAGE3_RHO_WATER_KG_M3 * weight;
            hourly.lower_layer_depth_m += lower.depth_m * weight;
            hourly.lower_layer_temperature_c +=
                Self::stage3_temperature_from_cold_content_values(
                    lower.mass_swe_m,
                    lower.cold_content_j_m2,
                ) * weight;
            hourly.lower_layer_cold_content_j_m2 += lower.cold_content_j_m2 * weight;
            hourly.lower_layer_effective_conductivity_w_m_k +=
                lower.conductivity_w_m_k * weight;
            hourly.lower_layer_thermal_resistance_m2_k_w +=
                lower.depth_m / lower.conductivity_w_m_k * weight;
            Self::accumulate_stage3_conduction_diagnostics(
                hourly,
                lower,
                substep,
                weight,
            );
        }
        hourly.active_lower_conduction_w_m2 += conduction.flux * weight;
        hourly.substep_count += 1;
        if hourly.minimum_substep_seconds == 0.0 {
            hourly.minimum_substep_seconds = duration_seconds;
        } else {
            hourly.minimum_substep_seconds =
                hourly.minimum_substep_seconds.min(duration_seconds);
        }
        hourly.maximum_active_energy_closure_residual_j_m2 = hourly
            .maximum_active_energy_closure_residual_j_m2
            .max(active_energy_closure_residual_j_m2.abs());
        hourly.maximum_lower_energy_closure_residual_j_m2 = hourly
            .maximum_lower_energy_closure_residual_j_m2
            .max(lower_energy_closure_residual_j_m2.abs());
        hourly.maximum_conduction_cancellation_residual_j_m2 = hourly
            .maximum_conduction_cancellation_residual_j_m2
            .max((conduction.active_energy + conduction.lower_energy).abs());
    }

    fn accumulate_stage3_conduction_diagnostics(
        hourly: &mut DirectSnowSurfaceEnergyHourDiagnostics,
        lower: Stage3ThermalControlVolume,
        substep: &Stage3SubstepDiagnostics,
        weight: f64,
    ) {
        let active = substep.active;
        let conduction = substep.conduction;
        let requested_conduction_w_m2 =
            conduction.requested_active_energy / substep.duration_seconds;
        let rejected_conduction_w_m2 =
            conduction.rejected_active_energy / substep.duration_seconds;
        hourly.requested_active_lower_conduction_w_m2 +=
            requested_conduction_w_m2 * weight;
        hourly.rejected_active_lower_conduction_w_m2 +=
            rejected_conduction_w_m2 * weight;
        if requested_conduction_w_m2.abs()
            <= hourly.peak_substep_requested_g0_w_m2.abs()
        {
            return;
        }
        hourly.peak_substep_applied_g0_w_m2 = conduction.flux;
        hourly.peak_substep_requested_g0_w_m2 =
            requested_conduction_w_m2;
        hourly.peak_substep_rejected_g0_w_m2 =
            rejected_conduction_w_m2;
        hourly.peak_substep_pressure_pa = substep.atmospheric_pressure_pa;
        hourly.peak_substep_active_temperature_c =
            Self::stage3_temperature_from_cold_content_values(
                active.mass_swe_m,
                active.cold_content_j_m2,
            );
        hourly.peak_substep_lower_temperature_c =
            Self::stage3_temperature_from_cold_content_values(
                lower.mass_swe_m,
                lower.cold_content_j_m2,
            );
        hourly.peak_substep_active_depth_m = active.depth_m;
        hourly.peak_substep_lower_depth_m = lower.depth_m;
        hourly.peak_substep_active_conductivity_w_m_k =
            active.conductivity_w_m_k;
        hourly.peak_substep_lower_conductivity_w_m_k =
            lower.conductivity_w_m_k;
        hourly.peak_substep_active_resistance_m2_k_w =
            active.depth_m / active.conductivity_w_m_k;
        hourly.peak_substep_lower_resistance_m2_k_w =
            lower.depth_m / lower.conductivity_w_m_k;
    }

    fn finish_stage3_hour_diagnostics(
        hourly: &mut DirectSnowSurfaceEnergyHourDiagnostics,
        latent_energy_j_m2: f64,
        latent_mass_energy_j_m2: f64,
    ) {
        hourly.latent_flux_w_m2 = latent_energy_j_m2 / STAGE3_SECONDS_PER_HOUR;
        if hourly.vapor_mass_exchange_kg_m2 != 0.0 {
            hourly.latent_heat_j_kg =
                latent_mass_energy_j_m2 / hourly.vapor_mass_exchange_kg_m2;
        }
    }

    fn stage3_layer_liquid_holding_capacity_m(
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
    ) -> f64 {
        if snow_depth_m <= WB11_ZERO_THRESHOLD
            || snow_density_kg_m3 <= WB11_ZERO_THRESHOLD
            || snow_density_kg_m3 >= SIMIMPL29_RHO_ICE_KG_M3
        {
            return 0.0;
        }
        let pore_fraction = 1.0 - (snow_density_kg_m3 / SIMIMPL29_RHO_ICE_KG_M3);
        (SIMIMPL29_LIQUID_HOLDING_CAPACITY_VOLUME_FRACTION * pore_fraction * snow_depth_m)
            .max(0.0)
    }

    fn stage3_temperature_from_cold_content(layer: &DirectSnowLayerState) -> f64 {
        Self::stage3_temperature_from_cold_content_values(layer.mass_swe_m, layer.cold_content_j_m2)
    }

    fn stage3_temperature_from_cold_content_values(mass_swe_m: f64, cold_content_j_m2: f64) -> f64 {
        if cold_content_j_m2 <= WB11_ZERO_THRESHOLD || mass_swe_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            -cold_content_j_m2
                / (mass_swe_m * STAGE3_RHO_WATER_KG_M3 * STAGE3_SPECIFIC_HEAT_ICE_J_KG_K)
        }
    }

    fn stage3_temperature(
        phase_class: HillslopeKernelPhaseClass,
        value_c: f64,
    ) -> Result<TemperatureCelsius, Wb11HydrologyKernelGuardError> {
        TemperatureCelsius::try_new(value_c).map_err(|_| {
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from("snow.stage3_temperature_c"),
                value: value_c,
            }
        })
    }

    fn stage3_domain_error(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &'static str,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Wb11HydrologyKernelGuardError {
        if !value.is_finite() {
            return Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value,
            };
        }
        Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
            phase_class,
            symbol: BoundarySymbol::from(symbol),
            value,
            minimum,
            maximum,
        }
    }

}

#[cfg(test)]
mod stage3_evaluation_validation_tests {
    use super::*;

    fn reconciliation_inputs() -> DirectActiveSnowPartitionInputs {
        let mut layer = DirectSnowLayerState::new(0.18, 0.40, 450.0, 12.0);
        layer.temperature_c = -8.0;
        layer.cold_content_j_m2 = 0.18 * 1_000.0 * 2_100.0 * 8.0;
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
            coe_boundary_depth_m: 0.40,
            coe_boundary_density_kg_m3: 450.0,
            coe_boundary_settle_day_count: 12.0,
            snow_albedo_model: None,
            snow_albedo_state: None,
            snow_layers: vec![layer],
            underlying_surface_albedo: 0.2,
            hourly: [DirectSnowHourlyForcing {
                radiation_mj_m2: 0.0,
                air_temperature_c: -5.0,
                ..DirectSnowHourlyForcing::zero()
            }; 24],
        }
    }

    #[test]
    fn paired_arm_fingerprint_mismatch_fails_closed() {
        let tag = Stage3EvaluationTag::new(
            SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
        );
        let mut summary = Stage3ShadowSummary::new(tag);
        summary.source_fingerprint = 1;
        summary.forcing_fingerprint = 2;
        summary.geometry_fingerprint = 3;
        summary.non_formulation_fingerprint = 4;
        summary.surface_arm_non_formulation_fingerprint = 5;
        summary.complete_arm_non_formulation_fingerprint = 6;
        summary.evaluated_seconds = summary.requested_seconds;
        for hour in &mut summary.hourly {
            hour.requested_seconds = STAGE3_SECONDS_PER_HOUR;
            hour.evaluated_seconds = STAGE3_SECONDS_PER_HOUR;
        }

        let error = Wb11HydrologyKernel::validate_stage3_shadow_summary(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            &summary,
        )
        .expect_err("paired fingerprint mismatch must fail");
        assert!(
            error
                .to_string()
                .contains("stage3_evaluation_paired_fingerprint_equality")
        );
    }

    #[test]
    fn operator_tags_have_exact_distinct_cadence() {
        let paired = Stage3EvaluationTag::new(
            SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
        );
        let sequential = Stage3EvaluationTag::new(
            SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
        );
        assert_eq!(paired.cadence_id, "stage3_fixed_hourly_immutable_snapshot_v1");
        assert_eq!(
            sequential.cadence_id,
            "stage3_dynamic_substep_with_hourly_forcing_v1"
        );
    }

    #[test]
    fn reconciliation_validator_rejects_global_order_reason_and_projection_mutations() {
        let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let inputs = reconciliation_inputs();
        let cold = vec![inputs.snow_layers[0].cold_content_j_m2];
        let tag = Stage3EvaluationTag::new(
            SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
        );
        let valid = Wb11HydrologyKernel::evaluate_stage3_same_state_paired_carrier(
            phase,
            tag,
            &inputs,
            &inputs.snow_layers,
            &cold,
        )
        .expect("valid same-state reconciliation");
        Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &valid)
            .expect("unmodified reconciliation must pass");

        let mut reordered = valid.clone();
        reordered.reconciliation.tuples.swap(0, 1);
        assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &reordered).is_err());

        let mut bad_reason = valid.clone();
        bad_reason.reconciliation.hourly_status[0].reason = "invented";
        assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &bad_reason).is_err());

        let mut bad_projection = valid;
        bad_projection.reconciliation.tuples[0].projection_id = "aligned_active_dynamic";
        assert!(
            Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &bad_projection).is_err()
        );
    }

    #[test]
    fn sequential_reconciliation_mass_guard_uses_mass_scale_tolerance() {
        let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let inputs = reconciliation_inputs();
        let cold = vec![inputs.snow_layers[0].cold_content_j_m2];
        let tag = Stage3EvaluationTag::new(
            SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
        );
        let supports = inputs
            .hourly
            .iter()
            .copied()
            .map(|forcing| DirectSnowStage3SupportInput {
                forcing,
                duration_seconds: STAGE3_SECONDS_PER_HOUR,
            })
            .collect::<Vec<_>>();
        let mut summary = Wb11HydrologyKernel::evaluate_stage3_sequential_melt_shadow(
            phase,
            tag,
            &inputs,
            &supports,
            inputs.snow_layers.clone(),
            cold,
            None,
            0.0,
            None,
        )
        .expect("valid sequential reconciliation");
        Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &summary)
            .expect("unmodified sequential reconciliation must pass");
        summary.reconciliation.tuples[0].total_ice_mass_after_kg_m2 += 1.0e-8;
        assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &summary).is_err());
    }

    #[test]
    fn sequential_reconciliation_serializes_exact_transition_continuity() {
        let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let mut inputs = reconciliation_inputs();
        inputs.runtime_swe_m = 0.02;
        inputs.runtime_depth_m = 0.04;
        inputs.runtime_density_kg_m3 = 500.0;
        inputs.coe_boundary_depth_m = 0.04;
        inputs.coe_boundary_density_kg_m3 = 500.0;
        inputs.snow_layers[0].mass_swe_m = 0.02;
        inputs.snow_layers[0].thickness_m = 0.04;
        inputs.snow_layers[0].density_kg_m3 = 500.0;
        inputs.snow_layers[0].cold_content_j_m2 = 0.02 * 1_000.0 * 2_100.0 * 8.0;
        let cold = vec![inputs.snow_layers[0].cold_content_j_m2];
        let tag = Stage3EvaluationTag::new(
            SnowStage3EvaluationOperator::SequentialResolvedShadowV1,
        );
        let supports = inputs
            .hourly
            .iter()
            .copied()
            .map(|forcing| DirectSnowStage3SupportInput {
                forcing,
                duration_seconds: STAGE3_SECONDS_PER_HOUR,
            })
            .collect::<Vec<_>>();
        let mut summary = Wb11HydrologyKernel::evaluate_stage3_sequential_melt_shadow(
            phase,
            tag,
            &inputs,
            &supports,
            inputs.snow_layers.clone(),
            cold,
            None,
            0.0,
            None,
        )
        .expect("valid sequential reconciliation");
        assert!(summary.reconciliation.tuples.len() > 1);
        assert_eq!(summary.reconciliation.tuples[0].hour_index, 0);
        assert_eq!(summary.reconciliation.tuples[0].substep_index, 0);
        assert_eq!(summary.reconciliation.tuples[1].hour_index, 0);
        assert_eq!(summary.reconciliation.tuples[1].substep_index, 1);
        for pair in summary.reconciliation.tuples.windows(2) {
            let (previous, next) = (&pair[0], &pair[1]);
            assert_eq!(
                previous.total_layer_state_fingerprint_after_fnv1a64,
                next.total_layer_state_fingerprint_before_fnv1a64
            );
            assert_eq!(
                previous.total_cold_after_j_m2.to_bits(),
                next.total_cold_before_j_m2.to_bits()
            );
            assert_eq!(
                previous.total_ice_mass_after_kg_m2.to_bits(),
                next.total_ice_mass_before_kg_m2.to_bits()
            );
        }
        Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &summary)
            .expect("exactly continuous reconciliation must pass");

        summary.reconciliation.tuples[1].total_layer_state_fingerprint_before_fnv1a64 ^=
            1;
        assert!(Wb11HydrologyKernel::validate_stage3_reconciliation(phase, &summary).is_err());
    }

    #[test]
    fn same_state_suppresses_day_start_pack_at_resolved_mass_boundary() {
        let phase = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let mut inputs = reconciliation_inputs();
        inputs.runtime_swe_m = STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
        inputs.runtime_depth_m = 0.002;
        inputs.runtime_density_kg_m3 = 500.0;
        inputs.snow_layers[0].mass_swe_m = STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
        inputs.snow_layers[0].thickness_m = 0.002;
        inputs.snow_layers[0].density_kg_m3 = 500.0;
        inputs.snow_layers[0].cold_content_j_m2 = 0.0;
        let tag = Stage3EvaluationTag::new(
            SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
        );
        let summary = Wb11HydrologyKernel::evaluate_stage3_same_state_paired_carrier(
            phase,
            tag,
            &inputs,
            &inputs.snow_layers,
            &[0.0],
        )
        .expect("boundary pack is a valid non-evaluated state");
        assert!(summary.reconciliation.tuples.is_empty());
        assert!(summary.reconciliation.hourly_status.iter().all(|status| {
            !status.evaluated && status.reason == "no_resolved_snow_at_day_start"
        }));
    }

    #[test]
    fn surface_projection_crosses_density_boundary_and_uses_active_cold_content() {
        let layers = vec![
            DirectSnowLayerState {
                mass_swe_m: 0.02,
                thickness_m: 0.10,
                density_kg_m3: 200.0,
                settle_day_count: 1.0,
                temperature_c: -10.0,
                liquid_water_m: 0.0,
                cold_content_j_m2: 420_000.0,
                refrozen_liquid_m: 0.0,
            },
            DirectSnowLayerState {
                mass_swe_m: 0.14,
                thickness_m: 0.30,
                density_kg_m3: 466.666_666_666_666_7,
                settle_day_count: 2.0,
                temperature_c: -2.0,
                liquid_water_m: 0.0,
                cold_content_j_m2: 588_000.0,
                refrozen_liquid_m: 0.0,
            },
        ];
        let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(17, layers)
            .expect("multilayer persistent state");
        let surface = Wb11HydrologyKernel::project_stage3_surface_state_v1(&state)
            .expect("canonical active-volume surface");
        assert_eq!(surface.active_depth_m.to_bits(), 0.25_f64.to_bits());
        assert!((surface.active_mass_kg_m2 - 90.0).abs() <= 1.0e-12);
        assert!((surface.surface_temperature_k - 269.372_222_222_222_2).abs() <= 1.0e-12);
        assert_ne!(
            surface.surface_temperature_k.to_bits(),
            (state.layers[0].temperature_c + 273.15).to_bits()
        );
        assert!(matches!(surface.selected_substep_seconds, 1_800.0 | 900.0 | 60.0));
        assert_ne!(surface.active_lower_partition_sha256, openwepp_coupled_time::Digest32::zero());
        assert_ne!(surface.beginning_stage3_state_sha256, openwepp_coupled_time::Digest32::zero());
    }

    #[test]
    fn surface_projection_selects_parent_medium_and_small_cadence() {
        for (lane_id, mass_swe_m, expected_seconds) in
            [(1, 0.08, 1_800.0_f64), (2, 0.02, 900.0_f64), (3, 0.005, 60.0_f64)]
        {
            let temperature_c = -3.0;
            let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(
                lane_id,
                vec![DirectSnowLayerState {
                    mass_swe_m,
                    thickness_m: mass_swe_m * 2.0,
                    density_kg_m3: 500.0,
                    settle_day_count: 1.0,
                    temperature_c,
                    liquid_water_m: 0.0,
                    cold_content_j_m2: mass_swe_m
                        * STAGE3_RHO_WATER_KG_M3
                        * STAGE3_SPECIFIC_HEAT_ICE_J_KG_K
                        * -temperature_c,
                    refrozen_liquid_m: 0.0,
                }],
            )
            .expect("cadence persistent state");
            let surface = Wb11HydrologyKernel::project_stage3_surface_state_v1(&state)
                .expect("cadence surface projection");
            assert_eq!(
                surface.selected_substep_seconds.to_bits(),
                expected_seconds.to_bits()
            );
        }
    }

    #[path = "persistent_tests.rs"]
    mod persistent_tests;
}
