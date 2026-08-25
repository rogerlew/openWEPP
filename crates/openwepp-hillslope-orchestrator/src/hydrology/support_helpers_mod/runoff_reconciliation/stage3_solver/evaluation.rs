use super::terminal_event::{TerminalFluxIntegral, TerminalState};
#[allow(clippy::wildcard_imports)]
use super::*;

#[allow(clippy::cast_precision_loss)]
fn support_duration_seconds(duration_ns: u128) -> f64 {
    duration_ns as f64 / 1_000_000_000.0
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn seconds_to_exact_ns(
    phase_class: HillslopeKernelPhaseClass,
    seconds: f64,
) -> Result<u128, DirectSnowStage3EvaluationError> {
    let nanos = seconds * 1_000_000_000.0;
    if !nanos.is_finite() || nanos < 0.0 || nanos.fract() != 0.0 {
        return Err(Wb11HydrologyKernel::stage3_domain_error(
            phase_class,
            "snow.terminal_trial_nanoseconds",
            seconds,
            Some(0.0),
            None,
        )
        .into());
    }
    Ok(nanos as u128)
}

impl Wb11HydrologyKernel {
    fn boundary_reconciliation(
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        surface_temperature_c: f64,
        boundary: Stage3SnowSurfaceBoundaryReceiptV1,
    ) -> Stage3CarrierReconciliation {
        let support_seconds = support_duration_seconds(boundary.support.duration_ns());
        let sensible_flux_w_m2 = boundary.sensible_energy_j_m2 / support_seconds;
        let latent_flux_w_m2 = boundary.latent_energy_j_m2 / support_seconds;
        let shortwave_flux_w_m2 = boundary.shortwave_energy_j_m2 / support_seconds;
        let net_longwave_w_m2 = boundary.net_longwave_energy_j_m2 / support_seconds;
        let precipitation_advected_flux_w_m2 =
            boundary.precipitation_advection_j_m2 / support_seconds;
        let snow_soil_heat_flux_w_m2 = boundary.snow_soil_heat_j_m2 / support_seconds;
        Stage3CarrierReconciliation {
            air_temperature_c: hourly.air_temperature_c,
            dewpoint_c: inputs.dewpoint_c,
            wind_speed_m_s: inputs.wind_m_s,
            air_pressure_pa: inputs.surface_energy_options.atmospheric_pressure_pa,
            hourly_radiation_mj_m2: shortwave_flux_w_m2 * STAGE3_SECONDS_PER_HOUR / 1_000_000.0,
            daily_solar_radiation_mj_m2: 0.0,
            daily_extraterrestrial_radiation_mj_m2: 0.0,
            daylight: false,
            canopy_cover_fraction: inputs.canopy_cover_fraction,
            rain_m: hourly.rain_m,
            snowfall_geometric_m: hourly.snowfall_m,
            rain_mass_flux_kg_m2_s: hourly.rain_m * STAGE3_RHO_WATER_KG_M3
                / STAGE3_SECONDS_PER_HOUR,
            snow_mass_flux_kg_m2_s: hourly.snowfall_m * 0.1 * STAGE3_RHO_WATER_KG_M3
                / STAGE3_SECONDS_PER_HOUR,
            rain_temperature_c: surface_temperature_c,
            snow_temperature_c: surface_temperature_c,
            rain_specific_heat_j_kg_k: 0.0,
            snow_specific_heat_j_kg_k: 0.0,
            incoming_shortwave_w_m2: shortwave_flux_w_m2,
            snow_albedo_fraction: 0.0,
            snow_albedo_source_id: "stage3_covered_boundary_v1",
            snow_albedo_model_id: None,
            snow_albedo_accumulated_positive_temperature_c_day: None,
            net_shortwave_w_m2: shortwave_flux_w_m2,
            actual_vapor_pressure_pa: 0.0,
            longwave_cloud_fraction: 0.0,
            sky_view_fraction: 0.0,
            atmospheric_longwave_w_m2: 0.0,
            canopy_longwave_w_m2: 0.0,
            subcanopy_longwave_w_m2: net_longwave_w_m2,
            outgoing_longwave_w_m2: 0.0,
            net_longwave_w_m2,
            longwave_model_id: "dilley_unsworth_subcanopy_v1",
            sublimation_model_id: "disabled",
            air_temperature_height_m: inputs
                .surface_energy_options
                .turbulent_geometry
                .air_temperature_height_m,
            vapor_pressure_height_m: inputs
                .surface_energy_options
                .turbulent_geometry
                .vapor_pressure_height_m,
            wind_speed_height_m: inputs
                .surface_energy_options
                .turbulent_geometry
                .wind_speed_height_m,
            aerodynamic_roughness_length_m: inputs
                .surface_energy_options
                .turbulent_geometry
                .aerodynamic_roughness_length_m,
            turbulent_options: TurbulentTransferOptions::default(),
            surface_vapor_pressure_pa: 0.0,
            surface_latent_heat_j_kg: Some(boundary.latent_heat_j_kg),
            turbulent: None,
            vapor_mass_flux_kg_m2_s: boundary.vapor_mass_kg_m2 / support_seconds,
            sensible_flux_w_m2,
            latent_flux_w_m2,
            precipitation_advected_flux_w_m2,
            snow_soil_heat_flux_w_m2,
            complete_external_flux_w_m2: shortwave_flux_w_m2
                + sensible_flux_w_m2
                + latent_flux_w_m2
                + net_longwave_w_m2
                + precipitation_advected_flux_w_m2
                + snow_soil_heat_flux_w_m2,
        }
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::too_many_lines,
        clippy::too_many_arguments
    )]
    pub(super) fn evaluate_stage3_sequential_melt_shadow(
        phase_class: HillslopeKernelPhaseClass,
        tag: Stage3EvaluationTag,
        inputs: &DirectActiveSnowPartitionInputs,
        supports: &[DirectSnowStage3SupportInput],
        mut layers: Vec<DirectSnowLayerState>,
        mut cold_content_by_layer: Vec<f64>,
        terminal_request: Option<DirectSnowTerminalEventRequest>,
        mut terminal_detached_liquid_kg_m2: f64,
        boundary: Option<Stage3SnowSurfaceBoundaryReceiptV1>,
        mut terminal_trial_context: Option<(
            u32,
            TimeSupport,
            CoveredTerminalJointTrialStateV1,
            &mut CoveredTerminalTrialProviderV1<'_>,
        )>,
    ) -> Result<Stage3ShadowSummary, DirectSnowStage3EvaluationError> {
        if supports.is_empty() || supports.len() > 24 {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_support_cardinality",
                supports.len() as f64,
                Some(1.0),
                Some(24.0),
            )
            .into());
        }
        for support in supports {
            if !support.duration_seconds.is_finite() || support.duration_seconds <= 0.0 {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_support_duration_seconds",
                    support.duration_seconds,
                    Some(f64::MIN_POSITIVE),
                    None,
                )
                .into());
            }
        }
        let mut summary = Stage3ShadowSummary::new(tag);
        summary.requested_seconds = supports
            .iter()
            .map(|support| support.duration_seconds)
            .sum::<f64>();
        Self::stage3_shadow_fingerprints(inputs, &layers, &cold_content_by_layer, &mut summary);
        summary.complete_arm_non_formulation_fingerprint = summary.non_formulation_fingerprint;
        for (hour, support) in summary.hourly.iter_mut().zip(supports) {
            hour.requested_seconds = support.duration_seconds;
        }
        let resolved_at_day_start =
            Self::stage3_total_ice_mass_swe_m(&layers) > STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
        let mut prepared_active_layer_count = None;
        'hours: for (hour_index, support) in supports.iter().copied().enumerate() {
            let hourly = support.forcing;
            let support_seconds = support.duration_seconds;
            if tag.operator == SnowStage3EvaluationOperator::PersistentAccumulationShadowV1
                && hourly.snowfall_m > 0.0
            {
                let mass_swe_m = hourly.snowfall_m * 0.1;
                let density_kg_m3 = inputs.newsnw_kg_m3;
                // The existing carrier applies snowfall advection once during
                // this hour. Insert at the 0 C reference state so subfreezing
                // hydrometeor enthalpy is not also preloaded as cold content.
                let temperature_c = 0.0;
                let cold_content_j_m2 = 0.0;
                layers.insert(
                    0,
                    DirectSnowLayerState {
                        mass_swe_m,
                        thickness_m: mass_swe_m * STAGE3_RHO_WATER_KG_M3 / density_kg_m3,
                        density_kg_m3,
                        settle_day_count: 0.0,
                        temperature_c,
                        liquid_water_m: 0.0,
                        cold_content_j_m2,
                        refrozen_liquid_m: 0.0,
                    },
                );
                cold_content_by_layer.insert(0, cold_content_j_m2);
                prepared_active_layer_count = None;
            }
            let mut elapsed_seconds = 0.0;
            let mut substep_index = 0;
            while elapsed_seconds < support_seconds && !layers.is_empty() {
                if Self::stage3_total_ice_mass_swe_m(&layers)
                    <= STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M
                {
                    if terminal_request.is_some()
                        && tag.operator
                            == SnowStage3EvaluationOperator::PersistentAccumulationShadowV1
                    {
                        let start_ice_kg_m2 =
                            Self::stage3_total_ice_mass_swe_m(&layers) * STAGE3_RHO_WATER_KG_M3;
                        let start_liquid_kg_m2 = layers
                            .iter()
                            .map(|layer| layer.liquid_water_m * STAGE3_RHO_WATER_KG_M3)
                            .sum::<f64>()
                            + terminal_detached_liquid_kg_m2;
                        terminal_detached_liquid_kg_m2 = 0.0;
                        let start_cold_content_j_m2 = cold_content_by_layer.iter().sum::<f64>();
                        let depth_m = layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
                        let density_kg_m3 = if depth_m > 0.0 {
                            start_ice_kg_m2 / depth_m
                        } else {
                            inputs.newsnw_kg_m3
                        };
                        let remaining_seconds = support_seconds - elapsed_seconds;
                        let initial_joint = terminal_trial_context
                            .as_ref()
                            .map(|(_, _, joint, _)| joint.clone());
                        let terminal_lane_id = terminal_trial_context
                            .as_ref()
                            .map(|(lane_id, _, _, _)| *lane_id);
                        let mut terminal = Self::solve_terminal_enthalpy_event(
                            phase_class,
                            hour_index,
                            elapsed_seconds,
                            remaining_seconds,
                            TerminalState {
                                ice_kg_m2: start_ice_kg_m2,
                                liquid_kg_m2: start_liquid_kg_m2,
                                cold_content_j_m2: start_cold_content_j_m2,
                            },
                            initial_joint,
                            |trial_state,
                             beginning_joint,
                             relative_start_seconds,
                             duration_seconds,
                             role,
                             attempt_ordinal| {
                                let surface_temperature_c =
                                    Self::stage3_temperature_from_cold_content_values(
                                        trial_state.ice_kg_m2 / STAGE3_RHO_WATER_KG_M3,
                                        trial_state.cold_content_j_m2,
                                    );
                                let trial_depth_m = if density_kg_m3 > 0.0 {
                                    trial_state.ice_kg_m2 / density_kg_m3
                                } else {
                                    0.0
                                };
                                let trial_transition = if let Some((lane_id, base_support, _, provider)) =
                                    terminal_trial_context.as_mut()
                                {
                                    let start_offset_ns = seconds_to_exact_ns(
                                        phase_class,
                                        elapsed_seconds + relative_start_seconds,
                                    )?;
                                    let duration_ns =
                                        seconds_to_exact_ns(phase_class, duration_seconds)?;
                                    let trial_start = ModelTimeNs::new(
                                        base_support
                                            .start_ns()
                                            .get()
                                            .checked_add(start_offset_ns)
                                            .ok_or_else(|| {
                                                Wb11HydrologyKernel::stage3_domain_error(
                                                    phase_class,
                                                    "snow.terminal_trial_start_overflow",
                                                    relative_start_seconds,
                                                    Some(0.0),
                                                    None,
                                                )
                                            })?,
                                    );
                                    let trial_end = ModelTimeNs::new(
                                        trial_start.get().checked_add(duration_ns).ok_or_else(
                                            || {
                                                Wb11HydrologyKernel::stage3_domain_error(
                                                    phase_class,
                                                    "snow.terminal_trial_end_overflow",
                                                    duration_seconds,
                                                    Some(0.0),
                                                    None,
                                                )
                                            },
                                        )?,
                                    );
                                    if trial_end > base_support.end_ns() {
                                        return Err(Wb11HydrologyKernel::stage3_domain_error(
                                            phase_class,
                                            "snow.terminal_trial_outside_support",
                                            duration_seconds,
                                            Some(0.0),
                                            Some(remaining_seconds),
                                        )
                                        .into());
                                    }
                                    let trial_support = TimeSupport::new(trial_start, trial_end)
                                        .map_err(|_| {
                                            Wb11HydrologyKernel::stage3_domain_error(
                                                phase_class,
                                                "snow.terminal_trial_support",
                                                duration_seconds,
                                                Some(f64::MIN_POSITIVE),
                                                None,
                                            )
                                        })?;
                                    let beginning_joint = beginning_joint.as_ref().ok_or_else(|| {
                                        Wb11HydrologyKernel::stage3_domain_error(
                                            phase_class,
                                            "snow.terminal_trial_missing_joint_state",
                                            1.0,
                                            Some(0.0),
                                            Some(0.0),
                                        )
                                    })?;
                                    let receipt = (**provider)(CoveredTerminalTrialRequestV1 {
                                        lane_id: *lane_id,
                                        support: trial_support,
                                        role,
                                        attempt_ordinal,
                                        ice_kg_m2: trial_state.ice_kg_m2,
                                        liquid_kg_m2: trial_state.liquid_kg_m2,
                                        cold_content_j_m2: trial_state.cold_content_j_m2,
                                        surface_temperature_c,
                                        snow_depth_m: trial_depth_m,
                                        snow_density_kg_m3: density_kg_m3,
                                        beginning_joint: beginning_joint.clone(),
                                    })?;
                                    if receipt.boundary.support != trial_support
                                        || receipt.beginning_joint != *beginning_joint
                                        || receipt.probe_child_identity.trial_support
                                            != trial_support
                                        || receipt.probe_child_identity.role != role
                                        || receipt.probe_child_identity.attempt_ordinal
                                            != attempt_ordinal
                                        || receipt.probe_child_identity.beginning_joint_sha256
                                            != beginning_joint.receipt_sha256()
                                    {
                                        return Err(Wb11HydrologyKernel::stage3_domain_error(
                                            phase_class,
                                            "snow.terminal_trial_boundary_support_join",
                                            duration_seconds,
                                            Some(duration_seconds),
                                            Some(duration_seconds),
                                        )
                                        .into());
                                    }
                                    Some(receipt)
                                } else {
                                    None
                                };
                                let trial_boundary = trial_transition
                                    .as_ref()
                                    .map(|transition| transition.boundary.clone());
                                let carrier = Self::stage3_hourly_surface_energy(
                                    phase_class,
                                    inputs,
                                    hourly,
                                    Stage3SurfaceInterval {
                                        surface_temperature_c,
                                        snow_depth_m: trial_depth_m,
                                        snow_density_kg_m3: density_kg_m3,
                                        duration_seconds,
                                        forcing_duration_seconds: duration_seconds,
                                        boundary: trial_boundary,
                                    },
                                    Some(tag.operator),
                                    DirectSnowDiagnosticCapture::Verbose,
                                )?;
                                let surface = carrier.diagnostics.ok_or_else(|| {
                                    Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                                        phase_class,
                                        symbol: BoundarySymbol::from(
                                            "snow.terminal_enthalpy_diagnostics",
                                        ),
                                    }
                                })?;
                                let flux = TerminalFluxIntegral {
                                    complete_energy_j_m2: surface.shadow_complete_energy_j_m2,
                                    vapor_mass_exchange_kg_m2: surface
                                        .shadow_vapor_mass_exchange_kg_m2,
                                    shortwave_energy_j_m2: carrier.shortwave_j_m2,
                                    longwave_energy_j_m2: carrier.longwave_j_m2,
                                    sensible_energy_j_m2: surface.shadow_sensible_flux_w_m2
                                        * duration_seconds,
                                    latent_energy_j_m2: surface.shadow_latent_flux_w_m2
                                        * duration_seconds,
                                    advected_energy_j_m2: surface.shadow_advected_flux_w_m2
                                        * duration_seconds,
                                    snow_soil_heat_energy_j_m2: carrier
                                        .reconciliation
                                        .as_ref()
                                        .map_or(0.0, |value| {
                                            value.snow_soil_heat_flux_w_m2 * duration_seconds
                                        }),
                                    external_liquid_kg_m2: hourly.rain_m
                                        * STAGE3_RHO_WATER_KG_M3
                                        * duration_seconds
                                        / support_seconds,
                                };
                                let ending_joint = trial_transition.map_or_else(
                                    || beginning_joint.clone(),
                                    |value| Some(value.ending_joint),
                                );
                                Ok((flux, ending_joint))
                            },
                            |state, joint| {
                                joint
                                    .map(|value| {
                                        let lane_id = terminal_lane_id.ok_or(
                                            DirectSnowStage3EvaluationError::TerminalCustody(
                                                "terminal joint lane identity",
                                            ),
                                        )?;
                                        value.with_terminal_hydrology_state(
                                            lane_id,
                                            state.ice_kg_m2,
                                            state.liquid_kg_m2,
                                            state.cold_content_j_m2,
                                        )
                                    })
                                    .transpose()
                            },
                        )?;
                        terminal.entry_solid_precipitation_kg_m2 =
                            hourly.snowfall_m * 0.1 * STAGE3_RHO_WATER_KG_M3;
                        let hour = &mut summary.hourly[hour_index];
                        hour.complete_energy_j_m2 += terminal.complete_energy_j_m2;
                        hour.shortwave_energy_j_m2 += terminal.shortwave_energy_j_m2;
                        hour.longwave_energy_j_m2 += terminal.longwave_energy_j_m2;
                        hour.sensible_flux_w_m2 += terminal.sensible_energy_j_m2 / support_seconds;
                        hour.latent_flux_w_m2 += terminal.latent_energy_j_m2 / support_seconds;
                        hour.advected_flux_w_m2 += terminal.advected_energy_j_m2 / support_seconds;
                        hour.cold_required_j_m2 += terminal.start_cold_content_j_m2;
                        hour.cold_energy_change_j_m2 += terminal.cold_energy_change_j_m2;
                        hour.excess_energy_j_m2 += (terminal.complete_energy_j_m2
                            - terminal.cold_energy_change_j_m2)
                            .max(0.0);
                        hour.sublimation_kg_m2 += terminal.sublimation_kg_m2;
                        hour.melt_kg_m2 += terminal.melt_kg_m2;
                        hour.unallocated_after_exhaustion_j_m2 +=
                            terminal.terminal_unallocated_energy_j_m2;
                        hour.energy_closure_residual_j_m2 += terminal.energy_closure_residual_j_m2;
                        hour.complete_carrier_evaluated = terminal.evaluated_seconds > 0.0;
                        hour.evaluated_seconds += terminal.evaluated_seconds;
                        summary.evaluated_seconds += terminal.evaluated_seconds;
                        summary.complete_energy_j_m2 += terminal.complete_energy_j_m2;
                        summary.complete_shortwave_j_m2 += terminal.shortwave_energy_j_m2;
                        summary.complete_longwave_j_m2 += terminal.longwave_energy_j_m2;
                        summary.complete_sensible_j_m2 += terminal.sensible_energy_j_m2;
                        summary.complete_latent_j_m2 += terminal.latent_energy_j_m2;
                        summary.complete_advected_j_m2 += terminal.advected_energy_j_m2;
                        summary.complete_vapor_mass_exchange_kg_m2 +=
                            terminal.deposition_kg_m2 - terminal.sublimation_kg_m2;
                        summary.terminal_refrozen_kg_m2 += terminal.refrozen_kg_m2;
                        summary.terminal_deposition_kg_m2 += terminal.deposition_kg_m2;
                        summary.cold_energy_change_j_m2 += terminal.cold_energy_change_j_m2
                            - STAGE3_LATENT_HEAT_FUSION_J_KG * terminal.refrozen_kg_m2;
                        summary.sublimation_kg_m2 += terminal.sublimation_kg_m2;
                        summary.melt_kg_m2 += terminal.melt_kg_m2;
                        summary.unallocated_after_exhaustion_j_m2 +=
                            terminal.terminal_unallocated_energy_j_m2;
                        summary.maximum_energy_closure_residual_j_m2 = summary
                            .maximum_energy_closure_residual_j_m2
                            .max(terminal.energy_closure_residual_j_m2.abs());
                        summary.reconciliation.hourly_status[hour_index] =
                            DirectSnowStage3ReconciliationHourStatus {
                                evaluated: terminal.evaluated_seconds > 0.0,
                                reason: "terminal_enthalpy_event_v1",
                            };
                        summary.terminal_event = Some(terminal);
                        summary.terminal_intervals.push(terminal);
                        if terminal.event_occurred {
                            for status in summary
                                .reconciliation
                                .hourly_status
                                .iter_mut()
                                .skip(hour_index + 1)
                            {
                                *status = DirectSnowStage3ReconciliationHourStatus {
                                    evaluated: false,
                                    reason: "post_terminal_event_censored",
                                };
                            }
                            layers.clear();
                            cold_content_by_layer.clear();
                            break 'hours;
                        }
                        if let Some(layer) = layers.first().copied() {
                            layers.clear();
                            cold_content_by_layer.clear();
                            layers.push(DirectSnowLayerState {
                                mass_swe_m: terminal.end_ice_kg_m2 / STAGE3_RHO_WATER_KG_M3,
                                thickness_m: terminal.end_ice_kg_m2 / density_kg_m3,
                                density_kg_m3,
                                settle_day_count: layer.settle_day_count,
                                temperature_c: Self::stage3_temperature_from_cold_content_values(
                                    terminal.end_ice_kg_m2 / STAGE3_RHO_WATER_KG_M3,
                                    terminal.end_cold_content_j_m2,
                                ),
                                liquid_water_m: terminal.terminal_liquid_kg_m2
                                    / STAGE3_RHO_WATER_KG_M3,
                                cold_content_j_m2: terminal.end_cold_content_j_m2,
                                refrozen_liquid_m: terminal.refrozen_kg_m2 / STAGE3_RHO_WATER_KG_M3,
                            });
                            cold_content_by_layer.push(terminal.end_cold_content_j_m2);
                        }
                        elapsed_seconds += terminal.evaluated_seconds;
                        if elapsed_seconds >= support_seconds {
                            break;
                        }
                    }
                    break;
                }
                let active_layer_count = prepared_active_layer_count.take().unwrap_or_else(|| {
                    Self::prepare_stage3_sequential_control_volume(
                        &mut layers,
                        &mut cold_content_by_layer,
                    )
                });
                let substep_seconds = Self::stage3_substep_seconds(&layers, active_layer_count)
                    .min(support_seconds - elapsed_seconds);
                let before_reconciliation = Self::stage3_reconciliation_state(
                    &layers,
                    &cold_content_by_layer,
                    active_layer_count,
                );
                let lower_cold_before_j_m2 = cold_content_by_layer[active_layer_count..]
                    .iter()
                    .sum::<f64>();
                let active_state = Self::stage3_control_volume_state(
                    phase_class,
                    &layers[..active_layer_count],
                    &cold_content_by_layer[..active_layer_count],
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?;
                let surface_temperature_c = Self::stage3_temperature_from_cold_content_values(
                    active_state.mass_swe_m,
                    active_state.cold_content_j_m2,
                );
                let carrier = Self::stage3_hourly_surface_energy(
                    phase_class,
                    inputs,
                    hourly,
                    Stage3SurfaceInterval {
                        surface_temperature_c,
                        snow_depth_m: active_state.depth_m,
                        snow_density_kg_m3: active_state.density_kg_m3,
                        duration_seconds: substep_seconds,
                        forcing_duration_seconds: support_seconds,
                        boundary,
                    },
                    Some(tag.operator),
                    DirectSnowDiagnosticCapture::Verbose,
                )?;
                let surface = carrier.diagnostics.ok_or_else(|| {
                    Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: BoundarySymbol::from("snow.stage3_shadow_diagnostics"),
                    }
                })?;
                let carrier_reconciliation = carrier.reconciliation.ok_or_else(|| {
                    Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: BoundarySymbol::from("snow.stage3_operator_reconciliation_carrier"),
                    }
                })?;
                let conduction = Self::apply_stage3_active_lower_conduction(
                    phase_class,
                    &layers,
                    &mut cold_content_by_layer,
                    active_layer_count,
                    substep_seconds,
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?;
                let cold_required_j_m2 = active_state.cold_content_j_m2;
                let shadow_surface_energy_j_m2 = surface.shadow_complete_energy_j_m2;
                let q_complete_j_m2 = shadow_surface_energy_j_m2 + conduction.active_energy;
                Self::apply_stage3_control_volume_energy(
                    shadow_surface_energy_j_m2,
                    &layers,
                    &mut cold_content_by_layer,
                    0,
                    active_layer_count,
                );
                let cold_after_surface_energy_j_m2 = cold_content_by_layer[..active_layer_count]
                    .iter()
                    .sum::<f64>();
                let cold_energy_change_before_refreeze_j_m2 =
                    cold_required_j_m2 - cold_after_surface_energy_j_m2;
                let lower_cold_after_j_m2 = cold_content_by_layer[active_layer_count..]
                    .iter()
                    .sum::<f64>();
                let lower_cold_energy_change_j_m2 = lower_cold_before_j_m2 - lower_cold_after_j_m2;
                let excess_energy_j_m2 =
                    (q_complete_j_m2 - cold_energy_change_before_refreeze_j_m2).max(0.0);
                let active_ice_kg_m2 = layers[..active_layer_count]
                    .iter()
                    .map(|layer| layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3)
                    .sum::<f64>();
                let sublimation_kg_m2 = (-surface.shadow_vapor_mass_exchange_kg_m2)
                    .max(0.0)
                    .min(active_ice_kg_m2);
                let ice_available_kg_m2 = (active_ice_kg_m2 - sublimation_kg_m2).max(0.0);
                let melt_kg_m2 =
                    (excess_energy_j_m2 / STAGE3_LATENT_HEAT_FUSION_J_KG).min(ice_available_kg_m2);
                let unallocated_j_m2 =
                    (excess_energy_j_m2 - STAGE3_LATENT_HEAT_FUSION_J_KG * melt_kg_m2).max(0.0);
                let mut removal_active_count = active_layer_count;
                let mut cold_content_export_j_m2 = 0.0;
                if melt_kg_m2 > 0.0 {
                    let (_, exported_j_m2, _) = Self::remove_stage3_active_sublimation(
                        melt_kg_m2 / STAGE3_RHO_WATER_KG_M3,
                        &mut layers,
                        &mut cold_content_by_layer,
                        &mut removal_active_count,
                    );
                    cold_content_export_j_m2 += exported_j_m2;
                }
                let external_rain_kg_m2 =
                    hourly.rain_m * STAGE3_RHO_WATER_KG_M3 * substep_seconds / support_seconds;
                let (routed_liquid_kg_m2, retained_liquid_kg_m2, refrozen_kg_m2) =
                    Self::route_stage3_persistent_liquid_through_layers(
                        phase_class,
                        external_rain_kg_m2 + melt_kg_m2,
                        &mut layers,
                        &mut cold_content_by_layer,
                    )?;
                let liquid_closure_residual_kg_m2 = external_rain_kg_m2 + melt_kg_m2
                    - refrozen_kg_m2
                    - retained_liquid_kg_m2
                    - routed_liquid_kg_m2;
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from("snow.stage3_persistent_liquid_residual_kg_m2"),
                    liquid_closure_residual_kg_m2.abs(),
                    Some(0.0),
                    Some(1.0e-12),
                )?;
                let refreeze_energy_j_m2 = STAGE3_LATENT_HEAT_FUSION_J_KG * refrozen_kg_m2;
                let actual_cold_energy_change_j_m2 =
                    cold_energy_change_before_refreeze_j_m2 + refreeze_energy_j_m2;
                let cold_energy_change_j_m2 = cold_energy_change_before_refreeze_j_m2;
                let closure_residual_j_m2 = q_complete_j_m2 + refreeze_energy_j_m2
                    - actual_cold_energy_change_j_m2
                    - STAGE3_LATENT_HEAT_FUSION_J_KG * melt_kg_m2
                    - unallocated_j_m2;
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from("snow.stage3_shadow_energy_residual_j_m2"),
                    closure_residual_j_m2.abs(),
                    None,
                    Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
                )?;
                if sublimation_kg_m2 > 0.0 && !layers.is_empty() {
                    removal_active_count = removal_active_count.min(layers.len());
                    let (_, exported_j_m2, _) = Self::remove_stage3_active_sublimation(
                        sublimation_kg_m2 / STAGE3_RHO_WATER_KG_M3,
                        &mut layers,
                        &mut cold_content_by_layer,
                        &mut removal_active_count,
                    );
                    cold_content_export_j_m2 += exported_j_m2;
                }
                let deposition_kg_m2 = surface.shadow_vapor_mass_exchange_kg_m2.max(0.0);
                if deposition_kg_m2 > 0.0 && !layers.is_empty() {
                    let deposition_swe_m = deposition_kg_m2 / STAGE3_RHO_WATER_KG_M3;
                    layers[0].mass_swe_m += deposition_swe_m;
                    layers[0].thickness_m =
                        layers[0].mass_swe_m * STAGE3_RHO_WATER_KG_M3 / layers[0].density_kg_m3;
                }

                let resolved_before_preparation = !layers.is_empty()
                    && Self::stage3_total_ice_mass_swe_m(&layers)
                        > STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
                let (after_active_layer_count, resolved_after) = if resolved_before_preparation {
                    let prepared = Self::prepare_stage3_sequential_control_volume(
                        &mut layers,
                        &mut cold_content_by_layer,
                    );
                    let still_resolved = Self::stage3_total_ice_mass_swe_m(&layers)
                        > STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
                    prepared_active_layer_count = still_resolved.then_some(prepared);
                    (prepared, still_resolved)
                } else if layers.is_empty() {
                    (0, false)
                } else {
                    (removal_active_count.clamp(1, layers.len()), false)
                };
                let after_reconciliation = Self::stage3_reconciliation_state(
                    &layers,
                    &cold_content_by_layer,
                    after_active_layer_count,
                );
                let after_surface_applicable = resolved_after;
                let tuple = Self::stage3_reconciliation_tuple(
                    &summary,
                    hour_index,
                    substep_index,
                    elapsed_seconds,
                    substep_seconds,
                    support_seconds,
                    "aligned_active_dynamic",
                    before_reconciliation,
                    after_reconciliation,
                    after_surface_applicable,
                    &carrier_reconciliation,
                    Stage3ReconciliationTransfer {
                        lower_cold_before_conduction_j_m2: Some(lower_cold_before_j_m2),
                        lower_cold_after_conduction_j_m2: Some(lower_cold_after_j_m2),
                        active_cold_energy_change_j_m2: Some(actual_cold_energy_change_j_m2),
                        lower_cold_energy_change_j_m2: Some(lower_cold_energy_change_j_m2),
                        cold_content_export_j_m2: Some(cold_content_export_j_m2),
                        internal_active_lower_conduction_j_m2: Some(conduction.active_energy),
                        melt_kg_m2: Some(melt_kg_m2),
                        refrozen_kg_m2: Some(refrozen_kg_m2),
                        sublimation_kg_m2: Some(sublimation_kg_m2),
                        deposition_kg_m2: Some(deposition_kg_m2),
                        legacy_sequential_complete_j_m2: Some(q_complete_j_m2),
                        energy_closure_residual_j_m2: Some(closure_residual_j_m2),
                    },
                );
                summary.reconciliation.tuples.push(tuple);
                summary.reconciliation.hourly_status[hour_index] =
                    DirectSnowStage3ReconciliationHourStatus {
                        evaluated: true,
                        reason: "evaluated",
                    };

                let hour = &mut summary.hourly[hour_index];
                let weight = substep_seconds / support_seconds;
                hour.sensible_flux_w_m2 += surface.shadow_sensible_flux_w_m2 * weight;
                hour.latent_flux_w_m2 += surface.shadow_latent_flux_w_m2 * weight;
                hour.advected_flux_w_m2 += surface.shadow_advected_flux_w_m2 * weight;
                hour.shortwave_energy_j_m2 += carrier.shortwave_j_m2;
                hour.longwave_energy_j_m2 += carrier.longwave_j_m2;
                hour.internal_active_lower_conduction_j_m2 += conduction.active_energy;
                hour.complete_energy_j_m2 += q_complete_j_m2;
                hour.vapor_mass_exchange_kg_m2 += surface.shadow_vapor_mass_exchange_kg_m2;
                hour.cold_required_j_m2 += cold_required_j_m2;
                hour.cold_energy_change_j_m2 += cold_energy_change_j_m2;
                hour.cold_content_export_j_m2 += cold_content_export_j_m2;
                hour.excess_energy_j_m2 += excess_energy_j_m2;
                hour.ice_available_kg_m2 = hour.ice_available_kg_m2.max(ice_available_kg_m2);
                hour.sublimation_kg_m2 += sublimation_kg_m2;
                hour.melt_kg_m2 += melt_kg_m2;
                hour.unallocated_after_exhaustion_j_m2 += unallocated_j_m2;
                hour.energy_closure_residual_j_m2 += closure_residual_j_m2;
                hour.complete_carrier_evaluated = true;
                hour.evaluated_seconds += substep_seconds;
                summary.evaluated_seconds += substep_seconds;
                summary.complete_shortwave_j_m2 += carrier.shortwave_j_m2;
                summary.complete_longwave_j_m2 += carrier.longwave_j_m2;
                summary.complete_sensible_j_m2 +=
                    surface.shadow_sensible_flux_w_m2 * substep_seconds;
                summary.complete_latent_j_m2 += surface.shadow_latent_flux_w_m2 * substep_seconds;
                summary.complete_advected_j_m2 +=
                    surface.shadow_advected_flux_w_m2 * substep_seconds;
                summary.complete_snow_soil_heat_j_m2 +=
                    carrier_reconciliation.snow_soil_heat_flux_w_m2 * substep_seconds;
                summary.internal_active_lower_conduction_j_m2 += conduction.active_energy;
                summary.complete_vapor_mass_exchange_kg_m2 +=
                    surface.shadow_vapor_mass_exchange_kg_m2;
                summary.cold_content_export_j_m2 += cold_content_export_j_m2;
                summary.available_ice_kg_m2 = summary.available_ice_kg_m2.max(ice_available_kg_m2);
                summary.complete_energy_j_m2 += q_complete_j_m2;
                summary.cold_energy_change_j_m2 += cold_energy_change_j_m2;
                summary.excess_energy_j_m2 += excess_energy_j_m2;
                summary.sublimation_kg_m2 += sublimation_kg_m2;
                summary.melt_kg_m2 += melt_kg_m2;
                summary.persistent_refrozen_kg_m2 += refrozen_kg_m2;
                summary.unallocated_after_exhaustion_j_m2 += unallocated_j_m2;
                summary.maximum_energy_closure_residual_j_m2 = summary
                    .maximum_energy_closure_residual_j_m2
                    .max(closure_residual_j_m2.abs());
                elapsed_seconds += substep_seconds;
                substep_index += 1;
            }
            if substep_index == 0 {
                summary.reconciliation.hourly_status[hour_index] =
                    DirectSnowStage3ReconciliationHourStatus {
                        evaluated: false,
                        reason: if resolved_at_day_start {
                            "thin_pack_boundary_reached"
                        } else {
                            "no_resolved_snow_at_day_start"
                        },
                    };
            }
        }
        for (layer, cold_content_j_m2) in
            layers.iter_mut().zip(cold_content_by_layer.iter().copied())
        {
            layer.cold_content_j_m2 = cold_content_j_m2;
            layer.temperature_c = Self::stage3_temperature_from_cold_content_values(
                layer.mass_swe_m,
                cold_content_j_m2,
            );
        }
        summary.final_layers = layers;
        Ok(summary)
    }

    pub(super) fn route_stage3_persistent_liquid_through_layers(
        phase_class: HillslopeKernelPhaseClass,
        incoming_liquid_kg_m2: f64,
        layers: &mut [DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
    ) -> Result<(f64, f64, f64), DirectSnowStage3EvaluationError> {
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_persistent_incoming_liquid_kg_m2"),
            incoming_liquid_kg_m2,
            Some(0.0),
            None,
        )?;
        let mut liquid_to_route_kg_m2 = incoming_liquid_kg_m2;
        let mut retained_kg_m2 = 0.0;
        let mut refrozen_kg_m2 = 0.0;
        for (layer, cold_content_j_m2) in layers.iter_mut().zip(cold_content_by_layer.iter_mut()) {
            let refreeze_capacity_kg_m2 =
                (*cold_content_j_m2 / STAGE3_LATENT_HEAT_FUSION_J_KG).max(0.0);
            let refrozen_here_kg_m2 = liquid_to_route_kg_m2.min(refreeze_capacity_kg_m2);
            liquid_to_route_kg_m2 -= refrozen_here_kg_m2;
            *cold_content_j_m2 -= refrozen_here_kg_m2 * STAGE3_LATENT_HEAT_FUSION_J_KG;
            let refrozen_here_m = refrozen_here_kg_m2 / STAGE3_RHO_WATER_KG_M3;
            layer.mass_swe_m += refrozen_here_m;
            layer.refrozen_liquid_m += refrozen_here_m;
            layer.thickness_m = layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3 / layer.density_kg_m3;
            refrozen_kg_m2 += refrozen_here_kg_m2;

            let capacity_kg_m2 = Self::stage3_layer_liquid_holding_capacity_m(
                layer.thickness_m,
                layer.density_kg_m3,
            ) * STAGE3_RHO_WATER_KG_M3;
            let existing_liquid_kg_m2 = layer.liquid_water_m * STAGE3_RHO_WATER_KG_M3;
            let retained_here_kg_m2 =
                liquid_to_route_kg_m2.min((capacity_kg_m2 - existing_liquid_kg_m2).max(0.0));
            liquid_to_route_kg_m2 -= retained_here_kg_m2;
            layer.liquid_water_m += retained_here_kg_m2 / STAGE3_RHO_WATER_KG_M3;
            retained_kg_m2 += retained_here_kg_m2;
            layer.cold_content_j_m2 = (*cold_content_j_m2).max(0.0);
            layer.temperature_c = Self::stage3_temperature_from_cold_content(layer);
        }
        Ok((
            liquid_to_route_kg_m2.max(0.0),
            retained_kg_m2,
            refrozen_kg_m2,
        ))
    }

    pub(super) fn prepare_stage3_sequential_control_volume(
        layers: &mut Vec<DirectSnowLayerState>,
        cold_content_by_layer: &mut Vec<f64>,
    ) -> usize {
        let mut active_layer_count =
            Self::align_stage3_active_layer_boundary(layers, cold_content_by_layer);
        let (_, lower_mass_swe_m) =
            Self::stage3_control_volume_masses_swe_m(layers, active_layer_count);
        if Self::stage3_lower_volume_is_subresolution_swe_m(lower_mass_swe_m) {
            active_layer_count = layers.len();
        }
        Self::normalize_stage3_control_volume_temperature(
            &mut layers[..active_layer_count],
            &mut cold_content_by_layer[..active_layer_count],
        );
        Self::normalize_stage3_control_volume_temperature(
            &mut layers[active_layer_count..],
            &mut cold_content_by_layer[active_layer_count..],
        );
        Self::coalesce_stage3_thermal_fragments(layers, cold_content_by_layer, active_layer_count)
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn evaluate_stage3_same_state_paired_carrier(
        phase_class: HillslopeKernelPhaseClass,
        tag: Stage3EvaluationTag,
        inputs: &DirectActiveSnowPartitionInputs,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &[f64],
    ) -> Result<Stage3ShadowSummary, DirectSnowStage3EvaluationError> {
        let mut summary = Stage3ShadowSummary::new(tag);
        Self::stage3_shadow_fingerprints(inputs, layers, cold_content_by_layer, &mut summary);
        summary.surface_arm_non_formulation_fingerprint = summary.non_formulation_fingerprint;
        Self::stage3_shadow_fingerprints(inputs, layers, cold_content_by_layer, &mut summary);
        summary.complete_arm_non_formulation_fingerprint = summary.non_formulation_fingerprint;
        for hour in &mut summary.hourly {
            hour.requested_seconds = STAGE3_SECONDS_PER_HOUR;
        }
        if Self::stage3_total_ice_mass_swe_m(layers) <= STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M {
            for status in &mut summary.reconciliation.hourly_status {
                *status = DirectSnowStage3ReconciliationHourStatus {
                    evaluated: false,
                    reason: "no_resolved_snow_at_day_start",
                };
            }
            return Ok(summary);
        }
        let snapshot = Self::stage3_control_volume_state(
            phase_class,
            layers,
            cold_content_by_layer,
            inputs.surface_energy_options.atmospheric_pressure_pa,
        )?;
        let surface_temperature_c = Self::stage3_temperature_from_cold_content_values(
            snapshot.mass_swe_m,
            snapshot.cold_content_j_m2,
        );
        let reconciliation_state =
            Self::stage3_reconciliation_state(layers, cold_content_by_layer, layers.len());
        for (hour_index, hourly) in inputs.hourly.iter().copied().enumerate() {
            let carrier = Self::stage3_hourly_surface_energy(
                phase_class,
                inputs,
                hourly,
                Stage3SurfaceInterval {
                    surface_temperature_c,
                    snow_depth_m: snapshot.depth_m,
                    snow_density_kg_m3: snapshot.density_kg_m3,
                    duration_seconds: STAGE3_SECONDS_PER_HOUR,
                    forcing_duration_seconds: STAGE3_SECONDS_PER_HOUR,
                    boundary: None,
                },
                Some(tag.operator),
                DirectSnowDiagnosticCapture::Verbose,
            )?;
            let surface = carrier.diagnostics.ok_or_else(|| {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("snow.stage3_shadow_diagnostics"),
                }
            })?;
            let carrier_reconciliation = carrier.reconciliation.ok_or_else(|| {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("snow.stage3_operator_reconciliation_carrier"),
                }
            })?;
            let hour = DirectSnowStage3EvaluationHourDiagnostics {
                sensible_flux_w_m2: surface.shadow_sensible_flux_w_m2,
                latent_flux_w_m2: surface.shadow_latent_flux_w_m2,
                advected_flux_w_m2: surface.shadow_advected_flux_w_m2,
                shortwave_energy_j_m2: carrier.shortwave_j_m2,
                longwave_energy_j_m2: carrier.longwave_j_m2,
                complete_energy_j_m2: surface.shadow_complete_energy_j_m2,
                vapor_mass_exchange_kg_m2: surface.shadow_vapor_mass_exchange_kg_m2,
                complete_carrier_evaluated: surface.shadow_complete_carrier_evaluated,
                requested_seconds: STAGE3_SECONDS_PER_HOUR,
                evaluated_seconds: STAGE3_SECONDS_PER_HOUR,
                ..DirectSnowStage3EvaluationHourDiagnostics::zero()
            };
            summary.hourly[hour_index] = hour;
            summary.evaluated_seconds += STAGE3_SECONDS_PER_HOUR;
            summary.surface_arm_shortwave_j_m2 += carrier.shortwave_j_m2;
            summary.surface_arm_longwave_j_m2 += carrier.longwave_j_m2;
            summary.surface_arm_latent_j_m2 += carrier.latent_j_m2;
            summary.surface_arm_total_j_m2 += carrier.total_j_m2;
            summary.complete_shortwave_j_m2 += carrier.shortwave_j_m2;
            summary.complete_longwave_j_m2 += carrier.longwave_j_m2;
            summary.complete_sensible_j_m2 += hour.sensible_flux_w_m2 * STAGE3_SECONDS_PER_HOUR;
            summary.complete_latent_j_m2 += hour.latent_flux_w_m2 * STAGE3_SECONDS_PER_HOUR;
            summary.complete_advected_j_m2 += hour.advected_flux_w_m2 * STAGE3_SECONDS_PER_HOUR;
            summary.complete_vapor_mass_exchange_kg_m2 += hour.vapor_mass_exchange_kg_m2;
            summary.complete_energy_j_m2 += hour.complete_energy_j_m2;
            summary.reconciliation.hourly_status[hour_index] =
                DirectSnowStage3ReconciliationHourStatus {
                    evaluated: true,
                    reason: "evaluated",
                };
            let tuple = Self::stage3_reconciliation_tuple(
                &summary,
                hour_index,
                0,
                0.0,
                STAGE3_SECONDS_PER_HOUR,
                STAGE3_SECONDS_PER_HOUR,
                "whole_column_immutable",
                reconciliation_state,
                reconciliation_state,
                true,
                &carrier_reconciliation,
                Stage3ReconciliationTransfer::SAME_STATE,
            );
            summary.reconciliation.tuples.push(tuple);
        }
        Ok(summary)
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn stage3_shadow_fingerprints(
        inputs: &DirectActiveSnowPartitionInputs,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &[f64],
        summary: &mut Stage3ShadowSummary,
    ) {
        let mut source = Self::stage3_fnv1a_start();
        for layer in layers {
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
                source = Self::stage3_fnv1a_u64(source, value.to_bits());
            }
        }
        for cold_content_j_m2 in cold_content_by_layer {
            source = Self::stage3_fnv1a_u64(source, cold_content_j_m2.to_bits());
        }
        let mut forcing = Self::stage3_fnv1a_start();
        for hour in inputs.hourly {
            for value in [
                hour.active_precipitation_m,
                hour.radiation_mj_m2,
                hour.air_temperature_c,
                hour.rain_m,
                hour.snowfall_m,
                hour.cloud_fraction,
                hour.rain_fraction,
                hour.snow_fraction,
            ] {
                forcing = Self::stage3_fnv1a_u64(forcing, value.to_bits());
            }
            forcing = Self::stage3_fnv1a_bytes(forcing, hour.phase_model.id().as_bytes());
            forcing = Self::stage3_fnv1a_u64(
                forcing,
                hour.hydrometeor_temperature_c
                    .map_or(u64::MAX, f64::to_bits),
            );
        }
        for value in [
            inputs.wind_m_s,
            inputs.dewpoint_c,
            inputs.canopy_cover_fraction,
            inputs.surface_energy_options.daily_solar_radiation_mj_m2,
            inputs
                .surface_energy_options
                .daily_extraterrestrial_radiation_mj_m2,
            inputs.surface_energy_options.atmospheric_pressure_pa,
            inputs.underlying_surface_albedo,
        ] {
            forcing = Self::stage3_fnv1a_u64(forcing, value.to_bits());
        }
        forcing =
            Self::stage3_fnv1a_u64(forcing, u64::from(inputs.surface_energy_options.daylight));
        forcing = Self::stage3_fnv1a_bytes(
            forcing,
            inputs.surface_energy_options.longwave_model.id().as_bytes(),
        );
        forcing = Self::stage3_fnv1a_bytes(
            forcing,
            inputs
                .surface_energy_options
                .sublimation_model
                .id()
                .as_bytes(),
        );
        match inputs.snow_albedo_state {
            Some(state) => {
                forcing = Self::stage3_fnv1a_u64(forcing, 1);
                forcing = Self::stage3_fnv1a_bytes(forcing, state.model.id().as_bytes());
                forcing = Self::stage3_fnv1a_u64(forcing, state.albedo.to_bits());
                forcing = Self::stage3_fnv1a_u64(
                    forcing,
                    state.accumulated_positive_temperature_c_day.to_bits(),
                );
            }
            None => forcing = Self::stage3_fnv1a_u64(forcing, 0),
        }
        let geometry = inputs.surface_energy_options.turbulent_geometry;
        let mut geometry_hash = Self::stage3_fnv1a_start();
        for value in [
            geometry.air_temperature_height_m,
            geometry.vapor_pressure_height_m,
            geometry.wind_speed_height_m,
            geometry.aerodynamic_roughness_length_m,
        ] {
            geometry_hash = Self::stage3_fnv1a_u64(geometry_hash, value.to_bits());
        }
        let mut combined = Self::stage3_fnv1a_start();
        for value in [source, forcing, geometry_hash] {
            combined = Self::stage3_fnv1a_u64(combined, value);
        }
        for value in [
            summary.tag.source_snapshot_id,
            summary.tag.support_id,
            summary.tag.cadence_id,
            summary.tag.carrier_id,
            summary.tag.coverage_id,
            summary.tag.unresolved_boundaries_id,
        ] {
            combined = Self::stage3_fnv1a_bytes(combined, value.as_bytes());
        }
        summary.source_fingerprint = source;
        summary.forcing_fingerprint = forcing;
        summary.geometry_fingerprint = geometry_hash;
        summary.non_formulation_fingerprint = combined;
    }

    pub(super) const fn stage3_fnv1a_start() -> u64 {
        0xcbf2_9ce4_8422_2325
    }

    pub(super) fn stage3_fnv1a_u64(mut hash: u64, value: u64) -> u64 {
        for byte in value.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    fn stage3_fnv1a_bytes(mut hash: u64, value: &[u8]) -> u64 {
        for byte in value {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    fn stage3_reconciliation_state(
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &[f64],
        active_layer_count: usize,
    ) -> Stage3ReconciliationState {
        let active_layer_count = active_layer_count.min(layers.len());
        let fingerprint = |layer_limit: usize| {
            let mut hash = Self::stage3_fnv1a_start();
            hash = Self::stage3_fnv1a_u64(hash, layer_limit as u64);
            for (index, layer) in layers.iter().take(layer_limit).enumerate() {
                for value in [
                    layer.mass_swe_m,
                    layer.thickness_m,
                    layer.density_kg_m3,
                    layer.temperature_c,
                    cold_content_by_layer[index],
                ] {
                    hash = Self::stage3_fnv1a_u64(hash, value.to_bits());
                }
            }
            hash
        };
        let active_mass_swe_m = layers[..active_layer_count]
            .iter()
            .map(|layer| layer.mass_swe_m)
            .sum::<f64>();
        let total_mass_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        let total_retained_liquid_kg_m2 = layers
            .iter()
            .map(|layer| layer.liquid_water_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>();
        let active_depth_m = layers[..active_layer_count]
            .iter()
            .map(|layer| layer.thickness_m)
            .sum::<f64>();
        let active_cold_j_m2 = cold_content_by_layer[..active_layer_count]
            .iter()
            .sum::<f64>();
        let total_cold_j_m2 = cold_content_by_layer.iter().sum::<f64>();
        let active_density_kg_m3 = if active_depth_m > 0.0 {
            active_mass_swe_m * STAGE3_RHO_WATER_KG_M3 / active_depth_m
        } else {
            0.0
        };
        let surface_temperature_c = if active_mass_swe_m > 0.0 {
            Self::stage3_temperature_from_cold_content_values(active_mass_swe_m, active_cold_j_m2)
        } else {
            0.0
        };
        let active_fingerprint = fingerprint(active_layer_count);
        let total_fingerprint = fingerprint(layers.len());
        let mut effective_input_fingerprint = Self::stage3_fnv1a_start();
        for value in [
            active_fingerprint,
            active_mass_swe_m.to_bits(),
            active_depth_m.to_bits(),
            active_density_kg_m3.to_bits(),
            active_cold_j_m2.to_bits(),
            surface_temperature_c.to_bits(),
        ] {
            effective_input_fingerprint =
                Self::stage3_fnv1a_u64(effective_input_fingerprint, value);
        }
        Stage3ReconciliationState {
            active_layer_count,
            total_layer_count: layers.len(),
            active_fingerprint,
            total_fingerprint,
            effective_input_fingerprint,
            active_ice_mass_kg_m2: active_mass_swe_m * STAGE3_RHO_WATER_KG_M3,
            total_ice_mass_kg_m2: total_mass_swe_m * STAGE3_RHO_WATER_KG_M3,
            total_retained_liquid_kg_m2,
            active_depth_m,
            active_density_kg_m3,
            active_cold_j_m2,
            total_cold_j_m2,
            surface_temperature_c,
        }
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn stage3_reconciliation_tuple(
        summary: &Stage3ShadowSummary,
        hour_index: usize,
        substep_index: usize,
        elapsed_start_seconds: f64,
        duration_seconds: f64,
        requested_seconds: f64,
        projection_id: &'static str,
        before: Stage3ReconciliationState,
        after: Stage3ReconciliationState,
        after_surface_applicable: bool,
        carrier: &Stage3CarrierReconciliation,
        transfer: Stage3ReconciliationTransfer,
    ) -> DirectSnowStage3ReconciliationTuple {
        let turbulent = carrier.turbulent;
        let fluxes = turbulent.map(|value| value.fluxes);
        DirectSnowStage3ReconciliationTuple {
            operator: summary.tag.operator,
            hour_index,
            substep_index,
            elapsed_start_seconds,
            requested_seconds,
            evaluated_seconds: duration_seconds,
            duration_seconds,
            applicable: true,
            applicability_reason: "evaluated",
            source_fingerprint_fnv1a64: summary.source_fingerprint,
            forcing_fingerprint_fnv1a64: summary.forcing_fingerprint,
            geometry_fingerprint_fnv1a64: summary.geometry_fingerprint,
            effective_input_fingerprint_fnv1a64: before.effective_input_fingerprint,
            projection_id,
            active_layer_prefix_count_before: before.active_layer_count,
            total_layer_count_before: before.total_layer_count,
            active_layer_state_fingerprint_before_fnv1a64: before.active_fingerprint,
            total_layer_state_fingerprint_before_fnv1a64: before.total_fingerprint,
            active_layer_prefix_count_after: after_surface_applicable
                .then_some(after.active_layer_count),
            total_layer_count_after: after.total_layer_count,
            active_layer_state_fingerprint_after_fnv1a64: after_surface_applicable
                .then_some(after.active_fingerprint),
            total_layer_state_fingerprint_after_fnv1a64: after.total_fingerprint,
            after_surface_applicable,
            after_surface_applicability_reason: if after_surface_applicable {
                "resolved_surface"
            } else {
                "post_substep_no_resolved_surface"
            },
            active_ice_mass_before_kg_m2: before.active_ice_mass_kg_m2,
            active_ice_mass_after_kg_m2: after_surface_applicable
                .then_some(after.active_ice_mass_kg_m2),
            total_ice_mass_before_kg_m2: before.total_ice_mass_kg_m2,
            total_ice_mass_after_kg_m2: after.total_ice_mass_kg_m2,
            total_retained_liquid_after_kg_m2: after.total_retained_liquid_kg_m2,
            active_depth_before_m: before.active_depth_m,
            active_depth_after_m: after_surface_applicable.then_some(after.active_depth_m),
            active_density_before_kg_m3: before.active_density_kg_m3,
            active_density_after_kg_m3: after_surface_applicable
                .then_some(after.active_density_kg_m3),
            active_cold_before_j_m2: before.active_cold_j_m2,
            active_cold_after_j_m2: after_surface_applicable.then_some(after.active_cold_j_m2),
            lower_cold_before_conduction_j_m2: transfer.lower_cold_before_conduction_j_m2,
            lower_cold_after_conduction_j_m2: transfer.lower_cold_after_conduction_j_m2,
            total_cold_before_j_m2: before.total_cold_j_m2,
            total_cold_after_j_m2: after.total_cold_j_m2,
            surface_temperature_before_c: before.surface_temperature_c,
            surface_temperature_after_c: after_surface_applicable
                .then_some(after.surface_temperature_c),
            air_temperature_c: carrier.air_temperature_c,
            dewpoint_c: carrier.dewpoint_c,
            wind_speed_m_s: carrier.wind_speed_m_s,
            air_pressure_pa: carrier.air_pressure_pa,
            hourly_radiation_mj_m2: carrier.hourly_radiation_mj_m2,
            daily_solar_radiation_mj_m2: carrier.daily_solar_radiation_mj_m2,
            daily_extraterrestrial_radiation_mj_m2: carrier.daily_extraterrestrial_radiation_mj_m2,
            daylight: carrier.daylight,
            canopy_cover_fraction: carrier.canopy_cover_fraction,
            rain_m: carrier.rain_m,
            snowfall_geometric_m: carrier.snowfall_geometric_m,
            rain_mass_flux_kg_m2_s: carrier.rain_mass_flux_kg_m2_s,
            snow_mass_flux_kg_m2_s: carrier.snow_mass_flux_kg_m2_s,
            rain_temperature_c: carrier.rain_temperature_c,
            snow_temperature_c: carrier.snow_temperature_c,
            rain_specific_heat_j_kg_k: carrier.rain_specific_heat_j_kg_k,
            snow_specific_heat_j_kg_k: carrier.snow_specific_heat_j_kg_k,
            incoming_shortwave_w_m2: carrier.incoming_shortwave_w_m2,
            snow_albedo_fraction: carrier.snow_albedo_fraction,
            snow_albedo_source_id: carrier.snow_albedo_source_id,
            snow_albedo_model_id: carrier.snow_albedo_model_id,
            snow_albedo_accumulated_positive_temperature_c_day: carrier
                .snow_albedo_accumulated_positive_temperature_c_day,
            net_shortwave_w_m2: carrier.net_shortwave_w_m2,
            actual_vapor_pressure_pa: carrier.actual_vapor_pressure_pa,
            longwave_cloud_fraction: carrier.longwave_cloud_fraction,
            sky_view_fraction: carrier.sky_view_fraction,
            atmospheric_longwave_w_m2: carrier.atmospheric_longwave_w_m2,
            canopy_longwave_w_m2: carrier.canopy_longwave_w_m2,
            subcanopy_longwave_w_m2: carrier.subcanopy_longwave_w_m2,
            outgoing_longwave_w_m2: carrier.outgoing_longwave_w_m2,
            net_longwave_w_m2: carrier.net_longwave_w_m2,
            longwave_model_id: carrier.longwave_model_id,
            sublimation_model_id: carrier.sublimation_model_id,
            air_temperature_height_m: carrier.air_temperature_height_m,
            vapor_pressure_height_m: carrier.vapor_pressure_height_m,
            wind_speed_height_m: carrier.wind_speed_height_m,
            aerodynamic_roughness_length_m: carrier.aerodynamic_roughness_length_m,
            turbulent_max_iterations: carrier.turbulent_options.max_iterations,
            turbulent_convergence_tolerance: carrier.turbulent_options.convergence_tolerance,
            surface_vapor_pressure_pa: carrier.surface_vapor_pressure_pa,
            air_potential_temperature_k: turbulent
                .and_then(|value| value.air_potential_temperature_k),
            surface_temperature_k: turbulent.and_then(|value| value.surface_temperature_k),
            specific_humidity_air_kg_kg: turbulent
                .and_then(|value| value.specific_humidity_air_kg_kg),
            specific_humidity_surface_kg_kg: turbulent
                .and_then(|value| value.specific_humidity_surface_kg_kg),
            air_density_kg_m3: turbulent.and_then(|value| value.air_density_kg_m3),
            displacement_height_m: turbulent.and_then(|value| value.displacement_height_m),
            log_momentum: turbulent.and_then(|value| value.log_momentum),
            log_sensible: turbulent.and_then(|value| value.log_sensible),
            log_latent: turbulent.and_then(|value| value.log_latent),
            turbulent_termination_status: turbulent
                .map_or("shared_boundary_v1", |value| value.termination_status.id()),
            stability_class: turbulent
                .map_or("shared_boundary_v1", |value| value.stability_class.id()),
            obukhov_length_m: fluxes.and_then(|value| value.obukhov_length_m),
            psi_momentum: turbulent.map_or(0.0, |value| value.momentum_stability_correction),
            psi_sensible: turbulent.map_or(0.0, |value| value.sensible_stability_correction),
            psi_latent: turbulent.map_or(0.0, |value| value.latent_stability_correction),
            turbulent_iterations: fluxes.map_or(0, |value| value.iterations),
            friction_velocity_m_s: turbulent.map_or(0.0, |value| value.friction_velocity_m_s),
            sensible_exchange_velocity_m_s: turbulent
                .and_then(|value| value.sensible_exchange_velocity_m_s),
            latent_exchange_velocity_m_s: turbulent
                .and_then(|value| value.latent_exchange_velocity_m_s),
            surface_latent_heat_j_kg: carrier.surface_latent_heat_j_kg,
            vapor_mass_flux_kg_m2_s: carrier.vapor_mass_flux_kg_m2_s,
            sensible_flux_w_m2: carrier.sensible_flux_w_m2,
            latent_flux_w_m2: carrier.latent_flux_w_m2,
            precipitation_advected_flux_w_m2: carrier.precipitation_advected_flux_w_m2,
            snow_soil_heat_flux_w_m2: carrier.snow_soil_heat_flux_w_m2,
            complete_external_flux_w_m2: carrier.complete_external_flux_w_m2,
            vapor_mass_exchange_kg_m2: carrier.vapor_mass_flux_kg_m2_s * duration_seconds,
            sublimation_kg_m2: transfer.sublimation_kg_m2,
            deposition_kg_m2: transfer.deposition_kg_m2,
            melt_kg_m2: transfer.melt_kg_m2,
            refrozen_kg_m2: transfer.refrozen_kg_m2,
            active_cold_energy_change_j_m2: transfer.active_cold_energy_change_j_m2,
            lower_cold_energy_change_j_m2: transfer.lower_cold_energy_change_j_m2,
            cold_content_export_j_m2: transfer.cold_content_export_j_m2,
            internal_active_lower_conduction_j_m2: transfer.internal_active_lower_conduction_j_m2,
            legacy_sequential_complete_j_m2: transfer.legacy_sequential_complete_j_m2,
            energy_closure_residual_j_m2: transfer.energy_closure_residual_j_m2,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn stage3_hourly_surface_energy(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        interval: Stage3SurfaceInterval,
        evaluation_operator: Option<SnowStage3EvaluationOperator>,
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<Stage3HourlySurfaceEnergy, DirectSnowStage3EvaluationError> {
        let Stage3SurfaceInterval {
            surface_temperature_c,
            snow_depth_m,
            snow_density_kg_m3,
            duration_seconds,
            forcing_duration_seconds,
            boundary,
        } = interval;

        if let Some(boundary) = boundary {
            let support_seconds = support_duration_seconds(boundary.support.duration_ns());
            if !support_seconds.is_finite()
                || support_seconds <= 0.0
                || duration_seconds <= 0.0
                || duration_seconds > support_seconds
            {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_covered_boundary_duration",
                    duration_seconds,
                    Some(f64::MIN_POSITIVE),
                    Some(support_seconds),
                )
                .into());
            }
            let scale = duration_seconds / support_seconds;
            let vapor_mass_exchange_kg_m2 = boundary.vapor_mass_kg_m2 * scale;
            let latent_heat_j_kg = boundary.latent_heat_j_kg;
            let latent_j_m2 = boundary.latent_energy_j_m2 * scale;
            let shortwave_j_m2 = boundary.shortwave_energy_j_m2 * scale;
            let longwave_j_m2 = boundary.net_longwave_energy_j_m2 * scale;
            let sensible_j_m2 = boundary.sensible_energy_j_m2 * scale;
            let advected_j_m2 = boundary.precipitation_advection_j_m2 * scale;
            let snow_soil_heat_j_m2 = boundary.snow_soil_heat_j_m2 * scale;
            let total_j_m2 = shortwave_j_m2
                + sensible_j_m2
                + latent_j_m2
                + longwave_j_m2
                + advected_j_m2
                + snow_soil_heat_j_m2;
            let sublimation_m = (-vapor_mass_exchange_kg_m2 / STAGE3_RHO_WATER_KG_M3).max(0.0);
            let diagnostics = capture.is_verbose().then(|| {
                let mut diagnostics = DirectSnowSurfaceEnergyHourDiagnostics::zero();
                diagnostics.surface_temperature_c = surface_temperature_c;
                diagnostics.net_shortwave_w_m2 = shortwave_j_m2 / duration_seconds;
                diagnostics.net_longwave_w_m2 = longwave_j_m2 / duration_seconds;
                diagnostics.vapor_mass_exchange_kg_m2 = vapor_mass_exchange_kg_m2;
                diagnostics.latent_heat_j_kg = latent_heat_j_kg;
                diagnostics.latent_flux_w_m2 = latent_j_m2 / duration_seconds;
                diagnostics.shadow_sensible_flux_w_m2 = sensible_j_m2 / duration_seconds;
                diagnostics.shadow_latent_flux_w_m2 = latent_j_m2 / duration_seconds;
                diagnostics.shadow_advected_flux_w_m2 = advected_j_m2 / duration_seconds;
                diagnostics.shadow_complete_energy_j_m2 = total_j_m2;
                diagnostics.shadow_vapor_mass_exchange_kg_m2 = vapor_mass_exchange_kg_m2;
                diagnostics.shadow_complete_carrier_evaluated = true;
                diagnostics.potential_surface_energy_j_m2 = total_j_m2;
                diagnostics
            });
            return Ok(Stage3HourlySurfaceEnergy {
                total_j_m2,
                shortwave_j_m2,
                longwave_j_m2,
                latent_j_m2,
                vapor_mass_exchange_kg_m2,
                latent_mass_energy_j_m2: vapor_mass_exchange_kg_m2 * latent_heat_j_kg,
                sublimation_m,
                mass_latent_identity_residual_j_m2: latent_j_m2
                    - vapor_mass_exchange_kg_m2 * latent_heat_j_kg,
                diagnostics,
                reconciliation: Some(Self::boundary_reconciliation(
                    inputs,
                    hourly,
                    surface_temperature_c,
                    boundary,
                )),
            });
        }
        let albedo_value = inputs
            .snow_albedo_state
            .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
        let (snow_albedo_source_id, snow_albedo_model_id, snow_albedo_temperature_state) = inputs
            .snow_albedo_state
            .map_or(("stage3_default_snow_albedo_0p82", None, None), |state| {
                (
                    "snow_albedo_state",
                    Some(state.model.id()),
                    Some(state.accumulated_positive_temperature_c_day),
                )
            });
        let albedo = FractionUnitInterval::try_new(albedo_value).map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_surface_albedo",
                albedo_value,
                Some(0.0),
                Some(1.0),
            )
        })?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_hourly_radiation_mj_m2"),
            hourly.radiation_mj_m2,
            Some(0.0),
            None,
        )?;
        // UNIT-CONVERSION-ALLOW: contract-bound MJ m^-2 hourly energy to W m^-2.
        let incoming_w_m2 = hourly.radiation_mj_m2 * 1_000_000.0 / forcing_duration_seconds;
        let shortwave = net_shortwave_radiation(
            RadiativeFluxWattsPerSquareMeter::try_new(incoming_w_m2).map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_hourly_shortwave_w_m2",
                    incoming_w_m2,
                    Some(0.0),
                    None,
                )
            })?,
            albedo,
        )
        .map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_net_shortwave_w_m2",
                incoming_w_m2,
                None,
                None,
            )
        })?;
        let mut longwave_w_m2 = 0.0;
        let mut longwave_cloud_fraction = 0.0;
        let mut sky_view_fraction = 0.0;
        let mut atmospheric_longwave_w_m2 = 0.0;
        let mut canopy_longwave_w_m2 = 0.0;
        let mut subcanopy_longwave_w_m2 = 0.0;
        let mut outgoing_longwave_w_m2 = 0.0;
        let mut diagnostics =
            capture
                .is_verbose()
                .then(|| DirectSnowSurfaceEnergyHourDiagnostics {
                    surface_temperature_c,
                    net_shortwave_w_m2: shortwave.as_watts_per_square_meter(),
                    ..DirectSnowSurfaceEnergyHourDiagnostics::zero()
                });
        if inputs.surface_energy_options.longwave_model
            == SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1
        {
            let air_temperature = Self::stage3_temperature(phase_class, hourly.air_temperature_c)?;
            let surface_temperature = Self::stage3_temperature(phase_class, surface_temperature_c)?;
            let actual_vapor_pressure = saturation_vapor_pressure_snobal_pa(
                Self::stage3_temperature(phase_class, inputs.dewpoint_c)?,
            )
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.actual_vapor_pressure",
                    inputs.dewpoint_c,
                    None,
                    None,
                )
            })?;
            let fluxes = snow_longwave_dilley_unsworth(SnowLongwaveInputs {
                air_temperature,
                surface_temperature,
                actual_vapor_pressure,
                daily_solar_radiation_mj_m2: inputs
                    .surface_energy_options
                    .daily_solar_radiation_mj_m2,
                daily_extraterrestrial_radiation_mj_m2: inputs
                    .surface_energy_options
                    .daily_extraterrestrial_radiation_mj_m2,
                daylight: inputs.surface_energy_options.daylight,
                canopy_cover: FractionUnitInterval::try_new(inputs.canopy_cover_fraction).map_err(
                    |_| {
                        Self::stage3_domain_error(
                            phase_class,
                            "snow.canopy_cover_fraction",
                            inputs.canopy_cover_fraction,
                            Some(0.0),
                            Some(1.0),
                        )
                    },
                )?,
            })
            .map_err(|error| match error {
                openwepp_meteorology::MeteorologyError::CloudForcingUnavailable => {
                    Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: BoundarySymbol::from("snow.cloud_forcing_unavailable"),
                    }
                }
                openwepp_meteorology::MeteorologyError::OutOfAuthority { value, .. } => {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.longwave_out_of_authority",
                        value,
                        Some(0.0),
                        Some(1.0),
                    )
                }
                _ => Self::stage3_domain_error(
                    phase_class,
                    "snow.longwave_forcing",
                    inputs
                        .surface_energy_options
                        .daily_extraterrestrial_radiation_mj_m2,
                    Some(1.0e-9),
                    None,
                ),
            })?;
            longwave_w_m2 = fluxes.net_longwave.as_watts_per_square_meter();
            longwave_cloud_fraction = fluxes.cloud_fraction.as_fraction();
            sky_view_fraction = fluxes.sky_view_fraction.as_fraction();
            atmospheric_longwave_w_m2 = fluxes.atmospheric_longwave.as_watts_per_square_meter();
            canopy_longwave_w_m2 = fluxes.canopy_longwave.as_watts_per_square_meter();
            subcanopy_longwave_w_m2 = fluxes.subcanopy_longwave.as_watts_per_square_meter();
            outgoing_longwave_w_m2 = fluxes.outgoing_longwave.as_watts_per_square_meter();
            if let Some(diagnostics) = diagnostics.as_mut() {
                diagnostics.atmospheric_longwave_w_m2 =
                    fluxes.atmospheric_longwave.as_watts_per_square_meter();
                diagnostics.canopy_longwave_w_m2 =
                    fluxes.canopy_longwave.as_watts_per_square_meter();
                diagnostics.sky_view_fraction = fluxes.sky_view_fraction.as_fraction();
                diagnostics.subcanopy_longwave_w_m2 =
                    fluxes.subcanopy_longwave.as_watts_per_square_meter();
                diagnostics.outgoing_longwave_w_m2 =
                    fluxes.outgoing_longwave.as_watts_per_square_meter();
                diagnostics.net_longwave_w_m2 = longwave_w_m2;
            }
        }
        let mut sublimation_m = 0.0;
        let mut latent_w_m2 = 0.0;
        let mut latent_heat_j_kg = 0.0;
        let mut vapor_mass_exchange_kg_m2 = 0.0;
        let mut reconciliation = None;
        if inputs.surface_energy_options.sublimation_model
            == SnowSurfaceSublimationModel::NeutralBulkStage3V1
        {
            sublimation_m = Self::coe_open_sublimation_hour_m(
                phase_class,
                inputs.canopy_cover_fraction,
                inputs.wind_m_s,
                hourly.air_temperature_c,
                inputs.dewpoint_c,
                snow_depth_m,
                surface_temperature_c,
                true,
            )? * (duration_seconds / forcing_duration_seconds);
            sublimation_m =
                sublimation_m.min(snow_depth_m * snow_density_kg_m3 / STAGE3_RHO_WATER_KG_M3);
            let mass_flux = MassFluxKilogramsPerSquareMeterSecond::try_new(
                -sublimation_m * STAGE3_RHO_WATER_KG_M3 / duration_seconds,
            )
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.sublimation_mass_flux",
                    sublimation_m,
                    None,
                    None,
                )
            })?;
            let latent_heat = latent_heat_for_surface_temperature(Self::stage3_temperature(
                phase_class,
                surface_temperature_c,
            )?)
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.latent_heat",
                    surface_temperature_c,
                    None,
                    None,
                )
            })?;
            latent_heat_j_kg = latent_heat.as_joules_per_kilogram();
            latent_w_m2 = latent_heat_flux_from_mass_flux(mass_flux, latent_heat)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.latent_heat_flux",
                        sublimation_m,
                        None,
                        None,
                    )
                })?
                .as_watts_per_square_meter();
            vapor_mass_exchange_kg_m2 = -sublimation_m * STAGE3_RHO_WATER_KG_M3;
            if let Some(diagnostics) = diagnostics.as_mut() {
                diagnostics.vapor_mass_exchange_kg_m2 = vapor_mass_exchange_kg_m2;
                diagnostics.latent_heat_j_kg = latent_heat_j_kg;
                diagnostics.latent_flux_w_m2 = latent_w_m2;
            }
        }
        if let Some(operator) = evaluation_operator {
            if inputs.surface_energy_options.longwave_model
                != SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1
            {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_shadow_requires_complete_longwave",
                    0.0,
                    Some(1.0),
                    Some(1.0),
                )
                .into());
            }
            let geometry = inputs.surface_energy_options.turbulent_geometry;
            let air_temperature = Self::stage3_temperature(phase_class, hourly.air_temperature_c)?;
            let surface_temperature = Self::stage3_temperature(phase_class, surface_temperature_c)?;
            let air_vapor_pressure = saturation_vapor_pressure_snobal_pa(Self::stage3_temperature(
                phase_class,
                inputs.dewpoint_c,
            )?)
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_shadow_air_vapor_pressure",
                    inputs.dewpoint_c,
                    None,
                    None,
                )
            })?;
            let surface_vapor_pressure = saturation_vapor_pressure_snobal_pa(surface_temperature)
                .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_shadow_surface_vapor_pressure",
                    surface_temperature_c,
                    None,
                    None,
                )
            })?;
            let length = |symbol: &'static str, value: f64| {
                PositiveLengthMeters::try_new(value).map_err(|_| {
                    Self::stage3_domain_error(phase_class, symbol, value, Some(0.0), None)
                })
            };
            let turbulent_options = TurbulentTransferOptions::default();
            let turbulent = turbulent_fluxes_monin_obukhov_with_diagnostics(TurbulentFluxInputs {
                air_pressure: PressurePascals::try_new(
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_shadow_air_pressure_pa",
                        inputs.surface_energy_options.atmospheric_pressure_pa,
                        Some(0.0),
                        None,
                    )
                })?,
                air_temperature,
                surface_temperature,
                air_vapor_pressure,
                surface_vapor_pressure,
                air_temperature_height: length(
                    "snow.stage3_air_temperature_height_m",
                    geometry.air_temperature_height_m,
                )?,
                vapor_pressure_height: length(
                    "snow.stage3_vapor_pressure_height_m",
                    geometry.vapor_pressure_height_m,
                )?,
                wind_speed: LinearRateMetersPerSecond::try_new(inputs.wind_m_s).map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_shadow_wind_m_s",
                        inputs.wind_m_s,
                        Some(0.0),
                        None,
                    )
                })?,
                wind_speed_height: length(
                    "snow.stage3_wind_speed_height_m",
                    geometry.wind_speed_height_m,
                )?,
                roughness_length: length(
                    "snow.stage3_aerodynamic_roughness_length_m",
                    geometry.aerodynamic_roughness_length_m,
                )?,
                options: turbulent_options,
            })
            .map_err(|source| {
                DirectSnowStage3EvaluationError::TurbulentTransfer(Box::new(
                    SnowStage3TurbulentTransferError {
                        phase_class,
                        source,
                        operator,
                        geometry,
                        atmospheric_pressure_pa: inputs
                            .surface_energy_options
                            .atmospheric_pressure_pa,
                        wind_speed_m_s: inputs.wind_m_s,
                        air_temperature_c: hourly.air_temperature_c,
                        surface_temperature_c,
                        air_vapor_pressure_pa: air_vapor_pressure.as_pascals(),
                        surface_vapor_pressure_pa: surface_vapor_pressure.as_pascals(),
                    },
                ))
            })?;
            let precipitation_temperature_c = if hourly.rain_m > 0.0 || hourly.snowfall_m > 0.0 {
                hourly.hydrometeor_temperature_c.ok_or_else(|| {
                    Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: BoundarySymbol::from(
                            "snow.stage3_shadow_hydrometeor_temperature_c",
                        ),
                    }
                })?
            } else {
                surface_temperature_c
            };
            let precipitation_temperature =
                Self::stage3_temperature(phase_class, precipitation_temperature_c)?;
            // Hourly precipitation is a forcing total. Hold its rate across
            // stability substeps so the hour's mass and advected heat are
            // integrated exactly once.
            let rain_mass_flux = hourly.rain_m * STAGE3_RHO_WATER_KG_M3 / forcing_duration_seconds;
            let snow_mass_flux =
                hourly.snowfall_m * 0.1 * STAGE3_RHO_WATER_KG_M3 / forcing_duration_seconds;
            let advected = precipitation_advected_heat_flux(PrecipitationAdvectedHeatInputs {
                rain_mass_flux: PrecipitationMassFluxKilogramsPerSquareMeterSecond::try_new(
                    rain_mass_flux,
                )
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_shadow_rain_mass_flux",
                        rain_mass_flux,
                        Some(0.0),
                        None,
                    )
                })?,
                rain_temperature: precipitation_temperature,
                snow_mass_flux: PrecipitationMassFluxKilogramsPerSquareMeterSecond::try_new(
                    snow_mass_flux,
                )
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_shadow_snow_mass_flux",
                        snow_mass_flux,
                        Some(0.0),
                        None,
                    )
                })?,
                snow_temperature: precipitation_temperature,
                surface_temperature,
            })
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_shadow_advected_heat",
                    precipitation_temperature_c,
                    None,
                    None,
                )
            })?;
            let shadow_surface_flux_w_m2 = shortwave.as_watts_per_square_meter()
                + longwave_w_m2
                + turbulent.fluxes.sensible_heat.as_watts_per_square_meter()
                + turbulent.fluxes.latent_heat.as_watts_per_square_meter()
                + advected.as_watts_per_square_meter();
            let rain_specific_heat_j_kg_k = specific_heat_water(precipitation_temperature)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_shadow_rain_specific_heat",
                        precipitation_temperature_c,
                        None,
                        None,
                    )
                })?
                .as_joules_per_kilogram_kelvin();
            let snow_specific_heat_j_kg_k = specific_heat_ice(precipitation_temperature)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_shadow_snow_specific_heat",
                        precipitation_temperature_c,
                        None,
                        None,
                    )
                })?
                .as_joules_per_kilogram_kelvin();
            reconciliation = Some(Stage3CarrierReconciliation {
                air_temperature_c: hourly.air_temperature_c,
                dewpoint_c: inputs.dewpoint_c,
                wind_speed_m_s: inputs.wind_m_s,
                air_pressure_pa: inputs.surface_energy_options.atmospheric_pressure_pa,
                hourly_radiation_mj_m2: hourly.radiation_mj_m2,
                daily_solar_radiation_mj_m2: inputs
                    .surface_energy_options
                    .daily_solar_radiation_mj_m2,
                daily_extraterrestrial_radiation_mj_m2: inputs
                    .surface_energy_options
                    .daily_extraterrestrial_radiation_mj_m2,
                daylight: inputs.surface_energy_options.daylight,
                canopy_cover_fraction: inputs.canopy_cover_fraction,
                rain_m: hourly.rain_m,
                snowfall_geometric_m: hourly.snowfall_m,
                rain_mass_flux_kg_m2_s: rain_mass_flux,
                snow_mass_flux_kg_m2_s: snow_mass_flux,
                rain_temperature_c: precipitation_temperature_c,
                snow_temperature_c: precipitation_temperature_c,
                rain_specific_heat_j_kg_k,
                snow_specific_heat_j_kg_k,
                incoming_shortwave_w_m2: incoming_w_m2,
                snow_albedo_fraction: albedo_value,
                snow_albedo_source_id,
                snow_albedo_model_id,
                snow_albedo_accumulated_positive_temperature_c_day: snow_albedo_temperature_state,
                net_shortwave_w_m2: shortwave.as_watts_per_square_meter(),
                actual_vapor_pressure_pa: air_vapor_pressure.as_pascals(),
                longwave_cloud_fraction,
                sky_view_fraction,
                atmospheric_longwave_w_m2,
                canopy_longwave_w_m2,
                subcanopy_longwave_w_m2,
                outgoing_longwave_w_m2,
                net_longwave_w_m2: longwave_w_m2,
                longwave_model_id: inputs.surface_energy_options.longwave_model.id(),
                sublimation_model_id: inputs.surface_energy_options.sublimation_model.id(),
                air_temperature_height_m: geometry.air_temperature_height_m,
                vapor_pressure_height_m: geometry.vapor_pressure_height_m,
                wind_speed_height_m: geometry.wind_speed_height_m,
                aerodynamic_roughness_length_m: geometry.aerodynamic_roughness_length_m,
                turbulent_options,
                surface_vapor_pressure_pa: surface_vapor_pressure.as_pascals(),
                surface_latent_heat_j_kg: turbulent.latent_heat_j_kg,
                turbulent: Some(turbulent),
                vapor_mass_flux_kg_m2_s: turbulent
                    .fluxes
                    .mass_flux
                    .as_kilograms_per_square_meter_second(),
                sensible_flux_w_m2: turbulent.fluxes.sensible_heat.as_watts_per_square_meter(),
                latent_flux_w_m2: turbulent.fluxes.latent_heat.as_watts_per_square_meter(),
                precipitation_advected_flux_w_m2: advected.as_watts_per_square_meter(),
                snow_soil_heat_flux_w_m2: 0.0,
                complete_external_flux_w_m2: shadow_surface_flux_w_m2,
            });
            if let Some(diagnostics) = diagnostics.as_mut() {
                diagnostics.shadow_sensible_flux_w_m2 =
                    turbulent.fluxes.sensible_heat.as_watts_per_square_meter();
                diagnostics.shadow_latent_flux_w_m2 =
                    turbulent.fluxes.latent_heat.as_watts_per_square_meter();
                diagnostics.shadow_advected_flux_w_m2 = advected.as_watts_per_square_meter();
                diagnostics.shadow_complete_energy_j_m2 =
                    shadow_surface_flux_w_m2 * duration_seconds;
                diagnostics.shadow_vapor_mass_exchange_kg_m2 = turbulent
                    .fluxes
                    .mass_flux
                    .as_kilograms_per_square_meter_second()
                    * duration_seconds;
                diagnostics.shadow_complete_carrier_evaluated = true;
            }
        }
        let zero = EnergyFluxWattsPerSquareMeter::try_new(0.0).map_err(|_| {
            Self::stage3_domain_error(phase_class, "snow.stage3_zero_flux", 0.0, None, None)
        })?;
        let balance = surface_energy_balance(SurfaceEnergyBalanceTerms {
            net_radiation: EnergyFluxWattsPerSquareMeter::try_new(
                shortwave.as_watts_per_square_meter() + longwave_w_m2,
            )
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_net_radiation",
                    longwave_w_m2,
                    None,
                    None,
                )
            })?,
            sensible_heat: zero,
            latent_heat: EnergyFluxWattsPerSquareMeter::try_new(latent_w_m2).map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_latent_flux",
                    latent_w_m2,
                    None,
                    None,
                )
            })?,
            conduction: zero,
            advected_heat: zero,
        })
        .map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_surface_energy_balance",
                longwave_w_m2 + latent_w_m2,
                None,
                None,
            )
        })?;
        Ok(Stage3HourlySurfaceEnergy {
            total_j_m2: balance.as_watts_per_square_meter() * duration_seconds,
            shortwave_j_m2: shortwave.as_watts_per_square_meter() * duration_seconds,
            longwave_j_m2: longwave_w_m2 * duration_seconds,
            latent_j_m2: latent_w_m2 * duration_seconds,
            vapor_mass_exchange_kg_m2,
            latent_mass_energy_j_m2: vapor_mass_exchange_kg_m2 * latent_heat_j_kg,
            sublimation_m,
            mass_latent_identity_residual_j_m2: latent_w_m2 * duration_seconds
                - vapor_mass_exchange_kg_m2 * latent_heat_j_kg,
            diagnostics: diagnostics.map(|diagnostics| DirectSnowSurfaceEnergyHourDiagnostics {
                potential_surface_energy_j_m2: balance.as_watts_per_square_meter()
                    * duration_seconds,
                ..diagnostics
            }),
            reconciliation,
        })
    }
}
