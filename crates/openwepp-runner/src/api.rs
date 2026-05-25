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
