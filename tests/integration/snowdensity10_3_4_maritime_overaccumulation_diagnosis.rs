use std::fs;

use serde_json::Value;

const PACKAGE: &str = "docs/work-packages/20260627-snowdensity-10-3-4-maritime-overaccumulation-diagnosis-001/package.md";
const REPORT: &str = "docs/work-packages/20260627-snowdensity-10-3-4-maritime-overaccumulation-diagnosis-001/artifacts/maritime_overaccumulation_diagnosis.json";
const TOOL: &str = "tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py";

#[test]
fn snowdensity10_3_4_package_defines_maritime_diagnosis_gate() {
    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.4 Maritime Over-Accumulation Diagnosis",
        "HJ Andrews, Sleepers, Harvard, and Hubbard Brook",
        "snow/rain partition near 0 degC",
        "rain-on-snow heat / warm-rain energy",
        "possible sub-canopy longwave",
        "No production physics changes.",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn snowdensity10_3_4_tool_preserves_diagnostic_boundaries() {
    let tool = read(TOOL);
    for marker in [
        "diagnostic snowbench replay only",
        "legacy_coe",
        "snow_rain_partition_near_zero_c",
        "winter_thaw_melt_response",
        "sub_canopy_longwave_or_forest_energy",
        "OBSERVATION-BLOCKED",
        "FORCING-LIMITED",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert_not_contains(&tool, "coe_shortwave_albedo_v1", TOOL);
    assert_not_contains(&tool, "snow_melt_model =", TOOL);
}

#[test]
fn snowdensity10_3_4_committed_report_has_ranked_disposition() {
    let report: Value = serde_json::from_str(&read(REPORT))
        .expect("maritime diagnosis report should be valid JSON");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-4-maritime-overaccumulation-diagnosis-v1"
    );
    assert_eq!(report["no_physics_change"], true);
    assert_eq!(report["no_tuning"], true);
    assert_eq!(report["summary"]["disposition"], "PARTITION-THAW-FIRST");

    let surfaces = report["surfaces"].as_array().expect("surfaces array");
    for site in ["hjandrews", "sleepers", "harvard", "hubbardbrook"] {
        assert!(
            surfaces.iter().any(|item| item["site_group"] == site),
            "missing site group {site}"
        );
    }
    assert!(surfaces.iter().any(|item| {
        item["site_group"] == "hubbardbrook" && item["verdict_scope"] == "observation_blocked"
    }));
    assert!(surfaces.iter().any(|item| {
        item["site_group"] == "sleepers" && item["paired_row_count"].as_u64().unwrap_or(0) > 0
    }));

    let mechanisms = report["mechanism_ranking"]
        .as_array()
        .expect("mechanism ranking array");
    assert!(mechanisms.iter().any(|item| {
        item["mechanism"] == "snow_rain_partition_near_zero_c"
            && item["disposition"] == "DEFECT-ELIGIBLE"
    }));
    assert!(mechanisms.iter().any(|item| {
        item["mechanism"] == "precipitation_bias" && item["disposition"] == "FORCING-LIMITED"
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
