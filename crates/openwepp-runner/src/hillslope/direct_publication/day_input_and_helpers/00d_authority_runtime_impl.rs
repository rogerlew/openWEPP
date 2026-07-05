impl DirectProductionLaneDayInputAuthority {
    fn percolation_inputs(
        &self,
        lane_index: usize,
        lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
        layers: &[DirectSubsurfaceLayerState],
    ) -> Result<DirectPercolationInputs, HillslopeCliError> {
        direct_production_validate_layers(lane_index, layers)?;
        let mut inputs = self.percolation.clone();
        inputs.soil_water_initial_m = direct_production_lane_soil_water(lane, lane_index)?;
        inputs.layers.clear();
        inputs.layers.extend_from_slice(layers);
        Ok(inputs)
    }

    fn subsurface_inputs(
        &self,
        lane_index: usize,
        layers: &[DirectSubsurfaceLayerState],
    ) -> Result<DirectSubsurfaceComputeInputs, HillslopeCliError> {
        direct_production_validate_layers(lane_index, layers)?;
        let mut inputs = self.subsurface.clone();
        inputs.soil_depth_m = layers.iter().map(|layer| layer.depth_m).sum::<f64>();
        inputs.layers = layers.iter().cloned().map(Into::into).collect();
        Ok(inputs)
    }

    fn hydrology_projection_inputs(
        &self,
        layers: &[DirectSubsurfaceLayerState],
    ) -> DirectHydrologyProjectionInputs {
        let mut inputs = self.hydrology_projection;
        inputs.frozen_soil_water_m = layers.iter().map(|layer| layer.frozen_water_m).sum();
        inputs.frost_depth_m = direct_production_frost_depth_m(layers);
        inputs
    }
}

impl DirectProductionResidueCoverAuthority {
    fn initial_state(self) -> DirectProductionResidueCoverState {
        DirectProductionResidueCoverState {
            surface_residue_kg_m2: self.initial_surface_residue_kg_m2,
            root_residue_kg_m2: self.initial_root_residue_kg_m2,
            interrill_ground_residue_kg_m2: self.initial_interrill_ground_kg_m2,
            rill_ground_residue_kg_m2: self.initial_rill_ground_kg_m2,
            pending_surface_litter_kg_m2: 0.0,
            residue_depth_m: self.initial_surface_residue_kg_m2
                * self.residue_depth_conversion_m_per_kg_m2,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn project_day(
        self,
        growth: &DirectProductionGrowthAuthority,
        day: &ClimateDayProjection,
        simulation_year: i32,
        ofe_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        state_before: DirectProductionResidueCoverState,
        plant_state_before: DirectGrowthStateSurface,
        plant_state_after: DirectGrowthStateSurface,
        water_stress: f64,
    ) -> Result<DirectProductionResidueCoverProjection, HillslopeCliError> {
        let runtime_year =
            direct_growth_i32_to_usize("simulation_year", simulation_year, 1, usize::MAX)?;
        let ofe_index_valid = direct_growth_validate_usize("ofe_index", ofe_index, 1, usize::MAX)?;
        let runtime_day = direct_growth_u16_to_usize("day", day.julian_day, 1, 366)?;
        let active_crop = if growth.active {
            growth
                .active_crop(runtime_year, runtime_day, ofe_index_valid)?
                .map(|selection| selection.crop)
        } else {
            None
        };
        let surface_litter_projection = direct_production_surface_litter_projection(
            active_crop,
            runtime_day,
            state_before,
            plant_state_before,
            plant_state_after,
        )?;
        let decomposition_inputs = self.decomposition_inputs(
            growth,
            day,
            simulation_year,
            ofe_index,
            forcing,
            state_before,
            surface_litter_projection.surface_litter_input_kg_m2,
            water_stress,
        )?;
        let decomposition_state = decomposition_inputs
            .compute_state()
            .map_err(|source| direct_publication_runtime_error(&source))?;
        let state_after = DirectProductionResidueCoverState {
            surface_residue_kg_m2: decomposition_state.surface_residue_kg_m2,
            root_residue_kg_m2: decomposition_state.root_residue_kg_m2,
            interrill_ground_residue_kg_m2: decomposition_state.interrill_ground_residue_kg_m2,
            rill_ground_residue_kg_m2: decomposition_state.rill_ground_residue_kg_m2,
            pending_surface_litter_kg_m2: surface_litter_projection
                .pending_surface_litter_after_kg_m2,
            residue_depth_m: decomposition_state.residue_depth_m,
        };
        Ok(DirectProductionResidueCoverProjection {
            decomposition_inputs,
            residue_partition_inputs: DirectResiduePartitionInputs {
                standing_residue_kg_m2: 0.0,
                flat_residue_offset_kg_m2: 0.0,
                buried_residue_kg_m2: 0.0,
                cover_fraction: 0.0,
            },
            state_before,
            state_after,
            surface_litter_input_kg_m2: surface_litter_projection.surface_litter_input_kg_m2,
            pending_surface_litter_after_kg_m2: surface_litter_projection
                .pending_surface_litter_after_kg_m2,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn decomposition_inputs(
        self,
        growth: &DirectProductionGrowthAuthority,
        day: &ClimateDayProjection,
        simulation_year: i32,
        ofe_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        state_before: DirectProductionResidueCoverState,
        surface_litter_input_kg_m2: f64,
        water_stress: f64,
    ) -> Result<DirectDecompositionInputs, HillslopeCliError> {
        if !growth.active {
            return Ok(DirectDecompositionInputs::zero());
        }
        let runtime_year =
            direct_growth_i32_to_usize("simulation_year", simulation_year, 1, usize::MAX)?;
        let ofe_index = direct_growth_validate_usize("ofe_index", ofe_index, 1, usize::MAX)?;
        let runtime_day = direct_growth_u16_to_usize("day", day.julian_day, 1, 366)?;
        let Some(selection) = growth.active_crop(runtime_year, runtime_day, ofe_index)? else {
            return Ok(DirectDecompositionInputs {
                surface_residue_seed_kg_m2: state_before.surface_residue_kg_m2,
                root_residue_seed_kg_m2: state_before.root_residue_kg_m2,
                interrill_ground_seed_kg_m2: state_before.interrill_ground_residue_kg_m2,
                rill_ground_seed_kg_m2: state_before.rill_ground_residue_kg_m2,
                residue_cover_factor: self.residue_cover_factor,
                surface_litter_input_kg_m2,
                residue_depth_conversion_m_per_kg_m2: self
                    .residue_depth_conversion_m_per_kg_m2,
                ..DirectDecompositionInputs::zero()
            });
        };
        let runtime_day = direct_growth_usize_to_u16("day", runtime_day)?;
        let slot_index = direct_growth_usize_to_u16("slot_index", selection.slot_index)?;
        let crop_slot_index =
            direct_growth_usize_to_u16("crop_slot_index", selection.crop_slot_index)?;
        let active_context = match selection.crop.imngmt {
            1 | 3 => DirectDecompositionActiveContext::AnnualOrFallow {
                active_slot_index: slot_index,
                active_crop_slot_index: crop_slot_index,
                runtime_day_of_year: runtime_day,
            },
            2 => DirectDecompositionActiveContext::Perennial {
                active_slot_index: slot_index,
                active_crop_slot_index: crop_slot_index,
                runtime_day_of_year: runtime_day,
            },
            _ => {
                return Err(direct_growth_failure(format!(
                    "unsupported direct production decomposition management class {}",
                    selection.crop.imngmt
                )));
            }
        };
        Ok(DirectDecompositionInputs {
            active_context,
            active_action: DirectDecompositionAction::None,
            residue_type_selector: self.residue_type_selector,
            surface_residue_seed_kg_m2: state_before.surface_residue_kg_m2,
            root_residue_seed_kg_m2: state_before.root_residue_kg_m2,
            interrill_ground_seed_kg_m2: state_before.interrill_ground_residue_kg_m2,
            rill_ground_seed_kg_m2: state_before.rill_ground_residue_kg_m2,
            residue_cover_factor: self.residue_cover_factor,
            surface_litter_input_kg_m2,
            residue_depth_conversion_m_per_kg_m2: self.residue_depth_conversion_m_per_kg_m2,
            temperature_max_c: forcing.tmax_c,
            temperature_min_c: forcing.tmin_c,
            precipitation_m: forcing.prcp_m,
            water_stress_fraction: water_stress,
            surface_decomposition_rate: selection.crop.surface_decomposition_rate(),
            root_decomposition_rate: selection.crop.orater,
            burn_surface_fraction: 0.0,
            remove_surface_fraction: 0.0,
            cut_transfer_fraction: 0.0,
            grazing_digest_fraction: 0.0,
        })
    }
}

fn direct_production_surface_litter_projection(
    active_crop: Option<&DirectProductionGrowthCropAuthority>,
    runtime_day: usize,
    residue_state_before: DirectProductionResidueCoverState,
    state_before: DirectGrowthStateSurface,
    state_after: DirectGrowthStateSurface,
) -> Result<DirectProductionSurfaceLitterProjection, HillslopeCliError> {
    let daily_litter_loss_kg_m2 =
        (state_before.live_biomass_kg_m2 - state_after.live_biomass_kg_m2).max(0.0);
    if !daily_litter_loss_kg_m2.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_residue_cover",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct production litter input must be finite, observed {daily_litter_loss_kg_m2}"
            ),
        });
    }
    let projection = match active_crop {
        Some(crop) if crop.uses_fall_litter_drop_schedule() => {
            let pending =
                residue_state_before.pending_surface_litter_kg_m2 + daily_litter_loss_kg_m2;
            if !pending.is_finite() || pending < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_production_residue_cover",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct production pending litter must be finite and nonnegative, observed {pending}"
                    ),
                });
            }
            if crop.fall_litter_drop_window_contains(runtime_day) {
                DirectProductionSurfaceLitterProjection {
                    surface_litter_input_kg_m2: pending,
                    pending_surface_litter_after_kg_m2: 0.0,
                }
            } else {
                DirectProductionSurfaceLitterProjection {
                    surface_litter_input_kg_m2: 0.0,
                    pending_surface_litter_after_kg_m2: pending,
                }
            }
        }
        _ => DirectProductionSurfaceLitterProjection {
            surface_litter_input_kg_m2: daily_litter_loss_kg_m2,
            pending_surface_litter_after_kg_m2: 0.0,
        },
    };
    if !projection.surface_litter_input_kg_m2.is_finite()
        || projection.surface_litter_input_kg_m2 < 0.0
        || !projection.pending_surface_litter_after_kg_m2.is_finite()
        || projection.pending_surface_litter_after_kg_m2 < 0.0
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_residue_cover",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct production litter projection must be finite and nonnegative, input={} pending={}",
                projection.surface_litter_input_kg_m2,
                projection.pending_surface_litter_after_kg_m2
            ),
        });
    }
    Ok(projection)
}

fn maybe_write_frost_residue_cover_trace(
    day_index: usize,
    lane_index: usize,
    projection: &DirectProductionResidueCoverProjection,
) -> Result<(), HillslopeCliError> {
    let Some(path) = std::env::var_os("OPENWEPP_FROST_RESIDUE_COVER_TRACE_PATH") else {
        return Ok(());
    };
    if path.is_empty() {
        return Ok(());
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_residue_cover_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed opening direct production residue-cover trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        })?;
    let line = format!(
        "{{\"schema\":\"openwepp-frost-residue-cover-trace-v1\",\
\"day_index\":{day_index},\
\"lane_index\":{lane_index},\
\"surface_residue_before_kg_m2\":{},\
\"root_residue_before_kg_m2\":{},\
\"pending_surface_litter_before_kg_m2\":{},\
\"residue_depth_before_m\":{},\
\"surface_litter_input_kg_m2\":{},\
\"surface_residue_after_kg_m2\":{},\
\"root_residue_after_kg_m2\":{},\
\"pending_surface_litter_after_kg_m2\":{},\
\"residue_depth_after_m\":{},\
\"residue_depth_conversion_m_per_kg_m2\":{},\
\"surface_decomposition_rate\":{},\
\"root_decomposition_rate\":{}}}",
        direct_production_trace_number(projection.state_before.surface_residue_kg_m2),
        direct_production_trace_number(projection.state_before.root_residue_kg_m2),
        direct_production_trace_number(projection.state_before.pending_surface_litter_kg_m2),
        direct_production_trace_number(projection.state_before.residue_depth_m),
        direct_production_trace_number(projection.surface_litter_input_kg_m2),
        direct_production_trace_number(projection.state_after.surface_residue_kg_m2),
        direct_production_trace_number(projection.state_after.root_residue_kg_m2),
        direct_production_trace_number(projection.pending_surface_litter_after_kg_m2),
        direct_production_trace_number(projection.state_after.residue_depth_m),
        direct_production_trace_number(
            projection
                .decomposition_inputs
                .residue_depth_conversion_m_per_kg_m2,
        ),
        direct_production_trace_number(projection.decomposition_inputs.surface_decomposition_rate),
        direct_production_trace_number(projection.decomposition_inputs.root_decomposition_rate),
    );
    let line = format!("{line}\n");
    std::io::Write::write_all(&mut file, line.as_bytes()).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_residue_cover_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed writing direct production residue-cover trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        }
    })
}

impl DirectProductionPeakRunoffAuthority {
    fn inputs(&self, hyetograph: Vec<DirectWb14HyetographInterval>) -> DirectPeakRunoffInputs {
        DirectPeakRunoffInputs {
            hyetograph,
            irrigation_rate_m_s: self.irrigation_rate_m_s,
            efflen_m: self.efflen_m,
            ealpha: self.ealpha,
            exponent_m: self.exponent_m,
        }
    }
}

fn direct_production_positive_frost_conductivity_limit_m_s(
    frost_infcap_m_s: Option<f64>,
    seeded_effective_conductivity_m_s: Option<f64>,
) -> Option<f64> {
    frost_infcap_m_s
        .filter(|value| *value > 0.0)
        .or(seeded_effective_conductivity_m_s)
        .filter(|value| *value > 0.0)
}

impl DirectProductionInfiltrationAuthority {
    #[allow(clippy::too_many_arguments)]
    fn inputs(
        &self,
        lane_index: usize,
        layers: &[DirectSubsurfaceLayerState],
        hyetograph: Vec<DirectWb14HyetographInterval>,
        frost_infcap_m_s: Option<f64>,
    ) -> Result<DirectInfiltrationDepressionInputs, HillslopeCliError> {
        direct_production_validate_layers(lane_index, layers)?;
        let effective_conductivity_m_s =
            if let Some(ksatadj_policy) = &self.ksatadj_policy {
                let ksatadj_effective_conductivity_m_s =
                    direct_production_ksatadj_effective_conductivity(ksatadj_policy, layers)?;
                direct_production_positive_frost_conductivity_limit_m_s(
                    frost_infcap_m_s,
                    self.effective_conductivity_m_s,
                )
                .map_or(ksatadj_effective_conductivity_m_s, |frost_limit_m_s| {
                    ksatadj_effective_conductivity_m_s.min(frost_limit_m_s)
                })
            } else {
                direct_production_positive_frost_conductivity_limit_m_s(
                    frost_infcap_m_s,
                    self.effective_conductivity_m_s,
                )
                    .or_else(|| layers.first().map(|layer| layer.conductivity_m_s))
                    .ok_or_else(|| {
                        direct_production_executor_blocked(
                            "direct production WB14 infiltration requires layer conductivity",
                        )
                    })?
            };
        let matric_potential_m = self.matric_potential_m.unwrap_or_else(|| {
            let first_layer = &layers[0];
            first_layer.depth_m * (first_layer.field_capacity_theta - first_layer.residual_theta).max(0.0)
        });
        let storage_capacity_m = direct_publication_wb14_top_storage_capacity(layers)?;
        Ok(DirectInfiltrationDepressionInputs {
            cumulative_infiltration_handoff_m: 0.0,
            depression_storage_delta_handoff_m: 0.0,
            producer_inputs: Some(DirectWb14InfiltrationProducerInputs {
            runon_hourly_supply_m: [0.0; 24],
                hyetograph,
                effective_conductivity_m_s,
                matric_potential_m,
                storage_capacity_m,
                depression_storage_capacity_m: self.depression_storage_capacity_m,
            }),
        })
    }
}

impl DirectProductionEvapotranspirationAuthority {
    #[allow(clippy::too_many_arguments)]
    fn inputs(
        &self,
        day: &ClimateDayProjection,
        forcing: &HillslopeDirectClimateDayForcing,
        stage_state: Option<DirectEvapotranspirationStageState>,
        layers: &[DirectSubsurfaceLayerState],
        climate_request: &HillslopeClimateRuntimeRequest,
    ) -> Result<DirectEvapotranspirationComputeInputs, HillslopeCliError> {
        let (et_demand_m, pmet) = if let Some(pmet_authority) = &self.pmet {
            let seed = pmet_authority.compute_seed(day, forcing, layers, self, climate_request)?;
            (
                seed.et_demand_m,
                Some(DirectEvapotranspirationPmetInputs {
                    soil_evaporation_m: seed.soil_evaporation_m,
                    plant_transpiration_m: seed.plant_transpiration_m,
                    soil_evaporation_storage_return_m: seed.soil_evaporation_storage_return_m,
                }),
            )
        } else {
            (
                self.priestley_taylor
                    .compute_demand(forcing, self.leaf_area_index, self.canopy_cover_fraction)?,
                None,
            )
        };
        Ok(DirectEvapotranspirationComputeInputs {
            et_demand_m,
            leaf_area_index: self.leaf_area_index,
            canopy_cover_fraction: self.canopy_cover_fraction,
            residue_interception_m: self.residue_interception_m,
            same_pass_infiltration_m: 0.0,
            outside_water_depth_m: 0.0,
            root_depth_m: self.root_depth_m,
            plant_tolerance: self.plant_tolerance,
            growth_context_required: false,
            stage_state: if pmet.is_some() { None } else { stage_state },
            pmet,
            pmet_compute: None,
        })
    }

    fn inputs_with_growth_surface(
        &self,
        day: &ClimateDayProjection,
        forcing: &HillslopeDirectClimateDayForcing,
        stage_state: Option<DirectEvapotranspirationStageState>,
        layers: &[DirectSubsurfaceLayerState],
        climate_request: &HillslopeClimateRuntimeRequest,
        growth_surface: DirectGrowthStateSurface,
    ) -> Result<DirectEvapotranspirationComputeInputs, HillslopeCliError> {
        let mut dynamic = self.clone();
        dynamic.apply_growth_surface(growth_surface);
        dynamic.inputs(day, forcing, stage_state, layers, climate_request)
    }

    fn apply_growth_surface(&mut self, growth_surface: DirectGrowthStateSurface) {
        self.leaf_area_index = growth_surface.leaf_area_index;
        self.canopy_cover_fraction = growth_surface.canopy_cover_fraction;
        self.root_depth_m = growth_surface.root_depth_m;
    }
}

impl DirectProductionPriestleyTaylorAuthority {
    fn compute_demand(
        &self,
        forcing: &HillslopeDirectClimateDayForcing,
        leaf_area_index: f64,
        canopy_cover_fraction: f64,
    ) -> Result<f64, HillslopeCliError> {
        if forcing.rad_ly < 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "rad must be >= 0.0 for direct production ET demand, observed {}",
                forcing.rad_ly
            )));
        }
        if !(0.0..=1.0).contains(&self.salb) {
            return Err(direct_production_executor_blocked(format!(
                "salb must be within [0,1] for direct production ET demand, observed {}",
                self.salb
            )));
        }
        let tave = 0.5 * (forcing.tmax_c + forcing.tmin_c);
        let tk = tave + 273.0;
        if tk <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "derived tk must be > 0.0 for direct production ET demand, observed {tk}"
            )));
        }
        let delta = (21.255 - 5304.0 / tk).exp() * 5304.0 / (tk * tk);
        let gamma = delta / (delta + 0.68);
        let eaj = (-0.5 * (canopy_cover_fraction + 0.1)).exp();
        let alb = if leaf_area_index > 0.0 {
            0.23 * (1.0 - eaj) + self.salb * eaj
        } else {
            self.salb
        };
        let demand_m = (0.00128 * ((forcing.rad_ly * (1.0 - alb)) / 58.3) * gamma).max(0.0);
        if !demand_m.is_finite() {
            return Err(direct_production_executor_blocked(format!(
                "derived direct production ET demand is non-finite ({demand_m})"
            )));
        }
        Ok(demand_m)
    }
}

impl DirectProductionPmetAuthority {
    #[allow(clippy::manual_midpoint, clippy::similar_names, clippy::too_many_lines)]
    fn compute_seed(
        &self,
        day: &ClimateDayProjection,
        forcing: &HillslopeDirectClimateDayForcing,
        layers: &[DirectSubsurfaceLayerState],
        et: &DirectProductionEvapotranspirationAuthority,
        climate_request: &HillslopeClimateRuntimeRequest,
    ) -> Result<DirectProductionEvappmSeed, HillslopeCliError> {
        direct_production_validate_layers(0, layers)?;
        if forcing.rad_ly < 0.0 || forcing.vwind_m_s < 0.0 {
            return Err(direct_production_executor_blocked(
                "direct production PMET requires nonnegative rad and vwind",
            ));
        }
        if self.canhgt < 0.0 || et.leaf_area_index < 0.0 || et.root_depth_m < 0.0 {
            return Err(direct_production_executor_blocked(
                "direct production PMET canopy and root controls must be nonnegative",
            ));
        }
        let tave = 0.5 * (forcing.tmax_c + forcing.tmin_c);
        let ed = saturation_vapor_pressure_kpa(forcing.tdpt_c);
        let emaxt = saturation_vapor_pressure_kpa(forcing.tmax_c);
        let emint = saturation_vapor_pressure_kpa(forcing.tmin_c);
        let ee = 0.5 * (emaxt + emint);
        if emaxt <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "derived emaxt must be > 0.0 for direct production PMET, observed {emaxt}"
            )));
        }
        let radpot = self.radpot_ly.unwrap_or_else(|| {
            legacy_sunmap_horizontal_radpot_ly(
                climate_request.direct_latitude_degrees(),
                f64::from(day.julian_day),
            )
        });
        if radpot <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "radpot must be > 0.0 for direct production PMET, observed {radpot}"
            )));
        }
        let ra = forcing.rad_ly / 23.9;
        let rso = radpot / 23.9;
        let rbo = (0.34 - 0.14 * ed.sqrt())
            * 4.9e-9
            * (((forcing.tmax_c + 273.2).powi(4) + (forcing.tmin_c + 273.2).powi(4)) / 2.0)
            * (1.35 * (ra / rso) - 0.35);
        let rn_mj_m2 = ra * 0.77 - rbo;
        let fwv_m_s = forcing.vwind_m_s * 4.87 / (67.8_f64.mul_add(10.0, -5.42)).ln();
        let dlt = 4098.0 / ((tave + 237.3) * (tave + 237.3))
            * saturation_vapor_pressure_kpa(tave);
        let pressure_base = 1.0 - 0.0065 * climate_request.direct_elevation_m() / 293.0;
        if pressure_base <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "legacy pressure base must be > 0.0 for direct production PMET, observed {pressure_base}"
            )));
        }
        let pb = 101.3 * pressure_base.powf(5.26);
        let gma = 0.000_665 * pb;
        let denominator = dlt + gma * (1.0 + 0.34 * fwv_m_s);
        if denominator <= 0.0 {
            return Err(direct_production_executor_blocked(format!(
                "direct production PMET etorc denominator must be > 0.0, observed {denominator}"
            )));
        }
        let etorc_mm =
            (0.408 * dlt * rn_mj_m2 + gma * (900.0 / (tave + 273.0)) * (ee - ed) * fwv_m_s)
                / denominator;
        let rhd_pct = ed / emaxt * 100.0;
        let height_factor = (self.canhgt / 3.0).powf(0.3);
        let kcbadj = if et.leaf_area_index > 0.0 && et.root_depth_m > 0.0 {
            self.kcb + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor
        } else {
            0.0
        };
        let kcbcon = kcbadj * (1.0 - (-0.45 * et.leaf_area_index).exp());
        let etke = if kcbadj > 0.0 {
            kcbadj * (-0.45 * et.leaf_area_index).exp()
        } else {
            1.2
        };

        let profile_depth_m = direct_production_profile_depth_m(layers)?;
        let epdp_m = 0.1_f64.min(profile_depth_m);
        let (tew_mm, rew_mm, wfevp_base_mm) =
            self.evaporation_storage_terms(layers, epdp_m)?;
        let wfevp_mm = wfevp_base_mm + et.residue_interception_m * 1_000.0;
        let etkr = if (tew_mm - wfevp_mm) <= rew_mm {
            1.0
        } else {
            let denominator = tew_mm - rew_mm;
            if denominator <= 0.0 {
                1.0
            } else {
                (wfevp_mm / denominator).powi(2)
            }
        };
        let tpdp_m = et.root_depth_m.min(profile_depth_m);
        let (taw_mm, wftrp_mm) =
            self.transpiration_storage_terms(layers, tpdp_m, wfevp_mm)?;
        let etcsc = kcbadj * etorc_mm;
        let rawpaj = self.rawp + 0.04 * (5.0 - etcsc);
        let raw_mm = rawpaj * taw_mm;
        let etksden = taw_mm - raw_mm;
        let etks = if etksden <= 0.0 || (taw_mm - wftrp_mm) <= raw_mm {
            1.0
        } else {
            wftrp_mm / etksden
        };
        let potes_m = etorc_mm * etke * 0.001;
        let es_raw_m = if potes_m > et.residue_interception_m {
            let bpotes_m = potes_m - et.residue_interception_m;
            let eaj = (-0.5 * (et.canopy_cover_fraction + 0.1)).exp();
            let kcmax = 1.2 + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor;
            let kecon = (etke * etkr).min(eaj * kcmax);
            kecon * bpotes_m / etke + et.residue_interception_m
        } else {
            potes_m
        };
        let soil_evaporation_storage_return_m = if es_raw_m < 0.0 { -es_raw_m } else { 0.0 };
        let soil_evaporation_m = es_raw_m.max(0.0);
        let ep_raw_m = etorc_mm * etks * kcbcon * 0.001;
        let plant_transpiration_m = ep_raw_m.max(0.0);
        for (name, value) in [
            ("pmet.etorc_mm", etorc_mm),
            ("pmet.rn_mj_m2", rn_mj_m2),
            ("pmet.fwv_m_s", fwv_m_s),
            ("pmet.rhd_pct", rhd_pct),
            ("pmet.kcbadj", kcbadj),
            ("pmet.kcbcon", kcbcon),
            ("pmet.etke", etke),
            ("pmet.etkr", etkr),
            ("pmet.etks", etks),
            ("pmet.tew_mm", tew_mm),
            ("pmet.rew_mm", rew_mm),
            ("pmet.wfevp_mm", wfevp_mm),
            ("pmet.taw_mm", taw_mm),
            ("pmet.raw_mm", raw_mm),
            ("pmet.wftrp_mm", wftrp_mm),
            ("pmet.es_m", soil_evaporation_m),
            (
                "pmet.es_storage_return_m",
                soil_evaporation_storage_return_m,
            ),
            ("pmet.ep_m", plant_transpiration_m),
        ] {
            if !value.is_finite() {
                return Err(direct_production_executor_blocked(format!(
                    "derived {name} must be finite, observed {value}"
                )));
            }
        }
        Ok(DirectProductionEvappmSeed {
            et_demand_m: plant_transpiration_m,
            soil_evaporation_m,
            plant_transpiration_m,
            soil_evaporation_storage_return_m,
        })
    }

    fn evaporation_storage_terms(
        &self,
        layers: &[DirectSubsurfaceLayerState],
        epdp_m: f64,
    ) -> Result<(f64, f64, f64), HillslopeCliError> {
        let mut tew_mm = 0.0_f64;
        let mut rew_mm = 0.0_f64;
        let mut wfevp_mm = 0.0_f64;
        let mut cumulative_depth_m = 0.0_f64;
        for (offset, layer) in layers.iter().enumerate() {
            let layer_index = offset + 1;
            let solthk = self.solthk(layer_index, cumulative_depth_m, layer.depth_m)?;
            let layer_fraction = if solthk <= epdp_m {
                1.0
            } else if cumulative_depth_m < epdp_m {
                (epdp_m - cumulative_depth_m) / (solthk - cumulative_depth_m)
            } else {
                0.0
            };
            if layer.residual_theta > layer.field_capacity_theta {
                return Err(direct_production_executor_blocked(format!(
                    "wb19_thetdr_{layer_index:04} must be <= wb19_thetfc_{layer_index:04}"
                )));
            }
            if layer_fraction > 0.0 {
                tew_mm +=
                    (layer.field_capacity_theta - 0.5 * layer.residual_theta)
                        * layer.depth_m
                        * 1_000.0
                        * layer_fraction;
                rew_mm +=
                    (layer.field_capacity_theta - layer.residual_theta)
                        * layer.depth_m
                        * 1_000.0
                        / 3.0
                        * layer_fraction;
                wfevp_mm += layer.theta_m * 1_000.0 * layer_fraction;
            }
            cumulative_depth_m = solthk;
            if cumulative_depth_m >= epdp_m {
                break;
            }
        }
        Ok((tew_mm, rew_mm, wfevp_mm))
    }

    fn transpiration_storage_terms(
        &self,
        layers: &[DirectSubsurfaceLayerState],
        tpdp_m: f64,
        wfevp_mm: f64,
    ) -> Result<(f64, f64), HillslopeCliError> {
        let mut taw_mm = 0.0_f64;
        let mut wftrp_mm = 0.0_f64;
        let mut cumulative_depth_m = 0.0_f64;
        for (offset, layer) in layers.iter().enumerate() {
            let layer_index = offset + 1;
            let solthk = self.solthk(layer_index, cumulative_depth_m, layer.depth_m)?;
            if tpdp_m <= 0.0 {
                break;
            }
            if solthk <= tpdp_m {
                taw_mm += (layer.field_capacity_theta - layer.residual_theta)
                    * layer.depth_m
                    * 1_000.0;
                wftrp_mm += layer.theta_m * 1_000.0;
            } else if cumulative_depth_m < tpdp_m {
                let layer_span_m = solthk - cumulative_depth_m;
                if layer_span_m <= 0.0 {
                    return Err(direct_production_executor_blocked(format!(
                        "wb19_solthk_{layer_index:04} must increase with depth for direct production PMET"
                    )));
                }
                let fraction = (tpdp_m - cumulative_depth_m) / layer_span_m;
                taw_mm += (layer.field_capacity_theta - layer.residual_theta)
                    * layer.depth_m
                    * 1_000.0
                    * fraction;
                wftrp_mm = wfevp_mm + layer.theta_m * 1_000.0 * fraction;
                break;
            }
            cumulative_depth_m = solthk;
            if cumulative_depth_m >= tpdp_m {
                break;
            }
        }
        Ok((taw_mm, wftrp_mm))
    }

    fn solthk(
        &self,
        layer_index: usize,
        cumulative_depth_m: f64,
        depth_m: f64,
    ) -> Result<f64, HillslopeCliError> {
        let solthk = self
            .solthk_m
            .get(layer_index - 1)
            .and_then(|value| *value)
            .unwrap_or(cumulative_depth_m + depth_m);
        if solthk <= cumulative_depth_m {
            return Err(direct_production_executor_blocked(format!(
                "wb19_solthk_{layer_index:04} must increase with depth for direct production PMET"
            )));
        }
        Ok(solthk)
    }
}

impl DirectProductionGrowthAuthority {
    fn inactive() -> Self {
        Self {
            active: false,
            rotation_years: 1,
            rotation_repeats: 1,
            slots: Vec::new(),
            monthly_temperature_max_c: [0.0; 12],
            monthly_temperature_min_c: [0.0; 12],
            soil_depth_m: 0.0,
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn inputs(
        &self,
        day: &ClimateDayProjection,
        simulation_year: i32,
        ofe_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        state_before: DirectGrowthStateSurface,
        water_stress: f64,
        et_inputs: &DirectEvapotranspirationComputeInputs,
    ) -> Result<(DirectGrowthInputs, DirectGrowthInputs), HillslopeCliError> {
        if !self.active {
            return Ok((DirectGrowthInputs::zero(), DirectGrowthInputs::zero()));
        }
        let runtime_year =
            direct_growth_i32_to_usize("simulation_year", simulation_year, 1, usize::MAX)?;
        let ofe_index = direct_growth_validate_usize("ofe_index", ofe_index, 1, usize::MAX)?;
        let runtime_day = direct_growth_u16_to_usize("day", day.julian_day, 1, 366)?;
        let Some(selection) = self.active_crop(runtime_year, runtime_day, ofe_index)? else {
            return Ok((DirectGrowthInputs::zero(), DirectGrowthInputs::zero()));
        };
        let runtime_day = direct_growth_usize_to_u16("day", runtime_day)?;
        let slot_index = direct_growth_usize_to_u16("slot_index", selection.slot_index)?;
        let crop_slot_index =
            direct_growth_usize_to_u16("crop_slot_index", selection.crop_slot_index)?;

        match selection.crop.imngmt {
            1 | 3 => {
                let active_action = if runtime_day == selection.crop.jdplt {
                    DirectGrowthAction::PlantingReset
                } else if runtime_day == selection.crop.jdharv {
                    DirectGrowthAction::HarvestReset
                } else {
                    DirectGrowthAction::None
                };
                Ok((
                    self.crop_inputs(
                        selection.crop,
                        DirectGrowthActiveContext::AnnualOrFallow {
                            active_slot_index: slot_index,
                            active_crop_slot_index: crop_slot_index,
                            runtime_day_of_year: runtime_day,
                        },
                        active_action,
                        forcing,
                        state_before,
                        water_stress,
                        et_inputs,
                    ),
                    DirectGrowthInputs::zero(),
                ))
            }
            2 => {
                let active_action = if selection.crop.jdplt != 0
                    && runtime_day == selection.crop.jdplt
                {
                    DirectGrowthAction::PlantingReset
                } else if selection.crop.jdstop != 0 && runtime_day == selection.crop.jdstop {
                    DirectGrowthAction::StopReset
                } else {
                    DirectGrowthAction::None
                };
                Ok((
                    DirectGrowthInputs::zero(),
                    self.crop_inputs(
                        selection.crop,
                        DirectGrowthActiveContext::Perennial {
                            active_slot_index: slot_index,
                            active_crop_slot_index: crop_slot_index,
                            runtime_day_of_year: runtime_day,
                        },
                        active_action,
                        forcing,
                        state_before,
                        water_stress,
                        et_inputs,
                    ),
                ))
            }
            _ => Err(direct_growth_failure(format!(
                "unsupported direct production growth management class {}",
                selection.crop.imngmt
            ))),
        }
    }

    fn active_crop(
        &self,
        runtime_year: usize,
        runtime_day: usize,
        ofe_index: usize,
    ) -> Result<Option<DirectGrowthActiveCropSelection<'_>>, HillslopeCliError> {
        let max_runtime_year = self.rotation_repeats.saturating_mul(self.rotation_years);
        if runtime_year > max_runtime_year {
            return Err(direct_growth_failure(format!(
                "year {runtime_year} exceeds direct growth rotation span {max_runtime_year}"
            )));
        }
        let rotation_index = ((runtime_year - 1) / self.rotation_years) + 1;
        let year_in_rotation = ((runtime_year - 1) % self.rotation_years) + 1;
        let year_slot_candidates = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| {
                slot.year_in_rotation == year_in_rotation && slot.rotation_index == rotation_index
            })
            .collect::<Vec<_>>();
        let mut slot_candidates = year_slot_candidates
            .iter()
            .copied()
            .filter(|(_, slot)| slot.ofe_index == ofe_index)
            .collect::<Vec<_>>();
        let (slot_offset, slot) = match slot_candidates.as_mut_slice() {
            [(slot_offset, slot)] => (*slot_offset, *slot),
            [] if year_slot_candidates.len() == 1 && year_slot_candidates[0].1.ofe_index == 1 => {
                year_slot_candidates[0]
            }
            [] => {
                return Err(direct_growth_failure(format!(
                    "missing direct growth PL slot for OFE {ofe_index} year_in_rotation={year_in_rotation}"
                )));
            }
            _ => {
                return Err(direct_growth_failure(format!(
                    "ambiguous direct growth PL slots for primary OFE year_in_rotation={year_in_rotation}"
                )));
            }
        };
        let mut crop_candidates = slot
            .crops
            .iter()
            .enumerate()
            .filter(|(_, crop)| crop.active_on_day(runtime_day))
            .collect::<Vec<_>>();
        let (crop_offset, crop) = match crop_candidates.as_mut_slice() {
            [(crop_offset, crop)] => (*crop_offset, *crop),
            [] => return Ok(None),
            _ => {
                return Err(direct_growth_failure(format!(
                    "ambiguous active direct growth crops for slot {} day {runtime_day}",
                    slot_offset + 1
                )));
            }
        };
        Ok(Some(DirectGrowthActiveCropSelection {
            slot_index: slot_offset + 1,
            crop_slot_index: crop_offset + 1,
            crop,
        }))
    }
    #[allow(clippy::too_many_arguments)]
    fn crop_inputs(
        &self,
        crop: &DirectProductionGrowthCropAuthority,
        active_context: DirectGrowthActiveContext,
        active_action: DirectGrowthAction,
        forcing: &HillslopeDirectClimateDayForcing,
        state_before: DirectGrowthStateSurface,
        water_stress: f64,
        et_inputs: &DirectEvapotranspirationComputeInputs,
    ) -> DirectGrowthInputs {
        DirectGrowthInputs {
            active_context,
            active_action,
            state_before,
            planting_day: crop.jdplt,
            harvest_day: crop.jdharv,
            stop_day: crop.jdstop,
            water_stress,
            temperature_max_c: forcing.tmax_c,
            temperature_min_c: forcing.tmin_c,
            radiation_mj_m2: forcing.rad_ly,
            monthly_temperature_max_c: self.monthly_temperature_max_c,
            monthly_temperature_min_c: self.monthly_temperature_min_c,
            soil_depth_m: self.soil_depth_m,
            btemp: crop.btemp,
            otemp: crop.otemp,
            gddmax: crop.gddmax,
            dlai: crop.dlai,
            dropfc: crop.dropfc,
            decfct: crop.decfct,
            spriod: crop.spriod,
            bb: crop.bb,
            beinp: crop.beinp,
            extnct: crop.extnct,
            hi: crop.hi,
            xmxlai: crop.xmxlai,
            rsr: crop.rsr,
            rtmmax: crop.rtmmax,
            rdmax: crop.rdmax,
            et_demand_m: et_inputs.et_demand_m,
            residue_interception_m: et_inputs.residue_interception_m,
            plant_tolerance: et_inputs.plant_tolerance,
        }
    }
}

impl DirectProductionGrowthCropAuthority {
    fn active_on_day(self, runtime_day: usize) -> bool {
        if self.schedule_imngmt == 2 {
            if self.jdplt == 0 {
                self.jdstop == 0 || runtime_day <= usize::from(self.jdstop)
            } else if self.jdstop == 0 {
                direct_growth_day_is_within_window(
                    runtime_day,
                    usize::from(self.jdplt),
                    usize::from(self.jdharv.max(1)),
                )
            } else {
                direct_growth_day_is_within_window(
                    runtime_day,
                    usize::from(self.jdplt),
                    usize::from(self.jdstop),
                )
            }
        } else {
            direct_growth_day_is_within_window(
                runtime_day,
                usize::from(self.jdplt),
                usize::from(self.jdharv.max(1)),
            )
        }
    }

    fn surface_decomposition_rate(self) -> f64 {
        if self.oratea == 0.0 && self.has_seasonal_litter_signal() {
            FOREST_LITTER_FALLBACK_DECAY_RATE_PER_DAY
        } else {
            self.oratea
        }
    }

    fn has_seasonal_litter_signal(self) -> bool {
        self.spriod > 0.0 && (self.dropfc < 1.0 || self.decfct < 1.0)
    }

    fn uses_fall_litter_drop_schedule(self) -> bool {
        self.imngmt == 2 && self.jdharv > 0 && self.has_seasonal_litter_signal()
    }

    fn fall_litter_drop_window_contains(self, runtime_day: usize) -> bool {
        if !self.uses_fall_litter_drop_schedule() {
            return false;
        }
        let end = usize::from(self.jdharv);
        let start = end
            .saturating_sub(FOREST_LITTER_DROP_WINDOW_DAYS)
            .max(1);
        runtime_day >= start && runtime_day <= end
    }
}

struct DirectGrowthActiveCropSelection<'a> {
    slot_index: usize,
    crop_slot_index: usize,
    crop: &'a DirectProductionGrowthCropAuthority,
}

fn direct_growth_interception_live_biomass_from_state(
    growth_state: DirectGrowthStateSurface,
) -> Result<f64, HillslopeCliError> {
    direct_growth_nonnegative_scalar("growth.vdmt", growth_state.live_biomass_kg_m2)?;
    direct_growth_validate_harvest_index(growth_state.harvest_index)?;
    if growth_state.interception_live_biomass_kg_m2 > 0.0 || growth_state.live_biomass_kg_m2 == 0.0
    {
        direct_growth_nonnegative_scalar(
            "growth.tlive",
            growth_state.interception_live_biomass_kg_m2,
        )?;
        Ok(growth_state.interception_live_biomass_kg_m2)
    } else {
        Ok(growth_state.live_biomass_kg_m2)
    }
}

fn direct_growth_validate_harvest_index(hia: f64) -> Result<(), HillslopeCliError> {
    if hia.is_finite() && (0.0..=1.0).contains(&hia) {
        Ok(())
    } else {
        Err(direct_production_executor_blocked(format!(
            "{SIMOUT_GUARD_ID} hia must be finite and within [0, 1] to construct direct WB15 tlive bridge, observed {hia}"
        )))
    }
}

fn direct_growth_nonnegative_scalar(symbol: &str, value: f64) -> Result<(), HillslopeCliError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(direct_production_executor_blocked(format!(
            "{SIMOUT_GUARD_ID} {symbol} must be finite and >= 0.0 for direct growth state, observed {value}"
        )))
    }
}

fn direct_growth_schedule_slot_symbol(slot_index: usize, root: &str) -> String {
    format!("pl_schedule_slot_{slot_index:04}_{root}")
}

fn direct_growth_schedule_slot_crop_symbol(
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> String {
    format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn direct_growth_slot_crop_symbol(
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> String {
    format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn direct_decomp_slot_crop_symbol(
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> String {
    format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn direct_growth_day_is_within_window(
    runtime_day: usize,
    start_day: usize,
    end_day: usize,
) -> bool {
    if start_day <= end_day {
        runtime_day >= start_day && runtime_day <= end_day
    } else {
        runtime_day >= start_day || runtime_day <= end_day
    }
}

fn direct_growth_integral_usize(
    symbol: &str,
    value: f64,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    if !value.is_finite() {
        return Err(direct_growth_failure(format!(
            "{symbol} must be finite for direct growth, observed {value}"
        )));
    }
    let rounded = value.round();
    if (value - rounded).abs() > 1.0e-12 || rounded < 0.0 {
        return Err(direct_growth_failure(format!(
            "{symbol} must be integral for direct growth, observed {value}"
        )));
    }
    let parsed = direct_growth_rounded_to_usize(symbol, rounded)?;
    if parsed < min_allowed || parsed > max_allowed {
        return Err(direct_growth_failure(format!(
            "{symbol} value {parsed} outside [{min_allowed}, {max_allowed}] for direct growth"
        )));
    }
    Ok(parsed)
}
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
fn direct_growth_rounded_to_usize(symbol: &str, value: f64) -> Result<usize, HillslopeCliError> {
    if value > usize::MAX as f64 {
        return Err(direct_growth_failure(format!(
            "{symbol} value {value} exceeds usize range"
        )));
    }
    Ok(value as usize)
}

fn direct_growth_i32_to_usize(
    symbol: &str,
    value: i32,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    if value < 0 {
        return Err(direct_growth_failure(format!(
            "{symbol} must be non-negative for direct growth, observed {value}"
        )));
    }
    let parsed = usize::try_from(value).map_err(|_| {
        direct_growth_failure(format!("{symbol} value {value} exceeds usize range"))
    })?;
    if parsed < min_allowed || parsed > max_allowed {
        return Err(direct_growth_failure(format!(
            "{symbol} value {parsed} outside [{min_allowed}, {max_allowed}] for direct growth"
        )));
    }
    Ok(parsed)
}

fn direct_growth_u16_to_usize(
    symbol: &str,
    value: u16,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    let parsed = usize::from(value);
    if parsed < min_allowed || parsed > max_allowed {
        return Err(direct_growth_failure(format!(
            "{symbol} value {parsed} outside [{min_allowed}, {max_allowed}] for direct growth"
        )));
    }
    Ok(parsed)
}

fn direct_growth_validate_usize(
    symbol: &str,
    value: usize,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    if value < min_allowed || value > max_allowed {
        return Err(direct_growth_failure(format!(
            "{symbol} value {value} outside [{min_allowed}, {max_allowed}] for direct growth"
        )));
    }
    Ok(value)
}

fn direct_growth_usize_to_u16(symbol: &str, value: usize) -> Result<u16, HillslopeCliError> {
    u16::try_from(value).map_err(|_| {
        direct_growth_failure(format!("{symbol} value {value} exceeds u16 range"))
    })
}

fn direct_growth_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!("{SIMOUT_GUARD_ID} {}", detail.into()),
    }
}
