use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::runtime_inputs::{
    HillslopeClimateRuntimeRequest, SlopeRuntimeSurfaceOptions,
    build_hillslope_climate_runtime_request,
    build_hillslope_runtime_surface_from_climate_request_with_context,
    build_hillslope_runtime_surface_from_frost, build_hillslope_runtime_surface_from_management,
    build_hillslope_runtime_surface_from_slope_with_options,
    build_hillslope_runtime_surface_from_snow, build_hillslope_runtime_surface_from_soil,
};
use openwepp_hillslope_orchestrator::{
    HillslopeDayFrame, HillslopePhaseScheduler, HillslopeWritebackSurface, OfeLaneExecutionInput,
    OfeLanePersistentState, OfeLanePersistentStateSequence, OfeLaneSequenceExecutionReport,
    SchedulerOutcomeClass, TransferInput, TransferOutput, Wb11HydrologyKernel,
    build_hillslope_hot_symbol_tables,
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

use crate::api::{HillslopeRunReport, HillslopeRunRequest};
use crate::hillslope::intake_lane_setup::StaticOfeLaneSlice;
use crate::constants::{
    DAILY_EXECUTION_LANE, DAILY_TIMESTEP_SECONDS, HILLSLOPE_RUN_MANIFEST_SCHEMA_ID,
    HILLSLOPE_RUNFILE_SCHEMA_ID, HOURLY_EXECUTION_LANE, HOURLY_TIMESTEP_SECONDS,
    REQUIRED_RUN_OUTPUT_LOSS, REQUIRED_RUN_OUTPUT_PASS, SCHEDULER_KERNEL_PUBLICATION_SOURCE,
    SIMCONS_INTAKE_GUARD_ID, SIMCOUP_GUARD_ID, SIMIMPL09_ADOPT_PROFILE,
    SIMIMPL10_FLAG_TOLERANCE, SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
    SIMMODE_TIMESTEP_GUARD_ID, SIMOUT_GUARD_ID, SIMPIPE_GUARD_ID, SUBHOURLY_EXECUTION_LANE,
    WB13_PUBLICATION_SOURCE_SIMULATION_OWNED, WB13_REPLAY_CANDIDATE_SURFACE_PASS,
    WB13_REPLAY_CANDIDATE_SURFACE_WAT, WUI_MODE_GUARD_ID,
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
    mode_selection: HillslopeModeSelectionProvenance,
    timestep_policy: HillslopeTimestepPolicyProvenance,
    adapter_boundary: HillslopeAdapterBoundaryProvenance,
    execution_provenance: HillslopeExecutionProvenance,
    wb13_publication: HillslopeWb13PublicationProvenance,
    mofe_hourly_carry: HillslopeMofeHourlyCarryProvenance,
    coupling_vectors: HillslopeCouplingVectorProvenance,
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
    symbol_registry: &'a SymbolRegistry,
    hot_symbol_tables: &'a HotSymbolTables,
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
    symbol_registry: &'a SymbolRegistry,
    hot_symbol_tables: &'a HotSymbolTables,
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
    let snow = parse_runfile_snow_sidecar(request, sidecar_overrides, &mut resolved_sidecars)?;
    let frost = parse_runfile_frost_sidecar(request, sidecar_overrides, &mut resolved_sidecars)?;
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
    let symbol_registry =
        symbol_registry_audit::build_registry_for_run(&execution_state, &inputs.climate, "indexed_runtime_surface")?;
    let hot_symbol_tables = build_hillslope_hot_symbol_tables(&symbol_registry);
    if let Some(persistent_lane_state) = execution_state.persistent_lane_state.as_mut() {
        persistent_lane_state
            .activate_indexed_writeback_authority(&symbol_registry)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "indexed_runtime_surface",
                detail: error.to_string(),
            })?;
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

fn execute_hillslope_climate_days(
    run_name: &str,
    output_hillslope_id: u32,
    state: HillslopeClimateExecutionState,
    climate: &ClimateFile,
) -> Result<HillslopeClimateExecution, HillslopeCliError> {
    let climate_request = build_hillslope_climate_runtime_request(climate).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: error.to_string(),
        }
    })?;
    let HillslopeClimateExecutionState {
        publication_area_m2,
        contributor_ofe_count,
        static_per_ofe_slice_count,
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        runtime_surface,
        lane_context,
        climate_span,
        mut persistent_lane_state,
        symbol_registry,
        hot_symbol_tables,
    } = state;
    let symbol_registry = symbol_registry.ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "indexed_runtime_surface",
        detail: format!("{SIMPIPE_GUARD_ID} frozen symbol registry was not initialized"),
    })?;
    let hot_symbol_tables = hot_symbol_tables.ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "indexed_runtime_surface",
        detail: format!("{SIMPIPE_GUARD_ID} hot symbol tables were not initialized"),
    })?;
    let persistent_lane_active = persistent_lane_state.is_some();
    let hphys0245_trace_config = hphys0245_trace_config_from_env()?;
    let context = ClimateExecutionContext {
        run_name,
        output_hillslope_id,
        lane: lane_context.lane,
        publication_area_m2,
        first_calendar_year: climate_span.first_day.year,
        hphys0245_trace_config: hphys0245_trace_config.as_ref(),
        symbol_registry: &symbol_registry,
        hot_symbol_tables: &hot_symbol_tables,
    };
    let mut accumulator =
        ClimateExecutionAccumulator::new(runtime_surface, climate_span.days.len(), contributor_ofe_count)?;

    for (day_index, day_projection) in climate_span.days.iter().enumerate() {
        let climate_surface = build_day_climate_surface(
            &climate_request,
            day_index,
            &accumulator.runtime_surface,
            day_projection,
        )?;
        let stale_climate_symbols = accumulator.previous_climate_symbols.clone();
        remove_stale_climate_symbols(&mut accumulator.runtime_surface, &stale_climate_symbols);
        let simulation_year =
            simulation_year_from_calendar_year(day_projection.year, context.first_calendar_year)?;
        accumulator.previous_climate_symbols.clear();
        accumulator
            .previous_climate_symbols
            .extend(climate_surface.state_surface.keys().cloned());
        let mut apply = HillslopeDayApply {
            persistent_lane_state: &mut persistent_lane_state,
            climate_surface,
            stale_climate_symbols: &stale_climate_symbols,
            per_ofe_lane_areas_m2: &per_ofe_lane_areas_m2,
            per_ofe_runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
            context,
            day_index,
            day_projection,
            simulation_year,
            runtime_swe_before_m: accumulator.runtime_swe_publication_state_m,
        };
        accumulator.apply_hillslope_day(&mut apply)?;
    }

    let coupling_vectors = accumulator.coupling_vectors.ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!(
                "{SIMPIPE_GUARD_ID} climate span contained no executable days after parser validation"
            ),
        }
    })?;
    let executed_day_count = climate_span.days.len();
    Ok(HillslopeClimateExecution {
        selected_lane: lane_context.lane,
        publication_area_m2,
        contributor_ofe_count,
        static_per_ofe_slice_count,
        persistent_lane_active,
        runtime_surface: accumulator.runtime_surface,
        climate_span,
        wb13_rows: accumulator.wb13_rows,
        pass_rows: accumulator.pass_rows,
        coupling_vectors,
        erod14_wave2_kernel_status_seen: accumulator.erod14_wave2_kernel_status_seen,
        scheduler_outcome_class: accumulator.scheduler_outcome_class,
        scheduler_status_message_id: accumulator.scheduler_status_message_id,
        kernel_phase_message_ids: accumulator.kernel_phase_message_ids.into_iter().collect(),
        hphys0245_trace_config,
        hphys0245_trace_rows: accumulator.hphys0245_trace_rows,
        per_ofe_internal_wb13_summary: accumulator.per_ofe_internal_wb13_summary,
        executed_day_count,
    })
}

impl ClimateExecutionAccumulator {
    fn new(
        runtime_surface: HillslopeWritebackSurface,
        day_count: usize,
        contributor_ofe_count: usize,
    ) -> Result<Self, HillslopeCliError> {
        let runtime_swe_publication_state_m =
            require_runtime_surface_scalar(&runtime_surface, "snow.runtime_swe")?;
        Ok(Self {
            runtime_surface,
            runtime_swe_publication_state_m,
            wb13_rows: Vec::with_capacity(day_count * contributor_ofe_count.max(1)),
            pass_rows: Vec::with_capacity(day_count),
            coupling_vectors: None,
            erod14_wave2_kernel_status_seen: false,
            scheduler_outcome_class: SchedulerOutcomeClass::Completed,
            scheduler_status_message_id: String::new(),
            previous_climate_symbols: Vec::new(),
            kernel_phase_message_ids: std::collections::BTreeSet::new(),
            hphys0245_trace_rows: Vec::new(),
            per_ofe_internal_wb13_summary: PerOfeInternalWb13RunSummary::default(),
        })
    }

    fn apply_hillslope_day(
        &mut self,
        apply: &mut HillslopeDayApply<'_>,
    ) -> Result<(), HillslopeCliError> {
        let context = SchedulerLifecycleContext {
            run_name: apply.context.run_name,
            execution_lane: apply.context.lane,
            publication_area_m2: apply.context.publication_area_m2,
            simulation_year: apply.simulation_year,
            sim_day_index: apply.day_index + 1,
            calendar_day: apply.day_projection,
            runtime_swe_before_m: apply.runtime_swe_before_m,
            hphys0245_trace_config: apply.context.hphys0245_trace_config,
            symbol_registry: apply.context.symbol_registry,
            hot_symbol_tables: apply.context.hot_symbol_tables,
        };
        if let Some(persistent_lane_state) = apply.persistent_lane_state.as_mut() {
            let persistent_result = execute_persistent_scheduler_kernel_lifecycle(
                persistent_lane_state,
                &apply.climate_surface,
                apply.stale_climate_symbols,
                apply.per_ofe_lane_areas_m2,
                apply.per_ofe_runoff_publication_geometries,
                context,
            )
            .map_err(|error| {
                annotate_day_runtime_error(error, apply.day_index, apply.day_projection)
            })?;
            self.publish_persistent_day_result(persistent_result, apply.context)?;
        } else {
            indexed_shadow_surface::observe_clone_source_surface(&self.runtime_surface)?;
            self.runtime_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
                std::mem::take(&mut self.runtime_surface),
                std::mem::take(&mut apply.climate_surface),
            );
            let execution_result = execute_scheduler_kernel_lifecycle(
                std::mem::take(&mut self.runtime_surface),
                context,
            )
            .map_err(|error| {
                annotate_day_runtime_error(error, apply.day_index, apply.day_projection)
            })?;
            self.publish_single_lane_day_result(execution_result, apply.context)?;
        }
        Ok(())
    }

    fn publish_persistent_day_result(
        &mut self,
        persistent_result: PersistentDailyExecutionResult,
        context: ClimateExecutionContext<'_>,
    ) -> Result<(), HillslopeCliError> {
        self.per_ofe_internal_wb13_summary
            .observe_day(&persistent_result.internal_wb13_collection)?;
        self.runtime_swe_publication_state_m = persistent_result
            .internal_wb13_collection
            .outlet_row()
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_internal_wb13",
                detail: format!("{SIMPIPE_GUARD_ID} internal WB13 collection has no outlet row"),
            })?
            .wb13_row
            .snow_water
            / 1_000.0;
        persistent_result
            .internal_wb13_collection
            .append_publication_rows_to(&mut self.wb13_rows);
        persistent_result
            .internal_wb13_collection
            .append_runoff_delivery_rows_to(
                hillslope_id_for_pass_output(context.output_hillslope_id)?,
                context.publication_area_m2,
                &mut self.pass_rows,
            )?;
        self.observe_persistent_day_result(persistent_result);
        Ok(())
    }

    fn publish_single_lane_day_result(
        &mut self,
        execution_result: DailyExecutionResult,
        context: ClimateExecutionContext<'_>,
    ) -> Result<(), HillslopeCliError> {
        self.runtime_swe_publication_state_m =
            execution_result.wb13_row.wb13_row.snow_water / 1_000.0;
        self.pass_rows.push(build_hillslope_pass_row(
            hillslope_id_for_pass_output(context.output_hillslope_id)?,
            &execution_result.wb13_row,
        )?);
        self.wb13_rows.push(execution_result.wb13_row.clone());
        indexed_shadow_surface::validate_shadow_surface(&execution_result.runtime_surface)?;
        self.observe_single_lane_day_result(execution_result);
        Ok(())
    }

    fn observe_persistent_day_result(&mut self, result: PersistentDailyExecutionResult) {
        self.runtime_surface = result.runtime_surface;
        self.scheduler_outcome_class = result.scheduler_outcome_class;
        self.scheduler_status_message_id = result.scheduler_status_message_id;
        self.coupling_vectors = Some(result.coupling_vectors);
        self.kernel_phase_message_ids
            .extend(result.kernel_phase_message_ids);
        self.erod14_wave2_kernel_status_seen |= result.erod14_wave2_kernel_status_seen;
        self.hphys0245_trace_rows
            .extend(result.hphys0245_trace_rows);
    }

    fn observe_single_lane_day_result(&mut self, result: DailyExecutionResult) {
        self.runtime_surface = result.runtime_surface;
        self.scheduler_outcome_class = result.scheduler_outcome_class;
        self.scheduler_status_message_id = result.scheduler_status_message_id;
        self.coupling_vectors = Some(result.coupling_vectors);
        self.kernel_phase_message_ids
            .extend(result.kernel_phase_message_ids);
        self.erod14_wave2_kernel_status_seen |= result.erod14_wave2_kernel_status_seen;
        self.hphys0245_trace_rows
            .extend(result.hphys0245_trace_rows);
    }
}

fn build_day_climate_surface(
    climate_request: &HillslopeClimateRuntimeRequest,
    day_index: usize,
    runtime_surface: &HillslopeWritebackSurface,
    day_projection: &ClimateDayProjection,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    build_hillslope_runtime_surface_from_climate_request_with_context(
        climate_request,
        day_index,
        &runtime_surface.state_surface,
    )
    .map_err(|error| {
        annotate_day_runtime_error(
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "climate",
                detail: error.to_string(),
            },
            day_index,
            day_projection,
        )
    })
}

fn remove_stale_climate_symbols(
    runtime_surface: &mut HillslopeWritebackSurface,
    stale_climate_symbols: &[BoundarySymbol],
) {
    for symbol in stale_climate_symbols {
        runtime_surface.state_surface.remove(symbol);
        runtime_surface.flux_surface.remove(symbol);
    }
}

fn annotate_day_runtime_error(
    error: HillslopeCliError,
    day_index: usize,
    day_projection: &ClimateDayProjection,
) -> HillslopeCliError {
    match error {
        HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface,
                detail: format!(
                    "{detail} [sim_day_index={}, calendar_year={}, julian_day={}]",
                    day_index + 1,
                    day_projection.year,
                    day_projection.julian_day
                ),
            }
        }
        other => other,
    }
}

fn hillslope_id_for_pass_output(output_hillslope_id: u32) -> Result<i32, HillslopeCliError> {
    i32::try_from(output_hillslope_id).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "outputs.pass_parquet",
        detail: format!("{SIMOUT_GUARD_ID} hillslope id {output_hillslope_id} exceeds i32 range"),
    })
}

fn build_hillslope_execution_provenance(
    execution: &HillslopeClimateExecution,
    sidecar_warnings: &mut Vec<String>,
) -> Result<HillslopeExecutionProvenance, HillslopeCliError> {
    let wb16_ealpha_compatibility_seed_used = parse_mofe03_binary_flag(
        WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL,
        runtime_surface_symbol_value(
            &execution.runtime_surface,
            WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL,
        )
        .unwrap_or(0.0),
    )?;
    if wb16_ealpha_compatibility_seed_used {
        sidecar_warnings.push(format!(
            "{WB16_EALPHA_SEED_WARNING_ID} WB16 ealpha seeded with compatibility constant 1.0 because no runtime producer was present; full baseline-authoritative ealpha producer-chain migration remains open."
        ));
    }
    let erod14_wave2_enabled = parse_mofe03_binary_flag(
        "erod14_wave2_enabled",
        runtime_surface_symbol_value(&execution.runtime_surface, "erod14_wave2_enabled")
            .unwrap_or(0.0),
    )?;
    let erod14_qin_source_policy = erod14_qin_source_policy(erod14_wave2_enabled, sidecar_warnings);
    Ok(HillslopeExecutionProvenance {
        scheduler_kernel_executed: true,
        publication_source: SCHEDULER_KERNEL_PUBLICATION_SOURCE.to_string(),
        simpipe_guard_id: SIMPIPE_GUARD_ID.to_string(),
        selected_lane: execution.selected_lane.as_str().to_string(),
        scheduler_outcome_class: scheduler_outcome_class_as_str(execution.scheduler_outcome_class)
            .to_string(),
        scheduler_status_message_id: execution.scheduler_status_message_id.clone(),
        climate_day_count: execution.climate_span.days.len(),
        executed_day_count: execution.executed_day_count,
        kernel_phase_message_ids: execution.kernel_phase_message_ids.clone(),
        erod14_wave2_enabled,
        erod14_wave2_kernel_status_seen: execution.erod14_wave2_kernel_status_seen,
        erod14_qin_source_policy: erod14_qin_source_policy.to_string(),
        erod14_qin_sediment_coupled: false,
        wb16_ealpha_compatibility_seed_used,
        wb16_ealpha_seed_policy: wb16_ealpha_seed_policy(wb16_ealpha_compatibility_seed_used),
    })
}

fn erod14_qin_source_policy(
    erod14_wave2_enabled: bool,
    sidecar_warnings: &mut Vec<String>,
) -> &'static str {
    if erod14_wave2_enabled {
        sidecar_warnings.push(format!(
            "{EROD14_QIN_WARNING_ID} EROD14 Wave-2 qin is seeded from water-transfer provenance only; true sediment-coupled qin/qout and particle-fraction handoff remains MOFE01 M-G follow-on scope."
        ));
        EROD14_QIN_POLICY_WATER_TRANSFER_ONLY
    } else {
        EROD14_QIN_POLICY_WAVE2_DISABLED
    }
}

fn wb16_ealpha_seed_policy(wb16_ealpha_compatibility_seed_used: bool) -> String {
    if wb16_ealpha_compatibility_seed_used {
        WB16_EALPHA_SEED_POLICY_COMPATIBILITY.to_string()
    } else {
        WB16_EALPHA_SEED_POLICY_RUNTIME_PROVIDED.to_string()
    }
}

fn build_hillslope_publication_provenance(
    execution: &HillslopeClimateExecution,
) -> Result<
    (
        HillslopeWb13PublicationProvenance,
        HillslopeMofeHourlyCarryProvenance,
    ),
    HillslopeCliError,
> {
    let per_ofe_summary = execution
        .persistent_lane_active
        .then_some(&execution.per_ofe_internal_wb13_summary);
    let wb13_publication = build_wb13_publication_provenance(
        &execution.wb13_rows,
        execution.contributor_ofe_count,
        execution.static_per_ofe_slice_count,
        execution.publication_area_m2,
        execution.persistent_lane_active,
        per_ofe_summary,
    )?;
    let mofe_hourly_carry = build_mofe_hourly_carry_provenance(
        &execution.runtime_surface,
        execution.contributor_ofe_count,
    )?;
    Ok((wb13_publication, mofe_hourly_carry))
}

fn write_hillslope_run_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    sidecars: &HillslopeSidecarResolution,
    execution: &HillslopeClimateExecution,
) -> Result<(), HillslopeCliError> {
    let pass_bytes = build_hbp_output(
        &targets.output_pass,
        &execution.wb13_rows,
        &execution.runtime_surface,
        execution.contributor_ofe_count,
    )?;
    let loss_text = build_loss_output_json(
        &inputs.runfile.run_name,
        &inputs.soil,
        &sidecars.snow,
        &sidecars.frost,
        &execution.climate_span,
        execution.executed_day_count,
    )?;
    ensure_hillslope_output_parent_directories(targets)?;
    fs::write(&targets.output_pass, pass_bytes).map_err(|source| {
        HillslopeCliError::OutputWrite {
            path: targets.output_pass.clone(),
            source,
        }
    })?;
    fs::write(&targets.output_loss, loss_text).map_err(|source| {
        HillslopeCliError::OutputWrite {
            path: targets.output_loss.clone(),
            source,
        }
    })?;
    write_hillslope_optional_outputs(inputs, targets, execution)?;
    validate_required_hillslope_outputs(targets)
}

fn ensure_hillslope_output_parent_directories(
    targets: &HillslopeOutputTargets,
) -> Result<(), HillslopeCliError> {
    for path in std::iter::once(&targets.output_pass)
        .chain(std::iter::once(&targets.output_loss))
        .chain(targets.optional_outputs.iter())
    {
        crate::hillslope::intake_lane_setup::ensure_output_parent_directory(path)?;
    }
    Ok(())
}

fn write_hillslope_optional_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    execution: &HillslopeClimateExecution,
) -> Result<(), HillslopeCliError> {
    if let Some(wat_output) = inputs.runfile.output_config.wat.as_ref() {
        let wat_rows = build_hillslope_wat_rows(&execution.wb13_rows)?;
        write_hillslope_wat_parquet(wat_output, &wat_rows, InterchangeVersion::default()).map_err(
            |error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.wat",
                detail: error.to_string(),
            },
        )?;
    }
    if let Some(pass_parquet_output) = inputs.runfile.output_config.pass_parquet.as_ref() {
        write_hillslope_pass_parquet(
            pass_parquet_output,
            &execution.pass_rows,
            InterchangeVersion::default(),
        )
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass_parquet",
            detail: error.to_string(),
        })?;
    }
    write_hphys0245_trace_output(execution)?;
    write_generic_optional_outputs(inputs, targets, execution)
}

fn write_hphys0245_trace_output(
    execution: &HillslopeClimateExecution,
) -> Result<(), HillslopeCliError> {
    if let Some(trace_config) = execution.hphys0245_trace_config.as_ref() {
        write_hphys0245_trace_jsonl(trace_config, &execution.hphys0245_trace_rows)?;
    }
    Ok(())
}

fn write_generic_optional_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    execution: &HillslopeClimateExecution,
) -> Result<(), HillslopeCliError> {
    for optional_output in targets
        .optional_outputs
        .iter()
        .filter(|path| Some(path.as_path()) != inputs.runfile.output_config.wat.as_deref())
        .filter(|path| Some(path.as_path()) != inputs.runfile.output_config.pass_parquet.as_deref())
    {
        let payload = build_optional_output_payload(
            &inputs.runfile.run_name,
            optional_output,
            &execution.climate_span,
            execution.executed_day_count,
        );
        fs::write(optional_output, payload).map_err(|source| HillslopeCliError::OutputWrite {
            path: optional_output.clone(),
            source,
        })?;
    }
    Ok(())
}

fn validate_required_hillslope_outputs(
    targets: &HillslopeOutputTargets,
) -> Result<(), HillslopeCliError> {
    if !targets.output_pass.is_file() {
        return Err(HillslopeCliError::MissingRequiredOutput {
            output_name: REQUIRED_RUN_OUTPUT_PASS,
        });
    }
    if !targets.output_loss.is_file() {
        return Err(HillslopeCliError::MissingRequiredOutput {
            output_name: REQUIRED_RUN_OUTPUT_LOSS,
        });
    }
    Ok(())
}

fn write_hillslope_run_manifest(
    publication: HillslopeManifestPublication<'_>,
) -> Result<PathBuf, HillslopeCliError> {
    let binary_path = std::env::current_exe().map_err(|source| HillslopeCliError::Io {
        path: PathBuf::from("<current_exe>"),
        source,
    })?;
    let binary_sidecar_path = write_release_sidecar_for_binary(&binary_path, BinaryRole::Hillslope)
        .map_err(|source| HillslopeCliError::ReleaseMetadata { source })?;
    let invoked_utc =
        utc_now_rfc3339().map_err(|detail| HillslopeCliError::TimeFormat { detail })?;
    let input_checksums =
        build_hillslope_input_checksums(publication.inputs, publication.sidecars.input_paths)?;
    let output_checksums = build_hillslope_output_checksums(publication.targets)?;
    let manifest_path = publication.request.manifest_path.clone().unwrap_or_else(|| {
        publication
            .request
            .output_dir
            .join("openwepp_hillslope_run_manifest.json")
    });
    let manifest = build_hillslope_run_manifest(
        publication,
        &binary_path,
        &binary_sidecar_path,
        invoked_utc,
        input_checksums,
        output_checksums,
    )?;
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })?;
    fs::write(&manifest_path, manifest_json).map_err(|source| {
        HillslopeCliError::ManifestWrite {
            path: manifest_path.clone(),
            source,
        }
    })?;
    Ok(manifest_path)
}

fn build_hillslope_input_checksums(
    inputs: &ParsedHillslopeRunInputs,
    sidecar_input_paths: &HillslopeSidecarInputPaths,
) -> Result<BTreeMap<String, String>, HillslopeCliError> {
    let mut checksums = BTreeMap::new();
    let mut input_paths: Vec<&Path> = vec![
        inputs.run_file_path.as_path(),
        inputs.soil_path.as_path(),
        inputs.management_path.as_path(),
        inputs.slope_path.as_path(),
        inputs.climate_path.as_path(),
    ];
    input_paths.extend(optional_sidecar_input_paths(sidecar_input_paths));
    for path in input_paths {
        checksums.insert(
            path.display().to_string(),
            sha256_file_hex(path).map_err(|source| HillslopeCliError::Io {
                path: path.to_path_buf(),
                source,
            })?,
        );
    }
    Ok(checksums)
}

fn optional_sidecar_input_paths(input_paths: &HillslopeSidecarInputPaths) -> Vec<&Path> {
    [
        input_paths.snow.as_deref(),
        input_paths.frost.as_deref(),
        input_paths.wepp_ui.as_deref(),
        input_paths.pmetpara.as_deref(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn build_hillslope_output_checksums(
    targets: &HillslopeOutputTargets,
) -> Result<BTreeMap<String, String>, HillslopeCliError> {
    let mut output_checksum_entries = Vec::new();
    for path in std::iter::once(&targets.output_pass)
        .chain(std::iter::once(&targets.output_loss))
        .chain(targets.optional_outputs.iter())
    {
        output_checksum_entries.push(OutputChecksumEntry::new(
            path.display().to_string(),
            sha256_file_hex(path).map_err(|source| HillslopeCliError::Io {
                path: path.clone(),
                source,
            })?,
        ));
    }
    assemble_output_checksums(&output_checksum_entries).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "manifest_output_checksums",
            detail: error.to_string(),
        }
    })
}

fn build_hillslope_run_manifest(
    publication: HillslopeManifestPublication<'_>,
    binary_path: &Path,
    binary_sidecar_path: &Path,
    invoked_utc: String,
    input_checksums: BTreeMap<String, String>,
    output_checksums: BTreeMap<String, String>,
) -> Result<HillslopeRunManifest, HillslopeCliError> {
    Ok(HillslopeRunManifest {
        schema: HILLSLOPE_RUN_MANIFEST_SCHEMA_ID.to_string(),
        engine: "openwepp".to_string(),
        binary_path: binary_path.display().to_string(),
        binary_sha256: sha256_file_hex(binary_path).map_err(|source| HillslopeCliError::Io {
            path: binary_path.to_path_buf(),
            source,
        })?,
        binary_sidecar_path: binary_sidecar_path.display().to_string(),
        binary_sidecar_sha256: sha256_file_hex(binary_sidecar_path).map_err(|source| {
            HillslopeCliError::Io {
                path: binary_sidecar_path.to_path_buf(),
                source,
            }
        })?,
        source_commit: git_source_commit_or_unknown(),
        invoked_utc,
        argv: publication.argv.to_vec(),
        run_dir: publication.request.run_dir.display().to_string(),
        run_file: publication.inputs.run_file_path.display().to_string(),
        sidecar_policy: publication.request.sidecar_policy.as_str().to_string(),
        sidecar_discovery_mode: publication.sidecars.discovery_mode.to_string(),
        resolved_sidecars: publication.sidecars.resolved_sidecars,
        input_checksums,
        output_checksums,
        mode_selection: publication.sidecars.mode_selection,
        timestep_policy: publication.timestep_policy,
        adapter_boundary: publication.adapter_boundary,
        execution_provenance: publication.execution_provenance,
        wb13_publication: publication.wb13_publication,
        mofe_hourly_carry: publication.mofe_hourly_carry,
        coupling_vectors: publication.coupling_vectors,
    })
}

pub fn execute_hillslope_run(
    request: &HillslopeRunRequest,
    argv: &[String],
) -> Result<HillslopeRunReport, HillslopeCliError> {
    if !request.run_dir.is_dir() {
        return Err(HillslopeCliError::RunDirectoryMissing {
            path: request.run_dir.clone(),
        });
    }

    fs::create_dir_all(&request.output_dir).map_err(|source| {
        HillslopeCliError::OutputDirectoryCreate {
            path: request.output_dir.clone(),
            source,
        }
    })?;

    let inputs = load_hillslope_run_inputs(request)?;
    let targets = resolve_hillslope_output_targets(&inputs.runfile)?;
    let mut sidecars = resolve_hillslope_sidecars(request, &inputs, &targets)?;
    let runtime_setup = build_static_hillslope_runtime_setup(request, &inputs, &mut sidecars)?;
    let timestep_policy = runtime_setup.timestep_policy;
    let adapter_boundary = runtime_setup.adapter_boundary;
    let symbol_registry_audit =
        symbol_registry_audit::begin_if_requested(&runtime_setup.execution_state, &inputs.climate)?;
    let indexed_shadow =
        indexed_shadow_surface::begin_if_requested(&runtime_setup.execution_state, &inputs.climate)?;
    let execution_result = execute_hillslope_climate_days(
        &inputs.runfile.run_name,
        targets.output_hillslope_id,
        runtime_setup.execution_state,
        &inputs.climate,
    );
    if let Some(symbol_registry_audit) = symbol_registry_audit {
        symbol_registry_audit.finish()?;
    }
    if let Some(indexed_shadow) = indexed_shadow {
        indexed_shadow.finish()?;
    }
    let execution = execution_result?;
    let execution_provenance =
        build_hillslope_execution_provenance(&execution, &mut sidecars.sidecar_warnings)?;
    let (wb13_publication, mofe_hourly_carry) = build_hillslope_publication_provenance(&execution)?;
    write_hillslope_run_outputs(&inputs, &targets, &sidecars, &execution)?;

    let HillslopeSidecarResolution {
        mode_selection,
        resolved_sidecars,
        sidecar_warnings,
        input_paths,
        discovery_mode,
        ..
    } = sidecars;
    let manifest_path = write_hillslope_run_manifest(HillslopeManifestPublication {
        request,
        argv,
        inputs: &inputs,
        targets: &targets,
        sidecars: HillslopeSidecarManifestInputs {
            discovery_mode,
            resolved_sidecars,
            input_paths: &input_paths,
            mode_selection,
        },
        timestep_policy,
        adapter_boundary,
        execution_provenance,
        wb13_publication,
        mofe_hourly_carry,
        coupling_vectors: execution.coupling_vectors,
    })?;

    Ok(HillslopeRunReport {
        output_pass: targets.output_pass,
        output_loss: targets.output_loss,
        optional_outputs: targets.optional_outputs,
        manifest_path,
        sidecar_warnings,
    })
}
