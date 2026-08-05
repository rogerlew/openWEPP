const SNOWFROST_STAGE2_INSULATION_MODEL_ENV: &str =
    "OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL";

impl DirectProductionSnowFrostAuthority {
    fn initial_snow_lane_state(&self) -> DirectSnowLaneState {
        DirectSnowLaneState::from_runtime_values(
            self.snow_runtime_swe_m,
            self.snow_runtime_depth_m,
            self.snow_runtime_density_kg_m3,
            self.snow_runtime_settle_day_count,
        )
    }

    fn current_snow_lane_state(
        &self,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
    ) -> DirectSnowLaneState {
        let lane_state = lane.winter_column.snow.clone();
        if lane_state.has_runtime_state() {
            lane_state
        } else {
            self.initial_snow_lane_state()
        }
    }

    fn snow_state_projected(&self, snow_lane_state: &DirectSnowLaneState) -> bool {
        self.snow_controls_projected || snow_lane_state.has_runtime_state()
    }

    fn initial_frost_lane_state(&self) -> DirectFrostLaneState {
        let mut state = DirectFrostLaneState::zero();
        state.active_frost_coupling = self.frost_active;
        state.dfrost_m = self.frost_runtime_depth_m;
        state.frdp_m = self.frost_runtime_depth_m;
        state.ws_frz_m = self.frost_runtime_frozen_water_m;
        state.frwatc_frozen_water_after_m = self.frost_runtime_frozen_water_m;
        state
    }

    fn current_frost_lane_state(
        &self,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
    ) -> DirectFrostLaneState {
        let lane_state = lane.winter_column.frost.clone();
        if lane_state.has_runtime_state() {
            lane_state
        } else {
            self.initial_frost_lane_state()
        }
    }

    fn active_forcing(
        hyetograph_rainfall_m: f64,
        runtime_swe_m: f64,
    ) -> Result<bool, HillslopeCliError> {
        if !hyetograph_rainfall_m.is_finite() || hyetograph_rainfall_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production active snow guard requires finite nonnegative rainfall, observed {hyetograph_rainfall_m}"
                ),
            });
        }
        if !runtime_swe_m.is_finite() || runtime_swe_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production active snow guard requires finite nonnegative SWE, observed {runtime_swe_m}"
                ),
            });
        }
        Ok(hyetograph_rainfall_m > 1.0e-12 || runtime_swe_m > 1.0e-12)
    }

    #[allow(clippy::too_many_arguments)]
    fn frost_day_context(
        &self,
        climate_request: &HillslopeClimateRuntimeRequest,
        day_index: usize,
        day: &ClimateDayProjection,
        lane_index: usize,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
        forcing: &HillslopeDirectClimateDayForcing,
        snow_lane_state: &DirectSnowLaneState,
        winter_hourly_geometry: DirectProductionWinterHourlyGeometry,
        clear_no_final_hydrology_layers: bool,
        residue_depth_m_override: Option<f64>,
        canopy_height_m_override: Option<f64>,
    ) -> Result<Option<DirectProductionFrostDayContext>, HillslopeCliError> {
        let frost_lane_state = self.current_frost_lane_state(lane);
        let frost_runtime_depth_m = frost_lane_state.dfrost_m;
        let frost_runtime_frozen_water_m = frost_lane_state.ws_frz_m;
        let should_project = frost_lane_state.has_runtime_state()
            || self.frost_active
            || self.active_frost_forcing(
                forcing,
                frost_runtime_depth_m,
                frost_runtime_frozen_water_m,
            )?;
        if !should_project {
            return Ok(None);
        }
        let Some(typed_authority) = self.frost_typed_authority.as_ref() else {
            return Err(direct_production_executor_blocked(format!(
                "direct production active frost requires frost controls for lane {}",
                lane_index + 1
            )));
        };
        let frost_hourly = self.frost_hourly_forcing(
            climate_request,
            day_index,
            snow_lane_state,
            frost_runtime_depth_m,
            frost_runtime_frozen_water_m,
            winter_hourly_geometry,
        )?;
        let typed_context = DirectProductionFrostTypedComputeContext {
            lane_index,
            lane,
            day,
            forcing,
            snow_lane_state,
            frost_lane_state: &frost_lane_state,
            typed_authority,
            residue_depth_m_override,
            canopy_height_m_override,
            hourly: frost_hourly,
        };
        let compute_inputs = Self::typed_winter_frost_compute_inputs(&typed_context)?;
        let frost_outcome =
            Self::compute_typed_winter_frost_outcome(&typed_context, &compute_inputs)?;
        if let Some(trace_path) = openwepp_hillslope_orchestrator::wp2_frost_pair_trace_path() {
            openwepp_hillslope_orchestrator::write_wp2_frost_pair_trace(
                trace_path,
                "builder",
                lane_index,
                day_index,
                lane.water.soil_water_m,
                lane.subsurface_layers.iter().map(|layer| layer.theta_m).sum(),
                lane.subsurface_layers
                    .iter()
                    .map(|layer| layer.frozen_water_m)
                    .sum(),
                frost_lane_state.dfrost_m,
                frost_lane_state.ws_frz_m,
                &frost_outcome,
            );
        }
        let target_soil_water_m = direct_production_lane_soil_water(lane, lane_index)?;
        let hydrology_layers = direct_production_same_day_frost_hydrology_layers(
            lane_index,
            &lane.subsurface_layers,
            &frost_outcome,
            target_soil_water_m,
            clear_no_final_hydrology_layers,
        )?;
        Ok(Some(DirectProductionFrostDayContext {
            compute_inputs,
            frozen_infiltration_capacity_m_s: frost_outcome.infcap_frz_m_s,
            storage_liquid_delta_m: direct_production_frost_storage_liquid_delta(&frost_outcome),
            layer_carry_projection: self.frost_layer_carry_projection.clone(),
            hydrology_layers,
            frost_outcome,
        }))
    }

    fn frost_hourly_forcing(
        &self,
        climate_request: &HillslopeClimateRuntimeRequest,
        day_index: usize,
        snow_lane_state: &DirectSnowLaneState,
        frost_runtime_depth_m: f64,
        frost_runtime_frozen_water_m: f64,
        winter_hourly_geometry: DirectProductionWinterHourlyGeometry,
    ) -> Result<
        [DirectFrostHourlyForcing;
            openwepp_hillslope_orchestrator::DIRECT_WINTER_HOURLY_FORCING_COUNT],
        HillslopeCliError,
    > {
        let hourly = climate_request
            .direct_winter_hourly_forcing(
                day_index,
                DirectWinterHourlyContext {
                    snow_runtime_swe_m: snow_lane_state.runtime_swe_m,
                    frost_runtime_depth_m,
                    frost_runtime_frozen_water_m,
                    frost_file_present: self.frost_file_present,
                    frost_wint_red_enabled: self.frost_wint_red_enabled,
                    avg_slope: winter_hourly_geometry.avg_slope,
                    azimuth: winter_hourly_geometry.azimuth,
                    snow_rst_c: self.snow_rst_c,
                    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel::LegacyRst,
                },
            )
            .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production frost hourly forcing failed: {source}"
                ),
            })?
            .ok_or_else(|| {
                direct_production_executor_blocked(format!(
                    "direct production active frost requires typed winter hourly forcing for day {}",
                    day_index + 1
                ))
            })?;
        let mut frost_hourly =
            [DirectFrostHourlyForcing::zero(); openwepp_hillslope_orchestrator::DIRECT_WINTER_HOURLY_FORCING_COUNT];
        for (index, hourly) in hourly.into_iter().enumerate() {
            frost_hourly[index] = DirectFrostHourlyForcing {
                radiation_mj_m2: hourly.radiation_mj_m2,
                air_temperature_c: hourly.air_temperature_c,
                cloud_fraction: hourly.cloud_fraction,
            };
        }
        Ok(frost_hourly)
    }

    fn typed_winter_frost_compute_inputs(
        context: &DirectProductionFrostTypedComputeContext<'_>,
    ) -> Result<DirectWinterFrostComputeInputs, HillslopeCliError> {
        Ok(DirectWinterFrostComputeInputs {
            controls: context.typed_authority.controls,
            thermal: Self::typed_winter_frost_thermal_inputs(context)?,
            theta_residual: context.typed_authority.theta_residual,
            theta_field_capacity: context.typed_authority.theta_field_capacity,
            soil_conductivity_m_s: context.typed_authority.soil_conductivity_m_s,
            layer_bulk_density_kg_m3: context.typed_authority.layer_bulk_density_kg_m3.clone(),
            hourly: context.hourly,
        })
    }

    fn typed_winter_frost_thermal_inputs(
        context: &DirectProductionFrostTypedComputeContext<'_>,
    ) -> Result<DirectFrostThermalInputs, HillslopeCliError> {
        let (snow_depth_m, snow_density_kg_m3) =
            Self::snow_frost_insulation_depth_density(context)?;
        let residue_depth_m = context
            .residue_depth_m_override
            .unwrap_or(context.typed_authority.residue_depth_m);
        if !residue_depth_m.is_finite() || residue_depth_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_frost_residue_depth",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production frost residue depth must be finite and nonnegative, observed {residue_depth_m}"
                ),
            });
        }
        let canopy_height_m = context
            .canopy_height_m_override
            .unwrap_or(context.typed_authority.canopy_height_m);
        if !canopy_height_m.is_finite() || canopy_height_m < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_frost_canopy_height",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production frost canopy height must be finite and nonnegative, observed {canopy_height_m}"
                ),
            });
        }
        Ok(DirectFrostThermalInputs {
            snow_depth_m,
            snow_density_kg_m3,
            residue_depth_m,
            wind_m_s: context.forcing.vwind_m_s,
            albedo: context.typed_authority.albedo,
            canopy_height_m,
            random_roughness_m: context.typed_authority.random_roughness_m,
            day_of_year: f64::from(context.day.julian_day),
            seasonal_temperature_curve: context.typed_authority.seasonal_temperature_curve,
        })
    }

    fn snow_frost_insulation_depth_density(
        context: &DirectProductionFrostTypedComputeContext<'_>,
    ) -> Result<(f64, f64), HillslopeCliError> {
        match direct_production_snow_frost_insulation_model()? {
            DirectProductionSnowFrostInsulationModel::BulkDepthDensity => Ok((
                context.snow_lane_state.runtime_depth_m,
                context.snow_lane_state.runtime_density_kg_m3,
            )),
            DirectProductionSnowFrostInsulationModel::LayeredResistanceV1 => {
                layered_snow_frost_insulation_depth_density(
                    context.snow_lane_state,
                    context.typed_authority.controls.ksnowf,
                )
            }
        }
    }

    fn compute_typed_winter_frost_outcome(
        context: &DirectProductionFrostTypedComputeContext<'_>,
        compute_inputs: &DirectWinterFrostComputeInputs,
    ) -> Result<DirectWinterFrostPartitionOutcome, HillslopeCliError> {
        let soil_conductivity_m_s =
            direct_production_typed_frost_soil_conductivity(
                context.typed_authority,
                &context.lane.subsurface_layers,
            )?;
        let layers = direct_production_frost_layer_inputs(
            context.lane_index,
            &context.lane.subsurface_layers,
            &context.typed_authority.layer_bulk_density_kg_m3,
        )?;
        let profile_depth_m = context
            .lane
            .subsurface_layers
            .iter()
            .map(|layer| layer.depth_m)
            .sum::<f64>();
        Wb11HydrologyKernel::compute_direct_winter_frost_partition(
            &DirectActiveFrostPartitionInputs {
                controls: compute_inputs.controls,
                thermal: compute_inputs.thermal,
                profile_depth_m,
                soil_water_m: context.lane.water.soil_water_m,
                theta_residual: compute_inputs.theta_residual,
                theta_field_capacity: compute_inputs.theta_field_capacity,
                soil_conductivity_m_s,
                prior_state: direct_production_frost_prior_state_input(context.frost_lane_state),
                layers,
                hourly: compute_inputs.hourly,
            },
        )
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct production typed active frost partition failed: {source}"
            ),
        })
    }

    fn active_frost_forcing(
        &self,
        forcing: &HillslopeDirectClimateDayForcing,
        frost_runtime_depth_m: f64,
        frost_runtime_frozen_water_m: f64,
    ) -> Result<bool, HillslopeCliError> {
        for (symbol, value) in [
            ("frost.runtime_dfrost", frost_runtime_depth_m),
            ("frost.runtime_ws_frz", frost_runtime_frozen_water_m),
            ("tmax", forcing.tmax_c),
            ("tmin", forcing.tmin_c),
        ] {
            if !value.is_finite() {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct production active frost guard requires finite {symbol}, observed {value}"
                    ),
                });
            }
        }
        if frost_runtime_depth_m > 1.0e-12 || frost_runtime_frozen_water_m > 1.0e-12 {
            return Ok(true);
        }
        Ok(self.frost_wint_red_enabled && forcing.tmin_c < 0.0)
    }

    fn snow_surface_energy_options(
        &self,
        daily_solar_radiation_mj_m2: f64,
        daily_extraterrestrial_radiation_mj_m2: f64,
        daylight: bool,
    ) -> openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions {
        openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions {
            longwave_model: self.snow_surface_longwave_model,
            sublimation_model: self.snow_surface_sublimation_model,
            daily_solar_radiation_mj_m2,
            daily_extraterrestrial_radiation_mj_m2,
            daylight,
            atmospheric_pressure_pa: self.snow_atmospheric_pressure_pa,
            turbulent_geometry:
                openwepp_hillslope_orchestrator::DirectSnowTurbulentGeometry::CLIGEN_V1,
            complete_carrier_shadow: false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn snow_liquid_partition(
        &self,
        climate_request: &HillslopeClimateRuntimeRequest,
        day_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        hyetograph_rainfall_m: f64,
        snow_lane_state: &DirectSnowLaneState,
        canopy_cover_fraction: f64,
        sturm_climate_class: Option<openwepp_hillslope_orchestrator::SnowClimateClass>,
        sturm_day_of_year: Option<f64>,
        winter_hourly_geometry: DirectProductionWinterHourlyGeometry,
        capture: openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture,
    ) -> Result<openwepp_hillslope_orchestrator::DirectSnowLiquidPartition, HillslopeCliError> {
        if !Self::active_forcing(hyetograph_rainfall_m, snow_lane_state.runtime_swe_m)? {
            return Ok(inactive_direct_snow_liquid_partition(
                self.snow_density_model,
                hyetograph_rainfall_m,
                snow_lane_state,
                capture,
            ));
        }
        let hourly = climate_request
            .direct_winter_hourly_forcing(
                day_index,
                DirectWinterHourlyContext {
                    snow_runtime_swe_m: snow_lane_state.runtime_swe_m,
                    frost_runtime_depth_m: 0.0,
                    frost_runtime_frozen_water_m: 0.0,
                    frost_file_present: false,
                    frost_wint_red_enabled: false,
                    avg_slope: winter_hourly_geometry.avg_slope,
                    azimuth: winter_hourly_geometry.azimuth,
                    snow_rst_c: self.snow_rst_c,
                    snow_phase_model: self.snow_phase_model,
                },
            )
            .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production typed winter hourly forcing failed: {source}"
                ),
            })?
            .ok_or_else(|| {
                direct_production_executor_blocked(format!(
                    "direct production active snow partition requires typed winter hourly forcing for day {}",
                    day_index + 1
                ))
            })?;
        let daily_solar_radiation_mj_m2 = hourly[0].daily_solar_radiation_mj_m2;
        let daily_extraterrestrial_radiation_mj_m2 =
            hourly[0].daily_extraterrestrial_radiation_mj_m2;
        let daylight = hourly[0].daylight;
        let mut snow_hourly =
            [DirectSnowHourlyForcing::zero(); openwepp_hillslope_orchestrator::runtime_inputs::DIRECT_WINTER_HOURLY_FORCING_COUNT];
        for (index, hourly) in hourly.into_iter().enumerate() {
            snow_hourly[index] = DirectSnowHourlyForcing {
                active_precipitation_m: hourly.active_precipitation_m,
                rain_m: hourly.rain_m,
                snowfall_m: hourly.snowfall_m,
                radiation_mj_m2: hourly.radiation_mj_m2,
                air_temperature_c: hourly.air_temperature_c,
                cloud_fraction: hourly.cloud_fraction,
                phase_model: hourly.phase_model,
                rain_fraction: hourly.rain_fraction,
                snow_fraction: hourly.snow_fraction,
                hydrometeor_temperature_c: hourly.hydrometeor_temperature_c,
            };
        }
        let partition_inputs = DirectActiveSnowPartitionInputs {
            hyetograph_rainfall_m,
            rst_c: self.snow_rst_c,
            newsnw_kg_m3: self.snow_newsnw_kg_m3,
            ssd_kg_m3: self.snow_ssd_kg_m3,
            runtime_swe_m: snow_lane_state.runtime_swe_m,
            runtime_depth_m: snow_lane_state.runtime_depth_m,
            runtime_density_kg_m3: snow_lane_state.runtime_density_kg_m3,
            runtime_settle_day_count: snow_lane_state.runtime_settle_day_count,
            liquid_water_retained_m: snow_lane_state.liquid_water_retained_m,
            tmax_c: forcing.tmax_c,
            tmin_c: forcing.tmin_c,
            canopy_cover_fraction,
            wind_m_s: forcing.vwind_m_s,
            dewpoint_c: forcing.tdpt_c,
            snow_melt_model: self.snow_melt_model,
            snow_density_model: self.snow_density_model,
            stage3_liquid_routing_model: self.stage3_liquid_routing_model,
            surface_energy_options: self.snow_surface_energy_options(
                daily_solar_radiation_mj_m2,
                daily_extraterrestrial_radiation_mj_m2,
                daylight,
            ),
            sturm_climate_class,
            sturm_day_of_year,
            coe_boundary_depth_m: snow_lane_state.coe_boundary_depth_m,
            coe_boundary_density_kg_m3: snow_lane_state.coe_boundary_density_kg_m3,
            coe_boundary_settle_day_count: snow_lane_state.coe_boundary_settle_day_count,
            snow_albedo_model: None,
            snow_albedo_state: snow_lane_state.snow_albedo_state,
            snow_layers: snow_lane_state.layers.clone(),
            underlying_surface_albedo: 0.2,
            hourly: snow_hourly,
        };
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_capture(
            &partition_inputs,
            capture,
        )
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} direct production typed snow partition failed: {source}"),
        })
    }
}

fn inactive_direct_snow_liquid_partition(
    snow_density_model: openwepp_hillslope_orchestrator::SnowDensityModel,
    hyetograph_rainfall_m: f64,
    snow_lane_state: &DirectSnowLaneState,
    capture: openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture,
) -> openwepp_hillslope_orchestrator::DirectSnowLiquidPartition {
    openwepp_hillslope_orchestrator::DirectSnowLiquidPartition {
        active_snow_coupling: false,
        snow_density_model,
        snow_coupling_signed_s_m: 0.0,
        mass_transition_ledgers:
            openwepp_hillslope_orchestrator::DirectSnowMassTransitionLedgers::zero(),
        hourly_routed_melt_m: [0.0; 24],
        accumulation_m: 0.0,
        rain_retained_m: 0.0,
        post_winter_rain_m: hyetograph_rainfall_m,
        runtime_swe_after_m: snow_lane_state.runtime_swe_m,
        runtime_depth_after_m: snow_lane_state.runtime_depth_m,
        runtime_density_after_kg_m3: snow_lane_state.runtime_density_kg_m3,
        runtime_settle_day_count_after: snow_lane_state.runtime_settle_day_count,
        liquid_holding_capacity_after_m: 0.0,
        liquid_water_retained_after_m: snow_lane_state.liquid_water_retained_m,
        liquid_water_released_m: 0.0,
        sublimation_m: 0.0,
        coe_boundary_depth_after_m: snow_lane_state.coe_boundary_depth_m,
        coe_boundary_density_after_kg_m3: snow_lane_state.coe_boundary_density_kg_m3,
        coe_boundary_settle_day_count_after: snow_lane_state.coe_boundary_settle_day_count,
        density_swe_identity_residual_m: 0.0,
        density_unbounded_swe_residual_m: 0.0,
        density_process_diagnostics:
            openwepp_hillslope_orchestrator::SnowDensityProcessDiagnostics::default(),
        verbose_diagnostics: capture.is_verbose().then(|| Box::new(
            openwepp_hillslope_orchestrator::DirectSnowVerboseDiagnostics {
                accumulation_melt:
                    openwepp_hillslope_orchestrator::DirectSnowAccumulationMeltDiagnostics::default(),
                stage3:
                    openwepp_hillslope_orchestrator::DirectSnowStage3Diagnostics::disabled(),
            },
        )),
        snow_albedo_state_after: snow_lane_state.snow_albedo_state,
        snow_layers_after: snow_lane_state.layers.clone(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectProductionSnowFrostInsulationModel {
    BulkDepthDensity,
    LayeredResistanceV1,
}

fn direct_production_snow_frost_insulation_model(
) -> Result<DirectProductionSnowFrostInsulationModel, HillslopeCliError> {
    match std::env::var(SNOWFROST_STAGE2_INSULATION_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "bulk_depth_density" => {
                Ok(DirectProductionSnowFrostInsulationModel::BulkDepthDensity)
            }
            "layered_resistance_v1" => {
                Ok(DirectProductionSnowFrostInsulationModel::LayeredResistanceV1)
            }
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_frost_insulation_model",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {SNOWFROST_STAGE2_INSULATION_MODEL_ENV} must be bulk_depth_density, layered_resistance_v1, or empty default, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => {
            Ok(DirectProductionSnowFrostInsulationModel::BulkDepthDensity)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_frost_insulation_model",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOWFROST_STAGE2_INSULATION_MODEL_ENV} must be UTF-8"
            ),
        }),
    }
}

fn layered_snow_frost_insulation_depth_density(
    snow: &DirectSnowLaneState,
    ksnowf: f64,
) -> Result<(f64, f64), HillslopeCliError> {
    const SNOW_DENSITY_CAP_KG_M3: f64 = 522.0;
    const SNOW_LAYER_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;

    if snow.runtime_depth_m <= SNOW_LAYER_CLOSURE_TOLERANCE_M
        && snow.runtime_swe_m <= SNOW_LAYER_CLOSURE_TOLERANCE_M
    {
        return Ok((snow.runtime_depth_m.max(0.0), 0.0));
    }
    if snow.layers.is_empty() {
        return Err(stage2_insulation_error(
            "layered_resistance_v1 requires nonempty prior-day snow layers when snow is present",
        ));
    }
    let layer_depth_m = snow
        .layers
        .iter()
        .map(|layer| require_stage2_layer_value("layer.thickness_m", layer.thickness_m))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<f64>();
    let layer_swe_m = snow
        .layers
        .iter()
        .map(|layer| require_stage2_layer_value("layer.mass_swe_m", layer.mass_swe_m))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<f64>();
    if (layer_depth_m - snow.runtime_depth_m).abs() > SNOW_LAYER_CLOSURE_TOLERANCE_M {
        return Err(stage2_insulation_error(&format!(
            "layered_resistance_v1 layer depth {layer_depth_m} m does not reconstruct runtime snow depth {} m",
            snow.runtime_depth_m
        )));
    }
    if (layer_swe_m - snow.runtime_swe_m).abs() > SNOW_LAYER_CLOSURE_TOLERANCE_M {
        return Err(stage2_insulation_error(&format!(
            "layered_resistance_v1 layer SWE {layer_swe_m} m does not reconstruct runtime snow SWE {} m",
            snow.runtime_swe_m
        )));
    }

    let mut resistance_m2_k_w = 0.0;
    for layer in &snow.layers {
        let thickness_m = require_stage2_layer_value("layer.thickness_m", layer.thickness_m)?;
        let density_kg_m3 =
            require_stage2_layer_value("layer.density_kg_m3", layer.density_kg_m3)?;
        if thickness_m <= SNOW_LAYER_CLOSURE_TOLERANCE_M {
            continue;
        }
        if density_kg_m3 <= 0.0 || density_kg_m3 > SNOW_DENSITY_CAP_KG_M3 {
            return Err(stage2_insulation_error(&format!(
                "layered_resistance_v1 layer density {density_kg_m3} kg m^-3 is outside 0..={SNOW_DENSITY_CAP_KG_M3}"
            )));
        }
        let conductivity_w_m_k = sturm1997_snow_conductivity_w_m_k(density_kg_m3, ksnowf)?;
        resistance_m2_k_w += thickness_m / conductivity_w_m_k;
    }
    if resistance_m2_k_w <= 0.0 || !resistance_m2_k_w.is_finite() {
        return Err(stage2_insulation_error(&format!(
            "layered_resistance_v1 computed invalid snow resistance {resistance_m2_k_w} m^2 K W^-1"
        )));
    }
    let effective_conductivity_w_m_k = snow.runtime_depth_m / resistance_m2_k_w;
    let effective_density_kg_m3 =
        invert_sturm1997_snow_density_kg_m3(effective_conductivity_w_m_k, ksnowf)?;
    Ok((snow.runtime_depth_m, effective_density_kg_m3))
}

fn require_stage2_layer_value(symbol: &str, value: f64) -> Result<f64, HillslopeCliError> {
    if !value.is_finite() || value < 0.0 {
        return Err(stage2_insulation_error(&format!(
            "layered_resistance_v1 {symbol} must be finite and nonnegative, observed {value}"
        )));
    }
    Ok(value)
}

fn sturm1997_snow_conductivity_w_m_k(
    density_kg_m3: f64,
    ksnowf: f64,
) -> Result<f64, HillslopeCliError> {
    if !density_kg_m3.is_finite() || density_kg_m3 < 0.0 {
        return Err(stage2_insulation_error(&format!(
            "Sturm 1997 snow conductivity density must be finite and nonnegative, observed {density_kg_m3}"
        )));
    }
    if !ksnowf.is_finite() || ksnowf <= 0.0 {
        return Err(stage2_insulation_error(&format!(
            "Sturm 1997 snow conductivity ksnowf must be finite and positive, observed {ksnowf}"
        )));
    }
    let density_g_cm3 = density_kg_m3 / 1_000.0;
    let base = if density_kg_m3 < 156.0 {
        0.023 + (0.234 * density_g_cm3)
    } else {
        0.138 - (1.01 * density_g_cm3) + (3.233 * density_g_cm3.powi(2))
    };
    let conductivity = base * ksnowf;
    if conductivity.is_finite() && conductivity > 0.0 {
        Ok(conductivity)
    } else {
        Err(stage2_insulation_error(&format!(
            "Sturm 1997 snow conductivity is invalid for density {density_kg_m3} kg m^-3 and ksnowf {ksnowf}: {conductivity}"
        )))
    }
}

fn invert_sturm1997_snow_density_kg_m3(
    target_conductivity_w_m_k: f64,
    ksnowf: f64,
) -> Result<f64, HillslopeCliError> {
    const SNOW_DENSITY_CAP_KG_M3: f64 = 522.0;
    const CONDUCTIVITY_TOLERANCE_W_M_K: f64 = 1.0e-12;
    if !target_conductivity_w_m_k.is_finite() || target_conductivity_w_m_k <= 0.0 {
        return Err(stage2_insulation_error(&format!(
            "layered_resistance_v1 target conductivity must be finite and positive, observed {target_conductivity_w_m_k}"
        )));
    }
    let min_conductivity = sturm1997_snow_conductivity_w_m_k(0.0, ksnowf)?;
    let max_conductivity = sturm1997_snow_conductivity_w_m_k(SNOW_DENSITY_CAP_KG_M3, ksnowf)?;
    if target_conductivity_w_m_k < min_conductivity - CONDUCTIVITY_TOLERANCE_W_M_K
        || target_conductivity_w_m_k > max_conductivity + CONDUCTIVITY_TOLERANCE_W_M_K
    {
        return Err(stage2_insulation_error(&format!(
            "layered_resistance_v1 target conductivity {target_conductivity_w_m_k} W m^-1 K^-1 is outside the 0..={SNOW_DENSITY_CAP_KG_M3} kg m^-3 Sturm 1997 range [{min_conductivity}, {max_conductivity}]"
        )));
    }
    let mut low = 0.0;
    let mut high = SNOW_DENSITY_CAP_KG_M3;
    for _ in 0..80 {
        let mid = f64::midpoint(low, high);
        let conductivity = sturm1997_snow_conductivity_w_m_k(mid, ksnowf)?;
        if conductivity < target_conductivity_w_m_k {
            low = mid;
        } else {
            high = mid;
        }
    }
    Ok(f64::midpoint(low, high))
}

fn stage2_insulation_error(detail: &str) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_snow_frost_insulation_model",
        detail: format!("{SIMOUT_GUARD_ID} {detail}"),
    }
}
