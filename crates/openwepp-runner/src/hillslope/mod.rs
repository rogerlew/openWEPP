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
use openwepp_input_contract::parsers::management::parse_management_from_path;
use openwepp_input_contract::parsers::pmetpara::{PmetparaParseOptions, parse_pmetpara_file};
use openwepp_input_contract::parsers::slope::{SlopeProfile, parse_slope_file};
use openwepp_input_contract::parsers::snow::{
    SnowParseOutput, parse_snow_file, parse_snow_from_str,
};
use openwepp_input_contract::parsers::soil::{SoilParserOptions, TopologyScope, parse_soil};
use openwepp_input_contract::parsers::wepp_ui::{
    WeppUiParseResult, WeppUiParserOptions, parse_wepp_ui_from_path,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
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
    publication_area_m2: f64,
    row_count: usize,
    sim_day_index_monotonic: bool,
    first_row_key: HillslopeWb13RowKeyProvenance,
    last_row_key: HillslopeWb13RowKeyProvenance,
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
}

const WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL: &str = "wb16_ealpha_compatibility_seed_used";
const WB16_EALPHA_SEED_POLICY_RUNTIME_PROVIDED: &str = "runtime_provided";
const WB16_EALPHA_SEED_POLICY_COMPATIBILITY: &str = "compatibility_seed_1p0";
const WB16_EALPHA_SEED_WARNING_ID: &str = "SIMPIPE-W-003";

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

    let (snow, frost, wepp_ui_mode_selection) = if request.legacy_sidecar_discovery {
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

        let _pmetpara = parse_pmetpara_file(
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

        (snow, frost, build_mode_selection_provenance(&wepp_ui)?)
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

        if let Some(pmetpara_path) = sidecar_overrides.pmetpara_path.clone() {
            pmetpara_input_path = Some(pmetpara_path.clone());
            resolved_sidecars.insert("pmetpara".to_string(), pmetpara_path.display().to_string());

            let _pmetpara = parse_pmetpara_file(
                &pmetpara_path,
                PmetparaParseOptions {
                    mode: request.sidecar_policy.as_pmetpara_parse_mode(),
                    require_sidecar: true,
                },
            )
            .map_err(|error| HillslopeCliError::ParseFailure {
                surface: "pmetpara",
                detail: error.to_string(),
            })?;
        }

        (snow, frost, build_mode_selection_provenance(&wepp_ui)?)
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
        merge_runtime_surfaces(snow_surface, frost_surface),
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
            lane_context.lane,
            publication_area_m2,
            simulation_year,
            day_index + 1,
            day_projection,
            runtime_swe_publication_state_m,
        )?;
        runtime_surface = execution_result.runtime_surface;
        runtime_swe_publication_state_m = execution_result.wb13_row.wb13_row.snow_water / 1_000.0;
        scheduler_outcome_class = execution_result.scheduler_outcome_class;
        scheduler_status_message_id = execution_result.scheduler_status_message_id;
        coupling_vectors = Some(execution_result.coupling_vectors);
        for message_id in execution_result.kernel_phase_message_ids {
            kernel_phase_message_ids.insert(message_id);
        }
        erod14_wave2_kernel_status_seen |= execution_result.erod14_wave2_kernel_status_seen;
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

    let nsl = scalar_to_usize(
        "nsl",
        require_runtime_surface_scalar(runtime_surface, "nsl")?,
    )?;
    if nsl == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} nsl must be >= 1 for WB11 seeding"),
        });
    }

    let tmax = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let tmin = require_runtime_surface_scalar(runtime_surface, "tmin")?;
    let rad = require_runtime_surface_scalar(runtime_surface, "rad")?;
    if rad < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} rad must be >= 0.0, observed {rad}"),
        });
    }
    let salb = require_runtime_surface_scalar(runtime_surface, "salb")?;
    if !(0.0..=1.0).contains(&salb) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} salb must be within [0,1], observed {salb}"),
        });
    }
    let cancov = require_runtime_surface_scalar(runtime_surface, "cancov")?;
    if cancov < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} cancov must be >= 0.0, observed {cancov}"),
        });
    }
    let lai = require_runtime_surface_scalar(runtime_surface, "lai")?;
    if lai < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} lai must be >= 0.0, observed {lai}"),
        });
    }
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

    // Baseline-authoritative Priestley-Taylor potential ET branch from evap.for.
    let tave = 0.5 * (tmax + tmin);
    let tk = tave + 273.0;
    if tk <= 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} derived tk must be > 0.0, observed {tk}"),
        });
    }
    let delta = (21.255 - 5304.0 / tk).exp() * 5304.0 / (tk * tk);
    let gamma = delta / (delta + 0.68);
    let eaj = (-0.5 * (cancov + 0.1)).exp();
    let alb = if lai > 0.0 {
        0.23 * (1.0 - eaj) + salb * eaj
    } else {
        salb
    };
    let wb11_et_demand = (0.00128 * ((rad * (1.0 - alb)) / 58.3) * gamma).max(0.0);
    if !wb11_et_demand.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} derived wb11_et_demand is non-finite ({wb11_et_demand})"
            ),
        });
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
            let dg_symbol = wb13_primary_layer_symbol("dg", layer_index);
            let fc_symbol = wb13_primary_layer_symbol("thetfc", layer_index);
            let wp_symbol = wb13_primary_layer_symbol("thetdr", layer_index);
            let ssc_symbol = wb13_primary_layer_symbol("ssc", layer_index);
            let por_symbol = wb13_primary_layer_symbol("por", layer_index);
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

    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(wb11_et_demand),
    );
    if runtime_surface_symbol_value(runtime_surface, "wb17_residue_interception").is_none() {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb17_residue_interception"),
            BoundaryValue::scalar(0.0),
        );
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
    execution_lane: ExecutionLane,
    publication_area_m2: f64,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_day: &ClimateDayProjection,
    runtime_swe_before_m: f64,
) -> Result<DailyExecutionResult, HillslopeCliError> {
    let mut runtime_surface = runtime_surface;
    seed_wb11_runtime_surface_inputs(&mut runtime_surface, execution_lane)?;
    runtime_surface
        .state_surface
        .retain(|symbol, _| symbol.as_str() != "pl_schedule_slot_count");

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
    let mut kernel = Wb11HydrologyKernel;
    let execution_report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!("{SIMPIPE_GUARD_ID} scheduler/kernel lifecycle failed: {error}"),
        })?;

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

    let wb13_row = build_simulation_owned_wb13_row(
        &execution_report.writeback_surface,
        publication_area_m2,
        simulation_year,
        sim_day_index,
        calendar_day,
        runtime_swe_before_m,
    )?;
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
    })
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

    let winter = HillslopeWinterCouplingProvenance {
        active: snow_file_present,
        snow_file_present,
        rst,
        newsnw,
        ssd,
        runtime_swe,
    };

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
        publication_area_m2,
        row_count: rows.len(),
        sim_day_index_monotonic,
        first_row_key: wb13_row_key_provenance(first_row),
        last_row_key: wb13_row_key_provenance(last_row),
    })
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
    let nsl = scalar_to_usize(
        "nsl",
        require_runtime_surface_scalar(runtime_surface, "nsl")?,
    )?;
    if nsl == 0 {
        return Err(wb13_simout_failure(
            "nsl must be >= 1 for ProfileFCStore layer aggregation",
        ));
    }

    let mut profile_fc_store_m = 0.0_f64;
    for layer_index in 1..=nsl {
        let thetfc_symbol = format!("thetfc_{layer_index:04}");
        let dg_symbol = format!("dg_{layer_index:04}");
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

    let q_m = require_runtime_surface_scalar(runtime_surface, "Q")?;
    if q_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Q must be >= 0.0, observed {q_m}"
        )));
    }
    let transpiration_ep_m = require_runtime_surface_scalar(runtime_surface, "Ep")?;
    if transpiration_ep_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Ep must be >= 0.0, observed {transpiration_ep_m}"
        )));
    }
    let soil_evap_es_m = require_runtime_surface_scalar(runtime_surface, "Es")?;
    if soil_evap_es_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Es must be >= 0.0, observed {soil_evap_es_m}"
        )));
    }
    let residue_evap_er_m = require_runtime_surface_scalar(runtime_surface, "Er")?;
    if residue_evap_er_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Er must be >= 0.0, observed {residue_evap_er_m}"
        )));
    }
    let dp_m = require_runtime_surface_scalar(runtime_surface, "D")?;
    if dp_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "D must be >= 0.0, observed {dp_m}"
        )));
    }
    let latqcc_m = require_runtime_surface_scalar(runtime_surface, "q")?;
    if latqcc_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "q must be >= 0.0, observed {latqcc_m}"
        )));
    }
    let tile_m = require_runtime_surface_scalar(runtime_surface, "Qdd")?;
    if tile_m < 0.0 {
        return Err(wb13_simout_failure(format!(
            "Qdd must be >= 0.0, observed {tile_m}"
        )));
    }
    let qd_source_m = require_runtime_surface_scalar(runtime_surface, "Qd")?;
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
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("por_0001"),
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
            BoundarySymbol::from("thetdr_0001"),
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
