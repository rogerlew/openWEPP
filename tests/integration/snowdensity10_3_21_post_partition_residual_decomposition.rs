use std::fs;

use serde_json::Value;

const TOOL: &str = "tools/snowfreeze_observed/post_partition_residual_decomposition.py";
const PACKAGE: &str = concat!(
    "docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/",
    "package.md"
);
const REPORT: &str = concat!(
    "docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/",
    "artifacts/post-partition-residual-decomposition.json"
);
const STRATEGY: &str = "docs/planning/snow-frost-fidelity-strategy.md";

#[test]
fn package_and_tool_are_diagnostic_only_and_authority_bound() {
    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.21",
        "diagnostic-only",
        "INV-SNOWFREEZE-050",
        "ADR-0028",
        "No production/default/cap/schema/fixture/frost change",
        "No new runtime selector",
        "Frost-attribution-threshold input is produced without deciding",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let tool = read(TOOL);
    for marker in [
        "snowdensity10-3-21-post-partition-residual-decomposition-v1",
        "Static + Reused Ran",
        "promotion_or_activation_decision_made",
        "frost_threshold_decision_made",
        "forcing_limited_absolute_swe_depth_report_only",
        "canopy_snow_interception_or_subcanopy_longwave",
        "wind_redistribution_or_forcing_representativeness",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
}

#[test]
fn committed_report_decomposes_current_default_without_unblocking_frost() {
    let report: Value = serde_json::from_str(&read(REPORT)).expect("10.3.21 report parses");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-21-post-partition-residual-decomposition-v1"
    );
    assert_eq!(report["diagnostic_only"], true);
    assert_eq!(report["promotion_or_activation_decision_made"], false);
    assert_eq!(report["frost_threshold_decision_made"], false);
    assert_eq!(
        report["summary"]["disposition"],
        "DIAGNOSTIC-COMPLETE-NO-PROMOTION-NO-FROST-DECISION"
    );
    assert_eq!(report["summary"]["current_default_robust_fail_count"], 15);
    assert_eq!(
        report["summary"]["current_default_robust_ordinal_score"],
        179
    );
    assert_eq!(
        report["summary"]["dominant_failed_signature"],
        "seasonal_densification_trajectory"
    );
    assert_eq!(
        report["protected_boundaries"]["production_default_changed"],
        false
    );
    assert_eq!(report["protected_boundaries"]["selector_added"], false);
    assert_eq!(
        report["protected_boundaries"]["frost_physics_changed"],
        false
    );

    let clusters = report["residual_clusters"]["by_cell_id"]
        .as_object()
        .expect("cell cluster counts");
    assert_eq!(clusters["seasonal_densification_trajectory"], 9);
    assert_eq!(clusters["seasonal_depth_swe_slope"], 2);
    assert_eq!(clusters["seasonal_ablation_meltout_date"], 2);
    assert_eq!(clusters["seasonal_peak_swe_date"], 2);

    assert_eq!(
        report["over_under_persistence_split"]["over_persistence_fail_count"],
        0
    );
    assert_eq!(
        report["over_under_persistence_split"]["under_persistence_fail_count"],
        4
    );
    assert_eq!(
        report["frost_attribution_threshold_input"]["not_a_decision"],
        true
    );
}

#[test]
fn strategy_records_10_3_21_as_threshold_input_not_frost_unblock() {
    let strategy = read(STRATEGY);
    for marker in [
        "SNOWDENSITY-10.3.21",
        "Post-partition residual decomposition",
        "15 / 179",
        "MIXED-NO-SINGLE-GLOBAL-SNOW-LEVER",
        "no frost-attribution threshold decision",
    ] {
        assert_contains(&strategy, marker, STRATEGY);
    }
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
