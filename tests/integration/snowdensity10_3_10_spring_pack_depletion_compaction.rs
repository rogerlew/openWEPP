use std::fs;

use serde_json::Value;

const PACKAGE: &str = "docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/package.md";
const REPORT: &str = "docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/artifacts/spring-pack-depletion-compaction-adjudication.json";
const TOOL: &str = "tools/snowfreeze_observed/spring_pack_depletion_compaction_adjudication.py";

#[test]
fn snowdensity10_3_10_package_defines_density_cap_feasibility_gate() {
    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.10 Spring Pack-Depletion and Compaction Adjudication",
        "522 kg m^-3",
        "compaction-only-feasible",
        "cap-limited-depletion",
        "patchy-meltout",
        "Diagnostic boundaries are preserved",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn snowdensity10_3_10_tool_preserves_diagnostic_boundaries() {
    let tool = read(TOOL);
    for marker in [
        "diagnostic-only",
        "SNOW_DENSITY_CAP_KG_M3 = 522.0",
        "COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP",
        "CAP_LIMITED_DEPLETION_REQUIRED",
        "PATCHY_MELTOUT_OR_DEPLETION_REQUIRED",
        "production_physics_changed",
        "density_cap_changed",
        "fixture_inputs_changed",
        "public_output_schema_changed",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert_not_contains(&tool, "subprocess.run", TOOL);
    assert_not_contains(&tool, "OPENWEPP_SNOWDENSITY10310", TOOL);
}

#[test]
fn snowdensity10_3_10_committed_report_has_required_disposition() {
    let report: Value = serde_json::from_str(&read(REPORT))
        .expect("spring pack depletion/compaction report should be valid JSON");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-10-spring-pack-depletion-compaction-adjudication-v1"
    );
    assert_eq!(
        report["summary"]["candidate_model"],
        "coe_liquid_holding_capacity_v1"
    );
    assert_eq!(report["summary"]["density_cap_kg_m3"], 522.0);
    assert_eq!(
        report["protected_boundaries"]["production_physics_changed"],
        false
    );
    assert_eq!(report["protected_boundaries"]["density_cap_changed"], false);
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
    assert!(
        report["summary"]["depletion_required_failure_count"]
            .as_u64()
            .expect("depletion-required count")
            > 0
    );

    let surfaces = report["surfaces"].as_array().expect("surfaces array");
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
