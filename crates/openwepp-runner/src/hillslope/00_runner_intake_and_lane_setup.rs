use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::hillslope::intake_lane_setup::{
    legacy_sunmap_horizontal_radpot_ly, saturation_vapor_pressure_kpa,
};
use openwepp_hillslope_orchestrator::runtime_inputs::{
    DirectWinterHourlyContext, HillslopeClimateRuntimeRequest, HillslopeDirectClimateDayForcing,
    SlopeRuntimeSurfaceOptions,
    build_hillslope_climate_runtime_request,
    build_hillslope_pl_runtime_surfaces_from_management,
    project_typed_frost_runtime, project_typed_snow_runtime, project_typed_soil_wb11_runtime,
    TypedSoilWb11RuntimeProjection,
};
use openwepp_hillslope_orchestrator::{
    DirectActiveFrostPartitionInputs, DirectActiveSnowPartitionInputs, DirectCanopyInterceptionInputs, DirectErod13Inputs,
    DirectErosionInputs, DirectEvapotranspirationComputeInputs,
    DirectEvapotranspirationPmetInputs, DirectEvapotranspirationStageState,
    DirectExecutorMode, DirectFrameExecutor,
    DirectFrostControlInputs, DirectFrostFineLayerProjection, DirectFrostHourlyForcing,
    DirectFrostLaneState, DirectFrostLayerCarryProjection, DirectFrostLayerInput,
    DirectFrostLayerShadowProjection,
    DirectFrostPriorStateInput, DirectFrostRuntimeCarry, DirectFrostThermalInputs,
    FrostSeasonalTemperatureCurve,
    DirectDecompositionAction, DirectDecompositionActiveContext, DirectDecompositionInputs,
    DirectGroundwaterAuthority,
    DirectGrowthAction, DirectGrowthActiveContext, DirectGrowthInputs, DirectGrowthStateSurface,
    DirectHydrologyProjectionInputs, DirectKsatadjEffectiveConductivityInputs,
    DirectKsatadjLayerInputs, DirectWinterFrostComputeInputs,
    DirectWinterFrostPartitionOutcome,
    DirectInfiltrationDepressionInputs, DirectLaneConstructorInputs, DirectLaneFrame,
    DirectLiquidInputInputs,
    DirectPeakRunoffInputs, DirectPercolationInputs, DirectPublicationCalendarDay,
    DirectPublicationDayInput, DirectPublicationDayRow, DirectPublicationRunMetadata,
    DirectRunConstructorInputs, DirectRunFrame,
    DirectRunIdentity, DirectRuntimeAuditSnapshot, DirectRuntimeError,
    DirectStreamingPublicationExecution,
    DirectSnowCouplingInputs, DirectSnowHourlyForcing, DirectSnowLaneState,
    DirectResiduePartitionInputs, DirectStorageInputInputs, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceLayerState, DirectWb14HyetographInterval,
    DirectWb14InfiltrationProducerInputs,
    Wb11HydrologyKernel, compute_direct_canopy_interception, direct_runtime_audit_snapshot,
};
#[cfg(test)]
use openwepp_hillslope_orchestrator::DirectRunPublicationFrame;
use openwepp_hillslope_output::contracts::{HillslopeOutputConfig, validate_output_contract};
use openwepp_hillslope_output::hillslope_pass::{
    HillslopePassParquetRowGroupWriter, HillslopePassRow,
};
use openwepp_hillslope_output::hillslope_wat::{
    HillslopeWatParquetRowGroupWriter, HillslopeWatRow, InterchangeVersion,
};
use openwepp_hillslope_output::manifest::{OutputChecksumEntry, assemble_output_checksums};
use openwepp_hillslope_output::writers::{optional_output_paths, required_output_paths};
use openwepp_input_contract::parsers::climate::{
    ClimateDailyRecord, ClimateFile, parse_climate_file,
};
use openwepp_input_contract::parsers::frost::{
    FrostParseOutput, parse_frost_from_path, parse_frost_from_str,
};
use openwepp_input_contract::parsers::gwcoeff::{GwcoeffFile, parse_gwcoeff_from_path};
use openwepp_input_contract::parsers::management::{
    ManagementParseOutput, YearlyScenarioData, parse_management_document_from_path,
};
use openwepp_input_contract::parsers::pmetpara::{
    ParseMode as PmetparaParseMode, PmetLookupState, PmetparaFile, PmetparaParseOptions,
    parse_pmetpara_file,
};
use openwepp_input_contract::parsers::slope::{SlopeProfile, parse_slope_file};
use openwepp_input_contract::parsers::snow::{
    SnowParseOutput, parse_snow_file, parse_snow_from_str,
};
use openwepp_input_contract::parsers::soil::{
    SoilParserOptions, SoilProfile, TopologyScope, parse_soil,
};
use openwepp_input_contract::parsers::wepp_ui::{
    WeppUiParseResult, WeppUiParserOptions, parse_wepp_ui_from_path,
};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue,
};
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterRequest, SidecarBinding, SidecarContract, SidecarDiscovery, SidecarId,
    SidecarRequirement, adapt_sidecar_bindings,
};
use serde::{Deserialize, Serialize};

use crate::api::{
    HillslopeDefaultRuntimeActivation, HillslopeRunReport, HillslopeRunRequest,
    HillslopeRuntimeSelection, HillslopeRuntimeSelectionPolicy,
    HillslopeRuntimeSelectionResolution,
};
use crate::hillslope::intake_lane_setup::StaticOfeLaneSlice;
use crate::constants::{
    DAILY_EXECUTION_LANE, DAILY_TIMESTEP_SECONDS, DIRECT_PUBLICATION_FRAME_PUBLICATION_SOURCE,
    HILLSLOPE_RUN_MANIFEST_SCHEMA_ID, HILLSLOPE_RUNFILE_SCHEMA_ID, HOURLY_EXECUTION_LANE,
    HOURLY_TIMESTEP_SECONDS, REQUIRED_RUN_OUTPUT_LOSS, REQUIRED_RUN_OUTPUT_PASS,
    SIMCONS_INTAKE_GUARD_ID, SIMCOUP_GUARD_ID, SIMIMPL09_ADOPT_PROFILE,
    SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
    SIMMODE_TIMESTEP_GUARD_ID, SIMOUT_GUARD_ID, SIMPIPE_GUARD_ID, SUBHOURLY_EXECUTION_LANE,
    WB13_PUBLICATION_SOURCE_DIRECT_PUBLICATION_FRAME, WUI_MODE_GUARD_ID,
};
use crate::errors::HillslopeCliError;
use crate::release::write_release_sidecar_for_binary;
use crate::role::BinaryRole;
use crate::shared::{
    file_name_string, git_source_commit_or_unknown, path_has_extension_case_insensitive,
    sha256_file_hex, utc_now_rfc3339,
};

const EROD14_QIN_POLICY_WAVE2_DISABLED: &str = "wave2-disabled";
// E.3 (INV-SED-012 / INV-RUNOFFPART-030 lift): multi-OFE erosion qin is the
// Wave-1 hourly sediment-coupled handoff (prior-lane erosion qout + qsout +
// class fractions + continuity state), never a water-transfer substitute.
const EROD14_QIN_POLICY_WAVE1_SEDIMENT_COUPLED: &str = "wave1-hourly-sediment-coupled-handoff";


#[derive(Debug, Serialize)]
struct HillslopeRunManifest {
    schema: String,
    engine: String,
    binary_path: String,
    binary_sha256: String,
    binary_sidecar_path: String,
    binary_sidecar_sha256: String,
    source_commit: String,
    invoked_utc: String,
    argv: Vec<String>,
    run_dir: String,
    run_file: String,
    sidecar_policy: String,
    sidecar_discovery_mode: String,
    resolved_sidecars: BTreeMap<String, String>,
    input_checksums: BTreeMap<String, String>,
    output_checksums: BTreeMap<String, String>,
    runtime_selection: HillslopeRuntimeSelectionProvenance,
    mode_selection: HillslopeModeSelectionProvenance,
    timestep_policy: HillslopeTimestepPolicyProvenance,
    adapter_boundary: HillslopeAdapterBoundaryProvenance,
    execution_provenance: HillslopeExecutionProvenance,
    wb13_publication: HillslopeWb13PublicationProvenance,
    mofe_hourly_carry: HillslopeMofeHourlyCarryProvenance,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_runtime_counters: Option<HillslopeDirectRuntimeCounterProvenance>,
    coupling_vectors: HillslopeCouplingVectorProvenance,
}

#[derive(Debug, Serialize)]
struct HillslopeDirectRuntimeCounterProvenance {
    run_frame_constructions: u64,
    day_frame_constructions: u64,
    day_frame_commits: u64,
    executor_constructions: u64,
    skeleton_runs: u64,
    publication_capture_runs: u64,
    phase_view_constructions: u64,
    erod14_qin_clamped_events: u64,
    /// E.3 (SC-SED-001 INV-SED-016 (f)): Wave-1 hour quanta refused by the
    /// flux-consistency diagnostic and skipped with zero sediment — the
    /// surfaced under-estimate count.
    wave1_flux_refused_quanta: u64,
    phase_span_runs: u64,
    direct_phase_entries: u64,
    direct_compute_operations: u64,
    direct_state_mutations: u64,
    downstream_operand_productions: u64,
    shadow_projections: u64,
    compatibility_edge_invocations: u64,
    ksatadj_effective_conductivity_evaluations: u64,
}

#[derive(Debug, Serialize)]
struct HillslopeRuntimeSelectionProvenance {
    requested: String,
    selected: String,
    selection_reason: String,
    default_activation_gate: String,
    fallback_reason: Option<String>,
    output_policy: String,
    rollback_runtime: String,
    compatibility_rollback_available: bool,
}

#[derive(Debug, Serialize)]
struct HillslopeModeSelectionProvenance {
    wepp_ui: WeppUiModeSelectionProvenance,
}

#[derive(Debug, Serialize)]
struct WeppUiModeSelectionProvenance {
    requested: i32,
    effective: i32,
    selected_lane: String,
    mode_divergence: bool,
    guard_id: String,
}

#[derive(Debug, Serialize)]
struct HillslopeTimestepPolicyProvenance {
    scheduler_mode: String,
    requested_mode: String,
    effective_mode: String,
    selected_lane: String,
    policy: String,
    timestep_seconds: u32,
    physics_enabled: bool,
    subhourly_scaffold_available: bool,
    guard_id: String,
}

#[derive(Debug, Serialize)]
struct HillslopeAdapterBoundaryProvenance {
    selected_lane: String,
    scheduler_mode: String,
    requested_mode: String,
    effective_mode: String,
    adopt_profile: String,
    reject_surfaces_excluded: bool,
    defer_surfaces_excluded: bool,
    guard_id: String,
}

#[derive(Debug, Serialize)]
#[allow(clippy::struct_excessive_bools)]
struct HillslopeExecutionProvenance {
    scheduler_kernel_executed: bool,
    publication_source: String,
    simpipe_guard_id: String,
    selected_lane: String,
    scheduler_outcome_class: String,
    scheduler_status_message_id: String,
    climate_day_count: usize,
    executed_day_count: usize,
    kernel_phase_message_ids: Vec<String>,
    erod14_wave2_enabled: bool,
    /// E.3 stage 2e: replaces `erod14_wave2_kernel_status_seen` (dead
    /// forever-false with the kernel deleted) — true when the Wave-1
    /// chain is the inter-OFE erosion authority (`ofe_count > 1`).
    multi_ofe_wave1_chained: bool,
    erod14_qin_source_policy: String,
    erod14_qin_sediment_coupled: bool,
    wb16_ealpha_compatibility_seed_used: bool,
    wb16_ealpha_seed_policy: String,
    /// Lane D seam shadow diagnostics — present ONLY when the opt-in
    /// shadow ran (`INV-OFEROUTE-012` activation increment).
    #[serde(skip_serializing_if = "Option::is_none")]
    laned_shadow: Option<LanedShadowProvenance>,
    /// Lane D ACTIVE owner evidence — present ONLY when the opt-in active
    /// selector ran (`SC-OFEROUTE-001` rev 27).
    #[serde(skip_serializing_if = "Option::is_none")]
    laned_active: Option<LanedActiveProvenance>,
}

#[derive(Debug, Serialize)]
struct LanedActiveProvenance {
    #[serde(skip_serializing_if = "Option::is_none")]
    trace_record_count: Option<usize>,
    mesh_policy: LanedActiveMeshPolicyProvenance,
    max_dt_s: f64,
    days_seen: u64,
    days_routed: u64,
    days_uniform_shape: u64,
    total_source_m3: f64,
    total_routed_outlet_m3: f64,
    total_end_window_storage_m3: f64,
    total_clamp_m3: f64,
    total_tail_fold_m3: f64,
    total_latqcc_outlet_m3: f64,
    groundwater_enabled_days: u64,
    total_groundwater_recharge_m3: f64,
    total_groundwater_baseflow_m3: f64,
    total_groundwater_deep_seepage_m3: f64,
    initial_groundwater_storage_m3: Option<f64>,
    terminal_groundwater_storage_m3: Option<f64>,
    terminal_groundwater_baseflow_m3: Option<f64>,
    terminal_groundwater_deep_seepage_m3: Option<f64>,
    max_supply_reconstruction_rel: f64,
    max_day_cascade_residual_rel: f64,
    max_day_seam_residual_rel: f64,
    max_day_identity_residual_rel: f64,
    lane_days_erosion_source_shape_degenerate: u64,
}

#[derive(Debug, Serialize)]
struct LanedActiveMeshPolicyProvenance {
    mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_cells: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_dx_m: Option<f64>,
    min_cells: usize,
    max_cells: usize,
}

#[derive(Debug, Serialize)]
struct LanedShadowProvenance {
    days_seen: u64,
    days_routed: u64,
    days_uniform_shape: u64,
    days_uniform_shape_with_routed_melt: u64,
    days_uniform_shape_without_routed_melt: u64,
    max_router_conservation_rel: f64,
    aggregate_router_conservation_rel: f64,
    max_supply_reconstruction_rel: f64,
    total_source_m3: f64,
    total_routed_outlet_m3: f64,
}

#[derive(Debug, Serialize)]
struct HillslopeWb13RowKeyProvenance {
    year: i32,
    julian_day: u16,
    ofe: u16,
    sim_day_index: i32,
}

#[derive(Debug, Serialize)]
#[allow(clippy::struct_excessive_bools)]
struct HillslopeWb13PublicationProvenance {
    source: String,
    projection_fallback_used: bool,
    guard_id: String,
    replay_candidate_surfaces: Vec<String>,
    publication_ofe_policy: String,
    contributor_ofe_count: usize,
    static_per_ofe_slice_count: usize,
    per_ofe_state_policy: String,
    per_ofe_dynamic_water_balance_state: bool,
    per_ofe_dynamic_wb_state: bool,
    per_ofe_record_count: usize,
    transfer_identity_status: String,
    per_element_identity_status: String,
    aggregate_identity_status: String,
    area_policy: String,
    storage_lineage_policy: String,
    per_ofe_internal_day_count: usize,
    per_ofe_expected_record_count: usize,
    transfer_identity_max_abs_mm: f64,
    per_element_identity_max_abs_mm: f64,
    aggregate_transfer_cancellation_max_abs_mm: f64,
    hillslope_total_identity_max_abs_mm: f64,
    publication_area_m2: f64,
    row_count: usize,
    sim_day_index_monotonic: bool,
    first_row_key: HillslopeWb13RowKeyProvenance,
    last_row_key: HillslopeWb13RowKeyProvenance,
}

#[derive(Debug, Serialize)]
struct HillslopeMofeHourlyCarryProvenance {
    policy: String,
    active: bool,
    substep_count: usize,
    required_arrays: Vec<String>,
    upstream_carry_total_m: f64,
    current_carry_total_m: f64,
}

#[derive(Debug, Serialize)]
struct HillslopeCouplingVectorProvenance {
    guard_id: String,
    winter: HillslopeWinterCouplingProvenance,
    soil: HillslopeSoilCouplingProvenance,
    frsoil: HillslopeFrozenSoilCouplingProvenance,
    hydout_equivalent: HillslopeHydoutEquivalentCouplingProvenance,
}

#[derive(Debug, Serialize)]
struct HillslopeWinterCouplingProvenance {
    active: bool,
    snow_file_present: bool,
    rst: f64,
    newsnw: f64,
    ssd: f64,
    runtime_swe: f64,
}

#[derive(Debug, Serialize)]
struct HillslopeSoilCouplingProvenance {
    ssc: f64,
    infiltration_capacity_frozen: f64,
    infcap_within_ssc: bool,
}

#[derive(Debug, Serialize)]
struct HillslopeFrozenSoilCouplingProvenance {
    active: bool,
    frost_file_present: bool,
    wint_red_enabled: bool,
    dfrost: f64,
    dthaw: f64,
    nft: f64,
    ws_frz: f64,
    infcap_frz: f64,
}

#[derive(Debug, Serialize)]
struct HillslopeHydoutEquivalentCouplingProvenance {
    source: String,
    total_soil: f64,
    frozwt: f64,
    snow_water: f64,
    soil_water_total: f64,
    closure_delta: f64,
    closure_tolerance: f64,
    closure_within_tolerance: bool,
}

const WB16_EALPHA_SEED_POLICY_RUNTIME_PROVIDED: &str = "runtime_provided";
const MOFE_HOURLY_CARRY_POLICY: &str = "baseline-wathour-24-slot-copy-forward";
const MOFE_HOURLY_CARRY_ARRAY_COUNT: usize = 24;
const MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT: &str = "ui_SUrunf";
const MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT: &str = "ui_SCrunf";
const MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT: &str = "ui_LfUrf";
const MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT: &str = "ui_LfCrf";
const MOFE_HOURLY_REQUIRED_ARRAYS: [&str; 4] = [
    MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT,
    MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
    MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
    MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutionLane {
    Daily,
    Hourly,
}

impl ExecutionLane {
    #[must_use]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Daily => DAILY_EXECUTION_LANE,
            Self::Hourly => HOURLY_EXECUTION_LANE,
        }
    }

    fn parse(value: &str) -> Result<Self, HillslopeCliError> {
        match value {
            DAILY_EXECUTION_LANE => Ok(Self::Daily),
            HOURLY_EXECUTION_LANE => Ok(Self::Hourly),
            _ => Err(timestep_policy_failure(format!(
                "unsupported execution lane '{value}' (supported lanes: {DAILY_EXECUTION_LANE}|{HOURLY_EXECUTION_LANE}; {SUBHOURLY_EXECUTION_LANE} scaffold is non-executable)"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimestepPolicy {
    Daily,
    Hourly,
    SubHourly { timestep_seconds: u32 },
}

impl TimestepPolicy {
    #[must_use]
    const fn from_lane(lane: ExecutionLane) -> Self {
        match lane {
            ExecutionLane::Daily => Self::Daily,
            ExecutionLane::Hourly => Self::Hourly,
        }
    }

    #[must_use]
    const fn scaffold_subhourly(timestep_seconds: u32) -> Self {
        Self::SubHourly { timestep_seconds }
    }

    #[must_use]
    const fn policy_name(self) -> &'static str {
        match self {
            Self::Daily => DAILY_EXECUTION_LANE,
            Self::Hourly => HOURLY_EXECUTION_LANE,
            Self::SubHourly { .. } => SUBHOURLY_EXECUTION_LANE,
        }
    }

    #[must_use]
    const fn scheduler_mode(self) -> &'static str {
        match self {
            Self::Daily => DAILY_EXECUTION_LANE,
            Self::Hourly => HOURLY_EXECUTION_LANE,
            Self::SubHourly { .. } => SUBHOURLY_EXECUTION_LANE,
        }
    }

    #[must_use]
    const fn timestep_seconds(self) -> u32 {
        match self {
            Self::Daily => DAILY_TIMESTEP_SECONDS,
            Self::Hourly => HOURLY_TIMESTEP_SECONDS,
            Self::SubHourly { timestep_seconds } => timestep_seconds,
        }
    }

    #[must_use]
    const fn physics_enabled(self) -> bool {
        !matches!(self, Self::SubHourly { .. })
    }
}

#[derive(Debug, Clone)]
struct ExecutionLaneContext {
    lane: ExecutionLane,
    requested_mode: &'static str,
    effective_mode: &'static str,
    timestep_policy: TimestepPolicy,
}

#[derive(Debug, Deserialize, Default)]
struct HillslopeRunfileDocument {
    schema: String,
    run_name: String,
    unit_system: String,
    #[serde(default)]
    inputs: HillslopeRunfileInputs,
    #[serde(default)]
    outputs: HillslopeRunfileOutputs,
}

#[derive(Debug, Deserialize, Default)]
struct HillslopeRunfileInputs {
    soil: String,
    management: String,
    slope: String,
    climate: String,
    #[serde(default)]
    wepp_ui: bool,
    pmetpara: Option<String>,
    snow: Option<RunfileSnowInline>,
    frost: Option<RunfileFrostInline>,
}

#[derive(Debug, Deserialize, Default)]
struct HillslopeRunfileOutputs {
    pass: String,
    loss: String,
    pass_parquet: Option<String>,
    wat: Option<String>,
    soil: Option<String>,
    plot: Option<String>,
    ebe: Option<String>,
    element: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
struct RunfileSnowInline {
    rst: f64,
    newsnw: f64,
    ssd: f64,
}

#[derive(Debug, Deserialize, Clone, Copy)]
struct RunfileFrostInline {
    #[serde(rename = "wintRed")]
    wint_red: i32,
    #[serde(rename = "fineTop")]
    fine_top: i32,
    #[serde(rename = "fineBot")]
    fine_bot: i32,
    ksnowf: f64,
    kresf: f64,
    ksoilf: f64,
    kfactor1: f64,
    kfactor2: f64,
    kfactor3: f64,
}

#[derive(Debug, Default)]
struct RunfileSidecarOverrides {
    wepp_ui: bool,
    pmetpara_path: Option<PathBuf>,
    snow: Option<RunfileSnowInline>,
    frost: Option<RunfileFrostInline>,
}

#[derive(Debug)]
struct RunfileExecutionConfig {
    run_name: String,
    soil_path: PathBuf,
    management_path: PathBuf,
    slope_path: PathBuf,
    climate_path: PathBuf,
    output_config: HillslopeOutputConfig,
    sidecar_overrides: RunfileSidecarOverrides,
}

struct ParsedHillslopeRunInputs {
    run_file_path: PathBuf,
    runfile: RunfileExecutionConfig,
    soil_path: PathBuf,
    management_path: PathBuf,
    slope_path: PathBuf,
    climate_path: PathBuf,
    slope: SlopeProfile,
    management: ManagementParseOutput,
    soil: SoilProfile,
    climate: ClimateFile,
}

struct HillslopeOutputTargets {
    output_pass: PathBuf,
    output_loss: PathBuf,
    optional_outputs: Vec<PathBuf>,
    laned_active_trace: Option<PathBuf>,
    output_hillslope_id: u32,
}

#[derive(Default)]
struct HillslopeSidecarInputPaths {
    snow: Option<PathBuf>,
    frost: Option<PathBuf>,
    wepp_ui: Option<PathBuf>,
    pmetpara: Option<PathBuf>,
    gwcoeff: Option<PathBuf>,
}

struct HillslopeSidecarResolution {
    snow: SnowParseOutput,
    frost: FrostParseOutput,
    gwcoeff: GwcoeffFile,
    mode_selection: HillslopeModeSelectionProvenance,
    pmetpara: PmetparaFile,
    pmetpara_mode: PmetparaParseMode,
    resolved_sidecars: BTreeMap<String, String>,
    sidecar_warnings: Vec<String>,
    input_paths: HillslopeSidecarInputPaths,
    discovery_mode: &'static str,
}

struct StaticHillslopeRuntimeSetup {
    timestep_policy: HillslopeTimestepPolicyProvenance,
    adapter_boundary: HillslopeAdapterBoundaryProvenance,
    execution_state: HillslopeClimateExecutionState,
}

struct HillslopeClimateExecutionState {
    per_ofe_lane_areas_m2: Vec<f64>,
    per_ofe_runoff_publication_geometries: Vec<Wb13RunoffPublicationGeometry>,
    lane_context: ExecutionLaneContext,
    climate_span: ClimateRunSpanSummary,
}

struct HillslopeClimateExecution {
    selected_lane: ExecutionLane,
    climate_span: ClimateRunSpanSummary,
    coupling_vectors: HillslopeCouplingVectorProvenance,
    multi_ofe_wave1_chained: bool,
    laned_shadow: Option<crate::hillslope::laned_shadow::LanedShadowSummary>,
    laned_active: Option<openwepp_hillslope_orchestrator::DirectLanedActiveRunSummary>,
    scheduler_outcome_class: &'static str,
    scheduler_status_message_id: String,
    kernel_phase_message_ids: Vec<String>,
    executed_day_count: usize,
    retained_direct_publication: Option<RetainedDirectPublication>,
    direct_publication: Option<DirectPublicationArtifacts>,
}

struct DirectPublicationArtifacts {
    execution: DirectStreamingPublicationExecution,
    summary: DirectPublicationOutputSummary,
    hbp_bytes: Vec<u8>,
    wat_rows_written: Option<usize>,
    pass_projection_rows_written: Option<usize>,
    loss_text: String,
    manifest_text: String,
}

struct RetainedDirectPublication {
    execution: DirectStreamingPublicationExecution,
    stream: DirectPublicationStreamResult,
    /// Lane D seam shadow summary (`OPENWEPP_LANED_SHADOW=1` opt-in);
    /// `None` when the shadow is off — the manifest then carries no
    /// shadow keys (`INV-OFEROUTE-010` byte-identity posture).
    laned_shadow: Option<crate::hillslope::laned_shadow::LanedShadowSummary>,
    /// Lane D ACTIVE owner run summary (`OPENWEPP_LANED_ACTIVE=1` opt-in,
    /// rev 27); `None` when inactive — the manifest then carries no active
    /// keys.
    laned_active: Option<openwepp_hillslope_orchestrator::DirectLanedActiveRunSummary>,
}

struct DirectPublicationStreamResult {
    summary: DirectPublicationOutputSummary,
    wat_rows_written: Option<usize>,
    pass_projection_rows_written: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
struct DirectPublicationOutputSummary {
    identity: DirectRunIdentity,
    metadata: DirectPublicationRunMetadata,
    row_count: usize,
    first_row: Option<DirectPublicationDayRow>,
    last_row: Option<DirectPublicationDayRow>,
    hbp_sediment_row: Option<DirectPublicationDayRow>,
    /// E.3: the CHAIN-AGGREGATED (Σ across lanes, same day) tdet/tdep for
    /// the captured EVENT day — the minor-1 EVENT totals, so the intake
    /// closure holds in its chain form `Σ S_h(exit) = Σ_lanes(tdet − tdep)`.
    hbp_event_chain_totals_kg: Option<(f64, f64)>,
    hbp_current_day_index: Option<i32>,
    hbp_current_day_tdet_kg: f64,
    hbp_current_day_tdep_kg: f64,
    parity_grade_row_seen: bool,
    area_by_ofe: BTreeMap<u32, f64>,
    sim_day_index_monotonic: bool,
    previous_sim_day_index: Option<i32>,
    upstream_carry_total_mm: f64,
}

#[derive(Clone)]
struct DirectPublicationStreamingTargets {
    wat: Option<PathBuf>,
    pass_parquet: Option<PathBuf>,
}

struct HillslopeManifestPublication<'a> {
    request: &'a HillslopeRunRequest,
    argv: &'a [String],
    inputs: &'a ParsedHillslopeRunInputs,
    targets: &'a HillslopeOutputTargets,
    sidecars: HillslopeSidecarManifestInputs<'a>,
    timestep_policy: HillslopeTimestepPolicyProvenance,
    adapter_boundary: HillslopeAdapterBoundaryProvenance,
    execution_provenance: HillslopeExecutionProvenance,
    wb13_publication: HillslopeWb13PublicationProvenance,
    mofe_hourly_carry: HillslopeMofeHourlyCarryProvenance,
    runtime_selection: HillslopeRuntimeSelectionProvenance,
    direct_runtime_counters: Option<HillslopeDirectRuntimeCounterProvenance>,
    coupling_vectors: HillslopeCouplingVectorProvenance,
}

struct HillslopeSidecarManifestInputs<'a> {
    discovery_mode: &'a str,
    resolved_sidecars: BTreeMap<String, String>,
    input_paths: &'a HillslopeSidecarInputPaths,
    mode_selection: HillslopeModeSelectionProvenance,
}

fn load_hillslope_run_inputs(
    request: &HillslopeRunRequest,
) -> Result<ParsedHillslopeRunInputs, HillslopeCliError> {
    let run_file_path =
        crate::hillslope::intake_lane_setup::resolve_run_file(&request.run_dir, &request.run_file);
    if !run_file_path.is_file() {
        return Err(HillslopeCliError::RunFileMissing {
            path: run_file_path,
        });
    }

    let runfile = crate::hillslope::intake_lane_setup::parse_runfile_execution_config(
        &run_file_path,
        request.legacy_sidecar_discovery,
    )?;
    let soil_path = runfile.soil_path.clone();
    let management_path = runfile.management_path.clone();
    let slope_path = runfile.slope_path.clone();
    let climate_path = runfile.climate_path.clone();
    let slope = parse_hillslope_slope_input(request, &slope_path)?;
    let management = parse_hillslope_management_input(request, &management_path)?;
    let soil = parse_hillslope_soil_input(request, &soil_path, &slope, &management)?;
    let climate = parse_hillslope_climate_input(request, &climate_path)?;

    Ok(ParsedHillslopeRunInputs {
        run_file_path,
        runfile,
        soil_path,
        management_path,
        slope_path,
        climate_path,
        slope,
        management,
        soil,
        climate,
    })
}

fn parse_hillslope_slope_input(
    request: &HillslopeRunRequest,
    slope_path: &Path,
) -> Result<SlopeProfile, HillslopeCliError> {
    parse_slope_file(
        slope_path,
        request.sidecar_policy.as_slope_parser_options(),
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "slope",
        detail: error.to_string(),
    })
}

fn parse_hillslope_management_input(
    request: &HillslopeRunRequest,
    management_path: &Path,
) -> Result<ManagementParseOutput, HillslopeCliError> {
    parse_management_document_from_path(
        management_path,
        request.sidecar_policy.as_management_parser_mode(),
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "management",
        detail: error.to_string(),
    })
}

fn parse_hillslope_soil_input(
    request: &HillslopeRunRequest,
    soil_path: &Path,
    slope: &SlopeProfile,
    management: &ManagementParseOutput,
) -> Result<SoilProfile, HillslopeCliError> {
    let soil_raw = fs::read_to_string(soil_path).map_err(|source| HillslopeCliError::Io {
        path: soil_path.to_path_buf(),
        source,
    })?;
    let expected_soil_topology_count = if slope.ofe_count == management.topology_count {
        Some(slope.ofe_count)
    } else {
        None
    };
    let soil = parse_soil(
        &soil_raw,
        SoilParserOptions {
            mode: request.sidecar_policy.as_soil_parser_mode(),
            allow_legacy_aliases: true,
            expected_topology_count: expected_soil_topology_count,
            topology_scope: expected_soil_topology_count.map(|_| TopologyScope::Hillslope),
        },
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "soil",
        detail: error.to_string(),
    })?;
    crate::hillslope::intake_lane_setup::validate_hillslope_ofe_topology_parity(
        slope.ofe_count,
        management.topology_count,
        soil.ntemp,
    )?;
    Ok(soil)
}

fn parse_hillslope_climate_input(
    request: &HillslopeRunRequest,
    climate_path: &Path,
) -> Result<ClimateFile, HillslopeCliError> {
    parse_climate_file(
        climate_path,
        request.sidecar_policy.as_climate_parser_mode(),
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "climate",
        detail: error.to_string(),
    })
}

fn resolve_hillslope_output_targets(
    runfile: &RunfileExecutionConfig,
) -> Result<HillslopeOutputTargets, HillslopeCliError> {
    let [output_pass, output_loss] = required_output_paths(&runfile.output_config);
    let optional_outputs = optional_output_paths(&runfile.output_config);
    let laned_active_trace = crate::hillslope::laned_active::trace_enabled().then(|| {
        output_pass.parent().map_or_else(
            || PathBuf::from("laned_active_trace.jsonl"),
            |parent| parent.join("laned_active_trace.jsonl"),
        )
    });
    let output_hillslope_id = parse_hillslope_id_from_output_pass_path(&output_pass)?;
    Ok(HillslopeOutputTargets {
        output_pass,
        output_loss,
        optional_outputs,
        laned_active_trace,
        output_hillslope_id,
    })
}

fn resolve_hillslope_sidecars(
    request: &HillslopeRunRequest,
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
) -> Result<HillslopeSidecarResolution, HillslopeCliError> {
    let soil_versions = vec![inputs.soil.datver.numeric(); inputs.soil.ofes.len().max(1)];
    if request.legacy_sidecar_discovery {
        resolve_legacy_hillslope_sidecars(request, inputs, targets, &soil_versions)
    } else {
        resolve_runfile_hillslope_sidecars(request, inputs, &soil_versions)
    }
}

fn resolve_legacy_hillslope_sidecars(
    request: &HillslopeRunRequest,
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    soil_versions: &[f64],
) -> Result<HillslopeSidecarResolution, HillslopeCliError> {
    let discovered_sidecars = crate::hillslope::intake_lane_setup::discover_sidecars(
        &request.run_dir,
        &legacy_sidecar_excluded_files(inputs, targets),
    )?;
    let sidecar_contracts = crate::hillslope::intake_lane_setup::hillslope_sidecar_contracts(true)?;
    let sidecar_response = adapt_sidecar_bindings(&SidecarAdapterRequest {
        policy: request.sidecar_policy.as_legacy_bridge_policy(),
        contracts: sidecar_contracts,
        discovered: discovered_sidecars,
    })
    .map_err(|source| HillslopeCliError::SidecarAdapter { source })?;

    let mut resolved_sidecars = resolved_sidecars_from_bindings(&sidecar_response.bindings);
    let mut sidecar_warnings = sidecar_response
        .warnings
        .iter()
        .map(|warning| format!("{} {}", warning.code.message_id(), warning.detail))
        .collect::<Vec<_>>();
    let mut input_paths = HillslopeSidecarInputPaths::default();
    let snow_path = legacy_sidecar_path(&sidecar_response.bindings, "snow", request, "snow.txt");
    let frost_path = legacy_sidecar_path(&sidecar_response.bindings, "frost", request, "frost.txt");
    let wepp_ui_path =
        legacy_sidecar_path(&sidecar_response.bindings, "wepp_ui", request, "wepp_ui.txt");
    let pmetpara_path =
        legacy_sidecar_path(&sidecar_response.bindings, "pmetpara", request, "pmetpara.txt");
    let gwcoeff_path =
        legacy_sidecar_path(&sidecar_response.bindings, "gwcoeff", request, "gwcoeff.txt");
    record_existing_legacy_sidecars(
        &mut resolved_sidecars,
        &mut input_paths,
        &snow_path,
        &frost_path,
        &wepp_ui_path,
        &pmetpara_path,
        &gwcoeff_path,
    );

    let snow = parse_legacy_snow_sidecar(request, &snow_path)?;
    let frost = parse_legacy_frost_sidecar(request, &frost_path)?;
    let wepp_ui_requested = wepp_ui_path.is_file();
    let wepp_ui = parse_wepp_ui_sidecar(request, &wepp_ui_path, wepp_ui_requested, soil_versions)?;
    sidecar_warnings.extend(wepp_ui_warnings(&wepp_ui));
    let pmetpara = parse_legacy_pmetpara_sidecar(request, &pmetpara_path)?;
    let gwcoeff = parse_gwcoeff_sidecar(request, &gwcoeff_path)?;
    sidecar_warnings.extend(gwcoeff_warnings(&gwcoeff));

    Ok(HillslopeSidecarResolution {
        snow,
        frost,
        gwcoeff,
        mode_selection: crate::hillslope::intake_lane_setup::build_mode_selection_provenance(
            &wepp_ui,
        )?,
        pmetpara,
        pmetpara_mode: request.sidecar_policy.as_pmetpara_parse_mode(),
        resolved_sidecars,
        sidecar_warnings,
        input_paths,
        discovery_mode: "legacy-sidecar-discovery",
    })
}

fn legacy_sidecar_excluded_files(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
) -> Vec<String> {
    let mut excluded_files = vec![
        file_name_string(&inputs.run_file_path),
        file_name_string(&inputs.soil_path),
        file_name_string(&inputs.management_path),
        file_name_string(&inputs.slope_path),
        file_name_string(&inputs.climate_path),
        "openwepp_hillslope_run_manifest.json".to_string(),
    ];
    excluded_files.extend(
        std::iter::once(file_name_string(&targets.output_pass))
            .chain(std::iter::once(file_name_string(&targets.output_loss)))
            .chain(
                targets
                    .optional_outputs
                    .iter()
                    .map(|path| file_name_string(path)),
            )
            .filter(|name| !name.is_empty()),
    );
    excluded_files
}

fn resolved_sidecars_from_bindings(bindings: &[SidecarBinding]) -> BTreeMap<String, String> {
    bindings
        .iter()
        .map(|binding| {
            (
                binding.sidecar_id.as_str().to_string(),
                binding.resolved_path.display().to_string(),
            )
        })
        .collect()
}

fn legacy_sidecar_path(
    bindings: &[SidecarBinding],
    sidecar_id: &'static str,
    request: &HillslopeRunRequest,
    fallback_name: &str,
) -> PathBuf {
    crate::hillslope::intake_lane_setup::optional_sidecar_binding_path(bindings, sidecar_id)
        .unwrap_or_else(|| request.run_dir.join(fallback_name))
}

fn record_existing_legacy_sidecars(
    resolved_sidecars: &mut BTreeMap<String, String>,
    input_paths: &mut HillslopeSidecarInputPaths,
    snow_path: &Path,
    frost_path: &Path,
    wepp_ui_path: &Path,
    pmetpara_path: &Path,
    gwcoeff_path: &Path,
) {
    record_existing_sidecar(resolved_sidecars, &mut input_paths.snow, "snow", snow_path);
    record_existing_sidecar(resolved_sidecars, &mut input_paths.frost, "frost", frost_path);
    record_existing_sidecar(
        resolved_sidecars,
        &mut input_paths.wepp_ui,
        "wepp_ui",
        wepp_ui_path,
    );
    record_existing_sidecar(
        resolved_sidecars,
        &mut input_paths.pmetpara,
        "pmetpara",
        pmetpara_path,
    );
    record_existing_sidecar(
        resolved_sidecars,
        &mut input_paths.gwcoeff,
        "gwcoeff",
        gwcoeff_path,
    );
}

fn record_existing_sidecar(
    resolved_sidecars: &mut BTreeMap<String, String>,
    input_path: &mut Option<PathBuf>,
    sidecar_id: &str,
    path: &Path,
) {
    if path.is_file() {
        *input_path = Some(path.to_path_buf());
        resolved_sidecars.insert(sidecar_id.to_string(), path.display().to_string());
    }
}

fn parse_legacy_snow_sidecar(
    request: &HillslopeRunRequest,
    snow_path: &Path,
) -> Result<SnowParseOutput, HillslopeCliError> {
    parse_snow_file(snow_path, request.sidecar_policy.as_snow_parse_options()).map_err(|error| {
        HillslopeCliError::ParseFailure {
            surface: "snow",
            detail: error.to_string(),
        }
    })
}

fn parse_legacy_frost_sidecar(
    request: &HillslopeRunRequest,
    frost_path: &Path,
) -> Result<FrostParseOutput, HillslopeCliError> {
    parse_frost_from_path(frost_path, request.sidecar_policy.as_frost_parse_mode()).map_err(
        |error| HillslopeCliError::ParseFailure {
            surface: "frost",
            detail: error.to_string(),
        },
    )
}

fn parse_wepp_ui_sidecar(
    request: &HillslopeRunRequest,
    wepp_ui_path: &Path,
    requested_hourly_seepage: bool,
    soil_versions: &[f64],
) -> Result<WeppUiParseResult, HillslopeCliError> {
    parse_wepp_ui_from_path(
        wepp_ui_path,
        WeppUiParserOptions {
            mode: request.sidecar_policy.as_wepp_ui_parse_mode(),
            requested_hourly_seepage,
            soil_versions: soil_versions.to_vec(),
        },
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "wepp_ui",
        detail: error.to_string(),
    })
}

fn wepp_ui_warnings(wepp_ui: &WeppUiParseResult) -> impl Iterator<Item = String> + '_ {
    wepp_ui
        .warnings
        .iter()
        .map(|warning| format!("{} {}", warning.code.as_str(), warning.message))
}

fn parse_legacy_pmetpara_sidecar(
    request: &HillslopeRunRequest,
    pmetpara_path: &Path,
) -> Result<PmetparaFile, HillslopeCliError> {
    parse_pmetpara_file(
        pmetpara_path,
        PmetparaParseOptions {
            mode: request.sidecar_policy.as_pmetpara_parse_mode(),
            require_sidecar: false,
        },
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "pmetpara",
        detail: error.to_string(),
    })
}

fn parse_gwcoeff_sidecar(
    request: &HillslopeRunRequest,
    gwcoeff_path: &Path,
) -> Result<GwcoeffFile, HillslopeCliError> {
    parse_gwcoeff_from_path(gwcoeff_path, request.sidecar_policy.as_gwcoeff_parse_options())
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "gwcoeff",
            detail: error.to_string(),
        })
}

fn gwcoeff_warnings(gwcoeff: &GwcoeffFile) -> impl Iterator<Item = String> + '_ {
    gwcoeff.warnings.iter().map(|warning| {
        format!(
            "{} {}",
            warning.code.as_str(),
            warning.message
        )
    })
}

fn resolve_runfile_hillslope_sidecars(
    request: &HillslopeRunRequest,
    inputs: &ParsedHillslopeRunInputs,
    soil_versions: &[f64],
) -> Result<HillslopeSidecarResolution, HillslopeCliError> {
    let sidecar_overrides = &inputs.runfile.sidecar_overrides;
    let mut resolved_sidecars = BTreeMap::new();
    let mut sidecar_warnings = Vec::new();
    let mut input_paths = HillslopeSidecarInputPaths::default();
    let snow = parse_runfile_snow_sidecar(
        request,
        sidecar_overrides,
        &mut resolved_sidecars,
        &mut input_paths.snow,
    )?;
    let frost = parse_runfile_frost_sidecar(
        request,
        sidecar_overrides,
        &mut resolved_sidecars,
        &mut input_paths.frost,
    )?;
    let wepp_ui_path = request.run_dir.join("wepp_ui.txt");
    record_existing_sidecar(
        &mut resolved_sidecars,
        &mut input_paths.wepp_ui,
        "wepp_ui",
        &wepp_ui_path,
    );
    let wepp_ui = parse_wepp_ui_sidecar(request, &wepp_ui_path, sidecar_overrides.wepp_ui, soil_versions)?;
    sidecar_warnings.extend(wepp_ui_warnings(&wepp_ui));
    let pmetpara = parse_runfile_pmetpara_sidecar(
        request,
        sidecar_overrides,
        &mut resolved_sidecars,
        &mut input_paths.pmetpara,
    )?;
    let default_gwcoeff_path = request.run_dir.join("gwcoeff.txt");
    record_existing_sidecar(
        &mut resolved_sidecars,
        &mut input_paths.gwcoeff,
        "gwcoeff",
        &default_gwcoeff_path,
    );
    let gwcoeff = parse_gwcoeff_sidecar(request, &default_gwcoeff_path)?;
    sidecar_warnings.extend(gwcoeff_warnings(&gwcoeff));

    Ok(HillslopeSidecarResolution {
        snow,
        frost,
        gwcoeff,
        mode_selection: crate::hillslope::intake_lane_setup::build_mode_selection_provenance(
            &wepp_ui,
        )?,
        pmetpara,
        pmetpara_mode: request.sidecar_policy.as_pmetpara_parse_mode(),
        resolved_sidecars,
        sidecar_warnings,
        input_paths,
        discovery_mode: "runfile-sidecar-overrides",
    })
}

fn parse_runfile_snow_sidecar(
    request: &HillslopeRunRequest,
    sidecar_overrides: &RunfileSidecarOverrides,
    resolved_sidecars: &mut BTreeMap<String, String>,
    input_path: &mut Option<PathBuf>,
) -> Result<SnowParseOutput, HillslopeCliError> {
    if let Some(snow_inline) = sidecar_overrides.snow {
        resolved_sidecars.insert("snow".to_string(), "<inline>".to_string());
        return parse_snow_from_str(
            &format!(
                "{}\n{}\n{}\n",
                snow_inline.rst, snow_inline.newsnw, snow_inline.ssd
            ),
            request.sidecar_policy.as_snow_parse_options(),
        )
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "snow",
            detail: error.to_string(),
        });
    }
    let default_snow_path = request.run_dir.join("snow.txt");
    if default_snow_path.is_file() {
        *input_path = Some(default_snow_path.clone());
        resolved_sidecars.insert("snow".to_string(), default_snow_path.display().to_string());
        return parse_snow_file(
            &default_snow_path,
            request.sidecar_policy.as_snow_parse_options(),
        )
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "snow",
            detail: error.to_string(),
        });
    }
    Ok(SnowParseOutput {
        sidecar_present: false,
        defaults_applied: true,
        rst: 0.0,
        newsnw: 100.0,
        ssd: 250.0,
        surplus_record_count: 0,
        trailing_token_lines: Vec::new(),
        prefix_variant_detected: false,
        warnings: Vec::new(),
    })
}

fn parse_runfile_frost_sidecar(
    request: &HillslopeRunRequest,
    sidecar_overrides: &RunfileSidecarOverrides,
    resolved_sidecars: &mut BTreeMap<String, String>,
    input_path: &mut Option<PathBuf>,
) -> Result<FrostParseOutput, HillslopeCliError> {
    if let Some(frost_inline) = sidecar_overrides.frost {
        resolved_sidecars.insert("frost".to_string(), "<inline>".to_string());
        return parse_frost_from_str(
            &format!(
                "{} {} {}\n{} {} {} {} {} {}\n",
                frost_inline.wint_red,
                frost_inline.fine_top,
                frost_inline.fine_bot,
                frost_inline.ksnowf,
                frost_inline.kresf,
                frost_inline.ksoilf,
                frost_inline.kfactor1,
                frost_inline.kfactor2,
                frost_inline.kfactor3
            ),
            request.sidecar_policy.as_frost_parse_mode(),
        )
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "frost",
            detail: error.to_string(),
        });
    }
    let default_frost_path = request.run_dir.join("frost.txt");
    if default_frost_path.is_file() {
        *input_path = Some(default_frost_path.clone());
        resolved_sidecars.insert(
            "frost".to_string(),
            default_frost_path.display().to_string(),
        );
        return parse_frost_from_path(
            &default_frost_path,
            request.sidecar_policy.as_frost_parse_mode(),
        )
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "frost",
            detail: error.to_string(),
        });
    }
    Ok(FrostParseOutput::defaults_for_missing_file(
        request.sidecar_policy.as_frost_parse_mode(),
    ))
}

fn parse_runfile_pmetpara_sidecar(
    request: &HillslopeRunRequest,
    sidecar_overrides: &RunfileSidecarOverrides,
    resolved_sidecars: &mut BTreeMap<String, String>,
    input_path: &mut Option<PathBuf>,
) -> Result<PmetparaFile, HillslopeCliError> {
    let default_pmetpara_path = request.run_dir.join("pmetpara.txt");
    let pmetpara_path = sidecar_overrides.pmetpara_path.clone().or_else(|| {
        default_pmetpara_path
            .is_file()
            .then_some(default_pmetpara_path)
    });
    if let Some(pmetpara_path) = pmetpara_path.as_ref() {
        *input_path = Some(pmetpara_path.to_owned());
        resolved_sidecars.insert("pmetpara".to_string(), pmetpara_path.display().to_string());
        return parse_pmetpara_file(
            pmetpara_path,
            PmetparaParseOptions {
                mode: request.sidecar_policy.as_pmetpara_parse_mode(),
                require_sidecar: true,
            },
        )
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "pmetpara",
            detail: error.to_string(),
        });
    }
    Ok(crate::hillslope::intake_lane_setup::absent_pmetpara_file())
}

fn build_static_hillslope_runtime_setup(
    _request: &HillslopeRunRequest,
    inputs: &ParsedHillslopeRunInputs,
    sidecars: &HillslopeSidecarResolution,
    runtime_selection: HillslopeRuntimeSelection,
) -> Result<StaticHillslopeRuntimeSetup, HillslopeCliError> {
    debug_assert_eq!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectProductionExecutor
    );
    let _publication_area_m2 = derive_mofe04_publication_area_from_slope(&inputs.slope)?;
    let static_per_ofe_slices =
        crate::hillslope::intake_lane_setup::build_static_per_ofe_lane_slices(
            &inputs.slope,
            &inputs.soil,
            inputs.management.topology_count,
        )?;
    let per_ofe_lane_areas_m2 = static_per_ofe_slices
        .iter()
        .map(|slice| slice.area_m2)
        .collect::<Vec<_>>();
    let per_ofe_runoff_publication_geometries =
        build_per_ofe_runoff_publication_geometries(&static_per_ofe_slices)?;
    let lane_context =
        crate::hillslope::intake_lane_setup::build_execution_lane_context(&sidecars.mode_selection)?;
    let timestep_policy =
        crate::hillslope::intake_lane_setup::build_timestep_policy_provenance(&lane_context);
    let adapter_boundary =
        crate::hillslope::intake_lane_setup::build_adapter_boundary_provenance(&lane_context)?;
    let climate_span = build_climate_run_span_summary(&inputs.climate)?;
    let execution_state = HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    };

    Ok(StaticHillslopeRuntimeSetup {
        timestep_policy,
        adapter_boundary,
        execution_state,
    })
}

fn build_per_ofe_runoff_publication_geometries(
    static_per_ofe_slices: &[StaticOfeLaneSlice],
) -> Result<Vec<Wb13RunoffPublicationGeometry>, HillslopeCliError> {
    let mut cumulative_runoff_length_m = 0.0;
    static_per_ofe_slices
        .iter()
        .map(|slice| {
            cumulative_runoff_length_m += slice.length_m;
            Wb13RunoffPublicationGeometry::new(slice.length_m, cumulative_runoff_length_m)
        })
        .collect()
}
