use std::fs;
use std::path::Path;

const PACKAGE: &str =
    "docs/work-packages/20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/package.md";
const TOOL: &str = "tools/snowfreeze_observed/winter_thaw_melt_response.py";
const REPORT_JSON: &str = "docs/work-packages/20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/artifacts/winter-thaw-melt-response.json";
const REPORT_MD: &str = "docs/work-packages/20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/artifacts/winter-thaw-melt-response.md";
const WORK_PACKAGE_INDEX: &str = "docs/work-packages/README.md";
const ROOT_CARGO: &str = "Cargo.toml";

fn repo_text(relative_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn assert_contains(haystack: &str, needle: &str, context: &str) {
    assert!(
        haystack.contains(needle),
        "{context} missing required marker: {needle}"
    );
}

#[test]
fn package_binds_winter_thaw_melt_response_scope() {
    let package = repo_text(PACKAGE);
    for marker in [
        "Winter-Thaw Melt Response Diagnosis",
        "observed snow-depth ablation intervals",
        "positive-temperature snowpack hours",
        "warm-rain heat equivalent as diagnostic context only",
        "No production physics changes",
        "No sub-canopy longwave or rain-heat correction",
        "No defect verdict for observation-blocked surfaces",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn diagnostic_tool_preserves_legacy_coe_replay_boundary() {
    let tool = repo_text(TOOL);
    for marker in [
        "openwepp-snowbench coe-melt --model legacy_coe",
        "legacy_coe",
        "positive-temperature snowpack hours",
        "warm-rain",
        "OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY",
        "WINTER-THAW-MELT-RESPONSE-",
        "does not change production physics",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert!(
        !tool.contains("snow_melt_model ="),
        "{TOOL} must not add a production snow-melt selector"
    );
    assert!(
        !tool.contains("OPENWEPP_SNOWDENSITY1035_PHASE_MODEL"),
        "{TOOL} must not rerun the phase-partition opt-in selector"
    );
}

#[test]
fn executed_report_records_event_window_disposition() {
    let report_json = repo_text(REPORT_JSON);
    for marker in [
        "snowdensity10-3-6-winter-thaw-melt-response-v1",
        "WINTER-THAW-MELT-RESPONSE-",
        "default_activation_changed",
        "parser_runfile_user_cli_selector_added",
        "fixture_inputs_changed",
        "public_output_schema_changed",
        "under_ablation_interval_count",
        "warm_rain_heat_melt_equiv_m",
        "observation_blocked_surface_ids",
        "sleepers_south_field",
        "harvard_open",
    ] {
        assert_contains(&report_json, marker, REPORT_JSON);
    }

    let report_md = repo_text(REPORT_MD);
    for marker in [
        "Evidence mode: Static/Ran.",
        "Default activation changed: `False`",
        "Warm-rain heat and sub-canopy longwave are reported as context only",
    ] {
        assert_contains(&report_md, marker, REPORT_MD);
    }
}

#[test]
fn work_package_index_and_test_target_are_registered() {
    let index = repo_text(WORK_PACKAGE_INDEX);
    assert_contains(
        &index,
        "20260627-snowdensity-10-3-6-winter-thaw-melt-response-001",
        WORK_PACKAGE_INDEX,
    );

    let cargo = repo_text(ROOT_CARGO);
    for marker in [
        "snowdensity10_3_6_winter_thaw_melt_response",
        "tests/integration/snowdensity10_3_6_winter_thaw_melt_response.rs",
    ] {
        assert_contains(&cargo, marker, ROOT_CARGO);
    }
}
