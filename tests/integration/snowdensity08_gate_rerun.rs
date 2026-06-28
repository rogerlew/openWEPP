use std::fs;
use std::path::Path;

use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-08-snow-frost-gate-rerun-001/package.md";
const SCRIPT: &str = "tools/snowfreeze_observed/snowdensity08_gate_rerun.py";
const REPORT: &str = concat!(
    "docs/work-packages/20260626-snowdensity-08-snow-frost-gate-rerun-001/",
    "artifacts/snowdensity08_gate_rerun.json"
);
const DIRECT_PUBLICATION_BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00_builders_and_authority.rs"
);
const CLI: &str = "crates/openwepp-runner/src/bin/openwepp-cli-hill.rs";

#[test]
fn snowdensity08_contract_and_package_bind_gate_rerun_authority() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 107",
        "INV-SNOWFREEZE-061",
        "OBL-SNOWFREEZE-P-036",
        "SNOWDENSITY-08 Snow/Frost Gate Rerun Addendum",
        "frost_attribution_authorized",
        "coupled opt-in WAT/publication run",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-08 Snow/Frost Gate Rerun",
        "No parser/runfile/user CLI density selector",
        "No WAT rewriting",
        "frost_attribution_authorized",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn snowdensity08_default_is_superseded_by_10_3_15_and_script_reports_blocker() {
    let builder = read(DIRECT_PUBLICATION_BUILDER);
    assert!(
        builder.contains("SNOWDENSITY09_DENSITY_MODEL_ENV")
            && builder.contains("Err(std::env::VarError::NotPresent)")
            && builder.contains("SnowDensityModel::PhysicsBulkDensityCompactionV1")
            && builder.contains("\"legacy_wepp\" => Ok"),
        "SNOWDENSITY-10.3.15 must supersede the SNOWDENSITY-08 default while retaining legacy rollback"
    );
    assert!(
        !read(CLI).contains("physics_bulk_density_compaction_v1"),
        "openwepp-cli-hill must not expose user CLI density activation"
    );

    let script = read(SCRIPT);
    for marker in [
        "frost_attribution_authorized",
        "non_snotel_runtime_opt_in_coupled",
        "NON-SNOTEL-COUPLED-OPT-IN-WAT-PATH-ABSENT",
        "offline snow-only depth",
    ] {
        assert_contains(&script, marker, SCRIPT);
    }
}

#[test]
fn snowdensity08_executed_report_keeps_frost_attribution_blocked() {
    let report: Value = serde_json::from_str(&read(REPORT)).expect("report JSON parses");
    assert_eq!(report["schema"], "snowdensity08-snow-frost-gate-rerun-v1");
    assert_eq!(
        report["summary"]["disposition"],
        "COMPLETE-08-SNOTEL-CLEARED-FROST-ATTRIBUTION-BLOCKED"
    );
    assert_eq!(
        report["summary"]["snotel_opt_in_density_gate_cleared"],
        true
    );
    assert_eq!(
        report["summary"]["non_snotel_runtime_opt_in_coupled"],
        false
    );
    assert_eq!(report["summary"]["frost_attribution_authorized"], false);
    assert_eq!(
        report["summary"]["blocker"],
        "NON-SNOTEL-COUPLED-OPT-IN-WAT-PATH-ABSENT"
    );
    assert_eq!(
        report["non_snotel"]["summary"]["snow_control_status_counts"]["SNOW_CONTROL_FAILED"],
        3
    );
    assert_eq!(
        report["non_snotel"]["summary"]["snow_control_status_counts"]["MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW"],
        2
    );
    assert_eq!(
        report["snotel"]["summary"]["best_model"],
        "coe_bound_density_compaction_v1_coe_shortwave_albedo_v1"
    );
    assert_eq!(report["snotel"]["summary"]["beats_openwepp_as_built"], true);
    assert_eq!(report["snotel"]["summary"]["beats_legacy_as_built"], true);
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
