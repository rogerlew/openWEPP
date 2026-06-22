use std::path::PathBuf;

use crate::policy::SidecarPolicy;

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HillslopeRuntimeSelection {
    #[default]
    Compatibility,
    DirectSkeletonNoop,
    DirectSkeletonShadowOnly,
    DirectPublicationFrameShadow,
    DirectPublicationFrameCutover,
    DirectProductionExecutor,
}

impl HillslopeRuntimeSelection {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatibility => "compatibility",
            Self::DirectSkeletonNoop => "direct-skeleton-noop",
            Self::DirectSkeletonShadowOnly => "direct-skeleton-shadow-only",
            Self::DirectPublicationFrameShadow => "direct-publication-frame-shadow",
            Self::DirectPublicationFrameCutover => "direct-publication-frame-cutover",
            Self::DirectProductionExecutor => "direct-production-executor",
        }
    }
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
