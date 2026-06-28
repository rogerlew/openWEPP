use std::fs;
use std::path::Path;

use openwepp_hillslope_orchestrator::{
    STURM1995_CDM_CRITICAL_TEMPERATURE_C, STURM1995_EPHEMERAL_CDM_THRESHOLD_C_MONTH,
    STURM1995_HIGH_LOW_CDM_THRESHOLD_C_MONTH, STURM1995_HIGH_PRECIP_SPR_THRESHOLD_MM_DAY,
    STURM1995_HIGH_WIND_MIN_M_S, STURM1995_LOW_WIND_MAX_M_S, SnowClimateClass, SnowDensityModel,
    SnowDensityRuntimeInputs, Sturm1995ClimateClassAssignmentError, Sturm1995ClimateNormals,
    sturm1995_climate_class_from_normals, sturm2010_bulk_density_kg_m3,
    sturm2010_density_parameters_for_class, update_snow_density_runtime_state,
};
use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = concat!(
    "docs/work-packages/",
    "20260628-snowdensity-10-3-22-climate-class-density-specialization-001/package.md"
);
const ARTIFACT: &str = concat!(
    "docs/work-packages/",
    "20260628-snowdensity-10-3-22-climate-class-density-specialization-001/",
    "artifacts/climate-class-density-specialization.json"
);
const AUTHORITY_GAP: &str = concat!(
    "docs/work-packages/",
    "20260628-snowdensity-10-3-22-climate-class-density-specialization-001/",
    "artifacts/authority-gap-and-disposition.md"
);
const BUILDER: &str = concat!(
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/",
    "00_builders_and_authority.rs"
);
const CLI: &str = "crates/openwepp-runner/src/bin/openwepp-cli-hill.rs";
const TOL: f64 = 1.0e-12;

#[test]
fn contract_and_package_bind_source_verified_climate_class_candidate() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 107",
        "physics_bulk_climate_class_density_v1",
        "snow_climate_class",
        "sturm1995_climate_normals",
        "sturm2010_density_parameters",
        "INV-SNOWFREEZE-077",
        "OBL-SNOWFREEZE-P-052",
        "REF-SNOWFREEZE-STURM2010-DENSITY",
        "REF-SNOWFREEZE-STURM1995-CLASSIFICATION",
        "REF-SNOWFREEZE-STURM2021-CLASSIFICATION-CROSSCHECK",
        "REF-SNOWFREEZE-NSIDC0768",
        "CDM < 30 degC-month",
        "0.5 < wind < 2.0 m s^-1",
        "fresh-snow/Anderson compaction behavior",
        "SNOWDENSITY-10.3.22 Climate-Class Density Specialization Addendum",
        "cross-SNOTEL primary gate",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "HOLD-GATE-FAILURE-NON-PROMOTION",
        "all six Sturm 1995 class labels",
        "sturm-thresholds-source-verification.md",
        "climate-class candidate profile: `16` robust fails / `168` robust score",
        "CDM threshold values `30` and `125 degC-month`",
        "SPR threshold `2 mm day^-1`",
        "wind bracket `0.5-2.0 m s^-1`",
        "Sturm/Liston 2021 is recorded as a cross-check",
        "fresh-snow/Anderson compaction behavior for ephemeral",
        "No thresholds, parameters, class mappings, or smoothing were fitted",
        "No fixture, public output schema, density cap, frost behavior",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn sturm1995_thresholds_and_tree_are_source_bound_and_fail_closed() {
    assert_close(STURM1995_CDM_CRITICAL_TEMPERATURE_C, 10.0);
    assert_close(STURM1995_EPHEMERAL_CDM_THRESHOLD_C_MONTH, 30.0);
    assert_close(STURM1995_HIGH_LOW_CDM_THRESHOLD_C_MONTH, 125.0);
    assert_close(STURM1995_HIGH_PRECIP_SPR_THRESHOLD_MM_DAY, 2.0);
    assert_close(STURM1995_LOW_WIND_MAX_M_S, 0.5);
    assert_close(STURM1995_HIGH_WIND_MIN_M_S, 2.0);

    assert_eq!(class(20.0, 0.0, 1.0), SnowClimateClass::Ephemeral);
    assert_eq!(class(150.0, 1.0, 0.5), SnowClimateClass::Taiga);
    assert_eq!(class(150.0, 1.0, 2.0), SnowClimateClass::Tundra);
    assert_eq!(class(80.0, 1.0, 0.5), SnowClimateClass::Alpine);
    assert_eq!(class(80.0, 1.0, 2.0), SnowClimateClass::Prairie);
    assert_eq!(class(80.0, 2.0, 1.0), SnowClimateClass::Maritime);

    let ambiguous = sturm1995_climate_class_from_normals(normals(80.0, 1.0, 1.0))
        .expect_err("wind-dependent branch must fail inside unresolved 1995 bracket");
    assert!(matches!(
        ambiguous,
        Sturm1995ClimateClassAssignmentError::AmbiguousWindThreshold { .. }
    ));

    let rare = sturm1995_climate_class_from_normals(normals(150.0, 2.0, 2.0))
        .expect_err("rare deep branch must not be silently reduced to six-class label");
    assert!(matches!(
        rare,
        Sturm1995ClimateClassAssignmentError::RareClassCombination { .. }
    ));
}

#[test]
fn sturm2010_density_parameters_are_explicit_and_ephemeral_fails_closed() {
    let alpine = sturm2010_density_parameters_for_class(SnowClimateClass::Alpine)
        .expect("alpine should have Sturm 2010 parameters");
    assert_close(alpine.max_density_g_cm3, 0.5975);
    assert_close(alpine.initial_density_g_cm3, 0.2237);
    assert_close(alpine.depth_densification_per_cm, 0.0012);
    assert_close(alpine.day_densification_per_day, 0.0038);

    let taiga_density = sturm2010_bulk_density_kg_m3(SnowClimateClass::Taiga, 1.0, 100.0).unwrap();
    assert_close(taiga_density, 217.0);

    let alpine_density =
        sturm2010_bulk_density_kg_m3(SnowClimateClass::Alpine, 1.0, 100.0).unwrap();
    let tundra_density =
        sturm2010_bulk_density_kg_m3(SnowClimateClass::Tundra, 1.0, 100.0).unwrap();
    assert!(alpine_density > tundra_density);

    assert!(sturm2010_density_parameters_for_class(SnowClimateClass::Ephemeral).is_none());
    let err = sturm2010_bulk_density_kg_m3(SnowClimateClass::Ephemeral, 0.2, 100.0)
        .expect_err("ephemeral must fail closed without parameters");
    assert_contains(&err.to_string(), "ephemeral", "ephemeral error");
}

#[test]
fn climate_class_candidate_requires_authoritative_operands_and_conserves_when_explicit() {
    let missing_class = update_snow_density_runtime_state(climate_inputs(None, Some(100.0)))
        .expect_err("candidate should require class assignment");
    assert_contains(
        &missing_class.to_string(),
        "requires forcing-derived Sturm climate class",
        "missing class error",
    );

    let missing_day =
        update_snow_density_runtime_state(climate_inputs(Some(SnowClimateClass::Alpine), None))
            .expect_err("candidate should require Sturm day");
    assert_contains(
        &missing_day.to_string(),
        "requires Sturm density day-of-year",
        "missing day error",
    );

    let ephemeral_fallback = update_snow_density_runtime_state(climate_inputs(
        Some(SnowClimateClass::Ephemeral),
        Some(100.0),
    ))
    .expect("ephemeral should retain process-first fresh-snow/Anderson fallback");
    assert_eq!(
        ephemeral_fallback.model,
        SnowDensityModel::PhysicsBulkClimateClassDensityV1
    );
    assert!(!ephemeral_fallback.sturm_density_form_fallback_used);
    assert_close(ephemeral_fallback.runtime_swe_after_m, 0.2);

    let outcome = update_snow_density_runtime_state(climate_inputs(
        Some(SnowClimateClass::Alpine),
        Some(100.0),
    ))
    .expect("explicit supported class should compute");
    assert_eq!(
        outcome.model,
        SnowDensityModel::PhysicsBulkClimateClassDensityV1
    );
    assert!(outcome.sturm_density_form_fallback_used);
    assert!(outcome.runtime_density_after_kg_m3 > 250.0);
    assert!(outcome.runtime_density_after_kg_m3 <= 522.0 + TOL);
    assert_close(outcome.runtime_swe_after_m, 0.2);
    assert!(outcome.max_abs_swe_identity_residual_m <= TOL);
}

#[test]
fn selector_is_internal_opt_in_only_and_artifact_records_non_promotion() {
    let builder = read(BUILDER);
    for marker in [
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "physics_bulk_climate_class_density_v1",
        "SnowDensityModel::PhysicsBulkClimateClassDensityV1",
        "sturm_climate_class",
        "sturm_day_of_year",
        "direct_production_sturm_climate_class_for_density_candidate",
        "sturm1995_climate_class_from_normals",
        "must be legacy_wepp, physics_bulk_density_compaction_v1, physics_bulk_shallow_guard_v1, or physics_bulk_climate_class_density_v1",
    ] {
        assert_contains(&builder, marker, BUILDER);
    }
    assert!(
        !read(CLI).contains("physics_bulk_climate_class_density_v1"),
        "climate-class candidate must not be exposed as user CLI"
    );

    let report: Value =
        serde_json::from_str(&read(ARTIFACT)).expect("artifact should be valid JSON");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-22-climate-class-density-specialization-v1"
    );
    assert_eq!(
        report["candidate"]["snow_density_model"],
        "physics_bulk_climate_class_density_v1"
    );
    assert_eq!(report["activation_authorized"], false);
    assert_eq!(report["default_changed"], false);
    assert_eq!(report["disposition"], "HOLD-GATE-FAILURE-NON-PROMOTION");
    assert_eq!(
        report["cross_snotel_rerun"]["candidate_robust_fail_count"],
        16
    );
    assert_eq!(
        report["cross_snotel_rerun"]["candidate_robust_ordinal_score"],
        168
    );
    assert_eq!(
        report["cross_snotel_rerun"]["candidate_worse_robust_cells_vs_activated"],
        13
    );
    assert_eq!(
        report["authority"]["sturm1995_numeric_decision_tree_thresholds_available"],
        true
    );
    assert_eq!(
        report["authority"]["sturm2010_ephemeral_density_parameters_available"],
        false
    );
    assert_eq!(
        report["authority"]["ephemeral_fresh_snow_anderson_fallback_documented"],
        true
    );
    assert_eq!(report["protected_boundaries"]["density_cap_changed"], false);

    let authority_gap = read(AUTHORITY_GAP);
    for marker in [
        "Ephemeral is part of the six-class Sturm snow-class system",
        "numeric Sturm 1995 binary decision-tree thresholds are now source-verified",
        "climate-class candidate: `16` robust fails / `168` robust score",
        "selector is reserved and fail-closed",
    ] {
        assert_contains(&authority_gap, marker, AUTHORITY_GAP);
    }
}

fn climate_inputs(
    sturm_climate_class: Option<SnowClimateClass>,
    sturm_day_of_year: Option<f64>,
) -> SnowDensityRuntimeInputs {
    SnowDensityRuntimeInputs {
        model: SnowDensityModel::PhysicsBulkClimateClassDensityV1,
        prior_swe_m: 0.2,
        prior_depth_m: 0.8,
        prior_density_kg_m3: 250.0,
        boundary_swe_after_m: 0.2,
        boundary_depth_after_m: 0.8,
        boundary_density_after_kg_m3: 250.0,
        snow_input_m: 0.0,
        liquid_for_compaction_m: 0.0,
        mean_air_temperature_c: -5.0,
        runtime_density_cap_kg_m3: 522.0,
        sturm_climate_class,
        sturm_day_of_year,
    }
}

fn normals(
    cooling_degree_month_c: f64,
    snowfall_precipitation_rate_mm_day: f64,
    winter_wind_m_s: f64,
) -> Sturm1995ClimateNormals {
    Sturm1995ClimateNormals {
        cooling_degree_month_c,
        snowfall_precipitation_rate_mm_day,
        winter_wind_m_s,
    }
}

fn class(
    cooling_degree_month_c: f64,
    snowfall_precipitation_rate_mm_day: f64,
    winter_wind_m_s: f64,
) -> SnowClimateClass {
    sturm1995_climate_class_from_normals(normals(
        cooling_degree_month_c,
        snowfall_precipitation_rate_mm_day,
        winter_wind_m_s,
    ))
    .expect("source-bound Sturm 1995 class should resolve")
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
