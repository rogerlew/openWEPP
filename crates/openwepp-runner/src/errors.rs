use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::process::ExitStatus;

use openwepp_legacy_bridge::sidecar::SidecarAdapterError;

use crate::role::BinaryRole;

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
    OfeTopologyMismatch {
        slope_ofe_count: usize,
        management_topology_count: usize,
        soil_topology_count: usize,
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
            Self::OfeTopologyMismatch { .. } => "CLIHILL-E-019",
        }
    }
}

impl fmt::Display for HillslopeCliError {
    #[allow(clippy::too_many_lines)]
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
            Self::OfeTopologyMismatch {
                slope_ofe_count,
                management_topology_count,
                soil_topology_count,
            } => write!(
                f,
                "{}",
                format_hillslope_topology_mismatch(
                    self.code(),
                    *slope_ofe_count,
                    *management_topology_count,
                    *soil_topology_count
                )
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

fn format_hillslope_topology_mismatch(
    error_code: &str,
    slope_ofe_count: usize,
    management_topology_count: usize,
    soil_topology_count: usize,
) -> String {
    let mut mismatches = Vec::new();
    if slope_ofe_count != management_topology_count {
        mismatches.push("slope-management");
    }
    if slope_ofe_count != soil_topology_count {
        mismatches.push("slope-soil");
    }
    if management_topology_count != soil_topology_count {
        mismatches.push("management-soil");
    }
    let mismatch_text = if mismatches.is_empty() {
        "none".to_string()
    } else {
        mismatches.join(",")
    };

    format!(
        "{error_code} hillslope OFE topology mismatch: slope={slope_ofe_count} management={management_topology_count} soil={soil_topology_count} mismatches=[{mismatch_text}]"
    )
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
            | Self::OfeTopologyMismatch { .. }
            | Self::MissingRequiredOutput { .. }
            | Self::TimeFormat { .. } => None,
        }
    }
}
