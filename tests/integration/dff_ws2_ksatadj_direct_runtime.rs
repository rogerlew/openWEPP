use std::fs;
use std::path::{Path, PathBuf};

use openwepp_runner::{HillslopeRunRequest, SidecarPolicy};
use serde_json::Value;

mod common;

#[test]
fn dff_ws2_forest_high_severity_loam_runs_with_live_direct_ksatadj_effect() {
    let fixture = fixture_path();
    let ksatadj_on_dir = copy_fixture_to_temp(&fixture, "dff_ws2_ksatadj_p313_on");
    let ksatadj_on_report = run_fixture(&ksatadj_on_dir);
    assert_p313_outputs(&ksatadj_on_report);
    assert!(
        ksatadj_evaluation_count(&ksatadj_on_report) > 0,
        "DFF-WS2 p313 should invoke the direct ksatadj evaluator when ksatadj=1"
    );

    let ksatadj_off_dir = copy_fixture_to_temp(&fixture, "dff_ws2_ksatadj_p313_off");
    disable_ksatadj_in_soil(&ksatadj_off_dir.join("p313.sol"));
    let ksatadj_off_report = run_fixture(&ksatadj_off_dir);
    assert_p313_outputs(&ksatadj_off_report);
    assert_eq!(
        ksatadj_evaluation_count(&ksatadj_off_report),
        0,
        "DFF-WS2 p313 should not invoke the direct ksatadj evaluator when only soil ksatadj is disabled"
    );
}

fn run_fixture(temp_run_dir: &Path) -> openwepp_runner::HillslopeRunReport {
    common::execute_with_complete_stage3_owner_seed(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.to_path_buf(),
            run_file: PathBuf::from("p313.run.toml"),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("DFF-WS2 disturbed-burn p313 fixture should run end-to-end")
}

fn assert_p313_outputs(report: &openwepp_runner::HillslopeRunReport) {
    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());
    assert!(
        report
            .optional_outputs
            .iter()
            .any(|path| path.ends_with("H313.wat.parquet")),
        "DFF-WS2 fixture should publish the optional WAT output"
    );
    let manifest = fs::read_to_string(&report.manifest_path).expect("manifest should read");
    assert!(manifest.contains("openwepp-hillslope-run-manifest-v1"));
    assert!(manifest.contains("H313.hbp"));
    assert!(manifest.contains("H313.loss.json"));
    assert!(manifest.contains("H313.wat.parquet"));
}

fn ksatadj_evaluation_count(report: &openwepp_runner::HillslopeRunReport) -> u64 {
    let manifest = fs::read_to_string(&report.manifest_path).expect("manifest should read");
    let manifest: Value = serde_json::from_str(&manifest).expect("manifest should parse");
    manifest
        .pointer("/direct_runtime_counters/ksatadj_effective_conductivity_evaluations")
        .and_then(Value::as_u64)
        .expect("manifest should carry the direct ksatadj evaluation counter")
}

fn disable_ksatadj_in_soil(soil_path: &Path) {
    let contents = fs::read_to_string(soil_path).expect("soil fixture should read");
    let mut changed = false;
    let updated = contents
        .lines()
        .map(|line| {
            if !changed && line.starts_with("1\t 'forest high sev fire'\t 'loam'") {
                changed = true;
                line.replacen("1\t", "0\t", 1)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        changed,
        "p313 disturbed-policy row should be present for ksatadj-off comparison"
    );
    fs::write(soil_path, format!("{updated}\n")).expect("soil fixture should be writable");
}

fn fixture_path() -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("fixtures")
        .join("disturbed_burn")
        .join("forest_high_severity_loam")
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
