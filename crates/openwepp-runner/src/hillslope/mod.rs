use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::runtime_inputs::{
    build_hillslope_climate_runtime_request,
    build_hillslope_runtime_surface_from_climate_request_with_context,
    build_hillslope_runtime_surface_from_frost, build_hillslope_runtime_surface_from_management,
    build_hillslope_runtime_surface_from_slope, build_hillslope_runtime_surface_from_snow,
    build_hillslope_runtime_surface_from_soil,
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
use openwepp_summary_accumulator::{
    SummaryScalarSurface, Wb13DailyWaterBalanceRow, Wb13DailyWaterBalanceSurface,
};
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
    let output_file_names: Vec<String> = required_output_paths(&runfile.output_config)
        .into_iter()
        .chain(optional_output_paths(&runfile.output_config))
        .map(|path| file_name_string(&path))
        .filter(|name| !name.is_empty())
        .collect();

    let (snow, frost, wepp_ui_mode_selection) = if request.legacy_sidecar_discovery {
        let mut excluded_files = vec![
            file_name_string(&run_file_path),
            file_name_string(&soil_path),
            file_name_string(&management_path),
            file_name_string(&slope_path),
            file_name_string(&climate_path),
            "openwepp_hillslope_run_manifest.json".to_string(),
        ];
        excluded_files.extend(output_file_names.clone());

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
    let slope_surface = build_hillslope_runtime_surface_from_slope(&slope).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "slope",
            detail: error.to_string(),
        }
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
        previous_climate_symbols = climate_surface.state_surface.keys().cloned().collect();
        runtime_surface = merge_runtime_surfaces(runtime_surface, climate_surface);

        let simulation_year =
            simulation_year_from_calendar_year(day_projection.year, climate_span.first_day.year)?;
        let execution_result = execute_scheduler_kernel_lifecycle(
            runtime_surface,
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
    };
    let wb13_publication =
        build_wb13_publication_provenance(&wb13_rows, contributor_ofe_count, publication_area_m2)?;
    let pass_text = build_h5_wat_output(&wb13_rows)?;
    let loss_text = build_loss_output_json(
        &runfile.run_name,
        &soil,
        &snow,
        &frost,
        &climate_span,
        executed_day_count,
    )?;

    let [output_pass, output_loss] = required_output_paths(&runfile.output_config);
    let optional_outputs = optional_output_paths(&runfile.output_config);

    for path in std::iter::once(&output_pass)
        .chain(std::iter::once(&output_loss))
        .chain(optional_outputs.iter())
    {
        ensure_output_parent_directory(path)?;
    }

    fs::write(&output_pass, pass_text).map_err(|source| HillslopeCliError::OutputWrite {
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
        let file_name_lower = file_name.to_ascii_lowercase();
        if path_has_extension_case_insensitive(&path, "hbp")
            || file_name_lower.ends_with(".pass.dat")
        {
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
) -> Result<(), HillslopeCliError> {
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
    if tmax < tmin {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!("{SIMPIPE_GUARD_ID} tmax ({tmax}) must be >= tmin ({tmin})"),
        });
    }
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
    let hyetograph_point_symbol =
        if runtime_surface_symbol_value(runtime_surface, "ninten").is_some() {
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

    let mut wb11_soil_water = 0.0_f64;
    let mut wb11_field_capacity = 0.0_f64;
    let mut wb11_drainable_storage = 0.0_f64;
    let mut wb11_drainage_coefficient = 0.0_f64;

    for layer_index in 1..=nsl {
        let dg_symbol = wb13_primary_layer_symbol("dg", layer_index);
        let fc_symbol = wb13_primary_layer_symbol("thetfc", layer_index);
        let wp_symbol = wb13_primary_layer_symbol("thetdr", layer_index);
        let ssc_symbol = wb13_primary_layer_symbol("ssc", layer_index);

        let dg = require_runtime_surface_scalar(runtime_surface, dg_symbol.as_str())?;
        let thetfc = require_runtime_surface_scalar(runtime_surface, fc_symbol.as_str())?;
        let thetdr = require_runtime_surface_scalar(runtime_surface, wp_symbol.as_str())?;
        let ssc = require_runtime_surface_scalar(runtime_surface, ssc_symbol.as_str())?;

        if dg <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!("{SIMPIPE_GUARD_ID} {dg_symbol} must be > 0.0, observed {dg}"),
            });
        }
        if thetfc < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!("{SIMPIPE_GUARD_ID} {fc_symbol} must be >= 0.0, observed {thetfc}"),
            });
        }
        if thetdr < 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!("{SIMPIPE_GUARD_ID} {wp_symbol} must be >= 0.0, observed {thetdr}"),
            });
        }
        if ssc <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!("{SIMPIPE_GUARD_ID} {ssc_symbol} must be > 0.0, observed {ssc}"),
            });
        }

        let fc_store = thetfc * dg;
        let wp_store = thetdr * dg;
        let ul_store = fc_store + wp_store;
        if ul_store <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} derived WB18 upper-limit store must be > 0.0 for layer {layer_index}"
                ),
            });
        }

        wb11_soil_water += fc_store;
        wb11_field_capacity += fc_store;
        wb11_drainable_storage += (fc_store - wp_store).max(0.0);
        wb11_drainage_coefficient += ssc * 86_400.0;

        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("wb18_perc_theta_{layer_index:04}")),
            BoundaryValue::scalar(fc_store),
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

    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(wb11_soil_water),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(wb11_et_demand),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
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
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(39.653_865_297_983_295),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb19_drain_enabled"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb19_drain_depth"),
        BoundaryValue::scalar(0.15),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb19_drain_spacing"),
        BoundaryValue::scalar(0.285),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb19_drain_diameter"),
        BoundaryValue::scalar(0.1),
    );
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
    if runtime_surface_symbol_value(runtime_surface, "ealpha").is_none() {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(1.0));
    }
    if runtime_surface_symbol_value(runtime_surface, "m").is_none() {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
    }
    seed_mofe03_wave2_runtime_surface_inputs(runtime_surface)?;

    Ok(())
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

fn execute_scheduler_kernel_lifecycle(
    runtime_surface: HillslopeWritebackSurface,
    publication_area_m2: f64,
    simulation_year: i32,
    sim_day_index: usize,
    calendar_day: &ClimateDayProjection,
    runtime_swe_before_m: f64,
) -> Result<DailyExecutionResult, HillslopeCliError> {
    let mut runtime_surface = runtime_surface;
    seed_wb11_runtime_surface_inputs(&mut runtime_surface)?;
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

fn build_h5_wat_output(wb13_rows: &[SimulationOwnedWb13Row]) -> Result<String, HillslopeCliError> {
    if wb13_rows.is_empty() {
        return Err(wb13_simout_failure(
            "WB13 surface emission requires at least one executed-day row",
        ));
    }
    let mut daily_surface = Wb13DailyWaterBalanceSurface::new();
    for row in wb13_rows {
        daily_surface
            .append_row(row.wb13_row.clone())
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb13_surface",
                detail: error.to_string(),
            })?;
    }

    Ok(daily_surface.render_h5_wat_dat())
}

fn build_hillslope_wat_rows(
    wb13_rows: &[SimulationOwnedWb13Row],
) -> Result<Vec<HillslopeWatRow>, HillslopeCliError> {
    wb13_rows.iter().map(build_hillslope_wat_row).collect()
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

    let tmax = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let tmin = require_runtime_surface_scalar(runtime_surface, "tmin")?;
    if tmax < tmin {
        return Err(wb13_simout_failure(format!(
            "tmax ({tmax}) must be >= tmin ({tmin}) for WB13 publication"
        )));
    }

    let nsl = scalar_to_usize(
        "nsl",
        require_runtime_surface_scalar(runtime_surface, "nsl")?,
    )?;
    if nsl == 0 {
        return Err(wb13_simout_failure(
            "nsl must be >= 1 for WB13 publication surface assembly",
        ));
    }

    let profile_depth_m = require_runtime_surface_scalar(runtime_surface, "solthk")?;
    if profile_depth_m <= 0.0 {
        return Err(wb13_simout_failure(format!(
            "solthk must be > 0.0, observed {profile_depth_m}"
        )));
    }
    let profile_depth_mm = profile_depth_m * 1_000.0;

    let mut profile_fc_store_mm = 0.0_f64;
    let mut profile_wp_store_mm = 0.0_f64;
    for layer_index in 1..=nsl {
        let dg_symbol = wb13_primary_layer_symbol("dg", layer_index);
        let dg_m = require_runtime_surface_scalar(runtime_surface, dg_symbol.as_str())?;
        if dg_m <= 0.0 {
            return Err(wb13_simout_failure(format!(
                "{dg_symbol} must be > 0.0, observed {dg_m}"
            )));
        }

        let fc_symbol = wb13_primary_layer_symbol("thetfc", layer_index);
        let thetfc = require_runtime_surface_scalar(runtime_surface, fc_symbol.as_str())?;
        if thetfc < 0.0 {
            return Err(wb13_simout_failure(format!(
                "{fc_symbol} must be >= 0.0, observed {thetfc}"
            )));
        }

        let wp_symbol = wb13_primary_layer_symbol("thetdr", layer_index);
        let thetdr = require_runtime_surface_scalar(runtime_surface, wp_symbol.as_str())?;
        if thetdr < 0.0 {
            return Err(wb13_simout_failure(format!(
                "{wp_symbol} must be >= 0.0, observed {thetdr}"
            )));
        }

        profile_fc_store_mm += thetfc * dg_m * 1_000.0;
        profile_wp_store_mm += thetdr * dg_m * 1_000.0;
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
    let q = q_m * 1_000.0;
    let ep = transpiration_ep_m * 1_000.0;
    let es = soil_evap_es_m * 1_000.0;
    let er = residue_evap_er_m * 1_000.0;
    let dp = dp_m * 1_000.0;
    let latqcc = latqcc_m * 1_000.0;
    let area = publication_area_m2;
    let soil_water_total = total_soil + frozwt;
    let profile_porosity_cap = profile_fc_store_mm.max(profile_wp_store_mm) + 20.0;

    let row_surface = SummaryScalarSurface::from_pairs([
        ("P", precipitation_mm),
        ("RM", rm),
        ("Q", q),
        ("Ep", ep),
        ("Es", es),
        ("Er", er),
        ("Dp", dp),
        ("UpStrmQ", 0.0),
        ("SubRIn", 0.0),
        ("latqcc", latqcc),
        ("Total-Soil", total_soil),
        ("frozwt", frozwt),
        ("Snow-Water", snow_water),
        ("QOFE", q),
        ("Tile", 0.0),
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
    use openwepp_input_contract::parsers::slope::{
        DatverSource, DistanceMode, SlopeOfe, SlopePoint, SlopeProfile,
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
        let pass_text = fs::read_to_string(&report.output_pass).unwrap_or_else(|error| {
            panic!(
                "pass output should be readable at {}: {error}",
                report.output_pass.display()
            )
        });

        let numeric_rows: Vec<&str> = pass_text
            .lines()
            .filter(|line| {
                line.split_whitespace()
                    .next()
                    .is_some_and(|token| token.parse::<f64>().is_ok())
            })
            .collect();

        assert_eq!(
            numeric_rows.len(),
            2,
            "fixture climate has two days; WB13 output must preserve full run span"
        );

        let first_tokens: Vec<&str> = numeric_rows[0].split_whitespace().collect();
        let second_tokens: Vec<&str> = numeric_rows[1].split_whitespace().collect();
        assert_eq!(
            first_tokens
                .get(2)
                .and_then(|value| value.parse::<i32>().ok()),
            Some(1),
            "WB13 Y key must use simulation-year semantics"
        );
        assert_eq!(
            second_tokens
                .get(2)
                .and_then(|value| value.parse::<i32>().ok()),
            Some(1),
            "WB13 Y key must remain simulation-year in same calendar year"
        );

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
