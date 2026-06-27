use std::fs;

use serde_json::Value;

const PACKAGE: &str = "docs/work-packages/20260627-snowdensity-10-3-9-march-april-residual-attribution-001/package.md";
const REPORT: &str = "docs/work-packages/20260627-snowdensity-10-3-9-march-april-residual-attribution-001/artifacts/march-april-residual-attribution.json";
const TOOL: &str = "tools/snowfreeze_observed/march_april_residual_attribution.py";

#[test]
fn snowdensity10_3_9_package_defines_march_april_attribution_gate() {
    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.9 March/April Residual Attribution",
        "SNOWDENSITY-10.3.8 opt-in liquid holding-capacity",
        "March/April",
        "observation-blocked",
        "recommended next one-lever route",
        "Diagnostic boundaries are preserved",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn snowdensity10_3_9_tool_preserves_diagnostic_boundaries() {
    let tool = read(TOOL);
    for marker in [
        "diagnostic-only",
        "MARCH_APRIL_MONTHS = {3, 4}",
        "coe_liquid_holding_capacity_v1",
        "PATCHY_MELTOUT_OR_SNOW_COVER_DEPLETION",
        "DENSITY_OR_COMPACTION_DEFICIT",
        "SWE_EXCESS_OR_ABLATION_DEFICIT",
        "DEPTH_ONLY_OVERPERSISTENCE_UNRESOLVED",
        "production_physics_changed",
        "fixture_inputs_changed",
        "public_output_schema_changed",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert_not_contains(&tool, "OPENWEPP_SNOWDENSITY", TOOL);
    assert_not_contains(&tool, "subprocess.run", TOOL);
}

#[test]
fn snowdensity10_3_9_committed_report_has_required_disposition() {
    let report: Value = serde_json::from_str(&read(REPORT))
        .expect("March/April attribution report should be valid JSON");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-9-march-april-residual-attribution-v1"
    );
    assert_eq!(
        report["summary"]["disposition"],
        "MARCH_APRIL-RESIDUALS-ATTRIBUTED"
    );
    assert_eq!(
        report["summary"]["candidate_model"],
        "coe_liquid_holding_capacity_v1"
    );
    assert_eq!(
        report["protected_boundaries"]["production_physics_changed"],
        false
    );
    assert_eq!(
        report["protected_boundaries"]["fixture_inputs_changed"],
        false
    );
    assert_eq!(
        report["protected_boundaries"]["public_output_schema_changed"],
        false
    );

    assert!(
        report["summary"]["march_april_fail_count"]
            .as_u64()
            .expect("March/April fail count")
            > 0
    );
    assert_eq!(
        report["summary"]["remaining_blocker"],
        "SNOW-CONTROL-NOT-CLEARED"
    );

    let surfaces = report["surfaces"].as_array().expect("surfaces array");
    assert_eq!(
        surfaces
            .iter()
            .filter(|item| item["verdict_scope"] == "paired_observation")
            .count(),
        4
    );
    assert!(surfaces.iter().any(|item| {
        item["surface_id"] == "hubbardbrook_mixed" && item["verdict_scope"] == "observation_blocked"
    }));
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

fn assert_not_contains(text: &str, marker: &str, path: &str) {
    assert!(
        !text.contains(marker),
        "expected {path} not to contain marker: {marker}"
    );
}
