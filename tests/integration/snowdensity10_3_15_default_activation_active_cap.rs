use std::fs;

use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = concat!(
    "docs/work-packages/20260627-snowdensity-10-3-15-default-activation-active-cap-001/",
    "package.md"
);
const BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00_builders_and_authority.rs"
);
const AUTHORITY_IMPL: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00a_snow_frost_authority_impl.rs"
);
const CLI: &str = "crates/openwepp-runner/src/bin/openwepp-cli-hill.rs";
const TOOL: &str = "tools/snowfreeze_observed/default_activation_active_cap.py";
const REPORT: &str = concat!(
    "docs/work-packages/20260627-snowdensity-10-3-15-default-activation-active-cap-001/",
    "artifacts/default-activation-active-cap.json"
);

#[test]
fn contract_and_package_bind_default_activation_authority() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 104",
        "REF-SNOWFREEZE-SNOWDENSITY1015",
        "INV-SNOWFREEZE-072",
        "OBL-SNOWFREEZE-P-047",
        "SNOWDENSITY-10.3.15 Default Activation Under Active Cap Addendum",
        "Empty selector values select the activated defaults",
        "The rejected `physics_bulk_spring_densification_v1`",
        "`498/1415` paired snow-depth residual failures",
        "SNOW-CONTROL-RESIDUALS-REMAIN",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.15 Default Activation Under Active Cap",
        "Activate the validated active-cap snow-depth bundle",
        "explicit rollback/test selectors",
        "Rejected spring densification is not accepted by the active default selector",
        "No Qwet/frzftp implementation or selector",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn implementation_selects_activated_default_without_user_surface() {
    let builder = read(BUILDER);
    for marker in [
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "snowdensity1015_default_snow_density_model",
        "snowdensity1015_default_snow_melt_model",
        "Err(std::env::VarError::NotPresent)",
        "SnowDensityModel::PhysicsBulkDensityCompactionV1",
        "SnowMeltModel::CoeLiquidHoldingCapacityV1",
        "must be legacy_wepp, physics_bulk_density_compaction_v1, or physics_bulk_shallow_guard_v1",
        "must be legacy_coe, coe_liquid_holding_capacity_v1, or coe_open_sublimation_stage_a_v1",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }
    assert!(
        !builder.contains("SnowDensityModel::PhysicsBulkSpringDensificationV1"),
        "active default selector must not accept rejected spring densification"
    );

    let authority = read(AUTHORITY_IMPL);
    for marker in [
        "snow_density_model: snowdensity1015_default_snow_density_model()?",
        "snow_melt_model: snowdensity1015_default_snow_melt_model()?",
    ] {
        assert_contains(&authority, marker, AUTHORITY_IMPL);
    }

    let cli = read(CLI);
    assert!(
        !cli.contains("SNOWDENSITY09")
            && !cli.contains("SNOWDENSITY1038")
            && !cli.contains("snow-density-model")
            && !cli.contains("snow-melt-model"),
        "activation must not expose parser/runfile/user CLI selector surfaces"
    );
}

#[test]
fn diagnostic_tool_proves_no_env_default_and_rollback_boundaries() {
    let tool = read(TOOL);
    for marker in [
        "snowdensity10-3-15-default-activation-active-cap-v1",
        "INV-SNOWFREEZE-072 OBL-SNOWFREEZE-P-047",
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "OPENWEPP_R7H_SNOW_TRACE_PATH",
        "coe_liquid_holding_capacity_v1",
        "physics_bulk_density_compaction_v1",
        "legacy_coe",
        "legacy_wepp",
        "EXPECTED_POLICY_B_FAILURES = 498",
        "parser_runfile_user_cli_selector_added",
        "qwet_or_frzftp_changed",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert_contains(&tool, "subprocess.run", TOOL);
}

#[test]
fn executed_report_records_default_activation_and_residual_blocker() {
    let report: Value =
        serde_json::from_str(&read(REPORT)).expect("default activation report should parse");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-15-default-activation-active-cap-v1"
    );
    assert_eq!(
        report["activated_default"]["snow_melt_model"],
        "coe_liquid_holding_capacity_v1"
    );
    assert_eq!(
        report["activated_default"]["snow_density_model"],
        "physics_bulk_density_compaction_v1"
    );
    assert_eq!(report["summary"]["activation_complete"], true);
    assert_eq!(report["summary"]["paired_row_count"], 1415);
    assert_eq!(report["summary"]["snow_control_fail_count"], 498);
    assert_eq!(report["summary"]["frost_attribution_unblocked"], false);
    assert_eq!(
        report["summary"]["frost_attribution_blocker"],
        "SNOW-CONTROL-RESIDUALS-REMAIN"
    );
    assert_eq!(
        report["rollback"]["snow_melt_model"], "legacy_coe",
        "legacy melt rollback must remain explicit"
    );
    assert_eq!(
        report["rollback"]["snow_density_model"], "legacy_wepp",
        "legacy density rollback must remain explicit"
    );
    assert_eq!(report["protected_boundaries"]["density_cap_changed"], false);
    assert_eq!(
        report["protected_boundaries"]["public_output_schema_changed"],
        false
    );
    assert!(
        report["activated_default"]["trace_proof"]["expected_snow_melt_model_count"]
            .as_u64()
            .expect("melt trace count")
            > 0
    );
    assert!(
        report["activated_default"]["trace_proof"]["expected_snow_density_model_count"]
            .as_u64()
            .expect("density trace count")
            > 0
    );
    assert!(
        report["rollback"]["trace_proof"]["expected_snow_melt_model_count"]
            .as_u64()
            .expect("rollback melt trace count")
            > 0
    );
    assert!(
        report["rollback"]["trace_proof"]["expected_snow_density_model_count"]
            .as_u64()
            .expect("rollback density trace count")
            > 0
    );
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}
