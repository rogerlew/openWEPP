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
    snow_hourly_rain_sum_m: Option<f64>,
    snow_hourly_rain_retained_sum_m: Option<f64>,
    snow_hourly_rain_released_sum_m: Option<f64>,
    snow_hourly_snowfall_depth_sum_m: Option<f64>,
    snow_hourly_snowfall_water_equiv_sum_m: Option<f64>,
    snow_hourly_melt_raw_sum_m: Option<f64>,
    snow_hourly_melt_sum_m: Option<f64>,
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
    "openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v13";
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

#[allow(clippy::too_many_lines)]
fn seed_wb11_runtime_surface_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
    execution_lane: ExecutionLane,
) -> Result<(), HillslopeCliError> {
    const WB11_STATE_SEED_COMPLETED_SYMBOL: &str = "wb11_state_seed_completed";
    const WB18_PERC_LANE_SUBSTEPS_SYMBOL: &str = "wb18_perc_lane_substeps";
    const WB19_LATERAL_DRAIN_LANE_SUBSTEPS_SYMBOL: &str = "wb19_lateral_drain_lane_substeps";

    let nsl_symbol = if runtime_surface_symbol_value(runtime_surface, "wb11_nsl").is_some() {
        "wb11_nsl"
    } else {
        "nsl"
    };
    let nsl = scalar_to_usize(
        nsl_symbol,
        require_runtime_surface_scalar(runtime_surface, nsl_symbol)?,
    )?;
    if nsl == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} nsl must be >= 1 for WB11 seeding"),
        });
    }

    let wb18_perc_lane_substeps = match execution_lane {
        ExecutionLane::Daily => 1.0,
        ExecutionLane::Hourly => 24.0,
    };
    let contributor_ofe_count = runtime_surface_ofe_count(runtime_surface)?;
    let mofe_hourly_carry_active = contributor_ofe_count > 1;
    let wb18_perc_lane_substeps = if mofe_hourly_carry_active {
        24.0
    } else {
        wb18_perc_lane_substeps
    };
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB18_PERC_LANE_SUBSTEPS_SYMBOL),
        BoundaryValue::scalar(wb18_perc_lane_substeps),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB19_LATERAL_DRAIN_LANE_SUBSTEPS_SYMBOL),
        BoundaryValue::scalar(wb18_perc_lane_substeps),
    );
    seed_mofe_hourly_carry_runtime_surface_inputs(runtime_surface, mofe_hourly_carry_active)?;

    let prcp = require_runtime_surface_scalar(runtime_surface, "prcp")?;
    if prcp < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} prcp must be >= 0.0, observed {prcp}"),
        });
    }
    let breakpoint_mode =
        runtime_surface_symbol_value(runtime_surface, "ibrkpt").is_some_and(|value| value >= 0.5);
    let hyetograph_point_symbol =
        if breakpoint_mode && runtime_surface_symbol_value(runtime_surface, "nbrkpt").is_some() {
            // Breakpoint climates are authoritative on `nbrkpt`; stale `ninten`
            // from prior days must not truncate the current-day event shape.
            "nbrkpt"
        } else if runtime_surface_symbol_value(runtime_surface, "ninten").is_some() {
            "ninten"
        } else {
            "nbrkpt"
        };
    let mut ninten = scalar_to_usize(
        hyetograph_point_symbol,
        require_runtime_surface_scalar(runtime_surface, hyetograph_point_symbol)?,
    )?;
    if ninten == 0 {
        let stmdur = runtime_surface_symbol_value(runtime_surface, "stmdur")
            .unwrap_or(1.0)
            .max(1.0);
        let intensity = if stmdur > 0.0 { prcp / stmdur } else { prcp };
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(2.0));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("timem_0001"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("timem_0002"),
            BoundaryValue::scalar(stmdur),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("intsty_0001"),
            BoundaryValue::scalar(intensity.max(0.0)),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("intsty_0002"),
            BoundaryValue::scalar(0.0),
        );
        ninten = 2;
    }
    let ninten_scalar = usize_to_scalar("ninten", ninten)?;
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ninten"),
        BoundaryValue::scalar(ninten_scalar),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("nbrkpt"),
        BoundaryValue::scalar(ninten_scalar),
    );

    let mut hyetograph_rainfall = 0.0_f64;
    for point_index in 1..ninten {
        let time_symbol = wb13_primary_layer_symbol("timem", point_index);
        let next_time_symbol = wb13_primary_layer_symbol("timem", point_index + 1);
        let intensity_symbol = wb13_primary_layer_symbol("intsty", point_index);

        let time_s = require_runtime_surface_scalar(runtime_surface, time_symbol.as_str())?;
        let next_time_s =
            require_runtime_surface_scalar(runtime_surface, next_time_symbol.as_str())?;
        let intensity = require_runtime_surface_scalar(runtime_surface, intensity_symbol.as_str())?;

        if next_time_s < time_s {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} {next_time_symbol} ({next_time_s}) must be >= {time_symbol} ({time_s})"
                ),
            });
        }
        if intensity < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} {intensity_symbol} must be >= 0.0, observed {intensity}"
                ),
            });
        }

        hyetograph_rainfall += intensity * (next_time_s - time_s);
    }

    let wb11_state_seeded = runtime_surface
        .state_surface
        .get(&BoundarySymbol::from(WB11_STATE_SEED_COMPLETED_SYMBOL))
        .copied()
        .map(BoundaryValue::as_f64)
        .is_some_and(|value| value >= 0.5)
        || runtime_surface_symbol_value(runtime_surface, "wb18_perc_theta_0001").is_some();
    if !wb11_state_seeded {
        let mut wb11_soil_water = 0.0_f64;
        let mut wb11_field_capacity = 0.0_f64;
        let mut wb11_drainable_storage = 0.0_f64;
        let mut wb11_drainage_coefficient = 0.0_f64;
        let mut sat = require_runtime_surface_scalar(runtime_surface, "sat")?;
        if sat < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!("{SIMPIPE_GUARD_ID} sat must be >= 0.0, observed {sat}"),
            });
        }
        let sat_cap = match execution_lane {
            ExecutionLane::Daily => 0.95,
            ExecutionLane::Hourly => 1.0,
        };
        if sat > sat_cap {
            sat = sat_cap;
        }

        for layer_index in 1..=nsl {
            let dg_symbol = format!("wb19_dg_{layer_index:04}");
            let fc_symbol = format!("wb19_thetfc_{layer_index:04}");
            let wp_symbol = format!("wb19_thetdr_{layer_index:04}");
            let ssc_symbol = wb13_primary_layer_symbol("ssc", layer_index);
            let por_symbol = format!("wb19_por_{layer_index:04}");
            let cpm_symbol = wb13_primary_layer_symbol("cpm", layer_index);

            let dg = require_runtime_surface_scalar(runtime_surface, dg_symbol.as_str())?;
            let thetfc = require_runtime_surface_scalar(runtime_surface, fc_symbol.as_str())?;
            let thetdr = require_runtime_surface_scalar(runtime_surface, wp_symbol.as_str())?;
            let ssc = require_runtime_surface_scalar(runtime_surface, ssc_symbol.as_str())?;
            let por = require_runtime_surface_scalar(runtime_surface, por_symbol.as_str())?;
            let cpm = require_runtime_surface_scalar(runtime_surface, cpm_symbol.as_str())?;

            if dg <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!("{SIMPIPE_GUARD_ID} {dg_symbol} must be > 0.0, observed {dg}"),
                });
            }
            if thetfc < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {fc_symbol} must be >= 0.0, observed {thetfc}"
                    ),
                });
            }
            if thetdr < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {wp_symbol} must be >= 0.0, observed {thetdr}"
                    ),
                });
            }
            if thetdr > thetfc {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {wp_symbol} must be <= {fc_symbol} (observed {thetdr} > {thetfc})"
                    ),
                });
            }
            if ssc <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {ssc_symbol} must be > 0.0, observed {ssc}"
                    ),
                });
            }
            if por <= 0.0 || por > 1.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {por_symbol} must be within (0,1], observed {por}"
                    ),
                });
            }
            if cpm <= 0.0 || cpm > 1.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {cpm_symbol} must be within (0,1], observed {cpm}"
                    ),
                });
            }
            if thetdr > por {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} {wp_symbol} must be <= {por_symbol} (observed {thetdr} > {por})"
                    ),
                });
            }

            let saturation_capacity = por * cpm;
            if !saturation_capacity.is_finite() || saturation_capacity <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} por*cpm must be finite and > 0.0, observed {saturation_capacity}"
                    ),
                });
            }
            let sat_floor = thetdr / saturation_capacity;
            if !sat_floor.is_finite() || !(0.0..=1.0).contains(&sat_floor) {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived saturation floor for layer {layer_index} must be within [0,1], observed {sat_floor}"
                    ),
                });
            }
            if sat < sat_floor {
                sat = sat_floor;
            }

            let fc_store = (thetfc - thetdr) * dg;
            if !fc_store.is_finite() || fc_store < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived wb18_perc_fc_{layer_index:04} must be finite and >= 0.0, observed {fc_store}"
                    ),
                });
            }

            let ul_store = (por - thetdr) * dg;
            if ul_store <= 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived WB18 upper-limit store must be > 0.0 for layer {layer_index}"
                    ),
                });
            }

            let saturation_theta = (sat * por) * cpm;
            let mut st_store = (saturation_theta - thetdr) * dg;
            if !st_store.is_finite() {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived wb18_perc_theta_{layer_index:04} is non-finite ({st_store})"
                    ),
                });
            }
            if st_store < -1.0e-10 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived wb18_perc_theta_{layer_index:04} must be >= 0.0, observed {st_store}"
                    ),
                });
            }
            if st_store < 1.0e-10 {
                st_store = 0.0;
            }

            let soilw_store = st_store + (thetdr * dg);
            if !soilw_store.is_finite() || soilw_store < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} derived layer soil-water store must be finite and >= 0.0 for layer {layer_index}, observed {soilw_store}"
                    ),
                });
            }

            wb11_soil_water += soilw_store;
            wb11_field_capacity += fc_store;
            wb11_drainable_storage += (st_store - fc_store).max(0.0);
            wb11_drainage_coefficient += ssc * 86_400.0;

            runtime_surface.state_surface.insert(
                BoundarySymbol::from(format!("wb18_perc_theta_{layer_index:04}")),
                BoundaryValue::scalar(st_store),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(format!("wb18_perc_fc_{layer_index:04}")),
                BoundaryValue::scalar(fc_store),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(format!("wb18_perc_ul_{layer_index:04}")),
                BoundaryValue::scalar(ul_store),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(format!("wb18_perc_ssc_{layer_index:04}")),
                BoundaryValue::scalar(ssc),
            );
        }

        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("sat"), BoundaryValue::scalar(sat));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(wb11_soil_water),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_field_capacity"),
            BoundaryValue::scalar(wb11_field_capacity),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_perc_fraction"),
            BoundaryValue::scalar(0.5),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_drainage_coefficient"),
            BoundaryValue::scalar(wb11_drainage_coefficient.max(1.0e-6)),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_drainable_storage"),
            BoundaryValue::scalar(wb11_drainable_storage),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(WB11_STATE_SEED_COMPLETED_SYMBOL),
            BoundaryValue::scalar(1.0),
        );
    }

    let wb11_soil_water = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    if wb11_soil_water < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} wb11_soil_water must be >= 0.0 before daily reconciliation seeding, observed {wb11_soil_water}"
            ),
        });
    }

    if runtime_surface_symbol_value(runtime_surface, "wb17_residue_interception").is_none() {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb17_residue_interception"),
            BoundaryValue::scalar(0.0),
        );
    }
    if runtime_surface_symbol_value(runtime_surface, "Ws").is_none() {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ws"), BoundaryValue::scalar(1.0));
    }
    if runtime_surface_symbol_value(runtime_surface, "wb19_lateral_anisotropy_ratio").is_none() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} missing required runtime symbol wb19_lateral_anisotropy_ratio"
            ),
        });
    }
    if runtime_surface_symbol_value(runtime_surface, "wb19_drain_enabled").is_none() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} missing required runtime symbol wb19_drain_enabled"
            ),
        });
    }
    let wb19_lateral_anisotropy_ratio =
        require_runtime_surface_scalar(runtime_surface, "wb19_lateral_anisotropy_ratio")?;
    if wb19_lateral_anisotropy_ratio <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} wb19_lateral_anisotropy_ratio must be > 0.0, observed {wb19_lateral_anisotropy_ratio}"
            ),
        });
    }
    let wb19_drain_enabled = require_runtime_surface_scalar(runtime_surface, "wb19_drain_enabled")?;
    let wb19_drain_enabled_flag = if wb19_drain_enabled.abs() <= 1.0e-12 {
        false
    } else if (wb19_drain_enabled - 1.0).abs() <= 1.0e-12 {
        true
    } else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} wb19_drain_enabled must be 0 or 1, observed {wb19_drain_enabled}"
            ),
        });
    };
    if wb19_drain_enabled_flag {
        let wb19_drain_depth = require_runtime_surface_scalar(runtime_surface, "wb19_drain_depth")?;
        let wb19_drain_spacing =
            require_runtime_surface_scalar(runtime_surface, "wb19_drain_spacing")?;
        let wb19_drain_diameter =
            require_runtime_surface_scalar(runtime_surface, "wb19_drain_diameter")?;
        if wb19_drain_depth <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} wb19_drain_depth must be > 0.0 when wb19_drain_enabled=1, observed {wb19_drain_depth}"
                ),
            });
        }
        if wb19_drain_spacing <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} wb19_drain_spacing must be > 0.0 when wb19_drain_enabled=1, observed {wb19_drain_spacing}"
                ),
            });
        }
        if wb19_drain_diameter <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} wb19_drain_diameter must be > 0.0 when wb19_drain_enabled=1, observed {wb19_drain_diameter}"
                ),
            });
        }
    }

    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(hyetograph_rainfall.max(prcp)),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.flux_surface.insert(
        BoundarySymbol::from("wb12_runoff_carryover"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(wb11_soil_water),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(wb11_soil_water),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(prcp),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    let wb11_et_seed = compute_wb11_et_demand_seed(runtime_surface)?;
    publish_wb11_et_demand_seed(runtime_surface, wb11_et_seed)?;

    if runtime_surface_symbol_value(runtime_surface, "efflen").is_none() {
        let slplen = require_runtime_surface_scalar(runtime_surface, "slplen")?;
        if slplen <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} slplen must be > 0.0 when seeding efflen, observed {slplen}"
                ),
            });
        }
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("efflen"),
            BoundaryValue::scalar(slplen),
        );
    }
    if runtime_surface_symbol_value(runtime_surface, "m").is_none() {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
    }
    let ealpha_seeded_prior =
        runtime_surface_symbol_value(runtime_surface, WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL)
            .is_some_and(|value| value >= 0.5);
    let ealpha_runtime_produced_this_day =
        produce_wb16_ealpha_from_runtime_surface(runtime_surface)?.is_some();
    if !ealpha_runtime_produced_this_day {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(1.0));
    }
    let ealpha_seeded_any_day = !ealpha_runtime_produced_this_day || ealpha_seeded_prior;
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL),
        BoundaryValue::scalar(if ealpha_seeded_any_day { 1.0 } else { 0.0 }),
    );
    seed_mofe03_wave2_runtime_surface_inputs(runtime_surface)?;

    Ok(())
}

const WB16_ACCGAV_M_S2: f64 = 9.807;
const WB16_INRFSO_CROPLAND: f64 = 4.07;
const WB16_FRCSOL_CROPLAND: f64 = 1.11;
const WB16_RRINIT_MIN_M: f64 = 0.006;
const WB16_RSPACE_DEFAULT_M: f64 = 1.0;
const WB16_TEMPORARY_WIDTH_DEFAULT_M: f64 = 0.15;
const WB16_COVER_CAP: f64 = 0.999;

#[allow(clippy::too_many_lines, clippy::similar_names)]
fn produce_wb16_ealpha_from_runtime_surface(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<Option<f64>, HillslopeCliError> {
    let Some(nelem_raw) = runtime_surface_symbol_value(runtime_surface, "nelem") else {
        return Ok(None);
    };
    let ofe_count = scalar_to_usize("nelem", nelem_raw)?;
    if ofe_count == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!("{SIMPIPE_GUARD_ID} nelem must be >= 1 for WB16 ealpha production"),
        });
    }

    let m = require_runtime_surface_scalar(runtime_surface, "m")?;
    if !m.is_finite() || m <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} m must be finite and > 0 for WB16 ealpha production, observed {m}"
            ),
        });
    }
    let power2 = 1.0 / m;
    let power3 = power2 + 1.0;

    let mut alpha_values = Vec::with_capacity(ofe_count);
    let mut slplen_values = Vec::with_capacity(ofe_count);

    for ofe_index in 1..=ofe_count {
        let Some(avgslp_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "avgslp")
        else {
            return Ok(None);
        };
        let Some(slplen_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "slplen")
        else {
            return Ok(None);
        };
        if !avgslp_raw.is_finite() || avgslp_raw <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_avgslp must be finite and > 0, observed {avgslp_raw}"
                ),
            });
        }
        if !slplen_raw.is_finite() || slplen_raw <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_slplen must be finite and > 0, observed {slplen_raw}"
                ),
            });
        }

        let Some(inrcov_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "inrcov")
        else {
            return Ok(None);
        };
        let Some(rilcov_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rilcov")
        else {
            return Ok(None);
        };
        let Some(rrinit_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rrinit")
        else {
            return Ok(None);
        };
        let Some(rspace_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rspace")
        else {
            return Ok(None);
        };
        let Some(width_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "width")
        else {
            return Ok(None);
        };
        let Some(rtyp_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rtyp")
        else {
            return Ok(None);
        };

        let Some(cancov_raw) = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "cancov")
            .or_else(|| wb16_optional_state_scalar(runtime_surface, "cancov"))
        else {
            return Ok(None);
        };
        let Some(bb_raw) = wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_bb_seed"),
        )
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "bb"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bb")) else {
            return Ok(None);
        };
        let Some(bbb_raw) = wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_bbb_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bbb_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "bbb"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "bbb")) else {
            return Ok(None);
        };
        let Some(flivmx_raw) = wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_flivmx_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "flivmx_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "flivmx"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "flivmx")) else {
            return Ok(None);
        };
        let Some(hmax_raw) = wb16_optional_state_scalar(
            runtime_surface,
            &format!("pl_growth_ofe{ofe_index}_hmax_seed"),
        )
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "hmax_seed"))
        .or_else(|| wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "hmax"))
        .or_else(|| wb16_optional_state_scalar(runtime_surface, "hmax")) else {
            return Ok(None);
        };

        for (symbol, value) in [
            ("inrcov", inrcov_raw),
            ("rilcov", rilcov_raw),
            ("rrinit", rrinit_raw),
            ("rspace", rspace_raw),
            ("width", width_raw),
            ("rtyp", rtyp_raw),
            ("cancov", cancov_raw),
            ("bb", bb_raw),
            ("bbb", bbb_raw),
            ("flivmx", flivmx_raw),
            ("hmax", hmax_raw),
        ] {
            if !value.is_finite() {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb16_ealpha_producer",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} ofe{ofe_index}_{symbol} must be finite for WB16 ealpha production, observed {value}"
                    ),
                });
            }
        }

        if inrcov_raw < 0.0 || rilcov_raw < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_inrcov/rilcov must be >= 0.0, observed inrcov={inrcov_raw}, rilcov={rilcov_raw}"
                ),
            });
        }
        if rrinit_raw < 0.0 || rspace_raw < 0.0 || width_raw < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_rrinit/rspace/width must be >= 0.0, observed rrinit={rrinit_raw}, rspace={rspace_raw}, width={width_raw}"
                ),
            });
        }
        if cancov_raw < 0.0 || bb_raw < 0.0 || bbb_raw < 0.0 || flivmx_raw < 0.0 || hmax_raw < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index} canopy/friction controls must be >= 0.0 (cancov={cancov_raw}, bb={bb_raw}, bbb={bbb_raw}, flivmx={flivmx_raw}, hmax={hmax_raw})"
                ),
            });
        }

        let inrcov = inrcov_raw.min(WB16_COVER_CAP);
        let rilcov = rilcov_raw.min(WB16_COVER_CAP);
        let cancov = cancov_raw.min(WB16_COVER_CAP);
        let rrinit = rrinit_raw.max(WB16_RRINIT_MIN_M);
        let rspace = if rspace_raw <= 0.0 {
            WB16_RSPACE_DEFAULT_M
        } else {
            rspace_raw
        };
        let mut width = width_raw;
        let rtyp = if rtyp_raw >= 1.5 { 2 } else { 1 };
        if rtyp == 1 && width <= 0.0 {
            width = WB16_TEMPORARY_WIDTH_DEFAULT_M;
        } else if rtyp == 2 && width <= 0.0 {
            width = rspace;
        }
        if width > rspace {
            width = rspace;
        }

        let rrc = wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "rrc")
            .or_else(|| wb16_optional_state_scalar(runtime_surface, "rrc"))
            .unwrap_or(rrinit);
        if !rrc.is_finite() || rrc < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_rrc must be finite and >= 0.0, observed {rrc}"
                ),
            });
        }

        let mut rrrinr = rrc / rrinit;
        if rrrinr > 1.0 {
            rrrinr = 1.0;
        }
        let inrfo = (3.024 - 5.042 * (-161.0 * rrinit).exp()).exp();
        let mut inrrou = 0.5 * inrfo.powf(1.128) * (-3.088 * (1.0 - rrrinr)).exp();
        if inrrou < WB16_INRFSO_CROPLAND {
            inrrou = WB16_INRFSO_CROPLAND;
        }
        let inrfro = inrrou - WB16_INRFSO_CROPLAND;
        let inrfco = if inrcov > 0.0 {
            14.5 * inrcov.powf(1.5544)
        } else {
            0.0
        };

        let canhgt = if let Some(canhgt_raw) =
            wb16_ofe_optional_state_scalar(runtime_surface, ofe_index, "canhgt")
                .or_else(|| wb16_optional_state_scalar(runtime_surface, "canhgt"))
        {
            if !canhgt_raw.is_finite() || canhgt_raw < 0.0 {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb16_ealpha_producer",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} ofe{ofe_index}_canhgt must be finite and >= 0.0, observed {canhgt_raw}"
                    ),
                });
            }
            canhgt_raw
        } else if hmax_raw <= 0.0 || bb_raw <= 0.0 {
            0.0
        } else {
            let mut vdmt = (1.0 - cancov).ln() / (-bb_raw);
            if vdmt < 0.0 {
                vdmt = 0.0;
            }
            (1.0 - (-bbb_raw * vdmt).exp()) * hmax_raw
        };
        let frlive = if hmax_raw > 0.0 {
            (canhgt / hmax_raw) * flivmx_raw
        } else {
            0.0
        };
        if !frlive.is_finite() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!("{SIMPIPE_GUARD_ID} ofe{ofe_index}_frlive is non-finite"),
            });
        }

        let inrfto = inrfro + inrfco + WB16_INRFSO_CROPLAND + frlive;
        let frccov = if rilcov > 0.0 {
            4.5 * rilcov.powf(1.5544)
        } else {
            0.0
        };
        let frctrl = frccov + frlive + WB16_FRCSOL_CROPLAND;
        let rillar = width / rspace;
        let frcteq = if rillar < 1.0 {
            inrfto + rillar * (frctrl - inrfto)
        } else {
            inrfto
        };
        if !frcteq.is_finite() || frcteq <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_frcteq must be finite and > 0.0, observed {frcteq}"
                ),
            });
        }
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_frcteq")),
            BoundaryValue::scalar(frcteq),
        );

        let alpha = ((avgslp_raw * 8.0 * WB16_ACCGAV_M_S2) / frcteq).sqrt();
        if !alpha.is_finite() || alpha <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} ofe{ofe_index}_alpha must be finite and > 0.0, observed {alpha}"
                ),
            });
        }
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_alpha")),
            BoundaryValue::scalar(alpha),
        );
        if ofe_index == 1 {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from("alpha"), BoundaryValue::scalar(alpha));
        }

        alpha_values.push(alpha);
        slplen_values.push(slplen_raw);
    }

    let ealpha = if ofe_count == 1 {
        alpha_values[0]
    } else {
        let suml: f64 = slplen_values.iter().sum();
        if !suml.is_finite() || suml <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} WB16 eplane sum length must be finite and > 0.0, observed {suml}"
                ),
            });
        }
        let mut cml = 0.0;
        let mut sdst = 0.0;
        let mut tmpvr2 = 0.0;
        for (slplen, alpha) in slplen_values.iter().zip(alpha_values.iter()) {
            cml += slplen;
            let tmpvr1 = cml.powf(power3);
            sdst += (tmpvr1 - tmpvr2) / alpha.powf(power2);
            tmpvr2 = tmpvr1;
        }
        if !sdst.is_finite() || sdst <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb16_ealpha_producer",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} WB16 eplane storage integral must be finite and > 0.0, observed {sdst}"
                ),
            });
        }
        (suml / sdst).powf(m) * suml
    };

    if !ealpha.is_finite() || ealpha <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb16_ealpha_producer",
            detail: format!(
                "{SIMPIPE_GUARD_ID} WB16 produced ealpha must be finite and > 0.0, observed {ealpha}"
            ),
        });
    }

    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ealpha"),
        BoundaryValue::scalar(ealpha),
    );
    Ok(Some(ealpha))
}

fn wb16_optional_state_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    runtime_surface_symbol_value(runtime_surface, symbol)
}

fn wb16_ofe_optional_state_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_index: usize,
    root: &str,
) -> Option<f64> {
    runtime_surface_symbol_value(runtime_surface, &format!("ofe{ofe_index}_{root}")).or_else(|| {
        if ofe_index == 1 {
            runtime_surface_symbol_value(runtime_surface, root)
        } else {
            None
        }
    })
}

const MOFE03_WAVE2_ENABLE_TOLERANCE: f64 = 1.0e-9;
const MOFE03_WAVE2_MIN_POSITIVE: f64 = 1.0e-6;
const MOFE03_WAVE2_DEFAULT_XTOP: f64 = 0.2;
const MOFE03_WAVE2_DEFAULT_XBOT: f64 = 0.5;
const MOFE03_WAVE2_DEFAULT_XDETST: f64 = 0.1;
const MOFE03_WAVE2_DEFAULT_LDTOP: f64 = 0.8;
const MOFE03_WAVE2_DEFAULT_LDBOT: f64 = 0.6;
const MOFE03_WAVE2_DEFAULT_LDDEND: f64 = 0.3;
const MOFE03_WAVE2_DEFAULT_KTRATO: f64 = 1.1;
const MOFE03_WAVE2_DEFAULT_AINTC: f64 = 0.4;
const MOFE03_WAVE2_DEFAULT_BINTC: f64 = 0.3;
const MOFE03_WAVE2_DEFAULT_CINTC: f64 = 0.2;
const MOFE03_WAVE2_DEFAULT_BETA: f64 = 0.5;
const MOFE03_WAVE2_DEFAULT_QOSTAR: f64 = 0.2;
const MOFE03_WAVE2_DEFAULT_SSA_SOIL: f64 = 5.0;
const MOFE03_ROUTE_SEGMENT_INDEX: usize = 2;

#[derive(Debug, Clone, Copy)]
struct Mofe03Wave2CaseScalars {
    case_value: f64,
    qj_minus_1: f64,
    vj: f64,
    qj: f64,
    fh: f64,
    fp: f64,
}

fn seed_mofe03_wave2_runtime_surface_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    let ofe_count = resolve_mofe03_ofe_count(runtime_surface)?;
    let wave2_enabled = resolve_mofe03_wave2_enabled(runtime_surface, ofe_count)?;
    write_mofe03_wave2_enabled(runtime_surface, wave2_enabled);
    if !wave2_enabled {
        return Ok(());
    }

    let slplen = require_mofe03_positive_runtime_surface_scalar(
        runtime_surface,
        "slplen",
        "Wave-2 seeding",
    )?;
    let qout = resolve_mofe03_wave2_qout(runtime_surface)?;
    let qin = resolve_mofe03_wave2_qin(runtime_surface)?;
    let qostar = (qout - qin).max(0.0);
    let case_scalars = build_mofe03_wave2_case_scalars(qout);

    seed_mofe03_wave2_core_scalars(runtime_surface, ofe_count, slplen, qout, qin, qostar)?;
    seed_mofe03_wave2_route_topology_ingress(runtime_surface, qostar);
    let (beta, theta) = resolve_mofe03_wave2_beta_theta(runtime_surface)?;
    seed_mofe03_wave2_case_state(runtime_surface, case_scalars, beta, theta);
    seed_mofe03_wave2_ssa_soil(runtime_surface)?;
    seed_mofe03_wave2_class_symbols(runtime_surface, ofe_count)?;
    Ok(())
}

fn resolve_mofe03_ofe_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    let ofe_count = scalar_to_usize(
        "nelem",
        require_mofe03_runtime_surface_scalar(runtime_surface, "nelem")?,
    )?;
    if ofe_count == 0 {
        return Err(mofe03_wave2_seed_failure(
            "nelem must be >= 1 for MOFE03 activation policy",
        ));
    }
    Ok(ofe_count)
}

fn resolve_mofe03_wave2_enabled(
    runtime_surface: &HillslopeWritebackSurface,
    ofe_count: usize,
) -> Result<bool, HillslopeCliError> {
    if let Some(value) = runtime_surface_symbol_value(runtime_surface, "erod14_wave2_enabled") {
        parse_mofe03_binary_flag("erod14_wave2_enabled", value)
    } else {
        Ok(ofe_count > 1)
    }
}

fn write_mofe03_wave2_enabled(runtime_surface: &mut HillslopeWritebackSurface, enabled: bool) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_wave2_enabled"),
        BoundaryValue::scalar(if enabled { 1.0 } else { 0.0 }),
    );
}

fn require_mofe03_positive_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
    context: &str,
) -> Result<f64, HillslopeCliError> {
    let value = require_mofe03_runtime_surface_scalar(runtime_surface, symbol)?;
    if value <= 0.0 {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} must be > 0.0 for {context}, observed {value}"
        )));
    }
    Ok(value)
}

fn resolve_mofe03_wave2_qout(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    require_mofe03_non_negative_seed_scalar(
        runtime_surface_symbol_value(runtime_surface, "Q")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "wb12_runoff_observed"))
            .unwrap_or(0.0),
        "erod14_qout",
    )
}

fn resolve_mofe03_wave2_qin(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    require_mofe03_non_negative_seed_scalar(
        runtime_surface_symbol_value(runtime_surface, "UpStrmQ").unwrap_or(0.0),
        "erod14_qin",
    )
}

fn build_mofe03_wave2_case_scalars(qout: f64) -> Mofe03Wave2CaseScalars {
    if qout > MOFE03_WAVE2_ENABLE_TOLERANCE {
        return Mofe03Wave2CaseScalars {
            case_value: 2.0,
            qj_minus_1: qout.max(MOFE03_WAVE2_MIN_POSITIVE),
            vj: (0.25 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
            qj: (0.50 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
            fh: qout.max(MOFE03_WAVE2_MIN_POSITIVE),
            fp: (0.5 * qout).max(MOFE03_WAVE2_MIN_POSITIVE),
        };
    }
    Mofe03Wave2CaseScalars {
        case_value: 4.0,
        qj_minus_1: MOFE03_WAVE2_MIN_POSITIVE,
        vj: 0.0,
        qj: 0.0,
        fh: 0.0,
        fp: 0.0,
    }
}

fn seed_mofe03_wave2_core_scalars(
    runtime_surface: &mut HillslopeWritebackSurface,
    ofe_count: usize,
    slplen: f64,
    qout: f64,
    qin: f64,
    qostar: f64,
) -> Result<(), HillslopeCliError> {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_class_count"),
        BoundaryValue::scalar(usize_to_scalar("erod14_class_count", ofe_count)?),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xtop"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_XTOP),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xbot"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_XBOT),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_xdetst"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_XDETST),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ldtop"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_LDTOP),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ldbot"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_LDBOT),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_lddend"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_LDDEND),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qout"),
        BoundaryValue::scalar(qout),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qin"),
        BoundaryValue::scalar(qin),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_qostar"),
        BoundaryValue::scalar(qostar.max(MOFE03_WAVE2_DEFAULT_QOSTAR)),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_slplen"),
        BoundaryValue::scalar(slplen),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ktrato"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_KTRATO),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ainftc"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_AINTC),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_binftc"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_BINTC),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_cinftc"),
        BoundaryValue::scalar(MOFE03_WAVE2_DEFAULT_CINTC),
    );
    Ok(())
}

fn seed_mofe03_wave2_route_topology_ingress(
    runtime_surface: &mut HillslopeWritebackSurface,
    qostar: f64,
) {
    let xu = runtime_surface_symbol_value(runtime_surface, "erod14_xtop")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_XTOP);
    let xl = runtime_surface_symbol_value(runtime_surface, "erod14_xbot")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_XBOT);
    let xdetst = runtime_surface_symbol_value(runtime_surface, "erod14_xdetst")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_XDETST);
    let lddend = runtime_surface_symbol_value(runtime_surface, "erod14_lddend")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_LDDEND);
    let ainftc = runtime_surface_symbol_value(runtime_surface, "erod14_ainftc")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_AINTC);
    let binftc = runtime_surface_symbol_value(runtime_surface, "erod14_binftc")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_BINTC);
    let cinftc = runtime_surface_symbol_value(runtime_surface, "erod14_cinftc")
        .unwrap_or(MOFE03_WAVE2_DEFAULT_CINTC);
    let segment = MOFE03_ROUTE_SEGMENT_INDEX;

    seed_mofe03_scalar_if_absent(
        runtime_surface,
        "qostar",
        qostar.max(MOFE03_WAVE2_DEFAULT_QOSTAR),
    );
    seed_mofe03_scalar_if_absent(runtime_surface, "xdetst", xdetst);
    seed_mofe03_scalar_if_absent(runtime_surface, "lddend", lddend);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "xu", segment, xu);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "xl", segment, xl);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "ainf", segment, ainftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "binf", segment, binftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "cinf", segment, cinftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "ainftc", segment, ainftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "binftc", segment, binftc);
    seed_mofe03_segment_scalar_if_absent(runtime_surface, "cinftc", segment, cinftc);
}

fn seed_mofe03_scalar_if_absent(
    runtime_surface: &mut HillslopeWritebackSurface,
    symbol: &str,
    value: f64,
) {
    if runtime_surface_symbol_value(runtime_surface, symbol).is_some() {
        return;
    }
    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

fn seed_mofe03_segment_scalar_if_absent(
    runtime_surface: &mut HillslopeWritebackSurface,
    root: &str,
    segment_index: usize,
    value: f64,
) {
    let symbol = format!("{root}_{segment_index:04}");
    seed_mofe03_scalar_if_absent(runtime_surface, &symbol, value);
}

fn resolve_mofe03_wave2_beta_theta(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<(f64, f64), HillslopeCliError> {
    let beta = match runtime_surface_symbol_value(runtime_surface, "beta") {
        Some(value) => require_mofe03_non_negative_seed_scalar(value, "beta")?,
        None => MOFE03_WAVE2_DEFAULT_BETA,
    };
    let theta = if let Some(value) = runtime_surface_symbol_value(runtime_surface, "theta") {
        require_mofe03_non_negative_seed_scalar(value, "theta")?
    } else {
        let thetdr = require_mofe03_non_negative_seed_scalar(
            require_mofe03_runtime_surface_scalar(runtime_surface, "thetdr")?,
            "thetdr",
        )?;
        let thetfc = require_mofe03_non_negative_seed_scalar(
            require_mofe03_runtime_surface_scalar(runtime_surface, "thetfc")?,
            "thetfc",
        )?;
        0.5 * (thetdr + thetfc)
    };
    Ok((beta, theta))
}

fn seed_mofe03_wave2_case_state(
    runtime_surface: &mut HillslopeWritebackSurface,
    case_scalars: Mofe03Wave2CaseScalars,
    beta: f64,
    theta: f64,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_beta"),
        BoundaryValue::scalar(beta),
    );
    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from("theta"), BoundaryValue::scalar(theta));
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Qj_minus_1"),
        BoundaryValue::scalar(case_scalars.qj_minus_1),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Vj"),
        BoundaryValue::scalar(case_scalars.vj),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Qj"),
        BoundaryValue::scalar(case_scalars.qj),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fh"),
        BoundaryValue::scalar(case_scalars.fh),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_Fp"),
        BoundaryValue::scalar(case_scalars.fp),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_case"),
        BoundaryValue::scalar(case_scalars.case_value),
    );
}

fn seed_mofe03_wave2_ssa_soil(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    let ssa_soil = match runtime_surface_symbol_value(runtime_surface, "erod14_ssa_soil") {
        Some(value) => require_mofe03_positive_seed_scalar(value, "erod14_ssa_soil")?,
        None => MOFE03_WAVE2_DEFAULT_SSA_SOIL,
    };
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("erod14_ssa_soil"),
        BoundaryValue::scalar(ssa_soil),
    );
    Ok(())
}

fn seed_mofe03_wave2_class_symbols(
    runtime_surface: &mut HillslopeWritebackSurface,
    ofe_count: usize,
) -> Result<(), HillslopeCliError> {
    let class_count_f64 = usize_to_scalar("erod14_class_count", ofe_count)?;
    let class_fraction = 1.0 / class_count_f64;
    for class_index in 1..=ofe_count {
        let class_index_f64 = usize_to_scalar("erod14_class_index", class_index)?;
        let reverse_class_index = ofe_count.saturating_sub(class_index) + 1;
        let reverse_class_index_f64 =
            usize_to_scalar("erod14_reverse_class_index", reverse_class_index)?;
        let class_offset = class_index.saturating_sub(1);
        let class_offset_f64 = usize_to_scalar("erod14_class_offset", class_offset)?;

        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_fall",
            class_index,
            (0.02 / class_index_f64).max(MOFE03_WAVE2_MIN_POSITIVE),
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_frcflw",
            class_index,
            class_fraction,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_frac",
            class_index,
            class_fraction,
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_fidel",
            class_index,
            (0.20 + (0.10 * class_index_f64)).min(0.95),
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_tcf1",
            class_index,
            0.20 + (0.05 * reverse_class_index_f64),
        )?;
        seed_mofe03_wave2_class_symbol(
            runtime_surface,
            "erod14_ssa_class",
            class_index,
            1.5 + (2.5 * class_offset_f64),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn execute_scheduler_kernel_lifecycle(
    runtime_surface: HillslopeWritebackSurface,
    context: SchedulerLifecycleContext<'_>,
) -> Result<DailyExecutionResult, HillslopeCliError> {
    let mut runtime_surface = runtime_surface;
    seed_wb11_runtime_surface_inputs(&mut runtime_surface, context.execution_lane)?;
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("year"),
        BoundaryValue::scalar(f64::from(context.simulation_year)),
    );
    prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)?;
    let trace_day = context
        .hphys0245_trace_config
        .is_some_and(|config| config.includes_day(context.sim_day_index));
    let snow_runtime_before = trace_day.then(|| {
        Hphys0245SnowRuntimeBeforeState::from_surface(
            &runtime_surface,
            context.runtime_swe_before_m,
        )
    });
    let mut hphys0245_trace_rows = Vec::new();
    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_seed",
            None,
            &runtime_surface,
            None,
            snow_runtime_before,
        ));
    }

    let topology_graph = TopologyGraph::new(1, 0, 0, Vec::new());
    let topology_report = validate_pre_execution_topology(&topology_graph).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!(
                "{SIMPIPE_GUARD_ID} failed building topology precondition report: {error}"
            ),
        }
    })?;

    let scheduler = HillslopePhaseScheduler::canonical();
    let execution_report = if trace_day {
        let mut kernel = Hphys0245TelemetryKernel::new(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            snow_runtime_before,
        );
        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "execution_provenance",
                detail: format!("{SIMPIPE_GUARD_ID} scheduler/kernel lifecycle failed: {error}"),
            })?;
        hphys0245_trace_rows.extend(kernel.into_rows());
        report
    } else {
        let mut kernel = Wb11HydrologyKernel;
        scheduler
            .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "execution_provenance",
                detail: format!("{SIMPIPE_GUARD_ID} scheduler/kernel lifecycle failed: {error}"),
            })?
    };

    if !execution_report.scheduler_report.is_success() {
        let scheduler_status = &execution_report.scheduler_report.scheduler_status;
        let phase_context = execution_report
            .phase_reports
            .last()
            .map(|phase_report| {
                let mut context = format!(
                    ", last_phase={}, last_kernel_message_id={}, last_decision_outcome={:?}, last_decision_message_id={}",
                    phase_report.phase.as_str(),
                    phase_report.kernel_status.message_id(),
                    phase_report.decision_outcome,
                    phase_report.decision_status.message_id()
                );

                if !phase_report.decision_violations.is_empty() {
                    let violation_summary = phase_report
                        .decision_violations
                        .iter()
                        .take(3)
                        .map(|violation| {
                            format!(
                                "{}:{}:{:?}",
                                violation.check_id, violation.subject, violation.details
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" | ");
                    context.push_str(", last_decision_violations=");
                    context.push_str(&violation_summary);
                }
                if phase_report.phase.as_str() == "storage_reconciliation" {
                    context.push_str(", wb12_terms=");
                    context.push_str(&format_wb12_storage_terms(
                        &execution_report.writeback_surface,
                    ));
                }
                if phase_report.phase.as_str() == "percolation_deep_seepage"
                    && phase_report.kernel_status.message_id() == "HKERNEL-WB11-PERC-E-003"
                {
                    context.push_str(", wb18_guard_terms=");
                    context.push_str(&format_wb18_perc_guard_terms(
                        &execution_report.writeback_surface,
                    ));
                }

                context
            })
            .unwrap_or_default();
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!(
                "{SIMPIPE_GUARD_ID} scheduler lifecycle did not complete successfully (outcome_class={}, status_class={:?}, boundary_class={}, message_id={}{})",
                scheduler_outcome_class_as_str(execution_report.scheduler_report.outcome_class),
                scheduler_status.classification(),
                scheduler_status.boundary_class().as_str(),
                scheduler_status.message_id(),
                phase_context
            ),
        });
    }

    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_scheduler",
            None,
            &execution_report.writeback_surface,
            None,
            snow_runtime_before,
        ));
    }

    let wb13_row = build_simulation_owned_wb13_row(
        &execution_report.writeback_surface,
        context.publication_area_m2,
        context.simulation_year,
        context.sim_day_index,
        context.calendar_day,
        context.runtime_swe_before_m,
    )?;
    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_wb13",
            None,
            &execution_report.writeback_surface,
            Some(&wb13_row),
            snow_runtime_before,
        ));
    }
    let coupling_vectors =
        build_simimpl10_coupling_vector_provenance(&execution_report.writeback_surface, &wb13_row)?;
    let kernel_phase_message_ids = execution_report
        .phase_reports
        .iter()
        .map(|phase| phase.kernel_status.message_id().to_string())
        .collect::<Vec<_>>();
    let erod14_wave2_kernel_status_seen = execution_report.phase_reports.iter().any(|phase| {
        let message_id = phase.kernel_status.message_id();
        message_id.contains("EROD14-WAVE2")
            || message_id.contains("EROD18-ROUTE")
            || message_id.contains("EROD19-ROUTE")
    });

    Ok(DailyExecutionResult {
        scheduler_outcome_class: execution_report.scheduler_report.outcome_class,
        scheduler_status_message_id: execution_report
            .scheduler_report
            .scheduler_status
            .message_id()
            .to_string(),
        coupling_vectors,
        wb13_row,
        runtime_surface: execution_report.writeback_surface,
        kernel_phase_message_ids,
        erod14_wave2_kernel_status_seen,
        hphys0245_trace_rows,
    })
}

fn prepare_pl_runtime_activation_for_scheduler(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    const PL_SCHEDULE_SLOT_COUNT_SYMBOL: &str = "pl_schedule_slot_count";

    if runtime_surface_symbol_value(runtime_surface, PL_SCHEDULE_SLOT_COUNT_SYMBOL).is_none() {
        return Ok(());
    }

    if pl_runtime_has_active_crop_for_scheduler_day(runtime_surface)? {
        return Ok(());
    }

    runtime_surface
        .state_surface
        .remove(&BoundarySymbol::from(PL_SCHEDULE_SLOT_COUNT_SYMBOL));
    Ok(())
}

fn pl_runtime_has_active_crop_for_scheduler_day(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<bool, HillslopeCliError> {
    let slot_count = require_runtime_usize_in_range(runtime_surface, "pl_schedule_slot_count", 1)?;
    let rotation_years =
        require_runtime_usize_in_range(runtime_surface, "pl_schedule_rotation_years", 1)?;
    let rotation_repeats =
        require_runtime_usize_in_range(runtime_surface, "pl_schedule_rotation_repeats", 1)?;
    let runtime_year = require_runtime_usize_in_range(runtime_surface, "year", 1)?;
    let day_of_year = require_runtime_usize_in_range(runtime_surface, "day", 1)?;
    if day_of_year > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("day must be in 1..=366 for PL activation, observed {day_of_year}"),
        });
    }

    let max_runtime_year = rotation_repeats
        .checked_mul(rotation_years)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: "rotation_repeats * rotation_years overflowed".to_string(),
        })?;
    if runtime_year > max_runtime_year {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "year must be in 1..={max_runtime_year} for PL activation, observed {runtime_year}"
            ),
        });
    }

    let rotation_index = ((runtime_year - 1) / rotation_years) + 1;
    let year_in_rotation = ((runtime_year - 1) % rotation_years) + 1;
    let mut slot_candidates = Vec::new();
    for slot_index in 1..=slot_count {
        let ofe_index = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("ofe_index", slot_index),
            1,
        )?;
        if ofe_index != 1 {
            continue;
        }
        let slot_year_in_rotation = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("year_in_rotation", slot_index),
            1,
        )?;
        let slot_rotation_index = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("rotation_index", slot_index),
            1,
        )?;
        if slot_year_in_rotation == year_in_rotation && slot_rotation_index == rotation_index {
            slot_candidates.push(slot_index);
        }
    }

    let [slot_index] = slot_candidates.as_slice() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "expected exactly one active PL slot for ofe=1 year_in_rotation={year_in_rotation} rotation_index={rotation_index}, observed {slot_candidates:?}"
            ),
        });
    };

    let crop_slots = require_runtime_usize_in_range(
        runtime_surface,
        &pl_schedule_slot_symbol("crop_slots", *slot_index),
        1,
    )?;
    let mut active_crop_count = 0usize;
    for crop_slot_index in 1..=crop_slots {
        if pl_crop_slot_is_active_for_day(
            runtime_surface,
            *slot_index,
            crop_slot_index,
            day_of_year,
        )? {
            active_crop_count += 1;
        }
    }

    match active_crop_count {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "expected at most one active PL crop for slot {slot_index} day {day_of_year}, observed {active_crop_count}"
            ),
        }),
    }
}

fn pl_crop_slot_is_active_for_day(
    runtime_surface: &HillslopeWritebackSurface,
    slot_index: usize,
    crop_slot_index: usize,
    day_of_year: usize,
) -> Result<bool, HillslopeCliError> {
    let imngmt = require_runtime_usize_in_range(
        runtime_surface,
        &pl_schedule_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
        1,
    )?;
    if imngmt > 3 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("imngmt must be in 1..=3 for PL activation, observed {imngmt}"),
        });
    }

    let jdplt = require_runtime_usize_in_range(
        runtime_surface,
        &pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
        0,
    )?;
    if jdplt > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("jdplt must be in 0..=366 for PL activation, observed {jdplt}"),
        });
    }
    let jdharv = require_runtime_usize_in_range(
        runtime_surface,
        &pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
        0,
    )?;
    if jdharv > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("jdharv must be in 0..=366 for PL activation, observed {jdharv}"),
        });
    }

    let (active_end, jdstop) = if imngmt == 2 {
        let jdstop = require_runtime_usize_in_range(
            runtime_surface,
            &pl_growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index),
            0,
        )?;
        if jdstop > 366 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "pl_runtime_activation",
                detail: format!("jdstop must be in 0..=366 for PL activation, observed {jdstop}"),
            });
        }
        if jdplt == 0 {
            return Ok(jdstop == 0 || day_of_year <= jdstop);
        }
        let active_end = if jdstop == 0 { jdharv.max(1) } else { jdstop };
        (active_end, jdstop)
    } else {
        (jdharv.max(1), 0)
    };

    if jdplt == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "jdplt must be in 1..=366 for non-perennial PL activation, observed jdplt={jdplt} jdharv={jdharv} jdstop={jdstop}"
            ),
        });
    }

    Ok(day_is_within_julian_window(day_of_year, jdplt, active_end))
}

fn require_runtime_usize_in_range(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
    min_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    let value = scalar_to_usize(symbol, value)?;
    if value < min_allowed {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "{symbol} must be >= {min_allowed} for PL activation, observed {value}"
            ),
        });
    }
    Ok(value)
}

fn day_is_within_julian_window(day_of_year: usize, start_day: usize, end_day: usize) -> bool {
    if start_day <= end_day {
        day_of_year >= start_day && day_of_year <= end_day
    } else {
        day_of_year >= start_day || day_of_year <= end_day
    }
}

fn pl_schedule_slot_symbol(root: &str, slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_{root}")
}

fn pl_schedule_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn pl_growth_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn hphys0245_trace_config_from_env() -> Result<Option<Hphys0245TraceConfig>, HillslopeCliError> {
    let Some(path_value) = std::env::var_os(HPHYS0245_TRACE_PATH_ENV) else {
        return Ok(None);
    };
    if path_value.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "hphys0245_trace",
            detail: format!("{HPHYS0245_TRACE_PATH_ENV} cannot be empty when set"),
        });
    }

    let max_days = match std::env::var(HPHYS0245_TRACE_MAX_DAYS_ENV) {
        Ok(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                let parsed = trimmed.parse::<usize>().map_err(|error| {
                    HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "hphys0245_trace",
                        detail: format!(
                            "{HPHYS0245_TRACE_MAX_DAYS_ENV} must be a positive integer, observed {trimmed}: {error}"
                        ),
                    }
                })?;
                if parsed == 0 {
                    return Err(HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "hphys0245_trace",
                        detail: format!("{HPHYS0245_TRACE_MAX_DAYS_ENV} must be >= 1"),
                    });
                }
                Some(parsed)
            }
        }
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "hphys0245_trace",
                detail: format!("{HPHYS0245_TRACE_MAX_DAYS_ENV} must be valid UTF-8"),
            });
        }
    };

    Ok(Some(Hphys0245TraceConfig {
        path: PathBuf::from(path_value),
        max_days,
    }))
}

fn write_hphys0245_trace_jsonl(
    config: &Hphys0245TraceConfig,
    rows: &[Hphys0245TraceRow],
) -> Result<(), HillslopeCliError> {
    ensure_output_parent_directory(&config.path)?;
    let mut payload = String::new();
    for row in rows {
        let line = serde_json::to_string(row).map_err(|source| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "hphys0245_trace",
                detail: format!("failed serializing trace row: {source}"),
            }
        })?;
        payload.push_str(&line);
        payload.push('\n');
    }
    fs::write(&config.path, payload).map_err(|source| HillslopeCliError::OutputWrite {
        path: config.path.clone(),
        source,
    })
}

fn hphys0245_surface_after_writeback(
    request: &HillslopeKernelRequest<'_>,
    payload: &KernelWritebackPayload,
) -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface {
        state_surface: request.state_surface.clone(),
        flux_surface: request.flux_surface.clone(),
    };
    for field in &payload.state_updates {
        surface
            .state_surface
            .insert(field.symbol.clone(), field.value);
    }
    for field in &payload.flux_updates {
        surface
            .flux_surface
            .insert(field.symbol.clone(), field.value);
    }
    surface
}

fn hphys0245_et_seed_branch(runtime_surface: &HillslopeWritebackSurface) -> Option<String> {
    if runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_evappm")
        .is_some_and(|value| value >= 0.5)
    {
        return Some("evappm_pmet".to_string());
    }
    if runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_priestley_taylor")
        .is_some_and(|value| value >= 0.5)
    {
        return Some("evap_priestley_taylor".to_string());
    }
    None
}

fn hphys0245_optional_delta(after: Option<f64>, before: Option<f64>) -> Option<f64> {
    match (after, before) {
        (Some(after), Some(before)) => Some(after - before),
        _ => None,
    }
}

#[allow(
    clippy::similar_names,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]
fn build_hphys0245_trace_row(
    run_name: &str,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_year: i32,
    julian_day: u16,
    boundary: &str,
    phase: Option<&str>,
    runtime_surface: &HillslopeWritebackSurface,
    wb13_row: Option<&SimulationOwnedWb13Row>,
    snow_runtime_before: Option<Hphys0245SnowRuntimeBeforeState>,
) -> Hphys0245TraceRow {
    let theta_layers =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_theta_");
    let wb18_ul_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_ul_");
    let wb18_thetdr_layers = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_thetdr_",
        "thetdr_",
    );
    let wb18_dg_layers_m = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_dg_",
        "dg_",
    );
    let wb18_fc_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_fc_");
    let wb19_coca_layers = hphys0245_prefixed_surface_values_with_fallback(
        &runtime_surface.state_surface,
        "wb19_coca_",
        "coca_",
    );
    let wb19_frzw_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "wb18_perc_frzw_");
    let wb19_drfc_layers_m =
        hphys0245_wb19_drfc_layers(&wb18_fc_layers_m, &wb18_dg_layers_m, &wb19_coca_layers);
    let wb19_fzdrfc_layers_m =
        hphys0245_wb19_fzdrfc_layers(&wb19_drfc_layers_m, &wb19_frzw_layers_m);
    let wb18_frozen_depth_layers_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb18_perc_frozen_depth_",
    );
    let pei_layers =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "wb18_perc_pei_");
    let potential_uptake_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "UPi_");
    let actual_uptake_layers_m =
        hphys0245_prefixed_surface_values(&runtime_surface.flux_surface, "Ui_");
    let wb19_lateral_withdrawal_layers_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_withdrawal_",
    );
    let wb19_lateral_capacity_active_count_layers = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_capacity_active_count_",
    );
    let wb19_lateral_conductivity_active_count_layers = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "wb19_lateral_conductivity_active_count_",
    );
    let theta_sum = hphys0245_sum_or_none(&theta_layers);
    let pei_sum = hphys0245_sum_or_none(&pei_layers);
    let wb11_soil_water = runtime_surface_symbol_value(runtime_surface, "wb11_soil_water");
    let wb12_infiltration_m = runtime_surface_symbol_value(runtime_surface, "wb12_infiltration");
    let wb18_recomputed_soil_water_m = hphys0245_recompute_wb18_soil_water(
        &theta_layers,
        &wb18_thetdr_layers,
        &wb18_dg_layers_m,
        &wb18_frozen_depth_layers_m,
    );
    let wb18_recomputed_minus_wb11_m = match (wb18_recomputed_soil_water_m, wb11_soil_water) {
        (Some(recomputed), Some(wb11)) => Some(recomputed - wb11),
        _ => None,
    };
    let wb11_minus_theta_sum_m = match (wb11_soil_water, theta_sum) {
        (Some(wb11), Some(theta)) => Some(wb11 - theta),
        _ => None,
    };
    let wb13_wat = wb13_row.map(|row| &row.wb13_row);
    let effective_pltol = runtime_surface_symbol_value(runtime_surface, "swu_effective_pltol");
    let wb17_swu_stress_threshold_layers_m =
        hphys0245_swu_stress_threshold_layers(&wb18_ul_layers_m, effective_pltol);
    let wb17_swu_storage_to_threshold_layers = hphys0245_swu_storage_to_threshold_layers(
        &theta_layers,
        &wb17_swu_stress_threshold_layers_m,
    );
    let snow_hourly_rain_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_m_",
    ));
    let snow_hourly_snowfall_depth_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.snowfall_m_",
    ));
    let snow_hourly_melt_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.melt_m_",
    ));
    let snow_hourly_melt_raw_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.melt_raw_m_",
    ));
    let snow_hourly_rain_retained_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_retained_m_",
    ));
    let snow_hourly_rain_released_sum_m = Some(hphys0245_sum_runtime_prefix(
        runtime_surface,
        "snow.hourly.rain_released_m_",
    ));
    let snow_hourly_melt_raw_m = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_raw_m_",
    );
    let snow_hourly_melt_m =
        hphys0245_prefixed_surface_values(&runtime_surface.state_surface, "snow.hourly.melt_m_");
    let snow_hourly_melt_amelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_amelt_in_",
    );
    let snow_hourly_melt_bmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_bmelt_in_",
    );
    let snow_hourly_melt_cmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_cmelt_in_",
    );
    let snow_hourly_melt_dmelt_in = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_dmelt_in_",
    );
    let snow_hourly_melt_hrtef_f = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_hrtef_f_",
    );
    let snow_hourly_melt_hrdtf_f = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_hrdtf_f_",
    );
    let snow_hourly_melt_vwmph = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_vwmph_",
    );
    let snow_hourly_melt_rainin = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_rainin_",
    );
    let snow_hourly_melt_wind_adjustment = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_wind_adjustment_",
    );
    let snow_hourly_melt_branch_active = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "snow.hourly.melt_branch_active_",
    );
    let winter_hourly_air_temp_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.air_temp_c_",
    );
    let winter_hourly_rad_mj_m2 = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.rad_mj_m2_",
    );
    let winter_hourly_cloud_fraction = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.cloud_fraction_",
    );
    let winter_hourly_dewpoint_c = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.dewpoint_c_",
    );
    let winter_hourly_wind_m_s = hphys0245_prefixed_surface_values(
        &runtime_surface.state_surface,
        "winter.hourly.wind_m_s_",
    );
    let snow_hourly_snowfall_water_equiv_sum_m = match (
        snow_hourly_snowfall_depth_sum_m,
        runtime_surface_symbol_value(runtime_surface, "snow.options.newsnw"),
    ) {
        (Some(depth_sum_m), Some(new_snow_density_kg_m3)) => {
            Some(depth_sum_m * new_snow_density_kg_m3 / 1_000.0)
        }
        _ => None,
    };
    let snow_s_m = runtime_surface_symbol_value_prefer_flux(runtime_surface, "S");
    let snow_runtime_swe_m = runtime_surface_symbol_value(runtime_surface, "snow.runtime_swe");
    let snow_runtime_depth_m =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_depth_m");
    let snow_runtime_density_kg_m3 =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_density_kg_m3");
    let snow_runtime_settle_day_count =
        runtime_surface_symbol_value(runtime_surface, "snow.runtime_settle_day_count");
    let snow_runtime_swe_before_m = snow_runtime_before.and_then(|state| state.swe_m);
    let snow_runtime_depth_before_m = snow_runtime_before.and_then(|state| state.depth_m);
    let snow_runtime_density_before_kg_m3 =
        snow_runtime_before.and_then(|state| state.density_kg_m3);
    let snow_runtime_settle_day_count_before =
        snow_runtime_before.and_then(|state| state.settle_day_count);
    let snow_runtime_swe_delta_m =
        hphys0245_optional_delta(snow_runtime_swe_m, snow_runtime_swe_before_m);
    let snow_runtime_depth_delta_m =
        hphys0245_optional_delta(snow_runtime_depth_m, snow_runtime_depth_before_m);
    let snow_runtime_density_delta_kg_m3 = hphys0245_optional_delta(
        snow_runtime_density_kg_m3,
        snow_runtime_density_before_kg_m3,
    );
    let snow_runtime_settle_day_count_delta = hphys0245_optional_delta(
        snow_runtime_settle_day_count,
        snow_runtime_settle_day_count_before,
    );
    let snow_runtime_swe_closure_error_m = match (
        snow_s_m,
        snow_hourly_melt_sum_m,
        snow_hourly_snowfall_water_equiv_sum_m,
        snow_hourly_rain_retained_sum_m,
        snow_hourly_rain_released_sum_m,
    ) {
        (
            Some(snow_s_m),
            Some(melt_sum_m),
            Some(snowfall_water_equiv_sum_m),
            Some(rain_retained_sum_m),
            Some(rain_released_sum_m),
        ) => Some(
            snow_s_m
                - ((melt_sum_m - rain_released_sum_m)
                    - snowfall_water_equiv_sum_m
                    - rain_retained_sum_m),
        ),
        _ => None,
    };

    Hphys0245TraceRow {
        schema: HPHYS0245_TRACE_SCHEMA,
        run_name: run_name.to_string(),
        sim_day_index,
        simulation_year,
        calendar_year,
        julian_day,
        boundary: boundary.to_string(),
        phase: phase.map(ToString::to_string),
        wb11_soil_water_m: wb11_soil_water,
        wb11_soil_water_mm: wb11_soil_water.map(|value| value * 1_000.0),
        wb12_infiltration_m,
        wb18_theta_sum_m: theta_sum,
        wb18_theta_layers_m: theta_layers,
        wb18_thetdr_layers,
        wb18_dg_layers_m,
        wb18_fc_layers_m,
        wb19_coca_layers,
        wb19_frzw_layers_m,
        wb19_drfc_layers_m,
        wb19_fzdrfc_layers_m,
        wb18_frozen_depth_layers_m,
        wb18_recomputed_soil_water_m,
        wb18_recomputed_minus_wb11_m,
        wb18_pei_sum_m: pei_sum,
        wb18_pei_layers_m: pei_layers,
        d_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "D"),
        pe_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Pe"),
        wb13_dp_mm: wb13_wat.map(|row| row.dp),
        wb13_total_soil_mm: wb13_wat.map(|row| row.total_soil),
        wb13_soil_water_total_mm: wb13_wat.map(|row| row.soil_water_total),
        snow_runtime_swe_m,
        snow_runtime_depth_m,
        snow_runtime_density_kg_m3,
        snow_runtime_settle_day_count,
        snow_runtime_swe_before_m,
        snow_runtime_depth_before_m,
        snow_runtime_density_before_kg_m3,
        snow_runtime_settle_day_count_before,
        snow_runtime_swe_delta_m,
        snow_runtime_depth_delta_m,
        snow_runtime_density_delta_kg_m3,
        snow_runtime_settle_day_count_delta,
        snow_s_m,
        snow_hourly_rain_sum_m,
        snow_hourly_rain_retained_sum_m,
        snow_hourly_rain_released_sum_m,
        snow_hourly_snowfall_depth_sum_m,
        snow_hourly_snowfall_water_equiv_sum_m,
        snow_hourly_melt_raw_sum_m,
        snow_hourly_melt_sum_m,
        snow_hourly_melt_raw_m,
        snow_hourly_melt_m,
        snow_hourly_melt_amelt_in,
        snow_hourly_melt_bmelt_in,
        snow_hourly_melt_cmelt_in,
        snow_hourly_melt_dmelt_in,
        snow_hourly_melt_hrtef_f,
        snow_hourly_melt_hrdtf_f,
        snow_hourly_melt_vwmph,
        snow_hourly_melt_rainin,
        snow_hourly_melt_wind_adjustment,
        snow_hourly_melt_branch_active,
        winter_hourly_air_temp_c,
        winter_hourly_rad_mj_m2,
        winter_hourly_cloud_fraction,
        winter_hourly_dewpoint_c,
        winter_hourly_wind_m_s,
        snow_runtime_swe_closure_error_m,
        wb13_p_mm: wb13_wat.map(|row| row.p),
        wb13_rm_mm: wb13_wat.map(|row| row.rm),
        wb13_snow_water_mm: wb13_wat.map(|row| row.snow_water),
        wb11_minus_theta_sum_m,
        pl_sumgdd: runtime_surface_symbol_value(runtime_surface, "sumgdd"),
        pl_vdmt: runtime_surface_symbol_value(runtime_surface, "vdmt"),
        pl_cancov: runtime_surface_symbol_value(runtime_surface, "cancov"),
        pl_lai: runtime_surface_symbol_value(runtime_surface, "lai"),
        pl_rtmass: runtime_surface_symbol_value(runtime_surface, "rtmass"),
        pl_rtd: runtime_surface_symbol_value(runtime_surface, "rtd"),
        pl_hia: runtime_surface_symbol_value(runtime_surface, "hia"),
        pl_pltol: runtime_surface_symbol_value(runtime_surface, "pltol"),
        pl_swu_effective_pltol: effective_pltol,
        pmet_sidecar_present: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.mode.sidecar_present",
        ),
        pmet_iflget: runtime_surface_symbol_value(runtime_surface, "pmetpara.mode.iflget"),
        pmet_selected_kcb: runtime_surface_symbol_value(runtime_surface, "pmetpara.selected.kcb"),
        pmet_selected_rawp: runtime_surface_symbol_value(runtime_surface, "pmetpara.selected.rawp"),
        pmet_selected_line_index: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.selected.line_index",
        ),
        pmet_lookup_fallback_first_row_used: runtime_surface_symbol_value(
            runtime_surface,
            "pmetpara.lookup.fallback_first_row_used",
        ),
        wb11_et_demand_m: runtime_surface_symbol_value(runtime_surface, "wb11_et_demand"),
        wb11_et_seed_branch: hphys0245_et_seed_branch(runtime_surface),
        pmet_etorc_mm: runtime_surface_symbol_value(runtime_surface, "pmet.etorc_mm"),
        pmet_rn_mj_m2: runtime_surface_symbol_value(runtime_surface, "pmet.rn_mj_m2"),
        pmet_fwv_m_s: runtime_surface_symbol_value(runtime_surface, "pmet.fwv_m_s"),
        pmet_rhd_pct: runtime_surface_symbol_value(runtime_surface, "pmet.rhd_pct"),
        pmet_kcbadj: runtime_surface_symbol_value(runtime_surface, "pmet.kcbadj"),
        pmet_kcbcon: runtime_surface_symbol_value(runtime_surface, "pmet.kcbcon"),
        pmet_etke: runtime_surface_symbol_value(runtime_surface, "pmet.etke"),
        pmet_etkr: runtime_surface_symbol_value(runtime_surface, "pmet.etkr"),
        pmet_etks: runtime_surface_symbol_value(runtime_surface, "pmet.etks"),
        pmet_tew_mm: runtime_surface_symbol_value(runtime_surface, "pmet.tew_mm"),
        pmet_rew_mm: runtime_surface_symbol_value(runtime_surface, "pmet.rew_mm"),
        pmet_wfevp_mm: runtime_surface_symbol_value(runtime_surface, "pmet.wfevp_mm"),
        pmet_taw_mm: runtime_surface_symbol_value(runtime_surface, "pmet.taw_mm"),
        pmet_raw_mm: runtime_surface_symbol_value(runtime_surface, "pmet.raw_mm"),
        pmet_wftrp_mm: runtime_surface_symbol_value(runtime_surface, "pmet.wftrp_mm"),
        pmet_es_m: runtime_surface_symbol_value(runtime_surface, "pmet.es_m"),
        pmet_ep_m: runtime_surface_symbol_value(runtime_surface, "pmet.ep_m"),
        etp_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Etp"),
        upi_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "UPi"),
        ui_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ui"),
        wb18_ul_layers_m,
        wb17_swu_stress_threshold_layers_m,
        wb17_swu_storage_to_threshold_layers,
        wb17_upi_layers_m: potential_uptake_layers_m,
        wb17_ui_layers_m: actual_uptake_layers_m,
        ep_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ep"),
        ws: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Ws"),
        wb19_q_lateral_potential_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_potential",
        ),
        wb19_q_lateral_target_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_target",
        ),
        wb19_lateral_capacity_tdv_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_lateral_capacity_tdv",
        ),
        wb19_tdvv_m: runtime_surface_symbol_value(runtime_surface, "wb19_tdvv"),
        wb19_q_lateral_unrealized_m: runtime_surface_symbol_value(
            runtime_surface,
            "wb19_q_lateral_unrealized",
        ),
        wb19_lateral_withdrawal_layers_m,
        wb19_lateral_capacity_active_count_layers,
        wb19_lateral_conductivity_active_count_layers,
        q_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "q"),
        qdd_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Qdd"),
        qd_m: runtime_surface_symbol_value_prefer_flux(runtime_surface, "Qd"),
    }
}

fn hphys0245_prefixed_surface_values(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    prefix: &str,
) -> BTreeMap<String, f64> {
    surface
        .iter()
        .filter_map(|(symbol, value)| {
            let symbol = symbol.as_str();
            symbol
                .strip_prefix(prefix)
                .map(|suffix| (suffix.to_string(), value.as_f64()))
        })
        .collect()
}

fn hphys0245_prefixed_surface_values_with_fallback(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    preferred_prefix: &str,
    fallback_prefix: &str,
) -> BTreeMap<String, f64> {
    let mut values = hphys0245_prefixed_surface_values(surface, fallback_prefix);
    values.extend(hphys0245_prefixed_surface_values(surface, preferred_prefix));
    values
}

fn hphys0245_prefixed_runtime_values(
    runtime_surface: &HillslopeWritebackSurface,
    prefix: &str,
) -> BTreeMap<String, f64> {
    let mut values = hphys0245_prefixed_surface_values(&runtime_surface.state_surface, prefix);
    values.extend(hphys0245_prefixed_surface_values(
        &runtime_surface.flux_surface,
        prefix,
    ));
    values
}

fn hphys0245_sum_runtime_prefix(runtime_surface: &HillslopeWritebackSurface, prefix: &str) -> f64 {
    hphys0245_prefixed_runtime_values(runtime_surface, prefix)
        .values()
        .copied()
        .sum()
}

fn hphys0245_sum_or_none(values: &BTreeMap<String, f64>) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.values().copied().sum())
    }
}

fn hphys0245_swu_stress_threshold_layers(
    ul_layers: &BTreeMap<String, f64>,
    effective_pltol: Option<f64>,
) -> BTreeMap<String, f64> {
    let Some(effective_pltol) = effective_pltol else {
        return BTreeMap::new();
    };
    if !effective_pltol.is_finite() || effective_pltol < 0.0 {
        return BTreeMap::new();
    }
    ul_layers
        .iter()
        .filter_map(|(suffix, ul)| {
            if ul.is_finite() && *ul >= 0.0 {
                let threshold = effective_pltol * *ul;
                if threshold.is_finite() {
                    return Some((suffix.clone(), threshold));
                }
            }
            None
        })
        .collect()
}

fn hphys0245_swu_storage_to_threshold_layers(
    theta_layers: &BTreeMap<String, f64>,
    threshold_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    threshold_layers
        .iter()
        .filter_map(|(suffix, threshold)| {
            if !threshold.is_finite() || *threshold <= 0.0 {
                return None;
            }
            let theta = theta_layers.get(suffix)?;
            let ratio = *theta / *threshold;
            if ratio.is_finite() {
                Some((suffix.clone(), ratio))
            } else {
                None
            }
        })
        .collect()
}

fn hphys0245_wb19_drfc_layers(
    fc_layers: &BTreeMap<String, f64>,
    dg_layers: &BTreeMap<String, f64>,
    coca_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    fc_layers
        .iter()
        .filter_map(|(suffix, fc)| {
            let dg = dg_layers.get(suffix)?;
            let coca = coca_layers.get(suffix)?;
            let drfc = *fc + ((1.0 - *coca) * *dg);
            if fc.is_finite() && dg.is_finite() && coca.is_finite() && drfc.is_finite() {
                Some((suffix.clone(), drfc))
            } else {
                None
            }
        })
        .collect()
}

fn hphys0245_wb19_fzdrfc_layers(
    drfc_layers: &BTreeMap<String, f64>,
    frzw_layers: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    drfc_layers
        .iter()
        .filter_map(|(suffix, drfc)| {
            let frzw = frzw_layers.get(suffix)?;
            let fzdrfc = (*drfc - frzw).max(0.0);
            if drfc.is_finite() && frzw.is_finite() && fzdrfc.is_finite() {
                Some((suffix.clone(), fzdrfc))
            } else {
                None
            }
        })
        .collect()
}

fn hphys0245_recompute_wb18_soil_water(
    theta_layers: &BTreeMap<String, f64>,
    thetdr_layers: &BTreeMap<String, f64>,
    dg_layers: &BTreeMap<String, f64>,
    frozen_depth_layers: &BTreeMap<String, f64>,
) -> Option<f64> {
    if theta_layers.is_empty() {
        return None;
    }
    let mut soil_water = 0.0;
    for (suffix, theta) in theta_layers {
        let thetdr = thetdr_layers.get(suffix)?;
        let dg = dg_layers.get(suffix)?;
        let frozen_depth = frozen_depth_layers.get(suffix).copied().unwrap_or(0.0);
        if !theta.is_finite()
            || !thetdr.is_finite()
            || !dg.is_finite()
            || !frozen_depth.is_finite()
            || *thetdr < 0.0
            || *dg <= 0.0
            || frozen_depth < 0.0
            || frozen_depth > *dg
        {
            return None;
        }
        let layer_soil_water = *theta + (*thetdr * (*dg - frozen_depth));
        if !layer_soil_water.is_finite() {
            return None;
        }
        soil_water += layer_soil_water;
    }
    if soil_water.is_finite() {
        Some(soil_water)
    } else {
        None
    }
}

fn format_wb12_storage_terms(runtime_surface: &HillslopeWritebackSurface) -> String {
    fn get(runtime_surface: &HillslopeWritebackSurface, symbol: &str) -> String {
        runtime_surface_symbol_value(runtime_surface, symbol)
            .map_or_else(|| "NA".to_string(), |value| format!("{value:.10}"))
    }

    let storage_initial = runtime_surface_symbol_value(runtime_surface, "wb12_storage_initial");
    let precip_input = runtime_surface_symbol_value(runtime_surface, "wb12_precip_input");
    let snow_coupling_s = runtime_surface_symbol_value(runtime_surface, "S");
    let irrigation_input = runtime_surface_symbol_value(runtime_surface, "Irr");
    let interception_i = runtime_surface_symbol_value(runtime_surface, "I");
    let q_runoff = runtime_surface_symbol_value(runtime_surface, "Q");
    let et = runtime_surface_symbol_value(runtime_surface, "ET");
    let percolation_loss = runtime_surface_symbol_value(runtime_surface, "D");
    let subsurface_loss = runtime_surface_symbol_value(runtime_surface, "Qd");
    let reconciled_est = match (
        storage_initial,
        precip_input,
        snow_coupling_s,
        irrigation_input,
        interception_i,
        q_runoff,
        et,
        percolation_loss,
        subsurface_loss,
    ) {
        (
            Some(storage_initial),
            Some(precip_input),
            Some(snow_coupling_s),
            Some(irrigation_input),
            Some(interception_i),
            Some(q_runoff),
            Some(et),
            Some(percolation_loss),
            Some(subsurface_loss),
        ) => format!(
            "{:.10}",
            storage_initial + precip_input + snow_coupling_s + irrigation_input
                - interception_i
                - q_runoff
                - et
                - percolation_loss
                - subsurface_loss
        ),
        _ => "NA".to_string(),
    };

    format!(
        "{{storage_initial={},precip_input={},S={},Irr={},I={},Q={},ET={},D={},Qd={},reconciled_est={}}}",
        get(runtime_surface, "wb12_storage_initial"),
        get(runtime_surface, "wb12_precip_input"),
        get(runtime_surface, "S"),
        get(runtime_surface, "Irr"),
        get(runtime_surface, "I"),
        get(runtime_surface, "Q"),
        get(runtime_surface, "ET"),
        get(runtime_surface, "D"),
        get(runtime_surface, "Qd"),
        reconciled_est
    )
}

fn format_wb18_perc_guard_terms(runtime_surface: &HillslopeWritebackSurface) -> String {
    let mut layer_suffixes = runtime_surface
        .state_surface
        .keys()
        .filter_map(|symbol| symbol.as_str().strip_prefix("wb18_perc_fc_"))
        .filter(|suffix| suffix.len() == 4 && suffix.chars().all(|ch| ch.is_ascii_digit()))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    layer_suffixes.sort_unstable();
    layer_suffixes.dedup();

    if layer_suffixes.is_empty() {
        return "{layers=none}".to_string();
    }

    let invalid_layers = layer_suffixes
        .iter()
        .filter_map(|suffix| {
            let fc = runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_fc_{suffix}"))?;
            let ul = runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_ul_{suffix}"))?;
            let theta =
                runtime_surface_symbol_value(runtime_surface, &format!("wb18_perc_theta_{suffix}"))?;
            let thetfc = runtime_surface_symbol_value(runtime_surface, &format!("thetfc_{suffix}"));
            let thetdr = runtime_surface_symbol_value(runtime_surface, &format!("thetdr_{suffix}"));
            let dg = runtime_surface_symbol_value(runtime_surface, &format!("dg_{suffix}"));
            let por = runtime_surface_symbol_value(runtime_surface, &format!("por_{suffix}"));
            let cpm = runtime_surface_symbol_value(runtime_surface, &format!("cpm_{suffix}"));
            let ratio = fc / ul;
            let stz = theta / ul;
            let dynamic_branch_active = stz.is_finite() && stz < 0.95;
            let ratio_domain_invalid = !ratio.is_finite() || ratio >= 1.0;
            let legacy_bi_zero_candidate = ratio.is_finite() && ratio <= 0.0;
            if !ratio_domain_invalid && !legacy_bi_zero_candidate {
                return None;
            }
            let fmt_opt = |value: Option<f64>| {
                value.map_or_else(|| "NA".to_string(), |observed| format!("{observed:.10}"))
            };
            Some(format!(
                "L{}(fc={:.10},ul={:.10},theta={:.10},ratio={:.10},stz={:.10},dynamic_branch_active={},ratio_domain_invalid={},legacy_bi_zero_candidate={},thetfc={},thetdr={},dg={},por={},cpm={})",
                suffix,
                fc,
                ul,
                theta,
                ratio,
                stz,
                dynamic_branch_active,
                ratio_domain_invalid,
                legacy_bi_zero_candidate,
                fmt_opt(thetfc),
                fmt_opt(thetdr),
                fmt_opt(dg),
                fmt_opt(por),
                fmt_opt(cpm),
            ))
        })
        .collect::<Vec<_>>();

    let invalid_summary = if invalid_layers.is_empty() {
        "none".to_string()
    } else {
        invalid_layers.join("|")
    };

    format!(
        "{{layer_count={},invalid_ratio_layers={}}}",
        layer_suffixes.len(),
        invalid_summary
    )
}

#[allow(clippy::too_many_lines)]
fn build_simimpl10_coupling_vector_provenance(
    runtime_surface: &HillslopeWritebackSurface,
    wb13_row: &SimulationOwnedWb13Row,
) -> Result<HillslopeCouplingVectorProvenance, HillslopeCliError> {
    let snow_file_present = parse_simimpl10_binary_flag(
        "snow.options.snow_file_present",
        require_simimpl10_coupling_scalar(runtime_surface, "snow.options.snow_file_present")?,
    )?;
    let rst = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.rst")?;
    let newsnw = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.newsnw")?;
    let ssd = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.ssd")?;
    let runtime_swe = wb13_row.wb13_row.snow_water / 1_000.0;

    if newsnw <= 0.0 {
        return Err(simcoup_failure(format!(
            "snow.options.newsnw must be > 0.0, observed {newsnw}"
        )));
    }
    if ssd <= 0.0 {
        return Err(simcoup_failure(format!(
            "snow.options.ssd must be > 0.0, observed {ssd}"
        )));
    }
    if newsnw > ssd {
        return Err(simcoup_failure(format!(
            "snow.options.newsnw must be <= snow.options.ssd, observed {newsnw} > {ssd}"
        )));
    }
    if runtime_swe < 0.0 {
        return Err(simcoup_failure(format!(
            "snow.runtime_swe must be >= 0.0, observed {runtime_swe}"
        )));
    }

    let frost_file_present = parse_simimpl10_binary_flag(
        "frost.options.frost_file_present",
        require_simimpl10_coupling_scalar(runtime_surface, "frost.options.frost_file_present")?,
    )?;
    let wint_red_enabled = parse_simimpl10_binary_flag(
        "frost.options.wintRed",
        require_simimpl10_coupling_scalar(runtime_surface, "frost.options.wintRed")?,
    )?;
    let dfrost = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_dfrost")?;
    let dthaw = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_dthaw")?;
    let nft = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_nft")?;
    let ws_frz = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_ws_frz")?;
    let infcap_frz =
        require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_infcap_frz")?;
    let ssc = require_simimpl10_coupling_scalar(runtime_surface, "ssc")?;
    let tmax = require_simimpl10_coupling_scalar(runtime_surface, "tmax")?;
    let tmin = require_simimpl10_coupling_scalar(runtime_surface, "tmin")?;
    let winter_active =
        runtime_swe > 0.0 || dfrost > 0.0 || ws_frz > 0.0 || f64::midpoint(tmax, tmin) < 0.0;

    let winter = HillslopeWinterCouplingProvenance {
        active: winter_active,
        snow_file_present,
        rst,
        newsnw,
        ssd,
        runtime_swe,
    };

    if !(0.0..=SIMIMPL10_FROST_MAX_DEPTH_M).contains(&dfrost) {
        return Err(simcoup_failure(format!(
            "frost.runtime_dfrost must be within [0.0,{SIMIMPL10_FROST_MAX_DEPTH_M}], observed {dfrost}"
        )));
    }
    if !(0.0..=SIMIMPL10_FROST_MAX_DEPTH_M).contains(&dthaw) {
        return Err(simcoup_failure(format!(
            "frost.runtime_dthaw must be within [0.0,{SIMIMPL10_FROST_MAX_DEPTH_M}], observed {dthaw}"
        )));
    }
    if nft < 0.0 {
        return Err(simcoup_failure(format!(
            "frost.runtime_nft must be >= 0.0, observed {nft}"
        )));
    }
    if ws_frz < 0.0 {
        return Err(simcoup_failure(format!(
            "frost.runtime_ws_frz must be >= 0.0, observed {ws_frz}"
        )));
    }
    if ssc < 0.0 {
        return Err(simcoup_failure(format!(
            "ssc must be >= 0.0 for frozen-soil coupling, observed {ssc}"
        )));
    }
    if infcap_frz < 0.0 || infcap_frz > ssc {
        return Err(simcoup_failure(format!(
            "frost.runtime_infcap_frz must be within [0.0,ssc], observed {infcap_frz} with ssc={ssc}"
        )));
    }

    let frsoil_active = frost_file_present && wint_red_enabled;
    let frsoil = HillslopeFrozenSoilCouplingProvenance {
        active: frsoil_active,
        frost_file_present,
        wint_red_enabled,
        dfrost,
        dthaw,
        nft,
        ws_frz,
        infcap_frz,
    };
    let soil = HillslopeSoilCouplingProvenance {
        ssc,
        infiltration_capacity_frozen: infcap_frz,
        infcap_within_ssc: infcap_frz <= ssc,
    };

    let total_soil = wb13_row.wb13_row.total_soil;
    let frozwt = wb13_row.wb13_row.frozwt;
    let snow_water = wb13_row.wb13_row.snow_water;
    let soil_water_total = wb13_row.wb13_row.soil_water_total;
    let closure_delta = soil_water_total - (total_soil + frozwt);
    let closure_within_tolerance = closure_delta.abs() <= SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM;
    if !closure_within_tolerance {
        return Err(simcoup_failure(format!(
            "hydout-equivalent closure violated: SoilWaterTotal - (Total-Soil + frozwt) = {closure_delta} exceeds tolerance {SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM}",
        )));
    }

    let hydout_equivalent = HillslopeHydoutEquivalentCouplingProvenance {
        source: WB13_PUBLICATION_SOURCE_SIMULATION_OWNED.to_string(),
        total_soil,
        frozwt,
        snow_water,
        soil_water_total,
        closure_delta,
        closure_tolerance: SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
        closure_within_tolerance,
    };

    Ok(HillslopeCouplingVectorProvenance {
        guard_id: SIMCOUP_GUARD_ID.to_string(),
        winter,
        soil,
        frsoil,
        hydout_equivalent,
    })
}

const MOFE04_PUBLICATION_OFE_POLICY: &str = "single-row-canonicalized-hillslope-aggregate";
const MOFE04_PUBLICATION_AREA_POLICY: &str = "sum-ofe-geometry-area";
const HPHYS0255_STORAGE_LINEAGE_POLICY: &str = "single-runtime-wb11-state";

fn build_wb13_publication_provenance(
    rows: &[SimulationOwnedWb13Row],
    contributor_ofe_count: usize,
    publication_area_m2: f64,
) -> Result<HillslopeWb13PublicationProvenance, HillslopeCliError> {
    let Some(first_row) = rows.first() else {
        return Err(wb13_simout_failure(
            "WB13 publication requires at least one executed-day row",
        ));
    };
    let Some(last_row) = rows.last() else {
        return Err(wb13_simout_failure(
            "WB13 publication requires at least one executed-day row",
        ));
    };
    if rows.iter().any(|row| row.sim_day_index <= 0) {
        return Err(wb13_simout_failure(
            "sim_day_index must be positive for every WB13 publication row",
        ));
    }
    if contributor_ofe_count == 0 {
        return Err(wb13_simout_failure(
            "contributor_ofe_count must be >= 1 for WB13 publication provenance",
        ));
    }
    if !publication_area_m2.is_finite() || publication_area_m2 <= 0.0 {
        return Err(wb13_simout_failure(format!(
            "publication_area_m2 must be finite and > 0.0, observed {publication_area_m2}"
        )));
    }
    if rows.iter().any(|row| row.wb13_row.ofe != 1) {
        return Err(wb13_simout_failure(
            "MOFE04 canonicalized publication policy requires WB13 OFE key = 1 for all rows",
        ));
    }
    let sim_day_index_monotonic = rows
        .windows(2)
        .all(|window| window[1].sim_day_index > window[0].sim_day_index);

    Ok(HillslopeWb13PublicationProvenance {
        source: WB13_PUBLICATION_SOURCE_SIMULATION_OWNED.to_string(),
        projection_fallback_used: false,
        guard_id: SIMOUT_GUARD_ID.to_string(),
        replay_candidate_surfaces: vec![
            WB13_REPLAY_CANDIDATE_SURFACE_WAT.to_string(),
            WB13_REPLAY_CANDIDATE_SURFACE_PASS.to_string(),
        ],
        publication_ofe_policy: MOFE04_PUBLICATION_OFE_POLICY.to_string(),
        contributor_ofe_count,
        area_policy: MOFE04_PUBLICATION_AREA_POLICY.to_string(),
        storage_lineage_policy: HPHYS0255_STORAGE_LINEAGE_POLICY.to_string(),
        publication_area_m2,
        row_count: rows.len(),
        sim_day_index_monotonic,
        first_row_key: wb13_row_key_provenance(first_row),
        last_row_key: wb13_row_key_provenance(last_row),
    })
}

fn build_mofe_hourly_carry_provenance(
    runtime_surface: &HillslopeWritebackSurface,
    contributor_ofe_count: usize,
) -> Result<HillslopeMofeHourlyCarryProvenance, HillslopeCliError> {
    let active = contributor_ofe_count > 1;
    let upstream_carry_total_m = if active {
        sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT,
            true,
        )? + sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
            true,
        )?
    } else {
        0.0
    };
    let current_carry_total_m = if active {
        sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
            true,
        )? + sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
            true,
        )?
    } else {
        0.0
    };

    Ok(HillslopeMofeHourlyCarryProvenance {
        policy: MOFE_HOURLY_CARRY_POLICY.to_string(),
        active,
        substep_count: MOFE_HOURLY_CARRY_ARRAY_COUNT,
        required_arrays: MOFE_HOURLY_REQUIRED_ARRAYS
            .iter()
            .map(|root| (*root).to_string())
            .collect(),
        upstream_carry_total_m,
        current_carry_total_m,
    })
}

fn sum_mofe_hourly_carry_array(
    runtime_surface: &HillslopeWritebackSurface,
    root: &str,
    required: bool,
) -> Result<f64, HillslopeCliError> {
    let mut total = 0.0_f64;
    for hour in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
        let symbol = mofe_hourly_carry_hour_symbol(root, hour);
        let Some(value) = runtime_surface_symbol_value(runtime_surface, &symbol) else {
            if required {
                return Err(mofe_hourly_carry_failure(format!(
                    "missing required runtime symbol {symbol}"
                )));
            }
            continue;
        };
        require_mofe_hourly_carry_non_negative(value, &symbol)?;
        total += value;
    }
    require_mofe_hourly_carry_non_negative(total, root)?;
    Ok(total)
}

fn scheduler_outcome_class_as_str(outcome_class: SchedulerOutcomeClass) -> &'static str {
    match outcome_class {
        SchedulerOutcomeClass::Completed => "completed",
        SchedulerOutcomeClass::TopologyPreconditionFailed => "topology_precondition_failed",
        SchedulerOutcomeClass::PhaseFailure => "phase_failure",
        SchedulerOutcomeClass::SchedulerInvariantFailure => "scheduler_invariant_failure",
    }
}

fn wb13_row_key_provenance(row: &SimulationOwnedWb13Row) -> HillslopeWb13RowKeyProvenance {
    HillslopeWb13RowKeyProvenance {
        year: row.wb13_row.year,
        julian_day: row.wb13_row.julian_day,
        ofe: row.wb13_row.ofe,
        sim_day_index: row.sim_day_index,
    }
}

const HBP_MAGIC: &[u8; 8] = b"WFPHBP01";
const HBP_FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
const HBP_SUPPORTED_MAJOR_V1: u16 = 1;
const HBP_DIM_SCALAR: u8 = 0;
const HBP_DIM_NOFE: u8 = 1;
const HBP_DIM_NOFE_LAYERS: u8 = 2;
const HBP_DEFAULT_CALENDAR_YEAR: i32 = 2004;
const HBP_DEFAULT_PARTICLE_DIAMETER_M: f64 = 0.001;
const HBP_SCALE_INV_I64: f64 = 1.0e9;
const HBP_I64_MIN_F64: f64 = -9_223_372_036_854_775_808.0;
const HBP_I64_MAX_F64: f64 = 9_223_372_036_854_775_807.0;
const HBP_REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];

#[derive(Clone, Copy)]
struct HbpEventFixtureInput {
    hillslope_id: u32,
    nofe: u16,
    julian_day: u16,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    sediment_concentration_kg_m3: f64,
    particle_flow_fraction: f64,
    particle_diameter_m: f64,
}

#[derive(Clone, Copy)]
struct HbpEventPayloadInput {
    nofe: u16,
    sim_year_index: u32,
    calendar_year: i32,
    julian_day: u16,
    peak_runoff_m3_s: f64,
    duration_seconds: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    sediment_concentration_kg_m3: f64,
    particle_flow_fraction: f64,
}

#[derive(Clone, Copy)]
struct HbpHeaderInput {
    schema_major: u16,
    schema_minor: u16,
    hillslope_id: u32,
    nofe: u16,
    nyear: u32,
    begin_year: i32,
    julian_day: u16,
    particle_diameter_m: f64,
}

fn build_hbp_output(
    output_pass: &Path,
    wb13_rows: &[SimulationOwnedWb13Row],
    runtime_surface: &HillslopeWritebackSurface,
    contributor_ofe_count: usize,
) -> Result<Vec<u8>, HillslopeCliError> {
    if wb13_rows.is_empty() {
        return Err(wb13_simout_failure(
            "WB13 surface emission requires at least one executed-day row",
        ));
    }

    let hillslope_id = parse_hillslope_id_from_output_pass_path(output_pass)?;
    let nofe = u16::try_from(contributor_ofe_count).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} contributor_ofe_count out of u16 range for HBP emission: {contributor_ofe_count}"
            ),
        }
    })?;
    if nofe == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} contributor_ofe_count must be >= 1 for HBP emission"
            ),
        });
    }

    let latest_row = wb13_rows
        .last()
        .ok_or_else(|| wb13_simout_failure("missing latest executed-day row for HBP emission"))?;

    let peak_runoff_m3_s = optional_non_negative_runtime_scalar(runtime_surface, "peakro", 0.0)?;
    let duration_seconds = optional_non_negative_runtime_scalar(runtime_surface, "watdur", 0.0)?;
    let total_detachment_kg =
        optional_non_negative_runtime_scalar(runtime_surface, "total_detachment_kg", 0.0)?;
    let total_deposition_kg =
        optional_non_negative_runtime_scalar(runtime_surface, "total_deposition_kg", 0.0)?;
    let sediment_concentration_kg_m3 = optional_non_negative_runtime_scalar(
        runtime_surface,
        "sediment_concentration_kg_m3_0001",
        0.0,
    )?;
    let particle_flow_fraction = 1.0;

    build_schema1_hbp_event_fixture(HbpEventFixtureInput {
        hillslope_id,
        nofe,
        julian_day: latest_row.wb13_row.julian_day,
        peak_runoff_m3_s,
        duration_seconds,
        total_detachment_kg,
        total_deposition_kg,
        sediment_concentration_kg_m3,
        particle_flow_fraction,
        particle_diameter_m: HBP_DEFAULT_PARTICLE_DIAMETER_M,
    })
}

fn parse_hillslope_id_from_output_pass_path(path: &Path) -> Result<u32, HillslopeCliError> {
    let file_name = file_name_string(path);
    let stem = file_name
        .strip_suffix(".hbp")
        .or_else(|| file_name.strip_suffix(".HBP"))
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} outputs.pass must use .hbp extension, observed {}",
                path.display()
            ),
        })?;
    let Some(id_text) = stem.strip_prefix('H').or_else(|| stem.strip_prefix('h')) else {
        return Ok(1);
    };
    if id_text.is_empty() || !id_text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(1);
    }

    let hillslope_id =
        id_text
            .parse::<u32>()
            .map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} outputs.pass hillslope id is not a valid u32: {id_text}"
                ),
            })?;
    if hillslope_id == 0 {
        return Ok(1);
    }

    Ok(hillslope_id)
}

fn optional_non_negative_runtime_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
    default_value: f64,
) -> Result<f64, HillslopeCliError> {
    let value = runtime_surface_symbol_value(runtime_surface, symbol).unwrap_or(default_value);
    if !value.is_finite() || value < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} runtime symbol {symbol} must be finite and >= 0.0 for HBP emission, observed {value}"
            ),
        });
    }
    Ok(value)
}

fn build_schema1_hbp_event_fixture(
    input: HbpEventFixtureInput,
) -> Result<Vec<u8>, HillslopeCliError> {
    let mut file = append_hbp_common_prefix(HbpHeaderInput {
        schema_major: HBP_SUPPORTED_MAJOR_V1,
        schema_minor: 0,
        hillslope_id: input.hillslope_id,
        nofe: input.nofe,
        nyear: 1,
        begin_year: HBP_DEFAULT_CALENDAR_YEAR,
        julian_day: input.julian_day,
        particle_diameter_m: input.particle_diameter_m,
    })?;
    let payload = build_hbp_event_payload(HbpEventPayloadInput {
        nofe: input.nofe,
        sim_year_index: 1,
        calendar_year: HBP_DEFAULT_CALENDAR_YEAR,
        julian_day: input.julian_day,
        peak_runoff_m3_s: input.peak_runoff_m3_s,
        duration_seconds: input.duration_seconds,
        total_detachment_kg: input.total_detachment_kg,
        total_deposition_kg: input.total_deposition_kg,
        sediment_concentration_kg_m3: input.sediment_concentration_kg_m3,
        particle_flow_fraction: input.particle_flow_fraction,
    })?;
    let payload_crc = crc32c(&payload);

    let directory_start = file.len();
    let directory_len = 4 + 27;
    let payload_offset = directory_start + directory_len;
    let payload_offset_u64 =
        u64::try_from(payload_offset).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!("{SIMOUT_GUARD_ID} HBP payload offset exceeds u64: {payload_offset}"),
        })?;
    let payload_len_u32 =
        u32::try_from(payload.len()).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP payload length exceeds u32: {}",
                payload.len()
            ),
        })?;
    let mut directory = Vec::new();
    put_u32(&mut directory, 1);
    put_u32(&mut directory, 1);
    put_i32(&mut directory, HBP_DEFAULT_CALENDAR_YEAR);
    put_u16(&mut directory, input.julian_day);
    put_u8(&mut directory, 2);
    put_u64(&mut directory, payload_offset_u64);
    put_u32(&mut directory, payload_len_u32);
    put_u32(&mut directory, payload_crc);

    file.extend_from_slice(&directory);
    file.extend_from_slice(&payload);

    let directory_crc = crc32c(&directory);
    put_u32(&mut file, directory_crc);
    let file_crc_pos = file.len();
    put_u32(&mut file, 0);
    put_u32(&mut file, 1);
    file.extend_from_slice(HBP_FOOTER_MAGIC);
    let file_crc = crc32c(&file);
    put_u32_at(&mut file, file_crc_pos, file_crc);
    Ok(file)
}

fn build_hbp_event_payload(input: HbpEventPayloadInput) -> Result<Vec<u8>, HillslopeCliError> {
    let nofe = u32::from(input.nofe);
    let max_layers = 1u32;

    let mut payload = Vec::new();
    put_u32(&mut payload, input.sim_year_index);
    put_i32(&mut payload, input.calendar_year);
    put_u16(&mut payload, input.julian_day);
    put_u8(&mut payload, 2);
    put_u16(&mut payload, 0);
    put_u16(
        &mut payload,
        u16::try_from(HBP_REQUIRED_STATE_IDS.len()).map_err(|_| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} HBP state registry length exceeds u16: {}",
                    HBP_REQUIRED_STATE_IDS.len()
                ),
            }
        })?,
    );
    put_f64(&mut payload, input.duration_seconds);
    put_f64(&mut payload, 0.5);
    put_f64(&mut payload, 0.8);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);
    put_f64(&mut payload, input.peak_runoff_m3_s);
    put_i64(&mut payload, scaled_i64(input.total_detachment_kg)?);
    put_i64(&mut payload, scaled_i64(input.total_deposition_kg)?);
    put_u32(&mut payload, 1);
    put_f64(&mut payload, input.sediment_concentration_kg_m3);
    put_u32(&mut payload, 1);
    put_f64(&mut payload, input.particle_flow_fraction);
    put_i64(&mut payload, 0);
    put_i64(&mut payload, 0);

    for state_id in HBP_REQUIRED_STATE_IDS {
        payload.extend_from_slice(&build_hbp_state_entry(*state_id, nofe, max_layers)?);
    }

    Ok(payload)
}

fn append_hbp_common_prefix(input: HbpHeaderInput) -> Result<Vec<u8>, HillslopeCliError> {
    let mut file = Vec::new();

    let mut header = Vec::new();
    header.extend_from_slice(HBP_MAGIC);
    put_u16(&mut header, input.schema_major);
    put_u16(&mut header, input.schema_minor);
    put_u8(&mut header, 1);
    let header_bytes_pos = header.len();
    put_u32(&mut header, 0);
    header.extend_from_slice(&[0u8; 32]);
    put_u8(&mut header, 1);
    put_string(&mut header, "openwepp-hillslope-cli")?;
    put_string(&mut header, "hs-cli")?;
    put_string(&mut header, "2026-05-29T00:00:00Z")?;
    put_string(&mut header, "metric-v1")?;
    header.extend_from_slice(&[0u8; 32]);
    let header_crc_pos = header.len();
    put_u32(&mut header, 0);
    let header_bytes =
        u32::try_from(header.len()).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP header byte count exceeds u32: {}",
                header.len()
            ),
        })?;
    put_u32_at(&mut header, header_bytes_pos, header_bytes);
    let header_crc = crc32c(&header);
    put_u32_at(&mut header, header_crc_pos, header_crc);
    file.extend_from_slice(&header);

    let npart = 1u16;
    let max_layers = 1u16;

    put_u32(&mut file, input.hillslope_id);
    put_u32(&mut file, input.nyear);
    put_i32(&mut file, input.begin_year);
    put_u16(&mut file, npart);
    put_u16(&mut file, input.nofe);
    put_u16(&mut file, max_layers);
    put_string(&mut file, "gregorian")?;
    put_u16(&mut file, 1);
    put_u8(&mut file, 1);

    put_string(&mut file, "p1.cli")?;
    put_i64(&mut file, 0);
    put_u32(&mut file, u32::from(npart));
    put_f64(&mut file, input.particle_diameter_m);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);
    put_f64(&mut file, 0.0);

    put_u32(&mut file, input.nyear);
    put_u32(&mut file, 1);
    put_i32(&mut file, input.begin_year);
    put_u16(&mut file, 1);
    put_u16(&mut file, input.julian_day);
    put_u16(&mut file, input.julian_day);
    put_u8(&mut file, 0);

    put_u32(
        &mut file,
        u32::try_from(HBP_REQUIRED_STATE_IDS.len()).map_err(|_| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} HBP state registry count exceeds u32: {}",
                    HBP_REQUIRED_STATE_IDS.len()
                ),
            }
        })?,
    );
    for state_id in HBP_REQUIRED_STATE_IDS {
        let (required_flag, representation_class, unit_class, rank, dims_kind) =
            expected_hbp_state_schema(*state_id).ok_or_else(|| {
                HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "outputs.pass",
                    detail: format!(
                        "{SIMOUT_GUARD_ID} missing HBP state schema for required state {state_id}"
                    ),
                }
            })?;
        put_u16(&mut file, *state_id);
        put_u8(&mut file, required_flag);
        put_u8(&mut file, representation_class);
        put_u16(&mut file, unit_class);
        put_u8(&mut file, rank);
        put_u8(&mut file, dims_kind);
        put_string(&mut file, &format!("state_{state_id}"))?;
    }

    Ok(file)
}

fn expected_hbp_state_schema(state_id: u16) -> Option<(u8, u8, u16, u8, u8)> {
    match state_id {
        1 => Some((1, 1, 1, 1, HBP_DIM_NOFE)),
        2 | 3 | 4 | 5 | 100 | 101 | 102 | 210 | 900 | 901 => {
            Some((1, 1, 2, 2, HBP_DIM_NOFE_LAYERS))
        }
        6 | 7 => Some((1, 2, 3, 2, HBP_DIM_NOFE_LAYERS)),
        103 | 104 | 200 | 202 | 203 | 204 | 205 | 206 | 207 | 208 | 209 => {
            Some((1, 1, 2, 1, HBP_DIM_NOFE))
        }
        201 => Some((1, 2, 4, 1, HBP_DIM_NOFE)),
        300 => Some((1, 1, 5, 0, HBP_DIM_SCALAR)),
        _ => None,
    }
}

fn build_hbp_state_entry(
    state_id: u16,
    nofe: u32,
    max_layers: u32,
) -> Result<Vec<u8>, HillslopeCliError> {
    let (required_flag, representation_class, unit_class, rank, dims_kind) =
        expected_hbp_state_schema(state_id).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass",
                detail: format!(
                    "{SIMOUT_GUARD_ID} missing HBP state schema for required state {state_id}"
                ),
            }
        })?;
    let dims = hbp_state_dims(dims_kind, nofe, max_layers);
    assert_eq!(dims.len(), usize::from(rank));

    let mut entry = Vec::new();
    put_u8(&mut entry, required_flag);
    put_u8(&mut entry, representation_class);
    put_u16(&mut entry, unit_class);
    put_u8(&mut entry, rank);
    for dim in &dims {
        put_u32(&mut entry, *dim);
    }

    let value_count = dims.iter().copied().product::<u32>().max(1) as usize;
    match representation_class {
        1 => {
            for _ in 0..value_count {
                put_i64(&mut entry, 0);
            }
        }
        2 => {
            for _ in 0..value_count {
                put_f64(&mut entry, 0.0);
            }
        }
        _ => panic!("unsupported representation class"),
    }

    let mut out = Vec::new();
    put_u16(&mut out, state_id);
    put_u32(
        &mut out,
        u32::try_from(entry.len()).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP state entry byte count exceeds u32 for state {state_id}: {}",
                entry.len()
            ),
        })?,
    );
    out.extend_from_slice(&entry);
    Ok(out)
}

fn hbp_state_dims(dims_kind: u8, nofe: u32, max_layers: u32) -> Vec<u32> {
    match dims_kind {
        HBP_DIM_SCALAR => vec![],
        HBP_DIM_NOFE => vec![nofe],
        HBP_DIM_NOFE_LAYERS => vec![nofe, max_layers],
        _ => panic!("unknown dims_kind {dims_kind}"),
    }
}

fn scaled_i64(value: f64) -> Result<i64, HillslopeCliError> {
    let scaled = value * HBP_SCALE_INV_I64;
    if !scaled.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!("{SIMOUT_GUARD_ID} HBP scaled integer is not finite for value {value}"),
        });
    }
    let rounded = scaled.round();
    if !(HBP_I64_MIN_F64..=HBP_I64_MAX_F64).contains(&rounded) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!("{SIMOUT_GUARD_ID} HBP scaled integer overflow for value {value}"),
        });
    }
    let rounded_text = format!("{rounded:.0}");
    rounded_text
        .parse::<i64>()
        .map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!("{SIMOUT_GUARD_ID} HBP scaled integer parse failure for value {value}"),
        })
}

fn put_u8(buf: &mut Vec<u8>, value: u8) {
    buf.push(value);
}

fn put_u16(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_u32(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_i32(buf: &mut Vec<u8>, value: i32) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(buf: &mut Vec<u8>, value: u64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_i64(buf: &mut Vec<u8>, value: i64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_f64(buf: &mut Vec<u8>, value: f64) {
    buf.extend_from_slice(&value.to_le_bytes());
}

fn put_string(buf: &mut Vec<u8>, value: &str) -> Result<(), HillslopeCliError> {
    put_u32(
        buf,
        u32::try_from(value.len()).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass",
            detail: format!(
                "{SIMOUT_GUARD_ID} HBP string length exceeds u32: {}",
                value.len()
            ),
        })?,
    );
    buf.extend_from_slice(value.as_bytes());
    Ok(())
}

fn put_u32_at(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn crc32c(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for value in data {
        crc ^= u32::from(*value);
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0x82F6_3B78;
            } else {
                crc >>= 1;
            }
            crc &= 0xFFFF_FFFF;
        }
    }
    crc ^ 0xFFFF_FFFF
}

fn build_hillslope_wat_rows(
    wb13_rows: &[SimulationOwnedWb13Row],
) -> Result<Vec<HillslopeWatRow>, HillslopeCliError> {
    let mut rows = Vec::with_capacity(wb13_rows.len());
    for wb13_row in wb13_rows {
        rows.push(build_hillslope_wat_row(wb13_row)?);
    }
    Ok(rows)
}

fn build_hillslope_wat_row(
    wb13_row: &SimulationOwnedWb13Row,
) -> Result<HillslopeWatRow, HillslopeCliError> {
    if wb13_row.sim_day_index <= 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!(
                "{SIMOUT_GUARD_ID} sim_day_index must be >= 1, observed {}",
                wb13_row.sim_day_index
            ),
        });
    }
    let year = i16::try_from(wb13_row.wb13_row.year).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!(
                "{SIMOUT_GUARD_ID} year out of i16 range: {}",
                wb13_row.wb13_row.year
            ),
        }
    })?;
    let julian = i16::try_from(wb13_row.wb13_row.julian_day).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!(
                "{SIMOUT_GUARD_ID} julian out of i16 range: {}",
                wb13_row.wb13_row.julian_day
            ),
        }
    })?;
    let ofe = i16::try_from(wb13_row.wb13_row.ofe).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!(
                "{SIMOUT_GUARD_ID} OFE out of i16 range: {}",
                wb13_row.wb13_row.ofe
            ),
        }
    })?;

    Ok(HillslopeWatRow {
        wepp_id: 1,
        ofe_id: ofe,
        year,
        sim_day_index: wb13_row.sim_day_index,
        julian,
        month: wb13_row.month,
        day_of_month: wb13_row.day_of_month,
        water_year: wb13_row.water_year,
        ofe,
        p: wb13_row.wb13_row.p,
        rm: wb13_row.wb13_row.rm,
        q: wb13_row.wb13_row.q,
        ep: wb13_row.wb13_row.ep,
        es: wb13_row.wb13_row.es,
        er: wb13_row.wb13_row.er,
        dp: wb13_row.wb13_row.dp,
        up_strm_q: wb13_row.wb13_row.upstrmq,
        sub_r_in: wb13_row.wb13_row.subrin,
        latqcc: wb13_row.wb13_row.latqcc,
        total_soil_water: wb13_row.wb13_row.total_soil,
        frozwt: wb13_row.wb13_row.frozwt,
        snow_water: wb13_row.wb13_row.snow_water,
        qofe: wb13_row.wb13_row.qofe,
        tile: wb13_row.wb13_row.tile,
        irr: wb13_row.wb13_row.irr,
        area: wb13_row.wb13_row.area,
        soil_water_total: Some(wb13_row.wb13_row.soil_water_total),
        profile_depth: Some(wb13_row.wb13_row.profile_depth),
        profile_porosity_cap: Some(wb13_row.wb13_row.profile_porosity_cap),
        profile_fc_store: Some(wb13_row.wb13_row.profile_fc_store),
        profile_wp_store: Some(wb13_row.wb13_row.profile_wp_store),
        interception_storage: None,
    })
}

fn derive_mofe04_publication_area_from_slope(
    slope: &SlopeProfile,
) -> Result<f64, HillslopeCliError> {
    if slope.ofes.is_empty() {
        return Err(wb13_simout_failure(
            "slope profile contains no OFE entries for Area derivation",
        ));
    }

    let mut area = 0.0_f64;
    for (ofe_position, ofe) in slope.ofes.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        if !ofe.fwidth.is_finite() || ofe.fwidth <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "OFE {ofe_index} fwidth must be > 0.0, observed {}",
                ofe.fwidth
            )));
        }
        if !ofe.slplen.is_finite() || ofe.slplen <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "OFE {ofe_index} slplen must be > 0.0, observed {}",
                ofe.slplen
            )));
        }

        area += ofe.fwidth * ofe.slplen;
    }

    if !area.is_finite() || area <= 0.0 {
        return Err(wb13_simout_failure(format!(
            "aggregate OFE Area must be > 0.0, observed {area}"
        )));
    }

    Ok(area)
}

fn derive_profile_fc_store_from_authoritative_layers(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    let nsl_symbol = if runtime_surface_symbol_value(runtime_surface, "wb11_nsl").is_some() {
        "wb11_nsl"
    } else {
        "nsl"
    };
    let nsl = scalar_to_usize(
        nsl_symbol,
        require_runtime_surface_scalar(runtime_surface, nsl_symbol)?,
    )?;
    if nsl == 0 {
        return Err(wb13_simout_failure(
            "nsl must be >= 1 for ProfileFCStore layer aggregation",
        ));
    }

    let mut profile_fc_store_m = 0.0_f64;
    for layer_index in 1..=nsl {
        let preferred_thetfc_symbol = format!("wb19_thetfc_{layer_index:04}");
        let legacy_thetfc_symbol = format!("thetfc_{layer_index:04}");
        let thetfc_symbol =
            if runtime_surface_symbol_value(runtime_surface, &preferred_thetfc_symbol).is_some() {
                preferred_thetfc_symbol
            } else {
                legacy_thetfc_symbol
            };
        let preferred_dg_symbol = format!("wb19_dg_{layer_index:04}");
        let legacy_dg_symbol = format!("dg_{layer_index:04}");
        let dg_symbol =
            if runtime_surface_symbol_value(runtime_surface, &preferred_dg_symbol).is_some() {
                preferred_dg_symbol
            } else {
                legacy_dg_symbol
            };
        let thetfc = require_runtime_surface_scalar(runtime_surface, &thetfc_symbol)?;
        let dg = require_runtime_surface_scalar(runtime_surface, &dg_symbol)?;
        if thetfc < 0.0 {
            return Err(wb13_simout_failure(format!(
                "{thetfc_symbol} must be >= 0.0, observed {thetfc}"
            )));
        }
        if dg <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "{dg_symbol} must be > 0.0, observed {dg}"
            )));
        }
        profile_fc_store_m += thetfc * dg;
    }

    if !profile_fc_store_m.is_finite() || profile_fc_store_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "ProfileFCStore layer aggregation must be finite and >= 0.0, observed {profile_fc_store_m}"
        )));
    }
    let profile_fc_tail_mm =
        require_runtime_surface_scalar(runtime_surface, "wb13_profile_fc_tail_mm")?;
    if !profile_fc_tail_mm.is_finite() {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_fc_tail_mm must be finite, observed {profile_fc_tail_mm}"
        )));
    }
    if profile_fc_tail_mm < 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_fc_tail_mm must be >= 0.0, observed {profile_fc_tail_mm}"
        )));
    }

    let profile_fc_store_mm = profile_fc_store_m * 1_000.0 + profile_fc_tail_mm;
    if !profile_fc_store_mm.is_finite() || profile_fc_store_mm < 0.0 {
        return Err(wb13_simout_failure(format!(
            "ProfileFCStore combined layer+tail storage must be finite and >= 0.0, observed {profile_fc_store_mm}"
        )));
    }

    Ok(profile_fc_store_mm)
}

#[allow(clippy::too_many_lines)]
fn build_simulation_owned_wb13_row(
    runtime_surface: &HillslopeWritebackSurface,
    publication_area_m2: f64,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_day: &ClimateDayProjection,
    runtime_swe_before_m: f64,
) -> Result<SimulationOwnedWb13Row, HillslopeCliError> {
    if simulation_year <= 0 {
        return Err(wb13_simout_failure(format!(
            "simulation-year key must be >= 1, observed {simulation_year}"
        )));
    }

    let calendar_year = calendar_day.year;
    let month = calendar_day.month;
    let day_of_month = calendar_day.day_of_month;
    let julian_day = day_of_year(calendar_year, month, day_of_month)?;
    if julian_day != calendar_day.julian_day {
        return Err(wb13_simout_failure(format!(
            "calendar day projection mismatch: computed julian {julian_day} differs from projected {}",
            calendar_day.julian_day
        )));
    }

    let precipitation_m = require_runtime_surface_scalar(runtime_surface, "prcp")?;
    if precipitation_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "precipitation symbol prcp must be >= 0.0, observed {precipitation_m}"
        )));
    }
    let precipitation_mm = precipitation_m * 1_000.0;

    let _tmax = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let _tmin = require_runtime_surface_scalar(runtime_surface, "tmin")?;

    let profile_depth_mm =
        require_runtime_surface_scalar(runtime_surface, "wb13_profile_depth_mm")?;
    if profile_depth_mm <= 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_depth_mm must be > 0.0, observed {profile_depth_mm}"
        )));
    }
    let profile_porosity_cap =
        require_runtime_surface_scalar(runtime_surface, "wb13_profile_porosity_cap_mm")?;
    if profile_porosity_cap < 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_porosity_cap_mm must be >= 0.0, observed {profile_porosity_cap}"
        )));
    }
    let profile_fc_store_mm = derive_profile_fc_store_from_authoritative_layers(runtime_surface)?;
    let profile_wp_store_mm =
        require_runtime_surface_scalar(runtime_surface, "wb13_profile_wp_store_mm")?;
    if profile_wp_store_mm < 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb13_profile_wp_store_mm must be >= 0.0, observed {profile_wp_store_mm}"
        )));
    }
    if profile_porosity_cap < profile_fc_store_mm {
        return Err(wb13_simout_failure(format!(
            "profile storage ordering invalid: ProfilePorosityCap ({profile_porosity_cap}) must be >= ProfileFCStore ({profile_fc_store_mm})"
        )));
    }
    if profile_fc_store_mm < profile_wp_store_mm {
        return Err(wb13_simout_failure(format!(
            "profile storage ordering invalid: ProfileFCStore ({profile_fc_store_mm}) must be >= ProfileWPStore ({profile_wp_store_mm})"
        )));
    }

    // SIMIMPL24 publication authority: Total-Soil must be WB11 runtime
    // aggregate lineage only (`wb11_soil_water` -> `watcon` -> `Total-Soil`).
    let wb11_soil_water_m = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    if wb11_soil_water_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "wb11_soil_water must be >= 0.0, observed {wb11_soil_water_m}"
        )));
    }
    let total_soil = wb11_soil_water_m * 1_000.0;

    let frozwt_m = require_runtime_surface_scalar(runtime_surface, "frost.runtime_ws_frz")?;
    if frozwt_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "frost.runtime_ws_frz must be >= 0.0, observed {frozwt_m}"
        )));
    }
    let frozwt = frozwt_m * 1_000.0;

    let runtime_swe_m = require_runtime_surface_scalar(runtime_surface, "snow.runtime_swe")?;
    if runtime_swe_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "snow.runtime_swe must be >= 0.0, observed {runtime_swe_m}"
        )));
    }
    let snow_water = runtime_swe_m * 1_000.0;

    let irrigation_m = require_runtime_surface_scalar(runtime_surface, "Irr")?;
    if irrigation_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Irr must be >= 0.0, observed {irrigation_m}"
        )));
    }
    let rm_m = precipitation_m + runtime_swe_before_m - runtime_swe_m + irrigation_m;
    if rm_m < -1.0e-12 {
        return Err(wb13_simout_failure(format!(
            "RM source (prcp + SWE_before - SWE_after + Irr) must be >= 0.0, observed {rm_m}"
        )));
    }
    let rm = rm_m.max(0.0) * 1_000.0;
    let irrigation_mm = irrigation_m * 1_000.0;

    let q_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Q")?;
    if q_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Q must be >= 0.0, observed {q_m}"
        )));
    }
    let transpiration_ep_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Ep")?;
    if transpiration_ep_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Ep must be >= 0.0, observed {transpiration_ep_m}"
        )));
    }
    let evappm_pmet_branch =
        runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_evappm")
            .is_some_and(|value| value >= 0.5);
    let soil_evap_es_m_raw = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Es")?;
    if soil_evap_es_m_raw < -1.0e-12 {
        return Err(wb13_simout_failure(format!(
            "Es must be >= 0.0 within tolerance, observed {soil_evap_es_m_raw}"
        )));
    }
    let soil_evap_es_m = if soil_evap_es_m_raw < 0.0 {
        0.0
    } else {
        soil_evap_es_m_raw
    };
    let residue_evap_er_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Er")?;
    if residue_evap_er_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Er must be >= 0.0, observed {residue_evap_er_m}"
        )));
    }
    let dp_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "D")?;
    if dp_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "D must be >= 0.0, observed {dp_m}"
        )));
    }
    let latqcc_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "q")?;
    if latqcc_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "q must be >= 0.0, observed {latqcc_m}"
        )));
    }
    let tile_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Qdd")?;
    if tile_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Qdd must be >= 0.0, observed {tile_m}"
        )));
    }
    let qd_source_m = require_runtime_surface_scalar_prefer_flux(runtime_surface, "Qd")?;
    if qd_source_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Qd must be >= 0.0, observed {qd_source_m}"
        )));
    }
    let sub_r_in_m = runtime_surface_symbol_value(runtime_surface, "SubRIn").unwrap_or(0.0);
    if sub_r_in_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "SubRIn must be >= 0.0, observed {sub_r_in_m}"
        )));
    }
    let q = q_m * 1_000.0;
    let ep = transpiration_ep_m * 1_000.0;
    let es = soil_evap_es_m * 1_000.0;
    let er = residue_evap_er_m * 1_000.0;
    let dp = dp_m * 1_000.0;
    let latqcc = latqcc_m * 1_000.0;
    let tile = tile_m * 1_000.0;
    let qd = qd_source_m * 1_000.0;
    let sub_r_in = sub_r_in_m * 1_000.0;
    if (qd - (latqcc + tile)).abs() > 1.0e-6 {
        return Err(wb13_simout_failure(format!(
            "Qd coupling closure violated: Qd ({qd}) must equal latqcc + Tile ({})",
            latqcc + tile
        )));
    }
    let area = publication_area_m2;
    let soil_water_total = total_soil + frozwt;

    let row_surface = SummaryScalarSurface::from_pairs([
        ("P", precipitation_mm),
        ("RM", rm),
        ("Q", q),
        ("Ep", ep),
        ("Es", es),
        ("Er", er),
        ("Dp", dp),
        ("UpStrmQ", 0.0),
        ("SubRIn", sub_r_in),
        ("latqcc", latqcc),
        ("Total-Soil", total_soil),
        ("frozwt", frozwt),
        ("Snow-Water", snow_water),
        ("QOFE", q),
        ("Tile", tile),
        ("Irr", irrigation_mm),
        ("Area", area),
        (
            "wb11_et_seed_branch_evappm",
            if evappm_pmet_branch { 1.0 } else { 0.0 },
        ),
        ("SoilWaterTotal", soil_water_total),
        ("ProfileDepth", profile_depth_mm),
        ("ProfilePorosityCap", profile_porosity_cap),
        ("ProfileFCStore", profile_fc_store_mm),
        ("ProfileWPStore", profile_wp_store_mm),
    ])
    .map_err(|error| {
        wb13_simout_failure(format!("failed building WB13 scalar surface: {error}"))
    })?;

    let wb13_row =
        Wb13DailyWaterBalanceRow::from_surface(1, julian_day, simulation_year, &row_surface)
            .map_err(|error| wb13_simout_failure(format!("failed building WB13 row: {error}")))?;

    let month_i8 = i8::try_from(month).map_err(|_| {
        wb13_simout_failure(format!(
            "month out of i8 range for WB13 publication: {month}"
        ))
    })?;
    let day_of_month_i8 = i8::try_from(day_of_month).map_err(|_| {
        wb13_simout_failure(format!(
            "day-of-month out of i8 range for WB13 publication: {day_of_month}"
        ))
    })?;
    let water_year = if month >= 10 {
        calendar_year + 1
    } else {
        calendar_year
    };
    let water_year_i16 = i16::try_from(water_year).map_err(|_| {
        wb13_simout_failure(format!(
            "water-year out of i16 range for WB13 publication: {water_year}"
        ))
    })?;
    let sim_day_index_i32 = i32::try_from(sim_day_index).map_err(|_| {
        wb13_simout_failure(format!(
            "sim_day_index out of i32 range for WB13 publication: {sim_day_index}"
        ))
    })?;

    Ok(SimulationOwnedWb13Row {
        wb13_row,
        month: month_i8,
        day_of_month: day_of_month_i8,
        water_year: water_year_i16,
        sim_day_index: sim_day_index_i32,
    })
}

fn runtime_surface_symbol_value(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    let key = BoundarySymbol::from(symbol);
    runtime_surface
        .state_surface
        .get(&key)
        .map(|value| value.as_f64())
        .or_else(|| {
            runtime_surface
                .flux_surface
                .get(&key)
                .map(|value| value.as_f64())
        })
}

fn runtime_surface_symbol_value_prefer_flux(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    let key = BoundarySymbol::from(symbol);
    runtime_surface
        .flux_surface
        .get(&key)
        .map(|value| value.as_f64())
        .or_else(|| {
            runtime_surface
                .state_surface
                .get(&key)
                .map(|value| value.as_f64())
        })
}

fn runtime_surface_ofe_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    if let Some(nelem) = runtime_surface_symbol_value(runtime_surface, "nelem") {
        let count = scalar_to_usize("nelem", nelem)?;
        if count == 0 {
            return Err(mofe_hourly_carry_failure(
                "nelem must be >= 1 for MOFE hourly carry seeding",
            ));
        }
        return Ok(count);
    }
    if let Some(nwsofe) = runtime_surface_symbol_value(runtime_surface, "nwsofe") {
        let count = scalar_to_usize("nwsofe", nwsofe)?;
        if count == 0 {
            return Err(mofe_hourly_carry_failure(
                "nwsofe must be >= 1 for MOFE hourly carry seeding",
            ));
        }
        return Ok(count);
    }
    Ok(1)
}

fn seed_mofe_hourly_carry_runtime_surface_inputs(
    runtime_surface: &mut HillslopeWritebackSurface,
    active: bool,
) -> Result<(), HillslopeCliError> {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from(MOFE_HOURLY_CARRY_ARRAYS_ENABLED_SYMBOL),
        BoundaryValue::scalar(if active { 1.0 } else { 0.0 }),
    );
    if active {
        runtime_surface
            .state_surface
            .entry(BoundarySymbol::from(MOFE_HOURLY_UPSTREAM_AREA_RATIO_SYMBOL))
            .or_insert_with(|| BoundaryValue::scalar(1.0));
    }

    for root in MOFE_HOURLY_REQUIRED_ARRAYS {
        for hour in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
            let symbol = mofe_hourly_carry_hour_symbol(root, hour);
            if let Some(existing) = runtime_surface_symbol_value(runtime_surface, &symbol) {
                require_mofe_hourly_carry_non_negative(existing, &symbol)?;
            } else if active {
                runtime_surface
                    .state_surface
                    .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(0.0));
            }
        }
    }
    Ok(())
}

fn mofe_hourly_carry_hour_symbol(root: &str, hour: usize) -> String {
    format!("{root}_{hour:04}")
}

fn require_mofe_hourly_carry_non_negative(
    value: f64,
    symbol: &str,
) -> Result<(), HillslopeCliError> {
    if !value.is_finite() || value < 0.0 {
        return Err(mofe_hourly_carry_failure(format!(
            "{symbol} must be finite and >= 0.0, observed {value}"
        )));
    }
    Ok(())
}

fn parse_mofe03_binary_flag(symbol: &str, value: f64) -> Result<bool, HillslopeCliError> {
    if !value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} must be finite, observed {value}"
        )));
    }
    if value.abs() <= MOFE03_WAVE2_ENABLE_TOLERANCE {
        return Ok(false);
    }
    if (value - 1.0).abs() <= MOFE03_WAVE2_ENABLE_TOLERANCE {
        return Ok(true);
    }
    Err(mofe03_wave2_seed_failure(format!(
        "{symbol} must be binary 0|1, observed {value}"
    )))
}

fn require_mofe03_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let Some(value) = runtime_surface_symbol_value(runtime_surface, symbol) else {
        return Err(mofe03_wave2_seed_failure(format!(
            "missing required runtime symbol {symbol}"
        )));
    };
    if !value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "runtime symbol {symbol} is non-finite ({value})"
        )));
    }
    Ok(value)
}

fn require_runtime_surface_scalar_prefer_flux(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let value = runtime_surface_symbol_value_prefer_flux(runtime_surface, symbol)
        .ok_or_else(|| wb13_simout_failure(format!("missing required runtime symbol {symbol}")))?;
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} must be finite, observed {value}"
        )));
    }
    Ok(value)
}

fn require_mofe03_non_negative_seed_scalar(
    value: f64,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    if !value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} seed value must be finite, observed {value}"
        )));
    }
    if value < 0.0 {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} seed value must be >= 0.0, observed {value}"
        )));
    }
    Ok(value)
}

fn require_mofe03_positive_seed_scalar(value: f64, symbol: &str) -> Result<f64, HillslopeCliError> {
    if !value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} seed value must be finite, observed {value}"
        )));
    }
    if value <= 0.0 {
        return Err(mofe03_wave2_seed_failure(format!(
            "{symbol} seed value must be > 0.0, observed {value}"
        )));
    }
    Ok(value)
}

fn seed_mofe03_wave2_class_symbol(
    runtime_surface: &mut HillslopeWritebackSurface,
    root: &str,
    class_index: usize,
    seed_value: f64,
) -> Result<(), HillslopeCliError> {
    if !seed_value.is_finite() {
        return Err(mofe03_wave2_seed_failure(format!(
            "{root}_{class_index:04} seed value must be finite, observed {seed_value}"
        )));
    }

    let symbol = mofe03_erod14_class_symbol(root, class_index);
    let value = if let Some(existing) = runtime_surface_symbol_value(runtime_surface, &symbol) {
        if !existing.is_finite() {
            return Err(mofe03_wave2_seed_failure(format!(
                "{symbol} must be finite when present, observed {existing}"
            )));
        }
        existing
    } else {
        seed_value
    };

    runtime_surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    Ok(())
}

fn mofe03_erod14_class_symbol(root: &str, class_index: usize) -> String {
    format!("{root}_{class_index:04}")
}

fn mofe03_wave2_seed_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "mofe03_wave2_seed",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}

fn mofe_hourly_carry_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "mofe_hourly_carry",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}

fn require_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let value = runtime_surface_symbol_value(runtime_surface, symbol)
        .ok_or_else(|| wb13_simout_failure(format!("missing required runtime symbol {symbol}")))?;
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} is non-finite ({value})"
        )));
    }
    Ok(value)
}

fn require_simimpl10_coupling_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<f64, HillslopeCliError> {
    let value = runtime_surface_symbol_value(runtime_surface, symbol)
        .ok_or_else(|| simcoup_failure(format!("missing required coupling symbol {symbol}")))?;
    if !value.is_finite() {
        return Err(simcoup_failure(format!(
            "coupling symbol {symbol} is non-finite ({value})"
        )));
    }
    Ok(value)
}

fn parse_simimpl10_binary_flag(field: &str, value: f64) -> Result<bool, HillslopeCliError> {
    if value.abs() <= SIMIMPL10_FLAG_TOLERANCE {
        return Ok(false);
    }
    if (value - 1.0).abs() <= SIMIMPL10_FLAG_TOLERANCE {
        return Ok(true);
    }
    Err(simcoup_failure(format!(
        "{field} must be binary 0|1, observed {value}"
    )))
}

fn scalar_to_i32(symbol: &str, value: f64) -> Result<i32, HillslopeCliError> {
    if !value.is_finite() {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} is non-finite ({value})"
        )));
    }
    let rounded = value.round();
    if (rounded - value).abs() > 1.0e-9 {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} must be integral for WB13 publication, observed {value}"
        )));
    }
    if rounded < f64::from(i32::MIN) || rounded > f64::from(i32::MAX) {
        return Err(wb13_simout_failure(format!(
            "runtime symbol {symbol} out of i32 range ({value})"
        )));
    }
    format!("{rounded:.0}")
        .parse::<i32>()
        .map_err(|error| wb13_simout_failure(format!("failed converting {symbol} to i32: {error}")))
}

fn scalar_to_usize(symbol: &str, value: f64) -> Result<usize, HillslopeCliError> {
    let int_value = scalar_to_i32(symbol, value)?;
    usize::try_from(int_value).map_err(|_| {
        wb13_simout_failure(format!(
            "runtime symbol {symbol} must be non-negative usize, observed {value}"
        ))
    })
}

fn usize_to_scalar(symbol: &str, value: usize) -> Result<f64, HillslopeCliError> {
    value.to_string().parse::<f64>().map_err(|error| {
        wb13_simout_failure(format!(
            "failed converting {symbol} count {value} to f64 for runtime seeding: {error}"
        ))
    })
}

fn wb13_primary_layer_symbol(root: &str, layer_index: usize) -> String {
    format!("{root}_{layer_index:04}")
}

fn mode_selection_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "mode_selection",
        detail: format!("{WUI_MODE_GUARD_ID} {}", detail.into()),
    }
}

fn timestep_policy_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "timestep_policy",
        detail: format!("{SIMMODE_TIMESTEP_GUARD_ID} {}", detail.into()),
    }
}

fn simcons_intake_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "adapter_boundary",
        detail: format!("{SIMCONS_INTAKE_GUARD_ID} {}", detail.into()),
    }
}

fn simcoup_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "coupling_vectors",
        detail: format!("{SIMCOUP_GUARD_ID} {}", detail.into()),
    }
}

fn wb13_simout_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "wb13_publication",
        detail: format!("{SIMOUT_GUARD_ID} {}", detail.into()),
    }
}

fn build_loss_output_json(
    run_name: &str,
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    snow: &openwepp_input_contract::parsers::snow::SnowParseOutput,
    frost: &openwepp_input_contract::parsers::frost::FrostParseOutput,
    climate_span: &ClimateRunSpanSummary,
    executed_day_count: usize,
) -> Result<String, HillslopeCliError> {
    let payload = serde_json::json!({
        "schema": "openwepp-hillslope-loss-v1",
        "run_name": run_name,
        "first_day_year": climate_span.first_day.year,
        "first_day_julian": climate_span.first_day.julian_day,
        "last_day_year": climate_span.last_day.year,
        "last_day_julian": climate_span.last_day.julian_day,
        "precipitation_mm": climate_span.first_day.precipitation_mm,
        "climate_day_count": climate_span.days.len(),
        "executed_day_count": executed_day_count,
        "ofe_count": soil.ofes.len(),
        "snow_override_applied": snow.sidecar_present,
        "frost_wint_red": frost.wint_red,
    });

    serde_json::to_string_pretty(&payload)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })
}

fn build_optional_output_payload(
    run_name: &str,
    output_path: &Path,
    climate_span: &ClimateRunSpanSummary,
    executed_day_count: usize,
) -> String {
    let file_name = file_name_string(output_path);
    format!(
        "openwepp_optional_output_v1\nrun_name={run_name}\nfile={file_name}\nfirst_year={}\nfirst_day={}\nlast_year={}\nlast_day={}\nclimate_day_count={}\nexecuted_day_count={}\nprecipitation_mm={:.3}\n",
        climate_span.first_day.year,
        climate_span.first_day.julian_day,
        climate_span.last_day.year,
        climate_span.last_day.julian_day,
        climate_span.days.len(),
        executed_day_count,
        climate_span.first_day.precipitation_mm
    )
}

#[derive(Debug, Clone, Copy)]
struct ClimateDayProjection {
    year: i32,
    month: i32,
    day_of_month: i32,
    julian_day: u16,
    precipitation_mm: f64,
}

#[derive(Debug, Clone)]
struct ClimateRunSpanSummary {
    days: Vec<ClimateDayProjection>,
    first_day: ClimateDayProjection,
    last_day: ClimateDayProjection,
}

fn climate_day_projection(
    record: &ClimateDailyRecord,
) -> Result<ClimateDayProjection, HillslopeCliError> {
    match record {
        ClimateDailyRecord::NoBreakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            Ok(ClimateDayProjection {
                year: day.year,
                month: day.mon,
                day_of_month: day.day,
                julian_day,
                precipitation_mm: (day.prcp * 1_000.0).max(0.0),
            })
        }
        ClimateDailyRecord::Breakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            let prcp_mm = day
                .breakpoints
                .last()
                .map_or(0.0, |point| (point.pptcum * 1_000.0).max(0.0));
            Ok(ClimateDayProjection {
                year: day.year,
                month: day.mon,
                day_of_month: day.day,
                julian_day,
                precipitation_mm: prcp_mm,
            })
        }
    }
}

fn build_climate_run_span_summary(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
) -> Result<ClimateRunSpanSummary, HillslopeCliError> {
    let mut days = Vec::with_capacity(climate.daily_records.len());
    for record in &climate.daily_records {
        days.push(climate_day_projection(record)?);
    }

    let Some(first_day) = days.first().copied() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: "climate daily record set is empty".to_string(),
        });
    };
    let Some(last_day) = days.last().copied() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: "climate daily record set is empty".to_string(),
        });
    };

    Ok(ClimateRunSpanSummary {
        days,
        first_day,
        last_day,
    })
}

fn simulation_year_from_calendar_year(
    calendar_year: i32,
    simulation_start_year: i32,
) -> Result<i32, HillslopeCliError> {
    let relative_year = calendar_year
        .checked_sub(simulation_start_year)
        .and_then(|offset| offset.checked_add(1))
        .ok_or_else(|| {
            wb13_simout_failure(format!(
                "simulation-year mapping overflow for calendar_year={calendar_year} and simulation_start_year={simulation_start_year}"
            ))
        })?;
    if relative_year <= 0 {
        return Err(wb13_simout_failure(format!(
            "simulation-year mapping must be >= 1, observed {relative_year} from calendar_year={calendar_year} and simulation_start_year={simulation_start_year}"
        )));
    }
    Ok(relative_year)
}

fn day_of_year(year: i32, month: i32, day: i32) -> Result<u16, HillslopeCliError> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: format!("invalid calendar date {year}-{month}-{day}"),
        });
    }

    let leap = is_leap_year(year);
    let month_lengths = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let max_day = month_lengths[usize::try_from(month - 1).unwrap_or(0)];
    if day > max_day {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: format!("invalid day-of-month {day} for month {month}"),
        });
    }

    let mut doy = day;
    for length in month_lengths
        .iter()
        .take(usize::try_from(month - 1).unwrap_or(0))
    {
        doy += *length;
    }

    u16::try_from(doy).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "climate",
        detail: format!("day-of-year out of u16 range: {doy}"),
    })
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SidecarPolicy;
    use openwepp_input_contract::parsers::hbp::{HbpParseOptions, parse_hbp_from_path};
    use openwepp_input_contract::parsers::slope::{
        DatverSource, DistanceMode, SlopeOfe, SlopePoint, SlopeProfile,
    };
    use openwepp_kernel_contract::{
        HillslopeConsumerAdapter, HillslopeKernel, HillslopeKernelPhaseClass,
        HillslopeKernelRequest, WritebackField,
    };
    use std::fs;
    use std::path::Path;
    use std::sync::{Mutex, OnceLock};

    #[test]
    fn simimpl09_timestep_policy_scaffolds_subhourly_without_physics_enablement() {
        let policy = TimestepPolicy::scaffold_subhourly(900);
        assert_eq!(policy.policy_name(), SUBHOURLY_EXECUTION_LANE);
        assert_eq!(policy.timestep_seconds(), 900);
        assert!(!policy.physics_enabled());
    }

    #[test]
    fn simimpl09_lane_context_matches_mode_selection_tuple() {
        let mode_selection = HillslopeModeSelectionProvenance {
            wepp_ui: WeppUiModeSelectionProvenance {
                requested: 1,
                effective: 1,
                selected_lane: HOURLY_EXECUTION_LANE.to_string(),
                mode_divergence: false,
                guard_id: WUI_MODE_GUARD_ID.to_string(),
            },
        };
        let lane_context = build_execution_lane_context(&mode_selection)
            .expect("hourly mode-selection tuple should map to hourly lane context");
        assert_eq!(lane_context.lane, ExecutionLane::Hourly);
        assert_eq!(lane_context.requested_mode, HOURLY_EXECUTION_LANE);
        assert_eq!(lane_context.effective_mode, HOURLY_EXECUTION_LANE);
        assert_eq!(
            lane_context.timestep_policy.timestep_seconds(),
            HOURLY_TIMESTEP_SECONDS
        );
        assert!(lane_context.timestep_policy.physics_enabled());
    }

    #[test]
    fn simimpl11_area_derives_from_aggregate_ofe_geometry() {
        let slope = SlopeProfile {
            datver: 2023.3,
            datver_source: DatverSource::Header,
            ofe_count: 2,
            ofes: vec![
                SlopeOfe {
                    index: 0,
                    azm: 180.0,
                    fwidth: 30.0,
                    elevation: None,
                    nslpts: 2,
                    slplen: 60.0,
                    distance_mode: DistanceMode::Normalized,
                    points: vec![
                        SlopePoint {
                            xinput: 0.0,
                            slpinp: 0.02,
                        },
                        SlopePoint {
                            xinput: 1.0,
                            slpinp: 0.06,
                        },
                    ],
                },
                SlopeOfe {
                    index: 1,
                    azm: 180.0,
                    fwidth: 30.0,
                    elevation: None,
                    nslpts: 2,
                    slplen: 40.0,
                    distance_mode: DistanceMode::Normalized,
                    points: vec![
                        SlopePoint {
                            xinput: 0.0,
                            slpinp: 0.06,
                        },
                        SlopePoint {
                            xinput: 1.0,
                            slpinp: 0.03,
                        },
                    ],
                },
            ],
        };

        let observed = derive_mofe04_publication_area_from_slope(&slope)
            .expect("valid aggregate OFE geometry should yield area");
        assert!((observed - 3_000.0).abs() < 1.0e-12);
    }

    #[test]
    fn simimpl14_contract_gate_continuous_wb13_span_and_keys() {
        let (report, _temp_run_dir) = execute_fixture_run("simimpl14_contract_span");
        let pass_parse = parse_hbp_from_path(&report.output_pass, HbpParseOptions::strict())
            .unwrap_or_else(|error| {
                panic!(
                    "pass output should be parseable binary HBP at {}: {error}",
                    report.output_pass.display()
                )
            });
        assert!(pass_parse.record_count >= 1);
        assert!(pass_parse.warnings.is_empty());

        let manifest_json = read_manifest_json(&report);
        assert_json_i64(&manifest_json, "/execution_provenance/climate_day_count", 2);
        assert_json_i64(
            &manifest_json,
            "/execution_provenance/executed_day_count",
            2,
        );
        assert_json_i64(&manifest_json, "/wb13_publication/row_count", 2);
        assert_json_i64(&manifest_json, "/wb13_publication/first_row_key/year", 1);
        let monotonic = manifest_json
            .pointer("/wb13_publication/sim_day_index_monotonic")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or_else(|| {
                panic!("missing bool JSON pointer /wb13_publication/sim_day_index_monotonic")
            });
        assert!(monotonic, "sim_day_index must be monotonic");
    }

    #[test]
    fn simimpl14_contract_gate_loss_output_is_run_span_truthful() {
        let (report, _temp_run_dir) = execute_fixture_run("simimpl14_contract_loss");
        let loss_text = fs::read_to_string(&report.output_loss).unwrap_or_else(|error| {
            panic!(
                "loss output should be readable at {}: {error}",
                report.output_loss.display()
            )
        });
        let loss_json: serde_json::Value =
            serde_json::from_str(&loss_text).expect("loss output should parse as JSON");

        assert_json_i64(&loss_json, "/climate_day_count", 2);
        assert_json_i64(&loss_json, "/executed_day_count", 2);
        assert_json_i64(&loss_json, "/first_day_julian", 1);
        assert_json_i64(&loss_json, "/last_day_julian", 2);
    }

    #[test]
    fn hphys0216_wb13_fc_storage_guard_rejects_missing_layer_authority_symbol() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("thetfc_0001"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing thetfc_0001 must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("missing required runtime symbol thetfc_0001"),
                    "expected missing thetfc_0001 typed guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hphys0216d_wb13_fc_storage_guard_rejects_missing_tail_symbol() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("wb13_profile_fc_tail_mm"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing wb13_profile_fc_tail_mm must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("missing required runtime symbol wb13_profile_fc_tail_mm"),
                    "expected missing wb13_profile_fc_tail_mm typed guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hphys0207_wb13_wp_storage_guard_is_exercised_by_direct_row_builder_probe() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_wp_store_mm"),
            BoundaryValue::scalar(-1.0),
        );

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("negative wb13_profile_wp_store_mm must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("wb13_profile_wp_store_mm must be >= 0.0"),
                    "expected wb13_profile_wp_store_mm typed guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hphys0209_wb13_wp_storage_guard_rejects_missing_authoritative_symbol() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("wb13_profile_wp_store_mm"));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("missing wb13_profile_wp_store_mm must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("missing required runtime symbol wb13_profile_wp_store_mm"),
                    "expected missing wb13_profile_wp_store_mm guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hphys0216d_wb13_profile_fc_publication_uses_layer_plus_tail_authority() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_store_mm"),
            BoundaryValue::scalar(100.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_tail_mm"),
            BoundaryValue::scalar(5.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_wp_store_mm"),
            BoundaryValue::scalar(55.0),
        );

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("valid WB13 probe surface should publish row");

        assert!(
            (row.wb13_row.profile_fc_store - 80.0).abs() < 1.0e-12,
            "ProfileFCStore must follow authoritative layer aggregation plus explicit normalized-tail contribution"
        );
        assert!(
            (row.wb13_row.profile_wp_store - 55.0).abs() < 1.0e-12,
            "ProfileWPStore must follow wb13_profile_wp_store_mm storage authority"
        );
    }

    #[test]
    fn hphys0203_wb13_dp_guard_rejects_negative_deep_percolation_source() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(-1.0e-6));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("negative D must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("D must be >= 0.0"),
                    "expected D domain guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hphys0233_wb13_dp_publication_prefers_flux_surface_over_stale_state_surface() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.030_000));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.000_200));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should use flux-authoritative D");

        assert!(
            (row.wb13_row.dp - 0.2).abs() < 1.0e-12,
            "Dp must follow flux-surface D when both state and flux values are present"
        );
    }

    #[test]
    fn hphys0234_wb13_subhyd_publication_prefers_flux_surface_over_stale_state_surface() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.030_000));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("Qdd"),
            BoundaryValue::scalar(0.020_000),
        );
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.050_000));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.000_700));
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("Qdd"),
            BoundaryValue::scalar(0.000_200),
        );
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.000_900));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should use flux-authoritative q/Qdd/Qd");

        assert!(
            (row.wb13_row.latqcc - 0.7).abs() < 1.0e-12,
            "latqcc must follow flux-surface q when both state and flux values are present"
        );
        assert!(
            (row.wb13_row.tile - 0.2).abs() < 1.0e-12,
            "Tile must follow flux-surface Qdd when both state and flux values are present"
        );
    }

    #[test]
    fn hphys0239_wb13_hydrology_publication_prefers_flux_surface_over_stale_state_surface() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.050_000));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.003_000));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(0.002_000));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Er"), BoundaryValue::scalar(0.001_000));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.000_800));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.000_300));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(0.000_150));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Er"), BoundaryValue::scalar(0.000_070));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should use flux-authoritative Q/Ep/Es/Er");

        assert!(
            (row.wb13_row.q - 0.8).abs() < 1.0e-12,
            "Q must follow flux-surface value when both state and flux are present"
        );
        assert!(
            (row.wb13_row.ep - 0.3).abs() < 1.0e-12,
            "Ep must follow flux-surface value when both state and flux are present"
        );
        assert!(
            (row.wb13_row.es - 0.15).abs() < 1.0e-12,
            "Es must follow flux-surface value when both state and flux are present"
        );
        assert!(
            (row.wb13_row.er - 0.07).abs() < 1.0e-12,
            "Er must follow flux-surface value when both state and flux are present"
        );
    }

    #[test]
    fn hphys0281_wb13_publication_canonicalizes_roundoff_negative_es_without_evappm_clamp() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(-1.0e-13));
        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("wb11_et_seed_branch_evappm"));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should snap within-tolerance negative Es roundoff");

        assert!(
            row.wb13_row.es.abs() < f64::EPSILON,
            "WB13 Es roundoff must canonicalize to zero without EVAPPM material-negative clamp behavior"
        );
    }

    #[test]
    fn hphys0250_scheduler_lifecycle_preserves_pl_runtime_sentinel_for_ep_lineage() {
        let source = include_str!("mod.rs");
        let sentinel = "pl_schedule_slot_count";
        let forbidden_fragment = ["symbol.as_str() != ", "\"", sentinel, "\""].concat();

        assert!(
            !source.contains(&forbidden_fragment),
            "runner scheduler lifecycle must not strip {sentinel}; PL growth must remain active so rtd can feed final Ep lineage"
        );
    }

    #[test]
    fn hphys0250_pl_activation_keeps_zero_date_perennial_slots_active() {
        let mut runtime_surface = HillslopeWritebackSurface {
            state_surface: BTreeMap::new(),
            flux_surface: BTreeMap::new(),
        };
        for (symbol, value) in [
            ("pl_schedule_slot_count", 1.0),
            ("pl_schedule_rotation_years", 4.0),
            ("pl_schedule_rotation_repeats", 1.0),
            ("year", 1.0),
            ("day", 1.0),
            ("pl_schedule_slot_0001_ofe_index", 1.0),
            ("pl_schedule_slot_0001_year_in_rotation", 1.0),
            ("pl_schedule_slot_0001_rotation_index", 1.0),
            ("pl_schedule_slot_0001_crop_slots", 1.0),
            ("pl_schedule_slot_0001_crop_0001_imngmt", 2.0),
            ("pl_growth_slot_0001_crop_0001_jdplt", 0.0),
            ("pl_growth_slot_0001_crop_0001_jdharv", 0.0),
            ("pl_growth_slot_0001_crop_0001_jdstop", 0.0),
        ] {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }

        prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)
            .expect("zero-date perennial PL slot should remain scheduler-active");

        assert!(
            runtime_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("pl_schedule_slot_count")),
            "zero-date perennial windows must keep PL activation sentinel for scheduler dispatch"
        );
    }

    #[test]
    fn hphys0250_wb13_ep_publication_consumes_final_root_uptake_flux() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.0));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.004_2));

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 publication should consume final root-uptake flux Ep");

        assert!(
            (row.wb13_row.ep - 4.2).abs() < 1.0e-12,
            "WB13 Ep must use final post-root-uptake flux even when stale state Ep is present"
        );
    }

    #[test]
    fn hphys0203_wb13_latqcc_guard_rejects_negative_lateral_source() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(-1.0e-6));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("negative q must fail WB13 publication guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("SIMOUT-E-001"),
                    "expected SIMOUT-E-001 guard id, observed: {detail}"
                );
                assert!(
                    detail.contains("q must be >= 0.0"),
                    "expected q domain guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hphys0212_wb13_subhyd_coupling_guard_rejects_qd_mismatch() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.002));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.001));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.002_5));

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("Qd mismatch must fail WB13 subsurface coupling guard");

        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
                assert_eq!(surface, "wb13_publication");
                assert!(
                    detail.contains("Qd coupling closure violated"),
                    "expected Qd coupling guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hphys0212_wb13_subhyd_publication_uses_qdd_and_subrin_lineage() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.0015));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.0005));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.0020));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("SubRIn"),
            BoundaryValue::scalar(0.0008),
        );

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("valid Qd coupling surface should publish WB13 row");

        assert!(
            (row.wb13_row.latqcc - 1.5).abs() < 1.0e-12,
            "latqcc must follow q source symbol in mm/day lane"
        );
        assert!(
            (row.wb13_row.tile - 0.5).abs() < 1.0e-12,
            "Tile must follow Qdd source symbol in mm/day lane"
        );
        assert!(
            (row.wb13_row.subrin - 0.8).abs() < 1.0e-12,
            "SubRIn must follow SubRIn source symbol in mm/day lane"
        );
    }

    #[test]
    fn hphys0203_wb13_soil_water_total_closure_is_conservation_consistent() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.081),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_ws_frz"),
            BoundaryValue::scalar(0.003),
        );

        let row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("valid WB13 probe surface should publish row");

        let closure_delta =
            row.wb13_row.soil_water_total - (row.wb13_row.total_soil + row.wb13_row.frozwt);
        assert!(
            closure_delta.abs() <= SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
            "SoilWaterTotal closure must remain conservation-consistent, observed delta={closure_delta}"
        );
    }

    #[test]
    fn hphys0203_wb13_profile_storage_perturbation_is_stable() {
        let baseline_surface = seeded_wb13_runtime_surface_probe();
        let baseline_row = build_simulation_owned_wb13_row(
            &baseline_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("baseline probe row should publish");

        let mut perturbed_surface = seeded_wb13_runtime_surface_probe();
        let baseline_thetfc = require_runtime_surface_scalar(&perturbed_surface, "thetfc_0001")
            .expect("seeded surface should include thetfc_0001");
        perturbed_surface.state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(baseline_thetfc + 1.0e-4),
        );
        let perturbed_row = build_simulation_owned_wb13_row(
            &perturbed_surface,
            1_000.0,
            1,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("perturbed probe row should publish");

        assert!(
            perturbed_row.wb13_row.profile_porosity_cap >= perturbed_row.wb13_row.profile_fc_store
                && perturbed_row.wb13_row.profile_fc_store
                    >= perturbed_row.wb13_row.profile_wp_store,
            "bounded profile perturbation must preserve profile storage ordering"
        );
        assert!(
            perturbed_row.wb13_row.profile_fc_store >= baseline_row.wb13_row.profile_fc_store,
            "positive bounded FC perturbation should not decrease published ProfileFCStore"
        );
        assert!(
            (perturbed_row.wb13_row.profile_fc_store - baseline_row.wb13_row.profile_fc_store)
                <= 5.0,
            "bounded FC perturbation produced unstable ProfileFCStore response: baseline={}, perturbed={}",
            baseline_row.wb13_row.profile_fc_store,
            perturbed_row.wb13_row.profile_fc_store
        );
    }

    fn wb11_seed_test_surface(symbols: &[(&str, f64)]) -> HillslopeWritebackSurface {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        for (symbol, value) in symbols {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(*symbol), BoundaryValue::scalar(*value));
        }
        runtime_surface
    }

    fn state_field_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        fields
            .iter()
            .find(|field| field.symbol.as_str() == symbol)
            .map(|field| field.value.as_f64())
    }

    fn flux_field_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        fields
            .iter()
            .find(|field| field.symbol.as_str() == symbol)
            .map(|field| field.value.as_f64())
    }

    fn insert_wb11_primary_layer_lineage_symbols(
        runtime_surface: &mut HillslopeWritebackSurface,
        sat: f64,
        include_cpm: bool,
    ) {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.25));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("wb11_nsl"), BoundaryValue::scalar(1.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("wb19_nsl"), BoundaryValue::scalar(1.0));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_dg_0001"),
            BoundaryValue::scalar(0.25),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("por_0001"),
            BoundaryValue::scalar(0.45),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_por_0001"),
            BoundaryValue::scalar(0.45),
        );
        if include_cpm {
            runtime_surface.state_surface.insert(
                BoundarySymbol::from("cpm_0001"),
                BoundaryValue::scalar(0.90),
            );
        }
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("sat"), BoundaryValue::scalar(sat));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("ssc_0001"),
            BoundaryValue::scalar(2.0e-6),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
            BoundaryValue::scalar(1.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_drain_enabled"),
            BoundaryValue::scalar(0.0),
        );
    }

    #[test]
    fn wshedimpl42_breakpoint_seed_uses_current_nbrkpt_not_stale_ninten() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("ibrkpt", 1.0),
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", -3.0),
            ("tmin", -6.9),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("nbrkpt", 3.0),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 36_479.88),
            ("timem_0003", 38_279.88),
            ("intsty_0001", 5.701_773_141_797_617e-8),
            ("intsty_0002", 5.111_111_111_111_11e-7),
            ("intsty_0003", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.55, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("seeding should accept current-day breakpoint cardinality");

        let ninten = require_runtime_surface_scalar(&runtime_surface, "ninten")
            .expect("ninten should be seeded");
        let nbrkpt = require_runtime_surface_scalar(&runtime_surface, "nbrkpt")
            .expect("nbrkpt should be seeded");
        let rainfall_input =
            require_runtime_surface_scalar(&runtime_surface, "wb12_rainfall_input")
                .expect("wb12_rainfall_input should be seeded");

        assert!(
            (ninten - 3.0).abs() < 1.0e-12,
            "ninten should track current-day breakpoint count"
        );
        assert!(
            (nbrkpt - 3.0).abs() < 1.0e-12,
            "nbrkpt should remain aligned with current-day breakpoint count"
        );
        assert!(
            (rainfall_input - 0.003).abs() < 1.0e-12,
            "rainfall seed should preserve full current-day breakpoint precipitation depth"
        );
    }

    #[test]
    fn hphys0250_wb11_seed_initializes_neutral_water_stress_for_decomposition() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("WB11 seed should publish neutral initial water stress");

        let water_stress = require_runtime_surface_scalar(&runtime_surface, "Ws")
            .expect("WB11 seed should publish Ws for pre-ET decomposition consumers");
        assert!(
            (water_stress - 1.0).abs() < 1.0e-12,
            "initial decomposition stress carryover must be neutral before ET computes same-day Ws"
        );
    }

    #[test]
    fn hphys0232_wb11_seed_daily_lane_sets_wb18_perc_lane_substeps_to_one() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("daily WB11 seed should succeed");

        let lane_substeps =
            require_runtime_surface_scalar(&runtime_surface, "wb18_perc_lane_substeps")
                .expect("daily WB11 seed should publish wb18_perc_lane_substeps");
        let wb19_lane_substeps =
            require_runtime_surface_scalar(&runtime_surface, "wb19_lateral_drain_lane_substeps")
                .expect("daily WB11 seed should publish wb19_lateral_drain_lane_substeps");
        assert!(
            (lane_substeps - 1.0).abs() < 1.0e-12,
            "daily lane must seed wb18_perc_lane_substeps=1"
        );
        assert!(
            (wb19_lane_substeps - 1.0).abs() < 1.0e-12,
            "daily lane must seed wb19_lateral_drain_lane_substeps=1"
        );
    }

    #[test]
    fn hphys0232_wb11_seed_hourly_lane_sets_wb18_perc_lane_substeps_to_twenty_four() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Hourly)
            .expect("hourly WB11 seed should succeed");

        let lane_substeps =
            require_runtime_surface_scalar(&runtime_surface, "wb18_perc_lane_substeps")
                .expect("hourly WB11 seed should publish wb18_perc_lane_substeps");
        let wb19_lane_substeps =
            require_runtime_surface_scalar(&runtime_surface, "wb19_lateral_drain_lane_substeps")
                .expect("hourly WB11 seed should publish wb19_lateral_drain_lane_substeps");
        assert!(
            (lane_substeps - 24.0).abs() < 1.0e-12,
            "hourly lane must seed wb18_perc_lane_substeps=24"
        );
        assert!(
            (wb19_lane_substeps - 24.0).abs() < 1.0e-12,
            "hourly lane must seed wb19_lateral_drain_lane_substeps=24"
        );
    }

    #[test]
    fn hphys0208_wb11_seed_uses_sat_por_cpm_layer_lineage() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("WB11 seeding should succeed for valid sat/por/cpm lineage");

        let theta = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_theta_0001")
            .expect("wb18_perc_theta_0001 should be seeded");
        let fc = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_fc_0001")
            .expect("wb18_perc_fc_0001 should be seeded");
        let ul = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_ul_0001")
            .expect("wb18_perc_ul_0001 should be seeded");
        let wb11_soil_water = require_runtime_surface_scalar(&runtime_surface, "wb11_soil_water")
            .expect("wb11_soil_water should be seeded");
        let wb11_drainable_storage =
            require_runtime_surface_scalar(&runtime_surface, "wb11_drainable_storage")
                .expect("wb11_drainable_storage should be seeded");

        let expected_fc = (0.30 - 0.12) * 0.25;
        let expected_ul = (0.45 - 0.12) * 0.25;
        let expected_theta = (((0.50 * 0.45) * 0.90) - 0.12) * 0.25;
        let expected_soilw = expected_theta + (0.12 * 0.25);

        assert!(
            (fc - expected_fc).abs() < 1.0e-12,
            "wb18_perc_fc must follow dg*(thetfc-thetdr)"
        );
        assert!(
            (ul - expected_ul).abs() < 1.0e-12,
            "wb18_perc_ul must follow (por-thetdr)*dg"
        );
        assert!(
            (theta - expected_theta).abs() < 1.0e-12,
            "wb18_perc_theta must follow (((sat*por)*cpm)-thetdr)*dg"
        );
        assert!(
            (wb11_soil_water - expected_soilw).abs() < 1.0e-12,
            "wb11_soil_water must follow Σ(st + thetdr*dg)"
        );
        assert!(
            wb11_drainable_storage.abs() < 1.0e-12,
            "wb11_drainable_storage must follow Σmax(st-fc,0)"
        );
    }

    #[test]
    fn auth12_wb11_seed_applies_cpm_for_disturbed_measured_fcwp_lineage() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("solwpv", 9002.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("WB11 seeding should succeed for disturbed measured FC/WP lineage");

        let theta = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_theta_0001")
            .expect("wb18_perc_theta_0001 should be seeded");
        let wb11_soil_water = require_runtime_surface_scalar(&runtime_surface, "wb11_soil_water")
            .expect("wb11_soil_water should be seeded");

        let expected_theta_without_cpm = ((0.50 * 0.45) - 0.12) * 0.25;
        let expected_theta_with_cpm = (((0.50 * 0.45) * 0.90) - 0.12) * 0.25;
        let expected_soilw = expected_theta_with_cpm + (0.12 * 0.25);

        assert!(
            (theta - expected_theta_with_cpm).abs() < 1.0e-12,
            "disturbed measured FC/WP lineage must apply sat*por*cpm scaling"
        );
        assert!(
            theta < expected_theta_without_cpm - 1.0e-12,
            "disturbed measured FC/WP lineage must not bypass cpm scaling"
        );
        assert!(
            (wb11_soil_water - expected_soilw).abs() < 1.0e-12,
            "wb11_soil_water must remain consistent with the disturbed measured FC/WP cpm-scaled saturation lineage"
        );
    }

    #[test]
    fn auth12_wb11_seed_applies_cpm_for_legacy_measured_theta_fcwp_lineage() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("solwpv", 7778.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("WB11 seeding should succeed for legacy measured-theta FC/WP lineage");

        let theta = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_theta_0001")
            .expect("wb18_perc_theta_0001 should be seeded");
        let wb11_soil_water = require_runtime_surface_scalar(&runtime_surface, "wb11_soil_water")
            .expect("wb11_soil_water should be seeded");

        let expected_theta_without_cpm = ((0.50 * 0.45) - 0.12) * 0.25;
        let expected_theta_with_cpm = (((0.50 * 0.45) * 0.90) - 0.12) * 0.25;
        let expected_soilw = expected_theta_with_cpm + (0.12 * 0.25);

        assert!(
            (theta - expected_theta_with_cpm).abs() < 1.0e-12,
            "legacy measured-theta FC/WP lineage must apply sat*por*cpm scaling"
        );
        assert!(
            theta < expected_theta_without_cpm - 1.0e-12,
            "legacy measured-theta FC/WP lineage must not bypass cpm scaling"
        );
        assert!(
            (wb11_soil_water - expected_soilw).abs() < 1.0e-12,
            "wb11_soil_water must remain consistent with the measured-theta cpm-scaled saturation lineage"
        );
    }

    #[test]
    fn hphys0212_wb11_seed_preserves_mutable_state_after_initialization() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("initial WB11 seed should succeed");

        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.012_345),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.100_123),
        );
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.001));

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("daily reseed should not reinitialize WB18/WB11 mutable state");

        let theta = require_runtime_surface_scalar(&runtime_surface, "wb18_perc_theta_0001")
            .expect("wb18_perc_theta_0001 should remain available");
        let storage_initial =
            require_runtime_surface_scalar(&runtime_surface, "wb12_storage_initial")
                .expect("wb12_storage_initial should be refreshed each day");

        assert!(
            (theta - 0.012_345).abs() < 1.0e-12,
            "daily reseed must preserve mutable wb18_perc_theta state"
        );
        assert!(
            (storage_initial - 0.100_123).abs() < 1.0e-12,
            "wb12_storage_initial must follow carried wb11_soil_water each day"
        );
    }

    #[test]
    fn hphys0212_wb11_seed_rejects_enabled_drain_without_geometry() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, true);
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_drain_enabled"),
            BoundaryValue::scalar(1.0),
        );

        let error = seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect_err("enabled drain without geometry symbols must fail WB11 seed");
        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { detail, .. } => {
                assert!(
                    detail.contains("missing required runtime symbol wb19_drain_depth"),
                    "expected missing wb19_drain_depth guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hphys0263_wb11_seed_uses_evappm_branch_when_pmetpara_selects_pmet() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 20.0),
            ("tmin", 10.0),
            ("tdpt", 8.0),
            ("rad", 20.0),
            ("radpot", 25.0),
            ("vwind", 2.0),
            ("elevm", 300.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 4.0),
            ("canhgt", 1.0),
            ("rtd", 0.2),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("pmetpara.mode.sidecar_present", 1.0),
            ("pmetpara.mode.iflget", 2.0),
            ("pmetpara.selected.kcb", 0.95),
            ("pmetpara.selected.rawp", 0.8),
            ("wb17_residue_interception", 0.000_2),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.80, true);
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_solthk_0001"),
            BoundaryValue::scalar(0.25),
        );

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("PMET-mode WB11 seed should succeed");

        let demand = require_runtime_surface_scalar(&runtime_surface, "wb11_et_demand")
            .expect("WB11 demand should be seeded");
        let evappm_branch =
            require_runtime_surface_scalar(&runtime_surface, "wb11_et_seed_branch_evappm")
                .expect("EVAPPM branch flag should be published");
        let priestley_branch = require_runtime_surface_scalar(
            &runtime_surface,
            "wb11_et_seed_branch_priestley_taylor",
        )
        .expect("Priestley branch flag should be published");
        let etorc = require_runtime_surface_scalar(&runtime_surface, "pmet.etorc_mm")
            .expect("migrated EVAPPM reference ET should be traced");
        let kcbcon = require_runtime_surface_scalar(&runtime_surface, "pmet.kcbcon")
            .expect("migrated EVAPPM basal canopy coefficient should be traced");

        assert!(
            (demand - 0.000_108_279_281_560_428_06).abs() < 1.0e-15,
            "WB11 demand must follow pinned evappm.for plant-transpiration demand"
        );
        assert!((evappm_branch - 1.0).abs() < 1.0e-12);
        assert!(priestley_branch.abs() < 1.0e-12);
        assert!((etorc - 0.139_042_184_372_870_16).abs() < 1.0e-12);
        assert!((kcbcon - 0.778_751_298_023_734_6).abs() < 1.0e-12);
    }

    #[test]
    fn hphys0281_wb11_evappm_seed_publishes_condensation_storage_return() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", -1.6),
            ("tmin", -14.6),
            ("tdpt", -1.0),
            ("rad", 200.0),
            ("radpot", 250.0),
            ("vwind", 3.0),
            ("elevm", 300.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 4.0),
            ("canhgt", 1.0),
            ("rtd", 0.2),
            ("prcp", 0.004_4),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
            ("pmetpara.mode.sidecar_present", 1.0),
            ("pmetpara.mode.iflget", 2.0),
            ("pmetpara.selected.kcb", 0.95),
            ("pmetpara.selected.rawp", 0.8),
            ("wb17_residue_interception", 0.000_2),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.80, true);
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_solthk_0001"),
            BoundaryValue::scalar(0.25),
        );

        seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect("supersaturated cold-day EVAPPM seed should not fail");

        let pmet_soil_evaporation = require_runtime_surface_scalar(&runtime_surface, "pmet.es_m")
            .expect("PMET soil evaporation should be published");
        let storage_return =
            require_runtime_surface_scalar(&runtime_surface, "pmet.es_storage_return_m")
                .expect("negative EVAPPM soil evaporation should publish a storage return");
        let storage_return_value = runtime_surface
            .state_surface
            .get(&BoundarySymbol::from("pmet.es_storage_return_m"))
            .expect("storage return boundary value should be present");
        let pmet_transpiration = require_runtime_surface_scalar(&runtime_surface, "pmet.ep_m")
            .expect("PMET transpiration should be published");
        let demand = require_runtime_surface_scalar(&runtime_surface, "wb11_et_demand")
            .expect("WB11 ET demand should be published");
        let etorc = require_runtime_surface_scalar(&runtime_surface, "pmet.etorc_mm")
            .expect("PMET reference ET diagnostic should be published");

        assert!(
            etorc < 0.0,
            "test vector must exercise condensation/reference-ET reversal, observed {etorc}"
        );
        assert!(
            pmet_soil_evaporation.abs() < f64::EPSILON,
            "material-negative PMET Es must publish as non-negative zero, observed {pmet_soil_evaporation}"
        );
        assert!(
            storage_return > 0.0,
            "negative raw EVAPPM Es magnitude must be carried as top-layer storage return"
        );
        assert_eq!(
            storage_return_value.unit_label(),
            "m",
            "storage return must publish as typed water-depth meters"
        );
        assert!(
            pmet_transpiration.abs() < f64::EPSILON,
            "condensation must not publish material-negative PMET transpiration, observed {pmet_transpiration}"
        );
        assert!(
            demand.abs() < f64::EPSILON,
            "WB11 PMET demand must follow canonicalized non-negative transpiration, observed {demand}"
        );
    }

    #[test]
    fn hphys0213_wb19_lateral_withdrawal_publishes_realized_flux_and_updates_wb11_soil_water() {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("solwpv"),
            BoundaryValue::scalar(2006.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_drainable_storage"),
            BoundaryValue::scalar(0.4),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.5),
        );
        state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
        state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(10.0));
        state_surface.insert(
            BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
            BoundaryValue::scalar(1.0e6),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.6),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0001"),
            BoundaryValue::scalar(0.2),
        );
        state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.2),
        );
        state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_ul_0001"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_ssc_0001"),
            BoundaryValue::scalar(1.0e-5),
        );
        state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
        state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("coca_0001"),
            BoundaryValue::scalar(1.0),
        );

        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("Pe"), BoundaryValue::scalar(0.0));

        let request = HillslopeKernelRequest::with_phase_context(
            "lateral_transfer",
            HillslopeKernelPhaseClass::HydrologyLateralTransfer,
            HillslopeConsumerAdapter::Watbal,
            None,
            &state_surface,
            &flux_surface,
        );

        let mut kernel = Wb11HydrologyKernel;
        let response = kernel.run_hillslope_phase(&request);
        assert_eq!(
            response.status.message_id(),
            "HKERNEL-WB11-LAT-OK-001",
            "lateral transfer must complete nominally for valid drainable pool inputs"
        );

        let q_lateral = flux_field_scalar(&response.writeback.flux_updates, "q")
            .expect("lateral transfer should publish q");
        let soil_water_after =
            state_field_scalar(&response.writeback.state_updates, "wb11_soil_water")
                .expect("lateral transfer should publish wb11_soil_water");
        let drainable_after =
            state_field_scalar(&response.writeback.state_updates, "wb11_drainable_storage")
                .expect("lateral transfer should publish wb11_drainable_storage");

        assert!(
            (q_lateral - 0.4).abs() < 1.0e-12,
            "published q must match realized top-down withdrawal capped by available pool"
        );
        assert!(
            (soil_water_after - 0.1).abs() < 1.0e-12,
            "wb11_soil_water must be reduced by realized q withdrawal"
        );
        assert!(
            drainable_after.abs() < 1.0e-12,
            "wb11_drainable_storage must close to zero after full realized withdrawal"
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0213_wb19_drainage_withdrawal_publishes_realized_qdd_and_qd() {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("solwpv"),
            BoundaryValue::scalar(2006.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_drainable_storage"),
            BoundaryValue::scalar(0.4),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.5),
        );
        state_surface.insert(
            BoundarySymbol::from("wb11_drainage_coefficient"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb19_drain_enabled"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb19_drain_depth"),
            BoundaryValue::scalar(0.8),
        );
        state_surface.insert(
            BoundarySymbol::from("wb19_drain_spacing"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb19_drain_diameter"),
            BoundaryValue::scalar(0.1),
        );
        state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.6),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0001"),
            BoundaryValue::scalar(0.2),
        );
        state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.2),
        );
        state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_ul_0001"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb18_perc_ssc_0001"),
            BoundaryValue::scalar(0.01),
        );
        state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
        state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
        state_surface.insert(
            BoundarySymbol::from("coca_0001"),
            BoundaryValue::scalar(1.0),
        );

        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.02));

        let request = HillslopeKernelRequest::with_phase_context(
            "drainage",
            HillslopeKernelPhaseClass::HydrologyDrainage,
            HillslopeConsumerAdapter::Perc,
            None,
            &state_surface,
            &flux_surface,
        );

        let mut kernel = Wb11HydrologyKernel;
        let response = kernel.run_hillslope_phase(&request);
        assert_eq!(
            response.status.message_id(),
            "HKERNEL-WB11-DRAIN-OK-001",
            "drainage phase must complete nominally for valid drain geometry inputs"
        );

        let qdd = flux_field_scalar(&response.writeback.flux_updates, "Qdd")
            .expect("drainage phase should publish Qdd");
        let qd = flux_field_scalar(&response.writeback.flux_updates, "Qd")
            .expect("drainage phase should publish Qd");
        let soil_water_after =
            state_field_scalar(&response.writeback.state_updates, "wb11_soil_water")
                .expect("drainage phase should publish wb11_soil_water");
        let drainable_after =
            state_field_scalar(&response.writeback.state_updates, "wb11_drainable_storage")
                .expect("drainage phase should publish wb11_drainable_storage");

        assert!(
            (qdd - 0.4).abs() < 1.0e-12,
            "published Qdd must match realized tile withdrawal capped by available drainable pool"
        );
        assert!(
            (qd - 0.42).abs() < 1.0e-12,
            "published Qd must follow q + Qdd coupling with realized Qdd"
        );
        assert!(
            (soil_water_after - 0.1).abs() < 1.0e-12,
            "wb11_soil_water must be reduced by realized Qdd withdrawal"
        );
        assert!(
            drainable_after.abs() < 1.0e-12,
            "wb11_drainable_storage must close to zero after realized drainage withdrawal"
        );
    }

    #[test]
    fn hphys0213_wb12_storage_reconciliation_accepts_realized_wb19_subsurface_flux() {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(
            BoundarySymbol::from("wb12_storage_initial"),
            BoundaryValue::scalar(0.5),
        );
        state_surface.insert(
            BoundarySymbol::from("wb12_storage_closure_tolerance"),
            BoundaryValue::scalar(1.0e-9),
        );
        state_surface.insert(
            BoundarySymbol::from("wb12_precip_input"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("wb12_storage_observed"),
            BoundaryValue::scalar(0.03),
        );

        let mut flux_surface = BTreeMap::new();
        flux_surface.insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("S"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("I"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("ET"), BoundaryValue::scalar(0.05));
        flux_surface.insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.0));
        flux_surface.insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.42));

        let request = HillslopeKernelRequest::with_phase_context(
            "storage_reconciliation",
            HillslopeKernelPhaseClass::HydrologyStorageReconciliation,
            HillslopeConsumerAdapter::Watbal,
            None,
            &state_surface,
            &flux_surface,
        );

        let mut kernel = Wb11HydrologyKernel;
        let response = kernel.run_hillslope_phase(&request);
        assert_eq!(
            response.status.message_id(),
            "HKERNEL-WB12-STORAGE-OK-001",
            "storage reconciliation must accept non-negative closure under realized WB19 subsurface losses"
        );

        let storage_reconciled =
            state_field_scalar(&response.writeback.state_updates, "wb12_storage_reconciled")
                .expect("storage reconciliation should publish wb12_storage_reconciled");
        let closure_delta = flux_field_scalar(
            &response.writeback.flux_updates,
            "wb12_storage_closure_delta",
        )
        .expect("storage reconciliation should publish wb12_storage_closure_delta");

        assert!(
            (storage_reconciled - 0.03).abs() < 1.0e-12,
            "storage reconciliation must preserve WB12 conservation under realized WB19 Qd"
        );
        assert!(
            closure_delta.abs() < 1.0e-12,
            "closure delta must remain within configured tolerance for realized WB19 outputs"
        );
    }

    #[test]
    fn hphys0208_wb11_seed_hard_fails_missing_cpm_symbol() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("nsl", 1.0),
            ("nelem", 1.0),
            ("slplen", 50.0),
            ("tmax", 12.0),
            ("tmin", 2.0),
            ("rad", 43.0),
            ("salb", 0.3),
            ("cancov", 0.0),
            ("lai", 0.0),
            ("prcp", 0.003),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 86_400.0),
            ("intsty_0001", 0.0),
        ]);
        insert_wb11_primary_layer_lineage_symbols(&mut runtime_surface, 0.50, false);

        let error = seed_wb11_runtime_surface_inputs(&mut runtime_surface, ExecutionLane::Daily)
            .expect_err("missing cpm_0001 must fail WB11 seed");
        assert_eq!(error.code(), "CLIHILL-E-011");
        match error {
            HillslopeCliError::RuntimeSurfaceFailure { detail, .. } => {
                assert!(
                    detail.contains("missing required runtime symbol cpm_0001"),
                    "expected missing cpm_0001 guard detail, observed: {detail}"
                );
            }
            other => panic!("expected RuntimeSurfaceFailure, observed {other}"),
        }
    }

    #[test]
    fn hillstab08_wb16_producer_single_ofe_projects_expected_alpha_lineage() {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("nelem"), BoundaryValue::scalar(1.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
        insert_wb16_ofe_projection_symbols(
            &mut runtime_surface,
            1,
            Wb16OfeSeedVector {
                avgslp: 0.04,
                slplen: 30.0,
                inrcov: 0.45,
                rilcov: 0.30,
                rrinit: 0.02,
                rspace: 1.20,
                width: 0.40,
                rtyp: 2.0,
                cancov: 0.50,
                canhgt: 1.00,
                bb_seed: 0.10,
                bbb_seed: 0.20,
                flivmx_seed: 0.60,
                hmax_seed: 2.00,
            },
        );

        let produced = produce_wb16_ealpha_from_runtime_surface(&mut runtime_surface)
            .expect("single-OFE WB16 producer should execute")
            .expect("single-OFE WB16 producer should return ealpha");
        let projected_primary_alpha =
            require_runtime_surface_scalar(&runtime_surface, "ofe1_alpha")
                .expect("producer should publish OFE alpha");
        let projected_equivalent_alpha = require_runtime_surface_scalar(&runtime_surface, "ealpha")
            .expect("producer should publish equivalent-plane alpha");
        let projected_frcteq = require_runtime_surface_scalar(&runtime_surface, "ofe1_frcteq")
            .expect("producer should publish OFE friction equivalent");

        let expected_frcteq = wb16_expected_frcteq(0.45, 0.30, 0.02, 1.20, 0.40, 0.60, 1.00, 2.00);
        let expected_alpha = ((0.04 * 8.0 * WB16_ACCGAV_M_S2) / expected_frcteq).sqrt();

        assert!(
            (projected_frcteq - expected_frcteq).abs() < 1.0e-12,
            "frcteq lineage should match baseline-authoritative chain"
        );
        assert!(
            (projected_primary_alpha - expected_alpha).abs() < 1.0e-12,
            "single-OFE alpha should match baseline-authoritative chain"
        );
        assert!(
            (projected_equivalent_alpha - expected_alpha).abs() < 1.0e-12,
            "single-OFE ealpha should equal alpha"
        );
        assert!(
            (produced - expected_alpha).abs() < 1.0e-12,
            "producer return value should match expected single-OFE ealpha"
        );
    }

    #[test]
    fn hillstab08_wb16_producer_multiofe_projects_expected_equivalent_plane_alpha() {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("nelem"), BoundaryValue::scalar(2.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
        insert_wb16_ofe_projection_symbols(
            &mut runtime_surface,
            1,
            Wb16OfeSeedVector {
                avgslp: 0.03,
                slplen: 20.0,
                inrcov: 0.50,
                rilcov: 0.25,
                rrinit: 0.02,
                rspace: 1.10,
                width: 0.30,
                rtyp: 2.0,
                cancov: 0.45,
                canhgt: 0.80,
                bb_seed: 0.10,
                bbb_seed: 0.20,
                flivmx_seed: 0.55,
                hmax_seed: 1.80,
            },
        );
        insert_wb16_ofe_projection_symbols(
            &mut runtime_surface,
            2,
            Wb16OfeSeedVector {
                avgslp: 0.06,
                slplen: 35.0,
                inrcov: 0.35,
                rilcov: 0.20,
                rrinit: 0.03,
                rspace: 1.30,
                width: 0.50,
                rtyp: 2.0,
                cancov: 0.40,
                canhgt: 0.70,
                bb_seed: 0.10,
                bbb_seed: 0.20,
                flivmx_seed: 0.50,
                hmax_seed: 1.70,
            },
        );

        let produced = produce_wb16_ealpha_from_runtime_surface(&mut runtime_surface)
            .expect("multi-OFE WB16 producer should execute")
            .expect("multi-OFE WB16 producer should return ealpha");
        let ofe1_alpha = require_runtime_surface_scalar(&runtime_surface, "ofe1_alpha")
            .expect("producer should publish first OFE alpha");
        let ofe2_alpha = require_runtime_surface_scalar(&runtime_surface, "ofe2_alpha")
            .expect("producer should publish second OFE alpha");
        let projected_ealpha = require_runtime_surface_scalar(&runtime_surface, "ealpha")
            .expect("producer should publish equivalent-plane alpha");

        let expected_ealpha =
            wb16_expected_multiofe_ealpha([20.0, 35.0], [ofe1_alpha, ofe2_alpha], 1.5);

        assert!(
            (projected_ealpha - expected_ealpha).abs() < 1.0e-12,
            "multi-OFE ealpha should match baseline-authoritative eplane projection"
        );
        assert!(
            (produced - expected_ealpha).abs() < 1.0e-12,
            "producer return value should match expected multi-OFE ealpha"
        );
    }

    #[derive(Clone, Copy)]
    struct Wb16OfeSeedVector {
        avgslp: f64,
        slplen: f64,
        inrcov: f64,
        rilcov: f64,
        rrinit: f64,
        rspace: f64,
        width: f64,
        rtyp: f64,
        cancov: f64,
        canhgt: f64,
        bb_seed: f64,
        bbb_seed: f64,
        flivmx_seed: f64,
        hmax_seed: f64,
    }

    fn insert_wb16_ofe_projection_symbols(
        runtime_surface: &mut HillslopeWritebackSurface,
        ofe_index: usize,
        seed: Wb16OfeSeedVector,
    ) {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_avgslp")),
            BoundaryValue::scalar(seed.avgslp),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_slplen")),
            BoundaryValue::scalar(seed.slplen),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_inrcov")),
            BoundaryValue::scalar(seed.inrcov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rilcov")),
            BoundaryValue::scalar(seed.rilcov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rrinit")),
            BoundaryValue::scalar(seed.rrinit),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rspace")),
            BoundaryValue::scalar(seed.rspace),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_width")),
            BoundaryValue::scalar(seed.width),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rtyp")),
            BoundaryValue::scalar(seed.rtyp),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_cancov")),
            BoundaryValue::scalar(seed.cancov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_canhgt")),
            BoundaryValue::scalar(seed.canhgt),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_bb_seed")),
            BoundaryValue::scalar(seed.bb_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_bbb_seed")),
            BoundaryValue::scalar(seed.bbb_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_flivmx_seed")),
            BoundaryValue::scalar(seed.flivmx_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_hmax_seed")),
            BoundaryValue::scalar(seed.hmax_seed),
        );
    }

    #[allow(clippy::too_many_arguments, clippy::similar_names)]
    fn wb16_expected_frcteq(
        inrcov: f64,
        rilcov: f64,
        rrinit: f64,
        rspace: f64,
        width: f64,
        flivmx_seed: f64,
        canhgt: f64,
        hmax_seed: f64,
    ) -> f64 {
        let inrfo = (3.024 - 5.042 * (-161.0 * rrinit).exp()).exp();
        let mut inrrou = 0.5 * inrfo.powf(1.128);
        if inrrou < WB16_INRFSO_CROPLAND {
            inrrou = WB16_INRFSO_CROPLAND;
        }
        let inrfro = inrrou - WB16_INRFSO_CROPLAND;
        let inrfco = if inrcov > 0.0 {
            14.5 * inrcov.powf(1.5544)
        } else {
            0.0
        };
        let frlive = if hmax_seed > 0.0 {
            (canhgt / hmax_seed) * flivmx_seed
        } else {
            0.0
        };
        let inrfto = inrfro + inrfco + WB16_INRFSO_CROPLAND + frlive;
        let frccov = if rilcov > 0.0 {
            4.5 * rilcov.powf(1.5544)
        } else {
            0.0
        };
        let frctrl = frccov + frlive + WB16_FRCSOL_CROPLAND;
        let width_ratio = width / rspace;
        if width_ratio < 1.0 {
            inrfto + width_ratio * (frctrl - inrfto)
        } else {
            inrfto
        }
    }

    fn wb16_expected_multiofe_ealpha(slplens: [f64; 2], alphas: [f64; 2], m: f64) -> f64 {
        let power2 = 1.0 / m;
        let power3 = power2 + 1.0;
        let sum_length = slplens.iter().sum::<f64>();
        let mut cumulative_length = 0.0;
        let mut storage_integral = 0.0;
        let mut last_power = 0.0;
        for (slope_length, alpha_value) in slplens.into_iter().zip(alphas) {
            cumulative_length += slope_length;
            let current_power = cumulative_length.powf(power3);
            storage_integral += (current_power - last_power) / alpha_value.powf(power2);
            last_power = current_power;
        }
        (sum_length / storage_integral).powf(m) * sum_length
    }

    fn seeded_wb13_runtime_surface_probe() -> HillslopeWritebackSurface {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.004));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(12.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(2.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.25));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.25));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );

        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_depth_mm"),
            BoundaryValue::scalar(250.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_porosity_cap_mm"),
            BoundaryValue::scalar(120.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_store_mm"),
            BoundaryValue::scalar(75.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_tail_mm"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_wp_store_mm"),
            BoundaryValue::scalar(30.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.075),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_ws_frz"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Irr"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.000_20));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(0.000_10));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Er"), BoundaryValue::scalar(0.000_05));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.000_10));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("SubRIn"), BoundaryValue::scalar(0.0));
        runtime_surface
    }

    fn canonical_calendar_day_probe() -> ClimateDayProjection {
        ClimateDayProjection {
            year: 2000,
            month: 1,
            day_of_month: 1,
            julian_day: 1,
            precipitation_mm: 4.0,
        }
    }

    fn execute_fixture_run(prefix: &str) -> (HillslopeRunReport, PathBuf) {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
        let output_dir = temp_run_dir.join("output");

        let report = execute_hillslope_run(
            &HillslopeRunRequest {
                run_dir: temp_run_dir.clone(),
                run_file: PathBuf::from("case.run"),
                output_dir,
                sidecar_policy: SidecarPolicy::Compat,
                legacy_sidecar_discovery: false,
                manifest_path: None,
            },
            &["openwepp-cli-hill".to_string()],
        )
        .expect("fixture run should complete");

        (report, temp_run_dir)
    }

    fn runner_execution_lock() -> &'static Mutex<()> {
        static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        RUN_LOCK.get_or_init(|| Mutex::new(()))
    }

    fn fixture_path(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/cli01")
            .join(name)
    }

    fn copy_fixture_to_temp(source_dir: &Path, prefix: &str) -> PathBuf {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("unix epoch should be before now")
            .as_nanos();
        let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));
        copy_dir_recursive(source_dir, &destination);
        destination
    }

    fn copy_dir_recursive(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).expect("destination directory should be creatable");

        for entry in fs::read_dir(source).expect("source directory should be readable") {
            let entry = entry.expect("directory entry should be readable");
            let source_path = entry.path();
            let destination_path = destination.join(entry.file_name());
            if source_path.is_dir() {
                copy_dir_recursive(&source_path, &destination_path);
            } else {
                fs::copy(&source_path, &destination_path).expect("file copy should succeed");
            }
        }
    }

    #[test]
    fn hphys0245_trace_config_limits_requested_days() {
        let config = Hphys0245TraceConfig {
            path: PathBuf::from("trace.jsonl"),
            max_days: Some(30),
        };

        assert!(config.includes_day(1));
        assert!(config.includes_day(30));
        assert!(!config.includes_day(31));

        let unbounded = Hphys0245TraceConfig {
            path: PathBuf::from("trace.jsonl"),
            max_days: None,
        };
        assert!(unbounded.includes_day(31));
    }

    #[test]
    fn hphys0245_trace_row_captures_storage_and_percolation_symbols() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.25),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.10),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0002"),
            BoundaryValue::scalar(0.12),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("wb18_perc_pei_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("wb18_perc_pei_0002"),
            BoundaryValue::scalar(0.004),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.004));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Pe"), BoundaryValue::scalar(0.004));

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            1,
            2013,
            1,
            "post_phase",
            Some("percolation_deep_seepage"),
            &surface,
            None,
            None,
        );

        assert_eq!(row.schema, HPHYS0245_TRACE_SCHEMA);
        assert_eq!(row.run_name, "H1");
        assert_eq!(row.boundary, "post_phase");
        assert_eq!(row.phase.as_deref(), Some("percolation_deep_seepage"));
        assert!((row.wb11_soil_water_m.expect("wb11") - 0.25).abs() < 1.0e-12);
        assert!((row.wb11_soil_water_mm.expect("wb11 mm") - 250.0).abs() < 1.0e-12);
        assert!((row.wb18_theta_sum_m.expect("theta sum") - 0.22).abs() < 1.0e-12);
        assert!((row.wb18_pei_sum_m.expect("pei sum") - 0.007).abs() < 1.0e-12);
        assert!((row.d_m.expect("D") - 0.004).abs() < 1.0e-12);
        assert!((row.pe_m.expect("Pe") - 0.004).abs() < 1.0e-12);
        assert!((row.wb11_minus_theta_sum_m.expect("delta") - 0.03).abs() < 1.0e-12);
    }

    #[test]
    fn hphys0259_trace_row_captures_wb19_lateral_diagnostics() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_q_lateral_potential"),
            BoundaryValue::scalar(0.120),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_q_lateral_target"),
            BoundaryValue::scalar(0.080),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_capacity_tdv"),
            BoundaryValue::scalar(0.080),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_tdvv"),
            BoundaryValue::scalar(0.080),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_q_lateral_unrealized"),
            BoundaryValue::scalar(0.020),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_withdrawal_0001"),
            BoundaryValue::scalar(0.030),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_withdrawal_0002"),
            BoundaryValue::scalar(0.050),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_capacity_active_count_0001"),
            BoundaryValue::scalar(24.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_conductivity_active_count_0001"),
            BoundaryValue::scalar(12.0),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.080));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.010));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.090));

        let row = build_hphys0245_trace_row(
            "H39",
            1,
            1,
            2013,
            1,
            "post_phase",
            Some("lateral_transfer"),
            &surface,
            None,
            None,
        );

        assert_eq!(row.schema, HPHYS0245_TRACE_SCHEMA);
        assert_eq!(row.phase.as_deref(), Some("lateral_transfer"));
        assert!((row.wb19_q_lateral_potential_m.expect("potential") - 0.120).abs() < 1.0e-12);
        assert!((row.wb19_q_lateral_target_m.expect("target") - 0.080).abs() < 1.0e-12);
        assert!((row.wb19_lateral_capacity_tdv_m.expect("capacity tdv") - 0.080).abs() < 1.0e-12);
        assert!((row.wb19_tdvv_m.expect("tdvv") - 0.080).abs() < 1.0e-12);
        assert!((row.wb19_q_lateral_unrealized_m.expect("unrealized") - 0.020).abs() < 1.0e-12);
        assert_eq!(
            row.wb19_lateral_withdrawal_layers_m.get("0001").copied(),
            Some(0.030)
        );
        assert_eq!(
            row.wb19_lateral_withdrawal_layers_m.get("0002").copied(),
            Some(0.050)
        );
        assert_eq!(
            row.wb19_lateral_capacity_active_count_layers
                .get("0001")
                .copied(),
            Some(24.0)
        );
        assert_eq!(
            row.wb19_lateral_conductivity_active_count_layers
                .get("0001")
                .copied(),
            Some(12.0)
        );
        assert!((row.q_m.expect("q") - 0.080).abs() < 1.0e-12);
        assert!((row.qdd_m.expect("Qdd") - 0.010).abs() < 1.0e-12);
        assert!((row.qd_m.expect("Qd") - 0.090).abs() < 1.0e-12);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0260_trace_row_captures_wb17_wb18_storage_diagnostics() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.256),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.10),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0002"),
            BoundaryValue::scalar(0.12),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetdr_0001"),
            BoundaryValue::scalar(0.05),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetdr_0002"),
            BoundaryValue::scalar(0.07),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_dg_0001"),
            BoundaryValue::scalar(0.30),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_dg_0002"),
            BoundaryValue::scalar(0.40),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0001"),
            BoundaryValue::scalar(0.030),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_fc_0002"),
            BoundaryValue::scalar(0.040),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("coca_0001"),
            BoundaryValue::scalar(0.80),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb19_coca_0002"),
            BoundaryValue::scalar(0.75),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_frzw_0002"),
            BoundaryValue::scalar(0.005),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_frzw_0001"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_frozen_depth_0002"),
            BoundaryValue::scalar(0.10),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("UPi"), BoundaryValue::scalar(0.005));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Ui"), BoundaryValue::scalar(0.0025));
        surface.flux_surface.insert(
            BoundarySymbol::from("UPi_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("UPi_0002"),
            BoundaryValue::scalar(0.002),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("Ui_0001"),
            BoundaryValue::scalar(0.001),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("Ui_0002"),
            BoundaryValue::scalar(0.0015),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.0025));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Etp"), BoundaryValue::scalar(0.005));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Ws"), BoundaryValue::scalar(0.5));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.004));
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Pe"), BoundaryValue::scalar(0.004));

        let row = build_hphys0245_trace_row(
            "H7",
            1,
            1,
            2013,
            1,
            "post_phase",
            Some("plant_root_uptake"),
            &surface,
            None,
            None,
        );

        assert_eq!(row.schema, HPHYS0245_TRACE_SCHEMA);
        assert_eq!(row.wb17_upi_layers_m.get("0001").copied(), Some(0.003));
        assert_eq!(row.wb17_upi_layers_m.get("0002").copied(), Some(0.002));
        assert_eq!(row.wb17_ui_layers_m.get("0001").copied(), Some(0.001));
        assert_eq!(row.wb17_ui_layers_m.get("0002").copied(), Some(0.0015));
        assert_eq!(row.wb18_thetdr_layers.get("0001").copied(), Some(0.05));
        assert_eq!(row.wb18_dg_layers_m.get("0002").copied(), Some(0.40));
        assert_eq!(row.wb18_fc_layers_m.get("0001").copied(), Some(0.030));
        assert_eq!(row.wb19_coca_layers.get("0001").copied(), Some(0.80));
        assert_eq!(row.wb19_coca_layers.get("0002").copied(), Some(0.75));
        assert_eq!(row.wb19_frzw_layers_m.get("0002").copied(), Some(0.005));
        assert!((row.wb19_drfc_layers_m["0001"] - 0.090).abs() < 1.0e-12);
        assert!((row.wb19_drfc_layers_m["0002"] - 0.140).abs() < 1.0e-12);
        assert!((row.wb19_fzdrfc_layers_m["0001"] - 0.090).abs() < 1.0e-12);
        assert!((row.wb19_fzdrfc_layers_m["0002"] - 0.135).abs() < 1.0e-12);
        assert_eq!(
            row.wb18_frozen_depth_layers_m.get("0002").copied(),
            Some(0.10)
        );
        assert!((row.wb18_recomputed_soil_water_m.expect("aggregate") - 0.256).abs() < 1.0e-12);
        assert!((row.wb18_recomputed_minus_wb11_m.expect("delta")).abs() < 1.0e-12);
        assert!((row.upi_m.expect("UPi") - 0.005).abs() < 1.0e-12);
        assert!((row.ui_m.expect("Ui") - 0.0025).abs() < 1.0e-12);
        assert!((row.ep_m.expect("Ep") - 0.0025).abs() < 1.0e-12);
        assert!((row.ws.expect("Ws") - 0.5).abs() < 1.0e-12);
        assert!((row.d_m.expect("D") - row.pe_m.expect("Pe")).abs() < 1.0e-12);
    }

    #[test]
    fn hphys0261_trace_row_captures_ep_initialization_magnitude_lineage() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_theta_0001"),
            BoundaryValue::scalar(0.052),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb18_perc_ul_0001"),
            BoundaryValue::scalar(0.113),
        );
        surface
            .state_surface
            .insert(BoundarySymbol::from("pltol"), BoundaryValue::scalar(0.33));
        surface.state_surface.insert(
            BoundarySymbol::from("swu_effective_pltol"),
            BoundaryValue::scalar(0.33),
        );
        surface
            .state_surface
            .insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(11.8));
        surface
            .state_surface
            .insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(1.8));
        surface.flux_surface.insert(
            BoundarySymbol::from("UPi_0001"),
            BoundaryValue::scalar(0.0001),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("Ui_0001"),
            BoundaryValue::scalar(0.0001),
        );
        surface.flux_surface.insert(
            BoundarySymbol::from("Etp"),
            BoundaryValue::scalar(0.000_385),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.000_385));

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            1,
            2013,
            1,
            "post_phase",
            Some("plant_root_uptake"),
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["pl_pltol"], 0.33);
        assert_eq!(document["pl_swu_effective_pltol"], 0.33);
        assert_eq!(document["wb18_ul_layers_m"]["0001"], 0.113);
        assert!(
            (document["wb17_swu_stress_threshold_layers_m"]["0001"]
                .as_f64()
                .unwrap()
                - 0.03729)
                .abs()
                < 1.0e-12
        );
        assert!(
            document["wb17_swu_storage_to_threshold_layers"]["0001"]
                .as_f64()
                .unwrap()
                > 1.0
        );
    }

    #[test]
    fn hphys0262_trace_row_captures_pmet_demand_seeding_lineage() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.mode.sidecar_present"),
            BoundaryValue::scalar(1.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.mode.iflget"),
            BoundaryValue::scalar(2.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.selected.kcb"),
            BoundaryValue::scalar(0.95),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.selected.rawp"),
            BoundaryValue::scalar(0.80),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.selected.line_index"),
            BoundaryValue::scalar(39.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pmetpara.lookup.fallback_first_row_used"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb11_et_demand"),
            BoundaryValue::scalar(0.000_385),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("wb11_et_seed_branch_priestley_taylor"),
            BoundaryValue::scalar(1.0),
        );

        let row = build_hphys0245_trace_row(
            "H39",
            1,
            1,
            2013,
            1,
            "post_seed",
            None,
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["pmet_sidecar_present"], 1.0);
        assert_eq!(document["pmet_iflget"], 2.0);
        assert_eq!(document["pmet_selected_kcb"], 0.95);
        assert_eq!(document["pmet_selected_rawp"], 0.80);
        assert_eq!(document["pmet_selected_line_index"], 39.0);
        assert_eq!(document["pmet_lookup_fallback_first_row_used"], 0.0);
        assert_eq!(document["wb11_et_demand_m"], 0.000_385);
        assert_eq!(document["wb11_et_seed_branch"], "evap_priestley_taylor");
    }

    #[test]
    fn hphys0262_projects_pmetpara_selected_crop_coefficients() {
        let fixture_dir = fixture_path("hillslope_run_dir");
        let management = parse_management_from_path(
            fixture_dir.join("case.man"),
            SidecarPolicy::Compat.as_management_parser_mode(),
        )
        .expect("fixture management should parse");
        let mut pmetpara = parse_pmetpara_file(
            fixture_dir.join("pmetpara.txt"),
            PmetparaParseOptions {
                mode: SidecarPolicy::Compat.as_pmetpara_parse_mode(),
                require_sidecar: true,
            },
        )
        .expect("fixture pmetpara should parse");

        let surface = build_hillslope_runtime_surface_from_pmetpara(
            &management,
            &mut pmetpara,
            SidecarPolicy::Compat.as_pmetpara_parse_mode(),
        )
        .expect("pmetpara should project");

        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.mode.sidecar_present"),
            Some(1.0)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.mode.iflget"),
            Some(2.0)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.selected.kcb"),
            Some(1.20)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.selected.rawp"),
            Some(0.55)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.selected.line_index"),
            Some(1.0)
        );
        assert_eq!(
            runtime_surface_symbol_value(&surface, "pmetpara.lookup.fallback_first_row_used"),
            Some(0.0)
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0245_trace_writer_serializes_jsonl_rows() {
        let temp_dir = std::env::temp_dir().join(format!(
            "openwepp_hphys0245_trace_writer_{}",
            std::process::id()
        ));
        let trace_path = temp_dir.join("trace.jsonl");
        let config = Hphys0245TraceConfig {
            path: trace_path.clone(),
            max_days: Some(1),
        };
        let row = Hphys0245TraceRow {
            schema: HPHYS0245_TRACE_SCHEMA,
            run_name: "H1".to_string(),
            sim_day_index: 1,
            simulation_year: 1,
            calendar_year: 2013,
            julian_day: 1,
            boundary: "post_seed".to_string(),
            phase: None,
            wb11_soil_water_m: Some(0.1),
            wb11_soil_water_mm: Some(100.0),
            wb12_infiltration_m: Some(0.003),
            wb18_theta_sum_m: Some(0.08),
            wb18_theta_layers_m: BTreeMap::from([("0001".to_string(), 0.08)]),
            wb18_thetdr_layers: BTreeMap::from([("0001".to_string(), 0.05)]),
            wb18_dg_layers_m: BTreeMap::from([("0001".to_string(), 0.40)]),
            wb18_fc_layers_m: BTreeMap::from([("0001".to_string(), 0.06)]),
            wb19_coca_layers: BTreeMap::from([("0001".to_string(), 0.75)]),
            wb19_frzw_layers_m: BTreeMap::from([("0001".to_string(), 0.01)]),
            wb19_drfc_layers_m: BTreeMap::from([("0001".to_string(), 0.16)]),
            wb19_fzdrfc_layers_m: BTreeMap::from([("0001".to_string(), 0.15)]),
            wb18_frozen_depth_layers_m: BTreeMap::new(),
            wb18_recomputed_soil_water_m: Some(0.10),
            wb18_recomputed_minus_wb11_m: Some(0.0),
            wb18_pei_sum_m: Some(0.0),
            wb18_pei_layers_m: BTreeMap::new(),
            d_m: None,
            pe_m: None,
            wb13_dp_mm: None,
            wb13_total_soil_mm: None,
            wb13_soil_water_total_mm: None,
            snow_runtime_swe_m: Some(0.42),
            snow_runtime_depth_m: Some(1.20),
            snow_runtime_density_kg_m3: Some(350.0),
            snow_runtime_settle_day_count: Some(4.0),
            snow_runtime_swe_before_m: Some(0.40),
            snow_runtime_depth_before_m: Some(1.10),
            snow_runtime_density_before_kg_m3: Some(340.0),
            snow_runtime_settle_day_count_before: Some(3.0),
            snow_runtime_swe_delta_m: Some(0.02),
            snow_runtime_depth_delta_m: Some(0.10),
            snow_runtime_density_delta_kg_m3: Some(10.0),
            snow_runtime_settle_day_count_delta: Some(1.0),
            snow_s_m: Some(0.002),
            snow_hourly_rain_sum_m: Some(0.001),
            snow_hourly_rain_retained_sum_m: Some(0.0),
            snow_hourly_rain_released_sum_m: Some(0.0),
            snow_hourly_snowfall_depth_sum_m: Some(0.010),
            snow_hourly_snowfall_water_equiv_sum_m: Some(0.001),
            snow_hourly_melt_raw_sum_m: Some(0.003),
            snow_hourly_melt_sum_m: Some(0.003),
            snow_hourly_melt_raw_m: BTreeMap::from([("0001".to_string(), 0.003)]),
            snow_hourly_melt_m: BTreeMap::from([("0001".to_string(), 0.003)]),
            snow_hourly_melt_amelt_in: BTreeMap::from([("0001".to_string(), 0.10)]),
            snow_hourly_melt_bmelt_in: BTreeMap::from([("0001".to_string(), 0.20)]),
            snow_hourly_melt_cmelt_in: BTreeMap::from([("0001".to_string(), 0.30)]),
            snow_hourly_melt_dmelt_in: BTreeMap::from([("0001".to_string(), 0.40)]),
            snow_hourly_melt_hrtef_f: BTreeMap::from([("0001".to_string(), 36.0)]),
            snow_hourly_melt_hrdtf_f: BTreeMap::from([("0001".to_string(), 30.0)]),
            snow_hourly_melt_vwmph: BTreeMap::from([("0001".to_string(), 4.0)]),
            snow_hourly_melt_rainin: BTreeMap::from([("0001".to_string(), 0.01)]),
            snow_hourly_melt_wind_adjustment: BTreeMap::from([("0001".to_string(), 1.07)]),
            snow_hourly_melt_branch_active: BTreeMap::from([("0001".to_string(), 1.0)]),
            winter_hourly_air_temp_c: BTreeMap::from([("0001".to_string(), 2.0)]),
            winter_hourly_rad_mj_m2: BTreeMap::from([("0001".to_string(), 1.5)]),
            winter_hourly_cloud_fraction: BTreeMap::from([("0001".to_string(), 0.5)]),
            winter_hourly_dewpoint_c: BTreeMap::from([("0001".to_string(), -1.0)]),
            winter_hourly_wind_m_s: BTreeMap::from([("0001".to_string(), 2.0)]),
            snow_runtime_swe_closure_error_m: Some(0.0),
            wb13_p_mm: Some(10.0),
            wb13_rm_mm: Some(2.0),
            wb13_snow_water_mm: Some(420.0),
            wb11_minus_theta_sum_m: Some(0.02),
            pl_sumgdd: Some(42.0),
            pl_vdmt: Some(1.5),
            pl_cancov: Some(0.4),
            pl_lai: Some(1.2),
            pl_rtmass: Some(0.7),
            pl_rtd: Some(0.6),
            pl_hia: Some(0.2),
            pl_pltol: Some(0.33),
            pl_swu_effective_pltol: Some(0.33),
            pmet_sidecar_present: Some(1.0),
            pmet_iflget: Some(2.0),
            pmet_selected_kcb: Some(0.95),
            pmet_selected_rawp: Some(0.8),
            pmet_selected_line_index: Some(1.0),
            pmet_lookup_fallback_first_row_used: Some(0.0),
            wb11_et_demand_m: Some(0.003),
            wb11_et_seed_branch: Some("evappm_pmet".to_string()),
            pmet_etorc_mm: Some(3.5),
            pmet_rn_mj_m2: Some(4.2),
            pmet_fwv_m_s: Some(2.1),
            pmet_rhd_pct: Some(60.0),
            pmet_kcbadj: Some(0.95),
            pmet_kcbcon: Some(0.7),
            pmet_etke: Some(0.3),
            pmet_etkr: Some(1.0),
            pmet_etks: Some(0.8),
            pmet_tew_mm: Some(25.0),
            pmet_rew_mm: Some(8.0),
            pmet_wfevp_mm: Some(12.0),
            pmet_taw_mm: Some(40.0),
            pmet_raw_mm: Some(20.0),
            pmet_wftrp_mm: Some(30.0),
            pmet_es_m: Some(0.001),
            pmet_ep_m: Some(0.003),
            etp_m: Some(0.003),
            upi_m: Some(0.003),
            ui_m: Some(0.002),
            wb18_ul_layers_m: BTreeMap::from([("0001".to_string(), 0.24)]),
            wb17_swu_stress_threshold_layers_m: BTreeMap::from([("0001".to_string(), 0.0792)]),
            wb17_swu_storage_to_threshold_layers: BTreeMap::from([(
                "0001".to_string(),
                1.010_101_010_101_010_2,
            )]),
            wb17_upi_layers_m: BTreeMap::from([("0001".to_string(), 0.003)]),
            wb17_ui_layers_m: BTreeMap::from([("0001".to_string(), 0.002)]),
            ep_m: Some(0.002),
            ws: Some(0.8),
            wb19_q_lateral_potential_m: Some(0.12),
            wb19_q_lateral_target_m: Some(0.08),
            wb19_lateral_capacity_tdv_m: Some(0.08),
            wb19_tdvv_m: Some(0.08),
            wb19_q_lateral_unrealized_m: Some(0.0),
            wb19_lateral_withdrawal_layers_m: BTreeMap::from([("0001".to_string(), 0.08)]),
            wb19_lateral_capacity_active_count_layers: BTreeMap::from([("0001".to_string(), 24.0)]),
            wb19_lateral_conductivity_active_count_layers: BTreeMap::from([(
                "0001".to_string(),
                24.0,
            )]),
            q_m: Some(0.08),
            qdd_m: Some(0.01),
            qd_m: Some(0.09),
        };

        write_hphys0245_trace_jsonl(&config, &[row]).expect("trace writer should succeed");

        let payload = fs::read_to_string(&trace_path).expect("trace file should be readable");
        let lines = payload.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);
        let document: serde_json::Value =
            serde_json::from_str(lines[0]).expect("trace row should parse as JSON");
        assert_eq!(document["schema"], HPHYS0245_TRACE_SCHEMA);
        assert_eq!(document["boundary"], "post_seed");
        assert_eq!(document["wb18_theta_layers_m"]["0001"], 0.08);
        assert_eq!(document["wb18_thetdr_layers"]["0001"], 0.05);
        assert_eq!(document["wb18_dg_layers_m"]["0001"], 0.40);
        assert_eq!(document["wb18_fc_layers_m"]["0001"], 0.06);
        assert_eq!(document["wb19_coca_layers"]["0001"], 0.75);
        assert_eq!(document["wb19_frzw_layers_m"]["0001"], 0.01);
        assert_eq!(document["wb19_drfc_layers_m"]["0001"], 0.16);
        assert_eq!(document["wb19_fzdrfc_layers_m"]["0001"], 0.15);
        assert_eq!(document["wb18_recomputed_soil_water_m"], 0.10);
        assert_eq!(document["pl_pltol"], 0.33);
        assert_eq!(document["pl_swu_effective_pltol"], 0.33);
        assert_eq!(document["pmet_iflget"], 2.0);
        assert_eq!(document["pmet_selected_kcb"], 0.95);
        assert_eq!(document["wb11_et_seed_branch"], "evappm_pmet");
        assert_eq!(document["wb18_ul_layers_m"]["0001"], 0.24);
        assert_eq!(
            document["wb17_swu_stress_threshold_layers_m"]["0001"],
            0.0792
        );
        assert_eq!(document["wb17_upi_layers_m"]["0001"], 0.003);
        assert_eq!(document["wb17_ui_layers_m"]["0001"], 0.002);
        assert_eq!(document["pl_rtd"], 0.6);
        assert_eq!(document["ep_m"], 0.002);
        assert_eq!(document["snow_runtime_swe_m"], 0.42);
        assert_eq!(document["snow_runtime_swe_before_m"], 0.40);
        assert_eq!(document["snow_runtime_swe_delta_m"], 0.02);
        assert_eq!(document["snow_hourly_snowfall_water_equiv_sum_m"], 0.001);
        assert_eq!(document["snow_hourly_rain_released_sum_m"], 0.0);
        assert_eq!(document["wb12_infiltration_m"], 0.003);
        assert_eq!(document["snow_hourly_melt_raw_m"]["0001"], 0.003);
        assert_eq!(document["snow_hourly_melt_m"]["0001"], 0.003);
        assert_eq!(document["snow_hourly_melt_amelt_in"]["0001"], 0.10);
        assert_eq!(document["winter_hourly_air_temp_c"]["0001"], 2.0);
        assert_eq!(document["snow_runtime_swe_closure_error_m"], 0.0);
        assert_eq!(document["wb13_rm_mm"], 2.0);
        assert_eq!(document["wb19_lateral_withdrawal_layers_m"]["0001"], 0.08);
        assert_eq!(document["q_m"], 0.08);

        fs::remove_dir_all(temp_dir).expect("temp trace directory should be removable");
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn hphys0268_trace_row_captures_spring_snowpack_lineage() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.120),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_depth_m"),
            BoundaryValue::scalar(0.600),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_density_kg_m3"),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_settle_day_count"),
            BoundaryValue::scalar(3.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.options.newsnw"),
            BoundaryValue::scalar(100.0),
        );
        surface
            .flux_surface
            .insert(BoundarySymbol::from("S"), BoundaryValue::scalar(0.002));
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_m_0001"),
            BoundaryValue::scalar(0.004),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.snowfall_m_0001"),
            BoundaryValue::scalar(0.010),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_m_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_raw_m_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_retained_m_0001"),
            BoundaryValue::scalar(0.0),
        );
        let wb13_row = SimulationOwnedWb13Row {
            wb13_row: Wb13DailyWaterBalanceRow {
                ofe: 1,
                julian_day: 99,
                year: 1,
                p: 10.0,
                rm: 12.0,
                q: 0.0,
                ep: 1.5,
                es: 0.2,
                er: 0.0,
                dp: 0.1,
                upstrmq: 0.0,
                subrin: 0.0,
                latqcc: 0.0,
                total_soil: 200.0,
                frozwt: 0.0,
                snow_water: 120.0,
                qofe: 0.0,
                tile: 0.0,
                irr: 0.0,
                area: 10_000.0,
                soil_water_total: 200.0,
                profile_depth: 1_000.0,
                profile_porosity_cap: 300.0,
                profile_fc_store: 220.0,
                profile_wp_store: 120.0,
            },
            month: 4,
            day_of_month: 9,
            water_year: 1,
            sim_day_index: 99,
        };

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            99,
            2013,
            99,
            "post_wb13",
            None,
            &surface,
            Some(&wb13_row),
            None,
        );

        assert!((row.snow_runtime_swe_m.expect("runtime swe") - 0.120).abs() < 1.0e-12);
        assert!((row.snow_runtime_depth_m.expect("runtime depth") - 0.600).abs() < 1.0e-12);
        assert!((row.snow_runtime_density_kg_m3.expect("runtime density") - 200.0).abs() < 1.0e-12);
        assert!(
            (row.snow_hourly_snowfall_water_equiv_sum_m
                .expect("snowfall water equivalent")
                - 0.001)
                .abs()
                < 1.0e-12
        );
        assert!(
            (row.snow_runtime_swe_closure_error_m
                .expect("signed S closure")
                - 0.0)
                .abs()
                < 1.0e-12
        );
        assert!((row.wb13_p_mm.expect("WB13 P") - 10.0).abs() < 1.0e-12);
        assert!((row.wb13_rm_mm.expect("WB13 RM") - 12.0).abs() < 1.0e-12);
        assert!((row.wb13_snow_water_mm.expect("WB13 Snow-Water") - 120.0).abs() < 1.0e-12);
    }

    #[test]
    fn hphys0288_trace_row_captures_rain_on_snow_release_without_snowpack_loss() {
        let mut surface = HillslopeWritebackSurface::default();
        surface
            .flux_surface
            .insert(BoundarySymbol::from("S"), BoundaryValue::scalar(-0.001));
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_m_0001"),
            BoundaryValue::scalar(0.003),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_retained_m_0001"),
            BoundaryValue::scalar(0.001),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.rain_released_m_0001"),
            BoundaryValue::scalar(0.002),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_m_0001"),
            BoundaryValue::scalar(0.002),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.snowfall_m_0001"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.options.newsnw"),
            BoundaryValue::scalar(100.0),
        );

        let row = build_hphys0245_trace_row(
            "H39",
            1,
            142,
            2014,
            506,
            "post_snow",
            Some("snow_coupling"),
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["snow_hourly_rain_released_sum_m"], 0.002);
        assert_eq!(document["snow_hourly_melt_sum_m"], 0.002);
        assert_eq!(document["snow_runtime_swe_closure_error_m"], 0.0);
    }

    #[test]
    fn hphys0270_trace_row_captures_pre_day_snowpack_state() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.120),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_depth_m"),
            BoundaryValue::scalar(0.600),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_density_kg_m3"),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_settle_day_count"),
            BoundaryValue::scalar(4.0),
        );
        let snow_runtime_before = Hphys0245SnowRuntimeBeforeState {
            swe_m: Some(0.150),
            depth_m: Some(0.750),
            density_kg_m3: Some(180.0),
            settle_day_count: Some(3.0),
        };

        let row = build_hphys0245_trace_row(
            "H39",
            1,
            115,
            2013,
            115,
            "post_wb13",
            None,
            &surface,
            None,
            Some(snow_runtime_before),
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["schema"], HPHYS0245_TRACE_SCHEMA);
        assert_eq!(document["snow_runtime_swe_before_m"], 0.150);
        assert_eq!(document["snow_runtime_depth_before_m"], 0.750);
        assert_eq!(document["snow_runtime_density_before_kg_m3"], 180.0);
        assert_eq!(document["snow_runtime_settle_day_count_before"], 3.0);
        assert!(
            (document["snow_runtime_swe_delta_m"]
                .as_f64()
                .expect("SWE delta")
                + 0.030)
                .abs()
                < 1.0e-12
        );
        assert!(
            (document["snow_runtime_depth_delta_m"]
                .as_f64()
                .expect("depth delta")
                + 0.150)
                .abs()
                < 1.0e-12
        );
        assert_eq!(document["snow_runtime_density_delta_kg_m3"], 20.0);
        assert_eq!(document["snow_runtime_settle_day_count_delta"], 1.0);
    }

    #[test]
    fn hphys0271_trace_row_captures_melt_term_hourly_forcing_maps() {
        let mut surface = HillslopeWritebackSurface::default();
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_raw_m_0001"),
            BoundaryValue::scalar(0.0254),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_m_0001"),
            BoundaryValue::scalar(0.0200),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_amelt_in_0001"),
            BoundaryValue::scalar(0.10),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_bmelt_in_0001"),
            BoundaryValue::scalar(0.20),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_cmelt_in_0001"),
            BoundaryValue::scalar(0.30),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_dmelt_in_0001"),
            BoundaryValue::scalar(0.40),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_hrtef_f_0001"),
            BoundaryValue::scalar(36.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_hrdtf_f_0001"),
            BoundaryValue::scalar(30.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_vwmph_0001"),
            BoundaryValue::scalar(4.47),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_rainin_0001"),
            BoundaryValue::scalar(0.02),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_wind_adjustment_0001"),
            BoundaryValue::scalar(1.07),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("snow.hourly.melt_branch_active_0001"),
            BoundaryValue::scalar(1.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.air_temp_c_0001"),
            BoundaryValue::scalar(2.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.rad_mj_m2_0001"),
            BoundaryValue::scalar(1.25),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.cloud_fraction_0001"),
            BoundaryValue::scalar(0.5),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.dewpoint_c_0001"),
            BoundaryValue::scalar(-1.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("winter.hourly.wind_m_s_0001"),
            BoundaryValue::scalar(2.0),
        );

        let row = build_hphys0245_trace_row(
            "H1",
            1,
            36,
            2013,
            36,
            "post_wb13",
            None,
            &surface,
            None,
            None,
        );
        let document = serde_json::to_value(&row).expect("trace row should serialize");

        assert_eq!(document["schema"], HPHYS0245_TRACE_SCHEMA);
        assert_eq!(document["snow_hourly_melt_raw_m"]["0001"], 0.0254);
        assert_eq!(document["snow_hourly_melt_m"]["0001"], 0.0200);
        assert_eq!(document["snow_hourly_melt_amelt_in"]["0001"], 0.10);
        assert_eq!(document["snow_hourly_melt_bmelt_in"]["0001"], 0.20);
        assert_eq!(document["snow_hourly_melt_cmelt_in"]["0001"], 0.30);
        assert_eq!(document["snow_hourly_melt_dmelt_in"]["0001"], 0.40);
        assert_eq!(document["snow_hourly_melt_hrtef_f"]["0001"], 36.0);
        assert_eq!(document["snow_hourly_melt_hrdtf_f"]["0001"], 30.0);
        assert_eq!(document["snow_hourly_melt_vwmph"]["0001"], 4.47);
        assert_eq!(document["snow_hourly_melt_rainin"]["0001"], 0.02);
        assert_eq!(document["snow_hourly_melt_wind_adjustment"]["0001"], 1.07);
        assert_eq!(document["snow_hourly_melt_branch_active"]["0001"], 1.0);
        assert_eq!(document["winter_hourly_air_temp_c"]["0001"], 2.0);
        assert_eq!(document["winter_hourly_rad_mj_m2"]["0001"], 1.25);
        assert_eq!(document["winter_hourly_cloud_fraction"]["0001"], 0.5);
        assert_eq!(document["winter_hourly_dewpoint_c"]["0001"], -1.0);
        assert_eq!(document["winter_hourly_wind_m_s"]["0001"], 2.0);
    }

    fn read_manifest_json(report: &HillslopeRunReport) -> serde_json::Value {
        let manifest_text = fs::read_to_string(&report.manifest_path).unwrap_or_else(|error| {
            panic!(
                "manifest should be readable at {}: {error}",
                report.manifest_path.display()
            )
        });
        serde_json::from_str(&manifest_text)
            .unwrap_or_else(|error| panic!("manifest should parse as JSON: {error}"))
    }

    fn assert_json_i64(document: &serde_json::Value, pointer: &str, expected: i64) {
        let observed = document
            .pointer(pointer)
            .and_then(serde_json::Value::as_i64)
            .unwrap_or_else(|| panic!("missing integer JSON pointer {pointer}"));
        assert_eq!(observed, expected, "unexpected value at {pointer}");
    }
}
