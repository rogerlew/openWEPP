use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_runner::{
    HillslopeCliError, HillslopeRunReport, HillslopeRunRequest, SidecarPolicy,
    execute_hillslope_run,
};

const RUNFILE_CONTRACT: &str =
    include_str!("../../docs/contracts/openwepp-hillslope-runfile-contract.md");
const WATERSHED_RUNFILE_CONTRACT: &str =
    include_str!("../../docs/contracts/openwepp-watershed-runfile-contract.md");
const RUNNER_CONTRACT: &str = include_str!("../../docs/contracts/openwepp-runner-contract.md");
const HILLSLOPE_CLI_SPEC: &str = include_str!(
    "../../docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md"
);
const RUNNER_CRATE_MANIFEST: &str = include_str!("../../crates/openwepp-runner/Cargo.toml");
const RUNNER_CRATE_LIB: &str = include_str!("../../crates/openwepp-runner/src/lib.rs");
const WATERSHED_CLI_SOURCE: &str =
    include_str!("../../crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs");
const OUTPUT_CRATE_MANIFEST: &str =
    include_str!("../../crates/openwepp-hillslope-output/Cargo.toml");
const OUTPUT_CRATE_LIB: &str = include_str!("../../crates/openwepp-hillslope-output/src/lib.rs");

#[test]
fn cli03_contract_surface_declares_metric_runfile_and_required_outputs() {
    for expected in [
        "openwepp-hillslope-runfile-v1",
        "unit_system",
        "metric",
        "`[outputs]` required keys:",
        "`pass` (`string` path, must end in `.hbp`)",
        "`loss` (`string` path, must end in `.json`)",
        "`wat` (`string`, optional, must end in `.parquet`)",
    ] {
        assert!(
            RUNFILE_CONTRACT.contains(expected),
            "runfile contract missing expected text: {expected}"
        );
    }

    assert!(
        RUNNER_CONTRACT.contains("`crates/openwepp-hillslope-output/`"),
        "runner contract must require dedicated output crate"
    );
    assert!(
        HILLSLOPE_CLI_SPEC.contains("Contract-Test Minimums (CLI03)")
            || HILLSLOPE_CLI_SPEC.contains("Contract-Test Minimums (CLI03/CLI04)"),
        "CLI spec must declare CLI03 contract-test minimums"
    );
}

#[test]
fn cli03_watershed_contract_surface_declares_pw0_inputs_and_hillslope_block() {
    for expected in [
        "openwepp-watershed-runfile-v1",
        "--legacy-sidecar-discovery",
        "pw0_str",
        "pw0_chn",
        "pw0_imp",
        "pw0_man",
        "pw0_slp",
        "pw0_cli",
        "pw0_sol",
        "inputs.hillslopes_block",
        "inputs.chaninp",
        "inputs.tcr",
        "ebe_pw0",
        "chan_out",
        "chanwb",
        "chnwb",
        "soil_pw0",
        "totalwatsed3",
        "loss_hill",
        "loss_chn",
        "loss_out",
        "loss_class_data",
        "loss_all_years_hill",
        "loss_all_years_chn",
        "loss_all_years_out",
        "loss_all_years_class_data",
    ] {
        assert!(
            WATERSHED_RUNFILE_CONTRACT.contains(expected),
            "watershed runfile contract missing expected text: {expected}"
        );
    }
}

#[test]
fn cli03_watershed_cli_surface_uses_runfile_pattern_with_legacy_discovery_flag() {
    for expected in [
        "\"--run-dir\"",
        "\"--run-file\"",
        "\"--output-dir\"",
        "\"--legacy-sidecar-discovery\"",
        "inputs.pw0_str",
        "inputs.pw0_chn",
        "inputs.pw0_imp",
        "inputs.pw0_man",
        "inputs.pw0_slp",
        "inputs.pw0_cli",
        "inputs.pw0_sol",
        "inputs.hillslopes_block",
        "outputs.loss_all_years_class_data",
        "write_watershed_interchange_outputs",
    ] {
        assert!(
            WATERSHED_CLI_SOURCE.contains(expected),
            "watershed CLI source missing expected runfile surface marker: {expected}"
        );
    }
}

#[test]
fn cli03_contract_surface_declares_output_crate_layout() {
    assert!(
        OUTPUT_CRATE_MANIFEST.contains("name = \"openwepp-hillslope-output\""),
        "output crate manifest should declare crate identity"
    );

    for module_name in [
        "pub mod contracts;",
        "pub mod writers;",
        "pub mod manifest;",
    ] {
        assert!(
            OUTPUT_CRATE_LIB.contains(module_name),
            "output crate lib must expose {module_name}"
        );
    }
}

#[test]
fn cli03_runner_crate_declares_hillslope_and_watershed_binary_targets() {
    assert!(RUNNER_CRATE_MANIFEST.contains("name = \"openwepp-cli-hill\""));
    assert!(RUNNER_CRATE_MANIFEST.contains("name = \"openwepp-cli-watershed\""));
}

#[test]
fn cli03_runner_crate_wires_output_surface_dependency() {
    assert!(
        RUNNER_CRATE_MANIFEST.contains("openwepp-hillslope-output"),
        "CLI03 requires runner output-surface delegation to openwepp-hillslope-output crate"
    );
    assert!(
        RUNNER_CRATE_MANIFEST.contains("openwepp-watershed-output"),
        "CLI03 requires watershed output-surface delegation to openwepp-watershed-output crate"
    );
    assert!(
        RUNNER_CRATE_LIB.contains("openwepp_hillslope_output"),
        "CLI03 requires lib wiring from runner boundary into openwepp-hillslope-output APIs"
    );
}

#[test]
fn cli03_runfile_validation_rejects_schema_mismatch() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v0"
run_name = "cli03-schema-mismatch"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let error = execute_fixture_with_runfile(runfile, "cli03_schema_mismatch")
        .expect_err("schema mismatch should fail hard");

    assert_eq!(error.code(), "CLIHILL-E-010");
    assert!(error.to_string().contains("unsupported schema"));
}

#[test]
fn cli03_runfile_validation_rejects_non_metric_unit_system() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-non-metric"
unit_system = "english"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let error = execute_fixture_with_runfile(runfile, "cli03_non_metric")
        .expect_err("non-metric unit_system should fail hard");

    assert!(
        error.to_string().contains("unit_system") || error.to_string().contains("metric"),
        "error must mention metric unit-system requirement, observed: {error}"
    );
}

#[test]
fn cli03_runfile_validation_rejects_unresolved_required_input_paths() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-missing-required-input"
unit_system = "metric"

[inputs]
soil = "missing.soil"
management = "missing.man"
slope = "missing.slp"
climate = "missing.cli"
wepp_ui = true

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let error = execute_fixture_with_runfile(runfile, "cli03_unresolved_inputs")
        .expect_err("unresolved required .run inputs should fail hard");

    assert!(
        error.to_string().contains("missing")
            || error.to_string().contains("unresolved")
            || error.to_string().contains("readable file"),
        "error must mention unresolved required input paths, observed: {error}"
    );
}

#[test]
fn cli03_fixture_run_emits_required_and_configured_optional_outputs_with_manifest_checksums() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-output-success"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
wat = "output/H1.wat.parquet"
plot = "output/H1.plot.parquet"
element = "output/H1.element.parquet"
"#;

    let (report, _temp_run_dir) =
        execute_fixture_with_runfile_report(runfile, "cli03_output_success")
            .expect("fixture run should succeed");

    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());
    assert_eq!(report.optional_outputs.len(), 3);
    for path in &report.optional_outputs {
        assert!(
            path.is_file(),
            "optional output missing: {}",
            path.display()
        );
    }

    let manifest =
        fs::read_to_string(&report.manifest_path).expect("manifest file should be readable");
    for expected in [
        "openwepp-hillslope-run-manifest-v1",
        "H1.hbp",
        "H1.loss.json",
        "H1.wat.parquet",
        "H1.plot.parquet",
        "H1.element.parquet",
    ] {
        assert!(
            manifest.contains(expected),
            "manifest should include {expected}"
        );
    }
}

#[test]
fn cli03_legacy_sidecar_discovery_mode_uses_legacy_sidecars_and_ignores_runfile_overrides() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-legacy-discovery"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false
pmetpara = "missing_in_legacy_mode.txt"

[inputs.snow]
rst = 99.0
newsnw = 999.0
ssd = 999.0

[inputs.frost]
wintRed = 1
fineTop = 9
fineBot = 99
ksnowf = 9.0
kresf = 9.0
ksoilf = 9.0
kfactor1 = 9.0
kfactor2 = 9.0
kfactor3 = 9.0

[outputs]
pass = "output/Hlegacy.hbp"
loss = "output/Hlegacy.loss.json"
"#;

    let (report, _temp_run_dir) =
        execute_fixture_with_runfile_report_with_mode(runfile, "cli03_legacy_discovery", true)
            .expect("legacy discovery run should succeed");

    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());

    let manifest =
        fs::read_to_string(&report.manifest_path).expect("manifest file should be readable");
    assert!(manifest.contains("\"sidecar_discovery_mode\": \"legacy-sidecar-discovery\""));
    assert!(manifest.contains("snow.txt"));
    assert!(manifest.contains("frost.txt"));
    assert!(!manifest.contains("<inline>"));
}

fn execute_fixture_with_runfile(
    runfile_payload: &str,
    prefix: &str,
) -> Result<(), HillslopeCliError> {
    execute_fixture_with_runfile_report(runfile_payload, prefix).map(|_| ())
}

fn execute_fixture_with_runfile_report(
    runfile_payload: &str,
    prefix: &str,
) -> Result<(HillslopeRunReport, PathBuf), HillslopeCliError> {
    execute_fixture_with_runfile_report_with_mode(runfile_payload, prefix, false)
}

fn execute_fixture_with_runfile_report_with_mode(
    runfile_payload: &str,
    prefix: &str,
    legacy_sidecar_discovery: bool,
) -> Result<(HillslopeRunReport, PathBuf), HillslopeCliError> {
    let _execution_guard = runner_execution_lock()
        .lock()
        .expect("runner execution lock should be acquirable");

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
    let run_file_path = temp_run_dir.join("case.run");
    fs::write(&run_file_path, runfile_payload).expect("runfile fixture should be writable");

    let output_dir = temp_run_dir.join("output");
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )?;

    Ok((report, temp_run_dir))
}

fn runner_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
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
