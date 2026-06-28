use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, SnowDensityModel, SnowMeltModel,
    Wb11HydrologyKernel, snow_density_spring_densification_v1_constants,
};
use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = "docs/work-packages/20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001/package.md";
const BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00_builders_and_authority.rs"
);
const TOOL: &str = "tools/snowfreeze_observed/spring_compaction_densification_candidate.py";
const REPORT: &str = concat!(
    "docs/work-packages/20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001/",
    "artifacts/spring-compaction-densification-candidate.json"
);
const TOL: f64 = 1.0e-12;

#[test]
fn contract_and_package_bind_spring_densification_candidate() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 105",
        "physics_bulk_spring_densification_v1",
        "INV-SNOWFREEZE-068",
        "OBL-SNOWFREEZE-P-043",
        "SNOWDENSITY-10.3.11 Opt-In Spring Compaction/Densification Addendum",
        "must not consume observed snow depth",
        "final density `<= 522 kg m^-3`",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.11 Spring Compaction/Densification Candidate",
        "physics_bulk_spring_densification_v1",
        "Densification must be a physical wet-snow/melt-freeze compaction process",
        "same total liquid",
        "liquid-compaction term",
        "does not claim activation or frost-attribution clearance",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn runtime_candidate_preserves_swe_and_density_cap() {
    let constants = snow_density_spring_densification_v1_constants();
    assert_eq!(constants.wet_compaction_substeps_per_day, 24);
    assert_close(constants.dry_compaction_max_density_kg_m3, 550.0);
    assert_close(constants.wet_compaction_max_density_kg_m3, 550.0);

    let density_baseline = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        wet_spring_inputs(SnowDensityModel::PhysicsBulkDensityCompactionV1),
    )
    .expect("density compaction baseline should compute");
    let spring = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
        wet_spring_inputs(SnowDensityModel::PhysicsBulkSpringDensificationV1),
    )
    .expect("spring densification candidate should compute");

    assert_eq!(
        spring.snow_density_model,
        SnowDensityModel::PhysicsBulkSpringDensificationV1
    );
    assert_close(spring.raw_melt_m, density_baseline.raw_melt_m);
    assert_close(
        spring.redistributed_melt_m,
        density_baseline.redistributed_melt_m,
    );
    assert_close(spring.routed_melt_m, density_baseline.routed_melt_m);
    assert_close(
        spring.snowpack_swe_loss_m,
        density_baseline.snowpack_swe_loss_m,
    );
    assert_close(
        spring.runtime_swe_after_m,
        density_baseline.runtime_swe_after_m,
    );
    assert!(spring.density_swe_identity_residual_m <= TOL);
    assert!(spring.runtime_density_after_kg_m3 <= 522.0 + TOL);
    assert!(
        spring.runtime_density_after_kg_m3 >= density_baseline.runtime_density_after_kg_m3 - TOL,
        "spring densification must not make a wet pack fluffier than density_compaction_v1"
    );
    assert!(
        spring.runtime_depth_after_m <= density_baseline.runtime_depth_after_m + TOL,
        "spring densification should not deepen the same-SWE wet pack"
    );
}

#[test]
fn selector_and_tool_keep_diagnostic_boundaries() {
    let builder = read(BUILDER);
    for marker in [
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "SnowDensityModel::PhysicsBulkDensityCompactionV1",
        "must be legacy_wepp, physics_bulk_density_compaction_v1, or physics_bulk_shallow_guard_v1",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }
    assert!(
        !builder.contains("SnowDensityModel::PhysicsBulkSpringDensificationV1"),
        "SNOWDENSITY-10.3.15 must not retain rejected spring densification in the active selector"
    );

    let tool = read(TOOL);
    for marker in [
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "physics_bulk_density_compaction_v1",
        "physics_bulk_spring_densification_v1",
        "observed_depth_or_density_consumed_by_runtime",
        "density_cap_changed",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
}

#[test]
fn executed_report_records_coupled_candidate_disposition() {
    let report: Value = serde_json::from_str(&read(REPORT))
        .expect("spring densification report should be valid JSON");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-11-spring-compaction-densification-candidate-v1"
    );
    assert_eq!(
        report["diagnostic_selector"]["spring_candidate"],
        "physics_bulk_spring_densification_v1"
    );
    assert_eq!(report["protected_boundaries"]["density_cap_changed"], false);
    assert_eq!(
        report["protected_boundaries"]["observed_depth_or_density_consumed_by_runtime"],
        false
    );
    assert!(
        report["summary"]["candidate_snow_control_fail_count"]
            .as_u64()
            .expect("candidate fail count")
            > 0,
        "package should not silently clear snow-control without evidence"
    );
}

fn wet_spring_inputs(model: SnowDensityModel) -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    for hour in hourly.iter_mut().take(8) {
        hour.air_temperature_c = 4.0;
        hour.radiation_mj_m2 = 4.0;
        hour.cloud_fraction = 1.0;
    }

    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.30,
        runtime_depth_m: 1.20,
        runtime_density_kg_m3: 250.0,
        runtime_settle_day_count: 20.0,
        liquid_water_retained_m: 0.0,
        tmax_c: 4.0,
        tmin_c: 1.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 1.0,
        dewpoint_c: 0.0,
        snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
        snow_density_model: model,
        coe_boundary_depth_m: 1.20,
        coe_boundary_density_kg_m3: 250.0,
        coe_boundary_settle_day_count: 20.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        underlying_surface_albedo: 0.2,
        hourly,
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
