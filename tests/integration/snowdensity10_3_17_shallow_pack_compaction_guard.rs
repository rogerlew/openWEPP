use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    SnowDensityModel, SnowDensityRuntimeInputs, snow_density_shallow_guard_v1_constants,
    update_snow_density_runtime_state,
};
use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = concat!(
    "docs/work-packages/",
    "20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001/package.md"
);
const BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00_builders_and_authority.rs"
);
const CLI: &str = "crates/openwepp-runner/src/bin/openwepp-cli-hill.rs";
const TOOL: &str = "tools/snowfreeze_observed/shallow_pack_compaction_guard.py";
const REPORT: &str = concat!(
    "docs/work-packages/",
    "20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001/",
    "artifacts/shallow-pack-compaction-guard.json"
);
const TOL: f64 = 1.0e-12;

#[test]
fn contract_and_package_bind_shallow_guard_candidate() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 111",
        "physics_bulk_shallow_guard_v1",
        "snow_shallow_compaction_guard_depth_threshold",
        "INV-SNOWFREEZE-074",
        "OBL-SNOWFREEZE-P-049",
        "SNOWDENSITY-10.3.17 Shallow-Pack Compaction Guard Addendum",
        "0.25 m",
        "must not consume observed snow depth",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.17 Shallow-Pack Compaction Guard",
        "physics_bulk_shallow_guard_v1",
        "density-arm-induced under-persistence",
        "harvard_hardwood",
        "No density-cap change",
        "close `HOLD`",
        "non-promotion",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn shallow_guard_reduces_only_shallow_density_aggression() {
    let constants = snow_density_shallow_guard_v1_constants();
    assert_close(constants.shallow_compaction_guard_depth_threshold_m, 0.25);
    assert_eq!(constants.wet_compaction_substeps_per_day, 1);
    assert_close(constants.dry_compaction_multiplier, 4.0);
    assert_close(constants.wet_compaction_multiplier, 2.0);

    let baseline_shallow = update_snow_density_runtime_state(&shallow_inputs(
        SnowDensityModel::PhysicsBulkDensityCompactionV1,
        0.18,
    ))
    .expect("density baseline should compute");
    let guarded_shallow = update_snow_density_runtime_state(&shallow_inputs(
        SnowDensityModel::PhysicsBulkShallowGuardV1,
        0.18,
    ))
    .expect("shallow guard candidate should compute");

    assert_eq!(
        guarded_shallow.model,
        SnowDensityModel::PhysicsBulkShallowGuardV1
    );
    assert_close(
        guarded_shallow.runtime_swe_after_m,
        baseline_shallow.runtime_swe_after_m,
    );
    assert!(guarded_shallow.max_abs_swe_identity_residual_m <= TOL);
    assert!(guarded_shallow.runtime_density_after_kg_m3 <= 522.0 + TOL);
    assert!(
        guarded_shallow.runtime_density_after_kg_m3 < baseline_shallow.runtime_density_after_kg_m3,
        "shallow guard should reduce density only for shallow packs"
    );
    assert!(
        guarded_shallow.runtime_depth_after_m > baseline_shallow.runtime_depth_after_m,
        "same-SWE shallow guard should retain more physical depth"
    );

    let baseline_deep = update_snow_density_runtime_state(&shallow_inputs(
        SnowDensityModel::PhysicsBulkDensityCompactionV1,
        0.35,
    ))
    .expect("deep baseline should compute");
    let guarded_deep = update_snow_density_runtime_state(&shallow_inputs(
        SnowDensityModel::PhysicsBulkShallowGuardV1,
        0.35,
    ))
    .expect("deep guard should compute");
    assert_close(
        guarded_deep.runtime_density_after_kg_m3,
        baseline_deep.runtime_density_after_kg_m3,
    );
    assert_close(
        guarded_deep.runtime_depth_after_m,
        baseline_deep.runtime_depth_after_m,
    );
}

#[test]
fn selector_and_tool_keep_diagnostic_boundaries() {
    let builder = read(BUILDER);
    for marker in [
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "physics_bulk_density_compaction_v1",
        "physics_bulk_shallow_guard_v1",
        "SnowDensityModel::PhysicsBulkShallowGuardV1",
        "must be legacy_wepp, physics_bulk_density_compaction_v1, physics_bulk_shallow_guard_v1, physics_bulk_climate_class_density_v1, or physics_bulk_multilayer_density_v1",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }
    assert!(
        !read(CLI).contains("physics_bulk_shallow_guard_v1"),
        "SNOWDENSITY-10.3.17 must not expose the diagnostic selector as user CLI"
    );

    let tool = read(TOOL);
    for marker in [
        "MELT_ENV = active15.MELT_ENV",
        "DENSITY_ENV = active15.DENSITY_ENV",
        "DEFAULT_DENSITY_MODEL = active15.DEFAULT_DENSITY_MODEL",
        "physics_bulk_shallow_guard_v1",
        "harvard_hardwood",
        "snow_state_conservation_ok",
        "observed_depth_or_density_consumed_by_runtime",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
}

#[test]
fn executed_report_records_coupled_candidate_disposition() {
    let report: Value =
        serde_json::from_str(&read(REPORT)).expect("shallow guard report should be valid JSON");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-17-shallow-pack-compaction-guard-v1"
    );
    assert_eq!(
        report["candidate"]["snow_density_model"],
        "physics_bulk_shallow_guard_v1"
    );
    assert_eq!(
        report["baseline"]["snow_density_model"],
        "physics_bulk_density_compaction_v1"
    );
    assert_eq!(report["protected_boundaries"]["density_cap_changed"], false);
    assert_eq!(
        report["protected_boundaries"]["observed_depth_or_density_consumed_by_runtime"],
        false
    );
    assert_eq!(report["summary"]["candidate_trace_ok"], true);
    assert_eq!(report["summary"]["threshold_authority_ok"], true);
    assert_eq!(report["summary"]["activation_authorized"], false);
    assert_eq!(report["summary"]["promotion_eligible"], false);
    assert_eq!(
        report["summary"]["disposition"],
        "NON-PROMOTION-SHALLOW-GUARD-GATE-NOT-MET"
    );
    assert_eq!(report["summary"]["over_persistence_not_worse"], false);
    assert!(
        report["summary"]["candidate_paired_row_count"]
            .as_u64()
            .expect("candidate paired row count")
            > 0,
        "coupled report must include paired snow-depth rows"
    );
}

fn shallow_inputs(model: SnowDensityModel, prior_depth_m: f64) -> SnowDensityRuntimeInputs {
    let prior_swe_m = prior_depth_m * 250.0 / 1_000.0;
    SnowDensityRuntimeInputs {
        model,
        prior_swe_m,
        prior_depth_m,
        prior_density_kg_m3: 250.0,
        prior_settle_day_count: 0.0,
        prior_layers: Vec::new(),
        boundary_swe_after_m: prior_swe_m,
        boundary_depth_after_m: prior_depth_m,
        boundary_density_after_kg_m3: 250.0,
        snow_input_m: 0.0,
        liquid_for_compaction_m: 0.015,
        mean_air_temperature_c: -1.0,
        runtime_density_cap_kg_m3: 522.0,
        sturm_climate_class: None,
        sturm_day_of_year: None,
    }
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

fn assert_close(left: f64, right: f64) {
    assert!(
        (left - right).abs() <= TOL,
        "expected {left} to equal {right} within {TOL}"
    );
}
