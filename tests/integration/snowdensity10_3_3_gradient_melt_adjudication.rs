use std::fs;

use serde_json::Value;

const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-10-3-3-gradient-melt-adjudication-001/package.md";
const REPORT: &str = "docs/work-packages/20260626-snowdensity-10-3-3-gradient-melt-adjudication-001/artifacts/gradient_melt_adjudication.json";
const TOOL: &str = "tools/snowfreeze_observed/cancov_gradient_melt_adjudication.py";

#[test]
fn snowdensity10_3_3_package_defines_gradient_melt_gate() {
    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.3 Gradient Melt Adjudication",
        "conifer, mixed, deciduous, and open/pasture",
        "No melt coefficient, albedo constant, shared-radiation, canopy, density,",
        "Marcell conifer, Marcell deciduous, Marcell open, Harvard hardwood, and",
        "Harvard hemlock is explicitly reported as unbound/non-verdict.",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn snowdensity10_3_3_tool_preserves_diagnostic_boundaries() {
    let tool = read(TOOL);
    for marker in [
        "diagnostic-only",
        "legacy_coe",
        "coe_shortwave_albedo_v1",
        "verdict_bearing",
        "diagnostic_unweighted_aggregate",
        "unbound_no_pure_conifer_fixture",
        "run_coe_melt",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert_not_contains(&tool, "DEFAULT_CANOPY_COVER_FRACTION", TOOL);
    assert_not_contains(&tool, "snow_melt_model =", TOOL);
}

#[test]
fn snowdensity10_3_3_committed_report_has_required_regime_profiles() {
    let report: Value =
        serde_json::from_str(&read(REPORT)).expect("gradient melt report should be valid JSON");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-3-gradient-melt-adjudication-v1"
    );
    assert_eq!(report["summary"]["default_activation_authorized"], false);
    assert_eq!(report["summary"]["promotion_authorized"], false);

    let regimes = report["summary"]["regimes"]
        .as_object()
        .expect("regimes object");
    for regime in ["conifer", "mixed", "deciduous", "open_pasture"] {
        assert!(regimes.contains_key(regime), "missing regime {regime}");
    }

    let comparisons = report["comparison_set"]
        .as_array()
        .expect("comparison_set array");
    assert!(comparisons.iter().any(|item| {
        item["fixture"] == "marcell_conifer_mn" && item["verdict_scope"] == "verdict_bearing"
    }));
    assert!(comparisons.iter().any(|item| {
        item["fixture"] == "harvard_mixed_ma" && item["verdict_scope"] == "diagnostic_only"
    }));

    let unbound = report["unbound_observations"]
        .as_array()
        .expect("unbound observations array");
    assert!(unbound.iter().any(|item| {
        item["observed_stratum"] == "hemlock"
            && item["binding_status"] == "unbound_no_pure_conifer_fixture"
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
