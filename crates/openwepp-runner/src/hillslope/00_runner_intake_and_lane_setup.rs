use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::runtime_inputs::{
    SlopeRuntimeSurfaceOptions, build_hillslope_climate_runtime_request,
    build_hillslope_runtime_surface_from_climate_request_with_context,
    build_hillslope_runtime_surface_from_frost, build_hillslope_runtime_surface_from_management,
    build_hillslope_runtime_surface_from_slope_with_options,
    build_hillslope_runtime_surface_from_snow, build_hillslope_runtime_surface_from_soil,
};
use openwepp_hillslope_orchestrator::{
    HillslopePhaseScheduler, HillslopeWritebackSurface, SchedulerOutcomeClass, Wb11HydrologyKernel,
};
use openwepp_hillslope_output::contracts::{HillslopeOutputConfig, validate_output_contract};
use openwepp_hillslope_output::hillslope_wat::{
    HillslopeWatRow, InterchangeVersion, write_hillslope_wat_parquet,
};
use openwepp_hillslope_output::manifest::{OutputChecksumEntry, assemble_output_checksums};
use openwepp_hillslope_output::writers::{optional_output_paths, required_output_paths};
use openwepp_input_contract::parsers::climate::{ClimateDailyRecord, parse_climate_file};
use openwepp_input_contract::parsers::frost::{parse_frost_from_path, parse_frost_from_str};
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
use openwepp_input_contract::parsers::soil::{SoilParserOptions, TopologyScope, parse_soil};
use openwepp_input_contract::parsers::wepp_ui::{
    WeppUiParseResult, WeppUiParserOptions, parse_wepp_ui_from_path,
};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeKernel, HillslopeKernelRequest, KernelRunResponse,
    KernelWritebackPayload,
};
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterRequest, SidecarBinding, SidecarContract, SidecarDiscovery, SidecarId,
    SidecarRequirement, adapt_sidecar_bindings,
};
use openwepp_summary_accumulator::{SummaryScalarSurface, Wb13DailyWaterBalanceRow};
use openwepp_topology::{TopologyGraph, validate_pre_execution_topology};
use serde::{Deserialize, Serialize};

use crate::api::{HillslopeRunReport, HillslopeRunRequest};
use crate::constants::{
    DAILY_EXECUTION_LANE, DAILY_TIMESTEP_SECONDS, HILLSLOPE_RUN_MANIFEST_SCHEMA_ID,
    HILLSLOPE_RUNFILE_SCHEMA_ID, HOURLY_EXECUTION_LANE, HOURLY_TIMESTEP_SECONDS,
    REQUIRED_RUN_OUTPUT_LOSS, REQUIRED_RUN_OUTPUT_PASS, SCHEDULER_KERNEL_PUBLICATION_SOURCE,
    SIMCONS_INTAKE_GUARD_ID, SIMCOUP_GUARD_ID, SIMIMPL09_ADOPT_PROFILE, SIMIMPL10_FLAG_TOLERANCE,
    SIMIMPL10_FROST_MAX_DEPTH_M, SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
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
struct HillslopeWb13PublicationProvenance {
    source: String,
    projection_fallback_used: bool,
    guard_id: String,
    replay_candidate_surfaces: Vec<String>,
    publication_ofe_policy: String,
    contributor_ofe_count: usize,
    area_policy: String,
    storage_lineage_policy: String,
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

#[allow(clippy::too_many_lines)]
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

    let run_file_path = resolve_run_file(&request.run_dir, &request.run_file);
    if !run_file_path.is_file() {
        return Err(HillslopeCliError::RunFileMissing {
            path: run_file_path,
        });
    }

    let runfile = parse_runfile_execution_config(&run_file_path, request.legacy_sidecar_discovery)?;

    let soil_path = runfile.soil_path.clone();
    let management_path = runfile.management_path.clone();
    let slope_path = runfile.slope_path.clone();
    let climate_path = runfile.climate_path.clone();

    let slope = parse_slope_file(
        &slope_path,
        request.sidecar_policy.as_slope_parser_options(),
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "slope",
        detail: error.to_string(),
    })?;

    let management = parse_management_from_path(
        &management_path,
        request.sidecar_policy.as_management_parser_mode(),
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "management",
        detail: error.to_string(),
    })?;

    let soil_raw = fs::read_to_string(&soil_path).map_err(|source| HillslopeCliError::Io {
        path: soil_path.clone(),
        source,
    })?;
    let expected_soil_topology_count = if slope.ofe_count == management.topology_count {
        Some(slope.ofe_count)
    } else {
        None
    };
    let soil_options = SoilParserOptions {
        mode: request.sidecar_policy.as_soil_parser_mode(),
        allow_legacy_aliases: true,
        expected_topology_count: expected_soil_topology_count,
        topology_scope: expected_soil_topology_count.map(|_| TopologyScope::Hillslope),
    };
    let soil =
        parse_soil(&soil_raw, soil_options).map_err(|error| HillslopeCliError::ParseFailure {
            surface: "soil",
            detail: error.to_string(),
        })?;
    validate_hillslope_ofe_topology_parity(slope.ofe_count, management.topology_count, soil.ntemp)?;

    let climate = parse_climate_file(
        &climate_path,
        request.sidecar_policy.as_climate_parser_mode(),
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "climate",
        detail: error.to_string(),
    })?;

    let mut resolved_sidecars = BTreeMap::new();
    let mut sidecar_warnings = Vec::new();
    let mut snow_input_path: Option<PathBuf> = None;
    let mut frost_input_path: Option<PathBuf> = None;
    let mut wepp_ui_input_path: Option<PathBuf> = None;
    let mut pmetpara_input_path: Option<PathBuf> = None;
    let sidecar_discovery_mode = if request.legacy_sidecar_discovery {
        "legacy-sidecar-discovery"
    } else {
        "runfile-sidecar-overrides"
    };

    let soil_versions = vec![soil.datver.numeric(); soil.ofes.len().max(1)];
    let [output_pass, output_loss] = required_output_paths(&runfile.output_config);
    let optional_outputs = optional_output_paths(&runfile.output_config);

    let (snow, frost, wepp_ui_mode_selection, mut pmetpara) = if request.legacy_sidecar_discovery {
        let mut excluded_files = vec![
            file_name_string(&run_file_path),
            file_name_string(&soil_path),
            file_name_string(&management_path),
            file_name_string(&slope_path),
            file_name_string(&climate_path),
            "openwepp_hillslope_run_manifest.json".to_string(),
        ];
        excluded_files.extend(
            std::iter::once(file_name_string(&output_pass))
                .chain(std::iter::once(file_name_string(&output_loss)))
                .chain(optional_outputs.iter().map(|path| file_name_string(path)))
                .filter(|name| !name.is_empty()),
        );

        let discovered_sidecars = discover_sidecars(&request.run_dir, &excluded_files)?;

        let sidecar_contracts = hillslope_sidecar_contracts(true)?;
        let sidecar_response = adapt_sidecar_bindings(&SidecarAdapterRequest {
            policy: request.sidecar_policy.as_legacy_bridge_policy(),
            contracts: sidecar_contracts,
            discovered: discovered_sidecars,
        })
        .map_err(|source| HillslopeCliError::SidecarAdapter { source })?;

        for binding in &sidecar_response.bindings {
            resolved_sidecars.insert(
                binding.sidecar_id.as_str().to_string(),
                binding.resolved_path.display().to_string(),
            );
        }
        sidecar_warnings = sidecar_response
            .warnings
            .iter()
            .map(|warning| format!("{} {}", warning.code.message_id(), warning.detail))
            .collect();

        let snow_path = optional_sidecar_binding_path(&sidecar_response.bindings, "snow")
            .unwrap_or_else(|| request.run_dir.join("snow.txt"));
        let frost_path = optional_sidecar_binding_path(&sidecar_response.bindings, "frost")
            .unwrap_or_else(|| request.run_dir.join("frost.txt"));
        let wepp_ui_path = optional_sidecar_binding_path(&sidecar_response.bindings, "wepp_ui")
            .unwrap_or_else(|| request.run_dir.join("wepp_ui.txt"));
        let pmetpara_path = optional_sidecar_binding_path(&sidecar_response.bindings, "pmetpara")
            .unwrap_or_else(|| request.run_dir.join("pmetpara.txt"));

        if snow_path.is_file() {
            snow_input_path = Some(snow_path.clone());
            resolved_sidecars.insert("snow".to_string(), snow_path.display().to_string());
        }
        if frost_path.is_file() {
            frost_input_path = Some(frost_path.clone());
            resolved_sidecars.insert("frost".to_string(), frost_path.display().to_string());
        }
        let wepp_ui_requested = wepp_ui_path.is_file();
        if wepp_ui_requested {
            wepp_ui_input_path = Some(wepp_ui_path.clone());
            resolved_sidecars.insert("wepp_ui".to_string(), wepp_ui_path.display().to_string());
        }
        if pmetpara_path.is_file() {
            pmetpara_input_path = Some(pmetpara_path.clone());
            resolved_sidecars.insert("pmetpara".to_string(), pmetpara_path.display().to_string());
        }

        let snow = parse_snow_file(&snow_path, request.sidecar_policy.as_snow_parse_options())
            .map_err(|error| HillslopeCliError::ParseFailure {
                surface: "snow",
                detail: error.to_string(),
            })?;
        let frost =
            parse_frost_from_path(&frost_path, request.sidecar_policy.as_frost_parse_mode())
                .map_err(|error| HillslopeCliError::ParseFailure {
                    surface: "frost",
                    detail: error.to_string(),
                })?;

        let wepp_ui = parse_wepp_ui_from_path(
            &wepp_ui_path,
            WeppUiParserOptions {
                mode: request.sidecar_policy.as_wepp_ui_parse_mode(),
                requested_hourly_seepage: wepp_ui_requested,
                soil_versions: soil_versions.clone(),
            },
        )
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "wepp_ui",
            detail: error.to_string(),
        })?;
        sidecar_warnings.extend(
            wepp_ui
                .warnings
                .iter()
                .map(|warning| format!("{} {}", warning.code.as_str(), warning.message)),
        );

        let pmetpara = parse_pmetpara_file(
            &pmetpara_path,
            PmetparaParseOptions {
                mode: request.sidecar_policy.as_pmetpara_parse_mode(),
                require_sidecar: false,
            },
        )
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "pmetpara",
            detail: error.to_string(),
        })?;

        (
            snow,
            frost,
            build_mode_selection_provenance(&wepp_ui)?,
            pmetpara,
        )
    } else {
        let sidecar_overrides = &runfile.sidecar_overrides;

        let snow = if let Some(snow_inline) = sidecar_overrides.snow {
            resolved_sidecars.insert("snow".to_string(), "<inline>".to_string());
            parse_snow_from_str(
                &format!(
                    "{}\n{}\n{}\n",
                    snow_inline.rst, snow_inline.newsnw, snow_inline.ssd
                ),
                request.sidecar_policy.as_snow_parse_options(),
            )
            .map_err(|error| HillslopeCliError::ParseFailure {
                surface: "snow",
                detail: error.to_string(),
            })?
        } else {
            SnowParseOutput {
                sidecar_present: false,
                defaults_applied: true,
                rst: 0.0,
                newsnw: 100.0,
                ssd: 250.0,
                surplus_record_count: 0,
                trailing_token_lines: Vec::new(),
                prefix_variant_detected: false,
                warnings: Vec::new(),
            }
        };

        let frost = if let Some(frost_inline) = sidecar_overrides.frost {
            resolved_sidecars.insert("frost".to_string(), "<inline>".to_string());
            parse_frost_from_str(
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
            })?
        } else {
            openwepp_input_contract::parsers::frost::FrostParseOutput::defaults_for_missing_file(
                request.sidecar_policy.as_frost_parse_mode(),
            )
        };

        let wepp_ui_path = request.run_dir.join("wepp_ui.txt");
        if wepp_ui_path.is_file() {
            wepp_ui_input_path = Some(wepp_ui_path.clone());
            resolved_sidecars.insert("wepp_ui".to_string(), wepp_ui_path.display().to_string());
        }
        let wepp_ui = parse_wepp_ui_from_path(
            &wepp_ui_path,
            WeppUiParserOptions {
                mode: request.sidecar_policy.as_wepp_ui_parse_mode(),
                requested_hourly_seepage: sidecar_overrides.wepp_ui,
                soil_versions: soil_versions.clone(),
            },
        )
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "wepp_ui",
            detail: error.to_string(),
        })?;
        sidecar_warnings.extend(
            wepp_ui
                .warnings
                .iter()
                .map(|warning| format!("{} {}", warning.code.as_str(), warning.message)),
        );

        let default_pmetpara_path = request.run_dir.join("pmetpara.txt");
        let pmetpara_path = sidecar_overrides.pmetpara_path.clone().or_else(|| {
            default_pmetpara_path
                .is_file()
                .then_some(default_pmetpara_path)
        });
        let pmetpara = if let Some(pmetpara_path) = pmetpara_path {
            pmetpara_input_path = Some(pmetpara_path.clone());
            resolved_sidecars.insert("pmetpara".to_string(), pmetpara_path.display().to_string());

            parse_pmetpara_file(
                &pmetpara_path,
                PmetparaParseOptions {
                    mode: request.sidecar_policy.as_pmetpara_parse_mode(),
                    require_sidecar: true,
                },
            )
            .map_err(|error| HillslopeCliError::ParseFailure {
                surface: "pmetpara",
                detail: error.to_string(),
            })?
        } else {
            absent_pmetpara_file()
        };

        (
            snow,
            frost,
            build_mode_selection_provenance(&wepp_ui)?,
            pmetpara,
        )
    };

    let publication_area_m2 = derive_mofe04_publication_area_from_slope(&slope)?;
    let contributor_ofe_count = slope.ofe_count;

    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "soil",
            detail: error.to_string(),
        }
    })?;
    let slope_surface = build_hillslope_runtime_surface_from_slope_with_options(
        &slope,
        SlopeRuntimeSurfaceOptions::compatibility(),
    )
    .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "slope",
        detail: error.to_string(),
    })?;
    let management_surface =
        build_hillslope_runtime_surface_from_management(&management).map_err(|error| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "management",
                detail: error.to_string(),
            }
        })?;
    let pmetpara_surface = build_hillslope_runtime_surface_from_pmetpara(
        &management,
        &mut pmetpara,
        request.sidecar_policy.as_pmetpara_parse_mode(),
    )?;
    let snow_surface = build_hillslope_runtime_surface_from_snow(&snow).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "snow",
            detail: error.to_string(),
        }
    })?;
    let frost_surface = build_hillslope_runtime_surface_from_frost(&frost).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "frost",
            detail: error.to_string(),
        }
    })?;

    let static_runtime_surface = merge_runtime_surfaces(
        merge_runtime_surfaces(
            merge_runtime_surfaces(management_surface, soil_surface),
            slope_surface,
        ),
        merge_runtime_surfaces(
            merge_runtime_surfaces(snow_surface, frost_surface),
            pmetpara_surface,
        ),
    );
    if static_runtime_surface.state_surface.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "merged",
            detail: "merged runtime surface is empty".to_string(),
        });
    }

    let lane_context = build_execution_lane_context(&wepp_ui_mode_selection)?;
    let timestep_policy = build_timestep_policy_provenance(&lane_context);
    let adapter_boundary = build_adapter_boundary_provenance(&lane_context)?;
    let climate_span = build_climate_run_span_summary(&climate)?;
    let climate_request = build_hillslope_climate_runtime_request(&climate).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: error.to_string(),
        }
    })?;

    let mut runtime_surface = static_runtime_surface;
    let mut runtime_swe_publication_state_m =
        require_runtime_surface_scalar(&runtime_surface, "snow.runtime_swe")?;
    let mut wb13_rows = Vec::with_capacity(climate_span.days.len());
    let mut coupling_vectors = None;
    let mut erod14_wave2_kernel_status_seen = false;
    let mut scheduler_outcome_class = SchedulerOutcomeClass::Completed;
    let mut scheduler_status_message_id = String::new();
    let mut previous_climate_symbols: Vec<BoundarySymbol> = Vec::new();
    let mut kernel_phase_message_ids = std::collections::BTreeSet::new();
    let hphys0245_trace_config = hphys0245_trace_config_from_env()?;
    let mut hphys0245_trace_rows = Vec::new();

    for (day_index, day_projection) in climate_span.days.iter().enumerate() {
        let climate_surface = build_hillslope_runtime_surface_from_climate_request_with_context(
            &climate_request,
            day_index,
            &runtime_surface.state_surface,
        )
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: error.to_string(),
        })?;
        for symbol in &previous_climate_symbols {
            runtime_surface.state_surface.remove(symbol);
            runtime_surface.flux_surface.remove(symbol);
        }
        previous_climate_symbols.clear();
        previous_climate_symbols.extend(climate_surface.state_surface.keys().cloned());
        runtime_surface = merge_runtime_surfaces(runtime_surface, climate_surface);

        let simulation_year =
            simulation_year_from_calendar_year(day_projection.year, climate_span.first_day.year)?;
        let execution_result = execute_scheduler_kernel_lifecycle(
            runtime_surface,
            SchedulerLifecycleContext {
                run_name: &runfile.run_name,
                execution_lane: lane_context.lane,
                publication_area_m2,
                simulation_year,
                sim_day_index: day_index + 1,
                calendar_day: day_projection,
                runtime_swe_before_m: runtime_swe_publication_state_m,
                hphys0245_trace_config: hphys0245_trace_config.as_ref(),
            },
        )
        .map_err(|error| match error {
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
        })?;
        runtime_surface = execution_result.runtime_surface;
        runtime_swe_publication_state_m = execution_result.wb13_row.wb13_row.snow_water / 1_000.0;
        scheduler_outcome_class = execution_result.scheduler_outcome_class;
        scheduler_status_message_id = execution_result.scheduler_status_message_id;
        coupling_vectors = Some(execution_result.coupling_vectors);
        for message_id in execution_result.kernel_phase_message_ids {
            kernel_phase_message_ids.insert(message_id);
        }
        erod14_wave2_kernel_status_seen |= execution_result.erod14_wave2_kernel_status_seen;
        hphys0245_trace_rows.extend(execution_result.hphys0245_trace_rows);
        wb13_rows.push(execution_result.wb13_row);
    }

    let executed_day_count = wb13_rows.len();
    let coupling_vectors = coupling_vectors.ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "execution_provenance",
        detail: format!(
            "{SIMPIPE_GUARD_ID} climate span contained no executable days after parser validation"
        ),
    })?;
    let wb16_ealpha_compatibility_seed_used = parse_mofe03_binary_flag(
        WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL,
        runtime_surface_symbol_value(&runtime_surface, WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL)
            .unwrap_or(0.0),
    )?;
    if wb16_ealpha_compatibility_seed_used {
        sidecar_warnings.push(format!(
            "{WB16_EALPHA_SEED_WARNING_ID} WB16 ealpha seeded with compatibility constant 1.0 because no runtime producer was present; full baseline-authoritative ealpha producer-chain migration remains open."
        ));
    }
    let wb16_ealpha_seed_policy = if wb16_ealpha_compatibility_seed_used {
        WB16_EALPHA_SEED_POLICY_COMPATIBILITY.to_string()
    } else {
        WB16_EALPHA_SEED_POLICY_RUNTIME_PROVIDED.to_string()
    };

    let execution_provenance = HillslopeExecutionProvenance {
        scheduler_kernel_executed: true,
        publication_source: SCHEDULER_KERNEL_PUBLICATION_SOURCE.to_string(),
        simpipe_guard_id: SIMPIPE_GUARD_ID.to_string(),
        selected_lane: lane_context.lane.as_str().to_string(),
        scheduler_outcome_class: scheduler_outcome_class_as_str(scheduler_outcome_class)
            .to_string(),
        scheduler_status_message_id,
        climate_day_count: climate_span.days.len(),
        executed_day_count,
        kernel_phase_message_ids: kernel_phase_message_ids.into_iter().collect(),
        erod14_wave2_enabled: parse_mofe03_binary_flag(
            "erod14_wave2_enabled",
            runtime_surface_symbol_value(&runtime_surface, "erod14_wave2_enabled").unwrap_or(0.0),
        )?,
        erod14_wave2_kernel_status_seen,
        wb16_ealpha_compatibility_seed_used,
        wb16_ealpha_seed_policy,
    };
    let wb13_publication =
        build_wb13_publication_provenance(&wb13_rows, contributor_ofe_count, publication_area_m2)?;
    let mofe_hourly_carry =
        build_mofe_hourly_carry_provenance(&runtime_surface, contributor_ofe_count)?;
    let pass_bytes = build_hbp_output(
        &output_pass,
        &wb13_rows,
        &runtime_surface,
        contributor_ofe_count,
    )?;
    let loss_text = build_loss_output_json(
        &runfile.run_name,
        &soil,
        &snow,
        &frost,
        &climate_span,
        executed_day_count,
    )?;

    for path in std::iter::once(&output_pass)
        .chain(std::iter::once(&output_loss))
        .chain(optional_outputs.iter())
    {
        ensure_output_parent_directory(path)?;
    }

    fs::write(&output_pass, pass_bytes).map_err(|source| HillslopeCliError::OutputWrite {
        path: output_pass.clone(),
        source,
    })?;
    fs::write(&output_loss, loss_text).map_err(|source| HillslopeCliError::OutputWrite {
        path: output_loss.clone(),
        source,
    })?;

    if let Some(wat_output) = runfile.output_config.wat.as_ref() {
        let wat_rows = build_hillslope_wat_rows(&wb13_rows)?;
        write_hillslope_wat_parquet(wat_output, &wat_rows, InterchangeVersion::default()).map_err(
            |error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.wat",
                detail: error.to_string(),
            },
        )?;
    }

    if let Some(trace_config) = hphys0245_trace_config.as_ref() {
        write_hphys0245_trace_jsonl(trace_config, &hphys0245_trace_rows)?;
    }

    for optional_output in optional_outputs
        .iter()
        .filter(|path| Some(path.as_path()) != runfile.output_config.wat.as_deref())
    {
        let payload = build_optional_output_payload(
            &runfile.run_name,
            optional_output,
            &climate_span,
            executed_day_count,
        );
        fs::write(optional_output, payload).map_err(|source| HillslopeCliError::OutputWrite {
            path: optional_output.clone(),
            source,
        })?;
    }

    if !output_pass.is_file() {
        return Err(HillslopeCliError::MissingRequiredOutput {
            output_name: REQUIRED_RUN_OUTPUT_PASS,
        });
    }
    if !output_loss.is_file() {
        return Err(HillslopeCliError::MissingRequiredOutput {
            output_name: REQUIRED_RUN_OUTPUT_LOSS,
        });
    }

    let binary_path = std::env::current_exe().map_err(|source| HillslopeCliError::Io {
        path: PathBuf::from("<current_exe>"),
        source,
    })?;
    let binary_sidecar_path = write_release_sidecar_for_binary(&binary_path, BinaryRole::Hillslope)
        .map_err(|source| HillslopeCliError::ReleaseMetadata { source })?;

    let invoked_utc =
        utc_now_rfc3339().map_err(|detail| HillslopeCliError::TimeFormat { detail })?;

    let mut input_checksums = BTreeMap::new();
    let mut input_paths = vec![
        run_file_path.as_path(),
        soil_path.as_path(),
        management_path.as_path(),
        slope_path.as_path(),
        climate_path.as_path(),
    ];
    if let Some(path) = snow_input_path.as_ref() {
        input_paths.push(path.as_path());
    }
    if let Some(path) = frost_input_path.as_ref() {
        input_paths.push(path.as_path());
    }
    if let Some(path) = wepp_ui_input_path.as_ref() {
        input_paths.push(path.as_path());
    }
    if let Some(path) = pmetpara_input_path.as_ref() {
        input_paths.push(path.as_path());
    }
    for path in input_paths {
        input_checksums.insert(
            path.display().to_string(),
            sha256_file_hex(path).map_err(|source| HillslopeCliError::Io {
                path: path.to_path_buf(),
                source,
            })?,
        );
    }

    let mut output_checksum_entries = Vec::new();
    for path in std::iter::once(&output_pass)
        .chain(std::iter::once(&output_loss))
        .chain(optional_outputs.iter())
    {
        output_checksum_entries.push(OutputChecksumEntry::new(
            path.display().to_string(),
            sha256_file_hex(path).map_err(|source| HillslopeCliError::Io {
                path: path.clone(),
                source,
            })?,
        ));
    }

    let output_checksums =
        assemble_output_checksums(&output_checksum_entries).map_err(|error| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "manifest_output_checksums",
                detail: error.to_string(),
            }
        })?;

    let manifest_path = request.manifest_path.clone().unwrap_or_else(|| {
        request
            .output_dir
            .join("openwepp_hillslope_run_manifest.json")
    });

    let manifest = HillslopeRunManifest {
        schema: HILLSLOPE_RUN_MANIFEST_SCHEMA_ID.to_string(),
        engine: "openwepp".to_string(),
        binary_path: binary_path.display().to_string(),
        binary_sha256: sha256_file_hex(&binary_path).map_err(|source| HillslopeCliError::Io {
            path: binary_path.clone(),
            source,
        })?,
        binary_sidecar_path: binary_sidecar_path.display().to_string(),
        binary_sidecar_sha256: sha256_file_hex(&binary_sidecar_path).map_err(|source| {
            HillslopeCliError::Io {
                path: binary_sidecar_path.clone(),
                source,
            }
        })?,
        source_commit: git_source_commit_or_unknown(),
        invoked_utc,
        argv: argv.to_vec(),
        run_dir: request.run_dir.display().to_string(),
        run_file: run_file_path.display().to_string(),
        sidecar_policy: request.sidecar_policy.as_str().to_string(),
        sidecar_discovery_mode: sidecar_discovery_mode.to_string(),
        resolved_sidecars,
        input_checksums,
        output_checksums,
        mode_selection: wepp_ui_mode_selection,
        timestep_policy,
        adapter_boundary,
        execution_provenance,
        wb13_publication,
        mofe_hourly_carry,
        coupling_vectors,
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })?;
    fs::write(&manifest_path, manifest_json).map_err(|source| {
        HillslopeCliError::ManifestWrite {
            path: manifest_path.clone(),
            source,
        }
    })?;

    Ok(HillslopeRunReport {
        output_pass,
        output_loss,
        optional_outputs,
        manifest_path,
        sidecar_warnings,
    })
}

fn resolve_run_file(run_dir: &Path, run_file: &Path) -> PathBuf {
    if run_file.is_absolute() {
        run_file.to_path_buf()
    } else {
        run_dir.join(run_file)
    }
}
#[allow(clippy::too_many_lines)]
fn parse_runfile_execution_config(
    run_file_path: &Path,
    legacy_sidecar_discovery: bool,
) -> Result<RunfileExecutionConfig, HillslopeCliError> {
    let payload = fs::read_to_string(run_file_path).map_err(|source| HillslopeCliError::Io {
        path: run_file_path.to_path_buf(),
        source,
    })?;

    let runfile: HillslopeRunfileDocument =
        toml::from_str(&payload).map_err(|error| HillslopeCliError::ParseFailure {
            surface: "run_file",
            detail: format!("invalid TOML in {}: {error}", run_file_path.display()),
        })?;

    if runfile.schema != HILLSLOPE_RUNFILE_SCHEMA_ID {
        return Err(HillslopeCliError::ParseFailure {
            surface: "run_file",
            detail: format!(
                "unsupported schema '{}' (expected '{}')",
                runfile.schema, HILLSLOPE_RUNFILE_SCHEMA_ID
            ),
        });
    }

    if runfile.run_name.trim().is_empty() {
        return Err(HillslopeCliError::ParseFailure {
            surface: "run_file",
            detail: "missing required non-empty run_name".to_string(),
        });
    }

    if runfile.unit_system.trim() != "metric" {
        return Err(HillslopeCliError::ParseFailure {
            surface: "run_file",
            detail: format!(
                "unsupported unit_system '{}' (expected 'metric')",
                runfile.unit_system
            ),
        });
    }

    let soil_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.soil, "inputs.soil")?;
    let management_path = resolve_required_runfile_path(
        run_file_path,
        &runfile.inputs.management,
        "inputs.management",
    )?;
    let slope_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.slope, "inputs.slope")?;
    let climate_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.climate, "inputs.climate")?;

    for (field, path) in [
        ("inputs.soil", &soil_path),
        ("inputs.management", &management_path),
        ("inputs.slope", &slope_path),
        ("inputs.climate", &climate_path),
    ] {
        if !path.is_file() {
            return Err(HillslopeCliError::ParseFailure {
                surface: "run_file",
                detail: format!(
                    "required {field} path '{}' is not a readable file",
                    path.display()
                ),
            });
        }
    }

    let output_config = HillslopeOutputConfig {
        pass: resolve_required_runfile_path(run_file_path, &runfile.outputs.pass, "outputs.pass")?,
        loss: resolve_required_runfile_path(run_file_path, &runfile.outputs.loss, "outputs.loss")?,
        wat: resolve_optional_runfile_path(
            run_file_path,
            runfile.outputs.wat.as_deref(),
            "outputs.wat",
        )?,
        soil: resolve_optional_runfile_path(
            run_file_path,
            runfile.outputs.soil.as_deref(),
            "outputs.soil",
        )?,
        plot: resolve_optional_runfile_path(
            run_file_path,
            runfile.outputs.plot.as_deref(),
            "outputs.plot",
        )?,
        ebe: resolve_optional_runfile_path(
            run_file_path,
            runfile.outputs.ebe.as_deref(),
            "outputs.ebe",
        )?,
        element: resolve_optional_runfile_path(
            run_file_path,
            runfile.outputs.element.as_deref(),
            "outputs.element",
        )?,
    };
    validate_output_contract(&output_config).map_err(|error| HillslopeCliError::ParseFailure {
        surface: "run_file",
        detail: error.to_string(),
    })?;

    let pmetpara_path = resolve_optional_runfile_path(
        run_file_path,
        runfile.inputs.pmetpara.as_deref(),
        "inputs.pmetpara",
    )?;
    if !legacy_sidecar_discovery
        && let Some(path) = pmetpara_path.as_ref()
        && !path.is_file()
    {
        return Err(HillslopeCliError::ParseFailure {
            surface: "run_file",
            detail: format!(
                "optional inputs.pmetpara path '{}' is not a readable file",
                path.display()
            ),
        });
    }

    Ok(RunfileExecutionConfig {
        run_name: runfile.run_name,
        soil_path,
        management_path,
        slope_path,
        climate_path,
        output_config,
        sidecar_overrides: RunfileSidecarOverrides {
            wepp_ui: runfile.inputs.wepp_ui,
            pmetpara_path,
            snow: runfile.inputs.snow,
            frost: runfile.inputs.frost,
        },
    })
}

fn resolve_runfile_relative_path(run_file_path: &Path, candidate: &str) -> PathBuf {
    let candidate_path = PathBuf::from(candidate);
    if candidate_path.is_absolute() {
        candidate_path
    } else {
        run_file_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(candidate_path)
    }
}

fn resolve_required_runfile_path(
    run_file_path: &Path,
    candidate: &str,
    field: &'static str,
) -> Result<PathBuf, HillslopeCliError> {
    let trimmed = candidate.trim();
    if trimmed.is_empty() {
        return Err(HillslopeCliError::ParseFailure {
            surface: "run_file",
            detail: format!("missing required non-empty {field}"),
        });
    }

    Ok(resolve_runfile_relative_path(run_file_path, trimmed))
}

fn resolve_optional_runfile_path(
    run_file_path: &Path,
    candidate: Option<&str>,
    field: &'static str,
) -> Result<Option<PathBuf>, HillslopeCliError> {
    candidate.map_or(Ok(None), |value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(HillslopeCliError::ParseFailure {
                surface: "run_file",
                detail: format!("{field} cannot be an empty string"),
            })
        } else {
            Ok(Some(resolve_runfile_relative_path(run_file_path, trimmed)))
        }
    })
}

fn ensure_output_parent_directory(path: &Path) -> Result<(), HillslopeCliError> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent).map_err(|source| HillslopeCliError::OutputDirectoryCreate {
        path: parent.to_path_buf(),
        source,
    })
}

fn discover_sidecars(
    run_dir: &Path,
    excluded_file_names: &[String],
) -> Result<Vec<SidecarDiscovery>, HillslopeCliError> {
    let mut discoveries = Vec::new();
    let entries = fs::read_dir(run_dir).map_err(|source| HillslopeCliError::Io {
        path: run_dir.to_path_buf(),
        source,
    })?;

    for entry_result in entries {
        let entry = entry_result.map_err(|source| HillslopeCliError::Io {
            path: run_dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = file_name_string(&path);
        if excluded_file_names
            .iter()
            .any(|excluded| excluded == &file_name)
        {
            continue;
        }
        if path_has_extension_case_insensitive(&path, "hbp") {
            continue;
        }

        discoveries.push(SidecarDiscovery::new(file_name, path));
    }

    discoveries.sort_by(|left, right| left.file_name.cmp(&right.file_name));
    Ok(discoveries)
}

fn hillslope_sidecar_contracts(
    legacy_optional_core_sidecars: bool,
) -> Result<Vec<SidecarContract>, HillslopeCliError> {
    let core_sidecars = [
        ("frost", "frost.txt"),
        ("snow", "snow.txt"),
        ("wepp_ui", "wepp_ui.txt"),
        ("pmetpara", "pmetpara.txt"),
    ];

    let optional = [
        ("irrigation_depletion", "irrigation_depletion.txt"),
        ("irrigation_fixeddate", "irrigation_fixeddate.ifd"),
        ("gwcoeff", "gwcoeff.txt"),
        ("phosphorus", "phosphorus.txt"),
        ("tc", "tc.txt"),
        ("tcr", "tcr.txt"),
        ("lcwb", "lcwb.txt"),
        ("chaninp", "chan.inp"),
    ];

    let mut contracts = Vec::new();
    for (id, file_name) in core_sidecars {
        let requirement = if legacy_optional_core_sidecars {
            SidecarRequirement::Optional
        } else {
            SidecarRequirement::Required
        };
        contracts.push(build_sidecar_contract(id, file_name, requirement)?);
    }
    for (id, file_name) in optional {
        contracts.push(build_sidecar_contract(
            id,
            file_name,
            SidecarRequirement::Optional,
        )?);
    }

    Ok(contracts)
}

fn build_sidecar_contract(
    id: &'static str,
    file_name: &'static str,
    requirement: SidecarRequirement,
) -> Result<SidecarContract, HillslopeCliError> {
    let sidecar_id =
        SidecarId::new(id).map_err(|error| HillslopeCliError::SidecarContractInvalid {
            detail: error.to_string(),
        })?;

    Ok(SidecarContract::new(
        sidecar_id,
        file_name,
        Vec::new(),
        requirement,
    ))
}

fn optional_sidecar_binding_path(
    bindings: &[SidecarBinding],
    sidecar_id: &'static str,
) -> Option<PathBuf> {
    bindings
        .iter()
        .find(|binding| binding.sidecar_id.as_str() == sidecar_id)
        .map(|binding| binding.resolved_path.clone())
}

fn merge_runtime_surfaces(
    mut base: HillslopeWritebackSurface,
    overlay: HillslopeWritebackSurface,
) -> HillslopeWritebackSurface {
    base.state_surface.extend(overlay.state_surface);
    base.flux_surface.extend(overlay.flux_surface);
    base
}

fn absent_pmetpara_file() -> PmetparaFile {
    PmetparaFile {
        sidecar_present: false,
        iflget: 1,
        record_count: 0,
        line_count_closed: true,
        records: Vec::new(),
        warnings: Vec::new(),
        lookup: PmetLookupState {
            fallback_first_row_used: false,
        },
    }
}

fn build_hillslope_runtime_surface_from_pmetpara(
    management: &ManagementParseOutput,
    pmetpara: &mut PmetparaFile,
    mode: PmetparaParseMode,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    let mut surface = HillslopeWritebackSurface::default();
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.mode.sidecar_present"),
        BoundaryValue::scalar(if pmetpara.sidecar_present { 1.0 } else { 0.0 }),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.mode.iflget"),
        BoundaryValue::scalar(f64::from(pmetpara.iflget)),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.record_count"),
        BoundaryValue::scalar(usize_to_scalar(
            "pmetpara.record_count",
            pmetpara.record_count,
        )?),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.line_count_closed"),
        BoundaryValue::scalar(if pmetpara.line_count_closed { 1.0 } else { 0.0 }),
    );

    if !pmetpara.sidecar_present {
        return Ok(surface);
    }

    let active_crop_name = active_management_crop_name(management)?;
    let (kcb, rawp, line_index) = {
        let record = pmetpara
            .lookup_record(active_crop_name, mode)
            .map_err(|error| HillslopeCliError::ParseFailure {
                surface: "pmetpara",
                detail: error.to_string(),
            })?;
        (record.kcb, record.rawp, record.line_index)
    };

    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.selected.kcb"),
        BoundaryValue::scalar(kcb),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.selected.rawp"),
        BoundaryValue::scalar(rawp),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.selected.line_index"),
        BoundaryValue::scalar(f64::from(line_index)),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.lookup.fallback_first_row_used"),
        BoundaryValue::scalar(if pmetpara.lookup.fallback_first_row_used {
            1.0
        } else {
            0.0
        }),
    );

    Ok(surface)
}

fn active_management_crop_name(
    management: &ManagementParseOutput,
) -> Result<&str, HillslopeCliError> {
    let first_slot = management.schedule.slots.first().ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} management schedule has no slot for PMET crop lookup"
            ),
        }
    })?;
    let yearly_ref = first_slot.yearly_refs.first().copied().ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} management schedule slot has no yearly ref for PMET crop lookup"
            ),
        }
    })?;
    if yearly_ref == 0 || yearly_ref > management.registries.yearlies.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} yearly ref {yearly_ref} out of range for PMET crop lookup"
            ),
        });
    }

    let yearly = &management.registries.yearlies[yearly_ref - 1];
    let YearlyScenarioData::Cropland(cropland) = &yearly.data;
    if cropland.itype == 0 || cropland.itype > management.registries.plants.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} plant ref {} out of range for PMET crop lookup",
                cropland.itype
            ),
        });
    }

    Ok(management.registries.plants[cropland.itype - 1]
        .meta
        .name
        .as_str())
}

#[derive(Debug, Clone, Copy)]
struct Wb11EtDemandSeed {
    demand_m: f64,
    branch_evappm: bool,
    diagnostics: Option<EvappmDemandDiagnostics>,
}

#[derive(Debug, Clone, Copy)]
struct EvappmDemandDiagnostics {
    etorc_mm: f64,
    rn_mj_m2: f64,
    fwv_m_s: f64,
    rhd_pct: f64,
    kcbadj: f64,
    kcbcon: f64,
    etke: f64,
    etkr: f64,
    etks: f64,
    tew_mm: f64,
    rew_mm: f64,
    wfevp_mm: f64,
    taw_mm: f64,
    raw_mm: f64,
    wftrp_mm: f64,
    es_m: f64,
    es_storage_return_m: f64,
    ep_m: f64,
}

fn wb11_seed_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "wb11_seed",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}

fn compute_wb11_et_demand_seed(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    let iflget =
        runtime_surface_symbol_value(runtime_surface, "pmetpara.mode.iflget").unwrap_or(1.0);
    if !iflget.is_finite() {
        return Err(wb11_seed_failure(format!(
            "pmetpara.mode.iflget must be finite when present, observed {iflget}"
        )));
    }
    if (iflget - 1.0).abs() <= 1.0e-12 {
        return compute_priestley_taylor_wb11_et_demand(runtime_surface);
    }
    compute_evappm_wb11_et_demand(runtime_surface)
}

fn compute_priestley_taylor_wb11_et_demand(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    let tmax = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let tmin = require_runtime_surface_scalar(runtime_surface, "tmin")?;
    let rad = require_runtime_surface_scalar(runtime_surface, "rad")?;
    if rad < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rad must be >= 0.0, observed {rad}"
        )));
    }
    let salb = require_runtime_surface_scalar(runtime_surface, "salb")?;
    if !(0.0..=1.0).contains(&salb) {
        return Err(wb11_seed_failure(format!(
            "salb must be within [0,1], observed {salb}"
        )));
    }
    let cancov = require_runtime_surface_scalar(runtime_surface, "cancov")?;
    if cancov < 0.0 {
        return Err(wb11_seed_failure(format!(
            "cancov must be >= 0.0, observed {cancov}"
        )));
    }
    let lai = require_runtime_surface_scalar(runtime_surface, "lai")?;
    if lai < 0.0 {
        return Err(wb11_seed_failure(format!(
            "lai must be >= 0.0, observed {lai}"
        )));
    }

    let tave = 0.5 * (tmax + tmin);
    let tk = tave + 273.0;
    if tk <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived tk must be > 0.0, observed {tk}"
        )));
    }
    let delta = (21.255 - 5304.0 / tk).exp() * 5304.0 / (tk * tk);
    let gamma = delta / (delta + 0.68);
    let eaj = (-0.5 * (cancov + 0.1)).exp();
    let alb = if lai > 0.0 {
        0.23 * (1.0 - eaj) + salb * eaj
    } else {
        salb
    };
    let demand_m = (0.00128 * ((rad * (1.0 - alb)) / 58.3) * gamma).max(0.0);
    if !demand_m.is_finite() {
        return Err(wb11_seed_failure(format!(
            "derived wb11_et_demand is non-finite ({demand_m})"
        )));
    }

    Ok(Wb11EtDemandSeed {
        demand_m,
        branch_evappm: false,
        diagnostics: None,
    })
}

#[allow(clippy::manual_midpoint, clippy::similar_names, clippy::too_many_lines)]
fn compute_evappm_wb11_et_demand(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    let tmax = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let tmin = require_runtime_surface_scalar(runtime_surface, "tmin")?;
    let tdpt = require_runtime_surface_scalar(runtime_surface, "tdpt")?;
    let rad = require_runtime_surface_scalar(runtime_surface, "rad")?;
    if rad < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rad must be >= 0.0, observed {rad}"
        )));
    }
    let radpot = evappm_radpot_ly(runtime_surface)?;
    if radpot <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "radpot must be > 0.0 for EVAPPM demand, observed {radpot}"
        )));
    }
    let vwind = require_runtime_surface_scalar(runtime_surface, "vwind")?;
    if vwind < 0.0 {
        return Err(wb11_seed_failure(format!(
            "vwind must be >= 0.0 for EVAPPM demand, observed {vwind}"
        )));
    }
    let elevm = require_runtime_surface_scalar(runtime_surface, "elevm")?;
    if elevm >= 45_076.923_076_923_08 {
        return Err(wb11_seed_failure(format!(
            "elevm keeps legacy pressure base positive, observed {elevm}"
        )));
    }
    let kcb = require_runtime_surface_scalar(runtime_surface, "pmetpara.selected.kcb")?;
    let rawp = require_runtime_surface_scalar(runtime_surface, "pmetpara.selected.rawp")?;
    let lai = require_runtime_surface_scalar(runtime_surface, "lai")?;
    if lai < 0.0 {
        return Err(wb11_seed_failure(format!(
            "lai must be >= 0.0, observed {lai}"
        )));
    }
    let canhgt = require_runtime_surface_scalar(runtime_surface, "canhgt")?;
    if canhgt < 0.0 {
        return Err(wb11_seed_failure(format!(
            "canhgt must be >= 0.0, observed {canhgt}"
        )));
    }
    let rtd = require_runtime_surface_scalar(runtime_surface, "rtd")?;
    if rtd < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rtd must be >= 0.0, observed {rtd}"
        )));
    }
    let cancov = require_runtime_surface_scalar(runtime_surface, "cancov")?;
    if cancov < 0.0 {
        return Err(wb11_seed_failure(format!(
            "cancov must be >= 0.0, observed {cancov}"
        )));
    }
    let residue_interception =
        require_runtime_surface_scalar(runtime_surface, "wb17_residue_interception")?;
    if residue_interception < 0.0 {
        return Err(wb11_seed_failure(format!(
            "wb17_residue_interception must be >= 0.0, observed {residue_interception}"
        )));
    }

    let tave = 0.5 * (tmax + tmin);
    let ed = saturation_vapor_pressure_kpa(tdpt);
    let emaxt = saturation_vapor_pressure_kpa(tmax);
    let emint = saturation_vapor_pressure_kpa(tmin);
    let ee = 0.5 * (emaxt + emint);
    if emaxt <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived emaxt must be > 0.0 for EVAPPM demand, observed {emaxt}"
        )));
    }
    let ra = rad / 23.9;
    let rso = radpot / 23.9;
    if rso <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived rso must be > 0.0 for EVAPPM demand, observed {rso}"
        )));
    }
    let rbo = (0.34 - 0.14 * ed.sqrt())
        * 4.9e-9
        * (((tmax + 273.2).powi(4) + (tmin + 273.2).powi(4)) / 2.0)
        * (1.35 * (ra / rso) - 0.35);
    let rn_mj_m2 = ra * 0.77 - rbo;
    let fwv_m_s = vwind * 4.87 / (67.8_f64.mul_add(10.0, -5.42)).ln();
    let dlt = 4098.0 / ((tave + 237.3) * (tave + 237.3)) * saturation_vapor_pressure_kpa(tave);
    let pressure_base = 1.0 - 0.0065 * elevm / 293.0;
    if pressure_base <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "legacy pressure base must be > 0.0 for EVAPPM demand, observed {pressure_base}"
        )));
    }
    let pb = 101.3 * pressure_base.powf(5.26);
    let gma = 0.000_665 * pb;
    let denominator = dlt + gma * (1.0 + 0.34 * fwv_m_s);
    if denominator <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "EVAPPM etorc denominator must be > 0.0, observed {denominator}"
        )));
    }
    let etorc_mm = (0.408 * dlt * rn_mj_m2 + gma * (900.0 / (tave + 273.0)) * (ee - ed) * fwv_m_s)
        / denominator;
    let rhd_pct = ed / emaxt * 100.0;
    let height_factor = (canhgt / 3.0).powf(0.3);
    let kcbadj = if lai > 0.0 && rtd > 0.0 {
        kcb + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor
    } else {
        0.0
    };
    let kcbcon = kcbadj * (1.0 - (-0.45 * lai).exp());
    let etke = if kcbadj > 0.0 {
        kcbadj * (-0.45 * lai).exp()
    } else {
        1.2
    };

    let nsl = scalar_to_usize(
        "wb11_nsl",
        runtime_surface_symbol_value(runtime_surface, "wb11_nsl")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "nsl"))
            .ok_or_else(|| wb11_seed_failure("missing required runtime symbol wb11_nsl/nsl"))?,
    )?;
    let mut profile_depth_m = 0.0_f64;
    for layer_index in 1..=nsl {
        profile_depth_m += require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_dg")?;
    }
    if profile_depth_m <= 0.0 {
        return Err(wb11_seed_failure(
            "soil profile depth must be > 0.0 for EVAPPM demand",
        ));
    }

    let epdp_m = 0.1_f64.min(profile_depth_m);
    let mut tew_mm = 0.0_f64;
    let mut rew_mm = 0.0_f64;
    let mut wfevp_mm = 0.0_f64;
    let mut cumulative_depth_m = 0.0_f64;
    for layer_index in 1..=nsl {
        let dg = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_dg")?;
        let solthk = runtime_surface_symbol_value(
            runtime_surface,
            format!("wb19_solthk_{layer_index:04}").as_str(),
        )
        .unwrap_or(cumulative_depth_m + dg);
        if solthk <= cumulative_depth_m {
            return Err(wb11_seed_failure(format!(
                "wb19_solthk_{layer_index:04} must increase with depth for EVAPPM demand"
            )));
        }
        let thetfc = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_thetfc")?;
        let thetdr = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_thetdr")?;
        let theta_store =
            require_evappm_layer_scalar(runtime_surface, layer_index, "wb18_perc_theta")?;
        if thetdr > thetfc {
            return Err(wb11_seed_failure(format!(
                "wb19_thetdr_{layer_index:04} must be <= wb19_thetfc_{layer_index:04}"
            )));
        }
        let layer_bottom_m = solthk;
        let layer_fraction = if layer_bottom_m <= epdp_m {
            1.0
        } else if cumulative_depth_m < epdp_m {
            (epdp_m - cumulative_depth_m) / (layer_bottom_m - cumulative_depth_m)
        } else {
            0.0
        };
        if layer_fraction > 0.0 {
            tew_mm += (thetfc - 0.5 * thetdr) * dg * 1_000.0 * layer_fraction;
            rew_mm += (thetfc - thetdr) * dg * 1_000.0 / 3.0 * layer_fraction;
            wfevp_mm += theta_store * 1_000.0 * layer_fraction;
        }
        cumulative_depth_m = layer_bottom_m;
        if cumulative_depth_m >= epdp_m {
            break;
        }
    }
    let wfevp_mm = wfevp_mm + residue_interception * 1_000.0;
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

    let tpdp_m = rtd.min(profile_depth_m);
    let mut taw_mm = 0.0_f64;
    let mut wftrp_mm = 0.0_f64;
    let mut cumulative_depth_m = 0.0_f64;
    for layer_index in 1..=nsl {
        let dg = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_dg")?;
        let solthk = runtime_surface_symbol_value(
            runtime_surface,
            format!("wb19_solthk_{layer_index:04}").as_str(),
        )
        .unwrap_or(cumulative_depth_m + dg);
        let thetfc = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_thetfc")?;
        let thetdr = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_thetdr")?;
        let theta_store =
            require_evappm_layer_scalar(runtime_surface, layer_index, "wb18_perc_theta")?;
        let layer_bottom_m = solthk;
        if tpdp_m <= 0.0 {
            break;
        }
        if layer_bottom_m <= tpdp_m {
            taw_mm += (thetfc - thetdr) * dg * 1_000.0;
            wftrp_mm += theta_store * 1_000.0;
        } else if cumulative_depth_m < tpdp_m {
            let layer_span_m = layer_bottom_m - cumulative_depth_m;
            if layer_span_m <= 0.0 {
                return Err(wb11_seed_failure(format!(
                    "wb19_solthk_{layer_index:04} must increase with depth for EVAPPM demand"
                )));
            }
            let fraction = (tpdp_m - cumulative_depth_m) / layer_span_m;
            taw_mm += (thetfc - thetdr) * dg * 1_000.0 * fraction;
            wftrp_mm = wfevp_mm + theta_store * 1_000.0 * fraction;
            break;
        }
        cumulative_depth_m = layer_bottom_m;
        if cumulative_depth_m >= tpdp_m {
            break;
        }
    }

    let etcsc = kcbadj * etorc_mm;
    let rawpaj = rawp + 0.04 * (5.0 - etcsc);
    let raw_mm = rawpaj * taw_mm;
    let etksden = taw_mm - raw_mm;
    let etks = if etksden <= 0.0 || (taw_mm - wftrp_mm) <= raw_mm {
        1.0
    } else {
        wftrp_mm / etksden
    };
    let potes_m = etorc_mm * etke * 0.001;
    let es_raw_m = if potes_m > residue_interception {
        let bpotes_m = potes_m - residue_interception;
        let eaj = (-0.5 * (cancov + 0.1)).exp();
        let kcmax = 1.2 + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor;
        let kecon = (etke * etkr).min(eaj * kcmax);
        kecon * bpotes_m / etke + residue_interception
    } else {
        potes_m
    };
    let es_storage_return_m = if es_raw_m < 0.0 { -es_raw_m } else { 0.0 };
    let es_m = if es_raw_m < 0.0 { 0.0 } else { es_raw_m };
    let ep_raw_m = etorc_mm * etks * kcbcon * 0.001;
    let ep_m = if ep_raw_m < 0.0 { 0.0 } else { ep_raw_m };

    let diagnostics = EvappmDemandDiagnostics {
        etorc_mm,
        rn_mj_m2,
        fwv_m_s,
        rhd_pct,
        kcbadj,
        kcbcon,
        etke,
        etkr,
        etks,
        tew_mm,
        rew_mm,
        wfevp_mm,
        taw_mm,
        raw_mm,
        wftrp_mm,
        es_m,
        es_storage_return_m,
        ep_m,
    };
    for (name, value) in [
        ("pmet.etorc_mm", diagnostics.etorc_mm),
        ("pmet.rn_mj_m2", diagnostics.rn_mj_m2),
        ("pmet.fwv_m_s", diagnostics.fwv_m_s),
        ("pmet.rhd_pct", diagnostics.rhd_pct),
        ("pmet.kcbadj", diagnostics.kcbadj),
        ("pmet.kcbcon", diagnostics.kcbcon),
        ("pmet.etke", diagnostics.etke),
        ("pmet.etkr", diagnostics.etkr),
        ("pmet.etks", diagnostics.etks),
        ("pmet.tew_mm", diagnostics.tew_mm),
        ("pmet.rew_mm", diagnostics.rew_mm),
        ("pmet.wfevp_mm", diagnostics.wfevp_mm),
        ("pmet.taw_mm", diagnostics.taw_mm),
        ("pmet.raw_mm", diagnostics.raw_mm),
        ("pmet.wftrp_mm", diagnostics.wftrp_mm),
        ("pmet.es_m", diagnostics.es_m),
        ("pmet.es_storage_return_m", diagnostics.es_storage_return_m),
        ("pmet.ep_m", diagnostics.ep_m),
    ] {
        if !value.is_finite() {
            return Err(wb11_seed_failure(format!(
                "derived {name} must be finite, observed {value}"
            )));
        }
    }

    Ok(Wb11EtDemandSeed {
        demand_m: ep_m,
        branch_evappm: true,
        diagnostics: Some(diagnostics),
    })
}

fn saturation_vapor_pressure_kpa(temperature_c: f64) -> f64 {
    0.6108 * (17.27 * temperature_c / (temperature_c + 237.3)).exp()
}

fn require_evappm_layer_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    layer_index: usize,
    root: &str,
) -> Result<f64, HillslopeCliError> {
    let symbol = wb13_primary_layer_symbol(root, layer_index);
    let value = require_runtime_surface_scalar(runtime_surface, symbol.as_str())?;
    if !value.is_finite() {
        return Err(wb11_seed_failure(format!(
            "{symbol} must be finite for EVAPPM demand, observed {value}"
        )));
    }
    Ok(value)
}

fn evappm_radpot_ly(runtime_surface: &HillslopeWritebackSurface) -> Result<f64, HillslopeCliError> {
    if let Some(radpot) = runtime_surface_symbol_value(runtime_surface, "radpot") {
        if !radpot.is_finite() {
            return Err(wb11_seed_failure(format!(
                "radpot must be finite when present, observed {radpot}"
            )));
        }
        return Ok(radpot);
    }

    let deglat = require_runtime_surface_scalar(runtime_surface, "deglat")?;
    let year = require_runtime_surface_scalar(runtime_surface, "year")?;
    let mon = require_runtime_surface_scalar(runtime_surface, "mon")?;
    let day = require_runtime_surface_scalar(runtime_surface, "day")?;
    let year = scalar_to_i32("year", year)?;
    let mon = scalar_to_i32("mon", mon)?;
    let day = scalar_to_i32("day", day)?;
    let sdate = f64::from(day_of_year(year, mon, day)?);
    Ok(legacy_sunmap_horizontal_radpot_ly(deglat, sdate))
}

fn legacy_sunmap_horizontal_radpot_ly(deglat: f64, sdate: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let radlat = deglat * pi / 180.0;
    let declination = 0.00698 - 0.4067 * ((sdate + 10.0) * 0.0172).cos();
    let earth_sun_distance_factor = 1.0 - 0.0167 * ((sdate - 3.0) * 0.0172).cos();
    let radiation_factor = (60.0 * 1.94) / (earth_sun_distance_factor * earth_sun_distance_factor);
    let sunset_argument = -(radlat.tan() * declination.tan()).clamp(-1.0, 1.0);
    let sunset_angle = sunset_argument.acos();
    radiation_factor
        * ((declination.sin() * radlat.sin() * (sunset_angle - -sunset_angle) * 12.0 / pi)
            + (declination.cos()
                * radlat.cos()
                * (sunset_angle.sin() - (-sunset_angle).sin())
                * 12.0
                / pi))
}

fn publish_wb11_et_demand_seed(
    runtime_surface: &mut HillslopeWritebackSurface,
    seed: Wb11EtDemandSeed,
) -> Result<(), HillslopeCliError> {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(seed.demand_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_priestley_taylor"),
        BoundaryValue::scalar(if seed.branch_evappm { 0.0 } else { 1.0 }),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(if seed.branch_evappm { 1.0 } else { 0.0 }),
    );
    if let Some(diagnostics) = seed.diagnostics {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("pmet.es_storage_return_m"),
            BoundaryValue::water_depth_meters(diagnostics.es_storage_return_m).map_err(|error| {
                wb11_seed_failure(format!(
                    "pmet.es_storage_return_m must be a non-negative finite water depth: {error}"
                ))
            })?,
        );
        for (symbol, value) in [
            ("pmet.etorc_mm", diagnostics.etorc_mm),
            ("pmet.rn_mj_m2", diagnostics.rn_mj_m2),
            ("pmet.fwv_m_s", diagnostics.fwv_m_s),
            ("pmet.rhd_pct", diagnostics.rhd_pct),
            ("pmet.kcbadj", diagnostics.kcbadj),
            ("pmet.kcbcon", diagnostics.kcbcon),
            ("pmet.etke", diagnostics.etke),
            ("pmet.etkr", diagnostics.etkr),
            ("pmet.etks", diagnostics.etks),
            ("pmet.tew_mm", diagnostics.tew_mm),
            ("pmet.rew_mm", diagnostics.rew_mm),
            ("pmet.wfevp_mm", diagnostics.wfevp_mm),
            ("pmet.taw_mm", diagnostics.taw_mm),
            ("pmet.raw_mm", diagnostics.raw_mm),
            ("pmet.wftrp_mm", diagnostics.wftrp_mm),
            ("pmet.es_m", diagnostics.es_m),
            ("pmet.ep_m", diagnostics.ep_m),
        ] {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }
    }
    Ok(())
}

fn validate_hillslope_ofe_topology_parity(
    slope_ofe_count: usize,
    management_topology_count: usize,
    soil_topology_count: usize,
) -> Result<(), HillslopeCliError> {
    if slope_ofe_count == management_topology_count && slope_ofe_count == soil_topology_count {
        return Ok(());
    }

    Err(HillslopeCliError::OfeTopologyMismatch {
        slope_ofe_count,
        management_topology_count,
        soil_topology_count,
    })
}

fn build_mode_selection_provenance(
    wepp_ui: &WeppUiParseResult,
) -> Result<HillslopeModeSelectionProvenance, HillslopeCliError> {
    if !matches!(wepp_ui.ui_run_requested, 0 | 1) {
        return Err(mode_selection_failure(format!(
            "requested ui_run must be in {{0,1}}, observed {}",
            wepp_ui.ui_run_requested
        )));
    }
    if !matches!(wepp_ui.ui_run, 0 | 1) {
        return Err(mode_selection_failure(format!(
            "effective ui_run must be in {{0,1}}, observed {}",
            wepp_ui.ui_run
        )));
    }

    let expected_divergence = wepp_ui.ui_run_requested != wepp_ui.ui_run;
    if wepp_ui.mode_divergence != expected_divergence {
        return Err(mode_selection_failure(format!(
            "mode_divergence mismatch: expected {} from requested/effective tuple ({}, {}), observed {}",
            expected_divergence, wepp_ui.ui_run_requested, wepp_ui.ui_run, wepp_ui.mode_divergence
        )));
    }

    let selected_lane = lane_name_from_effective_ui_run(wepp_ui.ui_run)?;

    Ok(HillslopeModeSelectionProvenance {
        wepp_ui: WeppUiModeSelectionProvenance {
            requested: wepp_ui.ui_run_requested,
            effective: wepp_ui.ui_run,
            selected_lane: selected_lane.to_string(),
            mode_divergence: wepp_ui.mode_divergence,
            guard_id: WUI_MODE_GUARD_ID.to_string(),
        },
    })
}

fn lane_name_from_effective_ui_run(
    effective_ui_run: i32,
) -> Result<&'static str, HillslopeCliError> {
    match effective_ui_run {
        0 => Ok(DAILY_EXECUTION_LANE),
        1 => Ok(HOURLY_EXECUTION_LANE),
        _ => Err(mode_selection_failure(format!(
            "effective ui_run must map to daily/hourly lane, observed {effective_ui_run}"
        ))),
    }
}

fn mode_name_from_ui_run(ui_run: i32) -> Result<&'static str, HillslopeCliError> {
    match ui_run {
        0 => Ok(DAILY_EXECUTION_LANE),
        1 => Ok(HOURLY_EXECUTION_LANE),
        _ => Err(timestep_policy_failure(format!(
            "ui_run must map to daily/hourly mode, observed {ui_run}"
        ))),
    }
}

fn build_execution_lane_context(
    mode_selection: &HillslopeModeSelectionProvenance,
) -> Result<ExecutionLaneContext, HillslopeCliError> {
    let requested_mode = mode_name_from_ui_run(mode_selection.wepp_ui.requested)?;
    let effective_mode = mode_name_from_ui_run(mode_selection.wepp_ui.effective)?;
    let lane = ExecutionLane::parse(mode_selection.wepp_ui.selected_lane.as_str())?;
    if lane.as_str() != effective_mode {
        return Err(timestep_policy_failure(format!(
            "selected lane '{}' must match effective mode '{effective_mode}'",
            lane.as_str()
        )));
    }

    Ok(ExecutionLaneContext {
        lane,
        requested_mode,
        effective_mode,
        timestep_policy: TimestepPolicy::from_lane(lane),
    })
}

fn build_timestep_policy_provenance(
    lane_context: &ExecutionLaneContext,
) -> HillslopeTimestepPolicyProvenance {
    let subhourly_scaffold = TimestepPolicy::scaffold_subhourly(900);
    HillslopeTimestepPolicyProvenance {
        scheduler_mode: lane_context.timestep_policy.scheduler_mode().to_string(),
        requested_mode: lane_context.requested_mode.to_string(),
        effective_mode: lane_context.effective_mode.to_string(),
        selected_lane: lane_context.lane.as_str().to_string(),
        policy: lane_context.timestep_policy.policy_name().to_string(),
        timestep_seconds: lane_context.timestep_policy.timestep_seconds(),
        physics_enabled: lane_context.timestep_policy.physics_enabled(),
        subhourly_scaffold_available: !subhourly_scaffold.physics_enabled(),
        guard_id: SIMMODE_TIMESTEP_GUARD_ID.to_string(),
    }
}

fn build_adapter_boundary_provenance(
    lane_context: &ExecutionLaneContext,
) -> Result<HillslopeAdapterBoundaryProvenance, HillslopeCliError> {
    let reject_surfaces_excluded = true;
    let defer_surfaces_excluded = true;
    if !reject_surfaces_excluded || !defer_surfaces_excluded {
        return Err(simcons_intake_failure(
            "SIMIMPL09 requires reject/defer intake surfaces to remain excluded",
        ));
    }

    Ok(HillslopeAdapterBoundaryProvenance {
        selected_lane: lane_context.lane.as_str().to_string(),
        scheduler_mode: lane_context.timestep_policy.scheduler_mode().to_string(),
        requested_mode: lane_context.requested_mode.to_string(),
        effective_mode: lane_context.effective_mode.to_string(),
        adopt_profile: SIMIMPL09_ADOPT_PROFILE.to_string(),
        reject_surfaces_excluded,
        defer_surfaces_excluded,
        guard_id: SIMCONS_INTAKE_GUARD_ID.to_string(),
    })
}
