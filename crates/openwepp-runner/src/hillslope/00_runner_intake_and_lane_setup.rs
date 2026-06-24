use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::hillslope::intake_lane_setup::{
    legacy_sunmap_horizontal_radpot_ly, saturation_vapor_pressure_kpa,
};
use openwepp_hillslope_orchestrator::runtime_inputs::{
    DirectWinterHourlyContext, HillslopeClimateRuntimeRequest, HillslopeDirectClimateDayForcing,
    SlopeRuntimeSurfaceOptions,
    build_hillslope_climate_runtime_request,
    build_hillslope_runtime_surface_from_climate_request_with_context,
    build_hillslope_runtime_surface_from_frost, build_hillslope_runtime_surface_from_management,
    build_hillslope_runtime_surface_from_slope_with_options,
    build_hillslope_runtime_surface_from_snow, build_hillslope_runtime_surface_from_soil,
};
use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectCanopyInterceptionInputs, DirectErod13Inputs,
    DirectErod14ClassInputs,
    DirectErod14Inputs, DirectErosionInputs, DirectEvapotranspirationComputeInputs,
    DirectEvapotranspirationPmetInputs, DirectEvapotranspirationStageState,
    DirectExecutionReport, DirectExecutorMode, DirectFrameExecutor, DirectFrostFineLayerCarry,
    DirectFrostLayerCarryProjection, DirectFrostLayerShadowCarry, DirectFrostRuntimeCarry,
    DirectFrostRunoffSurface, DirectHydrologyProjectionInputs,
    DirectInfiltrationDepressionInputs, DirectLaneConstructorInputs, DirectLiquidInputInputs,
    DirectPeakRunoffInputs, DirectPercolationInputs, DirectPublicationCalendarDay,
    DirectPublicationClimateOperands, DirectPublicationDayInput, DirectPublicationDayRow,
    DirectPublicationErosionOperands, DirectPublicationEvaporationOperands,
    DirectPublicationExecution, DirectPublicationInterceptionOperands,
    DirectPublicationLiquidInputOperands, DirectPublicationProfileOperands,
    DirectPublicationRunMetadata, DirectPublicationRunoffOperands,
    DirectPublicationStorageOperands, DirectPublicationSubsurfaceOperands,
    DirectPublicationTransferOperands, DirectRunConstructorInputs, DirectRunFrame,
    DirectRunIdentity, DirectRunPublicationFrame, DirectRuntimeAuditSnapshot, DirectRuntimeError,
    DirectSnowCouplingInputs, DirectSnowHourlyForcing, DirectSnowLaneState,
    DirectStorageInputInputs, DirectSubsurfaceComputeInputs, DirectSubsurfaceLayerInputs,
    DirectSubsurfaceLayerState, DirectWb14HyetographInterval,
    DirectWb14InfiltrationProducerInputs, HillslopeDayFrame,
    HillslopePhaseScheduler, HillslopeWritebackSurface, OfeLaneExecutionInput,
    OfeLanePersistentState, OfeLanePersistentStateSequence, OfeLaneSequenceExecutionReport,
    SchedulerOutcomeClass, TransferInput, TransferOutput, Wb11HydrologyKernel,
    build_hillslope_hot_symbol_tables, compute_direct_canopy_interception, direct_runtime_audit_snapshot,
    record_direct_runtime_compatibility_edge_invocation,
};
use openwepp_hillslope_output::contracts::{HillslopeOutputConfig, validate_output_contract};
use openwepp_hillslope_output::hillslope_pass::{
    HillslopePassRow, write_hillslope_pass_parquet,
};
use openwepp_hillslope_output::hillslope_wat::{
    HillslopeWatRow, InterchangeVersion, write_hillslope_wat_parquet,
};
use openwepp_hillslope_output::manifest::{OutputChecksumEntry, assemble_output_checksums};
use openwepp_hillslope_output::writers::{optional_output_paths, required_output_paths};
use openwepp_input_contract::parsers::climate::{
    ClimateDailyRecord, ClimateFile, parse_climate_file,
};
use openwepp_input_contract::parsers::frost::{
    FrostParseOutput, parse_frost_from_path, parse_frost_from_str,
};
use openwepp_input_contract::parsers::management::{
    ManagementParseOutput, YearlyScenarioData, parse_management_from_path,
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
    BoundarySymbol, BoundaryValue, HillslopeKernel, HillslopeKernelRequest, KernelRunResponse,
    HotSymbolTables, IndexedWritebackSurface, KernelWritebackPayload, SymbolRegistry,
};
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterRequest, SidecarBinding, SidecarContract, SidecarDiscovery, SidecarId,
    SidecarRequirement, adapt_sidecar_bindings,
};
use openwepp_summary_accumulator::{
    SummaryScalarSurface, WB13_PER_OFE_PUBLICATION_POLICY_SYMBOL, Wb13DailyWaterBalanceRow,
};
use openwepp_topology::{TopologyGraph, validate_pre_execution_topology};
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
    SCHEDULER_KERNEL_PUBLICATION_SOURCE,
    SIMCONS_INTAKE_GUARD_ID, SIMCOUP_GUARD_ID, SIMIMPL09_ADOPT_PROFILE,
    SIMIMPL10_FLAG_TOLERANCE, SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
    SIMMODE_TIMESTEP_GUARD_ID, SIMOUT_GUARD_ID, SIMPIPE_GUARD_ID, SUBHOURLY_EXECUTION_LANE,
    WB13_PUBLICATION_SOURCE_DIRECT_PUBLICATION_FRAME, WB13_PUBLICATION_SOURCE_SIMULATION_OWNED,
    WB13_REPLAY_CANDIDATE_SURFACE_PASS, WB13_REPLAY_CANDIDATE_SURFACE_WAT, WUI_MODE_GUARD_ID,
};
use crate::errors::HillslopeCliError;
use crate::release::write_release_sidecar_for_binary;
use crate::role::BinaryRole;
use crate::shared::{
    file_name_string, git_source_commit_or_unknown, path_has_extension_case_insensitive,
    sha256_file_hex, utc_now_rfc3339,
};

const EROD14_QIN_POLICY_WAVE2_DISABLED: &str = "wave2-disabled";
const EROD14_QIN_POLICY_WATER_TRANSFER_ONLY: &str =
    "water-transfer-only-mofe01-mg-sediment-coupling-follow-on";
const EROD14_QIN_WARNING_ID: &str = "MOFE01-MG-W-001";

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
    phase_span_runs: u64,
    direct_phase_entries: u64,
    direct_compute_operations: u64,
    direct_state_mutations: u64,
    downstream_operand_productions: u64,
    shadow_projections: u64,
    compatibility_edge_invocations: u64,
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
    erod14_wave2_kernel_status_seen: bool,
    erod14_qin_source_policy: String,
    erod14_qin_sediment_coupled: bool,
    wb16_ealpha_compatibility_seed_used: bool,
    wb16_ealpha_seed_policy: String,
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

#[derive(Debug, Clone)]
struct SimulationOwnedWb13Row {
    wb13_row: Wb13DailyWaterBalanceRow,
    interception_mm: f64,
    frdp_mm: f64,
    month: i8,
    day_of_month: i8,
    water_year: i16,
    sim_day_index: i32,
}

#[derive(Debug)]
struct DailyExecutionResult {
    scheduler_outcome_class: SchedulerOutcomeClass,
    scheduler_status_message_id: String,
    coupling_vectors: HillslopeCouplingVectorProvenance,
    wb13_row: SimulationOwnedWb13Row,
    runtime_surface: HillslopeWritebackSurface,
    kernel_phase_message_ids: Vec<String>,
    erod14_wave2_kernel_status_seen: bool,
    hphys0245_trace_rows: Vec<Hphys0245TraceRow>,
}

#[derive(Debug)]
struct PersistentDailyExecutionResult {
    scheduler_outcome_class: SchedulerOutcomeClass,
    scheduler_status_message_id: String,
    coupling_vectors: HillslopeCouplingVectorProvenance,
    runtime_surface: HillslopeWritebackSurface,
    internal_wb13_collection: DailyInternalPerOfeWb13Collection,
    kernel_phase_message_ids: Vec<String>,
    erod14_wave2_kernel_status_seen: bool,
    hphys0245_trace_rows: Vec<Hphys0245TraceRow>,
}

#[derive(Clone, Copy)]
struct SchedulerLifecycleContext<'a> {
    run_name: &'a str,
    execution_lane: ExecutionLane,
    publication_area_m2: f64,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_day: &'a ClimateDayProjection,
    runtime_swe_before_m: f64,
    hphys0245_trace_config: Option<&'a Hphys0245TraceConfig>,
    symbol_registry: Option<&'a SymbolRegistry>,
    hot_symbol_tables: Option<&'a HotSymbolTables>,
    indexed_scheduler_runtime_enabled: bool,
}

#[derive(Debug, Clone)]
struct Hphys0245TraceConfig {
    path: PathBuf,
    max_days: Option<usize>,
}

impl Hphys0245TraceConfig {
    #[must_use]
    fn includes_day(&self, sim_day_index: usize) -> bool {
        self.max_days
            .is_none_or(|max_days| sim_day_index <= max_days)
    }
}

#[derive(Debug, Clone, Copy)]
struct Hphys0245SnowRuntimeBeforeState {
    swe_m: Option<f64>,
    depth_m: Option<f64>,
    density_kg_m3: Option<f64>,
    settle_day_count: Option<f64>,
}

impl Hphys0245SnowRuntimeBeforeState {
    fn from_surface(runtime_surface: &HillslopeWritebackSurface, swe_m: f64) -> Self {
        Self {
            swe_m: Some(swe_m),
            depth_m: runtime_surface_symbol_value(runtime_surface, "snow.runtime_depth_m"),
            density_kg_m3: runtime_surface_symbol_value(
                runtime_surface,
                "snow.runtime_density_kg_m3",
            ),
            settle_day_count: runtime_surface_symbol_value(
                runtime_surface,
                "snow.runtime_settle_day_count",
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct Hphys0245TraceRow {
    schema: &'static str,
    run_name: String,
    sim_day_index: usize,
    simulation_year: i32,
    calendar_year: i32,
    julian_day: u16,
    boundary: String,
    phase: Option<String>,
    wb11_soil_water_m: Option<f64>,
    wb11_soil_water_mm: Option<f64>,
    wb12_infiltration_m: Option<f64>,
    wb12_rainfall_input_m: Option<f64>,
    wb12_runon_input_m: Option<f64>,
    wb12_depression_storage_delta_m: Option<f64>,
    wb12_partition_liquid_supply_m: Option<f64>,
    wb12_partition_residual_before_q_m: Option<f64>,
    wb14_soil_conductivity_m_s: Option<f64>,
    wb14_frost_infcap_m_s: Option<f64>,
    wb14_effective_conductivity_m_s: Option<f64>,
    wb14_soil_layer_depth_m: Option<f64>,
    wb14_theta_residual: Option<f64>,
    wb14_theta_field_capacity: Option<f64>,
    wb14_matric_potential_m: Option<f64>,
    wb18_theta_sum_m: Option<f64>,
    wb18_theta_layers_m: BTreeMap<String, f64>,
    wb18_thetdr_layers: BTreeMap<String, f64>,
    wb18_dg_layers_m: BTreeMap<String, f64>,
    wb18_fc_layers_m: BTreeMap<String, f64>,
    wb19_coca_layers: BTreeMap<String, f64>,
    wb19_frzw_layers_m: BTreeMap<String, f64>,
    wb19_drfc_layers_m: BTreeMap<String, f64>,
    wb19_fzdrfc_layers_m: BTreeMap<String, f64>,
    wb18_frozen_depth_layers_m: BTreeMap<String, f64>,
    wb18_recomputed_soil_water_m: Option<f64>,
    wb18_recomputed_minus_wb11_m: Option<f64>,
    wb18_pei_sum_m: Option<f64>,
    wb18_pei_layers_m: BTreeMap<String, f64>,
    d_m: Option<f64>,
    pe_m: Option<f64>,
    wb13_dp_mm: Option<f64>,
    wb13_total_soil_mm: Option<f64>,
    wb13_soil_water_total_mm: Option<f64>,
    snow_runtime_swe_m: Option<f64>,
    snow_runtime_depth_m: Option<f64>,
    snow_runtime_density_kg_m3: Option<f64>,
    snow_runtime_settle_day_count: Option<f64>,
    snow_runtime_swe_before_m: Option<f64>,
    snow_runtime_depth_before_m: Option<f64>,
    snow_runtime_density_before_kg_m3: Option<f64>,
    snow_runtime_settle_day_count_before: Option<f64>,
    snow_runtime_swe_delta_m: Option<f64>,
    snow_runtime_depth_delta_m: Option<f64>,
    snow_runtime_density_delta_kg_m3: Option<f64>,
    snow_runtime_settle_day_count_delta: Option<f64>,
    snow_s_m: Option<f64>,
    snow_routed_melt_m: Option<f64>,
    snow_post_winter_rain_m: Option<f64>,
    snow_hourly_rain_sum_m: Option<f64>,
    snow_hourly_rain_retained_sum_m: Option<f64>,
    snow_hourly_rain_released_sum_m: Option<f64>,
    snow_hourly_snowfall_depth_sum_m: Option<f64>,
    snow_hourly_snowfall_water_equiv_sum_m: Option<f64>,
    snow_hourly_melt_raw_sum_m: Option<f64>,
    snow_hourly_melt_sum_m: Option<f64>,
    snow_hourly_rain_m: BTreeMap<String, f64>,
    snow_hourly_snowfall_depth_m: BTreeMap<String, f64>,
    snow_hourly_stmtim_rain_m: BTreeMap<String, f64>,
    snow_hourly_stmtim_stmdur_s: BTreeMap<String, f64>,
    snow_hourly_stmtim_wntdur_h: BTreeMap<String, f64>,
    snow_hourly_stmtim_wnttim_h: BTreeMap<String, f64>,
    snow_hourly_stmtim_hrtemp_c: BTreeMap<String, f64>,
    snow_hourly_stmtim_rst_c: BTreeMap<String, f64>,
    snow_hourly_stmtim_hrrain_m: BTreeMap<String, f64>,
    snow_hourly_stmtim_hrsnow_m: BTreeMap<String, f64>,
    snow_hourly_stmtim_active_interval: BTreeMap<String, f64>,
    snow_hourly_stmtim_rain_branch: BTreeMap<String, f64>,
    snow_hourly_stmtim_snow_branch: BTreeMap<String, f64>,
    snow_hourly_depth_before_m: BTreeMap<String, f64>,
    snow_hourly_depth_available_m: BTreeMap<String, f64>,
    snow_hourly_depth_after_m: BTreeMap<String, f64>,
    snow_hourly_density_before_kg_m3: BTreeMap<String, f64>,
    snow_hourly_density_after_kg_m3: BTreeMap<String, f64>,
    snow_hourly_melt_raw_m: BTreeMap<String, f64>,
    snow_hourly_melt_m: BTreeMap<String, f64>,
    snow_hourly_melt_amelt_in: BTreeMap<String, f64>,
    snow_hourly_melt_bmelt_in: BTreeMap<String, f64>,
    snow_hourly_melt_cmelt_in: BTreeMap<String, f64>,
    snow_hourly_melt_dmelt_in: BTreeMap<String, f64>,
    snow_hourly_melt_hrtef_f: BTreeMap<String, f64>,
    snow_hourly_melt_hrdtf_f: BTreeMap<String, f64>,
    snow_hourly_melt_vwmph: BTreeMap<String, f64>,
    snow_hourly_melt_rainin: BTreeMap<String, f64>,
    snow_hourly_melt_wind_adjustment: BTreeMap<String, f64>,
    snow_hourly_melt_branch_active: BTreeMap<String, f64>,
    winter_hourly_air_temp_c: BTreeMap<String, f64>,
    winter_hourly_rad_mj_m2: BTreeMap<String, f64>,
    winter_hourly_cloud_fraction: BTreeMap<String, f64>,
    winter_hourly_dewpoint_c: BTreeMap<String, f64>,
    winter_hourly_wind_m_s: BTreeMap<String, f64>,
    snow_runtime_swe_closure_error_m: Option<f64>,
    wb13_p_mm: Option<f64>,
    wb13_rm_mm: Option<f64>,
    wb13_q_mm: Option<f64>,
    wb13_snow_water_mm: Option<f64>,
    wb11_minus_theta_sum_m: Option<f64>,
    pl_sumgdd: Option<f64>,
    pl_vdmt: Option<f64>,
    pl_cancov: Option<f64>,
    pl_lai: Option<f64>,
    pl_rtmass: Option<f64>,
    pl_rtd: Option<f64>,
    pl_hia: Option<f64>,
    pl_pltol: Option<f64>,
    pl_swu_effective_pltol: Option<f64>,
    pmet_sidecar_present: Option<f64>,
    pmet_iflget: Option<f64>,
    pmet_selected_kcb: Option<f64>,
    pmet_selected_rawp: Option<f64>,
    pmet_selected_line_index: Option<f64>,
    pmet_lookup_fallback_first_row_used: Option<f64>,
    wb11_et_demand_m: Option<f64>,
    wb11_et_seed_branch: Option<String>,
    pmet_etorc_mm: Option<f64>,
    pmet_rn_mj_m2: Option<f64>,
    pmet_fwv_m_s: Option<f64>,
    pmet_rhd_pct: Option<f64>,
    pmet_kcbadj: Option<f64>,
    pmet_kcbcon: Option<f64>,
    pmet_etke: Option<f64>,
    pmet_etkr: Option<f64>,
    pmet_etks: Option<f64>,
    pmet_tew_mm: Option<f64>,
    pmet_rew_mm: Option<f64>,
    pmet_wfevp_mm: Option<f64>,
    pmet_taw_mm: Option<f64>,
    pmet_raw_mm: Option<f64>,
    pmet_wftrp_mm: Option<f64>,
    pmet_es_m: Option<f64>,
    pmet_ep_m: Option<f64>,
    etp_m: Option<f64>,
    upi_m: Option<f64>,
    ui_m: Option<f64>,
    wb18_ul_layers_m: BTreeMap<String, f64>,
    wb17_swu_stress_threshold_layers_m: BTreeMap<String, f64>,
    wb17_swu_storage_to_threshold_layers: BTreeMap<String, f64>,
    wb17_upi_layers_m: BTreeMap<String, f64>,
    wb17_ui_layers_m: BTreeMap<String, f64>,
    ep_m: Option<f64>,
    ws: Option<f64>,
    wb19_q_lateral_potential_m: Option<f64>,
    wb19_q_lateral_target_m: Option<f64>,
    wb19_lateral_capacity_tdv_m: Option<f64>,
    wb19_tdvv_m: Option<f64>,
    wb19_q_lateral_unrealized_m: Option<f64>,
    wb19_lateral_withdrawal_layers_m: BTreeMap<String, f64>,
    wb19_lateral_capacity_active_count_layers: BTreeMap<String, f64>,
    wb19_lateral_conductivity_active_count_layers: BTreeMap<String, f64>,
    q_m: Option<f64>,
    qdd_m: Option<f64>,
    qd_m: Option<f64>,
}

struct Hphys0245TelemetryKernel<'a> {
    inner: Wb11HydrologyKernel,
    run_name: &'a str,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_year: i32,
    julian_day: u16,
    snow_runtime_before: Option<Hphys0245SnowRuntimeBeforeState>,
    rows: Vec<Hphys0245TraceRow>,
}

impl<'a> Hphys0245TelemetryKernel<'a> {
    fn new(
        run_name: &'a str,
        simulation_year: i32,
        sim_day_index: usize,
        calendar_year: i32,
        julian_day: u16,
        snow_runtime_before: Option<Hphys0245SnowRuntimeBeforeState>,
    ) -> Self {
        Self {
            inner: Wb11HydrologyKernel,
            run_name,
            simulation_year,
            sim_day_index,
            calendar_year,
            julian_day,
            snow_runtime_before,
            rows: Vec::new(),
        }
    }

    fn into_rows(self) -> Vec<Hphys0245TraceRow> {
        self.rows
    }
}

impl HillslopeKernel for Hphys0245TelemetryKernel<'_> {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        let response = self.inner.run_hillslope_phase(request);
        let post_phase_surface = hphys0245_surface_after_writeback(request, &response.writeback);
        self.rows.push(build_hphys0245_trace_row(
            self.run_name,
            self.simulation_year,
            self.sim_day_index,
            self.calendar_year,
            self.julian_day,
            "post_phase",
            Some(request.phase_name),
            &post_phase_surface,
            None,
            self.snow_runtime_before,
        ));
        response
    }
}

const WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL: &str = "wb16_ealpha_compatibility_seed_used";
const WB16_EALPHA_SEED_POLICY_RUNTIME_PROVIDED: &str = "runtime_provided";
const WB16_EALPHA_SEED_POLICY_COMPATIBILITY: &str = "compatibility_seed_1p0";
const WB16_EALPHA_SEED_WARNING_ID: &str = "SIMPIPE-W-003";
const HPHYS0245_TRACE_SCHEMA: &str =
    "openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v17";
const HPHYS0245_TRACE_PATH_ENV: &str = "OPENWEPP_HPHYS0245_TRACE_PATH";
const HPHYS0245_TRACE_MAX_DAYS_ENV: &str = "OPENWEPP_HPHYS0245_TRACE_MAX_DAYS";
const MOFE_HOURLY_CARRY_POLICY: &str = "baseline-wathour-24-slot-copy-forward";
const MOFE_HOURLY_CARRY_ARRAY_COUNT: usize = 24;
const MOFE_HOURLY_CARRY_ARRAYS_ENABLED_SYMBOL: &str = "mofe_hourly_carry_arrays_enabled";
const MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL: &str = "mofe_hourly_upstream_area_ratio";
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
    output_hillslope_id: u32,
}

#[derive(Default)]
struct HillslopeSidecarInputPaths {
    snow: Option<PathBuf>,
    frost: Option<PathBuf>,
    wepp_ui: Option<PathBuf>,
    pmetpara: Option<PathBuf>,
}

struct HillslopeSidecarResolution {
    snow: SnowParseOutput,
    frost: FrostParseOutput,
    mode_selection: HillslopeModeSelectionProvenance,
    pmetpara: PmetparaFile,
    resolved_sidecars: BTreeMap<String, String>,
    sidecar_warnings: Vec<String>,
    input_paths: HillslopeSidecarInputPaths,
    discovery_mode: &'static str,
}

struct StaticRuntimeSurfaceParts {
    runtime_surface: HillslopeWritebackSurface,
    snow_surface: HillslopeWritebackSurface,
    frost_surface: HillslopeWritebackSurface,
    pmetpara_template: PmetparaFile,
}

struct StaticHillslopeRuntimeSetup {
    timestep_policy: HillslopeTimestepPolicyProvenance,
    adapter_boundary: HillslopeAdapterBoundaryProvenance,
    execution_state: HillslopeClimateExecutionState,
}

struct HillslopeClimateExecutionState {
    publication_area_m2: f64,
    contributor_ofe_count: usize,
    static_per_ofe_slice_count: usize,
    per_ofe_lane_areas_m2: Vec<f64>,
    per_ofe_runoff_publication_geometries: Vec<Wb13RunoffPublicationGeometry>,
    runtime_surface: HillslopeWritebackSurface,
    lane_context: ExecutionLaneContext,
    climate_span: ClimateRunSpanSummary,
    persistent_lane_state: Option<OfeLanePersistentStateSequence>,
    symbol_registry: Option<SymbolRegistry>,
    hot_symbol_tables: Option<HotSymbolTables>,
}

struct HillslopeClimateExecution {
    selected_lane: ExecutionLane,
    publication_area_m2: f64,
    contributor_ofe_count: usize,
    static_per_ofe_slice_count: usize,
    persistent_lane_active: bool,
    runtime_surface: HillslopeWritebackSurface,
    climate_span: ClimateRunSpanSummary,
    wb13_rows: Vec<SimulationOwnedWb13Row>,
    pass_rows: Vec<HillslopePassRow>,
    coupling_vectors: HillslopeCouplingVectorProvenance,
    erod14_wave2_kernel_status_seen: bool,
    scheduler_outcome_class: SchedulerOutcomeClass,
    scheduler_status_message_id: String,
    kernel_phase_message_ids: Vec<String>,
    hphys0245_trace_config: Option<Hphys0245TraceConfig>,
    hphys0245_trace_rows: Vec<Hphys0245TraceRow>,
    per_ofe_internal_wb13_summary: PerOfeInternalWb13RunSummary,
    executed_day_count: usize,
    retained_direct_publication: Option<DirectPublicationExecution>,
    direct_publication: Option<DirectPublicationArtifacts>,
}

struct DirectPublicationArtifacts {
    execution: DirectPublicationExecution,
    hbp_bytes: Vec<u8>,
    wat_rows: Vec<HillslopeWatRow>,
    pass_projection_rows: Vec<HillslopePassRow>,
    loss_text: String,
    manifest_text: String,
}

#[derive(Clone, Copy)]
struct PersistentLaneStateInputs<'a> {
    static_per_ofe_slices: &'a [StaticOfeLaneSlice],
    slope: &'a SlopeProfile,
    soil: &'a SoilProfile,
    management: &'a ManagementParseOutput,
    snow_surface: &'a HillslopeWritebackSurface,
    frost_surface: &'a HillslopeWritebackSurface,
    pmetpara_template: &'a PmetparaFile,
    pmetpara_parse_mode: PmetparaParseMode,
}

#[derive(Clone, Copy)]
struct ClimateExecutionContext<'a> {
    run_name: &'a str,
    output_hillslope_id: u32,
    lane: ExecutionLane,
    publication_area_m2: f64,
    first_calendar_year: i32,
    hphys0245_trace_config: Option<&'a Hphys0245TraceConfig>,
    symbol_registry: Option<&'a SymbolRegistry>,
    hot_symbol_tables: Option<&'a HotSymbolTables>,
    indexed_scheduler_runtime_enabled: bool,
}

struct HillslopeDayApply<'a> {
    persistent_lane_state: &'a mut Option<OfeLanePersistentStateSequence>,
    climate_surface: HillslopeWritebackSurface,
    stale_climate_symbols: &'a [BoundarySymbol],
    per_ofe_lane_areas_m2: &'a [f64],
    per_ofe_runoff_publication_geometries: &'a [Wb13RunoffPublicationGeometry],
    context: ClimateExecutionContext<'a>,
    day_index: usize,
    day_projection: &'a ClimateDayProjection,
    simulation_year: i32,
    runtime_swe_before_m: f64,
}

struct ClimateExecutionAccumulator {
    runtime_surface: HillslopeWritebackSurface,
    runtime_swe_publication_state_m: f64,
    wb13_rows: Vec<SimulationOwnedWb13Row>,
    pass_rows: Vec<HillslopePassRow>,
    coupling_vectors: Option<HillslopeCouplingVectorProvenance>,
    erod14_wave2_kernel_status_seen: bool,
    scheduler_outcome_class: SchedulerOutcomeClass,
    scheduler_status_message_id: String,
    previous_climate_symbols: Vec<BoundarySymbol>,
    kernel_phase_message_ids: std::collections::BTreeSet<String>,
    hphys0245_trace_rows: Vec<Hphys0245TraceRow>,
    per_ofe_internal_wb13_summary: PerOfeInternalWb13RunSummary,
    retained_direct_publication: Option<DirectPublicationExecution>,
}

struct ClimateExecutionCompletion {
    selected_lane: ExecutionLane,
    publication_area_m2: f64,
    contributor_ofe_count: usize,
    static_per_ofe_slice_count: usize,
    persistent_lane_active: bool,
    climate_span: ClimateRunSpanSummary,
    hphys0245_trace_config: Option<Hphys0245TraceConfig>,
    executed_day_count: usize,
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
    parse_management_from_path(
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
    let output_hillslope_id = parse_hillslope_id_from_output_pass_path(&output_pass)?;
    Ok(HillslopeOutputTargets {
        output_pass,
        output_loss,
        optional_outputs,
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
    record_existing_legacy_sidecars(
        &mut resolved_sidecars,
        &mut input_paths,
        &snow_path,
        &frost_path,
        &wepp_ui_path,
        &pmetpara_path,
    );

    let snow = parse_legacy_snow_sidecar(request, &snow_path)?;
    let frost = parse_legacy_frost_sidecar(request, &frost_path)?;
    let wepp_ui_requested = wepp_ui_path.is_file();
    let wepp_ui = parse_wepp_ui_sidecar(request, &wepp_ui_path, wepp_ui_requested, soil_versions)?;
    sidecar_warnings.extend(wepp_ui_warnings(&wepp_ui));
    let pmetpara = parse_legacy_pmetpara_sidecar(request, &pmetpara_path)?;

    Ok(HillslopeSidecarResolution {
        snow,
        frost,
        mode_selection: crate::hillslope::intake_lane_setup::build_mode_selection_provenance(
            &wepp_ui,
        )?,
        pmetpara,
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

    Ok(HillslopeSidecarResolution {
        snow,
        frost,
        mode_selection: crate::hillslope::intake_lane_setup::build_mode_selection_provenance(
            &wepp_ui,
        )?,
        pmetpara,
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
    request: &HillslopeRunRequest,
    inputs: &ParsedHillslopeRunInputs,
    sidecars: &mut HillslopeSidecarResolution,
) -> Result<StaticHillslopeRuntimeSetup, HillslopeCliError> {
    let publication_area_m2 = derive_mofe04_publication_area_from_slope(&inputs.slope)?;
    let contributor_ofe_count = inputs.slope.ofe_count;
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
    let runtime_parts = build_static_runtime_surface_parts(request, inputs, sidecars)?;
    let persistent_lane_state = build_persistent_lane_state(PersistentLaneStateInputs {
        static_per_ofe_slices: &static_per_ofe_slices,
        slope: &inputs.slope,
        soil: &inputs.soil,
        management: &inputs.management,
        snow_surface: &runtime_parts.snow_surface,
        frost_surface: &runtime_parts.frost_surface,
        pmetpara_template: &runtime_parts.pmetpara_template,
        pmetpara_parse_mode: request.sidecar_policy.as_pmetpara_parse_mode(),
    })?;
    let lane_context =
        crate::hillslope::intake_lane_setup::build_execution_lane_context(&sidecars.mode_selection)?;
    let timestep_policy =
        crate::hillslope::intake_lane_setup::build_timestep_policy_provenance(&lane_context);
    let adapter_boundary =
        crate::hillslope::intake_lane_setup::build_adapter_boundary_provenance(&lane_context)?;
    let climate_span = build_climate_run_span_summary(&inputs.climate)?;
    let mut execution_state = HillslopeClimateExecutionState {
        publication_area_m2,
        contributor_ofe_count,
        static_per_ofe_slice_count: static_per_ofe_slices.len(),
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        runtime_surface: runtime_parts.runtime_surface,
        lane_context,
        climate_span,
        persistent_lane_state,
        symbol_registry: None,
        hot_symbol_tables: None,
    };
    let symbol_registry = symbol_registry_audit::build_registry_for_run(
        &execution_state,
        &inputs.climate,
        "indexed_runtime_surface",
    )?;
    let hot_symbol_tables = build_hillslope_hot_symbol_tables(&symbol_registry);
    if let Some(persistent_lane_state) = execution_state.persistent_lane_state.as_mut() {
        persistent_lane_state
            .activate_indexed_writeback_authority(&symbol_registry)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "indexed_runtime_surface",
                detail: error.to_string(),
            })?;
        if perfdeep03_lane_dense_state_enabled() {
            persistent_lane_state
                .activate_lane_dense_state(&symbol_registry, &hot_symbol_tables)
                .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "perfdeep03_lane_dense_state",
                    detail: error.to_string(),
                })?;
        }
    }
    execution_state.symbol_registry = Some(symbol_registry);
    execution_state.hot_symbol_tables = Some(hot_symbol_tables);

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

fn build_static_runtime_surface_parts(
    request: &HillslopeRunRequest,
    inputs: &ParsedHillslopeRunInputs,
    sidecars: &mut HillslopeSidecarResolution,
) -> Result<StaticRuntimeSurfaceParts, HillslopeCliError> {
    let soil_surface = build_hillslope_runtime_surface_from_soil(&inputs.soil).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "soil",
            detail: error.to_string(),
        }
    })?;
    let slope_surface = build_hillslope_runtime_surface_from_slope_with_options(
        &inputs.slope,
        SlopeRuntimeSurfaceOptions::compatibility(),
    )
    .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "slope",
        detail: error.to_string(),
    })?;
    let management_surface = build_hillslope_runtime_surface_from_management(&inputs.management)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "management",
            detail: error.to_string(),
        })?;
    let management_residue_depth_m = management_surface
        .state_surface
        .get(&BoundarySymbol::from("frost.runtime_residue_depth_m"))
        .copied();
    let pmetpara_template = sidecars.pmetpara.clone();
    let pmetpara_surface = crate::hillslope::intake_lane_setup::build_hillslope_runtime_surface_from_pmetpara(
        &inputs.management,
        &mut sidecars.pmetpara,
        request.sidecar_policy.as_pmetpara_parse_mode(),
    )?;
    let snow_surface = build_hillslope_runtime_surface_from_snow(&sidecars.snow).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "snow",
            detail: error.to_string(),
        }
    })?;
    let frost_surface =
        build_hillslope_runtime_surface_from_frost(&sidecars.frost).map_err(|error| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "frost",
                detail: error.to_string(),
            }
        })?;
    let runtime_surface = merge_static_runtime_surfaces(
        management_surface,
        soil_surface,
        slope_surface,
        snow_surface.clone(),
        frost_surface.clone(),
        pmetpara_surface,
        management_residue_depth_m,
    )?;
    Ok(StaticRuntimeSurfaceParts {
        runtime_surface,
        snow_surface,
        frost_surface,
        pmetpara_template,
    })
}

fn merge_static_runtime_surfaces(
    management_surface: HillslopeWritebackSurface,
    soil_surface: HillslopeWritebackSurface,
    slope_surface: HillslopeWritebackSurface,
    snow_surface: HillslopeWritebackSurface,
    frost_surface: HillslopeWritebackSurface,
    pmetpara_surface: HillslopeWritebackSurface,
    management_residue_depth_m: Option<BoundaryValue>,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    let mut runtime_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
        crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
            crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
                management_surface,
                soil_surface,
            ),
            slope_surface,
        ),
        crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
            crate::hillslope::intake_lane_setup::merge_runtime_surfaces(snow_surface, frost_surface),
            pmetpara_surface,
        ),
    );
    if let Some(residue_depth_m) = management_residue_depth_m {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("frost.runtime_residue_depth_m"), residue_depth_m);
    }
    if runtime_surface.state_surface.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "merged",
            detail: "merged runtime surface is empty".to_string(),
        });
    }
    Ok(runtime_surface)
}

fn build_persistent_lane_state(
    inputs: PersistentLaneStateInputs<'_>,
) -> Result<Option<OfeLanePersistentStateSequence>, HillslopeCliError> {
    if inputs.static_per_ofe_slices.len() <= 1 {
        return Ok(None);
    }
    let lane_states = inputs
        .static_per_ofe_slices
        .iter()
        .enumerate()
        .map(|(index, slice)| {
            let upstream_area_ratio = if index == 0 {
                1.0
            } else {
                inputs.static_per_ofe_slices[index - 1].area_m2 / slice.area_m2
            };
            crate::hillslope::intake_lane_setup::build_static_per_ofe_lane_runtime_surface(
                slice,
                inputs.slope,
                inputs.soil,
                inputs.management,
                inputs.snow_surface,
                inputs.frost_surface,
                inputs.pmetpara_template,
                inputs.pmetpara_parse_mode,
            )
            .map(|surface| {
                OfeLanePersistentState::with_upstream_area_ratio(
                    slice.ofe_id,
                    upstream_area_ratio,
                    surface,
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    OfeLanePersistentStateSequence::new(lane_states)
        .map(Some)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_dynamic_state",
            detail: format!(
                "{SIMPIPE_GUARD_ID} failed initializing persistent OFE lane state: {error}"
            ),
        })
}

fn perfdeep03_lane_dense_state_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| env_flag_enabled("OPENWEPP_PERFDEEP03_LANE_DENSE_STATE"))
}

fn env_flag_enabled(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| {
        matches!(
            value.as_str(),
            "1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON"
        )
    })
}
