#[allow(clippy::wildcard_imports)]
use super::*;

mod evaluation;

impl Wb11HydrologyKernel {
    pub(super) fn resolve_stage3_liquid_routing(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        incoming_liquid_m: f64,
        aggregate: Stage3AggregateState,
        layers: &mut Vec<DirectSnowLayerState>,
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<DirectSnowStage3Resolution, Wb11HydrologyKernelGuardError> {
        if !Self::stage3_liquid_routing_enabled(phase_class, inputs, incoming_liquid_m)? {
            return Ok(DirectSnowStage3Resolution::disabled(capture));
        }
        Self::prepare_stage3_layer_stack(phase_class, inputs, aggregate, layers)?;
        if layers.is_empty() {
            let meltwater_temperature_c = if incoming_liquid_m > WB11_ZERO_THRESHOLD {
                Some(Self::stage3_temperature(phase_class, 0.0)?)
            } else {
                None
            };
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
                diagnostics: capture.is_verbose().then(DirectSnowStage3Diagnostics::disabled),
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
        let shadow_summary = if inputs.surface_energy_options.complete_carrier_shadow
            && capture.is_verbose()
        {
            Some(Self::evaluate_stage3_sequential_melt_shadow(
                phase_class,
                inputs,
                layers.clone(),
                cold_content_by_layer.clone(),
            )?)
        } else {
            None
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
                    },
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
                if let Some(shadow) = shadow_summary {
                    let sequential = shadow.hourly[hour_index];
                    hour_diagnostics.shadow_sensible_flux_w_m2 =
                        sequential.shadow_sensible_flux_w_m2;
                    hour_diagnostics.shadow_latent_flux_w_m2 =
                        sequential.shadow_latent_flux_w_m2;
                    hour_diagnostics.shadow_advected_flux_w_m2 =
                        sequential.shadow_advected_flux_w_m2;
                    hour_diagnostics.shadow_complete_energy_j_m2 =
                        sequential.shadow_complete_energy_j_m2;
                    hour_diagnostics.shadow_vapor_mass_exchange_kg_m2 =
                        sequential.shadow_vapor_mass_exchange_kg_m2;
                    hour_diagnostics.shadow_cold_required_j_m2 =
                        sequential.shadow_cold_required_j_m2;
                    hour_diagnostics.shadow_cold_energy_change_j_m2 =
                        sequential.shadow_cold_energy_change_j_m2;
                    hour_diagnostics.shadow_excess_energy_j_m2 =
                        sequential.shadow_excess_energy_j_m2;
                    hour_diagnostics.shadow_ice_available_kg_m2 =
                        sequential.shadow_ice_available_kg_m2;
                    hour_diagnostics.shadow_sublimation_kg_m2 =
                        sequential.shadow_sublimation_kg_m2;
                    hour_diagnostics.shadow_melt_kg_m2 = sequential.shadow_melt_kg_m2;
                    hour_diagnostics.shadow_unallocated_after_exhaustion_j_m2 =
                        sequential.shadow_unallocated_after_exhaustion_j_m2;
                    hour_diagnostics.shadow_energy_closure_residual_j_m2 =
                        sequential.shadow_energy_closure_residual_j_m2;
                    hour_diagnostics.shadow_complete_carrier_evaluated =
                        sequential.shadow_complete_carrier_evaluated;
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
                    shadow_complete_energy_j_m2: shadow_summary
                        .map_or(0.0, |shadow| shadow.complete_energy_j_m2),
                    shadow_cold_energy_change_j_m2: shadow_summary
                        .map_or(0.0, |shadow| shadow.cold_energy_change_j_m2),
                    shadow_excess_energy_j_m2: shadow_summary
                        .map_or(0.0, |shadow| shadow.excess_energy_j_m2),
                    shadow_sublimation_kg_m2: shadow_summary
                        .map_or(0.0, |shadow| shadow.sublimation_kg_m2),
                    shadow_melt_kg_m2: shadow_summary
                        .map_or(0.0, |shadow| shadow.melt_kg_m2),
                    shadow_unallocated_after_exhaustion_j_m2: shadow_summary.map_or(
                        0.0,
                        |shadow| shadow.unallocated_after_exhaustion_j_m2,
                    ),
                    shadow_maximum_energy_closure_residual_j_m2: shadow_summary.map_or(
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
        })
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

    fn adjust_stage3_layer_swe_to_target(
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

    fn stage3_lower_volume_is_subresolution_swe_m(lower_mass_swe_m: f64) -> bool {
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

    fn remove_stage3_active_sublimation(
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
