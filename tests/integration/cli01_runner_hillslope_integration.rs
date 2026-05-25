use std::fs;
use std::path::{Path, PathBuf};

use openwepp_runner::{
    BinaryRole, HillslopeCliError, HillslopeRunRequest, SidecarPolicy, execute_hillslope_run,
    validate_release_sidecar, write_release_sidecar_for_binary,
};

const RUNNER_CRATE_MANIFEST: &str = include_str!("../../crates/openwepp-runner/Cargo.toml");

#[test]
fn cli01_contract_conformance_runner_crate_declares_hillslope_watershed_and_runner_binaries() {
    assert!(RUNNER_CRATE_MANIFEST.contains("name = \"openwepp-cli-hill\""));
    assert!(RUNNER_CRATE_MANIFEST.contains("name = \"openwepp-cli-watershed\""));
    assert!(RUNNER_CRATE_MANIFEST.contains("name = \"open_wepp_runner\""));
}

#[test]
fn cli01_contract_conformance_hillslope_run_emits_required_outputs_and_manifest() {
    let fixture = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&fixture, "cli01_hillslope_success");
    let output_dir = temp_run_dir.join("output");

    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: output_dir.clone(),
            sidecar_policy: SidecarPolicy::Strict,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &[
            "openwepp-cli-hill".to_string(),
            "--run-dir".to_string(),
            temp_run_dir.display().to_string(),
            "--run-file".to_string(),
            "case.run".to_string(),
            "--output-dir".to_string(),
            output_dir.display().to_string(),
            "--policy".to_string(),
            "strict".to_string(),
        ],
    )
    .expect("strict CLI01 run fixture should complete");

    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());
    assert!(report.manifest_path.is_file());

    let manifest = fs::read_to_string(&report.manifest_path).expect("manifest should read");
    assert!(manifest.contains("openwepp-hillslope-run-manifest-v1"));
    assert!(manifest.contains("H5.wat.parquet"));
    assert!(manifest.contains("H5.plot.parquet"));
}

#[test]
fn cli01_contract_conformance_strict_sidecar_policy_rejects_unknown_discovery() {
    let fixture = fixture_path("hillslope_run_dir_unknown");
    let temp_run_dir = copy_fixture_to_temp(&fixture, "cli01_hillslope_strict_unknown");
    let output_dir = temp_run_dir.join("output");

    let error = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir,
            run_file: PathBuf::from("case.run"),
            output_dir,
            sidecar_policy: SidecarPolicy::Strict,
            legacy_sidecar_discovery: true,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect_err("strict policy should reject unknown sidecar");

    match error {
        HillslopeCliError::SidecarAdapter { source } => {
            assert_eq!(source.code(), "LSB-E-009");
        }
        other => panic!("expected sidecar adapter error, got {other}"),
    }
}

#[test]
fn cli01_contract_conformance_compat_sidecar_policy_warns_on_unknown_discovery() {
    let fixture = fixture_path("hillslope_run_dir_unknown");
    let temp_run_dir = copy_fixture_to_temp(&fixture, "cli01_hillslope_compat_unknown");
    let output_dir = temp_run_dir.join("output");

    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir,
            run_file: PathBuf::from("case.run"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: true,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("compat policy should accept unknown sidecar with warning");

    assert!(
        report
            .sidecar_warnings
            .iter()
            .any(|warning| warning.contains("LSB-W-002"))
    );
}

#[test]
fn cli01_contract_conformance_generated_release_sidecar_is_schema_valid() {
    let binary_path = std::env::current_exe().expect("current executable path should resolve");
    let sidecar_path = write_release_sidecar_for_binary(&binary_path, BinaryRole::Hillslope)
        .expect("release sidecar generation should succeed");

    assert!(sidecar_path.is_file());
    validate_release_sidecar(&sidecar_path).expect("generated sidecar should validate");
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("fixtures")
        .join("cli01")
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
