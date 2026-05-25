use std::process::Command;

use crate::api::RunnerLaunchRequest;
use crate::errors::RunnerError;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::SidecarPolicy;
    use std::path::PathBuf;

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
