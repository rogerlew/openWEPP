use std::fs;
use std::path::Path;

const PACKAGE: &str = "docs/work-packages/20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001/package.md";
const TOOL: &str = "tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py";
const REPORT_JSON: &str = "docs/work-packages/20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001/artifacts/phase-partition-snowdepth-impact.json";
const REPORT_MD: &str = "docs/work-packages/20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001/artifacts/phase-partition-snowdepth-impact.md";
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
fn package_binds_phase_partition_snow_depth_impact_scope() {
    let package = repo_text(PACKAGE);
    for marker in [
        "Phase Partition Snow-Depth Impact Adjudication",
        "real direct-production WAT path",
        "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=harder_pomeroy_hourly",
        "No default activation",
        "No fixture input edits",
        "No public output schema changes",
        "No snow density, melt, canopy, radiation, albedo, frost",
        "No defect verdict for observation-blocked surfaces",
        "Jennings observed-phase validation alone proves snow-depth",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn diagnostic_tool_uses_direct_wat_path_not_snowbench_replay() {
    let tool = repo_text(TOOL);
    for marker in [
        "openwepp-cli-hill",
        "--direct-production-executor",
        "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL",
        "harder_pomeroy_hourly",
        "phase-partition-snowdepth-impact.json",
        "OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert!(
        !tool.contains("coe-melt"),
        "{TOOL} must not use the snowbench coe-melt replay path"
    );
}

#[test]
fn executed_report_records_disposition_and_boundaries() {
    let report_json = repo_text(REPORT_JSON);
    for marker in [
        "snowdensity10-3-5c-phase-partition-snowdepth-impact-v1",
        "PHASE-PARTITION-",
        "default_activation_changed",
        "parser_runfile_user_cli_selector_added",
        "fixture_inputs_changed",
        "public_output_schema_changed",
        "observation_blocked_surface_ids",
        "sleepers_south_field",
        "harvard_open",
    ] {
        assert_contains(&report_json, marker, REPORT_JSON);
    }

    let report_md = repo_text(REPORT_MD);
    for marker in [
        "Evidence mode: Static/Ran.",
        "Default activation changed: `false`.",
        "Observation-blocked surfaces are diagnostic-only",
    ] {
        assert_contains(&report_md, marker, REPORT_MD);
    }
}

#[test]
fn work_package_index_and_test_target_are_registered() {
    let index = repo_text(WORK_PACKAGE_INDEX);
    assert_contains(
        &index,
        "20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001",
        WORK_PACKAGE_INDEX,
    );

    let cargo = repo_text(ROOT_CARGO);
    for marker in [
        "snowdensity10_3_5c_phase_partition_snowdepth_impact",
        "tests/integration/snowdensity10_3_5c_phase_partition_snowdepth_impact.rs",
    ] {
        assert_contains(&cargo, marker, ROOT_CARGO);
    }
}
