use std::fs;
use std::path::Path;

use openwepp_runner::{JenningsPhaseValidationRequest, run_jennings_phase_validation};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = "docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/package.md";
const WORK_PACKAGE_INDEX: &str = "docs/work-packages/README.md";
const ROOT_CARGO: &str = "Cargo.toml";

fn repo_text(relative_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn assert_contains(haystack: &str, needle: &str, context: &str) {
    assert!(
        haystack.contains(needle),
        "{context} missing required marker: {needle}"
    );
}

#[test]
fn contract_authorizes_only_opt_in_hourly_partition_with_rollback() {
    let contract = repo_text(CONTRACT);

    for marker in [
        "contract_version: 95",
        "snow_phase_partition_model",
        "harder_pomeroy_hourly",
        "legacy_rst",
        "INV-SNOWFREEZE-065",
        "OBL-SNOWFREEZE-P-040",
        "Opt-In Hourly Partition And Jennings Validation Addendum",
        "`legacy_rst` remains the default",
        "exact saturation (`RH=1.0`)",
        "`hrrain + hrsnow / 10`",
        "real direct snow consumer must receive the selected hourly partition",
        "does not authorize default activation",
        "parser/runfile/user CLI selectors",
        "Jennings et al. observed-phase validation",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }
}

#[test]
fn package_and_index_bind_the_10_3_5b_execution_scope() {
    let package = repo_text(PACKAGE);
    for marker in [
        "Opt-In Hourly Partition And Jennings Validation",
        "Default behavior must remain `legacy_rst`",
        "No parser/runfile/user CLI activation",
        "real direct-production snow consumer reads the opt-in partition result",
        "full local file2 corpus",
        "No site calibration",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let index = repo_text(WORK_PACKAGE_INDEX);
    assert_contains(
        &index,
        "20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001",
        WORK_PACKAGE_INDEX,
    );
}

#[test]
fn test_target_is_registered_for_contract_gate_execution() {
    let cargo = repo_text(ROOT_CARGO);
    for marker in [
        "snowdensity10_3_5b_hourly_partition_jennings_contract",
        "tests/integration/snowdensity10_3_5b_hourly_partition_jennings_contract.rs",
    ] {
        assert_contains(&cargo, marker, ROOT_CARGO);
    }
}

#[test]
fn jennings_validator_writes_compact_report_from_station_thresholds() {
    let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/snowdensity10_3_5b_jennings_validator_smoke");
    let _ = fs::remove_dir_all(&output_dir);
    fs::create_dir_all(&output_dir).expect("target output dir should be creatable");
    let observations = output_dir.join("file2.csv");
    let thresholds = output_dir.join("file3.csv");
    fs::write(
        &observations,
        "\
Station_ID,Date,Hour,Air_Temp,Dewpoint,RH,gridded_data_pres,Prec_Type,Snow_Phase,Rain_Phase\n\
AAA01,2001-01-01,0,0.2,0.2,100.0,100.0,71,0,1\n\
AAA01,2001-01-01,1,-1.5,-1.5,100.0,100.0,71,1,0\n\
BBB02,2001-01-01,0,1.2,0.0,92.0,100.0,71,0,1\n",
    )
    .expect("synthetic observations should be writable");
    fs::write(&thresholds, "Station_ID,temp50\nAAA01,0.0\nBBB02,1.0\n")
        .expect("synthetic thresholds should be writable");

    let report = run_jennings_phase_validation(&JenningsPhaseValidationRequest {
        observations_path: observations,
        thresholds_path: thresholds,
        output_dir: output_dir.clone(),
        max_rows: None,
    })
    .expect("synthetic Jennings validation should complete");

    assert_eq!(report.rows_read, 3);
    assert_eq!(report.rows_scored, 3);
    assert_eq!(report.stations_scored, 2);
    assert!(report.harder_pomeroy_hourly.rows_scored == 3);
    assert!(output_dir.join("jennings-validation-report.json").is_file());
    assert!(output_dir.join("jennings-validation-report.md").is_file());
}
