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

use openwepp_hillslope_orchestrator::HillslopeWritebackSurface;
use openwepp_hillslope_orchestrator::runtime_inputs::{
    build_hillslope_runtime_surface_from_climate, build_hillslope_runtime_surface_from_frost,
    build_hillslope_runtime_surface_from_management, build_hillslope_runtime_surface_from_slope,
    build_hillslope_runtime_surface_from_snow, build_hillslope_runtime_surface_from_soil,
};
use openwepp_input_contract::parsers::climate::{
    ClimateDailyRecord, CompatibilityOptions, ParserMode as ClimateParserMode, parse_climate_file,
};
use openwepp_input_contract::parsers::frost::{ParseMode as FrostParseMode, parse_frost_from_path};
use openwepp_input_contract::parsers::management::{
    ParseMode as ManagementParseMode, parse_management_from_path,
};
use openwepp_input_contract::parsers::pmetpara::{
    ParseMode as PmetparaParseMode, PmetparaParseOptions, parse_pmetpara_file,
};
use openwepp_input_contract::parsers::slope::{SlopeParserOptions, parse_slope_file};
use openwepp_input_contract::parsers::snow::{
    ParseMode as SnowParseMode, SnowParseOptions, parse_snow_file,
};
use openwepp_input_contract::parsers::soil::{
    ParserMode as SoilParserMode, SoilParserOptions, parse_soil,
};
use openwepp_input_contract::parsers::wepp_ui::{
    WeppUiParserMode, WeppUiParserOptions, parse_wepp_ui_from_path,
};
use openwepp_legacy_bridge::policy::CompatibilityPolicy;
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterError, SidecarAdapterRequest, SidecarBinding, SidecarContract, SidecarDiscovery,
    SidecarId, SidecarRequirement, adapt_sidecar_bindings,
};
use openwepp_summary_accumulator::{
    SummaryScalarSurface, Wb13DailyWaterBalanceRow, Wb13DailyWaterBalanceSurface,
};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

pub const BINARY_RELEASE_SCHEMA_ID: &str = "openwepp-binary-release-metadata-v1";
pub const HILLSLOPE_RUN_MANIFEST_SCHEMA_ID: &str = "openwepp-hillslope-run-manifest-v1";
pub const H5_WAT_FILE_NAME: &str = "H5.wat.dat";
pub const H5_PLOT_FILE_NAME: &str = "H5.plot.dat";

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
pub enum RunnerEngine {
    LegacyWepp,
    Openwepp,
}

impl RunnerEngine {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LegacyWepp => "legacy_wepp",
            Self::Openwepp => "openwepp",
        }
    }
}

impl std::str::FromStr for RunnerEngine {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "legacy_wepp" => Ok(Self::LegacyWepp),
            "openwepp" => Ok(Self::Openwepp),
            _ => Err(format!(
                "unsupported engine selector '{value}' (expected legacy_wepp|openwepp)"
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
    UnsupportedEngineSelector { selector: String },
    HillslopeBinaryMissing { path: PathBuf },
    LaunchFailure { source: io::Error },
    NonZeroExit { status: ExitStatus },
    ReleaseLint { source: ReleaseLintError },
}

impl RunnerError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingArgument { .. } | Self::UnsupportedEngineSelector { .. } => "RUNNER-E-001",
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
            Self::UnsupportedEngineSelector { selector } => {
                write!(f, "{} unsupported engine selector {selector}", self.code())
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
            | Self::UnsupportedEngineSelector { .. }
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
    pub engine: RunnerEngine,
    pub hillslope_binary: PathBuf,
    pub run_dir: PathBuf,
    pub run_file: PathBuf,
    pub output_dir: PathBuf,
    pub sidecar_policy: SidecarPolicy,
    pub manifest_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct HillslopeRunRequest {
    pub run_dir: PathBuf,
    pub run_file: PathBuf,
    pub output_dir: PathBuf,
    pub sidecar_policy: SidecarPolicy,
    pub manifest_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct HillslopeRunReport {
    pub output_h5_wat: PathBuf,
    pub output_h5_plot: PathBuf,
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
    resolved_sidecars: BTreeMap<String, String>,
    input_checksums: BTreeMap<String, String>,
    output_checksums: BTreeMap<String, String>,
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

    if let Some(path) = &request.manifest_path {
        argv.push("--manifest-path".to_string());
        argv.push(path.display().to_string());
    }

    argv
}

pub fn launch_hillslope(request: &RunnerLaunchRequest) -> Result<(), RunnerError> {
    if request.engine != RunnerEngine::Openwepp {
        return Err(RunnerError::UnsupportedEngineSelector {
            selector: request.engine.as_str().to_string(),
        });
    }

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
    let metadata = build_release_metadata_document(binary_path, role)?;
    let sidecar_path = sidecar_path_for_binary(binary_path);
    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|source| ReleaseMetadataError::JsonSerialize { source })?;
    fs::write(&sidecar_path, json).map_err(|source| ReleaseMetadataError::Io {
        path: sidecar_path.clone(),
        source,
    })?;

    validate_release_sidecar(&sidecar_path)?;
    Ok(sidecar_path)
}

pub fn validate_release_sidecar(sidecar_path: &Path) -> Result<Value, ReleaseMetadataError> {
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

    let soil_path = discover_single_extension_file(&request.run_dir, "sol")?;
    let management_path = discover_single_extension_file(&request.run_dir, "man")?;
    let slope_path = discover_single_extension_file(&request.run_dir, "slp")?;
    let climate_path = discover_single_extension_file(&request.run_dir, "cli")?;

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

    let discovered_sidecars = discover_sidecars(
        &request.run_dir,
        &[
            file_name_string(&run_file_path),
            file_name_string(&soil_path),
            file_name_string(&management_path),
            file_name_string(&slope_path),
            file_name_string(&climate_path),
            H5_WAT_FILE_NAME.to_string(),
            H5_PLOT_FILE_NAME.to_string(),
            "openwepp_hillslope_run_manifest.json".to_string(),
        ],
    )?;

    let sidecar_contracts = hillslope_sidecar_contracts()?;
    let sidecar_response = adapt_sidecar_bindings(&SidecarAdapterRequest {
        policy: request.sidecar_policy.as_legacy_bridge_policy(),
        contracts: sidecar_contracts,
        discovered: discovered_sidecars,
    })
    .map_err(|source| HillslopeCliError::SidecarAdapter { source })?;

    let snow_path = required_sidecar_binding_path(&sidecar_response.bindings, "snow")?;
    let frost_path = required_sidecar_binding_path(&sidecar_response.bindings, "frost")?;
    let wepp_ui_path = required_sidecar_binding_path(&sidecar_response.bindings, "wepp_ui")?;
    let pmetpara_path = required_sidecar_binding_path(&sidecar_response.bindings, "pmetpara")?;

    let snow = parse_snow_file(&snow_path, request.sidecar_policy.as_snow_parse_options())
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "snow",
            detail: error.to_string(),
        })?;

    let frost = parse_frost_from_path(&frost_path, request.sidecar_policy.as_frost_parse_mode())
        .map_err(|error| HillslopeCliError::ParseFailure {
            surface: "frost",
            detail: error.to_string(),
        })?;

    let soil_versions = vec![soil.datver.numeric(); soil.ofes.len().max(1)];
    let _wepp_ui = parse_wepp_ui_from_path(
        &wepp_ui_path,
        WeppUiParserOptions {
            mode: request.sidecar_policy.as_wepp_ui_parse_mode(),
            requested_hourly_seepage: true,
            soil_versions,
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
            require_sidecar: true,
        },
    )
    .map_err(|error| HillslopeCliError::ParseFailure {
        surface: "pmetpara",
        detail: error.to_string(),
    })?;

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

    let wb13_text = build_h5_wat_output(&climate, &soil, &snow, &frost)?;
    let plot_text = build_h5_plot_output(&climate)?;

    let output_h5_wat = request.output_dir.join(H5_WAT_FILE_NAME);
    let output_h5_plot = request.output_dir.join(H5_PLOT_FILE_NAME);

    fs::write(&output_h5_wat, wb13_text).map_err(|source| HillslopeCliError::OutputWrite {
        path: output_h5_wat.clone(),
        source,
    })?;
    fs::write(&output_h5_plot, plot_text).map_err(|source| HillslopeCliError::OutputWrite {
        path: output_h5_plot.clone(),
        source,
    })?;

    if !output_h5_wat.is_file() {
        return Err(HillslopeCliError::MissingRequiredOutput {
            output_name: H5_WAT_FILE_NAME,
        });
    }
    if !output_h5_plot.is_file() {
        return Err(HillslopeCliError::MissingRequiredOutput {
            output_name: H5_PLOT_FILE_NAME,
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

    let input_paths = [
        run_file_path.as_path(),
        soil_path.as_path(),
        management_path.as_path(),
        slope_path.as_path(),
        climate_path.as_path(),
        snow_path.as_path(),
        frost_path.as_path(),
        wepp_ui_path.as_path(),
        pmetpara_path.as_path(),
    ];

    let mut input_checksums = BTreeMap::new();
    for path in input_paths {
        input_checksums.insert(
            path.display().to_string(),
            sha256_file_hex(path).map_err(|source| HillslopeCliError::Io {
                path: path.to_path_buf(),
                source,
            })?,
        );
    }

    let mut output_checksums = BTreeMap::new();
    output_checksums.insert(
        output_h5_wat.display().to_string(),
        sha256_file_hex(&output_h5_wat).map_err(|source| HillslopeCliError::Io {
            path: output_h5_wat.clone(),
            source,
        })?,
    );
    output_checksums.insert(
        output_h5_plot.display().to_string(),
        sha256_file_hex(&output_h5_plot).map_err(|source| HillslopeCliError::Io {
            path: output_h5_plot.clone(),
            source,
        })?,
    );

    let mut resolved_sidecars = BTreeMap::new();
    for binding in &sidecar_response.bindings {
        resolved_sidecars.insert(
            binding.sidecar_id.as_str().to_string(),
            binding.resolved_path.display().to_string(),
        );
    }

    let manifest_path = request.manifest_path.clone().unwrap_or_else(|| {
        request
            .output_dir
            .join("openwepp_hillslope_run_manifest.json")
    });

    let manifest = HillslopeRunManifest {
        schema: HILLSLOPE_RUN_MANIFEST_SCHEMA_ID.to_string(),
        engine: RunnerEngine::Openwepp.as_str().to_string(),
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
        resolved_sidecars,
        input_checksums,
        output_checksums,
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })?;
    fs::write(&manifest_path, manifest_json).map_err(|source| {
        HillslopeCliError::ManifestWrite {
            path: manifest_path.clone(),
            source,
        }
    })?;

    let sidecar_warnings = sidecar_response
        .warnings
        .iter()
        .map(|warning| format!("{} {}", warning.code.message_id(), warning.detail))
        .collect();

    Ok(HillslopeRunReport {
        output_h5_wat,
        output_h5_plot,
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

fn discover_single_extension_file(
    run_dir: &Path,
    extension: &'static str,
) -> Result<PathBuf, HillslopeCliError> {
    let mut matches = Vec::new();
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
        let Some(ext) = path.extension().and_then(OsStr::to_str) else {
            continue;
        };
        if ext.eq_ignore_ascii_case(extension) {
            matches.push(path);
        }
    }

    if matches.is_empty() {
        return Err(HillslopeCliError::CoreInputMissing {
            extension,
            run_dir: run_dir.to_path_buf(),
        });
    }
    if matches.len() > 1 {
        return Err(HillslopeCliError::CoreInputAmbiguous {
            extension,
            run_dir: run_dir.to_path_buf(),
            count: matches.len(),
        });
    }

    Ok(matches.remove(0))
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

fn hillslope_sidecar_contracts() -> Result<Vec<SidecarContract>, HillslopeCliError> {
    let required = [
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
    for (id, file_name) in required {
        contracts.push(build_sidecar_contract(
            id,
            file_name,
            SidecarRequirement::Required,
        )?);
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

fn required_sidecar_binding_path(
    bindings: &[SidecarBinding],
    sidecar_id: &'static str,
) -> Result<PathBuf, HillslopeCliError> {
    bindings
        .iter()
        .find(|binding| binding.sidecar_id.as_str() == sidecar_id)
        .map(|binding| binding.resolved_path.clone())
        .ok_or(HillslopeCliError::SidecarBindingMissing { sidecar_id })
}

fn merge_runtime_surfaces(
    mut base: HillslopeWritebackSurface,
    overlay: HillslopeWritebackSurface,
) -> HillslopeWritebackSurface {
    base.state_surface.extend(overlay.state_surface);
    base.flux_surface.extend(overlay.flux_surface);
    base
}

fn build_h5_wat_output(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
    soil: &openwepp_input_contract::parsers::soil::SoilProfile,
    snow: &openwepp_input_contract::parsers::snow::SnowParseOutput,
    frost: &openwepp_input_contract::parsers::frost::FrostParseOutput,
) -> Result<String, HillslopeCliError> {
    let (year, julian_day, precipitation_mm, tmax, tmin) = first_day_projection(climate)?;

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
    let ep = ((tmax - tmin).max(0.0) * 0.05).min(10.0);
    let es = (precipitation_mm * 0.08).min(10.0);
    let er = if snow_water > 0.0 {
        0.0
    } else {
        (ep * 0.25).min(5.0)
    };
    let dp = (precipitation_mm * 0.01).max(0.0);

    let row_surface = SummaryScalarSurface::from_pairs([
        ("P", precipitation_mm),
        ("RM", 0.0),
        ("Q", q),
        ("Ep", ep),
        ("Es", es),
        ("Er", er),
        ("Dp", dp),
        ("UpStrmQ", 0.0),
        ("SubRIn", 0.0),
        ("latqcc", 0.0),
        ("Total-Soil", total_soil),
        ("frozwt", frozwt),
        ("Snow-Water", snow_water),
        ("QOFE", q),
        ("Tile", 0.0),
        ("Irr", 0.0),
        ("Area", 1.0),
        ("SoilWaterTotal", soil_water_total),
        ("ProfileDepth", profile_depth),
        ("ProfilePorosityCap", profile_porosity_cap),
        ("ProfileFCStore", profile_fc_store),
        ("ProfileWPStore", profile_wp_store.min(profile_fc_store)),
    ])
    .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "wb13_row_surface",
        detail: error.to_string(),
    })?;

    let row = Wb13DailyWaterBalanceRow::from_surface(1, julian_day, year, &row_surface).map_err(
        |error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb13_row",
            detail: error.to_string(),
        },
    )?;

    let mut daily_surface = Wb13DailyWaterBalanceSurface::new();
    daily_surface
        .append_row(row)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb13_surface",
            detail: error.to_string(),
        })?;

    Ok(daily_surface.render_h5_wat_dat())
}

fn build_h5_plot_output(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
) -> Result<String, HillslopeCliError> {
    let (year, julian_day, precipitation_mm, _, _) = first_day_projection(climate)?;

    Ok(format!(
        "PLOT SUMMARY\nY J OFE P NLAYERS\n{year} {julian_day} 1 {precipitation_mm:.2} 1\n"
    ))
}

fn first_day_projection(
    climate: &openwepp_input_contract::parsers::climate::ClimateFile,
) -> Result<(i32, u16, f64, f64, f64), HillslopeCliError> {
    let Some(first_day) = climate.daily_records.first() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: "climate daily record set is empty".to_string(),
        });
    };

    match first_day {
        ClimateDailyRecord::NoBreakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            Ok((
                day.year,
                julian_day,
                (day.prcp * 1_000.0).max(0.0),
                day.tmax,
                day.tmin,
            ))
        }
        ClimateDailyRecord::Breakpoint(day) => {
            let julian_day = day_of_year(day.year, day.mon, day.day)?;
            let prcp_mm = day
                .breakpoints
                .last()
                .map_or(0.0, |point| (point.pptcum * 1_000.0).max(0.0));
            Ok((day.year, julian_day, prcp_mm, day.tmax, day.tmin))
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
            engine: RunnerEngine::Openwepp,
            hillslope_binary: PathBuf::from("/tmp/openwepp-cli-hill"),
            run_dir: PathBuf::from("/tmp/run"),
            run_file: PathBuf::from("case.run"),
            output_dir: PathBuf::from("/tmp/out"),
            sidecar_policy: SidecarPolicy::Compat,
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
                "--manifest-path",
                "/tmp/out/manifest.json",
            ]
        );
    }
}
