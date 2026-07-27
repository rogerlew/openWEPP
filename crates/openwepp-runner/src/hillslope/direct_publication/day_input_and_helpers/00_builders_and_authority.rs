const FOREST_LITTER_FALLBACK_DECAY_RATE_PER_DAY: f64 = 0.5 / 365.25;
const FOREST_LITTER_DROP_WINDOW_DAYS: usize = 45;
// WEPP default rill spacing (m) for managements that carry no rill
// parameterization; used only behind the disabled Wave-1 seed and flagged
// for enable-time adjudication.
const WEPP_DEFAULT_RILL_SPACING_M: f64 = 1.0;

struct DirectProductionDayInputBuilder<'a> {
    climate_request: &'a HillslopeClimateRuntimeRequest,
    climate_span: &'a ClimateRunSpanSummary,
    lane_authority: Vec<DirectProductionLaneDayInputAuthority>,
    residue_cover_state: std::cell::RefCell<Vec<DirectProductionResidueCoverState>>,
    forest_canopy_state:
        std::cell::RefCell<Vec<Option<openwepp_plant_phenology::ForestCanopyState>>>,
    canopy_research_pending: std::cell::RefCell<Vec<Option<NativeCanopyBuilderTrace>>>,
    winter_hourly_geometry: DirectProductionWinterHourlyGeometry,
    sturm_climate_class: Option<openwepp_hillslope_orchestrator::SnowClimateClass>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DirectLanedActiveDefaultEligibility {
    Complete,
    Absent,
    Mixed { present: usize, absent: usize },
}

impl DirectProductionDayInputBuilder<'_> {
    /// SC-OFEROUTE-001 rev 46: default active routing is authorized only
    /// when every scheduled lane carries native `routing_coefficients`;
    /// no-lane authority is a protected legacy/off fallback, and mixed
    /// authority must fail closed before streaming.
    pub(crate) fn laned_active_default_eligibility(&self) -> DirectLanedActiveDefaultEligibility {
        let present = self
            .lane_authority
            .iter()
            .filter(|lane| lane.ofe_routing.is_some())
            .count();
        match present {
            0 => DirectLanedActiveDefaultEligibility::Absent,
            count if count == self.lane_authority.len() => {
                DirectLanedActiveDefaultEligibility::Complete
            }
            count => DirectLanedActiveDefaultEligibility::Mixed {
                present: count,
                absent: self.lane_authority.len() - count,
            },
        }
    }

    /// Per-lane static geometry for the Lane D seam shadow (from the
    /// Wave-1 operand seeds): slope length, hillslope field width, and
    /// the mean profile gradient (integral of the normalized `a·x + b`
    /// segment fit), floored at 0.001 m/m so degenerate flat fits keep
    /// the bare-cell mesh valid.
    pub(crate) fn laned_shadow_geometry(
        &self,
    ) -> Result<Vec<crate::hillslope::laned_shadow::LanedShadowLaneGeometry>, HillslopeCliError>
    {
        self.laned_geometry_with_selector("OPENWEPP_LANED_SHADOW")
    }

    /// D15A (rev 27): the ACTIVE owner's per-lane configuration — the SAME
    /// rev-20/21/36 authority extraction as the shadow (fail-closed on
    /// missing native `routing_coefficients`). Dynamic `LAI`/`canhgt` comes
    /// from the live post-growth day frame at consumption time.
    pub(crate) fn laned_active_config(
        &self,
    ) -> Result<openwepp_hillslope_orchestrator::DirectLanedActiveConfig, HillslopeCliError> {
        let lanes = self
            .laned_geometry_with_selector("OPENWEPP_LANED_ACTIVE")?
            .into_iter()
            .zip(self.lane_authority.iter())
            .map(
                |(geometry, lane)| openwepp_hillslope_orchestrator::DirectLanedActiveLaneConfig {
                    slplen_m: geometry.slplen_m,
                    width_m: geometry.width_m,
                    mean_gradient: geometry.mean_gradient,
                    skin_friction_coefficient_ko: geometry.routing.skin_friction_coefficient_ko,
                    form_drag_coefficient: geometry.routing.form_drag_coefficient,
                    roughness_element_height_m: geometry.routing.roughness_element_height_m,
                    roughness_concentration: geometry.routing.roughness_concentration,
                    vegetation_drag_coefficient: geometry.routing.vegetation_drag_coefficient,
                    canopy_height_m: lane.evapotranspiration.canopy_height_m,
                },
            )
            .collect();
        let mesh_policy = crate::hillslope::laned_active::mesh_policy_from_env()?;
        let max_dt_s = crate::hillslope::laned_active::max_dt_s_from_env()?;
        let trace_enabled = crate::hillslope::laned_active::trace_enabled();
        let trace_detail_filter = crate::hillslope::laned_active::trace_detail_filter_from_env()?;
        let step_trace_enabled = crate::hillslope::laned_active::step_trace_enabled();
        Ok(openwepp_hillslope_orchestrator::DirectLanedActiveConfig {
            lanes,
            mesh_policy,
            max_dt_s,
            trace_enabled,
            trace_detail_filter,
            step_trace_enabled,
        })
    }

    fn laned_geometry_with_selector(
        &self,
        selector: &'static str,
    ) -> Result<Vec<crate::hillslope::laned_shadow::LanedShadowLaneGeometry>, HillslopeCliError>
    {
        self.lane_authority
            .iter()
            .enumerate()
            .map(|(lane_index, lane)| {
                let routing = lane.ofe_routing.ok_or_else(|| {
                    HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "laned_shadow_routing_coefficients",
                        detail: format!(
                            "{SIMOUT_GUARD_ID} {selector} requires a complete, schedule-consistent routing coefficient extension for every MOFE landuse; lane {} is missing or has inconsistent route_* authority symbols",
                            lane_index + 1
                        ),
                    }
                })?;
                let seed = &lane.erosion.erosion_inputs.wave1_operand_seed;
                let mean_gradient = seed
                    .segments
                    .iter()
                    .map(|segment| {
                        segment.a / 2.0
                            * (segment.xl * segment.xl - segment.xu * segment.xu)
                            + segment.b * (segment.xl - segment.xu)
                    })
                    .sum::<f64>()
                    .max(0.001);
                Ok(crate::hillslope::laned_shadow::LanedShadowLaneGeometry {
                    slplen_m: seed.slplen_m,
                    width_m: seed.field_width_m,
                    mean_gradient,
                    routing: routing.into_laned_shadow(),
                })
            })
            .collect()
    }
}

#[derive(Clone)]
struct DirectProductionSeedAuthority {
    lanes: Vec<DirectProductionLaneSeedAuthority>,
    winter_hourly_geometry: DirectProductionWinterHourlyGeometry,
    multi_ofe_wave1_chained: bool,
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
    ofe_routing: Option<DirectProductionOfeRoutingCoefficientAuthority>,
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
    ofe_routing: Option<DirectProductionOfeRoutingCoefficientAuthority>,
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
    ksatadj_policy: Option<DirectProductionKsatadjPolicy>,
    matric_potential_m: Option<f64>,
    depression_storage_capacity_m: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectProductionEvapotranspirationAuthority {
    leaf_area_index: f64,
    canopy_height_m: Option<f64>,
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
    bbb: f64,
    hmax: f64,
    beinp: f64,
    extnct: f64,
    hi: f64,
    xmxlai: f64,
    rsr: f64,
    rtmmax: f64,
    rdmax: f64,
    oratea: f64,
    orater: f64,
    forest_phenology: Option<DirectProductionForestPhenologyAuthority>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DirectProductionForestPhenologyAuthority {
    summer_foliar_biomass_kg_m2: f64,
    evergreen_fraction: f64,
    structural_canopy_cover_fraction: f64,
    structural_biomass_kg_m2: f64,
    minimum_temperature_inactive_c: f64,
    minimum_temperature_unconstrained_c: f64,
    vapor_pressure_deficit_unconstrained_pa: f64,
    vapor_pressure_deficit_inactive_pa: f64,
    photoperiod_inactive_hours: f64,
    photoperiod_unconstrained_hours: f64,
}

#[derive(Clone, Copy, Debug)]
struct NativeCanopyBuilderTrace {
    day_index: usize,
    lane_index: usize,
    year: i32,
    month: i32,
    day_of_month: i32,
    daily: openwepp_plant_phenology::ForestCanopyDailyResult,
    #[cfg(test)]
    canopy: openwepp_plant_phenology::ForestCanopyRealization,
    snow_canopy_cover_fraction: f64,
    interception_inputs: DirectCanopyInterceptionInputs,
    #[cfg(test)]
    interception_state: openwepp_hillslope_orchestrator::DirectCanopyInterceptionState,
    #[cfg(test)]
    projected_surface_residue_kg_m2: f64,
    #[cfg(test)]
    projected_residue_depth_m: f64,
    #[cfg(test)]
    frost_residue_depth_m: Option<f64>,
    #[cfg(test)]
    frost_canopy_height_m: Option<f64>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
struct NativeCanopyConsumerTrace {
    builder: NativeCanopyBuilderTrace,
    growth_state_after: DirectGrowthStateSurface,
    et_leaf_area_index: f64,
    et_canopy_cover_fraction: f64,
    et_canopy_height_m: f64,
    erosion_canopy_height_m: Option<f64>,
    interception_m: f64,
    decomposition_litter_kg_m2: f64,
    decomposition_surface_residue_kg_m2: f64,
    decomposition_residue_depth_m: f64,
    frost_residue_depth_m_consumed: Option<f64>,
    frost_canopy_height_m_consumed: Option<f64>,
    erosion_canopy_cover_fraction: Option<f64>,
    laned_active_canopy_height_m_consumed: Option<f64>,
    laned_shadow_canopy_height_m_consumed: f64,
}

#[cfg(test)]
fn native_canopy_builder_traces() -> &'static std::sync::Mutex<Vec<NativeCanopyBuilderTrace>> {
    static TRACES: std::sync::OnceLock<std::sync::Mutex<Vec<NativeCanopyBuilderTrace>>> =
        std::sync::OnceLock::new();
    TRACES.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

#[cfg(test)]
fn native_canopy_consumer_traces() -> &'static std::sync::Mutex<Vec<NativeCanopyConsumerTrace>> {
    static TRACES: std::sync::OnceLock<std::sync::Mutex<Vec<NativeCanopyConsumerTrace>>> =
        std::sync::OnceLock::new();
    TRACES.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

#[cfg(test)]
fn reset_native_canopy_runtime_traces() {
    native_canopy_builder_traces()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clear();
    native_canopy_consumer_traces()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clear();
}

#[cfg(test)]
fn record_native_canopy_builder_trace(trace: &NativeCanopyBuilderTrace) {
    native_canopy_builder_traces()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(*trace);
}

#[cfg(test)]
fn record_native_canopy_consumer_trace(
    day_frame: &openwepp_hillslope_orchestrator::DirectDayFrame,
) {
    let builder = {
        let traces = native_canopy_builder_traces()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        traces
            .iter()
            .find(|trace| {
                trace.day_index == day_frame.day_index && trace.lane_index == day_frame.lane_index
            })
            .copied()
    };
    let Some(builder) = builder else {
        return;
    };
    let growth_state_after = if day_frame.perennial_growth_inputs.active_context.is_active() {
        day_frame.perennial_growth.state_after
    } else {
        day_frame.annual_growth.state_after
    };
    let laned_shadow_canopy_height_m_consumed = build_laned_shadow_lane_day_operands(
        day_frame.lane_index,
        day_frame.day_index,
        day_frame.wb14_hourly_rainfall_m,
        *day_frame
            .snow_coupling_downstream_operands
            .hourly_routed_melt_m,
        day_frame.evapotranspiration_compute_inputs.leaf_area_index,
        Some(day_frame.evapotranspiration_compute_inputs.canopy_height_m),
    )
    .expect("native canopy trace must pass the real Lane D shadow operand seam")
    .canopy_height_m;
    native_canopy_consumer_traces()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(NativeCanopyConsumerTrace {
            builder,
            growth_state_after,
            et_leaf_area_index: day_frame.evapotranspiration_compute_inputs.leaf_area_index,
            et_canopy_cover_fraction: day_frame
                .evapotranspiration_compute_inputs
                .canopy_cover_fraction,
            et_canopy_height_m: day_frame.evapotranspiration_compute_inputs.canopy_height_m,
            erosion_canopy_height_m: day_frame.erosion_canopy_height_m_consumed,
            interception_m: day_frame.interception_m,
            decomposition_litter_kg_m2: day_frame.decomposition_inputs.surface_litter_input_kg_m2,
            decomposition_surface_residue_kg_m2: day_frame.decomposition.surface_residue_kg_m2,
            decomposition_residue_depth_m: day_frame.decomposition.residue_depth_m,
            frost_residue_depth_m_consumed: day_frame.frost_residue_depth_m_consumed,
            frost_canopy_height_m_consumed: day_frame.frost_canopy_height_m_consumed,
            erosion_canopy_cover_fraction: day_frame.erosion_canopy_cover_fraction_consumed,
            laned_active_canopy_height_m_consumed: day_frame
                .laned_active_routing
                .as_ref()
                .and_then(|routing| routing.canopy_height_m_consumed),
            laned_shadow_canopy_height_m_consumed,
        });
}

#[cfg(test)]
fn take_native_canopy_consumer_traces() -> Vec<NativeCanopyConsumerTrace> {
    std::mem::take(
        &mut *native_canopy_consumer_traces()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
    )
}

#[derive(Clone, Copy)]
struct DirectProductionResidueCoverAuthority {
    initial_surface_residue_kg_m2: f64,
    initial_interrill_ground_kg_m2: f64,
    initial_rill_ground_kg_m2: f64,
    residue_cover_factor: f64,
    rescov_interrill_weight: f64,
    initial_root_residue_kg_m2: f64,
    residue_type_selector: f64,
    residue_depth_conversion_m_per_kg_m2: f64,
}

#[derive(Clone, Copy)]
struct DirectProductionResidueCoverState {
    surface_residue_kg_m2: f64,
    root_residue_kg_m2: f64,
    /// GAP-SED-009 closure: the covcal ground pools (day-0 back-derived
    /// from the declared IC covers per `init1.for:295-297`; carried
    /// through the decomposition state thereafter).
    interrill_ground_residue_kg_m2: f64,
    rill_ground_residue_kg_m2: f64,
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
    erosion_inputs: DirectErosionInputs,
}

#[derive(Clone, Copy)]
struct DirectProductionWinterHourlyGeometry {
    avg_slope: f64,
    azimuth: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DirectProductionOfeRoutingCoefficientAuthority {
    skin_friction_coefficient_ko: f64,
    form_drag_coefficient: f64,
    roughness_element_height_m: f64,
    roughness_concentration: f64,
    vegetation_drag_coefficient: f64,
}

impl DirectProductionOfeRoutingCoefficientAuthority {
    fn into_laned_shadow(self) -> crate::hillslope::laned_shadow::LanedShadowRoutingCoefficients {
        crate::hillslope::laned_shadow::LanedShadowRoutingCoefficients {
            skin_friction_coefficient_ko: self.skin_friction_coefficient_ko,
            form_drag_coefficient: self.form_drag_coefficient,
            roughness_element_height_m: self.roughness_element_height_m,
            roughness_concentration: self.roughness_concentration,
            vegetation_drag_coefficient: self.vegetation_drag_coefficient,
        }
    }
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
    stage3_liquid_routing_model: openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel,
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
    frost_outcome: DirectWinterFrostPartitionOutcome,
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
    canopy_height_m_override: Option<f64>,
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
                day_input: direct_production_day_input_authority_from_typed_seed(typed_lane_seed),
            })
            .collect::<Vec<_>>();
        let winter_hourly_geometry =
            DirectProductionWinterHourlyGeometry::from_typed_inputs(inputs, lane_count)?;
        let multi_ofe_wave1_chained =
            direct_production_typed_multi_ofe_wave1_chained(inputs, lane_count)?;

        Ok(Self {
            lanes,
            winter_hourly_geometry,
            multi_ofe_wave1_chained,
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

    fn outlet_snow_frost(&self) -> Result<&DirectProductionSnowFrostAuthority, HillslopeCliError> {
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

    fn snowbench_export_seed(
        &self,
    ) -> Result<DirectProductionSnowbenchExportSeed, HillslopeCliError> {
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
                snow_phase_model:
                    openwepp_hillslope_orchestrator::SnowPhasePartitionModel::LegacyRst,
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
        ofe_routing: seed.ofe_routing,
    }
}

/// Forest `lanuse` authority reconciliation (ADR-0034, `LANUSE-AUTH-6`): a
/// native forest management must be backed by a matching disturbed soil policy.
/// Fails closed on mismatch; cropland-only managements are a no-op.
fn reconcile_forest_lanuse_or_fail(
    management: &ManagementParseOutput,
    soil: &SoilProfile,
) -> Result<(), HillslopeCliError> {
    openwepp_hillslope_orchestrator::reconcile_forest_lanuse_authority(management, soil).map_err(
        |error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "forest_lanuse_reconciliation",
            detail: error.to_string(),
        },
    )
}

// Large orchestration seed-builder: assembles every typed lane authority
// (soil / slope / management / snow / frost / peak-runoff / erosion / ...)
// from the parsed inputs. The line count is inherent to the fan-out.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
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
    reconcile_forest_lanuse_or_fail(management, soil)?;
    let management_projection = build_hillslope_pl_runtime_surfaces_from_management(management)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: error.to_string(),
        })?;
    let ofe_routing =
        direct_production_optional_lane_routing_coefficient_authority(&management_projection)?;
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
        soil,
        &soil_projection,
        &slope_projection,
        &management_projection,
        &peak_runoff,
        direct_production_management_has_active_tillage(management),
        direct_production_schedule_lanuse_is_cropland(management)?,
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
        infiltration: direct_production_typed_infiltration_authority(
            &soil_projection,
            &frost_projection,
        ),
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
        ofe_routing,
    })
}

fn direct_production_optional_lane_routing_coefficient_authority(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
) -> Result<Option<DirectProductionOfeRoutingCoefficientAuthority>, HillslopeCliError> {
    let Some(slot_count_value) =
        direct_production_pl_projection_optional_scalar(projection, "pl_schedule_slot_count")
    else {
        return Ok(None);
    };
    let slot_count =
        direct_growth_integral_usize("pl_schedule_slot_count", slot_count_value, 1, usize::MAX)?;
    let mut lane_authority = None;
    let mut present_slots = 0_usize;
    let mut absent_slots = 0_usize;
    for slot_index in 1..=slot_count {
        let crop_slots = direct_growth_projection_required_integral_usize(
            projection,
            &direct_growth_schedule_slot_symbol(slot_index, "crop_slots"),
            1,
            usize::MAX,
        )?;
        for crop_slot_index in 1..=crop_slots {
            match direct_production_optional_slot_crop_routing_coefficient_authority(
                projection,
                slot_index,
                crop_slot_index,
            )? {
                Some(slot_authority) => {
                    present_slots += 1;
                    if let Some(existing) = lane_authority {
                        if existing != slot_authority {
                            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                                surface: "direct_production_typed_seed",
                                detail: format!(
                                    "{SIMOUT_GUARD_ID} inconsistent routing coefficient extension across schedule crop slots: slot {slot_index} crop {crop_slot_index} differs from previous route_* authority"
                                ),
                            });
                        }
                    } else {
                        lane_authority = Some(slot_authority);
                    }
                }
                None => {
                    absent_slots += 1;
                }
            }
        }
    }
    if present_slots > 0 && absent_slots > 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} incomplete schedule routing coefficient extension: {present_slots} schedule crop slot(s) carry route_* authority and {absent_slots} slot(s) do not"
            ),
        });
    }
    Ok(lane_authority)
}

fn direct_production_optional_slot_crop_routing_coefficient_authority(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    slot_index: usize,
    crop_slot_index: usize,
) -> Result<Option<DirectProductionOfeRoutingCoefficientAuthority>, HillslopeCliError> {
    let skin_friction_coefficient_ko = direct_production_pl_projection_optional_scalar(
        projection,
        &direct_growth_schedule_slot_crop_symbol(
            slot_index,
            crop_slot_index,
            "route_skin_friction_coefficient_ko",
        ),
    );
    let form_drag_coefficient = direct_production_pl_projection_optional_scalar(
        projection,
        &direct_growth_schedule_slot_crop_symbol(
            slot_index,
            crop_slot_index,
            "route_form_drag_coefficient",
        ),
    );
    let roughness_element_height_m = direct_production_pl_projection_optional_scalar(
        projection,
        &direct_growth_schedule_slot_crop_symbol(
            slot_index,
            crop_slot_index,
            "route_roughness_element_height_m",
        ),
    );
    let roughness_concentration = direct_production_pl_projection_optional_scalar(
        projection,
        &direct_growth_schedule_slot_crop_symbol(
            slot_index,
            crop_slot_index,
            "route_roughness_concentration",
        ),
    );
    let vegetation_drag_coefficient = direct_production_pl_projection_optional_scalar(
        projection,
        &direct_growth_schedule_slot_crop_symbol(
            slot_index,
            crop_slot_index,
            "route_vegetation_drag_coefficient",
        ),
    );

    if [
        skin_friction_coefficient_ko,
        form_drag_coefficient,
        roughness_element_height_m,
        roughness_concentration,
        vegetation_drag_coefficient,
    ]
    .iter()
    .all(Option::is_none)
    {
        return Ok(None);
    }

    let required = |root: &'static str, value: Option<f64>| {
        value.ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_typed_seed",
            detail: format!(
                "{SIMOUT_GUARD_ID} partial routing coefficient extension for slot {slot_index} crop {crop_slot_index}: missing {root}"
            ),
        })
    };

    let authority = DirectProductionOfeRoutingCoefficientAuthority {
        skin_friction_coefficient_ko: required(
            "route_skin_friction_coefficient_ko",
            skin_friction_coefficient_ko,
        )?,
        form_drag_coefficient: required("route_form_drag_coefficient", form_drag_coefficient)?,
        roughness_element_height_m: required(
            "route_roughness_element_height_m",
            roughness_element_height_m,
        )?,
        roughness_concentration: required(
            "route_roughness_concentration",
            roughness_concentration,
        )?,
        vegetation_drag_coefficient: required(
            "route_vegetation_drag_coefficient",
            vegetation_drag_coefficient,
        )?,
    };
    validate_direct_production_routing_coefficient_authority(
        &format!("slot {slot_index} crop {crop_slot_index}"),
        authority,
    )?;
    Ok(Some(authority))
}

fn validate_direct_production_routing_coefficient_authority(
    context: &str,
    authority: DirectProductionOfeRoutingCoefficientAuthority,
) -> Result<(), HillslopeCliError> {
    let fields = [
        (
            "route_skin_friction_coefficient_ko",
            authority.skin_friction_coefficient_ko,
            "> 0.0",
        ),
        (
            "route_form_drag_coefficient",
            authority.form_drag_coefficient,
            ">= 0.0",
        ),
        (
            "route_roughness_element_height_m",
            authority.roughness_element_height_m,
            ">= 0.0",
        ),
        (
            "route_roughness_concentration",
            authority.roughness_concentration,
            "0.0..=1.0",
        ),
        (
            "route_vegetation_drag_coefficient",
            authority.vegetation_drag_coefficient,
            ">= 0.0",
        ),
    ];
    for (root, value, allowed) in fields {
        let valid = match root {
            "route_skin_friction_coefficient_ko" => value.is_finite() && value > 0.0,
            "route_roughness_concentration" => value.is_finite() && (0.0..=1.0).contains(&value),
            _ => value.is_finite() && value >= 0.0,
        };
        if !valid {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_typed_seed",
                detail: format!(
                    "{SIMOUT_GUARD_ID} routing coefficient extension {context} {root} must be finite and {allowed}, observed {value}"
                ),
            });
        }
    }
    Ok(())
}

fn direct_production_typed_peak_runoff_authority(
    slope: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSlopeRuntimeProjection,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
) -> Result<DirectProductionPeakRunoffAuthority, HillslopeCliError> {
    let first_ofe = slope.ofes.first().ok_or_else(|| {
        direct_production_executor_blocked("typed peak-runoff seed requires at least one slope OFE")
    })?;
    let efflen_and_m = project_typed_wb11_efflen_and_m(None, first_ofe.slplen_m, None)?;
    let ealpha =
        direct_production_typed_wb16_ealpha(slope, management_projection, efflen_and_m.exponent_m)?
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
    project_typed_wb16_ealpha_producer(&TypedWb16EalphaProducerInput { exponent_m, ofes }).map(Some)
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
        plant_growth_state: direct_growth_state_surface_from_pl_projection(management_projection)?,
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
    let initial_surface_residue_kg_m2 =
        direct_production_pl_projection_optional_nonnegative_scalar(
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
    // GAP-SED-009 closure: seed the covcal ground pools from the
    // DECLARED IC covers (`init1.for:295-297` inverse) with the residue
    // plant's cover factor. Zero declared cover or zero `cf` seeds zero
    // pools — the pre-fix behavior, so non-forest managements are
    // unchanged unless they declare cover.
    let declared_inrcov = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "inrcov",
    )?
    .unwrap_or(0.0);
    let declared_rilcov = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "rilcov",
    )?
    .unwrap_or(0.0);
    let residue_cover_factor = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "residue_cover_factor_cf",
    )?
    .unwrap_or(0.0);
    let ground_pool_from_declared_cover = |cover: f64| -> f64 {
        let cover = cover.min(0.999);
        if cover <= 0.0 || residue_cover_factor <= 0.0 {
            0.0
        } else {
            (1.0 - cover).ln() / -residue_cover_factor
        }
    };
    let initial_interrill_ground_kg_m2 = ground_pool_from_declared_cover(declared_inrcov);
    let initial_rill_ground_kg_m2 = ground_pool_from_declared_cover(declared_rilcov);
    // `covcal.for:176` composite weight `(rspace − width)/rspace`
    // (the `init1.for:130-133` `wght1` rule; `rspace <= 0` defaults to
    // 1 m, `width` capped at `rspace`).
    let rescov_rspace = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "rspace",
    )?
    .unwrap_or(0.0);
    let rescov_width = direct_production_pl_projection_optional_nonnegative_scalar(
        management_projection,
        "width",
    )?
    .unwrap_or(0.0);
    let rescov_rspace = if rescov_rspace <= 0.0 {
        1.0
    } else {
        rescov_rspace
    };
    let rescov_width = rescov_width.min(rescov_rspace);
    let rescov_interrill_weight = (rescov_rspace - rescov_width) / rescov_rspace;

    Ok(DirectProductionResidueCoverAuthority {
        initial_surface_residue_kg_m2,
        initial_interrill_ground_kg_m2,
        initial_rill_ground_kg_m2,
        residue_cover_factor,
        rescov_interrill_weight,
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
            solthk_m: soil
                .layers
                .iter()
                .map(|layer| Some(layer.solthk_m))
                .collect(),
        })
    };
    let canopy_height_m = if let Some(pmet) = &pmet {
        Some(pmet.canhgt)
    } else {
        direct_production_pl_projection_optional_nonnegative_scalar(
            management_projection,
            "canhgt",
        )?
    };

    Ok(DirectProductionEvapotranspirationAuthority {
        leaf_area_index: direct_production_pl_projection_required_ofe_scalar(
            management_projection,
            1,
            "lai",
        )?,
        canopy_height_m,
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

/// SC-SED-001 1b-C: build the per-lane **static** Wave-1 operand seed from
/// the parsed soil texture, the slope geometry, and the management cover
/// constants. Sources the real operands (particle classes + `veleff`,
/// `scon` consolidation baselines, normalized slope segments, geometry,
/// `hmax`/`flivmx`); the per-day assembly combines them with the daily
/// frame state. `enabled` is forced `false` here — the seed stays inert
/// until the production flip (Stage 4).
///
/// `is_cropland` selects the interrill delivery branch and is RESOLVED
/// from the schedule-scoped parsed lanuse by the caller
/// (`direct_production_schedule_lanuse_is_cropland`, SC-SED-001 rev 52
/// `INV-SED-017` (f)): Cropland ⇒ the legacy `drinti` branch, Forest ⇒
/// `intdr = 1`, mixed or missing lanuse fails closed. The former 1b-C
/// hardcoded-false first cut is retired; the roughness-delivery
/// universality question remains a flagged science item in the
/// contract.
///
/// `field_width_m` (E.1 / Increment 1c-fidelity) is sourced from the
/// parsed slope-file profile width (`fwidth`), matching the legacy
/// `sedseg.for` total-mass scaling (`tdet = sum2*fwidth*filoss`) so the
/// published `total_detachment_kg`/`total_deposition_kg` and the HBP
/// payload carry true kilograms (INV-SED-010 units). The toe
/// concentration is width-independent (`sloss.for:305-317`).
/// Shared `prtcmp`-lineage particle-class derivation from the parsed soil
/// surface layer (single-OFE scope). One producer, two consumers: the
/// Wave-1 operand seed and the HBP minor-1 EVENT writer (per-class
/// diameters) — the run-level composition is a pure function of the
/// texture, so both see identical classes by construction.
fn direct_production_erosion_particle_classes(
    parsed_soil: &SoilProfile,
) -> Result<[openwepp_hillslope_orchestrator::ErosionParticleClass; 5], HillslopeCliError> {
    let blocked = |detail: String| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_erosion_particle_classes",
        detail: format!("{SIMOUT_GUARD_ID} {detail}"),
    };
    let soil_ofe = parsed_soil.ofes.first().ok_or_else(|| {
        blocked("erosion particle classes require at least one soil OFE".to_string())
    })?;
    let surface_layer = soil_ofe.layers.first().ok_or_else(|| {
        blocked("erosion particle classes require at least one soil layer".to_string())
    })?;
    let sand = surface_layer.sand_pct / 100.0;
    let clay = surface_layer.clay_pct / 100.0;
    let silt = 1.0 - sand - clay;
    if !silt.is_finite() || silt < 0.0 {
        return Err(blocked(format!(
            "surface-layer silt remainder invalid (sand {sand}, clay {clay}, silt {silt}): \
             sand + clay exceeds 100%"
        )));
    }
    let orgmat = surface_layer.orgmat_pct / 100.0;
    let texture = openwepp_hillslope_orchestrator::ErosionTextureInputs {
        sand,
        clay,
        silt,
        orgmat,
    };
    openwepp_hillslope_orchestrator::erosion_particle_composition(&texture)
        .map_err(|e| blocked(format!("erosion particle composition failed: {e}")))
}

// Multi-operand sourcing builder: sources texture / classes / baselines /
// segments / geometry / cover constants from the parsed inputs. The line
// count is inherent to the per-field fail-closed sourcing.
#[allow(clippy::too_many_lines)]
fn direct_production_wave1_operand_seed(
    parsed_soil: &SoilProfile,
    soil_projection: &TypedSoilWb11RuntimeProjection,
    slope: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSlopeRuntimeProjection,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    peak_runoff: &DirectProductionPeakRunoffAuthority,
    is_cropland: bool,
) -> Result<openwepp_hillslope_orchestrator::DirectWave1OperandSeed, HillslopeCliError> {
    // The management PL projection indexes OFEs 1-based (`ofe1_*` / primary
    // symbol for the first OFE); the accessors only alias to the primary /
    // `_seed` symbols at `ofe_index == 1`, so the single-OFE Wave-1 seed
    // reads the first OFE as index 1 (index 0 would always miss and default).
    const FIRST_OFE_INDEX: usize = 1;

    let seed_blocked = |detail: String| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_wave1_operand_seed",
        detail: format!("{SIMOUT_GUARD_ID} {detail}"),
    };

    let soil_ofe = parsed_soil
        .ofes
        .first()
        .ok_or_else(|| seed_blocked("erosion seed requires at least one soil OFE".to_string()))?;
    let surface_layer = soil_ofe
        .layers
        .first()
        .ok_or_else(|| seed_blocked("erosion seed requires at least one soil layer".to_string()))?;
    let slope_ofe = slope
        .ofes
        .first()
        .ok_or_else(|| seed_blocked("erosion seed requires at least one slope OFE".to_string()))?;
    let corrected_layer = soil_projection.layers.first().ok_or_else(|| {
        seed_blocked("erosion seed requires at least one WB11 soil layer".to_string())
    })?;

    // Texture fractions (parser stores validated percents). The parser
    // validates sand and clay individually but NOT sand + clay <= 100, so
    // the silt remainder can be negative; fail closed on that invalid
    // texture state rather than clamping it away (the particle producer
    // would otherwise reject the masked negative silt downstream).
    let sand = surface_layer.sand_pct / 100.0;
    let clay = surface_layer.clay_pct / 100.0;
    let silt = 1.0 - sand - clay;
    if !silt.is_finite() || silt < 0.0 {
        return Err(seed_blocked(format!(
            "erosion seed surface-layer silt remainder invalid (sand {sand}, clay {clay}, \
             silt {silt}): sand + clay exceeds 100%"
        )));
    }
    let orgmat = surface_layer.orgmat_pct / 100.0;
    let rfg = surface_layer.rock_frag_pct / 100.0;

    let classes = direct_production_erosion_particle_classes(parsed_soil)?;
    let (diaeff, spgeff) = openwepp_hillslope_orchestrator::erosion_effective_particle(&classes)
        .map_err(|e| seed_blocked(format!("erosion effective particle failed: {e}")))?;
    let veleff_m_s = openwepp_hillslope_orchestrator::erosion_falvel(spgeff, diaeff);

    let baselines = openwepp_hillslope_orchestrator::erosion_consolidation_baselines(
        &openwepp_hillslope_orchestrator::ErosionConsolidationInputs {
            sand,
            silt,
            orgmat,
            thetfc: corrected_layer.thetfc,
            rock_fragment_fraction: rfg,
            ki: soil_ofe.ki,
            kr: soil_ofe.kr,
            shcrit: soil_ofe.shcrit,
        },
    )
    .map_err(|e| seed_blocked(format!("erosion consolidation baselines failed: {e}")))?;

    // Normalized slope segments (`profil.for` fit). The parsed slope
    // `xinput` is the WEPP normalized [0, 1] station fraction, but
    // `derive_wave1_slope_segments` expects the along-slope position in
    // METERS (it divides by `slplen` to normalize) — convert here.
    let points: Vec<(f64, f64)> = slope_ofe
        .points
        .iter()
        .map(|p| (p.xinput * slope_ofe.slplen_m, p.slpinp))
        .collect();
    let segments = openwepp_hillslope_orchestrator::derive_wave1_slope_segments(
        &points,
        slope_ofe.slplen_m,
        slope_ofe.avgslp,
    )
    .map_err(|e| seed_blocked(format!("erosion slope-segment fit failed: {e}")))?;
    let slpend = segments
        .last()
        .map_or(0.0, |seg| (seg.a + seg.b) * slope_ofe.avgslp);

    // Rill spacing (`rspace`) is a real operand (it materially feeds
    // `qshear`, the rill hydraulics, the adjustment factors, and `detinr`).
    // Source it from the management PL projection where present. Managements
    // without the residue/rill parameterization (the same ones for which
    // the WB16 canopy authority itself degrades to `None`, e.g. the minimal
    // `cli01` management) do not carry it; there the rill spacing is a
    // Stage-4 enable-time adjudication item (like `is_cropland` /
    // `field_width_m`), defaulted to the WEPP unit spacing behind the
    // disabled seed.
    let rspace_m = direct_production_pl_projection_optional_ofe_scalar(
        management_projection,
        FIRST_OFE_INDEX,
        "rspace",
    )
    // A non-positive `rspace` is the WEPP "no rill parameterization"
    // sentinel (same class as absent) — use the default unit spacing
    // rather than passing 0 into the rill-hydraulics domain guard.
    .filter(|value| *value > 0.0)
    .unwrap_or(WEPP_DEFAULT_RILL_SPACING_M);

    // Static rill-friction cover constants (`hmax`/`flivmx`); a burned
    // forest with no live canopy has neither, so absent defaults to 0.
    let hmax_m =
        direct_production_typed_wb16_canopy_scalar(management_projection, FIRST_OFE_INDEX, "hmax")
            .unwrap_or(0.0);
    let flivmx = direct_production_typed_wb16_canopy_scalar(
        management_projection,
        FIRST_OFE_INDEX,
        "flivmx",
    )
    .unwrap_or(0.0);

    // Random roughness (`rrinit`) — first-cut static value (no daily decay).
    // Days-since-disturbance (`daydi1`) seeds the consolidation carry;
    // absent (rill-unparameterized managements) defaults to a
    // freshly-disturbed 0. Both are behind the disabled seed here.
    let random_roughness_m = direct_production_pl_projection_optional_ofe_scalar(
        management_projection,
        FIRST_OFE_INDEX,
        "rrinit",
    )
    .unwrap_or(0.0);
    let initial_daydis = direct_production_pl_projection_optional_ofe_scalar(
        management_projection,
        FIRST_OFE_INDEX,
        "daydis",
    )
    .unwrap_or(0.0);

    Ok(openwepp_hillslope_orchestrator::DirectWave1OperandSeed {
        enabled: false,
        is_cropland,
        segments,
        slplen_m: slope_ofe.slplen_m,
        efflen_m: peak_runoff.efflen_m,
        cntlen_m: slope_ofe.slplen_m,
        rspace_m,
        field_width_m: slope_ofe.fwidth_m,
        avg_slope: slope_ofe.avgslp,
        slpend,
        sand,
        ssasol: openwepp_hillslope_orchestrator::erosion_surface_soil_ssa(sand, silt, clay, orgmat)
            .map_err(|e| {
                direct_production_executor_blocked(format!("erosion surface-soil SSA failed: {e}"))
            })?,
        classes,
        veleff_m_s,
        baselines,
        kr_s_m: soil_ofe.kr,
        ki: soil_ofe.ki,
        shcrit_pa: soil_ofe.shcrit,
        hmax_m,
        flivmx,
        random_roughness_m,
        initial_daydis,
    })
}

/// SC-SED-001 1b-C activation scope: whether the management schedules an
/// active tillage sequence (a real soil-disturbance operation). The
/// single-OFE Wave-1 first cut hardcodes the NON-TILLED (forest /
/// disturbed) operand assumptions (`is_cropland = false`, no tillage /
/// irrigation), which are correct for the validated forest-masquerade
/// targets — declared `Landuse = 1` (Cropland) but with Surface-Effect
/// index 0 (no tillage), e.g. `p61` and the DFF-WS3 disturbed-forest
/// cells. A management that applies a real tillage sequence is genuine
/// cropland whose operands are NOT yet sourced, so Wave-1 stays disabled
/// for it (narrowing the enable to the reviewed scope) until the cropland
/// operand port lands.
/// Schedule-scoped lanuse resolution for the erosion seed (the WS1
/// tie-in): every schedule-referenced yearly must agree on the lanuse
/// class — Cropland ⇒ the legacy `drinti` interrill-delivery branch
/// (`param.for:412-450`; the masquerade managements legacy actually
/// ran), Forest ⇒ the `lanuse ≠ 1` branch (`intdr = 1`). A mixed
/// schedule fails closed (one lanuse per lane; the WS1 `.man`↔`.sol`
/// reconciliation polices the forest side).
fn direct_production_schedule_lanuse_is_cropland(
    management: &ManagementParseOutput,
) -> Result<bool, HillslopeCliError> {
    let mut saw_cropland = false;
    let mut saw_forest = false;
    for slot in &management.schedule.slots {
        for &yearly_ref in &slot.yearly_refs {
            match management
                .registries
                .yearlies
                .get(yearly_ref.wrapping_sub(1))
                .map(|yearly| &yearly.data)
            {
                Some(
                    openwepp_input_contract::parsers::management::YearlyScenarioData::Cropland(_),
                ) => saw_cropland = true,
                Some(openwepp_input_contract::parsers::management::YearlyScenarioData::Forest(
                    _,
                )) => saw_forest = true,
                None => {}
            }
        }
    }
    if saw_cropland && saw_forest {
        return Err(direct_production_executor_blocked(
            "erosion lanuse resolution requires a single lanuse class per lane; \
             the schedule references both cropland and forest yearlies",
        ));
    }
    // Codex tie-in round-1: a schedule referencing NO lanuse-bearing
    // yearly has no authority for either interrill branch — fail closed
    // rather than silently selecting the non-cropland branch at the
    // exact boundary this resolution owns.
    if !saw_cropland && !saw_forest {
        return Err(direct_production_executor_blocked(
            "erosion lanuse resolution found no cropland or forest yearly \
             referenced by the schedule; the interrill branch has no \
             lanuse authority",
        ));
    }
    Ok(saw_cropland)
}

fn direct_production_management_has_active_tillage(management: &ManagementParseOutput) -> bool {
    // SCHEDULE-scoped (the PMET shape, Codex WS1-rebase round-1): only
    // yearlies the schedule actually references count. A registry-wide
    // scan would let an UNREFERENCED tilled yearly — or, on lane-sliced
    // MOFE managements (which filter the schedule per lane but clone the
    // full registries), ANOTHER OFE's yearly — disable a valid
    // native-forest/no-till lane.
    management.schedule.slots.iter().any(|slot| {
        slot.yearly_refs.iter().any(|&yearly_ref| {
            management
                .registries
                .yearlies
                .get(yearly_ref.wrapping_sub(1))
                .is_some_and(|yearly| match &yearly.data {
                    openwepp_input_contract::parsers::management::YearlyScenarioData::Cropland(
                        data,
                    ) => direct_production_tilseq_disturbs_surface(
                        data.tilseq,
                        &management.registries.surfaces,
                    ),
                    // Native forest yearlies carry no surface-effect
                    // sequence (LANUSE-AUTH-3) — never active tillage.
                    openwepp_input_contract::parsers::management::YearlyScenarioData::Forest(_) => {
                        false
                    }
                })
        })
    })
}

/// A yearly plan applies active tillage when it references a non-zero
/// surface-effect sequence (`tilseq`) whose surface scenario schedules at
/// least one operation with a positive tillage depth (`tildep > 0`). A
/// `tilseq` of 0, an out-of-range reference, or a surface with no
/// soil-disturbing operation is NOT active tillage.
fn direct_production_tilseq_disturbs_surface(
    tilseq: usize,
    surfaces: &[openwepp_input_contract::parsers::management::SurfaceScenario],
) -> bool {
    tilseq != 0
        && surfaces
            .get(tilseq - 1)
            .is_some_and(|surface| surface.operations.iter().any(|op| op.tildep > 0.0))
}

fn direct_production_typed_erosion_authority(
    parsed_soil: &SoilProfile,
    soil: &TypedSoilWb11RuntimeProjection,
    slope: &openwepp_hillslope_orchestrator::runtime_inputs::TypedSlopeRuntimeProjection,
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    peak_runoff: &DirectProductionPeakRunoffAuthority,
    management_has_active_tillage: bool,
    lanuse_is_cropland: bool,
) -> Result<DirectProductionErosionAuthority, HillslopeCliError> {
    // Wave-1 (SC-SED-001 sediment continuity): the spatial continuity
    // solve is PRODUCTION-ACTIVE post-1b-C via the operand-seed path
    // below — `direct_production_wave1_operand_seed` sources the full
    // static payload from real parsed inputs (1b-A/1b-B producers:
    // `frcfac`/`shears` hydraulics, `soil.for` daily adjustments,
    // `reid.for` `effint`/`effdrr`, `prtcmp`/`falvel`/`trcoef`
    // transport), the per-day assembly builds the daily state from the
    // frame, and the enable is gated at the end of this function
    // (single-OFE, no active tillage). `wave1_enabled` here is the
    // SEPARATE Increment-1 pointwise EROD13 coefficient check, which
    // stays disabled — it is not the continuity solve.
    let wave1_enabled = false;
    // E.3 stage 2e: the EROD14/Wave-2 arm is DELETED — Wave-1 is the
    // erosion engine on every lane, single- and multi-OFE alike.
    // SC-SED-001 1b-C: build the static Wave-1 operand seed on every lane
    // (the disabled seed is validated against real parsed inputs across the
    // full fixture suite). The per-day assembly consumes it once enabled.
    let mut wave1_operand_seed = direct_production_wave1_operand_seed(
        parsed_soil,
        soil,
        slope,
        management_projection,
        peak_runoff,
        lanuse_is_cropland,
    )?;
    // E.3 activation gate: Wave-1 enables on EVERY no-tillage lane —
    // single- and multi-OFE alike (each lane's seed is per-OFE by
    // construction via the intake slicing; the inter-OFE handoff supplies
    // qin/strldn/continuity). The no-tillage gate keeps the reviewed
    // forest/disturbed operand scope (the seed hardcodes
    // non-cropland/non-tilled operands); genuine tilled cropland stays
    // disabled until its operands are sourced.
    wave1_operand_seed.enabled = !management_has_active_tillage;
    Ok(DirectProductionErosionAuthority {
        erosion_inputs: DirectErosionInputs {
            wave1_enabled,
            wave1: DirectErod13Inputs::zero(),
            wave1_continuity: Box::new(
                openwepp_hillslope_orchestrator::DirectWave1ContinuityInputs::zero(),
            ),
            wave1_operand_seed: Box::new(wave1_operand_seed),
            hydrograph_shape_authority:
                openwepp_hillslope_orchestrator::DirectErosionHydrographShapeAuthority::Dc01SourceShape,
            routed_hydrograph_runoff_fraction: None,
        },
    })
}

fn direct_production_typed_growth_authority(
    management_projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    climate_request: &HillslopeClimateRuntimeRequest,
    soil: &TypedSoilWb11RuntimeProjection,
) -> Result<DirectProductionGrowthAuthority, HillslopeCliError> {
    let Some(slot_count_value) = direct_production_pl_projection_optional_scalar(
        management_projection,
        "pl_schedule_slot_count",
    ) else {
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
    let schedule = direct_production_typed_growth_crop_schedule_authority(
        management_projection,
        slot_index,
        crop_slot_index,
    )?;
    Ok(DirectProductionGrowthCropAuthority {
        schedule_imngmt: schedule.schedule_imngmt,
        imngmt: schedule.imngmt,
        jdharv: schedule.jdharv,
        jdplt: schedule.jdplt,
        jdstop: schedule.jdstop,
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
        bbb: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "bbb"),
        )?,
        hmax: direct_growth_projection_required_scalar(
            management_projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "hmax"),
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
        forest_phenology: schedule.forest_phenology,
    })
}

fn direct_production_forest_phenology_authority(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    slot_index: usize,
    crop_slot_index: usize,
) -> Result<Option<DirectProductionForestPhenologyAuthority>, HillslopeCliError> {
    let model_symbol =
        direct_growth_slot_crop_symbol(slot_index, crop_slot_index, "forest_phenology_model");
    let Some(model) = direct_production_pl_projection_optional_scalar(projection, &model_symbol)
    else {
        return Ok(None);
    };
    let model = direct_growth_integral_usize(&model_symbol, model, 1, 1)?;
    if model != 1 {
        return Err(direct_growth_failure(format!(
            "unsupported native forest phenology model {model}"
        )));
    }
    let required = |root: &str| {
        direct_growth_projection_required_scalar(
            projection,
            &direct_growth_slot_crop_symbol(slot_index, crop_slot_index, root),
        )
    };
    Ok(Some(DirectProductionForestPhenologyAuthority {
        summer_foliar_biomass_kg_m2: required("forest_summer_foliar_biomass_kg_m2")?,
        evergreen_fraction: required("forest_evergreen_fraction")?,
        structural_canopy_cover_fraction: required("forest_structural_canopy_cover_fraction")?,
        structural_biomass_kg_m2: required("forest_structural_biomass_kg_m2")?,
        minimum_temperature_inactive_c: required("forest_minimum_temperature_inactive_c")?,
        minimum_temperature_unconstrained_c: required(
            "forest_minimum_temperature_unconstrained_c",
        )?,
        vapor_pressure_deficit_unconstrained_pa: required(
            "forest_vapor_pressure_deficit_unconstrained_pa",
        )?,
        vapor_pressure_deficit_inactive_pa: required("forest_vapor_pressure_deficit_inactive_pa")?,
        photoperiod_inactive_hours: required("forest_photoperiod_inactive_hours")?,
        photoperiod_unconstrained_hours: required("forest_photoperiod_unconstrained_hours")?,
    }))
}

fn direct_growth_projection_required_scalar(
    projection: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopePlRuntimeSurfaces,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let value =
        direct_production_pl_projection_optional_scalar(projection, symbol).ok_or_else(|| {
            direct_growth_failure(format!("missing required direct growth symbol {symbol}"))
        })?;
    if !value.is_finite() {
        return Err(direct_growth_failure(format!(
            "required direct growth symbol {symbol} must be finite, observed {value}"
        )));
    }
    Ok(value)
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
    u8::try_from(parsed)
        .map_err(|_| direct_growth_failure(format!("{symbol} value {parsed} outside u8 range")))
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
    u16::try_from(parsed)
        .map_err(|_| direct_growth_failure(format!("{symbol} value {parsed} outside u16 range")))
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
        frost_active: frost_runtime_depth_m > 1.0e-12 || frost_runtime_frozen_water_m > 1.0e-12,
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
    let fine_top_count = direct_production_typed_frost_fine_count(
        "frost.options.fineTop",
        frost_projection.fine_top,
    )?;
    let fine_bot_count = direct_production_typed_frost_fine_count(
        "frost.options.fineBot",
        frost_projection.fine_bot,
    )?;
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
            detail: format!(
                "{SIMOUT_GUARD_ID} {symbol} must be an integer in [1,10], observed {value}"
            ),
        });
    }
    usize::try_from(value).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_production_typed_seed",
        detail: format!("{SIMOUT_GUARD_ID} {symbol} could not convert to usize: {value}"),
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
    soil: &TypedSoilWb11RuntimeProjection,
    frost_projection: &openwepp_hillslope_orchestrator::runtime_inputs::TypedFrostRuntimeProjection,
) -> DirectProductionInfiltrationAuthority {
    DirectProductionInfiltrationAuthority {
        effective_conductivity_m_s: Some(frost_projection.infcap_frz_m_s),
        ksatadj_policy: soil.ksatadj.then(|| DirectProductionKsatadjPolicy {
            solwpv: soil.solwpv,
            ksatfac_mm_h: soil.ksatfac_mm_h,
            ksatrec_per_day: soil.ksatrec_per_day,
            lkeff_mm_h: soil.lkeff_mm_h,
            layers: soil
                .layers
                .iter()
                .map(|layer| DirectProductionKsatadjLayerPolicy { cpm: layer.cpm })
                .collect(),
        }),
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
    let drain_enabled =
        direct_production_pl_projection_optional_flag(management_projection, "wb19_drain_enabled")?
            .unwrap_or(false);
    let drain_depth_m = if drain_enabled {
        direct_production_pl_projection_required_scalar(management_projection, "wb19_drain_depth")?
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
                detail: format!("{SIMOUT_GUARD_ID} typed PL projection missing finite {symbol}"),
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
        canopy_height_m: value("canhgt")?,
        canopy_cover_fraction: value("cancov")?,
        leaf_area_index: value("lai")?,
        root_mass_kg_m2: value("rtmass")?,
        root_depth_m: value("rtd")?,
        harvest_index: hia,
    })
}

/// E.3: the Wave-1 chain is the inter-OFE erosion authority ONLY when the
/// run is multi-OFE AND the Wave-1 seed actually enables (the no-tillage
/// scope) — a tilled multi-OFE run has NO erosion producer, and the
/// manifest must not claim sediment-coupled qin for it
/// (`SC-RUNOFFPART-001#INV-RUNOFFPART-030` disposition truthfulness;
/// Codex 2e round-1 High).
fn direct_production_typed_multi_ofe_wave1_chained(
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
            "the typed erosion seed requires at least one OFE",
        ));
    }
    let wave1_seed_enabled = !direct_production_management_has_active_tillage(&inputs.management);
    Ok(ofe_count > 1 && wave1_seed_enabled)
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
        .map(
            |((layer, store), frozen_depth_m)| DirectSubsurfaceLayerState {
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
            },
        )
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

#[cfg(test)]
mod erosion_tillage_scope_tests {
    use super::direct_production_tilseq_disturbs_surface;
    use openwepp_input_contract::parsers::management::{
        ScenarioMeta, SurfaceOperation, SurfaceScenario,
    };

    fn surface_with_tillage_depth(tildep: f64) -> SurfaceScenario {
        SurfaceScenario {
            meta: ScenarioMeta {
                name: String::new(),
                description: [String::new(), String::new(), String::new()],
                landuse: 1,
            },
            ntill: 1,
            operations: vec![SurfaceOperation {
                mdate: 100,
                op_ref: 1,
                tildep,
                typtil: 1,
            }],
        }
    }

    #[test]
    fn no_tillage_scope_disables_wave1_for_tilled_cropland_only() {
        let tilled = [surface_with_tillage_depth(0.1)];

        // tilseq 0 (the WEPP "no surface effect" sentinel, as p61 / the
        // DFF-WS3 forest-masquerade cells carry) is NOT active tillage.
        assert!(!direct_production_tilseq_disturbs_surface(0, &tilled));

        // A real tilseq referencing a soil-disturbing operation (tildep > 0)
        // IS active tillage -> Wave-1 stays disabled for it.
        assert!(direct_production_tilseq_disturbs_surface(1, &tilled));

        // A referenced surface whose only operation has zero tillage depth
        // does not disturb the soil -> not active tillage.
        let flat = [surface_with_tillage_depth(0.0)];
        assert!(!direct_production_tilseq_disturbs_surface(1, &flat));

        // An out-of-range tilseq (dangling reference) is not active tillage.
        assert!(!direct_production_tilseq_disturbs_surface(9, &tilled));
    }

    #[test]
    fn lanuse_resolution_is_schedule_scoped_and_fails_closed_without_authority() {
        // Codex tie-in round-1: a cropland-scheduled management resolves
        // cropland; the SAME registries with no referenced yearlies have
        // no lanuse authority and must fail closed (never a silent
        // non-cropland branch selection).
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/infile/management/canonical_rotation_nonzero_98_4.man");
        let mut management =
            openwepp_input_contract::parsers::management::parse_management_from_path(
                &fixture,
                openwepp_input_contract::parsers::management::ParseMode::Compatibility,
            )
            .expect("tilled rotation fixture parses");
        assert!(
            super::direct_production_schedule_lanuse_is_cropland(&management)
                .expect("scheduled cropland resolves"),
            "cropland yearlies resolve is_cropland = true"
        );
        for slot in &mut management.schedule.slots {
            slot.yearly_refs.clear();
        }
        assert!(
            super::direct_production_schedule_lanuse_is_cropland(&management).is_err(),
            "no referenced lanuse-bearing yearly must fail closed"
        );
    }

    #[test]
    fn tillage_detector_is_schedule_scoped_not_registry_scoped() {
        // Codex WS1-rebase round-1 (Medium): an UNREFERENCED tilled
        // yearly in the registry must not disable a no-till lane — on
        // lane-sliced MOFE managements the schedule is per-lane but the
        // registries are cloned whole, so registry scanning would let
        // another OFE's tillage leak across lanes.
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/infile/management/canonical_rotation_nonzero_98_4.man");
        let mut management =
            openwepp_input_contract::parsers::management::parse_management_from_path(
                &fixture,
                openwepp_input_contract::parsers::management::ParseMode::Compatibility,
            )
            .expect("tilled rotation fixture parses");
        assert!(
            super::direct_production_management_has_active_tillage(&management),
            "the schedule references the tilled yearly: active"
        );
        // Same registries, but no slot references any yearly: the tilled
        // scenario is now registry-resident-only and must NOT count.
        for slot in &mut management.schedule.slots {
            slot.yearly_refs.clear();
        }
        assert!(
            !super::direct_production_management_has_active_tillage(&management),
            "an unreferenced tilled yearly must not disable the lane"
        );
    }
}
