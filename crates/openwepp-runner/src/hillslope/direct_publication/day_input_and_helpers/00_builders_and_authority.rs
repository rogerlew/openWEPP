const SNOWDENSITY09_DENSITY_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL";
const SNOWDENSITY1035_PHASE_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL";
const SNOWDENSITY1037_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1037_MELT_MODEL";
const SNOWDENSITY1038_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL";
const SNOWFROST_STAGE2_INSULATION_MODEL_ENV: &str =
    "OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL";
const PARADIGM2_STAGE3_LIQUID_MODEL_ENV: &str =
    "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL";
const FOREST_LITTER_FALLBACK_DECAY_RATE_PER_DAY: f64 = 0.5 / 365.25;
const FOREST_LITTER_DROP_WINDOW_DAYS: usize = 45;

struct DirectProductionDayInputBuilder<'a> {
    climate_request: &'a HillslopeClimateRuntimeRequest,
    climate_span: &'a ClimateRunSpanSummary,
    lane_authority: Vec<DirectProductionLaneDayInputAuthority>,
    residue_cover_state: std::cell::RefCell<Vec<DirectProductionResidueCoverState>>,
    winter_hourly_geometry: DirectProductionWinterHourlyGeometry,
    sturm_climate_class: Option<openwepp_hillslope_orchestrator::SnowClimateClass>,
}

#[derive(Clone)]
struct DirectProductionSeedAuthority {
    lanes: Vec<DirectProductionLaneSeedAuthority>,
    winter_hourly_geometry: DirectProductionWinterHourlyGeometry,
    erod14_wave2_enabled: bool,
}

struct DirectProductionSnowbenchExportSeed {
    primary_canopy_cover_fraction: f64,
    winter_context: openwepp_hillslope_orchestrator::DirectWinterHourlyContext,
    snow_density_kg_m3: f64,
}

#[derive(Clone)]
struct DirectProductionLaneSeedAuthority {
    constructor: DirectProductionLaneConstructorSeed,
    day_input: DirectProductionLaneDayInputAuthority,
}

#[derive(Clone)]
struct DirectProductionTypedLaneSeedAuthority {
    constructor: DirectProductionLaneConstructorSeed,
    peak_runoff: DirectProductionPeakRunoffAuthority,
    percolation: DirectPercolationInputs,
    subsurface: DirectSubsurfaceComputeInputs,
    hydrology_projection: DirectHydrologyProjectionInputs,
    infiltration: DirectProductionInfiltrationAuthority,
    evapotranspiration: DirectProductionEvapotranspirationAuthority,
    residue_cover: DirectProductionResidueCoverAuthority,
    growth: DirectProductionGrowthAuthority,
    erosion: DirectProductionErosionAuthority,
    snow_frost: DirectProductionSnowFrostAuthority,
}

#[derive(Clone)]
struct DirectProductionLaneConstructorSeed {
    soil_water_m: f64,
    subsurface_layers: Vec<DirectSubsurfaceLayerState>,
    evapotranspiration_stage_state: Option<DirectEvapotranspirationStageState>,
    plant_growth_state: DirectGrowthStateSurface,
    plant_water_stress: f64,
    snow_lane_state: DirectSnowLaneState,
}

#[derive(Clone)]
struct DirectProductionTypedLayerSeed {
    soil_water_m: f64,
    layers: Vec<DirectSubsurfaceLayerState>,
}

#[derive(Clone)]
struct DirectProductionLaneDayInputAuthority {
    peak_runoff: DirectProductionPeakRunoffAuthority,
    percolation: DirectPercolationInputs,
    subsurface: DirectSubsurfaceComputeInputs,
    infiltration: DirectProductionInfiltrationAuthority,
    evapotranspiration: DirectProductionEvapotranspirationAuthority,
    residue_cover: DirectProductionResidueCoverAuthority,
    growth: DirectProductionGrowthAuthority,
    hydrology_projection: DirectHydrologyProjectionInputs,
    erosion: DirectProductionErosionAuthority,
    snow_frost: DirectProductionSnowFrostAuthority,
}

#[derive(Clone)]
struct DirectProductionPeakRunoffAuthority {
    irrigation_rate_m_s: f64,
    efflen_m: f64,
    ealpha: f64,
    exponent_m: f64,
}

#[derive(Clone)]
struct DirectProductionInfiltrationAuthority {
    effective_conductivity_m_s: Option<f64>,
    matric_potential_m: Option<f64>,
    depression_storage_capacity_m: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectProductionEvapotranspirationAuthority {
    leaf_area_index: f64,
    canopy_cover_fraction: f64,
    residue_interception_m: f64,
    root_depth_m: f64,
    plant_tolerance: f64,
    priestley_taylor: DirectProductionPriestleyTaylorAuthority,
    pmet: Option<DirectProductionPmetAuthority>,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectProductionPriestleyTaylorAuthority {
    salb: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectProductionPmetAuthority {
    kcb: f64,
    rawp: f64,
    canhgt: f64,
    radpot_ly: Option<f64>,
    solthk_m: Vec<Option<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectProductionGrowthAuthority {
    active: bool,
    rotation_years: usize,
    rotation_repeats: usize,
    slots: Vec<DirectProductionGrowthSlotAuthority>,
    monthly_temperature_max_c: [f64; 12],
    monthly_temperature_min_c: [f64; 12],
    soil_depth_m: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectProductionGrowthSlotAuthority {
    ofe_index: usize,
    year_in_rotation: usize,
    rotation_index: usize,
    crops: Vec<DirectProductionGrowthCropAuthority>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DirectProductionGrowthCropAuthority {
    schedule_imngmt: u8,
    imngmt: u8,
    jdharv: u16,
    jdplt: u16,
    jdstop: u16,
    btemp: f64,
    otemp: f64,
    gddmax: f64,
    dlai: f64,
    dropfc: f64,
    decfct: f64,
    spriod: f64,
    bb: f64,
    beinp: f64,
    extnct: f64,
    hi: f64,
    xmxlai: f64,
    rsr: f64,
    rtmmax: f64,
    rdmax: f64,
    oratea: f64,
    orater: f64,
}

#[derive(Clone, Copy)]
struct DirectProductionResidueCoverAuthority {
    initial_surface_residue_kg_m2: f64,
    initial_root_residue_kg_m2: f64,
    residue_type_selector: f64,
    residue_depth_conversion_m_per_kg_m2: f64,
}

#[derive(Clone, Copy)]
struct DirectProductionResidueCoverState {
    surface_residue_kg_m2: f64,
    root_residue_kg_m2: f64,
    pending_surface_litter_kg_m2: f64,
    residue_depth_m: f64,
}

#[derive(Clone, Copy)]
struct DirectProductionResidueCoverProjection {
    decomposition_inputs: DirectDecompositionInputs,
    residue_partition_inputs: DirectResiduePartitionInputs,
    state_before: DirectProductionResidueCoverState,
    state_after: DirectProductionResidueCoverState,
    surface_litter_input_kg_m2: f64,
    pending_surface_litter_after_kg_m2: f64,
}

#[derive(Clone, Copy)]
struct DirectProductionSurfaceLitterProjection {
    surface_litter_input_kg_m2: f64,
    pending_surface_litter_after_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectProductionErosionAuthority {
    wave2_enabled: bool,
    erosion_inputs: DirectErosionInputs,
}

#[derive(Clone, Copy)]
struct DirectProductionWinterHourlyGeometry {
    avg_slope: f64,
    azimuth: f64,
}

impl DirectProductionWinterHourlyGeometry {
    fn from_typed_inputs(
        inputs: &ParsedHillslopeRunInputs,
        lane_count: usize,
    ) -> Result<Self, HillslopeCliError> {
        let projection = openwepp_hillslope_orchestrator::runtime_inputs::project_typed_slope_runtime_with_options(
            &inputs.slope,
            SlopeRuntimeSurfaceOptions::compatibility(),
        )
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: error.to_string(),
        })?;
        let ofe_index = if lane_count <= 1 {
            0
        } else {
            lane_count.saturating_sub(1)
        };
        let ofe = projection.ofes.get(ofe_index).ok_or_else(|| {
            direct_production_executor_blocked(format!(
                "typed winter geometry missing OFE {} out of {} projected OFEs",
                ofe_index + 1,
                projection.ofes.len()
            ))
        })?;
        Ok(Self {
            avg_slope: ofe.avgslp,
            azimuth: ofe.azimuth_deg,
        })
    }

}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, PartialEq)]
struct DirectProductionSnowFrostAuthority {
    snow_file_present: bool,
    snow_runtime_swe_m: f64,
    snow_runtime_depth_m: f64,
    snow_runtime_density_kg_m3: f64,
    snow_runtime_settle_day_count: f64,
    snow_controls_projected: bool,
    snow_density_model: openwepp_hillslope_orchestrator::SnowDensityModel,
    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel,
    snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel,
    stage3_liquid_routing_model:
        openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel,
    snow_rst_c: f64,
    snow_newsnw_kg_m3: f64,
    snow_ssd_kg_m3: f64,
    frost_typed_authority: Option<DirectProductionFrostTypedAuthority>,
    frost_layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
    frost_file_present: bool,
    frost_wint_red_enabled: bool,
    frost_runtime_depth_m: f64,
    frost_runtime_frozen_water_m: f64,
    frost_active: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectProductionFrostTypedAuthority {
    controls: DirectFrostControlInputs,
    layer_bulk_density_kg_m3: Vec<f64>,
    soil_conductivity_m_s: Option<f64>,
    residue_depth_m: f64,
    theta_residual: f64,
    theta_field_capacity: f64,
    albedo: f64,
    canopy_height_m: f64,
    random_roughness_m: f64,
    seasonal_temperature_curve: FrostSeasonalTemperatureCurve,
}

struct DirectProductionFrostDayContext {
    compute_inputs: DirectWinterFrostComputeInputs,
    frozen_infiltration_capacity_m_s: f64,
    storage_liquid_delta_m: Option<f64>,
    layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
    hydrology_layers: Vec<DirectSubsurfaceLayerState>,
}

struct DirectProductionFrostTypedComputeContext<'a> {
    lane_index: usize,
    lane: &'a openwepp_hillslope_orchestrator::DirectLaneFrame,
    day: &'a ClimateDayProjection,
    forcing: &'a HillslopeDirectClimateDayForcing,
    snow_lane_state: &'a DirectSnowLaneState,
    frost_lane_state: &'a DirectFrostLaneState,
    typed_authority: &'a DirectProductionFrostTypedAuthority,
    residue_depth_m_override: Option<f64>,
    hourly: [DirectFrostHourlyForcing;
        openwepp_hillslope_orchestrator::DIRECT_WINTER_HOURLY_FORCING_COUNT],
}

#[allow(dead_code)]
impl DirectProductionSeedAuthority {
    fn from_typed_inputs(
        climate_request: &HillslopeClimateRuntimeRequest,
        inputs: &ParsedHillslopeRunInputs,
        sidecars: &HillslopeSidecarResolution,
        lane_count: usize,
        execution_lane: ExecutionLane,
    ) -> Result<Self, HillslopeCliError> {
        if lane_count == 0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production requires at least one lane seed authority"
                ),
            });
        }
        let typed_lane_seeds = direct_production_typed_lane_seed_authorities(
            climate_request,
            inputs,
            sidecars,
            lane_count,
            execution_lane,
        )?;
        if typed_lane_seeds.len() != lane_count {
            return Err(direct_production_executor_blocked(format!(
                "typed lane seed count {} does not match requested lane count {lane_count}",
                typed_lane_seeds.len()
            )));
        }

        let lanes = typed_lane_seeds
            .into_iter()
            .map(|typed_lane_seed| DirectProductionLaneSeedAuthority {
                constructor: typed_lane_seed.constructor.clone(),
                day_input: direct_production_day_input_authority_from_typed_seed(
                    typed_lane_seed,
                ),
            })
            .collect::<Vec<_>>();
        let winter_hourly_geometry =
            DirectProductionWinterHourlyGeometry::from_typed_inputs(inputs, lane_count)?;
        let erod14_wave2_enabled = direct_production_typed_erod14_wave2_enabled(inputs, lane_count)?;

        Ok(Self {
            lanes,
            winter_hourly_geometry,
            erod14_wave2_enabled,
        })
    }

    fn lane(
        &self,
        lane_index: usize,
    ) -> Result<&DirectProductionLaneSeedAuthority, HillslopeCliError> {
        if self.lanes.len() == 1 {
            return Ok(&self.lanes[0]);
        }
        self.lanes.get(lane_index).ok_or_else(|| {
            direct_production_executor_blocked(format!(
                "direct production lane {} has no typed seed authority out of {} lanes",
                lane_index + 1,
                self.lanes.len()
            ))
        })
    }

    fn outlet_snow_frost(
        &self,
    ) -> Result<&DirectProductionSnowFrostAuthority, HillslopeCliError> {
        let outlet_index = self.lanes.len().saturating_sub(1);
        Ok(&self.lane(outlet_index)?.day_input.snow_frost)
    }

    fn outlet_top_soil_conductivity_m_s(&self) -> Result<f64, HillslopeCliError> {
        let outlet_index = self.lanes.len().saturating_sub(1);
        let lane = self.lane(outlet_index)?;
        lane.constructor
            .subsurface_layers
            .first()
            .map(|layer| layer.conductivity_m_s)
            .ok_or_else(|| {
                direct_production_executor_blocked(
                    "direct production coupling provenance requires outlet top soil conductivity",
                )
            })
    }

    fn snowbench_export_seed(&self) -> Result<DirectProductionSnowbenchExportSeed, HillslopeCliError> {
        let primary_lane = self.lane(0)?;
        let outlet_snow_frost = self.outlet_snow_frost()?;
        Ok(DirectProductionSnowbenchExportSeed {
            primary_canopy_cover_fraction: primary_lane
                .day_input
                .evapotranspiration
                .canopy_cover_fraction,
            winter_context: openwepp_hillslope_orchestrator::DirectWinterHourlyContext {
                snow_runtime_swe_m: outlet_snow_frost.snow_runtime_swe_m,
                frost_runtime_depth_m: outlet_snow_frost.frost_runtime_depth_m,
                frost_runtime_frozen_water_m: outlet_snow_frost.frost_runtime_frozen_water_m,
                frost_file_present: outlet_snow_frost.frost_file_present,
                frost_wint_red_enabled: outlet_snow_frost.frost_wint_red_enabled,
                avg_slope: self.winter_hourly_geometry.avg_slope,
                azimuth: self.winter_hourly_geometry.azimuth,
                snow_rst_c: outlet_snow_frost.snow_rst_c,
                snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel::LegacyRst,
            },
            snow_density_kg_m3: outlet_snow_frost.snow_newsnw_kg_m3,
        })
    }
}

impl DirectProductionLaneConstructorSeed {
    fn apply_to_lane_constructor(&self, lane_inputs: &mut DirectLaneConstructorInputs) {
        lane_inputs.water.soil_water_m = self.soil_water_m;
        lane_inputs
            .subsurface_layers
            .clone_from(&self.subsurface_layers);
        lane_inputs.evapotranspiration_stage_state =
            self.evapotranspiration_stage_state.map(Box::new);
        *lane_inputs.plant_growth_state = self.plant_growth_state;
        lane_inputs.plant_water_stress = self.plant_water_stress;
        lane_inputs.winter_column.snow = self.snow_lane_state.clone();
    }
}

fn direct_production_typed_lane_seed_authorities(
    climate_request: &HillslopeClimateRuntimeRequest,
    inputs: &ParsedHillslopeRunInputs,
    sidecars: &HillslopeSidecarResolution,
    lane_count: usize,
    execution_lane: ExecutionLane,
) -> Result<Vec<DirectProductionTypedLaneSeedAuthority>, HillslopeCliError> {
    if lane_count == 0 {
        return Err(direct_production_executor_blocked(
            "typed lane constructor seeding requires at least one lane",
        ));
    }

    if lane_count == 1 {
        return Ok(vec![direct_production_typed_lane_seed_authority(
            climate_request,
            &inputs.soil,
            &inputs.slope,
            &inputs.management,
            &sidecars.snow,
            &sidecars.frost,
            &sidecars.pmetpara,
            sidecars.pmetpara_mode,
            execution_lane,
            lane_count,
        )?]);
    }

    let slices = crate::hillslope::intake_lane_setup::build_static_per_ofe_lane_slices(
        &inputs.slope,
        &inputs.soil,
        inputs.management.topology_count,
    )?;
    if slices.len() != lane_count {
        return Err(direct_production_executor_blocked(format!(
            "typed lane constructor expected {lane_count} static OFE slices, observed {}",
            slices.len()
        )));
    }
    slices
        .iter()
        .map(|slice| {
            let lane_soil =
                crate::hillslope::intake_lane_setup::build_lane_soil_profile(slice, &inputs.soil)?;
            let lane_slope = crate::hillslope::intake_lane_setup::build_lane_slope_profile(
                slice,
                &inputs.slope,
            )?;
            let lane_management =
                crate::hillslope::intake_lane_setup::build_lane_management_output(
                    slice,
                    &inputs.management,
                )?;
            direct_production_typed_lane_seed_authority(
                climate_request,
                &lane_soil,
                &lane_slope,
                &lane_management,
                &sidecars.snow,
                &sidecars.frost,
                &sidecars.pmetpara,
                sidecars.pmetpara_mode,
                execution_lane,
                lane_count,
            )
        })
        .collect()
}

fn direct_production_day_input_authority_from_typed_seed(
    seed: DirectProductionTypedLaneSeedAuthority,
) -> DirectProductionLaneDayInputAuthority {
    DirectProductionLaneDayInputAuthority {
        peak_runoff: seed.peak_runoff,
        percolation: seed.percolation,
        subsurface: seed.subsurface,
        infiltration: seed.infiltration,
        evapotranspiration: seed.evapotranspiration,
        residue_cover: seed.residue_cover,
        growth: seed.growth,
        hydrology_projection: seed.hydrology_projection,
        erosion: seed.erosion,
        snow_frost: seed.snow_frost,
    }
}

#[allow(clippy::too_many_arguments)]
fn direct_production_typed_lane_seed_authority(
    climate_request: &HillslopeClimateRuntimeRequest,
    soil: &SoilProfile,
    slope: &SlopeProfile,
    management: &ManagementParseOutput,
    snow: &SnowParseOutput,
    frost: &FrostParseOutput,
    pmetpara: &PmetparaFile,
    pmetpara_mode: PmetparaParseMode,
    execution_lane: ExecutionLane,
    contributor_ofe_count: usize,
) -> Result<DirectProductionTypedLaneSeedAuthority, HillslopeCliError> {
    let soil_projection = project_typed_soil_wb11_runtime(soil).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: error.to_string(),
        }
    })?;
    let slope_projection =
        openwepp_hillslope_orchestrator::runtime_inputs::project_typed_slope_runtime_with_options(
            slope,
            SlopeRuntimeSurfaceOptions::compatibility(),
        )
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: error.to_string(),
        })?;
    let layer_seed = direct_production_typed_layer_seed(&soil_projection, execution_lane)?;
    let management_projection = build_hillslope_pl_runtime_surfaces_from_management(management)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: error.to_string(),
        })?;
    let mut pmetpara_projection_source = pmetpara.clone();
    let pmetpara_projection = crate::hillslope::intake_lane_setup::project_typed_pmetpara_runtime(
        management,
        &mut pmetpara_projection_source,
        pmetpara_mode,
    )?;
    let optional_defaults = project_typed_wb11_optional_defaults(None, None);
    let snow_projection = project_typed_snow_runtime(snow).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: error.to_string(),
        }
    })?;
    let frost_projection = project_typed_frost_runtime(frost).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: error.to_string(),
        }
    })?;
    let lane_substeps = project_typed_wb11_lane_substeps(execution_lane, contributor_ofe_count)?;
    let peak_runoff =
        direct_production_typed_peak_runoff_authority(&slope_projection, &management_projection)?;
    let erosion = direct_production_typed_erosion_authority(
        &soil_projection,
        &slope_projection,
        &peak_runoff,
        contributor_ofe_count,
    )?;

    Ok(DirectProductionTypedLaneSeedAuthority {
        constructor: direct_production_typed_lane_constructor_seed_from_projections(
            &layer_seed,
            &management_projection,
            &optional_defaults,
            &snow_projection,
        )?,
        peak_runoff,
        percolation: direct_production_typed_percolation_inputs(
            &soil_projection,
            &layer_seed,
            lane_substeps.wb18_perc_lane_substeps,
        )?,
        subsurface: direct_production_typed_subsurface_inputs(
            &soil_projection,
            &slope_projection,
            &management_projection,
            &layer_seed,
            lane_substeps.wb19_lateral_drain_lane_substeps,
        )?,
        hydrology_projection: direct_production_typed_hydrology_projection_inputs(
            &soil_projection,
            &layer_seed,
        )?,
        infiltration: direct_production_typed_infiltration_authority(&frost_projection),
        evapotranspiration: direct_production_typed_evapotranspiration_authority(
            &soil_projection,
            &management_projection,
            &optional_defaults,
            &pmetpara_projection,
        )?,
        residue_cover: direct_production_typed_residue_cover_authority(&management_projection)?,
        growth: direct_production_typed_growth_authority(
            &management_projection,
            climate_request,
            &soil_projection,
        )?,
        erosion,
        snow_frost: direct_production_typed_snow_frost_authority(
            &soil_projection,
            &layer_seed,
            &management_projection,
            &snow_projection,
            &frost_projection,
            climate_request,
        )?,
    })
}

fn direct_production_typed_peak_runoff_authority(
    slope: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSlopeRuntimeProjection,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
) -> Result<DirectProductionPeakRunoffAuthority, HillslopeCliError> {
    let first_ofe = slope.ofes.first().ok_or_else(|| {
        direct_production_executor_blocked("typed peak-runoff seed requires at least one slope OFE")
    })?;
    let efflen_and_m = project_typed_wb11_efflen_and_m(None, first_ofe.slplen_m, None)?;
    let ealpha = direct_production_typed_wb16_ealpha(
        slope,
        management_projection,
        efflen_and_m.exponent_m,
    )?
    .map_or(1.0, |projection| projection.ealpha);
    Ok(DirectProductionPeakRunoffAuthority {
        irrigation_rate_m_s: 0.0,
        efflen_m: efflen_and_m.efflen_m,
        ealpha,
        exponent_m: efflen_and_m.exponent_m,
    })
}

fn direct_production_typed_wb16_ealpha(
    slope: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSlopeRuntimeProjection,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    exponent_m: f64,
) -> Result<Option<TypedWb16EalphaProducerProjection>, HillslopeCliError> {
    let mut ofes = Vec::with_capacity(slope.ofes.len());
    for (ofe_offset, slope_ofe) in slope.ofes.iter().enumerate() {
        let ofe_index = ofe_offset + 1;
        let Some(inrcov) = direct_production_pl_projection_optional_ofe_scalar(
            management_projection,
            ofe_index,
            "inrcov",
        ) else {
            return Ok(None);
        };
        let Some(rilcov) = direct_production_pl_projection_optional_ofe_scalar(
            management_projection,
            ofe_index,
            "rilcov",
        ) else {
            return Ok(None);
        };
        let Some(rrinit) = direct_production_pl_projection_optional_ofe_scalar(
            management_projection,
            ofe_index,
            "rrinit",
        ) else {
            return Ok(None);
        };
        let Some(rspace) = direct_production_pl_projection_optional_ofe_scalar(
            management_projection,
            ofe_index,
            "rspace",
        ) else {
            return Ok(None);
        };
        let Some(width) = direct_production_pl_projection_optional_ofe_scalar(
            management_projection,
            ofe_index,
            "width",
        ) else {
            return Ok(None);
        };
        let Some(rtyp) = direct_production_pl_projection_optional_ofe_scalar(
            management_projection,
            ofe_index,
            "rtyp",
        ) else {
            return Ok(None);
        };
        let Some(cancov) = direct_production_pl_projection_optional_ofe_scalar(
            management_projection,
            ofe_index,
            "cancov",
        ) else {
            return Ok(None);
        };
        let Some(bb) =
            direct_production_typed_wb16_canopy_scalar(management_projection, ofe_index, "bb")
        else {
            return Ok(None);
        };
        let Some(bbb) =
            direct_production_typed_wb16_canopy_scalar(management_projection, ofe_index, "bbb")
        else {
            return Ok(None);
        };
        let Some(flivmx) =
            direct_production_typed_wb16_canopy_scalar(management_projection, ofe_index, "flivmx")
        else {
            return Ok(None);
        };
        let Some(hmax) =
            direct_production_typed_wb16_canopy_scalar(management_projection, ofe_index, "hmax")
        else {
            return Ok(None);
        };
        ofes.push(TypedWb16OfeEalphaInput {
            avgslp: slope_ofe.avgslp,
            slplen: slope_ofe.slplen_m,
            inrcov,
            rilcov,
            rrinit,
            rspace,
            width,
            rtyp,
            cancov,
            bb,
            bbb,
            flivmx,
            hmax,
            rrc: direct_production_pl_projection_optional_ofe_scalar(
                management_projection,
                ofe_index,
                "rrc",
            ),
            canhgt: direct_production_pl_projection_optional_ofe_scalar(
                management_projection,
                ofe_index,
                "canhgt",
            ),
        });
    }
    project_typed_wb16_ealpha_producer(&TypedWb16EalphaProducerInput { exponent_m, ofes })
        .map(Some)
}

fn direct_production_typed_wb16_canopy_scalar(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    ofe_index: usize,
    root: &'static str,
) -> Option<f64> {
    direct_production_pl_projection_optional_scalar(
        projection,
        &format!("pl_growth_ofe{ofe_index}_{root}_seed"),
    )
    .or_else(|| {
        if root == "bbb" || root == "flivmx" || root == "hmax" {
            direct_production_pl_projection_optional_scalar(projection, &format!("{root}_seed"))
        } else {
            None
        }
    })
    .or_else(|| direct_production_pl_projection_optional_ofe_scalar(projection, ofe_index, root))
}

fn direct_production_typed_lane_constructor_seed_from_projections(
    layer_seed: &DirectProductionTypedLayerSeed,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    optional_defaults: &TypedWb11OptionalDefaultsProjection,
    snow_projection: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSnowRuntimeProjection,
) -> Result<DirectProductionLaneConstructorSeed, HillslopeCliError> {
    Ok(DirectProductionLaneConstructorSeed {
        soil_water_m: layer_seed.soil_water_m,
        subsurface_layers: layer_seed.layers.clone(),
        evapotranspiration_stage_state: None,
        plant_growth_state: direct_growth_state_surface_from_pl_projection(
            management_projection,
        )?,
        plant_water_stress: optional_defaults.water_stress,
        snow_lane_state: DirectSnowLaneState::from_runtime_values(
            snow_projection.runtime_swe_m,
            snow_projection.runtime_depth_m,
            snow_projection.runtime_density_kg_m3,
            snow_projection.runtime_settle_day_count,
        ),
    })
}

fn direct_production_typed_residue_cover_authority(
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
) -> Result<DirectProductionResidueCoverAuthority, HillslopeCliError> {
    let initial_surface_residue_kg_m2 = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "sumsrm_seed",
    )?
    .unwrap_or(0.0);
    let initial_root_residue_kg_m2 = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "sumrtm_seed",
    )?
    .unwrap_or(0.0);
    let residue_type_selector = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "iresd_seed",
    )?
    .unwrap_or(0.0);
    let initial_residue_depth_m = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "frost.runtime_residue_depth_m",
    )?
    .or(direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "resdep",
    )?)
    .unwrap_or(0.0);
    let residue_depth_conversion_m_per_kg_m2 = if initial_surface_residue_kg_m2 > 0.0 {
        initial_residue_depth_m / initial_surface_residue_kg_m2
    } else {
        0.0
    };
    if !residue_depth_conversion_m_per_kg_m2.is_finite()
        || residue_depth_conversion_m_per_kg_m2 < 0.0
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} typed residue depth conversion must be finite and nonnegative, observed {residue_depth_conversion_m_per_kg_m2}"
            ),
        });
    }
    Ok(DirectProductionResidueCoverAuthority {
        initial_surface_residue_kg_m2,
        initial_root_residue_kg_m2,
        residue_type_selector,
        residue_depth_conversion_m_per_kg_m2,
    })
}

fn direct_production_typed_evapotranspiration_authority(
    soil: &TypedSoilWb11RuntimeProjection,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    optional_defaults: &TypedWb11OptionalDefaultsProjection,
    pmetpara_projection: &crate::hillslope::intake_lane_setup::TypedPmetparaRuntimeProjection,
) -> Result<DirectProductionEvapotranspirationAuthority, HillslopeCliError> {
    let pmet = if pmetpara_projection.iflget == 1 {
        None
    } else {
        let selected = pmetpara_projection.selected.ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_typed_seed",
                detail: format!(
                    "{SIMOUT_GUARD_ID} typed PMET seed requires selected pmetpara coefficients when iflget={}",
                    pmetpara_projection.iflget
                ),
            }
        })?;
        Some(DirectProductionPmetAuthority {
            kcb: selected.kcb,
            rawp: selected.rawp,
            canhgt: direct_production_pl_projection_required_ofe_scalar(
                management_projection,
                1,
                "canhgt",
            )?,
            radpot_ly: None,
            solthk_m: soil.layers.iter().map(|layer| Some(layer.solthk_m)).collect(),
        })
    };

    Ok(DirectProductionEvapotranspirationAuthority {
        leaf_area_index: direct_production_pl_projection_required_ofe_scalar(
            management_projection,
            1,
            "lai",
        )?,
        canopy_cover_fraction: direct_production_pl_projection_required_ofe_scalar(
            management_projection,
            1,
            "cancov",
        )?,
        residue_interception_m: optional_defaults.residue_interception_m,
        root_depth_m: direct_production_pl_projection_required_ofe_scalar(
            management_projection,
            1,
            "rtd",
        )?,
        plant_tolerance: direct_production_pl_projection_optional_scalar(
            management_projection,
            "swu_effective_pltol",
        )
        .or_else(|| direct_production_pl_projection_optional_scalar(management_projection, "pltol"))
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} typed ET seed missing required swu_effective_pltol/pltol"
            ),
        })?,
        priestley_taylor: DirectProductionPriestleyTaylorAuthority { salb: soil.salb },
        pmet,
    })
}

fn direct_production_pl_projection_required_ofe_scalar(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    ofe_index: usize,
    root: &'static str,
) -> Result<f64, HillslopeCliError> {
    direct_production_pl_projection_optional_ofe_scalar(projection, ofe_index, root).ok_or_else(
        || HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} typed PL projection missing required OFE {ofe_index} {root}"
            ),
        },
    )
}

fn direct_production_typed_erosion_authority(
    soil: &TypedSoilWb11RuntimeProjection,
    slope: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSlopeRuntimeProjection,
    peak_runoff: &DirectProductionPeakRunoffAuthority,
    contributor_ofe_count: usize,
) -> Result<DirectProductionErosionAuthority, HillslopeCliError> {
    let wave1_enabled = false;
    let wave2_enabled = contributor_ofe_count > 1;
    let first_ofe = slope.ofes.first().ok_or_else(|| {
        direct_production_executor_blocked("typed erosion seed requires at least one slope OFE")
    })?;
    let first_layer = soil.layers.first().ok_or_else(|| {
        direct_production_executor_blocked("typed erosion seed requires at least one soil layer")
    })?;
    let wave2_projection = direct_seed_projections::project_typed_mofe03_wave2(
        direct_seed_projections::TypedMofe03Wave2Input {
            wave2_enabled,
            slplen_m: first_ofe.slplen_m,
            qout_m3_s: 0.0,
            qin_m3_s: 0.0,
            efflen_m: Some(peak_runoff.efflen_m),
            ssa_soil: None,
            beta: direct_seed_projections::MOFE03_WAVE2_DEFAULT_BETA,
            theta: 0.5 * (first_layer.thetdr + first_layer.thetfc),
        },
    )?;
    Ok(DirectProductionErosionAuthority {
        wave2_enabled,
        erosion_inputs: DirectErosionInputs {
            wave1_enabled,
            wave2_enabled,
            wave1: DirectErod13Inputs::zero(),
            wave2: direct_production_erod14_inputs_from_typed_projection(
                &wave2_projection,
                peak_runoff.efflen_m,
            ),
        },
    })
}

fn direct_production_erod14_inputs_from_typed_projection(
    projection: &direct_seed_projections::TypedMofe03Wave2Projection,
    efflen_m: f64,
) -> DirectErod14Inputs {
    if !projection.wave2_enabled {
        return DirectErod14Inputs::zero();
    }
    let classes = projection
        .classes
        .iter()
        .map(|class| DirectErod14ClassInputs {
            fall_m_s: class.fall_m_s,
            frcflw: class.frcflw,
            frac: class.frac,
            fidel: class.fidel,
            tcf1: class.tcf1,
            ssa_class: class.ssa_class,
        })
        .collect::<Vec<_>>();
    DirectErod14Inputs {
        xtop_m: projection.xtop_m,
        xbot_m: projection.xbot_m,
        xdetst_m: projection.xdetst_m,
        ldtop_kg_s_m: projection.ldtop_kg_s_m,
        ldbot_kg_s_m: projection.ldbot_kg_s_m,
        lddend_kg: projection.lddend_kg,
        qout_m3_s: projection.qout_m3_s,
        qin_m3_s: projection.qin_m3_s,
        qostar_m: projection
            .qostar_m
            .max(direct_seed_projections::MOFE03_WAVE2_DEFAULT_QOSTAR),
        hbp_sediment_concentration_scale: efflen_m / projection.slplen_m,
        slplen_m: projection.slplen_m,
        ktrato: projection.ktrato,
        aintc: projection.aintc,
        bintc: projection.bintc,
        cintc: projection.cintc,
        beta: projection.beta,
        qj_minus_1_m3_s: projection.qj_minus_1_m3_s,
        vj_m: projection.vj_m,
        qj_m3_s: projection.qj_m3_s,
        fh_m: projection.fh_m,
        fp_m: projection.fp_m,
        case_value: projection.case_value,
        peak_runoff_m3_s: 0.0,
        runoff_duration_s: 0.0,
        ssa_soil: projection.ssa_soil,
        theta: projection.theta,
        classes,
    }
}

fn direct_production_typed_growth_authority(
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    climate_request: &HillslopeClimateRuntimeRequest,
    soil: &TypedSoilWb11RuntimeProjection,
) -> Result<DirectProductionGrowthAuthority, HillslopeCliError> {
    let Some(slot_count_value) =
        direct_production_pl_projection_optional_scalar(management_projection, "pl_schedule_slot_count")
    else {
        return Ok(DirectProductionGrowthAuthority::inactive());
    };
    let slot_count =
        direct_growth_integral_usize("pl_schedule_slot_count", slot_count_value, 1, usize::MAX)?;
    let rotation_years = direct_growth_projection_required_integral_usize(
        management_projection,
        "pl_schedule_rotation_years",
        1,
        usize::MAX,
    )?;
    let rotation_repeats = direct_growth_projection_required_integral_usize(
        management_projection,
        "pl_schedule_rotation_repeats",
        1,
        usize::MAX,
    )?;
    let mut slots = Vec::with_capacity(slot_count);
    for slot_index in 1..=slot_count {
        slots.push(direct_production_typed_growth_slot_authority(
            management_projection,
            slot_index,
        )?);
    }
    if !soil.solthk_m.is_finite() || soil.solthk_m <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} typed growth soil depth solthk must be finite and > 0.0, observed {}",
                soil.solthk_m
            ),
        });
    }
    Ok(DirectProductionGrowthAuthority {
        active: true,
        rotation_years,
        rotation_repeats,
        slots,
        monthly_temperature_max_c: climate_request.direct_monthly_max_c(),
        monthly_temperature_min_c: climate_request.direct_monthly_min_c(),
        soil_depth_m: soil.solthk_m,
    })
}

fn direct_production_typed_growth_slot_authority(
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    slot_index: usize,
) -> Result<DirectProductionGrowthSlotAuthority, HillslopeCliError> {
    let crop_slots = direct_growth_projection_required_integral_usize(
        management_projection,
        &direct_growth_schedule_slot_symbol(slot_index, "crop_slots"),
        1,
        usize::MAX,
    )?;
    let mut crops = Vec::with_capacity(crop_slots);
    for crop_slot_index in 1..=crop_slots {
        crops.push(direct_production_typed_growth_crop_authority(
            management_projection,
            slot_index,
            crop_slot_index,
        )?);
    }
    Ok(DirectProductionGrowthSlotAuthority {
        ofe_index: direct_growth_projection_required_integral_usize(
            management_projection,
            &direct_growth_schedule_slot_symbol(slot_index, "ofe_index"),
            1,
            usize::MAX,
        )?,
        year_in_rotation: direct_growth_projection_required_integral_usize(
            management_projection,
            &direct_growth_schedule_slot_symbol(slot_index, "year_in_rotation"),
            1,
            usize::MAX,
        )?,
        rotation_index: direct_growth_projection_required_integral_usize(
            management_projection,
            &direct_growth_schedule_slot_symbol(slot_index, "rotation_index"),
            1,
            usize::MAX,
        )?,
        crops,
    })
}

#[allow(clippy::too_many_lines)]
fn direct_production_typed_growth_crop_authority(
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    slot_index: usize,
    crop_slot_index: usize,
) -> Result<DirectProductionGrowthCropAuthority, HillslopeCliError> {
    let schedule_imngmt = direct_growth_projection_required_integral_u8(
        management_projection,
        &direct_growth_schedule_slot_crop_symbol(slot_index, crop_slot_index, "imngmt"),
        1,
        3,
    )?;
    let imngmt = direct_growth_projection_required_integral_u8(
        management_projection,
        &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "imngmt"),
        1,
        3,
    )?;
    let jdplt_min = usize::from(schedule_imngmt != 2);
    let jdplt = direct_growth_projection_required_integral_u16(
        management_projection,
        &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "jdplt"),
        jdplt_min,
        366,
    )?;
    let jdharv = direct_growth_projection_required_integral_u16(
        management_projection,
        &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "jdharv"),
        0,
        366,
    )?;
    let (jdstop, _mgtopt) = if schedule_imngmt == 2 {
        (
            direct_growth_projection_required_integral_u16(
                management_projection,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "jdstop"),
                0,
                366,
            )?,
            direct_growth_projection_required_integral_u8(
                management_projection,
                &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "mgtopt"),
                1,
                3,
            )?,
        )
    } else {
        (0, 1)
    };
    Ok(DirectProductionGrowthCropAuthority {
        schedule_imngmt,
        imngmt,
        jdharv,
        jdplt,
        jdstop,
        btemp: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "btemp"),
        )?,
        otemp: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "otemp"),
        )?,
        gddmax: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "gddmax"),
        )?,
        dlai: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "dlai"),
        )?,
        dropfc: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "dropfc"),
        )?,
        decfct: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "decfct"),
        )?,
        spriod: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "spriod"),
        )?,
        bb: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "bb"),
        )?,
        beinp: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "beinp"),
        )?,
        extnct: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "extnct"),
        )?,
        hi: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "hi"),
        )?,
        xmxlai: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "xmxlai"),
        )?,
        rsr: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "rsr"),
        )?,
        rtmmax: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "rtmmax"),
        )?,
        rdmax: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "rdmax"),
        )?,
        oratea: direct_growth_projection_required_scalar(
            management_projection,
            &direct_decomp_slot_crop_symbol(slot_index, crop_slot_index, "oratea"),
        )?,
        orater: direct_growth_projection_required_scalar(
            management_projection,
            &direct_decomp_slot_crop_symbol(slot_index, crop_slot_index, "orater"),
        )?,
    })
}

fn direct_growth_projection_required_scalar(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    direct_production_pl_projection_optional_scalar(projection, symbol).ok_or_else(|| {
        direct_growth_failure(format!("missing required direct growth symbol {symbol}"))
    })
}

fn direct_growth_projection_required_integral_usize(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    let value = direct_growth_projection_required_scalar(projection, symbol)?;
    direct_growth_integral_usize(symbol, value, min_allowed, max_allowed)
}

fn direct_growth_projection_required_integral_u8(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<u8, HillslopeCliError> {
    let parsed = direct_growth_projection_required_integral_usize(
        projection,
        symbol,
        min_allowed,
        max_allowed,
    )?;
    u8::try_from(parsed).map_err(|_| {
        direct_growth_failure(format!("{symbol} value {parsed} outside u8 range"))
    })
}

fn direct_growth_projection_required_integral_u16(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<u16, HillslopeCliError> {
    let parsed = direct_growth_projection_required_integral_usize(
        projection,
        symbol,
        min_allowed,
        max_allowed,
    )?;
    u16::try_from(parsed).map_err(|_| {
        direct_growth_failure(format!("{symbol} value {parsed} outside u16 range"))
    })
}

fn direct_production_typed_snow_frost_authority(
    soil: &TypedSoilWb11RuntimeProjection,
    layer_seed: &DirectProductionTypedLayerSeed,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    snow_projection: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSnowRuntimeProjection,
    frost_projection: &openwepp_hillslope_orchestrator::runtime_inputs::TypedFrostRuntimeProjection,
    climate_request: &HillslopeClimateRuntimeRequest,
) -> Result<DirectProductionSnowFrostAuthority, HillslopeCliError> {
    if snow_projection.newsnw_kg_m3 > snow_projection.ssd_kg_m3 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} snow.options.newsnw must be <= snow.options.ssd for typed direct production snow state, observed {} > {}",
                snow_projection.newsnw_kg_m3, snow_projection.ssd_kg_m3
            ),
        });
    }
    let frost_file_present = frost_projection.frost_file_present;
    let frost_wint_red_enabled = frost_projection.wint_red;
    let frost_runtime_depth_m = frost_projection.dfrost_m;
    let frost_runtime_frozen_water_m = frost_projection.ws_frz_m;
    let frost_projection_present = frost_wint_red_enabled
        || frost_file_present
        || frost_runtime_depth_m > 1.0e-12
        || frost_runtime_frozen_water_m > 1.0e-12
        || frost_projection.fine_top > 0
        || frost_projection.fine_bot > 0;
    let frost_typed_authority = if frost_projection_present {
        Some(direct_production_typed_frost_authority(
            soil,
            management_projection,
            frost_projection,
            climate_request,
            frost_file_present,
            frost_wint_red_enabled,
        )?)
    } else {
        None
    };
    let frost_layer_carry_projection = if frost_wint_red_enabled {
        Some(direct_production_typed_frost_layer_carry_projection(
            layer_seed,
            frost_projection,
        )?)
    } else {
        None
    };
    Ok(DirectProductionSnowFrostAuthority {
        snow_file_present: snow_projection.snow_file_present,
        snow_runtime_swe_m: snow_projection.runtime_swe_m,
        snow_runtime_depth_m: snow_projection.runtime_depth_m,
        snow_runtime_density_kg_m3: snow_projection.runtime_density_kg_m3,
        snow_runtime_settle_day_count: snow_projection.runtime_settle_day_count,
        snow_controls_projected: true,
        snow_density_model: snowdensity1015_default_snow_density_model()?,
        snow_phase_model: snowdensity1035_diagnostic_snow_phase_model()?,
        snow_melt_model: snowdensity1015_default_snow_melt_model()?,
        stage3_liquid_routing_model: paradigm2_stage3_liquid_routing_model()?,
        snow_rst_c: snow_projection.rst_c,
        snow_newsnw_kg_m3: snow_projection.newsnw_kg_m3,
        snow_ssd_kg_m3: snow_projection.ssd_kg_m3,
        frost_typed_authority,
        frost_layer_carry_projection,
        frost_file_present,
        frost_wint_red_enabled,
        frost_runtime_depth_m,
        frost_runtime_frozen_water_m,
        frost_active: frost_runtime_depth_m > 1.0e-12
            || frost_runtime_frozen_water_m > 1.0e-12,
    })
}

fn direct_production_typed_frost_authority(
    soil: &TypedSoilWb11RuntimeProjection,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    frost_projection: &openwepp_hillslope_orchestrator::runtime_inputs::TypedFrostRuntimeProjection,
    climate_request: &HillslopeClimateRuntimeRequest,
    frost_file_present: bool,
    frost_wint_red_enabled: bool,
) -> Result<DirectProductionFrostTypedAuthority, HillslopeCliError> {
    let layer_bulk_density_kg_m3 = soil
        .layers
        .iter()
        .enumerate()
        .map(|(offset, layer)| {
            let layer_index = offset + 1;
            if !layer.bulk_density_kg_m3.is_finite() || layer.bulk_density_kg_m3 <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_production_typed_seed",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} typed wb19_bulk_density_kg_m3_{layer_index:04} must be finite and > 0.0 for direct production frost typed solver, observed {}",
                        layer.bulk_density_kg_m3
                    ),
                });
            }
            Ok(layer.bulk_density_kg_m3)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let first_layer = soil.layers.first().ok_or_else(|| {
        direct_production_executor_blocked("typed frost authority requires at least one soil layer")
    })?;
    Ok(DirectProductionFrostTypedAuthority {
        controls: DirectFrostControlInputs {
            frost_file_present,
            wint_red_enabled: frost_wint_red_enabled,
            fine_top_count: direct_production_typed_frost_fine_count(
                "frost.options.fineTop",
                frost_projection.fine_top,
            )?,
            fine_bot_count: direct_production_typed_frost_fine_count(
                "frost.options.fineBot",
                frost_projection.fine_bot,
            )?,
            ksnowf: direct_production_positive_typed_seed(
                "frost.options.ksnowf",
                frost_projection.ksnowf,
            )?,
            kresf: direct_production_positive_typed_seed(
                "frost.options.kresf",
                frost_projection.kresf,
            )?,
            ksoilf: direct_production_positive_typed_seed(
                "frost.options.ksoilf",
                frost_projection.ksoilf,
            )?,
            kfactor1: frost_projection.kfactor1,
            kfactor2: frost_projection.kfactor2,
            kfactor3: frost_projection.kfactor3,
            landuse_class_proxy: None,
        },
        layer_bulk_density_kg_m3,
        soil_conductivity_m_s: None,
        residue_depth_m: direct_production_pl_projection_optional_nonnegative_scalar(
            management_projection,
            "frost.runtime_residue_depth_m",
        )?
        .or(direct_production_pl_projection_optional_nonnegative_scalar(
            management_projection,
            "resdep",
        )?)
        .unwrap_or(0.0),
        theta_residual: first_layer.thetdr,
        theta_field_capacity: first_layer.thetfc,
        albedo: soil.salb,
        canopy_height_m: direct_production_pl_projection_optional_nonnegative_scalar(
            management_projection,
            "canhgt",
        )?
        .unwrap_or(0.0),
        random_roughness_m: direct_production_pl_projection_optional_nonnegative_scalar(
            management_projection,
            "rrc",
        )?
        .or(direct_production_pl_projection_optional_nonnegative_scalar(
            management_projection,
            "rrinit",
        )?)
        .unwrap_or(0.0),
        // Fitted once per lane at authority construction; the normals are
        // static for the run, so the kernel carries the curve instead of
        // re-fitting per solve.
        seasonal_temperature_curve: Wb11HydrologyKernel::fit_seasonal_temperature_curve(
            &climate_request.direct_monthly_max_c(),
            &climate_request.direct_monthly_min_c(),
        ),
    })
}

fn direct_production_typed_frost_layer_carry_projection(
    layer_seed: &DirectProductionTypedLayerSeed,
    frost_projection: &openwepp_hillslope_orchestrator::runtime_inputs::TypedFrostRuntimeProjection,
) -> Result<Vec<DirectFrostLayerCarryProjection>, HillslopeCliError> {
    let fine_top_count =
        direct_production_typed_frost_fine_count("frost.options.fineTop", frost_projection.fine_top)?;
    let fine_bot_count =
        direct_production_typed_frost_fine_count("frost.options.fineBot", frost_projection.fine_bot)?;
    let layer_count = layer_seed.layers.len();
    let mut projection = Vec::with_capacity(layer_count);
    for (offset, layer) in layer_seed.layers.iter().enumerate() {
        let layer_index = offset + 1;
        if !layer.depth_m.is_finite() || layer.depth_m <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_typed_seed",
                detail: format!(
                    "{SIMOUT_GUARD_ID} typed wb19_dg_{layer_index:04} must be finite and > 0.0, observed {}",
                    layer.depth_m
                ),
            });
        }
        let fine_layer_count = direct_publication_frost_fine_layer_count(
            layer_index,
            layer_count,
            layer.depth_m,
            fine_top_count,
            fine_bot_count,
        )?;
        let fine_layer_thickness_m =
            layer.depth_m / usize_to_scalar("frost.runtime_nfine", fine_layer_count)?;
        projection.push(DirectFrostLayerCarryProjection {
            layer_index,
            fine_layer_count,
            fine_layer_thickness_m,
        });
    }
    Ok(projection)
}

fn direct_production_typed_frost_fine_count(
    symbol: &'static str,
    value: i32,
) -> Result<usize, HillslopeCliError> {
    if !(1..=10).contains(&value) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!("{SIMOUT_GUARD_ID} {symbol} must be an integer in [1,10], observed {value}"),
        });
    }
    usize::try_from(value).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!("{SIMOUT_GUARD_ID} {symbol} could not convert to usize: {value}"),
        }
    })
}

fn direct_production_positive_typed_seed(
    symbol: &'static str,
    value: f64,
) -> Result<f64, HillslopeCliError> {
    if value.is_finite() && value > 0.0 {
        return Ok(value);
    }
    Err(HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_typed_seed",
        detail: format!(
            "{SIMOUT_GUARD_ID} {symbol} must be finite and > 0.0 for typed direct production seed, observed {value}"
        ),
    })
}

fn direct_production_typed_infiltration_authority(
    frost_projection: &openwepp_hillslope_orchestrator::runtime_inputs::TypedFrostRuntimeProjection,
) -> DirectProductionInfiltrationAuthority {
    DirectProductionInfiltrationAuthority {
        effective_conductivity_m_s: Some(frost_projection.infcap_frz_m_s),
        matric_potential_m: None,
        depression_storage_capacity_m: 0.0,
    }
}

fn direct_production_typed_percolation_inputs(
    soil: &TypedSoilWb11RuntimeProjection,
    layer_seed: &DirectProductionTypedLayerSeed,
    lane_substeps_f64: f64,
) -> Result<DirectPercolationInputs, HillslopeCliError> {
    let lane_substeps = scalar_to_usize("wb18_perc_lane_substeps", lane_substeps_f64)?;
    let restrictive_layer_enabled = soil
        .restrictive_layer
        .as_ref()
        .is_some_and(|restrictive| restrictive.slflag);
    let restrictive_layer_conductivity_m_s = if restrictive_layer_enabled {
        soil.restrictive_layer
            .as_ref()
            .map_or(0.0, |restrictive| restrictive.kslast_m_s)
    } else {
        0.0
    };
    let restrictive_layer_thickness_m = if restrictive_layer_enabled && lane_substeps > 1 {
        soil.restrictive_layer
            .as_ref()
            .map_or(0.0, |restrictive| restrictive.ui_bdrkth_m)
    } else {
        0.0
    };
    Ok(DirectPercolationInputs {
        soil_water_initial_m: layer_seed.soil_water_m,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps,
        restrictive_layer_enabled,
        restrictive_layer_conductivity_m_s,
        restrictive_layer_thickness_m,
        layers: layer_seed.layers.clone(),
    })
}

fn direct_production_typed_subsurface_inputs(
    soil: &TypedSoilWb11RuntimeProjection,
    slope: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSlopeRuntimeProjection,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    layer_seed: &DirectProductionTypedLayerSeed,
    lane_substeps_f64: f64,
) -> Result<DirectSubsurfaceComputeInputs, HillslopeCliError> {
    let lane_substeps = scalar_to_usize("wb19_lateral_drain_lane_substeps", lane_substeps_f64)?;
    let slope_ofe = slope.ofes.first().ok_or_else(|| {
        direct_production_executor_blocked("typed subsurface seed requires at least one slope OFE")
    })?;
    let drain_enabled = direct_production_pl_projection_optional_flag(
        management_projection,
        "wb19_drain_enabled",
    )?
    .unwrap_or(false);
    let drain_depth_m = if drain_enabled {
        direct_production_pl_projection_required_scalar(
            management_projection,
            "wb19_drain_depth",
        )?
    } else {
        0.5
    };
    let drain_spacing_m = if drain_enabled {
        direct_production_pl_projection_required_scalar(
            management_projection,
            "wb19_drain_spacing",
        )?
    } else {
        1.0
    };
    let drain_diameter_m = if drain_enabled {
        direct_production_pl_projection_required_scalar(
            management_projection,
            "wb19_drain_diameter",
        )?
    } else {
        0.1
    };
    Ok(DirectSubsurfaceComputeInputs {
        avg_slope: slope_ofe.avgslp,
        slope_length_m: slope_ofe.slplen_m,
        lateral_anisotropy_ratio: soil.lateral_anisotropy_ratio,
        soil_depth_m: layer_seed.layers.iter().map(|layer| layer.depth_m).sum(),
        solwpv_mode: scalar_to_i32("solwpv", soil.solwpv)?,
        mofe_hourly_carry_arrays_enabled: lane_substeps == 24,
        lane_substeps,
        drainage_capacity_m: 0.0,
        drain_enabled,
        drain_depth_m,
        drain_spacing_m,
        drain_diameter_m,
        layers: layer_seed.layers.iter().cloned().map(Into::into).collect(),
    })
}

fn direct_production_typed_hydrology_projection_inputs(
    soil: &TypedSoilWb11RuntimeProjection,
    _layer_seed: &DirectProductionTypedLayerSeed,
) -> Result<DirectHydrologyProjectionInputs, HillslopeCliError> {
    let profile_depth_m = direct_production_typed_static_mm_to_m(
        soil.profile_depth_mm,
        "wb13_profile_depth_mm",
        true,
    )?;
    let profile_porosity_cap_m = direct_production_typed_static_mm_to_m(
        soil.profile_porosity_cap_mm,
        "wb13_profile_porosity_cap_mm",
        false,
    )?;
    let profile_fc_tail_m = soil.profile_fc_tail_mm.unwrap_or(0.0) / 1_000.0;
    let profile_field_capacity_m = soil
        .layers
        .iter()
        .map(|layer| layer.thetfc * layer.dg_m)
        .sum::<f64>()
        + profile_fc_tail_m;
    let profile_wilting_point_m = direct_production_typed_static_mm_to_m(
        soil.profile_wp_store_mm,
        "wb13_profile_wp_store_mm",
        false,
    )?;
    if profile_porosity_cap_m < profile_field_capacity_m {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} typed parsed profile porosity cap must be >= field capacity store"
            ),
        });
    }
    if profile_field_capacity_m < profile_wilting_point_m {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} typed parsed profile field capacity store must be >= wilting point store"
            ),
        });
    }
    Ok(DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 1.0e-9,
        profile_depth_m: Some(profile_depth_m),
        profile_porosity_cap_m: Some(profile_porosity_cap_m),
        profile_field_capacity_m: Some(profile_field_capacity_m),
        profile_wilting_point_m: Some(profile_wilting_point_m),
        ..DirectHydrologyProjectionInputs::zero()
    })
}

fn direct_production_typed_static_mm_to_m(
    value_mm: Option<f64>,
    symbol: &'static str,
    require_positive: bool,
) -> Result<f64, HillslopeCliError> {
    let value_mm = value_mm.ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_typed_seed",
        detail: format!("{SIMOUT_GUARD_ID} typed seed missing required {symbol}"),
    })?;
    if !value_mm.is_finite()
        || if require_positive {
            value_mm <= 0.0
        } else {
            value_mm < 0.0
        }
    {
        let comparator = if require_positive { "> 0.0" } else { ">= 0.0" };
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} typed parsed direct publication profile symbol {symbol} must be finite and {comparator}, observed {value_mm}"
            ),
        });
    }
    Ok(value_mm / 1_000.0)
}

fn direct_production_pl_projection_optional_flag(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &'static str,
) -> Result<Option<bool>, HillslopeCliError> {
    direct_production_pl_projection_optional_scalar(projection, symbol)
        .map(|value| direct_publication_parse_enabled_flag(symbol, value))
        .transpose()
}

fn direct_production_pl_projection_required_scalar(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &'static str,
) -> Result<f64, HillslopeCliError> {
    direct_production_pl_projection_optional_scalar(projection, symbol).ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!("{SIMOUT_GUARD_ID} typed PL projection missing required {symbol}"),
        }
    })
}

fn direct_production_pl_projection_optional_nonnegative_scalar(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &'static str,
) -> Result<Option<f64>, HillslopeCliError> {
    direct_production_pl_projection_optional_scalar(projection, symbol)
        .map(|value| {
            if value.is_finite() && value >= 0.0 {
                Ok(value)
            } else {
                Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_production_typed_seed",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} typed PL projection {symbol} must be finite and >= 0.0, observed {value}"
                    ),
                })
            }
        })
        .transpose()
}

fn direct_production_pl_projection_optional_ofe_scalar(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    ofe_index: usize,
    root: &str,
) -> Option<f64> {
    direct_production_pl_projection_optional_scalar(projection, &format!("ofe{ofe_index}_{root}"))
        .or_else(|| {
            if ofe_index == 1 {
                direct_production_pl_projection_optional_scalar(projection, root)
            } else {
                None
            }
        })
        .or_else(|| {
            direct_production_pl_projection_optional_scalar(
                projection,
                &format!("pl_growth_ofe{ofe_index}_{root}_seed"),
            )
        })
        .or_else(|| {
            if ofe_index == 1 {
                direct_production_pl_projection_optional_scalar(projection, &format!("{root}_seed"))
            } else {
                None
            }
        })
}

fn direct_production_pl_projection_optional_scalar(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &str,
) -> Option<f64> {
    let key = BoundarySymbol::from(symbol);
    projection
        .pl_schedule_surface
        .get(&key)
        .or_else(|| projection.pl_growth_surface.get(&key))
        .or_else(|| projection.pl_decomp_surface.get(&key))
        .map(|value| (*value).as_f64())
}

fn direct_growth_state_surface_from_pl_projection(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
) -> Result<DirectGrowthStateSurface, HillslopeCliError> {
    let state = &projection.pl_growth_surface;
    let value = |symbol: &'static str| -> Result<f64, HillslopeCliError> {
        state
            .get(&BoundarySymbol::from(symbol))
            .map(|value: &BoundaryValue| (*value).as_f64())
            .filter(|value: &f64| value.is_finite())
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_typed_seed",
                detail: format!(
                    "{SIMOUT_GUARD_ID} typed PL projection missing finite {symbol}"
                ),
            })
    };
    let vdmt = value("vdmt")?;
    let hia = value("hia")?;
    Ok(DirectGrowthStateSurface {
        sumgdd: value("sumgdd")?,
        live_biomass_kg_m2: vdmt,
        interception_live_biomass_kg_m2: if hia > 0.0 {
            (vdmt * (1.0 - hia)).max(0.0)
        } else {
            vdmt
        },
        canopy_cover_fraction: value("cancov")?,
        leaf_area_index: value("lai")?,
        root_mass_kg_m2: value("rtmass")?,
        root_depth_m: value("rtd")?,
        harvest_index: hia,
    })
}

fn direct_production_typed_erod14_wave2_enabled(
    inputs: &ParsedHillslopeRunInputs,
    lane_count: usize,
) -> Result<bool, HillslopeCliError> {
    let ofe_count = if lane_count == 0 {
        inputs.slope.ofe_count
    } else {
        lane_count
    };
    if ofe_count == 0 {
        return Err(direct_production_executor_blocked(
            "typed Wave-2 seed requires at least one OFE",
        ));
    }
    Ok(ofe_count > 1)
}

fn direct_production_typed_layer_seed(
    soil: &TypedSoilWb11RuntimeProjection,
    execution_lane: ExecutionLane,
) -> Result<DirectProductionTypedLayerSeed, HillslopeCliError> {
    let layer_inputs = soil
        .layers
        .iter()
        .map(|layer| TypedWb11LayerSeedInput {
            dg: layer.dg_m,
            thetfc: layer.thetfc,
            thetdr: layer.thetdr,
            ssc: layer.ssc_m_s,
            por: layer.porosity,
            cpm: layer.cpm,
        })
        .collect::<Vec<_>>();
    let storage = project_typed_wb11_initial_storage(soil.sat, execution_lane, &layer_inputs)?;
    let frost_refresh_layers = soil
        .layers
        .iter()
        .map(|layer| TypedWb11FrozenDepthLayerInput {
            depth_m: layer.dg_m,
            fine_frozen_depths_m: None,
        })
        .collect::<Vec<_>>();
    let frozen_depths = project_typed_wb11_frozen_depth_refresh(Some(0.0), &frost_refresh_layers)?;

    let layers = soil
        .layers
        .iter()
        .zip(storage.layers.iter())
        .zip(frozen_depths.frozen_depths_m.iter().copied())
        .map(|((layer, store), frozen_depth_m)| DirectSubsurfaceLayerState {
            theta_m: store.theta,
            field_capacity_m: store.field_capacity,
            upper_limit_m: store.upper_limit,
            conductivity_m_s: layer.ssc_m_s,
            depth_m: layer.dg_m,
            residual_theta: layer.thetdr,
            frozen_depth_m,
            frozen_water_m: 0.0,
            porosity: layer.porosity,
            field_capacity_theta: layer.thetfc,
            coca: layer.coca,
            lateral_conductivity_m_s: layer.lateral_ssh_m_s,
        })
        .collect();

    Ok(DirectProductionTypedLayerSeed {
        soil_water_m: storage.totals.soil_water,
        layers,
    })
}

fn direct_production_frost_storage_liquid_delta(
    frost_outcome: &DirectWinterFrostPartitionOutcome,
) -> Option<f64> {
    const MATERIAL_FROST_THRESHOLD_M: f64 = 1.0e-12;
    if frost_outcome.frwatc_net_liquid_delta_m.abs() <= MATERIAL_FROST_THRESHOLD_M {
        return None;
    }
    Some(frost_outcome.frwatc_net_liquid_delta_m)
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy)]
struct DirectProductionEvappmSeed {
    et_demand_m: f64,
    soil_evaporation_m: f64,
    plant_transpiration_m: f64,
    soil_evaporation_storage_return_m: f64,
}

#[allow(dead_code)]
impl<'a> DirectProductionDayInputBuilder<'a> {
    fn new(
        climate_request: &'a HillslopeClimateRuntimeRequest,
        climate_span: &'a ClimateRunSpanSummary,
        seed_authority: &DirectProductionSeedAuthority,
    ) -> Result<Self, HillslopeCliError> {
        if seed_authority.lanes.is_empty() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production requires at least one lane seed authority"
                ),
            });
        }
        let lane_authority = seed_authority
            .lanes
            .iter()
            .map(|lane| lane.day_input.clone())
            .collect::<Vec<_>>();
        let sturm_climate_class = direct_production_sturm_climate_class_for_density_candidate(
            climate_request,
            climate_span,
            &lane_authority,
        )?;
        let residue_cover_state = lane_authority
            .iter()
            .map(|authority| authority.residue_cover.initial_state())
            .collect::<Vec<_>>();
        Ok(Self {
            climate_request,
            climate_span,
            lane_authority,
            residue_cover_state: std::cell::RefCell::new(residue_cover_state),
            winter_hourly_geometry: seed_authority.winter_hourly_geometry,
            sturm_climate_class,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn build(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<DirectPublicationDayInput, HillslopeCliError> {
        let (day, simulation_year, forcing) = self.climate_day_for_build(day_index)?;
        let lane = Self::frame_lane_for_build(frame, lane_index)?;
        let authority = self.lane_authority(lane_index)?;
        let precipitation_m = forcing.prcp_m;
        let mut hyetograph = direct_production_hyetograph(&forcing)?;
        let rainfall_input_m = direct_publication_hyetograph_rainfall_m(&hyetograph)?;
        let snow_lane_state = authority.snow_frost.current_snow_lane_state(lane);
        let growth_state_before = *lane.plant_growth_state;
        let pre_growth_evapotranspiration_compute_inputs =
            authority.evapotranspiration.inputs_with_growth_surface(
                &day,
                &forcing,
                lane.evapotranspiration_stage_state.as_deref().copied(),
                &lane.subsurface_layers,
                self.climate_request,
                growth_state_before,
            )?;
        let (annual_growth_inputs, perennial_growth_inputs) = authority.growth.inputs(
            &day,
            simulation_year,
            lane_index + 1,
            &forcing,
            growth_state_before,
            lane.plant_water_stress,
            &pre_growth_evapotranspiration_compute_inputs,
        )?;
        let growth_state_for_publication = direct_production_growth_state_for_publication(
            &annual_growth_inputs,
            &perennial_growth_inputs,
            growth_state_before,
        )?;
        let residue_cover_projection = self.residue_cover_projection_for_build(
            authority,
            day,
            simulation_year,
            lane_index,
            &forcing,
            growth_state_before,
            growth_state_for_publication,
            lane.plant_water_stress,
        )?;
        maybe_write_frost_residue_cover_trace(day_index, lane_index, &residue_cover_projection)?;
        Self::validate_active_snow_forcing(
            authority,
            lane_index,
            &forcing,
            rainfall_input_m,
            snow_lane_state.runtime_swe_m,
        )?;
        let sturm_day_of_year = self.sturm_climate_class.map(|_| f64::from(day.julian_day));
        let snow_liquid = authority.snow_frost.snow_liquid_partition(
            self.climate_request,
            day_index,
            &forcing,
            rainfall_input_m,
            &snow_lane_state,
            growth_state_for_publication.canopy_cover_fraction,
            self.sturm_climate_class,
            sturm_day_of_year,
            self.winter_hourly_geometry,
        )?;
        maybe_write_r7h_direct_production_snow_trace(
            day_index,
            lane_index,
            rainfall_input_m,
            &snow_lane_state,
            authority.snow_frost.snow_melt_model,
            authority.snow_frost.snow_phase_model,
            &snow_liquid,
        )?;
        let frost_context = authority.snow_frost.frost_day_context(
            self.climate_request,
            day_index,
            &day,
            lane_index,
            lane,
            &forcing,
            &snow_lane_state,
            self.winter_hourly_geometry,
            rainfall_input_m > 1.0e-12 || snow_liquid.routed_melt_m > 1.0e-12,
            Some(residue_cover_projection.state_after.residue_depth_m),
        )?;
        let interception_state = compute_direct_canopy_interception(
            DirectCanopyInterceptionInputs {
                hyetograph_rainfall_m: snow_liquid.post_winter_rain_m,
                interception_rainfall_input_m: snow_liquid.post_winter_rain_m,
                canopy_cover_fraction: growth_state_for_publication.canopy_cover_fraction,
                leaf_area_index: growth_state_for_publication.leaf_area_index,
                interception_live_biomass_kg_m2: direct_growth_interception_live_biomass_from_state(
                    growth_state_for_publication,
                )?,
            },
        )
        .map_err(|source| direct_publication_runtime_error(&source))?;
        maybe_write_r7h_direct_production_wb15_trace(
            day_index,
            lane_index,
            growth_state_before,
            growth_state_for_publication,
            snow_liquid.post_winter_rain_m,
            interception_state,
        )?;
        let post_winter_hyetograph = direct_publication_scaled_hyetograph_to_rainfall(
            &hyetograph,
            snow_liquid.post_winter_rain_m,
        )?;
        let post_interception_hyetograph = direct_publication_scaled_hyetograph(
            &post_winter_hyetograph,
            interception_state.rainfall_scale,
        )?;
        hyetograph = direct_publication_hyetograph_with_added_daily_depth(
            &post_interception_hyetograph,
            snow_liquid.routed_melt_m,
        )?;
        let hydrology_layers = frost_context
            .as_ref()
            .map_or(lane.subsurface_layers.as_slice(), |context| {
                context.hydrology_layers.as_slice()
            });

        let mut day_input =
            DirectPublicationDayInput::calendar_only(direct_publication_calendar_day(&day)?);
        day_input.precipitation_m = precipitation_m;
        day_input.effective_temperature_c = day.effective_temperature_c;
        day_input.interception_m = interception_state.interception_m;
        day_input.canopy_cover_fraction = Some(growth_state_for_publication.canopy_cover_fraction);
        day_input.initial_soil_water_m = Some(direct_production_lane_soil_water(lane, lane_index)?);
        day_input.storage_input_inputs = Some(DirectStorageInputInputs {
            precip_input_handoff_m: Some(precipitation_m),
        });
        day_input.liquid_input_inputs =
            Some(direct_publication_liquid_input_inputs(
                interception_state.liquid_after_interception_m + snow_liquid.routed_melt_m,
            )?);
        day_input.snow_coupling_inputs = Some(DirectSnowCouplingInputs {
            snow_coupling_handoff_m: snow_liquid.snow_coupling_signed_s_m,
            snow_state_projected: authority.snow_frost.snow_state_projected(&snow_lane_state),
            active_snow_coupling: snow_liquid.active_snow_coupling,
            raw_melt_m: snow_liquid.raw_melt_m,
            redistributed_melt_m: snow_liquid.redistributed_melt_m,
            routed_melt_m: snow_liquid.routed_melt_m,
            snowpack_swe_loss_m: snow_liquid.snowpack_swe_loss_m,
            sublimation_m: snow_liquid.sublimation_m,
            post_winter_rain_m: snow_liquid.post_winter_rain_m,
            runtime_swe_after_m: snow_liquid.runtime_swe_after_m,
            runtime_depth_after_m: snow_liquid.runtime_depth_after_m,
            runtime_density_after_kg_m3: snow_liquid.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_liquid.runtime_settle_day_count_after,
            coe_boundary_depth_after_m: snow_liquid.coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: snow_liquid.coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: snow_liquid
                .coe_boundary_settle_day_count_after,
            liquid_holding_capacity_after_m: snow_liquid.liquid_holding_capacity_after_m,
            liquid_water_retained_after_m: snow_liquid.liquid_water_retained_after_m,
            liquid_water_released_m: snow_liquid.liquid_water_released_m,
            snow_albedo_state_after: snow_liquid.snow_albedo_state_after,
            snow_layers_after: snow_liquid.snow_layers_after.clone(),
            stage3_diagnostics: snow_liquid.stage3_diagnostics.boxed_when_enabled(),
        });
        day_input.peak_runoff_inputs = Some(authority.peak_runoff.inputs(hyetograph.clone()));
        day_input.infiltration_depression_inputs = Some(
            authority
                .infiltration
                .inputs(
                    lane_index,
                    hydrology_layers,
                    hyetograph,
                    frost_context
                        .as_ref()
                        .map(|context| context.frozen_infiltration_capacity_m_s),
                )?,
        );
        day_input.percolation_inputs =
            Some(authority.percolation_inputs(lane_index, lane, hydrology_layers)?);
        day_input.subsurface_compute_inputs =
            Some(authority.subsurface_inputs(lane_index, hydrology_layers)?);
        let evapotranspiration_compute_inputs = pre_growth_evapotranspiration_compute_inputs;
        day_input.evapotranspiration_compute_inputs = Some(evapotranspiration_compute_inputs);
        day_input.decomposition_inputs = Some(residue_cover_projection.decomposition_inputs);
        day_input.residue_partition_inputs = Some(residue_cover_projection.residue_partition_inputs);
        day_input.annual_growth_inputs = Some(annual_growth_inputs);
        day_input.perennial_growth_inputs = Some(perennial_growth_inputs);
        let mut hydrology_projection_inputs =
            authority.hydrology_projection_inputs(hydrology_layers);
        hydrology_projection_inputs.snow_water_m = snow_liquid.runtime_swe_after_m;
        day_input.hydrology_projection_inputs = Some(hydrology_projection_inputs);
        let erosion_active = direct_production_erosion_active(authority, &day_input)?;
        apply_direct_production_erosion_inputs(&mut day_input, authority, erosion_active);
        apply_direct_production_frost_context(&mut day_input, frost_context);
        day_input.frost_runtime_carry =
            direct_publication_frost_runtime_carry_from_lane_state(&lane.winter_column.frost);
        Ok(day_input)
    }

    fn climate_day_for_build(
        &self,
        day_index: usize,
    ) -> Result<(ClimateDayProjection, i32, HillslopeDirectClimateDayForcing), HillslopeCliError>
    {
        let day = *self.climate_span.days.get(day_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production day index {} exceeds climate span {}",
                    day_index + 1,
                    self.climate_span.days.len()
                ),
            }
        })?;
        direct_publication_validate_day(&day)?;
        let simulation_year =
            simulation_year_from_calendar_year(day.year, self.climate_span.first_day.year)?;
        let forcing =
            self.climate_request
                .direct_day_forcing(day_index)
                .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "direct_publication_frame",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} direct production typed climate forcing failed: {source}"
                    ),
                })?;
        Ok((day, simulation_year, forcing))
    }

    fn frame_lane_for_build(
        frame: &DirectRunFrame,
        lane_index: usize,
    ) -> Result<&DirectLaneFrame, HillslopeCliError> {
        frame
            .lanes
            .get(lane_index)
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production lane index {} exceeds frame lane count {}",
                    lane_index + 1,
                    frame.lanes.len()
                ),
            })
    }

    #[allow(clippy::too_many_arguments)]
    fn residue_cover_projection_for_build(
        &self,
        authority: &DirectProductionLaneDayInputAuthority,
        day: ClimateDayProjection,
        simulation_year: i32,
        lane_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        growth_state_before: DirectGrowthStateSurface,
        growth_state_for_publication: DirectGrowthStateSurface,
        plant_water_stress: f64,
    ) -> Result<DirectProductionResidueCoverProjection, HillslopeCliError> {
        let mut states = self.residue_cover_state.borrow_mut();
        if lane_index >= states.len() {
            states.resize(lane_index + 1, authority.residue_cover.initial_state());
        }
        let projection = authority.residue_cover.project_day(
            &authority.growth,
            &day,
            simulation_year,
            lane_index + 1,
            forcing,
            states[lane_index],
            growth_state_before,
            growth_state_for_publication,
            plant_water_stress,
        )?;
        states[lane_index] = projection.state_after;
        Ok(projection)
    }

    fn lane_authority(
        &self,
        lane_index: usize,
    ) -> Result<&DirectProductionLaneDayInputAuthority, HillslopeCliError> {
        if self.lane_authority.len() == 1 {
            return Ok(&self.lane_authority[0]);
        }
        self.lane_authority.get(lane_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct production lane index {} exceeds lane authority count {}",
                    lane_index + 1,
                    self.lane_authority.len()
                ),
            }
        })
    }

    fn validate_active_snow_forcing(
        authority: &DirectProductionLaneDayInputAuthority,
        lane_index: usize,
        forcing: &HillslopeDirectClimateDayForcing,
        hyetograph_rainfall_m: f64,
        runtime_swe_m: f64,
    ) -> Result<(), HillslopeCliError> {
        let _active_snow = authority
            .snow_frost
            .active_forcing(forcing, hyetograph_rainfall_m, runtime_swe_m)?;
        let _ = lane_index;
        Ok(())
    }
}

fn maybe_write_r7h_direct_production_snow_trace(
    day_index: usize,
    lane_index: usize,
    hyetograph_rainfall_m: f64,
    snow_lane_state: &openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel,
    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> Result<(), HillslopeCliError> {
    let Some(path) = std::env::var_os("OPENWEPP_R7H_SNOW_TRACE_PATH") else {
        return Ok(());
    };
    if path.is_empty() {
        return Ok(());
    }
    if let Some(filter_day_index) =
        direct_production_trace_env_usize("OPENWEPP_R7H_SNOW_TRACE_DAY_INDEX")
        && filter_day_index != day_index
    {
        return Ok(());
    }
    if let Some(filter_lane_index) =
        direct_production_trace_env_usize("OPENWEPP_R7H_SNOW_TRACE_LANE_INDEX")
        && filter_lane_index != lane_index
    {
        return Ok(());
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed opening direct production snow trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        })?;
    let line = r7h_direct_production_snow_trace_line(
        day_index,
        lane_index,
        hyetograph_rainfall_m,
        snow_lane_state,
        snow_melt_model,
        snow_phase_model,
        snow_liquid,
    );
    std::io::Write::write_all(&mut file, line.as_bytes()).map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_snow_trace",
        detail: format!(
            "{SIMOUT_GUARD_ID} failed writing direct production snow trace {}: {error}",
            std::path::PathBuf::from(&path).display()
        ),
    })
}

fn r7h_direct_production_snow_trace_line(
    day_index: usize,
    lane_index: usize,
    hyetograph_rainfall_m: f64,
    snow_lane_state: &openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel,
    snow_phase_model: openwepp_hillslope_orchestrator::SnowPhasePartitionModel,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> String {
    let layer = direct_snow_trace_layer_diagnostics(snow_lane_state, snow_liquid);
    let line = format!(
        "{{\"schema\":\"openwepp-r7h-direct-production-snow-trace-v1\",\
\"day_index\":{day_index},\
\"lane_index\":{lane_index},\
\"hyetograph_rainfall_m\":{},\
\"runtime_swe_before_m\":{},\
\"runtime_depth_before_m\":{},\
\"runtime_density_before_kg_m3\":{},\
\"runtime_settle_day_count_before\":{},\
\"liquid_water_retained_before_m\":{},\
\"snow_layer_count_before\":{},\
\"snow_layer_swe_sum_before_m\":{},\
\"snow_layer_depth_sum_before_m\":{},\
\"snow_layer_surface_density_before_kg_m3\":{},\
\"snow_layer_basal_density_before_kg_m3\":{},\
\"snow_layer_density_gradient_before_kg_m3\":{},\
\"snow_density_model\":\"{}\",\
\"snow_melt_model\":\"{}\",\
\"snow_phase_model\":\"{}\",\
\"active_snow_coupling\":{},\
\"snow_coupling_signed_s_m\":{},\
\"raw_melt_m\":{},\
\"snowpack_swe_loss_m\":{},\
\"accumulation_m\":{},\
\"sublimation_m\":{},\
\"routed_melt_m\":{},\
\"rain_retained_m\":{},\
\"rain_released_m\":{},\
\"liquid_holding_capacity_after_m\":{},\
\"liquid_water_retained_after_m\":{},\
\"liquid_water_released_m\":{},\
\"post_winter_rain_m\":{},\
\"runtime_swe_after_m\":{},\
\"runtime_depth_after_m\":{},\
\"runtime_density_after_kg_m3\":{},\
\"runtime_settle_day_count_after\":{},\
\"snow_layer_count_after\":{},\
\"snow_layer_swe_sum_after_m\":{},\
\"snow_layer_depth_sum_after_m\":{},\
\"snow_layer_surface_density_after_kg_m3\":{},\
\"snow_layer_basal_density_after_kg_m3\":{},\
\"snow_layer_density_gradient_after_kg_m3\":{}}}",
        direct_production_trace_number(hyetograph_rainfall_m),
        direct_production_trace_number(snow_lane_state.runtime_swe_m),
        direct_production_trace_number(snow_lane_state.runtime_depth_m),
        direct_production_trace_number(snow_lane_state.runtime_density_kg_m3),
        direct_production_trace_number(snow_lane_state.runtime_settle_day_count),
        direct_production_trace_number(snow_lane_state.liquid_water_retained_m),
        layer.count_before,
        direct_production_trace_number(layer.swe_sum_before_m),
        direct_production_trace_number(layer.depth_sum_before_m),
        direct_production_trace_number(layer.surface_density_before_kg_m3),
        direct_production_trace_number(layer.basal_density_before_kg_m3),
        direct_production_trace_number(layer.density_gradient_before_kg_m3),
        snow_liquid.snow_density_model.id(),
        snow_melt_model.id(),
        snow_phase_model.id(),
        snow_liquid.active_snow_coupling,
        direct_production_trace_number(snow_liquid.snow_coupling_signed_s_m),
        direct_production_trace_number(snow_liquid.raw_melt_m),
        direct_production_trace_number(snow_liquid.snowpack_swe_loss_m),
        direct_production_trace_number(snow_liquid.accumulation_m),
        direct_production_trace_number(snow_liquid.sublimation_m),
        direct_production_trace_number(snow_liquid.routed_melt_m),
        direct_production_trace_number(snow_liquid.rain_retained_m),
        direct_production_trace_number(snow_liquid.rain_released_m),
        direct_production_trace_number(snow_liquid.liquid_holding_capacity_after_m),
        direct_production_trace_number(snow_liquid.liquid_water_retained_after_m),
        direct_production_trace_number(snow_liquid.liquid_water_released_m),
        direct_production_trace_number(snow_liquid.post_winter_rain_m),
        direct_production_trace_number(snow_liquid.runtime_swe_after_m),
        direct_production_trace_number(snow_liquid.runtime_depth_after_m),
        direct_production_trace_number(snow_liquid.runtime_density_after_kg_m3),
        direct_production_trace_number(snow_liquid.runtime_settle_day_count_after),
        layer.count_after,
        direct_production_trace_number(layer.swe_sum_after_m),
        direct_production_trace_number(layer.depth_sum_after_m),
        direct_production_trace_number(layer.surface_density_after_kg_m3),
        direct_production_trace_number(layer.basal_density_after_kg_m3),
        direct_production_trace_number(layer.density_gradient_after_kg_m3),
    );
    format!("{line}\n")
}

struct DirectSnowTraceLayerDiagnostics {
    count_before: usize,
    count_after: usize,
    swe_sum_before_m: f64,
    swe_sum_after_m: f64,
    depth_sum_before_m: f64,
    depth_sum_after_m: f64,
    surface_density_before_kg_m3: f64,
    basal_density_before_kg_m3: f64,
    density_gradient_before_kg_m3: f64,
    surface_density_after_kg_m3: f64,
    basal_density_after_kg_m3: f64,
    density_gradient_after_kg_m3: f64,
}

fn direct_snow_trace_layer_diagnostics(
    snow_lane_state: &openwepp_hillslope_orchestrator::DirectSnowLaneState,
    snow_liquid: &openwepp_hillslope_orchestrator::DirectSnowLiquidPartition,
) -> DirectSnowTraceLayerDiagnostics {
    let (surface_before, basal_before, gradient_before) =
        snow_layer_density_profile(&snow_lane_state.layers);
    let (surface_after, basal_after, gradient_after) =
        snow_layer_density_profile(&snow_liquid.snow_layers_after);
    DirectSnowTraceLayerDiagnostics {
        count_before: snow_lane_state.layers.len(),
        count_after: snow_liquid.snow_layers_after.len(),
        swe_sum_before_m: snow_layer_swe_sum(&snow_lane_state.layers),
        swe_sum_after_m: snow_layer_swe_sum(&snow_liquid.snow_layers_after),
        depth_sum_before_m: snow_layer_depth_sum(&snow_lane_state.layers),
        depth_sum_after_m: snow_layer_depth_sum(&snow_liquid.snow_layers_after),
        surface_density_before_kg_m3: surface_before,
        basal_density_before_kg_m3: basal_before,
        density_gradient_before_kg_m3: gradient_before,
        surface_density_after_kg_m3: surface_after,
        basal_density_after_kg_m3: basal_after,
        density_gradient_after_kg_m3: gradient_after,
    }
}

fn snow_layer_swe_sum(
    layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState],
) -> f64 {
    layers.iter().map(|layer| layer.mass_swe_m).sum()
}

fn snow_layer_depth_sum(
    layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState],
) -> f64 {
    layers.iter().map(|layer| layer.thickness_m).sum()
}

fn snow_layer_density_profile(
    layers: &[openwepp_hillslope_orchestrator::DirectSnowLayerState],
) -> (f64, f64, f64) {
    let Some(surface) = layers.first() else {
        return (0.0, 0.0, 0.0);
    };
    let basal = layers.last().unwrap_or(surface);
    let surface_density = surface.density_kg_m3;
    let basal_density = basal.density_kg_m3;
    (surface_density, basal_density, basal_density - surface_density)
}

fn maybe_write_r7h_direct_production_wb15_trace(
    day_index: usize,
    lane_index: usize,
    growth_state_before: DirectGrowthStateSurface,
    growth_state_for_publication: DirectGrowthStateSurface,
    post_winter_rain_m: f64,
    interception_state: openwepp_hillslope_orchestrator::DirectCanopyInterceptionState,
) -> Result<(), HillslopeCliError> {
    let Some(path) = direct_production_trace_output_path("OPENWEPP_R7H_WB15_TRACE_PATH") else {
        return Ok(());
    };
    if !direct_production_trace_filters_allow(
        day_index,
        lane_index,
        "OPENWEPP_R7H_WB15_TRACE_DAY_INDEX",
        "OPENWEPP_R7H_WB15_TRACE_LANE_INDEX",
    ) {
        return Ok(());
    }
    let projected_interception_live_biomass_kg_m2 =
        direct_growth_interception_live_biomass_from_state(growth_state_for_publication)?;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_wb15_trace",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed opening direct production WB15 trace {}: {error}",
                std::path::PathBuf::from(&path).display()
            ),
        })?;
    let line = format!(
        "{{\"schema\":\"openwepp-r7h-direct-production-wb15-trace-v1\",\
\"day_index\":{day_index},\
\"lane_index\":{lane_index},\
\"growth_vdmt_before_kg_m2\":{},\
\"growth_tlive_before_kg_m2\":{},\
\"growth_projected_tlive_before_kg_m2\":{},\
\"growth_hia_before\":{},\
\"growth_cancov_before\":{},\
\"growth_lai_before\":{},\
\"publication_vdmt_kg_m2\":{},\
\"publication_hia\":{},\
\"publication_cancov\":{},\
\"publication_lai\":{},\
\"post_winter_rain_m\":{},\
\"interception_m\":{},\
\"liquid_after_interception_m\":{},\
\"rainfall_scale\":{}}}",
        direct_production_trace_number(growth_state_before.live_biomass_kg_m2),
        direct_production_trace_number(growth_state_before.interception_live_biomass_kg_m2),
        direct_production_trace_number(projected_interception_live_biomass_kg_m2),
        direct_production_trace_number(growth_state_before.harvest_index),
        direct_production_trace_number(growth_state_before.canopy_cover_fraction),
        direct_production_trace_number(growth_state_before.leaf_area_index),
        direct_production_trace_number(growth_state_for_publication.live_biomass_kg_m2),
        direct_production_trace_number(growth_state_for_publication.harvest_index),
        direct_production_trace_number(growth_state_for_publication.canopy_cover_fraction),
        direct_production_trace_number(growth_state_for_publication.leaf_area_index),
        direct_production_trace_number(post_winter_rain_m),
        direct_production_trace_number(interception_state.interception_m),
        direct_production_trace_number(interception_state.liquid_after_interception_m),
        direct_production_trace_number(interception_state.rainfall_scale),
    );
    let line = format!("{line}\n");
    std::io::Write::write_all(&mut file, line.as_bytes()).map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_wb15_trace",
        detail: format!(
            "{SIMOUT_GUARD_ID} failed writing direct production WB15 trace {}: {error}",
            std::path::PathBuf::from(&path).display()
        ),
    })
}

fn direct_production_trace_output_path(name: &str) -> Option<std::ffi::OsString> {
    std::env::var_os(name).filter(|path| !path.is_empty())
}

fn direct_production_trace_filters_allow(
    day_index: usize,
    lane_index: usize,
    day_filter_name: &str,
    lane_filter_name: &str,
) -> bool {
    direct_production_trace_index_filter_allows(day_filter_name, day_index)
        && direct_production_trace_index_filter_allows(lane_filter_name, lane_index)
}

fn direct_production_trace_index_filter_allows(name: &str, observed: usize) -> bool {
    match direct_production_trace_env_usize(name) {
        Some(filter) => filter == observed,
        None => true,
    }
}

fn direct_production_trace_env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.trim().parse::<usize>().ok()
}

fn parse_snowdensity1015_default_snow_density_model(
    value: Option<&str>,
) -> Result<openwepp_hillslope_orchestrator::SnowDensityModel, HillslopeCliError> {
    match value.map_or("", str::trim) {
        "" | "physics_bulk_density_compaction_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkDensityCompactionV1)
        }
        "legacy_wepp" => Ok(openwepp_hillslope_orchestrator::SnowDensityModel::LegacyWepp),
        "physics_bulk_shallow_guard_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkShallowGuardV1,
        ),
        "physics_bulk_climate_class_density_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkClimateClassDensityV1,
        ),
        "physics_bulk_multilayer_density_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkMultilayerDensityV1,
        ),
        observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_density_model",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOWDENSITY09_DENSITY_MODEL_ENV} must be legacy_wepp, physics_bulk_density_compaction_v1, physics_bulk_shallow_guard_v1, physics_bulk_climate_class_density_v1, or physics_bulk_multilayer_density_v1, observed {observed}"
            ),
        }),
    }
}

fn parse_snowdensity1037_diagnostic_snow_melt_model(
    value: Option<&str>,
) -> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match value.map_or("", str::trim) {
        "" | "legacy_coe" => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe)
        }
        "coe_winter_thaw_state_loss_v1" => {
            Ok(openwepp_hillslope_orchestrator::SnowMeltModel::CoeWinterThawStateLossV1)
        }
        observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOWDENSITY1037_MELT_MODEL_ENV} must be legacy_coe or coe_winter_thaw_state_loss_v1, observed {observed}"
            ),
        }),
    }
}

fn parse_snowdensity1015_default_snow_melt_model(
    value: Option<&str>,
) -> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match value.map_or("", str::trim) {
        "" | "coe_liquid_holding_capacity_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeLiquidHoldingCapacityV1,
        ),
        "coe_open_sublimation_stage_a_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeOpenSublimationStageAV1,
        ),
        "coe_open_sublimation_stage_b_v1" => Ok(
            openwepp_hillslope_orchestrator::SnowMeltModel::CoeOpenSublimationStageBV1,
        ),
        "legacy_coe" => Ok(openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe),
        observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!(
                "{SIMOUT_GUARD_ID} {SNOWDENSITY1038_MELT_MODEL_ENV} must be legacy_coe, coe_liquid_holding_capacity_v1, coe_open_sublimation_stage_a_v1, or coe_open_sublimation_stage_b_v1, observed {observed}"
            ),
        }),
    }
}

fn snowdensity1015_default_snow_density_model(
) -> Result<openwepp_hillslope_orchestrator::SnowDensityModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY09_DENSITY_MODEL_ENV) {
        Ok(value) => parse_snowdensity1015_default_snow_density_model(Some(&value)),
        Err(std::env::VarError::NotPresent) => {
            parse_snowdensity1015_default_snow_density_model(None)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_density_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY09_DENSITY_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn direct_production_sturm_climate_class_for_density_candidate(
    climate_request: &HillslopeClimateRuntimeRequest,
    climate_span: &ClimateRunSpanSummary,
    lane_authority: &[DirectProductionLaneDayInputAuthority],
) -> Result<Option<openwepp_hillslope_orchestrator::SnowClimateClass>, HillslopeCliError> {
    if !lane_authority.iter().any(|authority| {
        authority.snow_frost.snow_density_model
            == openwepp_hillslope_orchestrator::SnowDensityModel::PhysicsBulkClimateClassDensityV1
    }) {
        return Ok(None);
    }
    let normals = direct_production_sturm1995_climate_normals(climate_request, climate_span)?;
    openwepp_hillslope_orchestrator::sturm1995_climate_class_from_normals(normals)
        .map(Some)
        .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_sturm1995_climate_class",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed assigning Sturm 1995 climate class from run forcing normals: {source}"
            ),
        })
}

fn paradigm2_stage3_liquid_routing_model(
) -> Result<openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel, HillslopeCliError> {
    match std::env::var(PARADIGM2_STAGE3_LIQUID_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "disabled" => {
                Ok(openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled)
            }
            "layered_thermal_liquid_v1" => Ok(
                openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
            ),
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_stage3_liquid_routing_model",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {PARADIGM2_STAGE3_LIQUID_MODEL_ENV} must be disabled, layered_thermal_liquid_v1, or empty default, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => {
            Ok(openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_stage3_liquid_routing_model",
            detail: format!("{SIMOUT_GUARD_ID} {PARADIGM2_STAGE3_LIQUID_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn direct_production_sturm1995_climate_normals(
    climate_request: &HillslopeClimateRuntimeRequest,
    climate_span: &ClimateRunSpanSummary,
) -> Result<openwepp_hillslope_orchestrator::Sturm1995ClimateNormals, HillslopeCliError> {
    if climate_span.days.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_sturm1995_climate_class",
            detail: format!(
                "{SIMOUT_GUARD_ID} cannot assign Sturm 1995 climate class for empty climate span"
            ),
        });
    }
    let mut months = [DirectProductionSturm1995MonthlyAccumulator::default(); 12];
    for (day_index, day) in climate_span.days.iter().enumerate() {
        let forcing = climate_request.direct_day_forcing(day_index).map_err(|source| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_sturm1995_climate_class",
                detail: format!(
                    "{SIMOUT_GUARD_ID} failed reading daily forcing for Sturm 1995 climate normals: {source}"
                ),
            }
        })?;
        let month_index =
            usize::try_from(day.month - 1).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_sturm1995_climate_class",
                detail: format!(
                    "{SIMOUT_GUARD_ID} invalid climate month {} for Sturm 1995 climate normals",
                    day.month
                ),
            })?;
        let Some(month) = months.get_mut(month_index) else {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_sturm1995_climate_class",
                detail: format!(
                    "{SIMOUT_GUARD_ID} invalid climate month {} for Sturm 1995 climate normals",
                    day.month
                ),
            });
        };
        month.add(
            f64::midpoint(forcing.tmax_c, forcing.tmin_c),
            (forcing.prcp_m * 1_000.0).max(0.0),
            forcing.vwind_m_s,
        );
    }

    let mut cdm_c_month = 0.0;
    let mut spr_sum_mm_day = 0.0;
    let mut cold_month_count = 0u32;
    let mut winter_wind_sum_m_s = 0.0;
    let mut winter_wind_day_count = 0u32;
    for month in months.iter().filter(|month| month.day_count > 0) {
        let mean_temperature_c = month.mean_temperature_c();
        if mean_temperature_c < openwepp_hillslope_orchestrator::STURM1995_CDM_CRITICAL_TEMPERATURE_C
        {
            cdm_c_month += openwepp_hillslope_orchestrator::STURM1995_CDM_CRITICAL_TEMPERATURE_C
                - mean_temperature_c;
            spr_sum_mm_day += month.mean_precipitation_mm_day();
            cold_month_count += 1;
            winter_wind_sum_m_s += month.wind_m_s_sum;
            winter_wind_day_count += month.day_count;
        }
    }
    Ok(openwepp_hillslope_orchestrator::Sturm1995ClimateNormals {
        cooling_degree_month_c: cdm_c_month,
        snowfall_precipitation_rate_mm_day: if cold_month_count > 0 {
            spr_sum_mm_day / f64::from(cold_month_count)
        } else {
            0.0
        },
        winter_wind_m_s: if winter_wind_day_count > 0 {
            winter_wind_sum_m_s / f64::from(winter_wind_day_count)
        } else {
            0.0
        },
    })
}

#[derive(Clone, Copy, Default)]
struct DirectProductionSturm1995MonthlyAccumulator {
    temperature_c_sum: f64,
    precipitation_mm_sum: f64,
    wind_m_s_sum: f64,
    day_count: u32,
}

impl DirectProductionSturm1995MonthlyAccumulator {
    fn add(&mut self, temperature_c: f64, precipitation_mm: f64, wind_m_s: f64) {
        self.temperature_c_sum += temperature_c;
        self.precipitation_mm_sum += precipitation_mm;
        self.wind_m_s_sum += wind_m_s;
        self.day_count += 1;
    }

    fn mean_temperature_c(self) -> f64 {
        self.temperature_c_sum / f64::from(self.day_count)
    }

    fn mean_precipitation_mm_day(self) -> f64 {
        self.precipitation_mm_sum / f64::from(self.day_count)
    }
}

fn snowdensity1035_diagnostic_snow_phase_model(
) -> Result<openwepp_hillslope_orchestrator::SnowPhasePartitionModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY1035_PHASE_MODEL_ENV) {
        Ok(value) => match value.trim() {
            "" | "harder_pomeroy_hourly" => Ok(
                openwepp_hillslope_orchestrator::SnowPhasePartitionModel::HarderPomeroyHourly,
            ),
            "legacy_rst" => {
                Ok(openwepp_hillslope_orchestrator::SnowPhasePartitionModel::LegacyRst)
            }
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_phase_model",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {SNOWDENSITY1035_PHASE_MODEL_ENV} must be legacy_rst, harder_pomeroy_hourly, or empty default, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => {
            Ok(openwepp_hillslope_orchestrator::SnowPhasePartitionModel::HarderPomeroyHourly)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_phase_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY1035_PHASE_MODEL_ENV} must be UTF-8"),
        }),
    }
}

#[allow(dead_code)]
fn snowdensity1037_diagnostic_snow_melt_model(
) -> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY1037_MELT_MODEL_ENV) {
        Ok(value) => parse_snowdensity1037_diagnostic_snow_melt_model(Some(&value)),
        Err(std::env::VarError::NotPresent) => {
            parse_snowdensity1037_diagnostic_snow_melt_model(None)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY1037_MELT_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn snowdensity1015_default_snow_melt_model(
) -> Result<openwepp_hillslope_orchestrator::SnowMeltModel, HillslopeCliError> {
    match std::env::var(SNOWDENSITY1038_MELT_MODEL_ENV) {
        Ok(value) => parse_snowdensity1015_default_snow_melt_model(Some(&value)),
        Err(std::env::VarError::NotPresent) => {
            parse_snowdensity1015_default_snow_melt_model(None)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_melt_model",
            detail: format!("{SIMOUT_GUARD_ID} {SNOWDENSITY1038_MELT_MODEL_ENV} must be UTF-8"),
        }),
    }
}

fn direct_production_erosion_active(
    authority: &DirectProductionLaneDayInputAuthority,
    day_input: &DirectPublicationDayInput,
) -> Result<bool, HillslopeCliError> {
    if !authority.erosion.wave2_enabled {
        return Ok(false);
    }
    let rainfall_m = direct_publication_hyetograph_rainfall_m(
        day_input
            .peak_runoff_inputs
            .as_ref()
            .map_or(&[][..], |inputs| inputs.hyetograph.as_slice()),
    )?;
    Ok(rainfall_m >= DIRECT_PUBLICATION_EROSION_MIN_POST_INTERCEPTION_RAINFALL_M)
}

fn apply_direct_production_erosion_inputs(
    day_input: &mut DirectPublicationDayInput,
    authority: &DirectProductionLaneDayInputAuthority,
    erosion_active: bool,
) {
    day_input.erosion_producer_required = erosion_active;
    if erosion_active {
        day_input.erosion_inputs = Some(authority.erosion.erosion_inputs.clone());
    }
}

fn apply_direct_production_frost_context(
    day_input: &mut DirectPublicationDayInput,
    frost_context: Option<DirectProductionFrostDayContext>,
) {
    if let Some(frost_context) = frost_context {
        day_input.winter_frost_compute_inputs = Some(frost_context.compute_inputs);
        day_input.frost_storage_liquid_delta_m = frost_context.storage_liquid_delta_m;
        day_input.frost_layer_carry_projection = frost_context.layer_carry_projection;
    }
}

fn direct_production_trace_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.17}")
    } else {
        "null".to_string()
    }
}

fn direct_production_growth_state_for_publication(
    annual_growth_inputs: &DirectGrowthInputs,
    perennial_growth_inputs: &DirectGrowthInputs,
    growth_state_before: DirectGrowthStateSurface,
) -> Result<DirectGrowthStateSurface, HillslopeCliError> {
    if perennial_growth_inputs.active_context.is_active() {
        return (*perennial_growth_inputs)
            .compute_perennial()
            .map(|growth| growth.state_after)
            .map_err(|source| direct_publication_runtime_error(&source));
    }
    if annual_growth_inputs.active_context.is_active() {
        return (*annual_growth_inputs)
            .compute_annual_or_fallow()
            .map(|growth| growth.state_after)
            .map_err(|source| direct_publication_runtime_error(&source));
    }
    Ok(growth_state_before)
}

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
        let effective_conductivity_m_s = frost_infcap_m_s
            .filter(|value| *value > 0.0)
            .or(self.effective_conductivity_m_s)
            .filter(|value| *value > 0.0)
            .or_else(|| layers.first().map(|layer| layer.conductivity_m_s))
            .ok_or_else(|| {
                direct_production_executor_blocked(
                    "direct production WB14 infiltration requires layer conductivity",
                )
            })?;
        let matric_potential_m = self.matric_potential_m.unwrap_or_else(|| {
            let first_layer = &layers[0];
            first_layer.depth_m * (first_layer.field_capacity_theta - first_layer.residual_theta).max(0.0)
        });
        let storage_capacity_m = direct_publication_wb14_top_storage_capacity(layers)?;
        Ok(DirectInfiltrationDepressionInputs {
            cumulative_infiltration_handoff_m: 0.0,
            depression_storage_delta_handoff_m: 0.0,
            producer_inputs: Some(DirectWb14InfiltrationProducerInputs {
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
