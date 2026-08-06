#[allow(clippy::wildcard_imports)]
use super::*;

impl Wb11HydrologyKernel {
    pub(super) fn evaluate_stage3_sequential_melt_shadow(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        mut layers: Vec<DirectSnowLayerState>,
        mut cold_content_by_layer: Vec<f64>,
    ) -> Result<Stage3ShadowSummary, Wb11HydrologyKernelGuardError> {
        let mut summary = Stage3ShadowSummary::ZERO;
        for (hour_index, hourly) in inputs.hourly.iter().copied().enumerate() {
            let mut elapsed_seconds = 0.0;
            while elapsed_seconds < STAGE3_SECONDS_PER_HOUR && !layers.is_empty() {
                if Self::stage3_total_ice_mass_swe_m(&layers)
                    <= STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M
                {
                    break;
                }
                let mut active_layer_count =
                    Self::align_stage3_active_layer_boundary(&mut layers, &mut cold_content_by_layer);
                let (_, lower_mass_swe_m) =
                    Self::stage3_control_volume_masses_swe_m(&layers, active_layer_count);
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
                active_layer_count = Self::coalesce_stage3_thermal_fragments(
                    &mut layers,
                    &mut cold_content_by_layer,
                    active_layer_count,
                );
                let substep_seconds = Self::stage3_substep_seconds(&layers, active_layer_count)
                    .min(STAGE3_SECONDS_PER_HOUR - elapsed_seconds);
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
                    },
                    DirectSnowDiagnosticCapture::Verbose,
                )?;
                let surface = carrier.diagnostics.ok_or_else(|| {
                    Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: BoundarySymbol::from("snow.stage3_shadow_diagnostics"),
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
                let cold_after_j_m2 = cold_content_by_layer[..active_layer_count]
                    .iter()
                    .sum::<f64>();
                let cold_energy_change_j_m2 = cold_required_j_m2 - cold_after_j_m2;
                let excess_energy_j_m2 =
                    (q_complete_j_m2 - cold_energy_change_j_m2).max(0.0);
                let active_ice_kg_m2 = layers[..active_layer_count]
                    .iter()
                    .map(|layer| layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3)
                    .sum::<f64>();
                let sublimation_kg_m2 = (-surface.shadow_vapor_mass_exchange_kg_m2)
                    .max(0.0)
                    .min(active_ice_kg_m2);
                let ice_available_kg_m2 =
                    (active_ice_kg_m2 - sublimation_kg_m2).max(0.0);
                let melt_kg_m2 = (excess_energy_j_m2 / STAGE3_LATENT_HEAT_FUSION_J_KG)
                    .min(ice_available_kg_m2);
                let unallocated_j_m2 = (excess_energy_j_m2
                    - STAGE3_LATENT_HEAT_FUSION_J_KG * melt_kg_m2)
                    .max(0.0);
                let closure_residual_j_m2 = q_complete_j_m2
                    - cold_energy_change_j_m2
                    - STAGE3_LATENT_HEAT_FUSION_J_KG * melt_kg_m2
                    - unallocated_j_m2;
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from("snow.stage3_shadow_energy_residual_j_m2"),
                    closure_residual_j_m2.abs(),
                    None,
                    Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
                )?;

                let mut removal_active_count = active_layer_count;
                if melt_kg_m2 > 0.0 {
                    let _ = Self::remove_stage3_active_sublimation(
                        melt_kg_m2 / STAGE3_RHO_WATER_KG_M3,
                        &mut layers,
                        &mut cold_content_by_layer,
                        &mut removal_active_count,
                    );
                }
                if sublimation_kg_m2 > 0.0 && !layers.is_empty() {
                    removal_active_count = removal_active_count.min(layers.len());
                    let _ = Self::remove_stage3_active_sublimation(
                        sublimation_kg_m2 / STAGE3_RHO_WATER_KG_M3,
                        &mut layers,
                        &mut cold_content_by_layer,
                        &mut removal_active_count,
                    );
                }
                let deposition_kg_m2 = surface.shadow_vapor_mass_exchange_kg_m2.max(0.0);
                if deposition_kg_m2 > 0.0 && !layers.is_empty() {
                    let deposition_swe_m = deposition_kg_m2 / STAGE3_RHO_WATER_KG_M3;
                    layers[0].mass_swe_m += deposition_swe_m;
                    layers[0].thickness_m = layers[0].mass_swe_m * STAGE3_RHO_WATER_KG_M3
                        / layers[0].density_kg_m3;
                }

                let hour = &mut summary.hourly[hour_index];
                let weight = substep_seconds / STAGE3_SECONDS_PER_HOUR;
                hour.shadow_sensible_flux_w_m2 += surface.shadow_sensible_flux_w_m2 * weight;
                hour.shadow_latent_flux_w_m2 += surface.shadow_latent_flux_w_m2 * weight;
                hour.shadow_advected_flux_w_m2 += surface.shadow_advected_flux_w_m2 * weight;
                hour.shadow_complete_energy_j_m2 += q_complete_j_m2;
                hour.shadow_vapor_mass_exchange_kg_m2 +=
                    surface.shadow_vapor_mass_exchange_kg_m2;
                hour.shadow_cold_required_j_m2 += cold_required_j_m2;
                hour.shadow_cold_energy_change_j_m2 += cold_energy_change_j_m2;
                hour.shadow_excess_energy_j_m2 += excess_energy_j_m2;
                hour.shadow_ice_available_kg_m2 = ice_available_kg_m2;
                hour.shadow_sublimation_kg_m2 += sublimation_kg_m2;
                hour.shadow_melt_kg_m2 += melt_kg_m2;
                hour.shadow_unallocated_after_exhaustion_j_m2 += unallocated_j_m2;
                hour.shadow_energy_closure_residual_j_m2 += closure_residual_j_m2;
                hour.shadow_complete_carrier_evaluated = true;
                summary.complete_energy_j_m2 += q_complete_j_m2;
                summary.cold_energy_change_j_m2 += cold_energy_change_j_m2;
                summary.excess_energy_j_m2 += excess_energy_j_m2;
                summary.sublimation_kg_m2 += sublimation_kg_m2;
                summary.melt_kg_m2 += melt_kg_m2;
                summary.unallocated_after_exhaustion_j_m2 += unallocated_j_m2;
                summary.maximum_energy_closure_residual_j_m2 = summary
                    .maximum_energy_closure_residual_j_m2
                    .max(closure_residual_j_m2.abs());
                elapsed_seconds += substep_seconds;
            }
        }
        Ok(summary)
    }

    pub(super) fn stage3_hourly_surface_energy(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        interval: Stage3SurfaceInterval,
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<Stage3HourlySurfaceEnergy, Wb11HydrologyKernelGuardError> {
        let Stage3SurfaceInterval {
            surface_temperature_c,
            snow_depth_m,
            snow_density_kg_m3,
            duration_seconds,
        } = interval;
        let albedo_value = inputs
            .snow_albedo_state
            .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
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
        let incoming_w_m2 = hourly.radiation_mj_m2 * 1_000_000.0 / STAGE3_SECONDS_PER_HOUR;
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
        let mut diagnostics = capture.is_verbose().then(|| {
            DirectSnowSurfaceEnergyHourDiagnostics {
                surface_temperature_c,
                net_shortwave_w_m2: shortwave.as_watts_per_square_meter(),
                ..DirectSnowSurfaceEnergyHourDiagnostics::zero()
            }
        });
        if inputs.surface_energy_options.longwave_model
            == SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1
        {
            let air_temperature = Self::stage3_temperature(phase_class, hourly.air_temperature_c)?;
            let surface_temperature =
                Self::stage3_temperature(phase_class, surface_temperature_c)?;
            let actual_vapor_pressure =
                saturation_vapor_pressure_snobal_pa(Self::stage3_temperature(
                    phase_class,
                    inputs.dewpoint_c,
                )?)
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
                canopy_cover: FractionUnitInterval::try_new(inputs.canopy_cover_fraction)
                    .map_err(|_| {
                        Self::stage3_domain_error(
                            phase_class,
                            "snow.canopy_cover_fraction",
                            inputs.canopy_cover_fraction,
                            Some(0.0),
                            Some(1.0),
                        )
                    })?,
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
            )?
            * (duration_seconds / STAGE3_SECONDS_PER_HOUR);
            sublimation_m = sublimation_m
            .min(snow_depth_m * snow_density_kg_m3 / STAGE3_RHO_WATER_KG_M3);
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
        if inputs.surface_energy_options.complete_carrier_shadow {
            if inputs.surface_energy_options.longwave_model
                != SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1
            {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_shadow_requires_complete_longwave",
                    0.0,
                    Some(1.0),
                    Some(1.0),
                ));
            }
            let geometry = inputs.surface_energy_options.turbulent_geometry;
            let air_temperature = Self::stage3_temperature(phase_class, hourly.air_temperature_c)?;
            let surface_temperature =
                Self::stage3_temperature(phase_class, surface_temperature_c)?;
            let air_vapor_pressure = saturation_vapor_pressure_snobal_pa(
                Self::stage3_temperature(phase_class, inputs.dewpoint_c)?,
            )
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
                    Self::stage3_domain_error(
                        phase_class,
                        symbol,
                        value,
                        Some(0.0),
                        None,
                    )
                })
            };
            let turbulent = turbulent_fluxes_monin_obukhov(TurbulentFluxInputs {
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
                options: TurbulentTransferOptions::default(),
            })
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_shadow_turbulent_flux",
                    inputs.wind_m_s,
                    Some(0.0),
                    None,
                )
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
            let rain_mass_flux =
                hourly.rain_m * STAGE3_RHO_WATER_KG_M3 / STAGE3_SECONDS_PER_HOUR;
            let snow_mass_flux = hourly.snowfall_m * 0.1 * STAGE3_RHO_WATER_KG_M3
                / STAGE3_SECONDS_PER_HOUR;
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
                + turbulent.sensible_heat.as_watts_per_square_meter()
                + turbulent.latent_heat.as_watts_per_square_meter()
                + advected.as_watts_per_square_meter();
            if let Some(diagnostics) = diagnostics.as_mut() {
                diagnostics.shadow_sensible_flux_w_m2 =
                    turbulent.sensible_heat.as_watts_per_square_meter();
                diagnostics.shadow_latent_flux_w_m2 =
                    turbulent.latent_heat.as_watts_per_square_meter();
                diagnostics.shadow_advected_flux_w_m2 = advected.as_watts_per_square_meter();
                diagnostics.shadow_complete_energy_j_m2 =
                    shadow_surface_flux_w_m2 * duration_seconds;
                diagnostics.shadow_vapor_mass_exchange_kg_m2 =
                    turbulent.mass_flux.as_kilograms_per_square_meter_second()
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
        })
    }

}
