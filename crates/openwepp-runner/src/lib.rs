#![forbid(unsafe_code)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::sync::{LazyLock, Mutex, MutexGuard};

use openwepp_hillslope_orchestrator::runtime_inputs::{
    build_hillslope_runtime_surface_from_climate, build_hillslope_runtime_surface_from_frost,
    build_hillslope_runtime_surface_from_management, build_hillslope_runtime_surface_from_slope,
    build_hillslope_runtime_surface_from_snow, build_hillslope_runtime_surface_from_soil,
};
use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseScheduler, HillslopeWritebackSurface, SchedulerOutcomeClass,
};
use openwepp_hillslope_output::contracts::{HillslopeOutputConfig, validate_output_contract};
use openwepp_hillslope_output::hillslope_wat::{
    HillslopeWatRow, InterchangeVersion, write_hillslope_wat_parquet,
};
use openwepp_hillslope_output::manifest::{OutputChecksumEntry, assemble_output_checksums};
use openwepp_hillslope_output::writers::{optional_output_paths, required_output_paths};
use openwepp_input_contract::parsers::climate::{
    ClimateDailyRecord, CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_file,
};
use openwepp_input_contract::parsers::frost::{
    ParseMode as FrostParseMode, parse_frost_from_path, parse_frost_from_str,
};
use openwepp_input_contract::parsers::management::{
    ParseMode as ManagementParseMode, parse_management_from_path,
};
use openwepp_input_contract::parsers::pmetpara::{
    ParseMode as PmetparaParseMode, PmetparaParseOptions, parse_pmetpara_file,
};
use openwepp_input_contract::parsers::slope::{SlopeParserOptions, parse_slope_file};
use openwepp_input_contract::parsers::snow::{
    ParseMode as SnowParseMode, SnowParseOptions, SnowParseOutput, parse_snow_file,
    parse_snow_from_str,
};
use openwepp_input_contract::parsers::soil::{
    ParserMode as SoilParserMode, SoilParserOptions, parse_soil,
};
use openwepp_input_contract::parsers::wepp_ui::{
    WeppUiParserMode, WeppUiParserOptions, parse_wepp_ui_from_path,
};
use openwepp_kernel_contract::{
    HillslopeKernel, HillslopeKernelRequest, KernelRunResponse, KernelWritebackPayload,
};
use openwepp_legacy_bridge::policy::CompatibilityPolicy;
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterError, SidecarAdapterRequest, SidecarBinding, SidecarContract, SidecarDiscovery,
    SidecarId, SidecarRequirement, adapt_sidecar_bindings,
};
use openwepp_summary_accumulator::{
    SummaryScalarSurface, Wb13DailyWaterBalanceRow, Wb13DailyWaterBalanceSurface,
};
use openwepp_topology::{TopologyGraph, validate_pre_execution_topology};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

pub const BINARY_RELEASE_SCHEMA_ID: &str = "openwepp-binary-release-metadata-v1";
pub const HILLSLOPE_RUN_MANIFEST_SCHEMA_ID: &str = "openwepp-hillslope-run-manifest-v1";
pub const HILLSLOPE_RUNFILE_SCHEMA_ID: &str = "openwepp-hillslope-runfile-v1";
pub const REQUIRED_RUN_OUTPUT_PASS: &str = "outputs.pass (.hbp)";
pub const REQUIRED_RUN_OUTPUT_LOSS: &str = "outputs.loss (.json)";
pub const SIMPIPE_GUARD_ID: &str = "HS-SIMPIPE-E-001";
pub const DAILY_EXECUTION_LANE: &str = "daily";
pub const SCHEDULER_KERNEL_PUBLICATION_SOURCE: &str = "scheduler-kernel";

static RELEASE_SIDECAR_IO_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidecarPolicy {
    Strict,
    Compat,
}

impl SidecarPolicy {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Compat => "compat",
        }
    }

    #[must_use]
    pub const fn as_legacy_bridge_policy(self) -> CompatibilityPolicy {
        match self {
            Self::Strict => CompatibilityPolicy::Strict,
            Self::Compat => CompatibilityPolicy::Compat,
        }
    }

    #[must_use]
    pub const fn as_soil_parser_mode(self) -> SoilParserMode {
        match self {
            Self::Strict => SoilParserMode::Strict,
            Self::Compat => SoilParserMode::Compatibility,
        }
    }

    #[must_use]
    pub const fn as_slope_parser_options(self) -> SlopeParserOptions {
        match self {
            Self::Strict => SlopeParserOptions::strict(),
            Self::Compat => SlopeParserOptions::compatibility(),
        }
    }

    #[must_use]
    pub const fn as_management_parser_mode(self) -> ManagementParseMode {
        match self {
            Self::Strict => ManagementParseMode::Strict,
            Self::Compat => ManagementParseMode::Compatibility,
        }
    }

    #[must_use]
    pub fn as_climate_parser_mode(self) -> ClimateParserMode {
        match self {
            Self::Strict => ClimateParserMode::Strict,
            Self::Compat => ClimateParserMode::Compatibility(CompatibilityOptions::default()),
        }
    }

    #[must_use]
    pub const fn as_snow_parse_options(self) -> SnowParseOptions {
        match self {
            Self::Strict => SnowParseOptions {
                mode: SnowParseMode::Strict,
            },
            Self::Compat => SnowParseOptions {
                mode: SnowParseMode::Compatibility,
            },
        }
    }

    #[must_use]
    pub const fn as_frost_parse_mode(self) -> FrostParseMode {
        match self {
            Self::Strict => FrostParseMode::Strict,
            Self::Compat => FrostParseMode::Compatibility,
        }
    }

    #[must_use]
    pub const fn as_wepp_ui_parse_mode(self) -> WeppUiParserMode {
        match self {
            Self::Strict => WeppUiParserMode::Strict,
            Self::Compat => WeppUiParserMode::Compatibility,
        }
    }

    #[must_use]
    pub const fn as_pmetpara_parse_mode(self) -> PmetparaParseMode {
        match self {
            Self::Strict => PmetparaParseMode::Strict,
            Self::Compat => PmetparaParseMode::Compatibility,
        }
    }
}

impl std::str::FromStr for SidecarPolicy {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "strict" => Ok(Self::Strict),
            "compat" => Ok(Self::Compat),
            _ => Err(format!(
                "unsupported sidecar policy '{value}' (expected strict|compat)"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryRole {
    Watershed,
    Hillslope,
    Replay,
}

impl BinaryRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Watershed => "watershed",
            Self::Hillslope => "hillslope",
            Self::Replay => "replay",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "watershed" => Some(Self::Watershed),
            "hillslope" => Some(Self::Hillslope),
            "replay" => Some(Self::Replay),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ReleaseMetadataError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    JsonSerialize {
        source: serde_json::Error,
    },
    JsonParse {
        path: PathBuf,
        source: serde_json::Error,
    },
    MissingField {
        field: &'static str,
    },
    InvalidField {
        field: &'static str,
        detail: String,
    },
}

impl ReleaseMetadataError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Io { .. } => "RELMD-E-001",
            Self::JsonSerialize { .. } => "RELMD-E-002",
            Self::JsonParse { .. } => "RELMD-E-003",
            Self::MissingField { .. } => "RELMD-E-004",
            Self::InvalidField { .. } => "RELMD-E-005",
        }
    }
}

impl fmt::Display for ReleaseMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(
                f,
                "{} io error at {}: {source}",
                self.code(),
                path.display()
            ),
            Self::JsonSerialize { source } => {
                write!(f, "{} failed to serialize JSON: {source}", self.code())
            }
            Self::JsonParse { path, source } => write!(
                f,
                "{} failed to parse JSON at {}: {source}",
                self.code(),
                path.display()
            ),
            Self::MissingField { field } => write!(f, "{} missing field {field}", self.code()),
            Self::InvalidField { field, detail } => {
                write!(f, "{} invalid field {field}: {detail}", self.code())
            }
        }
    }
}

impl Error for ReleaseMetadataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::JsonSerialize { source } | Self::JsonParse { source, .. } => Some(source),
            Self::MissingField { .. } | Self::InvalidField { .. } => None,
        }
    }
}

#[derive(Debug)]
pub enum ReleaseLintError {
    DirectoryRead {
        path: PathBuf,
        source: io::Error,
    },
    InvalidBinaryName {
        binary_name: String,
    },
    MissingSidecar {
        sidecar_path: PathBuf,
    },
    SidecarInvalid {
        sidecar_path: PathBuf,
        source: ReleaseMetadataError,
    },
    SidecarRoleMismatch {
        sidecar_path: PathBuf,
        expected: BinaryRole,
        observed: String,
    },
    SidecarBinaryNameMismatch {
        sidecar_path: PathBuf,
        expected: String,
        observed: String,
    },
    HbpPairMismatch {
        watershed: bool,
        hillslope: bool,
    },
    NoReleaseCandidates {
        release_dir: PathBuf,
    },
}

impl ReleaseLintError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidBinaryName { .. } | Self::NoReleaseCandidates { .. } => "RUNNER-E-006",
            Self::DirectoryRead { .. }
            | Self::MissingSidecar { .. }
            | Self::SidecarInvalid { .. }
            | Self::SidecarRoleMismatch { .. }
            | Self::SidecarBinaryNameMismatch { .. }
            | Self::HbpPairMismatch { .. } => "RUNNER-E-005",
        }
    }
}

impl fmt::Display for ReleaseLintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectoryRead { path, source } => write!(
                f,
                "{} failed to read release directory {}: {source}",
                self.code(),
                path.display()
            ),
            Self::InvalidBinaryName { binary_name } => write!(
                f,
                "{} binary name violates release contract: {binary_name}",
                self.code()
            ),
            Self::MissingSidecar { sidecar_path } => write!(
                f,
                "{} missing sidecar {}",
                self.code(),
                sidecar_path.display()
            ),
            Self::SidecarInvalid {
                sidecar_path,
                source,
            } => write!(
                f,
                "{} invalid sidecar {}: {source}",
                self.code(),
                sidecar_path.display()
            ),
            Self::SidecarRoleMismatch {
                sidecar_path,
                expected,
                observed,
            } => write!(
                f,
                "{} sidecar role mismatch for {}: expected {} observed {}",
                self.code(),
                sidecar_path.display(),
                expected.as_str(),
                observed
            ),
            Self::SidecarBinaryNameMismatch {
                sidecar_path,
                expected,
                observed,
            } => write!(
                f,
                "{} sidecar binary_name mismatch for {}: expected {} observed {}",
                self.code(),
                sidecar_path.display(),
                expected,
                observed
            ),
            Self::HbpPairMismatch {
                watershed,
                hillslope,
            } => write!(
                f,
                "{} watershed/hillslope hbp_supported mismatch: watershed={} hillslope={}",
                self.code(),
                watershed,
                hillslope
            ),
            Self::NoReleaseCandidates { release_dir } => write!(
                f,
                "{} no openwepp_* release candidates in {}",
                self.code(),
                release_dir.display()
            ),
        }
    }
}

impl Error for ReleaseLintError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DirectoryRead { source, .. } => Some(source),
            Self::SidecarInvalid { source, .. } => Some(source),
            Self::InvalidBinaryName { .. }
            | Self::MissingSidecar { .. }
            | Self::SidecarRoleMismatch { .. }
            | Self::SidecarBinaryNameMismatch { .. }
            | Self::HbpPairMismatch { .. }
            | Self::NoReleaseCandidates { .. } => None,
        }
    }
}

#[derive(Debug)]
pub enum RunnerError {
    MissingArgument { argument: String },
    HillslopeBinaryMissing { path: PathBuf },
    LaunchFailure { source: io::Error },
    NonZeroExit { status: ExitStatus },
    ReleaseLint { source: ReleaseLintError },
}

impl RunnerError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingArgument { .. } => "RUNNER-E-001",
            Self::HillslopeBinaryMissing { .. } => "RUNNER-E-002",
            Self::LaunchFailure { .. } => "RUNNER-E-003",
            Self::NonZeroExit { .. } => "RUNNER-E-004",
            Self::ReleaseLint { source } => source.code(),
        }
    }
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArgument { argument } => {
                write!(f, "{} missing required argument {argument}", self.code())
            }
            Self::HillslopeBinaryMissing { path } => write!(
                f,
                "{} missing hillslope binary {}",
                self.code(),
                path.display()
            ),
            Self::LaunchFailure { source } => {
                write!(
                    f,
                    "{} failed to launch child process: {source}",
                    self.code()
                )
            }
            Self::NonZeroExit { status } => {
                write!(f, "{} child process exited non-zero: {status}", self.code())
            }
            Self::ReleaseLint { source } => write!(f, "{source}"),
        }
    }
}

impl Error for RunnerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LaunchFailure { source } => Some(source),
            Self::ReleaseLint { source } => Some(source),
            Self::MissingArgument { .. }
            | Self::HillslopeBinaryMissing { .. }
            | Self::NonZeroExit { .. } => None,
        }
    }
}

#[derive(Debug)]
pub enum HillslopeCliError {
    MissingArgument {
        argument: &'static str,
    },
    RunDirectoryMissing {
        path: PathBuf,
    },
    RunFileMissing {
        path: PathBuf,
    },
    OutputDirectoryCreate {
        path: PathBuf,
        source: io::Error,
    },
    CoreInputMissing {
        extension: &'static str,
        run_dir: PathBuf,
    },
    CoreInputAmbiguous {
        extension: &'static str,
        run_dir: PathBuf,
        count: usize,
    },
    SidecarContractInvalid {
        detail: String,
    },
    SidecarAdapter {
        source: SidecarAdapterError,
    },
    SidecarBindingMissing {
        sidecar_id: &'static str,
    },
    ParseFailure {
        surface: &'static str,
        detail: String,
    },
    RuntimeSurfaceFailure {
        surface: &'static str,
        detail: String,
    },
    OutputWrite {
        path: PathBuf,
        source: io::Error,
    },
    MissingRequiredOutput {
        output_name: &'static str,
    },
    ReleaseMetadata {
        source: ReleaseMetadataError,
    },
    ManifestSerialize {
        source: serde_json::Error,
    },
    ManifestWrite {
        path: PathBuf,
        source: io::Error,
    },
    Io {
        path: PathBuf,
        source: io::Error,
    },
    TimeFormat {
        detail: String,
    },
}

impl HillslopeCliError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingArgument { .. } => "CLIHILL-E-001",
            Self::RunDirectoryMissing { .. } => "CLIHILL-E-002",
            Self::RunFileMissing { .. } => "CLIHILL-E-003",
            Self::OutputDirectoryCreate { .. } => "CLIHILL-E-004",
            Self::CoreInputMissing { .. } => "CLIHILL-E-005",
            Self::CoreInputAmbiguous { .. } => "CLIHILL-E-006",
            Self::SidecarContractInvalid { .. } => "CLIHILL-E-007",
            Self::SidecarAdapter { .. } => "CLIHILL-E-008",
            Self::SidecarBindingMissing { .. } => "CLIHILL-E-009",
            Self::ParseFailure { .. } => "CLIHILL-E-010",
            Self::RuntimeSurfaceFailure { .. } => "CLIHILL-E-011",
            Self::OutputWrite { .. } => "CLIHILL-E-012",
            Self::MissingRequiredOutput { .. } => "CLIHILL-E-013",
            Self::ReleaseMetadata { .. } => "CLIHILL-E-014",
            Self::ManifestSerialize { .. } => "CLIHILL-E-015",
            Self::ManifestWrite { .. } => "CLIHILL-E-016",
            Self::Io { .. } => "CLIHILL-E-017",
            Self::TimeFormat { .. } => "CLIHILL-E-018",
        }
    }
}

impl fmt::Display for HillslopeCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArgument { argument } => {
                write!(f, "{} missing required argument {argument}", self.code())
            }
            Self::RunDirectoryMissing { path } => {
                write!(
                    f,
                    "{} missing run directory {}",
                    self.code(),
                    path.display()
                )
            }
            Self::RunFileMissing { path } => {
                write!(f, "{} missing run file {}", self.code(), path.display())
            }
            Self::OutputDirectoryCreate { path, source } => write!(
                f,
                "{} failed to create output directory {}: {source}",
                self.code(),
                path.display()
            ),
            Self::CoreInputMissing { extension, run_dir } => write!(
                f,
                "{} missing required core input extension .{} in {}",
                self.code(),
                extension,
                run_dir.display()
            ),
            Self::CoreInputAmbiguous {
                extension,
                run_dir,
                count,
            } => write!(
                f,
                "{} ambiguous core input extension .{} in {} (count={})",
                self.code(),
                extension,
                run_dir.display(),
                count
            ),
            Self::SidecarContractInvalid { detail } => {
                write!(f, "{} invalid sidecar contract: {detail}", self.code())
            }
            Self::SidecarAdapter { source } => write!(
                f,
                "{} sidecar adapter failure: {} ({source})",
                self.code(),
                source.code()
            ),
            Self::SidecarBindingMissing { sidecar_id } => write!(
                f,
                "{} missing required sidecar binding for {}",
                self.code(),
                sidecar_id
            ),
            Self::ParseFailure { surface, detail } => {
                write!(f, "{} parse failure for {surface}: {detail}", self.code())
            }
            Self::RuntimeSurfaceFailure { surface, detail } => write!(
                f,
                "{} runtime surface failure for {surface}: {detail}",
                self.code()
            ),
            Self::OutputWrite { path, source } => write!(
                f,
                "{} failed writing output {}: {source}",
                self.code(),
                path.display()
            ),
            Self::MissingRequiredOutput { output_name } => {
                write!(f, "{} missing required output {}", self.code(), output_name)
            }
            Self::ReleaseMetadata { source } => {
                write!(f, "{} release metadata failure: {source}", self.code())
            }
            Self::ManifestSerialize { source } => {
                write!(f, "{} failed to serialize manifest: {source}", self.code())
            }
            Self::ManifestWrite { path, source } => write!(
                f,
                "{} failed writing manifest {}: {source}",
                self.code(),
                path.display()
            ),
            Self::Io { path, source } => write!(
                f,
                "{} io failure at {}: {source}",
                self.code(),
                path.display()
            ),
            Self::TimeFormat { detail } => {
                write!(f, "{} invalid UTC format: {detail}", self.code())
            }
        }
    }
}

impl Error for HillslopeCliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::OutputDirectoryCreate { source, .. }
            | Self::OutputWrite { source, .. }
            | Self::ManifestWrite { source, .. }
            | Self::Io { source, .. } => Some(source),
            Self::SidecarAdapter { source } => Some(source),
            Self::ReleaseMetadata { source } => Some(source),
            Self::ManifestSerialize { source } => Some(source),
            Self::MissingArgument { .. }
            | Self::RunDirectoryMissing { .. }
            | Self::RunFileMissing { .. }
            | Self::CoreInputMissing { .. }
            | Self::CoreInputAmbiguous { .. }
            | Self::SidecarContractInvalid { .. }
            | Self::SidecarBindingMissing { .. }
            | Self::ParseFailure { .. }
            | Self::RuntimeSurfaceFailure { .. }
            | Self::MissingRequiredOutput { .. }
            | Self::TimeFormat { .. } => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunnerLaunchRequest {
    pub hillslope_binary: PathBuf,
    pub run_dir: PathBuf,
    pub run_file: PathBuf,
    pub output_dir: PathBuf,
    pub sidecar_policy: SidecarPolicy,
    pub legacy_sidecar_discovery: bool,
    pub manifest_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct HillslopeRunRequest {
    pub run_dir: PathBuf,
    pub run_file: PathBuf,
    pub output_dir: PathBuf,
    pub sidecar_policy: SidecarPolicy,
    pub legacy_sidecar_discovery: bool,
    pub manifest_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct HillslopeRunReport {
    pub output_pass: PathBuf,
    pub output_loss: PathBuf,
    pub optional_outputs: Vec<PathBuf>,
    pub manifest_path: PathBuf,
    pub sidecar_warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ReleaseLintReport {
    pub checked_binaries: Vec<PathBuf>,
}

#[derive(Debug, Serialize)]
struct BinaryReleaseFeatures {
    hbp_supported: bool,
    hbp_schema_major: u32,
    hbp_schema_minor: u32,
    hbp_pass_family: String,
    legacy_ascii_pass_family: String,
    mode2_master_pass_prompt_required: bool,
}

#[derive(Debug, Serialize)]
struct BinaryReleaseValidation {
    schema_valid: bool,
    release_lint_level: String,
    validated_utc: String,
}

#[derive(Debug, Serialize)]
struct BinaryReleaseMetadataDocument {
    schema: String,
    binary_name: String,
    binary_role: String,
    release_tag: String,
    source_repo: String,
    source_commit: String,
    built_utc: String,
    sha256: String,
    features: BinaryReleaseFeatures,
    validation: BinaryReleaseValidation,
}

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
    execution_provenance: HillslopeExecutionProvenance,
}

#[derive(Debug, Serialize)]
struct HillslopeExecutionProvenance {
    scheduler_kernel_executed: bool,
    publication_source: String,
    simpipe_guard_id: String,
    selected_lane: String,
    scheduler_outcome_class: String,
    scheduler_status_message_id: String,
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

#[must_use]
pub fn build_hillslope_argv(request: &RunnerLaunchRequest) -> Vec<String> {
    let mut argv = vec![
        "--run-dir".to_string(),
        request.run_dir.display().to_string(),
        "--run-file".to_string(),
        request.run_file.display().to_string(),
        "--output-dir".to_string(),
        request.output_dir.display().to_string(),
        "--policy".to_string(),
        request.sidecar_policy.as_str().to_string(),
    ];

    if request.legacy_sidecar_discovery {
        argv.push("--legacy-sidecar-discovery".to_string());
    }

    if let Some(path) = &request.manifest_path {
        argv.push("--manifest-path".to_string());
        argv.push(path.display().to_string());
    }

    argv
}

pub fn launch_hillslope(request: &RunnerLaunchRequest) -> Result<(), RunnerError> {
    if !request.hillslope_binary.is_file() {
        return Err(RunnerError::HillslopeBinaryMissing {
            path: request.hillslope_binary.clone(),
        });
    }

    let argv = build_hillslope_argv(request);
    let status = Command::new(&request.hillslope_binary)
        .args(&argv)
        .status()
        .map_err(|source| RunnerError::LaunchFailure { source })?;

    if !status.success() {
        return Err(RunnerError::NonZeroExit { status });
    }

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn lint_release_directory(release_dir: &Path) -> Result<ReleaseLintReport, ReleaseLintError> {
    let entries = fs::read_dir(release_dir).map_err(|source| ReleaseLintError::DirectoryRead {
        path: release_dir.to_path_buf(),
        source,
    })?;

    let mut candidate_binaries = Vec::new();
    for entry_result in entries {
        let entry = entry_result.map_err(|source| ReleaseLintError::DirectoryRead {
            path: release_dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(OsStr::to_str) else {
            continue;
        };

        if !file_name.starts_with("openwepp_") || path_has_extension_case_insensitive(&path, "json")
        {
            continue;
        }

        candidate_binaries.push(path);
    }

    if candidate_binaries.is_empty() {
        return Err(ReleaseLintError::NoReleaseCandidates {
            release_dir: release_dir.to_path_buf(),
        });
    }

    let mut watershed_hbp_supported: Option<bool> = None;
    let mut hillslope_hbp_supported: Option<bool> = None;

    for binary_path in &candidate_binaries {
        let binary_name = file_name_string(binary_path);
        let expected_role = classify_release_binary_role(binary_name.as_str())?;
        if !release_binary_name_is_valid(binary_name.as_str(), expected_role) {
            return Err(ReleaseLintError::InvalidBinaryName { binary_name });
        }

        let sidecar_path = sidecar_path_for_binary(binary_path);
        if !sidecar_path.is_file() {
            return Err(ReleaseLintError::MissingSidecar { sidecar_path });
        }

        let metadata = validate_release_sidecar(&sidecar_path).map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path: sidecar_path.clone(),
                source,
            }
        })?;

        let observed_role = required_str(&metadata, "binary_role").map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path: sidecar_path.clone(),
                source,
            }
        })?;
        if BinaryRole::parse(observed_role).is_none() {
            return Err(ReleaseLintError::SidecarRoleMismatch {
                sidecar_path,
                expected: expected_role,
                observed: observed_role.to_string(),
            });
        }

        if BinaryRole::parse(observed_role) != Some(expected_role) {
            return Err(ReleaseLintError::SidecarRoleMismatch {
                sidecar_path,
                expected: expected_role,
                observed: observed_role.to_string(),
            });
        }

        let observed_binary_name = required_str(&metadata, "binary_name").map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path: sidecar_path.clone(),
                source,
            }
        })?;
        if observed_binary_name != binary_name {
            return Err(ReleaseLintError::SidecarBinaryNameMismatch {
                sidecar_path,
                expected: binary_name,
                observed: observed_binary_name.to_string(),
            });
        }

        let features = required_object(&metadata, "features").map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path: sidecar_path.clone(),
                source,
            }
        })?;
        let hbp_supported = required_bool(features, "hbp_supported").map_err(|source| {
            ReleaseLintError::SidecarInvalid {
                sidecar_path,
                source,
            }
        })?;

        match expected_role {
            BinaryRole::Watershed => watershed_hbp_supported = Some(hbp_supported),
            BinaryRole::Hillslope => hillslope_hbp_supported = Some(hbp_supported),
            BinaryRole::Replay => {}
        }
    }

    if let (Some(watershed), Some(hillslope)) = (watershed_hbp_supported, hillslope_hbp_supported)
        && watershed != hillslope
    {
        return Err(ReleaseLintError::HbpPairMismatch {
            watershed,
            hillslope,
        });
    }

    Ok(ReleaseLintReport {
        checked_binaries: candidate_binaries,
    })
}

pub fn write_release_sidecar_for_binary(
    binary_path: &Path,
    role: BinaryRole,
) -> Result<PathBuf, ReleaseMetadataError> {
    let _io_guard = lock_release_sidecar_io();
    let metadata = build_release_metadata_document(binary_path, role)?;
    let sidecar_path = sidecar_path_for_binary(binary_path);
    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|source| ReleaseMetadataError::JsonSerialize { source })?;
    fs::write(&sidecar_path, json).map_err(|source| ReleaseMetadataError::Io {
        path: sidecar_path.clone(),
        source,
    })?;

    validate_release_sidecar_unlocked(&sidecar_path)?;
    Ok(sidecar_path)
}

pub fn validate_release_sidecar(sidecar_path: &Path) -> Result<Value, ReleaseMetadataError> {
    let _io_guard = lock_release_sidecar_io();
    validate_release_sidecar_unlocked(sidecar_path)
}

fn validate_release_sidecar_unlocked(sidecar_path: &Path) -> Result<Value, ReleaseMetadataError> {
    let content = fs::read_to_string(sidecar_path).map_err(|source| ReleaseMetadataError::Io {
        path: sidecar_path.to_path_buf(),
        source,
    })?;
    let json: Value =
        serde_json::from_str(&content).map_err(|source| ReleaseMetadataError::JsonParse {
            path: sidecar_path.to_path_buf(),
            source,
        })?;

    let schema = required_str(&json, "schema")?;
    if schema != BINARY_RELEASE_SCHEMA_ID {
        return Err(ReleaseMetadataError::InvalidField {
            field: "schema",
            detail: format!("expected {BINARY_RELEASE_SCHEMA_ID}, observed {schema}"),
        });
    }

    let role = required_str(&json, "binary_role")?;
    if BinaryRole::parse(role).is_none() {
        return Err(ReleaseMetadataError::InvalidField {
            field: "binary_role",
            detail: format!("unsupported role {role}"),
        });
    }

    for field in [
        "binary_name",
        "release_tag",
        "source_repo",
        "source_commit",
        "built_utc",
        "sha256",
    ] {
        let _ = required_str(&json, field)?;
    }

    let features = required_object(&json, "features")?;
    let _ = required_bool(features, "hbp_supported")?;
    let _ = required_u64(features, "hbp_schema_major")?;
    let _ = required_u64(features, "hbp_schema_minor")?;
    let _ = required_map_str(features, "hbp_pass_family")?;
    let _ = required_map_str(features, "legacy_ascii_pass_family")?;
    let _ = required_bool(features, "mode2_master_pass_prompt_required")?;

    let validation = required_object(&json, "validation")?;
    let _ = required_bool(validation, "schema_valid")?;
    let _ = required_map_str(validation, "release_lint_level")?;
    let _ = required_map_str(validation, "validated_utc")?;

    Ok(json)
}

fn lock_release_sidecar_io() -> MutexGuard<'static, ()> {
    match RELEASE_SIDECAR_IO_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
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

    let soil_raw = fs::read_to_string(&soil_path).map_err(|source| HillslopeCliError::Io {
        path: soil_path.clone(),
        source,
    })?;
    let soil_options = SoilParserOptions {
        mode: request.sidecar_policy.as_soil_parser_mode(),
        allow_legacy_aliases: request.sidecar_policy == SidecarPolicy::Compat,
        expected_topology_count: None,
        topology_scope: None,
    };
    let soil =
        parse_soil(&soil_raw, soil_options).map_err(|error| HillslopeCliError::ParseFailure {
            surface: "soil",
            detail: error.to_string(),
        })?;

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

    let (snow, frost) = if request.legacy_sidecar_discovery {
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

        let _wepp_ui = parse_wepp_ui_from_path(
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

        (snow, frost)
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

        if sidecar_overrides.wepp_ui {
            let wepp_ui_path = request.run_dir.join("wepp_ui.txt");
            if wepp_ui_path.is_file() {
                wepp_ui_input_path = Some(wepp_ui_path.clone());
                resolved_sidecars.insert("wepp_ui".to_string(), wepp_ui_path.display().to_string());
            } else {
                sidecar_warnings.push(
                    "CLIHILL-W-001 wepp_ui=true in .run but wepp_ui.txt missing; feature flag ignored"
                        .to_string(),
                );
            }

            let _wepp_ui = parse_wepp_ui_from_path(
                &wepp_ui_path,
                WeppUiParserOptions {
                    mode: request.sidecar_policy.as_wepp_ui_parse_mode(),
                    requested_hourly_seepage: wepp_ui_path.is_file(),
                    soil_versions: soil_versions.clone(),
                },
            )
            .map_err(|error| HillslopeCliError::ParseFailure {
                surface: "wepp_ui",
                detail: error.to_string(),
            })?;
        }

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

        (snow, frost)
    };

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
    let climate_surface =
        build_hillslope_runtime_surface_from_climate(&climate, 0).map_err(|error| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "climate",
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

    let merged_runtime_surface = merge_runtime_surfaces(
        merge_runtime_surfaces(
            merge_runtime_surfaces(management_surface, soil_surface),
            slope_surface,
        ),
        merge_runtime_surfaces(
            climate_surface,
            merge_runtime_surfaces(snow_surface, frost_surface),
        ),
    );
    if merged_runtime_surface.state_surface.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "merged",
            detail: "merged runtime surface is empty".to_string(),
        });
    }

    let execution_provenance = execute_daily_scheduler_kernel_lifecycle(merged_runtime_surface)?;

    let pass_text = build_h5_wat_output(&climate, &soil, &snow, &frost)?;
    let loss_text = build_loss_output_json(&runfile.run_name, &climate, &soil, &snow, &frost)?;

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
        let wat_rows = build_hillslope_wat_rows(&climate, &soil, &snow, &frost)?;
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
        let payload = build_optional_output_payload(&runfile.run_name, optional_output, &climate)?;
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
        execution_provenance,
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

fn build_release_metadata_document(
    binary_path: &Path,
    role: BinaryRole,
) -> Result<BinaryReleaseMetadataDocument, ReleaseMetadataError> {
    let binary_name = file_name_string(binary_path);
    let built_utc = utc_now_rfc3339().map_err(|detail| ReleaseMetadataError::InvalidField {
        field: "built_utc",
        detail,
    })?;
    let source_repo = std::env::var("CARGO_PKG_REPOSITORY")
        .unwrap_or_else(|_| "https://github.com/rogerlew/openWEPP".to_string());

    Ok(BinaryReleaseMetadataDocument {
        schema: BINARY_RELEASE_SCHEMA_ID.to_string(),
        binary_name: binary_name.clone(),
        binary_role: role.as_str().to_string(),
        release_tag: infer_release_tag(binary_name.as_str()),
        source_repo,
        source_commit: git_source_commit_or_unknown(),
        built_utc: built_utc.clone(),
        sha256: sha256_file_hex(binary_path).map_err(|source| ReleaseMetadataError::Io {
            path: binary_path.to_path_buf(),
            source,
        })?,
        features: BinaryReleaseFeatures {
            hbp_supported: true,
            hbp_schema_major: 1,
            hbp_schema_minor: 0,
            hbp_pass_family: "H*.hbp".to_string(),
            legacy_ascii_pass_family: "H*.pass.dat".to_string(),
            mode2_master_pass_prompt_required: true,
        },
        validation: BinaryReleaseValidation {
            schema_valid: true,
            release_lint_level: "contract_v1".to_string(),
            validated_utc: built_utc,
        },
    })
}

fn required_object<'a>(
    json: &'a Value,
    field: &'static str,
) -> Result<&'a serde_json::Map<String, Value>, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_object)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn required_str<'a>(json: &'a Value, field: &'static str) -> Result<&'a str, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_str)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn required_map_str<'a>(
    json: &'a serde_json::Map<String, Value>,
    field: &'static str,
) -> Result<&'a str, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_str)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn required_bool(
    json: &serde_json::Map<String, Value>,
    field: &'static str,
) -> Result<bool, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_bool)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn required_u64(
    json: &serde_json::Map<String, Value>,
    field: &'static str,
) -> Result<u64, ReleaseMetadataError> {
    json.get(field)
        .and_then(Value::as_u64)
        .ok_or(ReleaseMetadataError::MissingField { field })
}

fn classify_release_binary_role(binary_name: &str) -> Result<BinaryRole, ReleaseLintError> {
    if binary_name.ends_with("_hill") {
        return Ok(BinaryRole::Hillslope);
    }
    if binary_name.ends_with("_replay") {
        return Ok(BinaryRole::Replay);
    }
    if binary_name.starts_with("openwepp_") {
        return Ok(BinaryRole::Watershed);
    }

    Err(ReleaseLintError::InvalidBinaryName {
        binary_name: binary_name.to_string(),
    })
}

fn release_binary_name_is_valid(binary_name: &str, role: BinaryRole) -> bool {
    if !binary_name.starts_with("openwepp_") {
        return false;
    }

    let stem = &binary_name["openwepp_".len()..];
    if stem.len() < 6 {
        return false;
    }

    let (date_part, suffix_part) = stem.split_at(6);
    if !date_part.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }

    if !suffix_part
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
    {
        return false;
    }

    match role {
        BinaryRole::Watershed => {
            !binary_name.ends_with("_hill") && !binary_name.ends_with("_replay")
        }
        BinaryRole::Hillslope => binary_name.ends_with("_hill"),
        BinaryRole::Replay => binary_name.ends_with("_replay"),
    }
}

fn infer_release_tag(binary_name: &str) -> String {
    if binary_name.starts_with("openwepp_") {
        binary_name.to_string()
    } else {
        "openwepp_dev".to_string()
    }
}

fn sidecar_path_for_binary(binary_path: &Path) -> PathBuf {
    PathBuf::from(format!("{}.json", binary_path.display()))
}

fn utc_now_rfc3339() -> Result<String, String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| error.to_string())
}

fn git_source_commit_or_unknown() -> String {
    if let Ok(value) = std::env::var("OPENWEPP_SOURCE_COMMIT") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    let output = Command::new("git").arg("rev-parse").arg("HEAD").output();

    if let Ok(out) = output
        && out.status.success()
        && let Ok(text) = String::from_utf8(out.stdout)
    {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    "unknown".to_string()
}

fn sha256_file_hex(path: &Path) -> Result<String, io::Error> {
    let bytes = fs::read(path)?;
    Ok(sha256_bytes_hex(&bytes))
}

fn sha256_bytes_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = fmt::Write::write_fmt(&mut hex, format_args!("{byte:02x}"));
    }
    hex
}

fn path_has_extension_case_insensitive(path: &Path, extension: &str) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
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

fn execute_daily_scheduler_kernel_lifecycle(
    runtime_surface: HillslopeWritebackSurface,
) -> Result<HillslopeExecutionProvenance, HillslopeCliError> {
    let mut runtime_surface = runtime_surface;
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
    let mut kernel = RunnerDailyPhaseKernel;
    let execution_report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!("{SIMPIPE_GUARD_ID} scheduler/kernel lifecycle failed: {error}"),
        })?;

    if !execution_report.scheduler_report.is_success() {
        let scheduler_status = &execution_report.scheduler_report.scheduler_status;
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!(
                "{SIMPIPE_GUARD_ID} scheduler lifecycle did not complete successfully (outcome_class={}, status_class={:?}, boundary_class={}, message_id={})",
                scheduler_outcome_class_as_str(execution_report.scheduler_report.outcome_class),
                scheduler_status.classification(),
                scheduler_status.boundary_class().as_str(),
                scheduler_status.message_id()
            ),
        });
    }

    Ok(HillslopeExecutionProvenance {
        scheduler_kernel_executed: true,
        publication_source: SCHEDULER_KERNEL_PUBLICATION_SOURCE.to_string(),
        simpipe_guard_id: SIMPIPE_GUARD_ID.to_string(),
        selected_lane: DAILY_EXECUTION_LANE.to_string(),
        scheduler_outcome_class: scheduler_outcome_class_as_str(
            execution_report.scheduler_report.outcome_class,
        )
        .to_string(),
        scheduler_status_message_id: execution_report
            .scheduler_report
            .scheduler_status
            .message_id()
            .to_string(),
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

#[derive(Debug, Default)]
struct RunnerDailyPhaseKernel;

impl HillslopeKernel for RunnerDailyPhaseKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        let phase =
            hillslope_phase_from_name(request.phase_name).unwrap_or(HillslopePhase::Normalization);
        let status = HillslopePhaseScheduler::nominal_phase_status(phase)
            .expect("nominal phase status constants are non-empty and valid");
        KernelRunResponse::new(status, KernelWritebackPayload::empty())
    }
}

fn hillslope_phase_from_name(phase_name: &str) -> Option<HillslopePhase> {
    match phase_name {
        "normalization" => Some(HillslopePhase::Normalization),
        "storage_bounds" => Some(HillslopePhase::StorageBounds),
        "decomposition_transition" => Some(HillslopePhase::DecompositionTransition),
        "residue_partition_transition" => Some(HillslopePhase::ResiduePartitionTransition),
        "annual_growth_transition" => Some(HillslopePhase::AnnualGrowthTransition),
        "perennial_growth_transition" => Some(HillslopePhase::PerennialGrowthTransition),
        "evapotranspiration" => Some(HillslopePhase::Evapotranspiration),
        "percolation_deep_seepage" => Some(HillslopePhase::PercolationDeepSeepage),
        "lateral_transfer" => Some(HillslopePhase::LateralTransfer),
        "drainage" => Some(HillslopePhase::Drainage),
        "runoff_reconciliation" => Some(HillslopePhase::RunoffReconciliation),
        "storage_reconciliation" => Some(HillslopePhase::StorageReconciliation),
        "closure_diagnostics" => Some(HillslopePhase::ClosureDiagnostics),
        _ => None,
    }
}

fn build_h5_wat_output(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    snow: &openwepp_input_contract::parsers::snow::SnowParseOutput,
    frost: &openwepp_input_contract::parsers::frost::FrostParseOutput,
) -> Result<String, HillslopeCliError> {
    let projection = build_first_day_wat_projection(climate, soil, snow, frost)?;

    let row_surface = SummaryScalarSurface::from_pairs([
        ("P", projection.precipitation_mm),
        ("RM", projection.rm),
        ("Q", projection.q),
        ("Ep", projection.ep),
        ("Es", projection.es),
        ("Er", projection.er),
        ("Dp", projection.dp),
        ("UpStrmQ", projection.up_strm_q),
        ("SubRIn", projection.sub_r_in),
        ("latqcc", projection.latqcc),
        ("Total-Soil", projection.total_soil),
        ("frozwt", projection.frozwt),
        ("Snow-Water", projection.snow_water),
        ("QOFE", projection.qofe),
        ("Tile", projection.tile),
        ("Irr", projection.irr),
        ("Area", projection.area),
        ("SoilWaterTotal", projection.soil_water_total),
        ("ProfileDepth", projection.profile_depth),
        ("ProfilePorosityCap", projection.profile_porosity_cap),
        ("ProfileFCStore", projection.profile_fc_store),
        ("ProfileWPStore", projection.profile_wp_store),
    ])
    .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "wb13_row_surface",
        detail: error.to_string(),
    })?;

    let row = Wb13DailyWaterBalanceRow::from_surface(
        1,
        projection.julian_day,
        projection.year,
        &row_surface,
    )
    .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "wb13_row",
        detail: error.to_string(),
    })?;

    let mut daily_surface = Wb13DailyWaterBalanceSurface::new();
    daily_surface
        .append_row(row)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb13_surface",
            detail: error.to_string(),
        })?;

    Ok(daily_surface.render_h5_wat_dat())
}

fn build_hillslope_wat_rows(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    snow: &openwepp_input_contract::parsers::snow::SnowParseOutput,
    frost: &openwepp_input_contract::parsers::frost::FrostParseOutput,
) -> Result<Vec<HillslopeWatRow>, HillslopeCliError> {
    let projection = build_first_day_wat_projection(climate, soil, snow, frost)?;

    let year =
        i16::try_from(projection.year).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!("year out of i16 range: {}", projection.year),
        })?;
    let month =
        i8::try_from(projection.month).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!("month out of i8 range: {}", projection.month),
        })?;
    let day_of_month = i8::try_from(projection.day_of_month).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!("day_of_month out of i8 range: {}", projection.day_of_month),
        }
    })?;
    let julian = i16::try_from(projection.julian_day).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!("julian out of i16 range: {}", projection.julian_day),
        }
    })?;
    let water_year = i16::try_from(projection.water_year).map_err(|_| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: format!("water_year out of i16 range: {}", projection.water_year),
        }
    })?;

    Ok(vec![HillslopeWatRow {
        wepp_id: 1,
        ofe_id: 1,
        year,
        sim_day_index: 1,
        julian,
        month,
        day_of_month,
        water_year,
        ofe: 1,
        p: projection.precipitation_mm,
        rm: projection.rm,
        q: projection.q,
        ep: projection.ep,
        es: projection.es,
        er: projection.er,
        dp: projection.dp,
        up_strm_q: projection.up_strm_q,
        sub_r_in: projection.sub_r_in,
        latqcc: projection.latqcc,
        total_soil_water: projection.total_soil,
        frozwt: projection.frozwt,
        snow_water: projection.snow_water,
        qofe: projection.qofe,
        tile: projection.tile,
        irr: projection.irr,
        area: projection.area,
        soil_water_total: Some(projection.soil_water_total),
        profile_depth: Some(projection.profile_depth),
        profile_porosity_cap: Some(projection.profile_porosity_cap),
        profile_fc_store: Some(projection.profile_fc_store),
        profile_wp_store: Some(projection.profile_wp_store),
        interception_storage: None,
    }])
}

fn build_loss_output_json(
    run_name: &str,
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    snow: &openwepp_input_contract::parsers::snow::SnowParseOutput,
    frost: &openwepp_input_contract::parsers::frost::FrostParseOutput,
) -> Result<String, HillslopeCliError> {
    let first_day = first_day_projection(climate)?;

    let payload = serde_json::json!({
        "schema": "openwepp-hillslope-loss-v1",
        "run_name": run_name,
        "first_day_julian": first_day.julian_day,
        "precipitation_mm": first_day.precipitation_mm,
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
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
) -> Result<String, HillslopeCliError> {
    let first_day = first_day_projection(climate)?;
    let file_name = file_name_string(output_path);
    Ok(format!(
        "openwepp_optional_output_v1\nrun_name={run_name}\nfile={file_name}\nyear={}\nday={}\nprecipitation_mm={:.3}\n",
        first_day.year, first_day.julian_day, first_day.precipitation_mm
    ))
}

#[derive(Debug, Clone, Copy)]
struct FirstDayClimateProjection {
    year: i32,
    month: i32,
    day_of_month: i32,
    julian_day: u16,
    precipitation_mm: f64,
    tmax: f64,
    tmin: f64,
}

#[derive(Debug, Clone, Copy)]
struct FirstDayWatProjection {
    year: i32,
    month: i32,
    day_of_month: i32,
    julian_day: u16,
    water_year: i32,
    precipitation_mm: f64,
    rm: f64,
    q: f64,
    ep: f64,
    es: f64,
    er: f64,
    dp: f64,
    up_strm_q: f64,
    sub_r_in: f64,
    latqcc: f64,
    total_soil: f64,
    frozwt: f64,
    snow_water: f64,
    qofe: f64,
    tile: f64,
    irr: f64,
    area: f64,
    soil_water_total: f64,
    profile_depth: f64,
    profile_porosity_cap: f64,
    profile_fc_store: f64,
    profile_wp_store: f64,
}

fn build_first_day_wat_projection(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    snow: &openwepp_input_contract::parsers::snow::SnowParseOutput,
    frost: &openwepp_input_contract::parsers::frost::FrostParseOutput,
) -> Result<FirstDayWatProjection, HillslopeCliError> {
    let first_day = first_day_projection(climate)?;

    let Some(primary_ofe) = soil.ofes.first() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "soil",
            detail: "missing primary OFE".to_string(),
        });
    };

    let mut previous_depth_mm = 0.0_f64;
    let mut profile_fc_store = 0.0_f64;
    let mut profile_wp_store = 0.0_f64;
    for layer in &primary_ofe.layers {
        let thickness_mm = (layer.depth_mm - previous_depth_mm).max(0.0);
        let fc = layer.fc_rosetta.unwrap_or(0.0).max(0.0);
        let wp = layer.theta_r_rosetta.unwrap_or(0.0).max(0.0);

        profile_fc_store += fc * thickness_mm;
        profile_wp_store += wp * thickness_mm;
        previous_depth_mm = layer.depth_mm;
    }

    let profile_depth = primary_ofe
        .layers
        .last()
        .map_or(0.0, |layer| layer.depth_mm.max(0.0));

    let total_soil = profile_fc_store.max(0.0);
    let frozwt = if frost.wint_red == 0 { 1.0 } else { 0.0 };
    let snow_water = snow.ssd.max(0.0);
    let soil_water_total = total_soil + frozwt;
    let profile_porosity_cap = profile_fc_store.max(profile_wp_store) + 20.0;
    let q = 0.0;
    let ep = ((first_day.tmax - first_day.tmin).max(0.0) * 0.05).min(10.0);
    let es = (first_day.precipitation_mm * 0.08).min(10.0);
    let er = if snow_water > 0.0 {
        0.0
    } else {
        (ep * 0.25).min(5.0)
    };
    let dp = (first_day.precipitation_mm * 0.01).max(0.0);
    let water_year = if first_day.month >= 10 {
        first_day.year + 1
    } else {
        first_day.year
    };

    Ok(FirstDayWatProjection {
        year: first_day.year,
        month: first_day.month,
        day_of_month: first_day.day_of_month,
        julian_day: first_day.julian_day,
        water_year,
        precipitation_mm: first_day.precipitation_mm,
        rm: 0.0,
        q,
        ep,
        es,
        er,
        dp,
        up_strm_q: 0.0,
        sub_r_in: 0.0,
        latqcc: 0.0,
        total_soil,
        frozwt,
        snow_water,
        qofe: q,
        tile: 0.0,
        irr: 0.0,
        area: 1.0,
        soil_water_total,
        profile_depth,
        profile_porosity_cap,
        profile_fc_store,
        profile_wp_store: profile_wp_store.min(profile_fc_store),
    })
}

fn first_day_projection(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
) -> Result<FirstDayClimateProjection, HillslopeCliError> {
    let Some(first_day) = climate.daily_records.first() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: "climate daily record set is empty".to_string(),
        });
    };

    match first_day {
        ClimateDailyRecord::NoBreakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            Ok(FirstDayClimateProjection {
                year: day.year,
                month: day.mon,
                day_of_month: day.day,
                julian_day,
                precipitation_mm: (day.prcp * 1_000.0).max(0.0),
                tmax: day.tmax,
                tmin: day.tmin,
            })
        }
        ClimateDailyRecord::Breakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            let prcp_mm = day
                .breakpoints
                .last()
                .map_or(0.0, |point| (point.pptcum * 1_000.0).max(0.0));
            Ok(FirstDayClimateProjection {
                year: day.year,
                month: day.mon,
                day_of_month: day.day,
                julian_day,
                precipitation_mm: prcp_mm,
                tmax: day.tmax,
                tmin: day.tmin,
            })
        }
    }
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

fn file_name_string(path: &Path) -> String {
    path.file_name()
        .and_then(OsStr::to_str)
        .map_or_else(String::new, ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_name_validator_accepts_expected_patterns() {
        assert!(release_binary_name_is_valid(
            "openwepp_260511",
            BinaryRole::Watershed
        ));
        assert!(release_binary_name_is_valid(
            "openwepp_260511_hill",
            BinaryRole::Hillslope
        ));
        assert!(release_binary_name_is_valid(
            "openwepp_260511a_replay",
            BinaryRole::Replay
        ));
    }

    #[test]
    fn release_name_validator_rejects_invalid_patterns() {
        assert!(!release_binary_name_is_valid(
            "openwepp_26051_hill",
            BinaryRole::Hillslope
        ));
        assert!(!release_binary_name_is_valid(
            "openwepp_260511_HILL",
            BinaryRole::Hillslope
        ));
        assert!(!release_binary_name_is_valid(
            "other_260511",
            BinaryRole::Watershed
        ));
    }

    #[test]
    fn launch_argv_contains_required_explicit_args() {
        let request = RunnerLaunchRequest {
            hillslope_binary: PathBuf::from("/tmp/openwepp-cli-hill"),
            run_dir: PathBuf::from("/tmp/run"),
            run_file: PathBuf::from("case.run"),
            output_dir: PathBuf::from("/tmp/out"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: true,
            manifest_path: Some(PathBuf::from("/tmp/out/manifest.json")),
        };

        let argv = build_hillslope_argv(&request);
        assert_eq!(
            argv,
            vec![
                "--run-dir",
                "/tmp/run",
                "--run-file",
                "case.run",
                "--output-dir",
                "/tmp/out",
                "--policy",
                "compat",
                "--legacy-sidecar-discovery",
                "--manifest-path",
                "/tmp/out/manifest.json",
            ]
        );
    }
}
