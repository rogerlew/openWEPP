use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_runner::{
    HillslopeCliError, HillslopeRunReport, HillslopeRunRequest, SidecarPolicy,
    execute_hillslope_run,
};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};
use serde_json::Value;

const RUNFILE_CONTRACT: &str =
    include_str!("../../docs/contracts/openwepp-hillslope-runfile-contract.md");
const WATERSHED_RUNFILE_CONTRACT: &str =
    include_str!("../../docs/contracts/openwepp-watershed-runfile-contract.md");
const RUNNER_CONTRACT: &str = include_str!("../../docs/contracts/openwepp-runner-contract.md");
const HILLSLOPE_CLI_SPEC: &str = include_str!(
    "../../docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md"
);
const RUNNER_CRATE_MANIFEST: &str = include_str!("../../crates/openwepp-runner/Cargo.toml");
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
        "manifest_file",
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
        "manifest_file",
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
        runner_src_tree_contains("openwepp_hillslope_output"),
        "CLI03 requires runner source-tree wiring into openwepp-hillslope-output APIs"
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
wepp_ui = false

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
wepp_ui = false

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
wepp_ui = false

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
fn cli03_fixture_run_publishes_wb16_ealpha_runtime_seed_provenance() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-wb16-ealpha-provenance"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let (report, _temp_run_dir) =
        execute_fixture_with_runfile_report(runfile, "cli03_wb16_ealpha_provenance")
            .expect("fixture run should succeed");

    assert!(
        report
            .sidecar_warnings
            .iter()
            .all(|warning| !warning.contains("SIMPIPE-W-003")),
        "did not expect WB16 ealpha compatibility warning when runtime producer is present: {:?}",
        report.sidecar_warnings
    );

    let manifest_payload =
        fs::read_to_string(&report.manifest_path).expect("manifest file should be readable");
    assert!(
        manifest_payload.contains("\"wb16_ealpha_compatibility_seed_used\": false"),
        "expected wb16_ealpha_compatibility_seed_used=false in manifest execution provenance"
    );
    assert!(
        manifest_payload.contains("\"wb16_ealpha_seed_policy\": \"runtime_provided\""),
        "expected wb16_ealpha_seed_policy=runtime_provided in manifest execution provenance"
    );
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

#[test]
fn cli03_mofe02_enables_hillslope_soil_topology_scope_guard_when_slope_and_management_align() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-mofe02-soil-topology-guard"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let error = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_mofe02_soil_guard",
        false,
        |run_dir| {
            write_single_ofe_slope(&run_dir.join("case.slp"));
            write_soil_with_ntemp(&run_dir.join("case.sol"), 2);
        },
    )
    .expect_err("soil topology guard should fail when ntemp differs from aligned slope/management");

    assert_eq!(error.code(), "CLIHILL-E-010");
    let message = error.to_string();
    assert!(message.contains("SOL-E-007"));
    assert!(message.contains("hillslope"));
}

#[test]
fn cli03_mofe02_rejects_slope_management_and_slope_soil_mismatch() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-mofe02-slope-mismatch"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let error = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_mofe02_slope_mismatch",
        false,
        |run_dir| write_two_ofe_slope(&run_dir.join("case.slp")),
    )
    .expect_err("slope topology mismatch should fail before runtime merge");

    assert_eq!(error.code(), "CLIHILL-E-019");
    let message = error.to_string();
    assert!(message.contains("slope-management"));
    assert!(message.contains("slope-soil"));
}

#[test]
fn cli03_mofe02_rejects_management_soil_mismatch() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-mofe02-management-soil-mismatch"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let error = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_mofe02_management_soil_mismatch",
        false,
        |run_dir| {
            write_two_ofe_slope(&run_dir.join("case.slp"));
            write_soil_with_ntemp(&run_dir.join("case.sol"), 2);
        },
    )
    .expect_err("management/soil topology mismatch should fail before runtime merge");

    assert_eq!(error.code(), "CLIHILL-E-019");
    let message = error.to_string();
    assert!(message.contains("management-soil"));
}

#[test]
fn cli03_mofe02_rejects_full_triad_topology_mismatch() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-mofe02-triad-mismatch"
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

    let error = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_mofe02_triad_mismatch",
        false,
        |run_dir| {
            write_two_ofe_slope(&run_dir.join("case.slp"));
            write_soil_with_ntemp(&run_dir.join("case.sol"), 3);
        },
    )
    .expect_err("triad topology mismatch should fail before runtime merge");

    assert_eq!(error.code(), "CLIHILL-E-019");
    let message = error.to_string();
    assert!(message.contains("slope-management"));
    assert!(message.contains("slope-soil"));
    assert!(message.contains("management-soil"));
    assert!(message.contains("slope=2"));
    assert!(message.contains("management=1"));
    assert!(message.contains("soil=3"));
}

#[test]
fn cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-mofe03-wave2-enabled"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_mofe03_wave2_enabled",
        false,
        |run_dir| {
            let _ = fs::remove_file(run_dir.join("wepp_ui.txt"));
            write_three_ofe_slope(&run_dir.join("case.slp"));
            write_soil_with_ntemp_low_conductivity(&run_dir.join("case.sol"), 3);
            write_high_runoff_climate(&run_dir.join("case.cli"));
            write_three_ofe_management(&run_dir.join("case.man"));
        },
    )
    .expect("aligned multi-OFE fixture should execute through Wave-2");

    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());

    let manifest =
        fs::read_to_string(&report.manifest_path).expect("manifest file should be readable");
    assert!(
        manifest.contains("\"erod14_wave2_enabled\": true"),
        "multi-OFE run should enable Wave-2 under MOFE03 policy, observed manifest: {manifest}"
    );
    assert!(
        manifest.contains("\"erod14_wave2_kernel_status_seen\": true"),
        "multi-OFE run should observe Wave-2 kernel status, observed manifest: {manifest}"
    );
    assert!(
        manifest.contains(
            "\"erod14_qin_source_policy\": \"water-transfer-only-mofe01-mg-sediment-coupling-follow-on\""
        ),
        "multi-OFE manifest should expose M-G qin source policy, observed manifest: {manifest}"
    );
    assert!(
        manifest.contains("\"erod14_qin_sediment_coupled\": false"),
        "multi-OFE manifest should not claim sediment-coupled qin closure, observed manifest: {manifest}"
    );
}

#[test]
fn cli03_mofe03_single_ofe_policy_disables_wave2_by_default() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-mofe03-wave2-disabled"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_mofe03_wave2_disabled",
        false,
        |run_dir| {
            let _ = fs::remove_file(run_dir.join("wepp_ui.txt"));
        },
    )
    .expect("single-OFE baseline fixture should execute");

    let manifest =
        fs::read_to_string(&report.manifest_path).expect("manifest file should be readable");
    assert!(
        manifest.contains("\"erod14_wave2_enabled\": false"),
        "single-OFE policy should keep Wave-2 disabled, observed manifest: {manifest}"
    );
    assert!(
        manifest.contains("\"erod14_wave2_kernel_status_seen\": false"),
        "single-OFE policy should keep Wave-2 kernel status absent, observed manifest: {manifest}"
    );
    assert!(
        manifest.contains("\"erod14_qin_source_policy\": \"wave2-disabled\"")
            && manifest.contains("\"erod14_qin_sediment_coupled\": false"),
        "single-OFE manifest should expose disabled M-G qin policy without coupling claim, observed manifest: {manifest}"
    );
}

#[test]
fn cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-mofe04-publication-multiofe"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
wat = "output/H1.wat.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_mofe04_publication_multiofe",
        false,
        |run_dir| {
            let _ = fs::remove_file(run_dir.join("wepp_ui.txt"));
            write_three_ofe_slope(&run_dir.join("case.slp"));
            write_soil_with_ntemp_low_conductivity(&run_dir.join("case.sol"), 3);
            write_high_runoff_climate(&run_dir.join("case.cli"));
            write_three_ofe_management(&run_dir.join("case.man"));
        },
    )
    .expect("aligned multi-OFE fixture should execute");

    let manifest =
        fs::read_to_string(&report.manifest_path).expect("manifest file should be readable");
    let manifest_json: Value =
        serde_json::from_str(&manifest).expect("manifest file should parse as JSON");
    assert_mf_multiofe_publication_manifest(&manifest_json, &manifest);

    let wat_output = report
        .optional_outputs
        .iter()
        .find(|path| path.file_name().and_then(|name| name.to_str()) == Some("H1.wat.parquet"))
        .expect("WAT output should be present");
    assert_mf_multiofe_publication_wat_rows(wat_output);
}

fn assert_mf_multiofe_publication_manifest(manifest_json: &Value, manifest: &str) {
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/publication_ofe_policy")
            .and_then(Value::as_str),
        Some("per-ofe-dynamic-water-balance-state"),
        "manifest missing M-F publication policy marker: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/area_policy")
            .and_then(Value::as_str),
        Some("sum-ofe-geometry-area"),
        "manifest missing MOFE04 area policy marker: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/contributor_ofe_count")
            .and_then(Value::as_u64),
        Some(3),
        "manifest missing M-F contributor OFE count marker: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/publication_area_m2")
            .and_then(Value::as_f64),
        Some(3600.0),
        "manifest missing MOFE04 publication area marker for multi-OFE case: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/storage_lineage_policy")
            .and_then(Value::as_str),
        Some("per-ofe-dynamic-wb-state"),
        "manifest missing M-F storage-lineage policy marker: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/per_ofe_state_policy")
            .and_then(Value::as_str),
        Some("published-per-ofe-wb13-records"),
        "manifest missing M-F per-OFE state policy marker: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/row_count")
            .and_then(Value::as_u64),
        Some(6),
        "manifest missing M-F public per-OFE row_count marker: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/per_ofe_record_count")
            .and_then(Value::as_u64),
        Some(6),
        "manifest missing M-F per-OFE record count marker: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/per_ofe_expected_record_count")
            .and_then(Value::as_u64),
        Some(6),
        "manifest missing M-F expected per-OFE record count marker: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/sim_day_index_monotonic")
            .and_then(Value::as_bool),
        Some(true),
        "manifest must mark grouped per-OFE day/OFE keys monotonic: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/first_row_key/ofe")
            .and_then(Value::as_u64),
        Some(1),
        "manifest first public per-OFE row must be OFE 1: {manifest}"
    );
    assert_eq!(
        manifest_json
            .pointer("/wb13_publication/last_row_key/ofe")
            .and_then(Value::as_u64),
        Some(3),
        "manifest last public per-OFE row must be outlet OFE 3: {manifest}"
    );
    assert_mf_multiofe_publication_carry_manifest(manifest);
}

fn assert_mf_multiofe_publication_carry_manifest(manifest: &str) {
    for expected in [
        "\"mofe_hourly_carry\"",
        "\"policy\": \"baseline-wathour-24-slot-copy-forward\"",
        "\"active\": true",
        "\"substep_count\": 24",
        "\"ui_SUrunf\"",
        "\"ui_SCrunf\"",
        "\"ui_LfUrf\"",
        "\"ui_LfCrf\"",
        "\"upstream_carry_total_m\"",
        "\"current_carry_total_m\"",
    ] {
        assert!(
            manifest.contains(expected),
            "manifest missing HPHYS0241 MOFE carry marker {expected}: {manifest}"
        );
    }
}

fn assert_mf_multiofe_publication_wat_rows(wat_output: &Path) {
    let wat_rows = read_parquet_rows(wat_output);
    assert_eq!(
        wat_rows.len(),
        6,
        "M-F multi-OFE WAT publication must emit days * nofe rows"
    );
    let ofe_ids = wat_rows
        .iter()
        .map(|row| row_i32_value(row, "ofe_id"))
        .collect::<Vec<_>>();
    assert_eq!(ofe_ids, vec![1, 2, 3, 1, 2, 3]);
    let ofe_column = wat_rows
        .iter()
        .map(|row| row_i32_value(row, "OFE"))
        .collect::<Vec<_>>();
    assert_eq!(ofe_column, ofe_ids);
    let sim_day_indices = wat_rows
        .iter()
        .map(|row| row_i32_value(row, "sim_day_index"))
        .collect::<Vec<_>>();
    assert_eq!(sim_day_indices, vec![1, 1, 1, 2, 2, 2]);
    assert_mf_multiofe_publication_surface_handoff(&wat_rows[0..3]);
    assert_mfredo2_qofe_local_depth_geometry(&wat_rows[0..3]);
    assert_mf_multiofe_publication_not_cloned(&wat_rows[0..3]);
}

fn assert_mf_multiofe_publication_surface_handoff(day_rows: &[Row]) {
    assert_eq!(
        day_rows.len(),
        3,
        "M-F active routed-day fixture must expose one row per OFE"
    );
    for ofe_offset in 1..day_rows.len() {
        let upstream_qofe = row_f64_value(&day_rows[ofe_offset - 1], "QOFE");
        let downstream_upstrmq = row_f64_value(&day_rows[ofe_offset], "UpStrmQ");
        assert!(
            upstream_qofe > 1.0e-9,
            "M-F-REDO requires nonzero upstream QOFE on active handoff rows"
        );
        assert!(
            downstream_upstrmq > 1.0e-9,
            "M-F-REDO requires nonzero downstream UpStrmQ on active handoff rows"
        );
        assert!(
            (downstream_upstrmq - upstream_qofe).abs() <= 1.0e-6,
            "downstream UpStrmQ ({downstream_upstrmq}) must equal previous OFE QOFE ({upstream_qofe})"
        );
    }
}

fn assert_mfredo2_qofe_local_depth_geometry(day_rows: &[Row]) {
    let expected_qofe_to_q_ratios = [1.0, 2.5, 6.0];
    for (row, expected_ratio) in day_rows.iter().zip(expected_qofe_to_q_ratios) {
        let ofe = row_i32_value(row, "OFE");
        let q = row_f64_value(row, "Q");
        let qofe = row_f64_value(row, "QOFE");
        if qofe <= 1.0e-9 {
            continue;
        }
        assert!(
            q > 1.0e-9,
            "M-F-REDO2 expects positive cumulative Q when OFE {ofe} has positive QOFE"
        );
        let ratio = qofe / q;
        assert!(
            (ratio - expected_ratio).abs() <= 1.0e-6,
            "M-F-REDO2 QOFE/Q ratio for OFE {ofe} must reflect local vs cumulative length normalization; expected {expected_ratio}, observed {ratio}"
        );
    }
    assert!(
        (row_f64_value(&day_rows[1], "QOFE") - row_f64_value(&day_rows[1], "Q")).abs() > 1.0e-9,
        "M-F-REDO2 requires downstream public QOFE to stop aliasing cumulative public Q"
    );
}

fn assert_mf_multiofe_publication_not_cloned(day_rows: &[Row]) {
    const HYDROLOGY_COLUMNS: [&str; 8] = [
        "Q",
        "QOFE",
        "UpStrmQ",
        "Es",
        "Ep",
        "Dp",
        "Total-Soil",
        "SoilWaterTotal",
    ];
    let first_vector = hydrology_vector(&day_rows[0], &HYDROLOGY_COLUMNS);
    let all_identical = day_rows[1..].iter().all(|row| {
        hydrology_vector(row, &HYDROLOGY_COLUMNS)
            .iter()
            .zip(&first_vector)
            .all(|(observed, expected)| (observed - expected).abs() <= 1.0e-9)
    });
    assert!(
        !all_identical,
        "M-F-REDO anti-clone gate: all OFE hydrology vectors are identical for active routed day"
    );
}

fn hydrology_vector(row: &Row, columns: &[&str]) -> Vec<f64> {
    columns
        .iter()
        .map(|column| row_f64_value(row, column))
        .collect()
}

#[test]
fn cli03_mofe04_single_ofe_publication_reports_single_contributor_policy() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-mofe04-publication-singleofe"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_mofe04_publication_singleofe",
        false,
        |run_dir| {
            let _ = fs::remove_file(run_dir.join("wepp_ui.txt"));
        },
    )
    .expect("single-OFE fixture should execute");

    let manifest =
        fs::read_to_string(&report.manifest_path).expect("manifest file should be readable");
    assert!(
        manifest.contains(
            "\"publication_ofe_policy\": \"single-row-canonicalized-hillslope-aggregate\""
        ),
        "single-OFE manifest should remain on aggregate publication policy: {manifest}"
    );
    assert!(
        manifest.contains("\"contributor_ofe_count\": 1"),
        "manifest missing MOFE04 contributor OFE count marker: {manifest}"
    );
    assert!(
        manifest.contains("\"publication_area_m2\": 1800.0"),
        "manifest missing MOFE04 publication area marker for single-OFE case: {manifest}"
    );
    assert!(
        manifest.contains("\"mofe_hourly_carry\"")
            && manifest.contains("\"policy\": \"baseline-wathour-24-slot-copy-forward\"")
            && manifest.contains("\"active\": false"),
        "single-OFE manifest must publish inactive HPHYS0241 carry metadata: {manifest}"
    );
}

#[test]
fn cli03_hphys0241_watershed_validator_requires_mofe_hourly_carry_metadata() {
    for expected in [
        "mofe_hourly_carry",
        "baseline-wathour-24-slot-copy-forward",
        "ui_SUrunf",
        "ui_SCrunf",
        "ui_LfUrf",
        "ui_LfCrf",
        "substep_count",
        "upstream_carry_total_m",
        "current_carry_total_m",
    ] {
        assert!(
            WATERSHED_CLI_SOURCE.contains(expected),
            "watershed CLI source must validate HPHYS0241 metadata marker {expected}"
        );
    }
}

#[test]
fn cli03_runtime_accepts_finite_daily_temperature_inversion_records() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli03-temperature-inversion-compatibility"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = false

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
wat = "output/H1.wat.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile,
        "cli03_temperature_inversion_compatibility",
        false,
        |run_dir| {
            write_temperature_inversion_climate(&run_dir.join("case.cli"));
        },
    )
    .expect("finite tmax<tmin climate records should remain compatibility-valid");

    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());
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
    execute_fixture_with_runfile_report_with_mode_and_customizer(
        runfile_payload,
        prefix,
        legacy_sidecar_discovery,
        |_| {},
    )
}

fn execute_fixture_with_runfile_report_with_mode_and_customizer(
    runfile_payload: &str,
    prefix: &str,
    legacy_sidecar_discovery: bool,
    customizer: impl FnOnce(&Path),
) -> Result<(HillslopeRunReport, PathBuf), HillslopeCliError> {
    let _execution_guard = runner_execution_lock()
        .lock()
        .expect("runner execution lock should be acquirable");

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
    customizer(&temp_run_dir);

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

fn write_single_ofe_slope(path: &Path) {
    let payload = "\
# Canonical one-OFE slope profile
97.5
1
180.0 30.0
3 60.0
0.0 0.0200 0.6 0.0800 1.0 0.0600
";
    fs::write(path, payload).expect("single-OFE slope fixture should be writable");
}

fn write_two_ofe_slope(path: &Path) {
    let payload = "\
# Canonical two-OFE slope profile
97.5
2
180.0 30.0
3 60.0
0.0 0.0200 0.6 0.0800 1.0 0.0600
180.0 30.0
3 40.0
0.0 0.0600 0.5 0.0400 1.0 0.0300
";
    fs::write(path, payload).expect("two-OFE slope fixture should be writable");
}

fn write_three_ofe_slope(path: &Path) {
    let payload = "\
# Canonical three-OFE slope profile
97.5
3
180.0 30.0
3 60.0
0.0 0.0200 0.6 0.0800 1.0 0.0600
180.0 30.0
3 40.0
0.0 0.0600 0.5 0.0400 1.0 0.0300
180.0 30.0
3 20.0
0.0 0.0300 0.5 0.0200 1.0 0.0100
";
    fs::write(path, payload).expect("three-OFE slope fixture should be writable");
}

fn write_soil_with_ntemp(path: &Path, ntemp: usize) {
    assert!(ntemp > 0, "ntemp must remain positive");
    let mut payload = format!("9002\nDisturbed soil profile\n{ntemp} 1\n");
    let ofe_block = "\
SOIL_B CLAY_LOAM 2 0.20 0.55 900000 0.005 4.2 10.5
1 forest silt_loam 0.20 0.001
100 1.25 15.0 1.20 0.30 0.15 35 25 2.0 15 5 0.05 0.45 0.02 1.40 120 0.16 0.31
250 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30
";
    for _ in 0..ntemp {
        payload.push_str(ofe_block);
    }
    payload.push_str("1 500 0.8\n");
    fs::write(path, payload).expect("soil fixture should be writable");
}

fn write_soil_with_ntemp_low_conductivity(path: &Path, ntemp: usize) {
    assert!(ntemp > 0, "ntemp must remain positive");
    let mut payload = format!("9002\nDisturbed soil profile\n{ntemp} 1\n");
    let ofe_block = "\
SOIL_B CLAY_LOAM 2 0.20 0.55 900000 0.005 4.2 10.5
1 forest silt_loam 0.20 0.001
100 1.25 0.05 1.20 0.30 0.15 35 25 2.0 15 5 0.05 0.45 0.02 1.40 120 0.16 0.31
250 1.30 0.02 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30
";
    for _ in 0..ntemp {
        payload.push_str(ofe_block);
    }
    payload.push_str("1 500 0.8\n");
    fs::write(path, payload).expect("low-conductivity soil fixture should be writable");
}

fn write_high_runoff_climate(path: &Path) {
    let payload = "\
5.30
1 0 0
TEST STATION 0001
DAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT
45.0 -120.0 1000.0 30 2000 1 CLIGEN 5.30 --seed 123
MONTHLY MAX TEMP HEADER
1 2 3 4 5 6 7 8 9 10 11 12
MONTHLY MIN TEMP HEADER
-5 -4 -3 -2 -1 0 1 2 3 4 5 6
MONTHLY RAD HEADER
100 101 102 103 104 105 106 107 108 109 110 111
MONTHLY RAIN HEADER
10 11 12 13 14 15 16 17 18 19 20 21
DAILY HEADER
DAILY UNITS
1 1 2000 120.0 0.25 0.10 8.0 12.0 2.0 200.0 3.0 180.0 -1.0
2 1 2000 0.0 0.0 0.0 0.0 10.0 1.0 190.0 2.5 170.0 -2.0
";
    fs::write(path, payload).expect("high-runoff climate fixture should be writable");
}

fn write_temperature_inversion_climate(path: &Path) {
    let payload = "\
5.30
1 0 0
TEST STATION 0001
DAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT
45.0 -120.0 1000.0 30 2000 1 CLIGEN 5.30 --seed 123
MONTHLY MAX TEMP HEADER
1 2 3 4 5 6 7 8 9 10 11 12
MONTHLY MIN TEMP HEADER
-5 -4 -3 -2 -1 0 1 2 3 4 5 6
MONTHLY RAD HEADER
100 101 102 103 104 105 106 107 108 109 110 111
MONTHLY RAIN HEADER
10 11 12 13 14 15 16 17 18 19 20 21
DAILY HEADER
DAILY UNITS
1 1 2000 10.0 2.0 0.25 3.0 11.3 11.4 200.0 3.0 180.0 -1.0
2 1 2000 0.0 0.0 0.0 0.0 10.0 1.0 190.0 2.5 170.0 -2.0
";
    fs::write(path, payload).expect("temperature-inversion climate fixture should be writable");
}

fn write_three_ofe_management(path: &Path) {
    let payload = fs::read_to_string(infile_fixture_path(
        "management/canonical_rotation_nonzero_98_4.man",
    ))
    .expect("three-OFE management fixture should be readable");
    fs::write(path, payload).expect("three-OFE management fixture should be writable");
}

fn read_parquet_rows(path: &Path) -> Vec<Row> {
    let file = File::open(path).unwrap_or_else(|error| {
        panic!(
            "parquet output should be readable ({}): {error}",
            path.display()
        )
    });
    let reader = SerializedFileReader::new(file).unwrap_or_else(|error| {
        panic!("parquet output should parse ({}): {error}", path.display())
    });
    reader
        .get_row_iter(None)
        .unwrap_or_else(|error| {
            panic!(
                "parquet row iterator should open ({}): {error}",
                path.display()
            )
        })
        .map(|row| {
            row.unwrap_or_else(|error| {
                panic!("parquet row should decode ({}): {error}", path.display())
            })
        })
        .collect()
}

fn row_index(row: &Row, column_name: &str) -> usize {
    row.get_column_iter()
        .enumerate()
        .find(|(_, (name, _))| name.as_str() == column_name)
        .map_or_else(
            || panic!("missing required parquet column '{column_name}'"),
            |(index, _)| index,
        )
}

fn row_i32_value(row: &Row, column_name: &str) -> i32 {
    let index = row_index(row, column_name);
    if let Ok(value) = row.get_byte(index) {
        return i32::from(value);
    }
    if let Ok(value) = row.get_short(index) {
        return i32::from(value);
    }
    if let Ok(value) = row.get_int(index) {
        return value;
    }
    if let Ok(value) = row.get_long(index) {
        return i32::try_from(value)
            .unwrap_or_else(|_| panic!("column '{column_name}' value {value} out of i32 range"));
    }
    panic!("column '{column_name}' does not decode as integer");
}

fn row_f64_value(row: &Row, column_name: &str) -> f64 {
    let index = row_index(row, column_name);
    if let Ok(value) = row.get_double(index) {
        return value;
    }
    if let Ok(value) = row.get_float(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_int(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_short(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_long(index) {
        let value = i32::try_from(value).unwrap_or_else(|_| {
            panic!("column '{column_name}' value {value} out of f64-safe range")
        });
        return f64::from(value);
    }
    panic!("column '{column_name}' does not decode as numeric");
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

fn infile_fixture_path(relative_path: &str) -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("fixtures")
        .join("infile")
        .join(relative_path)
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

fn runner_src_tree_contains(needle: &str) -> bool {
    let source_root = Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("../crates/openwepp-runner/src");
    source_tree_contains_rs(&source_root, needle)
}

fn source_tree_contains_rs(root: &Path, needle: &str) -> bool {
    fs::read_dir(root)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .any(|entry| {
            let path = entry.path();
            if path.is_dir() {
                return source_tree_contains_rs(&path, needle);
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                return false;
            }
            fs::read_to_string(path).is_ok_and(|contents| contents.contains(needle))
        })
}
