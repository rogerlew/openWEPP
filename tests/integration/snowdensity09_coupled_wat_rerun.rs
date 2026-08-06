use std::fs;
use std::path::Path;

use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-09-diagnostic-coupled-wat-rerun-001/package.md";
const SCRIPT: &str = "tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py";
const BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00c_day_input_builder_impl.rs"
);
const AUTHORITY_IMPL: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00a_snow_frost_authority_impl.rs"
);
const REPORT: &str = concat!(
    "docs/work-packages/20260626-snowdensity-09-diagnostic-coupled-wat-rerun-001/",
    "artifacts/snowdensity09_coupled_wat_rerun.json"
);
const CLI: &str = "crates/openwepp-runner/src/bin/openwepp-cli-hill.rs";

#[test]
fn snowdensity09_contract_and_package_authorize_diagnostic_coupled_wat() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 129",
        "INV-SNOWFREEZE-062",
        "OBL-SNOWFREEZE-P-037",
        "SNOWDENSITY-09 Diagnostic Coupled WAT Rerun Addendum",
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "WAT `Snow-Depth` remains the publication of",
        "diagnostic-only out-of-gate evidence",
        "not counted as pass, fail, or blocker",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-09 Diagnostic Coupled WAT Rerun",
        "No production parser/runfile/user CLI selector",
        "No WAT rewriting",
        "Diagnostic opt-in WAT runs set the exact package-bound selector",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn snowdensity09_selector_is_now_10_3_15_default_with_legacy_rollback() {
    let cli = read(CLI);
    assert!(
        !cli.contains("SNOWDENSITY09")
            && !cli.contains("snow-density-model")
            && !cli.contains("physics_bulk_density_compaction_v1"),
        "openwepp-cli-hill must not expose a user CLI density selector"
    );

    let builder = read(BUILDER);
    for marker in [
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "snowdensity1015_default_snow_density_model",
        "SnowDensityModel::LegacyWepp",
        "SnowDensityModel::PhysicsBulkDensityCompactionV1",
        "must be legacy_wepp, physics_bulk_density_compaction_v1, physics_bulk_shallow_guard_v1, physics_bulk_climate_class_density_v1, or physics_bulk_multilayer_density_v1",
        "\\\"snow_density_model\\\":\\\"{}\\\"",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }
    assert!(
        !builder.contains("SnowDensityModel::PhysicsBulkSpringDensificationV1"),
        "rejected spring densification must not remain accepted by the active default selector"
    );

    let authority_impl = read(AUTHORITY_IMPL);
    assert_contains(
        &authority_impl,
        "snow_density_model: self.snow_density_model",
        AUTHORITY_IMPL,
    );
}

#[test]
fn snowdensity09_script_preserves_coupled_path_and_report_contract() {
    let script = read(SCRIPT);
    for marker in [
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "OPENWEPP_R7H_SNOW_TRACE_PATH",
        "non_snotel_physics_bulk_density_compaction_v1",
        "coupled_opt_in_wat_path_available",
        "parser_runfile_user_cli_activation_added",
        "frost_attribution_authorized",
        "snow_control_gate_status_counts",
        "snow_control_gate_passed",
        "snow_control_out_of_gate_site_ids",
    ] {
        assert_contains(&script, marker, SCRIPT);
    }
}

#[test]
fn snowdensity09_executed_report_is_truthful_if_present() {
    if !Path::new(REPORT).is_file() {
        return;
    }
    let report: Value = serde_json::from_str(&read(REPORT)).expect("SNOWDENSITY-09 report parses");
    assert_eq!(
        report["schema"],
        "snowdensity09-diagnostic-coupled-wat-rerun-v1"
    );
    assert_eq!(report["summary"]["production_physics_changed"], false);
    assert_eq!(report["summary"]["default_activation_changed"], false);
    assert_eq!(
        report["summary"]["parser_runfile_user_cli_activation_added"],
        false
    );
    assert_eq!(
        report["diagnostic_selector"]["opt_in_value"],
        "physics_bulk_density_compaction_v1"
    );
    assert_eq!(report["summary"]["opt_in_snow_control_passed"], false);
    assert_eq!(
        report["opt_in_non_snotel"]["summary"]["snow_control_gate_status_counts"]["SNOW_CONTROL_FAILED"],
        3
    );
    assert!(
        report["opt_in_non_snotel"]["summary"]["snow_control_gate_status_counts"]
            ["MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW"]
            .is_null(),
        "no-observed-snow sites must not participate in the snow-control gate"
    );
    assert_eq!(
        report["summary"]["opt_in_snow_control_out_of_gate_site_ids"],
        serde_json::json!(["site3_scan_mandan_nd", "site5_reynolds_creek_us_rls_id"])
    );
    assert!(
        report["diagnostic_selector"]["trace_proof"]["opt_in_trace_selected_count"]
            .as_i64()
            .unwrap_or(0)
            > 0,
        "executed report must prove the opt-in model appeared in direct snow trace"
    );
}

fn read(path: &str) -> String {
    fs::read_to_string(Path::new(path)).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}
