use std::fs;

use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = concat!(
    "docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/",
    "package.md"
);
const BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00c_day_input_builder_impl.rs"
);
const STATIC_AUTHORITY_BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00_builders_and_authority.rs"
);
const AUTHORITY_IMPL: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00a_snow_frost_authority_impl.rs"
);
const CLI: &str = "crates/openwepp-runner/src/bin/openwepp-cli-hill.rs";
const TOOL: &str = "tools/snowfreeze_observed/harder_pomeroy_default_activation.py";
const REPORT: &str = concat!(
    "docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/",
    "artifacts/harder-pomeroy-default-activation.json"
);

#[test]
fn contract_and_package_bind_harder_pomeroy_default_activation() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 128",
        "REF-SNOWFREEZE-SNOWDENSITY1019",
        "INV-SNOWFREEZE-075",
        "OBL-SNOWFREEZE-P-050",
        "Harder-Pomeroy direct-production phase default",
        "absent, or empty selector values select the activated phase default",
        "`OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=legacy_rst` remains an explicit rollback/test selector",
        "15` robust fails and `179` robust score",
        "17` robust fails and `172` robust score",
        "The `.run` disable option is not authorized",
        "+23.6 kg m^-3",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.19 Harder-Pomeroy Default Activation",
        "Cross-SNOTEL forcing-robust rubric",
        "explicit `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=legacy_rst`",
        "No fixture, public output-schema, density-cap, frost",
        "No site calibration",
        "No activation of 10.3.16 sublimation or 10.3.17 shallow-pack guard",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn implementation_selects_harder_pomeroy_without_env_and_preserves_rollback() {
    let builder = format!("{}\n{}", read(BUILDER), read(STATIC_AUTHORITY_BUILDER));
    for marker in [
        "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL",
        "\"\" | \"harder_pomeroy_hourly\"",
        "\"legacy_rst\"",
        "SnowPhasePartitionModel::HarderPomeroyHourly",
        "SnowPhasePartitionModel::LegacyRst",
        "Err(std::env::VarError::NotPresent)",
        "must be legacy_rst, harder_pomeroy_hourly, or empty default",
        "\\\"snow_phase_model\\\":\\\"{}\\\"",
        "snow_phase_model.id()",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }

    assert_contains(
        &builder,
        "snow_phase_model: snowdensity1035_diagnostic_snow_phase_model()?",
        BUILDER,
    );

    let authority = read(AUTHORITY_IMPL);
    assert_contains(
        &authority,
        "snow_phase_model: self.snow_phase_model",
        AUTHORITY_IMPL,
    );
    assert_contains(
        &authority,
        "SnowPhasePartitionModel::LegacyRst",
        AUTHORITY_IMPL,
    );

    let cli = read(CLI);
    assert!(
        !cli.contains("SNOWDENSITY1035")
            && !cli.contains("snow-phase")
            && !cli.contains("harder_pomeroy"),
        "phase default must not expose parser/runfile/user CLI selector surfaces"
    );
}

#[test]
fn activation_tool_is_policy_b_bound_and_not_a_runfile_disable_surface() {
    let tool = read(TOOL);
    for marker in [
        "snowdensity10-3-19-harder-pomeroy-default-activation-v1",
        "INV-SNOWFREEZE-075 OBL-SNOWFREEZE-P-050",
        "INV-SNOWFREEZE-050",
        "activated_bundle",
        "harder_pomeroy_default",
        "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL",
        "legacy_rst",
        "harder_pomeroy_hourly",
        "cross_snotel_gate_pass",
        "partition_conservation_gate_pass",
        "source_hourly_reconstruction_guard",
        "run_file_disable_option_added",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert!(
        !tool.contains("--runfile-disable")
            && !tool.contains("--run-file-disable")
            && !tool.contains("--disable-phase")
            && !tool.contains("runfile_disable_enabled"),
        "10.3.19 must not add the separate .run disable option"
    );
}

#[test]
fn executed_report_records_activation_gates_and_release_notes() {
    let report: Value = serde_json::from_str(&read(REPORT)).expect("10.3.19 report should parse");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-19-harder-pomeroy-default-activation-v1"
    );
    assert_eq!(report["summary"]["cross_snotel_gate_pass"], true);
    assert_eq!(report["summary"]["selector_trace_gate_pass"], true);
    assert_eq!(report["summary"]["partition_conservation_gate_pass"], true);
    assert_eq!(report["summary"]["activation_complete"], true);
    assert!(
        report["summary"]["new_default_robust_fail_count"]
            .as_i64()
            .unwrap()
            <= report["summary"]["prior_activated_bundle_robust_fail_count"]
                .as_i64()
                .unwrap()
    );
    assert!(
        report["summary"]["new_default_robust_ordinal_score"]
            .as_i64()
            .unwrap()
            >= report["summary"]["prior_activated_bundle_robust_ordinal_score"]
                .as_i64()
                .unwrap()
    );
    assert_eq!(
        report["trace_proof"]["harder_pomeroy_default"]["expected"]["snow_phase_model"],
        "harder_pomeroy_hourly"
    );
    assert_eq!(
        report["trace_proof"]["activated_bundle"]["expected"]["snow_phase_model"],
        "legacy_rst"
    );
    assert_eq!(
        report["release_notes"]["humid_new_england_depth_regression"],
        "roadmap item; non-representative gate, not blocker"
    );
    assert_eq!(
        report["release_notes"]["density_bias_recovery_tracked_separately"],
        true
    );
    assert_eq!(
        report["protected_boundaries"]["run_file_disable_option_added"],
        false
    );
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(haystack: &str, needle: &str, context: &str) {
    assert!(
        haystack.contains(needle),
        "{context} missing required marker: {needle}"
    );
}
